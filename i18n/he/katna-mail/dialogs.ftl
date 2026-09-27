# Katna Mail, Hebrew (עברית).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = על Katna
about-tagline = דואר ויומן לשולחן העבודה של Linux
about-whats-new = מה חדש
about-changelog = יומן שינויים
about-source = קוד מקור
about-coffee = קנו לי קפה
about-coming-soon = בקרוב
about-follow = לעקוב אחרי היוצר
about-love-title = נוצר באהבה ל־Rust, ל־KDE ול־Linux
about-love-text = עם Rust, כתיבת אפליקציית דואר מהירה ובטוחה היא תענוג: ב־Katna אין קוד unsafe. שולחן העבודה Plasma של KDE וחבילת ה־PIM שלו היו ההשראה ל־Katna, ו־Linux וקהילת התוכנה החופשית בונות את הקרקע שעליה היא עומדת. תודה, ותודה גם לספריות שלמטה.
about-kde-text = KDE בונה את שולחן העבודה שבו Katna מרגישה הכי בבית, והוא נוצר על ידי מתנדבים וממומן על ידי אנשים כמוך. אם Plasma או האפליקציות של KDE מוצאות חן בעיניך, כדאי לשקול תרומה ל־KDE.
about-donate-kde = תרומה ל־KDE
about-gpui-title = בנוי על GPUI, מפרויקט Zed
about-gpui-text = כל הממשק של Katna Mail בנוי על GPUI, מסגרת הממשק המהירה עם האצת GPU ש־Zed Industries יצרה עבור העורך Zed. כל פיקסל, אנימציה וחלון שמופיעים על המסך מצוירים על ידה. תודה לצוות Zed על שבנה אותה בקוד פתוח. Apache-2.0.
about-gpui-github = GPUI ב־GitHub
about-personal-title = פרויקט אישי
about-personal-text = Katna Mail לא מנסה להיות חדשנית או מהפכנית. זו אפליקציית הדואר שהיוצר שלה רצה, והתכונות והמראה שלה שאולים מ־Gmail, מ־Mailspring ומ־Thunderbird. היא התאפשרה רק בזכות ההתקדמות של מודלי השפה הגדולים (LLM).
about-built-on = בנוי על תוכנה חופשית
about-credit-pimalaya = IMAP, SMTP והתחברות (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = קריאה וכתיבה של IMAP
about-credit-tantivy = חיפוש
about-credit-sqlite = מאגר הדואר
about-credit-rustls = חיבורים מאובטחים
about-credit-mail-parser = קריאת דואר, מבית Stalwart Labs
about-credit-html5ever = דואר HTML, מפרויקט Servo
about-credit-zbus = תקשורת עם שולחן העבודה דרך D-Bus ופורטלים
about-credit-oo7 = סיסמאות במחזיק המפתחות של שולחן העבודה
about-credit-hayro = הצגה והדפסה של קובצי PDF
about-credit-calamine = תצוגה מקדימה של גיליונות אלקטרוניים
about-credit-resvg = תמונות SVG
about-credit-jiff = תאריכים ואזורי זמן
about-credit-spellbook = בדיקת איות, מהעורך Helix
about-credit-smol = ביצוע דברים רבים בבת אחת
about-all-libraries = כל הספריות ש־Katna משתמשת בהן ({ $count })
about-library-authors = מאת { $authors }
about-license = Katna היא תוכנה חופשית תחת GNU GPL, גרסה 3 ואילך.
about-close = סגירה

## What’s new (shown after an update)

whats-new-title = מה חדש ב־Katna Mail
whats-new-updated = עודכן לגרסה { $version }
whats-new-version = גרסה { $version }
whats-new-more = { $count ->
    [one] ועוד שינוי אחד ביומן השינויים המלא.
    [two] ועוד { $count } שינויים ביומן השינויים המלא.
   *[other] ועוד { $count } שינויים ביומן השינויים המלא.
}
whats-new-changelog = יומן השינויים המלא
whats-new-got-it = הבנתי

## First run: welcome page

onboarding-welcome-title = ברוכים הבאים ל־Katna Mail
onboarding-welcome-lead = הדואר שלך במחשב שלך: חיפוש מהיר, קריאה גם בלי חיבור, ופרטיות.
onboarding-fast-title = מהירה, גם בלי חיבור
onboarding-fast-text = Katna שומרת כאן עותק של הדואר שלך, כך שפתיחה וחיפוש קורים מיד, עם חיבור או בלעדיו.
onboarding-providers-title = עובדת עם הדואר שלך
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud וכל חשבון IMAP או POP אחר.
onboarding-private-title = פרטית
onboarding-private-text = הדואר שלך מגיע ישירות מספק הדואר שלך למחשב הזה. אף שרת של Katna לא רואה אותו.
onboarding-get-started = בואו נתחיל

## First run: adding an account

onboarding-service-checking = בודקים את שירות הרקע של Katna…
onboarding-service-running = שירות הרקע של Katna פועל.
onboarding-service-missing = שירות הרקע של Katna לא פועל
onboarding-service-start = הוא מוריד ושולח את הדואר שלך. אפשר להפעיל אותו ממסוף ואז לבדוק שוב:
onboarding-check-again = בדיקה חוזרת
onboarding-account-title = הוספת חשבון הדואר שלך
onboarding-account-lead = מקלידים את כתובת האימייל והסיסמה, ו־Katna מוצאת את הגדרות השרת. Gmail, Yahoo ו־iCloud דורשים סיסמה לאפליקציה, שיוצרים בהגדרות האבטחה של החשבון.
onboarding-add-account = הוספת חשבון
onboarding-back = חזרה

## First run: choosing the look

onboarding-look-title = להתאים אישית
onboarding-look-lead = בוחרים איך הדואר נפתח ואיך Katna נראית. אפשר לשנות זאת בכל עת בהגדרות המהירות.
onboarding-reading-pane = חלונית הקריאה
onboarding-pane-right = ליד הרשימה
onboarding-pane-none = ללא פיצול
onboarding-theme = ערכת נושא
onboarding-theme-system = מערכת
onboarding-theme-light = בהירה
onboarding-theme-dark = כהה
onboarding-density = צפיפות
onboarding-density-default = ברירת מחדל
onboarding-density-compact = דחוס
onboarding-continue = המשך

## First run: done

onboarding-ready-title = הכול מוכן
onboarding-ready-lead = Katna מורידה את הדואר שלך. ההודעות מופיעות כשהן מגיעות, ודואר חדש יופיע מעצמו.
onboarding-ready-lead-address = Katna מורידה את הדואר של { $address }. ההודעות מופיעות כשהן מגיעות, ודואר חדש יופיע מעצמו.
onboarding-ready-tour = לצאת לסיור של דקה כדי לראות איפה הכול נמצא?
onboarding-skip = לא עכשיו
onboarding-take-tour = יציאה לסיור

## Asking to send crash reports (on its own and on the first-run pages)

share-title = עזרה בשיפור Katna
share-lead = כש־Katna קורסת, היא שומרת דוח במחשב הזה. שליחת הדוחות האלה עוזרת לתקן את מה שהשתבש. אפשר לשנות זאת בכל עת בהגדרות > משוב משתמשים.
share-sent = מה נשלח
share-sent-detail = דוח הקריסה כפי שאפשר לראות אותו בהגדרות: מה קרס והיכן ב־Katna, הגרסה, מערכת ה־Linux ושולחן העבודה שלך, ושורות היומן האחרונות של Katna, שעשויות לכלול שמות של תיקיות דואר.
share-never-sent = מה אף פעם לא נשלח
share-never-sent-detail = ההודעות, אנשי הקשר, הסיסמאות, כתובת ה־IP, שם המשתמש או שם המחשב שלך. כתובות אימייל מוסרות מהדוח.
share-where = לאן זה הולך
share-where-detail = למעקב הקריסות של Katna ב־Sentry, שמאוחסן באיחוד האירופי. אין מזהה שמקשר את הדוחות אליך.
share-dont-send = לא לשלוח
share-send = שליחת דוחות קריסה
share-sending = דוחות קריסה יישלחו. תודה.
share-local = דוחות הקריסה נשארים במחשב הזה.

## The tour (cards pointing at each part of the window)

tour-welcome-title = ברוכים הבאים ל־Katna Mail
tour-welcome-text = סיור של דקה מראה איפה הכול נמצא.
tour-not-now = לא עכשיו
tour-start = יציאה לסיור
tour-close = סגירה
tour-skip = דילוג על הסיור
tour-back = הקודם
tour-done = סיום
tour-next = הבא
tour-step = { $step } מתוך { $total }
tour-compose-title = כתיבת הודעה
tour-compose-text = כתיבה פותחת הודעה חדשה בפינה הימנית התחתונה, כך שאפשר להמשיך לקרוא תוך כדי כתיבה.
tour-search-title = חיפוש בכל הדואר שלך
tour-search-text = החיפוש עובד גם בלי חיבור. הכפתור בקצה הימני מוסיף מסננים: שולח, נמען, נושא, תאריכים וקבצים מצורפים.
tour-menu-title = הצגה או הסתרה של התיקיות
tour-menu-text = הכפתור הזה מקפל את רשימת התיקיות. כשהיא מוסתרת, מציבים את הסמן על אימייל בצד שמאל כדי לראות את התיקיות.
tour-apps-title = האפליקציות שלך
tour-apps-text = כרגע יש כאן אימייל. יומן, אנשי קשר, משימות, הערות ופידים יצטרפו אליו בסרגל הזה.
tour-tabs-title = כרטיסיות דואר נכנס
tour-tabs-text = דואר חדש ממוין לראשי, קידומי מכירות, רשתות חברתיות, עדכונים ופורומים. אפשר לכבות את הכרטיסיות בהגדרות המהירות.
tour-list-title = ההודעות שלך
tour-list-text = לוחצים על הודעה כדי לקרוא אותה. מציבים עליה את הסמן לפעולות מהירות, לוחצים לחיצה ימנית לעוד אפשרויות, או מסמנים כמה הודעות כדי לטפל בהן יחד.
tour-settings-title = הגדרות מהירות
tour-settings-text = כאן משנים את חלונית הקריאה, הצפיפות וערכת הנושא. משם אפשר גם להתחיל את הסיור מחדש.
tour-account-title = החשבון שלך
tour-account-text = כאן רואים באיזה חשבון נמצאים, ומוסיפים חשבון נוסף.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] שירות הרקע של Katna נעצר באופן בלתי צפוי.
    [one] שירות הרקע של Katna נעצר באופן בלתי צפוי. נשמר דוח קריסה נוסף אחד.
    [two] שירות הרקע של Katna נעצר באופן בלתי צפוי. נשמרו עוד { $more } דוחות קריסה.
   *[other] שירות הרקע של Katna נעצר באופן בלתי צפוי. נשמרו עוד { $more } דוחות קריסה.
}
crash-mail = { $more ->
    [0] Katna Mail נסגרה באופן בלתי צפוי בפעם הקודמת.
    [one] Katna Mail נסגרה באופן בלתי צפוי בפעם הקודמת. נשמר דוח קריסה נוסף אחד.
    [two] Katna Mail נסגרה באופן בלתי צפוי בפעם הקודמת. נשמרו עוד { $more } דוחות קריסה.
   *[other] Katna Mail נסגרה באופן בלתי צפוי בפעם הקודמת. נשמרו עוד { $more } דוחות קריסה.
}
crash-view = הצגת הדוח
crash-view-tooltip = פתיחת הדוח, השמור במחשב הזה
crash-copy = העתקת הדוח
crash-close = סגירה
