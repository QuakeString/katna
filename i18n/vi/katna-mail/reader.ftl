# Katna Mail, Vietnamese (Tiếng Việt).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Đóng
reader-back = Quay lại
reader-mark-unread = Đánh dấu là chưa đọc
reader-move-to = Di chuyển tới
reader-snooze = Tạm ẩn
reader-remind = Nhắc tôi
reader-more = Thêm
reader-original-colors = Hiện màu gốc
reader-dark-colors = Hiện bằng màu tối
reader-print-all = In tất cả
reader-new-window = Mở trong cửa sổ mới
reader-position = { $position } trong số { $total }
reader-newer = Mới hơn
reader-older = Cũ hơn

## Reading pane: the conversation

reader-removed = Cuộc hội thoại này đã bị xóa.
reader-no-subject = (không có tiêu đề)
reader-collapse-all = Thu gọn tất cả
reader-expand-all = Mở rộng tất cả
reader-unknown-sender = (người gửi không xác định)
reader-date-ago = { $date } ({ $ago })
reader-sending = Đang gửi…
reader-me = tôi
reader-to = tới { $names }
reader-to-label = tới
reader-tick-delivered = Đã chuyển phát { $when }
reader-tick-no-bounce = Đã gửi { $when }; không có thư trả về, nên nhiều khả năng thư đã đến
reader-tick-bounced = Không chuyển phát được: bị trả về { $when }
reader-tick-read = Đã đọc { $when } (xác nhận đã đọc)
reader-tick-opened = Đã mở, lần cuối { $when } (theo dõi lượt mở)
reader-starred = Có gắn dấu sao
reader-chip-remove = Xóa { $label }
reader-not-starred = Không có dấu sao
reader-too-long = Thư quá dài nên không thể hiển thị đầy đủ.
reader-encrypted-images = Hình ảnh từ web không bao giờ được tải trong thư đã mã hóa.
reader-window-failed = Không thể mở cửa sổ mới.

## Reading pane: message details (opened from "to me")

reader-details-from = từ:
reader-details-to = tới:
reader-details-cc = cc:
reader-details-date = ngày:
reader-details-subject = tiêu đề:

## Reading pane: downloading a message

reader-downloading = Đang tải thư này xuống từ máy chủ…
reader-download-failed = Không thể tải thư này xuống.
reader-download-failed-reason = Không thể tải xuống thư này. { $reason }
reader-download-offline = Tài khoản này đang ngoại tuyến. Hãy kết nối mạng để tải thư này xuống.
reader-try-again = Thử lại

## Reply row

reply-reply = Trả lời
reply-reply-all = Trả lời tất cả
reply-forward = Chuyển tiếp

## Encrypted and signed mail

security-decrypting = Đang giải mã…
security-checking = Đang kiểm tra chữ ký…
security-partly-encrypted = Chỉ một phần của thư này được mã hóa. Phần còn lại được thêm vào bên ngoài lớp bảo vệ và có thể đến từ bất kỳ ai.
security-partly-signed = Chỉ một phần của thư này được ký. Phần còn lại được thêm vào bên ngoài lớp bảo vệ và có thể đến từ bất kỳ ai.
security-encrypted = Thư đã mã hóa
security-encrypted-smime = Thư đã mã hóa (S/MIME)
security-no-key = Không thể giải mã thư này: thư được mã hóa cho một khóa mà bạn không có.
security-cancelled = Đã hủy giải mã.
security-damaged = Không thể giải mã thư này: dữ liệu mã hóa bị hỏng hoặc đã bị thay đổi.
security-decrypt-unavailable = Không thể giải mã thư này: hãy cài đặt { $tool } để đọc thư đã mã hóa.
security-decrypt-failed = Không thể giải mã thư này: { $reason }
security-unknown-signer = một người ký không xác định
security-signed-verified = Được ký bởi { $signer } · đã xác minh
security-signed-not-sender = Được ký bởi { $signer }, không phải người gửi
security-signed-untrusted = Được ký bởi { $signer }, bằng khóa bạn đã đánh dấu là không tin cậy
security-signed-unverified = Được ký bởi { $signer } · khóa chưa được xác minh
security-bad-signature = Chữ ký không hợp lệ: thư này đã bị thay đổi sau khi ký, hoặc chữ ký là giả mạo.
security-signature-expired = Được ký bởi { $signer } · chữ ký đã hết hạn
security-key-expired = Được ký bởi { $signer } · khóa đã hết hạn từ đó
security-key-revoked = Được ký bởi { $signer } bằng khóa đã bị thu hồi
security-missing-key = Được ký bằng khóa mà bạn không có, nên không thể kiểm tra
security-missing-key-id = Được ký bằng khóa mà bạn không có ({ $key }), nên không thể kiểm tra
security-signature-unavailable = Đã ký; hãy cài đặt { $tool } để kiểm tra chữ ký
security-signature-error = Không thể kiểm tra chữ ký.
security-look-up-key = Tra cứu khóa

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = Chữ ký đã xác minh
key-card-verified-detail = Chữ ký hợp lệ và bạn tin cậy khóa này.
key-card-unverified = Chữ ký chưa được xác minh
key-card-unverified-detail = Chữ ký hợp lệ, nhưng không có gì xác nhận khóa này là của họ. Hãy so dấu vân tay với họ, rồi tin cậy khóa trong GnuPG (Kleopatra hoặc gpg --edit-key).
key-card-not-sender = Được ký bởi người khác
key-card-not-sender-detail = Chữ ký hợp lệ, nhưng khóa không phải của người gửi.
key-card-untrusted = Khóa không tin cậy
key-card-untrusted-detail = Bạn đã đánh dấu khóa này là không tin cậy trong GnuPG.
key-card-signature-expired = Chữ ký đã hết hạn
key-card-signature-expired-detail = Chữ ký từng hợp lệ, nhưng đã hết hạn.
key-card-key-expired = Khóa đã hết hạn
key-card-key-expired-detail = Chữ ký hợp lệ, nhưng khóa đã hết hạn từ đó.
key-card-key-revoked = Khóa đã bị thu hồi
key-card-key-revoked-detail = Chủ sở hữu đã thu hồi khóa này, nên không thể tin cậy chữ ký.
key-card-bad = Chữ ký không hợp lệ
key-card-bad-detail = Thư này đã bị thay đổi sau khi ký, hoặc chữ ký là giả mạo.
key-card-signed-by = Được ký bởi
key-card-belongs-to = Thuộc về
key-card-fingerprint = Dấu vân tay
key-card-signed = Đã ký
key-card-key = Khóa
key-card-kind = { $standard }, { $algorithm }
key-card-created = Ngày tạo
key-card-expires = Hết hạn
key-card-never = Không bao giờ
key-card-issued-by = Cấp bởi
key-card-found-in = Tìm thấy trong
key-card-keyring = Chùm khóa GnuPG của bạn
key-card-copy = Sao chép dấu vân tay
key-card-import-title = Nhập khóa này?
key-card-from-directory = Tìm thấy trong thư mục khóa của { $domain }.
key-card-from-attachment = Từ tệp đính kèm { $name }.
key-card-import-note = Sau đó Katna có thể kiểm tra chữ ký của người này và mã hóa thư gửi cho họ. Để tin cậy hoàn toàn khóa này, hãy so dấu vân tay với họ.
key-card-cancel = Hủy
key-card-import = Nhập khóa
key-card-looking-up = Đang tra cứu khóa…
key-card-looking-up-detail = Đang hỏi thư mục khóa của { $domain }.
key-card-not-found = Không tìm thấy khóa
key-card-not-found-detail = { $domain } không công bố khóa cho địa chỉ này. Hãy đề nghị người gửi gửi khóa của họ cho bạn.
key-card-not-kept = Không thể dùng khóa đã tìm thấy.
key-card-failed = Không thể lấy khóa

## Sender checks: the banner on mail that failed the checks the user's mail
## provider ran on its sender (DMARC, DKIM, SPF), and the "?" on the picture
## of a sender nothing confirmed

sender-failed-title = Thư này có thể không phải từ { $domain }
sender-failed-body = Thư không vượt qua kiểm tra người gửi của { $provider }. Hãy cẩn thận với liên kết, tệp đính kèm và khi trả lời.
sender-provider-unknown = nhà cung cấp thư của bạn
sender-details = Chi tiết
sender-details-hide = Ẩn chi tiết
sender-looks-safe = Có vẻ an toàn
sender-move-to-spam = Chuyển vào thư rác
sender-checked-by = Được kiểm tra bởi { $provider }
sender-checked-by-server = Được kiểm tra bởi { $provider } ({ $server })
sender-dmarc = Tên miền người gửi (DMARC)
sender-dkim = Chữ ký (DKIM)
sender-spf = Máy chủ gửi (SPF)
sender-result-pass = Đạt
sender-result-fail = Không đạt
sender-result-unsure = Không chắc chắn
sender-result-none = Không có
sender-result-missing = Chưa kiểm tra
sender-dmarc-pass = { $domain } xác nhận người gửi này.
sender-dmarc-fail = Thư không khớp với cách { $domain } cho biết thư của họ được gửi.
sender-dmarc-none = { $domain } không công bố quy tắc nào cho thư của họ.
sender-dkim-pass = Được ký bởi { $domain }.
sender-dkim-fail = Chữ ký từ { $domain } không khớp với thư.
sender-dkim-none = Thư không được ký.
sender-spf-pass = Được gửi từ một máy chủ mà { $domain } liệt kê.
sender-spf-fail = Được gửi từ một máy chủ mà { $domain } không liệt kê.
sender-spf-none = { $domain } không liệt kê các máy chủ của họ.
sender-check-unsure = Việc kiểm tra không đưa ra được câu trả lời rõ ràng.
sender-unconfirmed = Theo { $provider }, không thể xác nhận thư này đến từ { $domain }. Ai cũng có thể ghi bất kỳ người gửi nào.
sender-link-title = Mở liên kết này?
sender-link-body = Thư này không vượt qua kiểm tra người gửi. Liên kết dẫn tới { $host }:
sender-link-cancel = Hủy
sender-link-open = Mở

## Open and click tracking and read receipts (the eye's popover beside a
## sent message's star, and the line above a read receipt)

tracking-opened = { $who } đã mở thư { $count } lần, lần cuối { $when }
tracking-opens-clicks = { $who } đã mở thư { $opens } lần và mở liên kết { $clicks } lần, lần cuối { $when }
tracking-clicked = { $who } đã mở liên kết { $clicks } lần, lần cuối { $when }
tracking-maybe-opened = { $who } có thể đã mở thư (Apple Mail tải hình ảnh để bảo vệ quyền riêng tư)
tracking-seen-none = Chưa có ai mở thư hoặc mở liên kết
tracking-receipt = { $who } đã gửi xác nhận đã đọc
tracking-receipt-read = { $who } đã đọc (xác nhận đã đọc), { $when }
tracking-receipt-displayed = Xác nhận đã đọc: { $who } đã mở thư của bạn
tracking-receipt-other = Xác nhận đã đọc: { $who } đã xóa hoặc xử lý thư của bạn mà không mở

## Remote images and pictures

remote-hidden = Hình ảnh trong thư này đang bị ẩn.
remote-hidden-unconfirmed = Hình ảnh đang bị ẩn: không thể xác nhận người gửi.
remote-hidden-failed = Hình ảnh đã bị ẩn: thư này không vượt qua kiểm tra người gửi.
remote-show = Hiển thị hình ảnh
remote-always-show = Luôn hiển thị hình ảnh từ người gửi này
remote-picture-use = Dùng
remote-picture-too-big = Hãy chọn ảnh có dung lượng tối đa 8 MB.
remote-picture-type = Hãy chọn ảnh PNG, JPEG, GIF, WebP hoặc SVG.
remote-picture-read-failed = Không thể đọc ảnh: { $error }
remote-picture-keep-failed = Không thể lưu ảnh: { $error }
remote-picture-remove-failed = Không thể xóa ảnh: { $error }

## Attachments

attachment-count = { $count } tệp đính kèm
attachment-save = Lưu
attachment-forward = Chuyển tiếp
attachment-save-all = Lưu tất cả
attachment-save-all-tooltip = Lưu mọi tệp đính kèm vào một thư mục
attachment-save-here = Lưu tại đây
attachment-not-downloaded = Thư này chưa được tải xuống.
attachment-open-message = Mở thư này để đọc tệp đính kèm.
attachment-not-found = Không tìm thấy tệp đính kèm này trong thư.
attachment-read-failed = Không thể đọc { $name }
attachment-numbered = tệp đính kèm { $number }
attachment-saved-all = Đã lưu { $count } tệp vào { $place }
attachment-saved-some = Đã lưu { $saved } trong số { $total } tệp vào { $place }. Không thể lưu { $failed }
attachment-saved-to = Đã lưu vào { $path }
attachment-save-failed = Không thể lưu { $name }: { $error }
attachment-open-failed = Không thể mở { $name }: { $error }
attachment-risky = Tệp này có thể chạy một chương trình nên Katna không mở nó. Hãy lưu tệp thay vì mở.
attachment-encrypted-open = Tệp này được gửi ở dạng mã hóa. Hãy lưu lại để mở ở nơi khác.

## Printing

print-failed = Không thể in: { $error }
print-no-font = không tìm thấy phông chữ nào
print-opened-as-pdf = Đã mở dưới dạng PDF để in từ đó.
print-preview-title = Xem trước khi in
print-preview-laying-out = Đang dàn trang…
print-preview-pages = { $count } trang
print-preview-more = và { $count } trang nữa
print-preview-failed = không thể hiển thị các trang
print-preview-paper = Giấy
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-layout = Bố cục
print-preview-as-shown = Như đang hiển thị
print-preview-simple = Chỉ văn bản
print-preview-backgrounds = Màu nền
print-preview-cancel = Hủy
print-preview-print = In
print-not-downloaded = (Chưa được tải xuống.)
print-encrypted = (Đã mã hóa. Hãy mở trong Katna Mail để in nội dung.)
print-to = Tới: { $addresses }
print-cc = Cc: { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = Ghim lên đầu
text-copy-address = Sao chép địa chỉ
text-copy = Sao chép
text-select-all = Chọn tất cả
