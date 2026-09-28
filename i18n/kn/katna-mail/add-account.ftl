# Katna Mail, Kannada (ಕನ್ನಡ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = ಮೇಲ್ ಖಾತೆಯನ್ನು ಸೇರಿಸಿ
add-account-looking = { $address } ನ ಮೇಲ್ ಸರ್ವರ್‌ಗಳನ್ನು ಹುಡುಕಲಾಗುತ್ತಿದೆ…
add-account-address-intro = ನಿಮ್ಮ ಇಮೇಲ್ ವಿಳಾಸವನ್ನು ನಮೂದಿಸಿ. Katna ನಿಮಗಾಗಿ ಸರ್ವರ್‌ಗಳನ್ನು ಹುಡುಕುತ್ತದೆ.
add-account-servers-title = ಸರ್ವರ್ ಸೆಟ್ಟಿಂಗ್‌ಗಳು
add-account-servers-intro = { $address } ಗಾಗಿ Katna ಮೇಲ್ ಅನ್ನು ಎಲ್ಲಿಂದ ಓದುತ್ತದೆ ಮತ್ತು ಕಳುಹಿಸುತ್ತದೆ.
add-account-password-title = ನಿಮ್ಮ ಪಾಸ್‌ವರ್ಡ್ ನಮೂದಿಸಿ
add-account-signing-in = ಸೈನ್ ಇನ್ ಆಗುತ್ತಿದೆ…
add-account-browser-title = ನಿಮ್ಮ ಬ್ರೌಸರ್‌ನಲ್ಲಿ ಮುಂದುವರಿಯಿರಿ
add-account-browser-intro = Katna ನಿಮ್ಮ ಬ್ರೌಸರ್‌ನಲ್ಲಿ { $provider } ಸೈನ್ ಇನ್ ಪುಟವನ್ನು ತೆರೆದಿದೆ. ಅಲ್ಲಿ ಸೈನ್ ಇನ್ ಮಾಡಿ ಮತ್ತು ನಿಮ್ಮ ಮೇಲ್ ಓದಲು ಹಾಗೂ ಕಳುಹಿಸಲು Katna ಗೆ ಅನುಮತಿಸಿ, ನಂತರ ಇಲ್ಲಿಗೆ ಹಿಂತಿರುಗಿ.
add-account-browser-hint = ಯಾವುದೇ ಪುಟ ತೆರೆಯಲಿಲ್ಲವೇ? ನಿಮ್ಮ ಬ್ರೌಸರ್‌ನ ವಿಂಡೋಗಳನ್ನು ಪರಿಶೀಲಿಸಿ, ಅಥವಾ ಹಿಂದೆ ಹೋಗಿ ಮತ್ತೆ ಪ್ರಯತ್ನಿಸಿ.

## Add a mail account: fields

add-account-field-address = ಇಮೇಲ್ ವಿಳಾಸ
add-account-incoming = ಒಳಬರುವ ಮೇಲ್ ({ $protocol })
add-account-outgoing = ಹೊರಹೋಗುವ ಮೇಲ್ ({ $protocol })
add-account-field-server = ಸರ್ವರ್
add-account-field-port = ಪೋರ್ಟ್
add-account-security-none = ಯಾವುದೂ ಇಲ್ಲ
add-account-field-username = ಬಳಕೆದಾರ ಹೆಸರು
add-account-field-password = ಪಾಸ್‌ವರ್ಡ್
add-account-show-password = ಪಾಸ್‌ವರ್ಡ್ ತೋರಿಸಿ
add-account-app-password-hint = ಇಲ್ಲಿ { $provider } ಗೆ ಆ್ಯಪ್ ಪಾಸ್‌ವರ್ಡ್ ಬೇಕು, ನೀವು ವೆಬ್‌ನಲ್ಲಿ ಬಳಸುವುದಲ್ಲ. ನಿಮ್ಮ { $provider } ಖಾತೆಯ ಭದ್ರತಾ ಸೆಟ್ಟಿಂಗ್‌ಗಳಲ್ಲಿ ಒಂದನ್ನು ಮಾಡಿ.
add-account-field-name = ನಿಮ್ಮ ಹೆಸರು (ಐಚ್ಛಿಕ)
add-account-name-hint = ನೀವು ಬರೆಯುವ ಜನರಿಗೆ ತೋರಿಸಲಾಗುತ್ತದೆ.
add-account-servers-pair = { $imap } ಮತ್ತು { $smtp }
add-account-servers-found = { $source ->
    [built-in] ಸರ್ವರ್‌ಗಳು: { $servers }, Katna ದ ಪೂರೈಕೆದಾರರ ಪಟ್ಟಿಯಲ್ಲಿ ಸಿಕ್ಕಿವೆ.
    [provider] ಸರ್ವರ್‌ಗಳು: { $servers }, ನಿಮ್ಮ ಪೂರೈಕೆದಾರರ ಸೆಟ್ಟಿಂಗ್‌ಗಳಲ್ಲಿ ಸಿಕ್ಕಿವೆ.
    [ispdb] ಸರ್ವರ್‌ಗಳು: { $servers }, Thunderbird ನ ಪೂರೈಕೆದಾರರ ಪಟ್ಟಿಯಲ್ಲಿ ಸಿಕ್ಕಿವೆ.
    [dns] ಸರ್ವರ್‌ಗಳು: { $servers }, ನಿಮ್ಮ ಡೊಮೇನ್‌ನ DNS ದಾಖಲೆಗಳಲ್ಲಿ ಸಿಕ್ಕಿವೆ.
   *[other] ಸರ್ವರ್‌ಗಳು: { $servers }, ಊಹೆಯಿಂದ; ಸೈನ್ ಇನ್ ವಿಫಲವಾದರೆ ಅವುಗಳನ್ನು ಪರಿಶೀಲಿಸಿ.
}
add-account-servers-entered = ಸರ್ವರ್‌ಗಳು: { $servers }, ನಮೂದಿಸಿದಂತೆ.
add-account-or = ಅಥವಾ
add-account-sign-in-with = { $provider } ಮೂಲಕ ಸೈನ್ ಇನ್ ಮಾಡಿ
add-account-sign-in-instead = ಬದಲಿಗೆ { $provider } ಮೂಲಕ ಸೈನ್ ಇನ್ ಮಾಡಿ

## Add a mail account: buttons

add-account-servers-button = ಸರ್ವರ್ ಸೆಟ್ಟಿಂಗ್‌ಗಳು
add-account-back = ಹಿಂದೆ
add-account-add = ಖಾತೆ ಸೇರಿಸಿ
add-account-next = ಮುಂದೆ
add-account-cancel = ರದ್ದುಮಾಡಿ

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] ಒಳಬರುವ ಸರ್ವರ್ ಅನ್ನು ನಮೂದಿಸಿ.
   *[outgoing] ಹೊರಹೋಗುವ ಸರ್ವರ್ ಅನ್ನು ನಮೂದಿಸಿ.
}
add-account-server-space = { $kind ->
    [incoming] ಒಳಬರುವ ಸರ್ವರ್‌ನ ಹೆಸರಿನಲ್ಲಿ ಸ್ಪೇಸ್ ಇದೆ.
   *[outgoing] ಹೊರಹೋಗುವ ಸರ್ವರ್‌ನ ಹೆಸರಿನಲ್ಲಿ ಸ್ಪೇಸ್ ಇದೆ.
}
add-account-port-invalid = { $kind ->
    [incoming] ಒಳಬರುವ ಪೋರ್ಟ್ { $min } ರಿಂದ { $max } ವರೆಗಿನ ಸಂಖ್ಯೆಯಾಗಿರಬೇಕು.
   *[outgoing] ಹೊರಹೋಗುವ ಪೋರ್ಟ್ { $min } ರಿಂದ { $max } ವರೆಗಿನ ಸಂಖ್ಯೆಯಾಗಿರಬೇಕು.
}
add-account-address-empty = ಇಮೇಲ್ ವಿಳಾಸವನ್ನು ನಮೂದಿಸಿ.
add-account-address-invalid = { $example } ನಂತಹ ಇಮೇಲ್ ವಿಳಾಸವನ್ನು ನಮೂದಿಸಿ.
add-account-not-found = Katna ಗೆ { $address } ನ ಸರ್ವರ್‌ಗಳು ಸಿಗಲಿಲ್ಲ, ಆದ್ದರಿಂದ ಅದು ಸಾಮಾನ್ಯ ಹೆಸರುಗಳನ್ನು ತುಂಬಿದೆ. ನಿಮ್ಮ ಪೂರೈಕೆದಾರರೊಂದಿಗೆ ಅವುಗಳನ್ನು ಪರಿಶೀಲಿಸಿ.
add-account-password-empty = ಪಾಸ್‌ವರ್ಡ್ ನಮೂದಿಸಿ.
add-account-name-is-password = ಹೆಸರು ಪಾಸ್‌ವರ್ಡ್‌ನಂತೆಯೇ ಇದೆ. ಅದರ ಬದಲು ಅಲ್ಲಿ ನಿಮ್ಮ ಹೆಸರನ್ನು, ಜನರು ನೋಡಬೇಕಾದಂತೆ, ಟೈಪ್ ಮಾಡಿ.
add-account-added = { $address } ಅನ್ನು ಸೇರಿಸಲಾಗಿದೆ. ನಿಮ್ಮ ಮೇಲ್ ತರಲಾಗುತ್ತಿದೆ…
add-account-app-password-refused = { $provider } ಪಾಸ್‌ವರ್ಡ್ ಅನ್ನು ನಿರಾಕರಿಸಿದೆ. ಅದಕ್ಕೆ ಆ್ಯಪ್ ಪಾಸ್‌ವರ್ಡ್ ಬೇಕು, ನೀವು ವೆಬ್‌ನಲ್ಲಿ ಬಳಸುವುದಲ್ಲ.
add-account-password-refused = ಸರ್ವರ್ ಪಾಸ್‌ವರ್ಡ್ ಅನ್ನು ನಿರಾಕರಿಸಿದೆ. ಅದನ್ನು ಪರಿಶೀಲಿಸಿ ಮತ್ತೆ ಪ್ರಯತ್ನಿಸಿ.
add-account-sign-in-refused = { $provider } Katna ಅನ್ನು ಒಳಗೆ ಬಿಡಲಿಲ್ಲ. ಮತ್ತೆ ಪ್ರಯತ್ನಿಸಿ, ಮತ್ತು ನಿಮ್ಮ ಮೇಲ್‌ಗೆ ಪ್ರವೇಶ ಅನುಮತಿಸಿ.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Katna ನ ಈ ಪ್ರತಿಗೆ ಇನ್ನೂ Microsoft ಖಾತೆಗಳಿಗೆ ಸೈನ್ ಇನ್ ಮಾಡಲು ಸಾಧ್ಯವಿಲ್ಲ.
    [Google] Katna ನ ಈ ಪ್ರತಿಗೆ ಇನ್ನೂ Google ಖಾತೆಗಳಿಗೆ ಸೈನ್ ಇನ್ ಮಾಡಲು ಸಾಧ್ಯವಿಲ್ಲ.
   *[other] ಈ ಪೂರೈಕೆದಾರರು ತಮ್ಮದೇ ಪುಟದಲ್ಲಿ ಮಾತ್ರ ಸೈನ್ ಇನ್ ಮಾಡಲು ಅನುಮತಿಸುತ್ತಾರೆ, ಅದನ್ನು Katna ಇನ್ನೂ ಅವರಿಗಾಗಿ ಮಾಡಲಾರದು.
}
add-account-signed-in = { $provider } ಮೂಲಕ ಸೈನ್ ಇನ್ ಆಗಿದೆ. ನಿಮ್ಮ ಮೇಲ್ ಪಡೆಯಲಾಗುತ್ತಿದೆ…

## The account menu (from the account button on the top bar)

add-account-menu-another = ಇನ್ನೊಂದು ಖಾತೆ ಸೇರಿಸಿ
add-account-menu-manage = ಖಾತೆಗಳನ್ನು ನಿರ್ವಹಿಸಿ
app-menu = ಮುಖ್ಯ ಮೆನು
