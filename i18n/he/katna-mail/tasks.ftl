# Katna Mail, Hebrew (עברית): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = יצירה
tasks-all = כל המשימות
tasks-starred = משימות מסומנות בכוכב
tasks-new-list = יצירת רשימה חדשה
tasks-on-this-computer = במחשב הזה
tasks-my-tasks = המשימות שלי
tasks-list-name-placeholder = שם הרשימה

## Lists and tasks

tasks-loading = קורא את המשימות שלך…
tasks-no-lists = רשימות המשימות שלך יופיעו כאן.
tasks-add = הוספת משימה
tasks-title-placeholder = כותרת
tasks-add-step = הוספת תת-משימה
tasks-empty = עדיין אין משימות. אפשר להוסיף אחת למעלה.
tasks-starred-empty = כדי לראות משימה כאן, מסמנים אותה בכוכב.
tasks-completed = { $count ->
    [one] הושלמה ({ $count })
    [two] הושלמו ({ $count })
   *[other] הושלמו ({ $count })
}
tasks-list-options = אפשרויות הרשימה
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
tasks-no-subject = (ללא נושא)

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
tasks-toast-deleted = המשימה נמחקה
tasks-toast-added = { $count ->
    [one] נוספה למשימות
    [two] נוספו { $count } משימות
   *[other] נוספו { $count } משימות
}
tasks-mail-gone = האימייל הזה כבר לא כאן.
tasks-toast-list-deleted = הרשימה נמחקה
tasks-toast-moved = הועברה אל { $list }
