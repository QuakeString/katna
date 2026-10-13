# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = _فتح البريد الوارد
tray-new-message = _رسالة جديدة
tray-new-task = _مهمة جديدة
tray-new-note = م_لاحظة جديدة
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
tray-password-refused = يلزم إدخال كلمة مرور جديدة لـ { $address }
tray-signed-out = سجّل الدخول مجددًا إلى { $address }
tray-accounts-need-you = حسابات تحتاج إليك: { $count }
tray-not-sent = { $count ->
    [zero] لم تُرسَل أي رسالة
    [one] لم تُرسَل رسالة واحدة
    [two] لم تُرسَل رسالتان
    [few] لم تُرسَل { $count } رسائل
    [many] لم تُرسَل { $count } رسالة
   *[other] لم تُرسَل { $count } رسالة
}
