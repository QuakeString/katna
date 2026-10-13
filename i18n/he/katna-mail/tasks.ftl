# Katna Mail, Hebrew (עברית): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = משימה חדשה
tasks-all = כל המשימות
tasks-today = היום
tasks-upcoming = בקרוב
tasks-starred = משימות מסומנות בכוכב
tasks-completed-view = הושלמו
tasks-new-list = יצירת רשימה חדשה
tasks-labels-heading = תוויות
tasks-on-this-computer = במחשב הזה
tasks-my-tasks = המשימות שלי
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = יש להתחבר שוב כדי להציג משימות
tasks-account-signed-in = התחברת שוב אל { $address }. המשימות שלך נטענות…
tasks-account-sign-in-refused = { $provider } לא הכניס את Katna. יש לנסות שוב ולאשר גישה למשימות שלך.
tasks-account-refused = השרת לא קיבל את הסיסמה. Yahoo, iCloud, Zoho ואחרים דורשים סיסמה לאפליקציה.
tasks-account-change-password = שינוי סיסמה
tasks-account-change-password-tooltip = יש להקליד את הסיסמה החדשה; Katna בודקת אותה מול השרת
tasks-account-not-enabled = הגישה למשימות עבור Katna עדיין לא הופעלה.
tasks-account-failed = לא ניתן היה לקרוא את רשימות המשימות.
# $reason is the server's own words, in English.
tasks-account-error = לא ניתן היה לקרוא את רשימות המשימות: { $reason }
tasks-account-none = לא נמצאו רשימות משימות
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = לא נמצאו רשימות משימות: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } מציג משימות רק ל-Katna שמחובר עם { $provider }.
tasks-account-sign-in-with = התחברות עם { $provider }
tasks-account-looking = מתבצע חיפוש רשימות משימות…
tasks-account-try-again = ניסיון נוסף
tasks-account-try-again-tooltip = בדיקה חוזרת של המשימות בחשבון הזה עכשיו
tasks-account-fixing = מטפלים בזה…
tasks-list-name-placeholder = שם הרשימה

## Lists and tasks

tasks-loading = קורא את המשימות שלך…
tasks-no-lists = רשימות המשימות שלך יופיעו כאן.
tasks-search = חיפוש במשימות
tasks-search-none = אין משימות שתואמות לחיפוש.
tasks-add = הוספת משימה
tasks-title-placeholder = כותרת
tasks-add-step = הוספת תת-משימה
tasks-empty = עדיין אין משימות. אפשר להוסיף אחת למעלה.
tasks-starred-empty = כדי לראות משימה כאן, מסמנים אותה בכוכב.
tasks-label-empty = אין משימות פתוחות עם התווית הזו.
tasks-today-empty = אין משימות להיום.
tasks-completed-empty = משימות שתשלימו יופיעו כאן.
tasks-upcoming-add = הוספת משימה ל־{ $day }
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = מדואר
tasks-from-note-quiet = מהערה
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = באיחור
tasks-completed = { $count ->
    [one] הושלמה ({ $count })
    [two] הושלמו ({ $count })
   *[other] הושלמו ({ $count })
}
tasks-list-options = אפשרויות הרשימה
tasks-sort-by = מיון לפי
tasks-sort-my-order = הסדר שלי
tasks-sort-date = תאריך
tasks-sort-starred = סומנו בכוכב לאחרונה
tasks-sort-title = כותרת
tasks-rename-list = שינוי שם הרשימה
tasks-delete-list = מחיקת הרשימה
tasks-mark-done = סימון כהושלמה
tasks-mark-open = סימון כלא הושלמה
tasks-star = סימון בכוכב
tasks-unstar = הסרת הכוכב
tasks-edit-title = עריכת הכותרת
tasks-details = פרטים
tasks-delete = מחיקה
tasks-move-to = העברה אל { $list }
tasks-from-mail = אימייל
tasks-open-mail = פתיחת האימייל
tasks-from-note = הערה
tasks-open-note = פתיחת ההערה
tasks-note-gone = ההערה הזאת כבר לא כאן.
tasks-no-subject = (ללא נושא)
tasks-selected = { $count ->
    [one] אחת נבחרה
    [two] { $count } נבחרו
   *[other] { $count } נבחרו
}
tasks-select-clear = ניקוי הבחירה
tasks-select-move = העברה לרשימה
tasks-select-date = קביעת תאריך
tasks-next-week = בשבוע הבא

## The details dialog

tasks-notes-placeholder = הוספת פרטים
tasks-date = תאריך
tasks-no-date = ללא תאריך
tasks-time-placeholder = הוספת שעה
tasks-repeat = חזרה
tasks-repeat-never = לא חוזרת
tasks-repeat-daily = מדי יום
tasks-repeat-weekly = מדי שבוע
tasks-repeat-monthly = מדי חודש
tasks-repeat-yearly = מדי שנה
tasks-repeat-other = מותאמת אישית
tasks-remind = הזכירו לי
tasks-remind-off = ללא תזכורת
tasks-remind-on-time = בזמן המשימה
tasks-remind-morning = ביום המשימה, { $time }
tasks-remind-hour-before = שעה לפני
tasks-remind-day-before = יום לפני
tasks-label-add = הוספת תווית
tasks-label-task = תיוג המשימה
tasks-files-attach = צירוף קבצים
tasks-files-pick = צירוף
tasks-file-open = פתיחה
tasks-file-remove = הסרת הקובץ
tasks-file-here = רק במחשב הזה
tasks-cancel = ביטול
tasks-save = שמירה
tasks-not-a-time = “{ $text }” אינו שעה, לדוגמה { $example }.

## Due days

tasks-due-today = היום
tasks-due-tomorrow = מחר
tasks-due-yesterday = אתמול
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = המשימה הושלמה
tasks-toast-next = בוצע. המופע הבא ב-{ $date }
tasks-toast-deleted = המשימה נמחקה
tasks-files-added = { $count ->
    [one] הקובץ צורף
    [two] { $count } קבצים צורפו
   *[other] { $count } קבצים צורפו
}
tasks-file-removed = „{ $name }” הוסר
tasks-files-left-out = לא צורפו: { $names }. אפשר לצרף למשימה קבצים של עד { $limit }, ולא תיקיות.
tasks-file-missing = הקובץ הזה כבר לא כאן.
tasks-toast-added = { $count ->
    [one] נוספה למשימות
    [two] נוספו { $count } משימות
   *[other] נוספו { $count } משימות
}
tasks-mail-gone = האימייל הזה כבר לא כאן.
tasks-toast-list-deleted = הרשימה נמחקה
tasks-toast-moved = הועברה אל { $list }
# A task dragged to another place in its own list.
tasks-toast-placed = המשימה הועברה
tasks-toast-rescheduled = המשימה תוזמנה מחדש
tasks-toast-rescheduled-several = { $count ->
    [one] מועד המשימה שונה
    [two] מועד { $count } משימות שונה
   *[other] מועד { $count } משימות שונה
}
tasks-toast-done-several = { $count ->
    [one] המשימה הושלמה
    [two] { $count } משימות הושלמו
   *[other] { $count } משימות הושלמו
}
tasks-toast-open-several = { $count ->
    [one] המשימה סומנה כלא הושלמה
    [two] { $count } משימות סומנו כלא הושלמו
   *[other] { $count } משימות סומנו כלא הושלמו
}
tasks-toast-starred = { $count ->
    [one] המשימה סומנה בכוכב
    [two] { $count } משימות סומנו בכוכב
   *[other] { $count } משימות סומנו בכוכב
}
tasks-toast-unstarred = { $count ->
    [one] הכוכב הוסר
    [two] הכוכבים הוסרו מ־{ $count } משימות
   *[other] הכוכבים הוסרו מ־{ $count } משימות
}
tasks-toast-deleted-several = { $count ->
    [one] המשימה נמחקה
    [two] { $count } משימות נמחקו
   *[other] { $count } משימות נמחקו
}
