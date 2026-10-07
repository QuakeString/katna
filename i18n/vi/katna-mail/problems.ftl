# Katna Mail, Vietnamese (Tiếng Việt).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = Máy chủ thư
problems-signed-out = { $provider } đã đăng xuất Katna khỏi { $address }. Thư đã ngừng đồng bộ.
problems-password-refused = { $provider } đã từ chối mật khẩu của { $address }. Có thể mật khẩu đã thay đổi.
problems-no-answer = { $provider } không phản hồi cho { $address }. Katna vẫn đang thử lại.
problems-offline = Bạn đang ngoại tuyến. Thư của bạn vẫn ở đây, và thư bạn gửi sẽ chờ cho đến khi bạn trực tuyến trở lại.
problems-accounts-need-you = { $count ->
   *[other] { $count } tài khoản cần bạn xử lý
}
problems-show = Hiện
problems-later = Để sau
problems-new-password = Mật khẩu mới
problems-try-again = Thử lại

## The New password card

problems-password-title = Mật khẩu mới
problems-password-detail = { $provider } đã từ chối mật khẩu đã lưu của { $address }. Hãy nhập mật khẩu mới; Katna sẽ kiểm tra trước khi lưu.
problems-password-placeholder = Mật khẩu
problems-password-show = Hiện mật khẩu
problems-password-hide = Ẩn mật khẩu
problems-password-cancel = Hủy
problems-password-save = Lưu
problems-password-checking = Đang kiểm tra…
problems-password-refused-again = { $provider } cũng từ chối mật khẩu này. Hãy kiểm tra và thử lại.
problems-password-saved = Đã lưu mật khẩu cho { $address }. Đang tải thư của bạn…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = Máy chủ thư của { $address } không chấp nhận việc di chuyển { $count ->
   *[other] { $count } thư, nên chúng đã trở về chỗ cũ.
}
problems-refused-flags = Máy chủ thư của { $address } không chấp nhận việc đánh dấu { $count ->
   *[other] { $count } thư (đã đọc, gắn dấu sao…), nên chúng đã trở về như cũ.
}
problems-refused-label = Máy chủ thư của { $address } không chấp nhận việc thay đổi nhãn của { $count ->
   *[other] { $count } thư, nên chúng đã trở về như cũ.
}
problems-refused-delete = Máy chủ thư của { $address } không chấp nhận việc xóa { $count ->
   *[other] { $count } thư, nên chúng đã được khôi phục.
}
problems-refused-other = Máy chủ thư của { $address } không chấp nhận { $count ->
   *[other] { $count } thay đổi, nên Katna đã khôi phục như cũ.
}
problems-details = Chi tiết

## Katna's background service (katna-daemon) isn't running

service-starting = Đang khởi động dịch vụ nền của Katna…
service-failed = Dịch vụ nền của Katna không khởi động được, nên thư không được đồng bộ.
service-start-again = Khởi động lại
service-started-again = Dịch vụ nền của Katna đã dừng và đã được khởi động lại.
service-details-title = Vì sao dịch vụ không khởi động được
service-details-body = Hãy sao chép nội dung này và gửi kèm báo cáo của bạn. Nó không chứa thư hay mật khẩu nào.
service-details-copy = Sao chép
service-details-close = Đóng
