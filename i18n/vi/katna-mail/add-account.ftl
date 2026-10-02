# Katna Mail, Vietnamese (Tiếng Việt).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Thêm tài khoản thư
add-account-providers-intro = Chọn nhà cung cấp thư của bạn. Katna sẽ lo phần còn lại.
add-account-provider-other = Thư khác
add-account-provider-other-detail = Mọi tài khoản IMAP hoặc POP3
add-account-provider-google-detail = Gmail và Google Workspace
add-account-provider-microsoft-detail = Outlook và Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = Đăng nhập vào { $provider }
add-account-form-title-other = Tài khoản thư của bạn
add-account-form-intro = Katna giữ mật khẩu của bạn trong kho khóa của hệ thống.
add-account-looking = Đang tìm máy chủ thư của { $address }…
add-account-address-intro = Nhập địa chỉ email của bạn. Katna sẽ tìm máy chủ giúp bạn.
add-account-servers-title = Cài đặt máy chủ
add-account-servers-intro = Nơi Katna đọc và gửi thư cho { $address }.
add-account-signing-in = Đang đăng nhập…
add-account-browser-title = Tiếp tục trong trình duyệt
add-account-browser-intro = Katna đã mở trang đăng nhập { $provider } trong trình duyệt của bạn. Hãy đăng nhập ở đó và cho phép Katna đọc và gửi thư của bạn, rồi quay lại đây.
add-account-browser-hint = Không thấy trang nào mở ra? Hãy kiểm tra các cửa sổ trình duyệt, hoặc quay lại và thử lại.
add-account-stage-browser = Đang chờ bạn đăng nhập trong trình duyệt…
add-account-stage-signing-in-at = Đang đăng nhập tại { $server }…
add-account-help-app-password-link = Cách tạo mật khẩu ứng dụng
add-account-help-turn-on-imap = { $provider } chỉ cho ứng dụng thư truy cập sau khi bạn bật quyền truy cập IMAP và POP3 trong cài đặt của trang web thư.
add-account-help-turn-on-imap-link = Cách bật

## Add a mail account: fields

add-account-field-address = Địa chỉ email
add-account-receive-with = Nhận thư bằng
add-account-imap-about = IMAP giữ thư và thư mục của bạn trên máy chủ, giống nhau trên mọi thiết bị. Hãy chọn IMAP khi có thể.
add-account-pop3-about = POP3 tải thư của bạn xuống máy tính này. Thư bạn đọc hoặc di chuyển ở đây vẫn giữ nguyên trên máy chủ và các thiết bị khác của bạn.
add-account-incoming = Thư đến ({ $protocol })
add-account-outgoing = Thư đi ({ $protocol })
add-account-field-server = Máy chủ
add-account-field-port = Cổng
add-account-security-none = Không
add-account-security-none-warning = Không được mã hóa: mật khẩu và thư của bạn có thể bị đọc trên đường truyền.
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

add-account-sign-in-with = Đăng nhập bằng { $provider }
add-account-sign-in-instead = Thay vào đó, đăng nhập bằng { $provider }
add-account-servers-button = Cài đặt máy chủ
add-account-back = Quay lại
add-account-add = Thêm tài khoản
add-account-done = Xong
add-account-another = Thêm tài khoản khác
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
add-account-app-password-refused = { $provider } đã từ chối mật khẩu. Cần mật khẩu ứng dụng, không phải mật khẩu bạn dùng trên web.
add-account-password-refused = Máy chủ đã từ chối mật khẩu. Hãy kiểm tra và thử lại.
add-account-sign-in-refused = { $provider } không cho Katna vào. Hãy thử lại và cho phép truy cập thư của bạn.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Bản Katna này chưa thể đăng nhập tài khoản Microsoft.
    [Google] Bản Katna này chưa thể đăng nhập tài khoản Google.
   *[other] Nhà cung cấp này chỉ cho phép đăng nhập trên trang riêng của họ, điều mà Katna chưa làm được với nhà cung cấp này.
}
add-account-smtp-not-found = Katna đã tìm thấy nơi đọc thư của bạn nhưng không tìm thấy nơi gửi thư. Hãy nhập máy chủ thư đi.

## Add a mail account: the last step

add-account-done-title = Tài khoản của bạn đã sẵn sàng
add-account-done-intro = Katna đang tải thư của bạn. Thư mới sẽ hiện khi đến.
add-account-done-sign-in = Đăng nhập
add-account-done-signed-in-with = Bằng { $provider }, trong trình duyệt của bạn
add-account-done-receiving = Nhận thư
add-account-done-sending = Gửi thư
add-account-done-on-server = Thư trên máy chủ
add-account-done-kept = Được giữ cho đến khi bạn xóa trong Katna
add-account-done-pop3-hint = Thay đổi điều gì xảy ra với thư trên máy chủ trong Cài đặt > Tài khoản.
add-account-done-zoho-title = Việc cần làm và lịch
add-account-done-zoho-about = Zoho giữ những mục này tách biệt với thư. Hãy đăng nhập bằng Zoho một lần để đưa chúng vào Katna.
add-account-done-linked = Đã kết nối việc cần làm và lịch

## The account menu (from the account button on the top bar)

add-account-menu-another = Thêm tài khoản khác
app-menu = Menu chính
app-menu-back = Quay lại
