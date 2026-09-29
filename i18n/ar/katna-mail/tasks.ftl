# Katna Mail, Arabic (العربية): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = إنشاء
tasks-all = كل المهام
tasks-starred = المميّزة بنجمة
tasks-new-list = إنشاء قائمة جديدة
tasks-on-this-computer = على هذا الكمبيوتر
tasks-my-tasks = مهامي
tasks-list-name-placeholder = اسم القائمة

## Lists and tasks

tasks-loading = جارٍ قراءة مهامك…
tasks-no-lists = تظهر قوائم مهامك هنا.
tasks-add = إضافة مهمة
tasks-title-placeholder = العنوان
tasks-add-step = إضافة مهمة فرعية
tasks-empty = لا توجد مهام بعد. أضف مهمة أعلاه.
tasks-starred-empty = ضع نجمة على مهمة لتظهر هنا.
tasks-completed = { $count ->
    [zero] مكتملة ({ $count })
    [one] مكتملة (مهمة واحدة)
    [two] مكتملة (مهمتان)
    [few] مكتملة ({ $count })
    [many] مكتملة ({ $count })
   *[other] مكتملة ({ $count })
}
tasks-list-options = خيارات القائمة
tasks-rename-list = إعادة تسمية القائمة
tasks-delete-list = حذف القائمة
tasks-mark-done = وضع علامة "مكتملة"
tasks-mark-open = وضع علامة "غير مكتملة"
tasks-star = تمييز بنجمة
tasks-unstar = إزالة النجمة
tasks-edit-title = تعديل العنوان
tasks-details = التفاصيل
tasks-delete = حذف
tasks-move-to = نقل إلى { $list }
tasks-from-mail = البريد
tasks-open-mail = فتح الرسالة
tasks-no-subject = (بلا موضوع)

## The details dialog

tasks-notes-placeholder = إضافة تفاصيل
tasks-date = التاريخ
tasks-no-date = بلا تاريخ
tasks-time-placeholder = إضافة وقت
tasks-repeat = التكرار
tasks-repeat-never = لا تتكرر
tasks-repeat-daily = يوميًا
tasks-repeat-weekly = أسبوعيًا
tasks-repeat-monthly = شهريًا
tasks-repeat-yearly = سنويًا
tasks-repeat-other = مخصّص
tasks-cancel = إلغاء
tasks-save = حفظ
tasks-not-a-time = «{ $text }» ليس وقتًا، مثل { $example }.

## Due days

tasks-due-today = اليوم
tasks-due-tomorrow = غدًا
tasks-due-yesterday = أمس
tasks-due-at = { $day }، { $time }

## Notes at the bottom

tasks-toast-done = اكتملت المهمة
tasks-toast-deleted = تم حذف المهمة
tasks-toast-added = { $count ->
    [zero] تمت إضافة { $count } مهمة
    [one] تمت الإضافة إلى المهام
    [two] تمت إضافة مهمتين
    [few] تمت إضافة { $count } مهام
    [many] تمت إضافة { $count } مهمة
   *[other] تمت إضافة { $count } مهمة
}
tasks-mail-gone = لم تعد هذه الرسالة موجودة هنا.
tasks-toast-list-deleted = تم حذف القائمة
tasks-toast-moved = تم النقل إلى { $list }
