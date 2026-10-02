# Katna Mail, Urdu (اردو).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = فائلیں تلاش کریں

## Left side (and chips on a phone)

files-all = تمام فائلیں
files-pictures = تصاویر
files-pdfs = PDFs
files-documents = دستاویزات
files-sheets = اسپریڈشیٹس
files-slides = سلائیڈز
files-other = دیگر
files-accounts = اکاؤنٹس
files-drives = ڈرائیوز
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = میرے ساتھ شیئر کردہ
files-shown = دکھائی گئی
files-received = موصول شدہ
files-sent = میری بھیجی ہوئی

## Over the files

files-count = { $count ->
    [one] { $count } فائل · { $size }
   *[other] { $count } فائلیں · { $size }
}
files-anyone = کوئی بھی
files-from-person = { $name } سے
files-time-any = کسی بھی وقت
files-time-today = آج
files-time-yesterday = کل
files-time-this-week = اس ہفتے
files-time-last-week = پچھلے ہفتے
files-time-this-month = اس مہینے
files-time-last-month = پچھلے مہینے
files-time-between = { $first } – { $last }
files-time-hint = کسی دن پر کلک کریں، یا کئی دنوں پر گھسیٹیں
files-time-summary = { $count ->
    [one] { $days } · { $count } فائل
   *[other] { $days } · { $count } فائلیں
}
files-time-clear = صاف کریں
files-time-month-back = پچھلا مہینہ
files-time-month-on = اگلا مہینہ
files-time-wheel = ان تاریخوں کو، ان کی مدت برقرار رکھتے ہوئے، ہلانے کے لیے اسکرول کریں
files-sort-newest = سب سے نئی پہلے
files-sort-oldest = سب سے پرانی پہلے
files-sort-largest = سب سے بڑی پہلے
files-sort-name = نام کے لحاظ سے
files-grid = کارڈز
files-list = فہرست
files-this-week = اس ہفتے
files-undated = کوئی تاریخ نہیں
files-me = میں
files-no-subject = (کوئی موضوع نہیں)
files-loading = آپ کی میل سے فائلیں جمع کی جا رہی ہیں…
files-empty = آپ کی میل کی فائلیں یہاں دکھائی دیتی ہیں۔
files-none-match = کوئی فائل مماثل نہیں۔
files-load-failed = فائلیں پڑھنے میں ناکامی: { $error }

## A file's menu and buttons

files-open = کھولیں
files-open-with = اس کے ساتھ کھولیں…
files-save = محفوظ کریں…
files-show-mail = میل دکھائیں
files-mail-window = میل نئی ونڈو میں کھولیں
files-forward = فائل آگے بھیجیں
files-from-them = { $name } کی فائلیں
files-copy-name = فائل کا نام کاپی کریں
files-name-copied = فائل کا نام کاپی ہو گیا
files-downloading = میل ڈاؤن لوڈ ہو رہی ہے…
files-download-failed = یہ میل ڈاؤن لوڈ نہیں ہو سکی۔

## A cloud drive in place of the mail files

files-drive-mine = میری ڈرائیو
files-drive-mine-onedrive = میری فائلیں
files-drive-results = ”{ $words }“
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 فائل
       *[other] { $files } فائلیں
    }
    [one] 1 فولڈر · { $files ->
        [one] 1 فائل
       *[other] { $files } فائلیں
    }
   *[other] { $folders } فولڈرز · { $files ->
        [one] 1 فائل
       *[other] { $files } فائلیں
    }
}
files-drive-folders = فولڈرز
files-drive-files = فائلیں
files-drive-folder = فولڈر
files-drive-meta = { $what } · { $date } کو ترمیم کی گئی
files-drive-as-link = { $what } · بطور لنک
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = لائی جا رہی ہے…
files-drive-loading = ڈرائیو کھل رہی ہے…
files-drive-empty = یہ فولڈر خالی ہے۔
files-drive-unreachable = { $drive } تک رسائی نہیں ہو سکتی۔
files-drive-try-again = دوبارہ کوشش کریں
files-drive-needs-permission = یہ ڈرائیو دکھانے کے لیے Katna کو ایک بار آپ کی اجازت درکار ہے۔ دوبارہ سائن ان کریں اور Katna کو اپنی فائلیں دیکھنے کی اجازت دیں۔
files-drive-allow = اجازت دیں
files-drive-allow-failed = سائن ان مکمل نہیں ہوا، اس لیے ڈرائیو بند رہتی ہے۔
files-drive-attach = منسلک کریں
files-drive-more = مزید
files-drive-download = ڈاؤن لوڈ کریں…
files-drive-open-web = { $drive } میں کھولیں
files-drive-copy-link = لنک کاپی کریں
files-drive-link-copied = لنک کاپی ہو گیا
files-drive-share = شیئر کریں…
files-drive-rename = نام بدلیں
files-drive-trash = ردی میں منتقل کریں
files-drive-trashed = ”{ $name }“ { $drive } کی ردی میں ہے
files-drive-renamed = نام بدل کر ”{ $name }“ کر دیا گیا
files-drive-getting = { $drive } سے { $name } لائی جا رہی ہے…
files-drive-get-failed = { $name } حاصل نہیں ہو سکی: { $error }
files-drive-upload = اپ لوڈ کریں
files-drive-upload-files = فائلیں اپ لوڈ کریں
files-drive-upload-folder = فولڈر اپ لوڈ کریں
files-drive-upload-failed = { $name } اپ لوڈ نہیں ہو سکی: { $error }
files-drive-upload-needs = اپ لوڈ کرنے کے لیے Katna کو ایک بار آپ کی اجازت درکار ہے: ترتیبات › ڈیفالٹ ایپس › فائلوں کا صفحہ میں اجازت دیں دبائیں۔

## The Share dialog of a drive file or folder

files-share-title = ”{ $name }“ شیئر کریں
files-share-add = نام یا پتے سے لوگ شامل کریں
files-share-not-address = ”{ $text }“ ای میل پتہ نہیں ہے
files-share-notify = { $drive } کو انہیں ای میل بھی کرنے دیں
files-share-people = رسائی والے لوگ
files-share-general = عمومی رسائی
files-share-loading = پڑھا جا رہا ہے کہ کس کو رسائی ہے…
files-share-restricted = محدود
files-share-restricted-about = صرف رسائی والے لوگ اسے لنک سے کھول سکتے ہیں
files-share-anyone = لنک رکھنے والا کوئی بھی
files-share-anyone-can = { $role ->
    [editor] لنک رکھنے والا کوئی بھی ترمیم کر سکتا ہے
    [commenter] لنک رکھنے والا کوئی بھی تبصرہ کر سکتا ہے
   *[viewer] لنک رکھنے والا کوئی بھی دیکھ سکتا ہے
}
files-share-anyone-about = { $role ->
    [editor] انٹرنیٹ پر لنک رکھنے والا کوئی بھی ترمیم کر سکتا ہے
    [commenter] انٹرنیٹ پر لنک رکھنے والا کوئی بھی تبصرہ کر سکتا ہے
   *[viewer] انٹرنیٹ پر لنک رکھنے والا کوئی بھی دیکھ سکتا ہے
}
files-share-role-owner = مالک
files-share-role-editor = مدیر
files-share-role-commenter = تبصرہ نگار
files-share-role-viewer = ناظر
files-share-you = { $name } (آپ)
files-share-domain = { $domain } پر سب
files-share-inherited = اس فولڈر سے رسائی جس میں یہ ہے
files-share-remove = رسائی ہٹائیں
files-share-copy-link = لنک کاپی کریں
files-share-share = شیئر کریں
files-share-done = ہو گیا
files-share-sharing = شیئر ہو رہا ہے…
files-share-shared = { $count ->
    [one] 1 شخص کے ساتھ شیئر کیا گیا
   *[other] { $count } افراد کے ساتھ شیئر کیا گیا
}
files-share-refused = { $drive } { $addresses } کے ساتھ شیئر نہیں کر سکا
files-share-failed = شیئرنگ تبدیل نہیں ہو سکی: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] 1 آئٹم اپ لوڈ ہو رہا ہے
   *[other] { $count } آئٹمز اپ لوڈ ہو رہے ہیں
}
files-tray-done = { $count ->
    [one] 1 اپ لوڈ مکمل
   *[other] { $count } اپ لوڈز مکمل
}
files-tray-some-failed = { $done } اپ لوڈ ہوئے، { $failed } ناکام
files-tray-minutes-left = { $minutes ->
    [one] تقریباً ایک منٹ باقی
   *[other] تقریباً { $minutes } منٹ باقی
}
files-tray-seconds-left = ایک منٹ سے کم باقی
files-tray-starting = شروع ہو رہا ہے…
files-tray-cancel-all = سب منسوخ کریں
files-tray-cancel = منسوخ کریں
files-tray-fold = فہرست چھپائیں
files-tray-unfold = فہرست دکھائیں
files-tray-close = بند کریں
files-tray-progress = { $place } · { $size } میں سے { $sent }
files-tray-in = { $place } میں
files-tray-cancelled = منسوخ ہو گیا
