# Katna Mail, Vietnamese (Tiếng Việt).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Quy tắc
settings-rules-summary = Tự động sắp xếp, gắn nhãn, chuyển tiếp hoặc tắt thông báo thư mới
settings-rules-intro = Quy tắc tự động sắp xếp thư mới, theo thứ tự này. Kéo để sắp xếp lại.
settings-rules-all-accounts = Tất cả tài khoản
settings-rules-new = Quy tắc mới
settings-rules-none = Chưa có quy tắc nào. Quy tắc tự động sắp xếp thư mới: theo người gửi, tiêu đề hoặc từ ngữ.
settings-rules-none-account = Tài khoản này chưa có quy tắc nào.
settings-rules-drag = Kéo để sắp xếp lại
settings-rules-edit = Sửa quy tắc
settings-rules-turn-off = Tắt quy tắc này
settings-rules-turn-on = Bật quy tắc này

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = Quy tắc mẫu
settings-rules-starters-intro = Tắt cho đến khi bạn bật. Chúng hoạt động với mọi tài khoản của bạn; hãy sửa một quy tắc để thay đổi nó.
settings-rules-starter-turning-on = Đang bật “{ $name }”…
settings-rules-starter-failed = Không thể bật “{ $name }”: { $error }
rules-starter-promotions = Im lặng thư quảng cáo
rules-starter-newsletters = Bản tin vào Đọc sau
rules-starter-receipts = Biên lai và hóa đơn
rules-starter-deliveries = Giao hàng
rules-starter-train = Vé tàu
rules-starter-flight = Vé máy bay
rules-starter-codes = Mã dùng một lần
rules-starter-security = Cảnh báo bảo mật
rules-starter-social = Thư mạng xã hội
rules-starter-invites = Lời mời lịch
rules-starter-folder-reading = Đọc sau
rules-starter-folder-receipts = Biên lai
rules-starter-folder-deliveries = Giao hàng
rules-starter-folder-travel = Du lịch
rules-starter-folder-social = Mạng xã hội
rules-runs-katna = Chạy trong Katna
rules-runs-gmail = Chạy trên Gmail
rules-runs-sieve = Chạy trên máy chủ
rules-stopped = Đã dừng
rules-error-folder-gone = Thư mục mà quy tắc này dùng không còn tồn tại. Hãy sửa quy tắc để chọn thư mục khác.
rules-error-no-archive = Tài khoản này không có thư mục lưu trữ. Hãy sửa quy tắc để làm việc khác.
rules-error-no-trash = Tài khoản này không có thư mục Thùng rác. Hãy sửa quy tắc để làm việc khác.
rules-error-cannot-send = Tài khoản này không gửi được thư, nên quy tắc không thể chuyển tiếp.
rules-error-other = { $error }. Hãy sửa quy tắc và bật lại.
settings-folders = Thư mục
settings-folders-summary = Số thư chưa đọc trong ngăn thư mục
settings-folders-unread-counts = Số thư chưa đọc trên mọi thư mục
settings-folders-unread-counts-detail = Khi tắt: chỉ Hộp thư đến hiện số thư chưa đọc

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } và { $next }
rules-summary-or = { $first } hoặc { $next }
rules-summary-more = { $count } từ khác
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Có tệp đính kèm
rules-summary-no-attachment = Không có tệp đính kèm
rules-summary-mailing-list = Từ một danh sách thư
rules-summary-not-mailing-list = Không từ danh sách thư
rules-summary-tab = Trong tab { $tab }
rules-summary-not-tab = Không trong tab { $tab }
rules-summary-move = chuyển vào { $folder }
rules-summary-archive = bỏ qua hộp thư đến
rules-summary-trash = chuyển vào thùng rác
rules-summary-mark-read = đánh dấu là đã đọc
rules-summary-star = gắn dấu sao
rules-summary-important = đánh dấu là quan trọng
rules-summary-label = gắn nhãn { $label }
rules-summary-forward = chuyển tiếp tới { $address }
rules-summary-dont-notify = không thông báo
rules-summary-read-after = { $count ->
   *[other] đánh dấu là đã đọc sau { $count } ngày
}
rules-summary-folder-gone = một thư mục không còn nữa

## The rule editor

rules-editor-new-title = Quy tắc mới
rules-editor-edit-title = Sửa quy tắc
rules-editor-name-hint = Tên quy tắc
rules-editor-when = Khi thư mới khớp với
rules-editor-of-these = điều kiện sau:
rules-mode-all = tất cả
rules-mode-any = bất kỳ
rules-field-from = Từ
rules-field-to = Tới
rules-field-cc = Cc
rules-field-any-recipient = Tới hoặc Cc
rules-field-reply-to = Trả lời tới
rules-field-subject = Tiêu đề
rules-field-body = Nội dung
rules-field-attachment-name = Tên tệp đính kèm
rules-field-has-attachment = Có tệp đính kèm
rules-field-mailing-list = Từ một danh sách thư
rules-field-tab = Tab hộp thư đến
rules-comparator-contains = chứa
rules-comparator-not-contains = không chứa
rules-comparator-begins-with = bắt đầu bằng
rules-comparator-ends-with = kết thúc bằng
rules-comparator-equals = chính xác là
rules-comparator-matches = khớp với mẫu
rules-has-yes = có
rules-has-no = không
rules-editor-value-hint = Từ ngữ hoặc địa chỉ
rules-editor-add-condition = Thêm điều kiện
rules-editor-remove = Xóa
rules-editor-then = Thì:
rules-action-move = Chuyển vào
rules-action-archive = Bỏ qua hộp thư đến (lưu trữ)
rules-action-trash = Chuyển vào thùng rác
rules-action-mark-read = Đánh dấu là đã đọc
rules-action-star = Gắn dấu sao
rules-action-important = Đánh dấu là quan trọng
rules-action-label = Thêm nhãn
rules-action-forward = Chuyển tiếp tới
rules-action-dont-notify = Không thông báo
rules-action-read-after = Đánh dấu là đã đọc sau
rules-editor-choose-folder = Chọn thư mục
rules-editor-choose-label = Chọn nhãn
rules-editor-new-folder = Mới: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = Địa chỉ email
rules-editor-days = ngày
rules-editor-add-action = Thêm hành động
rules-editor-stop = Dừng tại đây: các quy tắc sau không chạy trên thư này
rules-editor-accounts = Tài khoản:
rules-editor-accounts-none = Chọn tài khoản
rules-editor-accounts-many = { $count ->
   *[other] { $count } tài khoản
}
rules-editor-matches = Khớp với { $mails } trong { $days } ngày qua
rules-editor-mails = { $count ->
   *[other] { $count } thư
}
rules-editor-counting = Đang đếm số thư khớp…
rules-editor-show = Xem các thư đó
rules-editor-also-apply = Áp dụng cả cho { $count } thư này
rules-editor-runs-katna = Chạy trong Katna, khi máy tính này đang bật.
rules-editor-runs-gmail = Chạy trên Gmail, nên cũng hoạt động trên điện thoại của bạn và khi máy tính này tắt.
rules-editor-runs-sieve = Chạy trên máy chủ thư của bạn, nên cũng hoạt động trên điện thoại của bạn và khi máy tính này tắt.
rules-note-gmail-action = Chạy trong Katna: bộ lọc Gmail không thể “{ $action }”.
rules-note-sieve-action = Chạy trong Katna: quy tắc của máy chủ thư không thể “{ $action }”.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Chạy trong Katna: bộ lọc Gmail không thể kiểm tra “{ $test }” như Katna.
rules-note-sieve-condition = Chạy trong Katna: quy tắc của máy chủ thư không thể kiểm tra “{ $test }” như Katna.
rules-note-order = Chạy trong Katna, giống một quy tắc trước đó của tài khoản: các quy tắc chạy theo thứ tự trong danh sách.
rules-note-gmail-stop = Chạy trong Katna: bộ lọc Gmail không thể ngăn các quy tắc sau chạy.
rules-note-gmail-forward = Chạy trong Katna: Gmail chỉ chuyển tiếp tới các địa chỉ đã xác minh trong cài đặt của nó, và { $address } không nằm trong số đó.
rules-note-gmail-folder = Chạy trong Katna: Gmail không có nhãn cho một thư mục mà quy tắc này dùng.
rules-note-sieve-folder = Chạy trong Katna: máy chủ thư của bạn không có một thư mục mà quy tắc này dùng.
rules-note-gmail-sign-in = Chạy trong Katna cho đến khi bạn đăng nhập lại Google và cho phép Katna tạo bộ lọc Gmail.
rules-note-sieve-other-script = Chạy trong Katna: một tập lệnh quy tắc khác (“{ $name }”) đang hoạt động trên máy chủ thư của bạn.
rules-note-gmail-failed = Chạy trong Katna: Gmail không chấp nhận ({ $error }).
rules-note-sieve-failed = Chạy trong Katna: máy chủ thư của bạn không chấp nhận ({ $error }).
rules-editor-cancel = Hủy
rules-editor-save = Lưu
rules-editor-saving = Đang lưu…
rules-editor-delete = Xóa quy tắc
rules-editor-delete-ask = Xóa quy tắc này?
rules-editor-delete-keep = Giữ lại
rules-editor-delete-confirm = Xóa
rules-editor-needs-folder = Hãy chọn một thư mục cho mỗi “Chuyển vào” và một nhãn cho mỗi “Thêm nhãn”.
rules-editor-needs-days = “Đánh dấu là đã đọc sau” cần một số ngày, từ 1 đến 3650.
rules-saved = Đã lưu quy tắc
rules-saved-applied = { $count ->
   *[other] Đã lưu quy tắc và áp dụng cho { $count } thư
}
rules-apply-failed = Đã lưu quy tắc, nhưng áp dụng thất bại: { $error }
rules-deleted = Đã xóa quy tắc
rules-delete-failed = Không thể xóa quy tắc: { $error }
rules-change-failed = Không thể thay đổi quy tắc: { $error }
