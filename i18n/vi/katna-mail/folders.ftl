# Katna Mail, Vietnamese (Tiếng Việt).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Nhãn
nav-folders = Thư mục
nav-label-new = Tạo nhãn mới
nav-folder-new = Tạo thư mục mới
nav-menu-check-mail = Kiểm tra thư mới
nav-menu-check-inbox = Kiểm tra hộp thư đến này
nav-unified-leave-out = Bỏ khỏi Hộp thư đến hợp nhất
nav-unified-bring-back = Đưa trở lại Hộp thư đến hợp nhất
nav-menu-sign-in-again = Đăng nhập lại
nav-menu-new-mail = Thư mới từ tài khoản này
nav-menu-account-settings = Cài đặt tài khoản
nav-account-checked = Đã đồng bộ · kiểm tra { $ago }
nav-account-in-sync = Đã đồng bộ
nav-account-connecting = Đang kết nối…
nav-account-offline = Ngoại tuyến, đang thử lại
nav-account-signed-out = Phiên đăng nhập { $provider } đã hết hạn
nav-account-password-refused = Mật khẩu bị từ chối
nav-account-storage = Đã dùng { $used } trong { $total }
nav-menu-new-subfolder = Thư mục mới bên trong
nav-menu-new-sublabel = Nhãn mới bên trong
nav-menu-rename = Đổi tên
nav-menu-delete = Xóa
nav-menu-empty-trash = Dọn sạch Thùng rác
nav-account-unnamed = Tài khoản { $number }
nav-all-accounts = Tất cả tài khoản
nav-expand = Hiện thư mục
nav-collapse = Ẩn thư mục
storage-used = Đã dùng { $percent }% trong { $total }
storage-used-detail = { $address }: đã dùng { $used } trong { $total }

## Special folders (the user's own folders keep their names)

folder-inbox = Hộp thư đến
folder-starred = Có gắn dấu sao
folder-snoozed = Đã tạm ẩn
folder-unread = Chưa đọc
folder-important = Quan trọng
folder-drafts = Thư nháp
folder-sent = Đã gửi
folder-archive = Lưu trữ
folder-spam = Thư rác
folder-trash = Thùng rác
folder-all-mail = Tất cả thư
folder-scheduled = Đã lên lịch
folder-waiting = Đang chờ trả lời
folder-waiting-short = Đang chờ
folder-reminders = Lời nhắc
folder-outbox = Hộp thư đi
folder-activity = Hoạt động
folder-not-on-account = Tài khoản này không có thư mục đó.

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
label-rename-title = Đổi tên nhãn
label-folder-rename-title = Đổi tên thư mục
label-rename = Đổi tên
label-renaming = Đang đổi tên…
label-renamed = Đã đổi tên nhãn thành “{ $name }”.
label-folder-renamed = Đã đổi tên thư mục thành “{ $name }”.

## Deleting a folder or label (asked first)

folder-delete-title = Xóa “{ $name }”?
folder-delete-body = { $count ->
    [0] Thư mục này không chứa thư nào. Thư mục sẽ bị xóa khỏi máy chủ, nên webmail và điện thoại của bạn cũng mất nó.
   *[other] { $kind ->
        [conversation] { $count } cuộc hội thoại trong đó sẽ được chuyển vào Thùng rác, nên bạn vẫn có thể lấy lại.
       *[message] { $count } thư trong đó sẽ được chuyển vào Thùng rác, nên bạn vẫn có thể lấy lại.
    } Thư mục sẽ bị xóa khỏi máy chủ, nên webmail và điện thoại của bạn cũng mất nó.
}
folder-delete-forever-body = { $count ->
    [0] Thư mục này không chứa thư nào. Thư mục sẽ bị xóa khỏi máy chủ, nên webmail và điện thoại của bạn cũng mất nó.
   *[other] { $kind ->
        [conversation] { $count } cuộc hội thoại trong đó sẽ bị xóa vĩnh viễn; tài khoản này không có Thùng rác.
       *[message] { $count } thư trong đó sẽ bị xóa vĩnh viễn; tài khoản này không có Thùng rác.
    } Thư mục sẽ bị xóa khỏi máy chủ, nên webmail và điện thoại của bạn cũng mất nó.
}
folder-delete-label-body = Nhãn sẽ bị xóa. Thư của nhãn vẫn ở trong Tất cả thư và trong các nhãn khác.
folder-delete-confirm = Xóa thư mục
folder-delete-label-confirm = Xóa nhãn
folder-deleted = Đã xóa thư mục “{ $name }”
label-deleted = Đã xóa nhãn “{ $name }”
