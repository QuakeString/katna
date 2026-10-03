# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = 受信トレイを開く(_I)
tray-new-message = 新規メール(_N)
tray-preferences = 設定(_S)
tray-quit = 終了(_Q)

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] 未読メールはありません
   *[other] 未読メッセージ { $count } 件
}
