# M2 — Cầu nối Tauri IPC và trạng thái ứng dụng

Frontend gọi backend qua một API duy nhất ở `src/lib/api.js`. Khi chạy trong Tauri, API dùng `invoke` từ `@tauri-apps/api/core`; khi chạy dev server hoặc test trên Linux, API tự chuyển sang `mockApi` trong bộ nhớ. Điều này giữ luồng ứng dụng thật tách khỏi mock và cho phép kiểm thử UI không cần mở cửa sổ desktop.

Backend đăng ký command trong `src-tauri/src/lib.rs`. `AppState` được Tauri quản lý bằng `Mutex`, gồm snapshot giao diện và vault đang mở. Các command trả `Result<..., String>` để lỗi IPC được Promise phía frontend reject, đồng thời không đưa trực tiếp kiểu lỗi Rust nội bộ qua IPC.

Các command M2 gồm `app_get_state`, `app_set_theme`, `app_set_typography`, `vault_open`, `note_read`, `note_create` và `note_update`. Command ghi chú vẫn gọi `milknote-core`, vì vậy lớp Tauri chỉ là bridge, không chứa logic lưu trữ riêng.

Trạng thái frontend nằm trong Svelte store `src/lib/app-state.js`. Khi mount, store hydrate từ backend/mock; thay đổi theme và typography cập nhật qua IPC rồi ghi lại snapshot vào store. Trạng thái hiện gồm vault path, theme, font family, font scale, paper lines và lỗi gần nhất.
