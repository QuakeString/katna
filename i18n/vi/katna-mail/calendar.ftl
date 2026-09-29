# Katna Mail, Vietnamese (Tiếng Việt): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Hôm nay
calendar-today-tip = Chuyển đến hôm nay
calendar-view-day = Ngày
calendar-view-week = Tuần
calendar-view-month = Tháng
calendar-view-year = Năm
calendar-view-schedule = Lịch biểu
calendar-view-days =
    { $count ->
       *[other] { $count } ngày
    }
calendar-options = Tùy chọn
calendar-density = Mật độ
calendar-density-responsive = Tự điều chỉnh theo màn hình của bạn
calendar-density-comfortable = Thoải mái
calendar-density-compact = Gọn
calendar-custom-days = Chế độ xem tùy chỉnh
calendar-second-zone = Múi giờ thứ hai
calendar-zone-none = Không có
calendar-zone = { $zone } ({ $offset })
calendar-share-free = Chia sẻ giờ rảnh
calendar-free-subject = Những lúc tôi rảnh
calendar-free-intro = Đây là một số lúc tôi rảnh ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = Tôi không có thời gian rảnh trong vài ngày làm việc tới.
calendar-previous-day = Ngày trước
calendar-next-day = Ngày sau
calendar-previous-week = Tuần trước
calendar-next-week = Tuần sau
calendar-previous-month = Tháng trước
calendar-next-month = Tháng sau
calendar-previous-year = Năm trước
calendar-next-year = Năm sau
calendar-previous-period = Sớm hơn
calendar-next-period = Muộn hơn
calendar-title-months = { $first } – { $last }
calendar-loading = Đang tải…
calendar-read-failed = Không thể đọc lịch: { $error }
calendar-sets = Bộ lịch
calendar-set-add = Lưu các lịch đang hiển thị thành một bộ
calendar-set-name = Tên của bộ
calendar-set-remove = Xóa bộ
calendar-local = Trên máy tính này
calendar-account-gone = Tài khoản đã xóa
calendar-account-sign-in = Đăng nhập lại để hiện lịch
calendar-account-signed-in = Đã đăng nhập lại vào { $address }. Đang tải lịch của bạn…
calendar-account-sign-in-refused = { $provider } không cho Katna vào. Hãy thử lại và cho phép truy cập lịch của bạn.
calendar-account-refused = Máy chủ không chấp nhận mật khẩu. Yahoo, iCloud, Zoho và các dịch vụ khác cần mật khẩu ứng dụng.
calendar-account-change-password = Đổi mật khẩu
calendar-account-change-password-tooltip = Mở Cài đặt > Tài khoản
calendar-account-not-enabled = Quyền truy cập lịch cho Katna chưa được bật.
calendar-account-failed = Không đọc được lịch.
calendar-account-error = Không đọc được lịch: { $reason }
calendar-account-none = Không tìm thấy lịch nào
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = Không tìm thấy lịch nào: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } chỉ hiện lịch cho Katna khi đăng nhập bằng { $provider }.
calendar-account-sign-in-with = Đăng nhập bằng { $provider }
calendar-account-looking = Đang tìm lịch…
calendar-account-try-again = Thử lại
calendar-account-try-again-tooltip = Kiểm tra lại lịch của tài khoản này ngay
calendar-account-fixing = Đang xử lý…
calendar-birthdays = Sinh nhật
calendar-birthday-of = Sinh nhật của { $name }
calendar-empty-title = Chưa có lịch nào
calendar-empty-text = Katna hiển thị tại đây lịch của các tài khoản Google và Microsoft của bạn sau khi đồng bộ, cùng lịch của các máy chủ khác hỗ trợ CalDAV.
calendar-schedule-empty = Không có kế hoạch nào trong hai tháng tới.
calendar-search = Tìm sự kiện
calendar-search-past = Sự kiện đã qua
calendar-search-none = Không có sự kiện nào khớp với nội dung tìm kiếm.
calendar-no-title = (Không có tiêu đề)
calendar-all-day = Cả ngày
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } mục khác
calendar-repeats = Lặp lại
calendar-join = Tham gia
calendar-email-guests = Gửi thư cho khách
calendar-running-late = Tôi đến muộn
calendar-late-subject = Đến muộn: { $title }
calendar-late-body = Xin lỗi, tôi sẽ đến muộn vài phút cho { $title }. Tôi sẽ có mặt sớm.
calendar-guests =
    { $count ->
       *[other] { $count } khách
    }
calendar-guest-answers = { $yes } có, { $maybe } có thể, { $no } không, { $waiting } đang chờ
calendar-organizer = Người tổ chức
calendar-optional = Không bắt buộc
calendar-open-web = Mở trong trình duyệt
calendar-open-contact = Mở người liên hệ
calendar-close = Đóng

## Adding, changing and deleting events.

calendar-add-title = Thêm tiêu đề
calendar-add-location = Thêm vị trí
calendar-add-notes = Thêm nội dung mô tả
calendar-add-guests = Thêm khách
calendar-remove-guest = Xóa
calendar-add-meet = Thêm cuộc họp video trên Google Meet
calendar-add-teams = Thêm cuộc họp Teams
calendar-has-call = Đã thêm cuộc gọi video
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Cả ngày
calendar-more-options = Tùy chọn khác
calendar-save = Lưu
calendar-saved = Đã lưu sự kiện
calendar-deleted = Đã xóa sự kiện
calendar-discard = Bỏ thay đổi
calendar-edit = Chỉnh sửa sự kiện
calendar-delete = Xóa sự kiện
calendar-event-details = Chi tiết sự kiện
# Right-click menus on the calendar: on a free time or day, an event and
# a task.
calendar-menu-new-event = Sự kiện mới
# Shows the day right-clicked on its own, in the Day view.
calendar-menu-open-day = Mở ngày
calendar-menu-duplicate = Tạo bản sao
calendar-menu-color = Màu
# The event takes its calendar's color.
calendar-menu-color-calendar = Màu của lịch
# A task's new due day, a week from today.
calendar-menu-in-a-week = Sau một tuần
# Event colors, by the names Google Calendar gives them.
calendar-color-tomato = Cà chua
calendar-color-flamingo = Hồng hạc
calendar-color-tangerine = Quýt
calendar-color-banana = Chuối
calendar-color-sage = Xô thơm
calendar-color-basil = Húng quế
calendar-color-peacock = Chim công
calendar-color-blueberry = Việt quất
calendar-color-lavender = Oải hương
calendar-color-grape = Nho
calendar-color-graphite = Than chì
calendar-kind-event = Sự kiện
calendar-kind-focus = Thời gian tập trung
calendar-kind-out-of-office = Vắng mặt
calendar-kind-working-location = Địa điểm làm việc
calendar-working-home = Ở nhà
calendar-busy = Bận
calendar-free = Rảnh
calendar-cancel = Hủy
calendar-ok = OK
calendar-read-only = Bạn không thể thay đổi sự kiện trong lịch này
calendar-none-editable = Chưa có lịch nào để bạn thêm sự kiện
calendar-no-such-time = Thời điểm đó không tồn tại trong múi giờ của bạn
calendar-end-before-start = Sự kiện kết thúc trước khi bắt đầu
calendar-repeat-never = Không lặp lại
calendar-repeat-daily = Hằng ngày
calendar-repeat-weekly = Hằng tuần vào { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Hằng tháng vào { $weekday } đầu tiên
        [2] Hằng tháng vào { $weekday } thứ hai
        [3] Hằng tháng vào { $weekday } thứ ba
        [4] Hằng tháng vào { $weekday } thứ tư
       *[other] Hằng tháng vào { $weekday } cuối cùng
    }
calendar-repeat-yearly = Hằng năm vào { $day }
calendar-repeat-weekdays = Mọi ngày trong tuần (Thứ Hai đến Thứ Sáu)
calendar-repeat-custom = Tùy chỉnh
calendar-reminder-none = Không có thông báo
calendar-reminder-at-start = Vào lúc bắt đầu
calendar-reminder-minutes =
    { $count ->
       *[other] { $count } phút trước
    }
calendar-reminder-hours =
    { $count ->
       *[other] { $count } giờ trước
    }
calendar-reminder-days =
    { $count ->
       *[other] { $count } ngày trước
    }
calendar-scope-edit-title = Chỉnh sửa sự kiện định kỳ
calendar-scope-delete-title = Xóa sự kiện định kỳ
calendar-scope-this = Sự kiện này
calendar-scope-following = Sự kiện này và các sự kiện tiếp theo
calendar-scope-all = Tất cả sự kiện
calendar-scope-respond-title = Câu trả lời cho sự kiện định kỳ
calendar-going = Bạn có tham dự không?
calendar-answer-yes = Có
calendar-answer-no = Không
calendar-answer-maybe = Có thể
calendar-answered-yes = Bạn sẽ tham dự
calendar-answered-no = Bạn sẽ không tham dự
calendar-answered-maybe = Bạn có thể tham dự

## The card at the top of a mail with an invitation.

calendar-invite = Lời mời
calendar-invite-cancelled = Sự kiện đã bị hủy
calendar-invite-reply = { $name } đã trả lời
calendar-invite-reply-yes = { $name } đã chấp nhận
calendar-invite-reply-no = { $name } đã từ chối
calendar-invite-reply-maybe = { $name } có thể tham dự
calendar-invite-organizer = Người tổ chức: { $name }
calendar-invite-open = Mở trong Lịch
calendar-invite-not-yet = Chưa có trong lịch của bạn. Bạn có thể trả lời sau khi đồng bộ.
calendar-invite-by-mail = Không có trong lịch của bạn: câu trả lời của bạn được gửi đến người tổ chức qua email.
calendar-mail-yes = Đã chấp nhận: { $title }
calendar-mail-yes-body = { $name } đã chấp nhận lời mời này.
calendar-mail-no = Đã từ chối: { $title }
calendar-mail-no-body = { $name } đã từ chối lời mời này.
calendar-mail-maybe = Chưa chắc chắn: { $title }
calendar-mail-maybe-body = { $name } đã chấp nhận lời mời này một cách chưa chắc chắn.
calendar-invite-your-day = Ngày của bạn
calendar-invite-clashes =
    { $count ->
       *[other] Trùng với { $count } sự kiện
    }

## The day's agenda beside the mail.

agenda-show = Hiện chương trình trong ngày
agenda-hide = Ẩn chương trình
agenda-today = Hôm nay, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = Không có kế hoạch nào vào ngày này.
