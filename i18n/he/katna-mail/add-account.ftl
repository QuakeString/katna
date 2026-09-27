# Katna Mail, Hebrew (עברית).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = הוספת חשבון דואר
add-account-looking = מחפשים את שרתי הדואר של { $address }…
add-account-address-intro = מזינים את כתובת האימייל, ו־Katna מוצאת את השרתים.
add-account-servers-title = הגדרות שרת
add-account-servers-intro = השרתים שדרכם Katna קוראת ושולחת דואר עבור { $address }.
add-account-password-title = הזנת הסיסמה
add-account-signing-in = מתחברים…

## Add a mail account: fields

add-account-field-address = כתובת אימייל
add-account-incoming = דואר נכנס ({ $protocol })
add-account-outgoing = דואר יוצא ({ $protocol })
add-account-field-server = שרת
add-account-field-port = פורט
add-account-security-none = ללא
add-account-field-username = שם משתמש
add-account-field-password = סיסמה
add-account-show-password = הצגת הסיסמה
add-account-app-password-hint = כאן { $provider } דורש סיסמה לאפליקציה, ולא את הסיסמה שבה משתמשים באתר. אפשר ליצור אחת בהגדרות האבטחה של חשבון ה־{ $provider } שלך.
add-account-field-name = השם שלך (לא חובה)
add-account-name-hint = מוצג לאנשים שכותבים אליהם.
add-account-servers-pair = { $imap } ו־{ $smtp }
add-account-servers-found = { $source ->
    [built-in] שרתים: { $servers }, נמצאו ברשימת הספקים של Katna.
    [provider] שרתים: { $servers }, נמצאו בהגדרות של ספק הדואר שלך.
    [ispdb] שרתים: { $servers }, נמצאו ברשימת הספקים של Thunderbird.
    [dns] שרתים: { $servers }, נמצאו ברשומות ה־DNS של הדומיין שלך.
   *[other] שרתים: { $servers }, לפי ניחוש; כדאי לבדוק אותם אם ההתחברות נכשלת.
}
add-account-servers-entered = שרתים: { $servers }, כפי שהוזנו.

## Add a mail account: buttons

add-account-servers-button = הגדרות שרת
add-account-back = חזרה
add-account-add = הוספת חשבון
add-account-next = הבא
add-account-cancel = ביטול

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] יש להזין את שרת הדואר הנכנס.
   *[outgoing] יש להזין את שרת הדואר היוצא.
}
add-account-server-space = { $kind ->
    [incoming] בשם שרת הדואר הנכנס יש רווח.
   *[outgoing] בשם שרת הדואר היוצא יש רווח.
}
add-account-port-invalid = { $kind ->
    [incoming] הפורט של הדואר הנכנס חייב להיות מספר בין { $min } ל־{ $max }.
   *[outgoing] הפורט של הדואר היוצא חייב להיות מספר בין { $min } ל־{ $max }.
}
add-account-address-empty = יש להזין כתובת אימייל.
add-account-address-invalid = יש להזין כתובת אימייל כמו { $example }.
add-account-not-found = Katna לא הצליחה למצוא את השרתים של { $address }, ולכן מילאה את השמות הנפוצים. כדאי לבדוק אותם מול ספק הדואר.
add-account-password-empty = יש להזין את הסיסמה.
add-account-added = { $address } נוסף. מורידים את הדואר שלך…
add-account-app-password-refused = { $provider } דחה את הסיסמה. נדרשת סיסמה לאפליקציה, ולא הסיסמה שבה משתמשים באתר.
add-account-password-refused = השרת דחה את הסיסמה. כדאי לבדוק אותה ולנסות שוב.

## The account menu (from the account button on the top bar)

add-account-menu-another = הוספת חשבון נוסף
add-account-menu-manage = ניהול חשבונות
