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
compose-show-trimmed = הצגת התוכן שקוצץ

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
compose-track-unavailable = אי אפשר לעקוב אחר דואר חתום, מוצפן או בטקסט פשוט
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
