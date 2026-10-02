# Katna Mail, Persian (فارسی).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = جستجوی فایل‌ها

## Left side (and chips on a phone)

files-all = همهٔ فایل‌ها
files-pictures = تصاویر
files-pdfs = فایل‌های PDF
files-documents = سندها
files-sheets = صفحه‌گسترده‌ها
files-slides = اسلایدها
files-other = سایر
files-accounts = حساب‌ها
files-drives = درایوها
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = اشتراک‌گذاشته با من
files-shown = نمایش‌داده‌شده
files-received = دریافتی
files-sent = ارسالی من

## Over the files

files-count = { $count ->
    [one] { $count } فایل · { $size }
   *[other] { $count } فایل · { $size }
}
files-anyone = هر کسی
files-from-person = از { $name }
files-time-any = هر زمانی
files-time-today = امروز
files-time-yesterday = دیروز
files-time-this-week = این هفته
files-time-last-week = هفتهٔ گذشته
files-time-this-month = این ماه
files-time-last-month = ماه گذشته
files-time-between = { $first } – { $last }
files-time-hint = روی یک روز کلیک کنید، یا روی چند روز بکشید
files-time-summary = { $count ->
    [one] { $days } · { $count } فایل
   *[other] { $days } · { $count } فایل
}
files-time-clear = پاک کردن
files-time-month-back = ماه قبل
files-time-month-on = ماه بعد
files-time-wheel = برای جابه‌جا کردن این تاریخ‌ها با حفظ طولشان اسکرول کنید
files-sort-newest = جدیدترین اول
files-sort-oldest = قدیمی‌ترین اول
files-sort-largest = بزرگ‌ترین اول
files-sort-name = بر اساس نام
files-grid = کارت‌ها
files-list = فهرست
files-this-week = این هفته
files-undated = بدون تاریخ
files-me = من
files-no-subject = (بدون موضوع)
files-loading = در حال جمع‌آوری فایل‌ها از ایمیل‌هایتان…
files-empty = فایل‌های ایمیل‌هایتان این‌جا نشان داده می‌شوند.
files-none-match = هیچ فایلی مطابقت ندارد.
files-load-failed = خواندن فایل‌ها ممکن نشد: { $error }

## A file's menu and buttons

files-open = باز کردن
files-open-with = باز کردن با…
files-save = ذخیره…
files-show-mail = نمایش ایمیل
files-mail-window = باز کردن ایمیل در پنجرهٔ جدید
files-forward = بازارسال فایل
files-from-them = فایل‌ها از { $name }
files-copy-name = کپی نام فایل
files-name-copied = نام فایل کپی شد
files-downloading = در حال بارگیری ایمیل…
files-download-failed = بارگیری این ایمیل ممکن نشد.

## A cloud drive in place of the mail files

files-drive-mine = درایو من
files-drive-mine-onedrive = فایل‌های من
files-drive-results = «{ $words }»
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 فایل
       *[other] { $files } فایل
    }
    [one] 1 پوشه · { $files ->
        [one] 1 فایل
       *[other] { $files } فایل
    }
   *[other] { $folders } پوشه · { $files ->
        [one] 1 فایل
       *[other] { $files } فایل
    }
}
files-drive-folders = پوشه‌ها
files-drive-files = فایل‌ها
files-drive-folder = پوشه
files-drive-meta = { $what } · ویرایش‌شده { $date }
files-drive-as-link = { $what } · به‌صورت پیوند
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = در حال دریافت…
files-drive-loading = در حال باز کردن درایو…
files-drive-empty = این پوشه خالی است.
files-drive-unreachable = دسترسی به { $drive } ممکن نیست.
files-drive-try-again = دوباره امتحان کنید
files-drive-needs-permission = Katna برای نمایش این درایو یک بار به اجازهٔ شما نیاز دارد. دوباره وارد شوید و به Katna اجازه دهید فایل‌هایتان را ببیند.
files-drive-allow = اجازه دادن
files-drive-allow-failed = ورود کامل نشد، پس درایو بسته می‌ماند.
files-drive-attach = پیوست
files-drive-more = بیشتر
files-drive-download = بارگیری…
files-drive-open-web = باز کردن در { $drive }
files-drive-copy-link = کپی پیوند
files-drive-link-copied = پیوند کپی شد
files-drive-share = اشتراک‌گذاری…
files-drive-rename = تغییر نام
files-drive-trash = انتقال به سطل زباله
files-drive-trashed = «{ $name }» در سطل زبالهٔ { $drive } است
files-drive-renamed = نام به «{ $name }» تغییر کرد
files-drive-getting = در حال دریافت { $name } از { $drive }…
files-drive-get-failed = دریافت { $name } ممکن نشد: { $error }
files-drive-upload = بارگذاری
files-drive-upload-files = بارگذاری فایل‌ها
files-drive-upload-folder = بارگذاری پوشه
files-drive-upload-failed = بارگذاری { $name } ممکن نشد: { $error }
files-drive-upload-needs = برای بارگذاری، Katna یک بار به اجازهٔ شما نیاز دارد: در تنظیمات › برنامه‌های پیش‌فرض › صفحهٔ فایل‌ها، «اجازه دادن» را بزنید.

## The Share dialog of a drive file or folder

files-share-title = اشتراک‌گذاری «{ $name }»
files-share-add = افزودن افراد با نام یا نشانی
files-share-not-address = «{ $text }» نشانی ایمیل نیست
files-share-notify = { $drive } به آن‌ها هم ایمیل بزند
files-share-people = افراد دارای دسترسی
files-share-general = دسترسی عمومی
files-share-loading = در حال خواندن این‌که چه کسی دسترسی دارد…
files-share-restricted = محدود
files-share-restricted-about = فقط افراد دارای دسترسی می‌توانند آن را با پیوند باز کنند
files-share-anyone = هر کسی که پیوند را دارد
files-share-anyone-can = { $role ->
    [editor] هر کسی که پیوند را دارد می‌تواند ویرایش کند
    [commenter] هر کسی که پیوند را دارد می‌تواند نظر بدهد
   *[viewer] هر کسی که پیوند را دارد می‌تواند ببیند
}
files-share-anyone-about = { $role ->
    [editor] هر کسی در اینترنت که پیوند را دارد می‌تواند ویرایش کند
    [commenter] هر کسی در اینترنت که پیوند را دارد می‌تواند نظر بدهد
   *[viewer] هر کسی در اینترنت که پیوند را دارد می‌تواند ببیند
}
files-share-role-owner = مالک
files-share-role-editor = ویرایشگر
files-share-role-commenter = نظردهنده
files-share-role-viewer = بیننده
files-share-you = { $name } (شما)
files-share-domain = همه در { $domain }
files-share-inherited = دسترسی از پوشه‌ای که در آن است
files-share-remove = حذف دسترسی
files-share-copy-link = کپی پیوند
files-share-share = اشتراک‌گذاری
files-share-done = تمام
files-share-close = بستن
files-share-sharing = در حال اشتراک‌گذاری…
files-share-shared = { $count ->
    [one] با 1 نفر به اشتراک گذاشته شد
   *[other] با { $count } نفر به اشتراک گذاشته شد
}
files-share-refused = { $drive } نتوانست با { $addresses } به اشتراک بگذارد
files-share-failed = تغییر اشتراک‌گذاری ممکن نشد: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] در حال بارگذاری 1 مورد
   *[other] در حال بارگذاری { $count } مورد
}
files-tray-done = { $count ->
    [one] 1 بارگذاری انجام شد
   *[other] { $count } بارگذاری انجام شد
}
files-tray-some-failed = { $done } بارگذاری شد، { $failed } ناموفق
files-tray-minutes-left = { $minutes ->
    [one] حدود یک دقیقه مانده
   *[other] حدود { $minutes } دقیقه مانده
}
files-tray-seconds-left = کمتر از یک دقیقه مانده
files-tray-starting = در حال شروع…
files-tray-cancel-all = لغو همه
files-tray-cancel = لغو
files-tray-fold = پنهان کردن فهرست
files-tray-unfold = نمایش فهرست
files-tray-close = بستن
files-tray-progress = { $place } · { $sent } از { $size }
files-tray-in = در { $place }
files-tray-cancelled = لغو شد
