# Katna Mail, Persian (فارسی): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = ایجاد
tasks-all = همه کارها
tasks-today = امروز
tasks-starred = ستاره‌دار
tasks-new-list = ایجاد فهرست جدید
tasks-on-this-computer = روی این رایانه
tasks-my-tasks = کارهای من
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = برای نمایش کارها دوباره وارد شوید
tasks-account-signed-in = دوباره به { $address } وارد شدید. در حال دریافت کارهایتان…
tasks-account-sign-in-refused = { $provider } به Katna اجازهٔ ورود نداد. دوباره امتحان کنید و اجازهٔ دسترسی به کارهایتان را بدهید.
tasks-account-refused = سرور گذرواژه را نپذیرفت. Yahoo، iCloud، Zoho و دیگران به گذرواژهٔ برنامه نیاز دارند.
tasks-account-change-password = تغییر گذرواژه
tasks-account-change-password-tooltip = باز کردن تنظیمات > حساب‌ها
tasks-account-not-enabled = دسترسی Katna به کارها هنوز روشن نشده است.
tasks-account-failed = خواندن فهرست‌های کارها ممکن نشد.
# $reason is the server's own words, in English.
tasks-account-error = خواندن فهرست‌های کارها ممکن نشد: { $reason }
tasks-account-none = هیچ فهرست کاری پیدا نشد
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = هیچ فهرست کاری پیدا نشد: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } کارها را فقط به Katna‌ای نشان می‌دهد که با { $provider } وارد شده باشد.
tasks-account-sign-in-with = ورود با { $provider }
tasks-account-looking = در حال جست‌وجوی فهرست‌های کارها…
tasks-account-try-again = امتحان مجدد
tasks-account-try-again-tooltip = همین حالا کارهای این حساب را دوباره بررسی کنید
tasks-account-fixing = در حال انجام…
tasks-list-name-placeholder = نام فهرست

## Lists and tasks

tasks-loading = در حال خواندن کارهای شما…
tasks-no-lists = فهرست کارهای شما اینجا نمایش داده می‌شود.
tasks-search = جستجوی کارها
tasks-search-none = کاری مطابق جستجوی شما پیدا نشد.
tasks-add = افزودن کار
tasks-title-placeholder = عنوان
tasks-add-step = افزودن زیرکار
tasks-empty = هنوز کاری نیست. یکی از بالا اضافه کنید.
tasks-starred-empty = به یک کار ستاره بدهید تا اینجا دیده شود.
tasks-today-empty = امروز کاری سررسید ندارد.
tasks-today-date = { $weekday }، { $day }
tasks-overdue = عقب‌افتاده
tasks-completed = { $count ->
    [one] انجام‌شده ({ $count })
   *[other] انجام‌شده ({ $count })
}
tasks-list-options = گزینه‌های فهرست
tasks-rename-list = تغییر نام فهرست
tasks-delete-list = حذف فهرست
tasks-mark-done = علامت‌گذاری به‌عنوان انجام‌شده
tasks-mark-open = علامت‌گذاری به‌عنوان انجام‌نشده
tasks-star = ستاره‌دار کردن
tasks-unstar = حذف ستاره
tasks-edit-title = ویرایش عنوان
tasks-details = جزئیات
tasks-delete = حذف
tasks-move-to = انتقال به { $list }
tasks-from-mail = ایمیل
tasks-open-mail = باز کردن ایمیل
tasks-from-note = یادداشت
tasks-open-note = باز کردن یادداشت
tasks-note-gone = آن یادداشت دیگر اینجا نیست.
tasks-no-subject = (بدون موضوع)

## The details dialog

tasks-notes-placeholder = افزودن جزئیات
tasks-date = تاریخ
tasks-no-date = بدون تاریخ
tasks-time-placeholder = افزودن زمان
tasks-repeat = تکرار
tasks-repeat-never = تکرار نمی‌شود
tasks-repeat-daily = روزانه
tasks-repeat-weekly = هفتگی
tasks-repeat-monthly = ماهانه
tasks-repeat-yearly = سالانه
tasks-repeat-other = سفارشی
tasks-remind = یادآوری
tasks-remind-off = یادآوری نشود
tasks-remind-on-time = سر وقت
tasks-remind-morning = در همان روز، { $time }
tasks-remind-hour-before = یک ساعت قبل
tasks-remind-day-before = یک روز قبل
tasks-cancel = لغو
tasks-save = ذخیره
tasks-not-a-time = «{ $text }» زمان نیست، مثلاً { $example }.

## Due days

tasks-due-today = امروز
tasks-due-tomorrow = فردا
tasks-due-yesterday = دیروز
tasks-due-at = { $day }، { $time }

## Notes at the bottom

tasks-toast-done = کار انجام شد
tasks-toast-next = انجام شد. نوبت بعدی در { $date }
tasks-toast-deleted = کار حذف شد
tasks-toast-added = { $count ->
    [one] به کارها افزوده شد
   *[other] { $count } کار افزوده شد
}
tasks-mail-gone = آن ایمیل دیگر اینجا نیست.
tasks-toast-list-deleted = فهرست حذف شد
tasks-toast-moved = به { $list } منتقل شد
# A task dragged to another place in its own list.
tasks-toast-placed = کار جابه‌جا شد
tasks-toast-rescheduled = کار دوباره زمان‌بندی شد
