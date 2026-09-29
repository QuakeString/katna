# Katna Mail, Korean (한국어): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = 만들기
tasks-all = 모든 할 일
tasks-today = 오늘
tasks-starred = 별표 표시됨
tasks-new-list = 새 목록 만들기
tasks-on-this-computer = 이 컴퓨터
tasks-my-tasks = 내 할 일
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = 할 일을 표시하려면 다시 로그인하세요
tasks-account-signed-in = { $address }에 다시 로그인했습니다. 할 일을 가져오는 중…
tasks-account-sign-in-refused = { $provider }에서 Katna의 접근을 허용하지 않았습니다. 다시 시도하고 할 일에 대한 접근을 허용하세요.
tasks-account-refused = 서버에서 비밀번호를 받아들이지 않았습니다. Yahoo, iCloud, Zoho 등은 앱 비밀번호가 필요합니다.
tasks-account-change-password = 비밀번호 변경
tasks-account-change-password-tooltip = 설정 > 계정 열기
tasks-account-not-enabled = Katna의 할 일 접근이 아직 켜져 있지 않습니다.
tasks-account-failed = 할 일 목록을 읽을 수 없습니다.
# $reason is the server's own words, in English.
tasks-account-error = 할 일 목록을 읽을 수 없습니다: { $reason }
tasks-account-none = 할 일 목록을 찾을 수 없음
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = 할 일 목록을 찾을 수 없음: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider }에서는 { $provider }로 로그인한 Katna에만 할 일을 보여 줍니다.
tasks-account-sign-in-with = { $provider }로 로그인
tasks-account-looking = 할 일 목록을 찾는 중…
tasks-account-try-again = 다시 시도
tasks-account-try-again-tooltip = 지금 이 계정의 할 일을 다시 확인
tasks-account-fixing = 해결하는 중…
tasks-list-name-placeholder = 목록 이름

## Lists and tasks

tasks-loading = 할 일을 읽는 중…
tasks-no-lists = 할 일 목록이 여기에 표시됩니다.
tasks-search = 할 일 검색
tasks-search-none = 검색과 일치하는 할 일이 없습니다.
tasks-add = 할 일 추가
tasks-title-placeholder = 제목
tasks-add-step = 하위 할 일 추가
tasks-empty = 아직 할 일이 없습니다. 위에서 추가하세요.
tasks-starred-empty = 할 일에 별표를 표시하면 여기에 나타납니다.
tasks-today-empty = 오늘 마감인 할 일이 없습니다.
tasks-today-date = { $weekday }, { $day }
tasks-overdue = 기한 경과
tasks-completed = { $count ->
   *[other] 완료됨 ({ $count })
}
tasks-list-options = 목록 옵션
tasks-rename-list = 목록 이름 바꾸기
tasks-delete-list = 목록 삭제
tasks-mark-done = 완료로 표시
tasks-mark-open = 미완료로 표시
tasks-star = 별표 표시
tasks-unstar = 별표 삭제
tasks-edit-title = 제목 수정
tasks-details = 세부정보
tasks-delete = 삭제
tasks-move-to = { $list }(으)로 이동
tasks-from-mail = 메일
tasks-open-mail = 메일 열기
tasks-from-note = 메모
tasks-open-note = 메모 열기
tasks-note-gone = 해당 메모가 더 이상 없습니다.
tasks-no-subject = (제목 없음)

## The details dialog

tasks-notes-placeholder = 세부정보 추가
tasks-date = 날짜
tasks-no-date = 날짜 없음
tasks-time-placeholder = 시간 추가
tasks-repeat = 반복
tasks-repeat-never = 반복 안함
tasks-repeat-daily = 매일
tasks-repeat-weekly = 매주
tasks-repeat-monthly = 매월
tasks-repeat-yearly = 매년
tasks-repeat-other = 맞춤설정
tasks-remind = 알림
tasks-remind-off = 알리지 않음
tasks-remind-on-time = 정시
tasks-remind-morning = 당일 { $time }
tasks-remind-hour-before = 1시간 전
tasks-remind-day-before = 하루 전
tasks-cancel = 취소
tasks-save = 저장
tasks-not-a-time = “{ $text }”은(는) 시간이 아닙니다. 예: { $example }

## Due days

tasks-due-today = 오늘
tasks-due-tomorrow = 내일
tasks-due-yesterday = 어제
tasks-due-at = { $day } { $time }

## Notes at the bottom

tasks-toast-done = 할 일을 완료했습니다
tasks-toast-next = 완료했습니다. 다음 일정: { $date }
tasks-toast-deleted = 할 일을 삭제했습니다
tasks-toast-added = { $count ->
   *[other] 할 일 { $count }개를 추가했습니다
}
tasks-mail-gone = 해당 메일이 더 이상 없습니다.
tasks-toast-list-deleted = 목록을 삭제했습니다
tasks-toast-moved = { $list }(으)로 이동했습니다
tasks-toast-rescheduled = 할 일의 일정을 변경했습니다
