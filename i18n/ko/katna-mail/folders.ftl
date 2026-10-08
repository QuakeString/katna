# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = 라벨
nav-folders = 폴더
nav-label-new = 새 라벨 만들기
nav-folder-new = 새 폴더 만들기
nav-menu-check-mail = 새 메일 확인
nav-menu-check-inbox = 이 받은편지함 확인
nav-unified-leave-out = 통합 받은편지함에서 제외
nav-unified-bring-back = 통합 받은편지함에 다시 포함
nav-menu-sign-in-again = 다시 로그인
nav-menu-new-mail = 이 계정으로 새 메일 쓰기
nav-menu-account-settings = 계정 설정
nav-account-checked = 동기화됨 · { $ago } 확인
nav-account-in-sync = 동기화됨
nav-account-connecting = 연결하는 중…
nav-account-offline = 오프라인, 다시 시도하는 중
nav-account-signed-out = { $provider } 로그인이 만료되었습니다
nav-account-password-refused = 비밀번호가 거부되었습니다
nav-account-storage = { $total } 중 { $used } 사용
nav-menu-new-subfolder = 안에 새 폴더
nav-menu-new-sublabel = 안에 새 라벨
nav-menu-rename = 이름 바꾸기
nav-menu-delete = 삭제
nav-menu-empty-trash = 휴지통 비우기
nav-account-unnamed = 계정 { $number }
nav-all-accounts = 모든 계정
nav-expand = 폴더 보기
nav-collapse = 폴더 숨기기
storage-used = { $total } 중 { $percent }% 사용
storage-used-detail = { $address }: { $total } 중 { $used } 사용

## Special folders (the user's own folders keep their names)

folder-inbox = 받은편지함
folder-starred = 별표편지함
folder-snoozed = 다시 알림 항목
folder-unread = 읽지 않음
folder-important = 중요
folder-drafts = 임시보관함
folder-sent = 보낸편지함
folder-archive = 보관함
folder-spam = 스팸함
folder-trash = 휴지통
folder-all-mail = 전체보관함
folder-scheduled = 예약됨
folder-waiting = 답장 대기 중
folder-waiting-short = 대기 중
folder-reminders = 알림
folder-outbox = 보낼편지함
folder-activity = 활동

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
label-rename-title = 라벨 이름 바꾸기
label-folder-rename-title = 폴더 이름 바꾸기
label-rename = 이름 바꾸기
label-renaming = 이름 바꾸는 중…
label-renamed = 라벨 이름을 “{ $name }”(으)로 바꿨습니다.
label-folder-renamed = 폴더 이름을 “{ $name }”(으)로 바꿨습니다.

## Deleting a folder or label (asked first)

folder-delete-title = “{ $name }”을(를) 삭제할까요?
folder-delete-body = { $count ->
    [0] 메일이 없습니다. 폴더가 서버에서 삭제되므로 웹메일과 휴대전화에서도 사라집니다.
   *[other] { $kind ->
        [conversation] 대화 { $count }개가 휴지통으로 이동하므로 나중에 복원할 수 있습니다.
       *[message] 메일 { $count }개가 휴지통으로 이동하므로 나중에 복원할 수 있습니다.
    } 폴더가 서버에서 삭제되므로 웹메일과 휴대전화에서도 사라집니다.
}
folder-delete-forever-body = { $count ->
    [0] 메일이 없습니다. 폴더가 서버에서 삭제되므로 웹메일과 휴대전화에서도 사라집니다.
   *[other] { $kind ->
        [conversation] 대화 { $count }개가 영구삭제됩니다. 이 계정에는 휴지통이 없습니다.
       *[message] 메일 { $count }개가 영구삭제됩니다. 이 계정에는 휴지통이 없습니다.
    } 폴더가 서버에서 삭제되므로 웹메일과 휴대전화에서도 사라집니다.
}
folder-delete-label-body = 라벨이 삭제됩니다. 메일은 전체보관함과 다른 라벨에 그대로 남습니다.
folder-delete-confirm = 폴더 삭제
folder-delete-label-confirm = 라벨 삭제
folder-deleted = “{ $name }” 폴더를 삭제했습니다
label-deleted = “{ $name }” 라벨을 삭제했습니다
