# Progress

## M0 — Hoàn thành

- Khởi tạo git repository và Cargo workspace.
- Tạo `milknote-core`, `src-tauri`, frontend Vite/Svelte, docs và route manifest.
- Cổng: `cargo test -p milknote-core`, `cargo clippy --workspace --all-targets -- -D warnings`, `npm run lint`, `npm run build` đều đã chạy thành công.

## M1 — Hoàn thành

- `storage`: atomic write, hash/mtime conflict, path guard, orphan temp cleanup.
- `vault`: create/open, note CRUD, rename, trash.
- `recovery`: draft journal đọc/ghi/xóa.
- `history`: snapshot và restore.
- Cổng: 7 Rust tests pass, 0 fail; clippy sạch.

## Chưa làm

## M2 — Hoàn thành

- Thêm Tauri v2, `tauri.conf.json`, build script và icon tối thiểu.
- Thêm `AppState` với `Mutex`, command IPC cho trạng thái giao diện, vault và CRUD ghi chú.
- Thêm frontend API chọn giữa Tauri thật và mock dev/test, cùng Svelte store hydrate trạng thái.
- Cổng: `cargo check --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `npm test`, `npm run build` đều pass.

## Chưa làm

M3 trở đi chưa triển khai trong checkpoint này. Chưa chạy cửa sổ desktop Tauri tương tác trong sandbox; Windows installer và fault injection cấp tiến trình vẫn chưa được kiểm chứng.

## Lượt triển khai liền mạch — UI shell và phát hành Windows

- Nâng UI thành một cửa sổ có Trang chủ, Ghi chú, editor/preview Markdown cơ bản, Task, Note Cards, Ôn tập và Cài đặt.
- Thêm workflow Windows thật cho NSIS, MSI, portable ZIP và SHA-256 artifact.
- Frontend production build pass sau thay đổi.
- Các hệ thống nghiệp vụ nâng cao còn lại được giữ trong `KNOWN_ISSUES.md`, không đánh dấu là đã hoàn thành khi chưa có test và implementation đầy đủ.
