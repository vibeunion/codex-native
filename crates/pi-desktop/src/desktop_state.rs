use crate::{Language, ThemePreference};
use anyhow::Context as _;
use pi_agent_core::{auth::AuthProvider, EngineKind, PolicyProfile, Session};
use serde::{Deserialize, Serialize};
use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Preferences {
    pub api_base: Option<String>,
    /// Imported endpoints must not inherit process credentials after a restart.
    pub allow_environment_credentials: bool,
    pub model: String,
    pub language: Language,
    pub theme: ThemePreference,
    pub reduced_motion: bool,
    pub settings_section: Option<String>,
    /// Interface text size in px. Icons, spacing, and fixed workflow geometry stay unscaled.
    #[serde(default = "default_ui_font_size")]
    pub ui_font_size_px: u8,
    #[serde(default)]
    pub engine: EngineKind,
    #[serde(default)]
    pub policy_profile: PolicyProfile,
    #[serde(default)]
    pub auth_provider: Option<AuthProvider>,
}
fn default_ui_font_size() -> u8 {
    14
}
pub fn commit_ui_font_size(draft: &str, current: u8) -> u8 {
    let current = current.clamp(12, 20);
    let Some(parsed) = draft.trim().parse::<f32>().ok() else {
        return current;
    };
    if !parsed.is_finite() {
        return current;
    }
    parsed.round().clamp(12., 20.) as u8
}
pub fn ui_text_px(base: f32, font_size: u8) -> f32 {
    // Pi Desktop font tokens retain their pixel offset from the interface size.
    base + f32::from(font_size.clamp(12, 20)) - 14.
}
impl Default for Preferences {
    fn default() -> Self {
        Self {
            api_base: None,
            allow_environment_credentials: true,
            model: crate::MODELS[0].1.into(),
            language: Language::Chinese,
            theme: ThemePreference::default(),
            reduced_motion: false,
            settings_section: None,
            ui_font_size_px: default_ui_font_size(),
            engine: EngineKind::default(),
            policy_profile: PolicyProfile::default(),
            auth_provider: None,
        }
    }
}
impl Preferences {
    pub fn startup_api_key(&self, environment_key: Option<String>) -> String {
        if self.allow_environment_credentials {
            environment_key.unwrap_or_default()
        } else {
            String::new()
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Automation {
    pub id: String,
    pub name: String,
    pub prompt: String,
    pub minutes: u32,
    pub enabled: bool,
    pub next_run: u64,
}
impl Automation {
    pub fn validate(name: &str, prompt: &str, minutes: &str) -> Result<u32, &'static str> {
        if name.trim().is_empty() || prompt.trim().is_empty() {
            return Err("名称和任务不能为空 / Name and task are required");
        }
        let minutes = minutes
            .parse::<u32>()
            .map_err(|_| "请输入分钟数 / Enter minutes")?;
        if !(1..=10080).contains(&minutes) {
            return Err("间隔须为 1–10080 分钟 / Interval: 1–10080 minutes");
        }
        Ok(minutes)
    }
}
#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SavedDesktop {
    pub preferences: Preferences,
    pub sessions: HashMap<String, Session>,
    pub task_states: HashMap<String, crate::task_state::TaskState>,
    pub active_id: String,
    pub drafts: HashMap<String, String>,
    pub expanded_tools: HashSet<String>,
    pub automations: Vec<Automation>,
    pub agentconfig_configurations: Vec<crate::agentconfig_sources::Configuration>,
}
impl SavedDesktop {
    pub fn skip_missed_automations(&mut self, now: u64) {
        for job in &mut self.automations {
            if job.enabled && job.next_run <= now {
                job.next_run = now + u64::from(job.minutes.max(1)) * 60;
            }
        }
    }
}
pub struct Store {
    path: PathBuf,
    observed_revision: Cell<Option<u64>>,
}

#[derive(Deserialize)]
struct StateRevision {
    #[serde(default)]
    store_revision: u64,
}

#[derive(Serialize)]
struct VersionedState<'a> {
    store_revision: u64,
    #[serde(flatten)]
    desktop: &'a SavedDesktop,
}

struct StateLock(fs::File);

impl Drop for StateLock {
    fn drop(&mut self) {
        // A spawning child can briefly inherit the descriptor; close alone can delay release.
        let _ = self.0.unlock();
    }
}
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
pub fn new_id() -> String {
    format!(
        "task-{:x}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    )
}
impl Store {
    pub fn for_workspace(workspace: &Path) -> Self {
        let root = std::env::var_os("APP_STATE_DIR")
            .or_else(|| std::env::var_os("VOLT_STATE_DIR"))
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                PathBuf::from(
                    std::env::var_os("HOME")
                        .or_else(|| std::env::var_os("USERPROFILE"))
                        .unwrap_or_default(),
                )
                .join(".local/share/volt-desktop")
            });
        // Fixed FNV-1a isolates workspaces without storing chat history inside Git repositories.
        let hash = workspace
            .to_string_lossy()
            .bytes()
            .fold(0xcbf29ce484222325u64, |hash, byte| {
                (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
            });
        Self {
            path: root.join(format!("{hash:016x}/desktop.json")),
            observed_revision: Cell::new(None),
        }
    }
    pub fn load(&self) -> anyhow::Result<SavedDesktop> {
        match fs::read(&self.path) {
            Ok(bytes) => {
                let saved = serde_json::from_slice(&bytes)?;
                let revision: StateRevision = serde_json::from_slice(&bytes)?;
                self.observed_revision.set(Some(revision.store_revision));
                Ok(saved)
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                self.observed_revision.set(None);
                Ok(SavedDesktop::default())
            }
            Err(error) => Err(error.into()),
        }
    }
    pub fn save(&self, state: &SavedDesktop) -> anyhow::Result<()> {
        let parent = self.path.parent().expect("state parent");
        fs::create_dir_all(parent)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(parent, fs::Permissions::from_mode(0o700))?;
        }
        let _lock = self.lock_state()?;
        let disk_revision = self.disk_revision()?;
        if disk_revision != self.observed_revision.get() {
            anyhow::bail!("本地状态已被其他实例更新，请重新打开 / State changed in another instance; reopen before saving");
        }
        let revision = disk_revision
            .unwrap_or(0)
            .checked_add(1)
            .context("Desktop state revision exhausted")?;
        let encoded = serde_json::to_vec_pretty(&VersionedState {
            store_revision: revision,
            desktop: state,
        })?;
        let temp = parent.join(format!("{}.tmp", new_id()));
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options.open(&temp)?;
        let written = (|| -> anyhow::Result<()> {
            file.write_all(&encoded)?;
            file.sync_all()?;
            drop(file);
            fs::rename(&temp, &self.path)?;
            Ok(())
        })();
        if written.is_err() {
            let _ = fs::remove_file(temp);
        } else {
            self.observed_revision.set(Some(revision));
        }
        written
    }

    fn lock_state(&self) -> anyhow::Result<StateLock> {
        let mut options = OpenOptions::new();
        options.read(true).write(true).create(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        // The lock file remains stable while desktop.json is atomically replaced.
        let lock = options.open(self.path.with_extension("lock"))?;
        lock.try_lock()
            .context("另一个实例正在保存 / Another instance is saving")?;
        Ok(StateLock(lock))
    }

    fn disk_revision(&self) -> anyhow::Result<Option<u64>> {
        match fs::read(&self.path) {
            Ok(bytes) => {
                let revision: StateRevision = serde_json::from_slice(&bytes)?;
                Ok(Some(revision.store_revision))
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error.into()),
        }
    }
}
pub fn validate_endpoint(endpoint: &str) -> bool {
    reqwest::Url::parse(endpoint).is_ok_and(|url| {
        matches!(url.scheme(), "https" | "http")
            && url.host_str().is_some()
            && url.username().is_empty()
            && url.password().is_none()
            && url.query().is_none()
            && url.fragment().is_none()
    })
}
#[cfg(test)]
mod tests {
    #[test]
    fn agentconfig_credential_isolation_survives_serialization() {
        let preferences = super::Preferences {
            api_base: Some("https://imported.example/v1".into()),
            allow_environment_credentials: false,
            ..Default::default()
        };
        let restored: super::Preferences =
            serde_json::from_str(&serde_json::to_string(&preferences).unwrap()).unwrap();
        assert!(restored
            .startup_api_key(Some("synthetic-env-key".into()))
            .is_empty());
        assert_eq!(
            super::Preferences::default().startup_api_key(Some("synthetic-env-key".into())),
            "synthetic-env-key"
        );
    }

    #[test]
    fn interface_text_tokens_keep_the_reference_pixel_offsets() {
        for size in 12..=20 {
            for base in [9., 10., 11., 12., 13., 14., 16., 18., 24., 30.] {
                assert_eq!(super::ui_text_px(base, size) - f32::from(size), base - 14.);
                assert_eq!(super::ui_text_px(base, 14), base);
            }
        }
    }

    #[test]
    fn interface_text_tokens_clamp_the_preference_not_the_offset() {
        for base in [9., 10., 12., 13., 14., 24., 30.] {
            assert_eq!(super::ui_text_px(base, 0), base - 2.);
            assert_eq!(super::ui_text_px(base, 255), base + 6.);
        }
    }

    #[test]
    fn theme_defaults_to_light_and_preserves_saved_choices() {
        use super::{Preferences, SavedDesktop, ThemePreference};
        assert_eq!(Preferences::default().theme, ThemePreference::Light);
        assert_eq!(Preferences::default().ui_font_size_px, 14);
        assert_eq!(
            Preferences::default().engine,
            pi_agent_core::EngineKind::default()
        );
        assert_eq!(
            Preferences::default().policy_profile,
            pi_agent_core::PolicyProfile::default()
        );
        for json in ["{}", r#"{"preferences":{}}"#] {
            let saved: SavedDesktop = serde_json::from_str(json).unwrap();
            assert_eq!(saved.preferences.theme, ThemePreference::Light);
            assert_eq!(saved.preferences.ui_font_size_px, 14);
        }
        assert_eq!(super::commit_ui_font_size("", 16), 16);
        assert_eq!(super::commit_ui_font_size("11", 14), 12);
        assert_eq!(super::commit_ui_font_size("20.4", 14), 20);
        assert_eq!(super::commit_ui_font_size("99", 14), 20);
        assert_eq!(super::ui_text_px(14., 14), 14.);
        assert_eq!(super::ui_text_px(14., 20), 20.);
        for theme in [
            ThemePreference::Light,
            ThemePreference::Dark,
            ThemePreference::Auto,
        ] {
            let mut saved = SavedDesktop::default();
            saved.preferences.theme = theme;
            let restored: SavedDesktop =
                serde_json::from_str(&serde_json::to_string(&saved).unwrap()).unwrap();
            assert_eq!(restored.preferences.theme, theme);
        }
    }
    use super::*;
    #[test]
    fn round_trip_history_preferences_drafts() {
        let root = std::env::temp_dir().join(format!("history-roundtrip-{}", new_id()));
        let store = Store {
            path: root.join("desktop.json"),
            observed_revision: Cell::new(None),
        };
        let mut state = SavedDesktop::default();
        let mut session = Session::new("a", "中文会话");
        session
            .messages
            .push(pi_agent_core::Message::user("draft survives"));
        state.sessions.insert("a".into(), session);
        state.active_id = "a".into();
        state.drafts.insert("a".into(), "未发送".into());
        store.save(&state).unwrap();
        store.save(&state).unwrap();
        let restored = store.load().unwrap();
        assert_eq!(restored.sessions["a"].messages[0].content, "draft survives");
        assert_eq!(restored.drafts["a"], "未发送");
        assert!(!fs::read_to_string(&store.path).unwrap().contains("api_key"));
        fs::write(&store.path, "broken").unwrap();
        assert!(store.load().is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn agentconfig_sources_survive_restart_without_decrypted_credentials() {
        let model = pi_agent_core::ModelConfig {
            provider: "example".into(),
            model: "test-model".into(),
            api_base: Some("https://example.com/v1".into()),
            api_key: Some("synthetic-test-key".into()),
            ..Default::default()
        };
        let (bundle, _) = pi_agent_core::agent_config::export_bundle(
            &model,
            &[],
            pi_agent_core::agent_config::ExportOptions {
                trust: pi_agent_core::agent_config::TrustMode::Self_,
                password: Some("synthetic-test-password".into()),
                include_credentials: true,
                label: None,
            },
        )
        .unwrap();
        let root = std::env::temp_dir().join(format!("agentconfig-roundtrip-{}", new_id()));
        let store = Store {
            path: root.join("desktop.json"),
            observed_revision: Cell::new(None),
        };
        let mut state = SavedDesktop::default();
        state
            .agentconfig_configurations
            .push(crate::agentconfig_sources::Configuration {
                id: "source".into(),
                source_url: Some("https://example.com/team.acfg".into()),
                bundle: bundle.clone(),
                imported_at: 1,
                checked_at: Some(2),
            });
        store.save(&state).unwrap();
        let disk = fs::read_to_string(&store.path).unwrap();
        assert!(!disk.contains("synthetic-test-key"));
        assert!(!disk.contains("synthetic-test-password"));
        let restored = store.load().unwrap();
        assert_eq!(restored.agentconfig_configurations[0].bundle, bundle);
        assert_eq!(restored.agentconfig_configurations[0].checked_at, Some(2));
        assert!(SavedDesktop::default()
            .agentconfig_configurations
            .is_empty());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn agentconfig_state_rejects_stale_instances_and_concurrent_writers() {
        let root = std::env::temp_dir().join(format!("agentconfig-conflict-{}", new_id()));
        let first = Store {
            path: root.join("desktop.json"),
            observed_revision: Cell::new(None),
        };
        let second = Store {
            path: first.path.clone(),
            observed_revision: Cell::new(None),
        };
        let mut first_state = first.load().unwrap();
        let mut second_state = second.load().unwrap();
        first_state.preferences.model = "first".into();
        first.save(&first_state).unwrap();
        second_state.preferences.model = "stale".into();
        assert!(second.save(&second_state).is_err());
        assert_eq!(first.load().unwrap().preferences.model, "first");
        second_state = second.load().unwrap();
        second_state.preferences.model = "second".into();
        let lock = first.lock_state().unwrap();
        assert!(second.save(&second_state).is_err());
        drop(lock);
        second.save(&second_state).unwrap();
        assert!(first.save(&first_state).is_err());
        assert_eq!(second.load().unwrap().preferences.model, "second");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn agentconfig_state_upgrades_legacy_revision_without_overwriting_unreadable_data() {
        let root = std::env::temp_dir().join(format!("agentconfig-legacy-{}", new_id()));
        fs::create_dir_all(&root).unwrap();
        let store = Store {
            path: root.join("desktop.json"),
            observed_revision: Cell::new(None),
        };
        fs::write(&store.path, r#"{"preferences":{"model":"legacy"}}"#).unwrap();
        let saved = store.load().unwrap();
        assert_eq!(saved.preferences.model, "legacy");
        store.save(&saved).unwrap();
        let revision: StateRevision =
            serde_json::from_slice(&fs::read(&store.path).unwrap()).unwrap();
        assert_eq!(revision.store_revision, 1);
        fs::write(&store.path, "broken").unwrap();
        assert!(store.save(&saved).is_err());
        assert_eq!(fs::read_to_string(&store.path).unwrap(), "broken");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn agentconfig_repeated_saves_release_locks_while_children_spawn() {
        let root = std::env::temp_dir().join(format!("save-stress-{}", new_id()));
        let store = Store {
            path: root.join("desktop.json"),
            observed_revision: Cell::new(None),
        };
        let saved = store.load().unwrap();
        let children = std::thread::spawn(|| {
            for _ in 0..25 {
                assert!(std::process::Command::new(std::env::current_exe().unwrap())
                    .arg("--list")
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
                    .unwrap()
                    .success());
            }
        });
        for _ in 0..100 {
            store.save(&saved).unwrap();
        }
        children.join().unwrap();
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn endpoints_reject_credentials_and_non_http() {
        for value in [
            "file:///tmp",
            "https://user:secret@example.com",
            "https://a?key=x",
            "broken",
        ] {
            assert!(!validate_endpoint(value));
        }
        assert!(validate_endpoint("http://localhost:11434/v1"));
    }
    #[test]
    fn reopening_skips_missed_runs_and_preserves_future_runs() {
        let mut state = SavedDesktop::default();
        for (id, next_run) in [("past", 50), ("future", 500)] {
            state.automations.push(Automation {
                id: id.into(),
                name: id.into(),
                prompt: "test".into(),
                minutes: 1,
                enabled: true,
                next_run,
            });
        }
        state.skip_missed_automations(100);
        assert_eq!(state.automations[0].next_run, 160);
        assert_eq!(state.automations[1].next_run, 500);
    }

    #[test]
    fn automation_validation() {
        assert!(Automation::validate("", "hello", "60").is_err());
        assert!(Automation::validate("Test", "hello", "0").is_err());
        assert_eq!(Automation::validate("Test", "hello", "60").unwrap(), 60);
    }
}
