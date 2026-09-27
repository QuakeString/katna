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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Mở thư này để đọc tệp đính kèm.
text-copy = Sao chép
text-select-all = Chọn tất cả

## Settings page: its tabs

settings-tab-general = Chung
settings-tab-inbox = Hộp thư đến
settings-tab-accounts = Tài khoản
settings-tab-subscriptions = Gói đăng ký
settings-tab-appearance = Giao diện
settings-tab-shortcuts = Phím tắt
settings-tab-default-apps = Ứng dụng mặc định
settings-tab-folders-rules = Thư mục và quy tắc
settings-tab-compose = Soạn thư
settings-tab-mcp-server = Máy chủ MCP
settings-tab-feedback = Ý kiến phản hồi
settings-tab-experimental = Thử nghiệm

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Xem các bản tin và danh sách gửi thư bạn nhận được, và hủy đăng ký chỉ bằng một lần nhấp.
settings-tab-folders-rules-coming = Tạo, đổi tên, di chuyển và ẩn thư mục và nhãn, và chọn những mục được đồng bộ. Quy tắc tự động sắp xếp, gắn nhãn, chuyển tiếp hoặc xóa thư mới theo người gửi, tiêu đề hoặc từ ngữ.
settings-tab-mcp-server-coming = Cho phép trợ lý AI trên máy tính này tìm kiếm, đọc và soạn nháp thư của bạn, khi bạn đồng ý.

## Settings > General

settings-general-conversations = Chế độ xem cuộc hội thoại
settings-general-conversations-group = Nhóm các thư trả lời cho cùng một thư
settings-general-conversations-group-detail = Mỗi cuộc hội thoại một dòng trong danh sách
settings-general-reading = Đọc thư
settings-general-newest-first = Thư mới nhất trước
settings-general-newest-first-detail = Cuộc hội thoại bắt đầu bằng thư trả lời mới nhất
settings-general-full-headers = Hiện đầy đủ phần đầu thư
settings-general-full-headers-detail = Từ, tới, cc, ngày và tiêu đề hiện trên mọi thư
settings-general-full-names = Tên đầy đủ của người nhận
settings-general-full-names-detail = “tới tôi, Ada Lovelace” thay vì “tới tôi, Ada”
settings-general-mark-read = Đánh dấu là đã đọc
settings-general-mark-read-now = Ngay khi mở
settings-general-mark-read-1s = Sau khi mở 1 giây
settings-general-mark-read-3s = Sau khi mở 3 giây
settings-general-mark-read-never = Chỉ khi tôi đánh dấu là đã đọc
settings-general-reply-button = Nút trả lời
settings-general-reply-all = Trả lời tất cả mọi người
settings-general-reply-all-detail = Nút trả lời bên cạnh mỗi thư sẽ trả lời tất cả, không chỉ người gửi
settings-general-remote-images = Hình ảnh từ web
settings-general-remote-images-detail = Việc tải hình ảnh của một thư cho người gửi biết bạn đã mở thư, lúc nào và ở khoảng nơi nào. Khi tắt, mỗi thư sẽ hỏi trước, và bạn luôn có thể hiển thị hình ảnh của một người gửi.
settings-general-remote-images-always = Luôn hiển thị hình ảnh
settings-general-remote-images-always-detail = Trong mọi thư, không chỉ từ những người gửi bạn tin cậy
settings-general-sending = Gửi thư
settings-general-sending-detail = Thời gian thư đã gửi chờ trước khi đi, để có thể thu hồi.
settings-general-offline = Thư ngoại tuyến
settings-general-offline-detail = Thư gần đây được tải xuống đầy đủ để đọc khi không có kết nối. Thư cũ hơn được tải xuống khi bạn mở.
settings-general-offline-days = { $count } ngày
settings-general-offline-years = { $count } năm
settings-general-offline-all = Tất cả thư
settings-general-offline-note = Chọn ít ngày hơn vẫn giữ lại thư đã tải xuống. Không có gì thay đổi trên máy chủ.
settings-general-notifications = Thông báo
settings-general-notifications-detail = Cho thư mới trong Hộp thư đến, kể cả khi Katna Mail đang đóng.
settings-general-new-mail = Thông báo cho tôi khi có thư mới
settings-general-new-mail-detail = Có Trả lời tất cả, Đánh dấu là đã đọc và Lưu trữ
settings-general-new-mail-sound = Phát âm thanh
settings-general-new-mail-sound-detail = Âm thanh thư mới của môi trường máy tính
settings-general-desktop = Môi trường máy tính
settings-general-open-at-login = Mở Katna Mail khi đăng nhập
settings-general-open-at-login-detail = Thư vẫn đồng bộ khi đăng nhập dù chọn thế nào, miễn là dịch vụ đang chạy
settings-general-tray = Hiện Katna trong khay hệ thống
settings-general-tray-detail = Kèm số thư chưa đọc và một menu
settings-general-unread-badge = Số thư chưa đọc trên biểu tượng ở thanh tác vụ
settings-general-unread-badge-detail = Số thư chưa đọc trong Hộp thư đến

## Settings > Inbox

settings-inbox-tabs = Tab hộp thư đến
settings-inbox-tabs-detail = Sắp xếp hộp thư đến thành các tab, như trang web của nhà cung cấp thư của bạn.
settings-inbox-tabs-show = Hiện tab hộp thư đến
settings-inbox-tabs-show-detail = Khi tắt, mỗi tài khoản hiển thị một danh sách
settings-inbox-no-accounts = Hãy thêm tài khoản để chọn tab của tài khoản đó.
settings-inbox-tabs-automatic = Tự động: { $tabs } ({ $provider })
settings-inbox-tabs-off = Không có tab
settings-inbox-tabs-gmail = Chính, Quảng cáo, Mạng xã hội, Cập nhật, Diễn đàn
settings-inbox-tabs-focused = Ưu tiên và Khác
settings-inbox-tabs-zoho = Hộp thư đến, Bản tin và Thông báo
settings-inbox-tabs-shown = Các tab được hiển thị. Thư của tab bạn tắt sẽ ở lại trong { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Ngăn đọc
settings-appearance-reading-pane-detail = Nơi hiển thị cuộc hội thoại đang mở.
settings-appearance-pane-right = Bên phải danh sách
settings-appearance-pane-none = Không chia
settings-appearance-density = Mật độ
settings-appearance-density-default = Mặc định
settings-appearance-density-compact = Thu gọn
settings-appearance-scaling = Tỷ lệ
settings-appearance-scaling-detail = Làm mọi thứ trong Katna Mail to hơn hoặc nhỏ hơn, cộng thêm vào tỷ lệ của chính môi trường máy tính: chữ, biểu tượng, khoảng cách và đường phân cách. Thư bạn gửi vẫn giữ cỡ chữ riêng. Kích thước rất nhỏ có thể khiến biểu tượng khó nhấp.
settings-appearance-theme = Chủ đề
settings-appearance-theme-system = Giống môi trường máy tính
settings-appearance-theme-light = Sáng
settings-appearance-theme-dark = Tối
settings-appearance-desktop-colors = Màu của môi trường máy tính
settings-appearance-desktop-colors-use = Dùng màu của môi trường máy tính
settings-appearance-desktop-colors-use-detail = Bảng màu và màu nhấn của môi trường máy tính
settings-appearance-app-names = Tên ứng dụng
settings-appearance-app-names-show = Hiện tên ứng dụng
settings-appearance-app-names-show-detail = Tên dưới biểu tượng ứng dụng ở ngoài cùng bên trái
settings-appearance-sender-pictures = Ảnh người gửi
settings-appearance-sender-pictures-show = Hiện logo công ty
settings-appearance-sender-pictures-show-detail = Tra cứu theo tên miền của người gửi, không bao giờ theo thư, và được giữ trong một tuần
settings-appearance-important = Điểm đánh dấu quan trọng
settings-appearance-important-show = Hiện điểm đánh dấu quan trọng
settings-appearance-important-show-detail = Bên cạnh mỗi thư trong danh sách
settings-appearance-message-width = Chiều rộng thư
settings-appearance-message-width-limit = Giới hạn chiều rộng của thư
settings-appearance-message-width-limit-detail = Dòng dài dễ đọc hơn trong cửa sổ rộng
settings-appearance-mail-colors = Màu thư
settings-appearance-mail-colors-detail = Hầu hết thư được thiết kế cho trang trắng. Với chủ đề tối, màu của thư được đổi sang màu tối dễ đọc; khi tắt, thư giữ màu của người gửi trên trang sáng.
settings-appearance-dark-mail = Dùng màu tối cho cả thư
settings-appearance-dark-mail-detail = Chỉ khi chủ đề đang là tối
settings-appearance-attachment-previews = Xem trước tệp đính kèm
settings-appearance-attachment-previews-show = Hiện bản xem trước của tệp đính kèm
settings-appearance-attachment-previews-show-detail = Một hình nhỏ về nội dung của mỗi tệp trên thẻ của tệp đó

## Settings > Default apps

settings-default-apps-intro = Nơi mở tệp đính kèm khi bạn nhấp vào. Trình xem cũng luôn có thể mở tệp trong ứng dụng khác. Ứng dụng mặc định của môi trường máy tính được đặt trong phần cài đặt riêng của nó.
settings-default-apps-pdf = Tệp PDF
settings-default-apps-pdf-detail = Các trang, có thu phóng.
settings-default-apps-pictures = Hình ảnh
settings-default-apps-pictures-detail = Ảnh chụp (được xoay thẳng), PNG, GIF, WebP, BMP, TIFF và SVG.
settings-default-apps-text = Tệp văn bản
settings-default-apps-text-detail = Văn bản thuần, nhật ký, mã nguồn và các loại văn bản khác.
settings-default-apps-sheets = Bảng tính
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) và CSV.
settings-default-apps-documents = Tài liệu
settings-default-apps-documents-detail = Word (docx) và văn bản OpenDocument (odt).
settings-default-apps-katna = Trình xem của Katna Mail
settings-default-apps-system = Ứng dụng mặc định của môi trường máy tính
settings-default-apps-ask = Hỏi dùng ứng dụng nào mỗi lần
settings-default-apps-after-saving = Sau khi lưu
settings-default-apps-show-folder = Hiện tệp đã lưu trong thư mục của chúng
settings-default-apps-show-folder-detail = Mở trình quản lý tệp với các tệp đính kèm đã lưu được chọn sẵn

## Settings > Compose

settings-compose-send-from = Gửi thư mới từ
settings-compose-send-from-detail = Thư trả lời và chuyển tiếp luôn được gửi từ tài khoản bạn đang dùng.
settings-compose-send-from-current = Tài khoản bạn đang dùng
settings-compose-send-on-replies = Gửi khi trả lời
settings-compose-send-on-replies-detail = Nút Gửi làm gì khi trả lời hoặc chuyển tiếp. Menu bên cạnh nút Gửi có lựa chọn còn lại.
settings-compose-send-plain = Gửi
settings-compose-send-archive = Gửi và lưu trữ
settings-compose-signatures = Chữ ký
settings-compose-signatures-detail = Được thêm bên dưới thư của bạn, sau dòng “--”. Chọn chữ ký khác trong cửa sổ soạn thư.
settings-compose-untitled = Không có tiêu đề
settings-compose-signature-name = Tên, chẳng hạn Công việc
settings-compose-signature-first = Chữ ký của tôi
settings-compose-signature-numbered = Chữ ký { $number }
settings-compose-signature-delete = Xóa
settings-compose-signature-deleted = Đã xóa chữ ký
settings-compose-signature-new = Tạo mới
settings-compose-no-signatures = Chưa có chữ ký nào.
settings-compose-no-signature = Không có chữ ký
settings-compose-for-new-mail = Cho thư mới
settings-compose-for-replies = Cho thư trả lời và chuyển tiếp
settings-compose-for-replies-detail = Trong cuộc hội thoại mà bạn đã ký một thư, thư trả lời sẽ bắt đầu bằng chính chữ ký đó.
settings-compose-format = Định dạng
settings-compose-plain-text = Viết bằng văn bản thuần
settings-compose-plain-text-detail = Thư mới bắt đầu không có định dạng; có thể chuyển đổi trong cửa sổ soạn thư
settings-compose-spelling = Chính tả
settings-compose-spell-check = Kiểm tra chính tả khi tôi viết
settings-compose-spell-check-detail = Từ sai chính tả được gạch chân, có gợi ý khi nhấp chuột phải
settings-compose-spell-desktop = Ngôn ngữ của môi trường máy tính ({ $language })
settings-compose-templates = Mẫu thư
settings-compose-templates-detail = Lưu những thư bạn hay viết, rồi bắt đầu thư mới hoặc thư trả lời từ đó.

## Settings > Shortcuts

settings-shortcuts-set = Bộ phím tắt
settings-shortcuts-set-detail = Bắt đầu từ các phím của một ứng dụng thư bạn quen dùng. Ở đây Cmd là Ctrl. Các thay đổi của riêng bạn được áp lên trên bộ phím, và Khôi phục mặc định sẽ trở về các phím của bộ.
settings-shortcuts-single = Phím tắt một phím
settings-shortcuts-single-detail = Các phím không kèm Ctrl hoặc Alt, như trong webmail: e để lưu trữ, j và k để di chuyển, / để tìm kiếm. Chúng hoạt động trong danh sách và cuộc hội thoại đang mở, không bao giờ khi đang gõ.
settings-shortcuts-single-use = Dùng phím tắt một phím
settings-shortcuts-single-use-detail = Phím tắt có Ctrl luôn hoạt động
settings-shortcuts-how = Nhấp vào một phím để thay đổi, hoặc vào + để thêm, rồi nhấn phím mới. Esc để hủy.
settings-shortcuts-restore = Khôi phục mặc định
settings-shortcuts-no-key = Không có phím
settings-shortcuts-press = Nhấn phím…
settings-shortcuts-then = { $keys } rồi…
settings-shortcuts-moved = { $keys } giờ thực hiện “{ $action }” thay vì “{ $previous }”.
settings-shortcuts-single-off = Phím tắt một phím đang tắt, nên phím này sẽ hoạt động khi bạn bật chúng.
settings-shortcuts-restored = Mọi phím tắt đã trở về phím của bộ.

## Settings search: the line under a result

settings-general-language-summary = Ngôn ngữ của ứng dụng, ngày và số
settings-general-reading-summary = Thư mới nhất trước, đầy đủ phần đầu thư, tên đầy đủ của người nhận
settings-general-mark-read-summary = Khi nào cuộc hội thoại đã mở được đánh dấu là đã đọc: ngay lập tức, sau 1 hoặc 3 giây, hoặc thủ công
settings-general-reply-button-summary = Nút trả lời bên cạnh mỗi thư sẽ trả lời tất cả mọi người
settings-general-remote-images-summary = Luôn hiển thị hình ảnh của mọi thư
settings-general-sending-summary = Hủy gửi: thời gian thư đã gửi chờ trước khi đi, để có thể thu hồi
settings-general-offline-summary = Bao nhiêu ngày thư gần đây được tải xuống đầy đủ, để đọc khi không có kết nối
settings-general-notifications-summary = Thông báo thư mới và âm thanh của thông báo
settings-general-desktop-summary = Mở Katna Mail khi đăng nhập, biểu tượng ở khay hệ thống và số thư chưa đọc trên biểu tượng ở thanh tác vụ
settings-accounts-accounts-summary = Thêm hoặc xóa tài khoản, hoặc đổi ảnh của tài khoản
settings-appearance-density-summary = Dòng mặc định hoặc thu gọn trong danh sách
settings-appearance-scaling-summary = Làm mọi thứ to hơn hoặc nhỏ hơn: chữ, biểu tượng, khoảng cách và đường phân cách
settings-appearance-theme-summary = Giống môi trường máy tính, sáng hoặc tối
settings-appearance-sender-pictures-summary = Logo công ty, tra cứu theo tên miền của người gửi
settings-appearance-important-summary = Điểm đánh dấu quan trọng bên cạnh mỗi thư trong danh sách
settings-appearance-mail-colors-summary = Màu tối cho thư HTML khi dùng chủ đề tối, hoặc màu của người gửi
settings-appearance-attachment-previews-summary = Một hình nhỏ về nội dung của mỗi tệp đính kèm
settings-shortcuts-set-summary = Bắt đầu từ các phím của Gmail, Inbox by Gmail, Apple Mail, Outlook hoặc Thunderbird
settings-shortcuts-single-summary = Các phím không kèm Ctrl hoặc Alt, như trong webmail
settings-default-apps-pdf-summary = Nơi mở tệp đính kèm PDF
settings-default-apps-pictures-summary = Nơi mở ảnh chụp và hình ảnh
settings-default-apps-text-summary = Nơi mở văn bản thuần, nhật ký và mã nguồn
settings-default-apps-sheets-summary = Nơi mở tệp Excel, OpenDocument và CSV
settings-default-apps-documents-summary = Nơi mở văn bản Word và OpenDocument
settings-default-apps-after-saving-summary = Hiện tệp đính kèm đã lưu trong thư mục của chúng
settings-compose-send-from-summary = Tài khoản gửi thư mới: tài khoản bạn đang dùng, hoặc luôn cùng một tài khoản
settings-compose-send-on-replies-summary = Gửi, hoặc Gửi và lưu trữ cuộc hội thoại, khi trả lời và chuyển tiếp
settings-compose-signatures-summary = Được thêm bên dưới thư của bạn, sau dòng “--”
settings-compose-for-new-mail-summary = Chữ ký mở đầu cho thư mới
settings-compose-for-replies-summary = Chữ ký mở đầu cho thư trả lời và chuyển tiếp
settings-compose-format-summary = Viết thư mới bằng văn bản thuần
settings-compose-spelling-summary = Kiểm tra chính tả khi viết, và ngôn ngữ của từ điển
settings-compose-templates-summary = Sắp ra mắt: lưu những thư bạn hay viết, rồi bắt đầu thư mới hoặc thư trả lời từ đó
settings-feedback-crash-reports-summary = Lưu báo cáo sự cố trên máy tính này khi Katna Mail hoặc dịch vụ nền của nó gặp sự cố
settings-feedback-saved-summary = Xem, sao chép hoặc xóa báo cáo sự cố đã lưu trên máy tính này
settings-feedback-help-improve-summary = Gửi báo cáo sự cố để giúp khắc phục lỗi; tắt trừ khi bạn bật
settings-experimental-blur-summary = Môi trường máy tính hiện mờ qua thanh trên cùng, và menu có hiệu ứng kính mờ
settings-search-shortcut = Phím tắt
settings-search-tab = Tab cài đặt
settings-search-none = Không có cài đặt nào khớp với “{ $query }”.
settings-search-results = Cài đặt khớp với “{ $query }”

## Quick settings (the panel that slides in from the right)

quick-title = Cài đặt nhanh
quick-see-all = Xem tất cả chế độ cài đặt
quick-reading-pane = Ngăn đọc
quick-pane-right = Bên phải danh sách
quick-pane-none = Không chia
quick-density = Mật độ
quick-density-default = Mặc định
quick-density-compact = Thu gọn
quick-theme = Chủ đề
quick-theme-system = Giống môi trường máy tính
quick-theme-light = Sáng
quick-theme-dark = Tối
quick-desktop-colors = Màu của môi trường máy tính
quick-desktop-colors-detail = Bảng màu và màu nhấn của môi trường máy tính
quick-app-names = Tên ứng dụng
quick-app-names-detail = Tên dưới biểu tượng ứng dụng ở ngoài cùng bên trái
quick-inbox-tabs = Tab hộp thư đến
quick-inbox-tabs-detail = Các tab của nhà cung cấp thư của từng tài khoản
quick-choose-tabs = Chọn tab
quick-choose-tabs-detail = Theo từng tài khoản, trong Cài đặt
quick-sending = Gửi thư
quick-undo-send = Hủy gửi
quick-undo-send-off = Tắt
quick-undo-send-seconds = { $seconds } giây
quick-signatures = Chữ ký
quick-signatures-none = Chưa có
quick-signatures-one = { $name }, dùng theo mặc định
quick-signatures-many = { $count } chữ ký; mặc định là { $name }
quick-signatures-no-default = { $count }, không có mặc định
quick-signature-untitled = Không có tiêu đề
quick-threading = Chuỗi email
quick-conversation-view = Chế độ xem cuộc hội thoại
quick-conversation-view-detail = Nhóm các thư trả lời cho cùng một thư
quick-help = Trợ giúp
quick-tour = Tham quan ứng dụng
quick-whats-new = Có gì mới
quick-about = Giới thiệu về Katna

## Settings: opening at login

settings-open-at-login-failed = Không thể thay đổi việc mở khi đăng nhập: { $error }

## Settings > Appearance > Scaling

scale-letter = A
scale-percent = { $percent }%
scale-reset = Trở về { $percent }%

## Settings > Experimental > Look & Feel

look-intro = Các tính năng vẫn đang được thử nghiệm. Chúng có thể thay đổi hoặc bị gỡ bỏ.
look-heading = Giao diện và cảm nhận
look-window-frame = Khung cửa sổ
look-window-frame-detail = Ai vẽ thanh tiêu đề, các nút cửa sổ, các góc và bóng đổ.
look-frame-native-kde = Gốc: khung của KDE, theo chủ đề Plasma của bạn
look-frame-native = Gốc: khung của môi trường máy tính
look-frame-katna = Katna: thanh trên cùng trở thành thanh tiêu đề
look-frame-katna-note-named = Katna vẽ góc bo tròn và bóng đổ riêng. Khung không còn theo chủ đề { $desktop } nữa; các quy tắc cửa sổ vẫn được áp dụng.
look-frame-katna-note = Katna vẽ góc bo tròn và bóng đổ riêng. Khung không còn theo chủ đề của môi trường máy tính nữa; các quy tắc cửa sổ vẫn được áp dụng.
look-frame-client-side = Môi trường máy tính của bạn để mỗi ứng dụng tự vẽ khung, nên Katna đã tự vẽ khung của mình.
look-blurred-background = Nền mờ
look-blurred-background-detail = Môi trường máy tính hiện mờ qua thanh trên cùng và các thư mục, còn menu và cửa sổ bật lên như kính mờ.
look-blur = Làm mờ những gì phía sau cửa sổ
look-blur-detail = Thư vẫn nằm trên thẻ nền đặc, nên chữ giữ được độ tương phản
look-blur-off-kde = Hiệu ứng làm mờ của KDE đang tắt. Hãy bật Làm mờ trong Thiết lập hệ thống, Quản lí cửa sổ, Hiệu ứng màn hình nền, rồi mở lại Katna Mail.
look-blur-none-gnome = GNOME không làm mờ những gì phía sau cửa sổ.
look-blur-none-x11 = Trình quản lý cửa sổ của bạn không làm mờ những gì phía sau cửa sổ.
look-blur-none-wayland = Trình tổng hợp của bạn không làm mờ những gì phía sau cửa sổ.

## Settings > User feedback (crash reports)

feedback-intro-sending = Báo cáo sự cố mới được gửi đi để giúp khắc phục lỗi. Không có gì khác rời khỏi máy tính này.
feedback-intro-local = Katna không gửi gì đi đâu cả. Báo cáo sự cố được giữ trên máy tính này, để bạn xem hoặc đính kèm vào báo cáo lỗi.
feedback-crash-reports = Báo cáo sự cố
feedback-crash-reports-detail = Được ghi lại khi Katna Mail hoặc dịch vụ nền của nó gặp sự cố.
feedback-save = Lưu báo cáo sự cố trên máy tính này
feedback-save-detail = Thư mục chính, tên người dùng, tên máy tính và địa chỉ email của bạn được loại bỏ
feedback-saved = Báo cáo sự cố đã lưu
feedback-saved-detail = Giữ lại { $count } báo cáo mới nhất.
feedback-help-improve = Giúp cải thiện Katna
feedback-help-improve-detail = Tắt trừ khi bạn bật, và bạn có thể tắt tại đây bất cứ lúc nào.
feedback-send = Gửi báo cáo sự cố
feedback-send-detail = Báo cáo đã lưu, đúng như bạn thấy tại đây, được gửi tới trình theo dõi sự cố của Katna (Sentry, tại EU). Không có địa chỉ IP, thư hay địa chỉ email
feedback-none-saved = Không có báo cáo sự cố nào được lưu.
feedback-delete-all = Xóa tất cả
feedback-app-daemon = Dịch vụ nền
feedback-report-sent = { $date } · Đã gửi
feedback-view = Xem
feedback-view-tooltip = Mở báo cáo
feedback-copy-tooltip = Sao chép để dán vào báo cáo lỗi
feedback-copied = Đã sao chép báo cáo sự cố.
feedback-deleted-all = Đã xóa các báo cáo sự cố.
feedback-read-failed = Không thể đọc báo cáo sự cố: { $error }
feedback-delete-failed = Không thể xóa báo cáo sự cố: { $error }
feedback-delete-all-failed = Không thể xóa các báo cáo sự cố: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _Tệp
desktop-menu-new-message = Thư _mới
desktop-menu-quit = T_hoát
desktop-menu-edit = _Sửa
desktop-menu-undo = _Hoàn tác
desktop-menu-select-all = Chọn _tất cả
desktop-menu-select-none = _Bỏ chọn tất cả
desktop-menu-find = Tì_m…
desktop-menu-view = _Xem
desktop-menu-folder-list = Hiện danh sách thư _mục
desktop-menu-refresh = _Làm mới
desktop-menu-go = Đ_i tới
desktop-menu-inbox = _Hộp thư đến
desktop-menu-starred = Có gắn _dấu sao
desktop-menu-sent = Đã _gửi
desktop-menu-drafts = Thư _nháp
desktop-menu-all-mail = _Tất cả thư
desktop-menu-next = Cuộc hội thoại _tiếp theo
desktop-menu-previous = Cuộc hội thoại t_rước
desktop-menu-message = _Thư
desktop-menu-open = _Mở
desktop-menu-reply = T_rả lời
desktop-menu-reply-all = Trả lời tất _cả
desktop-menu-forward = Chuyển t_iếp
desktop-menu-archive = _Lưu trữ
desktop-menu-delete = _Xóa
desktop-menu-spam = Báo cáo thư _rác
desktop-menu-move-to = _Di chuyển tới…
desktop-menu-mark-read = Đánh dấu là đã đọ_c
desktop-menu-mark-unread = Đánh dấu là c_hưa đọc
desktop-menu-star = Gắn _dấu sao
desktop-menu-important = Đánh dấu là _quan trọng
desktop-menu-not-important = Đánh dấu là _không quan trọng
desktop-menu-settings = _Cài đặt
desktop-menu-quick-settings = Cài đặt _nhanh
desktop-menu-configure = Cấu _hình Katna Mail…
desktop-menu-help = Trợ _giúp
desktop-menu-shortcuts = _Phím tắt
desktop-menu-whats-new = Có gì _mới
desktop-menu-about = _Giới thiệu về Katna

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Di chuyển
shortcut-group-actions = Thao tác
shortcut-group-go-to = Đi tới
shortcut-group-app = Ứng dụng

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Cuộc hội thoại tiếp theo
shortcut-previous = Cuộc hội thoại trước
shortcut-down = Di chuyển xuống trong danh sách
shortcut-up = Di chuyển lên trong danh sách
shortcut-first = Đầu danh sách
shortcut-last = Cuối danh sách
shortcut-page-down = Xuống một trang trong danh sách
shortcut-page-up = Lên một trang trong danh sách
shortcut-open = Mở cuộc hội thoại
shortcut-back = Quay lại danh sách
shortcut-scroll-down = Cuộn xuống
shortcut-scroll-up = Cuộn lên
shortcut-scroll-page-down = Cuộn xuống một trang
shortcut-scroll-page-up = Cuộn lên một trang
shortcut-compose = Soạn thư
shortcut-reply = Trả lời
shortcut-reply-all = Trả lời tất cả
shortcut-forward = Chuyển tiếp
shortcut-archive = Lưu trữ
shortcut-delete = Xóa
shortcut-spam = Báo cáo thư rác
shortcut-move-to = Di chuyển tới
shortcut-mark-read = Đánh dấu là đã đọc
shortcut-mark-unread = Đánh dấu là chưa đọc
shortcut-star = Gắn hoặc xóa dấu sao
shortcut-important = Đánh dấu là quan trọng
shortcut-not-important = Đánh dấu là không quan trọng
shortcut-check = Chọn cuộc hội thoại
shortcut-select-all = Chọn tất cả cuộc hội thoại
shortcut-select-none = Bỏ chọn tất cả cuộc hội thoại
shortcut-undo = Hoàn tác thao tác vừa rồi
shortcut-go-inbox = Hộp thư đến
shortcut-go-starred = Có gắn dấu sao
shortcut-go-sent = Đã gửi
shortcut-go-drafts = Thư nháp
shortcut-go-all = Tất cả thư
shortcut-search = Tìm trong thư
shortcut-navigation = Hiện hoặc thu gọn menu
shortcut-quick-settings = Cài đặt nhanh
shortcut-settings = Tất cả chế độ cài đặt
shortcut-shortcuts = Phím tắt
shortcut-reload = Kiểm tra thư mới
shortcut-quit = Thoát

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } rồi { $second }

## Settings > Accounts

accounts-folder-pane = Ngăn thư mục
accounts-folder-pane-detail = Ngăn bên trái hiển thị thư mục của những tài khoản nào.
accounts-shown-one = Mỗi lần một tài khoản; chuyển đổi trong thẻ tài khoản
accounts-shown-all = Tất cả tài khoản, lần lượt từng tài khoản
accounts-row = Tài khoản
accounts-row-detail = Xóa một tài khoản sẽ xóa bản sao thư của tài khoản đó mà Katna lưu trên máy tính này. Thư vẫn còn trên máy chủ.
accounts-none = Chưa có tài khoản nào.
accounts-kind-imported = Đã nhập
accounts-picture-reset = Dùng ảnh của máy tính
accounts-picture-change = Đổi ảnh
accounts-remove = Xóa
accounts-delete-all-row = Xóa tất cả dữ liệu
accounts-delete-all-row-detail = Bắt đầu lại, như khi mới cài đặt.
accounts-delete-all-about = Xóa khỏi máy tính này mọi tài khoản, toàn bộ thư, danh bạ và lịch đã lưu, chỉ mục tìm kiếm, cài đặt và mật khẩu đã lưu của bạn. Không có gì thay đổi trên máy chủ thư của bạn.
accounts-delete-all-open = Xóa tất cả dữ liệu Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = Đã xóa { $address } khỏi Katna.
accounts-removed = Đã xóa { $address } khỏi Katna. Thư của tài khoản vẫn còn trên máy chủ.
accounts-all-deleted = Đã xóa tất cả dữ liệu Katna khỏi máy tính này.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Xóa { $address }?
accounts-remove-confirm = Xóa tài khoản
accounts-removing = Đang xóa…
accounts-remove-local-mail = { $folders ->
    [0] Toàn bộ thư đã nhập vào tài khoản này
   *[other] Toàn bộ thư đã nhập vào tài khoản này, trong { $folders } thư mục của tài khoản
}
accounts-remove-local-settings = Cài đặt Katna của tài khoản
accounts-remove-mail = { $folders ->
    [0] Toàn bộ thư của tài khoản này do Katna lưu
   *[other] Toàn bộ thư của tài khoản này do Katna lưu, trong { $folders } thư mục của tài khoản
}
accounts-remove-outbox = Các thư đang chờ trong hộp thư đi
accounts-remove-settings = Mật khẩu đã lưu và cài đặt Katna của tài khoản
accounts-delete-all-title = Xóa tất cả dữ liệu Katna?
accounts-delete-all-confirm = Xóa mọi thứ
accounts-deleting = Đang xóa…
accounts-delete-all-accounts = Mọi tài khoản, cùng toàn bộ thư và tệp đính kèm do Katna lưu
accounts-delete-all-contacts = Danh bạ, lịch và chỉ mục tìm kiếm
accounts-delete-all-settings = Tất cả cài đặt, chữ ký và phím tắt
accounts-delete-all-passwords = Mọi mật khẩu đã lưu
accounts-deleted-heading = Bị xóa khỏi máy tính này:
accounts-cannot-undo = Không thể hoàn tác thao tác này.
accounts-server-delete-all = Không có gì thay đổi trên máy chủ thư của bạn: thư vẫn ở đó, và khi thêm lại tài khoản, thư sẽ được tải xuống lại. Thư nhập từ tệp chỉ có trong Katna; các tệp đó không bị động đến.
accounts-server-local = Thư này được nhập từ tệp, nên Katna giữ bản sao duy nhất. Các tệp gốc không bị động đến; hãy nhập lại chúng để lấy lại thư.
accounts-server-remove = Không có gì thay đổi trên máy chủ thư: thư vẫn ở đó, và khi thêm lại tài khoản, thư sẽ được tải xuống lại.
accounts-confirm-word = xóa
accounts-confirm-placeholder = Nhập “{ accounts-confirm-word }”
accounts-confirm-prompt = Để xác nhận, hãy nhập “{ accounts-confirm-word }”:
accounts-cancel = Hủy
