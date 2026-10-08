# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = 郵件伺服器
problems-signed-out = { $provider } 已將 Katna 登出 { $address }，郵件已停止同步。
problems-password-refused = { $provider } 拒絕了 { $address } 的密碼，密碼可能已變更。
problems-no-answer = { $provider } 對 { $address } 沒有回應。Katna 會持續重試。
problems-offline = 你目前離線。你的郵件仍在這裡，寄出的郵件會等到你恢復連線後再傳送。
problems-accounts-need-you = { $count ->
   *[other] { $count } 個帳戶需要你處理
}
problems-show = 顯示
problems-later = 稍後
problems-new-password = 新密碼
problems-try-again = 再試一次

## The New password card

problems-password-title = 新密碼
problems-password-detail = { $provider } 拒絕了 { $address } 已儲存的密碼。請輸入新密碼；Katna 會先驗證再儲存。
problems-password-placeholder = 密碼
problems-password-show = 顯示密碼
problems-password-hide = 隱藏密碼
problems-password-cancel = 取消
problems-password-save = 儲存
problems-password-checking = 正在檢查…
problems-password-refused-again = { $provider } 也拒絕了這組密碼。請檢查後再試一次。
problems-password-saved = 已儲存 { $address } 的密碼。正在取得你的郵件…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $address } 的郵件伺服器未接受移動{ $count ->
   *[other] { $count } 封郵件，因此郵件已回到原本的位置。
}
problems-refused-flags = { $address } 的郵件伺服器未接受標示{ $count ->
   *[other] { $count } 封郵件（已讀取、已加星號…），因此已恢復原狀。
}
problems-refused-label = { $address } 的郵件伺服器未接受變更{ $count ->
   *[other] { $count } 封郵件的標籤，因此已恢復原狀。
}
problems-refused-delete = { $address } 的郵件伺服器未接受刪除{ $count ->
   *[other] { $count } 封郵件，因此郵件已恢復。
}
problems-refused-other = { $address } 的郵件伺服器未接受{ $count ->
   *[other] { $count } 項變更，因此 Katna 已恢復原狀。
}
problems-details = 詳細資料

## Katna's background service (katna-daemon) isn't running

service-starting = 正在啟動 Katna 的背景服務…
service-failed = Katna 的背景服務無法啟動，因此郵件未在同步。
service-start-again = 重新啟動
service-started-again = Katna 的背景服務曾停止，已重新啟動。
service-details-title = 服務無法啟動的原因
service-details-body = 請複製這段內容並隨回報一起傳送。其中不含任何郵件或密碼。
service-details-copy = 複製
service-details-close = 關閉
service-not-running = Katna 背景服務未執行。
service-no-answer = Katna 背景服務沒有回應：{ $error }
service-no-session = 沒有 D-Bus 工作階段：{ $error }

## Safe mode: an update left Katna's background service unable to start

safe-line = 更新發生問題後，Katna 已進入安全模式，因此郵件未在同步。
safe-try-again = 再試一次
safe-restore = 還原
safe-restoring = 正在從 { $when } 的副本還原你的資料…
safe-restored = 已從 { $when } 的副本還原你的資料。原本的資料保留在一個資料夾中。
safe-show-folder = 顯示資料夾
safe-restore-failed = 無法還原你的資料：{ $error }
safe-restore-title = 要還原更新前的資料嗎？
safe-restore-body = Katna 會回到你選擇的副本。之後收到的郵件會從你的帳戶重新下載。
safe-restore-none = 目前還沒有副本。每次更新變更你的資料之前，Katna 都會建立一份副本。
safe-restore-keep = 目前的資料（包括未傳送的郵件、草稿和尚未同步的變更）會先保留在一個資料夾中，因此不會遺失任何內容。
safe-restore-cancel = 取消
safe-restore-mail = 郵件
safe-restore-pim = 帳戶和聯絡人
safe-restore-blobs = 附件
safe-report-title = 偵錯報告
safe-report-body = 請複製這段內容並附加到你的錯誤回報中。其中不含任何郵件、地址或密碼。
safe-report-restore = 還原…
safe-report-copied = 已複製偵錯報告
