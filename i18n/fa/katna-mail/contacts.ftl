# Katna Mail, Persian (فارسی): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = مخاطبین
contacts-frequent = پرتماس‌ها
contacts-other = سایر مخاطبین
contacts-other-about = افرادی که از Gmail برایشان ایمیل فرستاده‌اید اما ذخیره نکرده‌اید
contacts-other-email = ارسال ایمیل
contacts-other-empty = سایر مخاطبینی وجود ندارد. افرادی که از Gmail برایشان ایمیل می‌فرستید اما ذخیره نمی‌کنید اینجا نمایش داده می‌شوند.
contacts-other-allow = برای دیدن سایر مخاطبین، دوباره وارد حساب Gmail خود شوید و به Katna اجازه دهید آن‌ها را ببیند.
contacts-labels = برچسب‌ها
contacts-label-options = گزینه‌های برچسب
contacts-label-rename = تغییر نام برچسب
contacts-label-email = ایمیل به همه
contacts-label-delete = حذف برچسب
contacts-label-new = برچسب جدید
contacts-label-name = نام برچسب
contacts-label-button = برچسب
contacts-label-menu = برچسب‌گذاری به‌عنوان:
contacts-label-added = به { $name } اضافه شد
contacts-label-removed = از { $name } حذف شد
contacts-label-renamed = نام برچسب به { $name } تغییر کرد
contacts-label-deleted = برچسب { $name } حذف شد
contacts-label-no-email = هیچ‌کس در این برچسب نشانی ایمیل ندارد
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = حساب‌ها
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = برای نمایش مخاطبین دوباره وارد شوید
contacts-account-signed-in = دوباره به { $address } وارد شدید. در حال دریافت مخاطبین‌تان…
contacts-account-sign-in-refused = { $provider } به Katna اجازهٔ ورود نداد. دوباره امتحان کنید و اجازهٔ دسترسی به مخاطبین‌تان را بدهید.
contacts-account-password = سرور گذرواژه را نپذیرفت. Yahoo، iCloud، Zoho و دیگران به گذرواژهٔ برنامه نیاز دارند.
contacts-account-change-password = تغییر گذرواژه
contacts-account-change-password-tooltip = باز کردن تنظیمات > حساب‌ها
contacts-account-failed = خواندن مخاطبین ممکن نشد.
# $reason is the server's own words, in English.
contacts-account-error = خواندن مخاطبین ممکن نشد: { $reason }
contacts-account-none = هیچ دفترچهٔ نشانی‌ای پیدا نشد
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = هیچ دفترچهٔ نشانی‌ای پیدا نشد: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider } مخاطبین را فقط به Katna‌ای نشان می‌دهد که با { $provider } وارد شده باشد.
contacts-account-sign-in-with = ورود با { $provider }
contacts-account-looking = در حال جست‌وجوی مخاطبین…
contacts-account-try-again = امتحان مجدد
contacts-account-try-again-tooltip = همین حالا مخاطبین این حساب را دوباره بررسی کنید
contacts-account-fixing = در حال انجام…
contacts-manage = اصلاح و مدیریت
contacts-merge = ادغام و اصلاح
contacts-merge-about = { $count ->
    [one] { $count } پیشنهاد: مخاطبانی که به نظر یک نفر هستند
   *[other] { $count } پیشنهاد: مخاطبانی که به نظر یک نفر هستند
}
contacts-merge-none = مورد تکراری وجود ندارد. مخاطبانی که نام یا شماره تلفن یکسان دارند اینجا نشان داده می‌شوند.
contacts-merge-count = { $count ->
   *[other] { $count } مخاطب
}
contacts-merge-all = ادغام همه
contacts-merge-button = ادغام
contacts-merge-dismiss = رد کردن
contacts-merged = { $count ->
    [1] مخاطبین ادغام شدند
   *[other] { $count } ادغام انجام شد
}
contacts-import = وارد کردن
contacts-export = صادر کردن
contacts-import-file = وارد کردن مخاطبین از فایل vCard یا CSV
contacts-imported = { $count ->
   *[other] { $count } مخاطب در { $place } وارد شد
}
contacts-imported-some = { $count ->
   *[other] { $count } مخاطب در { $place } وارد شد؛ { $skipped } مخاطبِ ازقبل‌ذخیره‌شده کنار گذاشته شد
}
contacts-import-none = مخاطبی در { $name } پیدا نشد
contacts-import-all-saved = همه افراد { $name } از قبل ذخیره شده‌اند
contacts-import-failed = خواندن { $name } ممکن نشد: { $error }
contacts-exported = { $count ->
   *[other] { $count } مخاطب به { $path } صادر شد
}
contacts-export-none = مخاطبی برای صادر کردن وجود ندارد
contacts-export-failed = صادر کردن مخاطبین ممکن نشد: { $error }
contacts-print = چاپ
contacts-print-title = مخاطبین
contacts-print-none = مخاطبی برای چاپ وجود ندارد
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = تولد: { $day }
contacts-print-nickname = نام مستعار: { $name }
contacts-create = ایجاد مخاطب

## Search and the list

contacts-search = جستجو در مخاطبین
contacts-loading = در حال بارگذاری مخاطبین…
contacts-empty = هنوز مخاطب ذخیره‌شده‌ای وجود ندارد. مخاطبینی که در Gmail، Outlook یا سرویس ایمیل خود ذخیره می‌کنید اینجا نمایش داده می‌شوند.
contacts-empty-no-books = مخاطبین حساب‌های شما پس از همگام‌سازی اینجا نمایش داده می‌شوند.
contacts-none-found = مخاطبی مطابق جستجوی شما پیدا نشد.
contacts-starred = { $count ->
    [one] مخاطب ستاره‌دار ({ $count })
   *[other] مخاطبین ستاره‌دار ({ $count })
}
contacts-count = مخاطبین ({ $count })
contacts-col-name = نام
contacts-col-email = ایمیل
contacts-col-phone = شماره تلفن
contacts-col-job = عنوان شغلی و شرکت
contacts-col-labels = برچسب‌ها

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = به Katna اجازه دهید مخاطبین { $address } را بخواند.
contacts-allow-many = { $more ->
    [one] به Katna اجازه دهید مخاطبین { $address } و { $more } حساب دیگر را بخواند.
   *[other] به Katna اجازه دهید مخاطبین { $address } و { $more } حساب دیگر را بخواند.
}
contacts-allow-button = اجازه دادن

## A contact's page

contacts-back = بازگشت به مخاطبین
contacts-edit = ویرایش
contacts-delete = حذف
contacts-qr = هم‌رسانی به‌صورت کد QR
contacts-qr-about = با دوربین تلفن همراه اسکن کنید تا مخاطب ذخیره شود.
contacts-qr-too-long = این مخاطب جزئیات بیش‌ازحدی دارد و در کد QR جا نمی‌شود.
contacts-qr-done = تمام
contacts-deleted = { $name } حذف شد
contacts-added = { $name } به مخاطبین اضافه شد
contacts-find-mail = ایمیل
contacts-details = جزئیات مخاطب
contacts-saved-in = ذخیره‌شده در
contacts-notes = یادداشت‌ها
contacts-birthday = تاریخ تولد
contacts-nickname = نام مستعار
contacts-this-computer = این رایانه
contacts-kind-home = خانه
contacts-kind-work = محل کار
contacts-kind-mobile = تلفن همراه
contacts-kind-other = سایر
contacts-source-google = مخاطبین Google
contacts-source-microsoft = مخاطبین Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = ایجاد مخاطب
contacts-edit-title = ویرایش مخاطب
contacts-edit-save = ذخیره
contacts-edit-saving = در حال ذخیره…
contacts-edit-cancel = لغو
contacts-saved = مخاطب ذخیره شد
contacts-edit-save-to = ذخیره در
contacts-edit-changes-go-to = تغییرات در { $place } ذخیره می‌شوند.
contacts-edit-given = نام
contacts-edit-family = نام خانوادگی
contacts-edit-company = شرکت
contacts-edit-job = عنوان شغلی
contacts-edit-email = ایمیل
contacts-edit-phone = تلفن
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = افزودن ایمیل
contacts-edit-add-phone = افزودن تلفن
contacts-edit-street = آدرس خیابان
contacts-edit-city = شهر
contacts-edit-postcode = کد پستی
contacts-edit-country = کشور
contacts-edit-birthday = تاریخ تولد (YYYY-MM-DD)
contacts-edit-empty = ابتدا یک نام، ایمیل یا شماره تلفن اضافه کنید.
