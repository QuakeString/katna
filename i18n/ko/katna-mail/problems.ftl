# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = 메일 서버
problems-signed-out = { $provider }에서 { $address }의 Katna 로그인을 해제했습니다. 메일 동기화가 중지되었습니다.
problems-password-refused = { $provider }에서 { $address }의 비밀번호를 거부했습니다. 비밀번호가 바뀌었을 수 있습니다.
problems-no-answer = { $provider }이(가) { $address }에 응답하지 않습니다. Katna가 계속 시도합니다.
problems-offline = 오프라인 상태입니다. 메일은 그대로 있으며, 보내는 메일은 다시 연결될 때까지 대기합니다.
problems-accounts-need-you = { $count ->
   *[other] 확인이 필요한 계정 { $count }개
}
problems-show = 표시
problems-later = 나중에
problems-new-password = 새 비밀번호
problems-try-again = 다시 시도

## The New password card

problems-password-title = 새 비밀번호
problems-password-detail = { $provider }에서 { $address }의 저장된 비밀번호를 거부했습니다. 새 비밀번호를 입력하세요. Katna가 확인한 후 저장합니다.
problems-password-placeholder = 비밀번호
problems-password-show = 비밀번호 표시
problems-password-hide = 비밀번호 숨기기
problems-password-cancel = 취소
problems-password-save = 저장
problems-password-checking = 확인하는 중…
problems-password-refused-again = { $provider }에서 이 비밀번호도 거부했습니다. 확인하고 다시 시도하세요.
problems-password-saved = { $address }의 비밀번호를 저장했습니다. 메일을 가져오는 중…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $count ->
   *[other] { $address }의 메일 서버가 메일 { $count }개의 이동을 받아들이지 않아 원래 위치로 되돌렸습니다.
}
problems-refused-flags = { $count ->
   *[other] { $address }의 메일 서버가 메일 { $count }개의 표시(읽음, 별표 등) 변경을 받아들이지 않아 원래대로 되돌렸습니다.
}
problems-refused-label = { $count ->
   *[other] { $address }의 메일 서버가 메일 { $count }개의 라벨 변경을 받아들이지 않아 원래대로 되돌렸습니다.
}
problems-refused-delete = { $count ->
   *[other] { $address }의 메일 서버가 메일 { $count }개의 삭제를 받아들이지 않아 되돌렸습니다.
}
problems-refused-other = { $count ->
   *[other] { $address }의 메일 서버가 변경사항 { $count }개를 받아들이지 않아 Katna가 원래대로 되돌렸습니다.
}
problems-details = 세부정보

## Katna's background service (katna-daemon) isn't running

service-starting = Katna 백그라운드 서비스를 시작하는 중…
service-failed = Katna 백그라운드 서비스가 시작되지 않아 메일이 동기화되지 않습니다.
service-start-again = 다시 시작
service-started-again = Katna 백그라운드 서비스가 중지되어 다시 시작했습니다.
service-details-title = 서비스가 시작되지 않는 이유
service-details-body = 이 내용을 복사하여 보고서와 함께 보내세요. 메일이나 비밀번호는 포함되어 있지 않습니다.
service-details-copy = 복사
service-details-close = 닫기
