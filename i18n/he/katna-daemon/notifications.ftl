# Katna Mail, Hebrew (עברית).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } הודעת דוא״ל חדשה
    [two] { $count } הודעות דוא״ל חדשות
   *[other] { $count } הודעות דוא״ל חדשות
}
notify-and-more = ועוד { $count }
notify-no-subject = (ללא נושא)
notify-unknown-sender = שולח לא ידוע
notify-snooze-back = חזרו מהשהיה
notify-no-reply = עדיין אין תשובה
notify-no-reply-to = אף אחד לא ענה על „{ $subject }”.
notify-tracking-opened = ההודעה { $subject } נפתחה אצל { $who }
notify-tracking-clicked = קישור בהודעה { $subject } נפתח אצל { $who }

notify-update-ready = אפשר לעדכן את Katna Mail
notify-update-ready-body = גרסה { $version } הורדה. עדכון מתקין אותה ומפעיל מחדש את Katna Mail.
notify-update = עדכון

## Its buttons

notify-open = פתיחה
notify-reply-all = תשובה לכולם
notify-mark-read = סימון כנקראו
notify-mark-all-read = סימון של הכול כנקרא
notify-archive = העברה לארכיון
