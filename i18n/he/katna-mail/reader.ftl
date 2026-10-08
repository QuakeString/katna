# Katna Mail, Hebrew (עברית).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = סגירה
reader-back = חזרה
reader-mark-unread = סימון כלא נקראה
reader-move-to = העברה אל
reader-snooze = השהיה
reader-remind = להזכיר לי
reader-more = עוד
reader-original-colors = הצגת הצבעים המקוריים
reader-dark-colors = הצגה בצבעים כהים
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
reader-sending = מתבצעת שליחה…
reader-me = אני
reader-to = אל { $names }
reader-to-label = אל
reader-tick-delivered = נמסרה { $when }
reader-tick-no-bounce = נשלחה { $when }; לא חזרה הודעת כשל, כך שסביר שהגיעה
reader-tick-bounced = לא נמסרה: חזרה { $when }
reader-tick-read = נקראה { $when } (אישור קריאה)
reader-tick-opened = נפתחה, לאחרונה { $when } (מעקב פתיחות)
reader-starred = מסומנת בכוכב
reader-chip-remove = הסרת { $label }
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
reader-download-failed-reason = לא ניתן היה להוריד את ההודעה הזו. { $reason }
reader-download-offline = החשבון הזה במצב לא מקוון. כדי להוריד את ההודעה הזו יש לעבור למצב מקוון.
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
security-look-up-key = חיפוש המפתח
key-card-verified = חתימה מאומתת
key-card-verified-detail = החתימה תקינה, והמפתח הזה מהימן בעיניך.
key-card-unverified = החתימה לא אומתה
key-card-unverified-detail = החתימה תקינה, אבל שום דבר לא מאשר שהמפתח באמת שייך לשולח. כדאי להשוות איתו את טביעת האצבע, ואז לסמן את המפתח כמהימן ב־GnuPG (ב־Kleopatra או עם gpg --edit-key).
key-card-not-sender = נחתמה על ידי מישהו אחר
key-card-not-sender-detail = החתימה תקינה, אבל המפתח אינו של השולח.
key-card-untrusted = המפתח לא מהימן
key-card-untrusted-detail = סימנת את המפתח הזה כלא מהימן ב־GnuPG.
key-card-signature-expired = תוקף החתימה פג
key-card-signature-expired-detail = החתימה הייתה תקינה, אבל תוקפה פג.
key-card-key-expired = תוקף המפתח פג
key-card-key-expired-detail = החתימה תקינה, אבל תוקף המפתח פג מאז.
key-card-key-revoked = המפתח בוטל
key-card-key-revoked-detail = הבעלים של המפתח ביטל אותו, ולכן אי אפשר לסמוך על החתימה.
key-card-bad = חתימה פגומה
key-card-bad-detail = ההודעה הזו שונתה אחרי שנחתמה, או שהחתימה מזויפת.
key-card-signed-by = נחתמה על ידי
key-card-belongs-to = בעלים
key-card-fingerprint = טביעת אצבע
key-card-signed = נחתמה
key-card-key = מפתח
key-card-kind = { $standard }, { $algorithm }
key-card-created = נוצר
key-card-expires = תוקף עד
key-card-never = אף פעם
key-card-issued-by = הונפק על ידי
key-card-found-in = מיקום
key-card-keyring = מחזיק המפתחות שלך ב־GnuPG
key-card-copy = העתקת טביעת האצבע
key-card-import-title = לייבא את המפתח הזה?
key-card-from-directory = נמצא בספריית המפתחות של { $domain }.
key-card-from-attachment = מהקובץ המצורף { $name }.
key-card-import-note = כך Katna תוכל לבדוק את החתימות של האדם הזה ולהצפין אליו דואר. כדי לסמוך על המפתח באופן מלא, כדאי להשוות איתו את טביעת האצבע.
key-card-cancel = ביטול
key-card-import = ייבוא המפתח
key-card-looking-up = מחפשים את המפתח…
key-card-looking-up-detail = פונים לספריית המפתחות של { $domain }.
key-card-not-found = לא נמצא מפתח
key-card-not-found-detail = { $domain } לא מפרסם מפתח לכתובת הזו. אפשר לבקש מהשולח לשלוח לך את המפתח שלו.
key-card-not-kept = לא ניתן להשתמש במפתח שנמצא.
key-card-failed = לא ניתן היה לקבל את המפתח
sender-failed-title = ייתכן שזה לא באמת מ־{ $domain }
sender-failed-body = ההודעה לא עברה את בדיקות השולח של { $provider }. כדאי להיזהר עם קישורים, קבצים מצורפים ותשובות.
sender-provider-unknown = ספק הדואר שלך
sender-details = פרטים
sender-details-hide = הסתרת הפרטים
sender-looks-safe = נראה בטוח
sender-move-to-spam = העברה לספאם
sender-checked-by = נבדק על ידי { $provider }
sender-checked-by-server = נבדק על ידי { $provider } ({ $server })
sender-dmarc = דומיין השולח (DMARC)
sender-dkim = חתימה (DKIM)
sender-spf = שרת השליחה (SPF)
sender-result-pass = עבר
sender-result-fail = נכשל
sender-result-unsure = לא ברור
sender-result-none = אין
sender-result-missing = לא נבדק
sender-dmarc-pass = { $domain } מאשר את השולח הזה.
sender-dmarc-fail = ההודעה לא תואמת לאופן שבו { $domain } אומר שהדואר שלו נשלח.
sender-dmarc-none = { $domain } לא מפרסם כללים לדואר שלו.
sender-dkim-pass = נחתמה על ידי { $domain }.
sender-dkim-fail = החתימה של { $domain } לא תואמת להודעה.
sender-dkim-none = ההודעה לא נחתמה.
sender-spf-pass = נשלחה משרת ש־{ $domain } מציין ברשימה שלו.
sender-spf-fail = נשלחה משרת ש־{ $domain } לא מציין ברשימה שלו.
sender-spf-none = { $domain } לא מפרסם רשימה של השרתים שלו.
sender-check-unsure = הבדיקה לא הצליחה לתת תשובה ברורה.
sender-unconfirmed = { $provider } לא הצליח לאשר שההודעה הגיעה מ־{ $domain }. כל אחד יכול לכתוב כל שולח.
sender-link-title = לפתוח את הקישור הזה?
sender-link-body = ההודעה הזו לא עברה את בדיקות השולח. הקישור מוביל אל { $host }:
sender-link-cancel = ביטול
sender-link-open = פתיחה
tracking-opened = נפתחה אצל { $who } { $count ->
    [one] פעם אחת
    [two] פעמיים
   *[other] { $count } פעמים
}, לאחרונה { $when }
tracking-opens-clicks = נפתחה אצל { $who } { $opens ->
    [one] פעם אחת
    [two] פעמיים
   *[other] { $opens } פעמים
} וקישור בה נפתח { $clicks ->
    [one] פעם אחת
    [two] פעמיים
   *[other] { $clicks } פעמים
}, לאחרונה { $when }
tracking-clicked = קישור בה נפתח אצל { $who } { $clicks ->
    [one] פעם אחת
    [two] פעמיים
   *[other] { $clicks } פעמים
}, לאחרונה { $when }
tracking-maybe-opened = ייתכן שנפתחה אצל { $who } (Apple Mail טוען תמונות לשמירה על הפרטיות)
tracking-seen-none = עדיין אף אחד לא פתח אותה או לחץ על קישור בה
tracking-receipt = התקבל אישור קריאה מ־{ $who }
tracking-receipt-read = { $who } קרא/ה אותה (אישור קריאה), { $when }
tracking-receipt-displayed = אישור קריאה: ההודעה שלך נפתחה אצל { $who }
tracking-receipt-other = אישור קריאה: ההודעה שלך נמחקה או טופלה אצל { $who } בלי שנפתחה

## Remote images and pictures

remote-hidden = התמונות בהודעה הזו מוסתרות.
remote-hidden-unconfirmed = התמונות מוסתרות: לא ניתן היה לאמת את השולח.
remote-hidden-failed = התמונות מוסתרות: ההודעה הזו לא עברה את בדיקות השולח.
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
attachment-forward = העברה
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
print-preview-title = תצוגה לפני הדפסה
print-preview-laying-out = העמודים מסודרים…
print-preview-pages = { $count ->
    [one] עמוד אחד
    [two] { $count } עמודים
   *[other] { $count } עמודים
}
print-preview-more = { $count ->
    [one] ועוד עמוד אחד
    [two] ועוד { $count } עמודים
   *[other] ועוד { $count } עמודים
}
print-preview-failed = לא ניתן היה להציג את העמודים
print-preview-paper = נייר
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-layout = פריסה
print-preview-as-shown = כמו שמוצג
print-preview-simple = טקסט פשוט
print-preview-backgrounds = רקעים
print-preview-cancel = ביטול
print-preview-print = הדפסה
print-not-downloaded = (עדיין לא הורדה.)
print-encrypted = (מוצפנת. יש לפתוח אותה ב־Katna Mail כדי להדפיס את הטקסט שלה.)
print-to = אל: { $addresses }
print-cc = עותק: { $addresses }
text-pin = הצמדה למעלה
text-copy-address = העתקת הכתובת

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = יש לפתוח את ההודעה הזו כדי לקרוא את הקבצים המצורפים שלה.
text-copy = העתקה
text-select-all = בחירת הכול
