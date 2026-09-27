# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = 폴더 창
accounts-folder-pane-detail = 왼쪽 창에 어떤 계정의 폴더를 표시할지 정합니다.
accounts-shown-one = 한 번에 계정 하나, 계정 카드에서 전환
accounts-shown-all = 모든 계정을 차례로
accounts-unified = 통합 받은편지함
accounts-unified-switch = 모든 계정의 메일을 함께 보기
accounts-unified-switch-detail = “모든 계정”이 폴더 창 맨 위에 있으며, 각 계정의 받은편지함, 보낸 메일 등을 하나의 목록으로 보여 줍니다. 그 아래의 계정은 접힌 상태로 시작합니다.
accounts-row = 계정
accounts-row-detail = 폴더 창과 계정 메뉴에는 계정이 이 순서대로 표시되며, 첫 번째 계정이 기본 계정입니다. 계정을 삭제하면 이 컴퓨터에 있는 Katna의 메일 사본이 삭제됩니다. 메일은 서버에 남습니다.
accounts-none = 아직 계정이 없습니다.
accounts-kind-imported = 가져옴
accounts-picture-reset = 데스크톱 사진 사용
accounts-picture-change = 사진 변경
accounts-picture-remove = 사진 삭제
accounts-rename = 이름 바꾸기
accounts-name-save = 저장
accounts-name-cancel = 취소
accounts-name-placeholder = 내 이름
accounts-rename-failed = 계정 이름을 바꿀 수 없습니다: { $error }
accounts-move-up = 위로 이동
accounts-move-down = 아래로 이동
accounts-drag = 드래그하여 순서 변경
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
reset-cache-about = Katna가 다운로드한 메일과 첨부파일, 보낸사람 사진, 검색 색인을 삭제한 다음 최근 메일을 다시 다운로드합니다. 계정, 설정, 이 컴퓨터에만 있는 메일은 그대로 유지됩니다.
reset-cache-button = 캐시 재설정
reset-cache-title = 캐시를 재설정할까요?
reset-cache-deleted = 삭제한 후 다시 다운로드하는 항목:
reset-cache-mail = IMAP 서버에서 다운로드한 메일과 첨부파일: 최근 메일은 지금 바로, 오래된 메일은 열 때 다시 다운로드됩니다
reset-cache-index = 검색 색인 (바로 다시 만들어집니다)
reset-cache-pictures = 보낸사람 사진
reset-cache-kept = 유지되는 항목: 계정, 비밀번호, 설정, 별표, 라벨, 읽음 표시, 고정, 임시보관함, 보낼편지함, 아직 서버에 반영되지 않은 변경사항, POP3 계정이나 가져온 파일의 메일(다른 사본이 없을 수 있음). 메일 서버에서는 아무것도 바뀌지 않습니다.
reset-cache-confirm = 캐시 재설정
reset-cache-busy = 재설정하는 중…
reset-cache-done = 캐시를 재설정했습니다. 최근 메일을 다시 다운로드하는 중입니다.
reset-cache-done-freed = 캐시를 재설정하고 { $size }를 확보했습니다. 최근 메일을 다시 다운로드하는 중입니다.
