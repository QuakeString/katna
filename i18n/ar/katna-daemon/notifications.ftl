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

notify-update-ready = يمكن تحديث Katna Mail
notify-update-ready-body = تم تنزيل الإصدار { $version }. يثبّته زر التحديث ويعيد تشغيل Katna Mail.
notify-update = تحديث
notify-event-now = الآن
notify-event-in-minutes = { $count ->
    [zero] بعد { $count } دقيقة
    [one] بعد { $count } دقيقة
    [two] بعد { $count } دقيقتين
    [few] بعد { $count } دقائق
    [many] بعد { $count } دقيقة
   *[other] بعد { $count } دقيقة
}
notify-event-in-hours = { $count ->
    [zero] بعد { $count } ساعة
    [one] بعد { $count } ساعة
    [two] بعد { $count } ساعتين
    [few] بعد { $count } ساعات
    [many] بعد { $count } ساعة
   *[other] بعد { $count } ساعة
}
notify-event-in-days = { $count ->
    [1] غدًا
    [zero] بعد { $count } يوم
    [one] بعد { $count } يوم
    [two] بعد { $count } يومين
    [few] بعد { $count } أيام
    [many] بعد { $count } يومًا
   *[other] بعد { $count } يوم
}
notify-event-all-day = طوال اليوم
notify-event-join = انضمام
notify-event-snooze = تأجيل 5 دقائق

## Its buttons

notify-open = فتح
notify-reply-all = الرد على الكل
notify-mark-read = وضع علامة «مقروءة»
notify-mark-all-read = وضع علامة «مقروءة» على الكل
notify-archive = أرشفة
