# Katna Mail, Vietnamese (Tiếng Việt): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Ghi chú
notes-view-reminders = Lời nhắc
notes-view-archive = Lưu trữ
notes-view-trash = Thùng rác
notes-edit-labels = Chỉnh sửa nhãn
notes-search = Tìm kiếm ghi chú
notes-loading = Đang mở ghi chú của bạn…

## Board

notes-take-a-note = Tạo ghi chú…
notes-new-list = Danh sách mới
notes-new-note = Ghi chú mới
notes-pinned = Đã ghim
notes-others = Khác
notes-empty = Ghi chú bạn thêm sẽ xuất hiện ở đây
notes-archive-empty = Ghi chú đã lưu trữ của bạn sẽ xuất hiện ở đây
notes-trash-empty = Không có ghi chú nào trong thùng rác
notes-none-found = Không có ghi chú phù hợp
notes-label-empty = Chưa có ghi chú nào có nhãn này
notes-reminders-empty = Ghi chú có lời nhắc sắp tới sẽ hiện ở đây
notes-trash-note = Ghi chú trong thùng rác sẽ bị xóa sau 7 ngày.
notes-empty-trash = Dọn sạch thùng rác
notes-ticked = { $count ->
   *[other] + { $count } mục đã đánh dấu
}
notes-select = Chọn ghi chú
notes-selected = { $count ->
   *[other] Đã chọn { $count }
}
notes-select-clear = Bỏ chọn

## A note's buttons

notes-pin = Ghim ghi chú
notes-unpin = Bỏ ghim ghi chú
notes-archive = Lưu trữ
notes-unarchive = Bỏ lưu trữ
notes-delete = Xóa ghi chú
notes-restore = Khôi phục
notes-delete-forever = Xóa vĩnh viễn
notes-color = Màu nền
notes-checkboxes = Hiện hoặc ẩn hộp kiểm
notes-labels = Nhãn
notes-close = Đóng
notes-more = Thêm
notes-make-copy = Tạo bản sao
notes-remind = Nhắc tôi
notes-add-picture = Thêm hình ảnh
notes-history = Lịch sử phiên bản
notes-ai = Giúp tôi viết
notes-send-as-mail = Gửi dưới dạng thư
notes-save-markdown = Lưu dưới dạng Markdown
notes-save-pdf = Lưu dưới dạng PDF

## The open note

notes-title = Tiêu đề
notes-edited = Đã chỉnh sửa { $date }
notes-on-this-computer = Trên máy tính này
notes-where = Nơi lưu ghi chú này
notes-untitled = Ghi chú không có tiêu đề

## Pictures

notes-picture-choose = Thêm hình ảnh
notes-picture-remove = Xóa hình ảnh
notes-picture-too-big = Ghi chú nhận hình ảnh tối đa { $size }
notes-picture-kind = Tệp đó không phải hình ảnh mà Katna có thể hiển thị
notes-picture-unreadable = Không thể đọc { $name }: { $error }

## Reminders

notes-remind-me = Nhắc tôi
notes-remind-off = Xóa lời nhắc
notes-remind-in-the-past = Hãy chọn thời điểm chưa qua
notes-remind-today = Hôm nay, { $time }
notes-remind-tomorrow = Ngày mai, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = Đã đặt lời nhắc vào { $when }
notes-reminder-off = Đã xóa lời nhắc

## Links between notes

notes-link-note = Liên kết ghi chú
notes-link-new = Ghi chú mới “{ $title }”
notes-linked-from = Được liên kết từ
notes-link-gone = Ghi chú đó không còn ở đây nữa
notes-new-note-gone = Ghi chú mới không còn nữa.

## Version history

notes-versions = Phiên bản
notes-version-now = Hiện tại
notes-version-here = Bạn, trên máy tính này
notes-version-yesterday = Hôm qua, { $time }
notes-version-changes = { $count ->
   *[other] { $count } thay đổi
}
notes-version-from = Từ { $device }
notes-version-elsewhere = Từ thiết bị khác
notes-version-created = Đã tạo
notes-version-restore = Khôi phục phiên bản này
notes-version-restored = Đã khôi phục phiên bản
notes-history-none = Chưa có phiên bản trước nào

## AI help

notes-ai-tidy = Chỉnh gọn văn bản
notes-ai-checklist = Chuyển thành danh sách kiểm
notes-ai-summarise = Tóm tắt
notes-ai-empty = Hãy viết gì đó trước
notes-ai-tidied = Đã chỉnh gọn văn bản. Ctrl+Z để hoàn tác.
notes-ai-listed = Đã chuyển thành danh sách kiểm. Ctrl+Z để hoàn tác.
notes-ai-summarised = Đã thêm bản tóm tắt ở trên cùng

## Labels

notes-label-note = Gắn nhãn cho ghi chú
notes-label-name = Nhập tên nhãn
notes-label-create = Tạo “{ $name }”
notes-label-remove = Xóa nhãn khỏi ghi chú
notes-label-delete = Xóa nhãn
notes-labels-none = Chưa có nhãn nào. Hãy thêm từ nút nhãn của một ghi chú.
notes-labels-done = Xong
notes-label-renamed = Đã đổi tên nhãn thành “{ $name }”
notes-label-deleted = Đã xóa nhãn “{ $name }”

## A note about a mail

notes-mail = Thư
notes-open-mail = Mở thư
notes-open-note = Mở ghi chú

## Meeting notes

notes-meeting-take = Ghi chú cuộc họp
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = Người tham dự: { $names }
notes-meeting-notes = Ghi chú
notes-meeting-actions = Việc cần làm
notes-event = Sự kiện
notes-open-event = Mở sự kiện

## Formatting

notes-format = Định dạng
notes-format-heading-1 = Tiêu đề 1
notes-format-heading-2 = Tiêu đề 2
notes-format-normal = Văn bản thường
notes-format-bold = In đậm
notes-format-italic = In nghiêng
notes-format-underline = Gạch chân
notes-format-quote = Trích dẫn
notes-format-code = Mã
notes-format-divider = Đường phân cách
notes-format-clear = Xóa định dạng

## Tasks

notes-make-task = Biến thành việc cần làm

## Colors (tooltips)

notes-color-none = Không có màu
notes-color-coral = San hô
notes-color-peach = Đào
notes-color-sand = Cát
notes-color-mint = Bạc hà
notes-color-sage = Xô thơm
notes-color-fog = Sương mù
notes-color-storm = Bão
notes-color-dusk = Hoàng hôn
notes-color-blossom = Hoa
notes-color-clay = Đất sét
notes-color-chalk = Phấn

## Messages at the foot of the window

notes-archived = Đã lưu trữ ghi chú
notes-unarchived = Đã bỏ lưu trữ ghi chú
notes-trashed = Đã chuyển ghi chú vào thùng rác
notes-restored = Đã khôi phục ghi chú
notes-saved = Đã lưu ghi chú
notes-pinned-count = { $count ->
   *[other] Đã ghim { $count } ghi chú
}
notes-unpinned-count = { $count ->
   *[other] Đã bỏ ghim { $count } ghi chú
}
notes-colored-count = { $count ->
   *[other] Đã đổi màu { $count } ghi chú
}
notes-archived-count = { $count ->
   *[other] Đã lưu trữ { $count } ghi chú
}
notes-unarchived-count = { $count ->
   *[other] Đã bỏ lưu trữ { $count } ghi chú
}
notes-trashed-count = { $count ->
   *[other] Đã chuyển { $count } ghi chú vào Thùng rác
}
notes-restored-count = { $count ->
   *[other] Đã khôi phục { $count } ghi chú
}
notes-copied-count = { $count ->
   *[other] Đã tạo { $count } bản sao
}
notes-empty-discarded = Đã bỏ ghi chú trống
notes-mail-gone = Thư đó không còn ở đây nữa
notes-deleted-forever = { $count ->
   *[other] Đã xóa vĩnh viễn { $count } ghi chú
}
