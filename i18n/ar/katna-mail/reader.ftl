# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = إغلاق
reader-back = رجوع
reader-mark-unread = وضع علامة «غير مقروءة»
reader-move-to = نقل إلى
reader-more = المزيد
reader-original-colors = إظهار الألوان الأصلية
reader-dark-colors = إظهار بألوان داكنة
reader-print-all = طباعة الكل
reader-new-window = في نافذة جديدة
reader-position = { $position } من { $total }
reader-newer = أحدث
reader-older = أقدم

## Reading pane: the conversation

reader-removed = تمت إزالة هذه المحادثة.
reader-no-subject = (بلا موضوع)
reader-collapse-all = تصغير الكل
reader-expand-all = توسيع الكل
reader-unknown-sender = (مُرسِل غير معروف)
reader-date-ago = { $date } ({ $ago })
reader-me = أنا
reader-to = إلى { $names }
reader-to-label = إلى
reader-tick-delivered = تم التسليم { $when }
reader-tick-no-bounce = أُرسلت { $when }؛ ولم يعد أي إشعار بفشل التسليم، فالأرجح أنها وصلت
reader-tick-bounced = لم تُسلَّم: أُعيدت { $when }
reader-tick-read = قُرئت { $when } (إشعار بالقراءة)
reader-tick-opened = فُتحت، آخر مرة { $when } (تتبُّع الفتح)
reader-starred = مميّزة بنجمة
reader-not-starred = غير مميّزة بنجمة
reader-too-long = الرسالة طويلة جدًا بحيث لا يمكن عرضها بالكامل.
reader-encrypted-images = لا يتم أبدًا تحميل الصور من الويب في البريد المشفّر.
reader-window-failed = تعذّر فتح نافذة جديدة.

## Reading pane: message details (opened from "to me")

reader-details-from = من:
reader-details-to = إلى:
reader-details-cc = نسخة إلى:
reader-details-date = التاريخ:
reader-details-subject = الموضوع:

## Reading pane: downloading a message

reader-downloading = جارٍ تنزيل هذه الرسالة من الخادم…
reader-download-failed = تعذّر تنزيل هذه الرسالة.
reader-try-again = إعادة المحاولة

## Reply row

reply-reply = رد
reply-reply-all = الرد على الكل
reply-forward = إعادة توجيه

## Encrypted and signed mail

security-decrypting = جارٍ فك التشفير…
security-checking = جارٍ التحقق من التوقيع…
security-partly-encrypted = جزء فقط من هذه الرسالة مشفّر. أُضيف الباقي خارج الحماية وقد يكون مصدره أي شخص.
security-partly-signed = جزء فقط من هذه الرسالة موقَّع. أُضيف الباقي خارج الحماية وقد يكون مصدره أي شخص.
security-encrypted = رسالة مشفّرة
security-encrypted-smime = رسالة مشفّرة (S/MIME)
security-no-key = يتعذّر فك تشفير هذه الرسالة: شُفّرت لمفتاح ليس لديك.
security-cancelled = تم إلغاء فك التشفير.
security-damaged = يتعذّر فك تشفير هذه الرسالة: البيانات المشفّرة تالفة أو تم تغييرها.
security-decrypt-unavailable = يتعذّر فك تشفير هذه الرسالة: ثبّت { $tool } لقراءة البريد المشفّر.
security-decrypt-failed = يتعذّر فك تشفير هذه الرسالة: { $reason }
security-unknown-signer = موقِّع غير معروف
security-signed-verified = وقّعها { $signer } · تم التحقق
security-signed-not-sender = وقّعها { $signer }، وهو ليس المُرسِل
security-signed-untrusted = وقّعها { $signer } بمفتاح وضعت عليه علامة «غير موثوق به»
security-signed-unverified = وقّعها { $signer } · لم يتم التحقق من المفتاح
security-bad-signature = توقيع غير صالح: تم تغيير هذه الرسالة بعد توقيعها، أو أن التوقيع مزوَّر.
security-signature-expired = وقّعها { $signer } · انتهت صلاحية التوقيع
security-key-expired = وقّعها { $signer } · انتهت صلاحية المفتاح منذ ذلك الحين
security-key-revoked = وقّعها { $signer } بمفتاح تم إبطاله
security-missing-key = موقَّعة بمفتاح ليس لديك، لذا يتعذّر التحقق منها
security-missing-key-id = موقَّعة بمفتاح ليس لديك ({ $key })، لذا يتعذّر التحقق منها
security-signature-unavailable = موقَّعة؛ ثبّت { $tool } للتحقق من التوقيع
security-signature-error = تعذّر التحقق من التوقيع.
tracking-opened = فتحها { $who } { $count ->
    [zero] { $count } مرة
    [one] مرة واحدة
    [two] مرتين
    [few] { $count } مرات
    [many] { $count } مرة
   *[other] { $count } مرة
}، آخرها { $when }
tracking-opens-clicks = فتحها { $who } { $opens ->
    [zero] { $opens } مرة
    [one] مرة واحدة
    [two] مرتين
    [few] { $opens } مرات
    [many] { $opens } مرة
   *[other] { $opens } مرة
} وتابع رابطًا { $clicks ->
    [zero] { $clicks } مرة
    [one] مرة واحدة
    [two] مرتين
    [few] { $clicks } مرات
    [many] { $clicks } مرة
   *[other] { $clicks } مرة
}، آخرها { $when }
tracking-clicked = تابع { $who } رابطًا { $clicks ->
    [zero] { $clicks } مرة
    [one] مرة واحدة
    [two] مرتين
    [few] { $clicks } مرات
    [many] { $clicks } مرة
   *[other] { $clicks } مرة
}، آخرها { $when }
tracking-maybe-opened = ربما فتحها { $who } (يحمّل Apple Mail الصور حفاظًا على الخصوصية)
tracking-seen-none = لم يفتحها أحد أو يتابع رابطًا فيها بعد
tracking-receipt = أرسل { $who } إشعارًا بالقراءة
tracking-receipt-displayed = إشعار بالقراءة: فتح { $who } رسالتك
tracking-receipt-other = إشعار بالقراءة: حذف { $who } رسالتك أو تعامل معها دون فتحها

## Remote images and pictures

remote-hidden = الصور في هذه الرسالة مخفية.
remote-show = عرض الصور
remote-always-show = العرض دائمًا من هذا المُرسِل
remote-picture-use = استخدام
remote-picture-too-big = اختر صورة لا يزيد حجمها عن 8 ميغابايت.
remote-picture-type = اختر صورة بتنسيق PNG أو JPEG أو GIF أو WebP أو SVG.
remote-picture-read-failed = تتعذّر قراءة الصورة: { $error }
remote-picture-keep-failed = يتعذّر الاحتفاظ بالصورة: { $error }
remote-picture-remove-failed = تتعذّر إزالة الصورة: { $error }

## Attachments

attachment-count = { $count ->
    [zero] { $count } مرفق
    [one] مرفق واحد
    [two] مرفقان
    [few] { $count } مرفقات
    [many] { $count } مرفقًا
   *[other] { $count } مرفق
}
attachment-save = حفظ
attachment-save-all = حفظ الكل
attachment-save-all-tooltip = حفظ كل المرفقات في مجلد
attachment-save-here = الحفظ هنا
attachment-not-downloaded = لم يتم تنزيل هذه الرسالة.
attachment-not-found = تعذّر العثور على هذا المرفق في الرسالة.
attachment-read-failed = تعذّرت قراءة { $name }
attachment-numbered = المرفق { $number }
attachment-saved-all = { $count ->
    [zero] تم حفظ { $count } ملف في { $place }
    [one] تم حفظ ملف واحد في { $place }
    [two] تم حفظ ملفين في { $place }
    [few] تم حفظ { $count } ملفات في { $place }
    [many] تم حفظ { $count } ملفًا في { $place }
   *[other] تم حفظ { $count } ملف في { $place }
}
attachment-saved-some = { $total ->
    [zero] تم حفظ { $saved } من أصل { $total } ملف في { $place }. تعذّر حفظ { $failed }
    [one] تم حفظ { $saved } من أصل ملف واحد في { $place }. تعذّر حفظ { $failed }
    [two] تم حفظ { $saved } من أصل ملفين في { $place }. تعذّر حفظ { $failed }
    [few] تم حفظ { $saved } من أصل { $total } ملفات في { $place }. تعذّر حفظ { $failed }
    [many] تم حفظ { $saved } من أصل { $total } ملفًا في { $place }. تعذّر حفظ { $failed }
   *[other] تم حفظ { $saved } من أصل { $total } ملف في { $place }. تعذّر حفظ { $failed }
}
attachment-saved-to = تم الحفظ في { $path }
attachment-save-failed = تعذّر حفظ { $name }: { $error }
attachment-open-failed = تعذّر فتح { $name }: { $error }
attachment-risky = قد يشغّل هذا الملف برنامجًا، لذا لا يفتحه Katna. احفظه بدلًا من ذلك.
attachment-encrypted-open = وصل هذا الملف مشفّرًا. احفظه لفتحه في مكان آخر.

## Printing

print-failed = تعذّرت الطباعة: { $error }
print-no-font = لم يتم العثور على أي خط
print-opened-as-pdf = تم الفتح كملف PDF للطباعة منه.
print-preview-title = معاينة الطباعة
print-preview-laying-out = جارٍ تنسيق الصفحات…
print-preview-pages = { $count ->
    [zero] { $count } صفحة
    [one] صفحة واحدة
    [two] صفحتان
    [few] { $count } صفحات
    [many] { $count } صفحة
   *[other] { $count } صفحة
}
print-preview-more = { $count ->
    [zero] و{ $count } صفحة أخرى
    [one] وصفحة أخرى
    [two] وصفحتان أخريان
    [few] و{ $count } صفحات أخرى
    [many] و{ $count } صفحة أخرى
   *[other] و{ $count } صفحة أخرى
}
print-preview-failed = تعذّر عرض الصفحات
print-preview-paper = الورق
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-layout = التخطيط
print-preview-as-shown = كما يظهر
print-preview-simple = نص بسيط
print-preview-backgrounds = الخلفيات
print-preview-cancel = إلغاء
print-preview-print = طباعة
print-not-downloaded = (لم يتم التنزيل بعد.)
print-encrypted = (مشفّرة. افتحها في Katna Mail لطباعة نصها.)
print-to = إلى: { $addresses }
print-cc = نسخة إلى: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = افتح هذه الرسالة لقراءة مرفقاتها.
text-copy = نسخ
text-select-all = تحديد الكل
