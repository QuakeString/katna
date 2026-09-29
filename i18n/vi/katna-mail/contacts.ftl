# Katna Mail, Vietnamese (Tiếng Việt): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Danh bạ
contacts-frequent = Thường xuyên
contacts-other = Danh bạ khác
contacts-other-about = Những người bạn đã gửi email từ Gmail nhưng chưa lưu
contacts-other-email = Gửi email
contacts-other-empty = Không có danh bạ khác. Những người bạn gửi email từ Gmail nhưng không lưu sẽ hiện ở đây.
contacts-other-allow = Để xem danh bạ khác, hãy đăng nhập lại tài khoản Gmail và cho phép Katna xem chúng.
contacts-labels = Nhãn
contacts-label-options = Tùy chọn nhãn
contacts-label-rename = Đổi tên nhãn
contacts-label-email = Gửi email cho tất cả
contacts-label-delete = Xóa nhãn
contacts-label-new = Nhãn mới
contacts-label-name = Tên nhãn
contacts-label-button = Gắn nhãn
contacts-label-menu = Gắn nhãn là:
contacts-label-added = Đã thêm vào { $name }
contacts-label-removed = Đã xóa khỏi { $name }
contacts-label-renamed = Đã đổi tên nhãn thành { $name }
contacts-label-deleted = Đã xóa nhãn { $name }
contacts-label-no-email = Không ai trong nhãn này có địa chỉ email
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = Tài khoản
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = Đăng nhập lại để hiện danh bạ
contacts-account-signed-in = Đã đăng nhập lại vào { $address }. Đang tải danh bạ của bạn…
contacts-account-sign-in-refused = { $provider } không cho Katna vào. Hãy thử lại và cho phép truy cập danh bạ của bạn.
contacts-account-password = Máy chủ không chấp nhận mật khẩu. Yahoo, iCloud, Zoho và các dịch vụ khác cần mật khẩu ứng dụng.
contacts-account-change-password = Đổi mật khẩu
contacts-account-change-password-tooltip = Mở Cài đặt > Tài khoản
contacts-account-failed = Không đọc được danh bạ.
# $reason is the server's own words, in English.
contacts-account-error = Không đọc được danh bạ: { $reason }
contacts-account-none = Không tìm thấy sổ địa chỉ nào
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = Không tìm thấy sổ địa chỉ nào: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider } chỉ hiện danh bạ cho Katna khi đăng nhập bằng { $provider }.
contacts-account-sign-in-with = Đăng nhập bằng { $provider }
contacts-account-looking = Đang tìm danh bạ…
contacts-account-try-again = Thử lại
contacts-account-try-again-tooltip = Kiểm tra lại danh bạ của tài khoản này ngay
contacts-account-fixing = Đang xử lý…
contacts-manage = Sửa và quản lý
contacts-merge = Hợp nhất và sửa
contacts-merge-about = { $count ->
   *[other] { $count } đề xuất: các liên hệ trông giống cùng một người
}
contacts-merge-none = Không có liên hệ trùng lặp. Các liên hệ có cùng tên hoặc số điện thoại sẽ xuất hiện ở đây.
contacts-merge-count = { $count ->
   *[other] { $count } liên hệ
}
contacts-merge-all = Hợp nhất tất cả
contacts-merge-button = Hợp nhất
contacts-merge-dismiss = Bỏ qua
contacts-merged = { $count ->
    [1] Đã hợp nhất các liên hệ
   *[other] Đã hoàn tất { $count } lần hợp nhất
}
contacts-import = Nhập
contacts-export = Xuất
contacts-import-file = Nhập liên hệ từ tệp vCard hoặc CSV
contacts-imported = { $count ->
   *[other] Đã nhập { $count } liên hệ vào { $place }
}
contacts-imported-some = { $count ->
   *[other] Đã nhập { $count } liên hệ vào { $place }; bỏ qua { $skipped } liên hệ đã lưu
}
contacts-import-none = Không tìm thấy liên hệ nào trong { $name }
contacts-import-all-saved = Mọi người trong { $name } đều đã được lưu
contacts-import-failed = Không thể đọc { $name }: { $error }
contacts-exported = { $count ->
   *[other] Đã xuất { $count } liên hệ sang { $path }
}
contacts-export-none = Không có liên hệ nào để xuất
contacts-export-failed = Không thể xuất liên hệ: { $error }
contacts-print = In
contacts-print-title = Danh bạ
contacts-print-none = Không có liên hệ nào để in
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = Sinh nhật: { $day }
contacts-print-nickname = Biệt danh: { $name }
contacts-create = Tạo người liên hệ

## Search and the list

contacts-search = Tìm kiếm danh bạ
contacts-loading = Đang tải danh bạ…
contacts-empty = Chưa có liên hệ nào được lưu. Các liên hệ bạn lưu trong Gmail, Outlook hoặc dịch vụ thư của bạn sẽ xuất hiện ở đây.
contacts-empty-no-books = Các liên hệ từ tài khoản của bạn sẽ xuất hiện ở đây sau khi được đồng bộ hóa.
contacts-none-found = Không có liên hệ nào khớp với nội dung tìm kiếm.
contacts-starred = { $count ->
   *[other] Liên hệ có gắn dấu sao ({ $count })
}
contacts-count = Danh bạ ({ $count })
contacts-col-name = Tên
contacts-col-email = Email
contacts-col-phone = Số điện thoại
contacts-col-job = Chức danh và công ty
contacts-col-labels = Nhãn

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Cho phép Katna đọc danh bạ của { $address }.
contacts-allow-many = { $more ->
   *[other] Cho phép Katna đọc danh bạ của { $address } và { $more } tài khoản khác.
}
contacts-allow-button = Cho phép

## A contact's page

contacts-back = Quay lại danh bạ
contacts-edit = Sửa
contacts-delete = Xóa
contacts-qr = Chia sẻ dưới dạng mã QR
contacts-qr-about = Quét mã này bằng camera điện thoại để lưu liên hệ.
contacts-qr-too-long = Liên hệ này có quá nhiều thông tin để vừa trong mã QR.
contacts-qr-done = Xong
contacts-deleted = Đã xóa { $name }
contacts-added = Đã thêm { $name } vào danh bạ
contacts-find-mail = Thư
contacts-details = Thông tin liên hệ
contacts-saved-in = Đã lưu trong
contacts-notes = Ghi chú
contacts-birthday = Sinh nhật
contacts-nickname = Biệt danh
contacts-this-computer = Máy tính này
contacts-kind-home = Nhà riêng
contacts-kind-work = Cơ quan
contacts-kind-mobile = Di động
contacts-kind-other = Khác
contacts-source-google = Danh bạ Google
contacts-source-microsoft = Danh bạ Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Tạo người liên hệ
contacts-edit-title = Sửa người liên hệ
contacts-edit-save = Lưu
contacts-edit-saving = Đang lưu…
contacts-edit-cancel = Hủy
contacts-saved = Đã lưu người liên hệ
contacts-edit-save-to = Lưu vào
contacts-edit-changes-go-to = Các thay đổi được lưu vào { $place }.
contacts-edit-given = Tên
contacts-edit-family = Họ
contacts-edit-company = Công ty
contacts-edit-job = Chức danh
contacts-edit-email = Email
contacts-edit-phone = Điện thoại
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = Thêm email
contacts-edit-add-phone = Thêm số điện thoại
contacts-edit-street = Địa chỉ đường phố
contacts-edit-city = Thành phố
contacts-edit-postcode = Mã bưu chính
contacts-edit-country = Quốc gia
contacts-edit-birthday = Sinh nhật (YYYY-MM-DD)
contacts-edit-empty = Hãy thêm tên, email hoặc số điện thoại trước.
