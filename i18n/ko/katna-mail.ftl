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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = 첨부파일을 보려면 이 메일을 여세요.
text-copy = 복사
text-select-all = 모두 선택

## Settings page: its tabs

settings-tab-general = 기본설정
settings-tab-inbox = 받은편지함
settings-tab-accounts = 계정
settings-tab-subscriptions = 구독
settings-tab-appearance = 모양
settings-tab-shortcuts = 단축키
settings-tab-default-apps = 기본 앱
settings-tab-folders-rules = 폴더 및 규칙
settings-tab-compose = 편지쓰기
settings-tab-mcp-server = MCP 서버
settings-tab-feedback = 사용자 의견
settings-tab-experimental = 실험실

## Settings page: tabs still to come

settings-tab-subscriptions-coming = 받고 있는 뉴스레터와 메일링 리스트를 확인하고 클릭 한 번으로 구독을 취소하세요.
settings-tab-folders-rules-coming = 폴더와 라벨을 만들고, 이름을 바꾸고, 옮기고, 숨기고, 동기화할 항목을 선택합니다. 규칙은 새 메일을 보낸사람, 제목 또는 단어에 따라 자동으로 분류하거나 라벨을 붙이거나 전달하거나 삭제합니다.
settings-tab-mcp-server-coming = 이 컴퓨터의 AI 어시스턴트가 사용자의 동의하에 메일을 검색하고 읽고 초안을 작성할 수 있게 합니다.

## Settings > General

settings-general-conversations = 대화형식으로 보기
settings-general-conversations-group = 같은 메일에 대한 답장을 묶기
settings-general-conversations-group-detail = 목록에서 대화마다 한 줄로 표시
settings-general-reading = 읽기
settings-general-newest-first = 최신 메일을 먼저 표시
settings-general-newest-first-detail = 대화가 가장 최근 답장부터 시작됩니다
settings-general-full-headers = 전체 헤더 표시
settings-general-full-headers-detail = 모든 메일에서 보낸사람, 받는사람, 참조, 날짜, 제목을 펼쳐서 표시
settings-general-full-names = 받는사람 전체 이름
settings-general-full-names-detail = “받는사람: 나, Ada” 대신 “받는사람: 나, Ada Lovelace”
settings-general-mark-read = 읽음으로 표시
settings-general-mark-read-now = 열자마자
settings-general-mark-read-1s = 1초 동안 열어 둔 후
settings-general-mark-read-3s = 3초 동안 열어 둔 후
settings-general-mark-read-never = 직접 읽음으로 표시할 때만
settings-general-reply-button = 답장 버튼
settings-general-reply-all = 전체답장
settings-general-reply-all-detail = 각 메일 옆의 답장 버튼이 보낸사람만이 아니라 모두에게 답장합니다
settings-general-remote-images = 웹 이미지
settings-general-remote-images-detail = 메일의 이미지를 불러오면 사용자가 메일을 열었다는 사실과 시간, 대략적인 위치가 보낸사람에게 전달됩니다. 사용 중지하면 메일마다 먼저 확인하며, 보낸사람의 이미지는 언제든지 표시할 수 있습니다.
settings-general-remote-images-always = 이미지 항상 표시
settings-general-remote-images-always-detail = 신뢰하는 보낸사람만이 아니라 모든 메일에서
settings-general-sending = 보내기
settings-general-sending-detail = 보낸 메일을 취소할 수 있도록 대기하는 시간입니다.
settings-general-offline = 오프라인 메일
settings-general-offline-detail = 최근 메일은 전체를 다운로드하여 인터넷 연결 없이 읽을 수 있습니다. 오래된 메일은 열 때 다운로드됩니다.
settings-general-offline-days = { $count }일
settings-general-offline-years = { $count }년
settings-general-offline-all = 전체 메일
settings-general-offline-note = 기간을 줄여도 이미 다운로드한 메일은 유지됩니다. 서버에서는 아무것도 바뀌지 않습니다.
settings-general-notifications = 알림
settings-general-notifications-detail = 받은편지함의 새 메일에 대해 알립니다. Katna Mail이 닫혀 있어도 알립니다.
settings-general-new-mail = 새 메일 알림 받기
settings-general-new-mail-detail = 전체답장, 읽음으로 표시, 보관처리 버튼 포함
settings-general-new-mail-sound = 소리 재생
settings-general-new-mail-sound-detail = 데스크톱의 새 메일 알림음
settings-general-desktop = 데스크톱
settings-general-open-at-login = 로그인할 때 Katna Mail 열기
settings-general-open-at-login-detail = 어느 경우든 서비스가 실행 중이면 로그인할 때 메일이 동기화됩니다
settings-general-tray = 시스템 트레이에 Katna 표시
settings-general-tray-detail = 읽지 않은 메일 수와 메뉴 포함
settings-general-unread-badge = 작업 표시줄 아이콘에 읽지 않은 메일 수 표시
settings-general-unread-badge-detail = 받은편지함에서 읽지 않은 메일의 수

## Settings > Inbox

settings-inbox-tabs = 받은편지함 탭
settings-inbox-tabs-detail = 메일 서비스 웹사이트처럼 받은편지함을 탭으로 분류합니다.
settings-inbox-tabs-show = 받은편지함 탭 표시
settings-inbox-tabs-show-detail = 사용 중지하면 계정마다 목록 하나로 표시
settings-inbox-no-accounts = 탭을 선택하려면 계정을 추가하세요.
settings-inbox-tabs-automatic = 자동: { $tabs }({ $provider })
settings-inbox-tabs-off = 탭 없음
settings-inbox-tabs-gmail = 기본, 프로모션, 소셜, 업데이트, 포럼
settings-inbox-tabs-focused = 중요 및 기타
settings-inbox-tabs-zoho = 받은편지함, 뉴스레터 및 알림
settings-inbox-tabs-shown = 표시할 탭입니다. 사용 중지한 탭의 메일은 { $tab }에 남습니다.

## Settings > Appearance

settings-appearance-reading-pane = 읽기 창
settings-appearance-reading-pane-detail = 연 대화가 표시되는 위치입니다.
settings-appearance-pane-right = 목록 오른쪽
settings-appearance-pane-none = 분할 안함
settings-appearance-density = 표시 밀도
settings-appearance-density-default = 기본값
settings-appearance-density-compact = 간단히
settings-appearance-scaling = 배율
settings-appearance-scaling-detail = 데스크톱 자체 배율에 더해 Katna Mail의 텍스트, 아이콘, 간격, 구분선을 모두 크게 또는 작게 합니다. 보내는 메일의 글꼴 크기는 바뀌지 않습니다. 너무 작게 하면 아이콘을 클릭하기 어려울 수 있습니다.
settings-appearance-theme = 테마
settings-appearance-theme-system = 데스크톱과 같게
settings-appearance-theme-light = 밝게
settings-appearance-theme-dark = 어둡게
settings-appearance-desktop-colors = 데스크톱 색상
settings-appearance-desktop-colors-use = 데스크톱 색상 사용
settings-appearance-desktop-colors-use-detail = 데스크톱의 색 구성표와 강조 색상
settings-appearance-app-names = 앱 이름
settings-appearance-app-names-show = 앱 이름 표시
settings-appearance-app-names-show-detail = 맨 왼쪽 앱 아이콘 아래에 이름 표시
settings-appearance-sender-pictures = 보낸사람 사진
settings-appearance-sender-pictures-show = 회사 로고 표시
settings-appearance-sender-pictures-show-detail = 메일이 아닌 보낸사람의 도메인으로 찾으며, 1주일 동안 보관합니다
settings-appearance-important = 중요 표시
settings-appearance-important-show = 중요 표시 보기
settings-appearance-important-show-detail = 목록의 각 메일 옆에 표시
settings-appearance-message-width = 메일 너비
settings-appearance-message-width-limit = 메일 너비 제한
settings-appearance-message-width-limit-detail = 넓은 창에서 긴 줄을 읽기 쉬워집니다
settings-appearance-mail-colors = 메일 색상
settings-appearance-mail-colors-detail = 대부분의 메일은 흰색 배경에 맞게 디자인되어 있습니다. 어두운 테마에서는 읽기 쉬운 어두운 색상으로 바꿔 표시하며, 사용 중지하면 밝은 배경에 보낸사람의 색상 그대로 표시합니다.
settings-appearance-dark-mail = 메일에도 어두운 색상 사용
settings-appearance-dark-mail-detail = 테마가 어두울 때만
settings-appearance-attachment-previews = 첨부파일 미리보기
settings-appearance-attachment-previews-show = 첨부파일 미리보기 표시
settings-appearance-attachment-previews-show-detail = 각 파일 카드에 내용을 작은 이미지로 표시

## Settings > Default apps

settings-default-apps-intro = 첨부파일을 클릭했을 때 여는 앱입니다. 뷰어에서 언제든지 다른 앱으로 파일을 열 수도 있습니다. 데스크톱의 기본 앱은 데스크톱 설정에서 지정합니다.
settings-default-apps-pdf = PDF 파일
settings-default-apps-pdf-detail = 페이지 보기, 확대/축소 가능.
settings-default-apps-pictures = 사진
settings-default-apps-pictures-detail = 사진(바로 세워서 표시), PNG, GIF, WebP, BMP, TIFF, SVG.
settings-default-apps-text = 텍스트 파일
settings-default-apps-text-detail = 일반 텍스트, 로그, 코드 및 기타 텍스트.
settings-default-apps-sheets = 스프레드시트
settings-default-apps-sheets-detail = Excel(xlsx, xls), OpenDocument(ods), CSV.
settings-default-apps-documents = 문서
settings-default-apps-documents-detail = Word(docx) 및 OpenDocument 텍스트(odt).
settings-default-apps-katna = Katna Mail 뷰어
settings-default-apps-system = 데스크톱의 기본 앱
settings-default-apps-ask = 매번 앱 선택
settings-default-apps-after-saving = 저장한 후
settings-default-apps-show-folder = 저장한 파일을 폴더에서 표시
settings-default-apps-show-folder-detail = 저장한 첨부파일이 선택된 상태로 파일 관리자를 엽니다

## Settings > Compose

settings-compose-send-from = 새 메일 보내는 계정
settings-compose-send-from-detail = 답장과 전달은 항상 현재 사용 중인 계정에서 보냅니다.
settings-compose-send-from-current = 현재 사용 중인 계정
settings-compose-send-on-replies = 답장 시 보내기
settings-compose-send-on-replies-detail = 답장이나 전달에서 보내기 버튼이 하는 동작입니다. 보내기 옆 메뉴에서 다른 동작을 선택할 수 있습니다.
settings-compose-send-plain = 보내기
settings-compose-send-archive = 보내고 보관처리
settings-compose-signatures = 서명
settings-compose-signatures-detail = 메일 본문 아래 “--” 줄 다음에 추가됩니다. 편지쓰기 창에서 다른 서명을 선택할 수 있습니다.
settings-compose-untitled = 제목 없음
settings-compose-signature-name = 이름(예: 회사)
settings-compose-signature-first = 내 서명
settings-compose-signature-numbered = 서명 { $number }
settings-compose-signature-delete = 삭제
settings-compose-signature-deleted = 서명을 삭제했습니다
settings-compose-signature-new = 새로 만들기
settings-compose-no-signatures = 아직 서명이 없습니다.
settings-compose-no-signature = 서명 없음
settings-compose-for-new-mail = 새 메일용
settings-compose-for-replies = 답장/전달용
settings-compose-for-replies-detail = 내가 서명한 메일이 있는 대화에서는 답장이 그 서명으로 시작됩니다.
settings-compose-format = 형식
settings-compose-plain-text = 일반 텍스트로 작성
settings-compose-plain-text-detail = 새 메일이 서식 없이 시작되며, 편지쓰기 창에서 전환할 수 있습니다
settings-compose-spelling = 맞춤법
settings-compose-spell-check = 작성하는 동안 맞춤법 검사
settings-compose-spell-check-detail = 맞춤법이 틀린 단어에 밑줄이 표시되며, 마우스 오른쪽 버튼을 클릭하면 추천 단어가 나옵니다
settings-compose-spell-desktop = 데스크톱 언어({ $language })
settings-compose-templates = 템플릿
settings-compose-templates-detail = 자주 쓰는 메일을 저장하고, 새 메일이나 답장을 템플릿으로 시작하세요.

## Settings > Shortcuts

settings-shortcuts-set = 단축키 세트
settings-shortcuts-set-detail = 익숙한 메일 앱의 키로 시작하세요. 여기서는 Cmd가 Ctrl입니다. 직접 변경한 키는 세트보다 우선하며, 기본값 복원을 누르면 세트의 키로 돌아갑니다.
settings-shortcuts-single = 단일 키 단축키
settings-shortcuts-single-detail = 웹메일처럼 Ctrl이나 Alt 없이 누르는 키입니다. e는 보관처리, j와 k는 이동, /는 검색. 목록과 열린 대화에서 작동하며, 입력 중에는 작동하지 않습니다.
settings-shortcuts-single-use = 단일 키 단축키 사용
settings-shortcuts-single-use-detail = Ctrl 단축키는 항상 작동합니다
settings-shortcuts-how = 키를 클릭하여 변경하거나 +를 클릭하여 추가한 다음 새 키를 누르세요. Esc를 누르면 취소됩니다.
settings-shortcuts-restore = 기본값 복원
settings-shortcuts-no-key = 키 없음
settings-shortcuts-press = 키를 누르세요…
settings-shortcuts-then = { $keys } 다음에…
settings-shortcuts-moved = 이제 { $keys } 키는 “{ $previous }” 대신 “{ $action }”을(를) 실행합니다.
settings-shortcuts-single-off = 단일 키 단축키가 사용 중지되어 있으므로, 이 키는 사용 설정하면 작동합니다.
settings-shortcuts-restored = 모든 단축키를 세트의 키로 되돌렸습니다.

## Settings search: the line under a result

settings-general-language-summary = 앱, 날짜, 숫자의 언어
settings-general-reading-summary = 최신 메일 먼저 표시, 전체 헤더, 받는사람 전체 이름
settings-general-mark-read-summary = 연 대화를 읽음으로 표시하는 시점: 즉시, 1초 또는 3초 후, 직접
settings-general-reply-button-summary = 각 메일 옆의 답장 버튼이 모두에게 답장
settings-general-remote-images-summary = 모든 메일의 이미지 항상 표시
settings-general-sending-summary = 보내기 취소: 보낸 메일을 취소할 수 있도록 대기하는 시간
settings-general-offline-summary = 인터넷 연결 없이 읽도록 최근 며칠간의 메일을 전체 다운로드할지
settings-general-notifications-summary = 새 메일 알림과 알림음
settings-general-desktop-summary = 로그인할 때 Katna Mail 열기, 시스템 트레이 아이콘, 작업 표시줄 아이콘의 읽지 않은 메일 수
settings-accounts-accounts-summary = 계정 추가 또는 삭제, 계정 사진 변경
settings-appearance-density-summary = 목록 줄을 기본값 또는 간단히 표시
settings-appearance-scaling-summary = 텍스트, 아이콘, 간격, 구분선을 모두 크게 또는 작게
settings-appearance-theme-summary = 데스크톱과 같게, 밝게 또는 어둡게
settings-appearance-sender-pictures-summary = 보낸사람의 도메인으로 찾은 회사 로고
settings-appearance-important-summary = 목록의 각 메일 옆에 있는 중요 표시
settings-appearance-mail-colors-summary = 어두운 테마에서 HTML 메일을 어두운 색상으로 표시하거나 보낸사람의 색상 유지
settings-appearance-attachment-previews-summary = 각 첨부파일 내용의 작은 이미지
settings-shortcuts-set-summary = Gmail, Inbox by Gmail, Apple Mail, Outlook 또는 Thunderbird의 키로 시작
settings-shortcuts-single-summary = 웹메일처럼 Ctrl이나 Alt 없이 누르는 키
settings-default-apps-pdf-summary = PDF 첨부파일을 여는 앱
settings-default-apps-pictures-summary = 사진과 이미지를 여는 앱
settings-default-apps-text-summary = 일반 텍스트, 로그, 코드를 여는 앱
settings-default-apps-sheets-summary = Excel, OpenDocument, CSV 파일을 여는 앱
settings-default-apps-documents-summary = Word와 OpenDocument 텍스트를 여는 앱
settings-default-apps-after-saving-summary = 저장한 첨부파일을 폴더에서 표시
settings-compose-send-from-summary = 새 메일을 보내는 계정: 현재 사용 중인 계정 또는 항상 같은 계정
settings-compose-send-on-replies-summary = 답장과 전달에서 보내기 또는 보내고 대화 보관처리
settings-compose-signatures-summary = 메일 본문 아래 “--” 줄 다음에 추가
settings-compose-for-new-mail-summary = 새 메일에 처음부터 들어가는 서명
settings-compose-for-replies-summary = 답장과 전달에 처음부터 들어가는 서명
settings-compose-format-summary = 새 메일을 일반 텍스트로 작성
settings-compose-spelling-summary = 작성하는 동안 맞춤법 검사, 사전 언어
settings-compose-templates-summary = 곧 제공: 자주 쓰는 메일을 저장하고, 새 메일이나 답장을 템플릿으로 시작
settings-feedback-crash-reports-summary = Katna Mail이나 백그라운드 서비스가 비정상 종료되면 이 컴퓨터에 오류 보고서 저장
settings-feedback-saved-summary = 이 컴퓨터에 저장된 오류 보고서 보기, 복사, 삭제
settings-feedback-help-improve-summary = 문제 해결에 도움이 되도록 오류 보고서 보내기(직접 사용 설정하기 전에는 사용 중지)
settings-experimental-blur-summary = 상단 바에 데스크톱이 흐리게 비치고 메뉴는 반투명 유리처럼 표시
settings-search-shortcut = 단축키
settings-search-tab = 설정 탭
settings-search-none = “{ $query }”에 해당하는 설정이 없습니다.
settings-search-results = “{ $query }”에 해당하는 설정

## Quick settings (the panel that slides in from the right)

quick-title = 빠른 설정
quick-see-all = 모든 설정 보기
quick-reading-pane = 읽기 창
quick-pane-right = 목록 오른쪽
quick-pane-none = 분할 안함
quick-density = 표시 밀도
quick-density-default = 기본값
quick-density-compact = 간단히
quick-theme = 테마
quick-theme-system = 데스크톱과 같게
quick-theme-light = 밝게
quick-theme-dark = 어둡게
quick-desktop-colors = 데스크톱 색상
quick-desktop-colors-detail = 데스크톱의 색 구성표와 강조 색상
quick-app-names = 앱 이름
quick-app-names-detail = 맨 왼쪽 앱 아이콘 아래에 이름 표시
quick-inbox-tabs = 받은편지함 탭
quick-inbox-tabs-detail = 각 계정의 메일 서비스 탭
quick-choose-tabs = 탭 선택
quick-choose-tabs-detail = 설정에서 계정별로 선택
quick-sending = 보내기
quick-undo-send = 보내기 취소
quick-undo-send-off = 사용 안함
quick-undo-send-seconds = { $seconds }초
quick-signatures = 서명
quick-signatures-none = 아직 없음
quick-signatures-one = { $name }, 기본으로 사용
quick-signatures-many = 서명 { $count }개, 기본은 { $name }
quick-signatures-no-default = { $count }개, 기본 서명 없음
quick-signature-untitled = 제목 없음
quick-threading = 이메일 스레드
quick-conversation-view = 대화형식으로 보기
quick-conversation-view-detail = 같은 메일에 대한 답장을 묶기
quick-help = 도움말
quick-tour = 둘러보기
quick-whats-new = 새로운 기능
quick-about = Katna 정보

## Settings: opening at login

settings-open-at-login-failed = 로그인할 때 열기 설정을 변경할 수 없습니다: { $error }

## Settings > Appearance > Scaling

scale-letter = 가
scale-percent = { $percent }%
scale-reset = { $percent }%로 되돌리기

## Settings > Experimental > Look & Feel

look-intro = 아직 시험 중인 기능입니다. 변경되거나 없어질 수 있습니다.
look-heading = 모양과 느낌
look-window-frame = 창 테두리
look-window-frame-detail = 제목 표시줄, 창 버튼, 모서리, 그림자를 누가 그릴지 정합니다.
look-frame-native-kde = 기본: Plasma 테마를 따르는 KDE 테두리
look-frame-native = 기본: 데스크톱 테두리
look-frame-katna = Katna: 상단 바가 제목 표시줄이 됨
look-frame-katna-note-named = Katna가 둥근 모서리와 자체 그림자를 그립니다. 테두리가 더 이상 { $desktop } 테마를 따르지 않지만, 창 규칙은 계속 적용됩니다.
look-frame-katna-note = Katna가 둥근 모서리와 자체 그림자를 그립니다. 테두리가 더 이상 데스크톱 테마를 따르지 않지만, 창 규칙은 계속 적용됩니다.
look-frame-client-side = 이 데스크톱은 테두리를 각 앱에 맡기므로 Katna가 이미 자체 테두리를 그립니다.
look-blurred-background = 흐린 배경
look-blurred-background-detail = 상단 바와 폴더에 데스크톱이 흐리게 비치고, 메뉴와 팝오버는 반투명 유리처럼 표시됩니다.
look-blur = 창 뒤를 흐리게
look-blur-detail = 메일은 불투명한 카드에 표시되므로 텍스트 대비가 유지됩니다
look-blur-off-kde = KDE의 흐리게 효과가 꺼져 있습니다. 시스템 설정 > 창 관리 > 데스크톱 효과에서 흐리게를 켠 다음 Katna Mail을 다시 여세요.
look-blur-none-gnome = GNOME은 창 뒤를 흐리게 하지 않습니다.
look-blur-none-x11 = 창 관리자가 창 뒤를 흐리게 하지 않습니다.
look-blur-none-wayland = 컴포지터가 창 뒤를 흐리게 하지 않습니다.

## Settings > User feedback (crash reports)

feedback-intro-sending = 문제 해결에 도움이 되도록 새 오류 보고서를 보냅니다. 그 밖의 정보는 이 컴퓨터 밖으로 나가지 않습니다.
feedback-intro-local = Katna는 어디에도 아무것도 보내지 않습니다. 오류 보고서는 이 컴퓨터에 남으며, 직접 살펴보거나 버그 신고에 첨부할 수 있습니다.
feedback-crash-reports = 오류 보고서
feedback-crash-reports-detail = Katna Mail이나 백그라운드 서비스가 비정상 종료되면 작성됩니다.
feedback-save = 이 컴퓨터에 오류 보고서 저장
feedback-save-detail = 홈 폴더, 사용자 및 컴퓨터 이름, 이메일 주소는 제외됩니다
feedback-saved = 저장된 오류 보고서
feedback-saved-detail = 최신 보고서 { $count }개를 보관합니다.
feedback-help-improve = Katna 개선에 참여
feedback-help-improve-detail = 직접 사용 설정하기 전에는 사용 중지되어 있으며, 여기서 언제든지 사용 중지할 수 있습니다.
feedback-send = 오류 보고서 보내기
feedback-send-detail = 저장된 보고서가 여기서 볼 수 있는 그대로 Katna의 오류 추적기(Sentry, EU 소재)로 전송됩니다. IP 주소, 메일, 이메일 주소는 보내지 않습니다
feedback-none-saved = 저장된 오류 보고서가 없습니다.
feedback-delete-all = 모두 삭제
feedback-app-daemon = 백그라운드 서비스
feedback-report-sent = { $date } · 보냄
feedback-view = 보기
feedback-view-tooltip = 보고서 열기
feedback-copy-tooltip = 복사하여 버그 신고에 붙여넣기
feedback-copied = 오류 보고서를 복사했습니다.
feedback-deleted-all = 오류 보고서를 삭제했습니다.
feedback-read-failed = 오류 보고서를 읽을 수 없습니다: { $error }
feedback-delete-failed = 오류 보고서를 삭제할 수 없습니다: { $error }
feedback-delete-all-failed = 오류 보고서를 삭제할 수 없습니다: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = 파일(_F)
desktop-menu-new-message = 새 메일(_N)
desktop-menu-quit = 끝내기(_Q)
desktop-menu-edit = 편집(_E)
desktop-menu-undo = 실행 취소(_U)
desktop-menu-select-all = 모두 선택(_A)
desktop-menu-select-none = 선택 해제(_N)
desktop-menu-find = 찾기(_F)…
desktop-menu-view = 보기(_V)
desktop-menu-folder-list = 폴더 목록 표시(_F)
desktop-menu-refresh = 새로고침(_R)
desktop-menu-go = 이동(_G)
desktop-menu-inbox = 받은편지함(_I)
desktop-menu-starred = 별표편지함(_S)
desktop-menu-sent = 보낸편지함(_E)
desktop-menu-drafts = 임시보관함(_D)
desktop-menu-all-mail = 전체보관함(_A)
desktop-menu-next = 다음 대화(_N)
desktop-menu-previous = 이전 대화(_P)
desktop-menu-message = 메일(_M)
desktop-menu-open = 열기(_O)
desktop-menu-reply = 답장(_R)
desktop-menu-reply-all = 전체답장(_A)
desktop-menu-forward = 전달(_F)
desktop-menu-archive = 보관처리(_H)
desktop-menu-delete = 삭제(_D)
desktop-menu-spam = 스팸신고(_S)
desktop-menu-move-to = 이동(_M)…
desktop-menu-mark-read = 읽음으로 표시(_E)
desktop-menu-mark-unread = 읽지 않음으로 표시(_U)
desktop-menu-star = 별표 추가(_T)
desktop-menu-important = 중요 표시(_P)
desktop-menu-not-important = 중요하지 않음으로 표시(_N)
desktop-menu-settings = 설정(_S)
desktop-menu-quick-settings = 빠른 설정(_Q)
desktop-menu-configure = Katna Mail 설정(_C)…
desktop-menu-help = 도움말(_H)
desktop-menu-shortcuts = 단축키(_K)
desktop-menu-whats-new = 새로운 기능(_W)
desktop-menu-about = Katna 정보(_A)

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = 이동
shortcut-group-actions = 작업
shortcut-group-go-to = 바로가기
shortcut-group-app = 애플리케이션

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = 다음 대화
shortcut-previous = 이전 대화
shortcut-down = 목록에서 아래로 이동
shortcut-up = 목록에서 위로 이동
shortcut-first = 목록의 처음으로
shortcut-last = 목록의 마지막으로
shortcut-page-down = 목록 한 페이지 아래로
shortcut-page-up = 목록 한 페이지 위로
shortcut-open = 대화 열기
shortcut-back = 목록으로 돌아가기
shortcut-scroll-down = 아래로 스크롤
shortcut-scroll-up = 위로 스크롤
shortcut-scroll-page-down = 한 페이지 아래로 스크롤
shortcut-scroll-page-up = 한 페이지 위로 스크롤
shortcut-compose = 편지쓰기
shortcut-reply = 답장
shortcut-reply-all = 전체답장
shortcut-forward = 전달
shortcut-archive = 보관처리
shortcut-delete = 삭제
shortcut-spam = 스팸신고
shortcut-move-to = 이동
shortcut-mark-read = 읽음으로 표시
shortcut-mark-unread = 읽지 않음으로 표시
shortcut-star = 별표 추가/삭제
shortcut-important = 중요 표시
shortcut-not-important = 중요하지 않음으로 표시
shortcut-check = 대화 선택
shortcut-select-all = 모든 대화 선택
shortcut-select-none = 모든 대화 선택 해제
shortcut-undo = 마지막 작업 실행취소
shortcut-go-inbox = 받은편지함
shortcut-go-starred = 별표편지함
shortcut-go-sent = 보낸편지함
shortcut-go-drafts = 임시보관함
shortcut-go-all = 전체보관함
shortcut-search = 메일 검색
shortcut-navigation = 메뉴 표시/접기
shortcut-quick-settings = 빠른 설정
shortcut-settings = 모든 설정
shortcut-shortcuts = 단축키
shortcut-reload = 새 메일 확인
shortcut-quit = 끝내기

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } 다음 { $second }

## Settings > Accounts

accounts-folder-pane = 폴더 창
accounts-folder-pane-detail = 왼쪽 창에 어떤 계정의 폴더를 표시할지 정합니다.
accounts-shown-one = 한 번에 계정 하나, 계정 카드에서 전환
accounts-shown-all = 모든 계정을 차례로
accounts-row = 계정
accounts-row-detail = 계정을 삭제하면 이 컴퓨터에 있는 Katna의 메일 사본이 삭제됩니다. 메일은 서버에 남습니다.
accounts-none = 아직 계정이 없습니다.
accounts-kind-imported = 가져옴
accounts-picture-reset = 데스크톱 사진 사용
accounts-picture-change = 사진 변경
accounts-remove = 삭제
accounts-delete-all-row = 모든 데이터 삭제
accounts-delete-all-row-detail = 새로 설치한 것처럼 처음부터 다시 시작합니다.
accounts-delete-all-about = 모든 계정, 저장된 모든 메일, 연락처, 캘린더, 검색 색인, 설정, 저장된 비밀번호를 이 컴퓨터에서 삭제합니다. 메일 서버에서는 아무것도 바뀌지 않습니다.
accounts-delete-all-open = Katna 데이터 모두 삭제

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } 계정을 Katna에서 삭제했습니다.
accounts-removed = { $address } 계정을 Katna에서 삭제했습니다. 메일은 서버에 그대로 있습니다.
accounts-all-deleted = 이 컴퓨터에서 Katna 데이터를 모두 삭제했습니다.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } 계정을 삭제하시겠습니까?
accounts-remove-confirm = 계정 삭제
accounts-removing = 삭제하는 중…
accounts-remove-local-mail = { $folders ->
    [0] 이 계정으로 가져온 모든 메일
   *[other] 이 계정의 폴더 { $folders }개로 가져온 모든 메일
}
accounts-remove-local-settings = 이 계정의 Katna 설정
accounts-remove-mail = { $folders ->
    [0] Katna에 저장된 이 계정의 모든 메일
   *[other] Katna에 저장된 이 계정의 폴더 { $folders }개에 있는 모든 메일
}
accounts-remove-outbox = 보낼편지함에서 대기 중인 메일
accounts-remove-settings = 저장된 비밀번호와 Katna 설정
accounts-delete-all-title = Katna 데이터를 모두 삭제하시겠습니까?
accounts-delete-all-confirm = 모두 삭제
accounts-deleting = 삭제하는 중…
accounts-delete-all-accounts = 모든 계정과 Katna에 저장된 모든 메일 및 첨부파일
accounts-delete-all-contacts = 연락처, 캘린더, 검색 색인
accounts-delete-all-settings = 모든 설정, 서명, 단축키
accounts-delete-all-passwords = 저장된 모든 비밀번호
accounts-deleted-heading = 이 컴퓨터에서 삭제되는 항목:
accounts-cannot-undo = 이 작업은 취소할 수 없습니다.
accounts-server-delete-all = 메일 서버에서는 아무것도 바뀌지 않습니다. 메일은 서버에 남아 있으며, 계정을 다시 추가하면 다시 다운로드됩니다. 파일에서 가져온 메일은 Katna에만 있으며, 원본 파일은 건드리지 않습니다.
accounts-server-local = 이 메일은 파일에서 가져온 것이므로 사본이 Katna에만 있습니다. 원본 파일은 건드리지 않으니, 다시 가져오면 복구할 수 있습니다.
accounts-server-remove = 메일 서버에서는 아무것도 바뀌지 않습니다. 메일은 서버에 남아 있으며, 계정을 다시 추가하면 다시 다운로드됩니다.
accounts-confirm-word = 삭제
accounts-confirm-placeholder = “{ accounts-confirm-word }” 입력
accounts-confirm-prompt = 확인하려면 “{ accounts-confirm-word }”를 입력하세요:
accounts-cancel = 취소
