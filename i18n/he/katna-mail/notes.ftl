# Katna Mail, Hebrew (עברית): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = הערות
notes-view-archive = ארכיון
notes-view-trash = אשפה
notes-edit-labels = עריכת תוויות
notes-search = חיפוש הערות
notes-loading = פותח את ההערות שלך…

## Board

notes-take-a-note = יצירת הערה…
notes-new-list = רשימה חדשה
notes-pinned = מוצמדות
notes-others = אחרות
notes-empty = הערות שתוסיפו יופיעו כאן
notes-archive-empty = הערות שהועברו לארכיון יופיעו כאן
notes-trash-empty = אין הערות באשפה
notes-none-found = לא נמצאו הערות תואמות
notes-label-empty = אין עדיין הערות עם התווית הזו
notes-trash-note = הערות באשפה נמחקות אחרי 7 ימים.
notes-empty-trash = ריקון האשפה
notes-ticked = { $count ->
    [one] + { $count } פריט מסומן
    [two] + { $count } פריטים מסומנים
   *[other] + { $count } פריטים מסומנים
}

## A note's buttons

notes-pin = הצמדת הערה
notes-unpin = ביטול הצמדת הערה
notes-archive = העברה לארכיון
notes-unarchive = הוצאה מהארכיון
notes-delete = מחיקת הערה
notes-restore = שחזור
notes-delete-forever = מחיקה סופית
notes-color = אפשרויות רקע
notes-checkboxes = הצגה או הסתרה של תיבות סימון
notes-labels = תוויות
notes-close = סגירה

## The open note

notes-title = כותרת
notes-edited = נערך לאחרונה: { $date }
notes-on-this-computer = במחשב הזה
notes-where = המקום שבו ההערה נשמרת

## Labels

notes-label-note = הוספת תווית להערה
notes-label-name = הזנת שם תווית
notes-label-create = יצירת "{ $name }"
notes-label-remove = הסרת תווית
notes-label-delete = מחיקת תווית
notes-labels-none = אין עדיין תוויות. אפשר להוסיף תווית מכפתור התווית של הערה.
notes-labels-done = סיום
notes-label-renamed = שם התווית שונה ל-"{ $name }"
notes-label-deleted = התווית "{ $name }" נמחקה

## A note about a mail

notes-mail = אימייל
notes-open-mail = פתיחת האימייל
notes-open-note = פתיחת ההערה

## Meeting notes

notes-meeting-take = רישום הערות לפגישה
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = משתתפים: { $names }
notes-meeting-notes = הערות
notes-meeting-actions = פריטי פעולה
notes-event = אירוע
notes-open-event = פתיחת האירוע

## Formatting

notes-format = עיצוב
notes-format-heading-1 = כותרת 1
notes-format-heading-2 = כותרת 2
notes-format-normal = טקסט רגיל
notes-format-bold = מודגש
notes-format-italic = נטוי
notes-format-underline = קו תחתון
notes-format-clear = ניקוי העיצוב

## Tasks

notes-make-task = הפיכה למשימה

## Colors (tooltips)

notes-color-none = ללא צבע
notes-color-coral = אלמוג
notes-color-peach = אפרסק
notes-color-sand = חול
notes-color-mint = מנטה
notes-color-sage = מרווה
notes-color-fog = ערפל
notes-color-storm = סערה
notes-color-dusk = דמדומים
notes-color-blossom = פריחה
notes-color-clay = חימר
notes-color-chalk = גיר

## Messages at the foot of the window

notes-archived = ההערה הועברה לארכיון
notes-unarchived = ההערה הוצאה מהארכיון
notes-trashed = ההערה הועברה לאשפה
notes-restored = ההערה שוחזרה
notes-empty-discarded = הערה ריקה נמחקה
notes-mail-gone = האימייל הזה כבר לא כאן
notes-deleted-forever = { $count ->
    [one] ההערה נמחקה לצמיתות
    [two] { $count } הערות נמחקו לצמיתות
   *[other] { $count } הערות נמחקו לצמיתות
}
