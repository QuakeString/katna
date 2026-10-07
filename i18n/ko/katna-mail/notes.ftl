# Katna Mail, Korean (한국어): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = 메모
notes-view-reminders = 알림
notes-view-archive = 보관처리
notes-view-trash = 휴지통
notes-edit-labels = 라벨 수정
notes-search = 메모 검색
notes-loading = 메모를 여는 중…

## Board

notes-take-a-note = 메모 작성…
notes-new-list = 새 목록
notes-new-note = 새 메모
notes-pinned = 고정됨
notes-others = 기타
notes-empty = 추가한 메모가 여기에 표시됩니다
notes-archive-empty = 보관처리된 메모가 여기에 표시됩니다
notes-trash-empty = 휴지통에 메모 없음
notes-none-found = 일치하는 메모 없음
notes-label-empty = 이 라벨이 지정된 메모가 아직 없습니다
notes-reminders-empty = 알림이 예정된 메모가 여기에 표시됩니다
notes-trash-note = 휴지통에 있는 메모는 7일 후에 삭제됩니다.
notes-empty-trash = 휴지통 비우기
notes-ticked = { $count ->
   *[other] + 체크한 항목 { $count }개
}
notes-select = 메모 선택
notes-selected = { $count ->
   *[other] { $count }개 선택됨
}
notes-select-clear = 선택 해제

## A note's buttons

notes-pin = 메모 고정
notes-unpin = 메모 고정 해제
notes-archive = 보관처리
notes-unarchive = 보관처리 해제
notes-delete = 메모 삭제
notes-restore = 복원
notes-delete-forever = 완전히 삭제
notes-color = 배경 옵션
notes-checkboxes = 체크박스 표시/숨기기
notes-labels = 라벨
notes-close = 닫기
notes-more = 더보기
notes-make-copy = 사본 만들기
notes-remind = 알림 받기
notes-add-picture = 사진 추가
notes-history = 버전 기록
notes-ai = 글쓰기 도움받기
notes-send-as-mail = 메일로 보내기
notes-save-markdown = Markdown으로 저장
notes-save-pdf = PDF로 저장

## The open note

notes-title = 제목
notes-edited = 수정됨: { $date }
notes-on-this-computer = 이 컴퓨터
notes-where = 메모 저장 위치
notes-untitled = 제목 없는 메모

## Pictures

notes-picture-choose = 사진 추가
notes-picture-remove = 사진 삭제
notes-picture-too-big = 메모에는 { $size } 이하의 사진만 넣을 수 있습니다
notes-picture-kind = Katna에서 표시할 수 있는 사진 파일이 아닙니다
notes-picture-unreadable = { $name }을(를) 읽을 수 없습니다: { $error }

## Reminders

notes-remind-me = 알림 받기
notes-remind-off = 알림 삭제
notes-remind-in-the-past = 아직 지나지 않은 시간을 선택하세요
notes-remind-today = 오늘 { $time }
notes-remind-tomorrow = 내일 { $time }
notes-remind-weekday = { $day } { $time }
notes-reminder-set = { $when }에 알림이 설정되었습니다
notes-reminder-off = 알림을 삭제했습니다

## Links between notes

notes-link-note = 메모 연결
notes-link-new = 새 메모 “{ $title }”
notes-linked-from = 이 메모를 연결한 메모
notes-link-gone = 해당 메모가 더 이상 없습니다

## Version history

notes-versions = 버전
notes-version-now = 현재
notes-version-here = 나, 이 컴퓨터에서
notes-version-yesterday = 어제 { $time }
notes-version-changes = { $count ->
   *[other] 변경 { $count }개
}
notes-version-from = { $device }에서
notes-version-elsewhere = 다른 기기에서
notes-version-created = 만듦
notes-version-restore = 이 버전 복원
notes-version-restored = 버전을 복원했습니다
notes-history-none = 아직 이전 버전이 없습니다

## AI help

notes-ai-tidy = 텍스트 다듬기
notes-ai-checklist = 체크리스트로 바꾸기
notes-ai-summarise = 요약
notes-ai-empty = 먼저 내용을 작성하세요
notes-ai-tidied = 텍스트를 다듬었습니다. Ctrl+Z를 누르면 되돌립니다.
notes-ai-listed = 체크리스트로 바꿨습니다. Ctrl+Z를 누르면 되돌립니다.
notes-ai-summarised = 맨 위에 요약을 추가했습니다

## Labels

notes-label-note = 메모에 라벨 지정
notes-label-name = 라벨 이름 입력
notes-label-create = "{ $name }" 만들기
notes-label-remove = 라벨 제거
notes-label-delete = 라벨 삭제
notes-labels-none = 라벨이 없습니다. 메모의 라벨 버튼으로 추가하세요.
notes-labels-done = 완료
notes-label-renamed = 라벨 이름이 "{ $name }"(으)로 변경되었습니다
notes-label-deleted = "{ $name }" 라벨을 삭제했습니다

## A note about a mail

notes-mail = 메일
notes-open-mail = 메일 열기
notes-open-note = 메모 열기

## Meeting notes

notes-meeting-take = 회의 메모 작성
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = 참석자: { $names }
notes-meeting-notes = 메모
notes-meeting-actions = 실행 항목
notes-event = 일정
notes-open-event = 일정 열기

## Formatting

notes-format = 서식
notes-format-heading-1 = 제목 1
notes-format-heading-2 = 제목 2
notes-format-normal = 일반 텍스트
notes-format-bold = 굵게
notes-format-italic = 기울임꼴
notes-format-underline = 밑줄
notes-format-quote = 인용
notes-format-code = 코드
notes-format-divider = 구분선
notes-format-clear = 서식 지우기

## Tasks

notes-make-task = 할 일로 만들기

## Colors (tooltips)

notes-color-none = 색상 없음
notes-color-coral = 코랄
notes-color-peach = 피치
notes-color-sand = 샌드
notes-color-mint = 민트
notes-color-sage = 세이지
notes-color-fog = 포그
notes-color-storm = 스톰
notes-color-dusk = 더스크
notes-color-blossom = 블로섬
notes-color-clay = 클레이
notes-color-chalk = 초크

## Messages at the foot of the window

notes-archived = 메모가 보관처리되었습니다
notes-unarchived = 메모 보관처리가 해제되었습니다
notes-trashed = 메모가 휴지통으로 이동되었습니다
notes-restored = 메모가 복원되었습니다
notes-saved = 메모를 저장했습니다
notes-pinned-count = { $count ->
    [1] 메모를 고정했습니다
   *[other] 메모 { $count }개를 고정했습니다
}
notes-unpinned-count = { $count ->
    [1] 메모 고정을 해제했습니다
   *[other] 메모 { $count }개의 고정을 해제했습니다
}
notes-colored-count = { $count ->
    [1] 색상을 변경했습니다
   *[other] 메모 { $count }개의 색상을 변경했습니다
}
notes-archived-count = { $count ->
    [1] 메모를 보관처리했습니다
   *[other] 메모 { $count }개를 보관처리했습니다
}
notes-unarchived-count = { $count ->
    [1] 메모 보관처리를 해제했습니다
   *[other] 메모 { $count }개의 보관처리를 해제했습니다
}
notes-trashed-count = { $count ->
    [1] 메모를 휴지통으로 이동했습니다
   *[other] 메모 { $count }개를 휴지통으로 이동했습니다
}
notes-restored-count = { $count ->
    [1] 메모를 복원했습니다
   *[other] 메모 { $count }개를 복원했습니다
}
notes-copied-count = { $count ->
    [1] 사본을 만들었습니다
   *[other] 사본 { $count }개를 만들었습니다
}
notes-empty-discarded = 빈 메모를 삭제했습니다
notes-mail-gone = 해당 메일이 더 이상 없습니다
notes-deleted-forever = { $count ->
   *[other] 메모 { $count }개를 완전히 삭제했습니다
}
