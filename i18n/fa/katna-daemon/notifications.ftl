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
notify-tracking-opened = { $who } «{ $subject }» را باز کرد
notify-tracking-clicked = { $who } روی پیوندی در «{ $subject }» کلیک کرد

notify-update-ready = Katna Mail را می‌توان به‌روزرسانی کرد
notify-update-ready-body = نسخهٔ { $version } بارگیری شده است. به‌روزرسانی آن را نصب می‌کند و Katna Mail را دوباره راه‌اندازی می‌کند.
notify-update = به‌روزرسانی
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
notify-reply-all = پاسخ به همه
notify-mark-read = علامت‌گذاری به‌عنوان خوانده‌شده
notify-mark-all-read = علامت‌گذاری همه به‌عنوان خوانده‌شده
notify-archive = بایگانی
notify-archived = بایگانی شد
notify-archived-count = { $count ->
    [one] { $count } پیام از صندوق ورودی بیرون برده شد
   *[other] { $count } پیام از صندوق ورودی بیرون برده شد
}
notify-undo = واگرد
notify-reply-sent = پاسخ برای { $name } ارسال شد
notify-open-in-katna = باز کردن در Katna
