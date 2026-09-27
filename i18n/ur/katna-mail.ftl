# Katna Mail, Urdu (اردو).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = زبان: { $language }
language-tooltip-system = زبان: { $language }، سسٹم کے مطابق
language-search = زبان تلاش کریں
language-system-default = سسٹم ڈیفالٹ
language-system-now = فی الحال { $language }
language-no-match = کوئی زبان ”{ $query }“ سے مماثل نہیں
language-machine = مشینی ترجمہ۔ اسے بہتر بنانے میں مدد کریں
language-setting = زبان
language-setting-detail = مینیو، بٹنوں اور پیغامات کی زبان، اور تاریخوں اور نمبروں کا فارمیٹ۔ سسٹم ڈیفالٹ ڈیسک ٹاپ کی ترتیب کی پیروی کرتا ہے۔

## Dates and sizes

ago-just-now = ابھی ابھی
ago-minutes = { $count ->
    [one] { $count } منٹ پہلے
   *[other] { $count } منٹ پہلے
}
ago-hours = { $count ->
    [one] { $count } گھنٹہ پہلے
   *[other] { $count } گھنٹے پہلے
}
ago-days = { $count ->
    [one] { $count } دن پہلے
   *[other] { $count } دن پہلے
}
size-bytes = { $count ->
    [one] { $count } بائٹ
   *[other] { $count } بائٹس
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = فولڈرز چھپائیں
folders-show = فولڈرز دکھائیں
compose = تحریر کریں
search = تلاش کریں
search-mail = میل تلاش کریں
search-settings = ترتیبات تلاش کریں
search-clear = تلاش صاف کریں
search-options-show = تلاش کے اختیارات دکھائیں
settings = ترتیبات
account-add = اکاؤنٹ شامل کریں

## App rail (and the bottom bar on a phone)

rail-mail = میل
rail-calendar = کیلنڈر
rail-contacts = رابطے
rail-tasks = کام
rail-notes = نوٹس
rail-feeds = فیڈز

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = جلد آ رہا ہے
app-calendar-promise = آپ کے CalDAV کیلنڈر، آپ کی میل میں آنے والے میٹنگ کے دعوت نامے اور یاد دہانیاں، آپ کے ان باکس کے ساتھ ہی۔
app-tasks-promise = CalDAV کے ساتھ ہم آہنگ ہونے والی کرنے کے کاموں کی فہرستیں، اور میل سے بنائے گئے کام۔
app-notes-promise = فوری نوٹس، اور بعد کے لیے کسی میل یا گفتگو پر نوٹس۔
app-feeds-promise = اپنی میل کے ساتھ ہی RSS اور Atom فیڈز پڑھیں۔

## Contacts page

app-contacts-loading = آپ کی میل سے لوگوں کو جمع کیا جا رہا ہے…
app-contacts-empty = جن لوگوں سے آپ میل پر بات کرتے ہیں وہ یہاں نظر آئیں گے۔
app-contacts-count = { $count ->
    [one] آپ کی میل سے { $count } فرد، سب سے زیادہ میل والے پہلے
   *[other] آپ کی میل سے { $count } لوگ، سب سے زیادہ میل والے پہلے
}
app-contacts-top = { $count ->
    [one] آپ کی میل سے سرفہرست { $count } فرد، سب سے زیادہ میل والے پہلے
   *[other] آپ کی میل سے سرفہرست { $count } لوگ، سب سے زیادہ میل والے پہلے
}
app-contacts-messages = { $count ->
    [one] { $count } پیغام
   *[other] { $count } پیغامات
}
app-contacts-last = آخری بار { $date }

## Navigation (the folders pane)

nav-labels = لیبلز
nav-folders = فولڈرز
nav-label-new = نیا لیبل بنائیں
nav-folder-new = نیا فولڈر بنائیں
nav-account-unnamed = اکاؤنٹ { $number }
nav-tab-new = { $count ->
    [one] { $count } نیا
   *[other] { $count } نئے
}

## Special folders (the user's own folders keep their names)

folder-inbox = ان باکس
folder-starred = ستارے والی
folder-drafts = ڈرافٹس
folder-sent = ارسال کردہ
folder-archive = آرکائیو
folder-spam = سپام
folder-trash = کوڑے دان
folder-all-mail = تمام میل
folder-scheduled = شیڈول کردہ

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

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = بنیادی
tab-promotions = پروموشنز
tab-social = سوشل
tab-updates = اپ ڈیٹس
tab-forums = فورمز
tab-focused = مرکوز
tab-other = دیگر
tab-inbox = ان باکس
tab-newsletters = نیوز لیٹرز
tab-notifications = اطلاعات
tab-new = { $count } نئے
tab-provider-other = Katna کی ترتیب

## Mail list: toolbar

list-select = منتخب کریں
list-refresh = ریفریش کریں
list-more = مزید
list-mark-read = بطور پڑھا ہوا نشان زد کریں
list-mark-unread = بطور ناخواندہ نشان زد کریں
list-move-to = یہاں منتقل کریں
list-archive = آرکائیو کریں
list-spam = سپام کی اطلاع دیں
list-delete = حذف کریں
list-newer = نئی
list-older = پرانی
list-range = { $total } میں سے { $first }–{ $last }
list-range-about = تقریباً { $total } میں سے { $first }–{ $last }
list-results = ”{ $query }“ کے نتائج
list-results-corrected = ”{ $query }“ کے نتائج دکھائے جا رہے ہیں
list-search-instead = اس کے بجائے ”{ $query }“ تلاش کریں
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = تمام
list-pick-none = کوئی نہیں
list-pick-read = پڑھی ہوئی
list-pick-unread = ناخواندہ
list-pick-starred = ستارے والی
list-pick-unstarred = بغیر ستارے والی

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } گفتگو منتخب ہے۔
       *[other] تمام { $count } گفتگوئیں منتخب ہیں۔
    }
   *[message] { $count ->
        [one] { $count } پیغام منتخب ہے۔
       *[other] تمام { $count } پیغامات منتخب ہیں۔
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } میں { $count } گفتگو منتخب ہے۔
       *[other] { $folder } میں تمام { $count } گفتگوئیں منتخب ہیں۔
    }
   *[message] { $count ->
        [one] { $folder } میں { $count } پیغام منتخب ہے۔
       *[other] { $folder } میں تمام { $count } پیغامات منتخب ہیں۔
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] اسکرین پر { $count } گفتگو منتخب ہے۔
       *[other] اسکرین پر تمام { $count } گفتگوئیں منتخب ہیں۔
    }
   *[message] { $count ->
        [one] اسکرین پر { $count } پیغام منتخب ہے۔
       *[other] اسکرین پر تمام { $count } پیغامات منتخب ہیں۔
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } گفتگو منتخب کریں
       *[other] تمام { $count } گفتگوئیں منتخب کریں
    }
   *[message] { $count ->
        [one] { $count } پیغام منتخب کریں
       *[other] تمام { $count } پیغامات منتخب کریں
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } میں { $count } گفتگو منتخب کریں
       *[other] { $folder } میں تمام { $count } گفتگوئیں منتخب کریں
    }
   *[message] { $count ->
        [one] { $folder } میں { $count } پیغام منتخب کریں
       *[other] { $folder } میں تمام { $count } پیغامات منتخب کریں
    }
}
list-clear-selection = انتخاب صاف کریں

## Mail list: empty states

list-empty-search = کوئی پیغام آپ کی تلاش سے مماثل نہیں۔
list-empty-tab = { $tab } میں کوئی میل نہیں۔
list-empty-tab-unknown = اس ٹیب میں کوئی میل نہیں۔
list-empty-folder = { $folder } میں کوئی پیغام نہیں۔
list-empty-folder-unknown = اس فولڈر میں کوئی پیغام نہیں۔
list-first-sync = آپ کی میل حاصل کی جا رہی ہے…
list-first-sync-detail = جیسے جیسے یہ آئے گی، یہاں نظر آئے گی۔

## Mail list: lines

row-removed = یہ پیغام ہٹا دیا گیا۔
row-starred = ستارے والا
row-not-starred = ستارہ نہیں لگا
row-important = اہم۔ غیر اہم کے بطور نشان زد کرنے کے لیے کلک کریں۔
row-mark-important = بطور اہم نشان زد کریں
row-pinned = سب سے اوپر پن کیا گیا
row-pin = سب سے اوپر پن کریں
row-unpin = پن ہٹائیں

## Mail list: More menu and right-click menu

menu-reply = جواب دیں
menu-reply-all = سب کو جواب دیں
menu-forward = آگے بھیجیں
menu-archive = آرکائیو کریں
menu-delete = حذف کریں
menu-spam = سپام کی اطلاع دیں
menu-mark-read = بطور پڑھا ہوا نشان زد کریں
menu-mark-unread = بطور ناخواندہ نشان زد کریں
menu-mark-all-read = سب کو بطور پڑھا ہوا نشان زد کریں
menu-star = ستارہ لگائیں
menu-unstar = ستارہ ہٹائیں
menu-important = بطور اہم نشان زد کریں
menu-not-important = بطور غیر اہم نشان زد کریں
menu-pin = سب سے اوپر پن کریں
menu-unpin = پن ہٹائیں
menu-print-all = سب پرنٹ کریں
menu-new-window = نئی ونڈو میں کھولیں
menu-move-to = یہاں منتقل کریں
menu-move-to-heading = یہاں منتقل کریں:
menu-find-from = { $name } کی ای میلز تلاش کریں

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] گفتگو آرکائیو کر دی گئی۔
       *[other] { $count } گفتگوئیں آرکائیو کر دی گئیں۔
    }
   *[message] { $count ->
        [one] پیغام آرکائیو کر دیا گیا۔
       *[other] { $count } پیغامات آرکائیو کر دیے گئے۔
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] گفتگو کوڑے دان میں منتقل کر دی گئی۔
       *[other] { $count } گفتگوئیں کوڑے دان میں منتقل کر دی گئیں۔
    }
   *[message] { $count ->
        [one] پیغام کوڑے دان میں منتقل کر دیا گیا۔
       *[other] { $count } پیغامات کوڑے دان میں منتقل کر دیے گئے۔
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] گفتگو منتقل کر دی گئی۔
       *[other] { $count } گفتگوئیں منتقل کر دی گئیں۔
    }
   *[message] { $count ->
        [one] پیغام منتقل کر دیا گیا۔
       *[other] { $count } پیغامات منتقل کر دیے گئے۔
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] گفتگو پر ستارہ لگا دیا گیا۔
       *[other] { $count } گفتگوئیں پر ستارہ لگا دیا گیا۔
    }
   *[message] { $count ->
        [one] پیغام پر ستارہ لگا دیا گیا۔
       *[other] { $count } پیغامات پر ستارہ لگا دیا گیا۔
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] گفتگو سے ستارہ ہٹا دیا گیا۔
       *[other] { $count } گفتگوئیں سے ستارہ ہٹا دیا گیا۔
    }
   *[message] { $count ->
        [one] پیغام سے ستارہ ہٹا دیا گیا۔
       *[other] { $count } پیغامات سے ستارہ ہٹا دیا گیا۔
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] گفتگو بطور اہم نشان زد کر دی گئی۔
       *[other] { $count } گفتگوئیں بطور اہم نشان زد کر دی گئیں۔
    }
   *[message] { $count ->
        [one] پیغام بطور اہم نشان زد کر دیا گیا۔
       *[other] { $count } پیغامات بطور اہم نشان زد کر دیے گئے۔
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] گفتگو بطور غیر اہم نشان زد کر دی گئی۔
       *[other] { $count } گفتگوئیں بطور غیر اہم نشان زد کر دی گئیں۔
    }
   *[message] { $count ->
        [one] پیغام بطور غیر اہم نشان زد کر دیا گیا۔
       *[other] { $count } پیغامات بطور غیر اہم نشان زد کر دیے گئے۔
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] گفتگو سب سے اوپر پن کر دی گئی۔
       *[other] { $count } گفتگوئیں سب سے اوپر پن کر دی گئیں۔
    }
   *[message] { $count ->
        [one] پیغام سب سے اوپر پن کر دیا گیا۔
       *[other] { $count } پیغامات سب سے اوپر پن کر دیے گئے۔
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] گفتگو سے پن ہٹا دیا گیا۔
       *[other] { $count } گفتگوئیں سے پن ہٹا دیا گیا۔
    }
   *[message] { $count ->
        [one] پیغام سے پن ہٹا دیا گیا۔
       *[other] { $count } پیغامات سے پن ہٹا دیا گیا۔
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] گفتگو کی بطور سپام اطلاع دے دی گئی۔
       *[other] { $count } گفتگوئیں کی بطور سپام اطلاع دے دی گئی۔
    }
   *[message] { $count ->
        [one] پیغام کی بطور سپام اطلاع دے دی گئی۔
       *[other] { $count } پیغامات کی بطور سپام اطلاع دے دی گئی۔
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] گفتگو ہمیشہ کے لیے حذف کر دی گئی۔
       *[other] { $count } گفتگوئیں ہمیشہ کے لیے حذف کر دی گئیں۔
    }
   *[message] { $count ->
        [one] پیغام ہمیشہ کے لیے حذف کر دیا گیا۔
       *[other] { $count } پیغامات ہمیشہ کے لیے حذف کر دیے گئے۔
    }
}
toast-undone = کارروائی کالعدم کر دی گئی۔
toast-undo = کالعدم کریں
toast-no-spam-folder = اس اکاؤنٹ میں کوئی سپام فولڈر نہیں ہے۔

## Reading pane: toolbar

reader-close = بند کریں
reader-back = واپس
reader-mark-unread = بطور ناخواندہ نشان زد کریں
reader-move-to = یہاں منتقل کریں
reader-more = مزید
reader-print-all = سب پرنٹ کریں
reader-new-window = نئی ونڈو میں
reader-position = { $total } میں سے { $position }
reader-newer = نئی
reader-older = پرانی

## Reading pane: the conversation

reader-removed = یہ گفتگو ہٹا دی گئی۔
reader-no-subject = (کوئی موضوع نہیں)
reader-collapse-all = سب سکیڑیں
reader-expand-all = سب پھیلائیں
reader-unknown-sender = (نامعلوم مرسل)
reader-date-ago = { $date } ({ $ago })
reader-me = میں
reader-to = بنام { $names }
reader-starred = ستارے والا
reader-not-starred = ستارہ نہیں لگا
reader-too-long = پیغام پورا دکھانے کے لیے بہت طویل ہے۔
reader-encrypted-images = مرموز میل میں ویب سے تصاویر کبھی لوڈ نہیں کی جاتیں۔
reader-window-failed = نئی ونڈو نہیں کھل سکی۔

## Reading pane: message details (opened from "to me")

reader-details-from = منجانب:
reader-details-to = بنام:
reader-details-cc = cc:
reader-details-date = تاریخ:
reader-details-subject = موضوع:

## Reading pane: downloading a message

reader-downloading = یہ پیغام سرور سے ڈاؤن لوڈ کیا جا رہا ہے…
reader-download-failed = یہ پیغام ڈاؤن لوڈ نہیں ہو سکا۔
reader-try-again = دوبارہ کوشش کریں

## Reply row

reply-reply = جواب دیں
reply-reply-all = سب کو جواب دیں
reply-forward = آگے بھیجیں

## Encrypted and signed mail

security-decrypting = رمز کشائی ہو رہی ہے…
security-checking = دستخط کی جانچ ہو رہی ہے…
security-partly-encrypted = اس پیغام کا صرف ایک حصہ مرموز ہے۔ باقی حصہ تحفظ کے باہر شامل کیا گیا تھا اور کسی کی طرف سے بھی آ سکتا ہے۔
security-partly-signed = اس پیغام کے صرف ایک حصے پر دستخط ہیں۔ باقی حصہ تحفظ کے باہر شامل کیا گیا تھا اور کسی کی طرف سے بھی آ سکتا ہے۔
security-encrypted = مرموز پیغام
security-encrypted-smime = مرموز پیغام (S/MIME)
security-no-key = اس پیغام کی رمز کشائی نہیں ہو سکتی: یہ ایسی کلید کے لیے مرموز کیا گیا تھا جو آپ کے پاس نہیں ہے۔
security-cancelled = رمز کشائی منسوخ کر دی گئی۔
security-damaged = اس پیغام کی رمز کشائی نہیں ہو سکتی: مرموز ڈیٹا خراب ہے یا تبدیل کیا گیا تھا۔
security-decrypt-unavailable = اس پیغام کی رمز کشائی نہیں ہو سکتی: مرموز میل پڑھنے کے لیے { $tool } انسٹال کریں۔
security-decrypt-failed = اس پیغام کی رمز کشائی نہیں ہو سکتی: { $reason }
security-unknown-signer = ایک نامعلوم دستخط کنندہ
security-signed-verified = { $signer } کے دستخط شدہ · تصدیق شدہ
security-signed-not-sender = { $signer } کے دستخط شدہ، جو مرسل نہیں ہے
security-signed-untrusted = { $signer } کے دستخط شدہ، ایسی کلید سے جسے آپ نے ناقابل اعتماد نشان زد کیا ہے
security-signed-unverified = { $signer } کے دستخط شدہ · کلید کی تصدیق نہیں ہوئی
security-bad-signature = غلط دستخط: یہ پیغام دستخط کے بعد تبدیل کیا گیا، یا دستخط جعلی ہیں۔
security-signature-expired = { $signer } کے دستخط شدہ · دستخط کی میعاد ختم ہو چکی ہے
security-key-expired = { $signer } کے دستخط شدہ · تب سے کلید کی میعاد ختم ہو چکی ہے
security-key-revoked = { $signer } کے دستخط شدہ، ایسی کلید سے جو منسوخ ہو چکی ہے
security-missing-key = ایسی کلید سے دستخط شدہ جو آپ کے پاس نہیں ہے، اس لیے جانچ نہیں ہو سکتی
security-missing-key-id = ایسی کلید ({ $key }) سے دستخط شدہ جو آپ کے پاس نہیں ہے، اس لیے جانچ نہیں ہو سکتی
security-signature-unavailable = دستخط شدہ؛ دستخط کی جانچ کے لیے { $tool } انسٹال کریں
security-signature-error = دستخط کی جانچ نہیں ہو سکی۔

## Remote images and pictures

remote-hidden = اس پیغام میں تصاویر چھپی ہوئی ہیں۔
remote-show = تصاویر دکھائیں
remote-always-show = اس مرسل سے ہمیشہ دکھائیں
remote-picture-use = استعمال کریں
remote-picture-too-big = 8 MB یا اس سے کم کی تصویر منتخب کریں۔
remote-picture-type = PNG، JPEG، GIF، WebP یا SVG تصویر منتخب کریں۔
remote-picture-read-failed = تصویر پڑھی نہیں جا سکتی: { $error }
remote-picture-keep-failed = تصویر محفوظ نہیں رکھی جا سکتی: { $error }
remote-picture-remove-failed = تصویر ہٹائی نہیں جا سکتی: { $error }

## Attachments

attachment-count = { $count ->
    [one] ایک اٹیچمنٹ
   *[other] { $count } اٹیچمنٹس
}
attachment-save = محفوظ کریں
attachment-save-all = سب محفوظ کریں
attachment-save-all-tooltip = تمام اٹیچمنٹس ایک فولڈر میں محفوظ کریں
attachment-save-here = یہاں محفوظ کریں
attachment-not-downloaded = یہ پیغام ڈاؤن لوڈ نہیں ہوا۔
attachment-not-found = یہ اٹیچمنٹ پیغام میں نہیں مل سکی۔
attachment-read-failed = { $name } پڑھی نہیں جا سکی
attachment-numbered = اٹیچمنٹ { $number }
attachment-saved-all = { $count ->
    [one] { $count } فائل { $place } میں محفوظ کی گئی
   *[other] { $count } فائلیں { $place } میں محفوظ کی گئیں
}
attachment-saved-some = { $total ->
    [one] { $total } فائل میں سے { $saved } { $place } میں محفوظ کی گئی۔ { $failed } محفوظ نہیں ہو سکی
   *[other] { $total } فائلوں میں سے { $saved } { $place } میں محفوظ کی گئیں۔ { $failed } محفوظ نہیں ہو سکی
}
attachment-saved-to = { $path } میں محفوظ کی گئی
attachment-save-failed = { $name } محفوظ نہیں ہو سکی: { $error }
attachment-open-failed = { $name } نہیں کھل سکی: { $error }
attachment-risky = یہ فائل کوئی پروگرام چلا سکتی ہے، اس لیے Katna اسے نہیں کھولتا۔ اس کے بجائے اسے محفوظ کریں۔
attachment-encrypted-open = یہ فائل مرموز حالت میں آئی تھی۔ اسے کہیں اور کھولنے کے لیے محفوظ کریں۔

## Printing

print-failed = پرنٹ نہیں ہو سکا: { $error }
print-no-font = کوئی فونٹ نہیں ملا
print-opened-as-pdf = وہاں سے پرنٹ کرنے کے لیے PDF کے طور پر کھولا گیا۔
print-not-downloaded = (ابھی تک ڈاؤن لوڈ نہیں ہوا۔)
print-encrypted = (مرموز۔ اس کا متن پرنٹ کرنے کے لیے اسے Katna Mail میں کھولیں۔)
print-to = بنام: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = اس کی اٹیچمنٹس پڑھنے کے لیے یہ پیغام کھولیں۔
text-copy = کاپی کریں
text-select-all = سب منتخب کریں

## Settings page: its tabs

settings-tab-general = عمومی
settings-tab-inbox = ان باکس
settings-tab-accounts = اکاؤنٹس
settings-tab-subscriptions = سبسکرپشنز
settings-tab-appearance = ظاہری شکل
settings-tab-shortcuts = شارٹ کٹس
settings-tab-default-apps = ڈیفالٹ ایپس
settings-tab-folders-rules = فولڈرز اور اصول
settings-tab-compose = تحریر
settings-tab-mcp-server = MCP سرور
settings-tab-feedback = صارف کی رائے
settings-tab-experimental = تجرباتی

## Settings page: tabs still to come

settings-tab-subscriptions-coming = آپ کو ملنے والے نیوز لیٹرز اور میلنگ لسٹس دیکھیں، اور ایک کلک میں ان کی سبسکرپشن ختم کریں۔
settings-tab-folders-rules-coming = فولڈرز اور لیبلز بنائیں، ان کا نام بدلیں، انہیں منتقل کریں اور چھپائیں، اور منتخب کریں کہ کون سے ہم آہنگ ہوں۔ اصول نئی میل کو مرسل، موضوع یا الفاظ کی بنیاد پر خود بخود ترتیب دیتے ہیں، لیبل لگاتے ہیں، آگے بھیجتے ہیں یا حذف کرتے ہیں۔
settings-tab-mcp-server-coming = اس کمپیوٹر پر موجود AI اسسٹنٹس کو آپ کی اجازت سے آپ کی میل تلاش کرنے، پڑھنے اور اس کے ڈرافٹ لکھنے دیں۔

## Settings > General

settings-general-conversations = گفتگو کا منظر
settings-general-conversations-group = ایک ہی میل کے جوابات کو گروپ کریں
settings-general-conversations-group-detail = فہرست میں ہر گفتگو کے لیے ایک لائن
settings-general-reading = پڑھنا
settings-general-newest-first = سب سے نیا پیغام پہلے
settings-general-newest-first-detail = گفتگو اپنے تازہ ترین جواب سے شروع ہوتی ہے
settings-general-full-headers = مکمل ہیڈرز دکھائیں
settings-general-full-headers-detail = ہر پیغام پر منجانب، بنام، cc، تاریخ اور موضوع کھلے رہتے ہیں
settings-general-full-names = وصول کنندگان کے پورے نام
settings-general-full-names-detail = ”بنام میں، Ada Lovelace“، نہ کہ ”بنام میں، Ada“
settings-general-mark-read = بطور پڑھا ہوا نشان زد کریں
settings-general-mark-read-now = کھلتے ہی
settings-general-mark-read-1s = 1 سیکنڈ کھلا رہنے کے بعد
settings-general-mark-read-3s = 3 سیکنڈ کھلا رہنے کے بعد
settings-general-mark-read-never = صرف جب میں اسے بطور پڑھا ہوا نشان زد کروں
settings-general-reply-button = جواب کا بٹن
settings-general-reply-all = سب کو جواب دیں
settings-general-reply-all-detail = ہر پیغام کے ساتھ موجود جواب کا بٹن سب کو جواب دیتا ہے، صرف مرسل کو نہیں
settings-general-remote-images = ویب سے تصاویر
settings-general-remote-images-detail = کسی پیغام کی تصاویر لوڈ کرنے سے اس کے مرسل کو پتا چل جاتا ہے کہ آپ نے اسے کھولا، کب، اور تقریباً کہاں سے۔ بند ہونے پر، ہر پیغام پہلے پوچھتا ہے، اور آپ کسی مرسل کی تصاویر ہمیشہ دکھا سکتے ہیں۔
settings-general-remote-images-always = ہمیشہ تصاویر دکھائیں
settings-general-remote-images-always-detail = ہر پیغام میں، صرف ان مرسلین سے نہیں جن پر آپ کو بھروسا ہے
settings-general-sending = بھیجنا
settings-general-sending-detail = بھیجا گیا پیغام کتنی دیر انتظار کرے، تاکہ اسے واپس لیا جا سکے۔
settings-general-offline = آف لائن میل
settings-general-offline-detail = حالیہ میل پوری ڈاؤن لوڈ کی جاتی ہے، تاکہ کنکشن کے بغیر پڑھی جا سکے۔ پرانی میل کھولنے پر ڈاؤن لوڈ ہوتی ہے۔
settings-general-offline-days = { $count ->
    [one] { $count } دن
   *[other] { $count } دن
}
settings-general-offline-years = { $count ->
    [one] { $count } سال
   *[other] { $count } سال
}
settings-general-offline-all = تمام میل
settings-general-offline-note = کم دن منتخب کرنے سے پہلے سے ڈاؤن لوڈ شدہ میل برقرار رہتی ہے۔ سرور پر کچھ نہیں بدلتا۔
settings-general-notifications = اطلاعات
settings-general-notifications-detail = ان باکس میں نئی میل کے لیے، اس وقت بھی جب Katna Mail بند ہو۔
settings-general-new-mail = نئی میل کی اطلاع دیں
settings-general-new-mail-detail = ”سب کو جواب دیں“، ”بطور پڑھا ہوا نشان زد کریں“ اور ”آرکائیو کریں“ کے ساتھ
settings-general-new-mail-sound = آواز چلائیں
settings-general-new-mail-sound-detail = ڈیسک ٹاپ کی نئی میل کی آواز
settings-general-desktop = ڈیسک ٹاپ
settings-general-open-at-login = لاگ ان پر Katna Mail کھولیں
settings-general-open-at-login-detail = دونوں صورتوں میں لاگ ان پر میل ہم آہنگ ہوتی ہے، جب تک سروس چل رہی ہو
settings-general-tray = Katna کو سسٹم ٹرے میں دکھائیں
settings-general-tray-detail = ناخواندہ تعداد اور ایک مینیو کے ساتھ
settings-general-unread-badge = ٹاسک بار آئیکن پر ناخواندہ تعداد
settings-general-unread-badge-detail = ان باکس کے کتنے پیغامات ناخواندہ ہیں

## Settings > Inbox

settings-inbox-tabs = ان باکس ٹیبز
settings-inbox-tabs-detail = ان باکس کو ٹیبز میں ترتیب دیں، جیسے آپ کے میل فراہم کنندہ کی ویب سائٹ کرتی ہے۔
settings-inbox-tabs-show = ان باکس ٹیبز دکھائیں
settings-inbox-tabs-show-detail = بند ہونے پر ہر اکاؤنٹ کے لیے ایک فہرست دکھائی دیتی ہے
settings-inbox-no-accounts = ٹیبز منتخب کرنے کے لیے ایک اکاؤنٹ شامل کریں۔
settings-inbox-tabs-automatic = خودکار: { $tabs } ({ $provider })
settings-inbox-tabs-off = کوئی ٹیب نہیں
settings-inbox-tabs-gmail = بنیادی، پروموشنز، سوشل، اپ ڈیٹس، فورمز
settings-inbox-tabs-focused = مرکوز اور دیگر
settings-inbox-tabs-zoho = ان باکس، نیوز لیٹرز اور اطلاعات
settings-inbox-tabs-shown = دکھائے گئے ٹیبز۔ جس ٹیب کو آپ بند کریں اس کی میل { $tab } میں رہتی ہے۔

## Settings > Appearance

settings-appearance-reading-pane = پڑھنے کا پین
settings-appearance-reading-pane-detail = کھلی گفتگو کہاں دکھائی دیتی ہے۔
settings-appearance-pane-right = فہرست کے ساتھ
settings-appearance-pane-none = کوئی تقسیم نہیں
settings-appearance-density = کثافت
settings-appearance-density-default = ڈیفالٹ
settings-appearance-density-compact = کمپیکٹ
settings-appearance-scaling = اسکیلنگ
settings-appearance-scaling-detail = Katna Mail میں ہر چیز کو ڈیسک ٹاپ کے اپنے اسکیل کے علاوہ بڑا یا چھوٹا کرتا ہے: متن، آئیکنز، فاصلہ اور تقسیم کار لائنیں۔ آپ کی بھیجی گئی میل اپنا فونٹ سائز برقرار رکھتی ہے۔ بہت چھوٹے سائز آئیکنز پر کلک کرنا مشکل بنا سکتے ہیں۔
settings-appearance-theme = تھیم
settings-appearance-theme-system = ڈیسک ٹاپ جیسی
settings-appearance-theme-light = ہلکی
settings-appearance-theme-dark = گہری
settings-appearance-desktop-colors = ڈیسک ٹاپ کے رنگ
settings-appearance-desktop-colors-use = ڈیسک ٹاپ کے رنگ استعمال کریں
settings-appearance-desktop-colors-use-detail = ڈیسک ٹاپ کی رنگ سکیم اور ایکسنٹ رنگ
settings-appearance-app-names = ایپس کے نام
settings-appearance-app-names-show = ایپس کے نام دکھائیں
settings-appearance-app-names-show-detail = بالکل دائیں جانب ایپ آئیکنز کے نیچے نام
settings-appearance-sender-pictures = مرسلین کی تصاویر
settings-appearance-sender-pictures-show = کمپنی کے لوگو دکھائیں
settings-appearance-sender-pictures-show-detail = مرسل کے ڈومین سے تلاش کیے جاتے ہیں، کبھی پیغام سے نہیں، اور ایک ہفتے تک رکھے جاتے ہیں
settings-appearance-important = اہم کے نشانات
settings-appearance-important-show = اہم کے نشانات دکھائیں
settings-appearance-important-show-detail = فہرست میں ہر پیغام کے ساتھ
settings-appearance-message-width = پیغام کی چوڑائی
settings-appearance-message-width-limit = پیغامات کی چوڑائی محدود کریں
settings-appearance-message-width-limit-detail = چوڑی ونڈو میں لمبی لائنیں پڑھنا آسان ہوتا ہے
settings-appearance-mail-colors = میل کے رنگ
settings-appearance-mail-colors-detail = زیادہ تر میل سفید صفحے کے لیے ڈیزائن کی جاتی ہے۔ گہری تھیم میں اس کے رنگ ایسے گہرے رنگوں میں بدل دیے جاتے ہیں جو اچھی طرح پڑھے جا سکیں؛ بند ہونے پر، یہ ہلکے صفحے پر اپنے مرسل کے رنگ برقرار رکھتی ہے۔
settings-appearance-dark-mail = میل کے لیے بھی گہرے رنگ
settings-appearance-dark-mail-detail = صرف جب تھیم گہری ہو
settings-appearance-attachment-previews = اٹیچمنٹ کے پیش منظر
settings-appearance-attachment-previews-show = اٹیچمنٹس کے پیش منظر دکھائیں
settings-appearance-attachment-previews-show-detail = ہر فائل کے کارڈ پر اس کے مواد کی ایک چھوٹی تصویر

## Settings > Default apps

settings-default-apps-intro = کلک کرنے پر اٹیچمنٹس کہاں کھلتی ہیں۔ ویوئر ہمیشہ کسی فائل کو کسی اور ایپ میں بھی کھول سکتا ہے۔ ڈیسک ٹاپ کی ڈیفالٹ ایپس اس کی اپنی ترتیبات میں سیٹ ہوتی ہیں۔
settings-default-apps-pdf = PDF فائلیں
settings-default-apps-pdf-detail = صفحات، زوم کے ساتھ۔
settings-default-apps-pictures = تصاویر
settings-default-apps-pictures-detail = فوٹوز (سیدھی کی گئی)، PNG، GIF، WebP، BMP، TIFF اور SVG۔
settings-default-apps-text = ٹیکسٹ فائلیں
settings-default-apps-text-detail = سادہ متن، لاگز، کوڈ اور دیگر متن۔
settings-default-apps-sheets = اسپریڈشیٹس
settings-default-apps-sheets-detail = Excel (xlsx، xls)، OpenDocument (ods) اور CSV۔
settings-default-apps-documents = دستاویزات
settings-default-apps-documents-detail = Word (docx) اور OpenDocument متن (odt)۔
settings-default-apps-katna = Katna Mail کا ویوئر
settings-default-apps-system = ڈیسک ٹاپ کی ڈیفالٹ ایپ
settings-default-apps-ask = ہر بار پوچھیں کہ کون سی ایپ
settings-default-apps-after-saving = محفوظ کرنے کے بعد
settings-default-apps-show-folder = محفوظ کردہ فائلیں ان کے فولڈر میں دکھائیں
settings-default-apps-show-folder-detail = محفوظ کردہ اٹیچمنٹس کو منتخب کر کے فائل مینیجر کھولتا ہے

## Settings > Compose

settings-compose-send-from = نئے پیغامات اس سے بھیجیں
settings-compose-send-from-detail = جوابات اور فارورڈز ہمیشہ اسی اکاؤنٹ سے جاتے ہیں جس میں آپ ہیں۔
settings-compose-send-from-current = وہ اکاؤنٹ جس میں آپ ہیں
settings-compose-send-on-replies = جوابات پر بھیجیں
settings-compose-send-on-replies-detail = جواب یا فارورڈ پر ”بھیجیں“ کیا کرتا ہے۔ ”بھیجیں“ کے ساتھ والا مینیو دوسرا اختیار دیتا ہے۔
settings-compose-send-plain = بھیجیں
settings-compose-send-archive = بھیجیں اور آرکائیو کریں
settings-compose-signatures = دستخط
settings-compose-signatures-detail = آپ کے پیغام کے نیچے، ”--“ لائن کے بعد شامل کیا جاتا ہے۔ تحریر کی ونڈو میں کوئی اور منتخب کریں۔
settings-compose-untitled = بلا عنوان
settings-compose-signature-name = نام، جیسے کام
settings-compose-signature-first = میرا دستخط
settings-compose-signature-numbered = دستخط { $number }
settings-compose-signature-delete = حذف کریں
settings-compose-signature-deleted = دستخط حذف کر دیا گیا
settings-compose-signature-new = نیا بنائیں
settings-compose-no-signatures = ابھی کوئی دستخط نہیں۔
settings-compose-no-signature = کوئی دستخط نہیں
settings-compose-for-new-mail = نئی میل کے لیے
settings-compose-for-replies = جوابات اور فارورڈز کے لیے
settings-compose-for-replies-detail = جس گفتگو میں آپ نے کسی پیغام پر دستخط کیا ہو، اس میں جواب اسی دستخط سے شروع ہوتا ہے۔
settings-compose-format = فارمیٹ
settings-compose-plain-text = سادہ متن میں لکھیں
settings-compose-plain-text-detail = نئی میل فارمیٹنگ کے بغیر شروع ہوتی ہے؛ تحریر کی ونڈو میں بدلا جا سکتا ہے
settings-compose-spelling = املا
settings-compose-spell-check = لکھتے وقت املا کی جانچ کریں
settings-compose-spell-check-detail = غلط املا والے الفاظ کے نیچے لکیر لگتی ہے، رائٹ کلک پر تجاویز کے ساتھ
settings-compose-spell-desktop = ڈیسک ٹاپ کی زبان ({ $language })
settings-compose-templates = ٹیمپلیٹس
settings-compose-templates-detail = اکثر لکھی جانے والی میل محفوظ کریں، اور اس سے نئی میل یا جواب شروع کریں۔

## Settings > Shortcuts

settings-shortcuts-set = شارٹ کٹ سیٹ
settings-shortcuts-set-detail = کسی ایسی میل ایپ کی کلیدوں سے شروع کریں جسے آپ جانتے ہیں۔ یہاں Cmd سے مراد Ctrl ہے۔ آپ کی اپنی تبدیلیاں سیٹ کے اوپر برقرار رہتی ہیں، اور ”ڈیفالٹس بحال کریں“ سیٹ کی کلیدوں پر واپس لے جاتا ہے۔
settings-shortcuts-single = ایک کلید والے شارٹ کٹس
settings-shortcuts-single-detail = Ctrl یا Alt کے بغیر کلیدیں، جیسے ویب میل میں: e آرکائیو کرتی ہے، j اور k حرکت دیتی ہیں، / تلاش کرتی ہے۔ یہ فہرست اور کھلی گفتگو میں کام کرتی ہیں، ٹائپ کرتے وقت کبھی نہیں۔
settings-shortcuts-single-use = ایک کلید والے شارٹ کٹس استعمال کریں
settings-shortcuts-single-use-detail = Ctrl شارٹ کٹس ہمیشہ کام کرتے ہیں
settings-shortcuts-how = کسی کلید کو بدلنے کے لیے اس پر کلک کریں، یا شامل کرنے کے لیے + پر، پھر نئی کلیدیں دبائیں۔ Esc منسوخ کرتا ہے۔
settings-shortcuts-restore = ڈیفالٹس بحال کریں
settings-shortcuts-no-key = کوئی کلید نہیں
settings-shortcuts-press = کلیدیں دبائیں…
settings-shortcuts-then = { $keys } پھر…
settings-shortcuts-moved = { $keys } اب ”{ $previous }“ کے بجائے ”{ $action }“ کرتی ہے۔
settings-shortcuts-single-off = ایک کلید والے شارٹ کٹس بند ہیں، اس لیے یہ کلید ان کے آن ہونے پر کام کرے گی۔
settings-shortcuts-restored = ہر شارٹ کٹ کی کلیدیں دوبارہ اس کے سیٹ والی ہیں۔

## Settings search: the line under a result

settings-general-language-summary = ایپ، تاریخوں اور نمبروں کی زبان
settings-general-reading-summary = سب سے نیا پیغام پہلے، مکمل ہیڈرز، وصول کنندگان کے پورے نام
settings-general-mark-read-summary = کھلی گفتگو کب پڑھی ہوئی نشان زد ہو: فوراً، 1 یا 3 سیکنڈ بعد، یا دستی طور پر
settings-general-reply-button-summary = ہر پیغام کے ساتھ موجود جواب کا بٹن سب کو جواب دیتا ہے
settings-general-remote-images-summary = ہر پیغام کی تصاویر ہمیشہ دکھائیں
settings-general-sending-summary = بھیجنا کالعدم کریں: بھیجا گیا پیغام کتنی دیر انتظار کرے، تاکہ اسے واپس لیا جا سکے
settings-general-offline-summary = حالیہ میل کے کتنے دن پورے ڈاؤن لوڈ کیے جائیں، تاکہ کنکشن کے بغیر پڑھے جا سکیں
settings-general-notifications-summary = نئی میل کی اطلاعات اور ان کی آواز
settings-general-desktop-summary = لاگ ان پر Katna Mail کھولیں، سسٹم ٹرے آئیکن اور ٹاسک بار آئیکن پر ناخواندہ تعداد
settings-accounts-accounts-summary = اکاؤنٹ شامل کریں یا ہٹائیں، یا اس کی تصویر بدلیں
settings-appearance-density-summary = فہرست میں ڈیفالٹ یا کمپیکٹ لائنیں
settings-appearance-scaling-summary = ہر چیز بڑی یا چھوٹی کریں: متن، آئیکنز، فاصلہ اور تقسیم کار لائنیں
settings-appearance-theme-summary = ڈیسک ٹاپ جیسی، ہلکی یا گہری
settings-appearance-sender-pictures-summary = کمپنی کے لوگو، مرسل کے ڈومین سے تلاش کیے گئے
settings-appearance-important-summary = فہرست میں ہر پیغام کے ساتھ اہم کا نشان
settings-appearance-mail-colors-summary = گہری تھیم میں HTML میل کے لیے گہرے رنگ، یا اس کے مرسل کے رنگ
settings-appearance-attachment-previews-summary = ہر اٹیچمنٹ کے مواد کی ایک چھوٹی تصویر
settings-shortcuts-set-summary = Gmail، Inbox by Gmail، Apple Mail، Outlook یا Thunderbird کی کلیدوں سے شروع کریں
settings-shortcuts-single-summary = Ctrl یا Alt کے بغیر کلیدیں، جیسے ویب میل میں
settings-default-apps-pdf-summary = PDF اٹیچمنٹس کہاں کھلتی ہیں
settings-default-apps-pictures-summary = فوٹوز اور تصاویر کہاں کھلتی ہیں
settings-default-apps-text-summary = سادہ متن، لاگز اور کوڈ کہاں کھلتے ہیں
settings-default-apps-sheets-summary = Excel، OpenDocument اور CSV فائلیں کہاں کھلتی ہیں
settings-default-apps-documents-summary = Word اور OpenDocument متن کہاں کھلتے ہیں
settings-default-apps-after-saving-summary = محفوظ کردہ اٹیچمنٹس ان کے فولڈر میں دکھائیں
settings-compose-send-from-summary = وہ اکاؤنٹ جس سے نئی میل جاتی ہے: جس میں آپ ہیں، یا ہمیشہ ایک ہی
settings-compose-send-on-replies-summary = جوابات اور فارورڈز پر ”بھیجیں“، یا ”بھیجیں اور آرکائیو کریں“
settings-compose-signatures-summary = آپ کے پیغام کے نیچے، ”--“ لائن کے بعد شامل کیا جاتا ہے
settings-compose-for-new-mail-summary = وہ دستخط جس سے نئی میل شروع ہوتی ہے
settings-compose-for-replies-summary = وہ دستخط جس سے جوابات اور فارورڈز شروع ہوتے ہیں
settings-compose-format-summary = نئی میل سادہ متن میں لکھیں
settings-compose-spelling-summary = لکھتے وقت املا کی جانچ، اور لغت کی زبان
settings-compose-templates-summary = جلد آ رہا ہے: اکثر لکھی جانے والی میل محفوظ کریں، اور اس سے نئی میل یا جواب شروع کریں
settings-feedback-crash-reports-summary = جب Katna Mail یا اس کی بیک گراؤنڈ سروس کریش ہو تو اس کمپیوٹر پر کریش رپورٹس محفوظ کریں
settings-feedback-saved-summary = اس کمپیوٹر پر محفوظ کریش رپورٹس دیکھیں، کاپی کریں یا حذف کریں
settings-feedback-help-improve-summary = خرابی ٹھیک کرنے میں مدد کے لیے کریش رپورٹس بھیجیں؛ جب تک آپ آن نہ کریں بند رہتا ہے
settings-experimental-blur-summary = ڈیسک ٹاپ اوپری بار کے پیچھے سے دھندلا نظر آتا ہے، اور مینیو دھندلے شیشے جیسے ہوتے ہیں
settings-search-shortcut = کی بورڈ شارٹ کٹ
settings-search-tab = ترتیبات کا ٹیب
settings-search-none = کوئی ترتیب ”{ $query }“ سے مماثل نہیں۔
settings-search-results = ”{ $query }“ سے مماثل ترتیبات

## Quick settings (the panel that slides in from the right)

quick-title = فوری ترتیبات
quick-see-all = تمام ترتیبات دیکھیں
quick-reading-pane = پڑھنے کا پین
quick-pane-right = فہرست کے ساتھ
quick-pane-none = کوئی تقسیم نہیں
quick-density = کثافت
quick-density-default = ڈیفالٹ
quick-density-compact = کمپیکٹ
quick-theme = تھیم
quick-theme-system = ڈیسک ٹاپ جیسی
quick-theme-light = ہلکی
quick-theme-dark = گہری
quick-desktop-colors = ڈیسک ٹاپ کے رنگ
quick-desktop-colors-detail = ڈیسک ٹاپ کی رنگ سکیم اور ایکسنٹ رنگ
quick-app-names = ایپس کے نام
quick-app-names-detail = بالکل دائیں جانب ایپ آئیکنز کے نیچے نام
quick-inbox-tabs = ان باکس ٹیبز
quick-inbox-tabs-detail = ہر اکاؤنٹ کے میل فراہم کنندہ کے ٹیبز
quick-choose-tabs = ٹیبز منتخب کریں
quick-choose-tabs-detail = ہر اکاؤنٹ کے لیے، ترتیبات میں
quick-sending = بھیجنا
quick-undo-send = بھیجنا کالعدم کریں
quick-undo-send-off = بند
quick-undo-send-seconds = { $seconds } سیکنڈ
quick-signatures = دستخط
quick-signatures-none = ابھی کوئی نہیں
quick-signatures-one = { $name }، بطور ڈیفالٹ استعمال
quick-signatures-many = { $count ->
    [one] { $count } دستخط؛ { $name } بطور ڈیفالٹ
   *[other] { $count } دستخط؛ { $name } بطور ڈیفالٹ
}
quick-signatures-no-default = { $count ->
    [one] { $count }، کوئی ڈیفالٹ نہیں
   *[other] { $count }، کوئی ڈیفالٹ نہیں
}
quick-signature-untitled = بلا عنوان
quick-threading = ای میل تھریڈنگ
quick-conversation-view = گفتگو کا منظر
quick-conversation-view-detail = ایک ہی میل کے جوابات کو گروپ کریں
quick-help = مدد
quick-tour = ٹور کریں
quick-whats-new = نیا کیا ہے
quick-about = Katna کے بارے میں

## Settings: opening at login

settings-open-at-login-failed = لاگ ان پر کھولنے کی ترتیب نہیں بدل سکی: { $error }

## Settings > Appearance > Scaling

scale-letter = ع
scale-percent = { $percent }%
scale-reset = { $percent }% پر واپس جائیں

## Settings > Experimental > Look & Feel

look-intro = ایسی خصوصیات جنہیں ابھی آزمایا جا رہا ہے۔ یہ بدل سکتی ہیں یا ختم ہو سکتی ہیں۔
look-heading = شکل و صورت
look-window-frame = ونڈو فریم
look-window-frame-detail = ٹائٹل بار، ونڈو بٹن، کونے اور سایہ کون بناتا ہے۔
look-frame-native-kde = مقامی: KDE کا فریم، آپ کی Plasma تھیم میں
look-frame-native = مقامی: ڈیسک ٹاپ کا فریم
look-frame-katna = Katna: اوپری بار ٹائٹل بار بن جاتا ہے
look-frame-katna-note-named = Katna گول کونے اور اپنا سایہ خود بناتا ہے۔ فریم اب { $desktop } تھیم کی پیروی نہیں کرتا؛ ونڈو کے اصول اب بھی لاگو ہوتے ہیں۔
look-frame-katna-note = Katna گول کونے اور اپنا سایہ خود بناتا ہے۔ فریم اب ڈیسک ٹاپ تھیم کی پیروی نہیں کرتا؛ ونڈو کے اصول اب بھی لاگو ہوتے ہیں۔
look-frame-client-side = آپ کا ڈیسک ٹاپ فریم ہر ایپ پر چھوڑتا ہے، اس لیے Katna پہلے ہی اپنا فریم خود بناتا ہے۔
look-blurred-background = دھندلا پس منظر
look-blurred-background-detail = ڈیسک ٹاپ اوپری بار اور فولڈرز کے پیچھے سے دھندلا نظر آتا ہے، اور مینیو اور پاپ اوورز دھندلے شیشے جیسے ہوتے ہیں۔
look-blur = ونڈو کے پیچھے کا منظر دھندلا کریں
look-blur-detail = میل ٹھوس کارڈز پر رہتی ہے، اس لیے متن کا کنٹراسٹ برقرار رہتا ہے
look-blur-off-kde = KDE کا دھندلاہٹ کا اثر بند ہے۔ ”سسٹم کی ترتیبات“، ”ونڈو مینجمنٹ“، ”ڈیسک ٹاپ اثرات“ میں ”دھندلاہٹ“ آن کریں، پھر Katna Mail دوبارہ کھولیں۔
look-blur-none-gnome = GNOME ونڈوز کے پیچھے کا منظر دھندلا نہیں کرتا۔
look-blur-none-x11 = آپ کا ونڈو مینیجر ونڈوز کے پیچھے کا منظر دھندلا نہیں کرتا۔
look-blur-none-wayland = آپ کا کمپوزیٹر ونڈوز کے پیچھے کا منظر دھندلا نہیں کرتا۔

## Settings > User feedback (crash reports)

feedback-intro-sending = نئی کریش رپورٹس خرابی ٹھیک کرنے میں مدد کے لیے بھیجی جاتی ہیں۔ اس کمپیوٹر سے اور کچھ نہیں جاتا۔
feedback-intro-local = Katna کہیں کچھ نہیں بھیجتا۔ کریش رپورٹس اس کمپیوٹر پر رہتی ہیں، تاکہ آپ انہیں دیکھ سکیں یا کسی بگ رپورٹ کے ساتھ منسلک کر سکیں۔
feedback-crash-reports = کریش رپورٹس
feedback-crash-reports-detail = جب Katna Mail یا اس کی بیک گراؤنڈ سروس کریش ہو تو لکھی جاتی ہیں۔
feedback-save = اس کمپیوٹر پر کریش رپورٹس محفوظ کریں
feedback-save-detail = آپ کا ہوم فولڈر، صارف اور کمپیوٹر کے نام اور ای میل پتے شامل نہیں کیے جاتے
feedback-saved = محفوظ کردہ کریش رپورٹس
feedback-saved-detail = { $count ->
    [one] تازہ ترین { $count } رپورٹ رکھی جاتی ہے۔
   *[other] تازہ ترین { $count } رپورٹس رکھی جاتی ہیں۔
}
feedback-help-improve = Katna کو بہتر بنانے میں مدد کریں
feedback-help-improve-detail = جب تک آپ آن نہ کریں بند رہتا ہے، اور آپ اسے یہاں کسی بھی وقت بند کر سکتے ہیں۔
feedback-send = کریش رپورٹس بھیجیں
feedback-send-detail = محفوظ کردہ رپورٹ، بالکل ویسی ہی جیسی آپ اسے یہاں دیکھ سکتے ہیں، Katna کے کریش ٹریکر (Sentry، یورپی یونین میں) کو جاتی ہے۔ کوئی IP پتہ، پیغامات یا ای میل پتے نہیں
feedback-none-saved = کوئی کریش رپورٹ محفوظ نہیں۔
feedback-delete-all = سب حذف کریں
feedback-app-daemon = بیک گراؤنڈ سروس
feedback-report-sent = { $date } · بھیجی گئی
feedback-view = دیکھیں
feedback-view-tooltip = رپورٹ کھولیں
feedback-copy-tooltip = بگ رپورٹ میں چسپاں کرنے کے لیے کاپی کریں
feedback-copied = کریش رپورٹ کاپی ہو گئی۔
feedback-deleted-all = کریش رپورٹس حذف کر دی گئیں۔
feedback-read-failed = کریش رپورٹ پڑھی نہیں جا سکی: { $error }
feedback-delete-failed = کریش رپورٹ حذف نہیں ہو سکی: { $error }
feedback-delete-all-failed = کریش رپورٹس حذف نہیں ہو سکیں: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _فائل
desktop-menu-new-message = _نیا پیغام
desktop-menu-quit = _باہر نکلیں
desktop-menu-edit = _ترمیم
desktop-menu-undo = _کالعدم کریں
desktop-menu-select-all = _سب منتخب کریں
desktop-menu-select-none = _انتخاب ختم کریں
desktop-menu-find = _تلاش کریں…
desktop-menu-view = _منظر
desktop-menu-folder-list = _فولڈر فہرست دکھائیں
desktop-menu-refresh = _ریفریش کریں
desktop-menu-go = _جائیں
desktop-menu-inbox = _ان باکس
desktop-menu-starred = _ستارے والی
desktop-menu-sent = _ارسال کردہ
desktop-menu-drafts = _ڈرافٹس
desktop-menu-all-mail = _تمام میل
desktop-menu-next = _اگلی گفتگو
desktop-menu-previous = _پچھلی گفتگو
desktop-menu-message = _پیغام
desktop-menu-open = _کھولیں
desktop-menu-reply = _جواب دیں
desktop-menu-reply-all = _سب کو جواب دیں
desktop-menu-forward = _آگے بھیجیں
desktop-menu-archive = _آرکائیو کریں
desktop-menu-delete = _حذف کریں
desktop-menu-spam = _سپام کی اطلاع دیں
desktop-menu-move-to = _یہاں منتقل کریں…
desktop-menu-mark-read = _بطور پڑھا ہوا نشان زد کریں
desktop-menu-mark-unread = _بطور ناخواندہ نشان زد کریں
desktop-menu-star = _ستارہ لگائیں
desktop-menu-important = _بطور اہم نشان زد کریں
desktop-menu-not-important = _بطور غیر اہم نشان زد کریں
desktop-menu-settings = _ترتیبات
desktop-menu-quick-settings = _فوری ترتیبات
desktop-menu-configure = _Katna Mail کو کنفیگر کریں…
desktop-menu-help = _مدد
desktop-menu-shortcuts = _کی بورڈ شارٹ کٹس
desktop-menu-whats-new = _نیا کیا ہے
desktop-menu-about = _Katna کے بارے میں

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = نیویگیشن
shortcut-group-actions = کارروائیاں
shortcut-group-go-to = یہاں جائیں
shortcut-group-app = ایپلیکیشن

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = اگلی گفتگو
shortcut-previous = پچھلی گفتگو
shortcut-down = فہرست میں نیچے جائیں
shortcut-up = فہرست میں اوپر جائیں
shortcut-first = فہرست میں پہلی
shortcut-last = فہرست میں آخری
shortcut-page-down = فہرست میں ایک صفحہ نیچے
shortcut-page-up = فہرست میں ایک صفحہ اوپر
shortcut-open = گفتگو کھولیں
shortcut-back = فہرست پر واپس
shortcut-scroll-down = نیچے اسکرول کریں
shortcut-scroll-up = اوپر اسکرول کریں
shortcut-scroll-page-down = ایک صفحہ نیچے اسکرول کریں
shortcut-scroll-page-up = ایک صفحہ اوپر اسکرول کریں
shortcut-compose = تحریر کریں
shortcut-reply = جواب دیں
shortcut-reply-all = سب کو جواب دیں
shortcut-forward = آگے بھیجیں
shortcut-archive = آرکائیو کریں
shortcut-delete = حذف کریں
shortcut-spam = سپام کی اطلاع دیں
shortcut-move-to = یہاں منتقل کریں
shortcut-mark-read = بطور پڑھا ہوا نشان زد کریں
shortcut-mark-unread = بطور ناخواندہ نشان زد کریں
shortcut-star = ستارہ لگائیں یا ہٹائیں
shortcut-important = بطور اہم نشان زد کریں
shortcut-not-important = بطور غیر اہم نشان زد کریں
shortcut-check = گفتگو پر نشان لگائیں
shortcut-select-all = تمام گفتگوؤں پر نشان لگائیں
shortcut-select-none = تمام گفتگوؤں سے نشان ہٹائیں
shortcut-undo = آخری کارروائی کالعدم کریں
shortcut-go-inbox = ان باکس
shortcut-go-starred = ستارے والی
shortcut-go-sent = ارسال کردہ
shortcut-go-drafts = ڈرافٹس
shortcut-go-all = تمام میل
shortcut-search = میل تلاش کریں
shortcut-navigation = مینیو دکھائیں یا سمیٹیں
shortcut-quick-settings = فوری ترتیبات
shortcut-settings = تمام ترتیبات
shortcut-shortcuts = کی بورڈ شارٹ کٹس
shortcut-reload = نئی میل چیک کریں
shortcut-quit = باہر نکلیں

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } پھر { $second }

## Settings > Accounts

accounts-folder-pane = فولڈر پین
accounts-folder-pane-detail = دائیں جانب والا پین کن اکاؤنٹس کے فولڈرز دکھائے۔
accounts-shown-one = ایک وقت میں ایک اکاؤنٹ؛ اکاؤنٹ کارڈ میں تبدیل کریں
accounts-shown-all = تمام اکاؤنٹس، ایک کے بعد ایک
accounts-row = اکاؤنٹس
accounts-row-detail = اکاؤنٹ ہٹانے سے اس کمپیوٹر پر اس کی میل کی Katna والی کاپی حذف ہو جاتی ہے۔ میل سرور پر رہتی ہے۔
accounts-none = ابھی کوئی اکاؤنٹ نہیں۔
accounts-kind-imported = درآمد شدہ
accounts-picture-reset = ڈیسک ٹاپ کی تصویر استعمال کریں
accounts-picture-change = تصویر بدلیں
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
