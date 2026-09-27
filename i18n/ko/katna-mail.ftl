# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = 언어: { $language }
language-tooltip-system = 언어: { $language }(시스템 설정에 따름)
language-search = 언어 검색
language-system-default = 시스템 기본값
language-system-now = 현재 { $language }
language-no-match = “{ $query }”에 해당하는 언어가 없습니다
language-machine = 기계 번역입니다. 개선에 참여해 주세요
language-setting = 언어
language-setting-detail = 메뉴, 버튼, 메시지의 언어와 날짜 및 숫자 형식입니다. 시스템 기본값을 선택하면 데스크톱 설정을 따릅니다.

## Dates and sizes

ago-just-now = 방금
ago-minutes = { $count }분 전
ago-hours = { $count }시간 전
ago-days = { $count }일 전
size-bytes = { $count }바이트
size-kb = { $size }KB
size-mb = { $size }MB
size-gb = { $size }GB
size-tb = { $size }TB

## Top bar

folders-hide = 폴더 숨기기
folders-show = 폴더 표시
compose = 편지쓰기
search = 검색
search-mail = 메일 검색
search-settings = 설정 검색
search-clear = 검색어 지우기
search-options-show = 검색 옵션 표시
settings = 설정
account-add = 계정 추가

## App rail (and the bottom bar on a phone)

rail-mail = 메일
rail-calendar = 캘린더
rail-contacts = 연락처
rail-tasks = 할 일
rail-notes = 메모
rail-feeds = 피드

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = 곧 제공 예정
app-calendar-promise = CalDAV 캘린더, 메일로 받은 회의 초대, 알림을 받은편지함 바로 옆에서 확인하세요.
app-tasks-promise = CalDAV와 동기화되는 할 일 목록과 메일에서 만든 할 일.
app-notes-promise = 빠른 메모, 그리고 나중을 위해 메일이나 대화에 남기는 메모.
app-feeds-promise = 메일 옆에서 RSS 및 Atom 피드를 읽으세요.

## Contacts page

app-contacts-loading = 메일에서 연락처를 모으는 중…
app-contacts-empty = 메일을 주고받은 사람이 여기에 표시됩니다.
app-contacts-count = 메일을 주고받은 사람 { $count }명, 많이 주고받은 순
app-contacts-top = 메일을 주고받은 상위 { $count }명, 많이 주고받은 순
app-contacts-messages = 메일 { $count }개
app-contacts-last = 최근 { $date }

## Navigation (the folders pane)

nav-labels = 라벨
nav-folders = 폴더
nav-label-new = 새 라벨 만들기
nav-folder-new = 새 폴더 만들기
nav-account-unnamed = 계정 { $number }
nav-tab-new = 새 메일 { $count }개

## Special folders (the user's own folders keep their names)

folder-inbox = 받은편지함
folder-starred = 별표편지함
folder-drafts = 임시보관함
folder-sent = 보낸편지함
folder-archive = 보관함
folder-spam = 스팸함
folder-trash = 휴지통
folder-all-mail = 전체보관함
folder-scheduled = 예약됨

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = 새 라벨
label-folder-new-title = 새 폴더
label-prompt = 새 라벨 이름을 입력하세요.
label-folder-prompt = 새 폴더 이름을 입력하세요.
label-name-hint = 라벨 이름
label-folder-name-hint = 폴더 이름
label-nest = 다음 라벨 아래에 중첩:
label-folder-nest = 다음 폴더 아래에 중첩:
label-cancel = 취소
label-create = 만들기
label-creating = 만드는 중…
label-created = “{ $name }” 라벨을 만들었습니다.
label-folder-created = “{ $name }” 폴더를 만들었습니다.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = 기본
tab-promotions = 프로모션
tab-social = 소셜
tab-updates = 업데이트
tab-forums = 포럼
tab-focused = 중요
tab-other = 기타
tab-inbox = 받은편지함
tab-newsletters = 뉴스레터
tab-notifications = 알림
tab-new = 새 메일 { $count }개
tab-provider-other = Katna에서 분류

## Mail list: toolbar

list-select = 선택
list-refresh = 새로고침
list-more = 더보기
list-mark-read = 읽음으로 표시
list-mark-unread = 읽지 않음으로 표시
list-move-to = 이동
list-archive = 보관처리
list-spam = 스팸신고
list-delete = 삭제
list-newer = 최신
list-older = 이전
list-range = { $first }–{ $last } / { $total }
list-range-about = { $first }–{ $last } / 약 { $total }
list-results = “{ $query }” 검색결과
list-results-corrected = “{ $query }” 검색결과를 표시합니다
list-search-instead = 대신 “{ $query }”(으)로 검색
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = 전체
list-pick-none = 선택 안함
list-pick-read = 읽음
list-pick-unread = 읽지 않음
list-pick-starred = 별표 있음
list-pick-unstarred = 별표 없음

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] 대화 { $count }개가 모두 선택되었습니다.
   *[message] 메일 { $count }개가 모두 선택되었습니다.
}
list-selected-all-in = { $kind ->
    [conversation] { $folder }의 대화 { $count }개가 모두 선택되었습니다.
   *[message] { $folder }의 메일 { $count }개가 모두 선택되었습니다.
}
list-selected-screen = { $kind ->
    [conversation] 이 페이지의 대화 { $count }개가 모두 선택되었습니다.
   *[message] 이 페이지의 메일 { $count }개가 모두 선택되었습니다.
}
list-select-all = { $kind ->
    [conversation] 대화 { $count }개 모두 선택
   *[message] 메일 { $count }개 모두 선택
}
list-select-all-in = { $kind ->
    [conversation] { $folder }의 대화 { $count }개 모두 선택
   *[message] { $folder }의 메일 { $count }개 모두 선택
}
list-clear-selection = 선택 해제

## Mail list: empty states

list-empty-search = 검색과 일치하는 메일이 없습니다.
list-empty-tab = { $tab }에 메일이 없습니다.
list-empty-tab-unknown = 이 탭에 메일이 없습니다.
list-empty-folder = { $folder }에 메일이 없습니다.
list-empty-folder-unknown = 이 폴더에 메일이 없습니다.
list-first-sync = 메일을 가져오는 중…
list-first-sync-detail = 메일이 도착하는 대로 여기에 표시됩니다.

## Mail list: lines

row-removed = 이 메일은 삭제되었습니다.
row-starred = 별표 있음
row-not-starred = 별표 없음
row-important = 중요. 클릭하면 중요하지 않음으로 표시합니다.
row-mark-important = 중요 표시
row-pinned = 상단에 고정됨
row-pin = 상단에 고정
row-unpin = 고정 해제

## Mail list: More menu and right-click menu

menu-reply = 답장
menu-reply-all = 전체답장
menu-forward = 전달
menu-archive = 보관처리
menu-delete = 삭제
menu-spam = 스팸신고
menu-mark-read = 읽음으로 표시
menu-mark-unread = 읽지 않음으로 표시
menu-mark-all-read = 모두 읽음으로 표시
menu-star = 별표 추가
menu-unstar = 별표 삭제
menu-important = 중요 표시
menu-not-important = 중요하지 않음으로 표시
menu-pin = 상단에 고정
menu-unpin = 고정 해제
menu-print-all = 모두 인쇄
menu-new-window = 새 창에서 열기
menu-move-to = 이동
menu-move-to-heading = 이동할 위치:
menu-find-from = { $name }님이 보낸 메일 찾기

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] 대화 { $count }개가 보관처리되었습니다.
   *[message] 메일 { $count }개가 보관처리되었습니다.
}
toast-trashed = { $kind ->
    [conversation] 대화 { $count }개가 휴지통으로 이동되었습니다.
   *[message] 메일 { $count }개가 휴지통으로 이동되었습니다.
}
toast-moved = { $kind ->
    [conversation] 대화 { $count }개가 이동되었습니다.
   *[message] 메일 { $count }개가 이동되었습니다.
}
toast-starred = { $kind ->
    [conversation] 대화 { $count }개에 별표를 추가했습니다.
   *[message] 메일 { $count }개에 별표를 추가했습니다.
}
toast-unstarred = { $kind ->
    [conversation] 대화 { $count }개의 별표를 삭제했습니다.
   *[message] 메일 { $count }개의 별표를 삭제했습니다.
}
toast-important = { $kind ->
    [conversation] 대화 { $count }개를 중요로 표시했습니다.
   *[message] 메일 { $count }개를 중요로 표시했습니다.
}
toast-not-important = { $kind ->
    [conversation] 대화 { $count }개를 중요하지 않음으로 표시했습니다.
   *[message] 메일 { $count }개를 중요하지 않음으로 표시했습니다.
}
toast-pinned = { $kind ->
    [conversation] 대화 { $count }개를 상단에 고정했습니다.
   *[message] 메일 { $count }개를 상단에 고정했습니다.
}
toast-unpinned = { $kind ->
    [conversation] 대화 { $count }개의 고정을 해제했습니다.
   *[message] 메일 { $count }개의 고정을 해제했습니다.
}
toast-spam = { $kind ->
    [conversation] 대화 { $count }개를 스팸으로 신고했습니다.
   *[message] 메일 { $count }개를 스팸으로 신고했습니다.
}
toast-deleted-forever = { $kind ->
    [conversation] 대화 { $count }개를 영구삭제했습니다.
   *[message] 메일 { $count }개를 영구삭제했습니다.
}
toast-undone = 작업을 실행취소했습니다.
toast-undo = 실행취소
toast-no-spam-folder = 이 계정에는 스팸함이 없습니다.

## Reading pane: toolbar

reader-close = 닫기
reader-back = 뒤로
reader-mark-unread = 읽지 않음으로 표시
reader-move-to = 이동
reader-more = 더보기
reader-print-all = 모두 인쇄
reader-new-window = 새 창에서 열기
reader-position = { $position } / { $total }
reader-newer = 최신
reader-older = 이전

## Reading pane: the conversation

reader-removed = 이 대화는 삭제되었습니다.
reader-no-subject = (제목 없음)
reader-collapse-all = 모두 접기
reader-expand-all = 모두 펼치기
reader-unknown-sender = (알 수 없는 보낸사람)
reader-date-ago = { $date }({ $ago })
reader-me = 나
reader-to = 받는사람: { $names }
reader-starred = 별표 있음
reader-not-starred = 별표 없음
reader-too-long = 메일이 너무 길어 전체를 표시할 수 없습니다.
reader-encrypted-images = 암호화된 메일에서는 웹 이미지를 불러오지 않습니다.
reader-window-failed = 새 창을 열 수 없습니다.

## Reading pane: message details (opened from "to me")

reader-details-from = 보낸사람:
reader-details-to = 받는사람:
reader-details-cc = 참조:
reader-details-date = 날짜:
reader-details-subject = 제목:

## Reading pane: downloading a message

reader-downloading = 서버에서 이 메일을 다운로드하는 중…
reader-download-failed = 이 메일을 다운로드할 수 없습니다.
reader-try-again = 다시 시도

## Reply row

reply-reply = 답장
reply-reply-all = 전체답장
reply-forward = 전달

## Encrypted and signed mail

security-decrypting = 복호화하는 중…
security-checking = 서명을 확인하는 중…
security-partly-encrypted = 이 메일은 일부만 암호화되어 있습니다. 나머지는 보호 범위 밖에서 추가된 것으로, 누구든 보냈을 수 있습니다.
security-partly-signed = 이 메일은 일부만 서명되어 있습니다. 나머지는 보호 범위 밖에서 추가된 것으로, 누구든 보냈을 수 있습니다.
security-encrypted = 암호화된 메일
security-encrypted-smime = 암호화된 메일(S/MIME)
security-no-key = 이 메일을 복호화할 수 없습니다. 내게 없는 키로 암호화되었습니다.
security-cancelled = 복호화가 취소되었습니다.
security-damaged = 이 메일을 복호화할 수 없습니다. 암호화된 데이터가 손상되었거나 변경되었습니다.
security-decrypt-unavailable = 이 메일을 복호화할 수 없습니다. 암호화된 메일을 읽으려면 { $tool }을(를) 설치하세요.
security-decrypt-failed = 이 메일을 복호화할 수 없습니다: { $reason }
security-unknown-signer = 알 수 없는 서명자
security-signed-verified = { $signer } 서명 · 확인됨
security-signed-not-sender = { $signer } 서명, 보낸사람이 아님
security-signed-untrusted = { $signer } 서명, 신뢰하지 않음으로 표시한 키 사용
security-signed-unverified = { $signer } 서명 · 키가 확인되지 않음
security-bad-signature = 잘못된 서명: 서명 후 이 메일이 변경되었거나 서명이 위조되었습니다.
security-signature-expired = { $signer } 서명 · 서명이 만료됨
security-key-expired = { $signer } 서명 · 이후 키가 만료됨
security-key-revoked = { $signer } 서명, 폐기된 키 사용
security-missing-key = 내게 없는 키로 서명되어 확인할 수 없음
security-missing-key-id = 내게 없는 키({ $key })로 서명되어 확인할 수 없음
security-signature-unavailable = 서명됨. 서명을 확인하려면 { $tool }을(를) 설치하세요
security-signature-error = 서명을 확인할 수 없습니다.

## Remote images and pictures

remote-hidden = 이 메일의 이미지가 숨겨져 있습니다.
remote-show = 이미지 표시
remote-always-show = 이 보낸사람의 이미지 항상 표시
remote-picture-use = 사용
remote-picture-too-big = 8 MB 이하의 사진을 선택하세요.
remote-picture-type = PNG, JPEG, GIF, WebP 또는 SVG 사진을 선택하세요.
remote-picture-read-failed = 사진을 읽을 수 없습니다: { $error }
remote-picture-keep-failed = 사진을 저장할 수 없습니다: { $error }
remote-picture-remove-failed = 사진을 삭제할 수 없습니다: { $error }

## Attachments

attachment-count = 첨부파일 { $count }개
attachment-save = 저장
attachment-save-all = 모두 저장
attachment-save-all-tooltip = 모든 첨부파일을 폴더에 저장
attachment-save-here = 여기에 저장
attachment-not-downloaded = 이 메일은 다운로드되지 않았습니다.
attachment-not-found = 메일에서 이 첨부파일을 찾을 수 없습니다.
attachment-read-failed = { $name }을(를) 읽을 수 없습니다
attachment-numbered = 첨부파일 { $number }
attachment-saved-all = 파일 { $count }개를 { $place }에 저장했습니다
attachment-saved-some = 파일 { $total }개 중 { $saved }개를 { $place }에 저장했습니다. { $failed }을(를) 저장할 수 없습니다
attachment-saved-to = { $path }에 저장했습니다
attachment-save-failed = { $name }을(를) 저장할 수 없습니다: { $error }
attachment-open-failed = { $name }을(를) 열 수 없습니다: { $error }
attachment-risky = 이 파일은 프로그램을 실행할 수 있어 Katna에서 열지 않습니다. 대신 저장하세요.
attachment-encrypted-open = 이 파일은 암호화된 상태로 받았습니다. 저장한 후 다른 곳에서 여세요.

## Printing

print-failed = 인쇄할 수 없습니다: { $error }
print-no-font = 글꼴을 찾을 수 없습니다
print-opened-as-pdf = PDF로 열었습니다. 열린 PDF에서 인쇄하세요.
print-not-downloaded = (아직 다운로드되지 않았습니다.)
print-encrypted = (암호화되어 있습니다. 본문을 인쇄하려면 Katna Mail에서 여세요.)
print-to = 받는사람: { $addresses }
print-cc = 참조: { $addresses }
