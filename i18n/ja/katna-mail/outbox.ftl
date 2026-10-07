# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = { $reason }ため、送信されませんでした。
outbox-retrying = { $reason }ため、まだ送信されていません。Katna が自動的に再試行します。
outbox-waiting-sign-in = { $address } への再サインインを待っています。サインインすると送信されます。
outbox-waiting-password = { $address } の新しいパスワードを待っています。入力すると送信されます。
outbox-waiting-connection = 接続を待っています。オンラインに戻ると送信されます。

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = 宛先がない
outbox-reason-address = 宛先のアドレスが存在しない
outbox-reason-too-large = メールサーバーにとってサイズが大きすぎる
outbox-reason-blocked = メールサーバーがブロックした
outbox-reason-gone = このパソコン上のコピーがなくなった
outbox-reason-refused = メールサーバーが拒否した

## Buttons and notes

outbox-try-again = 再試行
outbox-edit = 編集
outbox-delete = 削除
outbox-deleted = 送信トレイから削除しました
outbox-sending-again = もう一度送信しています…
outbox-snackbar-not-sent = { $reason }ため、「{ $subject }」は送信されませんでした。
outbox-open = 送信トレイ
