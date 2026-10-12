# Katna Mail, Urdu (اردو).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = لیبلز
nav-folders = فولڈرز
nav-label-new = نیا لیبل بنائیں
nav-folder-new = نیا فولڈر بنائیں
nav-menu-check-mail = نئی میل چیک کریں
nav-menu-check-inbox = یہ ان باکس چیک کریں
nav-unified-leave-out = یکجا ان باکس سے باہر رکھیں
nav-unified-bring-back = یکجا ان باکس میں واپس لائیں
nav-menu-sign-in-again = دوبارہ سائن ان کریں
nav-menu-new-mail = اس اکاؤنٹ سے نئی میل
nav-menu-account-settings = اکاؤنٹ کی ترتیبات
nav-account-checked = ہم آہنگ · { $ago } چیک کیا گیا
nav-account-in-sync = ہم آہنگ
nav-account-connecting = منسلک ہو رہا ہے…
nav-account-offline = آف لائن، دوبارہ کوشش ہو رہی ہے
nav-account-signed-out = { $provider } سائن ان کی میعاد ختم ہو گئی
nav-account-password-refused = پاس ورڈ مسترد ہو گیا
nav-account-storage = { $total } میں سے { $used } استعمال شدہ
nav-menu-new-subfolder = اندر نیا فولڈر
nav-menu-new-sublabel = اندر نیا لیبل
nav-menu-rename = نام بدلیں
nav-menu-delete = حذف کریں
nav-menu-empty-trash = کوڑے دان خالی کریں
nav-account-unnamed = اکاؤنٹ { $number }
nav-all-accounts = تمام اکاؤنٹس
nav-expand = فولڈرز دکھائیں
nav-collapse = فولڈرز چھپائیں
storage-used = { $total } میں سے { $percent }% استعمال ہو چکا
storage-used-detail = { $address }: { $total } میں سے { $used } استعمال ہو چکا

## Special folders (the user's own folders keep their names)

folder-inbox = ان باکس
folder-starred = ستارے والی
folder-snoozed = اسنوز شدہ
folder-unread = ناخواندہ
folder-important = اہم
folder-drafts = ڈرافٹس
folder-sent = ارسال کردہ
folder-archive = آرکائیو
folder-spam = سپام
folder-trash = کوڑے دان
folder-all-mail = تمام میل
folder-scheduled = شیڈول کردہ
folder-waiting = جواب کا انتظار
folder-waiting-short = منتظر
folder-reminders = یاد دہانیاں
folder-outbox = آؤٹ باکس
folder-activity = سرگرمی
folder-not-on-account = اس اکاؤنٹ میں ایسا کوئی فولڈر نہیں ہے۔

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = نیا لیبل
label-folder-new-title = نیا فولڈر
label-prompt = براہ کرم نئے لیبل کا نام درج کریں:
label-folder-prompt = براہ کرم نئے فولڈر کا نام درج کریں:
label-name-hint = لیبل کا نام
label-folder-name-hint = فولڈر کا نام
label-nest = لیبل کو اس کے تحت رکھیں:
label-folder-nest = فولڈر کو اس کے تحت رکھیں:
label-cancel = منسوخ کریں
label-create = بنائیں
label-creating = بنایا جا رہا ہے…
label-created = لیبل ”{ $name }“ بنا دیا گیا۔
label-folder-created = فولڈر ”{ $name }“ بنا دیا گیا۔
label-rename-title = لیبل کا نام بدلیں
label-folder-rename-title = فولڈر کا نام بدلیں
label-rename = نام بدلیں
label-renaming = نام بدلا جا رہا ہے…
label-renamed = لیبل کا نام بدل کر ”{ $name }“ کر دیا گیا۔
label-folder-renamed = فولڈر کا نام بدل کر ”{ $name }“ کر دیا گیا۔
folder-delete-title = ”{ $name }“ حذف کریں؟
folder-delete-body = { $count ->
    [0] اس میں کوئی میل نہیں۔ فولڈر سرور سے ہٹا دیا جاتا ہے، اس لیے ویب میل اور آپ کے فون سے بھی یہ ہٹ جاتا ہے۔
   *[other] { $kind ->
        [conversation] { $count ->
            [one] اس کی { $count } گفتگو کوڑے دان میں جاتی ہے، اس لیے آپ اسے اب بھی واپس لا سکتے ہیں۔
           *[other] اس کی { $count } گفتگوئیں کوڑے دان میں جاتی ہیں، اس لیے آپ انہیں اب بھی واپس لا سکتے ہیں۔
        }
       *[message] { $count ->
            [one] اس کا { $count } پیغام کوڑے دان میں جاتا ہے، اس لیے آپ اسے اب بھی واپس لا سکتے ہیں۔
           *[other] اس کے { $count } پیغامات کوڑے دان میں جاتے ہیں، اس لیے آپ انہیں اب بھی واپس لا سکتے ہیں۔
        }
    } فولڈر سرور سے ہٹا دیا جاتا ہے، اس لیے ویب میل اور آپ کے فون سے بھی یہ ہٹ جاتا ہے۔
}
folder-delete-forever-body = { $count ->
    [0] اس میں کوئی میل نہیں۔ فولڈر سرور سے ہٹا دیا جاتا ہے، اس لیے ویب میل اور آپ کے فون سے بھی یہ ہٹ جاتا ہے۔
   *[other] { $kind ->
        [conversation] { $count ->
            [one] اس کی { $count } گفتگو ہمیشہ کے لیے حذف ہو جاتی ہے؛ اس اکاؤنٹ میں کوڑے دان نہیں ہے۔
           *[other] اس کی { $count } گفتگوئیں ہمیشہ کے لیے حذف ہو جاتی ہیں؛ اس اکاؤنٹ میں کوڑے دان نہیں ہے۔
        }
       *[message] { $count ->
            [one] اس کا { $count } پیغام ہمیشہ کے لیے حذف ہو جاتا ہے؛ اس اکاؤنٹ میں کوڑے دان نہیں ہے۔
           *[other] اس کے { $count } پیغامات ہمیشہ کے لیے حذف ہو جاتے ہیں؛ اس اکاؤنٹ میں کوڑے دان نہیں ہے۔
        }
    } فولڈر سرور سے ہٹا دیا جاتا ہے، اس لیے ویب میل اور آپ کے فون سے بھی یہ ہٹ جاتا ہے۔
}
folder-delete-label-body = لیبل ہٹا دیا جاتا ہے۔ اس کی میل ”تمام میل“ اور اپنے دوسرے لیبلز میں رہتی ہے۔
folder-delete-confirm = فولڈر حذف کریں
folder-delete-label-confirm = لیبل حذف کریں
folder-deleted = فولڈر ”{ $name }“ حذف ہو گیا
label-deleted = لیبل ”{ $name }“ حذف ہو گیا
