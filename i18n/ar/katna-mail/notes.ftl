# Katna Mail, Arabic (العربية): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = الملاحظات
notes-view-reminders = التذكيرات
notes-view-archive = الأرشيف
notes-view-trash = سلة المهملات
notes-edit-labels = تعديل التصنيفات
notes-search = البحث في الملاحظات
notes-loading = جارٍ فتح ملاحظاتك…

## Board

notes-take-a-note = تدوين ملاحظة…
notes-new-list = قائمة جديدة
notes-new-note = ملاحظة جديدة
notes-pinned = المثبَّتة
notes-others = أخرى
notes-empty = تظهر الملاحظات التي تضيفها هنا
notes-archive-empty = تظهر الملاحظات المؤرشفة هنا
notes-trash-empty = لا توجد ملاحظات في سلة المهملات
notes-none-found = لا توجد ملاحظات مطابقة
notes-label-empty = لا توجد ملاحظات بهذا التصنيف بعد
notes-reminders-empty = تظهر هنا الملاحظات التي لها تذكيرات قادمة
notes-trash-note = تُحذف الملاحظات في سلة المهملات بعد 7 أيام.
notes-empty-trash = تفريغ سلة المهملات
notes-ticked = { $count ->
    [zero] + { $count } عنصر محدد
    [one] + { $count } عنصر محدد
    [two] + { $count } عنصران محددان
    [few] + { $count } عناصر محددة
    [many] + { $count } عنصرًا محددًا
   *[other] + { $count } عنصر محدد
}
notes-select = تحديد الملاحظة
notes-selected = { $count ->
    [zero] لم يُحدَّد شيء
    [one] تم تحديد ملاحظة واحدة
    [two] تم تحديد ملاحظتين
    [few] تم تحديد { $count } ملاحظات
    [many] تم تحديد { $count } ملاحظة
   *[other] تم تحديد { $count } ملاحظة
}
notes-select-clear = مسح التحديد

## A note's buttons

notes-pin = تثبيت الملاحظة
notes-unpin = إلغاء تثبيت الملاحظة
notes-archive = أرشفة
notes-unarchive = إلغاء الأرشفة
notes-delete = حذف الملاحظة
notes-restore = استعادة
notes-delete-forever = حذف نهائي
notes-color = خيارات الخلفية
notes-checkboxes = إظهار مربّعات الاختيار أو إخفاؤها
notes-labels = التصنيفات
notes-close = إغلاق
notes-more = المزيد
notes-make-copy = إنشاء نسخة
notes-remind = ذكّرني
notes-add-picture = إضافة صورة
notes-history = سجل الإصدارات
notes-ai = ساعدني في الكتابة
notes-send-as-mail = الإرسال كرسالة بريد
notes-save-markdown = الحفظ بتنسيق Markdown
notes-save-pdf = الحفظ بتنسيق PDF

## The open note

notes-title = العنوان
notes-edited = آخر تعديل: { $date }
notes-on-this-computer = على هذا الكمبيوتر
notes-where = مكان حفظ هذه الملاحظة
notes-untitled = ملاحظة بلا عنوان
notes-picture-choose = إضافة صور
notes-picture-remove = إزالة الصورة
notes-picture-too-big = يمكن إضافة صور يصل حجمها إلى { $size } في الملاحظة
notes-picture-kind = هذا الملف ليس صورة يستطيع Katna عرضها
notes-picture-unreadable = تعذّرت قراءة { $name }: { $error }
notes-remind-me = ذكّرني
notes-remind-off = إزالة التذكير
notes-remind-in-the-past = اختر وقتًا لم يمضِ بعد
notes-remind-today = اليوم، { $time }
notes-remind-tomorrow = غدًا، { $time }
notes-remind-weekday = { $day }، { $time }
notes-reminder-set = تم ضبط التذكير على { $when }
notes-reminder-off = تمت إزالة التذكير
notes-link-note = ربط ملاحظة
notes-link-new = ملاحظة جديدة «{ $title }»
notes-linked-from = مرتبطة من
notes-link-gone = لم تعد تلك الملاحظة موجودة
notes-versions = الإصدارات
notes-version-now = الآن
notes-version-here = أنت، على هذا الكمبيوتر
notes-version-yesterday = أمس، { $time }
notes-version-changes = { $count ->
    [zero] لا تغييرات
    [one] تغيير واحد
    [two] تغييران
    [few] { $count } تغييرات
    [many] { $count } تغييرًا
   *[other] { $count } تغيير
}
notes-version-from = من { $device }
notes-version-elsewhere = من جهاز آخر
notes-version-created = تاريخ الإنشاء
notes-version-restore = استعادة هذا الإصدار
notes-version-restored = تمت استعادة الإصدار
notes-history-none = لا توجد إصدارات سابقة بعد
notes-ai-tidy = ترتيب النص
notes-ai-checklist = تحويلها إلى قائمة تحقق
notes-ai-summarise = تلخيص
notes-ai-empty = اكتب شيئًا أولًا
notes-ai-tidied = تم ترتيب النص. يعيده Ctrl+Z كما كان.
notes-ai-listed = تم تحويلها إلى قائمة تحقق. يعيدها Ctrl+Z كما كانت.
notes-ai-summarised = تمت إضافة الملخص في الأعلى

## Labels

notes-label-note = تصنيف الملاحظة
notes-label-name = أدخل اسم التصنيف
notes-label-create = إنشاء «{ $name }»
notes-label-remove = إزالة التصنيف
notes-label-delete = حذف التصنيف
notes-labels-none = لا توجد تصنيفات بعد. أضِف تصنيفًا من زر التصنيفات في الملاحظة.
notes-labels-done = تم
notes-label-renamed = تمت إعادة تسمية التصنيف إلى «{ $name }»
notes-label-deleted = تم حذف التصنيف «{ $name }»

## A note about a mail

notes-mail = البريد
notes-open-mail = فتح الرسالة
notes-open-note = فتح الملاحظة

## Meeting notes

notes-meeting-take = تدوين ملاحظات الاجتماع
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = الحضور: { $names }
notes-meeting-notes = الملاحظات
notes-meeting-actions = بنود العمل
notes-event = حدث
notes-open-event = فتح الحدث

## Formatting

notes-format = التنسيق
notes-format-heading-1 = عنوان 1
notes-format-heading-2 = عنوان 2
notes-format-normal = نص عادي
notes-format-bold = غامق
notes-format-italic = مائل
notes-format-underline = تسطير
notes-format-quote = اقتباس
notes-format-code = تعليمات برمجية
notes-format-divider = فاصل
notes-format-clear = محو التنسيق

## Tasks

notes-make-task = تحويله إلى مهمة

## Colors (tooltips)

notes-color-none = بلا لون
notes-color-coral = مرجاني
notes-color-peach = خوخي
notes-color-sand = رملي
notes-color-mint = نعناعي
notes-color-sage = مريمي
notes-color-fog = ضبابي
notes-color-storm = عاصفي
notes-color-dusk = غسقي
notes-color-blossom = زهري
notes-color-clay = طيني
notes-color-chalk = طباشيري

## Messages at the foot of the window

notes-archived = تمت أرشفة الملاحظة
notes-unarchived = تم إلغاء أرشفة الملاحظة
notes-trashed = تم نقل الملاحظة إلى سلة المهملات
notes-restored = تمت استعادة الملاحظة
notes-saved = تم حفظ الملاحظة
notes-pinned-count = { $count ->
    [zero] لم تُثبَّت أي ملاحظة
    [one] تم تثبيت الملاحظة
    [two] تم تثبيت ملاحظتين
    [few] تم تثبيت { $count } ملاحظات
    [many] تم تثبيت { $count } ملاحظة
   *[other] تم تثبيت { $count } ملاحظة
}
notes-unpinned-count = { $count ->
    [zero] لم يُلغَ تثبيت أي ملاحظة
    [one] تم إلغاء تثبيت الملاحظة
    [two] تم إلغاء تثبيت ملاحظتين
    [few] تم إلغاء تثبيت { $count } ملاحظات
    [many] تم إلغاء تثبيت { $count } ملاحظة
   *[other] تم إلغاء تثبيت { $count } ملاحظة
}
notes-colored-count = { $count ->
    [zero] لم يتغير أي لون
    [one] تم تغيير اللون
    [two] تم تغيير لون ملاحظتين
    [few] تم تغيير لون { $count } ملاحظات
    [many] تم تغيير لون { $count } ملاحظة
   *[other] تم تغيير لون { $count } ملاحظة
}
notes-archived-count = { $count ->
    [zero] لم تُؤرشَف أي ملاحظة
    [one] تمت أرشفة الملاحظة
    [two] تمت أرشفة ملاحظتين
    [few] تمت أرشفة { $count } ملاحظات
    [many] تمت أرشفة { $count } ملاحظة
   *[other] تمت أرشفة { $count } ملاحظة
}
notes-unarchived-count = { $count ->
    [zero] لم يُلغَ أرشفة أي ملاحظة
    [one] تم إلغاء أرشفة الملاحظة
    [two] تم إلغاء أرشفة ملاحظتين
    [few] تم إلغاء أرشفة { $count } ملاحظات
    [many] تم إلغاء أرشفة { $count } ملاحظة
   *[other] تم إلغاء أرشفة { $count } ملاحظة
}
notes-trashed-count = { $count ->
    [zero] لم تُنقَل أي ملاحظة إلى سلة المهملات
    [one] تم نقل الملاحظة إلى سلة المهملات
    [two] تم نقل ملاحظتين إلى سلة المهملات
    [few] تم نقل { $count } ملاحظات إلى سلة المهملات
    [many] تم نقل { $count } ملاحظة إلى سلة المهملات
   *[other] تم نقل { $count } ملاحظة إلى سلة المهملات
}
notes-restored-count = { $count ->
    [zero] لم تُستعَد أي ملاحظة
    [one] تمت استعادة الملاحظة
    [two] تمت استعادة ملاحظتين
    [few] تمت استعادة { $count } ملاحظات
    [many] تمت استعادة { $count } ملاحظة
   *[other] تمت استعادة { $count } ملاحظة
}
notes-copied-count = { $count ->
    [zero] لم تُنشأ أي نسخة
    [one] تم إنشاء نسخة
    [two] تم إنشاء نسختين
    [few] تم إنشاء { $count } نسخ
    [many] تم إنشاء { $count } نسخة
   *[other] تم إنشاء { $count } نسخة
}
notes-empty-discarded = تم تجاهل الملاحظة الفارغة
notes-mail-gone = لم تعد هذه الرسالة موجودة هنا
notes-deleted-forever = { $count ->
    [zero] تم حذف { $count } ملاحظة نهائيًا
    [one] تم حذف الملاحظة نهائيًا
    [two] تم حذف الملاحظتين نهائيًا
    [few] تم حذف { $count } ملاحظات نهائيًا
    [many] تم حذف { $count } ملاحظةً نهائيًا
   *[other] تم حذف { $count } ملاحظة نهائيًا
}
