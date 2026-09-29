# Katna Mail, Hebrew (עברית): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = אנשי קשר
contacts-frequent = תדירים
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
contacts-create = יצירת איש קשר

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
