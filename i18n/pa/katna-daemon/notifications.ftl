# Katna Mail, Punjabi (ਪੰਜਾਬੀ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } ਨਵੀਂ ਈਮੇਲ
   *[other] { $count } ਨਵੀਆਂ ਈਮੇਲਾਂ
}
notify-and-more = ਅਤੇ { $count } ਹੋਰ
notify-no-subject = (ਕੋਈ ਵਿਸ਼ਾ ਨਹੀਂ)
notify-unknown-sender = ਅਗਿਆਤ ਭੇਜਣ ਵਾਲਾ
notify-snooze-back = ਸਨੂਜ਼ ਤੋਂ ਵਾਪਸ
notify-no-reply = ਅਜੇ ਕੋਈ ਜਵਾਬ ਨਹੀਂ
notify-no-reply-to = “{ $subject }” ਦਾ ਕਿਸੇ ਨੇ ਜਵਾਬ ਨਹੀਂ ਦਿੱਤਾ।
notify-tracking-opened = { $who } ਨੇ { $subject } ਖੋਲ੍ਹਿਆ
notify-tracking-clicked = { $who } ਨੇ { $subject } ਵਿੱਚ ਇੱਕ ਲਿੰਕ ’ਤੇ ਕਲਿੱਕ ਕੀਤਾ
notify-update-ready = Katna Mail ਨੂੰ ਅੱਪਡੇਟ ਕੀਤਾ ਜਾ ਸਕਦਾ ਹੈ
notify-update-ready-body = ਵਰਜਨ { $version } ਡਾਊਨਲੋਡ ਹੋ ਗਿਆ ਹੈ। ਅੱਪਡੇਟ ਇਸਨੂੰ ਸਥਾਪਤ ਕਰਦਾ ਹੈ ਅਤੇ Katna Mail ਨੂੰ ਰੀਸਟਾਰਟ ਕਰਦਾ ਹੈ।
notify-update = ਅੱਪਡੇਟ
notify-event-now = ਹੁਣੇ
notify-event-in-minutes = { $count ->
    [one] { $count } ਮਿੰਟ ਵਿੱਚ
   *[other] { $count } ਮਿੰਟ ਵਿੱਚ
}
notify-event-in-hours = { $count ->
    [one] { $count } ਘੰਟੇ ਵਿੱਚ
   *[other] { $count } ਘੰਟੇ ਵਿੱਚ
}
notify-event-in-days = { $count ->
    [1] ਕੱਲ੍ਹ
    [one] { $count } ਦਿਨਾਂ ਵਿੱਚ
   *[other] { $count } ਦਿਨਾਂ ਵਿੱਚ
}
notify-event-all-day = ਸਾਰਾ ਦਿਨ
notify-event-join = ਸ਼ਾਮਲ ਹੋਵੋ
notify-event-snooze = 5 ਮਿੰਟ ਸਨੂਜ਼ ਕਰੋ
notify-task-done = ਹੋ ਗਿਆ ਵਜੋਂ ਚਿੰਨ੍ਹਿਤ ਕਰੋ

## Its buttons

notify-open = ਖੋਲ੍ਹੋ
notify-peek = ਝਾਤ ਮਾਰੋ
notify-reply = ਜਵਾਬ ਦਿਓ
notify-reply-placeholder = { $name } ਨੂੰ ਜਵਾਬ ਦਿਓ…
notify-send = ਭੇਜੋ
notify-reply-all = ਸਭ ਨੂੰ ਜਵਾਬ ਦਿਓ
notify-mark-read = ਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
notify-mark-all-read = ਸਭ ਨੂੰ ਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
notify-archive = ਪੁਰਾਲੇਖਬੱਧ ਕਰੋ

## After Archive on a notification: a short note in the same place

notify-archived = ਪੁਰਾਲੇਖਬੱਧ ਕੀਤਾ
notify-archived-count = { $count ->
    [one] { $count } ਸੁਨੇਹਾ ਇਨਬਾਕਸ ਵਿੱਚੋਂ ਹਟਾਇਆ ਗਿਆ
   *[other] { $count } ਸੁਨੇਹੇ ਇਨਬਾਕਸ ਵਿੱਚੋਂ ਹਟਾਏ ਗਏ
}
notify-undo = ਵਾਪਸ ਲਓ

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = { $name } ਨੂੰ ਜਵਾਬ ਭੇਜਿਆ ਗਿਆ
notify-open-in-katna = Katna ਵਿੱਚ ਖੋਲ੍ਹੋ
