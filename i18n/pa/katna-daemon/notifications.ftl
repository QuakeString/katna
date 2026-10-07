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
notify-follow-up-sent = ਫ਼ਾਲੋ-ਅੱਪ ਭੇਜਿਆ ਗਿਆ
notify-follow-up-sent-to = “{ $subject }” ਦਾ ਕਿਸੇ ਨੇ ਜਵਾਬ ਨਹੀਂ ਦਿੱਤਾ ਸੀ, ਇਸ ਲਈ Katna ਨੇ ਫ਼ਾਲੋ-ਅੱਪ ਭੇਜਿਆ।
notify-follow-up-waiting = ਫ਼ਾਲੋ-ਅੱਪ ਨਹੀਂ ਭੇਜਿਆ ਗਿਆ
notify-follow-up-waiting-to = ਇਸਦਾ ਸਮਾਂ ਉਦੋਂ ਸੀ ਜਦੋਂ ਇਹ ਕੰਪਿਊਟਰ ਬੰਦ ਸੀ। “{ $subject }” ਤੁਹਾਡੇ ਇਨਬਾਕਸ ਵਿੱਚ ਵਾਪਸ ਹੈ।
notify-tracking-opened = { $who } ਨੇ { $subject } ਖੋਲ੍ਹਿਆ
notify-tracking-clicked = { $who } ਨੇ { $subject } ਵਿੱਚ ਇੱਕ ਲਿੰਕ ’ਤੇ ਕਲਿੱਕ ਕੀਤਾ
notify-update-ready = Katna Mail ਨੂੰ ਅੱਪਡੇਟ ਕੀਤਾ ਜਾ ਸਕਦਾ ਹੈ
notify-update-ready-body = ਵਰਜਨ { $version } ਡਾਊਨਲੋਡ ਹੋ ਗਿਆ ਹੈ। ਅੱਪਡੇਟ ਇਸਨੂੰ ਸਥਾਪਤ ਕਰਦਾ ਹੈ ਅਤੇ Katna Mail ਨੂੰ ਰੀਸਟਾਰਟ ਕਰਦਾ ਹੈ।
notify-update = ਅੱਪਡੇਟ

## Something needs the user, shown once per problem

notify-signed-out = ਦੁਬਾਰਾ ਸਾਈਨ ਇਨ ਕਰੋ
notify-signed-out-body = { $provider } ਨੇ Katna ਨੂੰ { $address } ਤੋਂ ਸਾਈਨ ਆਊਟ ਕਰ ਦਿੱਤਾ। ਮੇਲ ਸਿੰਕ ਹੋਣੀ ਬੰਦ ਹੋ ਗਈ।
notify-sign-in = ਸਾਈਨ ਇਨ ਕਰੋ
notify-password-refused = ਪਾਸਵਰਡ ਅਸਵੀਕਾਰ ਹੋਇਆ
notify-password-refused-body = ਮੇਲ ਸਰਵਰ ਨੇ { $address } ਦਾ ਪਾਸਵਰਡ ਅਸਵੀਕਾਰ ਕਰ ਦਿੱਤਾ। ਸ਼ਾਇਦ ਇਹ ਬਦਲ ਗਿਆ ਹੈ।
notify-new-password = ਨਵਾਂ ਪਾਸਵਰਡ
notify-not-sent = “{ $subject }” ਨਹੀਂ ਭੇਜਿਆ ਗਿਆ
notify-not-sent-no-subject = ਇੱਕ ਸੁਨੇਹਾ ਨਹੀਂ ਭੇਜਿਆ ਗਿਆ
notify-not-sent-body = ਇਹ ਆਊਟਬਾਕਸ ਵਿੱਚ ਹੈ, ਜਿੱਥੇ ਕਾਰਨ ਦੱਸਿਆ ਗਿਆ ਹੈ।
notify-open-outbox = ਆਊਟਬਾਕਸ ਖੋਲ੍ਹੋ
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
notify-snooze-hour = 1 ਘੰਟਾ ਸਨੂਜ਼ ਕਰੋ
notify-snooze-tomorrow = ਕੱਲ੍ਹ
notify-copy-code = { $code } ਕਾਪੀ ਕਰੋ
notify-link-verify = { $domain } ’ਤੇ ਪੁਸ਼ਟੀ ਕਰੋ
notify-link-confirm = { $domain } ’ਤੇ ਤਸਦੀਕ ਕਰੋ
notify-link-activate = { $domain } ’ਤੇ ਚਾਲੂ ਕਰੋ

## After Archive on a notification: a short note in the same place

notify-archived = ਪੁਰਾਲੇਖਬੱਧ ਕੀਤਾ
notify-archived-count = { $count ->
    [one] { $count } ਸੁਨੇਹਾ ਇਨਬਾਕਸ ਵਿੱਚੋਂ ਹਟਾਇਆ ਗਿਆ
   *[other] { $count } ਸੁਨੇਹੇ ਇਨਬਾਕਸ ਵਿੱਚੋਂ ਹਟਾਏ ਗਏ
}
notify-undo = ਵਾਪਸ ਲਓ

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = ਕੋਡ ਕਾਪੀ ਹੋ ਗਿਆ
notify-code-not-copied = ਕੋਡ ਕਾਪੀ ਨਹੀਂ ਹੋ ਸਕਿਆ

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = { $name } ਨੂੰ ਜਵਾਬ ਭੇਜਿਆ ਗਿਆ
notify-open-in-katna = Katna ਵਿੱਚ ਖੋਲ੍ਹੋ
