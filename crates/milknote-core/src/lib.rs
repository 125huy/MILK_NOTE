pub mod error;
pub mod history;
pub mod recovery;
pub mod storage;
pub mod vault;

pub use error::{AppError, Result};
pub use storage::{Storage, WriteOptions, WriteResult};
pub use vault::Vault;

#[cfg(test)]
mod integration_tests {
    use crate::{storage::WriteOptions, Vault};
    use tempfile::tempdir;
    #[test]
    fn luong_lam_viec_co_ban_khong_mat_du_lieu() {
        let dir = tempdir().unwrap();
        let vault = Vault::create(dir.path()).unwrap();
        vault
            .create_note("Daily/2026-10-24.md", "# Hôm nay\n")
            .unwrap();
        let original = vault
            .storage()
            .metadata("Daily/2026-10-24.md")
            .unwrap()
            .unwrap();
        vault
            .update_note(
                "Daily/2026-10-24.md",
                "# Hôm nay\n\n- [ ] Học Rust\n",
                WriteOptions {
                    expected_hash: Some(original.0),
                    expected_mtime: Some(original.1),
                },
            )
            .unwrap();
        assert!(vault
            .read_note("Daily/2026-10-24.md")
            .unwrap()
            .contains("Học Rust"));
    }
}
