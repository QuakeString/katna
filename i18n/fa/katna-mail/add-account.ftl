# Katna Mail, Persian (فارسی).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = افزودن حساب ایمیل
add-account-looking = در حال یافتن سرورهای ایمیل { $address }…
add-account-address-intro = نشانی ایمیل خود را وارد کنید. Katna سرورها را برایتان پیدا می‌کند.
add-account-servers-title = تنظیمات سرور
add-account-servers-intro = جایی که Katna ایمیل‌های { $address } را از آن می‌خواند و می‌فرستد.
add-account-password-title = گذرواژهٔ خود را وارد کنید
add-account-signing-in = در حال ورود…
add-account-browser-title = در مرورگر خود ادامه دهید
add-account-browser-intro = Katna صفحهٔ ورود { $provider } را در مرورگرتان باز کرد. آن‌جا وارد شوید و به Katna اجازه دهید ایمیل‌هایتان را بخواند و بفرستد، سپس به این‌جا برگردید.
add-account-browser-hint = صفحه‌ای باز نشد؟ پنجره‌های مرورگرتان را بررسی کنید، یا برگردید و دوباره امتحان کنید.

## Add a mail account: fields

add-account-field-address = نشانی ایمیل
add-account-incoming = ایمیل ورودی ({ $protocol })
add-account-outgoing = ایمیل خروجی ({ $protocol })
add-account-field-server = سرور
add-account-field-port = درگاه
add-account-security-none = هیچ
add-account-field-username = نام کاربری
add-account-field-password = گذرواژه
add-account-show-password = نمایش گذرواژه
add-account-app-password-hint = { $provider } این‌جا به گذرواژهٔ برنامه نیاز دارد، نه گذرواژه‌ای که در وب به کار می‌برید. یکی در تنظیمات امنیتی حساب { $provider } خود بسازید.
add-account-field-name = نام شما (اختیاری)
add-account-name-hint = به کسانی که برایشان می‌نویسید نشان داده می‌شود.
add-account-servers-pair = { $imap } و { $smtp }
add-account-servers-found = { $source ->
    [built-in] سرورها: { $servers }، یافته‌شده در فهرست سرویس‌دهنده‌های Katna.
    [provider] سرورها: { $servers }، یافته‌شده در تنظیمات سرویس‌دهندهٔ شما.
    [ispdb] سرورها: { $servers }، یافته‌شده در فهرست سرویس‌دهنده‌های Thunderbird.
    [dns] سرورها: { $servers }، یافته‌شده در رکوردهای DNS دامنهٔ شما.
   *[other] سرورها: { $servers }، حدس زده‌شده؛ اگر ورود ناموفق بود آن‌ها را بررسی کنید.
}
add-account-servers-entered = سرورها: { $servers }، همان‌طور که وارد شد.
add-account-or = یا
add-account-sign-in-with = ورود با { $provider }
add-account-sign-in-instead = به‌جای آن با { $provider } وارد شوید

## Add a mail account: buttons

add-account-servers-button = تنظیمات سرور
add-account-back = برگشت
add-account-add = افزودن حساب
add-account-next = بعدی
add-account-cancel = لغو

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] سرور ورودی را وارد کنید.
   *[outgoing] سرور خروجی را وارد کنید.
}
add-account-server-space = { $kind ->
    [incoming] نام سرور ورودی فاصله دارد.
   *[outgoing] نام سرور خروجی فاصله دارد.
}
add-account-port-invalid = { $kind ->
    [incoming] درگاه ورودی باید عددی از { $min } تا { $max } باشد.
   *[outgoing] درگاه خروجی باید عددی از { $min } تا { $max } باشد.
}
add-account-address-empty = یک نشانی ایمیل وارد کنید.
add-account-address-invalid = یک نشانی ایمیل مانند { $example } وارد کنید.
add-account-not-found = Katna نتوانست سرورهای { $address } را پیدا کند، پس نام‌های معمول را وارد کرد. آن‌ها را با سرویس‌دهندهٔ خود بررسی کنید.
add-account-password-empty = گذرواژه را وارد کنید.
add-account-name-is-password = نام با گذرواژه یکی است. به‌جای آن، نام خود را همان‌طور که دیگران باید ببینند آنجا بنویسید.
add-account-added = { $address } افزوده شد. در حال دریافت ایمیل‌های شما…
add-account-app-password-refused = { $provider } گذرواژه را نپذیرفت. به گذرواژهٔ برنامه نیاز دارد، نه گذرواژه‌ای که در وب به کار می‌برید.
add-account-password-refused = سرور گذرواژه را نپذیرفت. آن را بررسی کنید و دوباره امتحان کنید.
add-account-sign-in-refused = { $provider } به Katna اجازهٔ ورود نداد. دوباره امتحان کنید و اجازهٔ دسترسی به ایمیل‌هایتان را بدهید.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] این نسخه از Katna هنوز نمی‌تواند به حساب‌های Microsoft وارد شود.
    [Google] این نسخه از Katna هنوز نمی‌تواند به حساب‌های Google وارد شود.
   *[other] این سرویس‌دهنده فقط در صفحهٔ خودش اجازهٔ ورود می‌دهد، و Katna هنوز نمی‌تواند این کار را برای آن انجام دهد.
}
add-account-signed-in = با { $provider } وارد شدید. در حال دریافت ایمیل‌های شما…

## The account menu (from the account button on the top bar)

add-account-menu-another = افزودن حساب دیگر
add-account-menu-manage = مدیریت حساب‌ها
app-menu = منوی اصلی
