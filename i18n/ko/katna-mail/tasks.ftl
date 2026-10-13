# Katna Mail, Korean (한국어): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = 새 할 일
tasks-all = 모든 할 일
tasks-today = 오늘
tasks-upcoming = 예정
tasks-starred = 별표 표시됨
tasks-completed-view = 완료됨
tasks-new-list = 새 목록 만들기
tasks-labels-heading = 라벨
tasks-on-this-computer = 이 컴퓨터
tasks-my-tasks = 내 할 일
tasks-account-sign-in = 할 일을 표시하려면 다시 로그인하세요
tasks-account-signed-in = { $address }에 다시 로그인했습니다. 할 일을 가져오는 중…
tasks-account-sign-in-refused = { $provider }에서 Katna의 접근을 허용하지 않았습니다. 다시 시도하고 할 일에 대한 접근을 허용하세요.
tasks-account-refused = 서버에서 비밀번호를 받아들이지 않았습니다. Yahoo, iCloud, Zoho 등은 앱 비밀번호가 필요합니다.
tasks-account-change-password = 비밀번호 변경
tasks-account-change-password-tooltip = 새 비밀번호를 입력하세요. Katna가 서버에서 확인합니다
tasks-account-not-enabled = Katna의 할 일 접근이 아직 켜져 있지 않습니다.
tasks-account-failed = 할 일 목록을 읽을 수 없습니다.
tasks-account-error = 할 일 목록을 읽을 수 없습니다: { $reason }
tasks-account-none = 할 일 목록을 찾을 수 없음
tasks-account-none-why = 할 일 목록을 찾을 수 없음: { $reason }
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
tasks-label-empty = 이 라벨이 있는 미완료 할 일이 없습니다.
tasks-today-empty = 오늘 마감인 할 일이 없습니다.
tasks-completed-empty = 완료한 할 일이 여기에 표시됩니다.
tasks-upcoming-add = { $day }에 할 일 추가
tasks-upcoming-overdue-day = { $day }({ $weekday })
tasks-from-mail-quiet = 메일에서
tasks-from-note-quiet = 메모에서
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = 기한 경과
tasks-completed = { $count ->
   *[other] 완료됨 ({ $count })
}
tasks-list-options = 목록 옵션
tasks-sort-by = 정렬 기준
tasks-sort-my-order = 내 순서
tasks-sort-date = 날짜
tasks-sort-starred = 최근 별표 표시
tasks-sort-title = 제목
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

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
   *[other] { $count }개 선택됨
}
tasks-select-clear = 선택 해제
tasks-select-move = 목록으로 이동
tasks-select-date = 날짜 설정
tasks-next-week = 다음 주

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
tasks-label-add = 라벨 추가
tasks-label-task = 할 일에 라벨 지정
tasks-files-attach = 파일 첨부
tasks-files-pick = 첨부
tasks-file-open = 열기
tasks-file-remove = 파일 삭제
tasks-file-here = 이 컴퓨터에만 있음
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
tasks-files-added = { $count ->
    [1] 파일을 첨부했습니다
   *[other] 파일 { $count }개를 첨부했습니다
}
tasks-file-removed = “{ $name }”을(를) 삭제했습니다
tasks-files-left-out = 첨부되지 않음: { $names }. 할 일에는 { $limit } 이하의 파일만 첨부할 수 있으며 폴더는 첨부할 수 없습니다.
tasks-file-missing = 해당 파일이 더 이상 없습니다.
tasks-toast-added = { $count ->
   *[other] 할 일 { $count }개를 추가했습니다
}
tasks-mail-gone = 해당 메일이 더 이상 없습니다.
tasks-toast-list-deleted = 목록을 삭제했습니다
tasks-toast-moved = { $list }(으)로 이동했습니다
tasks-toast-placed = 할 일을 이동했습니다
tasks-toast-rescheduled = 할 일의 일정을 변경했습니다
tasks-toast-rescheduled-several = { $count ->
    [1] 할 일 일정을 변경했습니다
   *[other] 할 일 { $count }개의 일정을 변경했습니다
}
tasks-toast-done-several = { $count ->
    [1] 할 일을 완료했습니다
   *[other] 할 일 { $count }개를 완료했습니다
}
tasks-toast-open-several = { $count ->
    [1] 할 일을 미완료로 표시했습니다
   *[other] 할 일 { $count }개를 미완료로 표시했습니다
}
tasks-toast-starred = { $count ->
    [1] 할 일에 별표를 표시했습니다
   *[other] 할 일 { $count }개에 별표를 표시했습니다
}
tasks-toast-unstarred = { $count ->
    [1] 별표를 삭제했습니다
   *[other] 할 일 { $count }개에서 별표를 삭제했습니다
}
tasks-toast-deleted-several = { $count ->
    [1] 할 일을 삭제했습니다
   *[other] 할 일 { $count }개를 삭제했습니다
}
