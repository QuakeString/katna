# Katna Mail, Persian (فارسی).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = دربارهٔ Katna
about-tagline = ایمیل و تقویم برای میزکار لینوکس
about-whats-new = تازه‌ها
about-changelog = فهرست تغییرات
about-source = کد منبع
about-coffee = یک قهوه مهمانم کنید
about-coming-soon = به‌زودی
about-follow = نویسنده را دنبال کنید
about-love-title = ساخته‌شده با عشق برای Rust و KDE و لینوکس
about-love-text = Rust نوشتن یک برنامهٔ ایمیل سریع و امن را لذت‌بخش می‌کند: Katna هیچ کد unsafe ندارد. میزکار Plasma از KDE و مجموعهٔ PIM آن الهام‌بخش Katna بودند، و لینوکس و جامعهٔ نرم‌افزار آزاد زمینی را می‌سازند که Katna بر آن ایستاده است. سپاس، و سپاس از کتابخانه‌های زیر.
about-kde-text = KDE میزکاری را می‌سازد که Katna بیش از هر جای دیگر در آن احساس راحتی می‌کند، و آن را داوطلبان می‌سازند و مردمی مانند شما هزینه‌اش را می‌پردازند. اگر از Plasma یا برنامه‌های KDE لذت می‌برید، لطفاً کمک مالی به KDE را در نظر بگیرید.
about-donate-kde = کمک مالی به KDE
about-gpui-title = ساخته‌شده بر پایهٔ GPUI، از پروژهٔ Zed
about-gpui-text = تمام رابط کاربری Katna Mail بر پایهٔ GPUI ساخته شده است، چارچوب رابط کاربری سریع و شتاب‌گرفته با GPU که Zed Industries برای ویرایشگر Zed ساخت. هر پیکسل، پویانمایی و پنجره‌ای که می‌بینید با آن کشیده می‌شود. سپاس، تیم Zed، که آن را آشکارا می‌سازید. Apache-2.0.
about-gpui-github = GPUI در GitHub
about-personal-title = یک پروژهٔ شخصی
about-personal-text = Katna Mail نمی‌کوشد نو یا انقلابی باشد. این همان برنامهٔ ایمیلی است که نویسنده‌اش می‌خواست، و ویژگی‌ها و ظاهرش از Gmail و Mailspring و Thunderbird وام گرفته شده است. ساختنش تنها به لطف پیشرفت LLMها ممکن شد.
about-built-on = ساخته‌شده بر پایهٔ نرم‌افزار آزاد
about-credit-pimalaya = IMAP، SMTP و ورود به حساب (io-imap، io-smtp، io-sasl)
about-credit-imap-codec = خواندن و نوشتن IMAP
about-credit-tantivy = جستجو
about-credit-sqlite = انبارهٔ ایمیل
about-credit-rustls = اتصال‌های امن
about-credit-mail-parser = خواندن ایمیل، از Stalwart Labs
about-credit-html5ever = ایمیل HTML، از پروژهٔ Servo
about-credit-zbus = گفتگو با میزکار از راه D-Bus و درگاه‌ها
about-credit-oo7 = گذرواژه‌ها در دسته‌کلید میزکار
about-credit-hayro = دیدن و چاپ PDF
about-credit-calamine = پیش‌نمایش صفحه‌گسترده‌ها
about-credit-resvg = تصویرهای SVG
about-credit-jiff = تاریخ‌ها و منطقه‌های زمانی
about-credit-spellbook = غلط‌یابی املایی، از ویرایشگر Helix
about-credit-smol = انجام چند کار هم‌زمان
about-all-libraries = همهٔ کتابخانه‌هایی که Katna به کار می‌برد ({ $count })
about-library-authors = از { $authors }
about-license = Katna نرم‌افزار آزاد است، تحت GNU GPL نسخهٔ ۳ یا بالاتر.
about-close = بستن

## What’s new (shown after an update)

whats-new-title = تازه‌های Katna Mail
whats-new-updated = به نسخهٔ { $version } به‌روز شد
whats-new-version = نسخهٔ { $version }
whats-new-more = { $count ->
    [one] و { $count } مورد دیگر در فهرست کامل تغییرات.
   *[other] و { $count } مورد دیگر در فهرست کامل تغییرات.
}
whats-new-changelog = فهرست کامل تغییرات
whats-new-got-it = متوجه شدم

## First run: welcome page

onboarding-welcome-title = به Katna Mail خوش آمدید
onboarding-welcome-lead = ایمیل شما روی رایانهٔ خودتان: جستجوی سریع، خواندن بدون اینترنت، و خصوصی.
onboarding-fast-title = سریع، حتی بدون اینترنت
onboarding-fast-text = Katna یک نسخه از ایمیل‌های شما را همین‌جا نگه می‌دارد، پس باز کردن و جستجوی آن‌ها فوری است، با اتصال یا بدون آن.
onboarding-providers-title = با ایمیل شما کار می‌کند
onboarding-providers-text = Gmail، Outlook، Yahoo، iCloud و هر حساب IMAP یا POP دیگر.
onboarding-private-title = خصوصی
onboarding-private-text = ایمیل شما مستقیم از سرویس‌دهنده‌تان به این رایانه می‌آید. هیچ سرور Katna آن را نمی‌بیند.
onboarding-get-started = شروع کنیم

## First run: adding an account

onboarding-service-checking = در حال بررسی سرویس پس‌زمینهٔ Katna…
onboarding-service-running = سرویس پس‌زمینهٔ Katna در حال اجراست.
onboarding-service-missing = سرویس پس‌زمینهٔ Katna اجرا نمی‌شود
onboarding-service-start = این سرویس ایمیل‌های شما را دریافت و ارسال می‌کند. آن را از پایانه اجرا کنید و دوباره بررسی کنید:
onboarding-check-again = بررسی دوباره
onboarding-account-title = حساب ایمیل خود را اضافه کنید
onboarding-account-lead = نشانی ایمیل و گذرواژهٔ خود را بنویسید تا Katna تنظیمات سرور را پیدا کند. Gmail و Yahoo و iCloud به گذرواژهٔ برنامه نیاز دارند که در تنظیمات امنیتی حسابتان ساخته می‌شود.
onboarding-add-account = افزودن حساب
onboarding-back = برگشت

## First run: choosing the look

onboarding-look-title = آن را از آنِ خود کنید
onboarding-look-lead = انتخاب کنید ایمیل چگونه باز شود و Katna چه ظاهری داشته باشد. هر زمان در تنظیمات سریع می‌توانید این‌ها را تغییر دهید.
onboarding-reading-pane = قاب خواندن
onboarding-pane-right = کنار فهرست
onboarding-pane-none = بدون تقسیم
onboarding-theme = زمینه
onboarding-theme-system = سیستم
onboarding-theme-light = روشن
onboarding-theme-dark = تیره
onboarding-density = تراکم
onboarding-density-default = پیش‌فرض
onboarding-density-compact = فشرده
onboarding-continue = ادامه

## First run: done

onboarding-ready-title = همه‌چیز آماده است
onboarding-ready-lead = Katna در حال دریافت ایمیل‌های شماست. هر ایمیل همین که برسد نمایش داده می‌شود، و ایمیل‌های تازه خودشان ظاهر می‌شوند.
onboarding-ready-lead-address = Katna در حال دریافت ایمیل‌های { $address } است. هر ایمیل همین که برسد نمایش داده می‌شود، و ایمیل‌های تازه خودشان ظاهر می‌شوند.
onboarding-ready-tour = یک گشت یک‌دقیقه‌ای بزنید تا ببینید هر چیز کجاست؟
onboarding-skip = فعلاً رد شو
onboarding-take-tour = گشتی در برنامه

## Asking to send crash reports (on its own and on the first-run pages)

share-title = به بهبود Katna کمک کنید
share-lead = وقتی Katna خراب می‌شود، گزارشی روی این رایانه ذخیره می‌کند. فرستادن این گزارش‌ها به رفع مشکل کمک می‌کند. هر زمان می‌توانید این را در تنظیمات > بازخورد کاربر تغییر دهید.
share-sent = چه چیزی فرستاده می‌شود
share-sent-detail = گزارش خرابی، همان‌طور که در تنظیمات می‌بینید: چه چیزی و کجای Katna خراب شد، نسخه، سیستم لینوکس و میزکار شما، و آخرین خط‌های گزارش کار Katna که ممکن است نام پوشه‌های ایمیل را داشته باشند.
share-never-sent = چه چیزی هرگز فرستاده نمی‌شود
share-never-sent-detail = پیام‌ها، مخاطبین، گذرواژه‌ها، نشانی IP، نام کاربری یا نام رایانهٔ شما. نشانی‌های ایمیل از گزارش حذف می‌شوند.
share-where = به کجا می‌رود
share-where-detail = به ردیاب خرابی Katna در Sentry که در اتحادیهٔ اروپا نگهداری می‌شود. هیچ شناسه‌ای گزارش‌ها را به شما پیوند نمی‌دهد.
share-dont-send = نفرست
share-send = ارسال گزارش‌های خرابی
share-sending = گزارش‌های خرابی فرستاده می‌شوند. سپاس.
share-local = گزارش‌های خرابی روی این رایانه می‌مانند.

## The tour (cards pointing at each part of the window)

tour-welcome-title = به Katna Mail خوش آمدید
tour-welcome-text = یک گشت یک‌دقیقه‌ای نشان می‌دهد هر چیز کجاست.
tour-not-now = حالا نه
tour-start = گشتی در برنامه
tour-close = بستن
tour-skip = رد شدن از گشت
tour-back = قبلی
tour-done = تمام
tour-next = بعدی
tour-step = { $step } از { $total }
tour-compose-title = نوشتن پیام
tour-compose-text = «نوشتن» یک پیام تازه در پایین پنجره باز می‌کند تا بتوانید در حین نوشتن به خواندن ادامه دهید.
tour-search-title = جستجو در همهٔ ایمیل‌ها
tour-search-text = جستجو بدون اینترنت هم کار می‌کند. دکمهٔ انتهای کادر جستجو فیلتر اضافه می‌کند: فرستنده، گیرنده، موضوع، تاریخ‌ها و پیوست‌ها.
tour-menu-title = نمایش یا پنهان کردن پوشه‌ها
tour-menu-text = این دکمه فهرست پوشه‌ها را جمع می‌کند. وقتی پنهان است، نشانگر را روی «ایمیل» در نوار کناری نگه دارید تا پوشه‌ها را ببینید.
tour-apps-title = برنامه‌های شما
tour-apps-text = ایمیل اکنون این‌جاست. تقویم، مخاطبین، کارها، یادداشت‌ها و خوراک‌ها هم به این نوار می‌پیوندند.
tour-tabs-title = برگه‌های صندوق ورودی
tour-tabs-text = ایمیل‌های تازه در اصلی، تبلیغات، اجتماعی، به‌روزرسانی‌ها و انجمن‌ها دسته‌بندی می‌شوند. می‌توانید برگه‌ها را در تنظیمات سریع خاموش کنید.
tour-list-title = پیام‌های شما
tour-list-text = برای خواندن یک پیام روی آن کلیک کنید. برای کارهای سریع نشانگر را رویش نگه دارید، برای گزینه‌های بیشتر راست‌کلیک کنید، یا چند پیام را علامت بزنید تا با هم رویشان کار کنید.
tour-settings-title = تنظیمات سریع
tour-settings-text = قاب خواندن، تراکم و زمینه را این‌جا تغییر دهید. گشت را هم می‌توان از همان‌جا دوباره آغاز کرد.
tour-account-title = حساب شما
tour-account-text = ببینید در کدام حساب هستید، و حساب دیگری اضافه کنید.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] سرویس پس‌زمینهٔ Katna به‌طور غیرمنتظره متوقف شد.
    [one] سرویس پس‌زمینهٔ Katna به‌طور غیرمنتظره متوقف شد. { $more } گزارش خرابی دیگر هم ذخیره شده است.
   *[other] سرویس پس‌زمینهٔ Katna به‌طور غیرمنتظره متوقف شد. { $more } گزارش خرابی دیگر هم ذخیره شده است.
}
crash-mail = { $more ->
    [0] Katna Mail دفعهٔ پیش به‌طور غیرمنتظره بسته شد.
    [one] Katna Mail دفعهٔ پیش به‌طور غیرمنتظره بسته شد. { $more } گزارش خرابی دیگر هم ذخیره شده است.
   *[other] Katna Mail دفعهٔ پیش به‌طور غیرمنتظره بسته شد. { $more } گزارش خرابی دیگر هم ذخیره شده است.
}
crash-view = مشاهدهٔ گزارش
crash-view-tooltip = باز کردن گزارش ذخیره‌شده روی این رایانه
crash-copy = کپی گزارش
crash-close = بستن
sign-in-again-text = { $provider } از شما می‌خواهد دوباره به { $address } وارد شوید.
sign-in-again-button = ورود
sign-in-again-tooltip = باز کردن صفحهٔ ورود { $provider } در مرورگر
sign-in-again-waiting = در انتظار مرورگر شما…
sign-in-again-close = بستن
sign-in-again-done = دوباره به { $address } وارد شدید. در حال دریافت ایمیل‌های شما…
delete-ask-title = { $kind ->
    [conversation] { $count ->
        [one] این مکالمه به سطل زباله منتقل شود؟
       *[other] { $count } مکالمه به سطل زباله منتقل شود؟
    }
   *[message] { $count ->
        [one] این پیام به سطل زباله منتقل شود؟
       *[other] { $count } پیام به سطل زباله منتقل شود؟
    }
}
delete-ask-body = { $count ->
    [one] می‌توانید بی‌درنگ پس از آن واگرد کنید، یا بعداً آن را از سطل زباله برگردانید.
   *[other] می‌توانید بی‌درنگ پس از آن واگرد کنید، یا بعداً آن‌ها را از سطل زباله برگردانید.
}
delete-ask-confirm = انتقال به سطل زباله
delete-forever-title = { $kind ->
    [conversation] { $count ->
        [one] این مکالمه برای همیشه حذف شود؟
       *[other] { $count } مکالمه برای همیشه حذف شود؟
    }
   *[message] { $count ->
        [one] این پیام برای همیشه حذف شود؟
       *[other] { $count } پیام برای همیشه حذف شود؟
    }
}
delete-forever-body = { $count ->
    [one] روی سرور هم حذف می‌شود. این کار برگشت‌پذیر نیست.
   *[other] روی سرور هم حذف می‌شوند. این کار برگشت‌پذیر نیست.
}
delete-forever-confirm = حذف برای همیشه
delete-ask-dont-ask = دیگر نپرس
delete-ask-cancel = لغو
