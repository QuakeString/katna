# Katna Mail, Hebrew (עברית): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = היום
calendar-today-tip = מעבר להיום
calendar-view-day = יום
calendar-view-week = שבוע
calendar-view-month = חודש
calendar-view-schedule = לוח זמנים
calendar-previous-day = היום הקודם
calendar-next-day = היום הבא
calendar-previous-week = השבוע הקודם
calendar-next-week = השבוע הבא
calendar-previous-month = החודש הקודם
calendar-next-month = החודש הבא
calendar-previous-period = מוקדם יותר
calendar-next-period = מאוחר יותר
calendar-title-months = { $first } – { $last }
calendar-loading = טוען…
calendar-read-failed = לא ניתן היה לקרוא את היומן: { $error }
calendar-local = המחשב הזה
calendar-account-gone = חשבון שהוסר
calendar-empty-title = עדיין אין יומנים
calendar-empty-text = היומנים של חשבונות Google ו-Microsoft שלך יוצגו כאן לאחר הסנכרון, וגם יומנים משרתים אחרים שתומכים ב-CalDAV.
calendar-schedule-empty = אין שום דבר מתוכנן בחודשיים הקרובים.
calendar-no-title = (ללא כותרת)
calendar-all-day = כל היום
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } נוספים
calendar-repeats = חוזר
calendar-join = הצטרפות
calendar-guests =
    { $count ->
        [one] { $count } אורח
        [two] שני אורחים
       *[other] { $count } אורחים
    }
calendar-guest-answers = { $yes } כן, { $maybe } אולי, { $no } לא, { $waiting } ממתינים
calendar-organizer = מארגן
calendar-optional = אופציונלי
calendar-open-web = פתיחה בדפדפן
calendar-close = סגירה

## Adding, changing and deleting events.

calendar-add-title = הוספת כותרת
calendar-add-location = הוספת מיקום
calendar-add-notes = הוספת תיאור
calendar-add-guests = הוספת אורחים
calendar-remove-guest = הסרה
calendar-add-meet = הוספת שיחת וידאו ב-Google Meet
calendar-add-teams = הוספת פגישה ב-Teams
calendar-has-call = נוספה שיחת וידאו
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = כל היום
calendar-more-options = אפשרויות נוספות
calendar-save = שמירה
calendar-saved = האירוע נשמר
calendar-deleted = האירוע נמחק
calendar-discard = מחיקת השינויים
calendar-edit = עריכת אירוע
calendar-delete = מחיקת אירוע
calendar-event-details = פרטי האירוע
calendar-kind-event = אירוע
calendar-kind-focus = זמן ריכוז
calendar-kind-out-of-office = מחוץ למשרד
calendar-kind-working-location = מיקום עבודה
calendar-working-home = בית
calendar-busy = עסוק
calendar-free = פנוי
calendar-cancel = ביטול
calendar-ok = אישור
calendar-read-only = אי אפשר לשנות אירועים ביומן הזה
calendar-none-editable = עדיין אין יומן שאפשר להוסיף לו אירועים
calendar-no-such-time = השעה הזו לא קיימת באזור הזמן שלך
calendar-end-before-start = האירוע מסתיים לפני שהוא מתחיל
calendar-repeat-never = לא חוזר
calendar-repeat-daily = מדי יום
calendar-repeat-weekly = שבועי: { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] חודשי: { $weekday } הראשון
        [2] חודשי: { $weekday } השני
        [3] חודשי: { $weekday } השלישי
        [4] חודשי: { $weekday } הרביעי
       *[other] חודשי: { $weekday } האחרון
    }
calendar-repeat-yearly = שנתי: { $day }
calendar-repeat-weekdays = בכל יום חול (שני עד שישי)
calendar-repeat-custom = מותאם אישית
calendar-reminder-none = ללא התראה
calendar-reminder-at-start = בזמן ההתחלה
calendar-reminder-minutes =
    { $count ->
        [one] דקה אחת לפני
        [two] שתי דקות לפני
       *[other] { $count } דקות לפני
    }
calendar-reminder-hours =
    { $count ->
        [one] שעה אחת לפני
        [two] שעתיים לפני
       *[other] { $count } שעות לפני
    }
calendar-reminder-days =
    { $count ->
        [one] יום אחד לפני
        [two] יומיים לפני
       *[other] { $count } ימים לפני
    }
calendar-scope-edit-title = עריכת אירוע חוזר
calendar-scope-delete-title = מחיקת אירוע חוזר
calendar-scope-this = האירוע הזה
calendar-scope-following = האירוע הזה והאירועים הבאים
calendar-scope-all = כל האירועים
calendar-scope-respond-title = תשובה לאירוע חוזר
calendar-going = מגיעים?
calendar-answer-yes = כן
calendar-answer-no = לא
calendar-answer-maybe = אולי
calendar-answered-yes = אישרת הגעה
calendar-answered-no = סירבת להזמנה
calendar-answered-maybe = ענית: אולי

## The card at the top of a mail with an invitation.

calendar-invite = הזמנה
calendar-invite-cancelled = האירוע בוטל
calendar-invite-reply = { $name }: תשובה
calendar-invite-reply-yes = { $name }: אישור הגעה
calendar-invite-reply-no = { $name }: סירוב
calendar-invite-reply-maybe = { $name }: אולי
calendar-invite-organizer = מארגן: { $name }
calendar-invite-open = פתיחה ביומן
calendar-invite-not-yet = עדיין לא ביומן שלך. אפשר לענות אחרי הסנכרון.
calendar-invite-your-day = היום שלך
calendar-invite-clashes =
    { $count ->
        [one] מתנגש עם { $count } אירוע
        [two] מתנגש עם { $count } אירועים
       *[other] מתנגש עם { $count } אירועים
    }

## The day's agenda beside the mail.

agenda-show = הצגת סדר היום
agenda-hide = הסתרת סדר היום
agenda-today = היום, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = אין שום דבר מתוכנן ליום הזה.
