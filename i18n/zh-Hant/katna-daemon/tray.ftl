# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = 開啟收件匣(_I)
tray-new-message = 新郵件(_N)
tray-new-task = 新增工作(_T)
tray-new-note = 新增記事(_O)
tray-preferences = 設定(_S)
tray-quit = 結束(_Q)

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] 沒有未讀取的郵件
   *[other] { $count } 封未讀取的郵件
}
tray-password-refused = { $address } 需要新密碼
tray-signed-out = 請重新登入 { $address }
tray-accounts-need-you = { $count } 個帳戶需要你處理
tray-not-sent = { $count ->
   *[other] { $count } 封郵件未能傳送
}
