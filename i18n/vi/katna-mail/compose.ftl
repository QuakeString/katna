# Katna Mail, Vietnamese (Tiếng Việt).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = Thư mới
compose-restore = Khôi phục
compose-minimize = Thu nhỏ
compose-exit-full-screen = Thoát toàn màn hình
compose-open-window = Mở trong cửa sổ mới
compose-save-close = Lưu và đóng
compose-back-to-mail = Quay lại cửa sổ thư
compose-pop-out-reply = Mở thư trả lời riêng
compose-edit-recipients = Sửa người nhận
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Bcc: { $names }
compose-more-recipients = { $count } người nữa
compose-show-trimmed = Hiện nội dung bị rút gọn
compose-hide-trimmed = Ẩn nội dung bị rút gọn
compose-remove-trimmed = Xóa phần trích dẫn
compose-trimmed-removed = Đã xóa phần trích dẫn

## Recipients and subject

compose-to = Tới
compose-cc = Cc
compose-bcc = Bcc
compose-from = Từ
compose-from-choose = Gửi từ tài khoản khác
compose-recipients = Người nhận
compose-subject = Tiêu đề

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = Hãy gửi hoặc hủy thư đang mở trước.
compose-bad-address = “{ $address }” không phải là địa chỉ email.
compose-no-recipients = Thêm ít nhất một người nhận.
compose-attachments-too-large = Tệp đính kèm có dung lượng { $size }; máy chủ thư chỉ nhận tối đa { $limit }.
compose-no-account = Thêm một tài khoản để gửi thư.
compose-past-time = Chọn một thời điểm trong tương lai.
compose-scheduling = Đang lên lịch…
compose-sending = Đang gửi…
compose-scheduled = Đã lên lịch gửi vào { $when }
compose-sent-archived = Đã gửi và lưu trữ
compose-sent = Đã gửi thư
compose-discarded = Đã hủy thư nháp
compose-draft-saved = Đã lưu thư nháp
compose-draft-saving = Đang lưu…
compose-draft-failed = Không lưu được thư nháp: { $error }
compose-draft-not-opened = Không mở được thư nháp.

## Attachments

compose-picker-insert = Chèn
compose-picker-attach = Đính kèm
compose-file-too-large = { $name } quá lớn: một thư chỉ mang được tối đa { $limit }.
compose-forward-files-missing = Tệp của thư được chuyển tiếp chưa được tải xuống, nên chưa được đính kèm.
compose-attachment-size = ({ $size })
compose-remove-attachment = Xóa tệp đính kèm
compose-attachments-total = { $count } tệp, { $size }
compose-drive-note = { $name } vượt quá { $limit }, nên tệp sẽ được đưa lên Google Drive của bạn và thư sẽ kèm một liên kết.
compose-drive-tip = Trong Google Drive của bạn; thư kèm một liên kết
compose-drive-uploading = Đang tải lên { $percent }%
compose-drive-allow = Cho phép Drive
compose-drive-allow-tip = Đăng nhập lại bằng Google để Katna đưa các tệp lớn vào Drive của bạn
compose-drive-retry = Thử lại
compose-drive-sends-when-uploaded = Sẽ gửi sau khi { $name } tải lên xong
compose-drive-not-uploaded = { $name } chưa có trong Google Drive
compose-drive-share-failed = Không thể chia sẻ các tệp trong Google Drive: { $error }
compose-drive-share-title = Chia sẻ các tệp với mọi người?
compose-drive-share-text = { $count ->
   *[other] Google Drive không thể chia sẻ các tệp với { $addresses }, người không có tài khoản Google. Thay vào đó, bất kỳ ai có liên kết đều có thể mở chúng.
}
compose-drive-share-link = Chia sẻ bằng liên kết
compose-drive-send-without = Gửi mà không chia sẻ
compose-drive-share-cancel = Hủy
compose-drive-card-detail = { $size } · Google Drive
compose-drive-card-name = Google Drive
compose-onedrive-note = { $name } vượt quá { $limit }, nên tệp sẽ được đưa lên OneDrive của bạn và thư sẽ kèm một liên kết.
compose-onedrive-tip = Trong OneDrive của bạn; thư kèm một liên kết
compose-onedrive-allow = Cho phép OneDrive
compose-onedrive-allow-tip = Đăng nhập lại bằng Microsoft để Katna đưa các tệp lớn vào OneDrive của bạn
compose-onedrive-not-uploaded = { $name } chưa có trong OneDrive
compose-onedrive-share-failed = Không thể chia sẻ các tệp trong OneDrive: { $error }
compose-onedrive-share-text = { $count ->
   *[other] OneDrive không thể chia sẻ các tệp với { $addresses }. Thay vào đó, bất kỳ ai có liên kết đều có thể mở chúng.
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-onedrive-card-name = OneDrive
compose-drop-files = Thả tệp vào đây
compose-drop-here = Thả vào đây

## Paste options (a small bar under what was just pasted or dropped)

compose-paste-keep-formatting = Giữ định dạng
compose-paste-table = Bảng
compose-paste-picture = Ảnh
compose-paste-plain-text = Văn bản thuần
compose-paste-inline = Trong nội dung
compose-paste-attachment = Tệp đính kèm

## Encryption and signing (the toggles by the recipients)

compose-encrypt = Mã hóa
compose-encrypted = Đã mã hóa: chỉ người nhận đọc được
compose-sign = Ký
compose-signed = Đã ký: người nhận có thể kiểm tra thư là của bạn

## Open and click tracking and read receipts (toggles after Sign)

compose-track = Theo dõi lượt mở và lượt nhấp
compose-tracked = Đang theo dõi: bạn thấy khi mỗi người nhận mở thư hoặc mở một liên kết
compose-track-clicks = Theo dõi lượt nhấp liên kết (văn bản thuần không thể hiển thị lượt mở)
compose-tracked-clicks = Đang theo dõi: bạn thấy khi mỗi người nhận mở một liên kết
compose-track-sign-in = Đăng nhập tài khoản Katna để theo dõi lượt mở và lượt nhấp
compose-receipt = Yêu cầu xác nhận đã đọc
compose-receipt-on = Đã yêu cầu xác nhận đã đọc: ứng dụng của người nhận có thể hỏi họ có gửi xác nhận không
compose-delivery = Yêu cầu xác nhận đã chuyển phát
compose-delivery-on = Đã yêu cầu xác nhận đã chuyển phát: máy chủ thư của bạn sẽ gửi email cho bạn khi máy chủ của mỗi người nhận chấp nhận thư
compose-delivery-unavailable = Máy chủ thư của bạn không gửi xác nhận đã chuyển phát

## Spelling

spell-no-dictionary = Chưa cài từ điển chính tả cho { $language } (ví dụ hunspell-en_us).
spell-dictionary-error = Từ điển chính tả: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = Thêm “{ $words }”
grammar-remove = Xóa “{ $words }”
grammar-ignore = Bỏ qua

## Send checks (asked before a message goes out)

send-check-attachment-title = Bạn có định đính kèm tệp không?
send-check-attachment-text = Bạn có nhắc đến tệp đính kèm, nhưng chưa đính kèm gì.
send-check-attach = Đính kèm tệp
send-check-subject-title = Gửi mà không có tiêu đề?
send-check-subject-text = Thư này không có tiêu đề.
send-check-add-subject = Thêm tiêu đề
send-check-send-anyway = Vẫn gửi

## Recipients (To, Cc and Bcc)

recipient-not-valid = Không phải địa chỉ email hợp lệ
recipient-show-address = Hiện địa chỉ
recipient-remove = Xóa
recipient-bad-title = Kiểm tra địa chỉ
recipient-bad-text = “{ $address }” không phải là địa chỉ email hợp lệ. Hãy sửa hoặc xóa nó trước khi gửi.
recipient-bad-fix = Sửa
