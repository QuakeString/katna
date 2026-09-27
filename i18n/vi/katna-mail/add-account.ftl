# Katna Mail, Vietnamese (Tiếng Việt).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Thêm tài khoản thư
add-account-looking = Đang tìm máy chủ thư của { $address }…
add-account-address-intro = Nhập địa chỉ email của bạn. Katna sẽ tìm máy chủ giúp bạn.
add-account-servers-title = Cài đặt máy chủ
add-account-servers-intro = Nơi Katna đọc và gửi thư cho { $address }.
add-account-password-title = Nhập mật khẩu của bạn
add-account-signing-in = Đang đăng nhập…

## Add a mail account: fields

add-account-field-address = Địa chỉ email
add-account-incoming = Thư đến ({ $protocol })
add-account-outgoing = Thư đi ({ $protocol })
add-account-field-server = Máy chủ
add-account-field-port = Cổng
add-account-security-none = Không
add-account-field-username = Tên người dùng
add-account-field-password = Mật khẩu
add-account-show-password = Hiện mật khẩu
add-account-app-password-hint = { $provider } cần mật khẩu ứng dụng ở đây, không phải mật khẩu bạn dùng trên web. Hãy tạo một mật khẩu trong cài đặt bảo mật của tài khoản { $provider }.
add-account-field-name = Tên của bạn (không bắt buộc)
add-account-name-hint = Hiển thị cho những người bạn viết thư.
add-account-servers-pair = { $imap } và { $smtp }
add-account-servers-found = { $source ->
    [built-in] Máy chủ: { $servers }, tìm thấy trong danh sách nhà cung cấp của Katna.
    [provider] Máy chủ: { $servers }, tìm thấy trong cài đặt của nhà cung cấp.
    [ispdb] Máy chủ: { $servers }, tìm thấy trong danh sách nhà cung cấp của Thunderbird.
    [dns] Máy chủ: { $servers }, tìm thấy trong bản ghi DNS của tên miền.
   *[other] Máy chủ: { $servers }, được phỏng đoán; hãy kiểm tra nếu đăng nhập thất bại.
}
add-account-servers-entered = Máy chủ: { $servers }, như đã nhập.

## Add a mail account: buttons

add-account-servers-button = Cài đặt máy chủ
add-account-back = Quay lại
add-account-add = Thêm tài khoản
add-account-next = Tiếp
add-account-cancel = Hủy

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Nhập máy chủ thư đến.
   *[outgoing] Nhập máy chủ thư đi.
}
add-account-server-space = { $kind ->
    [incoming] Tên máy chủ thư đến có chứa dấu cách.
   *[outgoing] Tên máy chủ thư đi có chứa dấu cách.
}
add-account-port-invalid = { $kind ->
    [incoming] Cổng thư đến phải là số từ { $min } đến { $max }.
   *[outgoing] Cổng thư đi phải là số từ { $min } đến { $max }.
}
add-account-address-empty = Nhập địa chỉ email.
add-account-address-invalid = Nhập địa chỉ email, ví dụ { $example }.
add-account-not-found = Katna không tìm thấy máy chủ của { $address }, nên đã điền các tên thường dùng. Hãy kiểm tra lại với nhà cung cấp của bạn.
add-account-password-empty = Nhập mật khẩu.
add-account-name-is-password = Tên trùng với mật khẩu. Hãy nhập tên của bạn vào đó, theo cách mọi người sẽ thấy.
add-account-added = Đã thêm { $address }. Đang nhận thư của bạn…
add-account-app-password-refused = { $provider } đã từ chối mật khẩu. Cần mật khẩu ứng dụng, không phải mật khẩu bạn dùng trên web.
add-account-password-refused = Máy chủ đã từ chối mật khẩu. Hãy kiểm tra và thử lại.

## The account menu (from the account button on the top bar)

add-account-menu-another = Thêm tài khoản khác
add-account-menu-manage = Quản lý tài khoản
