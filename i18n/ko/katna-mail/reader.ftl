# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = 닫기
reader-back = 뒤로
reader-mark-unread = 읽지 않음으로 표시
reader-move-to = 이동
reader-more = 더보기
reader-original-colors = 원래 색상으로 보기
reader-dark-colors = 어두운 색상으로 보기
reader-print-all = 모두 인쇄
reader-new-window = 새 창에서 열기
reader-position = { $position } / { $total }
reader-newer = 최신
reader-older = 이전

## Reading pane: the conversation

reader-removed = 이 대화는 삭제되었습니다.
reader-no-subject = (제목 없음)
reader-collapse-all = 모두 접기
reader-expand-all = 모두 펼치기
reader-unknown-sender = (알 수 없는 보낸사람)
reader-date-ago = { $date }({ $ago })
reader-me = 나
reader-to = 받는사람: { $names }
reader-to-label = 받는사람:
reader-tick-delivered = 전달됨 { $when }
reader-tick-no-bounce = 보냄 { $when }. 반송 메일이 돌아오지 않아 도착했을 가능성이 높습니다
reader-tick-bounced = 전달 실패: { $when } 반송됨
reader-tick-read = 읽음 { $when }(읽음 확인)
reader-tick-opened = 열람, 마지막: { $when }(열람 추적)
reader-starred = 별표 있음
reader-not-starred = 별표 없음
reader-too-long = 메일이 너무 길어 전체를 표시할 수 없습니다.
reader-encrypted-images = 암호화된 메일에서는 웹 이미지를 불러오지 않습니다.
reader-window-failed = 새 창을 열 수 없습니다.

## Reading pane: message details (opened from "to me")

reader-details-from = 보낸사람:
reader-details-to = 받는사람:
reader-details-cc = 참조:
reader-details-date = 날짜:
reader-details-subject = 제목:

## Reading pane: downloading a message

reader-downloading = 서버에서 이 메일을 다운로드하는 중…
reader-download-failed = 이 메일을 다운로드할 수 없습니다.
reader-try-again = 다시 시도

## Reply row

reply-reply = 답장
reply-reply-all = 전체답장
reply-forward = 전달

## Encrypted and signed mail

security-decrypting = 복호화하는 중…
security-checking = 서명을 확인하는 중…
security-partly-encrypted = 이 메일은 일부만 암호화되어 있습니다. 나머지는 보호 범위 밖에서 추가된 것으로, 누구든 보냈을 수 있습니다.
security-partly-signed = 이 메일은 일부만 서명되어 있습니다. 나머지는 보호 범위 밖에서 추가된 것으로, 누구든 보냈을 수 있습니다.
security-encrypted = 암호화된 메일
security-encrypted-smime = 암호화된 메일(S/MIME)
security-no-key = 이 메일을 복호화할 수 없습니다. 내게 없는 키로 암호화되었습니다.
security-cancelled = 복호화가 취소되었습니다.
security-damaged = 이 메일을 복호화할 수 없습니다. 암호화된 데이터가 손상되었거나 변경되었습니다.
security-decrypt-unavailable = 이 메일을 복호화할 수 없습니다. 암호화된 메일을 읽으려면 { $tool }을(를) 설치하세요.
security-decrypt-failed = 이 메일을 복호화할 수 없습니다: { $reason }
security-unknown-signer = 알 수 없는 서명자
security-signed-verified = { $signer } 서명 · 확인됨
security-signed-not-sender = { $signer } 서명, 보낸사람이 아님
security-signed-untrusted = { $signer } 서명, 신뢰하지 않음으로 표시한 키 사용
security-signed-unverified = { $signer } 서명 · 키가 확인되지 않음
security-bad-signature = 잘못된 서명: 서명 후 이 메일이 변경되었거나 서명이 위조되었습니다.
security-signature-expired = { $signer } 서명 · 서명이 만료됨
security-key-expired = { $signer } 서명 · 이후 키가 만료됨
security-key-revoked = { $signer } 서명, 폐기된 키 사용
security-missing-key = 내게 없는 키로 서명되어 확인할 수 없음
security-missing-key-id = 내게 없는 키({ $key })로 서명되어 확인할 수 없음
security-signature-unavailable = 서명됨. 서명을 확인하려면 { $tool }을(를) 설치하세요
security-signature-error = 서명을 확인할 수 없습니다.
tracking-opened = { $who }님이 { $count }번 열었습니다. 마지막: { $when }
tracking-opens-clicks = { $who }님이 { $opens }번 열고 링크를 { $clicks }번 클릭했습니다. 마지막: { $when }
tracking-clicked = { $who }님이 링크를 { $clicks }번 클릭했습니다. 마지막: { $when }
tracking-maybe-opened = { $who }님이 열었을 수 있습니다(Apple Mail은 개인정보 보호를 위해 이미지를 불러옵니다)
tracking-not-opened = { $who }님이 아직 열지 않았습니다
tracking-receipt = { $who }님이 읽음 확인을 보냈습니다
tracking-receipt-displayed = 읽음 확인: { $who }님이 메일을 열었습니다
tracking-receipt-other = 읽음 확인: { $who }님이 메일을 열지 않고 삭제하거나 처리했습니다

## Remote images and pictures

remote-hidden = 이 메일의 이미지가 숨겨져 있습니다.
remote-show = 이미지 표시
remote-always-show = 이 보낸사람의 이미지 항상 표시
remote-picture-use = 사용
remote-picture-too-big = 8 MB 이하의 사진을 선택하세요.
remote-picture-type = PNG, JPEG, GIF, WebP 또는 SVG 사진을 선택하세요.
remote-picture-read-failed = 사진을 읽을 수 없습니다: { $error }
remote-picture-keep-failed = 사진을 저장할 수 없습니다: { $error }
remote-picture-remove-failed = 사진을 삭제할 수 없습니다: { $error }

## Attachments

attachment-count = 첨부파일 { $count }개
attachment-save = 저장
attachment-save-all = 모두 저장
attachment-save-all-tooltip = 모든 첨부파일을 폴더에 저장
attachment-save-here = 여기에 저장
attachment-not-downloaded = 이 메일은 다운로드되지 않았습니다.
attachment-not-found = 메일에서 이 첨부파일을 찾을 수 없습니다.
attachment-read-failed = { $name }을(를) 읽을 수 없습니다
attachment-numbered = 첨부파일 { $number }
attachment-saved-all = 파일 { $count }개를 { $place }에 저장했습니다
attachment-saved-some = 파일 { $total }개 중 { $saved }개를 { $place }에 저장했습니다. { $failed }을(를) 저장할 수 없습니다
attachment-saved-to = { $path }에 저장했습니다
attachment-save-failed = { $name }을(를) 저장할 수 없습니다: { $error }
attachment-open-failed = { $name }을(를) 열 수 없습니다: { $error }
attachment-risky = 이 파일은 프로그램을 실행할 수 있어 Katna에서 열지 않습니다. 대신 저장하세요.
attachment-encrypted-open = 이 파일은 암호화된 상태로 받았습니다. 저장한 후 다른 곳에서 여세요.

## Printing

print-failed = 인쇄할 수 없습니다: { $error }
print-no-font = 글꼴을 찾을 수 없습니다
print-opened-as-pdf = PDF로 열었습니다. 열린 PDF에서 인쇄하세요.
print-preview-title = 인쇄 미리보기
print-preview-laying-out = 페이지를 배치하는 중…
print-preview-pages = { $count }페이지
print-preview-more = 외 { $count }페이지
print-preview-failed = 페이지를 표시할 수 없습니다
print-preview-paper = 용지
print-preview-a4 = A4
print-preview-letter = 레터
print-preview-layout = 레이아웃
print-preview-as-shown = 보이는 대로
print-preview-simple = 텍스트만
print-preview-backgrounds = 배경
print-preview-cancel = 취소
print-preview-print = 인쇄
print-not-downloaded = (아직 다운로드되지 않았습니다.)
print-encrypted = (암호화되어 있습니다. 본문을 인쇄하려면 Katna Mail에서 여세요.)
print-to = 받는사람: { $addresses }
print-cc = 참조: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = 첨부파일을 보려면 이 메일을 여세요.
text-copy = 복사
text-select-all = 모두 선택
