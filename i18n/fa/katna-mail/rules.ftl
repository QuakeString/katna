# Katna Mail, Persian (فارسی).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = قوانین
settings-rules-summary = مرتب کردن، برچسب زدن، بازارسال یا بی‌صدا کردن خودکار ایمیل جدید
settings-rules-intro = قوانین، ایمیل جدید را خودکار و به همین ترتیب مرتب می‌کنند. برای تغییر ترتیب بکشید.
settings-rules-all-accounts = همهٔ حساب‌ها
settings-rules-new = قانون جدید
settings-rules-none = هنوز قانونی نیست. قانون، ایمیل جدید را خودکار بر اساس فرستنده، موضوع یا کلمات مرتب می‌کند.
settings-rules-none-account = هنوز قانونی برای این حساب نیست.
settings-rules-drag = برای تغییر ترتیب بکشید
settings-rules-edit = ویرایش قانون
settings-rules-turn-off = خاموش کردن این قانون
settings-rules-turn-on = روشن کردن این قانون

## Starter rules: offered under the user's own rules, switched off.

## Turning one on makes it one of the user's rules.

settings-rules-starters = قوانین آماده
settings-rules-starters-intro = تا وقتی روشنشان نکنید خاموش‌اند. برای همهٔ حساب‌هایتان کار می‌کنند؛ برای تغییر، ویرایششان کنید.
settings-rules-starter-turning-on = در حال روشن کردن «{ $name }»…
settings-rules-starter-failed = روشن کردن «{ $name }» ممکن نشد: { $error }
rules-starter-promotions = بی‌صدا کردن تبلیغات
rules-starter-newsletters = خبرنامه‌ها به «خواندنی‌ها»
rules-starter-receipts = رسیدها و فاکتورها
rules-starter-deliveries = مرسوله‌ها
rules-starter-train = بلیت قطار
rules-starter-flight = بلیت هواپیما
rules-starter-codes = کدهای یک‌بارمصرف
rules-starter-security = هشدارهای امنیتی
rules-starter-social = ایمیل‌های اجتماعی
rules-starter-invites = دعوت‌نامه‌های تقویم
rules-starter-folder-reading = خواندنی‌ها
rules-starter-folder-receipts = رسیدها
rules-starter-folder-deliveries = مرسوله‌ها
rules-starter-folder-travel = سفر
rules-starter-folder-social = اجتماعی
rules-runs-katna = اجرا در Katna
rules-runs-gmail = اجرا در Gmail
rules-runs-sieve = اجرا روی سرور
rules-stopped = متوقف
rules-error-folder-gone = پوشه‌ای که این قانون استفاده می‌کند دیگر وجود ندارد. قانون را ویرایش کنید تا پوشهٔ دیگری انتخاب کنید.
rules-error-no-archive = این حساب پوشهٔ بایگانی ندارد. قانون را ویرایش کنید تا کار دیگری انجام دهد.
rules-error-no-trash = این حساب پوشهٔ سطل زباله ندارد. قانون را ویرایش کنید تا کار دیگری انجام دهد.
rules-error-cannot-send = این حساب نمی‌تواند ایمیل ارسال کند، پس قانون نمی‌تواند آن را بازارسال کند.
rules-error-other = { $error }. قانون را ویرایش کنید و دوباره روشنش کنید.
settings-folders = پوشه‌ها
settings-folders-summary = تعداد خوانده‌نشده‌ها در قاب پوشه‌ها
settings-folders-unread-counts = تعداد خوانده‌نشده‌ها روی همهٔ پوشه‌ها
settings-folders-unread-counts-detail = خاموش: فقط صندوق ورودی تعداد خوانده‌نشده‌ها را نشان می‌دهد

## A rule in one line, on its row: "From contains substack.com → skip the

## inbox, label Reading".

rules-summary = { $when } ← { $then }
rules-summary-and = { $first } و { $next }
rules-summary-or = { $first } یا { $next }
rules-summary-more = { $count } مورد دیگر
rules-summary-list = { $first }، { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = پیوست دارد
rules-summary-no-attachment = پیوست ندارد
rules-summary-mailing-list = از یک فهرست پستی
rules-summary-not-mailing-list = نه از فهرست پستی
rules-summary-tab = در برگهٔ { $tab }
rules-summary-not-tab = نه در برگهٔ { $tab }
rules-summary-move = انتقال به { $folder }
rules-summary-archive = رد شدن از صندوق ورودی
rules-summary-trash = انتقال به سطل زباله
rules-summary-mark-read = علامت‌گذاری به‌عنوان خوانده‌شده
rules-summary-star = افزودن ستاره
rules-summary-important = علامت‌گذاری به‌عنوان مهم
rules-summary-label = برچسب { $label }
rules-summary-forward = بازارسال به { $address }
rules-summary-dont-notify = بدون اعلان
rules-summary-read-after = { $count ->
    [one] علامت‌گذاری به‌عنوان خوانده‌شده پس از { $count } روز
   *[other] علامت‌گذاری به‌عنوان خوانده‌شده پس از { $count } روز
}
rules-summary-folder-gone = پوشه‌ای که دیگر وجود ندارد

## The rule editor

rules-editor-new-title = قانون جدید
rules-editor-edit-title = ویرایش قانون
rules-editor-name-hint = نام قانون
rules-editor-when = وقتی ایمیل جدید با
rules-editor-of-these = این شرط‌ها مطابقت دارد:
rules-mode-all = همهٔ
rules-mode-any = هر یک از
rules-field-from = از
rules-field-to = به
rules-field-cc = رونوشت
rules-field-any-recipient = به یا رونوشت
rules-field-reply-to = پاسخ به
rules-field-subject = موضوع
rules-field-body = متن
rules-field-attachment-name = نام پیوست
rules-field-has-attachment = پیوست دارد
rules-field-mailing-list = از یک فهرست پستی
rules-field-tab = برگهٔ صندوق ورودی
rules-comparator-contains = شامل
rules-comparator-not-contains = شامل نیست
rules-comparator-begins-with = شروع می‌شود با
rules-comparator-ends-with = تمام می‌شود با
rules-comparator-equals = دقیقاً برابر است با
rules-comparator-matches = مطابق الگو است
rules-has-yes = بله
rules-has-no = خیر
rules-editor-value-hint = کلمات یا یک نشانی
rules-editor-add-condition = افزودن شرط
rules-editor-remove = حذف
rules-editor-then = سپس:
rules-action-move = انتقال به
rules-action-archive = رد شدن از صندوق ورودی (بایگانی)
rules-action-trash = انتقال به سطل زباله
rules-action-mark-read = علامت‌گذاری به‌عنوان خوانده‌شده
rules-action-star = افزودن ستاره
rules-action-important = علامت‌گذاری به‌عنوان مهم
rules-action-label = افزودن برچسب
rules-action-forward = بازارسال به
rules-action-dont-notify = بدون اعلان
rules-action-read-after = علامت‌گذاری به‌عنوان خوانده‌شده پس از
rules-editor-choose-folder = انتخاب پوشه
rules-editor-choose-label = انتخاب برچسب
rules-editor-new-folder = جدید: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = نشانی ایمیل
rules-editor-days = روز
rules-editor-add-action = افزودن کنش
rules-editor-stop = توقف در اینجا: قوانین بعدی روی این ایمیل اجرا نمی‌شوند
rules-editor-accounts = حساب‌ها:
rules-editor-accounts-none = انتخاب حساب‌ها
rules-editor-accounts-many = { $count ->
    [one] { $count } حساب
   *[other] { $count } حساب
}
rules-editor-matches = با { $mails } از { $days } روز گذشته مطابقت دارد
rules-editor-mails = { $count ->
    [one] { $count } ایمیل
   *[other] { $count } ایمیل
}
rules-editor-counting = در حال شمردن ایمیل‌های مطابق…
rules-editor-show = نمایش آن‌ها
rules-editor-also-apply = روی این { $count } مورد هم اعمال شود
rules-editor-runs-katna = در Katna اجرا می‌شود، تا وقتی این رایانه روشن است.
rules-editor-runs-gmail = در Gmail اجرا می‌شود، پس روی تلفن شما و با خاموش بودن این رایانه هم کار می‌کند.
rules-editor-runs-sieve = روی سرور ایمیل شما اجرا می‌شود، پس روی تلفن شما و با خاموش بودن این رایانه هم کار می‌کند.
rules-note-gmail-action = در Katna اجرا می‌شود: فیلترهای Gmail نمی‌توانند «{ $action }» را انجام دهند.
rules-note-sieve-action = در Katna اجرا می‌شود: قوانین سرور ایمیل شما نمی‌توانند «{ $action }» را انجام دهند.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = در Katna اجرا می‌شود: فیلترهای Gmail نمی‌توانند «{ $test }» را مثل Katna بررسی کنند.
rules-note-sieve-condition = در Katna اجرا می‌شود: قوانین سرور ایمیل شما نمی‌توانند «{ $test }» را مثل Katna بررسی کنند.
rules-note-order = در Katna اجرا می‌شود، چون یک قانون قبلی این حساب هم همین‌طور است: قوانین به ترتیب فهرست اجرا می‌شوند.
rules-note-gmail-stop = در Katna اجرا می‌شود: فیلترهای Gmail نمی‌توانند جلوی اجرای قوانین بعدی را بگیرند.
rules-note-gmail-forward = در Katna اجرا می‌شود: Gmail فقط به نشانی‌هایی بازارسال می‌کند که در تنظیماتش تأیید شده‌اند، و { $address } یکی از آن‌ها نیست.
rules-note-gmail-folder = در Katna اجرا می‌شود: Gmail برای پوشه‌ای که این قانون استفاده می‌کند برچسبی ندارد.
rules-note-sieve-folder = در Katna اجرا می‌شود: سرور ایمیل شما پوشه‌ای را که این قانون استفاده می‌کند ندارد.
rules-note-gmail-sign-in = در Katna اجرا می‌شود تا وقتی دوباره به Google وارد شوید و به Katna اجازهٔ ساختن فیلترهای Gmail بدهید.
rules-note-sieve-other-script = در Katna اجرا می‌شود: اسکریپت قانون دیگری («{ $name }») روی سرور ایمیل شما فعال است.
rules-note-gmail-failed = در Katna اجرا می‌شود: Gmail آن را نپذیرفت ({ $error }).
rules-note-sieve-failed = در Katna اجرا می‌شود: سرور ایمیل شما آن را نپذیرفت ({ $error }).
rules-editor-cancel = لغو
rules-editor-save = ذخیره
rules-editor-saving = در حال ذخیره…
rules-editor-delete = حذف قانون
rules-editor-delete-ask = این قانون حذف شود؟
rules-editor-delete-keep = نگه داشتن
rules-editor-delete-confirm = حذف
rules-editor-needs-folder = برای هر «انتقال به» یک پوشه و برای هر «افزودن برچسب» یک برچسب انتخاب کنید.
rules-editor-needs-days = «علامت‌گذاری به‌عنوان خوانده‌شده پس از» به تعداد روز، از ۱ تا ۳۶۵۰، نیاز دارد.
rules-saved = قانون ذخیره شد
rules-saved-applied = { $count ->
    [one] قانون ذخیره شد و روی { $count } ایمیل اعمال شد
   *[other] قانون ذخیره شد و روی { $count } ایمیل اعمال شد
}
rules-apply-failed = قانون ذخیره شد، اما اعمال آن ناموفق بود: { $error }
rules-deleted = قانون حذف شد
rules-delete-failed = حذف قانون ممکن نشد: { $error }
rules-change-failed = تغییر قوانین ممکن نشد: { $error }
