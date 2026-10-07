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
notify-follow-up-sent = הודעת המעקב נשלחה
notify-follow-up-sent-to = אף אחד לא ענה על „{ $subject }”, אז Katna שלחה הודעת מעקב.
notify-follow-up-waiting = הודעת המעקב לא נשלחה
notify-follow-up-waiting-to = מועד השליחה הגיע בזמן שהמחשב היה כבוי. „{ $subject }” חזרה לדואר הנכנס.
notify-tracking-opened = ההודעה { $subject } נפתחה אצל { $who }
notify-tracking-clicked = קישור בהודעה { $subject } נפתח אצל { $who }

notify-update-ready = אפשר לעדכן את Katna Mail
notify-update-ready-body = גרסה { $version } הורדה. עדכון מתקין אותה ומפעיל מחדש את Katna Mail.
notify-update = עדכון
notify-signed-out = יש להתחבר שוב
notify-signed-out-body = { $provider } ניתק את Katna מ־{ $address }. סנכרון הדואר נעצר.
notify-sign-in = התחברות
notify-password-refused = הסיסמה נדחתה
notify-password-refused-body = שרת הדואר דחה את הסיסמה של { $address }. ייתכן שהיא השתנתה.
notify-new-password = סיסמה חדשה
notify-not-sent = „{ $subject }” לא נשלחה
notify-not-sent-no-subject = הודעה לא נשלחה
notify-not-sent-body = היא בדואר היוצא, ושם כתוב למה.
notify-open-outbox = פתיחת הדואר היוצא
notify-event-now = עכשיו
notify-event-in-minutes = { $count ->
    [one] בעוד { $count } דקה
    [two] בעוד { $count } דקות
   *[other] בעוד { $count } דקות
}
notify-event-in-hours = { $count ->
    [one] בעוד { $count } שעה
    [two] בעוד { $count } שעות
   *[other] בעוד { $count } שעות
}
notify-event-in-days = { $count ->
    [1] מחר
    [one] בעוד { $count } יום
    [two] בעוד { $count } ימים
   *[other] בעוד { $count } ימים
}
notify-event-all-day = כל היום
notify-event-join = הצטרפות
notify-event-snooze = השהיה ל-5 דקות
notify-task-done = סימון כבוצע

## Its buttons

notify-open = פתיחה
notify-peek = הצצה
notify-reply = תשובה
notify-reply-placeholder = תשובה ל־{ $name }…
notify-send = שליחה
notify-reply-all = תשובה לכולם
notify-mark-read = סימון כנקראו
notify-mark-all-read = סימון של הכול כנקרא
notify-archive = העברה לארכיון
notify-snooze-hour = השהיה לשעה
notify-snooze-tomorrow = מחר
notify-copy-code = העתקת { $code }
notify-link-verify = אימות ב־{ $domain }
notify-link-confirm = אישור ב־{ $domain }
notify-link-activate = הפעלה ב־{ $domain }
notify-archived = הועבר לארכיון
notify-archived-count = { $count ->
    [one] הודעה אחת הועברה מהדואר הנכנס
    [two] { $count } הודעות הועברו מהדואר הנכנס
   *[other] { $count } הודעות הועברו מהדואר הנכנס
}
notify-undo = ביטול
notify-code-copied = הקוד הועתק
notify-code-not-copied = לא ניתן היה להעתיק את הקוד
notify-reply-sent = התשובה נשלחה אל { $name }
notify-open-in-katna = פתיחה ב־Katna
