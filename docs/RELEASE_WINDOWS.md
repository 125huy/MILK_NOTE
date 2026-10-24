# Phát hành MilkNote cho Windows

Sandbox hiện tại chạy Linux nên không thể tạo `.exe` Windows trực tiếp. Workflow `.github/workflows/windows-release.yml` đã được cấu hình để chạy trên `windows-latest` của GitHub Actions.

Có hai cách chạy workflow. Cách thứ nhất là vào tab **Actions**, chọn **MilkNote Windows Release**, bấm **Run workflow**, rồi tải artifact `MilkNote-Windows-artifacts`. Cách thứ hai là tạo tag, ví dụ `git tag v0.1.0 && git push origin v0.1.0`; workflow sẽ build và đính kèm NSIS `.exe`, MSI, portable ZIP và SHA256SUMS vào GitHub Release.

Portable ZIP chứa `MilkNote.exe` cùng `portable.flag`. Khi đặt cạnh file thực thi, ứng dụng sẽ dùng thư mục tương đối cho dữ liệu portable theo thiết kế. Installer NSIS là gói cài đặt chính cho người dùng Windows.

Việc còn cần kiểm chứng trên Windows thật gồm cài/gỡ, WebView2, IME Telex/VNI, DPI cao, phím tắt toàn cục, ổ rút và SmartScreen.
