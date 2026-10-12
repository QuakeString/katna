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
notify-follow-up-sent = تم إرسال رسالة المتابعة
notify-follow-up-sent-to = لم يرد أحد على «{ $subject }»، لذا أرسل Katna رسالة متابعة.
notify-follow-up-waiting = لم تُرسَل رسالة المتابعة
notify-follow-up-waiting-to = حان موعدها بينما كان هذا الكمبيوتر مطفأً. عادت «{ $subject }» إلى بريدك الوارد.
notify-tracking-opened = فتح { $who } رسالة { $subject }
notify-tracking-clicked = نقر { $who } على رابط في رسالة { $subject }

notify-update-ready = يمكن تحديث Katna Mail
notify-update-ready-body = تم تنزيل الإصدار { $version }. يثبّته زر التحديث ويعيد تشغيل Katna Mail.
notify-update = تحديث
notify-signed-out = سجّل الدخول مجددًا
notify-signed-out-body = سجّل { $provider } خروج Katna من { $address }. توقفت مزامنة البريد.
notify-sign-in = تسجيل الدخول
notify-password-refused = تم رفض كلمة المرور
notify-password-refused-body = رفض خادم البريد كلمة المرور لـ { $address }. ربما تغيّرت.
notify-new-password = كلمة مرور جديدة
notify-not-sent = لم تُرسَل «{ $subject }»
notify-not-sent-no-subject = لم تُرسَل رسالة
notify-not-sent-body = إنها في صندوق الصادر، حيث تجد السبب.
notify-open-outbox = فتح صندوق الصادر
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
notify-task-done = وضع علامة كمكتمل

## Its buttons

notify-open = فتح
notify-peek = إلقاء نظرة
notify-reply = رد
notify-reply-placeholder = الرد على { $name }…
notify-send = إرسال
notify-reply-quote-header = في { $date }، كتب { $from }:
notify-reply-quote-header-no-date = كتب { $from }:
notify-reply-all = الرد على الكل
notify-mark-read = وضع علامة «مقروءة»
notify-mark-all-read = وضع علامة «مقروءة» على الكل
notify-archive = أرشفة
notify-snooze-hour = تأجيل ساعة واحدة
notify-snooze-tomorrow = غدًا
notify-copy-code = نسخ { $code }
notify-link-verify = التحقق على { $domain }
notify-link-confirm = التأكيد على { $domain }
notify-link-activate = التفعيل على { $domain }
notify-archived = تمت الأرشفة
notify-archived-count = { $count ->
    [zero] تم نقل { $count } رسالة من البريد الوارد
    [one] تم نقل رسالة واحدة من البريد الوارد
    [two] تم نقل رسالتين من البريد الوارد
    [few] تم نقل { $count } رسائل من البريد الوارد
    [many] تم نقل { $count } رسالة من البريد الوارد
   *[other] تم نقل { $count } رسالة من البريد الوارد
}
notify-undo = تراجع
notify-code-copied = تم نسخ الرمز
notify-code-not-copied = تعذّر نسخ الرمز
notify-reply-sent = تم إرسال الرد إلى { $name }
notify-open-in-katna = الفتح في Katna
