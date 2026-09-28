# Katna Mail, Hebrew (עברית).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = הודעה חדשה
compose-restore = שחזור
compose-minimize = מזעור
compose-exit-full-screen = יציאה ממסך מלא
compose-open-window = פתיחה בחלון חדש
compose-save-close = שמירה וסגירה
compose-back-to-mail = חזרה לחלון הדואר
compose-pop-out-reply = פתיחת התשובה בחלון נפרד
compose-edit-recipients = עריכת הנמענים
compose-summary-cc = עותק: { $names }
compose-summary-bcc = עותק מוסתר: { $names }
compose-more-recipients = { $count } נוספים
compose-show-trimmed = הצגת התוכן שקוצץ
compose-hide-trimmed = הסתרת התוכן שקוצץ
compose-remove-trimmed = הסרת הטקסט המצוטט
compose-trimmed-removed = הטקסט המצוטט הוסר

## Recipients and subject

compose-to = אל
compose-cc = עותק
compose-bcc = עותק מוסתר
compose-from = מאת
compose-from-choose = שליחה מחשבון אחר
compose-recipients = נמענים
compose-subject = נושא

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = קודם יש לשלוח את ההודעה הפתוחה או למחוק אותה.
compose-bad-address = „{ $address }” אינה כתובת אימייל.
compose-no-recipients = יש להוסיף לפחות נמען אחד.
compose-attachments-too-large = גודל הקבצים המצורפים הוא { $size }; שרתי דואר מקבלים עד { $limit }.
compose-no-account = יש להוסיף חשבון שממנו תישלח הודעת הדואר.
compose-past-time = יש לבחור זמן עתידי.
compose-scheduling = השליחה מתוזמנת…
compose-sending = ההודעה נשלחת…
compose-scheduled = השליחה תוזמנה ל־{ $when }
compose-sent-archived = נשלחה והועברה לארכיון
compose-sent = ההודעה נשלחה
compose-discarded = הטיוטה נמחקה
compose-draft-saved = הטיוטה נשמרה
compose-draft-failed = לא ניתן לשמור את הטיוטה: { $error }
compose-draft-not-opened = לא ניתן לפתוח את הטיוטה.

## Attachments

compose-picker-insert = הוספה
compose-picker-attach = צירוף
compose-file-too-large = { $name } גדול מדי: הודעה יכולה להכיל עד { $limit }.
compose-attachment-size = ({ $size })
compose-remove-attachment = הסרת הקובץ המצורף
compose-attachments-total = { $count ->
    [one] קובץ אחד, { $size }
   *[other] { $count } קבצים, { $size }
}
compose-drive-note = { $name } גדול מ-{ $limit }, ולכן הוא עובר ל-Google Drive שלך וההודעה כוללת קישור.
compose-drive-tip = ב-Google Drive שלך; ההודעה כוללת קישור
compose-drive-uploading = מעלה { $percent }%
compose-drive-allow = אישור גישה ל-Drive
compose-drive-allow-tip = אפשר להיכנס שוב באמצעות Google כדי לאפשר ל-Katna לשמור קבצים גדולים ב-Drive שלך
compose-drive-retry = נסו שוב
compose-drive-sends-when-uploaded = ההודעה תישלח לאחר שהקובץ { $name } יועלה
compose-drive-not-uploaded = { $name } עדיין לא נמצא ב-Google Drive
compose-drive-share-failed = לא ניתן היה לשתף את הקבצים ב-Google Drive: { $error }
compose-drive-share-title = לשתף את הקבצים עם כולם?
compose-drive-share-text = { $count ->
    [one] Google Drive לא יכול לשתף את הקבצים עם { $addresses }, שאין לו חשבון Google. במקום זאת, כל מי שיש לו את הקישור יכול לפתוח אותם.
   *[other] Google Drive לא יכול לשתף את הקבצים עם { $addresses }, שאין להם חשבון Google. במקום זאת, כל מי שיש לו את הקישור יכול לפתוח אותם.
}
compose-drive-share-link = שיתוף באמצעות קישור
compose-drive-send-without = שליחה בלי שיתוף
compose-drive-share-cancel = ביטול
compose-drive-card-detail = { $size } · Google Drive
compose-onedrive-note = { $name } גדול מ-{ $limit }, ולכן הוא עובר ל-OneDrive שלך וההודעה כוללת קישור.
compose-onedrive-tip = ב-OneDrive שלך; ההודעה כוללת קישור
compose-onedrive-allow = אישור גישה ל-OneDrive
compose-onedrive-allow-tip = אפשר להיכנס שוב באמצעות Microsoft כדי לאפשר ל-Katna לשמור קבצים גדולים ב-OneDrive שלך
compose-onedrive-not-uploaded = { $name } עדיין לא נמצא ב-OneDrive
compose-onedrive-share-failed = לא ניתן היה לשתף את הקבצים ב-OneDrive: { $error }
compose-onedrive-share-text = { $count ->
    [one] OneDrive לא יכול לשתף את הקבצים עם { $addresses }. במקום זאת, כל מי שיש לו את הקישור יכול לפתוח אותם.
   *[other] OneDrive לא יכול לשתף את הקבצים עם { $addresses }. במקום זאת, כל מי שיש לו את הקישור יכול לפתוח אותם.
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-drop-files = אפשר לשחרר קבצים כאן
compose-drop-here = אפשר לשחרר כאן
compose-paste-keep-formatting = שמירת העיצוב
compose-paste-table = טבלה
compose-paste-picture = תמונה
compose-paste-plain-text = טקסט פשוט
compose-paste-inline = בתוך הטקסט
compose-paste-attachment = קובץ מצורף

## Encryption and signing (the toggles by the recipients)

compose-encrypt = הצפנה
compose-encrypted = מוצפנת: רק הנמענים יכולים לקרוא אותה
compose-sign = חתימה
compose-signed = חתומה: הנמענים יכולים לוודא שהיא ממך
compose-track = מעקב אחר פתיחות ולחיצות
compose-tracked = במעקב: יוצג מתי כל נמען פותח את ההודעה או לוחץ על קישור
compose-track-clicks = מעקב אחר לחיצות על קישורים (טקסט פשוט לא יכול להראות פתיחות)
compose-tracked-clicks = במעקב: יוצג מתי כל נמען לוחץ על קישור
compose-track-sign-in = כדי לעקוב אחר פתיחות ולחיצות צריך להתחבר לחשבון Katna
compose-receipt = בקשת אישור קריאה
compose-receipt-on = התבקש אישור קריאה: ייתכן שהיישום של הנמען יבקש ממנו לשלוח אותו
compose-delivery = בקשת אישור מסירה
compose-delivery-on = התבקש אישור מסירה: שרת הדואר שלך ישלח לך הודעה כשהשרת של כל נמען יקבל אותה
compose-delivery-unavailable = שרת הדואר שלך לא שולח אישורי מסירה

## Spelling

spell-no-dictionary = לא מותקן מילון איות עבור { $language } (לדוגמה hunspell-en_us).
spell-dictionary-error = מילון האיות: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = „{ $words }”
grammar-add = הוספת „{ $words }”
grammar-remove = הסרת „{ $words }”
grammar-ignore = התעלמות

## Send checks (asked before a message goes out)

send-check-attachment-title = התכוונת לצרף קבצים?
send-check-attachment-text = כתבת על קובץ מצורף, אבל לא צורף דבר.
send-check-attach = צירוף קובץ
send-check-subject-title = לשלוח בלי נושא?
send-check-subject-text = להודעה הזו אין נושא.
send-check-add-subject = הוספת נושא
send-check-send-anyway = לשלוח בכל זאת
recipient-not-valid = זו אינה כתובת אימייל תקינה
recipient-show-address = הצגת הכתובת
recipient-remove = הסרה
recipient-bad-title = בדיקת הכתובת
recipient-bad-text = „{ $address }” אינה כתובת אימייל תקינה. יש לתקן או להסיר אותה לפני השליחה.
recipient-bad-fix = תיקון
