# Katna Mail, Persian (فارسی).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = پیام جدید
compose-restore = بازگرداندن
compose-minimize = کوچک کردن
compose-exit-full-screen = خروج از تمام‌صفحه
compose-open-window = باز کردن در پنجرهٔ جدید
compose-save-close = ذخیره و بستن
compose-back-to-mail = بازگشت به پنجرهٔ ایمیل
compose-pop-out-reply = باز کردن پاسخ در پنجرهٔ جدا
compose-edit-recipients = ویرایش گیرندگان
compose-summary-cc = رونوشت: { $names }
compose-summary-bcc = رونوشت پنهان: { $names }
compose-more-recipients = { $count } نفر دیگر
compose-show-trimmed = نمایش محتوای کوتاه‌شده
compose-hide-trimmed = پنهان کردن محتوای کوتاه‌شده
compose-remove-trimmed = حذف متن نقل‌شده
compose-trimmed-removed = متن نقل‌شده حذف شد

## Recipients and subject

compose-to = به
compose-cc = رونوشت
compose-bcc = رونوشت پنهان
compose-from = از
compose-from-choose = ارسال از حسابی دیگر
compose-recipients = گیرندگان
compose-subject = موضوع

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = اول پیام باز را ارسال یا دور بیندازید.
compose-bad-address = «{ $address }» نشانی ایمیل نیست.
compose-no-recipients = دست‌کم یک گیرنده اضافه کنید.
compose-attachments-too-large = حجم پیوست‌ها { $size } است؛ سرورهای ایمیل تا { $limit } می‌پذیرند.
compose-no-account = یک حساب برای ارسال ایمیل اضافه کنید.
compose-past-time = زمانی در آینده انتخاب کنید.
compose-scheduling = در حال زمان‌بندی…
compose-sending = در حال ارسال…
compose-scheduled = ارسال برای { $when } زمان‌بندی شد
compose-sent-archived = ارسال و بایگانی شد
compose-sent = پیام ارسال شد
compose-discarded = پیش‌نویس دور انداخته شد
compose-draft-saved = پیش‌نویس ذخیره شد
compose-draft-failed = پیش‌نویس ذخیره نشد: { $error }
compose-draft-not-opened = پیش‌نویس باز نشد.

## Attachments

compose-picker-insert = درج
compose-picker-attach = پیوست
compose-file-too-large = { $name } بیش از حد بزرگ است: هر پیام تا { $limit } جا دارد.
compose-attachment-size = ({ $size })
compose-remove-attachment = حذف پیوست
compose-attachments-total = { $count ->
    [one] { $count } پرونده، { $size }
   *[other] { $count } پرونده، { $size }
}
compose-drive-note = { $name } بیشتر از { $limit } است؛ بنابراین در Google Drive شما ذخیره می‌شود و پیام یک پیوند همراه دارد.
compose-drive-tip = در Google Drive شما؛ پیام یک پیوند همراه دارد
compose-drive-uploading = در حال بارگذاری { $percent }%
compose-drive-allow = اجازه به Drive
compose-drive-allow-tip = دوباره با Google وارد شوید تا Katna بتواند پرونده‌های بزرگ را در Drive شما بگذارد
compose-drive-retry = تلاش دوباره
compose-drive-sends-when-uploaded = پس از بارگذاری { $name } ارسال می‌شود
compose-drive-not-uploaded = { $name } هنوز در Google Drive نیست
compose-drive-share-failed = اشتراک‌گذاری پرونده‌ها در Google Drive ممکن نشد: { $error }
compose-drive-share-title = پرونده‌ها با همه به اشتراک گذاشته شود؟
compose-drive-share-text = { $count ->
    [one] Google Drive نمی‌تواند پرونده‌ها را با { $addresses } که حساب Google ندارد به اشتراک بگذارد. در عوض هر کسی که پیوند را داشته باشد می‌تواند آن‌ها را باز کند.
   *[other] Google Drive نمی‌تواند پرونده‌ها را با { $addresses } که حساب Google ندارند به اشتراک بگذارد. در عوض هر کسی که پیوند را داشته باشد می‌تواند آن‌ها را باز کند.
}
compose-drive-share-link = اشتراک‌گذاری با پیوند
compose-drive-send-without = ارسال بدون اشتراک‌گذاری
compose-drive-share-cancel = لغو
compose-drive-card-detail = { $size } · Google Drive
compose-drop-files = پرونده‌ها را اینجا رها کنید
compose-drop-here = اینجا رها کنید
compose-paste-keep-formatting = حفظ قالب‌بندی
compose-paste-table = جدول
compose-paste-picture = تصویر
compose-paste-plain-text = متن ساده
compose-paste-inline = درون متن
compose-paste-attachment = پیوست

## Encryption and signing (the toggles by the recipients)

compose-encrypt = رمزگذاری
compose-encrypted = رمزگذاری‌شده: فقط گیرندگان می‌توانند آن را بخوانند
compose-sign = امضا
compose-signed = امضاشده: گیرندگان می‌توانند بررسی کنند که از طرف شماست
compose-track = ردیابی باز شدن و کلیک‌ها
compose-tracked = ردیابی‌شده: می‌بینید هر گیرنده چه زمانی آن را باز می‌کند یا پیوندی را دنبال می‌کند
compose-track-clicks = ردیابی کلیک روی پیوندها (متن ساده نمی‌تواند باز شدن را نشان دهد)
compose-tracked-clicks = ردیابی‌شده: می‌بینید هر گیرنده چه زمانی پیوندی را دنبال می‌کند
compose-track-sign-in = برای ردیابی باز شدن و کلیک‌ها، به یک حساب Katna وارد شوید
compose-receipt = درخواست رسید خواندن
compose-receipt-on = رسید خواندن درخواست شد: ممکن است برنامهٔ گیرنده از او بخواهد رسیدی بفرستد
compose-delivery = درخواست رسید تحویل
compose-delivery-on = رسید تحویل درخواست شد: وقتی سرور هر گیرنده آن را بپذیرد، سرور ایمیل شما ایمیلی برایتان می‌فرستد
compose-delivery-unavailable = سرور ایمیل شما رسید تحویل نمی‌فرستد

## Spelling

spell-no-dictionary = هیچ واژه‌نامهٔ املایی برای { $language } نصب نیست (برای نمونه hunspell-en_us).
spell-dictionary-error = واژه‌نامهٔ املایی: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = «{ $words }»
grammar-add = افزودن «{ $words }»
grammar-remove = حذف «{ $words }»
grammar-ignore = نادیده گرفتن

## Send checks (asked before a message goes out)

send-check-attachment-title = می‌خواستید پرونده پیوست کنید؟
send-check-attachment-text = از یک پیوست نوشته‌اید، اما چیزی پیوست نشده است.
send-check-attach = پیوست کردن پرونده
send-check-subject-title = ارسال بدون موضوع؟
send-check-subject-text = این پیام موضوع ندارد.
send-check-add-subject = افزودن موضوع
send-check-send-anyway = در هر صورت ارسال شود
recipient-not-valid = نشانی ایمیل معتبری نیست
recipient-show-address = نمایش نشانی
recipient-remove = حذف
recipient-bad-title = نشانی را بررسی کنید
recipient-bad-text = «{ $address }» نشانی ایمیل معتبری نیست. پیش از ارسال آن را درست یا حذف کنید.
recipient-bad-fix = درست کردن
