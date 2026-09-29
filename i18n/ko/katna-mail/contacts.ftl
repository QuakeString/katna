# Katna Mail, Korean (한국어): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = 연락처
contacts-frequent = 자주 연락하는 사람
contacts-other = 기타 연락처
contacts-other-about = Gmail에서 메일을 보냈지만 저장하지 않은 사람
contacts-other-email = 이메일 보내기
contacts-other-empty = 기타 연락처가 없습니다. Gmail에서 메일을 보냈지만 저장하지 않은 사람이 여기에 표시됩니다.
contacts-other-allow = 기타 연락처를 보려면 Gmail 계정에 다시 로그인하고 Katna가 볼 수 있도록 허용하세요.
contacts-labels = 라벨
contacts-label-options = 라벨 옵션
contacts-label-rename = 라벨 이름 바꾸기
contacts-label-email = 모두에게 메일 보내기
contacts-label-delete = 라벨 삭제
contacts-label-new = 새 라벨
contacts-label-name = 라벨 이름
contacts-label-button = 라벨
contacts-label-menu = 라벨 지정:
contacts-label-added = { $name }에 추가됨
contacts-label-removed = { $name }에서 삭제됨
contacts-label-renamed = 라벨 이름이 { $name }(으)로 변경됨
contacts-label-deleted = 라벨 { $name } 삭제됨
contacts-label-no-email = 이 라벨에는 이메일 주소가 있는 사람이 없습니다
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = 계정
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = 연락처를 표시하려면 다시 로그인하세요
contacts-account-signed-in = { $address }에 다시 로그인했습니다. 연락처를 가져오는 중…
contacts-account-sign-in-refused = { $provider }에서 Katna의 접근을 허용하지 않았습니다. 다시 시도하고 연락처에 대한 접근을 허용하세요.
contacts-account-password = 서버에서 비밀번호를 받아들이지 않았습니다. Yahoo, iCloud, Zoho 등은 앱 비밀번호가 필요합니다.
contacts-account-change-password = 비밀번호 변경
contacts-account-change-password-tooltip = 설정 > 계정 열기
contacts-account-failed = 연락처를 읽을 수 없습니다.
# $reason is the server's own words, in English.
contacts-account-error = 연락처를 읽을 수 없습니다: { $reason }
contacts-account-none = 주소록을 찾을 수 없음
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = 주소록을 찾을 수 없음: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider }에서는 { $provider }로 로그인한 Katna에만 연락처를 보여 줍니다.
contacts-account-sign-in-with = { $provider }로 로그인
contacts-account-looking = 연락처를 찾는 중…
contacts-account-try-again = 다시 시도
contacts-account-try-again-tooltip = 지금 이 계정의 연락처를 다시 확인
contacts-account-fixing = 해결하는 중…
contacts-manage = 수정 및 관리
contacts-merge = 병합 및 수정
contacts-merge-about = { $count ->
   *[other] 제안 { $count }개: 동일한 사람으로 보이는 연락처
}
contacts-merge-none = 중복 항목이 없습니다. 이름이나 전화번호가 같은 연락처가 여기에 표시됩니다.
contacts-merge-count = { $count ->
   *[other] 연락처 { $count }개
}
contacts-merge-all = 모두 병합
contacts-merge-button = 병합
contacts-merge-dismiss = 무시
contacts-merged = { $count ->
    [1] 연락처를 병합했습니다
   *[other] { $count }건 병합 완료
}
contacts-import = 가져오기
contacts-export = 내보내기
contacts-import-file = vCard 또는 CSV 파일에서 연락처 가져오기
contacts-imported = { $count ->
   *[other] 연락처 { $count }개를 { $place }에 가져왔습니다
}
contacts-imported-some = { $count ->
   *[other] 연락처 { $count }개를 { $place }에 가져왔습니다. 이미 저장된 { $skipped }개는 제외했습니다
}
contacts-import-none = { $name }에서 연락처를 찾을 수 없습니다
contacts-import-all-saved = { $name }의 모든 사용자가 이미 저장되어 있습니다
contacts-import-failed = { $name }을(를) 읽을 수 없습니다: { $error }
contacts-exported = { $count ->
   *[other] 연락처 { $count }개를 { $path }(으)로 내보냈습니다
}
contacts-export-none = 내보낼 연락처가 없습니다
contacts-export-failed = 연락처를 내보낼 수 없습니다: { $error }
contacts-print = 인쇄
contacts-print-title = 연락처
contacts-print-none = 인쇄할 연락처가 없습니다
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = 생일: { $day }
contacts-print-nickname = 닉네임: { $name }
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
contacts-qr = QR 코드로 공유
contacts-qr-about = 휴대전화 카메라로 스캔하여 연락처를 저장하세요.
contacts-qr-too-long = 이 연락처는 세부 정보가 너무 많아 QR 코드에 담을 수 없습니다.
contacts-qr-done = 완료
contacts-deleted = { $name } 삭제됨
contacts-added = { $name } 연락처에 추가됨
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
