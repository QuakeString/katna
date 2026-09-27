# Katna Mail, Persian (فارسی).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
