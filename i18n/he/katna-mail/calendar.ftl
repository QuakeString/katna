# Katna Mail, Hebrew (עברית): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = היום
calendar-today-tip = מעבר להיום
calendar-view-day = יום
calendar-view-week = שבוע
calendar-view-month = חודש
calendar-view-year = שנה
calendar-view-schedule = לוח זמנים
calendar-view-days =
    { $count ->
        [one] { $count } יום
        [two] יומיים
       *[other] { $count } ימים
    }
calendar-options = אפשרויות
calendar-density = צפיפות
calendar-density-responsive = מותאם למסך שלך
calendar-density-comfortable = נוחה
calendar-density-compact = קומפקטית
calendar-custom-days = תצוגה מותאמת אישית
calendar-second-zone = אזור זמן שני
calendar-zone-none = ללא
calendar-zone = { $zone } ({ $offset })
calendar-share-free = שיתוף זמנים פנויים
calendar-free-subject = הזמנים שבהם אני פנוי
calendar-free-intro = הנה כמה זמנים שבהם אני פנוי ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = אין לי זמן פנוי בימי העבודה הקרובים.
calendar-previous-day = היום הקודם
calendar-next-day = היום הבא
calendar-previous-week = השבוע הקודם
calendar-next-week = השבוע הבא
calendar-previous-month = החודש הקודם
calendar-next-month = החודש הבא
calendar-previous-year = השנה הקודמת
calendar-next-year = השנה הבאה
calendar-previous-period = מוקדם יותר
calendar-next-period = מאוחר יותר
calendar-title-months = { $first } – { $last }
calendar-loading = טוען…
calendar-read-failed = לא ניתן היה לקרוא את היומן: { $error }
calendar-sets = קבוצות יומנים
calendar-set-add = שמירת היומנים המוצגים כקבוצה
calendar-set-name = שם הקבוצה
calendar-set-remove = הסרת הקבוצה
calendar-local = המחשב הזה
calendar-account-gone = חשבון שהוסר
calendar-account-sign-in = יש להתחבר שוב כדי להציג יומנים
calendar-account-signed-in = התחברת שוב אל { $address }. היומנים שלך נטענים…
calendar-account-sign-in-refused = { $provider } לא הכניס את Katna. יש לנסות שוב ולאשר גישה ליומנים שלך.
calendar-account-refused = השרת לא קיבל את הסיסמה. Yahoo, iCloud, Zoho ואחרים דורשים סיסמה לאפליקציה.
calendar-account-change-password = שינוי סיסמה
calendar-account-change-password-tooltip = יש להקליד את הסיסמה החדשה; Katna בודקת אותה מול השרת
calendar-account-not-enabled = הגישה ליומן עבור Katna עדיין לא הופעלה.
calendar-account-failed = לא ניתן היה לקרוא את היומנים.
calendar-account-error = לא ניתן היה לקרוא את היומנים: { $reason }
calendar-account-none = לא נמצאו יומנים
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = לא נמצאו יומנים: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } מציג יומנים רק ל-Katna שמחובר עם { $provider }.
calendar-account-sign-in-with = התחברות עם { $provider }
calendar-account-looking = מתבצע חיפוש יומנים…
calendar-account-try-again = ניסיון נוסף
calendar-account-try-again-tooltip = בדיקה חוזרת של היומנים בחשבון הזה עכשיו
calendar-account-fixing = מטפלים בזה…
calendar-birthdays = ימי הולדת
calendar-tasks = משימות
calendar-birthday-of = יום ההולדת של { $name }
calendar-empty-title = עדיין אין יומנים
calendar-empty-text = היומנים של חשבונות Google ו-Microsoft שלך יוצגו כאן לאחר הסנכרון, וגם יומנים משרתים אחרים שתומכים ב-CalDAV.
calendar-schedule-empty = אין שום דבר מתוכנן בחודשיים הקרובים.
calendar-search = חיפוש באירועים
calendar-search-past = אירועים שעברו
calendar-search-none = אין אירועים שתואמים לחיפוש.
calendar-no-title = (ללא כותרת)
calendar-all-day = כל היום
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } נוספים
calendar-peek-day = { $weekday }, { $day }
calendar-repeats = חוזר
calendar-join = הצטרפות
calendar-join-with = הצטרפות עם { $service }
calendar-email-guests = שליחת מייל לאורחים
calendar-running-late = יש עיכוב
calendar-late-subject = יש עיכוב: { $title }
calendar-late-body = סליחה, יש לי עיכוב של כמה דקות ל-{ $title }. אגיע בקרוב.
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
calendar-open-mail = פתיחת ההודעה
calendar-open-contact = פתיחת איש הקשר
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
# Right-click menus on the calendar: on a free time or day, an event and
# a task.
calendar-menu-new-event = אירוע חדש
calendar-event-window-title = אירוע חדש
# Shows the day right-clicked on its own, in the Day view.
calendar-menu-open-day = פתיחת היום
calendar-menu-duplicate = שכפול
calendar-menu-color = צבע
# The event takes its calendar's color.
calendar-menu-color-calendar = צבע היומן
# A task's new due day, a week from today.
calendar-menu-in-a-week = בעוד שבוע
# Event colors, by the names Google Calendar gives them.
calendar-color-tomato = עגבנייה
calendar-color-flamingo = פלמינגו
calendar-color-tangerine = קלמנטינה
calendar-color-banana = בננה
calendar-color-sage = מרווה
calendar-color-basil = בזיליקום
calendar-color-peacock = טווס
calendar-color-blueberry = אוכמנית
calendar-color-lavender = לבנדר
calendar-color-grape = ענבים
calendar-color-graphite = גרפיט
calendar-menu-only-this = הצגת היומן הזה בלבד
calendar-menu-rename = שינוי שם
calendar-menu-remove = הסרה מהרשימה
calendar-menu-delete = מחיקה
calendar-menu-new-calendar = יומן חדש
calendar-menu-show-all = הצגת הכול
calendar-menu-hide-all = הסתרת הכול
calendar-menu-account-settings = הגדרות החשבון
calendar-why-main = היומן הראשי
calendar-why-last = היחיד כאן
calendar-why-owner = לבעלים בלבד
calendar-why-contacts = מאנשי הקשר
calendar-why-unreached = אין גישה
calendar-name-placeholder = שם היומן
calendar-toast-added = „{ $name }” נוסף
calendar-toast-renamed = שם היומן שונה
calendar-toast-recolored = צבע היומן שונה
calendar-toast-deleted = „{ $name }” נמחק
calendar-toast-removed = „{ $name }” הוסר מהרשימה שלך
calendar-edit-failed = היומן לא שונה: { $reason }
calendar-delete-title = למחוק את „{ $name }”?
calendar-delete-confirm = מחיקה
calendar-deleting = מוחקים…
calendar-delete-heading = יימחקו:
calendar-delete-events = היומן וכל האירועים שלו
calendar-delete-shared = עבור כל מי שהוא משותף איתו
calendar-delete-server = הוא נמחק מ־{ $account } בשירות הדואר, ולא רק ב־Katna.
calendar-delete-local = הוא נמחק מהמחשב הזה.
calendar-remove-title = להסיר את „{ $name }” מהרשימה שלך?
calendar-remove-confirm = הסרה
calendar-removing = מסירים…
calendar-remove-heading = מה ישתנה:
calendar-remove-events = האירועים שלו לא יוצגו לך יותר, כאן ובאפליקציות האחרות שלך
calendar-remove-server = היומן נשאר אצל הבעלים שלו, שיכולים לשתף אותו איתך שוב.
calendar-kind-event = אירוע
calendar-kind-task = משימה
calendar-kind-focus = זמן ריכוז
calendar-kind-out-of-office = מחוץ למשרד
calendar-kind-working-location = מיקום עבודה
calendar-task-added = המשימה נוספה
calendar-task-added-to = המשימה נוספה אל { $list }
calendar-task-list-local = במחשב הזה
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
calendar-invite-by-mail = לא ביומן שלך: התשובה שלך תישלח למארגן במייל.
calendar-mail-yes = התקבלה: { $title }
calendar-mail-yes-body = ההזמנה הזו התקבלה על ידי { $name }.
calendar-mail-no = נדחתה: { $title }
calendar-mail-no-body = ההזמנה הזו נדחתה על ידי { $name }.
calendar-mail-maybe = אולי: { $title }
calendar-mail-maybe-body = ההזמנה הזו התקבלה על ידי { $name } בסימן שאלה.
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
