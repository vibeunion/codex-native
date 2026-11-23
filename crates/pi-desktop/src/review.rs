use anyhow::{bail, Context, Result};
use std::{path::Path, process::Stdio, time::Duration};
use tokio::{
    io::{AsyncRead, AsyncReadExt},
    process::Command,
};

const PREVIEW_LIMIT: usize = 1_048_576;
const GIT_LIST_LIMIT: usize = 16 * PREVIEW_LIMIT;
const GIT_STDERR_LIMIT: usize = 64 * 1024;
const BINARY_PREVIEW: &str = "Binary file; text preview unavailable.";

#[cfg(test)]
fn git_timeout() -> Duration {
    // The unit-test harness runs hundreds of tests concurrently, so process
    // startup can be delayed by the harness without reflecting app behavior.
    Duration::from_secs(60)
}

#[cfg(not(test))]
fn git_timeout() -> Duration {
    Duration::from_secs(15)
}

struct CapturedOutput {
    bytes: Vec<u8>,
    truncated: bool,
    contains_nul: bool,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ReviewMode {
    #[default]
    Unstaged,
    Staged,
    Untracked,
}
#[derive(Default)]
pub struct ReviewState {
    pub mode: ReviewMode,
    pub files: Vec<String>,
    pub selected: Option<String>,
    pub diff: String,
    pub loading: bool,
    pub error: Option<String>,
    pub generation: u64,
}
pub struct ReviewSnapshot {
    pub files: Vec<String>,
    pub selected: Option<String>,
    pub diff: String,
}

pub fn workspace_relative_path(workspace: &Path, file: &str) -> Option<String> {
    // Git may emit either separator on Windows, while rooted paths without a
    // drive letter are not considered absolute by `std::path::Path` there.
    // Normalize lexically before validating containment so the same contract
    // applies on every platform.
    let normalize = |value: &str| value.replace('\\', "/");
    let workspace = normalize(&workspace.to_string_lossy());
    let workspace = workspace.trim_end_matches('/');
    let file = normalize(file);
    let file = if file.starts_with('/') {
        if workspace.is_empty() {
            file.strip_prefix('/').unwrap_or(&file)
        } else if file == workspace {
            ""
        } else {
            file.strip_prefix(&format!("{workspace}/"))?
        }
    } else {
        file.as_str()
    };
    let mut parts = Vec::new();
    for component in file.split('/') {
        match component {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            part => parts.push(part),
        }
    }
    (!parts.is_empty()).then(|| parts.join("/"))
}

async fn git(workspace: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let output = git_output(workspace, args, GIT_LIST_LIMIT).await?;
    if output.truncated {
        bail!("Git output exceeds 16 MiB");
    }
    Ok(output.bytes)
}

async fn capture_prefix(
    mut reader: impl AsyncRead + Unpin,
    limit: usize,
) -> std::io::Result<CapturedOutput> {
    let mut bytes = Vec::with_capacity(limit.min(8192));
    let mut truncated = false;
    let mut contains_nul = false;
    let mut buffer = [0; 8192];
    loop {
        let count = reader.read(&mut buffer).await?;
        if count == 0 {
            break;
        }
        let retained = count.min(limit - bytes.len());
        bytes.extend_from_slice(&buffer[..retained]);
        truncated |= retained < count;
        contains_nul |= buffer[..count].contains(&0);
    }
    Ok(CapturedOutput {
        bytes,
        truncated,
        contains_nul,
    })
}

async fn git_output(
    workspace: &Path,
    args: &[&str],
    stdout_limit: usize,
) -> Result<CapturedOutput> {
    tokio::time::timeout(git_timeout(), async {
        let mut child = Command::new("git")
            .args(["--no-pager", "-c", "color.ui=false"])
            .args(args)
            .current_dir(workspace)
            .env("GIT_OPTIONAL_LOCKS", "0")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .context("Git could not be started")?;
        let stdout = child.stdout.take().context("Git stdout unavailable")?;
        let stderr = child.stderr.take().context("Git stderr unavailable")?;
        let (stdout, stderr, status) = tokio::try_join!(
            capture_prefix(stdout, stdout_limit),
            capture_prefix(stderr, GIT_STDERR_LIMIT),
            child.wait(),
        )
        .context("Git process output unavailable")?;
        if !status.success() {
            let mut diagnostic = String::from_utf8_lossy(&stderr.bytes).into_owned();
            if stderr.truncated {
                diagnostic.push_str("\n[Git diagnostics truncated at 64 KiB]");
            }
            bail!("Git exited with {status}: {}", diagnostic.trim());
        }
        Ok(stdout)
    })
    .await
    .context("Git timed out; refresh to retry")?
}
pub async fn snapshot(
    workspace: &Path,
    mode: ReviewMode,
    selected: Option<String>,
) -> Result<ReviewSnapshot> {
    let args = match mode {
        ReviewMode::Unstaged => vec![
            "diff",
            "--no-ext-diff",
            "--no-textconv",
            "--name-only",
            "-z",
        ],
        ReviewMode::Staged => vec![
            "diff",
            "--cached",
            "--no-ext-diff",
            "--no-textconv",
            "--name-only",
            "-z",
        ],
        ReviewMode::Untracked => vec!["ls-files", "--others", "--exclude-standard", "-z"],
    };
    let bytes = git(workspace, &args).await?;
    let files: Vec<_> = bytes
        .split(|byte| *byte == 0)
        .filter(|name| !name.is_empty())
        .map(|name| String::from_utf8_lossy(name).into_owned())
        .collect();
    let selected = match selected {
        Some(path) if files.contains(&path) => Some(path),
        Some(_) => None,
        None => files.first().cloned(),
    };
    let diff = if let Some(path) = &selected {
        if mode == ReviewMode::Untracked {
            let full = workspace.join(path);
            match untracked_preview(workspace, &full).await {
                Ok(text) => text,
                Err(error) => format!("Preview unavailable: {error}"),
            }
        } else {
            let mut args = vec!["diff", "--no-ext-diff", "--no-textconv", "--no-color"];
            if mode == ReviewMode::Staged {
                args.push("--cached");
            }
            args.extend(["--", path]);
            let output = git_output(workspace, &args, PREVIEW_LIMIT + 1).await?;
            if output.contains_nul {
                BINARY_PREVIEW.into()
            } else {
                preview(output.bytes)
            }
        }
    } else {
        String::new()
    };
    Ok(ReviewSnapshot {
        files,
        selected,
        diff,
    })
}
async fn untracked_preview(workspace: &Path, full: &Path) -> Result<String> {
    let canonical = tokio::fs::canonicalize(full)
        .await
        .context("file cannot be read")?;
    let root = tokio::fs::canonicalize(workspace)
        .await
        .context("workspace cannot be read")?;
    if !canonical.starts_with(root) {
        bail!("file resolves outside this workspace");
    }
    let metadata = tokio::fs::metadata(&canonical)
        .await
        .context("file metadata unavailable")?;
    if !metadata.is_file() || metadata.len() > PREVIEW_LIMIT as u64 {
        bail!("not a regular file or larger than 1 MiB");
    }
    let file = tokio::fs::File::open(canonical)
        .await
        .context("file cannot be read")?;
    read_file_preview(file).await
}

async fn read_file_preview(file: tokio::fs::File) -> Result<String> {
    let mut bytes = Vec::new();
    file.take(PREVIEW_LIMIT as u64 + 1)
        .read_to_end(&mut bytes)
        .await
        .context("file cannot be read")?;
    Ok(preview(bytes))
}

fn preview(mut bytes: Vec<u8>) -> String {
    if bytes.contains(&0) {
        return BINARY_PREVIEW.into();
    }
    let truncated = bytes.len() > PREVIEW_LIMIT;
    bytes.truncate(PREVIEW_LIMIT);
    let mut text = String::from_utf8_lossy(&bytes).into_owned();
    if truncated {
        text.push_str("\n[Preview truncated at 1 MiB]");
    }
    text
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn oversized_git_output_is_rejected_instead_of_retained() {
        let directory = tempfile::tempdir().unwrap();
        let workspace = directory.path();
        git(workspace, &["init", "-q"]).await.unwrap();
        tokio::fs::write(
            workspace.join("oversized.txt"),
            vec![b'x'; 16 * 1_048_576 + 1],
        )
        .await
        .unwrap();
        git(workspace, &["add", "--", "oversized.txt"])
            .await
            .unwrap();
        match git(workspace, &["show", ":oversized.txt"]).await {
            Ok(bytes) => panic!(
                "retained {} bytes instead of rejecting the output",
                bytes.len()
            ),
            Err(error) => assert!(
                error.to_string().contains("Git output exceeds 16 MiB"),
                "{error:#}"
            ),
        }
    }

    #[tokio::test]
    async fn large_git_diff_drains_output_and_keeps_only_the_preview_prefix() {
        let directory = tempfile::tempdir().unwrap();
        let workspace = directory.path();
        git(workspace, &["init", "-q"]).await.unwrap();
        let path = workspace.join("large.txt");
        tokio::fs::write(&path, vec![b'a'; PREVIEW_LIMIT + 4096])
            .await
            .unwrap();
        git(workspace, &["add", "--", "large.txt"]).await.unwrap();
        tokio::fs::write(&path, vec![b'b'; PREVIEW_LIMIT + 4096])
            .await
            .unwrap();
        let output = git_output(
            workspace,
            &["diff", "--no-ext-diff", "--no-textconv", "--", "large.txt"],
            PREVIEW_LIMIT + 1,
        )
        .await
        .unwrap();
        assert_eq!(output.bytes.len(), PREVIEW_LIMIT + 1);
        assert!(output.truncated);
        let snapshot = snapshot(workspace, ReviewMode::Unstaged, None)
            .await
            .unwrap();
        assert_eq!(snapshot.files, vec!["large.txt"]);
        assert_eq!(snapshot.selected.as_deref(), Some("large.txt"));
        assert!(snapshot.diff.ends_with("[Preview truncated at 1 MiB]"));
        assert!(snapshot.diff.len() < PREVIEW_LIMIT + 64);
    }

    #[tokio::test]
    async fn late_nul_in_git_output_retains_the_binary_preview_classification() {
        let directory = tempfile::tempdir().unwrap();
        let workspace = directory.path();
        git(workspace, &["init", "-q"]).await.unwrap();
        let path = workspace.join("late-binary.dat");
        tokio::fs::write(&path, b"before\n").await.unwrap();
        git(workspace, &["add", "--", "late-binary.dat"])
            .await
            .unwrap();
        let mut contents = vec![b'x'; PREVIEW_LIMIT + 4096];
        contents.push(0);
        contents.extend_from_slice(b"tail\n");
        tokio::fs::write(&path, contents).await.unwrap();
        let snapshot = snapshot(workspace, ReviewMode::Unstaged, None)
            .await
            .unwrap();
        assert!(
            snapshot.diff == "Binary file; text preview unavailable.",
            "late NUL was omitted from classification (preview has {} bytes)",
            snapshot.diff.len()
        );
    }

    #[tokio::test]
    async fn file_growth_after_metadata_check_cannot_expand_the_preview_read() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("growing.txt");
        tokio::fs::write(&path, b"small").await.unwrap();
        let file = tokio::fs::File::open(&path).await.unwrap();
        assert!(file.metadata().await.unwrap().len() < PREVIEW_LIMIT as u64);
        tokio::fs::write(&path, vec![b'x'; PREVIEW_LIMIT + 4096])
            .await
            .unwrap();
        let text = read_file_preview(file).await.unwrap();
        assert!(text.starts_with("xxxx"));
        assert!(text.ends_with("[Preview truncated at 1 MiB]"));
        assert!(text.len() < PREVIEW_LIMIT + 64);
    }

    #[tokio::test]
    async fn reads_staged_unstaged_and_preserves_list_on_preview_failure() {
        let directory = tempfile::tempdir().unwrap();
        let workspace = directory.path();
        git(workspace, &["init", "-q"]).await.unwrap();
        tokio::fs::write(workspace.join("file with space.txt"), "before\n")
            .await
            .unwrap();
        git(workspace, &["add", "--", "file with space.txt"])
            .await
            .unwrap();
        git(
            workspace,
            &[
                "-c",
                "user.name=Test",
                "-c",
                "user.email=test@example.invalid",
                "commit",
                "-qm",
                "fixture",
            ],
        )
        .await
        .unwrap();
        tokio::fs::write(workspace.join("file with space.txt"), "after\n")
            .await
            .unwrap();
        let unstaged = snapshot(workspace, ReviewMode::Unstaged, None)
            .await
            .unwrap();
        assert_eq!(unstaged.files, vec!["file with space.txt"]);
        assert!(unstaged.diff.contains("+after"));
        assert!(snapshot(workspace, ReviewMode::Staged, None)
            .await
            .unwrap()
            .files
            .is_empty());
        git(workspace, &["add", "--", "file with space.txt"])
            .await
            .unwrap();
        assert!(snapshot(workspace, ReviewMode::Staged, None)
            .await
            .unwrap()
            .diff
            .contains("+after"));
        tokio::fs::write(workspace.join("readable.txt"), "readable")
            .await
            .unwrap();
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(workspace.join("missing"), workspace.join("broken"))
                .unwrap();
            let broken = snapshot(workspace, ReviewMode::Untracked, Some("broken".into()))
                .await
                .unwrap();
            assert!(broken.files.contains(&"readable.txt".to_owned()));
            assert!(broken.diff.contains("Preview unavailable"));
        }
        let readable = snapshot(
            workspace,
            ReviewMode::Untracked,
            Some("readable.txt".into()),
        )
        .await
        .unwrap();
        assert_eq!(readable.diff, "readable");
    }
    #[tokio::test]
    async fn non_repository_is_an_error_not_a_clean_tree() {
        let directory = tempfile::tempdir().unwrap();
        assert!(snapshot(directory.path(), ReviewMode::Unstaged, None)
            .await
            .is_err());
    }

    #[tokio::test]
    async fn explicit_missing_selection_does_not_fall_back_to_another_file() {
        let directory = tempfile::tempdir().unwrap();
        git(directory.path(), &["init", "-q"]).await.unwrap();
        git(
            directory.path(),
            &[
                "-c",
                "user.name=Test",
                "-c",
                "user.email=test@example.invalid",
                "commit",
                "--allow-empty",
                "-qm",
                "fixture",
            ],
        )
        .await
        .unwrap();
        tokio::fs::write(directory.path().join("actual.txt"), "changed\n")
            .await
            .unwrap();
        git(directory.path(), &["add", "actual.txt"]).await.unwrap();
        let snapshot = snapshot(
            directory.path(),
            ReviewMode::Staged,
            Some("missing.txt".into()),
        )
        .await
        .unwrap();
        assert_eq!(snapshot.files, vec!["actual.txt"]);
        assert!(snapshot.selected.is_none());
        assert!(snapshot.diff.is_empty());
    }

    #[test]
    fn file_targets_are_repository_relative_and_cannot_escape_the_workspace() {
        let workspace = Path::new("/workspace");
        for file in [
            "src/file.rs",
            "./src/file.rs",
            "/workspace/src/file.rs",
            "src/../src/file.rs",
        ] {
            assert_eq!(
                workspace_relative_path(workspace, file).as_deref(),
                Some("src/file.rs")
            );
        }
        for file in [
            "",
            ".",
            "../file.rs",
            "/elsewhere/file.rs",
            "src/../../file.rs",
        ] {
            assert!(workspace_relative_path(workspace, file).is_none());
        }
    }
    #[test]
    fn binary_and_large_previews_are_explicit() {
        assert!(preview(vec![0]).contains("Binary"));
        assert!(preview(vec![b'a'; 1_048_577]).ends_with("[Preview truncated at 1 MiB]"));
    }
}
