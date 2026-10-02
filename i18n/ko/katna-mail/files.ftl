# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = 파일 검색

## Left side (and chips on a phone)

files-all = 모든 파일
files-pictures = 사진
files-pdfs = PDF
files-documents = 문서
files-sheets = 스프레드시트
files-slides = 슬라이드
files-other = 기타
files-accounts = 계정
files-drives = 드라이브
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = 공유 문서함
files-shown = 표시
files-received = 받은 파일
files-sent = 내가 보낸 파일

## Over the files

files-count = { $count ->
   *[other] 파일 { $count }개 · { $size }
}
files-anyone = 모든 사람
files-from-person = 보낸사람: { $name }
files-time-any = 전체 기간
files-time-today = 오늘
files-time-yesterday = 어제
files-time-this-week = 이번 주
files-time-last-week = 지난주
files-time-this-month = 이번 달
files-time-last-month = 지난달
files-time-between = { $first } – { $last }
files-time-hint = 날짜를 클릭하거나 여러 날짜에 걸쳐 드래그하세요
files-time-summary = { $count ->
   *[other] { $days } · 파일 { $count }개
}
files-time-clear = 지우기
files-time-month-back = 이전 달
files-time-month-on = 다음 달
files-time-wheel = 스크롤하면 기간의 길이를 유지한 채 날짜를 이동합니다
files-sort-newest = 최신순
files-sort-oldest = 오래된순
files-sort-largest = 큰 파일순
files-sort-name = 이름순
files-grid = 카드
files-list = 목록
files-this-week = 이번 주
files-undated = 날짜 없음
files-me = 나
files-no-subject = (제목 없음)
files-loading = 메일에서 파일을 모으는 중…
files-empty = 메일의 파일이 여기에 표시됩니다.
files-none-match = 일치하는 파일이 없습니다.
files-load-failed = 파일을 읽지 못했습니다: { $error }

## A file's menu and buttons

files-open = 열기
files-open-with = 다른 앱으로 열기…
files-save = 저장…
files-show-mail = 메일 보기
files-mail-window = 새 창에서 메일 열기
files-forward = 파일 전달
files-from-them = { $name }님이 보낸 파일
files-copy-name = 파일 이름 복사
files-name-copied = 파일 이름을 복사했습니다
files-downloading = 메일을 다운로드하는 중…
files-download-failed = 이 메일을 다운로드할 수 없습니다.

## A cloud drive in place of the mail files

files-drive-mine = 내 드라이브
files-drive-mine-onedrive = 내 파일
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] 파일 { $files }개
   *[other] 폴더 { $folders }개 · 파일 { $files }개
}
files-drive-folders = 폴더
files-drive-files = 파일
files-drive-folder = 폴더
files-drive-meta = { $what } · { $date }에 수정함
files-drive-as-link = { $what } · 링크로 첨부
files-drive-google-doc = Google 문서
files-drive-google-sheet = Google 스프레드시트
files-drive-google-slides = Google 프레젠테이션
files-drive-google-drawing = Google 드로잉
files-drive-fetching = 가져오는 중…
files-drive-loading = 드라이브를 여는 중…
files-drive-empty = 이 폴더는 비어 있습니다.
files-drive-unreachable = { $drive }에 연결할 수 없습니다.
files-drive-try-again = 다시 시도
files-drive-needs-permission = 이 드라이브를 표시하려면 Katna에 한 번 권한을 허용해야 합니다. 다시 로그인하고 Katna가 파일을 볼 수 있도록 허용하세요.
files-drive-allow = 허용
files-drive-allow-failed = 로그인이 완료되지 않아 드라이브를 열 수 없습니다.
files-drive-attach = 첨부
files-drive-more = 더보기
files-drive-download = 다운로드…
files-drive-open-web = { $drive }에서 열기
files-drive-copy-link = 링크 복사
files-drive-link-copied = 링크를 복사했습니다
files-drive-share = 공유…
files-drive-rename = 이름 바꾸기
files-drive-trash = 휴지통으로 이동
files-drive-trashed = “{ $name }”을(를) { $drive } 휴지통으로 이동했습니다
files-drive-renamed = “{ $name }”(으)로 이름을 바꿨습니다
files-drive-getting = { $drive }에서 { $name }을(를) 가져오는 중…
files-drive-get-failed = { $name }을(를) 가져오지 못했습니다: { $error }
files-drive-upload = 업로드
files-drive-upload-files = 파일 업로드
files-drive-upload-folder = 폴더 업로드
files-drive-upload-failed = { $name }을(를) 업로드하지 못했습니다: { $error }
files-drive-upload-needs = 업로드하려면 Katna에 한 번 권한을 허용해야 합니다. 설정 › 기본 앱 › 파일 페이지에서 허용을 누르세요.

## The Share dialog of a drive file or folder

files-share-title = “{ $name }” 공유
files-share-add = 이름이나 주소로 사용자 추가
files-share-not-address = “{ $text }”은(는) 이메일 주소가 아닙니다
files-share-notify = { $drive }에서도 이메일로 알리기
files-share-people = 액세스 권한이 있는 사용자
files-share-general = 일반 액세스
files-share-loading = 액세스 권한이 있는 사용자를 확인하는 중…
files-share-restricted = 제한됨
files-share-restricted-about = 액세스 권한이 있는 사용자만 링크로 열 수 있습니다
files-share-anyone = 링크가 있는 모든 사용자
files-share-anyone-can = { $role ->
    [editor] 링크가 있는 모든 사용자가 편집할 수 있습니다
    [commenter] 링크가 있는 모든 사용자가 댓글을 달 수 있습니다
   *[viewer] 링크가 있는 모든 사용자가 볼 수 있습니다
}
files-share-anyone-about = { $role ->
    [editor] 인터넷에서 링크가 있는 모든 사용자가 편집할 수 있습니다
    [commenter] 인터넷에서 링크가 있는 모든 사용자가 댓글을 달 수 있습니다
   *[viewer] 인터넷에서 링크가 있는 모든 사용자가 볼 수 있습니다
}
files-share-role-owner = 소유자
files-share-role-editor = 편집자
files-share-role-commenter = 댓글 작성자
files-share-role-viewer = 뷰어
files-share-you = { $name }(나)
files-share-domain = { $domain }의 모든 사용자
files-share-inherited = 상위 폴더에서 받은 액세스 권한
files-share-remove = 액세스 권한 삭제
files-share-copy-link = 링크 복사
files-share-share = 공유
files-share-done = 완료
files-share-sharing = 공유하는 중…
files-share-shared = { $count ->
   *[other] { $count }명과 공유했습니다
}
files-share-refused = { $drive }에서 { $addresses }와(과) 공유할 수 없습니다
files-share-failed = 공유 설정을 변경하지 못했습니다: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
   *[other] 항목 { $count }개 업로드 중
}
files-tray-done = { $count ->
   *[other] 업로드 { $count }개 완료
}
files-tray-some-failed = { $done }개 업로드됨, { $failed }개 실패
files-tray-minutes-left = { $minutes ->
   *[other] 약 { $minutes }분 남음
}
files-tray-seconds-left = 1분 미만 남음
files-tray-starting = 시작하는 중…
files-tray-cancel-all = 모두 취소
files-tray-cancel = 취소
files-tray-fold = 목록 숨기기
files-tray-unfold = 목록 표시
files-tray-close = 닫기
files-tray-progress = { $place } · { $size } 중 { $sent }
files-tray-in = 위치: { $place }
files-tray-cancelled = 취소됨
