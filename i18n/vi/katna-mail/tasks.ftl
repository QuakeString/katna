# Katna Mail, Vietnamese (Tiếng Việt): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Tạo
tasks-all = Tất cả việc cần làm
tasks-today = Hôm nay
tasks-starred = Có gắn dấu sao
tasks-new-list = Tạo danh sách mới
tasks-on-this-computer = Trên máy tính này
tasks-my-tasks = Việc cần làm của tôi
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Đăng nhập lại để hiện việc cần làm
tasks-account-signed-in = Đã đăng nhập lại vào { $address }. Đang tải việc cần làm của bạn…
tasks-account-sign-in-refused = { $provider } không cho Katna vào. Hãy thử lại và cho phép truy cập việc cần làm của bạn.
tasks-account-refused = Máy chủ không chấp nhận mật khẩu. Yahoo, iCloud, Zoho và các dịch vụ khác cần mật khẩu ứng dụng.
tasks-account-change-password = Đổi mật khẩu
tasks-account-change-password-tooltip = Mở Cài đặt > Tài khoản
tasks-account-not-enabled = Quyền truy cập việc cần làm cho Katna chưa được bật.
tasks-account-failed = Không đọc được danh sách việc cần làm.
# $reason is the server's own words, in English.
tasks-account-error = Không đọc được danh sách việc cần làm: { $reason }
tasks-account-none = Không tìm thấy danh sách việc cần làm nào
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = Không tìm thấy danh sách việc cần làm nào: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } chỉ hiện việc cần làm cho Katna khi đăng nhập bằng { $provider }.
tasks-account-sign-in-with = Đăng nhập bằng { $provider }
tasks-account-looking = Đang tìm danh sách việc cần làm…
tasks-account-try-again = Thử lại
tasks-account-try-again-tooltip = Kiểm tra lại việc cần làm của tài khoản này ngay
tasks-account-fixing = Đang xử lý…
tasks-list-name-placeholder = Tên danh sách

## Lists and tasks

tasks-loading = Đang đọc các việc cần làm của bạn…
tasks-no-lists = Danh sách việc cần làm của bạn sẽ hiện ở đây.
tasks-search = Tìm việc cần làm
tasks-search-none = Không có việc cần làm nào khớp với nội dung tìm kiếm.
tasks-add = Thêm việc cần làm
tasks-title-placeholder = Tiêu đề
tasks-add-step = Thêm việc phụ
tasks-empty = Chưa có việc cần làm nào. Hãy thêm một việc ở trên.
tasks-starred-empty = Gắn dấu sao cho một việc để xem tại đây.
tasks-today-empty = Không có gì đến hạn hôm nay.
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Quá hạn
tasks-completed = { $count ->
   *[other] Đã hoàn thành ({ $count })
}
tasks-list-options = Tùy chọn danh sách
tasks-rename-list = Đổi tên danh sách
tasks-delete-list = Xóa danh sách
tasks-mark-done = Đánh dấu là đã hoàn thành
tasks-mark-open = Đánh dấu là chưa hoàn thành
tasks-star = Gắn dấu sao
tasks-unstar = Bỏ dấu sao
tasks-edit-title = Sửa tiêu đề
tasks-details = Chi tiết
tasks-delete = Xóa
tasks-move-to = Chuyển đến { $list }
tasks-from-mail = Thư
tasks-open-mail = Mở thư
tasks-from-note = Ghi chú
tasks-open-note = Mở ghi chú
tasks-note-gone = Ghi chú đó không còn ở đây nữa.
tasks-no-subject = (không có tiêu đề)

## The details dialog

tasks-notes-placeholder = Thêm chi tiết
tasks-date = Ngày
tasks-no-date = Không có ngày
tasks-time-placeholder = Thêm giờ
tasks-repeat = Lặp lại
tasks-repeat-never = Không lặp lại
tasks-repeat-daily = Hằng ngày
tasks-repeat-weekly = Hằng tuần
tasks-repeat-monthly = Hằng tháng
tasks-repeat-yearly = Hằng năm
tasks-repeat-other = Tùy chỉnh
tasks-remind = Nhắc tôi
tasks-remind-off = Không nhắc
tasks-remind-on-time = Đúng giờ
tasks-remind-morning = Vào ngày đó, { $time }
tasks-remind-hour-before = Trước một giờ
tasks-remind-day-before = Trước một ngày
tasks-cancel = Hủy
tasks-save = Lưu
tasks-not-a-time = “{ $text }” không phải là giờ, ví dụ { $example }.

## Due days

tasks-due-today = Hôm nay
tasks-due-tomorrow = Ngày mai
tasks-due-yesterday = Hôm qua
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Đã hoàn thành việc cần làm
tasks-toast-next = Xong. Lần tới vào { $date }
tasks-toast-deleted = Đã xóa việc cần làm
tasks-toast-added = { $count ->
   *[other] Đã thêm { $count } việc cần làm
}
tasks-mail-gone = Thư đó không còn ở đây nữa.
tasks-toast-list-deleted = Đã xóa danh sách
tasks-toast-moved = Đã chuyển đến { $list }
# A task dragged to another place in its own list.
tasks-toast-placed = Đã di chuyển việc cần làm
tasks-toast-rescheduled = Đã dời lịch tác vụ
