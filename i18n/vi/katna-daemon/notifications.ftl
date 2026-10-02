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

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } đã mở { $subject }
notify-tracking-clicked = { $who } đã nhấp vào một liên kết trong { $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = Có thể cập nhật Katna Mail
notify-update-ready-body = Phiên bản { $version } đã được tải xuống. Cập nhật sẽ cài đặt nó và khởi động lại Katna Mail.
notify-update = Cập nhật

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

## After Archive on a notification: a short note in the same place

notify-archived = Đã lưu trữ
notify-archived-count = { $count ->
   *[other] Đã chuyển { $count } thư ra khỏi hộp thư đến
}
notify-undo = Hoàn tác

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Đã gửi thư trả lời cho { $name }
notify-open-in-katna = Mở trong Katna
