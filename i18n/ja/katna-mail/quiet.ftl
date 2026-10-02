# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The bell on the list's toolbar, for the open folder or inbox tab

quiet-tip-rings = 通知をミュート
quiet-tip-off = ミュート中。クリックすると通知を再開します
quiet-tip-muted-until = { $when } までミュート中。クリックすると通知を再開します

## The right-click menu of a folder or an account

quiet-mute = ミュート…
quiet-unmute = ミュートを解除
quiet-notify = 新着メールを通知する

## How long to mute

quiet-for-hour = 1 時間
quiet-until-tomorrow = 明日の朝まで
quiet-until-unmuted = 自分でオンに戻すまで
quiet-turn-on = 新着メールを通知する

## The note after a change, with Undo; { $name } is a folder, tab or account

quiet-off = { $name } をミュートしました
quiet-muted-until = { $name } を { $when } までミュートしました
quiet-on = { $name } の通知を再開しました

## Conversations and senders (the reading pane's bell, the More menus and
## the right-click menu)

quiet-mute-conversation = スレッドをミュート
quiet-unmute-conversation = スレッドのミュートを解除
quiet-mute-sender = 送信者をミュート
quiet-unmute-sender = 送信者のミュートを解除
quiet-row-muted = ミュート中
quiet-conversation-strip = ミュート中。新しい返信は通知されず、未読数にも含まれません。
quiet-conversation-muted = { $count ->
   *[other] { $count } 件のスレッドをミュートしました
}
quiet-conversation-unmuted = { $count ->
   *[other] { $count } 件のスレッドのミュートを解除しました
}
quiet-sender-strip = ミュート中。この人からのメールは通知されず、未読数にも含まれません。
quiet-sender-muted = { $address } からのメールをミュートしました
quiet-sender-unmuted = { $address } からのメールの通知を再開しました
