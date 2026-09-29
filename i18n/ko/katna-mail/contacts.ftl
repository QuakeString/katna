# Katna Mail, Korean (한국어): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = 연락처
contacts-frequent = 자주 연락하는 사람
contacts-labels = 라벨
contacts-create = 연락처 만들기

## Search and the list

contacts-search = 연락처 검색
contacts-loading = 연락처를 불러오는 중…
contacts-empty = 저장된 연락처가 아직 없습니다. Gmail, Outlook 또는 메일 서비스에 저장한 연락처가 여기에 표시됩니다.
contacts-empty-no-books = 계정의 연락처는 동기화되면 여기에 표시됩니다.
contacts-none-found = 검색과 일치하는 연락처가 없습니다.
contacts-starred = { $count ->
   *[other] 별표 표시된 연락처 ({ $count })
}
contacts-count = 연락처 ({ $count })
contacts-col-name = 이름
contacts-col-email = 이메일
contacts-col-phone = 전화번호
contacts-col-job = 직책 및 회사
contacts-col-labels = 라벨

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Katna가 { $address }의 연락처를 읽도록 허용합니다.
contacts-allow-many = { $more ->
   *[other] Katna가 { $address } 외 계정 { $more }개의 연락처를 읽도록 허용합니다.
}
contacts-allow-button = 허용

## A contact's page

contacts-back = 연락처로 돌아가기
contacts-edit = 수정
contacts-delete = 삭제
contacts-deleted = { $name } 삭제됨
contacts-find-mail = 메일
contacts-details = 연락처 세부정보
contacts-saved-in = 저장 위치
contacts-notes = 메모
contacts-birthday = 생일
contacts-nickname = 닉네임
contacts-this-computer = 이 컴퓨터
contacts-kind-home = 집
contacts-kind-work = 직장
contacts-kind-mobile = 휴대전화
contacts-kind-other = 기타
contacts-source-google = Google 연락처
contacts-source-microsoft = Outlook 연락처
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = 연락처 만들기
contacts-edit-title = 연락처 수정
contacts-edit-save = 저장
contacts-edit-saving = 저장 중…
contacts-edit-cancel = 취소
contacts-saved = 연락처를 저장했습니다
contacts-edit-save-to = 저장할 위치
contacts-edit-changes-go-to = 변경사항 저장 위치: { $place }
contacts-edit-given = 이름
contacts-edit-family = 성
contacts-edit-company = 회사
contacts-edit-job = 직함
contacts-edit-email = 이메일
contacts-edit-phone = 전화
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = 이메일 추가
contacts-edit-add-phone = 전화번호 추가
contacts-edit-street = 도로명 주소
contacts-edit-city = 시
contacts-edit-postcode = 우편번호
contacts-edit-country = 국가
contacts-edit-birthday = 생일 (YYYY-MM-DD)
contacts-edit-empty = 이름, 이메일 또는 전화번호를 먼저 추가하세요.
