# Katna Mail, Urdu (اردو).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } نئی ای میل
   *[other] { $count } نئی ای میلز
}
notify-and-more = اور { $count } مزید
notify-no-subject = (کوئی موضوع نہیں)
notify-unknown-sender = نامعلوم مرسل
notify-snooze-back = اسنوز سے واپس
notify-no-reply = ابھی تک کوئی جواب نہیں
notify-no-reply-to = ”{ $subject }“ کا کسی نے جواب نہیں دیا۔
notify-tracking-opened = { $who } نے { $subject } کھولا
notify-tracking-clicked = { $who } نے { $subject } میں ایک لنک پر کلک کیا

notify-update-ready = Katna Mail کو اپ ڈیٹ کیا جا سکتا ہے
notify-update-ready-body = ورژن { $version } ڈاؤن لوڈ ہو چکا ہے۔ اپ ڈیٹ اسے انسٹال کرتا ہے اور Katna Mail کو دوبارہ شروع کرتا ہے۔
notify-update = اپ ڈیٹ
notify-event-now = ابھی
notify-event-in-minutes = { $count ->
    [one] { $count } منٹ میں
   *[other] { $count } منٹ میں
}
notify-event-in-hours = { $count ->
    [one] { $count } گھنٹے میں
   *[other] { $count } گھنٹے میں
}
notify-event-in-days = { $count ->
    [1] کل
    [one] { $count } دن میں
   *[other] { $count } دن میں
}
notify-event-all-day = پورا دن
notify-event-join = شامل ہوں
notify-event-snooze = 5 منٹ اسنوز کریں

## Its buttons

notify-open = کھولیں
notify-reply-all = سب کو جواب دیں
notify-mark-read = بطور پڑھا ہوا نشان زد کریں
notify-mark-all-read = سب کو بطور پڑھا ہوا نشان زد کریں
notify-archive = آرکائیو کریں
