# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Katna 정보
about-tagline = Linux 데스크톱을 위한 메일과 캘린더
about-copy-version = 버전 정보 복사
about-version-copied = 복사됨
about-version-built = 빌드: { $date }
about-version-system = 시스템: { $system }
about-whats-new = 새로운 기능

## Updates, in a box under the version in About (only in packages that
## update themselves). $version is a version such as 0.0.0.r236.g1a2b3c4.

about-update-not-checked = 아직 업데이트를 확인하지 않았습니다
about-update-checking = 업데이트를 확인하는 중…
about-update-up-to-date = Katna Mail이 최신 버전입니다
about-update-check-failed = 업데이트를 확인할 수 없습니다
about-update-available = 버전 { $version }을(를) 사용할 수 있습니다
about-update-downloading = 버전 { $version } 다운로드 중… { $percent }%
about-update-download-failed = 버전 { $version } 다운로드를 완료하지 못했습니다
about-update-ready = 버전 { $version } 설치 준비가 되었습니다
about-update-ready-detail = 업데이트를 마치려면 Katna Mail이 다시 시작됩니다.
about-update-confirm = 버전 { $version }을(를) 설치할까요?
about-update-confirm-detail = Katna Mail이 종료되고 업데이트를 설치한 다음, 하던 곳에서 다시 열립니다. 컴퓨터가 비밀번호를 요청합니다.
about-update-confirm-detail-windows = Katna Mail이 종료되고 업데이트를 설치한 다음 잠시 후 다시 열립니다.
about-update-installing = 버전 { $version } 설치하는 중…
about-update-installing-detail = 열린 창에 비밀번호를 입력하세요.
about-update-installing-detail-windows = Katna Mail이 지금 종료되며, 업데이트가 설치되면 다시 열립니다.
about-update-cancelled = 비밀번호를 입력하지 않아 업데이트가 설치되지 않았습니다.
about-update-failed = 업데이트를 설치할 수 없습니다: { $error }
about-update-not-self-updating = 이 Katna Mail은 스스로 업데이트하지 않습니다. 설치한 방법으로 업데이트하세요.
about-update-restart-failed = 업데이트는 설치되었지만 Katna Mail을 다시 열 수 없었습니다({ $error }). 직접 열어 주세요.
about-update-check = 업데이트 확인
about-update-download = 다운로드
about-update-retry = 다시 시도
about-update-button = 업데이트
about-update-restart = 업데이트하고 다시 시작
about-update-cancel = 나중에
about-changelog = 변경 기록
about-source = 소스 코드
about-coffee = 커피 한 잔 사 주기
about-coffee-coffee = 커피?
about-coffee-tea = 차?
about-coffee-pizza = 피자?
about-coffee-nothing = 아무것도? 정말요?
about-coffee-water = 물만 마셔도 살아요!!
about-coffee-thanks = Katna를 써 주셔서 고마워요
about-coming-soon = 곧 제공 예정
about-follow-me = 팔로우하기:
about-love-title = Rust, KDE, Linux를 향한 사랑으로 만들었습니다
about-love-text = Rust 덕분에 빠르고 안전한 메일 앱을 즐겁게 만들 수 있습니다. Katna에는 unsafe 코드가 없습니다. KDE의 Plasma 데스크톱과 PIM 제품군은 Katna에 영감을 주었고, Linux와 자유 소프트웨어 커뮤니티는 Katna가 서 있는 토대를 만들어 줍니다. 감사합니다. 아래의 라이브러리에도 감사드립니다.
about-kde-text = KDE는 Katna가 가장 편안하게 어울리는 데스크톱을 만듭니다. KDE는 자원봉사자들이 만들고 여러분 같은 사람들의 후원으로 운영됩니다. Plasma나 KDE 앱이 마음에 드신다면 KDE에 기부해 주세요.
about-donate-kde = KDE에 기부하기
about-gpui-title = Zed 프로젝트의 GPUI로 만들었습니다
about-gpui-text = Katna Mail의 인터페이스는 모두 Zed Industries가 Zed 편집기를 위해 만든 빠른 GPU 가속 UI 프레임워크인 GPUI로 만들어졌습니다. 화면에 보이는 모든 픽셀, 애니메이션, 창을 GPUI가 그립니다. 공개적으로 개발해 준 Zed 팀에 감사드립니다. Apache-2.0.
about-gpui-github = GitHub의 GPUI
about-personal-title = 개인 프로젝트
about-personal-text = Katna Mail은 새롭거나 혁신적인 앱이 되려고 하지 않습니다. 개발자가 원하던 메일 앱이며, 기능과 모양은 Gmail, Mailspring, Thunderbird에서 빌려 왔습니다. LLM이 이만큼 발전했기에 가능했습니다.
about-built-on = 자유 소프트웨어 기반
about-credit-pimalaya = IMAP, SMTP 및 로그인(io-imap, io-smtp, io-sasl)
about-credit-imap-codec = IMAP 읽기 및 쓰기
about-credit-tantivy = 검색
about-credit-sqlite = 메일 저장소
about-credit-rustls = 보안 연결
about-credit-mail-parser = 메일 읽기, Stalwart Labs 제공
about-credit-html5ever = HTML 메일, Servo 프로젝트 제공
about-credit-zbus = D-Bus와 포털을 통한 데스크톱 연동
about-credit-oo7 = 데스크톱 키링의 비밀번호
about-credit-hayro = PDF 보기 및 인쇄
about-credit-calamine = 스프레드시트 미리 보기
about-credit-resvg = SVG 그림
about-credit-jiff = 날짜와 시간대
about-credit-spellbook = 맞춤법 검사, Helix 편집기 제공
about-credit-smol = 여러 작업 동시 처리
about-credit-color-schemes = 기본 제공 색 구성표의 팔레트
about-all-libraries = Katna가 사용하는 모든 라이브러리({ $count }개)
about-library-authors = 만든 사람: { $authors }
about-license = Katna는 GNU GPL 버전 3 이상에 따른 자유 소프트웨어입니다.
about-close = 닫기

## What’s new (shown after an update)

whats-new-title = Katna Mail의 새로운 기능
whats-new-updated = 버전 { $version }(으)로 업데이트됨
whats-new-version = 버전 { $version }
whats-new-more = 전체 변경 기록에서 { $count }개의 변경 사항을 더 볼 수 있습니다.
whats-new-changelog = 전체 변경 기록
whats-new-got-it = 확인

## First run: welcome page

onboarding-welcome-title = Katna Mail에 오신 것을 환영합니다
onboarding-welcome-lead = 내 컴퓨터에 있는 내 메일: 빠르게 검색하고, 오프라인에서도 읽고, 비공개로 보관하세요.
onboarding-fast-title = 오프라인에서도 빠르게
onboarding-fast-text = Katna는 메일 사본을 이 컴퓨터에 보관하므로 연결 여부와 관계없이 메일을 바로 열고 검색할 수 있습니다.
onboarding-providers-title = 사용 중인 메일과 함께
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud 및 기타 모든 IMAP 또는 POP 계정.
onboarding-private-title = 비공개
onboarding-private-text = 메일은 메일 제공업체에서 이 컴퓨터로 바로 전달됩니다. Katna 서버는 메일을 볼 수 없습니다.
onboarding-get-started = 시작하기

## First run: adding an account

onboarding-service-checking = Katna 백그라운드 서비스 확인 중…
onboarding-service-running = Katna 백그라운드 서비스가 실행 중입니다.
onboarding-service-missing = Katna 백그라운드 서비스가 실행되고 있지 않습니다
onboarding-service-start = 이 서비스가 메일을 가져오고 보냅니다. 터미널에서 서비스를 시작한 다음 다시 확인하세요.
onboarding-check-again = 다시 확인
onboarding-account-title = 메일 계정 추가
onboarding-account-lead = 이메일 주소와 비밀번호를 입력하면 Katna가 서버 설정을 찾아 줍니다. Gmail, Yahoo, iCloud는 계정의 보안 설정에서 만든 앱 비밀번호가 필요합니다.
onboarding-add-account = 계정 추가
onboarding-back = 뒤로

## First run: choosing the look

onboarding-look-title = 나에게 맞게 꾸미기
onboarding-look-lead = 메일을 여는 방식과 Katna의 모양을 선택하세요. 빠른 설정에서 언제든지 바꿀 수 있습니다.
onboarding-reading-pane = 읽기 창
onboarding-pane-right = 목록 오른쪽
onboarding-pane-none = 분할 안함
onboarding-theme = 테마
onboarding-theme-system = 시스템
onboarding-theme-light = 밝게
onboarding-theme-dark = 어둡게
onboarding-density = 표시 밀도
onboarding-density-default = 기본값
onboarding-density-compact = 간단히
onboarding-continue = 계속

## First start: the Katna account page. A Katna account is an account on
## Katna's own server, not a mail account; see katna-account.ftl.

onboarding-katna-title = Katna 계정으로 더 많은 기능을 이용하세요
onboarding-katna-lead = 선택 사항입니다. Katna의 온라인 기능을 사용할 수 있으며, 나중에 설정 > 구독에서 만들 수도 있습니다.
onboarding-katna-receipts-title = 읽음 확인
onboarding-katna-receipts-text = 보낸 메일을 상대방이 언제 열었는지 확인하세요.
onboarding-katna-links-title = 링크 추적
onboarding-katna-links-text = 메일의 어떤 링크가 클릭되었는지 확인하세요.
onboarding-katna-activity-title = 활동
onboarding-katna-activity-text = 보낸 모든 메일의 열람과 클릭을 한곳에서 확인하세요.
onboarding-katna-translate-title = 자동 번역
onboarding-katna-translate-text = 다른 언어로 된 메일을 내 언어로 읽으세요.
onboarding-katna-private = 별도의 비밀번호를 사용합니다. 메일 로그인 정보는 이 컴퓨터 밖으로 나가지 않습니다.

## First run: done

onboarding-ready-title = 모든 준비가 끝났습니다
onboarding-ready-lead = Katna가 메일을 가져오고 있습니다. 메일은 도착하는 대로 표시되며, 새 메일도 자동으로 나타납니다.
onboarding-ready-lead-address = Katna가 { $address }의 메일을 가져오고 있습니다. 메일은 도착하는 대로 표시되며, 새 메일도 자동으로 나타납니다.
onboarding-apps = 사용할 앱
onboarding-ready-tour = 1분 둘러보기로 각 기능의 위치를 확인하시겠습니까?
onboarding-skip = 나중에
onboarding-take-tour = 둘러보기

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Katna 개선에 참여
share-lead = Katna에 오류가 발생하면 이 컴퓨터에 보고서가 저장됩니다. 이 보고서를 보내 주시면 문제를 고치는 데 도움이 됩니다. 설정 > 사용자 의견에서 언제든지 바꿀 수 있습니다.
share-sent = 보내는 정보
share-sent-detail = 설정에서 볼 수 있는 그대로의 오류 보고서: 무엇이 Katna의 어디에서 멈췄는지, 버전, Linux 시스템과 데스크톱, 그리고 Katna 로그의 마지막 줄(메일 폴더 이름이 들어 있을 수 있음).
share-never-sent = 절대 보내지 않는 정보
share-never-sent-detail = 메시지, 연락처, 비밀번호, IP 주소, 사용자 이름, 컴퓨터 이름. 이메일 주소는 보고서에서 삭제됩니다.
share-where = 보내는 곳
share-where-detail = Sentry에 있는 Katna의 오류 추적기이며, EU에 저장됩니다. 보고서를 사용자와 연결하는 ID는 없습니다.
share-dont-send = 보내지 않기
share-send = 오류 보고서 보내기
share-sending = 오류 보고서를 보냅니다. 감사합니다.
share-local = 오류 보고서는 이 컴퓨터에만 남습니다.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Katna Mail에 오신 것을 환영합니다
tour-welcome-text = 1분 둘러보기로 각 기능의 위치를 알려 드립니다.
tour-not-now = 나중에
tour-start = 둘러보기
tour-close = 닫기
tour-skip = 둘러보기 건너뛰기
tour-back = 뒤로
tour-done = 완료
tour-next = 다음
tour-step = { $step }/{ $total }
tour-compose-title = 메시지 쓰기
tour-compose-text = 편지쓰기를 누르면 오른쪽 아래에 새 메시지가 열리므로, 쓰는 동안에도 계속 메일을 읽을 수 있습니다.
tour-search-title = 모든 메일 검색
tour-search-text = 검색은 오프라인에서도 됩니다. 오른쪽 끝의 버튼으로 보낸사람, 받는사람, 제목, 날짜, 첨부파일 필터를 추가할 수 있습니다.
tour-menu-title = 폴더 표시 또는 숨기기
tour-menu-text = 이 버튼은 폴더 목록을 접습니다. 목록이 숨겨져 있을 때 왼쪽의 메일에 포인터를 올리면 폴더가 보입니다.
tour-apps-title = 앱
tour-apps-text = 메일은 캘린더, 연락처, 할 일, 메모, 파일과 함께 여기에 있습니다.
tour-tabs-title = 받은편지함 탭
tour-tabs-text = 새 메일은 기본, 프로모션, 소셜, 업데이트, 포럼으로 분류됩니다. 빠른 설정에서 탭을 끌 수 있습니다.
tour-list-title = 메시지
tour-list-text = 메시지를 클릭하면 읽을 수 있습니다. 포인터를 올리면 빠른 작업이, 마우스 오른쪽 버튼을 클릭하면 더 많은 작업이 나타나며, 여러 개를 선택해 한 번에 처리할 수도 있습니다.
tour-settings-title = 빠른 설정
tour-settings-text = 여기에서 읽기 창, 표시 밀도, 테마를 바꿀 수 있습니다. 둘러보기도 여기에서 다시 시작할 수 있습니다.
tour-account-title = 계정
tour-account-text = 현재 사용 중인 계정을 확인하고 다른 계정을 추가할 수 있습니다.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Katna 백그라운드 서비스가 예기치 않게 중지되었습니다.
   *[other] Katna 백그라운드 서비스가 예기치 않게 중지되었습니다. 저장된 오류 보고서가 { $more }개 더 있습니다.
}
crash-mail = { $more ->
    [0] 지난번에 Katna Mail이 예기치 않게 종료되었습니다.
   *[other] 지난번에 Katna Mail이 예기치 않게 종료되었습니다. 저장된 오류 보고서가 { $more }개 더 있습니다.
}
crash-view = 보고서 보기
crash-view-tooltip = 이 컴퓨터에 저장된 보고서 열기
crash-copy = 보고서 복사
crash-close = 닫기

## Sign in again (a bar at the bottom when Google or Microsoft stopped
## letting an account in; $provider: Google or Microsoft)

sign-in-again-button = 로그인
sign-in-again-tooltip = 브라우저에서 { $provider } 로그인 페이지 열기
sign-in-again-waiting = 브라우저를 기다리는 중…
google-api-off = Katna의 Google Cloud 프로젝트에서 { $api }이(가) 꺼져 있습니다.
google-api-turn-on = 켜기
google-api-turn-on-tooltip = Google Cloud를 열어 { $api }을(를) 켠 다음 다시 시도를 누르세요
sign-in-again-done = { $address }에 다시 로그인했습니다. 메일을 가져오는 중…

## Before deleting several conversations, or deleting for good

delete-ask-title = { $kind ->
    [conversation] 대화 { $count }개를 휴지통으로 이동하시겠습니까?
   *[message] 메일 { $count }개를 휴지통으로 이동하시겠습니까?
}
delete-ask-body = { $count ->
   *[other] 바로 뒤에 실행취소할 수 있고, 나중에 휴지통에서 되돌릴 수도 있습니다.
}
delete-ask-confirm = 휴지통으로 이동
delete-forever-title = { $kind ->
    [conversation] 대화 { $count }개를 영구삭제하시겠습니까?
   *[message] 메일 { $count }개를 영구삭제하시겠습니까?
}
delete-forever-body = { $count ->
   *[other] 서버에서도 삭제됩니다. 되돌릴 수 없습니다.
}
delete-forever-confirm = 영구삭제
delete-ask-dont-ask = 다시 묻지 않기
delete-ask-cancel = 취소
