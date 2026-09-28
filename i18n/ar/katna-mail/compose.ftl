# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = رسالة جديدة
compose-restore = استعادة
compose-minimize = تصغير
compose-exit-full-screen = الخروج من وضع ملء الشاشة
compose-open-window = فتح في نافذة جديدة
compose-save-close = حفظ وإغلاق
compose-back-to-mail = العودة إلى نافذة البريد
compose-pop-out-reply = فتح الرد في نافذة منفصلة
compose-edit-recipients = تعديل المستلمين
compose-summary-cc = نسخة: { $names }
compose-summary-bcc = نسخة مخفية: { $names }
compose-show-trimmed = عرض المحتوى المقتطع
compose-hide-trimmed = إخفاء المحتوى المقتطع
compose-remove-trimmed = إزالة النص المقتبس
compose-trimmed-removed = تمت إزالة النص المقتبس

## Recipients and subject

compose-to = إلى
compose-cc = نسخة
compose-bcc = نسخة مخفية
compose-from = من
compose-from-choose = إرسال من حساب آخر
compose-recipients = المستلمون
compose-subject = الموضوع

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = أرسل الرسالة المفتوحة أو تجاهلها أولًا.
compose-bad-address = «{ $address }» ليس عنوان بريد إلكتروني.
compose-no-recipients = أضف مستلمًا واحدًا على الأقل.
compose-attachments-too-large = حجم المرفقات { $size }، وتقبل خوادم البريد حتى { $limit }.
compose-no-account = أضف حسابًا لإرسال البريد منه.
compose-past-time = اختر وقتًا في المستقبل.
compose-scheduling = جارٍ الجدولة…
compose-sending = جارٍ الإرسال…
compose-scheduled = تمت جدولة الإرسال في { $when }
compose-sent-archived = تم الإرسال والأرشفة
compose-sent = تم إرسال الرسالة
compose-discarded = تم تجاهل المسودة
compose-draft-saved = تم حفظ المسودة
compose-draft-failed = تعذّر حفظ المسودة: { $error }
compose-draft-not-opened = تعذّر فتح المسودة.

## Attachments

compose-picker-insert = إدراج
compose-picker-attach = إرفاق
compose-file-too-large = الملف { $name } كبير جدًا: يمكن أن تحمل الرسالة حتى { $limit }.
compose-attachment-size = ({ $size })
compose-remove-attachment = إزالة المرفق
compose-attachments-total = { $count ->
    [zero] { $count } ملف، { $size }
    [one] ملف واحد، { $size }
    [two] ملفان، { $size }
    [few] { $count } ملفات، { $size }
    [many] { $count } ملفًا، { $size }
   *[other] { $count } ملف، { $size }
}
compose-drop-files = أفلت الملفات هنا
compose-drop-here = أفلت هنا
compose-paste-keep-formatting = الإبقاء على التنسيق
compose-paste-table = جدول
compose-paste-picture = صورة
compose-paste-plain-text = نص عادي
compose-paste-inline = داخل النص
compose-paste-attachment = مرفق

## Encryption and signing (the toggles by the recipients)

compose-encrypt = تشفير
compose-encrypted = مشفّرة: لا يقرؤها إلا المستلمون
compose-sign = توقيع
compose-signed = موقّعة: يمكن للمستلمين التحقق من أنها منك
compose-track = تتبُّع الفتح والنقرات
compose-tracked = متتبَّعة: ترى متى يفتحها كل مستلم أو يتابع رابطًا فيها
compose-track-clicks = تتبُّع النقر على الروابط (لا يمكن للنص العادي إظهار الفتح)
compose-tracked-clicks = متتبَّعة: ترى متى يتابع كل مستلم رابطًا فيها
compose-track-sign-in = سجّل الدخول إلى حساب Katna لتتبُّع الفتح والنقرات
compose-receipt = طلب إشعار بالقراءة
compose-receipt-on = طُلب إشعار بالقراءة: قد يطلب تطبيق المستلم منه إرساله
compose-delivery = طلب إشعار بالتسليم
compose-delivery-on = طُلب إشعار بالتسليم: سيرسل إليك خادم بريدك رسالة عندما يقبلها خادم كل مستلم
compose-delivery-unavailable = خادم بريدك لا يرسل إشعارات التسليم

## Spelling

spell-no-dictionary = لا يوجد قاموس إملائي مثبّت للغة { $language } (مثل hunspell-en_us).
spell-dictionary-error = القاموس الإملائي: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = «{ $words }»
grammar-add = إضافة «{ $words }»
grammar-remove = إزالة «{ $words }»
grammar-ignore = تجاهل

## Send checks (asked before a message goes out)

send-check-attachment-title = هل كنت تنوي إرفاق ملفات؟
send-check-attachment-text = ذكرت مرفقًا في رسالتك، لكن لا يوجد شيء مرفق.
send-check-attach = إرفاق ملف
send-check-subject-title = الإرسال بلا موضوع؟
send-check-subject-text = هذه الرسالة بلا موضوع.
send-check-add-subject = إضافة موضوع
send-check-send-anyway = الإرسال على أي حال
recipient-not-valid = ليس عنوان بريد إلكتروني صالحًا
recipient-show-address = إظهار العنوان
recipient-remove = إزالة
recipient-bad-title = تحقّق من العنوان
recipient-bad-text = «{ $address }» ليس عنوان بريد إلكتروني صالحًا. صحّحه أو أزِله قبل الإرسال.
recipient-bad-fix = تصحيح
