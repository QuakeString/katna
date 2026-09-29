# Katna Mail, Arabic (العربية): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = إنشاء
tasks-all = كل المهام
tasks-today = اليوم
tasks-starred = المميّزة بنجمة
tasks-new-list = إنشاء قائمة جديدة
tasks-on-this-computer = على هذا الكمبيوتر
tasks-my-tasks = مهامي
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = سجّل الدخول مجددًا لإظهار المهام
tasks-account-signed-in = تم تسجيل الدخول إلى { $address } مجددًا. جارٍ جلب مهامك…
tasks-account-sign-in-refused = لم يسمح { $provider } لـ Katna بالدخول. حاول مجددًا، واسمح بالوصول إلى مهامك.
tasks-account-refused = لم يقبل الخادم كلمة المرور. تحتاج Yahoo وiCloud وZoho وغيرها إلى كلمة مرور للتطبيقات.
tasks-account-change-password = تغيير كلمة المرور
tasks-account-change-password-tooltip = فتح الإعدادات > الحسابات
tasks-account-not-enabled = لم يُفعَّل الوصول إلى المهام لـ Katna بعد.
tasks-account-failed = تعذّرت قراءة قوائم المهام.
# $reason is the server's own words, in English.
tasks-account-error = تعذّرت قراءة قوائم المهام: { $reason }
tasks-account-none = لم يُعثر على أي قائمة مهام
tasks-account-looking = جارٍ البحث عن قوائم المهام…
tasks-account-try-again = إعادة المحاولة
tasks-account-try-again-tooltip = التحقق من مهام هذا الحساب مجددًا الآن
tasks-account-fixing = جارٍ العمل على ذلك…
tasks-list-name-placeholder = اسم القائمة

## Lists and tasks

tasks-loading = جارٍ قراءة مهامك…
tasks-no-lists = تظهر قوائم مهامك هنا.
tasks-search = البحث في المهام
tasks-search-none = لا توجد مهام تطابق بحثك.
tasks-add = إضافة مهمة
tasks-title-placeholder = العنوان
tasks-add-step = إضافة مهمة فرعية
tasks-empty = لا توجد مهام بعد. أضف مهمة أعلاه.
tasks-starred-empty = ضع نجمة على مهمة لتظهر هنا.
tasks-today-empty = لا شيء مستحق اليوم.
tasks-today-date = { $weekday }، { $day }
tasks-overdue = متأخرة
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
tasks-from-note = ملاحظة
tasks-open-note = فتح الملاحظة
tasks-note-gone = لم تعد هذه الملاحظة موجودة هنا.
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
tasks-remind = ذكّرني
tasks-remind-off = بدون تذكير
tasks-remind-on-time = في الوقت المحدد
tasks-remind-morning = في اليوم نفسه، { $time }
tasks-remind-hour-before = قبل ساعة
tasks-remind-day-before = قبل يوم
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
tasks-toast-next = تم. المرة التالية في { $date }
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
tasks-toast-rescheduled = أُعيدت جدولة المهمة
