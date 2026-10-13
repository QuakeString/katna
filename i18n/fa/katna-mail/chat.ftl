# Katna Mail, Persian (فارسی).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.
chat-heading = خواندن
chat-view = مکالمه‌ها به شکل گفتگو
chat-view-detail = ایمیل بین افراد مثل یک گفتگوی گروهی خوانده می‌شود: برای هر ایمیل یک حباب فقط با آنچه نوشته شده، و ایمیل‌های خودتان در سمت راست. خبرنامه‌ها نمای معمول را حفظ می‌کنند.
chat-view-switch = نمایش مکالمه‌ها به شکل گفتگو
chat-view-switch-detail = ایمیل نقل‌قول‌شده و امضاها در هر حباب پشت ··· می‌مانند
chat-switch-chat = گفتگو
chat-switch-mail = ایمیل
chat-people = { $names } و شما · { $count ->
    [one] { $count } ایمیل
   *[other] { $count } ایمیل
}
chat-people-heading = { $count ->
    [one] در این گفتگو · { $count } نفر
   *[other] در این گفتگو · { $count } نفر
}
chat-member-mails = { $count ->
    [0] بدون ایمیل
    [one] { $count } ایمیل
   *[other] { $count } ایمیل
}
chat-today = امروز
chat-yesterday = دیروز
chat-added = { $who }، { $names } را اضافه کرد
chat-renamed = { $who } موضوع را به «{ $subject }» تغییر داد
chat-you = شما
chat-not-downloaded = هنوز بارگیری نشده
chat-forwarded = بازارسال‌شده
chat-show-quoted = نمایش ایمیل نقل‌قول‌شده و امضا
chat-hide-quoted = پنهان کردن ایمیل نقل‌قول‌شده و امضا
chat-hide-dots = پنهان کردن ···
chat-show-card = نمایش کارت او
chat-reply-all = پاسخ به همه
chat-more = بیشتر
chat-reply-only = پاسخ فقط به { $name }
chat-forward = بازارسال
chat-copy-text = کپی متن
chat-show-as-mail = نمایش به شکل ایمیل
chat-go-down = رفتن به جدیدترین ایمیل
chat-pin = سنجاق کردن به بالا
chat-pin-file = سنجاق کردن فایل به بالا
chat-unpin = برداشتن سنجاق
chat-unpin-file = برداشتن سنجاق فایل
chat-pinned-of = سنجاق { $at } از { $count }
chat-pins-all = همهٔ سنجاق‌ها
chat-pins-heading = سنجاق‌شده · { $count } از { $most }
chat-pins-drag = برای تغییر ترتیب بکشید
chat-pin-from-mail = ایمیل از { $name } · { $when }
chat-pin-from-file = فایل از { $name } · { $when }
chat-pin-from-text = متن از { $name } · { $when }
chat-pins-full = این گفتگو از قبل ۵ سنجاق دارد
chat-pins-replace-title = جایگزینی یک سنجاق
chat-pins-replace-hint = هر گفتگو حداکثر ۵ سنجاق دارد. یکی را برای برداشتن انتخاب کنید.
chat-pins-replace = جایگزینی
chat-pins-cancel = لغو
chat-undo = واگرد
chat-reply-to = پاسخ به { $names }
chat-send = ارسال (Ctrl+Enter). برای گزینه‌های بیشتر راست‌کلیک کنید یا نگه دارید
chat-send-now = ارسال اکنون
chat-attach = پیوست
chat-attach-photo = عکس
chat-attach-file = فایل
chat-attach-library = از فایل‌ها
chat-attach-template = الگو
chat-attach-signature = امضا
chat-replying-to = در حال پاسخ به { $name }
chat-reply-newest = پاسخ به جدیدترین ایمیل

## The attach picker (paperclip > From Files)

picker-title = پیوست از فایل‌ها
picker-search = جستجوی نام‌ها، افراد، موضوع‌ها
picker-search-drive = جستجو در این درایو
picker-mail-files = فایل‌های ایمیل
picker-this-chat = این مکالمه
picker-this-computer = این رایانه…
picker-in-chat = در این مکالمه
picker-recent = اخیر
picker-preview = پیش‌نمایش
picker-cancel = لغو
picker-attach = پیوست
picker-attach-count = پیوست { $count }
picker-selected = { $count } انتخاب‌شده
picker-of-limit = از { $limit }
picker-in-mail = { $size } در ایمیل
picker-drive-links = { $count ->
    [one] 1 به‌صورت پیوند Google Drive
   *[other] { $count } به‌صورت پیوند Google Drive
}
picker-onedrive-links = { $count ->
    [one] 1 به‌صورت پیوند OneDrive
   *[other] { $count } به‌صورت پیوند OneDrive
}
picker-over = { $size }، بیشتر از { $limit } که یک ایمیل می‌تواند ببرد
picker-getting = { $count ->
    [one] در حال دریافت فایل از درایو…
   *[other] در حال دریافت { $count } فایل از درایو…
}
picker-some-failed = { $count ->
    [one] یک فایل خوانده نشد
   *[other] { $count } فایل خوانده نشد
}
