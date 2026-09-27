# Katna Mail, Persian (فارسی).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = قاب پوشه‌ها
accounts-folder-pane-detail = پوشه‌های کدام حساب‌ها در قاب سمت راست نشان داده شود.
accounts-shown-one = یک حساب در هر زمان؛ جابه‌جایی از کارت حساب
accounts-shown-all = همهٔ حساب‌ها، یکی پس از دیگری
accounts-unified = صندوق ورودی یکپارچه
accounts-unified-switch = نمایش ایمیل‌های همهٔ حساب‌ها با هم
accounts-unified-switch-detail = «همهٔ حساب‌ها» در بالای قاب پوشه‌ها است و صندوق ورودی، ایمیل‌های ارسال‌شده و موارد دیگرِ هر حساب را در یک فهرست نشان می‌دهد. حساب‌های زیر آن در ابتدا جمع‌شده هستند.
accounts-row = حساب‌ها
accounts-row-detail = قاب پوشه‌ها و منوی حساب، حساب‌ها را به همین ترتیب نشان می‌دهند؛ اولی پیش‌فرض است. حذف یک حساب، نسخهٔ Katna از ایمیل‌های آن را از این رایانه پاک می‌کند. ایمیل‌ها روی سرور می‌مانند.
accounts-none = هنوز حسابی نیست.
accounts-kind-imported = واردشده
accounts-picture-reset = استفاده از تصویر میزکار
accounts-picture-change = تغییر تصویر
accounts-picture-remove = حذف تصویر
accounts-rename = تغییر نام
accounts-name-save = ذخیره
accounts-name-cancel = لغو
accounts-name-placeholder = نام شما
accounts-rename-failed = تغییر نام حساب ممکن نشد: { $error }
accounts-move-up = انتقال به بالا
accounts-move-down = انتقال به پایین
accounts-drag = برای تغییر ترتیب بکشید
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
reset-cache-about = ایمیل‌ها و پیوست‌هایی که Katna بارگیری کرده، تصاویر فرستندگان و نمایهٔ جستجو را حذف می‌کند و سپس ایمیل‌های اخیر را دوباره بارگیری می‌کند. حساب‌ها، تنظیمات و ایمیل‌هایی که فقط روی این رایانه هستند می‌مانند.
reset-cache-button = بازنشانی حافظهٔ نهان
reset-cache-title = حافظهٔ نهان بازنشانی شود؟
reset-cache-deleted = حذف و سپس دوباره بارگیری می‌شود:
reset-cache-mail = ایمیل‌ها و پیوست‌های بارگیری‌شده از سرورهای IMAP شما: ایمیل‌های اخیر همین حالا دوباره بارگیری می‌شوند و ایمیل‌های قدیمی‌تر هنگامی که بازشان کنید
reset-cache-index = نمایهٔ جستجو، که بلافاصله از نو ساخته می‌شود
reset-cache-pictures = تصاویر فرستندگان
reset-cache-kept = می‌ماند: حساب‌ها، گذرواژه‌ها و تنظیمات شما؛ ستاره‌ها، برچسب‌ها، علامت‌های خوانده‌شده و سنجاق‌ها؛ پیش‌نویس‌ها، صندوق خروجی و تغییراتی که هنوز روی سرور نیستند؛ و ایمیل‌های حساب‌های POP3 یا فایل‌های واردشده، که شاید نسخهٔ دیگری نداشته باشند. چیزی روی سرورهای ایمیل شما تغییر نمی‌کند.
reset-cache-confirm = بازنشانی حافظهٔ نهان
reset-cache-busy = در حال بازنشانی…
reset-cache-done = حافظهٔ نهان بازنشانی شد. ایمیل‌های اخیر دوباره در حال بارگیری‌اند.
reset-cache-done-freed = حافظهٔ نهان بازنشانی شد و { $size } آزاد شد. ایمیل‌های اخیر دوباره در حال بارگیری‌اند.
