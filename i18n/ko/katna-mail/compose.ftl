# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = 새 메일
compose-restore = 원래 크기로
compose-minimize = 최소화
compose-exit-full-screen = 전체화면 종료
compose-open-window = 새 창에서 열기
compose-save-close = 저장 후 닫기
compose-back-to-mail = 메일 창으로 돌아가기
compose-pop-out-reply = 답장을 새 창에서 열기
compose-edit-recipients = 받는사람 편집
compose-summary-cc = 참조: { $names }
compose-summary-bcc = 숨은참조: { $names }
compose-more-recipients = 외 { $count }명
compose-show-trimmed = 생략된 내용 표시
compose-hide-trimmed = 생략된 내용 숨기기
compose-remove-trimmed = 인용된 텍스트 삭제
compose-trimmed-removed = 인용된 텍스트를 삭제했습니다

## The quoted or forwarded message, in the mail itself

compose-quote-header = { $date }, { $from }님이 작성:
compose-forward-header = ---------- 전달된 메일 ---------
compose-forward-from = 보낸사람: { $from }
compose-forward-date = 날짜: { $date }
compose-forward-subject = 제목: { $subject }
compose-forward-to = 받는사람: { $to }
compose-forward-cc = 참조: { $cc }

## Recipients and subject

compose-to = 받는사람
compose-cc = 참조
compose-bcc = 숨은참조
compose-from = 보낸사람
compose-from-choose = 다른 계정에서 보내기
compose-recipients = 받는사람
compose-subject = 제목

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = 열려 있는 메일을 먼저 보내거나 삭제하세요.
compose-bad-address = “{ $address }”은(는) 이메일 주소가 아닙니다.
compose-no-recipients = 받는사람을 한 명 이상 추가하세요.
compose-attachments-too-large = 첨부파일이 { $size }입니다. 메일 서버는 최대 { $limit }까지 받습니다.
compose-no-account = 메일을 보낼 계정을 추가하세요.
compose-past-time = 미래의 시간을 선택하세요.
compose-scheduling = 예약하는 중…
compose-sending = 보내는 중…
compose-scheduled = { $when }에 전송 예약됨
compose-sent-archived = 보내고 보관처리함
compose-sent = 메일을 보냈습니다
compose-discarded = 임시보관 메일을 삭제했습니다
compose-draft-saved = 임시보관함에 저장했습니다
compose-draft-saving = 저장하는 중…
compose-draft-failed = 임시보관 메일을 저장하지 못했습니다: { $error }
compose-draft-not-opened = 임시보관 메일을 열지 못했습니다.

## Attachments

compose-picker-insert = 삽입
compose-picker-attach = 첨부
compose-file-too-large = { $name } 파일이 너무 큽니다. 메일 한 통에는 최대 { $limit }까지 담을 수 있습니다.
compose-forward-files-missing = 전달할 메일의 파일이 다운로드되지 않아 첨부되지 않았습니다.
compose-attachment-size = ({ $size })
compose-remove-attachment = 첨부파일 삭제
compose-attachment-open-tip = 열어서 확인
compose-attachments-total = 파일 { $count }개, { $size }
compose-drive-note = { $name } 파일이 { $limit } 제한을 넘어 Google Drive에 저장되며, 메일에는 링크가 포함됩니다.
compose-drive-tip = Google Drive에 있습니다. 메일에는 링크가 포함됩니다
compose-drive-uploading = 업로드 중 { $percent }%
compose-drive-allow = Drive 허용
compose-drive-allow-tip = Google로 다시 로그인하면 Katna가 큰 파일을 Drive에 넣을 수 있습니다
compose-drive-retry = 다시 시도
compose-drive-sends-when-uploaded = { $name } 업로드가 끝나면 보냅니다
compose-drive-not-uploaded = { $name } 파일이 아직 Google Drive에 없습니다
compose-drive-share-failed = Google Drive에서 파일을 공유하지 못했습니다: { $error }
compose-drive-share-title = 모든 사람과 파일을 공유할까요?
compose-drive-share-text = { $count ->
   *[other] Google Drive에서는 Google 계정이 없는 받는사람({ $addresses })과 파일을 공유할 수 없습니다. 대신 링크가 있는 모든 사용자가 파일을 열 수 있습니다.
}
compose-drive-share-link = 링크로 공유
compose-drive-send-without = 공유하지 않고 보내기
compose-drive-share-cancel = 취소
compose-drive-card-detail = { $size } · Google Drive
compose-drive-card-name = Google Drive
compose-onedrive-note = { $name } 파일이 { $limit } 제한을 넘어 OneDrive에 저장되며, 메일에는 링크가 포함됩니다.
compose-onedrive-tip = OneDrive에 있습니다. 메일에는 링크가 포함됩니다
compose-onedrive-allow = OneDrive 허용
compose-onedrive-allow-tip = Microsoft로 다시 로그인하면 Katna가 큰 파일을 OneDrive에 넣을 수 있습니다
compose-onedrive-not-uploaded = { $name } 파일이 아직 OneDrive에 없습니다
compose-onedrive-share-failed = OneDrive에서 파일을 공유하지 못했습니다: { $error }
compose-onedrive-share-text = { $count ->
   *[other] OneDrive에서는 받는사람({ $addresses })과 파일을 공유할 수 없습니다. 대신 링크가 있는 모든 사용자가 파일을 열 수 있습니다.
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-onedrive-card-name = OneDrive
compose-drop-files = 여기에 파일을 놓으세요
compose-drop-here = 여기에 놓으세요

## Paste options (a small bar under what was just pasted or dropped)

compose-paste-keep-formatting = 서식 유지
compose-paste-table = 표
compose-paste-picture = 그림
compose-paste-plain-text = 일반 텍스트
compose-paste-inline = 본문에 넣기
compose-paste-attachment = 첨부파일

## Encryption and signing (the toggles by the recipients)

compose-encrypt = 암호화
compose-encrypted = 암호화됨: 받는사람만 읽을 수 있습니다
compose-sign = 서명
compose-signed = 서명됨: 받는사람이 보낸 사람이 나인지 확인할 수 있습니다

## Open and click tracking and read receipts (toggles after Sign)

compose-track = 열람 및 클릭 추적
compose-tracked = 추적 중: 받는사람이 각각 언제 메일을 열거나 링크를 클릭했는지 볼 수 있습니다
compose-track-clicks = 링크 클릭 추적(일반 텍스트는 열람을 표시할 수 없음)
compose-tracked-clicks = 추적 중: 받는사람이 각각 언제 링크를 클릭했는지 볼 수 있습니다
compose-track-sign-in = 열람 및 클릭을 추적하려면 Katna 계정에 로그인하세요
compose-receipt = 읽음 확인 요청
compose-receipt-on = 읽음 확인 요청됨: 받는사람의 앱에서 읽음 확인을 보낼지 물어볼 수 있습니다
compose-delivery = 배달 확인 요청
compose-delivery-on = 배달 확인 요청됨: 각 받는사람의 서버가 메일을 받으면 메일 서버가 이메일로 알려 줍니다
compose-delivery-unavailable = 메일 서버가 배달 확인을 보내지 않습니다

## Spelling

spell-no-dictionary = { $language }용 맞춤법 사전이 설치되어 있지 않습니다(예: hunspell-en_us).
spell-dictionary-error = 맞춤법 사전: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = “{ $words }” 추가
grammar-remove = “{ $words }” 삭제
grammar-ignore = 무시

## Send checks (asked before a message goes out)

send-check-attachment-title = 파일을 첨부하려고 하셨나요?
send-check-attachment-text = 본문에 첨부파일이 언급되어 있지만 첨부된 파일이 없습니다.
send-check-attach = 파일 첨부
send-check-subject-title = 제목 없이 보낼까요?
send-check-subject-text = 이 메일에는 제목이 없습니다.
send-check-add-subject = 제목 추가
send-check-send-anyway = 그래도 보내기

## Recipients (To, Cc and Bcc)

recipient-not-valid = 올바른 이메일 주소가 아닙니다
recipient-show-address = 주소 보기
recipient-remove = 삭제
recipient-bad-title = 주소를 확인하세요
recipient-bad-text = “{ $address }”은(는) 올바른 이메일 주소가 아닙니다. 보내기 전에 수정하거나 삭제하세요.
recipient-bad-fix = 수정
