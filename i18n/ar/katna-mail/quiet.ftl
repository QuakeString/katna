# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The bell on the list's toolbar, for the open folder or inbox tab

quiet-tip-rings = كتم الإشعارات
quiet-tip-off = مكتوم. انقر لاستئناف الإشعارات
quiet-tip-muted-until = مكتوم حتى { $when }. انقر لاستئناف الإشعارات

## The right-click menu of a folder or an account

quiet-mute = كتم…
quiet-unmute = إلغاء الكتم
quiet-notify = الإشعار بالبريد الجديد

## How long to mute

quiet-for-hour = لمدة ساعة
quiet-until-tomorrow = حتى صباح الغد
quiet-until-unmuted = حتى أعيد تفعيله
quiet-turn-on = الإشعار بالبريد الجديد

## The note after a change, with Undo; { $name } is a folder, tab or account

quiet-off = تم كتم { $name }
quiet-muted-until = تم كتم { $name } حتى { $when }
quiet-on = عادت إشعارات { $name }

## Conversations and senders (the reading pane's bell, the More menus and

## the right-click menu)

quiet-mute-conversation = كتم المحادثة
quiet-unmute-conversation = إلغاء كتم المحادثة
quiet-mute-sender = كتم المُرسِل
quiet-unmute-sender = إلغاء كتم المُرسِل
quiet-row-muted = مكتومة
quiet-conversation-strip = مكتومة. لن تُصدر الردود الجديدة إشعارات ولن تُحتسب.
quiet-conversation-muted = { $count ->
    [zero] تم كتم { $count } محادثة
    [one] تم كتم المحادثة
    [two] تم كتم محادثتين
    [few] تم كتم { $count } محادثات
    [many] تم كتم { $count } محادثة
   *[other] تم كتم { $count } محادثة
}
quiet-conversation-unmuted = { $count ->
    [zero] تم إلغاء كتم { $count } محادثة
    [one] تم إلغاء كتم المحادثة
    [two] تم إلغاء كتم محادثتين
    [few] تم إلغاء كتم { $count } محادثات
    [many] تم إلغاء كتم { $count } محادثة
   *[other] تم إلغاء كتم { $count } محادثة
}
quiet-sender-strip = مكتوم. لن يُصدر بريده إشعارات ولن يُحتسب.
quiet-sender-muted = تم كتم البريد من { $address }
quiet-sender-unmuted = عادت إشعارات البريد من { $address }
