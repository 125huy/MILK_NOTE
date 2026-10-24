use crate::{
    error::Result,
    storage::{Storage, WriteOptions},
};
use std::{
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn snapshot(storage: &Storage, source: impl AsRef<Path>, content: &[u8]) -> Result<String> {
    let source = source.as_ref();
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let safe = source.to_string_lossy().replace(['/', '\\'], "__");
    let rel = format!(".milknote/history/{safe}/{stamp}.md");
    storage.write_atomic(&rel, content, &WriteOptions::default())?;
    Ok(rel)
}

pub fn restore(
    storage: &Storage,
    snapshot_rel: impl AsRef<Path>,
    target: impl AsRef<Path>,
) -> Result<()> {
    let content = storage.read(snapshot_rel)?;
    storage.write_atomic(target, &content, &WriteOptions::default())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    #[test]
    fn snapshot_va_khoi_phuc() {
        let dir = tempdir().unwrap();
        let storage = Storage::new(dir.path());
        let snap = snapshot(&storage, "note.md", b"ban cu").unwrap();
        storage
            .write_atomic("note.md", b"ban moi", &WriteOptions::default())
            .unwrap();
        restore(&storage, snap, "note.md").unwrap();
        assert_eq!(storage.read("note.md").unwrap(), b"ban cu");
    }
}
