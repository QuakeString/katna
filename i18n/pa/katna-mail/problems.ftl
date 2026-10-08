# Katna Mail, Punjabi (ਪੰਜਾਬੀ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = ਮੇਲ ਸਰਵਰ

problems-signed-out = { $provider } ਨੇ Katna ਨੂੰ { $address } ਤੋਂ ਸਾਈਨ ਆਊਟ ਕਰ ਦਿੱਤਾ। ਮੇਲ ਸਿੰਕ ਹੋਣੀ ਬੰਦ ਹੋ ਗਈ।
problems-password-refused = { $provider } ਨੇ { $address } ਦਾ ਪਾਸਵਰਡ ਨਾਮਨਜ਼ੂਰ ਕਰ ਦਿੱਤਾ। ਹੋ ਸਕਦਾ ਹੈ ਇਹ ਬਦਲ ਗਿਆ ਹੋਵੇ।
problems-no-answer = { $provider } { $address } ਲਈ ਜਵਾਬ ਨਹੀਂ ਦੇ ਰਿਹਾ। Katna ਕੋਸ਼ਿਸ਼ ਕਰਦਾ ਰਹਿੰਦਾ ਹੈ।
problems-offline = ਤੁਸੀਂ ਆਫ਼ਲਾਈਨ ਹੋ। ਤੁਹਾਡੀ ਮੇਲ ਅਜੇ ਵੀ ਇੱਥੇ ਹੈ, ਅਤੇ ਤੁਹਾਡੇ ਵੱਲੋਂ ਭੇਜੀ ਮੇਲ ਤੁਹਾਡੇ ਵਾਪਸ ਆਉਣ ਤੱਕ ਉਡੀਕ ਕਰਦੀ ਹੈ।
problems-accounts-need-you = { $count ->
    [one] 1 ਖਾਤੇ ਨੂੰ ਤੁਹਾਡੀ ਲੋੜ ਹੈ
   *[other] { $count } ਖਾਤਿਆਂ ਨੂੰ ਤੁਹਾਡੀ ਲੋੜ ਹੈ
}
problems-show = ਦਿਖਾਓ
problems-later = ਬਾਅਦ ਵਿੱਚ
problems-new-password = ਨਵਾਂ ਪਾਸਵਰਡ
problems-try-again = ਦੁਬਾਰਾ ਕੋਸ਼ਿਸ਼ ਕਰੋ

## The New password card

problems-password-title = ਨਵਾਂ ਪਾਸਵਰਡ
problems-password-detail = { $provider } ਨੇ { $address } ਦਾ ਸੰਭਾਲਿਆ ਪਾਸਵਰਡ ਨਾਮਨਜ਼ੂਰ ਕਰ ਦਿੱਤਾ। ਨਵਾਂ ਟਾਈਪ ਕਰੋ; Katna ਇਸਨੂੰ ਰੱਖਣ ਤੋਂ ਪਹਿਲਾਂ ਜਾਂਚਦਾ ਹੈ।
problems-password-placeholder = ਪਾਸਵਰਡ
problems-password-show = ਪਾਸਵਰਡ ਦਿਖਾਓ
problems-password-hide = ਪਾਸਵਰਡ ਲੁਕਾਓ
problems-password-cancel = ਰੱਦ ਕਰੋ
problems-password-save = ਸੰਭਾਲੋ
problems-password-checking = ਜਾਂਚ ਹੋ ਰਹੀ ਹੈ…
problems-password-refused-again = { $provider } ਨੇ ਇਹ ਪਾਸਵਰਡ ਵੀ ਨਾਮਨਜ਼ੂਰ ਕਰ ਦਿੱਤਾ। ਇਸਨੂੰ ਜਾਂਚੋ ਅਤੇ ਦੁਬਾਰਾ ਕੋਸ਼ਿਸ਼ ਕਰੋ।
problems-password-saved = { $address } ਲਈ ਪਾਸਵਰਡ ਸੰਭਾਲਿਆ ਗਿਆ। ਤੁਹਾਡੀ ਮੇਲ ਲਿਆ ਰਿਹਾ ਹੈ…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $count ->
    [one] { $address } ਦੇ ਮੇਲ ਸਰਵਰ ਨੇ ਇੱਕ ਸੁਨੇਹੇ ਨੂੰ ਹਿਲਾਉਣਾ ਸਵੀਕਾਰ ਨਹੀਂ ਕੀਤਾ, ਇਸ ਲਈ ਉਹ ਜਿੱਥੇ ਸੀ ਉੱਥੇ ਵਾਪਸ ਹੈ।
   *[other] { $address } ਦੇ ਮੇਲ ਸਰਵਰ ਨੇ { $count } ਸੁਨੇਹਿਆਂ ਨੂੰ ਹਿਲਾਉਣਾ ਸਵੀਕਾਰ ਨਹੀਂ ਕੀਤਾ, ਇਸ ਲਈ ਉਹ ਜਿੱਥੇ ਸਨ ਉੱਥੇ ਵਾਪਸ ਹਨ।
}
problems-refused-flags = { $count ->
    [one] { $address } ਦੇ ਮੇਲ ਸਰਵਰ ਨੇ ਇੱਕ ਸੁਨੇਹੇ ਨੂੰ ਨਿਸ਼ਾਨਦੇਹ ਕਰਨਾ (ਪੜ੍ਹਿਆ, ਤਾਰਾਬੱਧ…) ਸਵੀਕਾਰ ਨਹੀਂ ਕੀਤਾ, ਇਸ ਲਈ ਉਹ ਪਹਿਲਾਂ ਵਾਂਗ ਹੈ।
   *[other] { $address } ਦੇ ਮੇਲ ਸਰਵਰ ਨੇ { $count } ਸੁਨੇਹਿਆਂ ਨੂੰ ਨਿਸ਼ਾਨਦੇਹ ਕਰਨਾ (ਪੜ੍ਹਿਆ, ਤਾਰਾਬੱਧ…) ਸਵੀਕਾਰ ਨਹੀਂ ਕੀਤਾ, ਇਸ ਲਈ ਉਹ ਪਹਿਲਾਂ ਵਾਂਗ ਹਨ।
}
problems-refused-label = { $count ->
    [one] { $address } ਦੇ ਮੇਲ ਸਰਵਰ ਨੇ ਇੱਕ ਸੁਨੇਹੇ ਦੇ ਲੇਬਲ ਬਦਲਣਾ ਸਵੀਕਾਰ ਨਹੀਂ ਕੀਤਾ, ਇਸ ਲਈ ਉਹ ਪਹਿਲਾਂ ਵਾਂਗ ਹੈ।
   *[other] { $address } ਦੇ ਮੇਲ ਸਰਵਰ ਨੇ { $count } ਸੁਨੇਹਿਆਂ ਦੇ ਲੇਬਲ ਬਦਲਣਾ ਸਵੀਕਾਰ ਨਹੀਂ ਕੀਤਾ, ਇਸ ਲਈ ਉਹ ਪਹਿਲਾਂ ਵਾਂਗ ਹਨ।
}
problems-refused-delete = { $count ->
    [one] { $address } ਦੇ ਮੇਲ ਸਰਵਰ ਨੇ ਇੱਕ ਸੁਨੇਹਾ ਮਿਟਾਉਣਾ ਸਵੀਕਾਰ ਨਹੀਂ ਕੀਤਾ, ਇਸ ਲਈ ਉਹ ਵਾਪਸ ਹੈ।
   *[other] { $address } ਦੇ ਮੇਲ ਸਰਵਰ ਨੇ { $count } ਸੁਨੇਹੇ ਮਿਟਾਉਣਾ ਸਵੀਕਾਰ ਨਹੀਂ ਕੀਤਾ, ਇਸ ਲਈ ਉਹ ਵਾਪਸ ਹਨ।
}
problems-refused-other = { $count ->
    [one] { $address } ਦੇ ਮੇਲ ਸਰਵਰ ਨੇ ਇੱਕ ਬਦਲਾਅ ਸਵੀਕਾਰ ਨਹੀਂ ਕੀਤਾ, ਇਸ ਲਈ Katna ਨੇ ਉਸਨੂੰ ਪਹਿਲਾਂ ਵਾਂਗ ਕਰ ਦਿੱਤਾ।
   *[other] { $address } ਦੇ ਮੇਲ ਸਰਵਰ ਨੇ { $count } ਬਦਲਾਅ ਸਵੀਕਾਰ ਨਹੀਂ ਕੀਤੇ, ਇਸ ਲਈ Katna ਨੇ ਉਨ੍ਹਾਂ ਨੂੰ ਪਹਿਲਾਂ ਵਾਂਗ ਕਰ ਦਿੱਤਾ।
}
problems-details = ਵੇਰਵੇ

## Katna's background service (katna-daemon) isn't running

service-starting = Katna ਦੀ ਬੈਕਗ੍ਰਾਊਂਡ ਸੇਵਾ ਸ਼ੁਰੂ ਹੋ ਰਹੀ ਹੈ…
service-failed = Katna ਦੀ ਬੈਕਗ੍ਰਾਊਂਡ ਸੇਵਾ ਸ਼ੁਰੂ ਨਹੀਂ ਹੋ ਰਹੀ, ਇਸ ਲਈ ਮੇਲ ਸਿੰਕ ਨਹੀਂ ਹੋ ਰਹੀ।
service-start-again = ਦੁਬਾਰਾ ਸ਼ੁਰੂ ਕਰੋ
service-started-again = Katna ਦੀ ਬੈਕਗ੍ਰਾਊਂਡ ਸੇਵਾ ਰੁਕ ਗਈ ਸੀ ਅਤੇ ਦੁਬਾਰਾ ਸ਼ੁਰੂ ਕੀਤੀ ਗਈ।
service-details-title = ਸੇਵਾ ਸ਼ੁਰੂ ਕਿਉਂ ਨਹੀਂ ਹੋ ਰਹੀ
service-details-body = ਇਸਨੂੰ ਕਾਪੀ ਕਰੋ ਅਤੇ ਆਪਣੀ ਰਿਪੋਰਟ ਨਾਲ ਭੇਜੋ। ਇਸ ਵਿੱਚ ਕੋਈ ਮੇਲ ਜਾਂ ਪਾਸਵਰਡ ਨਹੀਂ ਹੈ।
service-details-copy = ਕਾਪੀ ਕਰੋ
service-details-close = ਬੰਦ ਕਰੋ
service-not-running = Katna ਦੀ ਬੈਕਗ੍ਰਾਊਂਡ ਸੇਵਾ ਨਹੀਂ ਚੱਲ ਰਹੀ।
service-no-answer = Katna ਦੀ ਬੈਕਗ੍ਰਾਊਂਡ ਸੇਵਾ ਨੇ ਜਵਾਬ ਨਹੀਂ ਦਿੱਤਾ: { $error }
service-no-session = ਕੋਈ D-Bus ਸੈਸ਼ਨ ਨਹੀਂ: { $error }
