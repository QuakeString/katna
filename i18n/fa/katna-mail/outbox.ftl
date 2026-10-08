# Katna Mail, Persian (فارسی).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = ارسال نشد، چون { $reason }.
outbox-retrying = هنوز ارسال نشده، چون { $reason }. Katna خودش دوباره تلاش می‌کند.
outbox-waiting-sign-in = در انتظار ورود دوبارهٔ شما به { $address }. پس از آن ارسال می‌شود.
outbox-waiting-password = در انتظار گذرواژهٔ جدید { $address }. پس از آن ارسال می‌شود.
outbox-waiting-connection = در انتظار اتصال. وقتی دوباره آنلاین شوید ارسال می‌شود.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = گیرنده‌ای ندارد
outbox-reason-address = نشانی‌ای که به آن ارسال می‌شود وجود ندارد
outbox-reason-too-large = برای سرور ایمیل بیش از حد بزرگ است
outbox-reason-blocked = سرور ایمیل آن را مسدود کرد
outbox-reason-gone = نسخهٔ آن روی این رایانه دیگر وجود ندارد
outbox-reason-refused = سرور ایمیل آن را نپذیرفت

## Buttons and notes

outbox-try-again = تلاش دوباره
outbox-edit = ویرایش
outbox-delete = حذف
outbox-deleted = از صندوق خروجی حذف شد
outbox-sending-again = در حال ارسال دوباره…
outbox-snackbar-not-sent = «{ $subject }» ارسال نشد، چون { $reason }.
outbox-open = صندوق خروجی
