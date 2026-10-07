# Katna Mail, Hebrew (עברית).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The signature being edited

signature-placeholder = השם שלך, וכל מה שיש להוסיף מתחתיו

## Its formatting bar

signature-bold = מודגש
signature-italic = נטוי
signature-underline = קו תחתון
signature-link = קישור
signature-link-apply = החלה
signature-picture = הוספת תמונה
signature-align-left = יישור לשמאל
signature-align-center = מרכוז
signature-align-right = יישור לימין
signature-numbered-list = רשימה ממוספרת
signature-bulleted-list = רשימת תבליטים
signature-remove-formatting = הסרת העיצוב

## Adding a picture

signature-picture-choose = הוספה
signature-picture-too-big = תמונות בחתימה יכולות להיות בגודל של עד { $size }.
signature-picture-kind = יש לבחור תמונה מסוג PNG, JPEG, GIF או WebP.
signature-picture-unreadable = { $name }: { $error }
signature-layout = פריסה
signature-layout-own = משלך
signature-layout-classic = קלאסית
signature-layout-logo-left = לוגו משמאל
signature-layout-photo = תמונה
signature-layout-band = פס צבע
signature-layout-one-line = שורה אחת
signature-layout-centred = ממורכזת
signature-layout-banner = עם באנר
signature-layout-underline = קו תחתון
signature-layout-side-bar = פס צדדי
signature-layout-card = כרטיס
signature-layout-monogram = מונוגרמה
signature-layout-plain = טקסט פשוט
signature-layout-mobile-label = נייד:
signature-layout-office-label = משרד:
signature-layout-email-label = אימייל:
signature-layout-name = שם
signature-layout-job = תפקיד
signature-layout-company = חברה
signature-layout-mobile = נייד
signature-layout-office = משרד
signature-layout-email = אימייל
signature-layout-website = אתר
signature-layout-address = כתובת
signature-layout-pictures = תמונות
signature-layout-logo = לוגו
signature-layout-photo-picture = תמונה
signature-layout-banner-picture = באנר
signature-layout-remove-picture = הסרה
signature-layout-pages = דפים
signature-layout-page-placeholder = הוספת כתובת של דף
signature-layout-colour = צבע
signature-layout-picture-failed = לא ניתן היה להשתמש ב־{ $name } כתמונה.
signature-layout-preview = איך הקורא רואה אותה
signature-layout-light = בהירה
signature-layout-dark = כהה
signature-layout-text = טקסט פשוט
signature-layout-inside = התמונות נשלחות בתוך הדואר, כך שהן מוצגות גם במקומות שבהם תמונות מהאינטרנט כבויות. זו מוסיפה { $size } לכל הודעה.
signature-layout-free = רוצים משהו אחר?
signature-layout-edit = עריכה ידנית
signature-layout-edit-confirm = לערוך אותה ידנית? השדות והפריסה שלה יוסרו, והיא תשמור על המראה שלה ככל שהעורך יכול.
signature-layout-use-confirm = להשתמש בפריסה { $layout }? היא תחליף את החתימה הזו, וימולאו בה הפרטים ממנה.
signature-layout-use = שימוש בפריסה
signature-layout-cancel = ביטול
signature-html-title = הדבקת HTML
signature-html-subtitle = לחתימה שעיצבת במקום אחר
signature-html-placeholder = כאן מדביקים את ה־HTML של החתימה
signature-html-name = מודבקת
signature-html-new = נשמרה כחתימה חדשה, „{ $name }”
signature-html-replaces = תישמר במקום „{ $name }”
signature-html-cancel = ביטול
signature-html-save = שמירה
signature-html-fetching = מורידים את התמונות שלה…
signature-html-pictures-inside = { $count ->
    [one] תמונה אחת הורדה והוכנסה לתוך הדואר ({ $size })
    [two] { $count } תמונות הורדו והוכנסו לתוך הדואר ({ $size })
   *[other] { $count } תמונות הורדו והוכנסו לתוך הדואר ({ $size })
}
signature-html-pictures-web = { $count ->
    [one] לא ניתן היה להוריד תמונה אחת, ולכן הקוראים טוענים אותה מהאינטרנט
    [two] לא ניתן היה להוריד { $count } תמונות, ולכן הקוראים טוענים אותן מהאינטרנט
   *[other] לא ניתן היה להוריד { $count } תמונות, ולכן הקוראים טוענים אותן מהאינטרנט
}
signature-html-removed = הוסרו סקריפטים, טפסים ופיקסלי מעקב, שאפליקציות דואר חוסמות בכל מקרה
signature-html-style-sheet = גיליון סגנונות הושמט: בדואר נשמרים רק הסגנונות שכתובים על כל רכיב
signature-html-links = הוסרו קישורים שהובילו למקום שאינו אתר, כתובת או טלפון
signature-html-plain-text = נוצרה ממנה גרסת טקסט פשוט, לאפליקציות דואר שמציגות רק טקסט
signature-import-title = ייבוא
signature-import-subtitle = מ־Gmail, Thunderbird, Evolution ו־KMail
signature-import-looking = מחפשים חתימות…
signature-import-none = לא נמצאו חתימות. לאפליקציה אחרת, אפשר להעתיק את ה־HTML של החתימה שלה ולהשתמש ב„הדבקת HTML”.
signature-import-from = מ־{ $app }
signature-import-already = כבר ב־Katna
signature-import-gmail-sign-in = { $address }: יש להתחבר שוב ב„הגדרות” > „חשבונות” כדי ש־Katna תוכל לקרוא את החתימות של Gmail.
signature-import-gmail-failed = { $address }: { $error }
signature-import-cancel = ביטול
signature-import-do = { $count ->
    [one] ייבוא חתימה אחת
    [two] ייבוא { $count } חתימות
   *[other] ייבוא { $count } חתימות
}
signature-import-name = { $name } ({ $app })
