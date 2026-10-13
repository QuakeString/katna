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
service-not-running = Katna 백그라운드 서비스가 실행 중이 아닙니다.
service-no-answer = Katna 백그라운드 서비스가 응답하지 않았습니다: { $error }
service-no-session = D-Bus 세션이 없습니다: { $error }

## Safe mode: an update left Katna's background service unable to start

safe-line = 업데이트에 문제가 있어 Katna가 안전 모드로 실행 중이며, 메일이 동기화되지 않습니다.
safe-try-again = 다시 시도
safe-restore = 복원
safe-restoring = { $when }의 데이터를 복원하는 중…
safe-restored = { $when }의 데이터를 복원했습니다. 이전에 있던 데이터는 폴더에 보관되어 있습니다.
safe-show-folder = 폴더 표시
safe-restore-failed = 데이터를 복원할 수 없습니다: { $error }
safe-restore-title = 업데이트 전의 데이터로 복원할까요?
safe-restore-body = Katna가 선택한 사본으로 돌아갑니다. 그 이후에 도착한 메일은 계정에서 다시 다운로드됩니다.
safe-restore-none = 아직 사본이 없습니다. Katna는 업데이트가 데이터를 변경하기 전마다 사본을 만듭니다.
safe-restore-keep = 보내지 않은 메일, 임시보관 메일, 아직 동기화되지 않은 변경 사항을 포함한 현재 데이터는 먼저 폴더에 보관되므로 아무것도 잃지 않습니다.
safe-restore-cancel = 취소
safe-restore-mail = 메일
safe-restore-pim = 계정 및 연락처
safe-restore-blobs = 첨부파일
safe-report-title = 디버그 보고서
safe-report-body = 이 내용을 복사하여 버그 신고에 첨부하세요. 메일, 주소, 비밀번호는 포함되어 있지 않습니다.
safe-report-restore = 복원…
safe-report-copied = 디버그 보고서를 복사했습니다
