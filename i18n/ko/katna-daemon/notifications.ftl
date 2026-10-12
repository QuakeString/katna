# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = 새 이메일 { $count }개
notify-and-more = 외 { $count }개
notify-no-subject = (제목 없음)
notify-unknown-sender = 알 수 없는 보낸사람

## Reminders the user asked for (same buttons)

notify-snooze-back = 다시 알림 시간이 되었습니다
notify-no-reply = 아직 답장 없음
notify-no-reply-to = “{ $subject }”에 아무도 답장하지 않았습니다.
notify-follow-up-sent = 후속 메일을 보냈습니다
notify-follow-up-sent-to = “{ $subject }”에 아무도 답장하지 않아 Katna가 후속 메일을 보냈습니다.
notify-follow-up-waiting = 후속 메일을 보내지 않음
notify-follow-up-waiting-to = 이 컴퓨터가 꺼져 있는 동안 보낼 시간이 지났습니다. “{ $subject }”이(가) 받은편지함으로 돌아왔습니다.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who }님이 { $subject } 메일을 열었습니다
notify-tracking-clicked = { $who }님이 { $subject } 메일의 링크를 클릭했습니다

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail을 업데이트할 수 있습니다
notify-update-ready-body = 버전 { $version }을(를) 다운로드했습니다. 업데이트를 누르면 설치되고 Katna Mail이 다시 시작됩니다.
notify-update = 업데이트

## Something needs the user, shown once per problem

notify-signed-out = 다시 로그인하세요
notify-signed-out-body = { $provider }에서 { $address }의 Katna 로그인을 해제했습니다. 메일 동기화가 중지되었습니다.
notify-sign-in = 로그인
notify-password-refused = 비밀번호가 거부됨
notify-password-refused-body = 메일 서버가 { $address }의 비밀번호를 거부했습니다. 비밀번호가 바뀌었을 수 있습니다.
notify-new-password = 새 비밀번호
notify-not-sent = “{ $subject }”을(를) 보내지 못했습니다
notify-not-sent-no-subject = 메일을 보내지 못했습니다
notify-not-sent-body = 보낼편지함에서 이유를 확인할 수 있습니다.
notify-open-outbox = 보낼편지함 열기

## Reminders of calendar events

notify-event-now = 지금
notify-event-in-minutes = { $count ->
   *[other] { $count }분 후
}
notify-event-in-hours = { $count ->
   *[other] { $count }시간 후
}
notify-event-in-days = { $count ->
    [1] 내일
   *[other] { $count }일 후
}
notify-event-all-day = 종일
notify-event-join = 참여
notify-event-snooze = 5분 후 다시 알림
notify-task-done = 완료로 표시

## The buttons of new-mail notifications and reminders

notify-open = 열기
notify-peek = 미리 보기
notify-reply = 답장
notify-reply-placeholder = { $name }님에게 답장…
notify-send = 보내기
notify-reply-quote-header = { $date }, { $from }님이 작성:
notify-reply-quote-header-no-date = { $from }님이 작성:
notify-reply-all = 전체답장
notify-mark-read = 읽음으로 표시
notify-mark-all-read = 모두 읽음으로 표시
notify-archive = 보관처리
notify-snooze-hour = 1시간 후 다시 알림
notify-snooze-tomorrow = 내일
notify-copy-code = { $code } 복사
notify-link-verify = { $domain }에서 인증
notify-link-confirm = { $domain }에서 확인
notify-link-activate = { $domain }에서 활성화

## After Archive on a notification: a short note in the same place

notify-archived = 보관처리됨
notify-archived-count = { $count ->
   *[other] 메일 { $count }개를 받은편지함에서 옮겼습니다
}
notify-undo = 실행취소

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = 코드를 복사했습니다
notify-code-not-copied = 코드를 복사할 수 없습니다

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = { $name }님에게 답장을 보냈습니다
notify-open-in-katna = Katna에서 열기
