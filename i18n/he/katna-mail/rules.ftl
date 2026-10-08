# Katna Mail, Hebrew (עברית).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = כללים
settings-rules-summary = מיון, תיוג, העברה או השתקה של דואר חדש באופן אוטומטי
settings-rules-intro = כללים ממיינים דואר חדש באופן אוטומטי, לפי הסדר הזה. אפשר לגרור כדי לשנות את הסדר.
settings-rules-all-accounts = כל החשבונות
settings-rules-new = כלל חדש
settings-rules-none = עדיין אין כללים. כלל ממיין דואר חדש באופן אוטומטי: לפי שולח, נושא או מילים.
settings-rules-none-account = עדיין אין כללים לחשבון הזה.
settings-rules-drag = גרירה לשינוי הסדר
settings-rules-edit = עריכת הכלל
settings-rules-turn-off = כיבוי הכלל הזה
settings-rules-turn-on = הפעלת הכלל הזה

## Starter rules: offered under the user's own rules, switched off.

## Turning one on makes it one of the user's rules.

settings-rules-starters = כללים מוכנים
settings-rules-starters-intro = כבויים עד שמפעילים אותם. הם עובדים בכל החשבונות שלך; כדי לשנות אחד יש לערוך אותו.
settings-rules-starter-turning-on = מפעילים את „{ $name }”…
settings-rules-starter-failed = לא ניתן היה להפעיל את „{ $name }”: { $error }
rules-starter-promotions = השתקת קידומי מכירות
rules-starter-newsletters = ניוזלטרים לקריאה
rules-starter-receipts = קבלות וחשבוניות
rules-starter-deliveries = משלוחים
rules-starter-train = כרטיסי רכבת
rules-starter-flight = כרטיסי טיסה
rules-starter-codes = קודים חד־פעמיים
rules-starter-security = התראות אבטחה
rules-starter-social = דואר מרשתות חברתיות
rules-starter-invites = הזמנות ליומן
rules-starter-folder-reading = לקריאה
rules-starter-folder-receipts = קבלות
rules-starter-folder-deliveries = משלוחים
rules-starter-folder-travel = נסיעות
rules-starter-folder-social = רשתות חברתיות
rules-runs-katna = פועל ב־Katna
rules-runs-gmail = פועל ב־Gmail
rules-runs-sieve = פועל בשרת
rules-stopped = נעצר
rules-error-folder-gone = התיקייה שהכלל הזה משתמש בה כבר לא קיימת. יש לערוך את הכלל ולבחור תיקייה אחרת.
rules-error-no-archive = לחשבון הזה אין תיקיית ארכיון. יש לערוך את הכלל כך שיעשה משהו אחר.
rules-error-no-trash = לחשבון הזה אין תיקיית אשפה. יש לערוך את הכלל כך שיעשה משהו אחר.
rules-error-cannot-send = החשבון הזה לא יכול לשלוח דואר, ולכן הכלל לא יכול להעביר אותו.
rules-error-other = { $error }. יש לערוך את הכלל ולהפעיל אותו שוב.
settings-folders = תיקיות
settings-folders-summary = מספר ההודעות שלא נקראו בחלונית התיקיות
settings-folders-unread-counts = מספר ההודעות שלא נקראו בכל תיקייה
settings-folders-unread-counts-detail = כבוי: רק הדואר הנכנס מציג כמה הודעות לא נקראו

## A rule in one line, on its row: "From contains substack.com → skip the

## inbox, label Reading".

rules-summary = { $when } ← { $then }
rules-summary-and = { $first } וגם { $next }
rules-summary-or = { $first } או { $next }
rules-summary-more = עוד { $count }
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = יש קובץ מצורף
rules-summary-no-attachment = אין קובץ מצורף
rules-summary-mailing-list = מרשימת תפוצה
rules-summary-not-mailing-list = לא מרשימת תפוצה
rules-summary-tab = בכרטיסייה { $tab }
rules-summary-not-tab = לא בכרטיסייה { $tab }
rules-summary-move = העברה אל { $folder }
rules-summary-archive = דילוג על הדואר הנכנס
rules-summary-trash = העברה לאשפה
rules-summary-mark-read = סימון כנקראה
rules-summary-star = סימון בכוכב
rules-summary-important = סימון כחשובה
rules-summary-label = תיוג { $label }
rules-summary-forward = העברה אל { $address }
rules-summary-dont-notify = בלי התראה
rules-summary-read-after = { $count ->
    [one] סימון כנקראה אחרי יום אחד
    [two] סימון כנקראה אחרי יומיים
   *[other] סימון כנקראה אחרי { $count } ימים
}
rules-summary-folder-gone = תיקייה שכבר לא קיימת

## The rule editor

rules-editor-new-title = כלל חדש
rules-editor-edit-title = עריכת הכלל
rules-editor-name-hint = שם הכלל
rules-editor-when = כשדואר חדש תואם ל
rules-editor-of-these = מהתנאים האלה:
rules-mode-all = כל
rules-mode-any = אחד
rules-field-from = מאת
rules-field-to = אל
rules-field-cc = עותק
rules-field-any-recipient = אל או עותק
rules-field-reply-to = תשובה אל
rules-field-subject = נושא
rules-field-body = טקסט
rules-field-attachment-name = שם הקובץ המצורף
rules-field-has-attachment = יש קובץ מצורף
rules-field-mailing-list = מרשימת תפוצה
rules-field-tab = כרטיסייה בדואר הנכנס
rules-comparator-contains = מכיל
rules-comparator-not-contains = לא מכיל
rules-comparator-begins-with = מתחיל ב
rules-comparator-ends-with = מסתיים ב
rules-comparator-equals = שווה בדיוק ל
rules-comparator-matches = תואם לתבנית
rules-has-yes = כן
rules-has-no = לא
rules-editor-value-hint = מילים או כתובת
rules-editor-add-condition = הוספת תנאי
rules-editor-remove = הסרה
rules-editor-then = אז:
rules-action-move = העברה אל
rules-action-archive = דילוג על הדואר הנכנס (ארכיון)
rules-action-trash = העברה לאשפה
rules-action-mark-read = סימון כנקראה
rules-action-star = סימון בכוכב
rules-action-important = סימון כחשובה
rules-action-label = הוספת תווית
rules-action-forward = העברה אל
rules-action-dont-notify = בלי התראה
rules-action-read-after = סימון כנקראה אחרי
rules-editor-choose-folder = בחירת תיקייה
rules-editor-choose-label = בחירת תווית
rules-editor-new-folder = חדשה: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = כתובת אימייל
rules-editor-days = ימים
rules-editor-add-action = הוספת פעולה
rules-editor-stop = לעצור כאן: כללים מאוחרים יותר לא יפעלו על הדואר הזה
rules-editor-accounts = חשבונות:
rules-editor-accounts-none = בחירת חשבונות
rules-editor-accounts-many = { $count ->
    [one] חשבון אחד
    [two] { $count } חשבונות
   *[other] { $count } חשבונות
}
rules-editor-matches = תואם ל־{ $mails } מ־{ $days } הימים האחרונים
rules-editor-mails = { $count ->
    [one] הודעה אחת
    [two] { $count } הודעות
   *[other] { $count } הודעות
}
rules-editor-counting = סופרים את הדואר שתואם…
rules-editor-show = הצגה
rules-editor-also-apply = להחיל גם על { $count } אלה
rules-editor-runs-katna = פועל ב־Katna, כל עוד המחשב הזה דולק.
rules-editor-runs-gmail = פועל ב־Gmail, כך שהוא עובד גם בטלפון וגם כשהמחשב הזה כבוי.
rules-editor-runs-sieve = פועל בשרת הדואר שלך, כך שהוא עובד גם בטלפון וגם כשהמחשב הזה כבוי.
rules-note-gmail-action = פועל ב־Katna: המסננים של Gmail לא יכולים לבצע „{ $action }”.
rules-note-sieve-action = פועל ב־Katna: הכללים של שרת הדואר שלך לא יכולים לבצע „{ $action }”.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = פועל ב־Katna: המסננים של Gmail לא יכולים לבדוק „{ $test }” כמו ש־Katna בודקת.
rules-note-sieve-condition = פועל ב־Katna: הכללים של שרת הדואר שלך לא יכולים לבדוק „{ $test }” כמו ש־Katna בודקת.
rules-note-order = פועל ב־Katna, כמו כלל קודם של החשבון: הכללים פועלים לפי סדר הרשימה.
rules-note-gmail-stop = פועל ב־Katna: המסננים של Gmail לא יכולים למנוע מכללים מאוחרים יותר לפעול.
rules-note-gmail-forward = פועל ב־Katna: Gmail מעביר רק לכתובות שאומתו בהגדרות שלו, ו־{ $address } היא לא אחת מהן.
rules-note-gmail-folder = פועל ב־Katna: ב־Gmail אין תווית עבור תיקייה שהכלל הזה משתמש בה.
rules-note-sieve-folder = פועל ב־Katna: בשרת הדואר שלך אין תיקייה שהכלל הזה משתמש בה.
rules-note-gmail-sign-in = פועל ב־Katna עד שתתחברו שוב ל־Google ותאפשרו ל־Katna ליצור מסננים ב־Gmail.
rules-note-sieve-other-script = פועל ב־Katna: סקריפט כללים אחר („{ $name }”) פעיל בשרת הדואר שלך.
rules-note-gmail-failed = פועל ב־Katna: Gmail לא קיבל אותו ({ $error }).
rules-note-sieve-failed = פועל ב־Katna: שרת הדואר שלך לא קיבל אותו ({ $error }).
rules-editor-cancel = ביטול
rules-editor-save = שמירה
rules-editor-saving = שומרים…
rules-editor-delete = מחיקת הכלל
rules-editor-delete-ask = למחוק את הכלל הזה?
rules-editor-delete-keep = להשאיר
rules-editor-delete-confirm = מחיקה
rules-editor-needs-folder = יש לבחור תיקייה לכל „העברה אל” ותווית לכל „הוספת תווית”.
rules-editor-needs-days = „סימון כנקראה אחרי” מקבל מספר ימים, מ־1 עד 3650.
rules-saved = הכלל נשמר
rules-saved-applied = { $count ->
    [one] הכלל נשמר והוחל על הודעה אחת
    [two] הכלל נשמר והוחל על { $count } הודעות
   *[other] הכלל נשמר והוחל על { $count } הודעות
}
rules-apply-failed = הכלל נשמר, אבל ההחלה שלו נכשלה: { $error }
rules-deleted = הכלל נמחק
rules-delete-failed = לא ניתן היה למחוק את הכלל: { $error }
rules-change-failed = לא ניתן היה לשנות את הכללים: { $error }
