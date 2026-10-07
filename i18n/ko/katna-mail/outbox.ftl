# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = { $reason } 보내지 못했습니다.
outbox-retrying = { $reason } 아직 보내지 못했습니다. Katna가 자동으로 다시 시도합니다.
outbox-waiting-sign-in = { $address }에 다시 로그인하기를 기다리는 중입니다. 로그인하면 전송됩니다.
outbox-waiting-password = { $address }의 새 비밀번호를 기다리는 중입니다. 입력하면 전송됩니다.
outbox-waiting-connection = 연결을 기다리는 중입니다. 다시 온라인 상태가 되면 전송됩니다.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = 받는사람이 없어
outbox-reason-address = 받는사람 주소가 존재하지 않아
outbox-reason-too-large = 메일 서버에 비해 크기가 너무 커서
outbox-reason-blocked = 메일 서버가 차단하여
outbox-reason-gone = 이 컴퓨터에 있던 사본이 없어져
outbox-reason-refused = 메일 서버가 거부하여

## Buttons and notes

outbox-try-again = 다시 시도
outbox-edit = 수정
outbox-delete = 삭제
outbox-deleted = 보낼편지함에서 삭제했습니다
outbox-sending-again = 다시 보내는 중…
outbox-snackbar-not-sent = “{ $subject }” 메일을 { $reason } 보내지 못했습니다.
outbox-open = 보낼편지함
