# Katna Mail, Hebrew (עברית): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = אנשי קשר
contacts-frequent = תדירים
contacts-other = אנשי קשר אחרים
contacts-other-about = אנשים ששלחת להם מייל מ-Gmail אבל לא שמרת
contacts-other-email = שליחת אימייל
contacts-other-empty = אין אנשי קשר אחרים. אנשים ששלחת להם מייל מ-Gmail ולא שמרת יופיעו כאן.
contacts-other-allow = כדי לראות אנשי קשר אחרים, יש להיכנס שוב לחשבון Gmail ולאפשר ל-Katna לראות אותם.
contacts-labels = תוויות
contacts-label-options = אפשרויות תווית
contacts-label-rename = שינוי שם התווית
contacts-label-email = שליחת אימייל לכולם
contacts-label-delete = מחיקת התווית
contacts-label-new = תווית חדשה
contacts-label-name = שם התווית
contacts-label-button = תווית
contacts-label-menu = הוספת תווית:
contacts-label-added = נוסף אל { $name }
contacts-label-removed = הוסר מ-{ $name }
contacts-label-renamed = שם התווית שונה ל-{ $name }
contacts-label-deleted = התווית { $name } נמחקה
contacts-label-no-email = לאף אחד בתווית הזו אין כתובת אימייל
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = חשבונות
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = יש להתחבר שוב כדי להציג אנשי קשר
contacts-account-signed-in = התחברת שוב אל { $address }. אנשי הקשר שלך נטענים…
contacts-account-sign-in-refused = { $provider } לא הכניס את Katna. יש לנסות שוב ולאשר גישה לאנשי הקשר שלך.
contacts-account-password = השרת לא קיבל את הסיסמה. Yahoo, iCloud, Zoho ואחרים דורשים סיסמה לאפליקציה.
contacts-account-change-password = שינוי סיסמה
contacts-account-change-password-tooltip = פתיחת הגדרות > חשבונות
contacts-account-failed = לא ניתן היה לקרוא את אנשי הקשר.
# $reason is the server's own words, in English.
contacts-account-error = לא ניתן היה לקרוא את אנשי הקשר: { $reason }
contacts-account-none = לא נמצא פנקס כתובות
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = לא נמצא פנקס כתובות: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider } מציג אנשי קשר רק ל-Katna שמחובר עם { $provider }.
contacts-account-sign-in-with = התחברות עם { $provider }
contacts-account-looking = מתבצע חיפוש אנשי קשר…
contacts-account-try-again = ניסיון נוסף
contacts-account-try-again-tooltip = בדיקה חוזרת של אנשי הקשר בחשבון הזה עכשיו
contacts-account-fixing = מטפלים בזה…
contacts-manage = תיקון וניהול
contacts-merge = מיזוג ותיקון
contacts-merge-about = { $count ->
   *[other] { $count } הצעות: אנשי קשר שנראים כמו אותו אדם
}
contacts-merge-none = אין כפילויות. אנשי קשר עם אותו שם או מספר טלפון יופיעו כאן.
contacts-merge-count = { $count ->
    [one] { $count } איש קשר
    [two] { $count } אנשי קשר
   *[other] { $count } אנשי קשר
}
contacts-merge-all = מיזוג הכול
contacts-merge-button = מיזוג
contacts-merge-dismiss = התעלמות
contacts-merged = { $count ->
    [1] אנשי הקשר מוזגו
    [one] בוצע מיזוג { $count }
    [two] בוצעו { $count } מיזוגים
   *[other] בוצעו { $count } מיזוגים
}
contacts-import = ייבוא
contacts-export = ייצוא
contacts-import-file = ייבוא אנשי קשר מקובץ vCard או CSV
contacts-imported = { $count ->
    [one] יובא { $count } איש קשר אל { $place }
    [two] יובאו { $count } אנשי קשר אל { $place }
   *[other] יובאו { $count } אנשי קשר אל { $place }
}
contacts-imported-some = { $count ->
    [one] יובא { $count } איש קשר אל { $place }; { $skipped } שכבר שמורים לא נכללו
    [two] יובאו { $count } אנשי קשר אל { $place }; { $skipped } שכבר שמורים לא נכללו
   *[other] יובאו { $count } אנשי קשר אל { $place }; { $skipped } שכבר שמורים לא נכללו
}
contacts-import-none = לא נמצאו אנשי קשר ב-{ $name }
contacts-import-all-saved = כל האנשים ב-{ $name } כבר שמורים
contacts-import-failed = לא ניתן לקרוא את { $name }: { $error }
contacts-exported = { $count ->
    [one] יוצא { $count } איש קשר אל { $path }
    [two] יוצאו { $count } אנשי קשר אל { $path }
   *[other] יוצאו { $count } אנשי קשר אל { $path }
}
contacts-export-none = אין אנשי קשר לייצוא
contacts-export-failed = לא ניתן לייצא אנשי קשר: { $error }
contacts-print = הדפסה
contacts-print-title = אנשי קשר
contacts-print-none = אין אנשי קשר להדפסה
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = יום הולדת: { $day }
contacts-print-nickname = כינוי: { $name }
contacts-create = איש קשר חדש

## Search and the list

contacts-search = חיפוש אנשי קשר
contacts-loading = טוען אנשי קשר…
contacts-empty = עדיין אין אנשי קשר שמורים. אנשי קשר ששמרת ב-Gmail, ב-Outlook או בשירות הדואר שלך יופיעו כאן.
contacts-empty-no-books = אנשי קשר מהחשבונות שלך יופיעו כאן לאחר הסנכרון.
contacts-none-found = אין אנשי קשר שתואמים לחיפוש.
contacts-starred = { $count ->
    [one] איש קשר מסומן בכוכב ({ $count })
    [two] אנשי קשר מסומנים בכוכב ({ $count })
   *[other] אנשי קשר מסומנים בכוכב ({ $count })
}
contacts-count = אנשי קשר ({ $count })
contacts-col-name = שם
contacts-col-email = אימייל
contacts-col-phone = מספר טלפון
contacts-col-job = תפקיד וחברה
contacts-col-labels = תוויות

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = לאפשר ל-Katna לקרוא את אנשי הקשר של { $address }.
contacts-allow-many = { $more ->
    [one] לאפשר ל-Katna לקרוא את אנשי הקשר של { $address } ושל חשבון נוסף אחד.
    [two] לאפשר ל-Katna לקרוא את אנשי הקשר של { $address } ושל { $more } חשבונות נוספים.
   *[other] לאפשר ל-Katna לקרוא את אנשי הקשר של { $address } ושל { $more } חשבונות נוספים.
}
contacts-allow-button = אפשר

## A contact's page

contacts-back = חזרה לאנשי הקשר
contacts-edit = עריכה
contacts-delete = מחיקה
contacts-qr = שיתוף כקוד QR
contacts-qr-about = סרקו את הקוד במצלמת הטלפון כדי לשמור את איש הקשר.
contacts-qr-too-long = לאיש קשר זה יש יותר מדי פרטים ואי אפשר להכניס אותם לקוד QR.
contacts-qr-done = בוצע
contacts-deleted = איש הקשר { $name } נמחק
contacts-added = { $name } נוסף לאנשי הקשר
contacts-find-mail = אימייל
contacts-details = פרטי איש קשר
contacts-saved-in = נשמר ב
contacts-notes = הערות
contacts-birthday = יום הולדת
contacts-nickname = כינוי
contacts-this-computer = המחשב הזה
contacts-kind-home = בית
contacts-kind-work = עבודה
contacts-kind-mobile = נייד
contacts-kind-other = אחר
contacts-source-google = אנשי קשר ב-Google
contacts-source-microsoft = אנשי קשר ב-Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = יצירת איש קשר
contacts-edit-title = עריכת איש קשר
contacts-edit-save = שמירה
contacts-edit-saving = שומר…
contacts-edit-cancel = ביטול
contacts-saved = איש הקשר נשמר
contacts-edit-save-to = שמירה ב
contacts-edit-changes-go-to = השינויים נשמרים ב-{ $place }.
contacts-edit-given = שם פרטי
contacts-edit-family = שם משפחה
contacts-edit-company = חברה
contacts-edit-job = תפקיד
contacts-edit-email = אימייל
contacts-edit-phone = טלפון
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = הוספת אימייל
contacts-edit-add-phone = הוספת טלפון
contacts-edit-street = כתובת
contacts-edit-city = עיר
contacts-edit-postcode = מיקוד
contacts-edit-country = מדינה
contacts-edit-birthday = יום הולדת (YYYY-MM-DD)
contacts-edit-empty = קודם יש להוסיף שם, אימייל או מספר טלפון.
