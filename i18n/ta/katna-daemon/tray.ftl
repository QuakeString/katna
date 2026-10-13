# Katna Mail, Tamil (தமிழ்).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = _இன்பாக்ஸைத் திற
tray-new-message = _புதிய மெசேஜ்
tray-new-task = புதிய _பணி
tray-new-note = புதிய _குறிப்பு
tray-preferences = _அமைப்புகள்
tray-quit = _வெளியேறு

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] படிக்காத மின்னஞ்சல் இல்லை
    [one] படிக்காத { $count } மெசேஜ்
   *[other] படிக்காத { $count } மெசேஜ்கள்
}
tray-password-refused = { $address } க்குப் புதிய கடவுச்சொல் தேவை
tray-signed-out = { $address } இல் மீண்டும் உள்நுழையுங்கள்
tray-accounts-need-you = { $count } கணக்குகளுக்கு உங்கள் கவனம் தேவை
tray-not-sent = { $count ->
    [one] { $count } மெசேஜ் அனுப்பப்படவில்லை
   *[other] { $count } மெசேஜ்கள் அனுப்பப்படவில்லை
}
