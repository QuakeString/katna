# Katna Mail, Persian (فارسی).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = گزینه‌های جستجو
search-options-close = بستن
search-from = از
search-to = به
search-subject = موضوع
search-has-words = دارای این واژه‌ها
search-without = بدون این واژه‌ها
search-date-within = بازهٔ زمانی
search-has-attachment = دارای پیوست
search-attachment-custom = سفارشی
search-attachment-custom-hint = یک پسوند بنویسید، مثلاً png، سپس کلید فاصله را بزنید
search-attachment-remove = حذف
search-clear-filter = پاک کردن فیلتر

## Search options: "Date within" choices

search-within-any = هر زمان
search-within-days = { $count ->
    [one] { $count } روز
   *[other] { $count } روز
}
search-within-weeks = { $count ->
    [one] { $count } هفته
   *[other] { $count } هفته
}
search-within-months = { $count ->
    [one] { $count } ماه
   *[other] { $count } ماه
}
search-within-years = { $count ->
    [one] { $count } سال
   *[other] { $count } سال
}
search-within-custom = سفارشی

## Search options: custom dates (the calendar popover)

search-dates-on = در
search-dates-before = پیش از
search-dates-since = از
search-dates-between = بین
search-dates-from = آغاز
search-dates-to = پایان
search-dates-placeholder = سسسس-مم-رر
search-dates-missing = تاریخی انتخاب کنید
search-dates-unreadable = تاریخی مانند 2026-09-01 به کار ببرید
search-dates-out-of-range = این تاریخ خارج از محدوده است
search-dates-chip-before = پیش از { $date }
search-dates-chip-since = از { $date } به بعد
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = لغو
search-dates-done = تمام
search-dates-month-back = ماه قبل
search-dates-month-on = ماه بعد
search-dates-year-back = سال قبل
search-dates-year-on = سال بعد
