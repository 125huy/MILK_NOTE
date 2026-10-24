use crate::{
    error::{AppError, Result},
    storage::{hash_bytes, Storage, WriteOptions},
};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Draft {
    pub source: String,
    pub original_hash: String,
    pub saved_at: u64,
    pub content: String,
}

pub fn write_draft(
    storage: &Storage,
    source: impl AsRef<Path>,
    original_hash: &str,
    saved_at: u64,
    content: &str,
) -> Result<String> {
    let key = hash_bytes(source.as_ref().to_string_lossy().as_bytes());
    let rel = format!(".milknote/recovery/{key}.draft");
    let draft = Draft {
        source: source.as_ref().to_string_lossy().into_owned(),
        original_hash: original_hash.into(),
        saved_at,
        content: content.into(),
    };
    let bytes = serde_json::to_vec(&draft).map_err(|e| AppError::Data(e.to_string()))?;
    storage.write_atomic(&rel, &bytes, &WriteOptions::default())?;
    Ok(rel)
}

pub fn read_draft(storage: &Storage, relative: impl AsRef<Path>) -> Result<Draft> {
    let bytes = storage.read(relative)?;
    serde_json::from_slice(&bytes).map_err(|e| AppError::Data(e.to_string()))
}

pub fn clear_draft(storage: &Storage, relative: impl AsRef<Path>) -> Result<()> {
    let path = storage.resolve_in_vault(relative)?;
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    #[test]
    fn draft_duoc_luu_va_doc_lai() {
        let dir = tempdir().unwrap();
        let storage = Storage::new(dir.path());
        let rel = write_draft(&storage, "note.md", "old", 42, "nháp tiếng Việt").unwrap();
        let draft = read_draft(&storage, rel).unwrap();
        assert_eq!(draft.content, "nháp tiếng Việt");
        clear_draft(&storage, ".milknote/recovery/does-not-exist").unwrap();
    }
}
