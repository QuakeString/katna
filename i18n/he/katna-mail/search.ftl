# Katna Mail, Hebrew (עברית).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = אפשרויות חיפוש
search-options-close = סגירה
search-from = מאת
search-to = אל
search-subject = נושא
search-has-words = מכיל את המילים
search-without = לא מכיל
search-date-within = תאריך בטווח של
search-has-attachment = עם קובץ מצורף
search-attachment-custom = מותאם אישית
search-attachment-image = תמונה
search-attachment-custom-hint = יש להקליד סיומת, כמו png, ואז רווח
search-attachment-remove = הסרה
search-clear-filter = ניקוי המסנן

## Search options: "Date within" choices

search-within-any = בכל זמן
search-within-days = { $count ->
    [one] יום אחד
    [two] יומיים
   *[other] { $count } ימים
}
search-within-weeks = { $count ->
    [one] שבוע אחד
    [two] שבועיים
   *[other] { $count } שבועות
}
search-within-months = { $count ->
    [one] חודש אחד
    [two] חודשיים
   *[other] { $count } חודשים
}
search-within-years = { $count ->
    [one] שנה אחת
    [two] שנתיים
   *[other] { $count } שנים
}
search-within-custom = מותאם אישית

## Search options: custom dates (the calendar popover)

search-dates-on = בתאריך
search-dates-before = לפני
search-dates-since = מאז
search-dates-between = בין
search-dates-from = מתאריך
search-dates-to = עד תאריך
search-dates-placeholder = שששש-חח-יי
search-dates-missing = בחירת תאריך
search-dates-unreadable = יש להשתמש בתאריך כמו 2026-09-01
search-dates-out-of-range = התאריך הזה מחוץ לטווח
search-dates-chip-before = לפני { $date }
search-dates-chip-since = מאז { $date }
search-dates-chip-between = { $first } – { $last }
search-dates-cancel = ביטול
search-dates-done = סיום
search-dates-month-back = החודש הקודם
search-dates-month-on = החודש הבא
search-dates-year-back = השנה הקודמת
search-dates-year-on = השנה הבאה
search-server-more = תוצאות נוספות בשרת
search-server-searching = מחפשים דואר בשרת…
search-server-empty-searching = אין כאן עדיין כלום. מחפשים דואר בשרת…
search-server-nothing = אין תוצאות נוספות בשרת
search-server-failed = לא ניתן היה לחפש בשרת.
search-server-again = ניסיון חוזר
