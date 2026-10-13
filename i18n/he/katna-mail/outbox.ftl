# Katna Mail, Hebrew (עברית).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = לא נשלחה כי { $reason }.
outbox-retrying = עוד לא נשלחה כי { $reason }. Katna תנסה שוב בעצמה.
outbox-waiting-sign-in = ממתינה שתתחברו שוב אל { $address }. אז היא תצא.
outbox-waiting-password = ממתינה לסיסמה החדשה של { $address }. אז היא תצא.
outbox-waiting-connection = ממתינה לחיבור. היא תצא כשתחזרו להיות מקוונים.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = אין לה נמענים
outbox-reason-address = כתובת שהיא נשלחת אליה לא קיימת
outbox-reason-too-large = היא גדולה מדי עבור שרת הדואר
outbox-reason-blocked = שרת הדואר חסם אותה
outbox-reason-gone = העותק שלה במחשב הזה נעלם
outbox-reason-refused = שרת הדואר דחה אותה

## Buttons and notes

outbox-try-again = ניסיון נוסף
outbox-edit = עריכה
outbox-delete = מחיקה
outbox-deleted = נמחקה מהדואר היוצא
outbox-sending-again = שולחים שוב…
outbox-snackbar-not-sent = „{ $subject }” לא נשלחה כי { $reason }.
outbox-open = דואר יוצא
