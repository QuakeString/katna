# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = 받은편지함 열기(_I)
tray-new-message = 새 메일(_N)
tray-preferences = 설정(_S)
tray-quit = 끝내기(_Q)

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] 읽지 않은 메일 없음
   *[other] 읽지 않은 메일 { $count }개
}
