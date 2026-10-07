# Katna Mail, Vietnamese (Tiếng Việt).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = Tạm ẩn đến…
snooze-later-today = Cuối ngày hôm nay
snooze-tomorrow = Ngày mai
snooze-this-weekend = Cuối tuần này
snooze-next-week = Tuần sau
snooze-pick = Chọn ngày và giờ
snooze-back = Quay lại danh sách thời điểm
snooze-type-placeholder = Nhập thời điểm
snooze-type-hint = Ví dụ “thứ ba 15:00”, “ngày mai” hoặc “sau 2 giờ”
snooze-type-hint-unclear = Katna không hiểu đó là thời điểm nào
snooze-type-unclear = “{ $text }” không phải thời điểm mà Katna hiểu được

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = Tạm ẩn
remind-tab = Nhắc tôi
snooze-says = Ẩn thư cho đến lúc đó
remind-says = Giữ thư ở nguyên chỗ và thông báo cho bạn
remind-before-due = Trước hạn chót
remind-note = Ghi chú (không bắt buộc)
remind-note-placeholder = Tiêu đề thư, nếu để trống
toast-remind-set = Đã đặt lời nhắc vào { $date }
remind-chat-line = Lời nhắc { $date } · { $title }
remind-done = Xong
toast-remind-done = Đã xong lời nhắc
snooze-chat-line = Đã tạm ẩn đến { $date }
snooze-chat-change = Thay đổi

## The date and time picker

snooze-cancel = Hủy
snooze-save = Lưu
snooze-in-the-past = Hãy chọn một thời điểm sau bây giờ.

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = Theo dõi nếu không có trả lời…
follow-up-title = Theo dõi nếu không có trả lời
follow-up-off = Tắt
follow-up-days = { $days ->
   *[other] { $days } ngày
}
follow-up-weeks = { $weeks ->
   *[other] { $weeks } tuần
}
follow-up-pick = Chọn…
follow-up-pick-title = Theo dõi nếu không có trả lời trước
follow-up-remind = Nhắc tôi
follow-up-remind-note = Cuộc hội thoại sẽ quay lại đầu Hộp thư đến của bạn
follow-up-send = Gửi thư theo dõi giúp tôi
follow-up-send-note = Tới cùng những người nhận, trong cùng cuộc hội thoại
follow-up-send-encrypted = Không dùng được cho thư mã hóa
follow-up-text-placeholder = Nội dung cần viết
follow-up-text-named = Chào { $name }, mình chỉ muốn hỏi xem bạn đã thấy thư bên dưới của mình chưa.
follow-up-text = Chào bạn, mình chỉ muốn hỏi xem bạn đã thấy thư bên dưới của mình chưa.
follow-up-template = Dùng mẫu
follow-up-signature = Chữ ký của bạn sẽ được thêm vào
follow-up-again = Nếu vẫn chưa có trả lời, gửi thư theo dõi lần nữa sau
follow-up-note = Dừng ngay khi có người trong cuộc hội thoại trả lời. Thư trả lời tự động không được tính.
follow-up-note-send = Dừng ngay khi có người trong cuộc hội thoại trả lời. Được gửi vào ngày thường từ { $start } đến { $end }, và không bao giờ trễ quá một ngày.
follow-up-cancel = Hủy
follow-up-done = Xong
follow-up-chip-send = Thư theo dõi sau { $time }
follow-up-chip-remind = Lời nhắc sau { $time }
follow-up-chip-send-on = Thư theo dõi { $date }
follow-up-chip-remind-on = Lời nhắc { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = Chưa có trả lời
follow-up-card-title-waiting = Thư theo dõi của bạn đang chờ
follow-up-card-send = Katna sẽ gửi thư theo dõi của bạn vào { $date }. Sẽ dừng khi có người trả lời.
follow-up-card-send-twice = Katna sẽ gửi thư theo dõi của bạn vào { $date }, rồi thêm một lần nữa sau đó. Sẽ dừng khi có người trả lời.
follow-up-card-remind = Nếu không ai trả lời, cuộc hội thoại này sẽ quay lại Hộp thư đến của bạn vào { $date }.
follow-up-card-waiting = Thư đến hạn khi máy tính của bạn đang tắt, nên chưa được gửi muộn. Hãy gửi ngay, chọn thời điểm mới hoặc dừng lại.
follow-up-card-edit = Sửa
follow-up-card-edit-title = Theo dõi vào
follow-up-card-send-now = Gửi ngay
follow-up-card-stop = Dừng
follow-up-chat-send = Thư theo dõi · { $date } nếu không ai trả lời
follow-up-chat-step = Thư theo dõi { $step }/{ $steps } · { $date } nếu không ai trả lời
follow-up-chat-waiting = Thư theo dõi đang chờ · đã đến hạn khi máy tính của bạn đang tắt
follow-up-chat-remind = Quay lại Hộp thư đến { $date } nếu không có trả lời
toast-follow-up-sent = Đã gửi thư theo dõi
toast-follow-up-stopped = Đã dừng thư theo dõi
toast-follow-up-moved = Đã dời thư theo dõi sang { $date }
nudge-row = Đã gửi { $days ->
   *[other] { $days } ngày trước
}. Gửi thư theo dõi?
nudge-row-tip = Viết thư theo dõi gửi tới mọi người trong cuộc hội thoại
nudge-follow-up = Theo dõi
nudge-dismiss = Bỏ qua
nudge-card-title = Chưa có trả lời
nudge-card-text = Bạn đã hỏi một điều { $days ->
   *[other] { $days } ngày trước
} và chưa ai trả lời.
nudge-chat-line = Đã gửi { $days ->
   *[other] { $days } ngày trước
}, chưa có trả lời
toast-nudge-dismissed = Đã bỏ qua lời nhắc nhở
