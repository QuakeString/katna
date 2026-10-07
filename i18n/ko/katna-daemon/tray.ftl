# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = 받은편지함 열기(_I)
tray-new-message = 새 메일(_N)
tray-new-task = 새 할 일(_T)
tray-new-note = 새 메모(_O)
tray-preferences = 설정(_S)
tray-quit = 끝내기(_Q)

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] 읽지 않은 메일 없음
   *[other] 읽지 않은 메일 { $count }개
}
tray-password-refused = { $address }의 새 비밀번호가 필요합니다
tray-signed-out = { $address }에 다시 로그인하세요
tray-accounts-need-you = 확인이 필요한 계정 { $count }개
tray-not-sent = { $count ->
   *[other] 메일 { $count }개를 보내지 못했습니다
}
