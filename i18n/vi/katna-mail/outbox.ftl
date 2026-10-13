# Katna Mail, Vietnamese (Tiếng Việt).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = Chưa gửi được vì { $reason }.
outbox-retrying = Chưa gửi được vì { $reason }. Katna sẽ tự thử lại.
outbox-waiting-sign-in = Đang chờ bạn đăng nhập lại vào { $address }. Thư sẽ được gửi đi sau đó.
outbox-waiting-password = Đang chờ mật khẩu mới của { $address }. Thư sẽ được gửi đi sau đó.
outbox-waiting-connection = Đang chờ kết nối. Thư sẽ được gửi đi khi bạn trực tuyến trở lại.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = thư không có người nhận
outbox-reason-address = một địa chỉ nhận không tồn tại
outbox-reason-too-large = thư quá lớn đối với máy chủ thư
outbox-reason-blocked = máy chủ thư đã chặn thư
outbox-reason-gone = bản sao của thư trên máy tính này không còn nữa
outbox-reason-refused = máy chủ thư đã từ chối thư

## Buttons and notes

outbox-try-again = Thử lại
outbox-edit = Sửa
outbox-delete = Xóa
outbox-deleted = Đã xóa khỏi Hộp thư đi
outbox-sending-again = Đang gửi lại…
outbox-snackbar-not-sent = Chưa gửi được “{ $subject }” vì { $reason }.
outbox-open = Hộp thư đi
