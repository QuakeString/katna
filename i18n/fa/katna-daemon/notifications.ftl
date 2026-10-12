# Katna Mail, Persian (فارسی).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count } ایمیل جدید
notify-and-more = و { $count } مورد دیگر
notify-no-subject = (بدون موضوع)
notify-unknown-sender = فرستندهٔ ناشناس
notify-snooze-back = بازگشت از تعویق
notify-no-reply = هنوز پاسخی نیامده
notify-no-reply-to = کسی به «{ $subject }» پاسخ نداده است.
notify-follow-up-sent = پیگیری ارسال شد
notify-follow-up-sent-to = کسی به «{ $subject }» پاسخ نداده بود، پس Katna پیگیری کرد.
notify-follow-up-waiting = پیگیری ارسال نشد
notify-follow-up-waiting-to = زمانش وقتی بود که این رایانه خاموش بود. «{ $subject }» به صندوق ورودی شما برگشت.
notify-tracking-opened = { $who } «{ $subject }» را باز کرد
notify-tracking-clicked = { $who } روی پیوندی در «{ $subject }» کلیک کرد

notify-update-ready = Katna Mail را می‌توان به‌روزرسانی کرد
notify-update-ready-body = نسخهٔ { $version } بارگیری شده است. به‌روزرسانی آن را نصب می‌کند و Katna Mail را دوباره راه‌اندازی می‌کند.
notify-update = به‌روزرسانی
notify-signed-out = دوباره وارد شوید
notify-signed-out-body = { $provider } حساب { $address } را از Katna خارج کرد. همگام‌سازی ایمیل متوقف شد.
notify-sign-in = ورود
notify-password-refused = گذرواژه پذیرفته نشد
notify-password-refused-body = سرور ایمیل گذرواژهٔ { $address } را نپذیرفت. شاید تغییر کرده باشد.
notify-new-password = گذرواژهٔ جدید
notify-not-sent = «{ $subject }» ارسال نشد
notify-not-sent-no-subject = یک پیام ارسال نشد
notify-not-sent-body = در صندوق خروجی است و آنجا دلیلش را می‌گوید.
notify-open-outbox = باز کردن صندوق خروجی
notify-event-now = اکنون
notify-event-in-minutes = { $count ->
    [one] { $count } دقیقه دیگر
   *[other] { $count } دقیقه دیگر
}
notify-event-in-hours = { $count ->
    [one] { $count } ساعت دیگر
   *[other] { $count } ساعت دیگر
}
notify-event-in-days = { $count ->
    [1] فردا
    [one] { $count } روز دیگر
   *[other] { $count } روز دیگر
}
notify-event-all-day = تمام روز
notify-event-join = پیوستن
notify-event-snooze = تعویق 5 دقیقه‌ای
notify-task-done = علامت‌گذاری به‌عنوان انجام‌شده

## Its buttons

notify-open = باز کردن
notify-peek = نگاه سریع
notify-reply = پاسخ
notify-reply-placeholder = پاسخ به { $name }…
notify-send = ارسال
notify-reply-quote-header = در { $date }، { $from } نوشت:
notify-reply-quote-header-no-date = { $from } نوشت:
notify-reply-all = پاسخ به همه
notify-mark-read = علامت‌گذاری به‌عنوان خوانده‌شده
notify-mark-all-read = علامت‌گذاری همه به‌عنوان خوانده‌شده
notify-archive = بایگانی
notify-snooze-hour = تعویق ۱ ساعته
notify-snooze-tomorrow = فردا
notify-copy-code = کپی { $code }
notify-link-verify = تأیید در { $domain }
notify-link-confirm = تأیید در { $domain }
notify-link-activate = فعال‌سازی در { $domain }
notify-archived = بایگانی شد
notify-archived-count = { $count ->
    [one] { $count } پیام از صندوق ورودی بیرون برده شد
   *[other] { $count } پیام از صندوق ورودی بیرون برده شد
}
notify-undo = واگرد
notify-code-copied = کد کپی شد
notify-code-not-copied = کپی کردن کد ممکن نشد
notify-reply-sent = پاسخ برای { $name } ارسال شد
notify-open-in-katna = باز کردن در Katna
