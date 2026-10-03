# Katna Mail, Persian (فارسی).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = افزودن حساب ایمیل
add-account-providers-intro = ارائه‌دهندهٔ ایمیل خود را انتخاب کنید. Katna بقیه را پیدا می‌کند.
add-account-provider-other = ایمیل دیگر
add-account-provider-other-detail = هر حساب IMAP یا POP3
add-account-provider-google-detail = Gmail و Google Workspace
add-account-provider-microsoft-detail = Outlook و Microsoft 365
add-account-provider-mail = ایمیل { $provider }
add-account-form-title = ورود به { $provider }
add-account-form-title-other = حساب ایمیل شما
add-account-form-intro = Katna گذرواژهٔ شما را در دسته‌کلید سیستم نگه می‌دارد.
add-account-looking = در حال یافتن سرورهای ایمیل { $address }…
add-account-address-intro = نشانی ایمیل خود را وارد کنید. Katna سرورها را برایتان پیدا می‌کند.
add-account-servers-title = تنظیمات سرور
add-account-servers-intro = جایی که Katna ایمیل‌های { $address } را از آن می‌خواند و می‌فرستد.
add-account-signing-in = در حال ورود…
add-account-browser-title = در مرورگر خود ادامه دهید
add-account-browser-intro = Katna صفحهٔ ورود { $provider } را در مرورگرتان باز کرد. آن‌جا وارد شوید و به Katna اجازه دهید ایمیل‌هایتان را بخواند و بفرستد، سپس به این‌جا برگردید.
add-account-browser-hint = صفحه‌ای باز نشد؟ پنجره‌های مرورگرتان را بررسی کنید، یا برگردید و دوباره امتحان کنید.
add-account-stage-browser = در انتظار ورود شما در مرورگر…
add-account-stage-signing-in-at = در حال ورود به { $server }…
add-account-help-app-password-link = چگونه گذرواژهٔ برنامه بسازیم
add-account-help-turn-on-imap = { $provider } فقط وقتی به برنامه‌های ایمیل اجازهٔ ورود می‌دهد که دسترسی IMAP و POP3 در تنظیمات ایمیل وب آن روشن شده باشد.
add-account-help-turn-on-imap-link = چگونه روشنش کنیم

## Add a mail account: fields

add-account-field-address = نشانی ایمیل
add-account-receive-with = دریافت ایمیل با
add-account-imap-about = IMAP ایمیل‌ها و پوشه‌هایتان را روی سرور نگه می‌دارد، در همهٔ دستگاه‌ها یکسان. هر وقت ممکن است آن را انتخاب کنید.
add-account-pop3-about = POP3 ایمیل‌هایتان را روی این رایانه بارگیری می‌کند. ایمیلی که این‌جا می‌خوانید یا جابه‌جا می‌کنید روی سرور و دستگاه‌های دیگرتان همان‌طور می‌ماند.
add-account-incoming = ایمیل ورودی ({ $protocol })
add-account-outgoing = ایمیل خروجی ({ $protocol })
add-account-field-server = سرور
add-account-field-port = درگاه
add-account-security-none = هیچ
add-account-security-none-warning = رمزگذاری‌نشده: گذرواژه و ایمیل شما در مسیر قابل خواندن است.
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
add-account-sign-in-with = ورود با { $provider }
add-account-sign-in-instead = به‌جای آن با { $provider } وارد شوید

## Add a mail account: buttons

add-account-servers-button = تنظیمات سرور
add-account-back = برگشت
add-account-add = افزودن حساب
add-account-done = تمام
add-account-another = افزودن حساب دیگر
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
add-account-app-password-refused = { $provider } گذرواژه را نپذیرفت. به گذرواژهٔ برنامه نیاز دارد، نه گذرواژه‌ای که در وب به کار می‌برید.
add-account-password-refused = سرور گذرواژه را نپذیرفت. آن را بررسی کنید و دوباره امتحان کنید.
add-account-sign-in-refused = { $provider } به Katna اجازهٔ ورود نداد. دوباره امتحان کنید و اجازهٔ دسترسی به ایمیل‌هایتان را بدهید.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] این نسخه از Katna هنوز نمی‌تواند به حساب‌های Microsoft وارد شود.
    [Google] این نسخه از Katna هنوز نمی‌تواند به حساب‌های Google وارد شود.
   *[other] این سرویس‌دهنده فقط در صفحهٔ خودش اجازهٔ ورود می‌دهد، و Katna هنوز نمی‌تواند این کار را برای آن انجام دهد.
}
add-account-smtp-not-found = Katna پیدا کرد ایمیل‌تان را از کجا بخواند، اما نه این‌که از کجا بفرستد. سرور خروجی را وارد کنید.
add-account-done-title = حساب شما آماده است
add-account-done-intro = Katna اکنون در حال دریافت ایمیل‌های شماست. ایمیل‌های جدید با رسیدن نشان داده می‌شوند.
add-account-done-sign-in = ورود
add-account-done-signed-in-with = با { $provider }، در مرورگر شما
add-account-done-receiving = دریافت ایمیل
add-account-done-sending = ارسال ایمیل
add-account-done-on-server = ایمیل روی سرور
add-account-done-kept = تا وقتی در Katna حذفش کنید نگه داشته می‌شود
add-account-done-pop3-hint = آنچه با ایمیل روی سرور می‌شود را در تنظیمات > حساب‌ها تغییر دهید.
add-account-done-zoho-title = کارها و تقویم‌ها
add-account-done-zoho-about = Zoho این‌ها را جدا از ایمیل نگه می‌دارد. یک بار با Zoho وارد شوید تا به Katna آورده شوند.
add-account-done-linked = کارها و تقویم‌ها متصل شدند

## The account menu (from the account button on the top bar)

add-account-menu-another = افزودن حساب دیگر
app-menu = منوی اصلی
app-menu-back = برگشت
