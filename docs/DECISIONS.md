# Quyết định kỹ thuật

## 2026-10-24 — Khởi tạo M0/M1

- Chọn Rust workspace với `milknote-core` thuần để test nghiệp vụ độc lập với GUI.
- Tạm dùng lớp `src-tauri` mỏng chưa kéo dependency Tauri trong sandbox Linux; tránh khóa build vào thư viện GTK/WebKit trước khi phần lõi ổn định.
- Ghi dữ liệu qua `Storage::write_atomic`: tệp tạm cùng thư mục, flush, sync và rename nguyên tử.
- Xung đột được phát hiện bằng hash/mtime kỳ vọng; không ghi đè im lặng.
- Thùng rác nằm trong `.milknote/trash`; lịch sử và recovery nằm trong `.milknote/history` và `.milknote/recovery`.
- Frontend M0 dùng Vite + Svelte scaffold để có cổng build sớm.
