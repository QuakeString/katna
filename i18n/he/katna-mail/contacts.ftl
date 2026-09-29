# Katna Mail, Hebrew (עברית): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = אנשי קשר
contacts-frequent = תדירים
contacts-labels = תוויות

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
