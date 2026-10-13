# Katna Mail, Arabic (العربية).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = لم تُرسَل لأن { $reason }.
outbox-retrying = لم تُرسَل بعد لأن { $reason }. يعيد Katna المحاولة تلقائيًا.
outbox-waiting-sign-in = بانتظار تسجيل دخولك إلى { $address } مجددًا. ستُرسَل حينها.
outbox-waiting-password = بانتظار كلمة المرور الجديدة لـ { $address }. ستُرسَل حينها.
outbox-waiting-connection = بانتظار الاتصال. ستُرسَل عند عودتك للاتصال.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = ليس لها مستلمون
outbox-reason-address = أحد العناوين التي تُرسَل إليها غير موجود
outbox-reason-too-large = حجمها كبير جدًا على خادم البريد
outbox-reason-blocked = خادم البريد حظرها
outbox-reason-gone = نسختها على هذا الكمبيوتر لم تعد موجودة
outbox-reason-refused = خادم البريد رفضها

## Buttons and notes

outbox-try-again = إعادة المحاولة
outbox-edit = تعديل
outbox-delete = حذف
outbox-deleted = تم الحذف من صندوق الصادر
outbox-sending-again = جارٍ الإرسال مجددًا…
outbox-snackbar-not-sent = لم تُرسَل «{ $subject }» لأن { $reason }.
outbox-open = صندوق الصادر
