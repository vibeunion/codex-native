use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use serde_json::Value;
use std::path::PathBuf;

use super::{Tool, WorkspacePath};

pub struct ReadFileTool {
    workspace: WorkspacePath,
}

impl ReadFileTool {
    pub fn new(workspace_dir: PathBuf) -> Self {
        Self {
            workspace: WorkspacePath::new(workspace_dir),
        }
    }
}

#[async_trait]
impl Tool for ReadFileTool {
    fn name(&self) -> &'static str {
        "read_file"
    }

    fn description(&self) -> &'static str {
        "Read contents of a file with optional line range (start_line, end_line, 1-indexed)."
    }

    fn parameters_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to read"
                },
                "start_line": {
                    "type": "integer",
                    "description": "Optional start line number (1-indexed, inclusive)"
                },
                "end_line": {
                    "type": "integer",
                    "description": "Optional end line number (1-indexed, inclusive)"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, arguments: Value) -> Result<String> {
        let path_str = arguments["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'path'"))?;
        let full_path = self.workspace.existing(path_str)?;

        let content = tokio::fs::read_to_string(&full_path)
            .await
            .with_context(|| format!("Failed to read file: {:?}", full_path))?;

        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        let start = line_parameter(&arguments, "start_line")?.unwrap_or(1);
        let requested_end = line_parameter(&arguments, "end_line")?;
        if requested_end.is_some_and(|end| end < start) {
            bail!("end_line must be greater than or equal to start_line");
        }
        let end = requested_end.unwrap_or(total_lines).min(total_lines);
        if start > total_lines {
            return Ok(format!(
                "File has only {} lines (requested start at line {})",
                total_lines, start
            ));
        }
        let slice = &lines[(start - 1)..end];
        let mut out = String::new();
        for (i, line) in slice.iter().enumerate() {
            out.push_str(&format!("{:4} | {}\n", start + i, line));
        }
        Ok(out)
    }
}

fn line_parameter(arguments: &Value, name: &str) -> Result<Option<usize>> {
    arguments
        .get(name)
        .map(|input| {
            let number = input
                .as_u64()
                .filter(|number| *number > 0)
                .with_context(|| format!("{name} must be a positive integer"))?;
            usize::try_from(number).with_context(|| format!("{name} is too large"))
        })
        .transpose()
}

pub struct WriteFileTool {
    workspace: WorkspacePath,
}

impl WriteFileTool {
    pub fn new(workspace_dir: PathBuf) -> Self {
        Self {
            workspace: WorkspacePath::new(workspace_dir),
        }
    }
}

#[async_trait]
impl Tool for WriteFileTool {
    fn name(&self) -> &'static str {
        "write_file"
    }

    fn description(&self) -> &'static str {
        "Write full content to a file (creates directories if they do not exist)."
    }

    fn parameters_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to create or overwrite"
                },
                "content": {
                    "type": "string",
                    "description": "Content to write into the file"
                }
            },
            "required": ["path", "content"]
        })
    }

    async fn execute(&self, arguments: Value) -> Result<String> {
        let path_str = arguments["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'path'"))?;
        let content = arguments["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'content'"))?;

        let full_path = self.workspace.for_write(path_str)?;
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        tokio::fs::write(&full_path, content).await?;
        Ok(format!(
            "Successfully wrote {} bytes to {:?}",
            content.len(),
            full_path
        ))
    }
}

pub struct EditFileTool {
    workspace: WorkspacePath,
}

impl EditFileTool {
    pub fn new(workspace_dir: PathBuf) -> Self {
        Self {
            workspace: WorkspacePath::new(workspace_dir),
        }
    }
}

#[async_trait]
impl Tool for EditFileTool {
    fn name(&self) -> &'static str {
        "edit_file"
    }

    fn description(&self) -> &'static str {
        "Edit a file by precisely replacing an existing target string with new content."
    }

    fn parameters_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to edit"
                },
                "target_content": {
                    "type": "string",
                    "description": "The exact string within the file to replace"
                },
                "replacement_content": {
                    "type": "string",
                    "description": "The replacement string"
                }
            },
            "required": ["path", "target_content", "replacement_content"]
        })
    }

    async fn execute(&self, arguments: Value) -> Result<String> {
        let path_str = arguments["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'path'"))?;
        let target = arguments["target_content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'target_content'"))?;
        let replacement = arguments["replacement_content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'replacement_content'"))?;

        let full_path = self.workspace.existing(path_str)?;
        let content = tokio::fs::read_to_string(&full_path).await?;

        let count = content.matches(target).count();
        if count == 0 {
            bail!("target_content not found in {:?}", full_path);
        }
        if count > 1 {
            bail!(
                "target_content found {} times in {:?}; must be unique",
                count,
                full_path
            );
        }

        let new_content = content.replacen(target, replacement, 1);
        tokio::fs::write(&full_path, new_content).await?;
        Ok(format!(
            "Successfully replaced 1 occurrence in {:?}",
            full_path
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{ReadFileTool, Tool};
    use serde_json::json;

    #[tokio::test]
    async fn rejects_reversed_line_ranges_without_panicking() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("sample.txt");
        std::fs::write(&path, "one\ntwo\nthree\n").unwrap();
        let tool = ReadFileTool::new(root.path().to_owned());

        let result = tool
            .execute(json!({
                "path": "sample.txt",
                "start_line": 3,
                "end_line": 2
            }))
            .await;

        assert!(result.is_err());
    }
}
