# Katna Mail, Korean (한국어): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = 오늘
calendar-today-tip = 오늘로 이동
calendar-view-day = 일
calendar-view-week = 주
calendar-view-month = 월
calendar-view-year = 연
calendar-view-schedule = 일정
calendar-view-days =
    { $count ->
       *[other] { $count }일
    }
calendar-options = 설정
calendar-density = 밀도
calendar-density-responsive = 화면에 맞게 조정
calendar-density-comfortable = 편안하게
calendar-density-compact = 간결하게
calendar-custom-days = 사용자 지정 보기
calendar-second-zone = 보조 시간대
calendar-zone-none = 없음
calendar-zone = { $zone } ({ $offset })
calendar-share-free = 빈 시간 공유
calendar-free-subject = 제가 비는 시간
calendar-free-intro = 비는 시간을 알려 드립니다({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = 앞으로 며칠간의 근무일에는 비는 시간이 없습니다.
calendar-previous-day = 전날
calendar-next-day = 다음 날
calendar-previous-week = 이전 주
calendar-next-week = 다음 주
calendar-previous-month = 이전 달
calendar-next-month = 다음 달
calendar-previous-year = 이전 해
calendar-next-year = 다음 해
calendar-previous-period = 이전
calendar-next-period = 다음
calendar-title-months = { $first } – { $last }
calendar-loading = 로드 중…
calendar-read-failed = 캘린더를 읽을 수 없습니다: { $error }
calendar-sets = 캘린더 세트
calendar-set-add = 표시 중인 캘린더를 세트로 저장
calendar-set-name = 세트 이름
calendar-set-remove = 세트 삭제
calendar-local = 이 컴퓨터
calendar-account-gone = 삭제된 계정
calendar-account-sign-in = 캘린더를 표시하려면 다시 로그인하세요
calendar-account-signed-in = { $address }에 다시 로그인했습니다. 캘린더를 가져오는 중…
calendar-account-sign-in-refused = { $provider }에서 Katna의 접근을 허용하지 않았습니다. 다시 시도하고 캘린더에 대한 접근을 허용하세요.
calendar-account-refused = 서버에서 비밀번호를 받아들이지 않았습니다. Yahoo, iCloud, Zoho 등은 앱 비밀번호가 필요합니다.
calendar-account-change-password = 비밀번호 변경
calendar-account-change-password-tooltip = 새 비밀번호를 입력하세요. Katna가 서버에서 확인합니다
calendar-account-not-enabled = Katna의 캘린더 접근이 아직 켜져 있지 않습니다.
calendar-account-failed = 캘린더를 읽을 수 없습니다.
calendar-account-error = 캘린더를 읽을 수 없습니다: { $reason }
calendar-account-none = 캘린더를 찾을 수 없음
calendar-account-none-why = 캘린더를 찾을 수 없음: { $reason }
calendar-account-use-sign-in = { $provider }에서는 { $provider }로 로그인한 Katna에만 캘린더를 보여 줍니다.
calendar-account-sign-in-with = { $provider }로 로그인
calendar-account-looking = 캘린더를 찾는 중…
calendar-account-try-again = 다시 시도
calendar-account-try-again-tooltip = 지금 이 계정의 캘린더를 다시 확인
calendar-account-fixing = 해결하는 중…
calendar-birthdays = 생일
calendar-tasks = 할 일
calendar-birthday-of = { $name }님의 생일
calendar-empty-title = 아직 캘린더가 없습니다
calendar-empty-text = Google 및 Microsoft 계정의 캘린더는 동기화되면 여기에 표시됩니다. CalDAV를 지원하는 다른 서버의 캘린더도 표시됩니다.
calendar-schedule-empty = 향후 2개월 동안 예정된 일정이 없습니다.
calendar-search = 일정 검색
calendar-search-past = 지난 일정
calendar-search-none = 검색과 일치하는 일정이 없습니다.
calendar-no-title = (제목 없음)
calendar-all-day = 종일
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count }개 더보기
calendar-peek-day = { $day } { $weekday }
calendar-repeats = 반복
calendar-join = 참여
calendar-join-with = { $service }(으)로 참여
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
calendar-open-mail = 메일 열기
calendar-open-contact = 연락처 열기
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
calendar-menu-new-event = 새 일정
calendar-event-window-title = 새 일정
calendar-menu-open-day = 날짜 열기
calendar-menu-duplicate = 복제
calendar-menu-color = 색상
calendar-menu-color-calendar = 캘린더 색상
calendar-menu-in-a-week = 1주 후
calendar-color-tomato = 토마토
calendar-color-flamingo = 플라밍고
calendar-color-tangerine = 귤
calendar-color-banana = 바나나
calendar-color-sage = 세이지
calendar-color-basil = 바질
calendar-color-peacock = 공작
calendar-color-blueberry = 블루베리
calendar-color-lavender = 라벤더
calendar-color-grape = 포도
calendar-color-graphite = 흑연
calendar-menu-only-this = 이 캘린더만 표시
calendar-menu-rename = 이름 바꾸기
calendar-menu-remove = 목록에서 삭제
calendar-menu-delete = 삭제
calendar-menu-new-calendar = 새 캘린더
calendar-menu-show-all = 모두 표시
calendar-menu-hide-all = 모두 숨기기
calendar-menu-account-settings = 계정 설정
calendar-why-main = 기본 캘린더
calendar-why-last = 하나뿐임
calendar-why-owner = 소유자만 가능
calendar-why-contacts = 연락처에서 가져옴
calendar-why-unreached = 연결 안 됨
calendar-name-placeholder = 캘린더 이름
calendar-toast-added = “{ $name }” 캘린더를 추가했습니다
calendar-toast-renamed = 캘린더 이름을 바꿨습니다
calendar-toast-recolored = 캘린더 색상을 바꿨습니다
calendar-toast-deleted = “{ $name }” 캘린더를 삭제했습니다
calendar-toast-removed = “{ $name }” 캘린더를 목록에서 삭제했습니다
calendar-edit-failed = 캘린더를 변경하지 못했습니다: { $reason }
calendar-delete-title = “{ $name }” 캘린더를 삭제할까요?
calendar-delete-confirm = 삭제
calendar-deleting = 삭제하는 중…
calendar-delete-heading = 삭제되는 항목:
calendar-delete-events = 캘린더와 모든 일정
calendar-delete-shared = 공유된 모든 사람에게서
calendar-delete-server = Katna에서만이 아니라 메일 서비스의 { $account } 계정에서도 삭제됩니다.
calendar-delete-local = 이 컴퓨터에서 삭제됩니다.
calendar-remove-title = “{ $name }” 캘린더를 목록에서 삭제할까요?
calendar-remove-confirm = 삭제
calendar-removing = 삭제하는 중…
calendar-remove-heading = 변경되는 사항:
calendar-remove-events = 여기와 다른 앱에서 이 캘린더의 일정이 더 이상 표시되지 않습니다
calendar-remove-server = 캘린더는 소유자에게 남아 있으며, 소유자가 다시 공유할 수 있습니다.
calendar-kind-event = 일정
calendar-kind-task = 할 일
calendar-kind-focus = 집중 시간
calendar-kind-out-of-office = 부재중
calendar-kind-working-location = 근무 위치
calendar-task-added = 할 일을 추가했습니다
calendar-task-added-to = { $list }에 할 일을 추가했습니다
calendar-task-list-local = 이 컴퓨터
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
