# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = 읽기
chat-view = 대화를 채팅으로
chat-view-detail = 사람들 사이의 메일을 단체 채팅처럼 읽을 수 있습니다. 메일마다 작성한 내용만 말풍선으로 표시되며, 내 메일은 오른쪽에 표시됩니다. 뉴스레터는 평소대로 표시됩니다.
chat-view-switch = 대화를 채팅으로 표시
chat-view-switch-detail = 인용된 메일과 서명은 각 말풍선의 ··· 뒤에 숨겨집니다
chat-switch-chat = 채팅
chat-switch-mail = 메일
chat-people = { $names }, 나 · { $count ->
   *[other] 메일 { $count }개
}
chat-people-heading = { $count ->
   *[other] 이 채팅의 참여자 · { $count }명
}
chat-member-mails = { $count ->
    [0] 메일 없음
   *[other] 메일 { $count }개
}
chat-today = 오늘
chat-yesterday = 어제
chat-added = { $who }님이 { $names }님을 추가했습니다
chat-renamed = { $who }님이 제목을 “{ $subject }”(으)로 바꿨습니다
chat-you = 나
chat-not-downloaded = 아직 다운로드되지 않음
chat-forwarded = 전달된 메일
chat-show-quoted = 인용된 메일과 서명 표시
chat-hide-quoted = 인용된 메일과 서명 숨기기
chat-hide-dots = ··· 숨기기
chat-show-card = 연락처 카드 보기
chat-reply-all = 전체답장
chat-more = 더보기
chat-reply-only = { $name }님에게만 답장
chat-forward = 전달
chat-copy-text = 텍스트 복사
chat-show-as-mail = 메일로 보기
chat-pin = 상단에 고정
chat-pin-file = 파일을 상단에 고정
chat-unpin = 고정 해제
chat-unpin-file = 파일 고정 해제
chat-pinned-of = 고정된 항목 { $count }개 중 { $at }번째
chat-pins-all = 모든 고정 항목
chat-pins-heading = 고정됨 · { $most }개 중 { $count }개
chat-pins-drag = 드래그하여 순서 변경
chat-pin-from-mail = { $name }님의 메일 · { $when }
chat-pin-from-file = { $name }님의 파일 · { $when }
chat-pin-from-text = { $name }님의 텍스트 · { $when }
chat-pins-full = 이 채팅에는 이미 고정된 항목이 5개 있습니다
chat-pins-replace-title = 고정 항목 바꾸기
chat-pins-replace-hint = 채팅에는 최대 5개까지 고정할 수 있습니다. 고정을 해제할 항목을 선택하세요.
chat-pins-replace = 바꾸기
chat-pins-cancel = 취소
chat-undo = 실행취소
chat-reply-to = { $names }님에게 답장
chat-send = 보내기(Ctrl+Enter)
chat-attach = 첨부
chat-attach-photo = 사진
chat-attach-file = 파일
chat-attach-library = 파일에서
chat-attach-template = 템플릿
chat-attach-signature = 서명
chat-replying-to = { $name }님에게 답장하는 중
chat-reply-newest = 최신 메일에 답장

## The attach picker (paperclip > From Files)

picker-title = 파일에서 첨부
picker-search = 이름, 사람, 제목 검색
picker-search-drive = 이 드라이브 검색
picker-mail-files = 메일 파일
picker-this-chat = 이 대화
picker-this-computer = 이 컴퓨터…
picker-in-chat = 이 대화의 파일
picker-recent = 최근
picker-preview = 미리보기
picker-cancel = 취소
picker-attach = 첨부
picker-attach-count = { $count }개 첨부
picker-selected = { $count }개 선택됨
picker-of-limit = / { $limit }
picker-in-mail = 메일에 { $size } 포함
picker-drive-links = { $count ->
   *[other] { $count }개는 Google Drive 링크로
}
picker-onedrive-links = { $count ->
   *[other] { $count }개는 OneDrive 링크로
}
picker-over = { $size }, 메일 한 통에 담을 수 있는 { $limit }을(를) 초과합니다
picker-getting = { $count ->
   *[other] 드라이브에서 파일 { $count }개를 가져오는 중…
}
picker-some-failed = { $count ->
   *[other] 파일 { $count }개를 읽을 수 없습니다
}
