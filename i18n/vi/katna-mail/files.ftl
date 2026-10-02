# Katna Mail, Vietnamese (Tiếng Việt).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Tìm tệp

## Left side (and chips on a phone)

files-all = Tất cả tệp
files-pictures = Hình ảnh
files-pdfs = PDF
files-documents = Tài liệu
files-sheets = Bảng tính
files-slides = Trang chiếu
files-other = Khác
files-accounts = Tài khoản
files-drives = Ổ lưu trữ
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Được chia sẻ với tôi
files-shown = Hiển thị
files-received = Đã nhận
files-sent = Do tôi gửi

## Over the files

files-count = { $count ->
   *[other] { $count } tệp · { $size }
}
files-anyone = Bất kỳ ai
files-from-person = Từ { $name }
files-time-any = Mọi lúc
files-time-today = Hôm nay
files-time-yesterday = Hôm qua
files-time-this-week = Tuần này
files-time-last-week = Tuần trước
files-time-this-month = Tháng này
files-time-last-month = Tháng trước
files-time-between = { $first } – { $last }
files-time-hint = Nhấp vào một ngày, hoặc kéo qua nhiều ngày
files-time-summary = { $count ->
   *[other] { $days } · { $count } tệp
}
files-time-clear = Xóa
files-time-month-back = Tháng trước
files-time-month-on = Tháng sau
files-time-wheel = Cuộn để dời các ngày này, giữ nguyên độ dài
files-sort-newest = Mới nhất trước
files-sort-oldest = Cũ nhất trước
files-sort-largest = Lớn nhất trước
files-sort-name = Theo tên
files-grid = Thẻ
files-list = Danh sách
files-this-week = Tuần này
files-undated = Không có ngày
files-me = Tôi
files-no-subject = (không có tiêu đề)
files-loading = Đang thu thập tệp từ thư của bạn…
files-empty = Tệp từ thư của bạn sẽ hiện ở đây.
files-none-match = Không có tệp nào khớp.
files-load-failed = Không đọc được tệp: { $error }

## A file's menu and buttons

files-open = Mở
files-open-with = Mở bằng…
files-save = Lưu…
files-show-mail = Hiện thư
files-mail-window = Mở thư trong cửa sổ mới
files-forward = Chuyển tiếp tệp
files-from-them = Tệp từ { $name }
files-copy-name = Sao chép tên tệp
files-name-copied = Đã sao chép tên tệp
files-downloading = Đang tải thư xuống…
files-download-failed = Không thể tải thư này xuống.

## A cloud drive in place of the mail files

files-drive-mine = Drive của tôi
files-drive-mine-onedrive = Tệp của tôi
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files } tệp
   *[other] { $folders } thư mục · { $files } tệp
}
files-drive-folders = Thư mục
files-drive-files = Tệp
files-drive-folder = Thư mục
files-drive-meta = { $what } · Đã sửa { $date }
files-drive-as-link = { $what } · dạng liên kết
files-drive-google-doc = Google Tài liệu
files-drive-google-sheet = Google Trang tính
files-drive-google-slides = Google Trang trình bày
files-drive-google-drawing = Google Bản vẽ
files-drive-fetching = Đang lấy…
files-drive-loading = Đang mở ổ lưu trữ…
files-drive-empty = Thư mục này trống.
files-drive-unreachable = Không kết nối được với { $drive }.
files-drive-try-again = Thử lại
files-drive-needs-permission = Katna cần bạn cấp quyền một lần để hiển thị ổ lưu trữ này. Hãy đăng nhập lại và cho phép Katna xem tệp của bạn.
files-drive-allow = Cho phép
files-drive-allow-failed = Đăng nhập chưa hoàn tất, nên ổ lưu trữ vẫn đóng.
files-drive-attach = Đính kèm
files-drive-more = Thêm
files-drive-download = Tải xuống…
files-drive-open-web = Mở trong { $drive }
files-drive-copy-link = Sao chép liên kết
files-drive-link-copied = Đã sao chép liên kết
files-drive-share = Chia sẻ…
files-drive-rename = Đổi tên
files-drive-trash = Chuyển vào thùng rác
files-drive-trashed = “{ $name }” đã ở trong thùng rác của { $drive }
files-drive-renamed = Đã đổi tên thành “{ $name }”
files-drive-getting = Đang lấy { $name } từ { $drive }…
files-drive-get-failed = Không lấy được { $name }: { $error }
files-drive-upload = Tải lên
files-drive-upload-files = Tải tệp lên
files-drive-upload-folder = Tải thư mục lên
files-drive-upload-failed = Không tải lên được { $name }: { $error }
files-drive-upload-needs = Để tải lên, Katna cần bạn cấp quyền một lần: hãy nhấn Cho phép trong Cài đặt › Ứng dụng mặc định › Trang Tệp.

## The Share dialog of a drive file or folder

files-share-title = Chia sẻ “{ $name }”
files-share-add = Thêm người theo tên hoặc địa chỉ
files-share-not-address = “{ $text }” không phải là địa chỉ email
files-share-notify = Để { $drive } cũng gửi email cho họ
files-share-people = Những người có quyền truy cập
files-share-general = Quyền truy cập chung
files-share-loading = Đang đọc những ai có quyền truy cập…
files-share-restricted = Bị hạn chế
files-share-restricted-about = Chỉ những người có quyền truy cập mới mở được bằng liên kết
files-share-anyone = Bất kỳ ai có liên kết
files-share-anyone-can = { $role ->
    [editor] Bất kỳ ai có liên kết đều có thể chỉnh sửa
    [commenter] Bất kỳ ai có liên kết đều có thể nhận xét
   *[viewer] Bất kỳ ai có liên kết đều có thể xem
}
files-share-anyone-about = { $role ->
    [editor] Bất kỳ ai trên Internet có liên kết đều có thể chỉnh sửa
    [commenter] Bất kỳ ai trên Internet có liên kết đều có thể nhận xét
   *[viewer] Bất kỳ ai trên Internet có liên kết đều có thể xem
}
files-share-role-owner = Chủ sở hữu
files-share-role-editor = Người chỉnh sửa
files-share-role-commenter = Người nhận xét
files-share-role-viewer = Người xem
files-share-you = { $name } (bạn)
files-share-domain = Mọi người tại { $domain }
files-share-inherited = Quyền truy cập từ thư mục chứa mục này
files-share-remove = Xóa quyền truy cập
files-share-copy-link = Sao chép liên kết
files-share-share = Chia sẻ
files-share-done = Xong
files-share-sharing = Đang chia sẻ…
files-share-shared = { $count ->
   *[other] Đã chia sẻ với { $count } người
}
files-share-refused = { $drive } không thể chia sẻ với { $addresses }
files-share-failed = Không thể thay đổi việc chia sẻ: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
   *[other] Đang tải lên { $count } mục
}
files-tray-done = { $count ->
   *[other] Đã tải lên xong { $count } mục
}
files-tray-some-failed = Đã tải lên { $done }, { $failed } bị lỗi
files-tray-minutes-left = { $minutes ->
   *[other] Còn khoảng { $minutes } phút
}
files-tray-seconds-left = Còn chưa đến một phút
files-tray-starting = Đang bắt đầu…
files-tray-cancel-all = Hủy tất cả
files-tray-cancel = Hủy
files-tray-fold = Ẩn danh sách
files-tray-unfold = Hiện danh sách
files-tray-close = Đóng
files-tray-progress = { $place } · { $sent } trong { $size }
files-tray-in = Trong { $place }
files-tray-cancelled = Đã hủy
