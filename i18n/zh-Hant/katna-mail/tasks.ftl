# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = 新增工作
tasks-all = 所有工作
tasks-today = 今天
tasks-upcoming = 即將到來
tasks-starred = 已加星號
tasks-completed-view = 已完成
tasks-new-list = 建立新清單
tasks-labels-heading = 標籤
tasks-on-this-computer = 這部電腦
tasks-my-tasks = 我的工作
tasks-account-sign-in = 重新登入以顯示工作
tasks-account-signed-in = 已重新登入 { $address }。正在取得你的工作…
tasks-account-sign-in-refused = { $provider } 未允許 Katna 存取。請再試一次，並允許存取你的工作。
tasks-account-refused = 伺服器未接受這組密碼。Yahoo、iCloud、Zoho 等需要應用程式密碼。
tasks-account-change-password = 變更密碼
tasks-account-change-password-tooltip = 輸入新密碼，Katna 會向伺服器驗證
tasks-account-not-enabled = Katna 的工作存取權限尚未開啟。
tasks-account-failed = 無法讀取工作清單。
tasks-account-error = 無法讀取工作清單：{ $reason }
tasks-account-none = 找不到工作清單
tasks-account-none-why = 找不到工作清單：{ $reason }
tasks-account-use-sign-in = { $provider } 只向使用 { $provider } 登入的 Katna 顯示工作。
tasks-account-sign-in-with = 使用 { $provider } 登入
tasks-account-looking = 正在尋找工作清單…
tasks-account-try-again = 再試一次
tasks-account-try-again-tooltip = 立即重新檢查此帳號的工作
tasks-account-fixing = 正在處理…
tasks-list-name-placeholder = 清單名稱

## Lists and tasks

tasks-loading = 正在讀取你的工作…
tasks-no-lists = 你的工作清單會顯示在這裡。
tasks-search = 搜尋工作
tasks-search-none = 沒有與搜尋相符的工作。
tasks-add = 新增工作
tasks-title-placeholder = 標題
tasks-add-step = 新增子工作
tasks-empty = 還沒有工作。請在上方新增。
tasks-starred-empty = 為工作加上星號後，就會顯示在這裡。
tasks-label-empty = 沒有含此標籤的未完成工作。
tasks-today-empty = 今天沒有到期的工作。
tasks-completed-empty = 你完成的工作會顯示在這裡。
tasks-upcoming-add = 新增{ $day }的工作
tasks-upcoming-overdue-day = { $weekday }，{ $day }
tasks-from-mail-quiet = 來自郵件
tasks-from-note-quiet = 來自記事
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }，{ $day }
tasks-overdue = 已逾期
tasks-completed = { $count ->
   *[other] 已完成 ({ $count })
}
tasks-list-options = 清單選項
tasks-sort-by = 排序方式
tasks-sort-my-order = 我的排序
tasks-sort-date = 日期
tasks-sort-starred = 最近加上星號
tasks-sort-title = 標題
tasks-rename-list = 重新命名清單
tasks-delete-list = 刪除清單
tasks-mark-done = 標示為已完成
tasks-mark-open = 標示為未完成
tasks-star = 加上星號
tasks-unstar = 移除星號
tasks-edit-title = 編輯標題
tasks-details = 詳細資料
tasks-delete = 刪除
tasks-move-to = 移至 { $list }
tasks-from-mail = 郵件
tasks-open-mail = 開啟郵件
tasks-from-note = 記事
tasks-open-note = 開啟記事
tasks-note-gone = 這則記事已不存在。
tasks-no-subject = （無主旨）

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
   *[other] 已選取 { $count } 項
}
tasks-select-clear = 清除選取
tasks-select-move = 移至清單
tasks-select-date = 設定日期
tasks-next-week = 下週

## The details dialog

tasks-notes-placeholder = 新增詳細資料
tasks-date = 日期
tasks-no-date = 無日期
tasks-time-placeholder = 新增時間
tasks-repeat = 重複
tasks-repeat-never = 不重複
tasks-repeat-daily = 每天
tasks-repeat-weekly = 每週
tasks-repeat-monthly = 每月
tasks-repeat-yearly = 每年
tasks-repeat-other = 自訂
tasks-remind = 提醒我
tasks-remind-off = 不提醒
tasks-remind-on-time = 準時
tasks-remind-morning = 當天 { $time }
tasks-remind-hour-before = 提前 1 小時
tasks-remind-day-before = 提前 1 天
tasks-label-add = 新增標籤
tasks-label-task = 為工作加上標籤
tasks-files-attach = 附加檔案
tasks-files-pick = 附加
tasks-file-open = 開啟
tasks-file-remove = 移除檔案
tasks-file-here = 僅在這部電腦上
tasks-cancel = 取消
tasks-save = 儲存
tasks-not-a-time = 「{ $text }」不是有效的時間，例如 { $example }。

## Due days

tasks-due-today = 今天
tasks-due-tomorrow = 明天
tasks-due-yesterday = 昨天
tasks-due-at = { $day } { $time }

## Notes at the bottom

tasks-toast-done = 工作已完成
tasks-toast-next = 已完成。下一次在 { $date }
tasks-toast-deleted = 工作已刪除
tasks-files-added = { $count ->
   *[other] 已附加 { $count } 個檔案
}
tasks-file-removed = 已移除「{ $name }」
tasks-files-left-out = 未附加：{ $names }。工作只能附加 { $limit } 以內的檔案，無法附加資料夾。
tasks-file-missing = 該檔案已不在這裡。
tasks-toast-added = { $count ->
   *[other] 已新增 { $count } 項工作
}
tasks-mail-gone = 這封郵件已不存在。
tasks-toast-list-deleted = 清單已刪除
tasks-toast-moved = 已移至 { $list }
tasks-toast-placed = 工作已移動
tasks-toast-rescheduled = 工作已重新安排
tasks-toast-rescheduled-several = { $count ->
   *[other] 已重新排定 { $count } 項工作
}
tasks-toast-done-several = { $count ->
   *[other] 已完成 { $count } 項工作
}
tasks-toast-open-several = { $count ->
   *[other] 已將 { $count } 項工作標示為未完成
}
tasks-toast-starred = { $count ->
   *[other] 已為 { $count } 項工作加上星號
}
tasks-toast-unstarred = { $count ->
   *[other] 已移除 { $count } 項工作的星號
}
tasks-toast-deleted-several = { $count ->
   *[other] 已刪除 { $count } 項工作
}
