# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The signature being edited

signature-placeholder = 이름과 그 아래에 덧붙일 내용

## Its formatting bar

signature-bold = 굵게
signature-italic = 기울임꼴
signature-underline = 밑줄
signature-link = 링크
signature-link-apply = 적용
signature-picture = 그림 삽입
signature-align-left = 왼쪽 정렬
signature-align-center = 가운데 정렬
signature-align-right = 오른쪽 정렬
signature-numbered-list = 번호 매기기 목록
signature-bulleted-list = 글머리기호 목록
signature-remove-formatting = 서식 지우기

## Adding a picture

signature-picture-choose = 삽입
signature-picture-too-big = 서명에 넣는 그림은 최대 { $size }까지 가능합니다.
signature-picture-kind = PNG, JPEG, GIF 또는 WebP 그림을 선택하세요.
signature-picture-unreadable = { $name }: { $error }

## Layouts

signature-layout = 레이아웃
signature-layout-own = 직접 작성
signature-layout-classic = 클래식
signature-layout-logo-left = 왼쪽 로고
signature-layout-photo = 사진
signature-layout-band = 색상 띠
signature-layout-one-line = 한 줄
signature-layout-centred = 가운데 정렬
signature-layout-banner = 배너 포함
signature-layout-underline = 밑줄
signature-layout-side-bar = 사이드바
signature-layout-card = 카드
signature-layout-monogram = 모노그램
signature-layout-plain = 일반 텍스트
signature-layout-mobile-label = M:
signature-layout-office-label = O:
signature-layout-email-label = E:
signature-layout-name = 이름
signature-layout-job = 직함
signature-layout-company = 회사
signature-layout-mobile = 휴대전화
signature-layout-office = 사무실
signature-layout-email = 이메일
signature-layout-website = 웹사이트
signature-layout-address = 주소
signature-layout-pictures = 사진
signature-layout-logo = 로고
signature-layout-photo-picture = 사진
signature-layout-banner-picture = 배너
signature-layout-remove-picture = 삭제
signature-layout-pages = 페이지
signature-layout-page-placeholder = 페이지 주소 추가
signature-layout-colour = 색상
signature-layout-picture-failed = { $name }은(는) 사진으로 사용할 수 없습니다.
signature-layout-preview = 받는사람에게 보이는 모습
signature-layout-light = 밝게
signature-layout-dark = 어둡게
signature-layout-text = 일반 텍스트
signature-layout-inside = 사진은 메일 안에 포함되어 전송되므로 웹 이미지를 끈 곳에서도 표시됩니다. 이 사진으로 메일마다 { $size }가 늘어납니다.
signature-layout-free = 다른 것을 원하세요?
signature-layout-edit = 직접 편집
signature-layout-edit-confirm = 직접 편집할까요? 입력란과 레이아웃은 없어지며, 편집기가 지원하는 범위에서 모양은 유지됩니다.
signature-layout-use-confirm = { $layout } 레이아웃을 사용할까요? 이 서명의 내용으로 채운 레이아웃이 이 서명을 대체합니다.
signature-layout-use = 레이아웃 사용
signature-layout-cancel = 취소

## Paste HTML

signature-html-title = HTML 붙여넣기
signature-html-subtitle = 다른 곳에서 디자인한 서명용
signature-html-placeholder = 서명의 HTML을 여기에 붙여넣으세요
signature-html-name = 붙여넣은 서명
signature-html-new = 새 서명 “{ $name }”(으)로 저장됩니다
signature-html-replaces = “{ $name }”에 덮어쓰기합니다
signature-html-cancel = 취소
signature-html-save = 저장
signature-html-fetching = 사진을 다운로드하는 중…
signature-html-pictures-inside = { $count ->
   *[other] 사진 { $count }개를 다운로드하여 메일 안에 포함함({ $size })
}
signature-html-pictures-web = { $count ->
   *[other] 사진 { $count }개를 다운로드할 수 없어 받는사람이 웹에서 불러옵니다
}
signature-html-removed = 스크립트, 양식, 추적 픽셀을 삭제함(메일 앱에서 어차피 차단됨)
signature-html-style-sheet = 스타일 시트 제외함: 메일에는 각 요소에 직접 작성한 스타일만 유지됨
signature-html-links = 웹사이트, 이메일 주소, 전화번호가 아닌 곳으로 연결되는 링크를 삭제함
signature-html-plain-text = 텍스트만 표시하는 메일 앱을 위해 일반 텍스트 버전을 만듦

## Import

signature-import-title = 가져오기
signature-import-subtitle = Gmail, Thunderbird, Evolution, KMail에서
signature-import-looking = 서명을 찾는 중…
signature-import-none = 서명을 찾을 수 없습니다. 다른 앱의 서명은 HTML을 복사한 다음 HTML 붙여넣기를 사용하세요.
signature-import-from = { $app }에서
signature-import-already = 이미 Katna에 있음
signature-import-gmail-sign-in = { $address }: Katna가 Gmail 서명을 읽을 수 있도록 설정 > 계정에서 다시 로그인하세요.
signature-import-gmail-failed = { $address }: { $error }
signature-import-cancel = 취소
signature-import-do = { $count ->
   *[other] 서명 { $count }개 가져오기
}
signature-import-name = { $name }({ $app })
