# Katna Mail, Persian (فارسی): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = امروز
calendar-today-tip = رفتن به امروز
calendar-view-day = روز
calendar-view-week = هفته
calendar-view-month = ماه
calendar-view-year = سال
calendar-view-schedule = برنامه
calendar-view-days =
    { $count ->
        [one] { $count } روز
       *[other] { $count } روز
    }
calendar-options = گزینه‌ها
calendar-density = تراکم
calendar-density-responsive = متناسب با صفحه‌نمایش شما
calendar-density-comfortable = راحت
calendar-density-compact = فشرده
calendar-custom-days = نمای سفارشی
calendar-second-zone = منطقه زمانی دوم
calendar-zone-none = هیچ
calendar-zone = { $zone } ({ $offset })
calendar-share-free = هم‌رسانی زمان‌های آزاد
calendar-free-subject = زمان‌های آزاد من
calendar-free-intro = این چند زمان آزاد من است ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = در چند روز کاری آینده زمان آزاد ندارم.
calendar-previous-day = روز قبل
calendar-next-day = روز بعد
calendar-previous-week = هفتهٔ قبل
calendar-next-week = هفتهٔ بعد
calendar-previous-month = ماه قبل
calendar-next-month = ماه بعد
calendar-previous-year = سال قبل
calendar-next-year = سال بعد
calendar-previous-period = قبلی
calendar-next-period = بعدی
calendar-title-months = { $first } – { $last }
calendar-loading = در حال بارگیری…
calendar-read-failed = خواندن تقویم ممکن نشد: { $error }
calendar-sets = مجموعه‌های تقویم
calendar-set-add = ذخیره تقویم‌های نمایش‌داده‌شده به‌عنوان مجموعه
calendar-set-name = نام مجموعه
calendar-set-remove = حذف مجموعه
calendar-local = این رایانه
calendar-account-gone = حساب حذف‌شده
calendar-account-sign-in = برای نمایش تقویم‌ها دوباره وارد شوید
calendar-account-signed-in = دوباره به { $address } وارد شدید. در حال دریافت تقویم‌هایتان…
calendar-account-sign-in-refused = { $provider } به Katna اجازهٔ ورود نداد. دوباره امتحان کنید و اجازهٔ دسترسی به تقویم‌هایتان را بدهید.
calendar-account-refused = سرور گذرواژه را نپذیرفت. Yahoo، iCloud، Zoho و دیگران به گذرواژهٔ برنامه نیاز دارند.
calendar-account-change-password = تغییر گذرواژه
calendar-account-change-password-tooltip = باز کردن تنظیمات > حساب‌ها
calendar-account-not-enabled = دسترسی Katna به تقویم هنوز روشن نشده است.
calendar-account-failed = خواندن تقویم‌ها ممکن نشد.
calendar-account-error = خواندن تقویم‌ها ممکن نشد: { $reason }
calendar-account-none = هیچ تقویمی پیدا نشد
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = هیچ تقویمی پیدا نشد: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } تقویم‌ها را فقط به Katna‌ای نشان می‌دهد که با { $provider } وارد شده باشد.
calendar-account-sign-in-with = ورود با { $provider }
calendar-account-looking = در حال جست‌وجوی تقویم‌ها…
calendar-account-try-again = امتحان مجدد
calendar-account-try-again-tooltip = همین حالا تقویم‌های این حساب را دوباره بررسی کنید
calendar-account-fixing = در حال انجام…
calendar-birthdays = تولدها
calendar-birthday-of = تولد { $name }
calendar-empty-title = هنوز تقویمی وجود ندارد
calendar-empty-text = تقویم‌های حساب‌های Google و Microsoft شما پس از همگام‌سازی اینجا نمایش داده می‌شوند، همچنین تقویم‌های سرورهای دیگری که از CalDAV پشتیبانی می‌کنند.
calendar-schedule-empty = برای دو ماه آینده چیزی برنامه‌ریزی نشده است.
calendar-search = جستجوی رویدادها
calendar-search-past = رویدادهای گذشته
calendar-search-none = رویدادی مطابق جستجوی شما پیدا نشد.
calendar-no-title = (بدون عنوان)
calendar-all-day = تمام روز
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }، { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } مورد دیگر
calendar-peek-day = { $weekday }، { $day }
calendar-repeats = تکرار
calendar-join = پیوستن
calendar-join-with = پیوستن با { $service }
calendar-email-guests = ارسال ایمیل به مهمانان
calendar-running-late = دیر می‌رسم
calendar-late-subject = دیر می‌رسم: { $title }
calendar-late-body = ببخشید، چند دقیقه دیرتر به { $title } می‌رسم. به‌زودی آنجا هستم.
calendar-guests =
    { $count ->
        [one] { $count } مهمان
       *[other] { $count } مهمان
    }
calendar-guest-answers = { $yes } بله، { $maybe } شاید، { $no } خیر، { $waiting } در انتظار
calendar-organizer = برگزارکننده
calendar-optional = اختیاری
calendar-open-web = باز کردن در مرورگر
calendar-open-mail = باز کردن ایمیل
calendar-open-contact = باز کردن مخاطب
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
# Right-click menus on the calendar: on a free time or day, an event and
# a task.
calendar-menu-new-event = رویداد جدید
# Shows the day right-clicked on its own, in the Day view.
calendar-menu-open-day = باز کردن روز
calendar-menu-duplicate = تکثیر
calendar-menu-color = رنگ
# The event takes its calendar's color.
calendar-menu-color-calendar = رنگ تقویم
# A task's new due day, a week from today.
calendar-menu-in-a-week = یک هفته بعد
# Event colors, by the names Google Calendar gives them.
calendar-color-tomato = گوجه‌فرنگی
calendar-color-flamingo = فلامینگو
calendar-color-tangerine = نارنگی
calendar-color-banana = موز
calendar-color-sage = مریم‌گلی
calendar-color-basil = ریحان
calendar-color-peacock = طاووس
calendar-color-blueberry = بلوبری
calendar-color-lavender = اسطوخودوس
calendar-color-grape = انگور
calendar-color-graphite = گرافیت
calendar-menu-only-this = فقط نمایش این
calendar-menu-rename = تغییر نام
calendar-menu-remove = حذف از فهرست
calendar-menu-delete = حذف
calendar-menu-new-calendar = تقویم جدید
calendar-menu-show-all = نمایش همه
calendar-menu-hide-all = پنهان کردن همه
calendar-menu-account-settings = تنظیمات حساب
calendar-why-main = تقویم اصلی
calendar-why-last = تنها تقویم این‌جا
calendar-why-owner = فقط مالک
calendar-why-contacts = از مخاطبین
calendar-why-unreached = دسترسی نشد
calendar-name-placeholder = نام تقویم
calendar-toast-added = «{ $name }» اضافه شد
calendar-toast-renamed = نام تقویم تغییر کرد
calendar-toast-recolored = رنگ تقویم تغییر کرد
calendar-toast-deleted = «{ $name }» حذف شد
calendar-toast-removed = «{ $name }» از فهرست شما حذف شد
calendar-edit-failed = تقویم تغییر نکرد: { $reason }
calendar-delete-title = «{ $name }» حذف شود؟
calendar-delete-confirm = حذف
calendar-deleting = در حال حذف…
calendar-delete-heading = حذف می‌شود:
calendar-delete-events = تقویم و همهٔ رویدادهایش
calendar-delete-shared = برای همهٔ کسانی که با آن‌ها به اشتراک گذاشته شده
calendar-delete-server = از { $account } در سرویس ایمیل حذف می‌شود، نه فقط در Katna.
calendar-delete-local = از این رایانه حذف می‌شود.
calendar-remove-title = «{ $name }» از فهرست شما حذف شود؟
calendar-remove-confirm = حذف
calendar-removing = در حال حذف…
calendar-remove-heading = چه چیزی تغییر می‌کند:
calendar-remove-events = دیگر رویدادهایش را نمی‌بینید، نه این‌جا و نه در برنامه‌های دیگرتان
calendar-remove-server = تقویم نزد مالکش می‌ماند و او می‌تواند دوباره آن را با شما به اشتراک بگذارد.
calendar-kind-event = رویداد
calendar-kind-task = کار
calendar-kind-focus = زمان تمرکز
calendar-kind-out-of-office = خارج از دفتر
calendar-kind-working-location = محل کار
calendar-task-added = کار اضافه شد
calendar-task-added-to = کار به { $list } اضافه شد
calendar-task-list-local = روی این رایانه
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
calendar-invite-by-mail = در تقویم شما نیست: پاسخ شما با ایمیل به برگزارکننده فرستاده می‌شود.
calendar-mail-yes = پذیرفته شد: { $title }
calendar-mail-yes-body = { $name } این دعوت را پذیرفت.
calendar-mail-no = رد شد: { $title }
calendar-mail-no-body = { $name } این دعوت را رد کرد.
calendar-mail-maybe = احتمالی: { $title }
calendar-mail-maybe-body = { $name } این دعوت را به‌صورت احتمالی پذیرفت.
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
