# Katna Mail, Persian (فارسی).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = برچسب‌ها
nav-folders = پوشه‌ها
nav-label-new = ایجاد برچسب جدید
nav-folder-new = ایجاد پوشهٔ جدید
nav-menu-check-mail = بررسی ایمیل جدید
nav-menu-check-inbox = بررسی این صندوق ورودی
nav-unified-leave-out = بیرون گذاشتن از صندوق ورودی یکپارچه
nav-unified-bring-back = بازگرداندن به صندوق ورودی یکپارچه
nav-menu-sign-in-again = ورود دوباره
nav-menu-new-mail = ایمیل جدید از این حساب
nav-menu-account-settings = تنظیمات حساب
nav-account-checked = همگام · بررسی‌شده { $ago }
nav-account-in-sync = همگام
nav-account-connecting = در حال اتصال…
nav-account-offline = آفلاین، در حال تلاش دوباره
nav-account-signed-out = ورود { $provider } منقضی شده
nav-account-password-refused = گذرواژه پذیرفته نشد
nav-account-storage = { $used } از { $total } استفاده‌شده
nav-menu-new-subfolder = پوشهٔ جدید درون آن
nav-menu-new-sublabel = برچسب جدید درون آن
nav-menu-rename = تغییر نام
nav-menu-delete = حذف
nav-menu-empty-trash = خالی کردن سطل زباله
nav-account-unnamed = حساب { $number }
nav-all-accounts = همهٔ حساب‌ها
nav-expand = نمایش پوشه‌ها
nav-collapse = پنهان کردن پوشه‌ها
storage-used = { $percent }٪ از { $total } استفاده شده
storage-used-detail = { $address }: { $used } از { $total } استفاده شده

## Special folders (the user's own folders keep their names)

folder-inbox = صندوق ورودی
folder-starred = ستاره‌دار
folder-snoozed = به تعویق افتاده
folder-unread = خوانده‌نشده
folder-important = مهم
folder-drafts = پیش‌نویس‌ها
folder-sent = ارسال‌شده
folder-archive = بایگانی
folder-spam = هرزنامه
folder-trash = سطل زباله
folder-all-mail = همهٔ ایمیل‌ها
folder-scheduled = زمان‌بندی‌شده
folder-waiting = در انتظار پاسخ
folder-waiting-short = در انتظار
folder-reminders = یادآورها
folder-outbox = صندوق خروجی
folder-activity = فعالیت

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
label-rename-title = تغییر نام برچسب
label-folder-rename-title = تغییر نام پوشه
label-rename = تغییر نام
label-renaming = در حال تغییر نام…
label-renamed = نام برچسب به «{ $name }» تغییر کرد.
label-folder-renamed = نام پوشه به «{ $name }» تغییر کرد.
folder-delete-title = «{ $name }» حذف شود؟
folder-delete-body = { $count ->
    [0] هیچ ایمیلی در آن نیست. پوشه از سرور حذف می‌شود، پس از ایمیل وب و تلفن شما هم حذف می‌شود.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] { $count } مکالمهٔ آن به سطل زباله می‌رود، پس هنوز می‌توانید برش گردانید.
           *[other] { $count } مکالمهٔ آن به سطل زباله می‌روند، پس هنوز می‌توانید برشان گردانید.
        }
       *[message] { $count ->
            [one] { $count } پیام آن به سطل زباله می‌رود، پس هنوز می‌توانید برش گردانید.
           *[other] { $count } پیام آن به سطل زباله می‌روند، پس هنوز می‌توانید برشان گردانید.
        }
    } پوشه از سرور حذف می‌شود، پس از ایمیل وب و تلفن شما هم حذف می‌شود.
}
folder-delete-forever-body = { $count ->
    [0] هیچ ایمیلی در آن نیست. پوشه از سرور حذف می‌شود، پس از ایمیل وب و تلفن شما هم حذف می‌شود.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] { $count } مکالمهٔ آن برای همیشه حذف می‌شود؛ این حساب سطل زباله ندارد.
           *[other] { $count } مکالمهٔ آن برای همیشه حذف می‌شوند؛ این حساب سطل زباله ندارد.
        }
       *[message] { $count ->
            [one] { $count } پیام آن برای همیشه حذف می‌شود؛ این حساب سطل زباله ندارد.
           *[other] { $count } پیام آن برای همیشه حذف می‌شوند؛ این حساب سطل زباله ندارد.
        }
    } پوشه از سرور حذف می‌شود، پس از ایمیل وب و تلفن شما هم حذف می‌شود.
}
folder-delete-label-body = برچسب حذف می‌شود. ایمیل‌هایش در «همهٔ ایمیل‌ها» و برچسب‌های دیگرش می‌مانند.
folder-delete-confirm = حذف پوشه
folder-delete-label-confirm = حذف برچسب
folder-deleted = پوشهٔ «{ $name }» حذف شد
label-deleted = برچسب «{ $name }» حذف شد
