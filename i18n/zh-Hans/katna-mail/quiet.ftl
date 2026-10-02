# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The bell on the list's toolbar, for the open folder or inbox tab

quiet-tip-rings = 静音通知
quiet-tip-off = 已静音。点击可恢复通知
quiet-tip-muted-until = 静音至 { $when }。点击可恢复通知

## The right-click menu of a folder or an account

quiet-mute = 静音…
quiet-unmute = 取消静音
quiet-notify = 收到新邮件时通知

## How long to mute

quiet-for-hour = 1 小时
quiet-until-tomorrow = 直到明天早上
quiet-until-unmuted = 直到我重新开启
quiet-turn-on = 收到新邮件时通知

## The note after a change, with Undo; { $name } is a folder, tab or account

quiet-off = 已将“{ $name }”静音
quiet-muted-until = “{ $name }”静音至 { $when }
quiet-on = “{ $name }”已恢复通知

## Conversations and senders (the reading pane's bell, the More menus and
## the right-click menu)

quiet-mute-conversation = 将会话静音
quiet-unmute-conversation = 取消会话静音
quiet-mute-sender = 将发件人静音
quiet-unmute-sender = 取消发件人静音
quiet-row-muted = 已静音
quiet-conversation-strip = 已静音。新回复不会通知，也不计入未读数。
quiet-conversation-muted = { $count ->
   *[other] 已将 { $count } 个会话静音
}
quiet-conversation-unmuted = { $count ->
   *[other] 已取消 { $count } 个会话的静音
}
quiet-sender-strip = 已静音。此人的邮件不会通知，也不计入未读数。
quiet-sender-muted = 已将来自 { $address } 的邮件静音
quiet-sender-unmuted = 来自 { $address } 的邮件已恢复通知
