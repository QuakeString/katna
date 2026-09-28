# Katna Mail, Vietnamese (Tiếng Việt).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Giới thiệu về Katna
about-tagline = Thư và lịch cho máy tính Linux
about-whats-new = Có gì mới
about-update-not-checked = Chưa kiểm tra cập nhật
about-update-checking = Đang kiểm tra cập nhật…
about-update-up-to-date = Katna Mail đã là phiên bản mới nhất
about-update-check-failed = Không thể kiểm tra cập nhật
about-update-available = Phiên bản { $version } đã có sẵn
about-update-downloading = Đang tải xuống phiên bản { $version }… { $percent }%
about-update-download-failed = Việc tải xuống phiên bản { $version } chưa hoàn tất
about-update-ready = Phiên bản { $version } đã sẵn sàng để cài đặt
about-update-ready-detail = Katna Mail sẽ khởi động lại để hoàn tất việc cập nhật.
about-update-confirm = Cài đặt phiên bản { $version }?
about-update-confirm-detail = Katna Mail sẽ đóng lại, cài đặt bản cập nhật rồi mở lại đúng chỗ bạn đang dừng. Máy tính của bạn sẽ hỏi mật khẩu của bạn.
about-update-installing = Đang cài đặt phiên bản { $version }…
about-update-installing-detail = Nhập mật khẩu của bạn vào cửa sổ vừa mở.
about-update-cancelled = Bản cập nhật chưa được cài đặt, vì mật khẩu chưa được cung cấp.
about-update-failed = Không thể cài đặt bản cập nhật: { $error }
about-update-unsupported = Bản sao Katna Mail này được cập nhật bởi trình quản lý gói của bạn.
about-update-restart-failed = Bản cập nhật đã được cài đặt, nhưng Katna Mail không thể mở lại được ({ $error }). Hãy tự mở nó.
about-update-check = Kiểm tra cập nhật
about-update-download = Tải xuống
about-update-retry = Thử lại
about-update-button = Cập nhật
about-update-restart = Cập nhật và khởi động lại
about-update-cancel = Để sau
about-changelog = Nhật ký thay đổi
about-source = Mã nguồn
about-coffee = Mời tôi một ly cà phê
about-coming-soon = Sắp ra mắt
about-follow-me = Theo dõi tôi trên
about-love-title = Được tạo ra bằng tình yêu dành cho Rust, KDE và Linux
about-love-text = Rust giúp việc viết một ứng dụng thư nhanh và an toàn trở thành niềm vui: Katna không có mã unsafe nào. Môi trường Plasma của KDE và bộ PIM của KDE đã truyền cảm hứng cho Katna, còn Linux và cộng đồng phần mềm tự do xây nên nền móng để Katna đứng vững. Cảm ơn các bạn, và cảm ơn các thư viện bên dưới.
about-kde-text = KDE xây dựng môi trường máy tính mà Katna cảm thấy như ở nhà nhất; KDE do tình nguyện viên tạo ra và được những người như bạn tài trợ. Nếu bạn thích Plasma hoặc các ứng dụng của KDE, hãy cân nhắc ủng hộ KDE.
about-donate-kde = Ủng hộ KDE
about-gpui-title = Xây dựng trên GPUI, từ dự án Zed
about-gpui-text = Toàn bộ giao diện của Katna Mail được xây dựng trên GPUI, framework giao diện nhanh, tăng tốc bằng GPU mà Zed Industries tạo ra cho trình soạn thảo Zed. Mọi điểm ảnh, hoạt ảnh và cửa sổ bạn thấy đều do nó vẽ. Cảm ơn đội ngũ Zed đã phát triển nó một cách công khai. Apache-2.0.
about-gpui-github = GPUI trên GitHub
about-personal-title = Một dự án cá nhân
about-personal-text = Katna Mail không cố trở nên mới mẻ hay mang tính cách mạng. Đây là ứng dụng thư mà tác giả mong muốn, với tính năng và giao diện mượn từ Gmail, Mailspring và Thunderbird. Nó chỉ có thể thành hiện thực nhờ những bước tiến xa của LLM.
about-built-on = XÂY DỰNG TRÊN PHẦN MỀM TỰ DO
about-credit-pimalaya = IMAP, SMTP và đăng nhập (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = Đọc và ghi IMAP
about-credit-tantivy = Tìm kiếm
about-credit-sqlite = Kho thư
about-credit-rustls = Kết nối an toàn
about-credit-mail-parser = Đọc thư, từ Stalwart Labs
about-credit-html5ever = Thư HTML, từ dự án Servo
about-credit-zbus = Giao tiếp với môi trường máy tính qua D-Bus và portal
about-credit-oo7 = Mật khẩu trong chùm khóa của môi trường máy tính
about-credit-hayro = Xem và in PDF
about-credit-calamine = Xem trước bảng tính
about-credit-resvg = Hình ảnh SVG
about-credit-jiff = Ngày tháng và múi giờ
about-credit-spellbook = Kiểm tra chính tả, từ trình soạn thảo Helix
about-credit-smol = Làm nhiều việc cùng lúc
about-all-libraries = Mọi thư viện Katna sử dụng ({ $count })
about-library-authors = bởi { $authors }
about-license = Katna là phần mềm tự do theo giấy phép GNU GPL, phiên bản 3 trở lên.
about-close = Đóng

## What’s new (shown after an update)

whats-new-title = Có gì mới trong Katna Mail
whats-new-updated = Đã cập nhật lên phiên bản { $version }
whats-new-version = Phiên bản { $version }
whats-new-more = { $count ->
   *[other] Và { $count } thay đổi khác trong nhật ký thay đổi đầy đủ.
}
whats-new-changelog = Nhật ký thay đổi đầy đủ
whats-new-got-it = Đã hiểu

## First run: welcome page

onboarding-welcome-title = Chào mừng đến với Katna Mail
onboarding-welcome-lead = Thư của bạn trên chính máy tính của bạn: tìm kiếm nhanh, đọc được khi ngoại tuyến và riêng tư.
onboarding-fast-title = Nhanh, kể cả khi ngoại tuyến
onboarding-fast-text = Katna giữ một bản sao thư của bạn tại đây, nên việc mở và tìm kiếm diễn ra tức thì, dù có kết nối hay không.
onboarding-providers-title = Hoạt động với thư của bạn
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud và mọi tài khoản IMAP hoặc POP khác.
onboarding-private-title = Riêng tư
onboarding-private-text = Thư đi thẳng từ nhà cung cấp của bạn về máy tính này. Không máy chủ Katna nào thấy được thư.
onboarding-get-started = Bắt đầu

## First run: adding an account

onboarding-service-checking = Đang kiểm tra dịch vụ nền của Katna…
onboarding-service-running = Dịch vụ nền của Katna đang chạy.
onboarding-service-missing = Dịch vụ nền của Katna không chạy
onboarding-service-start = Dịch vụ này nhận và gửi thư của bạn. Hãy khởi động nó từ terminal, rồi kiểm tra lại:
onboarding-check-again = Kiểm tra lại
onboarding-account-title = Thêm tài khoản thư của bạn
onboarding-account-lead = Nhập địa chỉ email và mật khẩu, Katna sẽ tìm cài đặt máy chủ. Gmail, Yahoo và iCloud cần mật khẩu ứng dụng, được tạo trong cài đặt bảo mật của tài khoản.
onboarding-add-account = Thêm tài khoản
onboarding-back = Quay lại

## First run: choosing the look

onboarding-look-title = Tùy chỉnh theo ý bạn
onboarding-look-lead = Chọn cách thư được mở và giao diện của Katna. Bạn có thể thay đổi bất cứ lúc nào trong cài đặt nhanh.
onboarding-reading-pane = Ngăn đọc
onboarding-pane-right = Bên phải danh sách
onboarding-pane-none = Không chia
onboarding-theme = Chủ đề
onboarding-theme-system = Hệ thống
onboarding-theme-light = Sáng
onboarding-theme-dark = Tối
onboarding-density = Mật độ
onboarding-density-default = Mặc định
onboarding-density-compact = Thu gọn
onboarding-continue = Tiếp tục

## First run: done

onboarding-ready-title = Mọi thứ đã sẵn sàng
onboarding-ready-lead = Katna đang nhận thư của bạn. Thư hiện ra khi về tới, và thư mới sẽ tự xuất hiện.
onboarding-ready-lead-address = Katna đang nhận thư của { $address }. Thư hiện ra khi về tới, và thư mới sẽ tự xuất hiện.
onboarding-ready-tour = Tham quan một phút để xem mọi thứ nằm ở đâu?
onboarding-skip = Để sau
onboarding-take-tour = Tham quan

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Giúp cải thiện Katna
share-lead = Khi Katna gặp sự cố, nó lưu một báo cáo trên máy tính này. Gửi các báo cáo này giúp sửa lỗi đã xảy ra. Bạn có thể thay đổi bất cứ lúc nào trong Cài đặt > Ý kiến phản hồi.
share-sent = Những gì được gửi
share-sent-detail = Báo cáo sự cố đúng như bạn xem được trong Cài đặt: thành phần nào gặp sự cố và ở đâu trong Katna, phiên bản, hệ thống Linux và môi trường máy tính của bạn, cùng các dòng nhật ký cuối của Katna, có thể chứa tên thư mục thư.
share-never-sent = Những gì không bao giờ được gửi
share-never-sent-detail = Thư, danh bạ, mật khẩu, địa chỉ IP, tên người dùng hay tên máy tính của bạn. Địa chỉ email được xóa khỏi báo cáo.
share-where = Được gửi tới đâu
share-where-detail = Trình theo dõi sự cố của Katna tại Sentry, lưu trữ tại EU. Không có ID nào liên kết báo cáo với bạn.
share-dont-send = Không gửi
share-send = Gửi báo cáo sự cố
share-sending = Báo cáo sự cố sẽ được gửi. Cảm ơn bạn.
share-local = Báo cáo sự cố được giữ trên máy tính này.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Chào mừng đến với Katna Mail
tour-welcome-text = Chuyến tham quan một phút cho bạn thấy mọi thứ nằm ở đâu.
tour-not-now = Để sau
tour-start = Tham quan
tour-close = Đóng
tour-skip = Bỏ qua
tour-back = Quay lại
tour-done = Xong
tour-next = Tiếp
tour-step = { $step } / { $total }
tour-compose-title = Viết thư
tour-compose-text = Soạn thư mở một thư mới ở góc dưới bên phải, để bạn vừa viết vừa đọc tiếp.
tour-search-title = Tìm trong toàn bộ thư
tour-search-text = Tìm kiếm hoạt động cả khi ngoại tuyến. Nút ở đầu bên phải thêm bộ lọc: người gửi, người nhận, tiêu đề, ngày và tệp đính kèm.
tour-menu-title = Hiện hoặc ẩn thư mục
tour-menu-text = Nút này thu gọn danh sách thư mục. Khi danh sách bị ẩn, hãy đặt con trỏ lên Thư ở bên trái để xem thư mục.
tour-apps-title = Ứng dụng của bạn
tour-apps-text = Thư đang ở đây. Lịch, Danh bạ, Việc cần làm, Ghi chú và Nguồn cấp sẽ cùng có mặt trên thanh này.
tour-tabs-title = Các thẻ Hộp thư đến
tour-tabs-text = Thư mới được xếp vào Chính, Quảng cáo, Mạng xã hội, Cập nhật và Diễn đàn. Bạn có thể tắt các thẻ trong cài đặt nhanh.
tour-list-title = Thư của bạn
tour-list-text = Nhấp vào một thư để đọc. Di chuột lên thư để có thao tác nhanh, nhấp chuột phải để có thêm tùy chọn, hoặc đánh dấu nhiều thư để xử lý cùng lúc.
tour-settings-title = Cài đặt nhanh
tour-settings-text = Thay đổi ngăn đọc, mật độ và chủ đề tại đây. Bạn cũng có thể bắt đầu lại chuyến tham quan từ đó.
tour-account-title = Tài khoản của bạn
tour-account-text = Xem bạn đang ở tài khoản nào, và thêm tài khoản khác.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Dịch vụ nền của Katna đã dừng đột ngột.
   *[other] Dịch vụ nền của Katna đã dừng đột ngột. Còn { $more } báo cáo sự cố khác đã được lưu.
}
crash-mail = { $more ->
    [0] Lần trước Katna Mail đã đóng đột ngột.
   *[other] Lần trước Katna Mail đã đóng đột ngột. Còn { $more } báo cáo sự cố khác đã được lưu.
}
crash-view = Xem báo cáo
crash-view-tooltip = Mở báo cáo đã lưu trên máy tính này
crash-copy = Sao chép báo cáo
crash-close = Đóng
sign-in-again-text = { $provider } yêu cầu bạn đăng nhập lại vào { $address }.
sign-in-again-button = Đăng nhập
sign-in-again-tooltip = Mở trang đăng nhập { $provider } trong trình duyệt
sign-in-again-waiting = Đang chờ trình duyệt…
sign-in-again-close = Đóng
sign-in-again-done = Đã đăng nhập lại vào { $address }. Đang nhận thư của bạn…
delete-ask-title = { $kind ->
    [conversation] { $count ->
       *[other] Chuyển { $count } cuộc hội thoại vào Thùng rác?
    }
   *[message] { $count ->
       *[other] Chuyển { $count } thư vào Thùng rác?
    }
}
delete-ask-body = { $count ->
   *[other] Bạn có thể hoàn tác ngay sau đó, hoặc lấy lại từ Thùng rác về sau.
}
delete-ask-confirm = Chuyển vào Thùng rác
delete-forever-title = { $kind ->
    [conversation] { $count ->
       *[other] Xóa vĩnh viễn { $count } cuộc hội thoại?
    }
   *[message] { $count ->
       *[other] Xóa vĩnh viễn { $count } thư?
    }
}
delete-forever-body = { $count ->
   *[other] Chúng cũng bị xóa trên máy chủ. Việc này không thể hoàn tác.
}
delete-forever-confirm = Xóa vĩnh viễn
delete-ask-dont-ask = Không hỏi lại
delete-ask-cancel = Hủy
