# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = 다시 알림 시간…
snooze-later-today = 오늘 늦게
snooze-tomorrow = 내일
snooze-this-weekend = 이번 주말
snooze-next-week = 다음 주
snooze-pick = 날짜 및 시간 선택
snooze-back = 시간 목록으로 돌아가기
snooze-type-placeholder = 시간 입력
snooze-type-hint = 예: “화 오후 3시”, “내일”, “2시간 후”
snooze-type-hint-unclear = Katna가 시간으로 인식할 수 없습니다
snooze-type-unclear = “{ $text }”은(는) Katna가 인식할 수 있는 시간이 아닙니다

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = 다시 알림
remind-tab = 알림 받기
snooze-says = 그때까지 숨깁니다
remind-says = 그대로 두고 알려 드립니다
remind-before-due = 기한 전
remind-note = 메모(선택사항)
remind-note-placeholder = 비워 두면 제목
toast-remind-set = { $date }에 알림이 설정되었습니다
remind-chat-line = 알림 { $date } · { $title }
remind-done = 완료
toast-remind-done = 알림을 완료했습니다
snooze-chat-line = { $date }까지 다시 알림
snooze-chat-change = 변경

## The date and time picker

snooze-cancel = 취소
snooze-save = 저장
snooze-in-the-past = 현재 이후의 시간을 선택하세요.

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = 답장이 없으면 후속 조치…
follow-up-title = 답장이 없으면 후속 조치
follow-up-off = 끄기
follow-up-days = { $days ->
   *[other] { $days }일
}
follow-up-weeks = { $weeks ->
   *[other] { $weeks }주
}
follow-up-pick = 선택…
follow-up-pick-title = 다음 시간까지 답장이 없으면 후속 조치
follow-up-remind = 알림 받기
follow-up-remind-note = 대화가 받은편지함 맨 위로 돌아옵니다
follow-up-send = 후속 메일 대신 보내기
follow-up-send-note = 같은 사람들에게 같은 대화로 보냅니다
follow-up-send-encrypted = 암호화된 메일에는 사용할 수 없음
follow-up-text-placeholder = 작성할 내용
follow-up-text-named = { $name }님, 안녕하세요. 아래 메일을 확인하셨는지 여쭤봅니다.
follow-up-text = 안녕하세요. 아래 메일을 확인하셨는지 여쭤봅니다.
follow-up-template = 템플릿 사용
follow-up-signature = 서명이 추가됩니다
follow-up-again = 그래도 답장이 없으면 다음 기간 후 다시 후속 조치
follow-up-note = 대화의 누군가가 답장하면 바로 중지됩니다. 자동 응답은 포함되지 않습니다.
follow-up-note-send = 대화의 누군가가 답장하면 바로 중지됩니다. 평일 { $start }~{ $end }에 보내며, 하루 넘게 늦게 보내지는 않습니다.
follow-up-cancel = 취소
follow-up-done = 완료
follow-up-chip-send = { $time } 후 후속 메일
follow-up-chip-remind = { $time } 후 알림
follow-up-chip-send-on = 후속 메일 { $date }
follow-up-chip-remind-on = 알림 { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = 아직 답장 없음
follow-up-card-title-waiting = 후속 메일이 대기 중입니다
follow-up-card-send = Katna가 { $date }에 후속 메일을 보냅니다. 누군가 답장하면 중지됩니다.
follow-up-card-send-twice = Katna가 { $date }에 후속 메일을 보내고, 나중에 한 번 더 보냅니다. 누군가 답장하면 중지됩니다.
follow-up-card-remind = 아무도 답장하지 않으면 이 대화가 { $date }에 받은편지함으로 돌아옵니다.
follow-up-card-waiting = 컴퓨터가 꺼져 있는 동안 보낼 시간이 지나서 늦게 보내지 않았습니다. 지금 보내거나, 새 시간을 선택하거나, 중지하세요.
follow-up-card-edit = 수정
follow-up-card-edit-title = 후속 조치 시간
follow-up-card-send-now = 지금 보내기
follow-up-card-stop = 중지
follow-up-chat-send = 후속 메일 · 아무도 답장하지 않으면 { $date }
follow-up-chat-step = 후속 메일 { $step }/{ $steps } · 아무도 답장하지 않으면 { $date }
follow-up-chat-waiting = 후속 메일 대기 중 · 컴퓨터가 꺼져 있는 동안 보낼 시간이 지났습니다
follow-up-chat-remind = 답장이 없으면 { $date }에 받은편지함으로 돌아옴
toast-follow-up-sent = 후속 메일을 보냈습니다
toast-follow-up-stopped = 후속 조치를 중지했습니다
toast-follow-up-moved = 후속 조치를 { $date }(으)로 옮겼습니다
nudge-row = { $days ->
   *[other] { $days }일 전에 보냄
}. 후속 조치할까요?
nudge-row-tip = 대화의 모든 사람에게 후속 메일 쓰기
nudge-follow-up = 후속 조치
nudge-dismiss = 닫기
nudge-card-title = 아직 답장 없음
nudge-card-text = { $days ->
   *[other] { $days }일 전에
} 질문했지만 아무도 답하지 않았습니다.
nudge-chat-line = { $days ->
   *[other] { $days }일 전에 보냄
}, 아직 답장 없음
toast-nudge-dismissed = 알림을 닫았습니다
