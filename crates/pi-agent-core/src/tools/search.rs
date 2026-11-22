use anyhow::{Context, Result};
use async_trait::async_trait;
use regex::Regex;
use serde_json::Value;
use std::path::PathBuf;
use tokio::fs::{self, ReadDir};

use super::{Tool, WorkspacePath};

fn normalized_relative_path(path: &std::path::Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn search_result_limit(arguments: &Value) -> Result<usize> {
    let Some(input) = arguments.get("max_results") else {
        return Ok(100);
    };
    let limit = input
        .as_u64()
        .filter(|limit| (1..=10_000).contains(limit))
        .ok_or_else(|| anyhow::anyhow!("max_results must be an integer between 1 and 10000"))?;
    Ok(limit as usize)
}

/// Depth-first traversal whose filesystem operations and directory boundaries
/// yield to the runtime. Dropping a search drops this traversal as well.
struct SearchWalk {
    root: Option<PathBuf>,
    pending_dir: Option<PathBuf>,
    directories: Vec<ReadDir>,
}
impl SearchWalk {
    fn new(root: PathBuf) -> Self {
        Self {
            root: Some(root),
            pending_dir: None,
            directories: Vec::new(),
        }
    }

    async fn next(&mut self) -> Result<Option<(PathBuf, bool)>> {
        if let Some(root) = self.root.take() {
            let metadata = fs::metadata(&root)
                .await
                .with_context(|| format!("Failed to inspect search path: {}", root.display()))?;
            if metadata.is_dir() {
                self.pending_dir = Some(root.clone());
            }
            return Ok(Some((root, metadata.is_file())));
        }
        loop {
            tokio::task::yield_now().await;
            if let Some(path) = self.pending_dir.take() {
                let directory = fs::read_dir(&path)
                    .await
                    .with_context(|| format!("Failed to search directory: {}", path.display()))?;
                self.directories.push(directory);
            }
            let Some(directory) = self.directories.last_mut() else {
                return Ok(None);
            };
            let entry = match directory
                .next_entry()
                .await
                .context("Failed to read a search directory entry")?
            {
                Some(entry) => entry,
                None => {
                    self.directories.pop();
                    continue;
                }
            };
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with('.') || name == "target" || name == "node_modules" {
                continue;
            }
            let path = entry.path();
            let kind = entry
                .file_type()
                .await
                .with_context(|| format!("Failed to inspect search entry: {}", path.display()))?;
            if kind.is_dir() {
                self.pending_dir = Some(path.clone());
            }
            return Ok(Some((path, kind.is_file())));
        }
    }
}

pub struct GrepTool {
    workspace: WorkspacePath,
}

impl GrepTool {
    pub fn new(workspace_dir: PathBuf) -> Self {
        Self {
            workspace: WorkspacePath::new(workspace_dir),
        }
    }
}

#[async_trait]
impl Tool for GrepTool {
    fn name(&self) -> &'static str {
        "grep"
    }

    fn description(&self) -> &'static str {
        "Search file contents for a regex pattern within the workspace. Recursive searches skip non-UTF-8 files; explicit file read errors and traversal errors are reported."
    }

    fn parameters_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "Regular expression pattern to search for"
                },
                "path": {
                    "type": "string",
                    "description": "Subdirectory or file path to search in (relative to workspace)",
                    "default": "."
                },
                "max_results": {
                    "type": "integer",
                    "description": "Maximum matching lines to return",
                    "default": 100
                }
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, arguments: Value) -> Result<String> {
        let pattern_str = arguments["pattern"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'pattern'"))?;
        let sub_path = match arguments.get("path") {
            Some(input) => input
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("path must be a string"))?,
            None => ".",
        };
        let max_results = search_result_limit(&arguments)?;
        let workspace_root = self.workspace.root()?;
        let root = self.workspace.existing(sub_path)?;
        let explicit_file = fs::metadata(&root)
            .await
            .with_context(|| format!("Failed to inspect search path: {}", root.display()))?
            .is_file();

        let re = Regex::new(pattern_str)?;
        let mut matches = Vec::new();

        let mut entries = SearchWalk::new(root);
        while let Some((path, is_file)) = entries.next().await? {
            if !is_file {
                continue;
            }

            let content = match fs::read_to_string(&path).await {
                Ok(content) => content,
                Err(error) if !explicit_file && error.kind() == std::io::ErrorKind::InvalidData => {
                    continue;
                }
                Err(error) => {
                    return Err(error).with_context(|| {
                        format!("Failed to read search file: {}", path.display())
                    });
                }
            };
            let rel = path.strip_prefix(&workspace_root).unwrap_or(&path);
            for (idx, line) in content.lines().enumerate() {
                if idx % 256 == 0 {
                    tokio::task::yield_now().await;
                }
                if re.is_match(line) {
                    matches.push(format!(
                        "{}:{}: {}",
                        normalized_relative_path(rel),
                        idx + 1,
                        line.trim()
                    ));
                    if matches.len() >= max_results {
                        break;
                    }
                }
            }
            if matches.len() >= max_results {
                break;
            }
        }

        if matches.is_empty() {
            Ok(format!("No matches found for regex: {:?}", pattern_str))
        } else {
            Ok(matches.join("\n"))
        }
    }
}

pub struct FindTool {
    workspace: WorkspacePath,
}

impl FindTool {
    pub fn new(workspace_dir: PathBuf) -> Self {
        Self {
            workspace: WorkspacePath::new(workspace_dir),
        }
    }
}

#[async_trait]
impl Tool for FindTool {
    fn name(&self) -> &'static str {
        "find"
    }

    fn description(&self) -> &'static str {
        "Find file and directory paths matching a substring or pattern in the workspace."
    }

    fn parameters_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "Substring to search for in file paths"
                },
                "max_results": {
                    "type": "integer",
                    "description": "Maximum number of results to return",
                    "default": 100
                }
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, arguments: Value) -> Result<String> {
        let pattern_str = arguments["pattern"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'pattern'"))?
            .to_lowercase();
        let max_results = search_result_limit(&arguments)?;

        let mut matches = Vec::new();
        let workspace_root = self.workspace.root()?;
        let mut entries = SearchWalk::new(workspace_root.clone());
        while let Some((path, _)) = entries.next().await? {
            let rel = path.strip_prefix(&workspace_root).unwrap_or(&path);
            let rel_str = normalized_relative_path(rel);
            if rel_str.to_lowercase().contains(&pattern_str) {
                matches.push(rel_str);
                if matches.len() >= max_results {
                    break;
                }
            }
        }

        if matches.is_empty() {
            Ok(format!("No files found matching {:?}", pattern_str))
        } else {
            Ok(matches.join("\n"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "current_thread")]
    async fn traversal_yields_and_can_be_cancelled_after_starting() {
        let root = tempfile::tempdir().unwrap();
        for index in 0..512 {
            std::fs::write(root.path().join(format!("file-{index}")), "evidence").unwrap();
        }
        let mut entries = SearchWalk::new(root.path().to_owned());
        let (started, waiting) = tokio::sync::oneshot::channel();
        let scan = tokio::spawn(async move {
            let mut started = Some(started);
            let mut count = 0;
            while entries.next().await.unwrap().is_some() {
                count += 1;
                if count == 64 {
                    let _ = started.take().unwrap().send(());
                }
            }
            count
        });
        waiting.await.unwrap();
        // On a single-thread runtime this requires the traversal to yield while
        // there is still work, not merely before or after the complete scan.
        assert!(!scan.is_finished());
        scan.abort();
        assert!(scan.await.unwrap_err().is_cancelled());
    }

    #[tokio::test]
    async fn traversal_preserves_exclusions_and_nested_matches() {
        let root = tempfile::tempdir().unwrap();
        for directory in ["src", ".hidden", "target", "node_modules"] {
            std::fs::create_dir(root.path().join(directory)).unwrap();
            std::fs::write(root.path().join(directory).join("hit.txt"), "needle").unwrap();
        }
        let find = FindTool::new(root.path().to_owned())
            .execute(serde_json::json!({"pattern":"hit"}))
            .await
            .unwrap();
        assert_eq!(find, "src/hit.txt");
        let grep = GrepTool::new(root.path().to_owned())
            .execute(serde_json::json!({"pattern":"needle"}))
            .await
            .unwrap();
        assert_eq!(grep, "src/hit.txt:1: needle");
    }

    #[tokio::test]
    async fn traversal_reports_missing_root_instead_of_an_empty_walk() {
        let root = tempfile::tempdir().unwrap();
        let mut entries = SearchWalk::new(root.path().join("missing"));
        let error = entries.next().await.unwrap_err();
        assert!(format!("{error:#}").contains("missing"));
    }

    #[tokio::test]
    async fn traversal_reports_directory_disappearing_before_descent() {
        let root = tempfile::tempdir().unwrap();
        let directory = root.path().join("removed");
        std::fs::create_dir(&directory).unwrap();
        let mut entries = SearchWalk::new(directory.clone());
        assert_eq!(
            entries.next().await.unwrap(),
            Some((directory.clone(), false))
        );
        std::fs::remove_dir(&directory).unwrap();
        let error = entries.next().await.unwrap_err();
        assert!(format!("{error:#}").contains("removed"));
    }
}
