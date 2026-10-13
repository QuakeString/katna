# Katna Mail, Persian (فارسی).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = سرور ایمیل
problems-signed-out = { $provider } حساب { $address } را از Katna خارج کرد. همگام‌سازی ایمیل متوقف شد.
problems-password-refused = { $provider } گذرواژهٔ { $address } را نپذیرفت. شاید تغییر کرده باشد.
problems-no-answer = { $provider } برای { $address } پاسخ نمی‌دهد. Katna به تلاش ادامه می‌دهد.
problems-offline = آفلاین هستید. ایمیل‌هایتان هنوز اینجاست و ایمیلی که ارسال می‌کنید تا برگشتن اتصال منتظر می‌ماند.
problems-accounts-need-you = { $count ->
    [one] ۱ حساب به شما نیاز دارد
   *[other] { $count } حساب به شما نیاز دارند
}
problems-show = نمایش
problems-later = بعداً
problems-new-password = گذرواژهٔ جدید
problems-try-again = تلاش دوباره

## The New password card

problems-password-title = گذرواژهٔ جدید
problems-password-detail = { $provider } گذرواژهٔ ذخیره‌شدهٔ { $address } را نپذیرفت. گذرواژهٔ جدید را تایپ کنید؛ Katna پیش از نگه داشتن آن را بررسی می‌کند.
problems-password-placeholder = گذرواژه
problems-password-show = نمایش گذرواژه
problems-password-hide = پنهان کردن گذرواژه
problems-password-cancel = لغو
problems-password-save = ذخیره
problems-password-checking = در حال بررسی…
problems-password-refused-again = { $provider } این گذرواژه را هم نپذیرفت. آن را بررسی کنید و دوباره امتحان کنید.
problems-password-saved = گذرواژهٔ { $address } ذخیره شد. در حال دریافت ایمیل‌ها…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $count ->
    [one] سرور ایمیل { $address } انتقال یک پیام را نپذیرفت، پس به جای قبلی‌اش برگشت.
   *[other] سرور ایمیل { $address } انتقال { $count } پیام را نپذیرفت، پس به جای قبلی‌شان برگشتند.
}
problems-refused-flags = { $count ->
    [one] سرور ایمیل { $address } علامت‌گذاری یک پیام (خوانده‌شده، ستاره‌دار…) را نپذیرفت، پس به حالت قبلی برگشت.
   *[other] سرور ایمیل { $address } علامت‌گذاری { $count } پیام (خوانده‌شده، ستاره‌دار…) را نپذیرفت، پس به حالت قبلی برگشتند.
}
problems-refused-label = { $count ->
    [one] سرور ایمیل { $address } تغییر برچسب‌های یک پیام را نپذیرفت، پس به حالت قبلی برگشت.
   *[other] سرور ایمیل { $address } تغییر برچسب‌های { $count } پیام را نپذیرفت، پس به حالت قبلی برگشتند.
}
problems-refused-delete = { $count ->
    [one] سرور ایمیل { $address } حذف یک پیام را نپذیرفت، پس برگشت.
   *[other] سرور ایمیل { $address } حذف { $count } پیام را نپذیرفت، پس برگشتند.
}
problems-refused-other = { $count ->
    [one] سرور ایمیل { $address } یک تغییر را نپذیرفت، پس Katna آن را به حالت قبلی برگرداند.
   *[other] سرور ایمیل { $address } { $count } تغییر را نپذیرفت، پس Katna آن‌ها را به حالت قبلی برگرداند.
}
problems-details = جزئیات

## Katna's background service (katna-daemon) isn't running

service-starting = در حال راه‌اندازی سرویس پس‌زمینهٔ Katna…
service-failed = سرویس پس‌زمینهٔ Katna راه‌اندازی نمی‌شود، پس ایمیل همگام نمی‌شود.
service-start-again = راه‌اندازی دوباره
service-started-again = سرویس پس‌زمینهٔ Katna متوقف شد و دوباره راه‌اندازی شد.
service-details-title = چرا سرویس راه‌اندازی نمی‌شود
service-details-body = این را کپی کنید و همراه گزارشتان بفرستید. هیچ ایمیل یا گذرواژه‌ای در آن نیست.
service-details-copy = کپی
service-details-close = بستن
service-not-running = سرویس پس‌زمینهٔ Katna در حال اجرا نیست.
service-no-answer = سرویس پس‌زمینهٔ Katna پاسخ نداد: { $error }
service-no-session = نشست D-Bus وجود ندارد: { $error }
safe-line = Katna پس از مشکلی در به‌روزرسانی در حالت امن است، بنابراین ایمیل همگام‌سازی نمی‌شود.
safe-try-again = تلاش دوباره
safe-restore = بازگردانی
safe-restoring = در حال بازگردانی داده‌هایتان از { $when }…
safe-restored = داده‌هایتان از { $when } بازگردانده شد. آنچه پیش‌تر بود در یک پوشه نگه داشته شده است.
safe-show-folder = نمایش پوشه
safe-restore-failed = بازگردانی داده‌هایتان ممکن نشد: { $error }
safe-restore-title = داده‌هایتان از پیش از یک به‌روزرسانی بازگردانده شود؟
safe-restore-body = Katna به نسخه‌ای که انتخاب می‌کنید برمی‌گردد. ایمیلی که پس از آن رسیده دوباره از حساب‌هایتان بارگیری می‌شود.
safe-restore-none = هنوز هیچ نسخه‌ای نیست. Katna پیش از هر به‌روزرسانی که داده‌هایتان را تغییر می‌دهد یک نسخه می‌سازد.
safe-restore-keep = آنچه اکنون هست، از جمله ایمیل ارسال‌نشده، پیش‌نویس‌ها و تغییرهایی که هنوز همگام نشده‌اند، اول در یک پوشه نگه داشته می‌شود، پس چیزی از دست نمی‌رود.
safe-restore-cancel = لغو
safe-restore-mail = ایمیل
safe-restore-pim = حساب‌ها و مخاطبان
safe-restore-blobs = پیوست‌ها
safe-report-title = گزارش اشکال‌زدایی
safe-report-body = این را کپی کنید و به گزارش اشکال خود پیوست کنید. هیچ ایمیل، نشانی یا گذرواژه‌ای در آن نیست.
safe-report-restore = بازگردانی…
safe-report-copied = گزارش اشکال‌زدایی کپی شد
