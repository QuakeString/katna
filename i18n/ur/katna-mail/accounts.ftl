# Katna Mail, Urdu (اردو).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = فولڈر پین
accounts-folder-pane-detail = دائیں جانب والا پین کن اکاؤنٹس کے فولڈرز دکھائے۔
accounts-shown-one = ایک وقت میں ایک اکاؤنٹ؛ اکاؤنٹ کارڈ میں تبدیل کریں
accounts-shown-all = تمام اکاؤنٹس، ایک کے بعد ایک
accounts-unified = یکجا ان باکس
accounts-unified-switch = تمام اکاؤنٹس کی میل ایک ساتھ دکھائیں
accounts-unified-switch-detail = ”تمام اکاؤنٹس“ فولڈر پین میں سب سے اوپر ہوتا ہے، جس میں ہر اکاؤنٹ کا ان باکس، بھیجی گئی میل اور بہت کچھ ایک ہی فہرست میں ہوتا ہے۔ اس کے نیچے والے اکاؤنٹس شروع میں سمٹے ہوئے ہوتے ہیں۔
accounts-row = اکاؤنٹس
accounts-row-detail = فولڈر پین اور اکاؤنٹ مینیو اکاؤنٹس کو اسی ترتیب میں دکھاتے ہیں؛ پہلا ڈیفالٹ ہے۔ اکاؤنٹ ہٹانے سے اس کمپیوٹر پر اس کی میل کی Katna والی کاپی حذف ہو جاتی ہے۔ میل سرور پر رہتی ہے۔
accounts-none = ابھی کوئی اکاؤنٹ نہیں۔
accounts-pop3-row = سرور پر میل
accounts-pop3-row-detail = POP3 اکاؤنٹس میل اس کمپیوٹر پر ڈاؤن لوڈ کرتے ہیں۔ منتخب کریں کہ پھر سرور پر موجود کاپی کا کیا ہو۔
accounts-pop3-with-katna = جب تک میں اسے Katna میں حذف نہ کروں، رکھیں
accounts-pop3-at-once = ڈاؤن لوڈ ہوتے ہی حذف کریں
accounts-pop3-after-days = { $count ->
    [one] { $count } دن بعد حذف کریں
   *[other] { $count } دن بعد حذف کریں
}
accounts-pop3-never = کبھی حذف نہ کریں
accounts-pop3-days-less = کم دن
accounts-pop3-days-more = زیادہ دن
accounts-kind-imported = درآمد شدہ
accounts-picture-reset = ڈیسک ٹاپ کی تصویر استعمال کریں
accounts-picture-change = تصویر بدلیں
accounts-picture-remove = تصویر ہٹائیں
accounts-rename = نام بدلیں
accounts-name-save = محفوظ کریں
accounts-name-cancel = منسوخ کریں
accounts-name-placeholder = آپ کا نام
accounts-rename-failed = اکاؤنٹ کا نام نہیں بدلا جا سکا: { $error }
accounts-move-up = اوپر لے جائیں
accounts-move-down = نیچے لے جائیں
accounts-drag = ترتیب بدلنے کے لیے گھسیٹیں
accounts-remove = ہٹائیں
accounts-delete-all-row = تمام ڈیٹا حذف کریں
accounts-delete-all-row-detail = نئے انسٹال کی طرح، دوبارہ شروع کریں۔
accounts-delete-all-about = اس کمپیوٹر سے ہر اکاؤنٹ، تمام محفوظ میل، رابطے اور کیلنڈر، تلاش کا انڈیکس، آپ کی ترتیبات اور محفوظ پاس ورڈز حذف کر دیتا ہے۔ آپ کے میل سرورز پر کچھ نہیں بدلتا۔
accounts-delete-all-open = Katna کا تمام ڈیٹا حذف کریں

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } کو Katna سے ہٹا دیا گیا۔
accounts-removed = { $address } کو Katna سے ہٹا دیا گیا۔ اس کی میل ابھی بھی سرور پر ہے۔
accounts-all-deleted = Katna کا تمام ڈیٹا اس کمپیوٹر سے حذف کر دیا گیا۔

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } کو ہٹائیں؟
accounts-remove-confirm = اکاؤنٹ ہٹائیں
accounts-removing = ہٹایا جا رہا ہے…
accounts-remove-local-mail = { $folders ->
    [0] اس اکاؤنٹ میں درآمد کی گئی تمام میل
    [one] اس اکاؤنٹ میں درآمد کی گئی تمام میل، اس کے فولڈر میں
   *[other] اس اکاؤنٹ میں درآمد کی گئی تمام میل، اس کے { $folders } فولڈرز میں
}
accounts-remove-local-settings = اس کی Katna ترتیبات
accounts-remove-mail = { $folders ->
    [0] Katna میں محفوظ اس اکاؤنٹ کی تمام میل
    [one] Katna میں محفوظ اس اکاؤنٹ کی تمام میل، اس کے فولڈر میں
   *[other] Katna میں محفوظ اس اکاؤنٹ کی تمام میل، اس کے { $folders } فولڈرز میں
}
accounts-remove-outbox = آؤٹ باکس میں منتظر اس کے پیغامات
accounts-remove-settings = اس کا محفوظ پاس ورڈ اور اس کی Katna ترتیبات
accounts-delete-all-title = Katna کا تمام ڈیٹا حذف کریں؟
accounts-delete-all-confirm = سب کچھ حذف کریں
accounts-deleting = حذف کیا جا رہا ہے…
accounts-delete-all-accounts = ہر اکاؤنٹ، اور Katna میں محفوظ تمام میل اور اٹیچمنٹس
accounts-delete-all-contacts = رابطے، کیلنڈر اور تلاش کا انڈیکس
accounts-delete-all-settings = تمام ترتیبات، دستخط اور کی بورڈ شارٹ کٹس
accounts-delete-all-passwords = ہر محفوظ پاس ورڈ
accounts-deleted-heading = اس کمپیوٹر سے حذف ہو گا:
accounts-cannot-undo = اسے کالعدم نہیں کیا جا سکتا۔
accounts-server-delete-all = آپ کے میل سرورز پر کچھ نہیں بدلتا: آپ کی میل وہیں رہتی ہے، اور اکاؤنٹ دوبارہ شامل کرنے سے یہ دوبارہ ڈاؤن لوڈ ہو جاتی ہے۔ فائلوں سے درآمد کی گئی میل صرف Katna میں ہے؛ فائلوں کو نہیں چھیڑا جاتا۔
accounts-server-local = یہ میل فائلوں سے درآمد کی گئی تھی، اس لیے واحد کاپی Katna کے پاس ہے۔ جن فائلوں سے یہ آئی انہیں نہیں چھیڑا جاتا؛ اسے واپس حاصل کرنے کے لیے انہیں دوبارہ درآمد کریں۔
accounts-server-remove = میل سرور پر کچھ نہیں بدلتا: آپ کی میل وہیں رہتی ہے، اور اکاؤنٹ دوبارہ شامل کرنے سے یہ دوبارہ ڈاؤن لوڈ ہو جاتی ہے۔
accounts-confirm-word = حذف
accounts-confirm-placeholder = ”{ accounts-confirm-word }“ ٹائپ کریں
accounts-confirm-prompt = تصدیق کے لیے، ”{ accounts-confirm-word }“ ٹائپ کریں:
accounts-cancel = منسوخ کریں
reset-cache-about = Katna کی ڈاؤن لوڈ کی گئی میل اور اٹیچمنٹس، مرسلین کی تصاویر اور تلاش کا انڈیکس حذف کر دیتا ہے، پھر حالیہ میل دوبارہ ڈاؤن لوڈ کرتا ہے۔ اکاؤنٹس، ترتیبات اور وہ میل جو صرف اس کمپیوٹر پر ہے، برقرار رہتے ہیں۔
reset-cache-button = کیش ری سیٹ کریں
reset-cache-title = کیش ری سیٹ کریں؟
reset-cache-deleted = حذف، پھر دوبارہ ڈاؤن لوڈ ہوتا ہے:
reset-cache-mail = آپ کے IMAP سرورز سے ڈاؤن لوڈ کی گئی میل اور اٹیچمنٹس: حالیہ میل ابھی دوبارہ ڈاؤن لوڈ ہوتی ہے، پرانی میل جب آپ اسے کھولیں
reset-cache-index = تلاش کا انڈیکس، جو فوراً دوبارہ بنایا جاتا ہے
reset-cache-pictures = مرسلین کی تصاویر
reset-cache-kept = برقرار: آپ کے اکاؤنٹس، پاس ورڈز اور ترتیبات؛ ستارے، لیبلز، پڑھے جانے کے نشانات اور پنز؛ ڈرافٹس، آؤٹ باکس اور وہ تبدیلیاں جو ابھی سرور تک نہیں پہنچیں؛ اور POP3 اکاؤنٹس یا درآمد شدہ فائلوں کی میل، جس کی شاید کوئی اور کاپی نہ ہو۔ آپ کے میل سرورز پر کچھ نہیں بدلتا۔
reset-cache-confirm = کیش ری سیٹ کریں
reset-cache-busy = ری سیٹ ہو رہا ہے…
reset-cache-done = کیش ری سیٹ ہو گیا۔ حالیہ میل دوبارہ ڈاؤن لوڈ ہو رہی ہے۔
reset-cache-done-freed = کیش ری سیٹ ہو گیا اور { $size } جگہ خالی ہوئی۔ حالیہ میل دوبارہ ڈاؤن لوڈ ہو رہی ہے۔
