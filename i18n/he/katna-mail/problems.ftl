# Katna Mail, Hebrew (עברית).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = שרת הדואר
problems-signed-out = { $provider } ניתק את Katna מ־{ $address }. סנכרון הדואר נעצר.
problems-password-refused = { $provider } דחה את הסיסמה של { $address }. ייתכן שהיא השתנתה.
problems-no-answer = { $provider } לא עונה עבור { $address }. Katna ממשיכה לנסות.
problems-offline = אין חיבור. הדואר שלך עדיין כאן, ודואר שנשלח ממתין עד שהחיבור יחזור.
problems-accounts-need-you = { $count ->
    [one] חשבון אחד צריך את תשומת לבך
    [two] { $count } חשבונות צריכים את תשומת לבך
   *[other] { $count } חשבונות צריכים את תשומת לבך
}
problems-show = הצגה
problems-later = מאוחר יותר
problems-new-password = סיסמה חדשה
problems-try-again = ניסיון נוסף

## The New password card

problems-password-title = סיסמה חדשה
problems-password-detail = { $provider } דחה את הסיסמה השמורה של { $address }. יש להקליד את החדשה; Katna בודקת אותה לפני שהיא נשמרת.
problems-password-placeholder = סיסמה
problems-password-show = הצגת הסיסמה
problems-password-hide = הסתרת הסיסמה
problems-password-cancel = ביטול
problems-password-save = שמירה
problems-password-checking = בודקים…
problems-password-refused-again = { $provider } דחה גם את הסיסמה הזו. יש לבדוק אותה ולנסות שוב.
problems-password-saved = הסיסמה של { $address } נשמרה. מביאים את הדואר שלך…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = שרת הדואר של { $address } לא אישר העברה של { $count ->
    [one] הודעה, ולכן היא חזרה למקומה.
    [two] { $count } הודעות, ולכן הן חזרו למקומן.
   *[other] { $count } הודעות, ולכן הן חזרו למקומן.
}
problems-refused-flags = שרת הדואר של { $address } לא אישר סימון של { $count ->
    [one] הודעה (נקראה, כוכב…), ולכן היא חזרה למצבה הקודם.
    [two] { $count } הודעות (נקראו, כוכב…), ולכן הן חזרו למצבן הקודם.
   *[other] { $count } הודעות (נקראו, כוכב…), ולכן הן חזרו למצבן הקודם.
}
problems-refused-label = שרת הדואר של { $address } לא אישר שינוי של התוויות של { $count ->
    [one] הודעה, ולכן היא חזרה למצבה הקודם.
    [two] { $count } הודעות, ולכן הן חזרו למצבן הקודם.
   *[other] { $count } הודעות, ולכן הן חזרו למצבן הקודם.
}
problems-refused-delete = שרת הדואר של { $address } לא אישר מחיקה של { $count ->
    [one] הודעה, ולכן היא חזרה.
    [two] { $count } הודעות, ולכן הן חזרו.
   *[other] { $count } הודעות, ולכן הן חזרו.
}
problems-refused-other = שרת הדואר של { $address } לא אישר { $count ->
    [one] שינוי, ולכן Katna החזירה אותו למצבו הקודם.
    [two] { $count } שינויים, ולכן Katna החזירה אותם למצבם הקודם.
   *[other] { $count } שינויים, ולכן Katna החזירה אותם למצבם הקודם.
}
problems-details = פרטים

## Katna's background service (katna-daemon) isn't running

service-starting = מפעילים את שירות הרקע של Katna…
service-failed = שירות הרקע של Katna לא מצליח לעלות, ולכן הדואר לא מסתנכרן.
service-start-again = הפעלה מחדש
service-started-again = שירות הרקע של Katna נעצר והופעל מחדש.
service-details-title = למה השירות לא עולה
service-details-body = אפשר להעתיק את זה ולשלוח עם הדיווח. אין בו דואר או סיסמאות.
service-details-copy = העתקה
service-details-close = סגירה
service-not-running = שירות הרקע של Katna לא פועל.
service-no-answer = שירות הרקע של Katna לא ענה: { $error }
service-no-session = אין הפעלת D-Bus: { $error }
safe-line = Katna נמצאת במצב בטוח אחרי בעיה בעדכון, ולכן הדואר לא מסתנכרן.
safe-try-again = ניסיון נוסף
safe-restore = שחזור
safe-restoring = הנתונים שלך משוחזרים מ־{ $when }…
safe-restored = הנתונים שלך שוחזרו מ־{ $when }. מה שהיה שם קודם נשמר בתיקייה.
safe-show-folder = הצגת התיקייה
safe-restore-failed = לא ניתן היה לשחזר את הנתונים שלך: { $error }
safe-restore-title = לשחזר את הנתונים שלך ממה שהיה לפני עדכון?
safe-restore-body = Katna חוזרת לעותק שבחרת. דואר שהגיע אחריו יורד שוב מהחשבונות שלך.
safe-restore-none = עדיין אין עותקים. Katna יוצרת עותק לפני כל עדכון שמשנה את הנתונים שלך.
safe-restore-keep = מה שיש עכשיו, כולל דואר שלא נשלח, טיוטות ושינויים שעדיין לא סונכרנו, נשמר קודם בתיקייה, כך ששום דבר לא הולך לאיבוד.
safe-restore-cancel = ביטול
safe-restore-mail = דואר
safe-restore-pim = חשבונות ואנשי קשר
safe-restore-blobs = קבצים מצורפים
safe-report-title = דוח ניפוי באגים
safe-report-body = אפשר להעתיק את זה ולצרף לדיווח על הבאג. אין בו דואר, כתובות או סיסמאות.
safe-report-restore = שחזור…
safe-report-copied = דוח ניפוי הבאגים הועתק
