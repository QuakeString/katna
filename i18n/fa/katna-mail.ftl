# Katna Mail, Persian (فارسی).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = زبان: { $language }
language-tooltip-system = زبان: { $language }، مطابق سیستم
language-search = جستجوی زبان
language-system-default = پیش‌فرض سیستم
language-system-now = اکنون: { $language }
language-no-match = هیچ زبانی با «{ $query }» مطابقت ندارد
language-machine = ترجمهٔ ماشینی. به بهبود آن کمک کنید
language-setting = زبان
language-setting-detail = زبان منوها، دکمه‌ها و پیام‌ها و قالب تاریخ‌ها و اعداد. «پیش‌فرض سیستم» از میزکار پیروی می‌کند.

## Dates and sizes

ago-just-now = همین الان
ago-minutes = { $count ->
    [one] { $count } دقیقه پیش
   *[other] { $count } دقیقه پیش
}
ago-hours = { $count ->
    [one] { $count } ساعت پیش
   *[other] { $count } ساعت پیش
}
ago-days = { $count ->
    [one] { $count } روز پیش
   *[other] { $count } روز پیش
}
size-bytes = { $count ->
    [one] { $count } بایت
   *[other] { $count } بایت
}
size-kb = { $size } کیلوبایت
size-mb = { $size } مگابایت
size-gb = { $size } گیگابایت
size-tb = { $size } ترابایت

## Top bar

folders-hide = پنهان کردن پوشه‌ها
folders-show = نمایش پوشه‌ها
compose = نوشتن
search = جستجو
search-mail = جستجوی ایمیل
search-settings = جستجوی تنظیمات
search-clear = پاک کردن جستجو
search-options-show = نمایش گزینه‌های جستجو
settings = تنظیمات
account-add = افزودن حساب

## App rail (and the bottom bar on a phone)

rail-mail = ایمیل
rail-calendar = تقویم
rail-contacts = مخاطبین
rail-tasks = کارها
rail-notes = یادداشت‌ها
rail-feeds = خوراک‌ها

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = به‌زودی
app-calendar-promise = تقویم‌های CalDAV، دعوت‌نامه‌های جلسه از ایمیل‌هایتان و یادآورها، کنار صندوق ورودی‌تان.
app-tasks-promise = فهرست‌های کار که با CalDAV همگام می‌شوند، و کارهایی که از ایمیل ساخته می‌شوند.
app-notes-promise = یادداشت‌های سریع، و یادداشت روی یک ایمیل یا مکالمه برای بعد.
app-feeds-promise = خوراک‌های RSS و Atom را کنار ایمیل‌هایتان بخوانید.

## Contacts page

app-contacts-loading = در حال جمع‌آوری افراد از ایمیل‌هایتان…
app-contacts-empty = افرادی که با آن‌ها مکاتبه می‌کنید اینجا نشان داده می‌شوند.
app-contacts-count = { $count ->
    [one] { $count } نفر از ایمیل‌هایتان، به ترتیب بیشترین مکاتبه
   *[other] { $count } نفر از ایمیل‌هایتان، به ترتیب بیشترین مکاتبه
}
app-contacts-top = { $count ->
    [one] { $count } نفر برتر از ایمیل‌هایتان، به ترتیب بیشترین مکاتبه
   *[other] { $count } نفر برتر از ایمیل‌هایتان، به ترتیب بیشترین مکاتبه
}
app-contacts-messages = { $count ->
    [one] { $count } پیام
   *[other] { $count } پیام
}
app-contacts-last = آخرین بار { $date }

## Navigation (the folders pane)

nav-labels = برچسب‌ها
nav-folders = پوشه‌ها
nav-label-new = ایجاد برچسب جدید
nav-folder-new = ایجاد پوشهٔ جدید
nav-account-unnamed = حساب { $number }
nav-tab-new = { $count ->
    [one] { $count } جدید
   *[other] { $count } جدید
}

## Special folders (the user's own folders keep their names)

folder-inbox = صندوق ورودی
folder-starred = ستاره‌دار
folder-drafts = پیش‌نویس‌ها
folder-sent = ارسال‌شده
folder-archive = بایگانی
folder-spam = هرزنامه
folder-trash = سطل زباله
folder-all-mail = همهٔ ایمیل‌ها
folder-scheduled = زمان‌بندی‌شده

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = برچسب جدید
label-folder-new-title = پوشهٔ جدید
label-prompt = لطفاً نام برچسب جدید را وارد کنید:
label-folder-prompt = لطفاً نام پوشهٔ جدید را وارد کنید:
label-name-hint = نام برچسب
label-folder-name-hint = نام پوشه
label-nest = قرار دادن برچسب زیر:
label-folder-nest = قرار دادن پوشه زیر:
label-cancel = لغو
label-create = ایجاد
label-creating = در حال ایجاد…
label-created = برچسب «{ $name }» ایجاد شد.
label-folder-created = پوشهٔ «{ $name }» ایجاد شد.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = اصلی
tab-promotions = تبلیغات
tab-social = اجتماعی
tab-updates = به‌روزرسانی‌ها
tab-forums = انجمن‌ها
tab-focused = متمرکز
tab-other = سایر
tab-inbox = صندوق ورودی
tab-newsletters = خبرنامه‌ها
tab-notifications = اعلان‌ها
tab-new = { $count } جدید
tab-provider-other = مرتب‌شده توسط Katna

## Mail list: toolbar

list-select = انتخاب
list-refresh = بازخوانی
list-more = بیشتر
list-mark-read = علامت‌گذاری به‌عنوان خوانده‌شده
list-mark-unread = علامت‌گذاری به‌عنوان خوانده‌نشده
list-move-to = انتقال به
list-archive = بایگانی
list-spam = گزارش هرزنامه
list-delete = حذف
list-newer = جدیدتر
list-older = قدیمی‌تر
list-range = { $first }–{ $last } از { $total }
list-range-about = { $first }–{ $last } از حدود { $total }
list-results = نتایج برای «{ $query }»
list-results-corrected = نمایش نتایج برای «{ $query }»
list-search-instead = به‌جای آن «{ $query }» را جستجو کنید
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = همه
list-pick-none = هیچ‌کدام
list-pick-read = خوانده‌شده
list-pick-unread = خوانده‌نشده
list-pick-starred = ستاره‌دار
list-pick-unstarred = بدون ستاره

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } مکالمه انتخاب شده است.
       *[other] همه { $count } مکالمه انتخاب شده‌اند.
    }
   *[message] { $count ->
        [one] { $count } پیام انتخاب شده است.
       *[other] همه { $count } پیام انتخاب شده‌اند.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $count } مکالمه در { $folder } انتخاب شده است.
       *[other] همه { $count } مکالمه در { $folder } انتخاب شده‌اند.
    }
   *[message] { $count ->
        [one] { $count } پیام در { $folder } انتخاب شده است.
       *[other] همه { $count } پیام در { $folder } انتخاب شده‌اند.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] { $count } مکالمه روی صفحه انتخاب شده است.
       *[other] همه { $count } مکالمه روی صفحه انتخاب شده‌اند.
    }
   *[message] { $count ->
        [one] { $count } پیام روی صفحه انتخاب شده است.
       *[other] همه { $count } پیام روی صفحه انتخاب شده‌اند.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] انتخاب { $count } مکالمه
       *[other] انتخاب همه { $count } مکالمه
    }
   *[message] { $count ->
        [one] انتخاب { $count } پیام
       *[other] انتخاب همه { $count } پیام
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] انتخاب { $count } مکالمه در { $folder }
       *[other] انتخاب همه { $count } مکالمه در { $folder }
    }
   *[message] { $count ->
        [one] انتخاب { $count } پیام در { $folder }
       *[other] انتخاب همه { $count } پیام در { $folder }
    }
}
list-clear-selection = پاک کردن انتخاب

## Mail list: empty states

list-empty-search = هیچ پیامی با جستجوی شما مطابقت ندارد.
list-empty-tab = هیچ ایمیلی در { $tab } نیست.
list-empty-tab-unknown = هیچ ایمیلی در این برگه نیست.
list-empty-folder = هیچ پیامی در { $folder } نیست.
list-empty-folder-unknown = هیچ پیامی در این پوشه نیست.
list-first-sync = در حال دریافت ایمیل‌هایتان…
list-first-sync-detail = ایمیل‌ها همزمان با رسیدن اینجا نشان داده می‌شوند.

## Mail list: lines

row-removed = این پیام حذف شد.
row-starred = ستاره‌دار
row-not-starred = بدون ستاره
row-important = مهم. برای علامت‌گذاری به‌عنوان غیرمهم کلیک کنید.
row-mark-important = علامت‌گذاری به‌عنوان مهم
row-pinned = سنجاق‌شده در بالا
row-pin = سنجاق کردن به بالا
row-unpin = برداشتن سنجاق

## Mail list: More menu and right-click menu

menu-reply = پاسخ
menu-reply-all = پاسخ به همه
menu-forward = بازارسال
menu-archive = بایگانی
menu-delete = حذف
menu-spam = گزارش هرزنامه
menu-mark-read = علامت‌گذاری به‌عنوان خوانده‌شده
menu-mark-unread = علامت‌گذاری به‌عنوان خوانده‌نشده
menu-mark-all-read = علامت‌گذاری همه به‌عنوان خوانده‌شده
menu-star = افزودن ستاره
menu-unstar = برداشتن ستاره
menu-important = علامت‌گذاری به‌عنوان مهم
menu-not-important = علامت‌گذاری به‌عنوان غیرمهم
menu-pin = سنجاق کردن به بالا
menu-unpin = برداشتن سنجاق
menu-print-all = چاپ همه
menu-new-window = باز کردن در پنجرهٔ جدید
menu-move-to = انتقال به
menu-move-to-heading = انتقال به:
menu-find-from = یافتن ایمیل‌های { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] مکالمه بایگانی شد.
       *[other] { $count } مکالمه بایگانی شد.
    }
   *[message] { $count ->
        [one] پیام بایگانی شد.
       *[other] { $count } پیام بایگانی شد.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] مکالمه به سطل زباله منتقل شد.
       *[other] { $count } مکالمه به سطل زباله منتقل شد.
    }
   *[message] { $count ->
        [one] پیام به سطل زباله منتقل شد.
       *[other] { $count } پیام به سطل زباله منتقل شد.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] مکالمه منتقل شد.
       *[other] { $count } مکالمه منتقل شد.
    }
   *[message] { $count ->
        [one] پیام منتقل شد.
       *[other] { $count } پیام منتقل شد.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] مکالمه ستاره‌دار شد.
       *[other] { $count } مکالمه ستاره‌دار شد.
    }
   *[message] { $count ->
        [one] پیام ستاره‌دار شد.
       *[other] { $count } پیام ستاره‌دار شد.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] ستارهٔ مکالمه برداشته شد.
       *[other] ستارهٔ { $count } مکالمه برداشته شد.
    }
   *[message] { $count ->
        [one] ستارهٔ پیام برداشته شد.
       *[other] ستارهٔ { $count } پیام برداشته شد.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] مکالمه به‌عنوان مهم علامت‌گذاری شد.
       *[other] { $count } مکالمه به‌عنوان مهم علامت‌گذاری شد.
    }
   *[message] { $count ->
        [one] پیام به‌عنوان مهم علامت‌گذاری شد.
       *[other] { $count } پیام به‌عنوان مهم علامت‌گذاری شد.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] مکالمه به‌عنوان غیرمهم علامت‌گذاری شد.
       *[other] { $count } مکالمه به‌عنوان غیرمهم علامت‌گذاری شد.
    }
   *[message] { $count ->
        [one] پیام به‌عنوان غیرمهم علامت‌گذاری شد.
       *[other] { $count } پیام به‌عنوان غیرمهم علامت‌گذاری شد.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] مکالمه به بالا سنجاق شد.
       *[other] { $count } مکالمه به بالا سنجاق شد.
    }
   *[message] { $count ->
        [one] پیام به بالا سنجاق شد.
       *[other] { $count } پیام به بالا سنجاق شد.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] سنجاق مکالمه برداشته شد.
       *[other] سنجاق { $count } مکالمه برداشته شد.
    }
   *[message] { $count ->
        [one] سنجاق پیام برداشته شد.
       *[other] سنجاق { $count } پیام برداشته شد.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] مکالمه به‌عنوان هرزنامه گزارش شد.
       *[other] { $count } مکالمه به‌عنوان هرزنامه گزارش شد.
    }
   *[message] { $count ->
        [one] پیام به‌عنوان هرزنامه گزارش شد.
       *[other] { $count } پیام به‌عنوان هرزنامه گزارش شد.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] مکالمه برای همیشه حذف شد.
       *[other] { $count } مکالمه برای همیشه حذف شد.
    }
   *[message] { $count ->
        [one] پیام برای همیشه حذف شد.
       *[other] { $count } پیام برای همیشه حذف شد.
    }
}
toast-undone = کار واگرد شد.
toast-undo = واگرد
toast-no-spam-folder = این حساب پوشهٔ هرزنامه ندارد.

## Reading pane: toolbar

reader-close = بستن
reader-back = برگشت
reader-mark-unread = علامت‌گذاری به‌عنوان خوانده‌نشده
reader-move-to = انتقال به
reader-more = بیشتر
reader-print-all = چاپ همه
reader-new-window = در پنجرهٔ جدید
reader-position = { $position } از { $total }
reader-newer = جدیدتر
reader-older = قدیمی‌تر

## Reading pane: the conversation

reader-removed = این مکالمه حذف شد.
reader-no-subject = (بدون موضوع)
reader-collapse-all = جمع کردن همه
reader-expand-all = باز کردن همه
reader-unknown-sender = (فرستندهٔ ناشناس)
reader-date-ago = { $date } ({ $ago })
reader-me = من
reader-to = به { $names }
reader-starred = ستاره‌دار
reader-not-starred = بدون ستاره
reader-too-long = این پیام برای نمایش کامل بیش از حد طولانی است.
reader-encrypted-images = تصاویر وب هرگز در ایمیل رمزگذاری‌شده بار نمی‌شوند.
reader-window-failed = باز کردن پنجرهٔ جدید ممکن نشد.

## Reading pane: message details (opened from "to me")

reader-details-from = از:
reader-details-to = به:
reader-details-cc = رونوشت:
reader-details-date = تاریخ:
reader-details-subject = موضوع:

## Reading pane: downloading a message

reader-downloading = در حال بارگیری این پیام از سرور…
reader-download-failed = بارگیری این پیام ممکن نشد.
reader-try-again = امتحان مجدد

## Reply row

reply-reply = پاسخ
reply-reply-all = پاسخ به همه
reply-forward = بازارسال

## Encrypted and signed mail

security-decrypting = در حال رمزگشایی…
security-checking = در حال بررسی امضا…
security-partly-encrypted = فقط بخشی از این پیام رمزگذاری شده است. بقیه بیرون از محافظت اضافه شده و ممکن است از هر کسی باشد.
security-partly-signed = فقط بخشی از این پیام امضا شده است. بقیه بیرون از محافظت اضافه شده و ممکن است از هر کسی باشد.
security-encrypted = پیام رمزگذاری‌شده
security-encrypted-smime = پیام رمزگذاری‌شده (S/MIME)
security-no-key = رمزگشایی این پیام ممکن نیست: برای کلیدی رمزگذاری شده که آن را ندارید.
security-cancelled = رمزگشایی لغو شد.
security-damaged = رمزگشایی این پیام ممکن نیست: داده‌های رمزگذاری‌شده آسیب دیده یا تغییر کرده است.
security-decrypt-unavailable = رمزگشایی این پیام ممکن نیست: برای خواندن ایمیل رمزگذاری‌شده { $tool } را نصب کنید.
security-decrypt-failed = رمزگشایی این پیام ممکن نیست: { $reason }
security-unknown-signer = امضاکننده‌ای ناشناس
security-signed-verified = امضاشده توسط { $signer } · تأییدشده
security-signed-not-sender = امضاشده توسط { $signer } که فرستنده نیست
security-signed-untrusted = امضاشده توسط { $signer } با کلیدی که آن را غیرقابل‌اعتماد علامت زده‌اید
security-signed-unverified = امضاشده توسط { $signer } · کلید تأیید نشده است
security-bad-signature = امضای نامعتبر: این پیام پس از امضا تغییر کرده یا امضا جعلی است.
security-signature-expired = امضاشده توسط { $signer } · امضا منقضی شده است
security-key-expired = امضاشده توسط { $signer } · کلید از آن زمان منقضی شده است
security-key-revoked = امضاشده توسط { $signer } با کلیدی که باطل شده است
security-missing-key = با کلیدی امضا شده که آن را ندارید، بنابراین بررسی آن ممکن نیست
security-missing-key-id = با کلیدی امضا شده که آن را ندارید ({ $key })، بنابراین بررسی آن ممکن نیست
security-signature-unavailable = امضاشده؛ برای بررسی امضا { $tool } را نصب کنید
security-signature-error = بررسی امضا ممکن نشد.

## Remote images and pictures

remote-hidden = تصاویر این پیام پنهان شده‌اند.
remote-show = نمایش تصاویر
remote-always-show = همیشه از این فرستنده نمایش داده شود
remote-picture-use = استفاده
remote-picture-too-big = تصویری با حجم 8 مگابایت یا کمتر انتخاب کنید.
remote-picture-type = تصویری با قالب PNG، JPEG، GIF، WebP یا SVG انتخاب کنید.
remote-picture-read-failed = خواندن تصویر ممکن نیست: { $error }
remote-picture-keep-failed = نگه‌داشتن تصویر ممکن نیست: { $error }
remote-picture-remove-failed = حذف تصویر ممکن نیست: { $error }

## Attachments

attachment-count = { $count ->
    [one] { $count } پیوست
   *[other] { $count } پیوست
}
attachment-save = ذخیره
attachment-save-all = ذخیرهٔ همه
attachment-save-all-tooltip = ذخیرهٔ همهٔ پیوست‌ها در یک پوشه
attachment-save-here = ذخیره در اینجا
attachment-not-downloaded = این پیام بارگیری نشده است.
attachment-not-found = این پیوست در پیام پیدا نشد.
attachment-read-failed = خواندن { $name } ممکن نشد
attachment-numbered = پیوست { $number }
attachment-saved-all = { $count ->
    [one] { $count } فایل در { $place } ذخیره شد
   *[other] { $count } فایل در { $place } ذخیره شد
}
attachment-saved-some = { $total ->
    [one] { $saved } از { $total } فایل در { $place } ذخیره شد. ذخیرهٔ { $failed } ممکن نشد
   *[other] { $saved } از { $total } فایل در { $place } ذخیره شد. ذخیرهٔ { $failed } ممکن نشد
}
attachment-saved-to = در { $path } ذخیره شد
attachment-save-failed = ذخیرهٔ { $name } ممکن نشد: { $error }
attachment-open-failed = باز کردن { $name } ممکن نشد: { $error }
attachment-risky = این فایل ممکن است برنامه‌ای را اجرا کند، بنابراین Katna آن را باز نمی‌کند. به‌جای آن ذخیره‌اش کنید.
attachment-encrypted-open = این فایل رمزگذاری‌شده رسیده است. برای باز کردن آن در جای دیگر، ذخیره‌اش کنید.

## Printing

print-failed = چاپ ممکن نشد: { $error }
print-no-font = هیچ قلمی پیدا نشد
print-opened-as-pdf = به‌صورت PDF باز شد تا از آنجا چاپ شود.
print-not-downloaded = (هنوز بارگیری نشده است.)
print-encrypted = (رمزگذاری‌شده. برای چاپ متن آن، آن را در Katna Mail باز کنید.)
print-to = به: { $addresses }
print-cc = رونوشت: { $addresses }
