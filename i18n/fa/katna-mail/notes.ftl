# Katna Mail, Persian (فارسی): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = یادداشت‌ها
notes-view-archive = بایگانی
notes-view-trash = سطل زباله
notes-edit-labels = ویرایش برچسب‌ها
notes-search = جستجوی یادداشت‌ها
notes-loading = در حال باز کردن یادداشت‌های شما…

## Board

notes-take-a-note = یادداشتی بنویسید…
notes-new-list = فهرست جدید
notes-pinned = سنجاق‌شده
notes-others = سایر
notes-empty = یادداشت‌هایی که اضافه می‌کنید اینجا نشان داده می‌شوند
notes-archive-empty = یادداشت‌های بایگانی‌شده اینجا نشان داده می‌شوند
notes-trash-empty = یادداشتی در سطل زباله نیست
notes-none-found = یادداشت منطبقی پیدا نشد
notes-label-empty = هنوز یادداشتی با این برچسب نیست
notes-trash-note = یادداشت‌های سطل زباله بعد از 7 روز حذف می‌شوند.
notes-empty-trash = خالی کردن سطل زباله
notes-ticked = { $count ->
    [one] + { $count } مورد علامت‌خورده
   *[other] + { $count } مورد علامت‌خورده
}

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

## The open note

notes-title = عنوان
notes-edited = آخرین ویرایش: { $date }
notes-on-this-computer = روی این رایانه
notes-where = جای نگهداری این یادداشت

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
notes-empty-discarded = یادداشت خالی کنار گذاشته شد
notes-mail-gone = آن ایمیل دیگر اینجا نیست
notes-deleted-forever = { $count ->
    [one] یادداشت برای همیشه حذف شد
   *[other] { $count } یادداشت برای همیشه حذف شد
}
