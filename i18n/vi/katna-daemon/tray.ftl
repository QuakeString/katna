# Katna Mail, Vietnamese (Tiếng Việt).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = Mở _Hộp thư đến
tray-new-message = Thư _mới
tray-new-task = _Việc cần làm mới
tray-new-note = _Ghi chú mới
tray-preferences = _Cài đặt
tray-quit = _Thoát

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] Không có thư chưa đọc
   *[other] { $count } thư chưa đọc
}
tray-password-refused = Cần mật khẩu mới cho { $address }
tray-signed-out = Đăng nhập lại vào { $address }
tray-accounts-need-you = { $count } tài khoản cần bạn xử lý
tray-not-sent = { $count ->
   *[other] Chưa gửi được { $count } thư
}
