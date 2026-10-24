# Known Issues

- Chưa build installer Windows trong sandbox Linux; cần chạy GitHub Actions trên `windows-latest`.
- Chưa chạy cửa sổ desktop Tauri tương tác trong sandbox; IPC đã được kiểm tra qua compile và frontend mock contract.
- Chưa có fault-injection kill tiến trình giữa ghi; hiện mới có test atomic write, conflict và path traversal.
- Chưa kiểm tra IME Telex/VNI trên Windows thật.
- Chưa kết nối Google Drive.
- Các hệ thống V1 nâng cao như CodeMirror, SQLite FTS5, YAML Properties đầy đủ, Mermaid, ZIP import/export an toàn, SM-2 state.db, OAuth Drive và Quick Capture toàn cục chưa hoàn thiện trong lượt này; UI hiện đã có các màn hình nền tảng và mock flow tương ứng.
- Chưa thể tuyên bố có file `.exe` đã build; artifact Windows chỉ được tạo khi workflow chạy trên `windows-latest`.
