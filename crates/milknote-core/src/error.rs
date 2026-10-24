use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("đường dẫn không hợp lệ hoặc nằm ngoài vault: {0}")]
    InvalidPath(String),
    #[error("tệp không tồn tại: {0}")]
    NotFound(PathBuf),
    #[error("xung đột dữ liệu: tệp đã thay đổi bên ngoài")]
    Conflict { disk_hash: String, disk_mtime: u64 },
    #[error("lỗi hệ thống tệp: {0}")]
    Io(#[from] std::io::Error),
    #[error("lỗi dữ liệu: {0}")]
    Data(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
