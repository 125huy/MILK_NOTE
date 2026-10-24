use crate::{
    error::{AppError, Result},
    storage::{Storage, WriteOptions},
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Clone)]
pub struct Vault {
    storage: Storage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrashEntry {
    pub original: String,
    pub trashed: String,
    pub deleted_at: u64,
}

impl Vault {
    pub fn create(root: impl Into<PathBuf>) -> Result<Self> {
        let root = root.into();
        fs::create_dir_all(&root)?;
        let vault = Self {
            storage: Storage::new(root),
        };
        for dir in [
            "Cards",
            "Daily",
            "Templates",
            "attachments",
            ".milknote/history",
            ".milknote/recovery",
            ".milknote/trash",
        ] {
            fs::create_dir_all(vault.storage.root().join(dir))?;
        }
        vault.storage.clean_orphan_temps()?;
        Ok(vault)
    }
    pub fn open(root: impl Into<PathBuf>) -> Result<Self> {
        Self::create(root)
    }
    pub fn storage(&self) -> &Storage {
        &self.storage
    }
    pub fn create_note(&self, relative: impl AsRef<Path>, content: &str) -> Result<()> {
        let relative = relative.as_ref();
        validate_note(relative)?;
        if self.storage.resolve_in_vault(relative)?.exists() {
            return Err(AppError::Data("ghi chú đã tồn tại".into()));
        }
        self.storage
            .write_atomic(relative, content.as_bytes(), &WriteOptions::default())?;
        Ok(())
    }
    pub fn read_note(&self, relative: impl AsRef<Path>) -> Result<String> {
        let bytes = self.storage.read(relative)?;
        String::from_utf8(bytes).map_err(|e| AppError::Data(format!("tệp không phải UTF-8: {e}")))
    }
    pub fn update_note(
        &self,
        relative: impl AsRef<Path>,
        content: &str,
        opts: WriteOptions,
    ) -> Result<()> {
        validate_note(relative.as_ref())?;
        self.storage
            .write_atomic(relative, content.as_bytes(), &opts)?;
        Ok(())
    }
    pub fn rename_note(&self, from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
        validate_note(from.as_ref())?;
        validate_note(to.as_ref())?;
        let source = self.storage.resolve_in_vault(from.as_ref())?;
        let target = self.storage.resolve_in_vault(to.as_ref())?;
        if !source.exists() {
            return Err(AppError::NotFound(source));
        }
        if target.exists() {
            return Err(AppError::Data("tên ghi chú đích đã tồn tại".into()));
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::rename(source, target)?;
        Ok(())
    }
    pub fn delete_note(&self, relative: impl AsRef<Path>) -> Result<TrashEntry> {
        let relative = relative.as_ref();
        validate_note(relative)?;
        let source = self.storage.resolve_in_vault(relative)?;
        if !source.exists() {
            return Err(AppError::NotFound(source));
        }
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let safe = relative.to_string_lossy().replace(['/', '\\'], "__");
        let trashed_rel = format!(".milknote/trash/{stamp}-{safe}");
        let target = self.storage.resolve_in_vault(&trashed_rel)?;
        fs::rename(&source, &target)?;
        let entry = TrashEntry {
            original: relative.to_string_lossy().into_owned(),
            trashed: trashed_rel,
            deleted_at: stamp,
        };
        let manifest = format!(".milknote/trash/{stamp}-{safe}.json");
        self.storage.write_atomic(
            manifest,
            serde_json::to_vec_pretty(&entry)
                .map_err(|e| AppError::Data(e.to_string()))?
                .as_slice(),
            &WriteOptions::default(),
        )?;
        Ok(entry)
    }
}
fn validate_note(path: &Path) -> Result<()> {
    if path.extension().and_then(|x| x.to_str()) != Some("md") {
        return Err(AppError::Data("ghi chú phải có phần mở rộng .md".into()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    #[test]
    fn crud_ghi_chu_va_thung_rac() {
        let dir = tempdir().unwrap();
        let vault = Vault::create(dir.path()).unwrap();
        vault.create_note("hoc/bai.md", "# Bài học").unwrap();
        assert_eq!(vault.read_note("hoc/bai.md").unwrap(), "# Bài học");
        vault.rename_note("hoc/bai.md", "hoc/bai-moi.md").unwrap();
        let trash = vault.delete_note("hoc/bai-moi.md").unwrap();
        assert_eq!(trash.original, "hoc/bai-moi.md");
        assert!(!vault
            .storage()
            .resolve_in_vault("hoc/bai-moi.md")
            .unwrap()
            .exists());
    }
}
