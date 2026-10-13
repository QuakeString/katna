# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = 未发送，因为{ $reason }。
outbox-retrying = 尚未发送，因为{ $reason }。Katna 会自动重试。
outbox-waiting-sign-in = 正在等待你重新登录 { $address }。登录后即会发出。
outbox-waiting-password = 正在等待 { $address } 的新密码。届时即会发出。
outbox-waiting-connection = 正在等待网络连接。恢复联网后即会发出。

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = 没有收件人
outbox-reason-address = 某个收件地址不存在
outbox-reason-too-large = 邮件对邮件服务器来说太大
outbox-reason-blocked = 邮件服务器阻止了它
outbox-reason-gone = 此电脑上的副本已不存在
outbox-reason-refused = 邮件服务器拒绝了它

## Buttons and notes

outbox-try-again = 重试
outbox-edit = 编辑
outbox-delete = 删除
outbox-deleted = 已从发件箱删除
outbox-sending-again = 正在重新发送…
outbox-snackbar-not-sent = “{ $subject }”未发送，因为{ $reason }。
outbox-open = 发件箱
