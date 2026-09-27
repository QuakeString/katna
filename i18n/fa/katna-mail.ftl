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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = برای خواندن پیوست‌های این پیام، آن را باز کنید.
text-copy = کپی
text-select-all = انتخاب همه

## Settings page: its tabs

settings-tab-general = عمومی
settings-tab-inbox = صندوق ورودی
settings-tab-accounts = حساب‌ها
settings-tab-subscriptions = اشتراک‌ها
settings-tab-appearance = ظاهر
settings-tab-shortcuts = میان‌برها
settings-tab-default-apps = برنامه‌های پیش‌فرض
settings-tab-folders-rules = پوشه‌ها و قوانین
settings-tab-compose = نوشتن
settings-tab-mcp-server = سرور MCP
settings-tab-feedback = بازخورد کاربر
settings-tab-experimental = آزمایشی

## Settings page: tabs still to come

settings-tab-subscriptions-coming = خبرنامه‌ها و فهرست‌های پستی‌ای را که دریافت می‌کنید ببینید و با یک کلیک اشتراکشان را لغو کنید.
settings-tab-folders-rules-coming = پوشه‌ها و برچسب‌ها را بسازید، نامشان را تغییر دهید، جابه‌جا یا پنهانشان کنید و انتخاب کنید کدام‌ها همگام شوند. قوانین، ایمیل‌های جدید را خودکار بر اساس فرستنده، موضوع یا کلمات مرتب می‌کنند، برچسب می‌زنند، بازارسال یا حذف می‌کنند.
settings-tab-mcp-server-coming = به دستیارهای هوش مصنوعی روی این رایانه اجازه دهید با نظر شما ایمیل‌هایتان را جستجو کنند، بخوانند و پیش‌نویس بنویسند.

## Settings > General

settings-general-conversations = نمای مکالمه
settings-general-conversations-group = گروه‌بندی پاسخ‌های یک ایمیل
settings-general-conversations-group-detail = یک سطر برای هر مکالمه در فهرست
settings-general-reading = خواندن
settings-general-newest-first = جدیدترین پیام اول
settings-general-newest-first-detail = مکالمه با آخرین پاسخ خود شروع می‌شود
settings-general-full-headers = نمایش سرایندهای کامل
settings-general-full-headers-detail = از، به، رونوشت، تاریخ و موضوع در هر پیام باز نشان داده می‌شوند
settings-general-full-names = نام کامل گیرندگان
settings-general-full-names-detail = «به من، Ada Lovelace» به‌جای «به من، Ada»
settings-general-mark-read = علامت‌گذاری به‌عنوان خوانده‌شده
settings-general-mark-read-now = به‌محض باز شدن
settings-general-mark-read-1s = پس از یک ثانیه باز بودن
settings-general-mark-read-3s = پس از 3 ثانیه باز بودن
settings-general-mark-read-never = فقط وقتی خودم آن را خوانده‌شده علامت بزنم
settings-general-reply-button = دکمهٔ پاسخ
settings-general-reply-all = پاسخ به همه
settings-general-reply-all-detail = دکمهٔ پاسخ کنار هر پیام به همه پاسخ می‌دهد، نه فقط به فرستنده
settings-general-remote-images = تصاویر وب
settings-general-remote-images-detail = بار کردن تصاویر یک پیام به فرستنده‌اش خبر می‌دهد که آن را باز کرده‌اید، چه زمانی و تقریباً از کجا. اگر خاموش باشد، هر پیام اول می‌پرسد و همیشه می‌توانید تصاویر یک فرستنده را نمایش دهید.
settings-general-remote-images-always = همیشه نمایش تصاویر
settings-general-remote-images-always-detail = در همهٔ پیام‌ها، نه فقط از فرستندگانی که به آن‌ها اعتماد دارید
settings-general-sending = ارسال
settings-general-sending-detail = مدتی که پیام ارسال‌شده منتظر می‌ماند تا بتوان آن را پس گرفت.
settings-general-offline = ایمیل آفلاین
settings-general-offline-detail = ایمیل‌های اخیر به‌طور کامل بارگیری می‌شوند تا بدون اتصال خوانده شوند. ایمیل‌های قدیمی‌تر هنگام باز کردن بارگیری می‌شوند.
settings-general-offline-days = { $count ->
    [one] { $count } روز
   *[other] { $count } روز
}
settings-general-offline-years = { $count ->
    [one] { $count } سال
   *[other] { $count } سال
}
settings-general-offline-all = همهٔ ایمیل‌ها
settings-general-offline-note = انتخاب روزهای کمتر، ایمیل‌های بارگیری‌شده را نگه می‌دارد. چیزی روی سرور تغییر نمی‌کند.
settings-general-notifications = اعلان‌ها
settings-general-notifications-detail = برای ایمیل‌های جدید صندوق ورودی، حتی وقتی Katna Mail بسته است.
settings-general-new-mail = اعلان ایمیل‌های جدید
settings-general-new-mail-detail = با «پاسخ به همه»، «علامت‌گذاری به‌عنوان خوانده‌شده» و «بایگانی»
settings-general-new-mail-sound = پخش صدا
settings-general-new-mail-sound-detail = صدای ایمیل جدید میزکار
settings-general-desktop = میزکار
settings-general-open-at-login = باز کردن Katna Mail هنگام ورود
settings-general-open-at-login-detail = در هر صورت، تا وقتی سرویس اجرا می‌شود، ایمیل‌ها هنگام ورود همگام می‌شوند
settings-general-tray = نمایش Katna در سینی سیستم
settings-general-tray-detail = با تعداد خوانده‌نشده‌ها و یک منو
settings-general-unread-badge = تعداد خوانده‌نشده‌ها روی نماد نوار وظیفه
settings-general-unread-badge-detail = تعداد پیام‌های خوانده‌نشدهٔ صندوق ورودی

## Settings > Inbox

settings-inbox-tabs = برگه‌های صندوق ورودی
settings-inbox-tabs-detail = صندوق ورودی را مانند وب‌سایت ارائه‌دهندهٔ ایمیلتان در برگه‌ها مرتب کنید.
settings-inbox-tabs-show = نمایش برگه‌های صندوق ورودی
settings-inbox-tabs-show-detail = اگر خاموش باشد، برای هر حساب یک فهرست نشان داده می‌شود
settings-inbox-no-accounts = برای انتخاب برگه‌ها، یک حساب اضافه کنید.
settings-inbox-tabs-automatic = خودکار: { $tabs } ({ $provider })
settings-inbox-tabs-off = بدون برگه
settings-inbox-tabs-gmail = اصلی، تبلیغات، اجتماعی، به‌روزرسانی‌ها، انجمن‌ها
settings-inbox-tabs-focused = متمرکز و سایر
settings-inbox-tabs-zoho = صندوق ورودی، خبرنامه‌ها و اعلان‌ها
settings-inbox-tabs-shown = برگه‌های نمایش داده‌شده. ایمیل‌های برگه‌ای که خاموش کنید در { $tab } می‌ماند.

## Settings > Appearance

settings-appearance-reading-pane = قاب خواندن
settings-appearance-reading-pane-detail = جایی که مکالمهٔ باز نشان داده می‌شود.
settings-appearance-pane-right = کنار فهرست
settings-appearance-pane-none = بدون تقسیم
settings-appearance-density = تراکم
settings-appearance-density-default = پیش‌فرض
settings-appearance-density-compact = فشرده
settings-appearance-scaling = مقیاس
settings-appearance-scaling-detail = همه‌چیز را در Katna Mail، علاوه بر مقیاس خود میزکار، بزرگ‌تر یا کوچک‌تر می‌کند: متن، نمادها، فاصله‌ها و جداکننده‌ها. ایمیلی که ارسال می‌کنید اندازهٔ قلم خودش را حفظ می‌کند. اندازه‌های خیلی کوچک ممکن است کلیک روی نمادها را دشوار کنند.
settings-appearance-theme = زمینه
settings-appearance-theme-system = مانند میزکار
settings-appearance-theme-light = روشن
settings-appearance-theme-dark = تیره
settings-appearance-desktop-colors = رنگ‌های میزکار
settings-appearance-desktop-colors-use = استفاده از رنگ‌های میزکار
settings-appearance-desktop-colors-use-detail = طرح رنگ و رنگ تأکیدی میزکار
settings-appearance-app-names = نام برنامه‌ها
settings-appearance-app-names-show = نمایش نام برنامه‌ها
settings-appearance-app-names-show-detail = نام‌ها زیر نمادهای برنامه‌ها در منتهی‌الیه سمت راست
settings-appearance-sender-pictures = تصاویر فرستندگان
settings-appearance-sender-pictures-show = نمایش لوگوی شرکت‌ها
settings-appearance-sender-pictures-show-detail = بر اساس دامنهٔ فرستنده جستجو می‌شود، هرگز بر اساس پیام، و یک هفته نگه داشته می‌شود
settings-appearance-important = نشانگرهای مهم
settings-appearance-important-show = نمایش نشانگرهای مهم
settings-appearance-important-show-detail = کنار هر پیام در فهرست
settings-appearance-message-width = عرض پیام
settings-appearance-message-width-limit = محدود کردن عرض پیام‌ها
settings-appearance-message-width-limit-detail = خواندن سطرهای بلند در پنجرهٔ عریض آسان‌تر است
settings-appearance-mail-colors = رنگ‌های ایمیل
settings-appearance-mail-colors-detail = بیشتر ایمیل‌ها برای صفحهٔ سفید طراحی شده‌اند. با زمینهٔ تیره، رنگ‌هایشان به رنگ‌های تیره‌ای تغییر می‌کند که خوب خوانده می‌شوند؛ اگر خاموش باشد، رنگ‌های فرستنده روی صفحه‌ای روشن حفظ می‌شود.
settings-appearance-dark-mail = رنگ‌های تیره برای ایمیل‌ها هم
settings-appearance-dark-mail-detail = فقط وقتی زمینه تیره است
settings-appearance-attachment-previews = پیش‌نمایش پیوست‌ها
settings-appearance-attachment-previews-show = نمایش پیش‌نمایش پیوست‌ها
settings-appearance-attachment-previews-show-detail = تصویری کوچک از محتوای هر فایل روی کارت آن

## Settings > Default apps

settings-default-apps-intro = جایی که پیوست‌ها با کلیک روی آن‌ها باز می‌شوند. نمایشگر همیشه می‌تواند فایل را در برنامه‌ای دیگر هم باز کند. برنامه‌های پیش‌فرض میزکار در تنظیمات خود آن تعیین می‌شوند.
settings-default-apps-pdf = فایل‌های PDF
settings-default-apps-pdf-detail = صفحه‌ها، با بزرگ‌نمایی.
settings-default-apps-pictures = تصاویر
settings-default-apps-pictures-detail = عکس‌ها (با جهت درست)، PNG، GIF، WebP، BMP، TIFF و SVG.
settings-default-apps-text = فایل‌های متنی
settings-default-apps-text-detail = متن ساده، گزارش‌ها، کد و سایر متن‌ها.
settings-default-apps-sheets = صفحه‌گسترده‌ها
settings-default-apps-sheets-detail = Excel (xlsx، xls)، OpenDocument (ods) و CSV.
settings-default-apps-documents = سندها
settings-default-apps-documents-detail = Word (docx) و متن OpenDocument (odt).
settings-default-apps-katna = نمایشگر Katna Mail
settings-default-apps-system = برنامهٔ پیش‌فرض میزکار
settings-default-apps-ask = هر بار پرسیدن که با کدام برنامه
settings-default-apps-after-saving = پس از ذخیره
settings-default-apps-show-folder = نمایش فایل‌های ذخیره‌شده در پوشه‌شان
settings-default-apps-show-folder-detail = مدیر فایل را با پیوست‌های ذخیره‌شدهٔ انتخاب‌شده باز می‌کند

## Settings > Compose

settings-compose-send-from = ارسال پیام‌های جدید از
settings-compose-send-from-detail = پاسخ‌ها و بازارسال‌ها همیشه از حسابی که در آن هستید ارسال می‌شوند.
settings-compose-send-from-current = حسابی که در آن هستید
settings-compose-send-on-replies = ارسال در پاسخ‌ها
settings-compose-send-on-replies-detail = کاری که «ارسال» در پاسخ یا بازارسال انجام می‌دهد. منوی کنار «ارسال» گزینهٔ دیگر را ارائه می‌کند.
settings-compose-send-plain = ارسال
settings-compose-send-archive = ارسال و بایگانی
settings-compose-signatures = امضاها
settings-compose-signatures-detail = زیر پیامتان، پس از سطر «--» اضافه می‌شود. امضای دیگری را در پنجرهٔ نوشتن انتخاب کنید.
settings-compose-untitled = بی‌عنوان
settings-compose-signature-name = نام، مثلاً کار
settings-compose-signature-first = امضای من
settings-compose-signature-numbered = امضای { $number }
settings-compose-signature-delete = حذف
settings-compose-signature-deleted = امضا حذف شد
settings-compose-signature-new = ایجاد جدید
settings-compose-no-signatures = هنوز امضایی نیست.
settings-compose-no-signature = بدون امضا
settings-compose-for-new-mail = برای ایمیل‌های جدید
settings-compose-for-replies = برای پاسخ‌ها و بازارسال‌ها
settings-compose-for-replies-detail = در مکالمه‌ای که پیامی را در آن امضا کرده‌اید، پاسخ به‌جای آن با همان امضا شروع می‌شود.
settings-compose-format = قالب
settings-compose-plain-text = نوشتن با متن ساده
settings-compose-plain-text-detail = ایمیل جدید بدون قالب‌بندی شروع می‌شود؛ در پنجرهٔ نوشتن می‌توان آن را تغییر داد
settings-compose-spelling = املا
settings-compose-spell-check = بررسی املا هنگام نوشتن
settings-compose-spell-check-detail = زیر واژه‌های غلط خط کشیده می‌شود و با کلیک راست پیشنهادها نشان داده می‌شوند
settings-compose-spell-desktop = زبان میزکار ({ $language })
settings-compose-templates = الگوها
settings-compose-templates-detail = ایمیل‌هایی را که زیاد می‌نویسید ذخیره کنید و ایمیل جدید یا پاسخ را از آن‌ها شروع کنید.

## Settings > Shortcuts

settings-shortcuts-set = مجموعهٔ میان‌برها
settings-shortcuts-set-detail = از کلیدهای یک برنامهٔ ایمیل که می‌شناسید شروع کنید. Cmd در اینجا همان Ctrl است. تغییرات خودتان روی مجموعه باقی می‌ماند و «بازگرداندن پیش‌فرض‌ها» به کلیدهای مجموعه برمی‌گردد.
settings-shortcuts-single = میان‌برهای تک‌کلیدی
settings-shortcuts-single-detail = کلیدهای بدون Ctrl یا Alt، مانند ایمیل وب: e بایگانی می‌کند، j و k جابه‌جا می‌شوند، / جستجو می‌کند. در فهرست و مکالمهٔ باز کار می‌کنند، هرگز هنگام تایپ.
settings-shortcuts-single-use = استفاده از میان‌برهای تک‌کلیدی
settings-shortcuts-single-use-detail = میان‌برهای Ctrl همیشه کار می‌کنند
settings-shortcuts-how = برای تغییر یک کلید روی آن کلیک کنید، یا برای افزودن روی +، سپس کلیدهای جدید را فشار دهید. Esc لغو می‌کند.
settings-shortcuts-restore = بازگرداندن پیش‌فرض‌ها
settings-shortcuts-no-key = بدون کلید
settings-shortcuts-press = کلیدها را فشار دهید…
settings-shortcuts-then = { $keys } سپس…
settings-shortcuts-moved = { $keys } اکنون به‌جای «{ $previous }»، «{ $action }» را انجام می‌دهد.
settings-shortcuts-single-off = میان‌برهای تک‌کلیدی خاموش‌اند، پس این کلید پس از روشن شدن آن‌ها کار می‌کند.
settings-shortcuts-restored = همهٔ میان‌برها دوباره کلیدهای مجموعهٔ خود را دارند.

## Settings search: the line under a result

settings-general-language-summary = زبان برنامه، تاریخ‌ها و اعداد
settings-general-reading-summary = جدیدترین پیام اول، سرایندهای کامل، نام کامل گیرندگان
settings-general-mark-read-summary = زمانی که مکالمهٔ باز خوانده‌شده علامت می‌خورد: بلافاصله، پس از 1 یا 3 ثانیه، یا دستی
settings-general-reply-button-summary = دکمهٔ پاسخ کنار هر پیام به همه پاسخ می‌دهد
settings-general-remote-images-summary = همیشه نمایش تصاویر همهٔ پیام‌ها
settings-general-sending-summary = واگرد ارسال: مدتی که پیام ارسال‌شده منتظر می‌ماند تا بتوان آن را پس گرفت
settings-general-offline-summary = چند روز از ایمیل‌های اخیر به‌طور کامل بارگیری شود تا بدون اتصال خوانده شود
settings-general-notifications-summary = اعلان‌های ایمیل جدید و صدای آن‌ها
settings-general-desktop-summary = باز کردن Katna Mail هنگام ورود، نماد سینی سیستم و تعداد خوانده‌نشده‌ها روی نماد نوار وظیفه
settings-accounts-accounts-summary = افزودن یا حذف حساب، یا تغییر تصویر آن
settings-appearance-density-summary = سطرهای پیش‌فرض یا فشرده در فهرست
settings-appearance-scaling-summary = بزرگ‌تر یا کوچک‌تر کردن همه‌چیز: متن، نمادها، فاصله‌ها و جداکننده‌ها
settings-appearance-theme-summary = مانند میزکار، روشن یا تیره
settings-appearance-sender-pictures-summary = لوگوی شرکت‌ها، که بر اساس دامنهٔ فرستنده جستجو می‌شود
settings-appearance-important-summary = نشانگر مهم کنار هر پیام در فهرست
settings-appearance-mail-colors-summary = رنگ‌های تیره برای ایمیل‌های HTML در زمینهٔ تیره، یا رنگ‌های فرستنده
settings-appearance-attachment-previews-summary = تصویری کوچک از محتوای هر پیوست
settings-shortcuts-set-summary = شروع از کلیدهای Gmail، Inbox by Gmail، Apple Mail، Outlook یا Thunderbird
settings-shortcuts-single-summary = کلیدهای بدون Ctrl یا Alt، مانند ایمیل وب
settings-default-apps-pdf-summary = جایی که پیوست‌های PDF باز می‌شوند
settings-default-apps-pictures-summary = جایی که عکس‌ها و تصاویر باز می‌شوند
settings-default-apps-text-summary = جایی که متن ساده، گزارش‌ها و کد باز می‌شوند
settings-default-apps-sheets-summary = جایی که فایل‌های Excel، OpenDocument و CSV باز می‌شوند
settings-default-apps-documents-summary = جایی که Word و متن OpenDocument باز می‌شوند
settings-default-apps-after-saving-summary = نمایش پیوست‌های ذخیره‌شده در پوشه‌شان
settings-compose-send-from-summary = حسابی که ایمیل جدید از آن ارسال می‌شود: حسابی که در آن هستید، یا همیشه همان حساب
settings-compose-send-on-replies-summary = «ارسال» یا «ارسال و بایگانی» مکالمه، در پاسخ‌ها و بازارسال‌ها
settings-compose-signatures-summary = زیر پیامتان، پس از سطر «--» اضافه می‌شود
settings-compose-for-new-mail-summary = امضایی که ایمیل جدید با آن شروع می‌شود
settings-compose-for-replies-summary = امضایی که پاسخ‌ها و بازارسال‌ها با آن شروع می‌شوند
settings-compose-format-summary = نوشتن ایمیل جدید با متن ساده
settings-compose-spelling-summary = بررسی املا هنگام نوشتن، و زبان واژه‌نامه
settings-compose-templates-summary = به‌زودی: ایمیل‌هایی را که زیاد می‌نویسید ذخیره کنید و ایمیل جدید یا پاسخ را از آن‌ها شروع کنید
settings-feedback-crash-reports-summary = ذخیرهٔ گزارش‌های خرابی روی این رایانه هنگام از کار افتادن Katna Mail یا سرویس پس‌زمینهٔ آن
settings-feedback-saved-summary = مشاهده، کپی یا حذف گزارش‌های خرابی ذخیره‌شده روی این رایانه
settings-feedback-help-improve-summary = ارسال گزارش‌های خرابی برای کمک به رفع مشکل؛ خاموش مگر اینکه روشنش کنید
settings-experimental-blur-summary = میزکار به‌صورت مات از پشت نوار بالا دیده می‌شود و منوها مانند شیشهٔ مات هستند
settings-search-shortcut = میان‌بر صفحه‌کلید
settings-search-tab = برگهٔ تنظیمات
settings-search-none = هیچ تنظیمی با «{ $query }» مطابقت ندارد.
settings-search-results = تنظیمات مطابق با «{ $query }»

## Quick settings (the panel that slides in from the right)

quick-title = تنظیمات سریع
quick-see-all = دیدن همهٔ تنظیمات
quick-reading-pane = قاب خواندن
quick-pane-right = کنار فهرست
quick-pane-none = بدون تقسیم
quick-density = تراکم
quick-density-default = پیش‌فرض
quick-density-compact = فشرده
quick-theme = زمینه
quick-theme-system = مانند میزکار
quick-theme-light = روشن
quick-theme-dark = تیره
quick-desktop-colors = رنگ‌های میزکار
quick-desktop-colors-detail = طرح رنگ و رنگ تأکیدی میزکار
quick-app-names = نام برنامه‌ها
quick-app-names-detail = نام‌ها زیر نمادهای برنامه‌ها در منتهی‌الیه سمت راست
quick-inbox-tabs = برگه‌های صندوق ورودی
quick-inbox-tabs-detail = برگه‌های ارائه‌دهندهٔ ایمیل هر حساب
quick-choose-tabs = انتخاب برگه‌ها
quick-choose-tabs-detail = برای هر حساب، در تنظیمات
quick-sending = ارسال
quick-undo-send = واگرد ارسال
quick-undo-send-off = خاموش
quick-undo-send-seconds = { $seconds } ث
quick-signatures = امضاها
quick-signatures-none = هنوز هیچ
quick-signatures-one = { $name }، پیش‌فرض
quick-signatures-many = { $count ->
    [one] { $count } امضا؛ { $name } پیش‌فرض
   *[other] { $count } امضا؛ { $name } پیش‌فرض
}
quick-signatures-no-default = { $count ->
    [one] { $count }، بدون پیش‌فرض
   *[other] { $count }، بدون پیش‌فرض
}
quick-signature-untitled = بی‌عنوان
quick-threading = رشته‌بندی ایمیل‌ها
quick-conversation-view = نمای مکالمه
quick-conversation-view-detail = گروه‌بندی پاسخ‌های یک ایمیل
quick-help = راهنما
quick-tour = گشتی در برنامه
quick-whats-new = تازه‌ها
quick-about = دربارهٔ Katna

## Settings: opening at login

settings-open-at-login-failed = تغییر باز شدن هنگام ورود ممکن نشد: { $error }

## Settings > Appearance > Scaling

scale-letter = ع
scale-percent = { $percent }٪
scale-reset = بازگشت به { $percent }٪

## Settings > Experimental > Look & Feel

look-intro = ویژگی‌هایی که هنوز در حال آزمایش‌اند. ممکن است تغییر کنند یا حذف شوند.
look-heading = ظاهر و احساس
look-window-frame = قاب پنجره
look-window-frame-detail = چه کسی نوار عنوان، دکمه‌های پنجره، گوشه‌ها و سایه را می‌کشد.
look-frame-native-kde = بومی: قاب KDE، با زمینهٔ Plasma شما
look-frame-native = بومی: قاب میزکار
look-frame-katna = Katna: نوار بالا نوار عنوان می‌شود
look-frame-katna-note-named = Katna گوشه‌های گرد و سایهٔ خودش را می‌کشد. قاب دیگر از زمینهٔ { $desktop } پیروی نمی‌کند؛ قوانین پنجره همچنان اعمال می‌شوند.
look-frame-katna-note = Katna گوشه‌های گرد و سایهٔ خودش را می‌کشد. قاب دیگر از زمینهٔ میزکار پیروی نمی‌کند؛ قوانین پنجره همچنان اعمال می‌شوند.
look-frame-client-side = میزکار شما قاب را به هر برنامه واگذار می‌کند، پس Katna از قبل قاب خودش را می‌کشد.
look-blurred-background = پس‌زمینهٔ مات
look-blurred-background-detail = میزکار به‌صورت مات از پشت نوار بالا و پوشه‌ها دیده می‌شود و منوها و پنجره‌های بازشو مانند شیشهٔ مات هستند.
look-blur = مات کردن پشت پنجره
look-blur-detail = ایمیل‌ها روی کارت‌های توپر می‌مانند تا متن کنتراست خود را حفظ کند
look-blur-off-kde = جلوهٔ محو KDE خاموش است. «محو» را در «تنظیمات سیستم»، «مدیریت پنجره»، «جلوه‌های میزکار» روشن کنید، سپس Katna Mail را دوباره باز کنید.
look-blur-none-gnome = GNOME پشت پنجره‌ها را مات نمی‌کند.
look-blur-none-x11 = مدیر پنجرهٔ شما پشت پنجره‌ها را مات نمی‌کند.
look-blur-none-wayland = کامپوزیتور شما پشت پنجره‌ها را مات نمی‌کند.

## Settings > User feedback (crash reports)

feedback-intro-sending = گزارش‌های خرابی جدید برای کمک به رفع مشکل ارسال می‌شوند. هیچ چیز دیگری از این رایانه خارج نمی‌شود.
feedback-intro-local = Katna هیچ چیزی به هیچ جا نمی‌فرستد. گزارش‌های خرابی روی این رایانه می‌مانند تا آن‌ها را ببینید یا به گزارش اشکال پیوست کنید.
feedback-crash-reports = گزارش‌های خرابی
feedback-crash-reports-detail = هنگام از کار افتادن Katna Mail یا سرویس پس‌زمینهٔ آن نوشته می‌شوند.
feedback-save = ذخیرهٔ گزارش‌های خرابی روی این رایانه
feedback-save-detail = پوشهٔ خانگی، نام کاربر و رایانه و نشانی‌های ایمیل شما حذف می‌شوند
feedback-saved = گزارش‌های خرابی ذخیره‌شده
feedback-saved-detail = { $count ->
    [one] جدیدترین { $count } گزارش نگه داشته می‌شود.
   *[other] جدیدترین { $count } گزارش نگه داشته می‌شوند.
}
feedback-help-improve = کمک به بهبود Katna
feedback-help-improve-detail = خاموش است مگر اینکه روشنش کنید، و هر زمان می‌توانید اینجا خاموشش کنید.
feedback-send = ارسال گزارش‌های خرابی
feedback-send-detail = گزارش ذخیره‌شده، دقیقاً همان‌طور که اینجا می‌توانید ببینید، به ردیاب خرابی Katna (Sentry، در اتحادیهٔ اروپا) می‌رود. بدون نشانی IP، پیام یا نشانی ایمیل
feedback-none-saved = هیچ گزارش خرابی ذخیره نشده است.
feedback-delete-all = حذف همه
feedback-app-daemon = سرویس پس‌زمینه
feedback-report-sent = { $date } · ارسال شد
feedback-view = مشاهده
feedback-view-tooltip = باز کردن گزارش
feedback-copy-tooltip = کپی برای چسباندن در گزارش اشکال
feedback-copied = گزارش خرابی کپی شد.
feedback-deleted-all = گزارش‌های خرابی حذف شدند.
feedback-read-failed = خواندن گزارش خرابی ممکن نشد: { $error }
feedback-delete-failed = حذف گزارش خرابی ممکن نشد: { $error }
feedback-delete-all-failed = حذف گزارش‌های خرابی ممکن نشد: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _پرونده
desktop-menu-new-message = _پیام جدید
desktop-menu-quit = _خروج
desktop-menu-edit = _ویرایش
desktop-menu-undo = _واگرد
desktop-menu-select-all = _انتخاب همه
desktop-menu-select-none = _لغو انتخاب
desktop-menu-find = _یافتن…
desktop-menu-view = _نما
desktop-menu-folder-list = _نمایش فهرست پوشه‌ها
desktop-menu-refresh = _بازخوانی
desktop-menu-go = _رفتن
desktop-menu-inbox = _صندوق ورودی
desktop-menu-starred = _ستاره‌دار
desktop-menu-sent = _ارسال‌شده
desktop-menu-drafts = _پیش‌نویس‌ها
desktop-menu-all-mail = _همهٔ ایمیل‌ها
desktop-menu-next = _مکالمهٔ بعدی
desktop-menu-previous = _مکالمهٔ قبلی
desktop-menu-message = _پیام
desktop-menu-open = _باز کردن
desktop-menu-reply = _پاسخ
desktop-menu-reply-all = _پاسخ به همه
desktop-menu-forward = _بازارسال
desktop-menu-archive = _بایگانی
desktop-menu-delete = _حذف
desktop-menu-spam = _گزارش هرزنامه
desktop-menu-move-to = _انتقال به…
desktop-menu-mark-read = _علامت‌گذاری به‌عنوان خوانده‌شده
desktop-menu-mark-unread = _علامت‌گذاری به‌عنوان خوانده‌نشده
desktop-menu-star = _افزودن ستاره
desktop-menu-important = _علامت‌گذاری به‌عنوان مهم
desktop-menu-not-important = _علامت‌گذاری به‌عنوان غیرمهم
desktop-menu-settings = _تنظیمات
desktop-menu-quick-settings = _تنظیمات سریع
desktop-menu-configure = _پیکربندی Katna Mail…
desktop-menu-help = _راهنما
desktop-menu-shortcuts = _میان‌برهای صفحه‌کلید
desktop-menu-whats-new = _تازه‌ها
desktop-menu-about = _دربارهٔ Katna

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = جابه‌جایی
shortcut-group-actions = عملیات
shortcut-group-go-to = رفتن به
shortcut-group-app = برنامه

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = مکالمهٔ بعدی
shortcut-previous = مکالمهٔ قبلی
shortcut-down = پایین رفتن در فهرست
shortcut-up = بالا رفتن در فهرست
shortcut-first = اولین مورد فهرست
shortcut-last = آخرین مورد فهرست
shortcut-page-down = یک صفحه پایین در فهرست
shortcut-page-up = یک صفحه بالا در فهرست
shortcut-open = باز کردن مکالمه
shortcut-back = بازگشت به فهرست
shortcut-scroll-down = پیمایش به پایین
shortcut-scroll-up = پیمایش به بالا
shortcut-scroll-page-down = پیمایش یک صفحه به پایین
shortcut-scroll-page-up = پیمایش یک صفحه به بالا
shortcut-compose = نوشتن
shortcut-reply = پاسخ
shortcut-reply-all = پاسخ به همه
shortcut-forward = بازارسال
shortcut-archive = بایگانی
shortcut-delete = حذف
shortcut-spam = گزارش هرزنامه
shortcut-move-to = انتقال به
shortcut-mark-read = علامت‌گذاری به‌عنوان خوانده‌شده
shortcut-mark-unread = علامت‌گذاری به‌عنوان خوانده‌نشده
shortcut-star = افزودن یا برداشتن ستاره
shortcut-important = علامت‌گذاری به‌عنوان مهم
shortcut-not-important = علامت‌گذاری به‌عنوان غیرمهم
shortcut-check = علامت زدن مکالمه
shortcut-select-all = علامت زدن همهٔ مکالمه‌ها
shortcut-select-none = برداشتن علامت همهٔ مکالمه‌ها
shortcut-undo = واگرد آخرین کار
shortcut-go-inbox = صندوق ورودی
shortcut-go-starred = ستاره‌دار
shortcut-go-sent = ارسال‌شده
shortcut-go-drafts = پیش‌نویس‌ها
shortcut-go-all = همهٔ ایمیل‌ها
shortcut-search = جستجوی ایمیل
shortcut-navigation = نمایش یا جمع کردن منو
shortcut-quick-settings = تنظیمات سریع
shortcut-settings = همهٔ تنظیمات
shortcut-shortcuts = میان‌برهای صفحه‌کلید
shortcut-reload = بررسی ایمیل جدید
shortcut-quit = خروج

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } سپس { $second }

## Settings > Accounts

accounts-folder-pane = قاب پوشه‌ها
accounts-folder-pane-detail = پوشه‌های کدام حساب‌ها در قاب سمت راست نشان داده شود.
accounts-shown-one = یک حساب در هر زمان؛ جابه‌جایی از کارت حساب
accounts-shown-all = همهٔ حساب‌ها، یکی پس از دیگری
accounts-row = حساب‌ها
accounts-row-detail = حذف یک حساب، نسخهٔ Katna از ایمیل‌های آن را از این رایانه پاک می‌کند. ایمیل‌ها روی سرور می‌مانند.
accounts-none = هنوز حسابی نیست.
accounts-kind-imported = واردشده
accounts-picture-reset = استفاده از تصویر میزکار
accounts-picture-change = تغییر تصویر
accounts-remove = حذف
accounts-delete-all-row = حذف همهٔ داده‌ها
accounts-delete-all-row-detail = شروع دوباره، مانند یک نصب تازه.
accounts-delete-all-about = همهٔ حساب‌ها، همهٔ ایمیل‌های ذخیره‌شده، مخاطبین و تقویم‌ها، نمایهٔ جستجو، تنظیمات و گذرواژه‌های ذخیره‌شدهٔ شما را از این رایانه حذف می‌کند. چیزی روی سرورهای ایمیل شما تغییر نمی‌کند.
accounts-delete-all-open = حذف همهٔ داده‌های Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } از Katna حذف شد.
accounts-removed = { $address } از Katna حذف شد. ایمیل‌های آن هنوز روی سرور است.
accounts-all-deleted = همهٔ داده‌های Katna از این رایانه حذف شد.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } حذف شود؟
accounts-remove-confirm = حذف حساب
accounts-removing = در حال حذف…
accounts-remove-local-mail = { $folders ->
    [0] همهٔ ایمیل‌های واردشده به این حساب
    [one] همهٔ ایمیل‌های واردشده به این حساب، در پوشهٔ آن
   *[other] همهٔ ایمیل‌های واردشده به این حساب، در { $folders } پوشهٔ آن
}
accounts-remove-local-settings = تنظیمات Katna آن
accounts-remove-mail = { $folders ->
    [0] همهٔ ایمیل‌های این حساب که Katna ذخیره کرده است
    [one] همهٔ ایمیل‌های این حساب که Katna در پوشهٔ آن ذخیره کرده است
   *[other] همهٔ ایمیل‌های این حساب که Katna در { $folders } پوشهٔ آن ذخیره کرده است
}
accounts-remove-outbox = پیام‌های آن که در صندوق خروجی منتظرند
accounts-remove-settings = گذرواژهٔ ذخیره‌شده و تنظیمات Katna آن
accounts-delete-all-title = همهٔ داده‌های Katna حذف شود؟
accounts-delete-all-confirm = حذف همه‌چیز
accounts-deleting = در حال حذف…
accounts-delete-all-accounts = همهٔ حساب‌ها، و همهٔ ایمیل‌ها و پیوست‌هایی که Katna ذخیره کرده است
accounts-delete-all-contacts = مخاطبین، تقویم‌ها و نمایهٔ جستجو
accounts-delete-all-settings = همهٔ تنظیمات، امضاها و میان‌برهای صفحه‌کلید
accounts-delete-all-passwords = همهٔ گذرواژه‌های ذخیره‌شده
accounts-deleted-heading = از این رایانه حذف می‌شود:
accounts-cannot-undo = این کار برگشت‌پذیر نیست.
accounts-server-delete-all = چیزی روی سرورهای ایمیل شما تغییر نمی‌کند: ایمیل‌هایتان آنجا می‌ماند و افزودن دوبارهٔ حساب آن‌ها را دوباره بارگیری می‌کند. ایمیل‌هایی که از فایل‌ها وارد شده‌اند فقط در Katna هستند؛ به خود فایل‌ها دست زده نمی‌شود.
accounts-server-local = این ایمیل‌ها از فایل‌ها وارد شده‌اند، پس تنها نسخهٔ آن‌ها در Katna است. به فایل‌های مبدأ دست زده نمی‌شود؛ برای بازگرداندن ایمیل‌ها، دوباره واردشان کنید.
accounts-server-remove = چیزی روی سرور ایمیل تغییر نمی‌کند: ایمیل‌هایتان آنجا می‌ماند و افزودن دوبارهٔ حساب آن‌ها را دوباره بارگیری می‌کند.
accounts-confirm-word = حذف
accounts-confirm-placeholder = «{ accounts-confirm-word }» را تایپ کنید
accounts-confirm-prompt = برای تأیید، «{ accounts-confirm-word }» را تایپ کنید:
accounts-cancel = لغو
