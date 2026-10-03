# Katna Mail, Urdu (اردو).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = میل اکاؤنٹ شامل کریں
add-account-providers-intro = اپنا میل فراہم کنندہ منتخب کریں۔ باقی Katna خود ڈھونڈ لیتا ہے۔
add-account-provider-other = دیگر میل
add-account-provider-other-detail = کوئی بھی IMAP یا POP3 اکاؤنٹ
add-account-provider-google-detail = Gmail اور Google Workspace
add-account-provider-microsoft-detail = Outlook اور Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = { $provider } میں سائن ان کریں
add-account-form-title-other = آپ کا میل اکاؤنٹ
add-account-form-intro = Katna آپ کا پاس ورڈ آپ کے سسٹم کی کی رنگ میں رکھتا ہے۔
add-account-looking = { $address } کے میل سرورز تلاش کیے جا رہے ہیں…
add-account-address-intro = اپنا ای میل پتہ درج کریں۔ Katna آپ کے لیے سرورز تلاش کر لے گا۔
add-account-servers-title = سرور کی ترتیبات
add-account-servers-intro = Katna، { $address } کی میل کہاں سے پڑھتا اور بھیجتا ہے۔
add-account-signing-in = سائن ان ہو رہا ہے…
add-account-browser-title = اپنے براؤزر میں جاری رکھیں
add-account-browser-intro = Katna نے آپ کے براؤزر میں { $provider } کا سائن ان صفحہ کھول دیا ہے۔ وہاں سائن ان کریں اور Katna کو اپنی میل پڑھنے اور بھیجنے کی اجازت دیں، پھر یہاں واپس آئیں۔
add-account-browser-hint = کوئی صفحہ نہیں کھلا؟ اپنے براؤزر کی ونڈوز دیکھیں، یا واپس جا کر دوبارہ کوشش کریں۔
add-account-stage-browser = آپ کے براؤزر میں سائن ان کا انتظار ہے…
add-account-stage-signing-in-at = { $server } پر سائن ان ہو رہا ہے…
add-account-help-app-password-link = ایپ پاس ورڈ کیسے بنائیں
add-account-help-turn-on-imap = { $provider } میل ایپس کو تبھی آنے دیتا ہے جب اس کی ویب میل کی ترتیبات میں IMAP اور POP3 رسائی آن ہو۔
add-account-help-turn-on-imap-link = اسے کیسے آن کریں

## Add a mail account: fields

add-account-field-address = ای میل پتہ
add-account-receive-with = میل وصول کریں بذریعہ
add-account-imap-about = IMAP آپ کی میل اور فولڈرز سرور پر رکھتا ہے، ہر ڈیوائس پر ایک جیسے۔ جب ممکن ہو، یہی منتخب کریں۔
add-account-pop3-about = POP3 آپ کی میل اس کمپیوٹر پر ڈاؤن لوڈ کرتا ہے۔ جو میل آپ یہاں پڑھیں یا منتقل کریں، وہ سرور اور آپ کی دوسری ڈیوائسز پر ویسی ہی رہتی ہے۔
add-account-incoming = آنے والی میل ({ $protocol })
add-account-outgoing = جانے والی میل ({ $protocol })
add-account-field-server = سرور
add-account-field-port = پورٹ
add-account-security-none = کوئی نہیں
add-account-security-none-warning = غیر خفیہ کاری شدہ: آپ کا پاس ورڈ اور میل راستے میں پڑھے جا سکتے ہیں۔
add-account-field-username = صارف نام
add-account-field-password = پاس ورڈ
add-account-show-password = پاس ورڈ دکھائیں
add-account-app-password-hint = یہاں { $provider } کو ایپ پاس ورڈ درکار ہے، وہ نہیں جو آپ ویب پر استعمال کرتے ہیں۔ اپنے { $provider } اکاؤنٹ کی سیکیورٹی ترتیبات میں ایک بنائیں۔
add-account-field-name = آپ کا نام (اختیاری)
add-account-name-hint = ان لوگوں کو دکھایا جاتا ہے جنہیں آپ لکھتے ہیں۔
add-account-servers-pair = { $imap } اور { $smtp }
add-account-servers-found = { $source ->
    [built-in] سرورز: { $servers }، Katna کی فراہم کنندگان کی فہرست میں ملے۔
    [provider] سرورز: { $servers }، آپ کے فراہم کنندہ کی ترتیبات میں ملے۔
    [ispdb] سرورز: { $servers }، Thunderbird کی فراہم کنندگان کی فہرست میں ملے۔
    [dns] سرورز: { $servers }، آپ کے ڈومین کے DNS ریکارڈز میں ملے۔
   *[other] سرورز: { $servers }، اندازے سے؛ اگر سائن ان ناکام ہو تو انہیں جانچیں۔
}
add-account-servers-entered = سرورز: { $servers }، جیسے درج کیے گئے۔
add-account-sign-in-with = { $provider } کے ساتھ سائن ان کریں
add-account-sign-in-instead = اس کی بجائے { $provider } کے ساتھ سائن ان کریں

## Add a mail account: buttons

add-account-servers-button = سرور کی ترتیبات
add-account-back = واپس
add-account-add = اکاؤنٹ شامل کریں
add-account-done = ہو گیا
add-account-another = ایک اور اکاؤنٹ شامل کریں
add-account-cancel = منسوخ کریں

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] آنے والی میل کا سرور درج کریں۔
   *[outgoing] جانے والی میل کا سرور درج کریں۔
}
add-account-server-space = { $kind ->
    [incoming] آنے والی میل کے سرور کے نام میں خالی جگہ ہے۔
   *[outgoing] جانے والی میل کے سرور کے نام میں خالی جگہ ہے۔
}
add-account-port-invalid = { $kind ->
    [incoming] آنے والی میل کا پورٹ { $min } سے { $max } تک کا عدد ہونا چاہیے۔
   *[outgoing] جانے والی میل کا پورٹ { $min } سے { $max } تک کا عدد ہونا چاہیے۔
}
add-account-address-empty = ای میل پتہ درج کریں۔
add-account-address-invalid = { $example } جیسا ای میل پتہ درج کریں۔
add-account-not-found = Katna کو { $address } کے سرورز نہیں ملے، اس لیے اس نے عام نام بھر دیے ہیں۔ اپنے فراہم کنندہ سے ان کی تصدیق کریں۔
add-account-password-empty = پاس ورڈ درج کریں۔
add-account-name-is-password = نام وہی ہے جو پاس ورڈ ہے۔ اس کے بجائے وہاں اپنا نام لکھیں، جیسا لوگوں کو نظر آنا چاہیے۔
add-account-app-password-refused = { $provider } نے پاس ورڈ مسترد کر دیا۔ اسے ایپ پاس ورڈ درکار ہے، وہ نہیں جو آپ ویب پر استعمال کرتے ہیں۔
add-account-password-refused = سرور نے پاس ورڈ مسترد کر دیا۔ اسے جانچیں اور دوبارہ کوشش کریں۔
add-account-sign-in-refused = { $provider } نے Katna کو اندر آنے نہیں دیا۔ دوبارہ کوشش کریں، اور اپنی میل تک رسائی کی اجازت دیں۔
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Katna کی یہ کاپی ابھی Microsoft اکاؤنٹس میں سائن ان نہیں کر سکتی۔
    [Google] Katna کی یہ کاپی ابھی Google اکاؤنٹس میں سائن ان نہیں کر سکتی۔
   *[other] یہ فراہم کنندہ صرف اپنے صفحے پر سائن ان کی اجازت دیتا ہے، جو Katna اس کے لیے ابھی نہیں کر سکتا۔
}
add-account-smtp-not-found = Katna کو یہ تو مل گیا کہ آپ کی میل کہاں سے پڑھنی ہے، لیکن یہ نہیں کہ کہاں سے بھیجنی ہے۔ آؤٹ گوئنگ سرور درج کریں۔
add-account-done-title = آپ کا اکاؤنٹ تیار ہے
add-account-done-intro = Katna ابھی آپ کی میل لا رہا ہے۔ نئی میل آتے ہی دکھائی دیتی ہے۔
add-account-done-sign-in = سائن ان
add-account-done-signed-in-with = { $provider } کے ساتھ، آپ کے براؤزر میں
add-account-done-receiving = میل وصول کرنا
add-account-done-sending = میل بھیجنا
add-account-done-on-server = سرور پر میل
add-account-done-kept = جب تک آپ اسے Katna میں حذف نہ کریں، رکھی جاتی ہے
add-account-done-pop3-hint = سرور پر میل کے ساتھ کیا ہو، یہ ترتیبات > اکاؤنٹس میں بدلیں۔
add-account-done-zoho-title = کام اور کیلنڈرز
add-account-done-zoho-about = Zoho انہیں میل سے الگ رکھتا ہے۔ انہیں Katna میں لانے کے لیے ایک بار Zoho کے ساتھ سائن ان کریں۔
add-account-done-linked = کام اور کیلنڈرز منسلک ہو گئے

## The account menu (from the account button on the top bar)

add-account-menu-another = ایک اور اکاؤنٹ شامل کریں
app-menu = مرکزی مینیو
app-menu-back = واپس
