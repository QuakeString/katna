# Katna Mail, Hebrew (עברית): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = הערות
notes-view-reminders = תזכורות
notes-view-archive = ארכיון
notes-view-trash = אשפה
notes-edit-labels = עריכת תוויות
notes-search = חיפוש הערות
notes-loading = פותח את ההערות שלך…

## Board

notes-take-a-note = יצירת הערה…
notes-new-list = רשימה חדשה
notes-new-note = הערה חדשה
notes-pinned = מוצמדות
notes-others = אחרות
notes-empty = הערות שתוסיפו יופיעו כאן
notes-archive-empty = הערות שהועברו לארכיון יופיעו כאן
notes-trash-empty = אין הערות באשפה
notes-none-found = לא נמצאו הערות תואמות
notes-label-empty = אין עדיין הערות עם התווית הזו
notes-reminders-empty = הערות עם תזכורות קרובות יופיעו כאן
notes-trash-note = הערות באשפה נמחקות אחרי 7 ימים.
notes-empty-trash = ריקון האשפה
notes-ticked = { $count ->
    [one] + { $count } פריט מסומן
    [two] + { $count } פריטים מסומנים
   *[other] + { $count } פריטים מסומנים
}
notes-select = בחירת הערה
notes-selected = { $count ->
    [one] אחת נבחרה
    [two] { $count } נבחרו
   *[other] { $count } נבחרו
}
notes-select-clear = ניקוי הבחירה

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
notes-more = עוד
notes-make-copy = יצירת עותק
notes-remind = להזכיר לי
notes-add-picture = הוספת תמונה
notes-history = היסטוריית גרסאות
notes-ai = עזרה בכתיבה
notes-send-as-mail = שליחה כדואר
notes-save-markdown = שמירה כ־Markdown
notes-save-pdf = שמירה כ־PDF

## The open note

notes-title = כותרת
notes-edited = נערך לאחרונה: { $date }
notes-on-this-computer = במחשב הזה
notes-where = המקום שבו ההערה נשמרת
notes-untitled = הערה ללא שם
notes-picture-choose = הוספת תמונות
notes-picture-remove = הסרת התמונה
notes-picture-too-big = אפשר להוסיף להערה תמונות בגודל של עד { $size }
notes-picture-kind = הקובץ הזה הוא לא תמונה ש־Katna יכולה להציג
notes-picture-unreadable = לא ניתן היה לקרוא את { $name }: { $error }
notes-remind-me = להזכיר לי
notes-remind-off = הסרת התזכורת
notes-remind-in-the-past = יש לבחור שעה שעוד לא עברה
notes-remind-today = היום, { $time }
notes-remind-tomorrow = מחר, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = נקבעה תזכורת ל־{ $when }
notes-reminder-off = התזכורת הוסרה
notes-link-note = קישור להערה
notes-link-new = הערה חדשה "{ $title }"
notes-linked-from = מקושרת מ־
notes-link-gone = ההערה הזו כבר לא כאן
notes-versions = גרסאות
notes-version-now = עכשיו
notes-version-here = את/ה, במחשב הזה
notes-version-yesterday = אתמול, { $time }
notes-version-changes = { $count ->
    [one] שינוי אחד
    [two] { $count } שינויים
   *[other] { $count } שינויים
}
notes-version-from = מ־{ $device }
notes-version-elsewhere = ממכשיר אחר
notes-version-created = נוצרה
notes-version-restore = שחזור הגרסה הזו
notes-version-restored = הגרסה שוחזרה
notes-history-none = עדיין אין גרסאות קודמות
notes-ai-tidy = סידור הטקסט
notes-ai-checklist = הפיכה לרשימת סימון
notes-ai-summarise = סיכום
notes-ai-empty = יש לכתוב משהו קודם
notes-ai-tidied = הטקסט סודר. Ctrl+Z מחזיר אותו.
notes-ai-listed = הפכה לרשימת סימון. Ctrl+Z מחזיר אותה.
notes-ai-summarised = הסיכום נוסף למעלה

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
notes-format-quote = ציטוט
notes-format-code = קוד
notes-format-divider = קו מפריד
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
notes-saved = ההערה נשמרה
notes-pinned-count = { $count ->
    [one] ההערה הוצמדה
    [two] { $count } הערות הוצמדו
   *[other] { $count } הערות הוצמדו
}
notes-unpinned-count = { $count ->
    [one] הצמדת ההערה בוטלה
    [two] הצמדת { $count } הערות בוטלה
   *[other] הצמדת { $count } הערות בוטלה
}
notes-colored-count = { $count ->
    [one] הצבע שונה
    [two] הצבע שונה ב־{ $count } הערות
   *[other] הצבע שונה ב־{ $count } הערות
}
notes-archived-count = { $count ->
    [one] ההערה הועברה לארכיון
    [two] { $count } הערות הועברו לארכיון
   *[other] { $count } הערות הועברו לארכיון
}
notes-unarchived-count = { $count ->
    [one] ההערה הוצאה מהארכיון
    [two] { $count } הערות הוצאו מהארכיון
   *[other] { $count } הערות הוצאו מהארכיון
}
notes-trashed-count = { $count ->
    [one] ההערה הועברה לאשפה
    [two] { $count } הערות הועברו לאשפה
   *[other] { $count } הערות הועברו לאשפה
}
notes-restored-count = { $count ->
    [one] ההערה שוחזרה
    [two] { $count } הערות שוחזרו
   *[other] { $count } הערות שוחזרו
}
notes-copied-count = { $count ->
    [one] נוצר עותק
    [two] נוצרו { $count } עותקים
   *[other] נוצרו { $count } עותקים
}
notes-empty-discarded = הערה ריקה נמחקה
notes-mail-gone = האימייל הזה כבר לא כאן
notes-deleted-forever = { $count ->
    [one] ההערה נמחקה לצמיתות
    [two] { $count } הערות נמחקו לצמיתות
   *[other] { $count } הערות נמחקו לצמיתות
}
