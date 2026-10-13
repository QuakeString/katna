# Katna Mail, Urdu (اردو): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = نوٹس
notes-view-reminders = یاد دہانیاں
notes-view-archive = آرکائیو
notes-view-trash = کوڑے دان
notes-edit-labels = لیبلز میں ترمیم کریں
notes-search = نوٹس تلاش کریں
notes-loading = آپ کے نوٹس کھولے جا رہے ہیں…

## Board

notes-take-a-note = نوٹ لکھیں…
notes-new-list = نئی فہرست
notes-new-note = نیا نوٹ
notes-pinned = پن کردہ
notes-others = دیگر
notes-empty = آپ کے شامل کردہ نوٹس یہاں ظاہر ہوں گے
notes-archive-empty = آپ کے آرکائیو کردہ نوٹس یہاں ظاہر ہوں گے
notes-trash-empty = کوڑے دان میں کوئی نوٹ نہیں
notes-none-found = کوئی مماثل نوٹ نہیں ملا
notes-label-empty = ابھی اس لیبل والا کوئی نوٹ نہیں
notes-reminders-empty = آنے والی یاد دہانیوں والے نوٹس یہاں ظاہر ہوتے ہیں
notes-trash-note = کوڑے دان میں موجود نوٹس 7 دن بعد حذف ہو جاتے ہیں۔
notes-empty-trash = کوڑے دان خالی کریں
notes-ticked = { $count ->
    [one] + { $count } نشان زد آئٹم
   *[other] + { $count } نشان زد آئٹمز
}
notes-select = نوٹ منتخب کریں
notes-selected = { $count ->
    [one] { $count } منتخب
   *[other] { $count } منتخب
}
notes-select-clear = انتخاب صاف کریں

## A note's buttons

notes-pin = نوٹ پن کریں
notes-unpin = نوٹ کا پن ہٹائیں
notes-archive = آرکائیو کریں
notes-unarchive = آرکائیو سے نکالیں
notes-delete = نوٹ حذف کریں
notes-restore = بحال کریں
notes-delete-forever = ہمیشہ کے لیے حذف کریں
notes-color = پس منظر کے اختیارات
notes-checkboxes = چیک باکس دکھائیں یا چھپائیں
notes-labels = لیبلز
notes-close = بند کریں
notes-more = مزید
notes-make-copy = کاپی بنائیں
notes-remind = مجھے یاد دلائیں
notes-add-picture = تصویر شامل کریں
notes-history = ورژن کی تاریخ
notes-ai = لکھنے میں میری مدد کریں
notes-send-as-mail = بطور میل بھیجیں
notes-save-markdown = بطور Markdown محفوظ کریں
notes-save-pdf = بطور PDF محفوظ کریں

## The open note

notes-title = عنوان
notes-edited = آخری ترمیم: { $date }
notes-on-this-computer = اس کمپیوٹر پر
notes-where = اس نوٹ کے محفوظ ہونے کی جگہ
notes-untitled = بلا عنوان نوٹ
notes-picture-choose = تصاویر شامل کریں
notes-picture-remove = تصویر ہٹائیں
notes-picture-too-big = نوٹ میں { $size } تک کی تصاویر شامل ہو سکتی ہیں
notes-picture-kind = یہ فائل ایسی تصویر نہیں جسے Katna دکھا سکے
notes-picture-unreadable = { $name } پڑھی نہیں جا سکی: { $error }
notes-remind-me = مجھے یاد دلائیں
notes-remind-off = یاد دہانی ہٹائیں
notes-remind-in-the-past = ایسا وقت منتخب کریں جو ابھی گزرا نہ ہو
notes-remind-today = آج، { $time }
notes-remind-tomorrow = کل، { $time }
notes-remind-weekday = { $day }، { $time }
notes-reminder-set = یاد دہانی { $when } کے لیے سیٹ ہو گئی
notes-reminder-off = یاد دہانی ہٹا دی گئی
notes-link-note = نوٹ لنک کریں
notes-link-new = نیا نوٹ ”{ $title }“
notes-linked-from = یہاں سے لنک شدہ
notes-link-gone = وہ نوٹ اب یہاں نہیں ہے
notes-new-note-gone = نیا نوٹ غائب ہو گیا ہے۔
notes-versions = ورژنز
notes-version-now = ابھی
notes-version-here = آپ، اس کمپیوٹر پر
notes-version-yesterday = گزشتہ کل، { $time }
notes-version-changes = { $count ->
    [one] { $count } تبدیلی
   *[other] { $count } تبدیلیاں
}
notes-version-from = { $device } سے
notes-version-elsewhere = کسی اور ڈیوائس سے
notes-version-created = بنایا گیا
notes-version-restore = یہ ورژن بحال کریں
notes-version-restored = ورژن بحال ہو گیا
notes-history-none = ابھی کوئی پرانا ورژن نہیں
notes-ai-tidy = متن کو صاف ستھرا کریں
notes-ai-checklist = اسے چیک لسٹ میں بدلیں
notes-ai-summarise = خلاصہ کریں
notes-ai-empty = پہلے کچھ لکھیں
notes-ai-tidied = متن صاف ستھرا ہو گیا۔ Ctrl+Z اسے واپس لاتا ہے۔
notes-ai-listed = چیک لسٹ بن گئی۔ Ctrl+Z اسے واپس لاتا ہے۔
notes-ai-summarised = خلاصہ سب سے اوپر شامل ہو گیا

## Labels

notes-label-note = نوٹ پر لیبل لگائیں
notes-label-name = لیبل کا نام درج کریں
notes-label-create = “{ $name }” بنائیں
notes-label-remove = لیبل ہٹائیں
notes-label-delete = لیبل حذف کریں
notes-labels-none = ابھی کوئی لیبل نہیں۔ نوٹ کے لیبل بٹن سے شامل کریں۔
notes-labels-done = ہو گیا
notes-label-renamed = لیبل کا نام بدل کر “{ $name }” کر دیا گیا
notes-label-deleted = لیبل “{ $name }” حذف ہو گیا

## A note about a mail

notes-mail = میل
notes-open-mail = میل کھولیں
notes-open-note = نوٹ کھولیں

## Meeting notes

notes-meeting-take = میٹنگ کے نوٹس لکھیں
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = شرکاء: { $names }
notes-meeting-notes = نوٹس
notes-meeting-actions = کرنے کے کام
notes-event = ایونٹ
notes-open-event = ایونٹ کھولیں

## Formatting

notes-format = فارمیٹنگ
notes-format-heading-1 = سرخی 1
notes-format-heading-2 = سرخی 2
notes-format-normal = عام متن
notes-format-bold = جلی
notes-format-italic = ترچھا
notes-format-underline = خط کشیدہ
notes-format-quote = اقتباس
notes-format-code = کوڈ
notes-format-divider = حد فاصل
notes-format-clear = فارمیٹنگ صاف کریں

## Tasks

notes-make-task = کام بنائیں

## Colors (tooltips)

notes-color-none = کوئی رنگ نہیں
notes-color-coral = مرجانی
notes-color-peach = آڑو
notes-color-sand = ریتیلا
notes-color-mint = پودینہ
notes-color-sage = سیج
notes-color-fog = دھند
notes-color-storm = طوفان
notes-color-dusk = شفق
notes-color-blossom = شگوفہ
notes-color-clay = مٹی
notes-color-chalk = چاک

## Messages at the foot of the window

notes-archived = نوٹ آرکائیو ہو گیا
notes-unarchived = نوٹ آرکائیو سے نکل گیا
notes-trashed = نوٹ کوڑے دان میں منتقل ہو گیا
notes-restored = نوٹ بحال ہو گیا
notes-saved = نوٹ محفوظ ہو گیا
notes-pinned-count = { $count ->
    [one] نوٹ پن ہو گیا
   *[other] { $count } نوٹس پن ہو گئے
}
notes-unpinned-count = { $count ->
    [one] نوٹ کا پن ہٹ گیا
   *[other] { $count } نوٹس کے پن ہٹ گئے
}
notes-colored-count = { $count ->
    [one] رنگ بدل گیا
   *[other] { $count } نوٹس کا رنگ بدل گیا
}
notes-archived-count = { $count ->
    [one] نوٹ آرکائیو ہو گیا
   *[other] { $count } نوٹس آرکائیو ہو گئے
}
notes-unarchived-count = { $count ->
    [one] نوٹ آرکائیو سے نکل گیا
   *[other] { $count } نوٹس آرکائیو سے نکل گئے
}
notes-trashed-count = { $count ->
    [one] نوٹ کوڑے دان میں منتقل ہو گیا
   *[other] { $count } نوٹس کوڑے دان میں منتقل ہو گئے
}
notes-restored-count = { $count ->
    [one] نوٹ بحال ہو گیا
   *[other] { $count } نوٹس بحال ہو گئے
}
notes-copied-count = { $count ->
    [one] کاپی بن گئی
   *[other] { $count } کاپیاں بن گئیں
}
notes-empty-discarded = خالی نوٹ رد کر دیا گیا
notes-mail-gone = وہ میل اب یہاں نہیں ہے
notes-deleted-forever = { $count ->
    [one] نوٹ ہمیشہ کے لیے حذف ہو گیا
   *[other] { $count } نوٹس ہمیشہ کے لیے حذف ہو گئے
}
