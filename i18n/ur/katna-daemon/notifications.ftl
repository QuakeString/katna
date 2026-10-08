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
notify-follow-up-sent = فالو اپ بھیج دیا گیا
notify-follow-up-sent-to = ”{ $subject }“ کا کسی نے جواب نہیں دیا تھا، اس لیے Katna نے فالو اپ بھیج دیا۔
notify-follow-up-waiting = فالو اپ نہیں بھیجا گیا
notify-follow-up-waiting-to = اس کا وقت تب آیا جب یہ کمپیوٹر بند تھا۔ ”{ $subject }“ واپس آپ کے ان باکس میں ہے۔
notify-tracking-opened = { $who } نے { $subject } کھولا
notify-tracking-clicked = { $who } نے { $subject } میں ایک لنک پر کلک کیا

notify-update-ready = Katna Mail کو اپ ڈیٹ کیا جا سکتا ہے
notify-update-ready-body = ورژن { $version } ڈاؤن لوڈ ہو چکا ہے۔ اپ ڈیٹ اسے انسٹال کرتا ہے اور Katna Mail کو دوبارہ شروع کرتا ہے۔
notify-update = اپ ڈیٹ
notify-signed-out = دوبارہ سائن ان کریں
notify-signed-out-body = { $provider } نے Katna کو { $address } سے سائن آؤٹ کر دیا۔ میل کی ہم آہنگی رک گئی۔
notify-sign-in = سائن ان کریں
notify-password-refused = پاس ورڈ مسترد ہو گیا
notify-password-refused-body = میل سرور نے { $address } کا پاس ورڈ مسترد کر دیا۔ شاید یہ بدل گیا ہے۔
notify-new-password = نیا پاس ورڈ
notify-not-sent = ”{ $subject }“ نہیں بھیجا گیا
notify-not-sent-no-subject = ایک پیغام نہیں بھیجا گیا
notify-not-sent-body = یہ آؤٹ باکس میں ہے، جہاں وجہ بتائی گئی ہے۔
notify-open-outbox = آؤٹ باکس کھولیں
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
notify-task-done = مکمل کے بطور نشان زد کریں

## Its buttons

notify-open = کھولیں
notify-peek = جھلک دیکھیں
notify-reply = جواب دیں
notify-reply-placeholder = { $name } کو جواب دیں…
notify-send = بھیجیں
notify-reply-all = سب کو جواب دیں
notify-mark-read = بطور پڑھا ہوا نشان زد کریں
notify-mark-all-read = سب کو بطور پڑھا ہوا نشان زد کریں
notify-archive = آرکائیو کریں
notify-snooze-hour = 1 گھنٹہ اسنوز کریں
notify-snooze-tomorrow = کل
notify-copy-code = { $code } کاپی کریں
notify-link-verify = { $domain } پر تصدیق کریں
notify-link-confirm = { $domain } پر توثیق کریں
notify-link-activate = { $domain } پر فعال کریں
notify-archived = آرکائیو ہو گئی
notify-archived-count = { $count ->
    [one] { $count } پیغام ان باکس سے باہر منتقل ہو گیا
   *[other] { $count } پیغامات ان باکس سے باہر منتقل ہو گئے
}
notify-undo = کالعدم کریں
notify-code-copied = کوڈ کاپی ہو گیا
notify-code-not-copied = کوڈ کاپی نہیں ہو سکا
notify-reply-sent = جواب { $name } کو بھیج دیا گیا
notify-open-in-katna = Katna میں کھولیں
