# Katna Mail, Arabic (العربية): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = الملاحظات
notes-view-archive = الأرشيف
notes-view-trash = سلة المهملات
notes-edit-labels = تعديل التصنيفات
notes-search = البحث في الملاحظات
notes-loading = جارٍ فتح ملاحظاتك…

## Board

notes-take-a-note = تدوين ملاحظة…
notes-new-list = قائمة جديدة
notes-pinned = المثبَّتة
notes-others = أخرى
notes-empty = تظهر الملاحظات التي تضيفها هنا
notes-archive-empty = تظهر الملاحظات المؤرشفة هنا
notes-trash-empty = لا توجد ملاحظات في سلة المهملات
notes-none-found = لا توجد ملاحظات مطابقة
notes-label-empty = لا توجد ملاحظات بهذا التصنيف بعد
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

## The open note

notes-title = العنوان
notes-edited = آخر تعديل: { $date }
notes-on-this-computer = على هذا الكمبيوتر
notes-where = مكان حفظ هذه الملاحظة

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
