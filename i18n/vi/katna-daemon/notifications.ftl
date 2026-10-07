# Katna Mail, Vietnamese (Tiếng Việt).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = { $count } email mới
notify-and-more = và { $count } thư khác
notify-no-subject = (không có tiêu đề)
notify-unknown-sender = Người gửi không xác định

## Reminders the user asked for (same buttons)

notify-snooze-back = Thư tạm ẩn đã quay lại
notify-no-reply = Chưa có trả lời
notify-no-reply-to = Chưa ai trả lời “{ $subject }”.
notify-follow-up-sent = Đã gửi thư theo dõi
notify-follow-up-sent-to = Chưa ai trả lời “{ $subject }”, nên Katna đã gửi thư theo dõi.
notify-follow-up-waiting = Chưa gửi thư theo dõi
notify-follow-up-waiting-to = Thư đến hạn khi máy tính này đang tắt. “{ $subject }” đã quay lại Hộp thư đến của bạn.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } đã mở { $subject }
notify-tracking-clicked = { $who } đã nhấp vào một liên kết trong { $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = Có thể cập nhật Katna Mail
notify-update-ready-body = Phiên bản { $version } đã được tải xuống. Cập nhật sẽ cài đặt nó và khởi động lại Katna Mail.
notify-update = Cập nhật

## Something needs the user, shown once per problem

notify-signed-out = Đăng nhập lại
notify-signed-out-body = { $provider } đã đăng xuất Katna khỏi { $address }. Thư đã ngừng đồng bộ.
notify-sign-in = Đăng nhập
notify-password-refused = Mật khẩu bị từ chối
notify-password-refused-body = Máy chủ thư đã từ chối mật khẩu của { $address }. Có thể mật khẩu đã thay đổi.
notify-new-password = Mật khẩu mới
notify-not-sent = Chưa gửi được “{ $subject }”
notify-not-sent-no-subject = Chưa gửi được một thư
notify-not-sent-body = Thư đang ở Hộp thư đi, nơi cho biết lý do.
notify-open-outbox = Mở Hộp thư đi

## Reminders of calendar events

notify-event-now = Bây giờ
notify-event-in-minutes = { $count ->
   *[other] Sau { $count } phút
}
notify-event-in-hours = { $count ->
   *[other] Sau { $count } giờ
}
notify-event-in-days = { $count ->
    [1] Ngày mai
   *[other] Sau { $count } ngày
}
notify-event-all-day = Cả ngày
notify-event-join = Tham gia
notify-event-snooze = Báo lại sau 5 phút
notify-task-done = Đánh dấu là đã hoàn thành

## The buttons of new-mail notifications and reminders

notify-open = Mở
notify-peek = Xem nhanh
notify-reply = Trả lời
notify-reply-placeholder = Trả lời { $name }…
notify-send = Gửi
notify-reply-all = Trả lời tất cả
notify-mark-read = Đánh dấu là đã đọc
notify-mark-all-read = Đánh dấu tất cả là đã đọc
notify-archive = Lưu trữ
notify-snooze-hour = Tạm ẩn 1 giờ
notify-snooze-tomorrow = Ngày mai
notify-copy-code = Sao chép { $code }
notify-link-verify = Xác minh trên { $domain }
notify-link-confirm = Xác nhận trên { $domain }
notify-link-activate = Kích hoạt trên { $domain }

## After Archive on a notification: a short note in the same place

notify-archived = Đã lưu trữ
notify-archived-count = { $count ->
   *[other] Đã chuyển { $count } thư ra khỏi hộp thư đến
}
notify-undo = Hoàn tác

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = Đã sao chép mã
notify-code-not-copied = Không thể sao chép mã

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Đã gửi thư trả lời cho { $name }
notify-open-in-katna = Mở trong Katna
