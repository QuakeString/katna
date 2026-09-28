# Katna Mail, Vietnamese (Tiếng Việt).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
list-checking = Đang kiểm tra thư mới…
list-more = Thêm
list-mark-read = Đánh dấu là đã đọc
list-mark-unread = Đánh dấu là chưa đọc
list-move-to = Di chuyển tới
list-archive = Lưu trữ
list-spam = Báo cáo thư rác
list-delete = Xóa
list-snooze = Tạm ẩn
list-unsnooze = Bỏ tạm ẩn
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
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] Đã chọn tất cả { $count } cuộc hội thoại đã đọc trên trang này.
       *[message] Đã chọn tất cả { $count } thư đã đọc trên trang này.
    }
   *[unread] { $kind ->
        [conversation] Đã chọn tất cả { $count } cuộc hội thoại chưa đọc trên trang này.
       *[message] Đã chọn tất cả { $count } thư chưa đọc trên trang này.
    }
    [starred] { $kind ->
        [conversation] Đã chọn tất cả { $count } cuộc hội thoại có gắn dấu sao trên trang này.
       *[message] Đã chọn tất cả { $count } thư có gắn dấu sao trên trang này.
    }
    [unstarred] { $kind ->
        [conversation] Đã chọn tất cả { $count } cuộc hội thoại không có dấu sao trên trang này.
       *[message] Đã chọn tất cả { $count } thư không có dấu sao trên trang này.
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] Chọn tất cả { $count } cuộc hội thoại đã đọc
       *[message] Chọn tất cả { $count } thư đã đọc
    }
   *[unread] { $kind ->
        [conversation] Chọn tất cả { $count } cuộc hội thoại chưa đọc
       *[message] Chọn tất cả { $count } thư chưa đọc
    }
    [starred] { $kind ->
        [conversation] Chọn tất cả { $count } cuộc hội thoại có gắn dấu sao
       *[message] Chọn tất cả { $count } thư có gắn dấu sao
    }
    [unstarred] { $kind ->
        [conversation] Chọn tất cả { $count } cuộc hội thoại không có dấu sao
       *[message] Chọn tất cả { $count } thư không có dấu sao
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] Chọn tất cả { $count } cuộc hội thoại đã đọc trong { $folder }
       *[message] Chọn tất cả { $count } thư đã đọc trong { $folder }
    }
   *[unread] { $kind ->
        [conversation] Chọn tất cả { $count } cuộc hội thoại chưa đọc trong { $folder }
       *[message] Chọn tất cả { $count } thư chưa đọc trong { $folder }
    }
    [starred] { $kind ->
        [conversation] Chọn tất cả { $count } cuộc hội thoại có gắn dấu sao trong { $folder }
       *[message] Chọn tất cả { $count } thư có gắn dấu sao trong { $folder }
    }
    [unstarred] { $kind ->
        [conversation] Chọn tất cả { $count } cuộc hội thoại không có dấu sao trong { $folder }
       *[message] Chọn tất cả { $count } thư không có dấu sao trong { $folder }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] Đã chọn tất cả { $count } cuộc hội thoại đã đọc.
       *[message] Đã chọn tất cả { $count } thư đã đọc.
    }
   *[unread] { $kind ->
        [conversation] Đã chọn tất cả { $count } cuộc hội thoại chưa đọc.
       *[message] Đã chọn tất cả { $count } thư chưa đọc.
    }
    [starred] { $kind ->
        [conversation] Đã chọn tất cả { $count } cuộc hội thoại có gắn dấu sao.
       *[message] Đã chọn tất cả { $count } thư có gắn dấu sao.
    }
    [unstarred] { $kind ->
        [conversation] Đã chọn tất cả { $count } cuộc hội thoại không có dấu sao.
       *[message] Đã chọn tất cả { $count } thư không có dấu sao.
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] Đã chọn tất cả { $count } cuộc hội thoại đã đọc trong { $folder }.
       *[message] Đã chọn tất cả { $count } thư đã đọc trong { $folder }.
    }
   *[unread] { $kind ->
        [conversation] Đã chọn tất cả { $count } cuộc hội thoại chưa đọc trong { $folder }.
       *[message] Đã chọn tất cả { $count } thư chưa đọc trong { $folder }.
    }
    [starred] { $kind ->
        [conversation] Đã chọn tất cả { $count } cuộc hội thoại có gắn dấu sao trong { $folder }.
       *[message] Đã chọn tất cả { $count } thư có gắn dấu sao trong { $folder }.
    }
    [unstarred] { $kind ->
        [conversation] Đã chọn tất cả { $count } cuộc hội thoại không có dấu sao trong { $folder }.
       *[message] Đã chọn tất cả { $count } thư không có dấu sao trong { $folder }.
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Không có cuộc hội thoại đã đọc nào ở đây.
       *[message] Không có thư đã đọc nào ở đây.
    }
   *[unread] { $kind ->
        [conversation] Không có cuộc hội thoại chưa đọc nào ở đây.
       *[message] Không có thư chưa đọc nào ở đây.
    }
    [starred] { $kind ->
        [conversation] Không có cuộc hội thoại có gắn dấu sao nào ở đây.
       *[message] Không có thư có gắn dấu sao nào ở đây.
    }
    [unstarred] { $kind ->
        [conversation] Không có cuộc hội thoại không có dấu sao nào ở đây.
       *[message] Không có thư không có dấu sao nào ở đây.
    }
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
row-tracking-none = Đang theo dõi. Chưa ai mở
row-tracking-opened = { $opened } trên { $recipients } người đã mở
row-tracking-clicked = { $opened } trên { $recipients } người đã mở, { $clicked } người đã mở liên kết
row-pin = Ghim lên đầu
row-unpin = Bỏ ghim
row-snoozed-until = Tạm ẩn đến { $when }

## Mail list: More menu and right-click menu

menu-reply = Trả lời
menu-reply-all = Trả lời tất cả
menu-forward = Chuyển tiếp
menu-archive = Lưu trữ
menu-delete = Xóa
menu-delete-forever = Xóa vĩnh viễn
menu-move-to-inbox = Chuyển vào Hộp thư đến
menu-spam = Báo cáo thư rác
menu-not-spam = Không phải thư rác
menu-mark-read = Đánh dấu là đã đọc
menu-mark-unread = Đánh dấu là chưa đọc
menu-mark-all-read = Đánh dấu tất cả là đã đọc
menu-star = Thêm dấu sao
menu-unstar = Xóa dấu sao
menu-important = Đánh dấu là quan trọng
menu-not-important = Đánh dấu là không quan trọng
menu-pin = Ghim lên đầu
menu-unpin = Bỏ ghim
menu-snooze = Tạm ẩn
menu-unsnooze = Bỏ tạm ẩn
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
toast-snoozed = { $kind ->
    [conversation] Đã tạm ẩn { $count } cuộc hội thoại đến { $when }.
   *[message] Đã tạm ẩn { $count } thư đến { $when }.
}
toast-unsnoozed = { $kind ->
    [conversation] Đã đưa { $count } cuộc hội thoại trở lại Hộp thư đến.
   *[message] Đã đưa { $count } thư trở lại Hộp thư đến.
}
toast-spam = { $kind ->
    [conversation] Đã báo cáo { $count } cuộc hội thoại là thư rác.
   *[message] Đã báo cáo { $count } thư là thư rác.
}
toast-not-spam = { $kind ->
    [conversation] Đã đánh dấu { $count } cuộc hội thoại không phải thư rác và chuyển vào hộp thư đến.
   *[message] Đã đánh dấu { $count } thư không phải thư rác và chuyển vào hộp thư đến.
}
toast-deleted-forever = { $kind ->
    [conversation] Đã xóa vĩnh viễn { $count } cuộc hội thoại.
   *[message] Đã xóa vĩnh viễn { $count } thư.
}
toast-marked-read = { $kind ->
    [conversation] Đã đánh dấu { $count } cuộc hội thoại là đã đọc.
   *[message] Đã đánh dấu { $count } thư là đã đọc.
}
toast-marked-unread = { $kind ->
    [conversation] Đã đánh dấu { $count } cuộc hội thoại là chưa đọc.
   *[message] Đã đánh dấu { $count } thư là chưa đọc.
}
toast-undone = Đã hoàn tác thao tác.
toast-nothing-to-undo = Không có gì để hoàn tác.
toast-cannot-undo-delete-forever = Thư đã xóa vĩnh viễn thì không thể khôi phục.
toast-send-undone = Đã hoàn tác việc gửi.
toast-too-late-to-undo-send = Quá muộn để hoàn tác: thư đã được gửi đi.
toast-undo = Hoàn tác
toast-close = Đóng
toast-no-spam-folder = Tài khoản này không có thư mục thư rác.
