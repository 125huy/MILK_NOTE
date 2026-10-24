use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Component, Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use sha2::{Digest, Sha256};

use crate::error::{AppError, Result};

#[derive(Debug, Clone, Default)]
pub struct WriteOptions {
    pub expected_hash: Option<String>,
    pub expected_mtime: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteResult {
    pub hash: String,
    pub mtime: u64,
}

#[derive(Debug, Clone)]
pub struct Storage {
    root: PathBuf,
}

impl Storage {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn resolve_in_vault(&self, relative: impl AsRef<Path>) -> Result<PathBuf> {
        let input = relative.as_ref();
        if input.is_absolute() {
            return Err(AppError::InvalidPath(input.display().to_string()));
        }
        for component in input.components() {
            match component {
                Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                    return Err(AppError::InvalidPath(input.display().to_string()))
                }
                Component::CurDir | Component::Normal(_) => {}
            }
        }
        let candidate = self.root.join(input);
        if candidate.exists() {
            let canonical_root = fs::canonicalize(&self.root)?;
            let canonical = fs::canonicalize(&candidate)?;
            if !canonical.starts_with(&canonical_root) {
                return Err(AppError::InvalidPath(input.display().to_string()));
            }
        }
        Ok(candidate)
    }

    pub fn read(&self, relative: impl AsRef<Path>) -> Result<Vec<u8>> {
        let path = self.resolve_in_vault(relative)?;
        if !path.exists() {
            return Err(AppError::NotFound(path));
        }
        Ok(fs::read(path)?)
    }

    pub fn metadata(&self, relative: impl AsRef<Path>) -> Result<Option<(String, u64)>> {
        let path = self.resolve_in_vault(relative)?;
        if !path.exists() {
            return Ok(None);
        }
        let bytes = fs::read(&path)?;
        let mtime = modified_secs(&fs::metadata(path)?.modified()?);
        Ok(Some((hash_bytes(&bytes), mtime)))
    }

    pub fn write_atomic(
        &self,
        relative: impl AsRef<Path>,
        bytes: &[u8],
        opts: &WriteOptions,
    ) -> Result<WriteResult> {
        let relative = relative.as_ref();
        let path = self.resolve_in_vault(relative)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        if let Some((current_hash, current_mtime)) = self.metadata(relative)? {
            if opts
                .expected_hash
                .as_deref()
                .is_some_and(|expected| expected != current_hash)
                || opts
                    .expected_mtime
                    .is_some_and(|expected| expected != current_mtime)
            {
                return Err(AppError::Conflict {
                    disk_hash: current_hash,
                    disk_mtime: current_mtime,
                });
            }
        } else if opts.expected_hash.is_some() || opts.expected_mtime.is_some() {
            return Err(AppError::Conflict {
                disk_hash: String::new(),
                disk_mtime: 0,
            });
        }
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let name = path.file_name().and_then(|x| x.to_str()).unwrap_or("file");
        let tmp = path.with_file_name(format!(".{name}.milknote.tmp.{nonce}"));
        let result: std::io::Result<WriteResult> = (|| {
            let mut file = OpenOptions::new().create_new(true).write(true).open(&tmp)?;
            file.write_all(bytes)?;
            file.flush()?;
            file.sync_all()?;
            fs::rename(&tmp, &path)?;
            let hash = hash_bytes(bytes);
            let mtime = modified_secs(&fs::metadata(&path)?.modified()?);
            Ok(WriteResult { hash, mtime })
        })();
        if result.is_err() {
            let _ = fs::remove_file(&tmp);
        }
        result.map_err(AppError::from)
    }

    pub fn clean_orphan_temps(&self) -> Result<usize> {
        if !self.root.exists() {
            return Ok(0);
        }
        let mut removed = 0;
        for entry in walk_files(&self.root)? {
            if entry
                .file_name()
                .and_then(|x| x.to_str())
                .is_some_and(|x| x.contains(".milknote.tmp."))
            {
                fs::remove_file(entry)?;
                removed += 1;
            }
        }
        Ok(removed)
    }
}

pub fn hash_bytes(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
fn modified_secs(time: &SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn walk_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    if !root.exists() {
        return Ok(out);
    }
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            out.extend(walk_files(&path)?);
        } else {
            out.push(path);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn ghi_nguyen_tu_va_doc_lai_duoc() {
        let dir = tempdir().unwrap();
        let storage = Storage::new(dir.path());
        let result = storage
            .write_atomic("note.md", b"xin chao", &WriteOptions::default())
            .unwrap();
        assert_eq!(storage.read("note.md").unwrap(), b"xin chao");
        assert_eq!(result.hash, hash_bytes(b"xin chao"));
    }

    #[test]
    fn tu_choi_xung_dot_hash() {
        let dir = tempdir().unwrap();
        let storage = Storage::new(dir.path());
        let first = storage
            .write_atomic("note.md", b"a", &WriteOptions::default())
            .unwrap();
        storage
            .write_atomic("note.md", b"b", &WriteOptions::default())
            .unwrap();
        let error = storage
            .write_atomic(
                "note.md",
                b"c",
                &WriteOptions {
                    expected_hash: Some(first.hash),
                    expected_mtime: None,
                },
            )
            .unwrap_err();
        assert!(matches!(error, AppError::Conflict { .. }));
    }

    #[test]
    fn chan_path_traversal() {
        let dir = tempdir().unwrap();
        let storage = Storage::new(dir.path());
        assert!(matches!(
            storage.resolve_in_vault("../outside"),
            Err(AppError::InvalidPath(_))
        ));
    }
}
