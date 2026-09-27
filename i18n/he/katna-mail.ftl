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
