# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = 未傳送，原因：{ $reason }。
outbox-retrying = 尚未傳送，原因：{ $reason }。Katna 會自動再試一次。
outbox-waiting-sign-in = 正在等候你重新登入 { $address }，登入後就會寄出。
outbox-waiting-password = 正在等候 { $address } 的新密碼，取得後就會寄出。
outbox-waiting-connection = 正在等候網路連線，恢復連線後就會寄出。

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = 沒有收件者
outbox-reason-address = 某個收件地址不存在
outbox-reason-too-large = 郵件太大，郵件伺服器無法接受
outbox-reason-blocked = 郵件伺服器封鎖了這封郵件
outbox-reason-gone = 這部電腦上的副本已不存在
outbox-reason-refused = 郵件伺服器拒絕了這封郵件

## Buttons and notes

outbox-try-again = 再試一次
outbox-edit = 編輯
outbox-delete = 刪除
outbox-deleted = 已從寄件匣刪除
outbox-sending-again = 正在重新傳送…
outbox-snackbar-not-sent = 「{ $subject }」未能傳送，原因：{ $reason }。
outbox-open = 寄件匣
