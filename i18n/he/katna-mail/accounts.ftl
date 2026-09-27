# Katna Mail, Hebrew (עברית).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = חלונית התיקיות
accounts-folder-pane-detail = התיקיות של אילו חשבונות מוצגות בחלונית שבצד ימין.
accounts-shown-one = חשבון אחד בכל פעם; מחליפים בכרטיס החשבון
accounts-shown-all = כל החשבונות, אחד אחרי השני
accounts-row = חשבונות
accounts-row-detail = חלונית התיקיות ותפריט החשבון מציגים את החשבונות בסדר הזה; הראשון הוא ברירת המחדל. הסרת חשבון מוחקת את העותק של Katna מהדואר שלו במחשב הזה. הדואר נשאר בשרת.
accounts-none = אין עדיין חשבונות.
accounts-kind-imported = מיובא
accounts-picture-reset = שימוש בתמונה של שולחן העבודה
accounts-picture-change = החלפת התמונה
accounts-picture-remove = הסרת התמונה
accounts-rename = שינוי שם
accounts-name-save = שמירה
accounts-name-cancel = ביטול
accounts-name-placeholder = השם שלך
accounts-rename-failed = לא ניתן לשנות את שם החשבון: { $error }
accounts-move-up = העברה למעלה
accounts-move-down = העברה למטה
accounts-drag = אפשר לגרור כדי לשנות את הסדר
accounts-remove = הסרה
accounts-delete-all-row = מחיקת כל הנתונים
accounts-delete-all-row-detail = התחלה מחדש, כמו בהתקנה חדשה.
accounts-delete-all-about = נמחקים מהמחשב הזה כל החשבונות, כל הדואר השמור, אנשי הקשר והיומנים, אינדקס החיפוש, ההגדרות והסיסמאות השמורות שלך. שום דבר לא משתנה בשרתי הדואר שלך.
accounts-delete-all-open = מחיקת כל הנתונים של Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } הוסר מ־Katna.
accounts-removed = { $address } הוסר מ־Katna. הדואר שלו עדיין בשרת.
accounts-all-deleted = כל הנתונים של Katna נמחקו מהמחשב הזה.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = להסיר את { $address }?
accounts-remove-confirm = הסרת החשבון
accounts-removing = מתבצעת הסרה…
accounts-remove-local-mail = { $folders ->
    [0] כל הדואר שיובא לחשבון הזה
    [one] כל הדואר שיובא לחשבון הזה, בתיקייה שלו
    [two] כל הדואר שיובא לחשבון הזה, ב־{ $folders } התיקיות שלו
   *[other] כל הדואר שיובא לחשבון הזה, ב־{ $folders } התיקיות שלו
}
accounts-remove-local-settings = ההגדרות שלו ב־Katna
accounts-remove-mail = { $folders ->
    [0] כל הדואר של החשבון הזה ש־Katna שומרת
    [one] כל הדואר של החשבון הזה ש־Katna שומרת, בתיקייה שלו
    [two] כל הדואר של החשבון הזה ש־Katna שומרת, ב־{ $folders } התיקיות שלו
   *[other] כל הדואר של החשבון הזה ש־Katna שומרת, ב־{ $folders } התיקיות שלו
}
accounts-remove-outbox = ההודעות שלו שממתינות בדואר היוצא
accounts-remove-settings = הסיסמה השמורה שלו וההגדרות שלו ב־Katna
accounts-delete-all-title = למחוק את כל הנתונים של Katna?
accounts-delete-all-confirm = מחיקת הכול
accounts-deleting = מתבצעת מחיקה…
accounts-delete-all-accounts = כל החשבונות, וכל הדואר והקבצים המצורפים ש־Katna שומרת
accounts-delete-all-contacts = אנשי קשר, יומנים ואינדקס החיפוש
accounts-delete-all-settings = כל ההגדרות, החתימות וקיצורי המקלדת
accounts-delete-all-passwords = כל הסיסמאות השמורות
accounts-deleted-heading = יימחקו מהמחשב הזה:
accounts-cannot-undo = אי אפשר לבטל את הפעולה הזו.
accounts-server-delete-all = שום דבר לא משתנה בשרתי הדואר שלך: הדואר נשאר שם, והוספה מחדש של חשבון מורידה אותו שוב. דואר שיובא מקבצים נמצא רק ב־Katna; הקבצים עצמם לא משתנים.
accounts-server-local = הדואר הזה יובא מקבצים, ולכן ל־Katna יש את העותק היחיד. הקבצים שמהם הוא הגיע לא משתנים; אפשר לייבא אותם שוב כדי לקבל אותו בחזרה.
accounts-server-remove = שום דבר לא משתנה בשרת הדואר: הדואר נשאר שם, והוספה מחדש של החשבון מורידה אותו שוב.
accounts-confirm-word = מחיקה
accounts-confirm-placeholder = יש להקליד „{ accounts-confirm-word }”
accounts-confirm-prompt = לאישור, יש להקליד „{ accounts-confirm-word }”:
accounts-cancel = ביטול
