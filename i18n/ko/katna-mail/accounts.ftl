# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
