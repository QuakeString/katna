# Katna Mail, Persian (فارسی): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = یادداشت‌ها
notes-view-reminders = یادآورها
notes-view-archive = بایگانی
notes-view-trash = سطل زباله
notes-edit-labels = ویرایش برچسب‌ها
notes-search = جستجوی یادداشت‌ها
notes-loading = در حال باز کردن یادداشت‌های شما…

## Board

notes-take-a-note = یادداشتی بنویسید…
notes-new-list = فهرست جدید
notes-new-note = یادداشت جدید
notes-pinned = سنجاق‌شده
notes-others = سایر
notes-empty = یادداشت‌هایی که اضافه می‌کنید اینجا نشان داده می‌شوند
notes-archive-empty = یادداشت‌های بایگانی‌شده اینجا نشان داده می‌شوند
notes-trash-empty = یادداشتی در سطل زباله نیست
notes-none-found = یادداشت منطبقی پیدا نشد
notes-label-empty = هنوز یادداشتی با این برچسب نیست
notes-reminders-empty = یادداشت‌هایی که یادآور پیش رو دارند اینجا نمایش داده می‌شوند
notes-trash-note = یادداشت‌های سطل زباله بعد از 7 روز حذف می‌شوند.
notes-empty-trash = خالی کردن سطل زباله
notes-ticked = { $count ->
    [one] + { $count } مورد علامت‌خورده
   *[other] + { $count } مورد علامت‌خورده
}
notes-select = انتخاب یادداشت
notes-selected = { $count ->
    [one] { $count } انتخاب‌شده
   *[other] { $count } انتخاب‌شده
}
notes-select-clear = لغو انتخاب

## A note's buttons

notes-pin = سنجاق کردن یادداشت
notes-unpin = برداشتن سنجاق یادداشت
notes-archive = بایگانی
notes-unarchive = خارج کردن از بایگانی
notes-delete = حذف یادداشت
notes-restore = بازیابی
notes-delete-forever = حذف دائمی
notes-color = گزینه‌های پس‌زمینه
notes-checkboxes = نمایش یا پنهان کردن کادرهای تیک
notes-labels = برچسب‌ها
notes-close = بستن
notes-more = بیشتر
notes-make-copy = ساختن رونوشت
notes-remind = یادآوری کن
notes-add-picture = افزودن تصویر
notes-history = تاریخچهٔ نسخه‌ها
notes-ai = کمکم کن بنویسم
notes-send-as-mail = ارسال به‌صورت ایمیل
notes-save-markdown = ذخیره به‌صورت Markdown
notes-save-pdf = ذخیره به‌صورت PDF

## The open note

notes-title = عنوان
notes-edited = آخرین ویرایش: { $date }
notes-on-this-computer = روی این رایانه
notes-where = جای نگهداری این یادداشت
notes-untitled = یادداشت بی‌عنوان
notes-picture-choose = افزودن تصاویر
notes-picture-remove = حذف تصویر
notes-picture-too-big = تصاویر تا { $size } را می‌توان در یادداشت گذاشت
notes-picture-kind = این فایل تصویری نیست که Katna بتواند نمایش دهد
notes-picture-unreadable = خواندن { $name } ممکن نشد: { $error }
notes-remind-me = یادآوری کن
notes-remind-off = حذف یادآور
notes-remind-in-the-past = زمانی را انتخاب کنید که هنوز نگذشته باشد
notes-remind-today = امروز، { $time }
notes-remind-tomorrow = فردا، { $time }
notes-remind-weekday = { $day }، { $time }
notes-reminder-set = یادآور برای { $when } تنظیم شد
notes-reminder-off = یادآور حذف شد
notes-link-note = پیوند به یادداشت
notes-link-new = یادداشت جدید «{ $title }»
notes-linked-from = پیوندشده از
notes-link-gone = آن یادداشت دیگر اینجا نیست
notes-new-note-gone = یادداشت جدید دیگر وجود ندارد.
notes-versions = نسخه‌ها
notes-version-now = اکنون
notes-version-here = شما، روی این رایانه
notes-version-yesterday = دیروز، { $time }
notes-version-changes = { $count ->
    [one] { $count } تغییر
   *[other] { $count } تغییر
}
notes-version-from = از { $device }
notes-version-elsewhere = از دستگاهی دیگر
notes-version-created = ساخته شد
notes-version-restore = بازگرداندن این نسخه
notes-version-restored = نسخه بازگردانده شد
notes-history-none = هنوز نسخهٔ قبلی‌ای نیست
notes-ai-tidy = مرتب کردن متن
notes-ai-checklist = تبدیل به فهرست بررسی
notes-ai-summarise = خلاصه کردن
notes-ai-empty = اول چیزی بنویسید
notes-ai-tidied = متن مرتب شد. Ctrl+Z آن را برمی‌گرداند.
notes-ai-listed = به فهرست بررسی تبدیل شد. Ctrl+Z آن را برمی‌گرداند.
notes-ai-summarised = خلاصه در بالا افزوده شد

## Labels

notes-label-note = برچسب‌گذاری یادداشت
notes-label-name = نام برچسب را وارد کنید
notes-label-create = ایجاد «{ $name }»
notes-label-remove = برداشتن برچسب
notes-label-delete = حذف برچسب
notes-labels-none = هنوز برچسبی وجود ندارد. از دکمهٔ برچسب یادداشت یکی اضافه کنید.
notes-labels-done = تمام
notes-label-renamed = نام برچسب به «{ $name }» تغییر کرد
notes-label-deleted = برچسب «{ $name }» حذف شد

## A note about a mail

notes-mail = ایمیل
notes-open-mail = باز کردن ایمیل
notes-open-note = باز کردن یادداشت

## Meeting notes

notes-meeting-take = یادداشت‌برداری جلسه
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = حاضران: { $names }
notes-meeting-notes = یادداشت‌ها
notes-meeting-actions = اقدام‌ها
notes-event = رویداد
notes-open-event = باز کردن رویداد

## Formatting

notes-format = قالب‌بندی
notes-format-heading-1 = عنوان 1
notes-format-heading-2 = عنوان 2
notes-format-normal = متن عادی
notes-format-bold = پررنگ
notes-format-italic = مورب
notes-format-underline = زیرخط
notes-format-quote = نقل‌قول
notes-format-code = کد
notes-format-divider = جداکننده
notes-format-clear = پاک کردن قالب‌بندی

## Tasks

notes-make-task = تبدیل به کار

## Colors (tooltips)

notes-color-none = بدون رنگ
notes-color-coral = مرجانی
notes-color-peach = هلویی
notes-color-sand = شنی
notes-color-mint = نعنایی
notes-color-sage = مریم‌گلی
notes-color-fog = مه‌آلود
notes-color-storm = طوفانی
notes-color-dusk = غروب
notes-color-blossom = شکوفه
notes-color-clay = خاکی
notes-color-chalk = گچی

## Messages at the foot of the window

notes-archived = یادداشت بایگانی شد
notes-unarchived = یادداشت از بایگانی خارج شد
notes-trashed = یادداشت به سطل زباله منتقل شد
notes-restored = یادداشت بازیابی شد
notes-saved = یادداشت ذخیره شد
notes-pinned-count = { $count ->
    [one] یادداشت سنجاق شد
   *[other] { $count } یادداشت سنجاق شد
}
notes-unpinned-count = { $count ->
    [one] سنجاق یادداشت برداشته شد
   *[other] سنجاق { $count } یادداشت برداشته شد
}
notes-colored-count = { $count ->
    [one] رنگ تغییر کرد
   *[other] رنگ { $count } یادداشت تغییر کرد
}
notes-archived-count = { $count ->
    [one] یادداشت بایگانی شد
   *[other] { $count } یادداشت بایگانی شد
}
notes-unarchived-count = { $count ->
    [one] یادداشت از بایگانی خارج شد
   *[other] { $count } یادداشت از بایگانی خارج شد
}
notes-trashed-count = { $count ->
    [one] یادداشت به سطل زباله رفت
   *[other] { $count } یادداشت به سطل زباله رفت
}
notes-restored-count = { $count ->
    [one] یادداشت بازگردانده شد
   *[other] { $count } یادداشت بازگردانده شد
}
notes-copied-count = { $count ->
    [one] رونوشت ساخته شد
   *[other] { $count } رونوشت ساخته شد
}
notes-empty-discarded = یادداشت خالی کنار گذاشته شد
notes-mail-gone = آن ایمیل دیگر اینجا نیست
notes-deleted-forever = { $count ->
    [one] یادداشت برای همیشه حذف شد
   *[other] { $count } یادداشت برای همیشه حذف شد
}
