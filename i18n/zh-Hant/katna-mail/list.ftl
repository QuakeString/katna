# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = 主要
tab-promotions = 促銷內容
tab-social = 社交網路
tab-updates = 最新快訊
tab-forums = 論壇
tab-focused = 焦點
tab-other = 其他
tab-inbox = 收件匣
tab-newsletters = 電子報
tab-notifications = 通知
tab-new = { $count } 封新郵件
tab-provider-other = 由 Katna 分類

## Mail list: toolbar

list-select = 選取
list-refresh = 重新整理
list-more = 更多
list-mark-read = 標示為已讀取
list-mark-unread = 標示為未讀取
list-move-to = 移至
list-archive = 封存
list-spam = 檢舉垃圾郵件
list-delete = 刪除
list-newer = 較新
list-older = 較舊
list-range = 第 { $first }–{ $last } 列，共 { $total } 列
list-range-about = 第 { $first }–{ $last } 列，共約 { $total } 列
list-results = 「{ $query }」的搜尋結果
list-results-corrected = 目前顯示的是「{ $query }」的搜尋結果
list-search-instead = 改為搜尋「{ $query }」
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = 全部
list-pick-none = 無
list-pick-read = 已讀取
list-pick-unread = 未讀取
list-pick-starred = 已加星號
list-pick-unstarred = 未加星號

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] 已選取全部 { $count } 個會話群組。
   *[message] 已選取全部 { $count } 封郵件。
}
list-selected-all-in = { $kind ->
    [conversation] 已選取「{ $folder }」中的全部 { $count } 個會話群組。
   *[message] 已選取「{ $folder }」中的全部 { $count } 封郵件。
}
list-selected-screen = { $kind ->
    [conversation] 已選取此頁上的全部 { $count } 個會話群組。
   *[message] 已選取此頁上的全部 { $count } 封郵件。
}
list-select-all = { $kind ->
    [conversation] 選取全部 { $count } 個會話群組
   *[message] 選取全部 { $count } 封郵件
}
list-select-all-in = { $kind ->
    [conversation] 選取「{ $folder }」中的全部 { $count } 個會話群組
   *[message] 選取「{ $folder }」中的全部 { $count } 封郵件
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] 已選取全部 { $count } 個已讀取會話群組。
       *[message] 已選取全部 { $count } 封已讀取郵件。
    }
   *[unread] { $kind ->
        [conversation] 已選取全部 { $count } 個未讀取會話群組。
       *[message] 已選取全部 { $count } 封未讀取郵件。
    }
    [starred] { $kind ->
        [conversation] 已選取全部 { $count } 個已加星號的會話群組。
       *[message] 已選取全部 { $count } 封已加星號的郵件。
    }
    [unstarred] { $kind ->
        [conversation] 已選取全部 { $count } 個未加星號的會話群組。
       *[message] 已選取全部 { $count } 封未加星號的郵件。
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] 已選取「{ $folder }」中的全部 { $count } 個已讀取會話群組。
       *[message] 已選取「{ $folder }」中的全部 { $count } 封已讀取郵件。
    }
   *[unread] { $kind ->
        [conversation] 已選取「{ $folder }」中的全部 { $count } 個未讀取會話群組。
       *[message] 已選取「{ $folder }」中的全部 { $count } 封未讀取郵件。
    }
    [starred] { $kind ->
        [conversation] 已選取「{ $folder }」中的全部 { $count } 個已加星號的會話群組。
       *[message] 已選取「{ $folder }」中的全部 { $count } 封已加星號的郵件。
    }
    [unstarred] { $kind ->
        [conversation] 已選取「{ $folder }」中的全部 { $count } 個未加星號的會話群組。
       *[message] 已選取「{ $folder }」中的全部 { $count } 封未加星號的郵件。
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] 這裡沒有已讀取會話群組。
       *[message] 這裡沒有已讀取郵件。
    }
   *[unread] { $kind ->
        [conversation] 這裡沒有未讀取會話群組。
       *[message] 這裡沒有未讀取郵件。
    }
    [starred] { $kind ->
        [conversation] 這裡沒有已加星號的會話群組。
       *[message] 這裡沒有已加星號的郵件。
    }
    [unstarred] { $kind ->
        [conversation] 這裡沒有未加星號的會話群組。
       *[message] 這裡沒有未加星號的郵件。
    }
}
list-clear-selection = 清除選取

## Mail list: empty states

list-empty-search = 沒有符合搜尋條件的郵件。
list-empty-tab = 「{ $tab }」中沒有郵件。
list-empty-tab-unknown = 這個分頁中沒有郵件。
list-empty-folder = 「{ $folder }」中沒有郵件。
list-empty-folder-unknown = 這個資料夾中沒有郵件。
list-first-sync = 正在取得你的郵件…
list-first-sync-detail = 郵件送達後會顯示在這裡。

## Mail list: lines

row-removed = 這封郵件已遭移除。
row-starred = 已加星號
row-not-starred = 未加星號
row-important = 重要。按一下即可標示為不重要。
row-mark-important = 標示為重要
row-pinned = 已置頂
row-pin = 置頂
row-unpin = 取消置頂

## Mail list: More menu and right-click menu

menu-reply = 回覆
menu-reply-all = 全部回覆
menu-forward = 轉寄
menu-archive = 封存
menu-delete = 刪除
menu-delete-forever = 永久刪除
menu-move-to-inbox = 移至收件匣
menu-spam = 檢舉垃圾郵件
menu-not-spam = 非垃圾郵件
menu-mark-read = 標示為已讀取
menu-mark-unread = 標示為未讀取
menu-mark-all-read = 全部標示為已讀取
menu-star = 加上星號
menu-unstar = 移除星號
menu-important = 標示為重要
menu-not-important = 標示為不重要
menu-pin = 置頂
menu-unpin = 取消置頂
menu-print-all = 全部列印
menu-new-window = 在新視窗中開啟
menu-move-to = 移至
menu-move-to-heading = 移至：
menu-find-from = 搜尋來自 { $name } 的郵件

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] 已封存 { $count } 個會話群組。
   *[message] 已封存 { $count } 封郵件。
}
toast-trashed = { $kind ->
    [conversation] 已將 { $count } 個會話群組移至垃圾桶。
   *[message] 已將 { $count } 封郵件移至垃圾桶。
}
toast-moved = { $kind ->
    [conversation] 已移動 { $count } 個會話群組。
   *[message] 已移動 { $count } 封郵件。
}
toast-starred = { $kind ->
    [conversation] 已為 { $count } 個會話群組加上星號。
   *[message] 已為 { $count } 封郵件加上星號。
}
toast-unstarred = { $kind ->
    [conversation] 已移除 { $count } 個會話群組的星號。
   *[message] 已移除 { $count } 封郵件的星號。
}
toast-important = { $kind ->
    [conversation] 已將 { $count } 個會話群組標示為重要。
   *[message] 已將 { $count } 封郵件標示為重要。
}
toast-not-important = { $kind ->
    [conversation] 已將 { $count } 個會話群組標示為不重要。
   *[message] 已將 { $count } 封郵件標示為不重要。
}
toast-pinned = { $kind ->
    [conversation] 已置頂 { $count } 個會話群組。
   *[message] 已置頂 { $count } 封郵件。
}
toast-unpinned = { $kind ->
    [conversation] 已取消置頂 { $count } 個會話群組。
   *[message] 已取消置頂 { $count } 封郵件。
}
toast-spam = { $kind ->
    [conversation] 已將 { $count } 個會話群組檢舉為垃圾郵件。
   *[message] 已將 { $count } 封郵件檢舉為垃圾郵件。
}
toast-not-spam = { $kind ->
    [conversation] 已將 { $count } 個會話群組標示為非垃圾郵件並移至收件匣。
   *[message] 已將 { $count } 封郵件標示為非垃圾郵件並移至收件匣。
}
toast-deleted-forever = { $kind ->
    [conversation] 已永久刪除 { $count } 個會話群組。
   *[message] 已永久刪除 { $count } 封郵件。
}
toast-marked-read = { $kind ->
    [conversation] 已將 { $count } 個會話群組標示為已讀取。
   *[message] 已將 { $count } 封郵件標示為已讀取。
}
toast-marked-unread = { $kind ->
    [conversation] 已將 { $count } 個會話群組標示為未讀取。
   *[message] 已將 { $count } 封郵件標示為未讀取。
}
toast-undone = 已復原動作。
toast-nothing-to-undo = 沒有可復原的動作。
toast-cannot-undo-delete-forever = 永久刪除的郵件無法救回。
toast-send-undone = 已復原傳送。
toast-too-late-to-undo-send = 來不及復原：郵件已經寄出。
toast-undo = 復原
toast-no-spam-folder = 這個帳戶沒有垃圾郵件資料夾。
