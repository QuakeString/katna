# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = _इनबॉक्स खोलें
tray-new-message = _नया मैसेज
tray-new-task = नया _टास्क
tray-new-note = नया _नोट
tray-preferences = _सेटिंग
tray-quit = _बाहर निकलें

## The tray icon's tooltip

tray-unread = { $count ->
    [0] कोई बिना पढ़ा मेल नहीं
    [one] { $count } बिना पढ़ा मैसेज
   *[other] { $count } बिना पढ़े मैसेज
}
tray-password-refused = { $address } के लिए नया पासवर्ड चाहिए
tray-signed-out = { $address } में फिर से साइन इन करें
tray-accounts-need-you = { $count } खातों को आपकी ज़रूरत है
tray-not-sent = { $count ->
    [one] { $count } मैसेज नहीं भेजा गया
   *[other] { $count } मैसेज नहीं भेजे गए
}
