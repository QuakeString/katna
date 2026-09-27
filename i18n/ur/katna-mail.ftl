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
