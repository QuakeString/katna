# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = 新郵件
compose-restore = 還原
compose-minimize = 最小化
compose-exit-full-screen = 結束全螢幕
compose-open-window = 在新視窗中開啟
compose-save-close = 儲存並關閉
compose-back-to-mail = 返回郵件視窗
compose-pop-out-reply = 以獨立視窗回覆
compose-edit-recipients = 編輯收件者
compose-summary-cc = 副本：{ $names }
compose-summary-bcc = 密件副本：{ $names }
compose-more-recipients = 另外 { $count } 人
compose-show-trimmed = 顯示已省略的內容
compose-hide-trimmed = 隱藏已省略的內容
compose-remove-trimmed = 移除引用的文字
compose-trimmed-removed = 已移除引用的文字

## The quoted or forwarded message, in the mail itself

compose-quote-header = { $from } 於 { $date } 寫道：
compose-forward-header = ---------- 轉寄的郵件 ---------
compose-forward-from = 寄件者：{ $from }
compose-forward-date = 日期：{ $date }
compose-forward-subject = 主旨：{ $subject }
compose-forward-to = 收件者：{ $to }
compose-forward-cc = 副本：{ $cc }

## Recipients and subject

compose-to = 收件者
compose-cc = 副本
compose-bcc = 密件副本
compose-from = 寄件者
compose-from-choose = 從其他帳戶寄送
compose-recipients = 收件者
compose-subject = 主旨

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = 請先傳送或捨棄已開啟的郵件。
compose-bad-address = 「{ $address }」不是電子郵件地址。
compose-no-recipients = 請至少新增一位收件者。
compose-attachments-too-large = 附件共 { $size }；郵件伺服器最多只接受 { $limit }。
compose-no-account = 請新增用來寄送郵件的帳戶。
compose-past-time = 請選擇未來的時間。
compose-scheduling = 正在排定…
compose-sending = 正在傳送…
compose-scheduled = 已排定於 { $when } 傳送
compose-sent-archived = 已傳送並封存
compose-sent = 郵件已傳送
compose-discarded = 已捨棄草稿
compose-draft-saved = 草稿已儲存
compose-draft-saving = 正在儲存…
compose-draft-failed = 無法儲存草稿：{ $error }
compose-draft-not-opened = 無法開啟草稿。

## Attachments

compose-picker-insert = 插入
compose-picker-attach = 附加
compose-file-too-large = { $name } 太大：一封郵件最多只能夾帶 { $limit }。
compose-forward-files-missing = 轉寄郵件中的檔案尚未下載，因此不會附加。
compose-attachment-size = （{ $size }）
compose-remove-attachment = 移除附件
compose-attachment-open-tip = 開啟以檢查
compose-attachments-total = { $count } 個檔案，共 { $size }
compose-drive-note = { $name } 超過 { $limit }，因此會存入你的 Google Drive，郵件中會附上連結。
compose-drive-tip = 在你的 Google Drive 中；郵件中會附上連結
compose-drive-uploading = 正在上傳 { $percent }%
compose-drive-allow = 允許 Drive
compose-drive-allow-tip = 重新使用 Google 登入，讓 Katna 可以把大型檔案放入你的 Drive
compose-drive-retry = 再試一次
compose-drive-sends-when-uploaded = { $name } 上傳完成後即傳送
compose-drive-not-uploaded = { $name } 尚未上傳到 Google Drive
compose-drive-share-failed = 無法在 Google Drive 中共用這些檔案：{ $error }
compose-drive-share-title = 與所有人共用這些檔案？
compose-drive-share-text = { $count ->
   *[other] Google Drive 無法與沒有 Google 帳戶的 { $addresses } 共用這些檔案。改為讓任何擁有連結的人都可以開啟。
}
compose-drive-share-link = 透過連結共用
compose-drive-send-without = 不共用直接傳送
compose-drive-share-cancel = 取消
compose-drive-card-detail = { $size } · Google Drive
compose-drive-card-name = Google Drive
compose-onedrive-note = { $name } 超過 { $limit }，因此會存入你的 OneDrive，郵件中會附上連結。
compose-onedrive-tip = 在你的 OneDrive 中；郵件中會附上連結
compose-onedrive-allow = 允許 OneDrive
compose-onedrive-allow-tip = 重新使用 Microsoft 登入，讓 Katna 可以把大型檔案放入你的 OneDrive
compose-onedrive-not-uploaded = { $name } 尚未上傳到 OneDrive
compose-onedrive-share-failed = 無法在 OneDrive 中共用這些檔案：{ $error }
compose-onedrive-share-text = { $count ->
   *[other] OneDrive 無法與 { $addresses } 共用這些檔案。改為讓任何擁有連結的人都可以開啟。
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-onedrive-card-name = OneDrive
compose-drop-files = 將檔案拖放到這裡
compose-drop-here = 拖放到這裡

## Paste options (a small bar under what was just pasted or dropped)

compose-paste-keep-formatting = 保留格式
compose-paste-table = 表格
compose-paste-picture = 圖片
compose-paste-plain-text = 純文字
compose-paste-inline = 內嵌於內文
compose-paste-attachment = 附件

## Encryption and signing (the toggles by the recipients)

compose-encrypt = 加密
compose-encrypted = 已加密：只有收件者能閱讀
compose-sign = 簽署
compose-signed = 已簽署：收件者可以驗證郵件確實來自你

## Open and click tracking and read receipts (toggles after Sign)

compose-track = 追蹤開信和點閱
compose-tracked = 已追蹤：每位收件者開啟郵件或點開連結時，你都能看到
compose-track-clicks = 追蹤連結點閱（純文字郵件無法顯示是否已開啟）
compose-tracked-clicks = 已追蹤：每位收件者點開連結時，你都能看到
compose-track-sign-in = 登入 Katna 帳戶即可追蹤開信和點閱
compose-receipt = 要求已讀回條
compose-receipt-on = 已要求已讀回條：收件者的應用程式可能會詢問對方是否傳送回條
compose-delivery = 要求送達回條
compose-delivery-on = 已要求送達回條：每位收件者的伺服器接收郵件時，你的郵件伺服器會寄信通知你
compose-delivery-unavailable = 你的郵件伺服器不會傳送送達回條

## Spelling

spell-no-dictionary = 尚未安裝 { $language } 的拼字字典（例如 hunspell-en_us）。
spell-dictionary-error = 拼字字典：{ $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = 「{ $words }」
grammar-add = 新增「{ $words }」
grammar-remove = 移除「{ $words }」
grammar-ignore = 忽略

## Send checks (asked before a message goes out)

send-check-attachment-title = 你是否想附加檔案？
send-check-attachment-text = 你在郵件中提到了附件，但沒有附加任何檔案。
send-check-attach = 附加檔案
send-check-subject-title = 要在沒有主旨的情況下傳送嗎？
send-check-subject-text = 這封郵件沒有主旨。
send-check-add-subject = 新增主旨
send-check-send-anyway = 仍要傳送

## Recipients (To, Cc and Bcc)

recipient-not-valid = 不是有效的電子郵件地址
recipient-show-address = 顯示地址
recipient-remove = 移除
recipient-bad-title = 檢查地址
recipient-bad-text = 「{ $address }」不是有效的電子郵件地址。請在傳送前修正或移除。
recipient-bad-fix = 修正
