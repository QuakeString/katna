# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = خيارات البحث
search-options-close = إغلاق
search-from = من
search-to = إلى
search-subject = الموضوع
search-has-words = يتضمن الكلمات
search-without = لا يتضمن
search-date-within = التاريخ خلال
search-has-attachment = يحتوي على مرفق
search-attachment-custom = مخصص
search-attachment-image = صورة
search-attachment-custom-hint = اكتب امتدادًا، مثل png، ثم اضغط مسافة
search-attachment-remove = إزالة
search-clear-filter = مسح عامل التصفية

## Search options: "Date within" choices

search-within-any = أي وقت
search-within-days = { $count ->
    [zero] { $count } يوم
    [one] يوم واحد
    [two] يومان
    [few] { $count } أيام
    [many] { $count } يومًا
   *[other] { $count } يوم
}
search-within-weeks = { $count ->
    [zero] { $count } أسبوع
    [one] أسبوع واحد
    [two] أسبوعان
    [few] { $count } أسابيع
    [many] { $count } أسبوعًا
   *[other] { $count } أسبوع
}
search-within-months = { $count ->
    [zero] { $count } شهر
    [one] شهر واحد
    [two] شهران
    [few] { $count } أشهر
    [many] { $count } شهرًا
   *[other] { $count } شهر
}
search-within-years = { $count ->
    [zero] { $count } سنة
    [one] سنة واحدة
    [two] سنتان
    [few] { $count } سنوات
    [many] { $count } سنةً
   *[other] { $count } سنة
}
search-within-custom = مخصص

## Search options: custom dates (the calendar popover)

search-dates-on = في
search-dates-before = قبل
search-dates-since = منذ
search-dates-between = بين
search-dates-from = من
search-dates-to = إلى
search-dates-placeholder = سسسس-شش-يي
search-dates-missing = اختر تاريخًا
search-dates-unreadable = استخدم تاريخًا مثل 2026-09-01
search-dates-out-of-range = هذا التاريخ خارج النطاق
search-dates-chip-before = قبل { $date }
search-dates-chip-since = منذ { $date }
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = إلغاء
search-dates-done = تم
search-dates-month-back = الشهر السابق
search-dates-month-on = الشهر التالي
search-dates-year-back = السنة السابقة
search-dates-year-on = السنة التالية
search-server-more = نتائج أخرى على الخادم
search-server-searching = جارٍ البحث في البريد على الخادم…
search-server-empty-searching = لا شيء هنا بعد. جارٍ البحث في البريد على الخادم…
search-server-nothing = لا نتائج أخرى على الخادم
search-server-failed = تعذّر البحث في الخادم.
search-server-again = حاول مجددًا
