# MilkNote

MilkNote là ứng dụng ghi chú Markdown offline-first dành cho học tập, ưu tiên dữ liệu dạng tệp mở.

## Trạng thái

Đã hoàn thành M0 và M1:

- Workspace Rust với `milknote-core` thuần, không phụ thuộc GUI.
- `storage::write_atomic` với hash/mtime conflict guard, kiểm tra path trong vault và dọn tệp tạm mồ côi.
- Vault tạo/mở, CRUD ghi chú, đổi tên, xóa vào thùng rác.
- Journal recovery và snapshot/restore history.
- Frontend Vite + Svelte scaffold bằng tiếng Việt.

Đã hoàn thành M2:

- Tauri v2 IPC commands và `AppState` dùng `Mutex`.
- Frontend API dùng Tauri thật trong desktop và mock có kiểu trong dev/test.
- Store trạng thái giao diện có hydrate, đổi theme và đổi font/cỡ chữ.

## Chạy kiểm tra

```bash
source "$HOME/.cargo/env"
cargo fmt --all -- --check
cargo test -p milknote-core
cargo clippy --workspace --all-targets -- -D warnings
npm install
npm run lint
npm run build
npm test
```

## Cấu trúc

- `crates/milknote-core`: logic nghiệp vụ thuần Rust và test.
- `src-tauri`: lớp ứng dụng mỏng; tích hợp cửa sổ Tauri sẽ nối sau phần lõi.
- `src`: frontend Svelte.
- `docs`: quyết định, dependency, tiến độ và các ghi chú kỹ thuật.

## Ghi chú môi trường

Sandbox hiện tại là Linux nên không thể tạo hoặc kiểm chứng trực tiếp installer Windows `.exe`. Workflow `.github/workflows/windows-release.yml` chạy trên `windows-latest`, tạo NSIS installer, MSI và portable ZIP có `MilkNote.exe` cùng `portable.flag`. Có thể chạy thủ công bằng GitHub Actions hoặc tạo tag `v0.1.0` để phát hành artifact.
