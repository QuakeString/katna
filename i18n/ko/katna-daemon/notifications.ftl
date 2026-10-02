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

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who }님이 { $subject } 메일을 열었습니다
notify-tracking-clicked = { $who }님이 { $subject } 메일의 링크를 클릭했습니다

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail을 업데이트할 수 있습니다
notify-update-ready-body = 버전 { $version }을(를) 다운로드했습니다. 업데이트를 누르면 설치되고 Katna Mail이 다시 시작됩니다.
notify-update = 업데이트

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
notify-reply-all = 전체답장
notify-mark-read = 읽음으로 표시
notify-mark-all-read = 모두 읽음으로 표시
notify-archive = 보관처리

## After Archive on a notification: a short note in the same place

notify-archived = 보관처리됨
notify-archived-count = { $count ->
   *[other] 메일 { $count }개를 받은편지함에서 옮겼습니다
}
notify-undo = 실행취소

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = { $name }님에게 답장을 보냈습니다
notify-open-in-katna = Katna에서 열기
