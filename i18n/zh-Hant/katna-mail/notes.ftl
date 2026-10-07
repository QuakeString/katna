# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = 記事
notes-view-reminders = 提醒
notes-view-archive = 封存
notes-view-trash = 垃圾桶
notes-edit-labels = 編輯標籤
notes-search = 搜尋記事
notes-loading = 正在開啟你的記事…

## Board

notes-take-a-note = 新增記事…
notes-new-list = 新增清單
notes-new-note = 新增記事
notes-pinned = 已置頂
notes-others = 其他
notes-empty = 你新增的記事會顯示在這裡
notes-archive-empty = 已封存的記事會顯示在這裡
notes-trash-empty = 垃圾桶中沒有記事
notes-none-found = 找不到相符的記事
notes-label-empty = 還沒有含此標籤的記事
notes-reminders-empty = 有即將到來提醒的記事會顯示在這裡
notes-trash-note = 垃圾桶中的記事會在 7 天後刪除。
notes-empty-trash = 清空垃圾桶
notes-ticked = { $count ->
   *[other] + { $count } 個已勾選項目
}
notes-select = 選取記事
notes-selected = { $count ->
   *[other] 已選取 { $count } 則
}
notes-select-clear = 清除選取

## A note's buttons

notes-pin = 置頂記事
notes-unpin = 取消置頂記事
notes-archive = 封存
notes-unarchive = 取消封存
notes-delete = 刪除記事
notes-restore = 還原
notes-delete-forever = 永久刪除
notes-color = 背景選項
notes-checkboxes = 顯示/隱藏核取方塊
notes-labels = 標籤
notes-close = 關閉
notes-more = 更多
notes-make-copy = 建立副本
notes-remind = 提醒我
notes-add-picture = 新增圖片
notes-history = 版本記錄
notes-ai = 協助我撰寫
notes-send-as-mail = 以郵件傳送
notes-save-markdown = 另存為 Markdown
notes-save-pdf = 另存為 PDF

## The open note

notes-title = 標題
notes-edited = 編輯時間：{ $date }
notes-on-this-computer = 這部電腦
notes-where = 記事的儲存位置
notes-untitled = 未命名記事

## Pictures

notes-picture-choose = 新增圖片
notes-picture-remove = 移除圖片
notes-picture-too-big = 記事中的圖片最大可為 { $size }
notes-picture-kind = 這個檔案不是 Katna 能顯示的圖片
notes-picture-unreadable = 無法讀取 { $name }：{ $error }

## Reminders

notes-remind-me = 提醒我
notes-remind-off = 移除提醒
notes-remind-in-the-past = 請選擇尚未過去的時間
notes-remind-today = 今天 { $time }
notes-remind-tomorrow = 明天 { $time }
notes-remind-weekday = { $day } { $time }
notes-reminder-set = 已設定提醒：{ $when }
notes-reminder-off = 已移除提醒

## Links between notes

notes-link-note = 連結記事
notes-link-new = 新增記事「{ $title }」
notes-linked-from = 連結來源
notes-link-gone = 該記事已不存在

## Version history

notes-versions = 版本
notes-version-now = 目前
notes-version-here = 你，在這部電腦上
notes-version-yesterday = 昨天 { $time }
notes-version-changes = { $count ->
   *[other] { $count } 處變更
}
notes-version-from = 來自 { $device }
notes-version-elsewhere = 來自其他裝置
notes-version-created = 建立時
notes-version-restore = 還原此版本
notes-version-restored = 已還原版本
notes-history-none = 尚無較早的版本

## AI help

notes-ai-tidy = 整理文字
notes-ai-checklist = 轉換為核取清單
notes-ai-summarise = 摘要
notes-ai-empty = 請先寫點內容
notes-ai-tidied = 已整理文字。按 Ctrl+Z 可復原。
notes-ai-listed = 已轉換為核取清單。按 Ctrl+Z 可復原。
notes-ai-summarised = 已在頂端加上摘要

## Labels

notes-label-note = 為記事加上標籤
notes-label-name = 輸入標籤名稱
notes-label-create = 建立「{ $name }」
notes-label-remove = 移除標籤
notes-label-delete = 刪除標籤
notes-labels-none = 尚無標籤。可從記事的標籤按鈕新增。
notes-labels-done = 完成
notes-label-renamed = 標籤已重新命名為「{ $name }」
notes-label-deleted = 已刪除標籤「{ $name }」

## A note about a mail

notes-mail = 郵件
notes-open-mail = 開啟郵件
notes-open-note = 開啟記事

## Meeting notes

notes-meeting-take = 撰寫會議記事
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = 與會者：{ $names }
notes-meeting-notes = 記事
notes-meeting-actions = 待辦事項
notes-event = 活動
notes-open-event = 開啟活動

## Formatting

notes-format = 格式
notes-format-heading-1 = 標題 1
notes-format-heading-2 = 標題 2
notes-format-normal = 內文
notes-format-bold = 粗體
notes-format-italic = 斜體
notes-format-underline = 底線
notes-format-quote = 引文
notes-format-code = 程式碼
notes-format-divider = 分隔線
notes-format-clear = 清除格式

## Tasks

notes-make-task = 設為工作

## Colors (tooltips)

notes-color-none = 無顏色
notes-color-coral = 珊瑚色
notes-color-peach = 桃色
notes-color-sand = 沙色
notes-color-mint = 薄荷色
notes-color-sage = 鼠尾草色
notes-color-fog = 霧灰色
notes-color-storm = 暴風雨色
notes-color-dusk = 暮色
notes-color-blossom = 花朵色
notes-color-clay = 黏土色
notes-color-chalk = 粉筆色

## Messages at the foot of the window

notes-archived = 記事已封存
notes-unarchived = 記事已取消封存
notes-trashed = 記事已移至垃圾桶
notes-restored = 記事已還原
notes-saved = 記事已儲存
notes-pinned-count = { $count ->
   *[other] 已置頂 { $count } 則記事
}
notes-unpinned-count = { $count ->
   *[other] 已取消置頂 { $count } 則記事
}
notes-colored-count = { $count ->
   *[other] 已變更 { $count } 則記事的顏色
}
notes-archived-count = { $count ->
   *[other] 已封存 { $count } 則記事
}
notes-unarchived-count = { $count ->
   *[other] 已取消封存 { $count } 則記事
}
notes-trashed-count = { $count ->
   *[other] 已將 { $count } 則記事移至垃圾桶
}
notes-restored-count = { $count ->
   *[other] 已還原 { $count } 則記事
}
notes-copied-count = { $count ->
   *[other] 已建立 { $count } 份副本
}
notes-empty-discarded = 已捨棄空白記事
notes-mail-gone = 這封郵件已不存在
notes-deleted-forever = { $count ->
   *[other] 已永久刪除 { $count } 則記事
}
