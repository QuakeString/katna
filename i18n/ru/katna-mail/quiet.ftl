# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The bell on the list's toolbar, for the open folder or inbox tab

quiet-tip-rings = Отключить уведомления
quiet-tip-off = Без звука. Нажмите, чтобы снова уведомлять
quiet-tip-muted-until = Без звука до { $when }. Нажмите, чтобы снова уведомлять

## The right-click menu of a folder or an account

quiet-mute = Отключить уведомления…
quiet-unmute = Включить уведомления
quiet-notify = Уведомлять о новых письмах

## How long to mute

quiet-for-hour = На 1 час
quiet-until-tomorrow = До завтрашнего утра
quiet-until-unmuted = Пока я не включу снова
quiet-turn-on = Уведомлять о новых письмах

## The note after a change, with Undo; { $name } is a folder, tab or account

quiet-off = { $name }: уведомления отключены
quiet-muted-until = { $name }: уведомления отключены до { $when }
quiet-on = { $name }: уведомления снова включены

## Conversations and senders (the reading pane's bell, the More menus and

## the right-click menu)

quiet-mute-conversation = Отключить уведомления цепочки
quiet-unmute-conversation = Включить уведомления цепочки
quiet-mute-sender = Отключить уведомления от отправителя
quiet-unmute-sender = Включить уведомления от отправителя
quiet-row-muted = Без звука
quiet-conversation-strip = Без звука. Новые ответы не уведомляют и не учитываются в счётчике.
quiet-conversation-muted = { $count ->
    [one] Уведомления { $count } цепочки отключены
    [few] Уведомления { $count } цепочек отключены
    [many] Уведомления { $count } цепочек отключены
   *[other] Уведомления { $count } цепочки отключены
}
quiet-conversation-unmuted = { $count ->
    [one] Уведомления { $count } цепочки включены
    [few] Уведомления { $count } цепочек включены
    [many] Уведомления { $count } цепочек включены
   *[other] Уведомления { $count } цепочки включены
}
quiet-sender-strip = Без звука. Их письма не уведомляют и не учитываются в счётчике.
quiet-sender-muted = Уведомления о письмах от { $address } отключены
quiet-sender-unmuted = Уведомления о письмах от { $address } снова включены
