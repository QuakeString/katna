# Katna Mail, Korean (한국어): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = 오늘
calendar-today-tip = 오늘로 이동
calendar-view-day = 일
calendar-view-week = 주
calendar-view-month = 월
calendar-view-schedule = 일정
calendar-previous-day = 전날
calendar-next-day = 다음 날
calendar-previous-week = 이전 주
calendar-next-week = 다음 주
calendar-previous-month = 이전 달
calendar-next-month = 다음 달
calendar-previous-period = 이전
calendar-next-period = 다음
calendar-title-months = { $first } – { $last }
calendar-loading = 로드 중…
calendar-read-failed = 캘린더를 읽을 수 없습니다: { $error }
calendar-local = 이 컴퓨터
calendar-account-gone = 삭제된 계정
calendar-empty-title = 아직 캘린더가 없습니다
calendar-empty-text = Google 및 Microsoft 계정의 캘린더는 동기화되면 여기에 표시됩니다. CalDAV를 지원하는 다른 서버의 캘린더도 표시됩니다.
calendar-schedule-empty = 향후 2개월 동안 예정된 일정이 없습니다.
calendar-no-title = (제목 없음)
calendar-all-day = 종일
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count }개 더보기
calendar-repeats = 반복
calendar-join = 참여
calendar-email-guests = 게스트에게 이메일 보내기
calendar-running-late = 늦는 중
calendar-late-subject = 늦는 중: { $title }
calendar-late-body = 죄송합니다. { $title }에 몇 분 늦을 것 같습니다. 곧 도착하겠습니다.
calendar-guests =
    { $count ->
       *[other] 게스트 { $count }명
    }
calendar-guest-answers = 예 { $yes }, 미정 { $maybe }, 아니요 { $no }, 응답 대기 중 { $waiting }
calendar-organizer = 주최자
calendar-optional = 선택사항
calendar-open-web = 브라우저에서 열기
calendar-close = 닫기

## Adding, changing and deleting events.

calendar-add-title = 제목 추가
calendar-add-location = 위치 추가
calendar-add-notes = 설명 추가
calendar-add-guests = 게스트 추가
calendar-remove-guest = 삭제
calendar-add-meet = Google Meet 화상 회의 추가
calendar-add-teams = Teams 회의 추가
calendar-has-call = 화상 통화가 추가됨
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = 종일
calendar-more-options = 옵션 더보기
calendar-save = 저장
calendar-saved = 일정을 저장했습니다
calendar-deleted = 일정을 삭제했습니다
calendar-discard = 변경사항 삭제
calendar-edit = 일정 수정
calendar-delete = 일정 삭제
calendar-event-details = 일정 세부정보
calendar-kind-event = 일정
calendar-kind-focus = 집중 시간
calendar-kind-out-of-office = 부재중
calendar-kind-working-location = 근무 위치
calendar-working-home = 집
calendar-busy = 바쁨
calendar-free = 한가함
calendar-cancel = 취소
calendar-ok = 확인
calendar-read-only = 이 캘린더의 일정은 수정할 수 없습니다
calendar-none-editable = 일정을 추가할 수 있는 캘린더가 아직 없습니다
calendar-no-such-time = 사용 중인 시간대에는 없는 시간입니다
calendar-end-before-start = 일정이 시작하기 전에 끝납니다
calendar-repeat-never = 반복 안함
calendar-repeat-daily = 매일
calendar-repeat-weekly = 매주 { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] 매월 첫 번째 { $weekday }
        [2] 매월 두 번째 { $weekday }
        [3] 매월 세 번째 { $weekday }
        [4] 매월 네 번째 { $weekday }
       *[other] 매월 마지막 { $weekday }
    }
calendar-repeat-yearly = 매년 { $day }
calendar-repeat-weekdays = 주중 매일(월요일~금요일)
calendar-repeat-custom = 맞춤
calendar-reminder-none = 알림 없음
calendar-reminder-at-start = 시작 시
calendar-reminder-minutes =
    { $count ->
       *[other] { $count }분 전
    }
calendar-reminder-hours =
    { $count ->
       *[other] { $count }시간 전
    }
calendar-reminder-days =
    { $count ->
       *[other] { $count }일 전
    }
calendar-scope-edit-title = 반복 일정 수정
calendar-scope-delete-title = 반복 일정 삭제
calendar-scope-this = 이 일정
calendar-scope-following = 이 일정 및 향후 일정
calendar-scope-all = 모든 일정
calendar-scope-respond-title = 반복 일정에 대한 응답
calendar-going = 참석하시나요?
calendar-answer-yes = 예
calendar-answer-no = 아니요
calendar-answer-maybe = 미정
calendar-answered-yes = 참석합니다
calendar-answered-no = 참석하지 않습니다
calendar-answered-maybe = 참석할 수도 있습니다

## The card at the top of a mail with an invitation.

calendar-invite = 초대장
calendar-invite-cancelled = 일정이 취소되었습니다
calendar-invite-reply = { $name }님이 응답했습니다
calendar-invite-reply-yes = { $name }님이 수락했습니다
calendar-invite-reply-no = { $name }님이 거절했습니다
calendar-invite-reply-maybe = { $name }님이 참석할 수도 있습니다
calendar-invite-organizer = 주최자: { $name }
calendar-invite-open = 캘린더에서 열기
calendar-invite-not-yet = 아직 캘린더에 없습니다. 동기화되면 응답할 수 있습니다.
calendar-invite-by-mail = 내 캘린더에 없음: 응답은 주최자에게 메일로 전송됩니다.
calendar-mail-yes = 수락함: { $title }
calendar-mail-yes-body = { $name }님이 이 초대를 수락했습니다.
calendar-mail-no = 거절함: { $title }
calendar-mail-no-body = { $name }님이 이 초대를 거절했습니다.
calendar-mail-maybe = 미정: { $title }
calendar-mail-maybe-body = { $name }님이 이 초대를 잠정 수락했습니다.
calendar-invite-your-day = 내 하루
calendar-invite-clashes =
    { $count ->
       *[other] 일정 { $count }개와 겹칩니다
    }

## The day's agenda beside the mail.

agenda-show = 오늘 일정 표시
agenda-hide = 일정 숨기기
agenda-today = 오늘, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = 이 날은 예정된 일정이 없습니다.
