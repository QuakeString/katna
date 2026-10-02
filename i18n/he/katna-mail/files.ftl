# Katna Mail, Hebrew (עברית).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = חיפוש קבצים

## Left side (and chips on a phone)

files-all = כל הקבצים
files-pictures = תמונות
files-pdfs = קובצי PDF
files-documents = מסמכים
files-sheets = גיליונות אלקטרוניים
files-slides = מצגות
files-other = אחר
files-accounts = חשבונות
files-drives = כוננים
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = משותף איתי
files-shown = מוצגים
files-received = התקבלו
files-sent = נשלחו על ידיי

## Over the files

files-count = { $count ->
    [one] קובץ אחד · { $size }
    [two] { $count } קבצים · { $size }
   *[other] { $count } קבצים · { $size }
}
files-anyone = כל אחד
files-from-person = מ־{ $name }
files-time-any = כל זמן
files-time-today = היום
files-time-yesterday = אתמול
files-time-this-week = השבוע
files-time-last-week = בשבוע שעבר
files-time-this-month = החודש
files-time-last-month = בחודש שעבר
files-time-between = { $first } – { $last }
files-time-hint = לחיצה על יום, או גרירה על פני כמה ימים
files-time-summary = { $count ->
    [one] { $days } · קובץ אחד
    [two] { $days } · { $count } קבצים
   *[other] { $days } · { $count } קבצים
}
files-time-clear = ניקוי
files-time-month-back = החודש הקודם
files-time-month-on = החודש הבא
files-time-wheel = גלילה מזיזה את התאריכים האלה ושומרת על האורך שלהם
files-sort-newest = החדשים ביותר קודם
files-sort-oldest = הישנים ביותר קודם
files-sort-largest = הגדולים ביותר קודם
files-sort-name = לפי שם
files-grid = כרטיסים
files-list = רשימה
files-this-week = השבוע
files-undated = ללא תאריך
files-me = אני
files-no-subject = (ללא נושא)
files-loading = אוספים קבצים מהדואר שלך…
files-empty = קבצים מהדואר שלך יופיעו כאן.
files-none-match = אין קבצים תואמים.
files-load-failed = קריאת הקבצים נכשלה: { $error }

## A file's menu and buttons

files-open = פתיחה
files-open-with = פתיחה באמצעות…
files-save = שמירה…
files-show-mail = הצגת ההודעה
files-mail-window = פתיחת ההודעה בחלון חדש
files-forward = העברת הקובץ
files-from-them = קבצים מ־{ $name }
files-copy-name = העתקת שם הקובץ
files-name-copied = שם הקובץ הועתק
files-downloading = מורידים את ההודעה…
files-download-failed = לא ניתן היה להוריד את ההודעה הזו.

## A cloud drive in place of the mail files

files-drive-mine = האחסון שלי
files-drive-mine-onedrive = הקבצים שלי
files-drive-results = „{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] קובץ אחד
        [two] { $files } קבצים
       *[other] { $files } קבצים
    }
    [one] תיקייה אחת · { $files ->
        [one] קובץ אחד
        [two] { $files } קבצים
       *[other] { $files } קבצים
    }
    [two] { $folders } תיקיות · { $files ->
        [one] קובץ אחד
        [two] { $files } קבצים
       *[other] { $files } קבצים
    }
   *[other] { $folders } תיקיות · { $files ->
        [one] קובץ אחד
        [two] { $files } קבצים
       *[other] { $files } קבצים
    }
}
files-drive-folders = תיקיות
files-drive-files = קבצים
files-drive-folder = תיקייה
files-drive-meta = { $what } · נערך { $date }
files-drive-as-link = { $what } · כקישור
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = מביאים…
files-drive-loading = פותחים את הכונן…
files-drive-empty = התיקייה הזו ריקה.
files-drive-unreachable = אין גישה אל { $drive }.
files-drive-try-again = ניסיון נוסף
files-drive-needs-permission = כדי להציג את הכונן הזה, Katna צריכה את ההרשאה שלך פעם אחת. יש להתחבר שוב ולאפשר ל־Katna לראות את הקבצים שלך.
files-drive-allow = מתן הרשאה
files-drive-allow-failed = ההתחברות לא הושלמה, ולכן הכונן נשאר סגור.
files-drive-attach = צירוף
files-drive-more = עוד
files-drive-download = הורדה…
files-drive-open-web = פתיחה ב־{ $drive }
files-drive-copy-link = העתקת הקישור
files-drive-link-copied = הקישור הועתק
files-drive-share = שיתוף…
files-drive-rename = שינוי שם
files-drive-trash = העברה לאשפה
files-drive-trashed = „{ $name }” נמצא באשפה של { $drive }
files-drive-renamed = השם שונה ל„{ $name }”
files-drive-getting = מביאים את { $name } מ־{ $drive }…
files-drive-get-failed = לא ניתן היה להביא את { $name }: { $error }
files-drive-upload = העלאה
files-drive-upload-files = העלאת קבצים
files-drive-upload-folder = העלאת תיקייה
files-drive-upload-failed = לא ניתן היה להעלות את { $name }: { $error }
files-drive-upload-needs = כדי להעלות, Katna צריכה את ההרשאה שלך פעם אחת: יש ללחוץ על „מתן הרשאה” ב„הגדרות” › „אפליקציות ברירת מחדל” › „דף הקבצים”.

## The Share dialog of a drive file or folder

files-share-title = שיתוף „{ $name }”
files-share-add = הוספת אנשים לפי שם או כתובת
files-share-not-address = „{ $text }” אינו כתובת אימייל
files-share-notify = לאפשר ל־{ $drive } לשלוח להם גם אימייל
files-share-people = אנשים עם גישה
files-share-general = גישה כללית
files-share-loading = בודקים למי יש גישה…
files-share-restricted = מוגבלת
files-share-restricted-about = רק אנשים עם גישה יכולים לפתוח באמצעות הקישור
files-share-anyone = כל מי שיש לו את הקישור
files-share-anyone-can = { $role ->
    [editor] כל מי שיש לו את הקישור יכול לערוך
    [commenter] כל מי שיש לו את הקישור יכול להגיב
   *[viewer] כל מי שיש לו את הקישור יכול לצפות
}
files-share-anyone-about = { $role ->
    [editor] כל אחד באינטרנט שיש לו את הקישור יכול לערוך
    [commenter] כל אחד באינטרנט שיש לו את הקישור יכול להגיב
   *[viewer] כל אחד באינטרנט שיש לו את הקישור יכול לצפות
}
files-share-role-owner = בעלים
files-share-role-editor = עורך
files-share-role-commenter = מגיב
files-share-role-viewer = צופה
files-share-you = { $name } (אני)
files-share-domain = כל מי שב־{ $domain }
files-share-inherited = גישה מתיקייה שהוא נמצא בה
files-share-remove = הסרת הגישה
files-share-copy-link = העתקת הקישור
files-share-share = שיתוף
files-share-done = סיום
files-share-sharing = משתפים…
files-share-shared = { $count ->
    [one] שותף עם אדם אחד
    [two] שותף עם { $count } אנשים
   *[other] שותף עם { $count } אנשים
}
files-share-refused = { $drive } לא הצליח לשתף עם { $addresses }
files-share-failed = לא ניתן היה לשנות את השיתוף: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] מעלים פריט אחד
    [two] מעלים { $count } פריטים
   *[other] מעלים { $count } פריטים
}
files-tray-done = { $count ->
    [one] העלאה אחת הושלמה
    [two] { $count } העלאות הושלמו
   *[other] { $count } העלאות הושלמו
}
files-tray-some-failed = { $done } הועלו, { $failed } נכשלו
files-tray-minutes-left = { $minutes ->
    [one] נותרה בערך דקה
    [two] נותרו בערך { $minutes } דקות
   *[other] נותרו בערך { $minutes } דקות
}
files-tray-seconds-left = נותרה פחות מדקה
files-tray-starting = מתחילים…
files-tray-cancel-all = ביטול הכול
files-tray-cancel = ביטול
files-tray-fold = הסתרת הרשימה
files-tray-unfold = הצגת הרשימה
files-tray-close = סגירה
files-tray-progress = { $place } · { $sent } מתוך { $size }
files-tray-in = ב־{ $place }
files-tray-cancelled = בוטל
