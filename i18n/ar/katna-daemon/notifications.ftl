# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [zero] { $count } رسالة جديدة
    [one] { $count } رسالة جديدة
    [two] رسالتان جديدتان
    [few] { $count } رسائل جديدة
    [many] { $count } رسالة جديدة
   *[other] { $count } رسالة جديدة
}
notify-and-more = { $count ->
    [one] ورسالة أخرى
    [two] ورسالتان أخريان
    [few] و{ $count } رسائل أخرى
    [many] و{ $count } رسالة أخرى
   *[other] و{ $count } رسالة أخرى
}
notify-no-subject = (بلا موضوع)
notify-unknown-sender = مُرسِل غير معروف
notify-snooze-back = عاد من التأجيل
notify-no-reply = لا رد بعد
notify-no-reply-to = لم يرد أحد على «{ $subject }».
notify-tracking-opened = فتح { $who } رسالة { $subject }
notify-tracking-clicked = نقر { $who } على رابط في رسالة { $subject }

## Its buttons

notify-open = فتح
notify-reply-all = الرد على الكل
notify-mark-read = وضع علامة «مقروءة»
notify-mark-all-read = وضع علامة «مقروءة» على الكل
notify-archive = أرشفة
