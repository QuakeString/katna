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
