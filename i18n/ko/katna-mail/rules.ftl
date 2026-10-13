# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = 규칙
settings-rules-summary = 새 메일을 자동으로 분류하거나 라벨을 붙이거나 전달하거나 알림을 끄기
settings-rules-intro = 규칙은 이 순서대로 새 메일을 자동으로 분류합니다. 드래그하여 순서를 바꾸세요.
settings-rules-all-accounts = 모든 계정
settings-rules-new = 새 규칙
settings-rules-none = 아직 규칙이 없습니다. 규칙은 보낸사람, 제목 또는 단어에 따라 새 메일을 자동으로 분류합니다.
settings-rules-none-account = 이 계정에는 아직 규칙이 없습니다.
settings-rules-drag = 드래그하여 순서 바꾸기
settings-rules-edit = 규칙 수정
settings-rules-turn-off = 이 규칙 끄기
settings-rules-turn-on = 이 규칙 켜기

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = 기본 제공 규칙
settings-rules-starters-intro = 직접 켜기 전까지는 꺼져 있습니다. 모든 계정에 적용되며, 수정하여 바꿀 수 있습니다.
settings-rules-starter-turning-on = “{ $name }” 켜는 중…
settings-rules-starter-failed = “{ $name }”을(를) 켤 수 없습니다: { $error }
rules-starter-promotions = 프로모션 알림 끄기
rules-starter-newsletters = 뉴스레터를 읽을거리로
rules-starter-receipts = 영수증 및 청구서
rules-starter-deliveries = 배송
rules-starter-train = 기차표
rules-starter-flight = 항공권
rules-starter-codes = 일회용 코드
rules-starter-security = 보안 알림
rules-starter-social = 소셜 메일
rules-starter-invites = 캘린더 초대
rules-starter-folder-reading = 읽을거리
rules-starter-folder-receipts = 영수증
rules-starter-folder-deliveries = 배송
rules-starter-folder-travel = 여행
rules-starter-folder-social = 소셜
rules-runs-katna = Katna에서 실행
rules-runs-gmail = Gmail에서 실행
rules-runs-sieve = 서버에서 실행
rules-stopped = 중지됨
rules-error-folder-gone = 이 규칙이 사용하는 폴더가 더 이상 없습니다. 규칙을 수정하여 다른 폴더를 선택하세요.
rules-error-no-archive = 이 계정에는 보관함 폴더가 없습니다. 규칙을 수정하여 다른 동작을 선택하세요.
rules-error-no-trash = 이 계정에는 휴지통 폴더가 없습니다. 규칙을 수정하여 다른 동작을 선택하세요.
rules-error-cannot-send = 이 계정은 메일을 보낼 수 없으므로 규칙이 메일을 전달할 수 없습니다.
rules-error-other = { $error }. 규칙을 수정한 다음 다시 켜세요.
settings-folders = 폴더
settings-folders-summary = 폴더 창의 읽지 않은 메일 수
settings-folders-unread-counts = 모든 폴더에 읽지 않은 메일 수 표시
settings-folders-unread-counts-detail = 끄면 받은편지함에만 읽지 않은 메일 수가 표시됩니다

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } 그리고 { $next }
rules-summary-or = { $first } 또는 { $next }
rules-summary-more = 외 { $count }개
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator }: { $value }
rules-summary-has-attachment = 첨부파일 있음
rules-summary-no-attachment = 첨부파일 없음
rules-summary-mailing-list = 메일링 리스트에서 보냄
rules-summary-not-mailing-list = 메일링 리스트에서 보내지 않음
rules-summary-tab = { $tab } 탭에 있음
rules-summary-not-tab = { $tab } 탭에 없음
rules-summary-move = { $folder }(으)로 이동
rules-summary-archive = 받은편지함 건너뛰기
rules-summary-trash = 휴지통으로 이동
rules-summary-mark-read = 읽음으로 표시
rules-summary-star = 별표 표시
rules-summary-important = 중요 표시
rules-summary-label = { $label } 라벨 지정
rules-summary-forward = { $address }(으)로 전달
rules-summary-dont-notify = 알리지 않음
rules-summary-read-after = { $count ->
   *[other] { $count }일 후 읽음으로 표시
}
rules-summary-folder-gone = 삭제된 폴더

## The rule editor

rules-editor-new-title = 새 규칙
rules-editor-edit-title = 규칙 수정
rules-editor-name-hint = 규칙 이름
rules-editor-when = 새 메일이 다음 조건 중
rules-editor-of-these = 일치할 때:
rules-mode-all = 모두
rules-mode-any = 하나라도
rules-field-from = 보낸사람
rules-field-to = 받는사람
rules-field-cc = 참조
rules-field-any-recipient = 받는사람 또는 참조
rules-field-reply-to = 회신 주소
rules-field-subject = 제목
rules-field-body = 본문
rules-field-attachment-name = 첨부파일 이름
rules-field-has-attachment = 첨부파일 있음
rules-field-mailing-list = 메일링 리스트에서 보냄
rules-field-tab = 받은편지함 탭
rules-comparator-contains = 포함
rules-comparator-not-contains = 포함 안 함
rules-comparator-begins-with = 시작 문구
rules-comparator-ends-with = 끝 문구
rules-comparator-equals = 정확히 일치
rules-comparator-matches = 패턴 일치
rules-has-yes = 예
rules-has-no = 아니요
rules-editor-value-hint = 단어 또는 주소
rules-editor-add-condition = 조건 추가
rules-editor-remove = 삭제
rules-editor-then = 동작:
rules-action-move = 이동
rules-action-archive = 받은편지함 건너뛰기(보관처리)
rules-action-trash = 휴지통으로 이동
rules-action-mark-read = 읽음으로 표시
rules-action-star = 별표 표시
rules-action-important = 중요 표시
rules-action-label = 라벨 추가
rules-action-forward = 전달
rules-action-dont-notify = 알리지 않음
rules-action-read-after = 다음 기간 후 읽음으로 표시
rules-editor-choose-folder = 폴더 선택
rules-editor-choose-label = 라벨 선택
rules-editor-new-folder = 새 폴더: { $name }
rules-editor-folder-of = { $folder }({ $account })
rules-editor-forward-hint = 이메일 주소
rules-editor-days = 일
rules-editor-add-action = 동작 추가
rules-editor-stop = 여기서 중지: 이 메일에 이후 규칙을 실행하지 않음
rules-editor-accounts = 계정:
rules-editor-accounts-none = 계정 선택
rules-editor-accounts-many = { $count ->
   *[other] 계정 { $count }개
}
rules-editor-matches = 최근 { $days }일 동안 { $mails } 일치
rules-editor-mails = { $count ->
   *[other] 메일 { $count }개
}
rules-editor-counting = 일치하는 메일을 세는 중…
rules-editor-show = 보기
rules-editor-also-apply = 이 { $count }개에도 적용
rules-editor-runs-katna = 이 컴퓨터가 켜져 있는 동안 Katna에서 실행됩니다.
rules-editor-runs-gmail = Gmail에서 실행되므로 휴대전화에서도, 이 컴퓨터가 꺼져 있을 때도 작동합니다.
rules-editor-runs-sieve = 메일 서버에서 실행되므로 휴대전화에서도, 이 컴퓨터가 꺼져 있을 때도 작동합니다.
rules-note-gmail-action = Katna에서 실행: Gmail 필터는 “{ $action }” 동작을 할 수 없습니다.
rules-note-sieve-action = Katna에서 실행: 메일 서버의 규칙은 “{ $action }” 동작을 할 수 없습니다.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Katna에서 실행: Gmail 필터는 Katna처럼 “{ $test }” 조건을 검사할 수 없습니다.
rules-note-sieve-condition = Katna에서 실행: 메일 서버의 규칙은 Katna처럼 “{ $test }” 조건을 검사할 수 없습니다.
rules-note-order = 이 계정의 앞선 규칙이 Katna에서 실행되므로 이 규칙도 Katna에서 실행됩니다. 규칙은 목록 순서대로 실행됩니다.
rules-note-gmail-stop = Katna에서 실행: Gmail 필터는 이후 규칙의 실행을 막을 수 없습니다.
rules-note-gmail-forward = Katna에서 실행: Gmail은 설정에서 인증된 주소로만 전달하며, { $address }은(는) 인증된 주소가 아닙니다.
rules-note-gmail-folder = Katna에서 실행: 이 규칙이 사용하는 폴더에 해당하는 Gmail 라벨이 없습니다.
rules-note-sieve-folder = Katna에서 실행: 이 규칙이 사용하는 폴더가 메일 서버에 없습니다.
rules-note-gmail-sign-in = Google에 다시 로그인하여 Katna가 Gmail 필터를 만들도록 허용할 때까지 Katna에서 실행됩니다.
rules-note-sieve-other-script = Katna에서 실행: 메일 서버에서 다른 규칙 스크립트(“{ $name }”)가 사용 중입니다.
rules-note-gmail-failed = Katna에서 실행: Gmail이 받아들이지 않았습니다({ $error }).
rules-note-sieve-failed = Katna에서 실행: 메일 서버가 받아들이지 않았습니다({ $error }).
rules-editor-cancel = 취소
rules-editor-save = 저장
rules-editor-saving = 저장하는 중…
rules-editor-delete = 규칙 삭제
rules-editor-delete-ask = 이 규칙을 삭제할까요?
rules-editor-delete-keep = 유지
rules-editor-delete-confirm = 삭제
rules-editor-needs-folder = 각 “이동”에는 폴더를, 각 “라벨 추가”에는 라벨을 선택하세요.
rules-editor-needs-days = “다음 기간 후 읽음으로 표시”에는 1~3650 사이의 일수를 입력해야 합니다.
rules-saved = 규칙을 저장했습니다
rules-saved-applied = { $count ->
   *[other] 규칙을 저장하고 메일 { $count }개에 적용했습니다
}
rules-apply-failed = 규칙을 저장했지만 적용하지 못했습니다: { $error }
rules-deleted = 규칙을 삭제했습니다
rules-delete-failed = 규칙을 삭제할 수 없습니다: { $error }
rules-change-failed = 규칙을 변경할 수 없습니다: { $error }
