use anyhow::{bail, Context, Result};
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) struct WorkspacePath {
    root: PathBuf,
}

impl WorkspacePath {
    pub(crate) fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub(crate) fn existing(&self, raw_path: &str) -> Result<PathBuf> {
        let root = self.root()?;
        let candidate = Self::candidate(&root, raw_path)?;
        let resolved = fs::canonicalize(&candidate)
            .with_context(|| format!("Path is unavailable: {}", candidate.display()))?;
        Self::ensure_inside(&root, resolved)
    }

    pub(crate) fn for_write(&self, raw_path: &str) -> Result<PathBuf> {
        let root = self.root()?;
        let candidate = Self::candidate(&root, raw_path)?;
        let mut probe = candidate.clone();
        let mut missing = Vec::<OsString>::new();

        loop {
            match fs::symlink_metadata(&probe) {
                Ok(metadata) => {
                    if metadata.file_type().is_symlink() {
                        bail!("Path contains a symlink: {}", probe.display());
                    }
                    break;
                }
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    let Some(name) = probe.file_name() else {
                        bail!("Path cannot be resolved inside the workspace: {raw_path}");
                    };
                    missing.push(name.to_os_string());
                    probe.pop();
                }
                Err(error) => {
                    return Err(error)
                        .with_context(|| format!("Path is unavailable: {}", probe.display()));
                }
            }
        }

        let mut resolved = Self::ensure_inside(
            &root,
            fs::canonicalize(&probe)
                .with_context(|| format!("Path is unavailable: {}", probe.display()))?,
        )?;
        for component in missing.iter().rev() {
            resolved.push(component);
        }
        Ok(resolved)
    }

    pub(crate) fn root(&self) -> Result<PathBuf> {
        let root = fs::canonicalize(&self.root).context("Workspace directory is unavailable")?;
        if !root.is_dir() {
            bail!("Workspace path is not a directory: {}", root.display());
        }
        Ok(root)
    }

    fn candidate(root: &Path, raw_path: &str) -> Result<PathBuf> {
        if raw_path.trim().is_empty() {
            bail!("Path must not be empty");
        }
        let path = Path::new(raw_path);
        Ok(if path.is_absolute() {
            path.to_path_buf()
        } else {
            root.join(path)
        })
    }

    fn ensure_inside(root: &Path, path: PathBuf) -> Result<PathBuf> {
        if path.starts_with(root) {
            Ok(path)
        } else {
            bail!("Path escapes the workspace: {}", path.display());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WorkspacePath;

    #[test]
    fn rejects_paths_outside_workspace() {
        let root = tempfile::tempdir().unwrap();
        let workspace = WorkspacePath::new(root.path());
        assert!(workspace.existing("../outside").is_err());
        assert!(workspace.for_write("../outside/new.txt").is_err());
    }

    #[test]
    fn resolves_new_files_without_leaving_workspace() {
        let root = tempfile::tempdir().unwrap();
        let workspace = WorkspacePath::new(root.path());
        let path = workspace.for_write("nested/new.txt").unwrap();
        assert!(path.starts_with(workspace.root().unwrap()));
        assert_eq!(
            path.file_name().and_then(|name| name.to_str()),
            Some("new.txt")
        );
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlinked_write_targets() {
        let root = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let link = root.path().join("linked");
        std::os::unix::fs::symlink(outside.path(), &link).unwrap();
        let workspace = WorkspacePath::new(root.path());

        assert!(workspace.for_write("linked/new.txt").is_err());
    }

    #[cfg(unix)]
    #[test]
    fn rejects_broken_symlinks_without_creating_their_targets() {
        let root = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let destination = outside.path().join("missing");
        std::os::unix::fs::symlink(&destination, root.path().join("broken")).unwrap();
        let workspace = WorkspacePath::new(root.path());
        assert!(workspace.for_write("broken").is_err());
        assert!(workspace.for_write("broken/new.txt").is_err());
        assert!(!destination.exists());
    }
}
