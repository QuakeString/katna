# Katna Mail, Punjabi (ਪੰਜਾਬੀ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = ਮੇਲ ਖਾਤਾ ਸ਼ਾਮਲ ਕਰੋ
add-account-looking = { $address } ਦੇ ਮੇਲ ਸਰਵਰ ਲੱਭੇ ਜਾ ਰਹੇ ਹਨ…
add-account-address-intro = ਆਪਣਾ ਈਮੇਲ ਪਤਾ ਦਰਜ ਕਰੋ। Katna ਤੁਹਾਡੇ ਲਈ ਸਰਵਰ ਲੱਭ ਲੈਂਦਾ ਹੈ।
add-account-servers-title = ਸਰਵਰ ਸੈਟਿੰਗਾਂ
add-account-servers-intro = { $address } ਲਈ Katna ਕਿੱਥੋਂ ਮੇਲ ਪੜ੍ਹਦਾ ਅਤੇ ਭੇਜਦਾ ਹੈ।
add-account-password-title = ਆਪਣਾ ਪਾਸਵਰਡ ਦਰਜ ਕਰੋ
add-account-signing-in = ਸਾਈਨ ਇਨ ਹੋ ਰਿਹਾ ਹੈ…
add-account-browser-title = ਆਪਣੇ ਬ੍ਰਾਊਜ਼ਰ ਵਿੱਚ ਜਾਰੀ ਰੱਖੋ
add-account-browser-intro = Katna ਨੇ ਤੁਹਾਡੇ ਬ੍ਰਾਊਜ਼ਰ ਵਿੱਚ { $provider } ਦਾ ਸਾਈਨ-ਇਨ ਪੰਨਾ ਖੋਲ੍ਹਿਆ ਹੈ। ਉੱਥੇ ਸਾਈਨ ਇਨ ਕਰੋ ਅਤੇ Katna ਨੂੰ ਤੁਹਾਡੀ ਮੇਲ ਪੜ੍ਹਨ ਅਤੇ ਭੇਜਣ ਦੀ ਇਜਾਜ਼ਤ ਦਿਓ, ਫਿਰ ਇੱਥੇ ਵਾਪਸ ਆਓ।
add-account-browser-hint = ਕੋਈ ਪੰਨਾ ਨਹੀਂ ਖੁੱਲ੍ਹਿਆ? ਆਪਣੇ ਬ੍ਰਾਊਜ਼ਰ ਦੀਆਂ ਵਿੰਡੋਆਂ ਦੇਖੋ, ਜਾਂ ਪਿੱਛੇ ਜਾ ਕੇ ਦੁਬਾਰਾ ਕੋਸ਼ਿਸ਼ ਕਰੋ।

## Add a mail account: fields

add-account-field-address = ਈਮੇਲ ਪਤਾ
add-account-incoming = ਆਉਣ ਵਾਲੀ ਮੇਲ ({ $protocol })
add-account-outgoing = ਜਾਣ ਵਾਲੀ ਮੇਲ ({ $protocol })
add-account-field-server = ਸਰਵਰ
add-account-field-port = ਪੋਰਟ
add-account-security-none = ਕੋਈ ਨਹੀਂ
add-account-field-username = ਵਰਤੋਂਕਾਰ ਨਾਮ
add-account-field-password = ਪਾਸਵਰਡ
add-account-show-password = ਪਾਸਵਰਡ ਦਿਖਾਓ
add-account-app-password-hint = { $provider } ਨੂੰ ਇੱਥੇ ਐਪ ਪਾਸਵਰਡ ਚਾਹੀਦਾ ਹੈ, ਉਹ ਨਹੀਂ ਜੋ ਤੁਸੀਂ ਵੈੱਬ ’ਤੇ ਵਰਤਦੇ ਹੋ। ਆਪਣੇ { $provider } ਖਾਤੇ ਦੀਆਂ ਸੁਰੱਖਿਆ ਸੈਟਿੰਗਾਂ ਵਿੱਚ ਇੱਕ ਬਣਾਓ।
add-account-field-name = ਤੁਹਾਡਾ ਨਾਮ (ਵਿਕਲਪਿਕ)
add-account-name-hint = ਉਹਨਾਂ ਲੋਕਾਂ ਨੂੰ ਦਿਖਾਇਆ ਜਾਂਦਾ ਹੈ ਜਿਨ੍ਹਾਂ ਨੂੰ ਤੁਸੀਂ ਲਿਖਦੇ ਹੋ।
add-account-servers-pair = { $imap } ਅਤੇ { $smtp }
add-account-servers-found = { $source ->
    [built-in] ਸਰਵਰ: { $servers }, Katna ਦੀ ਪ੍ਰਦਾਤਾ ਸੂਚੀ ਵਿੱਚ ਮਿਲੇ।
    [provider] ਸਰਵਰ: { $servers }, ਤੁਹਾਡੇ ਪ੍ਰਦਾਤਾ ਦੀਆਂ ਸੈਟਿੰਗਾਂ ਵਿੱਚ ਮਿਲੇ।
    [ispdb] ਸਰਵਰ: { $servers }, Thunderbird ਦੀ ਪ੍ਰਦਾਤਾ ਸੂਚੀ ਵਿੱਚ ਮਿਲੇ।
    [dns] ਸਰਵਰ: { $servers }, ਤੁਹਾਡੇ ਡੋਮੇਨ ਦੇ DNS ਰਿਕਾਰਡਾਂ ਵਿੱਚ ਮਿਲੇ।
   *[other] ਸਰਵਰ: { $servers }, ਅੰਦਾਜ਼ੇ ਨਾਲ; ਜੇ ਸਾਈਨ ਇਨ ਅਸਫਲ ਹੋਵੇ ਤਾਂ ਇਹਨਾਂ ਦੀ ਜਾਂਚ ਕਰੋ।
}
add-account-servers-entered = ਸਰਵਰ: { $servers }, ਜਿਵੇਂ ਦਰਜ ਕੀਤੇ।
add-account-or = ਜਾਂ
add-account-sign-in-with = { $provider } ਨਾਲ ਸਾਈਨ ਇਨ ਕਰੋ
add-account-sign-in-instead = ਇਸਦੀ ਬਜਾਏ { $provider } ਨਾਲ ਸਾਈਨ ਇਨ ਕਰੋ

## Add a mail account: buttons

add-account-servers-button = ਸਰਵਰ ਸੈਟਿੰਗਾਂ
add-account-back = ਪਿੱਛੇ
add-account-add = ਖਾਤਾ ਸ਼ਾਮਲ ਕਰੋ
add-account-next = ਅੱਗੇ
add-account-cancel = ਰੱਦ ਕਰੋ

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] ਆਉਣ ਵਾਲੀ ਮੇਲ ਦਾ ਸਰਵਰ ਦਰਜ ਕਰੋ।
   *[outgoing] ਜਾਣ ਵਾਲੀ ਮੇਲ ਦਾ ਸਰਵਰ ਦਰਜ ਕਰੋ।
}
add-account-server-space = { $kind ->
    [incoming] ਆਉਣ ਵਾਲੀ ਮੇਲ ਦੇ ਸਰਵਰ ਦੇ ਨਾਮ ਵਿੱਚ ਖਾਲੀ ਥਾਂ ਹੈ।
   *[outgoing] ਜਾਣ ਵਾਲੀ ਮੇਲ ਦੇ ਸਰਵਰ ਦੇ ਨਾਮ ਵਿੱਚ ਖਾਲੀ ਥਾਂ ਹੈ।
}
add-account-port-invalid = { $kind ->
    [incoming] ਆਉਣ ਵਾਲੀ ਮੇਲ ਦਾ ਪੋਰਟ { $min } ਤੋਂ { $max } ਤੱਕ ਦਾ ਅੰਕ ਹੋਣਾ ਚਾਹੀਦਾ ਹੈ।
   *[outgoing] ਜਾਣ ਵਾਲੀ ਮੇਲ ਦਾ ਪੋਰਟ { $min } ਤੋਂ { $max } ਤੱਕ ਦਾ ਅੰਕ ਹੋਣਾ ਚਾਹੀਦਾ ਹੈ।
}
add-account-address-empty = ਈਮੇਲ ਪਤਾ ਦਰਜ ਕਰੋ।
add-account-address-invalid = { $example } ਵਰਗਾ ਈਮੇਲ ਪਤਾ ਦਰਜ ਕਰੋ।
add-account-not-found = Katna ਨੂੰ { $address } ਲਈ ਸਰਵਰ ਨਹੀਂ ਮਿਲੇ, ਇਸ ਲਈ ਆਮ ਨਾਮ ਭਰ ਦਿੱਤੇ ਗਏ। ਆਪਣੇ ਪ੍ਰਦਾਤਾ ਤੋਂ ਇਹਨਾਂ ਦੀ ਪੁਸ਼ਟੀ ਕਰੋ।
add-account-password-empty = ਪਾਸਵਰਡ ਦਰਜ ਕਰੋ।
add-account-name-is-password = ਨਾਮ ਪਾਸਵਰਡ ਵਰਗਾ ਹੀ ਹੈ। ਉੱਥੇ ਇਸਦੀ ਬਜਾਏ ਆਪਣਾ ਨਾਮ ਲਿਖੋ, ਜਿਵੇਂ ਲੋਕਾਂ ਨੂੰ ਦਿਖਣਾ ਚਾਹੀਦਾ ਹੈ।
add-account-added = { $address } ਸ਼ਾਮਲ ਕੀਤਾ ਗਿਆ। ਤੁਹਾਡੀ ਮੇਲ ਲਿਆਂਦੀ ਜਾ ਰਹੀ ਹੈ…
add-account-app-password-refused = { $provider } ਨੇ ਪਾਸਵਰਡ ਅਸਵੀਕਾਰ ਕਰ ਦਿੱਤਾ। ਇਸਨੂੰ ਐਪ ਪਾਸਵਰਡ ਚਾਹੀਦਾ ਹੈ, ਉਹ ਨਹੀਂ ਜੋ ਤੁਸੀਂ ਵੈੱਬ ’ਤੇ ਵਰਤਦੇ ਹੋ।
add-account-password-refused = ਸਰਵਰ ਨੇ ਪਾਸਵਰਡ ਅਸਵੀਕਾਰ ਕਰ ਦਿੱਤਾ। ਇਸਦੀ ਜਾਂਚ ਕਰਕੇ ਦੁਬਾਰਾ ਕੋਸ਼ਿਸ਼ ਕਰੋ।
add-account-sign-in-refused = { $provider } ਨੇ Katna ਨੂੰ ਅੰਦਰ ਨਹੀਂ ਆਉਣ ਦਿੱਤਾ। ਦੁਬਾਰਾ ਕੋਸ਼ਿਸ਼ ਕਰੋ, ਅਤੇ ਆਪਣੀ ਮੇਲ ਤੱਕ ਪਹੁੰਚ ਦੀ ਇਜਾਜ਼ਤ ਦਿਓ।
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Katna ਦੀ ਇਹ ਕਾਪੀ ਹਾਲੇ Microsoft ਖਾਤਿਆਂ ਵਿੱਚ ਸਾਈਨ ਇਨ ਨਹੀਂ ਕਰ ਸਕਦੀ।
    [Google] Katna ਦੀ ਇਹ ਕਾਪੀ ਹਾਲੇ Google ਖਾਤਿਆਂ ਵਿੱਚ ਸਾਈਨ ਇਨ ਨਹੀਂ ਕਰ ਸਕਦੀ।
   *[other] ਇਹ ਪ੍ਰਦਾਤਾ ਸਿਰਫ਼ ਆਪਣੇ ਪੰਨੇ ’ਤੇ ਹੀ ਸਾਈਨ ਇਨ ਕਰਨ ਦਿੰਦਾ ਹੈ, ਜੋ Katna ਹਾਲੇ ਇਸ ਲਈ ਨਹੀਂ ਕਰ ਸਕਦਾ।
}
add-account-signed-in = { $provider } ਨਾਲ ਸਾਈਨ ਇਨ ਹੋ ਗਿਆ। ਤੁਹਾਡੀ ਮੇਲ ਲਿਆ ਰਿਹਾ ਹੈ…

## The account menu (from the account button on the top bar)

add-account-menu-another = ਇੱਕ ਹੋਰ ਖਾਤਾ ਸ਼ਾਮਲ ਕਰੋ
add-account-menu-manage = ਖਾਤਿਆਂ ਦਾ ਪ੍ਰਬੰਧਨ ਕਰੋ
app-menu = ਮੁੱਖ ਮੀਨੂ
