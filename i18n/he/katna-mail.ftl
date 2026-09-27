# Katna Mail, Hebrew (עברית).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = שפה: { $language }
language-tooltip-system = שפה: { $language }, לפי המערכת
language-search = חיפוש שפה
language-system-default = ברירת המחדל של המערכת
language-system-now = כרגע: { $language }
language-no-match = אין שפה שתואמת את „{ $query }”
language-machine = תרגום מכונה. אפשר לעזור לשפר אותו
language-setting = שפה
language-setting-detail = השפה של התפריטים, הכפתורים וההודעות, ותבנית התאריכים והמספרים. „ברירת המחדל של המערכת” פועלת לפי שולחן העבודה.

## Dates and sizes

ago-just-now = הרגע
ago-minutes = { $count ->
    [one] לפני דקה
    [two] לפני { $count } דקות
   *[other] לפני { $count } דקות
}
ago-hours = { $count ->
    [one] לפני שעה
    [two] לפני שעתיים
   *[other] לפני { $count } שעות
}
ago-days = { $count ->
    [one] לפני יום
    [two] לפני יומיים
   *[other] לפני { $count } ימים
}
size-bytes = { $count ->
    [one] בייט אחד
    [two] { $count } בייטים
   *[other] { $count } בייטים
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = הסתרת התיקיות
folders-show = הצגת התיקיות
compose = כתיבה
search = חיפוש
search-mail = חיפוש באימייל
search-settings = חיפוש בהגדרות
search-clear = ניקוי החיפוש
search-options-show = הצגת אפשרויות החיפוש
settings = הגדרות
account-add = הוספת חשבון

## App rail (and the bottom bar on a phone)

rail-mail = אימייל
rail-calendar = יומן
rail-contacts = אנשי קשר
rail-tasks = משימות
rail-notes = הערות
rail-feeds = פידים

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = בקרוב
app-calendar-promise = יומני CalDAV, הזמנות לפגישות מהאימייל ותזכורות, לצד תיבת הדואר הנכנס.
app-tasks-promise = רשימות משימות שמסונכרנות עם CalDAV, ומשימות שנוצרות מאימייל.
app-notes-promise = הערות מהירות, והערות על אימייל או שיחה לשימוש מאוחר יותר.
app-feeds-promise = קריאת פידים של RSS ו־Atom לצד האימייל.

## Contacts page

app-contacts-loading = אוספים אנשים מהאימייל שלך…
app-contacts-empty = האנשים שהתכתבת איתם יופיעו כאן.
app-contacts-count = { $count ->
    [one] אדם אחד מהאימייל שלך, לפי מידת ההתכתבות
    [two] { $count } אנשים מהאימייל שלך, לפי מידת ההתכתבות
   *[other] { $count } אנשים מהאימייל שלך, לפי מידת ההתכתבות
}
app-contacts-top = { $count ->
    [one] האדם המוביל מהאימייל שלך, לפי מידת ההתכתבות
    [two] { $count } האנשים המובילים מהאימייל שלך, לפי מידת ההתכתבות
   *[other] { $count } האנשים המובילים מהאימייל שלך, לפי מידת ההתכתבות
}
app-contacts-messages = { $count ->
    [one] הודעה אחת
    [two] { $count } הודעות
   *[other] { $count } הודעות
}
app-contacts-last = לאחרונה { $date }

## Navigation (the folders pane)

nav-labels = תוויות
nav-folders = תיקיות
nav-label-new = יצירת תווית חדשה
nav-folder-new = יצירת תיקייה חדשה
nav-account-unnamed = חשבון { $number }
nav-tab-new = { $count ->
    [one] { $count } חדשה
    [two] { $count } חדשות
   *[other] { $count } חדשות
}

## Special folders (the user's own folders keep their names)

folder-inbox = דואר נכנס
folder-starred = מסומנות בכוכב
folder-drafts = טיוטות
folder-sent = נשלחו
folder-archive = ארכיון
folder-spam = ספאם
folder-trash = אשפה
folder-all-mail = כל הדואר
folder-scheduled = מתוזמנות

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = תווית חדשה
label-folder-new-title = תיקייה חדשה
label-prompt = יש להזין שם לתווית החדשה:
label-folder-prompt = יש להזין שם לתיקייה החדשה:
label-name-hint = שם התווית
label-folder-name-hint = שם התיקייה
label-nest = הצבת התווית מתחת ל:
label-folder-nest = הצבת התיקייה מתחת ל:
label-cancel = ביטול
label-create = יצירה
label-creating = בתהליך יצירה…
label-created = התווית „{ $name }” נוצרה.
label-folder-created = התיקייה „{ $name }” נוצרה.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = ראשי
tab-promotions = קידומי מכירות
tab-social = רשתות חברתיות
tab-updates = עדכונים
tab-forums = פורומים
tab-focused = ממוקד
tab-other = אחר
tab-inbox = דואר נכנס
tab-newsletters = ניוזלטרים
tab-notifications = התראות
tab-new = { $count } חדשות
tab-provider-other = ממוין על ידי Katna

## Mail list: toolbar

list-select = בחירה
list-refresh = רענון
list-more = עוד
list-mark-read = סימון כנקראו
list-mark-unread = סימון כלא נקראו
list-move-to = העברה אל
list-archive = העברה לארכיון
list-spam = דיווח על ספאם
list-delete = מחיקה
list-newer = חדשות יותר
list-older = ישנות יותר
list-range = { $first }–{ $last } מתוך { $total }
list-range-about = { $first }–{ $last } מתוך כ־{ $total }
list-results = תוצאות עבור „{ $query }”
list-results-corrected = מוצגות תוצאות עבור „{ $query }”
list-search-instead = חיפוש „{ $query }” במקום זאת
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = הכול
list-pick-none = ללא
list-pick-read = נקראו
list-pick-unread = לא נקראו
list-pick-starred = מסומנות בכוכב
list-pick-unstarred = לא מסומנות בכוכב

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] נבחרה שיחה אחת.
        [two] כל { $count } השיחות נבחרו.
       *[other] כל { $count } השיחות נבחרו.
    }
   *[message] { $count ->
        [one] נבחרה הודעה אחת.
        [two] כל { $count } ההודעות נבחרו.
       *[other] כל { $count } ההודעות נבחרו.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] נבחרה שיחה אחת ב־{ $folder }.
        [two] כל { $count } השיחות ב־{ $folder } נבחרו.
       *[other] כל { $count } השיחות ב־{ $folder } נבחרו.
    }
   *[message] { $count ->
        [one] נבחרה הודעה אחת ב־{ $folder }.
        [two] כל { $count } ההודעות ב־{ $folder } נבחרו.
       *[other] כל { $count } ההודעות ב־{ $folder } נבחרו.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] נבחרה השיחה היחידה שבמסך.
        [two] כל { $count } השיחות שבמסך נבחרו.
       *[other] כל { $count } השיחות שבמסך נבחרו.
    }
   *[message] { $count ->
        [one] נבחרה ההודעה היחידה שבמסך.
        [two] כל { $count } ההודעות שבמסך נבחרו.
       *[other] כל { $count } ההודעות שבמסך נבחרו.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] בחירת שיחה אחת
        [two] בחירת כל { $count } השיחות
       *[other] בחירת כל { $count } השיחות
    }
   *[message] { $count ->
        [one] בחירת הודעה אחת
        [two] בחירת כל { $count } ההודעות
       *[other] בחירת כל { $count } ההודעות
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] בחירת שיחה אחת ב־{ $folder }
        [two] בחירת כל { $count } השיחות ב־{ $folder }
       *[other] בחירת כל { $count } השיחות ב־{ $folder }
    }
   *[message] { $count ->
        [one] בחירת הודעה אחת ב־{ $folder }
        [two] בחירת כל { $count } ההודעות ב־{ $folder }
       *[other] בחירת כל { $count } ההודעות ב־{ $folder }
    }
}
list-clear-selection = ניקוי הבחירה

## Mail list: empty states

list-empty-search = אין הודעות שתואמות לחיפוש.
list-empty-tab = אין דואר ב־{ $tab }.
list-empty-tab-unknown = אין דואר בכרטיסייה הזו.
list-empty-folder = אין הודעות ב־{ $folder }.
list-empty-folder-unknown = אין הודעות בתיקייה הזו.
list-first-sync = מביאים את הדואר שלך…
list-first-sync-detail = הוא יופיע כאן כשיגיע.

## Mail list: lines

row-removed = ההודעה הזו הוסרה.
row-starred = מסומנת בכוכב
row-not-starred = לא מסומנת בכוכב
row-important = חשובה. יש ללחוץ כדי לסמן כלא חשובה.
row-mark-important = סימון כחשובה
row-pinned = מוצמדת למעלה
row-pin = הצמדה למעלה
row-unpin = ביטול ההצמדה

## Mail list: More menu and right-click menu

menu-reply = תשובה
menu-reply-all = תשובה לכולם
menu-forward = העברה
menu-archive = העברה לארכיון
menu-delete = מחיקה
menu-spam = דיווח על ספאם
menu-mark-read = סימון כנקראו
menu-mark-unread = סימון כלא נקראו
menu-mark-all-read = סימון של הכול כנקרא
menu-star = הוספת כוכב
menu-unstar = הסרת הכוכב
menu-important = סימון כחשובה
menu-not-important = סימון כלא חשובה
menu-pin = הצמדה למעלה
menu-unpin = ביטול ההצמדה
menu-print-all = הדפסת הכול
menu-new-window = פתיחה בחלון חדש
menu-move-to = העברה אל
menu-move-to-heading = העברה אל:
menu-find-from = חיפוש הודעות מאת { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] השיחה הועברה לארכיון.
        [two] { $count } שיחות הועברו לארכיון.
       *[other] { $count } שיחות הועברו לארכיון.
    }
   *[message] { $count ->
        [one] ההודעה הועברה לארכיון.
        [two] { $count } הודעות הועברו לארכיון.
       *[other] { $count } הודעות הועברו לארכיון.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] השיחה הועברה לאשפה.
        [two] { $count } שיחות הועברו לאשפה.
       *[other] { $count } שיחות הועברו לאשפה.
    }
   *[message] { $count ->
        [one] ההודעה הועברה לאשפה.
        [two] { $count } הודעות הועברו לאשפה.
       *[other] { $count } הודעות הועברו לאשפה.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] השיחה הועברה.
        [two] { $count } שיחות הועברו.
       *[other] { $count } שיחות הועברו.
    }
   *[message] { $count ->
        [one] ההודעה הועברה.
        [two] { $count } הודעות הועברו.
       *[other] { $count } הודעות הועברו.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] השיחה סומנה בכוכב.
        [two] { $count } שיחות סומנו בכוכב.
       *[other] { $count } שיחות סומנו בכוכב.
    }
   *[message] { $count ->
        [one] ההודעה סומנה בכוכב.
        [two] { $count } הודעות סומנו בכוכב.
       *[other] { $count } הודעות סומנו בכוכב.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] הכוכב הוסר מהשיחה.
        [two] הכוכב הוסר מ־{ $count } שיחות.
       *[other] הכוכב הוסר מ־{ $count } שיחות.
    }
   *[message] { $count ->
        [one] הכוכב הוסר מההודעה.
        [two] הכוכב הוסר מ־{ $count } הודעות.
       *[other] הכוכב הוסר מ־{ $count } הודעות.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] השיחה סומנה כחשובה.
        [two] { $count } שיחות סומנו כחשובות.
       *[other] { $count } שיחות סומנו כחשובות.
    }
   *[message] { $count ->
        [one] ההודעה סומנה כחשובה.
        [two] { $count } הודעות סומנו כחשובות.
       *[other] { $count } הודעות סומנו כחשובות.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] השיחה סומנה כלא חשובה.
        [two] { $count } שיחות סומנו כלא חשובות.
       *[other] { $count } שיחות סומנו כלא חשובות.
    }
   *[message] { $count ->
        [one] ההודעה סומנה כלא חשובה.
        [two] { $count } הודעות סומנו כלא חשובות.
       *[other] { $count } הודעות סומנו כלא חשובות.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] השיחה הוצמדה למעלה.
        [two] { $count } שיחות הוצמדו למעלה.
       *[other] { $count } שיחות הוצמדו למעלה.
    }
   *[message] { $count ->
        [one] ההודעה הוצמדה למעלה.
        [two] { $count } הודעות הוצמדו למעלה.
       *[other] { $count } הודעות הוצמדו למעלה.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] הצמדת השיחה בוטלה.
        [two] ההצמדה של { $count } שיחות בוטלה.
       *[other] ההצמדה של { $count } שיחות בוטלה.
    }
   *[message] { $count ->
        [one] הצמדת ההודעה בוטלה.
        [two] ההצמדה של { $count } הודעות בוטלה.
       *[other] ההצמדה של { $count } הודעות בוטלה.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] השיחה דווחה כספאם.
        [two] { $count } שיחות דווחו כספאם.
       *[other] { $count } שיחות דווחו כספאם.
    }
   *[message] { $count ->
        [one] ההודעה דווחה כספאם.
        [two] { $count } הודעות דווחו כספאם.
       *[other] { $count } הודעות דווחו כספאם.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] השיחה נמחקה לצמיתות.
        [two] { $count } שיחות נמחקו לצמיתות.
       *[other] { $count } שיחות נמחקו לצמיתות.
    }
   *[message] { $count ->
        [one] ההודעה נמחקה לצמיתות.
        [two] { $count } הודעות נמחקו לצמיתות.
       *[other] { $count } הודעות נמחקו לצמיתות.
    }
}
toast-undone = הפעולה בוטלה.
toast-undo = ביטול
toast-no-spam-folder = אין לחשבון הזה תיקיית ספאם.

## Reading pane: toolbar

reader-close = סגירה
reader-back = חזרה
reader-mark-unread = סימון כלא נקראה
reader-move-to = העברה אל
reader-more = עוד
reader-print-all = הדפסת הכול
reader-new-window = בחלון חדש
reader-position = { $position } מתוך { $total }
reader-newer = חדשה יותר
reader-older = ישנה יותר

## Reading pane: the conversation

reader-removed = השיחה הזו הוסרה.
reader-no-subject = (ללא נושא)
reader-collapse-all = כיווץ הכול
reader-expand-all = הרחבת הכול
reader-unknown-sender = (שולח לא ידוע)
reader-date-ago = { $date } ({ $ago })
reader-me = אני
reader-to = אל { $names }
reader-starred = מסומנת בכוכב
reader-not-starred = לא מסומנת בכוכב
reader-too-long = ההודעה ארוכה מדי ולא ניתן להציג אותה במלואה.
reader-encrypted-images = תמונות מהאינטרנט אף פעם לא נטענות בדואר מוצפן.
reader-window-failed = לא ניתן לפתוח חלון חדש.

## Reading pane: message details (opened from "to me")

reader-details-from = מאת:
reader-details-to = אל:
reader-details-cc = עותק:
reader-details-date = תאריך:
reader-details-subject = נושא:

## Reading pane: downloading a message

reader-downloading = ההודעה הזו מורדת מהשרת…
reader-download-failed = לא ניתן להוריד את ההודעה הזו.
reader-try-again = ניסיון נוסף

## Reply row

reply-reply = תשובה
reply-reply-all = תשובה לכולם
reply-forward = העברה

## Encrypted and signed mail

security-decrypting = מתבצע פענוח…
security-checking = החתימה נבדקת…
security-partly-encrypted = רק חלק מההודעה הזו מוצפן. שאר התוכן נוסף מחוץ להגנה ויכול להגיע מכל אחד.
security-partly-signed = רק חלק מההודעה הזו חתום. שאר התוכן נוסף מחוץ להגנה ויכול להגיע מכל אחד.
security-encrypted = הודעה מוצפנת
security-encrypted-smime = הודעה מוצפנת (S/MIME)
security-no-key = לא ניתן לפענח את ההודעה הזו: היא הוצפנה למפתח שאין לך.
security-cancelled = הפענוח בוטל.
security-damaged = לא ניתן לפענח את ההודעה הזו: הנתונים המוצפנים פגומים או ששונו.
security-decrypt-unavailable = לא ניתן לפענח את ההודעה הזו: יש להתקין את { $tool } כדי לקרוא דואר מוצפן.
security-decrypt-failed = לא ניתן לפענח את ההודעה הזו: { $reason }
security-unknown-signer = חותם לא ידוע
security-signed-verified = נחתמה על ידי { $signer } · מאומתת
security-signed-not-sender = נחתמה על ידי { $signer }, שאינו השולח
security-signed-untrusted = נחתמה על ידי { $signer }, במפתח שסימנת כלא מהימן
security-signed-unverified = נחתמה על ידי { $signer } · המפתח לא אומת
security-bad-signature = חתימה פגומה: ההודעה הזו שונתה אחרי שנחתמה, או שהחתימה מזויפת.
security-signature-expired = נחתמה על ידי { $signer } · תוקף החתימה פג
security-key-expired = נחתמה על ידי { $signer } · תוקף המפתח פג מאז
security-key-revoked = נחתמה על ידי { $signer } במפתח שבוטל
security-missing-key = נחתמה במפתח שאין לך, ולכן לא ניתן לבדוק אותה
security-missing-key-id = נחתמה במפתח שאין לך ({ $key }), ולכן לא ניתן לבדוק אותה
security-signature-unavailable = חתומה; יש להתקין את { $tool } כדי לבדוק את החתימה
security-signature-error = לא ניתן היה לבדוק את החתימה.

## Remote images and pictures

remote-hidden = התמונות בהודעה הזו מוסתרות.
remote-show = הצגת התמונות
remote-always-show = תמיד להציג מהשולח הזה
remote-picture-use = שימוש
remote-picture-too-big = יש לבחור תמונה בגודל 8 MB לכל היותר.
remote-picture-type = יש לבחור תמונה מסוג PNG, JPEG, GIF, WebP או SVG.
remote-picture-read-failed = לא ניתן לקרוא את התמונה: { $error }
remote-picture-keep-failed = לא ניתן לשמור את התמונה: { $error }
remote-picture-remove-failed = לא ניתן להסיר את התמונה: { $error }

## Attachments

attachment-count = { $count ->
    [one] קובץ מצורף אחד
    [two] { $count } קבצים מצורפים
   *[other] { $count } קבצים מצורפים
}
attachment-save = שמירה
attachment-save-all = שמירת הכול
attachment-save-all-tooltip = שמירת כל הקבצים המצורפים בתיקייה
attachment-save-here = שמירה כאן
attachment-not-downloaded = ההודעה הזו לא הורדה.
attachment-not-found = הקובץ המצורף הזה לא נמצא בהודעה.
attachment-read-failed = לא ניתן היה לקרוא את { $name }
attachment-numbered = קובץ מצורף { $number }
attachment-saved-all = { $count ->
    [one] קובץ אחד נשמר ב־{ $place }
    [two] { $count } קבצים נשמרו ב־{ $place }
   *[other] { $count } קבצים נשמרו ב־{ $place }
}
attachment-saved-some = { $total ->
    [one] { $saved } מתוך קובץ אחד נשמרו ב־{ $place }. לא ניתן היה לשמור את { $failed }
    [two] { $saved } מתוך { $total } קבצים נשמרו ב־{ $place }. לא ניתן היה לשמור את { $failed }
   *[other] { $saved } מתוך { $total } קבצים נשמרו ב־{ $place }. לא ניתן היה לשמור את { $failed }
}
attachment-saved-to = נשמר ב־{ $path }
attachment-save-failed = לא ניתן היה לשמור את { $name }: { $error }
attachment-open-failed = לא ניתן היה לפתוח את { $name }: { $error }
attachment-risky = הקובץ הזה יכול להריץ תוכנה, ולכן Katna לא פותחת אותו. אפשר לשמור אותו במקום זאת.
attachment-encrypted-open = הקובץ הזה הגיע מוצפן. יש לשמור אותו כדי לפתוח אותו במקום אחר.

## Printing

print-failed = לא ניתן היה להדפיס: { $error }
print-no-font = לא נמצא גופן
print-opened-as-pdf = נפתח כקובץ PDF כדי להדפיס ממנו.
print-not-downloaded = (עדיין לא הורדה.)
print-encrypted = (מוצפנת. יש לפתוח אותה ב־Katna Mail כדי להדפיס את הטקסט שלה.)
print-to = אל: { $addresses }
print-cc = עותק: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = יש לפתוח את ההודעה הזו כדי לקרוא את הקבצים המצורפים שלה.
text-copy = העתקה
text-select-all = בחירת הכול

## Settings page: its tabs

settings-tab-general = כללי
settings-tab-inbox = דואר נכנס
settings-tab-accounts = חשבונות
settings-tab-subscriptions = מינויים
settings-tab-appearance = מראה
settings-tab-shortcuts = קיצורי דרך
settings-tab-default-apps = אפליקציות ברירת מחדל
settings-tab-folders-rules = תיקיות וכללים
settings-tab-compose = כתיבה
settings-tab-mcp-server = שרת MCP
settings-tab-feedback = משוב משתמשים
settings-tab-experimental = ניסיוני

## Settings page: tabs still to come

settings-tab-subscriptions-coming = צפייה בניוזלטרים וברשימות התפוצה שמגיעים אליך, וביטול מינוי בלחיצה אחת.
settings-tab-folders-rules-coming = יצירה, שינוי שם, העברה והסתרה של תיקיות ותוויות, ובחירה אילו מהן יסונכרנו. כללים ממיינים, מתייגים, מעבירים או מוחקים דואר חדש באופן אוטומטי, לפי שולח, נושא או מילים.
settings-tab-mcp-server-coming = מתן אפשרות לעוזרי AI במחשב הזה לחפש, לקרוא ולנסח טיוטות בדואר שלך, באישורך.

## Settings > General

settings-general-conversations = תצוגת שיחות
settings-general-conversations-group = קיבוץ תשובות לאותה הודעה
settings-general-conversations-group-detail = שורה אחת לכל שיחה ברשימה
settings-general-reading = קריאה
settings-general-newest-first = ההודעה החדשה ביותר ראשונה
settings-general-newest-first-detail = השיחה מתחילה בתשובה האחרונה שלה
settings-general-full-headers = הצגת כותרות מלאות
settings-general-full-headers-detail = מאת, אל, עותק, תאריך ונושא פתוחים בכל הודעה
settings-general-full-names = שמות מלאים של הנמענים
settings-general-full-names-detail = „אליי, Ada Lovelace” במקום „אליי, Ada”
settings-general-mark-read = סימון כנקראה
settings-general-mark-read-now = מיד כשהיא נפתחת
settings-general-mark-read-1s = אחרי שהיא פתוחה שנייה אחת
settings-general-mark-read-3s = אחרי שהיא פתוחה 3 שניות
settings-general-mark-read-never = רק בסימון ידני כנקראה
settings-general-reply-button = כפתור התשובה
settings-general-reply-all = תשובה לכולם
settings-general-reply-all-detail = כפתור התשובה שליד כל הודעה עונה לכולם, ולא רק לשולח
settings-general-remote-images = תמונות מהאינטרנט
settings-general-remote-images-detail = טעינת התמונות של הודעה מגלה לשולח שפתחת אותה, מתי ובערך מאיפה. כשהאפשרות כבויה, כל הודעה שואלת קודם, ותמיד אפשר להציג את התמונות של שולח מסוים.
settings-general-remote-images-always = תמיד להציג תמונות
settings-general-remote-images-always-detail = בכל הודעה, ולא רק משולחים מהימנים
settings-general-sending = שליחה
settings-general-sending-detail = כמה זמן הודעה שנשלחה ממתינה, כדי שאפשר יהיה לבטל את שליחתה.
settings-general-offline = דואר במצב לא מקוון
settings-general-offline-detail = דואר אחרון מורד במלואו, לקריאה ללא חיבור. דואר ישן יותר מורד כשפותחים אותו.
settings-general-offline-days = { $count ->
    [one] יום אחד
    [two] יומיים
   *[other] { $count } ימים
}
settings-general-offline-years = { $count ->
    [one] שנה אחת
    [two] שנתיים
   *[other] { $count } שנים
}
settings-general-offline-all = כל הדואר
settings-general-offline-note = בחירה בפחות ימים משאירה את הדואר שכבר הורד. שום דבר לא משתנה בשרת.
settings-general-notifications = התראות
settings-general-notifications-detail = על דואר חדש בדואר הנכנס, גם כש־Katna Mail סגורה.
settings-general-new-mail = התראה על דואר חדש
settings-general-new-mail-detail = עם „תשובה לכולם”, „סימון כנקראה” ו„העברה לארכיון”
settings-general-new-mail-sound = השמעת צליל
settings-general-new-mail-sound-detail = צליל הדואר החדש של שולחן העבודה
settings-general-desktop = שולחן העבודה
settings-general-open-at-login = פתיחת Katna Mail בכניסה למערכת
settings-general-open-at-login-detail = הדואר מסתנכרן בכניסה בכל מקרה, כל עוד השירות פועל
settings-general-tray = הצגת Katna במגש המערכת
settings-general-tray-detail = עם מספר ההודעות שלא נקראו ותפריט
settings-general-unread-badge = מספר ההודעות שלא נקראו על סמל שורת המשימות
settings-general-unread-badge-detail = כמה הודעות בדואר הנכנס לא נקראו

## Settings > Inbox

settings-inbox-tabs = כרטיסיות בדואר הנכנס
settings-inbox-tabs-detail = מיון הדואר הנכנס לכרטיסיות, כמו באתר של ספק הדואר שלך.
settings-inbox-tabs-show = הצגת כרטיסיות בדואר הנכנס
settings-inbox-tabs-show-detail = כשהאפשרות כבויה מוצגת רשימה אחת לכל חשבון
settings-inbox-no-accounts = יש להוסיף חשבון כדי לבחור את הכרטיסיות שלו.
settings-inbox-tabs-automatic = אוטומטי: { $tabs } ({ $provider })
settings-inbox-tabs-off = ללא כרטיסיות
settings-inbox-tabs-gmail = ראשי, קידומי מכירות, רשתות חברתיות, עדכונים, פורומים
settings-inbox-tabs-focused = ממוקד ואחר
settings-inbox-tabs-zoho = דואר נכנס, ניוזלטרים והתראות
settings-inbox-tabs-shown = הכרטיסיות המוצגות. דואר של כרטיסייה שמכבים נשאר ב־{ $tab }.

## Settings > Appearance

settings-appearance-reading-pane = חלונית הקריאה
settings-appearance-reading-pane-detail = איפה מוצגת שיחה פתוחה.
settings-appearance-pane-right = ליד הרשימה
settings-appearance-pane-none = ללא פיצול
settings-appearance-density = צפיפות
settings-appearance-density-default = ברירת מחדל
settings-appearance-density-compact = דחוס
settings-appearance-scaling = קנה מידה
settings-appearance-scaling-detail = מגדיל או מקטין את כל מה שב־Katna Mail, בנוסף לקנה המידה של שולחן העבודה עצמו: טקסט, סמלים, ריווח וקווים מפרידים. דואר שנשלח שומר על גודל הגופן שלו. גדלים קטנים מאוד עלולים להקשות על לחיצה על סמלים.
settings-appearance-theme = ערכת נושא
settings-appearance-theme-system = כמו שולחן העבודה
settings-appearance-theme-light = בהירה
settings-appearance-theme-dark = כהה
settings-appearance-desktop-colors = צבעי שולחן העבודה
settings-appearance-desktop-colors-use = שימוש בצבעי שולחן העבודה
settings-appearance-desktop-colors-use-detail = ערכת הצבעים וצבע ההדגשה של שולחן העבודה
settings-appearance-app-names = שמות האפליקציות
settings-appearance-app-names-show = הצגת שמות האפליקציות
settings-appearance-app-names-show-detail = שמות מתחת לסמלי האפליקציות בקצה הימני
settings-appearance-sender-pictures = תמונות שולחים
settings-appearance-sender-pictures-show = הצגת לוגואים של חברות
settings-appearance-sender-pictures-show-detail = מאותרים לפי הדומיין של השולח, אף פעם לא לפי ההודעה, ונשמרים לשבוע
settings-appearance-important = סמני חשיבות
settings-appearance-important-show = הצגת סמני חשיבות
settings-appearance-important-show-detail = ליד כל הודעה ברשימה
settings-appearance-message-width = רוחב ההודעה
settings-appearance-message-width-limit = הגבלת רוחב ההודעות
settings-appearance-message-width-limit-detail = קל יותר לקרוא שורות ארוכות בחלון רחב
settings-appearance-mail-colors = צבעי הדואר
settings-appearance-mail-colors-detail = רוב הדואר מעוצב לדף לבן. בערכת נושא כהה הצבעים שלו מוחלפים בצבעים כהים שקל לקרוא; כשהאפשרות כבויה, הוא שומר על צבעי השולח על דף בהיר.
settings-appearance-dark-mail = צבעים כהים גם לדואר
settings-appearance-dark-mail-detail = רק כשערכת הנושא כהה
settings-appearance-attachment-previews = תצוגות מקדימות של קבצים מצורפים
settings-appearance-attachment-previews-show = הצגת תצוגות מקדימות של קבצים מצורפים
settings-appearance-attachment-previews-show-detail = תמונה קטנה של תוכן כל קובץ על הכרטיס שלו

## Settings > Default apps

settings-default-apps-intro = איפה נפתחים קבצים מצורפים כשלוחצים עליהם. המציג תמיד יכול לפתוח קובץ גם באפליקציה אחרת. אפליקציות ברירת המחדל של שולחן העבודה נקבעות בהגדרות שלו.
settings-default-apps-pdf = קובצי PDF
settings-default-apps-pdf-detail = עמודים, עם זום.
settings-default-apps-pictures = תמונות
settings-default-apps-pictures-detail = צילומים (מסובבים לכיוון הנכון), PNG, GIF, WebP, BMP, TIFF ו־SVG.
settings-default-apps-text = קובצי טקסט
settings-default-apps-text-detail = טקסט פשוט, יומנים, קוד וטקסט אחר.
settings-default-apps-sheets = גיליונות אלקטרוניים
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) ו־CSV.
settings-default-apps-documents = מסמכים
settings-default-apps-documents-detail = Word (docx) וטקסט OpenDocument (odt).
settings-default-apps-katna = המציג של Katna Mail
settings-default-apps-system = אפליקציית ברירת המחדל של שולחן העבודה
settings-default-apps-ask = לשאול בכל פעם באיזו אפליקציה
settings-default-apps-after-saving = אחרי השמירה
settings-default-apps-show-folder = הצגת קבצים שנשמרו בתיקייה שלהם
settings-default-apps-show-folder-detail = פותח את מנהל הקבצים כשהקבצים המצורפים שנשמרו מסומנים

## Settings > Compose

settings-compose-send-from = שליחת הודעות חדשות מהחשבון
settings-compose-send-from-detail = תשובות והעברות תמיד יוצאות מהחשבון שבו נמצאים.
settings-compose-send-from-current = החשבון שבו נמצאים
settings-compose-send-on-replies = שליחה בתשובות
settings-compose-send-on-replies-detail = מה „שליחה” עושה בתשובה או בהעברה. התפריט שליד „שליחה” מציע את האפשרות השנייה.
settings-compose-send-plain = שליחה
settings-compose-send-archive = שליחה והעברה לארכיון
settings-compose-signatures = חתימות
settings-compose-signatures-detail = נוספת מתחת להודעה, אחרי שורת „--”. אפשר לבחור חתימה אחרת בחלון הכתיבה.
settings-compose-untitled = ללא שם
settings-compose-signature-name = שם, למשל עבודה
settings-compose-signature-first = החתימה שלי
settings-compose-signature-numbered = חתימה { $number }
settings-compose-signature-delete = מחיקה
settings-compose-signature-deleted = החתימה נמחקה
settings-compose-signature-new = יצירת חתימה חדשה
settings-compose-no-signatures = אין עדיין חתימות.
settings-compose-no-signature = ללא חתימה
settings-compose-for-new-mail = להודעות חדשות
settings-compose-for-replies = לתשובות ולהעברות
settings-compose-for-replies-detail = בשיחה שבה חתמת על הודעה, תשובה מתחילה במקום זאת באותה חתימה.
settings-compose-format = פורמט
settings-compose-plain-text = כתיבה בטקסט פשוט
settings-compose-plain-text-detail = דואר חדש מתחיל ללא עיצוב; אפשר להחליף בחלון הכתיבה
settings-compose-spelling = איות
settings-compose-spell-check = בדיקת איות בזמן הכתיבה
settings-compose-spell-check-detail = מילים עם שגיאות איות מסומנות בקו תחתון, עם הצעות בלחיצה ימנית
settings-compose-spell-desktop = השפה של שולחן העבודה ({ $language })
settings-compose-templates = תבניות
settings-compose-templates-detail = שמירת הודעות שכותבים לעיתים קרובות, והתחלת הודעה חדשה או תשובה מהן.

## Settings > Shortcuts

settings-shortcuts-set = ערכת קיצורים
settings-shortcuts-set-detail = אפשר להתחיל מהמקשים של אפליקציית דואר מוכרת. Cmd הוא Ctrl כאן. השינויים שלך נשמרים מעל הערכה, ו„שחזור ברירות המחדל” מחזיר את המקשים של הערכה.
settings-shortcuts-single = קיצורי מקש יחיד
settings-shortcuts-single-detail = מקשים בלי Ctrl או Alt, כמו בדואר אינטרנט: e מעביר לארכיון, j ו־k מזיזים, / מחפש. הם פועלים ברשימה ובשיחה הפתוחה, אף פעם לא בזמן הקלדה.
settings-shortcuts-single-use = שימוש בקיצורי מקש יחיד
settings-shortcuts-single-use-detail = קיצורי Ctrl תמיד פועלים
settings-shortcuts-how = יש ללחוץ על מקש כדי לשנות אותו, או על + כדי להוסיף, ואז להקיש את המקשים החדשים. Esc מבטל.
settings-shortcuts-restore = שחזור ברירות המחדל
settings-shortcuts-no-key = אין מקש
settings-shortcuts-press = יש להקיש על מקשים…
settings-shortcuts-then = { $keys } ואז…
settings-shortcuts-moved = { $keys } מבצע עכשיו „{ $action }” במקום „{ $previous }”.
settings-shortcuts-single-off = קיצורי מקש יחיד כבויים, לכן המקש הזה יפעל אחרי שיופעלו.
settings-shortcuts-restored = כל הקיצורים חזרו למקשים של הערכה שלהם.

## Settings search: the line under a result

settings-general-language-summary = שפת האפליקציה, התאריכים והמספרים
settings-general-reading-summary = ההודעה החדשה ביותר ראשונה, כותרות מלאות, שמות מלאים של הנמענים
settings-general-mark-read-summary = מתי שיחה פתוחה מסומנת כנקראה: מיד, אחרי 1 או 3 שניות, או ידנית
settings-general-reply-button-summary = כפתור התשובה שליד כל הודעה עונה לכולם
settings-general-remote-images-summary = תמיד להציג את התמונות של כל הודעה
settings-general-sending-summary = ביטול שליחה: כמה זמן הודעה שנשלחה ממתינה, כדי שאפשר יהיה לבטל את שליחתה
settings-general-offline-summary = כמה ימים של דואר אחרון מורדים במלואם, לקריאה ללא חיבור
settings-general-notifications-summary = התראות על דואר חדש והצליל שלהן
settings-general-desktop-summary = פתיחת Katna Mail בכניסה למערכת, הסמל במגש המערכת ומספר ההודעות שלא נקראו על סמל שורת המשימות
settings-accounts-accounts-summary = הוספה או הסרה של חשבון, או החלפת התמונה שלו
settings-appearance-density-summary = שורות ברירת מחדל או דחוסות ברשימה
settings-appearance-scaling-summary = הגדלה או הקטנה של הכול: טקסט, סמלים, ריווח וקווים מפרידים
settings-appearance-theme-summary = כמו שולחן העבודה, בהירה או כהה
settings-appearance-sender-pictures-summary = לוגואים של חברות, מאותרים לפי הדומיין של השולח
settings-appearance-important-summary = סמן החשיבות ליד כל הודעה ברשימה
settings-appearance-mail-colors-summary = צבעים כהים לדואר HTML בערכת נושא כהה, או צבעי השולח
settings-appearance-attachment-previews-summary = תמונה קטנה של תוכן כל קובץ מצורף
settings-shortcuts-set-summary = התחלה מהמקשים של Gmail, Inbox by Gmail, Apple Mail, Outlook או Thunderbird
settings-shortcuts-single-summary = מקשים בלי Ctrl או Alt, כמו בדואר אינטרנט
settings-default-apps-pdf-summary = איפה נפתחים קבצים מצורפים מסוג PDF
settings-default-apps-pictures-summary = איפה נפתחים צילומים ותמונות
settings-default-apps-text-summary = איפה נפתחים טקסט פשוט, יומנים וקוד
settings-default-apps-sheets-summary = איפה נפתחים קובצי Excel, OpenDocument ו־CSV
settings-default-apps-documents-summary = איפה נפתחים Word וטקסט OpenDocument
settings-default-apps-after-saving-summary = הצגת קבצים מצורפים שנשמרו בתיקייה שלהם
settings-compose-send-from-summary = החשבון שממנו יוצא דואר חדש: החשבון שבו נמצאים, או תמיד אותו חשבון
settings-compose-send-on-replies-summary = „שליחה” או „שליחה והעברה לארכיון” של השיחה, בתשובות ובהעברות
settings-compose-signatures-summary = נוספת מתחת להודעה, אחרי שורת „--”
settings-compose-for-new-mail-summary = החתימה שבה מתחיל דואר חדש
settings-compose-for-replies-summary = החתימה שבה מתחילות תשובות והעברות
settings-compose-format-summary = כתיבת דואר חדש בטקסט פשוט
settings-compose-spelling-summary = בדיקת איות בזמן הכתיבה, ושפת המילון
settings-compose-templates-summary = בקרוב: שמירת הודעות שכותבים לעיתים קרובות, והתחלת הודעה חדשה או תשובה מהן
settings-feedback-crash-reports-summary = שמירת דוחות קריסה במחשב הזה כש־Katna Mail או שירות הרקע שלה קורסים
settings-feedback-saved-summary = הצגה, העתקה או מחיקה של דוחות הקריסה שנשמרו במחשב הזה
settings-feedback-help-improve-summary = שליחת דוחות קריסה כדי לעזור לתקן את מה שהשתבש; כבוי אלא אם מפעילים אותו
settings-experimental-blur-summary = שולחן העבודה נראה מבעד לסרגל העליון, מטושטש, והתפריטים כמו זכוכית חלבית
settings-search-shortcut = קיצור מקלדת
settings-search-tab = כרטיסיית הגדרות
settings-search-none = אין הגדרות שתואמות את „{ $query }”.
settings-search-results = הגדרות שתואמות את „{ $query }”

## Quick settings (the panel that slides in from the right)

quick-title = הגדרות מהירות
quick-see-all = הצגת כל ההגדרות
quick-reading-pane = חלונית הקריאה
quick-pane-right = ליד הרשימה
quick-pane-none = ללא פיצול
quick-density = צפיפות
quick-density-default = ברירת מחדל
quick-density-compact = דחוס
quick-theme = ערכת נושא
quick-theme-system = כמו שולחן העבודה
quick-theme-light = בהירה
quick-theme-dark = כהה
quick-desktop-colors = צבעי שולחן העבודה
quick-desktop-colors-detail = ערכת הצבעים וצבע ההדגשה של שולחן העבודה
quick-app-names = שמות האפליקציות
quick-app-names-detail = שמות מתחת לסמלי האפליקציות בקצה הימני
quick-inbox-tabs = כרטיסיות בדואר הנכנס
quick-inbox-tabs-detail = הכרטיסיות של ספק הדואר של כל חשבון
quick-choose-tabs = בחירת כרטיסיות
quick-choose-tabs-detail = לכל חשבון, בהגדרות
quick-sending = שליחה
quick-undo-send = ביטול שליחה
quick-undo-send-off = כבוי
quick-undo-send-seconds = { $seconds } שנ׳
quick-signatures = חתימות
quick-signatures-none = אין עדיין
quick-signatures-one = { $name }, ברירת המחדל
quick-signatures-many = { $count ->
    [one] חתימה אחת; { $name } כברירת מחדל
    [two] { $count } חתימות; { $name } כברירת מחדל
   *[other] { $count } חתימות; { $name } כברירת מחדל
}
quick-signatures-no-default = { $count ->
    [one] { $count }, ללא ברירת מחדל
    [two] { $count }, ללא ברירת מחדל
   *[other] { $count }, ללא ברירת מחדל
}
quick-signature-untitled = ללא שם
quick-threading = שרשור הודעות
quick-conversation-view = תצוגת שיחות
quick-conversation-view-detail = קיבוץ תשובות לאותה הודעה
quick-help = עזרה
quick-tour = סיור מודרך
quick-whats-new = מה חדש
quick-about = מידע על Katna

## Settings: opening at login

settings-open-at-login-failed = לא ניתן היה לשנות את הפתיחה בכניסה למערכת: { $error }

## Settings > Appearance > Scaling

scale-letter = א
scale-percent = { $percent }%
scale-reset = חזרה ל־{ $percent }%

## Settings > Experimental > Look & Feel

look-intro = תכונות שעדיין בניסוי. הן עשויות להשתנות או להיעלם.
look-heading = מראה ותחושה
look-window-frame = מסגרת החלון
look-window-frame-detail = מי מצייר את שורת הכותרת, כפתורי החלון, הפינות והצל.
look-frame-native-kde = מקורי: המסגרת של KDE, בערכת הנושא של Plasma שלך
look-frame-native = מקורי: המסגרת של שולחן העבודה
look-frame-katna = Katna: הסרגל העליון הופך לשורת הכותרת
look-frame-katna-note-named = Katna מציירת פינות מעוגלות וצל משלה. המסגרת כבר לא עוקבת אחר ערכת הנושא של { $desktop }; כללי החלונות עדיין חלים.
look-frame-katna-note = Katna מציירת פינות מעוגלות וצל משלה. המסגרת כבר לא עוקבת אחר ערכת הנושא של שולחן העבודה; כללי החלונות עדיין חלים.
look-frame-client-side = שולחן העבודה שלך משאיר את המסגרת לכל אפליקציה, כך ש־Katna כבר מציירת מסגרת משלה.
look-blurred-background = רקע מטושטש
look-blurred-background-detail = שולחן העבודה נראה מבעד לסרגל העליון ולתיקיות, מטושטש, והתפריטים והחלונות הקופצים עשויים זכוכית חלבית.
look-blur = טשטוש מה שמאחורי החלון
look-blur-detail = הדואר נשאר על כרטיסים אטומים, כך שהטקסט שומר על הניגודיות שלו
look-blur-off-kde = אפקט הטשטוש של KDE כבוי. יש להפעיל את „טשטוש” ב„הגדרות המערכת”, „ניהול חלונות”, „אפקטים של שולחן העבודה”, ואז לפתוח את Katna Mail מחדש.
look-blur-none-gnome = GNOME לא מטשטש את מה שמאחורי החלונות.
look-blur-none-x11 = מנהל החלונות שלך לא מטשטש את מה שמאחורי החלונות.
look-blur-none-wayland = הקומפוזיטור שלך לא מטשטש את מה שמאחורי החלונות.

## Settings > User feedback (crash reports)

feedback-intro-sending = דוחות קריסה חדשים נשלחים כדי לעזור לתקן את מה שהשתבש. שום דבר אחר לא יוצא מהמחשב הזה.
feedback-intro-local = Katna לא שולחת שום דבר לשום מקום. דוחות הקריסה נשארים במחשב הזה, לעיון או לצירוף לדיווח על באג.
feedback-crash-reports = דוחות קריסה
feedback-crash-reports-detail = נכתבים כש־Katna Mail או שירות הרקע שלה קורסים.
feedback-save = שמירת דוחות קריסה במחשב הזה
feedback-save-detail = תיקיית הבית, שמות המשתמש והמחשב וכתובות האימייל שלך לא נכללים
feedback-saved = דוחות קריסה שמורים
feedback-saved-detail = { $count ->
    [one] נשמר רק הדוח האחרון.
    [two] נשמרים { $count } הדוחות האחרונים.
   *[other] נשמרים { $count } הדוחות האחרונים.
}
feedback-help-improve = עזרה בשיפור Katna
feedback-help-improve-detail = כבוי אלא אם מפעילים אותו, ואפשר לכבות אותו כאן בכל עת.
feedback-send = שליחת דוחות קריסה
feedback-send-detail = הדוח השמור, בדיוק כפי שאפשר לראות אותו כאן, נשלח למערכת מעקב הקריסות של Katna (Sentry, באיחוד האירופי). ללא כתובת IP, הודעות או כתובות אימייל
feedback-none-saved = אין דוחות קריסה שמורים.
feedback-delete-all = מחיקת הכול
feedback-app-daemon = שירות רקע
feedback-report-sent = { $date } · נשלח
feedback-view = הצגה
feedback-view-tooltip = פתיחת הדוח
feedback-copy-tooltip = העתקה להדבקה בדיווח על באג
feedback-copied = דוח הקריסה הועתק.
feedback-deleted-all = דוחות הקריסה נמחקו.
feedback-read-failed = לא ניתן היה לקרוא את דוח הקריסה: { $error }
feedback-delete-failed = לא ניתן היה למחוק את דוח הקריסה: { $error }
feedback-delete-all-failed = לא ניתן היה למחוק את דוחות הקריסה: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _קובץ
desktop-menu-new-message = _הודעה חדשה
desktop-menu-quit = _יציאה
desktop-menu-edit = _עריכה
desktop-menu-undo = _ביטול פעולה
desktop-menu-select-all = _בחירת הכול
desktop-menu-select-none = _ביטול הבחירה
desktop-menu-find = _חיפוש…
desktop-menu-view = _תצוגה
desktop-menu-folder-list = _הצגת רשימת התיקיות
desktop-menu-refresh = _רענון
desktop-menu-go = _מעבר
desktop-menu-inbox = _דואר נכנס
desktop-menu-starred = _מסומנות בכוכב
desktop-menu-sent = _נשלחו
desktop-menu-drafts = _טיוטות
desktop-menu-all-mail = _כל הדואר
desktop-menu-next = _השיחה הבאה
desktop-menu-previous = _השיחה הקודמת
desktop-menu-message = _הודעה
desktop-menu-open = _פתיחה
desktop-menu-reply = _תשובה
desktop-menu-reply-all = _תשובה לכולם
desktop-menu-forward = _העברה
desktop-menu-archive = _העברה לארכיון
desktop-menu-delete = _מחיקה
desktop-menu-spam = _דיווח על ספאם
desktop-menu-move-to = _העברה אל…
desktop-menu-mark-read = _סימון כנקראה
desktop-menu-mark-unread = _סימון כלא נקראה
desktop-menu-star = _הוספת כוכב
desktop-menu-important = _סימון כחשובה
desktop-menu-not-important = _סימון כלא חשובה
desktop-menu-settings = _הגדרות
desktop-menu-quick-settings = _הגדרות מהירות
desktop-menu-configure = _הגדרת Katna Mail…
desktop-menu-help = _עזרה
desktop-menu-shortcuts = _קיצורי מקלדת
desktop-menu-whats-new = _מה חדש
desktop-menu-about = _על Katna

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = ניווט
shortcut-group-actions = פעולות
shortcut-group-go-to = מעבר אל
shortcut-group-app = אפליקציה

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = השיחה הבאה
shortcut-previous = השיחה הקודמת
shortcut-down = ירידה ברשימה
shortcut-up = עלייה ברשימה
shortcut-first = הראשונה ברשימה
shortcut-last = האחרונה ברשימה
shortcut-page-down = עמוד למטה ברשימה
shortcut-page-up = עמוד למעלה ברשימה
shortcut-open = פתיחת השיחה
shortcut-back = חזרה לרשימה
shortcut-scroll-down = גלילה למטה
shortcut-scroll-up = גלילה למעלה
shortcut-scroll-page-down = גלילה עמוד למטה
shortcut-scroll-page-up = גלילה עמוד למעלה
shortcut-compose = כתיבה
shortcut-reply = תשובה
shortcut-reply-all = תשובה לכולם
shortcut-forward = העברה
shortcut-archive = העברה לארכיון
shortcut-delete = מחיקה
shortcut-spam = דיווח על ספאם
shortcut-move-to = העברה אל
shortcut-mark-read = סימון כנקראה
shortcut-mark-unread = סימון כלא נקראה
shortcut-star = הוספה או הסרה של כוכב
shortcut-important = סימון כחשובה
shortcut-not-important = סימון כלא חשובה
shortcut-check = סימון השיחה
shortcut-select-all = סימון כל השיחות
shortcut-select-none = ביטול הסימון של כל השיחות
shortcut-undo = ביטול הפעולה האחרונה
shortcut-go-inbox = דואר נכנס
shortcut-go-starred = מסומנות בכוכב
shortcut-go-sent = נשלחו
shortcut-go-drafts = טיוטות
shortcut-go-all = כל הדואר
shortcut-search = חיפוש בדואר
shortcut-navigation = הצגה או קיפול של התפריט
shortcut-quick-settings = הגדרות מהירות
shortcut-settings = כל ההגדרות
shortcut-shortcuts = קיצורי מקלדת
shortcut-reload = בדיקת דואר חדש
shortcut-quit = יציאה

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } ואז { $second }

## Settings > Accounts

accounts-folder-pane = חלונית התיקיות
accounts-folder-pane-detail = התיקיות של אילו חשבונות מוצגות בחלונית שבצד ימין.
accounts-shown-one = חשבון אחד בכל פעם; מחליפים בכרטיס החשבון
accounts-shown-all = כל החשבונות, אחד אחרי השני
accounts-row = חשבונות
accounts-row-detail = הסרת חשבון מוחקת את העותק של Katna מהדואר שלו במחשב הזה. הדואר נשאר בשרת.
accounts-none = אין עדיין חשבונות.
accounts-kind-imported = מיובא
accounts-picture-reset = שימוש בתמונה של שולחן העבודה
accounts-picture-change = החלפת התמונה
accounts-remove = הסרה
accounts-delete-all-row = מחיקת כל הנתונים
accounts-delete-all-row-detail = התחלה מחדש, כמו בהתקנה חדשה.
accounts-delete-all-about = נמחקים מהמחשב הזה כל החשבונות, כל הדואר השמור, אנשי הקשר והיומנים, אינדקס החיפוש, ההגדרות והסיסמאות השמורות שלך. שום דבר לא משתנה בשרתי הדואר שלך.
accounts-delete-all-open = מחיקת כל הנתונים של Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } הוסר מ־Katna.
accounts-removed = { $address } הוסר מ־Katna. הדואר שלו עדיין בשרת.
accounts-all-deleted = כל הנתונים של Katna נמחקו מהמחשב הזה.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = להסיר את { $address }?
accounts-remove-confirm = הסרת החשבון
accounts-removing = מתבצעת הסרה…
accounts-remove-local-mail = { $folders ->
    [0] כל הדואר שיובא לחשבון הזה
    [one] כל הדואר שיובא לחשבון הזה, בתיקייה שלו
    [two] כל הדואר שיובא לחשבון הזה, ב־{ $folders } התיקיות שלו
   *[other] כל הדואר שיובא לחשבון הזה, ב־{ $folders } התיקיות שלו
}
accounts-remove-local-settings = ההגדרות שלו ב־Katna
accounts-remove-mail = { $folders ->
    [0] כל הדואר של החשבון הזה ש־Katna שומרת
    [one] כל הדואר של החשבון הזה ש־Katna שומרת, בתיקייה שלו
    [two] כל הדואר של החשבון הזה ש־Katna שומרת, ב־{ $folders } התיקיות שלו
   *[other] כל הדואר של החשבון הזה ש־Katna שומרת, ב־{ $folders } התיקיות שלו
}
accounts-remove-outbox = ההודעות שלו שממתינות בדואר היוצא
accounts-remove-settings = הסיסמה השמורה שלו וההגדרות שלו ב־Katna
accounts-delete-all-title = למחוק את כל הנתונים של Katna?
accounts-delete-all-confirm = מחיקת הכול
accounts-deleting = מתבצעת מחיקה…
accounts-delete-all-accounts = כל החשבונות, וכל הדואר והקבצים המצורפים ש־Katna שומרת
accounts-delete-all-contacts = אנשי קשר, יומנים ואינדקס החיפוש
accounts-delete-all-settings = כל ההגדרות, החתימות וקיצורי המקלדת
accounts-delete-all-passwords = כל הסיסמאות השמורות
accounts-deleted-heading = יימחקו מהמחשב הזה:
accounts-cannot-undo = אי אפשר לבטל את הפעולה הזו.
accounts-server-delete-all = שום דבר לא משתנה בשרתי הדואר שלך: הדואר נשאר שם, והוספה מחדש של חשבון מורידה אותו שוב. דואר שיובא מקבצים נמצא רק ב־Katna; הקבצים עצמם לא משתנים.
accounts-server-local = הדואר הזה יובא מקבצים, ולכן ל־Katna יש את העותק היחיד. הקבצים שמהם הוא הגיע לא משתנים; אפשר לייבא אותם שוב כדי לקבל אותו בחזרה.
accounts-server-remove = שום דבר לא משתנה בשרת הדואר: הדואר נשאר שם, והוספה מחדש של החשבון מורידה אותו שוב.
accounts-confirm-word = מחיקה
accounts-confirm-placeholder = יש להקליד „{ accounts-confirm-word }”
accounts-confirm-prompt = לאישור, יש להקליד „{ accounts-confirm-word }”:
accounts-cancel = ביטול
