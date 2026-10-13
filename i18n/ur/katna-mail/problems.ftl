# Katna Mail, Urdu (اردو).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = میل سرور
problems-signed-out = { $provider } نے Katna کو { $address } سے سائن آؤٹ کر دیا۔ میل کی ہم آہنگی رک گئی۔
problems-password-refused = { $provider } نے { $address } کا پاس ورڈ مسترد کر دیا۔ شاید یہ بدل گیا ہے۔
problems-no-answer = { $provider } { $address } کے لیے جواب نہیں دے رہا۔ Katna کوشش جاری رکھتا ہے۔
problems-offline = آپ آف لائن ہیں۔ آپ کی میل اب بھی یہیں ہے، اور آپ جو میل بھیجیں وہ آپ کے واپس آنے تک انتظار کرتی ہے۔
problems-accounts-need-you = { $count ->
    [one] 1 اکاؤنٹ کو آپ کی ضرورت ہے
   *[other] { $count } اکاؤنٹس کو آپ کی ضرورت ہے
}
problems-show = دکھائیں
problems-later = بعد میں
problems-new-password = نیا پاس ورڈ
problems-try-again = دوبارہ کوشش کریں

## The New password card

problems-password-title = نیا پاس ورڈ
problems-password-detail = { $provider } نے { $address } کا محفوظ پاس ورڈ مسترد کر دیا۔ نیا ٹائپ کریں؛ Katna اسے رکھنے سے پہلے چیک کرتا ہے۔
problems-password-placeholder = پاس ورڈ
problems-password-show = پاس ورڈ دکھائیں
problems-password-hide = پاس ورڈ چھپائیں
problems-password-cancel = منسوخ کریں
problems-password-save = محفوظ کریں
problems-password-checking = چیک ہو رہا ہے…
problems-password-refused-again = { $provider } نے یہ پاس ورڈ بھی مسترد کر دیا۔ اسے چیک کریں اور دوبارہ کوشش کریں۔
problems-password-saved = { $address } کا پاس ورڈ محفوظ ہو گیا۔ آپ کی میل لائی جا رہی ہے…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $address } کے میل سرور نے { $count ->
    [one] ایک پیغام کی منتقلی قبول نہیں کی، اس لیے یہ واپس وہیں ہے جہاں تھا۔
   *[other] { $count } پیغامات کی منتقلی قبول نہیں کی، اس لیے وہ واپس وہیں ہیں جہاں تھے۔
}
problems-refused-flags = { $address } کے میل سرور نے { $count ->
    [one] ایک پیغام پر نشان (پڑھا ہوا، ستارہ…) لگانا قبول نہیں کیا، اس لیے یہ پہلے جیسا ہے۔
   *[other] { $count } پیغامات پر نشان (پڑھا ہوا، ستارہ…) لگانا قبول نہیں کیا، اس لیے وہ پہلے جیسے ہیں۔
}
problems-refused-label = { $address } کے میل سرور نے { $count ->
    [one] ایک پیغام کے لیبلز بدلنا قبول نہیں کیا، اس لیے یہ پہلے جیسا ہے۔
   *[other] { $count } پیغامات کے لیبلز بدلنا قبول نہیں کیا، اس لیے وہ پہلے جیسے ہیں۔
}
problems-refused-delete = { $address } کے میل سرور نے { $count ->
    [one] ایک پیغام حذف کرنا قبول نہیں کیا، اس لیے یہ واپس آ گیا ہے۔
   *[other] { $count } پیغامات حذف کرنا قبول نہیں کیا، اس لیے وہ واپس آ گئے ہیں۔
}
problems-refused-other = { $address } کے میل سرور نے { $count ->
    [one] ایک تبدیلی قبول نہیں کی، اس لیے Katna نے اسے پہلے جیسا کر دیا۔
   *[other] { $count } تبدیلیاں قبول نہیں کیں، اس لیے Katna نے انہیں پہلے جیسا کر دیا۔
}
problems-details = تفصیلات

## Katna's background service (katna-daemon) isn't running

service-starting = Katna کی پس منظر سروس شروع ہو رہی ہے…
service-failed = Katna کی پس منظر سروس شروع نہیں ہو رہی، اس لیے میل ہم آہنگ نہیں ہو رہی۔
service-start-again = دوبارہ شروع کریں
service-started-again = Katna کی پس منظر سروس رک گئی تھی اور اسے دوبارہ شروع کیا گیا۔
service-details-title = سروس شروع کیوں نہیں ہو رہی
service-details-body = اسے کاپی کریں اور اپنی رپورٹ کے ساتھ بھیجیں۔ اس میں کوئی میل یا پاس ورڈ نہیں ہے۔
service-details-copy = کاپی کریں
service-details-close = بند کریں
service-not-running = Katna کی پس منظر سروس نہیں چل رہی۔
service-no-answer = Katna کی پس منظر سروس نے جواب نہیں دیا: { $error }
service-no-session = کوئی D-Bus سیشن نہیں: { $error }
safe-line = اپ ڈیٹ میں مسئلے کے بعد Katna سیف موڈ میں ہے، اس لیے میل سنک نہیں ہو رہی۔
safe-try-again = دوبارہ کوشش کریں
safe-restore = بحال کریں
safe-restoring = { $when } سے آپ کا ڈیٹا بحال ہو رہا ہے…
safe-restored = { $when } سے آپ کا ڈیٹا بحال ہو گیا۔ جو پہلے تھا وہ ایک فولڈر میں محفوظ ہے۔
safe-show-folder = فولڈر دکھائیں
safe-restore-failed = آپ کا ڈیٹا بحال نہیں ہو سکا: { $error }
safe-restore-title = اپ ڈیٹ سے پہلے کا ڈیٹا بحال کریں؟
safe-restore-body = Katna آپ کی منتخب کردہ کاپی پر واپس چلا جاتا ہے۔ اس کے بعد آنے والی میل آپ کے اکاؤنٹس سے دوبارہ ڈاؤن لوڈ ہو جاتی ہے۔
safe-restore-none = ابھی کوئی کاپی نہیں ہے۔ ہر اپ ڈیٹ سے پہلے، جو آپ کا ڈیٹا بدلے، Katna ایک کاپی بناتا ہے۔
safe-restore-keep = جو اب موجود ہے، بشمول نہ بھیجی گئی میل، ڈرافٹس اور ابھی سنک نہ ہونے والی تبدیلیاں، پہلے ایک فولڈر میں محفوظ کر لیا جاتا ہے، اس لیے کچھ ضائع نہیں ہوتا۔
safe-restore-cancel = منسوخ کریں
safe-restore-mail = میل
safe-restore-pim = اکاؤنٹس اور رابطے
safe-restore-blobs = اٹیچمنٹس
safe-report-title = ڈیبگ رپورٹ
safe-report-body = اسے کاپی کریں اور اپنی بگ رپورٹ کے ساتھ منسلک کریں۔ اس میں کوئی میل، پتے یا پاس ورڈ نہیں ہیں۔
safe-report-restore = بحال کریں…
safe-report-copied = ڈیبگ رپورٹ کاپی ہو گئی
