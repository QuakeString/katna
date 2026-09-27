# Katna Mail, Vietnamese (Tiếng Việt).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Ngôn ngữ: { $language }
language-tooltip-system = Ngôn ngữ: { $language }, theo hệ thống
language-search = Tìm ngôn ngữ
language-system-default = Mặc định của hệ thống
language-system-now = Hiện là { $language }
language-no-match = Không có ngôn ngữ nào khớp với “{ $query }”
language-machine = Bản dịch máy. Hãy giúp cải thiện
language-setting = Ngôn ngữ
language-setting-detail = Ngôn ngữ của menu, nút và thông báo, cùng định dạng ngày và số. Mặc định của hệ thống sẽ theo cài đặt của môi trường máy tính.

## Dates and sizes

ago-just-now = vừa xong
ago-minutes = { $count } phút trước
ago-hours = { $count } giờ trước
ago-days = { $count } ngày trước
size-bytes = { $count } byte
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = Ẩn thư mục
folders-show = Hiện thư mục
compose = Soạn thư
search = Tìm kiếm
search-mail = Tìm trong thư
search-settings = Tìm trong cài đặt
search-clear = Xóa nội dung tìm kiếm
search-options-show = Hiện tùy chọn tìm kiếm
settings = Cài đặt
account-add = Thêm tài khoản

## App rail (and the bottom bar on a phone)

rail-mail = Thư
rail-calendar = Lịch
rail-contacts = Danh bạ
rail-tasks = Việc cần làm
rail-notes = Ghi chú
rail-feeds = Nguồn cấp

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Sắp ra mắt
app-calendar-promise = Lịch CalDAV, lời mời họp trong thư và lời nhắc của bạn, ngay bên cạnh hộp thư đến.
app-tasks-promise = Danh sách việc cần làm đồng bộ với CalDAV, và việc cần làm tạo từ thư.
app-notes-promise = Ghi chú nhanh, và ghi chú trên thư hoặc cuộc hội thoại để xem lại sau.
app-feeds-promise = Đọc nguồn cấp RSS và Atom ngay bên cạnh thư của bạn.

## Contacts page

app-contacts-loading = Đang tập hợp mọi người từ thư của bạn…
app-contacts-empty = Những người bạn trao đổi thư sẽ hiện ở đây.
app-contacts-count = { $count } người từ thư của bạn, người trao đổi nhiều nhất ở trên cùng
app-contacts-top = { $count } người hàng đầu từ thư của bạn, người trao đổi nhiều nhất ở trên cùng
app-contacts-messages = { $count } thư
app-contacts-last = lần cuối { $date }

## Navigation (the folders pane)

nav-labels = Nhãn
nav-folders = Thư mục
nav-label-new = Tạo nhãn mới
nav-folder-new = Tạo thư mục mới
nav-account-unnamed = Tài khoản { $number }
nav-tab-new = { $count } thư mới

## Special folders (the user's own folders keep their names)

folder-inbox = Hộp thư đến
folder-starred = Có gắn dấu sao
folder-drafts = Thư nháp
folder-sent = Đã gửi
folder-archive = Lưu trữ
folder-spam = Thư rác
folder-trash = Thùng rác
folder-all-mail = Tất cả thư
folder-scheduled = Đã lên lịch

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Nhãn mới
label-folder-new-title = Thư mục mới
label-prompt = Vui lòng nhập tên nhãn mới:
label-folder-prompt = Vui lòng nhập tên thư mục mới:
label-name-hint = Tên nhãn
label-folder-name-hint = Tên thư mục
label-nest = Lồng nhãn dưới:
label-folder-nest = Lồng thư mục dưới:
label-cancel = Hủy
label-create = Tạo
label-creating = Đang tạo…
label-created = Đã tạo nhãn “{ $name }”.
label-folder-created = Đã tạo thư mục “{ $name }”.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Chính
tab-promotions = Quảng cáo
tab-social = Mạng xã hội
tab-updates = Cập nhật
tab-forums = Diễn đàn
tab-focused = Ưu tiên
tab-other = Khác
tab-inbox = Hộp thư đến
tab-newsletters = Bản tin
tab-notifications = Thông báo
tab-new = { $count } thư mới
tab-provider-other = do Katna sắp xếp

## Mail list: toolbar

list-select = Chọn
list-refresh = Làm mới
list-more = Thêm
list-mark-read = Đánh dấu là đã đọc
list-mark-unread = Đánh dấu là chưa đọc
list-move-to = Di chuyển tới
list-archive = Lưu trữ
list-spam = Báo cáo thư rác
list-delete = Xóa
list-newer = Mới hơn
list-older = Cũ hơn
list-range = { $first }–{ $last } trong số { $total }
list-range-about = { $first }–{ $last } trong số khoảng { $total }
list-results = Kết quả cho “{ $query }”
list-results-corrected = Đang hiển thị kết quả cho “{ $query }”
list-search-instead = Thay vào đó, tìm “{ $query }”
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Tất cả
list-pick-none = Không chọn
list-pick-read = Đã đọc
list-pick-unread = Chưa đọc
list-pick-starred = Có gắn dấu sao
list-pick-unstarred = Không có dấu sao

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] Đã chọn tất cả { $count } cuộc hội thoại.
   *[message] Đã chọn tất cả { $count } thư.
}
list-selected-all-in = { $kind ->
    [conversation] Đã chọn tất cả { $count } cuộc hội thoại trong { $folder }.
   *[message] Đã chọn tất cả { $count } thư trong { $folder }.
}
list-selected-screen = { $kind ->
    [conversation] Đã chọn tất cả { $count } cuộc hội thoại trên trang này.
   *[message] Đã chọn tất cả { $count } thư trên trang này.
}
list-select-all = { $kind ->
    [conversation] Chọn tất cả { $count } cuộc hội thoại
   *[message] Chọn tất cả { $count } thư
}
list-select-all-in = { $kind ->
    [conversation] Chọn tất cả { $count } cuộc hội thoại trong { $folder }
   *[message] Chọn tất cả { $count } thư trong { $folder }
}
list-clear-selection = Bỏ chọn

## Mail list: empty states

list-empty-search = Không có thư nào khớp với nội dung tìm kiếm của bạn.
list-empty-tab = Không có thư nào trong { $tab }.
list-empty-tab-unknown = Không có thư nào trong tab này.
list-empty-folder = Không có thư nào trong { $folder }.
list-empty-folder-unknown = Không có thư nào trong thư mục này.
list-first-sync = Đang tải thư của bạn…
list-first-sync-detail = Thư sẽ hiện ở đây khi được tải về.

## Mail list: lines

row-removed = Thư này đã bị xóa.
row-starred = Có gắn dấu sao
row-not-starred = Không có dấu sao
row-important = Quan trọng. Nhấp để đánh dấu là không quan trọng.
row-mark-important = Đánh dấu là quan trọng
row-pinned = Đã ghim lên đầu
row-pin = Ghim lên đầu
row-unpin = Bỏ ghim

## Mail list: More menu and right-click menu

menu-reply = Trả lời
menu-reply-all = Trả lời tất cả
menu-forward = Chuyển tiếp
menu-archive = Lưu trữ
menu-delete = Xóa
menu-spam = Báo cáo thư rác
menu-mark-read = Đánh dấu là đã đọc
menu-mark-unread = Đánh dấu là chưa đọc
menu-mark-all-read = Đánh dấu tất cả là đã đọc
menu-star = Thêm dấu sao
menu-unstar = Xóa dấu sao
menu-important = Đánh dấu là quan trọng
menu-not-important = Đánh dấu là không quan trọng
menu-pin = Ghim lên đầu
menu-unpin = Bỏ ghim
menu-print-all = In tất cả
menu-new-window = Mở trong cửa sổ mới
menu-move-to = Di chuyển tới
menu-move-to-heading = Di chuyển tới:
menu-find-from = Tìm email từ { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] Đã lưu trữ { $count } cuộc hội thoại.
   *[message] Đã lưu trữ { $count } thư.
}
toast-trashed = { $kind ->
    [conversation] Đã chuyển { $count } cuộc hội thoại vào Thùng rác.
   *[message] Đã chuyển { $count } thư vào Thùng rác.
}
toast-moved = { $kind ->
    [conversation] Đã di chuyển { $count } cuộc hội thoại.
   *[message] Đã di chuyển { $count } thư.
}
toast-starred = { $kind ->
    [conversation] Đã gắn dấu sao cho { $count } cuộc hội thoại.
   *[message] Đã gắn dấu sao cho { $count } thư.
}
toast-unstarred = { $kind ->
    [conversation] Đã xóa dấu sao khỏi { $count } cuộc hội thoại.
   *[message] Đã xóa dấu sao khỏi { $count } thư.
}
toast-important = { $kind ->
    [conversation] Đã đánh dấu { $count } cuộc hội thoại là quan trọng.
   *[message] Đã đánh dấu { $count } thư là quan trọng.
}
toast-not-important = { $kind ->
    [conversation] Đã đánh dấu { $count } cuộc hội thoại là không quan trọng.
   *[message] Đã đánh dấu { $count } thư là không quan trọng.
}
toast-pinned = { $kind ->
    [conversation] Đã ghim { $count } cuộc hội thoại lên đầu.
   *[message] Đã ghim { $count } thư lên đầu.
}
toast-unpinned = { $kind ->
    [conversation] Đã bỏ ghim { $count } cuộc hội thoại.
   *[message] Đã bỏ ghim { $count } thư.
}
toast-spam = { $kind ->
    [conversation] Đã báo cáo { $count } cuộc hội thoại là thư rác.
   *[message] Đã báo cáo { $count } thư là thư rác.
}
toast-deleted-forever = { $kind ->
    [conversation] Đã xóa vĩnh viễn { $count } cuộc hội thoại.
   *[message] Đã xóa vĩnh viễn { $count } thư.
}
toast-undone = Đã hoàn tác thao tác.
toast-undo = Hoàn tác
toast-no-spam-folder = Tài khoản này không có thư mục thư rác.

## Reading pane: toolbar

reader-close = Đóng
reader-back = Quay lại
reader-mark-unread = Đánh dấu là chưa đọc
reader-move-to = Di chuyển tới
reader-more = Thêm
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
reader-me = tôi
reader-to = tới { $names }
reader-starred = Có gắn dấu sao
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

## Remote images and pictures

remote-hidden = Hình ảnh trong thư này đang bị ẩn.
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
attachment-save-all = Lưu tất cả
attachment-save-all-tooltip = Lưu mọi tệp đính kèm vào một thư mục
attachment-save-here = Lưu tại đây
attachment-not-downloaded = Thư này chưa được tải xuống.
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
print-not-downloaded = (Chưa được tải xuống.)
print-encrypted = (Đã mã hóa. Hãy mở trong Katna Mail để in nội dung.)
print-to = Tới: { $addresses }
print-cc = Cc: { $addresses }
