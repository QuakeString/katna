# Katna Mail, Urdu (اردو).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Katna کے بارے میں
about-tagline = Linux ڈیسک ٹاپ کے لیے میل اور کیلنڈر
about-whats-new = نیا کیا ہے
about-changelog = تبدیلیوں کی فہرست
about-source = سورس کوڈ
about-coffee = مجھے ایک کافی پلائیں
about-coming-soon = جلد آ رہا ہے
about-follow = مصنف کو فالو کریں
about-love-title = Rust، KDE اور Linux سے محبت کے ساتھ بنایا گیا
about-love-text = Rust ایک تیز اور محفوظ میل ایپ لکھنے کو خوشگوار بنا دیتا ہے: Katna میں کوئی unsafe کوڈ نہیں۔ KDE کے Plasma ڈیسک ٹاپ اور اس کے PIM سوٹ نے Katna کو متاثر کیا، اور Linux اور آزاد سافٹ ویئر کمیونٹی وہ بنیاد بناتی ہیں جس پر یہ کھڑا ہے۔ شکریہ، اور نیچے دی گئی لائبریریوں کا بھی شکریہ۔
about-kde-text = KDE وہ ڈیسک ٹاپ بناتا ہے جس پر Katna سب سے زیادہ گھر جیسا محسوس کرتا ہے، اور اسے رضاکار بناتے ہیں اور آپ جیسے لوگ مالی مدد دیتے ہیں۔ اگر آپ کو Plasma یا KDE کی ایپس پسند ہیں تو براہ کرم KDE کو عطیہ دینے پر غور کریں۔
about-donate-kde = KDE کو عطیہ دیں
about-gpui-title = Zed پروجیکٹ کے GPUI پر بنایا گیا
about-gpui-text = Katna Mail کا پورا انٹرفیس GPUI پر بنا ہے، جو ایک تیز، GPU سے تیز کیا گیا UI فریم ورک ہے جسے Zed Industries نے Zed ایڈیٹر کے لیے بنایا۔ آپ جو بھی پکسل، اینیمیشن اور ونڈو دیکھتے ہیں وہ اسی سے بنتی ہے۔ Zed ٹیم، اسے کھلے عام بنانے کا شکریہ۔ Apache-2.0۔
about-gpui-github = GitHub پر GPUI
about-personal-title = ایک ذاتی پروجیکٹ
about-personal-text = Katna Mail نیا یا انقلابی بننے کی کوشش نہیں کرتا۔ یہ وہ میل ایپ ہے جو اس کا مصنف چاہتا تھا، اور اس کی خصوصیات اور شکل Gmail، Mailspring اور Thunderbird سے لی گئی ہیں۔ یہ صرف اس لیے ممکن ہوا کہ LLMs اتنی ترقی کر چکے ہیں۔
about-built-on = آزاد سافٹ ویئر پر بنایا گیا
about-credit-pimalaya = IMAP، SMTP اور سائن ان (io-imap، io-smtp، io-sasl)
about-credit-imap-codec = IMAP پڑھنا اور لکھنا
about-credit-tantivy = تلاش
about-credit-sqlite = میل کا ذخیرہ
about-credit-rustls = محفوظ کنکشنز
about-credit-mail-parser = میل پڑھنا، Stalwart Labs کی جانب سے
about-credit-html5ever = HTML میل، Servo پروجیکٹ کی جانب سے
about-credit-zbus = D-Bus اور پورٹلز کے ذریعے ڈیسک ٹاپ سے بات چیت
about-credit-oo7 = ڈیسک ٹاپ کی کی رنگ میں پاس ورڈز
about-credit-hayro = PDF دیکھنا اور پرنٹ کرنا
about-credit-calamine = اسپریڈشیٹ کے پیش منظر
about-credit-resvg = SVG تصاویر
about-credit-jiff = تاریخیں اور ٹائم زونز
about-credit-spellbook = املا کی جانچ، Helix ایڈیٹر کی جانب سے
about-credit-smol = ایک وقت میں کئی کام کرنا
about-all-libraries = Katna کی استعمال کردہ تمام لائبریریاں ({ $count })
about-library-authors = از { $authors }
about-license = Katna، GNU GPL ورژن 3 یا اس کے بعد کے تحت آزاد سافٹ ویئر ہے۔
about-close = بند کریں

## What’s new (shown after an update)

whats-new-title = Katna Mail میں نیا کیا ہے
whats-new-updated = ورژن { $version } پر اپ ڈیٹ ہو گیا
whats-new-version = ورژن { $version }
whats-new-more = { $count ->
    [one] اور مکمل تبدیلیوں کی فہرست میں ایک اور۔
   *[other] اور مکمل تبدیلیوں کی فہرست میں { $count } مزید۔
}
whats-new-changelog = تبدیلیوں کی مکمل فہرست
whats-new-got-it = سمجھ گیا

## First run: welcome page

onboarding-welcome-title = Katna Mail میں خوش آمدید
onboarding-welcome-lead = آپ کی میل آپ کے اپنے کمپیوٹر پر: تلاش میں تیز، آف لائن پڑھنے کے قابل اور نجی۔
onboarding-fast-title = تیز، آف لائن بھی
onboarding-fast-text = Katna آپ کی میل کی ایک کاپی یہاں رکھتا ہے، اس لیے کنکشن ہو یا نہ ہو، اسے کھولنا اور تلاش کرنا فوری ہوتا ہے۔
onboarding-providers-title = آپ کی میل کے ساتھ کام کرتا ہے
onboarding-providers-text = Gmail، Outlook، Yahoo، iCloud اور کوئی بھی دوسرا IMAP یا POP اکاؤنٹ۔
onboarding-private-title = نجی
onboarding-private-text = آپ کی میل آپ کے فراہم کنندہ سے سیدھی اس کمپیوٹر پر آتی ہے۔ کوئی Katna سرور اسے نہیں دیکھتا۔
onboarding-get-started = شروع کریں

## First run: adding an account

onboarding-service-checking = Katna کی بیک گراؤنڈ سروس کی جانچ ہو رہی ہے…
onboarding-service-running = Katna کی بیک گراؤنڈ سروس چل رہی ہے۔
onboarding-service-missing = Katna کی بیک گراؤنڈ سروس نہیں چل رہی
onboarding-service-start = یہ آپ کی میل لاتی اور بھیجتی ہے۔ اسے ٹرمینل سے شروع کریں، پھر دوبارہ جانچیں:
onboarding-check-again = دوبارہ جانچیں
onboarding-account-title = اپنا میل اکاؤنٹ شامل کریں
onboarding-account-lead = اپنا ای میل پتہ اور پاس ورڈ ٹائپ کریں، اور Katna سرور کی ترتیبات تلاش کر لے گا۔ Gmail، Yahoo اور iCloud کو ایپ پاس ورڈ درکار ہے، جو آپ کے اکاؤنٹ کی سیکیورٹی ترتیبات میں بنتا ہے۔
onboarding-add-account = اکاؤنٹ شامل کریں
onboarding-back = واپس

## First run: choosing the look

onboarding-look-title = اسے اپنا بنائیں
onboarding-look-lead = چنیں کہ میل کیسے کھلے اور Katna کیسا دکھائی دے۔ آپ انہیں کسی بھی وقت فوری ترتیبات میں بدل سکتے ہیں۔
onboarding-reading-pane = پڑھنے کا پین
onboarding-pane-right = فہرست کے ساتھ
onboarding-pane-none = کوئی تقسیم نہیں
onboarding-theme = تھیم
onboarding-theme-system = ڈیسک ٹاپ جیسی
onboarding-theme-light = ہلکی
onboarding-theme-dark = گہری
onboarding-density = کثافت
onboarding-density-default = ڈیفالٹ
onboarding-density-compact = کمپیکٹ
onboarding-continue = جاری رکھیں

## First run: done

onboarding-ready-title = سب تیار ہے
onboarding-ready-lead = Katna آپ کی میل لا رہا ہے۔ یہ پہنچتے ہی نظر آتی ہے، اور نئی میل خود بخود ظاہر ہوتی ہے۔
onboarding-ready-lead-address = Katna، { $address } کی میل لا رہا ہے۔ یہ پہنچتے ہی نظر آتی ہے، اور نئی میل خود بخود ظاہر ہوتی ہے۔
onboarding-ready-tour = یہ دیکھنے کے لیے کہ سب کچھ کہاں ہے، ایک منٹ کا ٹور کریں؟
onboarding-skip = ابھی کے لیے چھوڑیں
onboarding-take-tour = ٹور کریں

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Katna کو بہتر بنانے میں مدد کریں
share-lead = جب Katna کریش ہوتا ہے تو وہ اس کمپیوٹر پر ایک رپورٹ محفوظ کرتا ہے۔ یہ رپورٹس بھیجنے سے خرابی ٹھیک کرنے میں مدد ملتی ہے۔ آپ اسے کسی بھی وقت ترتیبات > صارف کی رائے میں بدل سکتے ہیں۔
share-sent = کیا بھیجا جاتا ہے
share-sent-detail = کریش رپورٹ جیسی آپ اسے ترتیبات میں دیکھ سکتے ہیں: کیا کریش ہوا اور Katna میں کہاں، ورژن، آپ کا Linux سسٹم اور ڈیسک ٹاپ، اور Katna کی آخری لاگ لائنیں، جن میں میل فولڈرز کے نام ہو سکتے ہیں۔
share-never-sent = کیا کبھی نہیں بھیجا جاتا
share-never-sent-detail = آپ کے پیغامات، رابطے، پاس ورڈز، IP پتہ، صارف نام یا کمپیوٹر کا نام۔ ای میل پتے رپورٹ سے ہٹا دیے جاتے ہیں۔
share-where = یہ کہاں جاتی ہے
share-where-detail = Sentry پر Katna کا کریش ٹریکر، یورپی یونین میں محفوظ۔ کوئی ID رپورٹس کو آپ سے نہیں جوڑتی۔
share-dont-send = نہ بھیجیں
share-send = کریش رپورٹس بھیجیں
share-sending = کریش رپورٹس بھیجی جائیں گی۔ شکریہ۔
share-local = کریش رپورٹس اس کمپیوٹر پر رہتی ہیں۔

## The tour (cards pointing at each part of the window)

tour-welcome-title = Katna Mail میں خوش آمدید
tour-welcome-text = ایک منٹ کا ٹور دکھاتا ہے کہ سب کچھ کہاں ہے۔
tour-not-now = ابھی نہیں
tour-start = ٹور کریں
tour-close = بند کریں
tour-skip = ٹور چھوڑیں
tour-back = واپس
tour-done = ہو گیا
tour-next = اگلا
tour-step = { $total } میں سے { $step }
tour-compose-title = پیغام لکھیں
tour-compose-text = تحریر کریں نیچے دائیں جانب ایک نیا پیغام کھولتا ہے، تاکہ آپ لکھتے ہوئے پڑھتے رہیں۔
tour-search-title = اپنی تمام میل تلاش کریں
tour-search-text = تلاش آف لائن بھی کام کرتی ہے۔ دائیں سرے والا بٹن فلٹرز شامل کرتا ہے: بھیجنے والا، وصول کنندہ، موضوع، تاریخیں اور منسلکات۔
tour-menu-title = فولڈرز دکھائیں یا چھپائیں
tour-menu-text = یہ بٹن فولڈرز کی فہرست کو سمیٹ دیتا ہے۔ جب یہ چھپی ہو تو فولڈرز دیکھنے کے لیے بائیں جانب میل پر پوائنٹر رکھیں۔
tour-apps-title = آپ کی ایپس
tour-apps-text = میل اب یہاں ہے۔ کیلنڈر، رابطے، کام، نوٹس اور فیڈز اس بار میں اس کے ساتھ شامل ہوں گے۔
tour-tabs-title = ان باکس ٹیبز
tour-tabs-text = نئی میل بنیادی، پروموشنز، سوشل، اپ ڈیٹس اور فورمز میں ترتیب دی جاتی ہے۔ آپ ٹیبز کو فوری ترتیبات میں بند کر سکتے ہیں۔
tour-list-title = آپ کے پیغامات
tour-list-text = پیغام پڑھنے کے لیے اس پر کلک کریں۔ فوری کارروائیوں کے لیے اس پر پوائنٹر رکھیں، مزید کے لیے دائیں کلک کریں، یا کئی پر ایک ساتھ کارروائی کے لیے انہیں نشان زد کریں۔
tour-settings-title = فوری ترتیبات
tour-settings-text = پڑھنے کا پین، کثافت اور تھیم یہاں بدلیں۔ ٹور بھی وہیں سے دوبارہ شروع کیا جا سکتا ہے۔
tour-account-title = آپ کا اکاؤنٹ
tour-account-text = دیکھیں کہ آپ کس اکاؤنٹ میں ہیں، اور ایک اور شامل کریں۔

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Katna کی بیک گراؤنڈ سروس غیر متوقع طور پر رک گئی۔
    [one] Katna کی بیک گراؤنڈ سروس غیر متوقع طور پر رک گئی۔ ایک اور کریش رپورٹ محفوظ ہے۔
   *[other] Katna کی بیک گراؤنڈ سروس غیر متوقع طور پر رک گئی۔ { $more } مزید کریش رپورٹس محفوظ ہیں۔
}
crash-mail = { $more ->
    [0] پچھلی بار Katna Mail غیر متوقع طور پر بند ہو گیا۔
    [one] پچھلی بار Katna Mail غیر متوقع طور پر بند ہو گیا۔ ایک اور کریش رپورٹ محفوظ ہے۔
   *[other] پچھلی بار Katna Mail غیر متوقع طور پر بند ہو گیا۔ { $more } مزید کریش رپورٹس محفوظ ہیں۔
}
crash-view = رپورٹ دیکھیں
crash-view-tooltip = اس کمپیوٹر پر محفوظ رپورٹ کھولیں
crash-copy = رپورٹ کاپی کریں
crash-close = بند کریں
