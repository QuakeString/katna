# Katna Mail, Hebrew (עברית).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = הוספת חשבון דואר
add-account-providers-intro = יש לבחור את ספק הדואר שלך. Katna תמצא את השאר.
add-account-provider-other = דואר אחר
add-account-provider-other-detail = כל חשבון IMAP או POP3
add-account-provider-google-detail = Gmail ו־Google Workspace
add-account-provider-microsoft-detail = Outlook ו־Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = התחברות ל־{ $provider }
add-account-form-title-other = חשבון הדואר שלך
add-account-form-intro = Katna שומרת את הסיסמה שלך במחזיק המפתחות של המערכת.
add-account-looking = מחפשים את שרתי הדואר של { $address }…
add-account-address-intro = מזינים את כתובת האימייל, ו־Katna מוצאת את השרתים.
add-account-servers-title = הגדרות שרת
add-account-servers-intro = השרתים שדרכם Katna קוראת ושולחת דואר עבור { $address }.
add-account-signing-in = מתחברים…
add-account-browser-title = ממשיכים בדפדפן
add-account-browser-intro = Katna פתחה את דף ההתחברות של { $provider } בדפדפן. יש להתחבר שם ולאשר ל־Katna לקרוא ולשלוח את הדואר שלך, ואז לחזור לכאן.
add-account-browser-hint = לא נפתח דף? כדאי לבדוק את חלונות הדפדפן, או לחזור אחורה ולנסות שוב.
add-account-stage-browser = ממתינים להתחברות שלך בדפדפן…
add-account-stage-signing-in-at = מתחברים אל { $server }…
add-account-help-app-password-link = איך יוצרים סיסמת אפליקציה
add-account-help-turn-on-imap = { $provider } מאפשר לאפליקציות דואר להתחבר רק אחרי שמפעילים גישת IMAP ו־POP3 בהגדרות של הדואר באינטרנט שלו.
add-account-help-turn-on-imap-link = איך מפעילים את זה

## Add a mail account: fields

add-account-field-address = כתובת אימייל
add-account-receive-with = קבלת דואר באמצעות
add-account-imap-about = IMAP שומר את הדואר והתיקיות שלך בשרת, זהים בכל מכשיר. כדאי לבחור בו כשאפשר.
add-account-pop3-about = POP3 מוריד את הדואר שלך למחשב הזה. דואר שקוראים או מעבירים כאן נשאר כמו שהוא בשרת ובמכשירים האחרים שלך.
add-account-incoming = דואר נכנס ({ $protocol })
add-account-outgoing = דואר יוצא ({ $protocol })
add-account-field-server = שרת
add-account-field-port = פורט
add-account-security-none = ללא
add-account-security-none-warning = לא מוצפן: אפשר לקרוא את הסיסמה והדואר שלך בדרך.
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
add-account-sign-in-with = התחברות עם { $provider }
add-account-sign-in-instead = התחברות עם { $provider } במקום זאת

## Add a mail account: buttons

add-account-servers-button = הגדרות שרת
add-account-back = חזרה
add-account-add = הוספת חשבון
add-account-done = סיום
add-account-another = הוספת חשבון נוסף
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
add-account-name-is-password = השם זהה לסיסמה. יש להקליד שם את השם שלך, כפי שאנשים אמורים לראות אותו.
add-account-app-password-refused = { $provider } דחה את הסיסמה. נדרשת סיסמה לאפליקציה, ולא הסיסמה שבה משתמשים באתר.
add-account-password-refused = השרת דחה את הסיסמה. כדאי לבדוק אותה ולנסות שוב.
add-account-sign-in-refused = { $provider } לא הכניס את Katna. יש לנסות שוב ולאשר גישה לדואר שלך.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] העותק הזה של Katna עדיין לא יכול להתחבר לחשבונות Microsoft.
    [Google] העותק הזה של Katna עדיין לא יכול להתחבר לחשבונות Google.
   *[other] הספק הזה מאפשר להתחבר רק בדף שלו, ו־Katna עדיין לא יכולה לעשות זאת עבורו.
}
add-account-smtp-not-found = Katna מצאה מאיפה לקרוא את הדואר שלך, אבל לא לאן לשלוח אותו. יש להזין את שרת הדואר היוצא.
add-account-done-title = החשבון שלך מוכן
add-account-done-intro = Katna מורידה עכשיו את הדואר שלך. דואר חדש יופיע ברגע שיגיע.
add-account-done-sign-in = התחברות
add-account-done-signed-in-with = עם { $provider }, בדפדפן שלך
add-account-done-receiving = קבלת דואר
add-account-done-sending = שליחת דואר
add-account-done-on-server = דואר בשרת
add-account-done-kept = נשמר עד שמוחקים אותו ב־Katna
add-account-done-pop3-hint = אפשר לשנות מה קורה לדואר בשרת ב„הגדרות” > „חשבונות”.
add-account-done-zoho-title = משימות ויומנים
add-account-done-zoho-about = Zoho שומרת אותם בנפרד מהדואר. יש להתחבר עם Zoho פעם אחת כדי להביא אותם ל־Katna.
add-account-done-linked = המשימות והיומנים חוברו

## The account menu (from the account button on the top bar)

add-account-menu-another = הוספת חשבון נוסף
app-menu = התפריט הראשי
app-menu-back = חזרה
