# Katna Mail, Vietnamese (Tiếng Việt).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = Đọc thư
chat-view = Cuộc hội thoại dạng trò chuyện
chat-view-detail = Thư giữa mọi người được hiển thị như một cuộc trò chuyện nhóm: mỗi thư là một bong bóng chỉ chứa nội dung được viết, thư của bạn ở bên phải. Bản tin vẫn giữ chế độ xem thông thường.
chat-view-switch = Hiện cuộc hội thoại dạng trò chuyện
chat-view-switch-detail = Thư được trích dẫn và chữ ký nằm sau nút ··· trong mỗi bong bóng
chat-switch-chat = Trò chuyện
chat-switch-mail = Thư
chat-people = { $names } và bạn · { $count ->
   *[other] { $count } thư
}
chat-people-heading = { $count ->
   *[other] Trong cuộc trò chuyện này · { $count } người
}
chat-member-mails = { $count ->
    [0] Không có thư
   *[other] { $count } thư
}
chat-today = Hôm nay
chat-yesterday = Hôm qua
chat-added = { $who } đã thêm { $names }
chat-renamed = { $who } đã đổi tiêu đề thành “{ $subject }”
chat-you = Bạn
chat-not-downloaded = Chưa tải xuống
chat-forwarded = Thư chuyển tiếp
chat-show-quoted = Hiện thư được trích dẫn và chữ ký
chat-hide-quoted = Ẩn thư được trích dẫn và chữ ký
chat-hide-dots = Ẩn ···
chat-show-card = Hiện thẻ liên hệ
chat-reply-all = Trả lời tất cả
chat-more = Thêm
chat-reply-only = Chỉ trả lời { $name }
chat-forward = Chuyển tiếp
chat-copy-text = Sao chép văn bản
chat-show-as-mail = Hiện dạng thư
chat-go-down = Đi tới thư mới nhất
chat-pin = Ghim lên đầu
chat-pin-file = Ghim tệp lên đầu
chat-unpin = Bỏ ghim
chat-unpin-file = Bỏ ghim tệp
chat-pinned-of = Đã ghim { $at } trên { $count }
chat-pins-all = Tất cả mục đã ghim
chat-pins-heading = Đã ghim · { $count } trên { $most }
chat-pins-drag = Kéo để sắp xếp lại
chat-pin-from-mail = Thư từ { $name } · { $when }
chat-pin-from-file = Tệp từ { $name } · { $when }
chat-pin-from-text = Văn bản từ { $name } · { $when }
chat-pins-full = Cuộc trò chuyện này đã có 5 mục được ghim
chat-pins-replace-title = Thay một mục đã ghim
chat-pins-replace-hint = Mỗi cuộc trò chuyện có tối đa 5 mục được ghim. Hãy chọn mục cần bỏ.
chat-pins-replace = Thay thế
chat-pins-cancel = Hủy
chat-undo = Hoàn tác
chat-reply-to = Trả lời { $names }
chat-send = Gửi (Ctrl+Enter). Nhấp chuột phải hoặc nhấn giữ để xem thêm
chat-send-now = Gửi ngay
chat-attach = Đính kèm
chat-attach-photo = Ảnh
chat-attach-file = Tệp
chat-attach-library = Từ Tệp
chat-attach-template = Mẫu thư
chat-attach-signature = Chữ ký
chat-replying-to = Đang trả lời { $name }
chat-reply-newest = Trả lời thư mới nhất

## The attach picker (paperclip > From Files)

picker-title = Đính kèm từ Tệp
picker-search = Tìm tên, người, tiêu đề
picker-search-drive = Tìm trong ổ lưu trữ này
picker-mail-files = Tệp trong thư
picker-this-chat = Cuộc hội thoại này
picker-this-computer = Máy tính này…
picker-in-chat = TRONG CUỘC HỘI THOẠI NÀY
picker-recent = GẦN ĐÂY
picker-preview = Xem trước
picker-cancel = Hủy
picker-attach = Đính kèm
picker-attach-count = Đính kèm { $count }
picker-selected = Đã chọn { $count }
picker-of-limit = trên { $limit }
picker-in-mail = { $size } trong thư
picker-drive-links = { $count ->
   *[other] { $count } dạng liên kết Google Drive
}
picker-onedrive-links = { $count ->
   *[other] { $count } dạng liên kết OneDrive
}
picker-over = { $size }, vượt quá giới hạn { $limit } của một thư
picker-getting = { $count ->
    [1] Đang lấy tệp từ ổ lưu trữ…
   *[other] Đang lấy { $count } tệp từ ổ lưu trữ…
}
picker-some-failed = { $count ->
    [1] Không đọc được một tệp
   *[other] Không đọc được { $count } tệp
}
