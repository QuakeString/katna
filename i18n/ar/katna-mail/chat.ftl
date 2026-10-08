# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.
chat-heading = القراءة
chat-view = المحادثات كدردشات
chat-view-detail = يُقرأ البريد المتبادل بين الأشخاص كدردشة جماعية: فقاعة لكل رسالة تحمل ما كُتب فقط، ورسائلك على الجانب الآخر. تحتفظ النشرات الإخبارية بالعرض المعتاد.
chat-view-switch = عرض المحادثات كدردشات
chat-view-switch-detail = يبقى البريد المقتبس والتوقيعات خلف ··· في كل فقاعة
chat-switch-chat = دردشة
chat-switch-mail = بريد
chat-people = { $names } وأنت · { $count ->
    [zero] { $count } رسالة
    [one] رسالة واحدة
    [two] رسالتان
    [few] { $count } رسائل
    [many] { $count } رسالة
   *[other] { $count } رسالة
}
chat-people-heading = { $count ->
    [zero] في هذه الدردشة · { $count } شخص
    [one] في هذه الدردشة · شخص واحد
    [two] في هذه الدردشة · شخصان
    [few] في هذه الدردشة · { $count } أشخاص
    [many] في هذه الدردشة · { $count } شخصًا
   *[other] في هذه الدردشة · { $count } شخص
}
chat-member-mails = { $count ->
    [0] لا رسائل
    [zero] { $count } رسالة
    [one] رسالة واحدة
    [two] رسالتان
    [few] { $count } رسائل
    [many] { $count } رسالة
   *[other] { $count } رسالة
}
chat-today = اليوم
chat-yesterday = أمس
chat-added = أضاف { $who } { $names }
chat-renamed = غيّر { $who } الموضوع إلى «{ $subject }»
chat-you = أنت
chat-not-downloaded = لم يتم التنزيل بعد
chat-forwarded = مُعاد توجيهها
chat-show-quoted = عرض البريد المقتبس والتوقيع
chat-hide-quoted = إخفاء البريد المقتبس والتوقيع
chat-hide-dots = إخفاء ···
chat-show-card = عرض بطاقته
chat-reply-all = الرد على الكل
chat-more = المزيد
chat-reply-only = الرد على { $name } فقط
chat-forward = إعادة توجيه
chat-copy-text = نسخ النص
chat-show-as-mail = العرض كبريد
chat-go-down = الانتقال إلى أحدث بريد
chat-pin = التثبيت في الأعلى
chat-pin-file = تثبيت الملف في الأعلى
chat-unpin = إلغاء التثبيت
chat-unpin-file = إلغاء تثبيت الملف
chat-pinned-of = المثبَّت { $at } من { $count }
chat-pins-all = كل العناصر المثبَّتة
chat-pins-heading = المثبَّتة · { $count } من { $most }
chat-pins-drag = اسحب لإعادة الترتيب
chat-pin-from-mail = رسالة من { $name } · { $when }
chat-pin-from-file = ملف من { $name } · { $when }
chat-pin-from-text = نص من { $name } · { $when }
chat-pins-full = تحتوي هذه الدردشة على 5 عناصر مثبَّتة بالفعل
chat-pins-replace-title = استبدال عنصر مثبَّت
chat-pins-replace-hint = تتسع الدردشة لـ 5 عناصر مثبَّتة كحد أقصى. اختر العنصر الذي تريد إزالته.
chat-pins-replace = استبدال
chat-pins-cancel = إلغاء
chat-undo = تراجع
chat-reply-to = الرد على { $names }
chat-send = إرسال (Ctrl+Enter). انقر بزر الماوس الأيمن أو اضغط مطوّلًا لخيارات إضافية
chat-send-now = الإرسال الآن
chat-attach = إرفاق
chat-attach-photo = صورة
chat-attach-file = ملف
chat-attach-library = من «الملفات»
chat-attach-template = نموذج
chat-attach-signature = توقيع
chat-replying-to = الرد على { $name }
chat-reply-newest = الرد على أحدث رسالة

## The attach picker (paperclip > From Files)

picker-title = الإرفاق من «الملفات»
picker-search = البحث في الأسماء والأشخاص والمواضيع
picker-search-drive = البحث في هذا المحرك
picker-mail-files = ملفات البريد
picker-this-chat = هذه المحادثة
picker-this-computer = هذا الكمبيوتر…
picker-in-chat = في هذه المحادثة
picker-recent = الأخيرة
picker-preview = معاينة
picker-cancel = إلغاء
picker-attach = إرفاق
picker-attach-count = إرفاق { $count }
picker-selected = تم تحديد { $count }
picker-of-limit = من { $limit }
picker-in-mail = { $size } في الرسالة
picker-drive-links = { $count ->
    [zero] { $count } كروابط Google Drive
    [one] واحد كرابط Google Drive
    [two] اثنان كرابطَي Google Drive
    [few] { $count } كروابط Google Drive
    [many] { $count } كروابط Google Drive
   *[other] { $count } كروابط Google Drive
}
picker-onedrive-links = { $count ->
    [zero] { $count } كروابط OneDrive
    [one] واحد كرابط OneDrive
    [two] اثنان كرابطَي OneDrive
    [few] { $count } كروابط OneDrive
    [many] { $count } كروابط OneDrive
   *[other] { $count } كروابط OneDrive
}
picker-over = { $size }، أكثر من { $limit } التي تتسع لها الرسالة
picker-getting = { $count ->
    [zero] جارٍ جلب { $count } ملف من المحرك…
    [one] جارٍ جلب الملف من المحرك…
    [two] جارٍ جلب ملفين من المحرك…
    [few] جارٍ جلب { $count } ملفات من المحرك…
    [many] جارٍ جلب { $count } ملفًا من المحرك…
   *[other] جارٍ جلب { $count } ملف من المحرك…
}
picker-some-failed = { $count ->
    [zero] تعذّرت قراءة { $count } ملف
    [one] تعذّرت قراءة ملف واحد
    [two] تعذّرت قراءة ملفين
    [few] تعذّرت قراءة { $count } ملفات
    [many] تعذّرت قراءة { $count } ملفًا
   *[other] تعذّرت قراءة { $count } ملف
}
