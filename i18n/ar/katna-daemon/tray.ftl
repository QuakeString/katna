# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = _فتح البريد الوارد
tray-new-message = _رسالة جديدة
tray-preferences = _الإعدادات
tray-quit = _إنهاء

## The tray icon's tooltip

tray-unread = { $count ->
    [0] لا توجد رسائل غير مقروءة
    [zero] { $count } رسالة غير مقروءة
    [one] رسالة واحدة غير مقروءة
    [two] رسالتان غير مقروءتين
    [few] { $count } رسائل غير مقروءة
    [many] { $count } رسالة غير مقروءة
   *[other] { $count } رسالة غير مقروءة
}
