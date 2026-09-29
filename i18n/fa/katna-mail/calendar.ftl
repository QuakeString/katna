# Katna Mail, Persian (فارسی): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = امروز
calendar-today-tip = رفتن به امروز
calendar-view-day = روز
calendar-view-week = هفته
calendar-view-month = ماه
calendar-view-schedule = برنامه
calendar-previous-day = روز قبل
calendar-next-day = روز بعد
calendar-previous-week = هفتهٔ قبل
calendar-next-week = هفتهٔ بعد
calendar-previous-month = ماه قبل
calendar-next-month = ماه بعد
calendar-previous-period = قبلی
calendar-next-period = بعدی
calendar-title-months = { $first } – { $last }
calendar-loading = در حال بارگیری…
calendar-read-failed = خواندن تقویم ممکن نشد: { $error }
calendar-local = این رایانه
calendar-account-gone = حساب حذف‌شده
calendar-empty-title = هنوز تقویمی وجود ندارد
calendar-empty-text = تقویم‌های حساب‌های Google و Microsoft شما پس از همگام‌سازی اینجا نمایش داده می‌شوند، همچنین تقویم‌های سرورهای دیگری که از CalDAV پشتیبانی می‌کنند.
calendar-schedule-empty = برای دو ماه آینده چیزی برنامه‌ریزی نشده است.
calendar-no-title = (بدون عنوان)
calendar-all-day = تمام روز
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }، { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } مورد دیگر
calendar-repeats = تکرار
calendar-join = پیوستن
calendar-guests =
    { $count ->
        [one] { $count } مهمان
       *[other] { $count } مهمان
    }
calendar-guest-answers = { $yes } بله، { $maybe } شاید، { $no } خیر، { $waiting } در انتظار
calendar-organizer = برگزارکننده
calendar-optional = اختیاری
calendar-open-web = باز کردن در مرورگر
calendar-close = بستن

## Adding, changing and deleting events.

calendar-add-title = افزودن عنوان
calendar-add-location = افزودن مکان
calendar-add-notes = افزودن توضیحات
calendar-add-guests = افزودن مهمان
calendar-remove-guest = حذف
calendar-add-meet = افزودن ویدیوکنفرانس Google Meet
calendar-add-teams = افزودن جلسه Teams
calendar-has-call = تماس ویدیویی اضافه شد
calendar-weekday-day = { $weekday }، { $day }
calendar-all-day-box = تمام روز
calendar-more-options = گزینه‌های بیشتر
calendar-save = ذخیره
calendar-saved = رویداد ذخیره شد
calendar-deleted = رویداد حذف شد
calendar-discard = رد کردن تغییرات
calendar-edit = ویرایش رویداد
calendar-delete = حذف رویداد
calendar-event-details = جزئیات رویداد
calendar-kind-event = رویداد
calendar-kind-focus = زمان تمرکز
calendar-kind-out-of-office = خارج از دفتر
calendar-kind-working-location = محل کار
calendar-working-home = خانه
calendar-busy = مشغول
calendar-free = آزاد
calendar-cancel = لغو
calendar-ok = تأیید
calendar-read-only = نمی‌توانید رویدادهای این تقویم را تغییر دهید
calendar-none-editable = هنوز تقویمی برای افزودن رویداد ندارید
calendar-no-such-time = این زمان در منطقهٔ زمانی شما وجود ندارد
calendar-end-before-start = رویداد پیش از شروع تمام می‌شود
calendar-repeat-never = تکرار نمی‌شود
calendar-repeat-daily = روزانه
calendar-repeat-weekly = هفتگی در { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] ماهانه در { $weekday } اول
        [2] ماهانه در { $weekday } دوم
        [3] ماهانه در { $weekday } سوم
        [4] ماهانه در { $weekday } چهارم
       *[other] ماهانه در { $weekday } آخر
    }
calendar-repeat-yearly = سالانه در { $day }
calendar-repeat-weekdays = هر روز کاری (دوشنبه تا جمعه)
calendar-repeat-custom = سفارشی
calendar-reminder-none = بدون اعلان
calendar-reminder-at-start = در زمان شروع
calendar-reminder-minutes =
    { $count ->
        [one] { $count } دقیقه قبل
       *[other] { $count } دقیقه قبل
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } ساعت قبل
       *[other] { $count } ساعت قبل
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } روز قبل
       *[other] { $count } روز قبل
    }
calendar-scope-edit-title = ویرایش رویداد تکرارشونده
calendar-scope-delete-title = حذف رویداد تکرارشونده
calendar-scope-this = این رویداد
calendar-scope-following = این رویداد و رویدادهای بعدی
calendar-scope-all = همهٔ رویدادها
calendar-scope-respond-title = پاسخ برای رویداد تکرارشونده
calendar-going = شرکت می‌کنید؟
calendar-answer-yes = بله
calendar-answer-no = خیر
calendar-answer-maybe = شاید
calendar-answered-yes = شرکت می‌کنید
calendar-answered-no = شرکت نمی‌کنید
calendar-answered-maybe = شاید شرکت کنید

## The card at the top of a mail with an invitation.

calendar-invite = دعوت‌نامه
calendar-invite-cancelled = رویداد لغو شد
calendar-invite-reply = { $name } پاسخ داد
calendar-invite-reply-yes = { $name } پذیرفت
calendar-invite-reply-no = { $name } رد کرد
calendar-invite-reply-maybe = { $name } شاید شرکت کند
calendar-invite-organizer = برگزارکننده: { $name }
calendar-invite-open = باز کردن در تقویم
calendar-invite-not-yet = هنوز در تقویم شما نیست. پس از همگام‌سازی می‌توانید پاسخ دهید.
calendar-invite-your-day = روز شما
calendar-invite-clashes =
    { $count ->
        [one] با { $count } رویداد تداخل دارد
       *[other] با { $count } رویداد تداخل دارد
    }

## The day's agenda beside the mail.

agenda-show = نمایش برنامهٔ روز
agenda-hide = پنهان کردن برنامه
agenda-today = امروز، { $date }
agenda-day = { $weekday }، { $date }
agenda-empty = برای این روز چیزی برنامه‌ریزی نشده است.
