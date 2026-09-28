# Katna Mail, Vietnamese (Tiếng Việt).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = Ngăn thư mục
accounts-folder-pane-detail = Ngăn bên trái hiển thị thư mục của những tài khoản nào.
accounts-shown-one = Mỗi lần một tài khoản; chuyển đổi trong thẻ tài khoản
accounts-shown-all = Tất cả tài khoản, lần lượt từng tài khoản
accounts-unified = Hộp thư đến hợp nhất
accounts-unified-switch = Hiện thư của tất cả tài khoản cùng lúc
accounts-unified-switch-detail = “Tất cả tài khoản” nằm ở đầu ngăn thư mục, với hộp thư đến, thư đã gửi và nhiều mục khác của mọi tài khoản trong một danh sách. Các tài khoản bên dưới ban đầu được thu gọn.
accounts-row = Tài khoản
accounts-row-detail = Ngăn thư mục và menu tài khoản liệt kê các tài khoản theo thứ tự này; tài khoản đầu tiên là mặc định. Xóa một tài khoản sẽ xóa bản sao thư của tài khoản đó mà Katna lưu trên máy tính này. Thư vẫn còn trên máy chủ.
accounts-none = Chưa có tài khoản nào.
accounts-kind-imported = Đã nhập
accounts-picture-reset = Dùng ảnh của máy tính
accounts-picture-change = Đổi ảnh
accounts-picture-remove = Xóa ảnh
accounts-rename = Đổi tên
accounts-name-save = Lưu
accounts-name-cancel = Hủy
accounts-name-placeholder = Tên của bạn
accounts-rename-failed = Không thể đổi tên tài khoản: { $error }
accounts-move-up = Chuyển lên
accounts-move-down = Chuyển xuống
accounts-drag = Kéo để đổi thứ tự
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
reset-cache-about = Xóa thư và tệp đính kèm Katna đã tải xuống, ảnh người gửi và chỉ mục tìm kiếm, rồi tải xuống lại thư gần đây. Tài khoản, cài đặt và thư chỉ có trên máy tính này vẫn được giữ.
reset-cache-button = Đặt lại bộ nhớ đệm
reset-cache-title = Đặt lại bộ nhớ đệm?
reset-cache-deleted = Bị xóa, rồi được tải xuống lại:
reset-cache-mail = Thư và tệp đính kèm tải xuống từ máy chủ IMAP của bạn: thư gần đây được tải xuống lại ngay, thư cũ hơn khi bạn mở
reset-cache-index = Chỉ mục tìm kiếm, được tạo lại ngay
reset-cache-pictures = Ảnh người gửi
reset-cache-kept = Được giữ lại: tài khoản, mật khẩu và cài đặt của bạn; dấu sao, nhãn, dấu đã đọc và ghim; thư nháp, hộp thư đi và các thay đổi chưa lên máy chủ; cùng thư từ tài khoản POP3 hoặc tệp đã nhập, vốn có thể không có bản sao nào khác. Không có gì thay đổi trên máy chủ thư của bạn.
reset-cache-confirm = Đặt lại bộ nhớ đệm
reset-cache-busy = Đang đặt lại…
reset-cache-done = Đã đặt lại bộ nhớ đệm. Thư gần đây đang được tải xuống lại.
reset-cache-done-freed = Đã đặt lại bộ nhớ đệm và giải phóng { $size }. Thư gần đây đang được tải xuống lại.
