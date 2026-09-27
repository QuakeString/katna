# Katna Mail, Hebrew (עברית).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings page: its tabs

settings-tab-general = כללי
settings-tab-inbox = דואר נכנס
settings-tab-accounts = חשבונות
settings-tab-subscriptions = מינויים
settings-tab-appearance = מראה
settings-tab-shortcuts = קיצורי דרך
settings-tab-default-apps = אפליקציות ברירת מחדל
settings-tab-folders-rules = תיקיות וכללים
settings-tab-compose = כתיבה
settings-tab-mcp-server = שרת MCP
settings-tab-feedback = משוב משתמשים
settings-tab-experimental = ניסיוני

## Settings page: tabs still to come

settings-tab-subscriptions-coming = צפייה בניוזלטרים וברשימות התפוצה שמגיעים אליך, וביטול מינוי בלחיצה אחת.
settings-tab-folders-rules-coming = יצירה, שינוי שם, העברה והסתרה של תיקיות ותוויות, ובחירה אילו מהן יסונכרנו. כללים ממיינים, מתייגים, מעבירים או מוחקים דואר חדש באופן אוטומטי, לפי שולח, נושא או מילים.
settings-tab-mcp-server-coming = מתן אפשרות לעוזרי AI במחשב הזה לחפש, לקרוא ולנסח טיוטות בדואר שלך, באישורך.

## Settings > General

settings-general-conversations = תצוגת שיחות
settings-general-conversations-group = קיבוץ תשובות לאותה הודעה
settings-general-conversations-group-detail = שורה אחת לכל שיחה ברשימה
settings-general-reading = קריאה
settings-general-newest-first = ההודעה החדשה ביותר ראשונה
settings-general-newest-first-detail = השיחה מתחילה בתשובה האחרונה שלה
settings-general-full-headers = הצגת כותרות מלאות
settings-general-full-headers-detail = מאת, אל, עותק, תאריך ונושא פתוחים בכל הודעה
settings-general-full-names = שמות מלאים של הנמענים
settings-general-full-names-detail = „אליי, Ada Lovelace” במקום „אליי, Ada”
settings-general-mark-read = סימון כנקראה
settings-general-mark-read-now = מיד כשהיא נפתחת
settings-general-mark-read-1s = אחרי שהיא פתוחה שנייה אחת
settings-general-mark-read-3s = אחרי שהיא פתוחה 3 שניות
settings-general-mark-read-never = רק בסימון ידני כנקראה
settings-general-auto-advance = מעבר אוטומטי
settings-general-auto-advance-detail = אחרי מחיקה, העברה לארכיון או העברה של השיחה הפתוחה
settings-general-auto-advance-next = פתיחת השיחה הבאה
settings-general-auto-advance-previous = פתיחת השיחה הקודמת
settings-general-auto-advance-list = חזרה לרשימה
settings-general-reply-button = כפתור התשובה
settings-general-reply-all = תשובה לכולם
settings-general-reply-all-detail = כפתור התשובה שליד כל הודעה עונה לכולם, ולא רק לשולח
settings-general-remote-images = תמונות מהאינטרנט
settings-general-remote-images-detail = טעינת התמונות של הודעה מגלה לשולח שפתחת אותה, מתי ובערך מאיפה. כשהאפשרות כבויה, כל הודעה שואלת קודם, ותמיד אפשר להציג את התמונות של שולח מסוים.
settings-general-remote-images-always = תמיד להציג תמונות
settings-general-remote-images-always-detail = בכל הודעה, ולא רק משולחים מהימנים
settings-general-sending = שליחה
settings-general-sending-detail = כמה זמן הודעה שנשלחה ממתינה, כדי שאפשר יהיה לבטל את שליחתה.
settings-general-offline = דואר במצב לא מקוון
settings-general-offline-detail = דואר אחרון מורד במלואו, לקריאה ללא חיבור. דואר ישן יותר מורד כשפותחים אותו.
settings-general-offline-days = { $count ->
    [one] יום אחד
    [two] יומיים
   *[other] { $count } ימים
}
settings-general-offline-years = { $count ->
    [one] שנה אחת
    [two] שנתיים
   *[other] { $count } שנים
}
settings-general-offline-all = כל הדואר
settings-general-offline-note = בחירה בפחות ימים משאירה את הדואר שכבר הורד. שום דבר לא משתנה בשרת.
settings-general-notifications = התראות
settings-general-notifications-detail = על דואר חדש בדואר הנכנס, גם כש־Katna Mail סגורה.
settings-general-new-mail = התראה על דואר חדש
settings-general-new-mail-detail = עם „תשובה לכולם”, „סימון כנקראה” ו„העברה לארכיון”
settings-general-new-mail-sound = השמעת צליל
settings-general-new-mail-sound-detail = צליל הדואר החדש של שולחן העבודה
settings-general-reset-cache = איפוס המטמון
settings-general-reset-cache-detail = כשהדואר נראה שגוי או לא מעודכן, או כדי לפנות מקום בדיסק. שום דבר לא משתנה בשרתי הדואר שלך.
settings-general-desktop = שולחן העבודה
settings-general-start-at-login = הפעלת Katna בכניסה למערכת
settings-general-start-at-login-detail = מסנכרן דואר ומציג התראות על דואר חדש ואת הסמל במגש המערכת, בלי לפתוח את החלון
settings-general-login-window = פתיחה גם של החלון של Katna Mail
settings-general-login-window-detail = גם החלון נפתח בכניסה למערכת
settings-general-tray = הצגת Katna במגש המערכת
settings-general-tray-detail = עם מספר ההודעות שלא נקראו ותפריט
settings-general-unread-badge = מספר ההודעות שלא נקראו על סמל שורת המשימות
settings-general-unread-badge-detail = כמה הודעות בדואר הנכנס לא נקראו

## Settings > Inbox

settings-inbox-tabs = כרטיסיות בדואר הנכנס
settings-inbox-tabs-detail = מיון הדואר הנכנס לכרטיסיות, כמו באתר של ספק הדואר שלך.
settings-inbox-tabs-show = הצגת כרטיסיות בדואר הנכנס
settings-inbox-tabs-show-detail = כשהאפשרות כבויה מוצגת רשימה אחת לכל חשבון
settings-inbox-no-accounts = יש להוסיף חשבון כדי לבחור את הכרטיסיות שלו.
settings-inbox-tabs-automatic = אוטומטי: { $tabs } ({ $provider })
settings-inbox-tabs-off = ללא כרטיסיות
settings-inbox-tabs-gmail = ראשי, קידומי מכירות, רשתות חברתיות, עדכונים, פורומים
settings-inbox-tabs-focused = ממוקד ואחר
settings-inbox-tabs-zoho = דואר נכנס, ניוזלטרים והתראות
settings-inbox-tabs-shown = הכרטיסיות המוצגות. דואר של כרטיסייה שמכבים נשאר ב־{ $tab }.

## Settings > Appearance

settings-appearance-reading-pane = חלונית הקריאה
settings-appearance-reading-pane-detail = איפה מוצגת שיחה פתוחה.
settings-appearance-pane-right = ליד הרשימה
settings-appearance-pane-none = ללא פיצול
settings-appearance-density = צפיפות
settings-appearance-density-default = ברירת מחדל
settings-appearance-density-compact = דחוס
settings-appearance-scaling = קנה מידה
settings-appearance-scaling-detail = מגדיל או מקטין את כל מה שב־Katna Mail, בנוסף לקנה המידה של שולחן העבודה עצמו: טקסט, סמלים, ריווח וקווים מפרידים. דואר שנשלח שומר על גודל הגופן שלו. גדלים קטנים מאוד עלולים להקשות על לחיצה על סמלים.
settings-appearance-theme = ערכת נושא
settings-appearance-theme-system = מערכת
settings-appearance-theme-light = בהירה
settings-appearance-theme-dark = כהה
settings-appearance-desktop-colors = צבעי שולחן העבודה
settings-appearance-desktop-colors-use = שימוש בצבעי שולחן העבודה
settings-appearance-desktop-colors-use-detail = ערכת הצבעים וצבע ההדגשה של שולחן העבודה
settings-appearance-app-names = שמות האפליקציות
settings-appearance-app-names-show = הצגת שמות האפליקציות
settings-appearance-app-names-show-detail = שמות מתחת לסמלי האפליקציות בקצה הימני
settings-appearance-sender-pictures = תמונות שולחים
settings-appearance-sender-pictures-show = הצגת לוגואים של חברות
settings-appearance-sender-pictures-show-detail = מאותרים לפי הדומיין של השולח, אף פעם לא לפי ההודעה, ונשמרים לשבוע
settings-appearance-important = סמני חשיבות
settings-appearance-important-show = הצגת סמני חשיבות
settings-appearance-important-show-detail = ליד כל הודעה ברשימה
settings-appearance-message-width = רוחב ההודעה
settings-appearance-message-width-limit = הגבלת רוחב ההודעות
settings-appearance-message-width-limit-detail = קל יותר לקרוא שורות ארוכות בחלון רחב
settings-appearance-mail-colors = צבעי הדואר
settings-appearance-mail-colors-detail = רוב הדואר מעוצב לדף לבן. בערכת נושא כהה הצבעים שלו מוחלפים בצבעים כהים שקל לקרוא; כשהאפשרות כבויה, הוא שומר על צבעי השולח על דף בהיר.
settings-appearance-dark-mail = צבעים כהים גם לדואר
settings-appearance-dark-mail-detail = רק כשערכת הנושא כהה
settings-appearance-attachment-previews = תצוגות מקדימות של קבצים מצורפים
settings-appearance-attachment-previews-show = הצגת תצוגות מקדימות של קבצים מצורפים
settings-appearance-attachment-previews-show-detail = תמונה קטנה של תוכן כל קובץ על הכרטיס שלו

## Settings > Default apps

settings-default-apps-intro = איפה נפתחים קבצים מצורפים כשלוחצים עליהם. המציג תמיד יכול לפתוח קובץ גם באפליקציה אחרת. אפליקציות ברירת המחדל של שולחן העבודה נקבעות בהגדרות שלו.
settings-default-apps-pdf = קובצי PDF
settings-default-apps-pdf-detail = עמודים, עם זום.
settings-default-apps-pictures = תמונות
settings-default-apps-pictures-detail = צילומים (מסובבים לכיוון הנכון), PNG, GIF, WebP, BMP, TIFF ו־SVG.
settings-default-apps-text = קובצי טקסט
settings-default-apps-text-detail = טקסט פשוט, יומנים, קוד וטקסט אחר.
settings-default-apps-sheets = גיליונות אלקטרוניים
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) ו־CSV.
settings-default-apps-documents = מסמכים
settings-default-apps-documents-detail = Word (docx, doc), טקסט OpenDocument (odt) ומצגות (pptx, ppt, odp).
settings-default-apps-katna = המציג של Katna Mail
settings-default-apps-system = אפליקציית ברירת המחדל של שולחן העבודה
settings-default-apps-ask = לשאול בכל פעם באיזו אפליקציה
settings-default-apps-after-saving = אחרי השמירה
settings-default-apps-show-folder = הצגת קבצים שנשמרו בתיקייה שלהם
settings-default-apps-show-folder-detail = פותח את מנהל הקבצים כשהקבצים המצורפים שנשמרו מסומנים

## Settings > Compose

settings-compose-send-from = שליחת הודעות חדשות מהחשבון
settings-compose-send-from-detail = תשובות והעברות תמיד יוצאות מהחשבון שבו נמצאים.
settings-compose-send-from-current = החשבון שבו נמצאים
settings-compose-send-on-replies = שליחה בתשובות
settings-compose-send-on-replies-detail = מה „שליחה” עושה בתשובה או בהעברה. התפריט שליד „שליחה” מציע את האפשרות השנייה.
settings-compose-send-plain = שליחה
settings-compose-send-archive = שליחה והעברה לארכיון
settings-compose-signatures = חתימות
settings-compose-signatures-detail = נוספת מתחת להודעה, אחרי שורת „--”. אפשר לבחור חתימה אחרת בחלון הכתיבה.
settings-compose-untitled = ללא שם
settings-compose-signature-name = שם, למשל עבודה
settings-compose-signature-first = החתימה שלי
settings-compose-signature-numbered = חתימה { $number }
settings-compose-signature-delete = מחיקה
settings-compose-signature-deleted = החתימה נמחקה
settings-compose-signature-new = יצירת חתימה חדשה
settings-compose-no-signatures = אין עדיין חתימות.
settings-compose-no-signature = ללא חתימה
settings-compose-for-new-mail = להודעות חדשות
settings-compose-for-replies = לתשובות ולהעברות
settings-compose-for-replies-detail = בשיחה שבה חתמת על הודעה, תשובה מתחילה במקום זאת באותה חתימה.
settings-compose-format = פורמט
settings-compose-plain-text = כתיבה בטקסט פשוט
settings-compose-plain-text-detail = דואר חדש מתחיל ללא עיצוב; אפשר להחליף בחלון הכתיבה
settings-compose-spelling = איות
settings-compose-spell-check = בדיקת איות בזמן הכתיבה
settings-compose-spell-check-detail = מילים עם שגיאות איות מסומנות בקו תחתון, עם הצעות בלחיצה ימנית
settings-compose-spell-desktop = השפה של שולחן העבודה ({ $language })
settings-compose-templates = תבניות
settings-compose-templates-detail = שמירת הודעות שכותבים לעיתים קרובות, והתחלת הודעה חדשה או תשובה מהן.

## Settings > Shortcuts

settings-shortcuts-set = ערכת קיצורים
settings-shortcuts-set-detail = אפשר להתחיל מהמקשים של אפליקציית דואר מוכרת. Cmd הוא Ctrl כאן. השינויים שלך נשמרים מעל הערכה, ו„שחזור ברירות המחדל” מחזיר את המקשים של הערכה.
settings-shortcuts-single = קיצורי מקש יחיד
settings-shortcuts-single-detail = מקשים בלי Ctrl או Alt, כמו בדואר אינטרנט: e מעביר לארכיון, j ו־k מזיזים, / מחפש. הם פועלים ברשימה ובשיחה הפתוחה, אף פעם לא בזמן הקלדה.
settings-shortcuts-single-use = שימוש בקיצורי מקש יחיד
settings-shortcuts-single-use-detail = קיצורי Ctrl תמיד פועלים
settings-shortcuts-how = יש ללחוץ על מקש כדי לשנות אותו, או על + כדי להוסיף, ואז להקיש את המקשים החדשים. Esc מבטל.
settings-shortcuts-restore = שחזור ברירות המחדל
settings-shortcuts-no-key = אין מקש
settings-shortcuts-press = יש להקיש על מקשים…
settings-shortcuts-then = { $keys } ואז…
settings-shortcuts-moved = { $keys } מבצע עכשיו „{ $action }” במקום „{ $previous }”.
settings-shortcuts-single-off = קיצורי מקש יחיד כבויים, לכן המקש הזה יפעל אחרי שיופעלו.
settings-shortcuts-restored = כל הקיצורים חזרו למקשים של הערכה שלהם.

## Settings search: the line under a result

settings-general-language-summary = שפת האפליקציה, התאריכים והמספרים
settings-general-reading-summary = ההודעה החדשה ביותר ראשונה, כותרות מלאות, שמות מלאים של הנמענים
settings-general-mark-read-summary = מתי שיחה פתוחה מסומנת כנקראה: מיד, אחרי 1 או 3 שניות, או ידנית
settings-general-auto-advance-summary = מה נפתח אחרי מחיקה, העברה לארכיון או העברה של השיחה הפתוחה: השיחה הבאה, הקודמת או הרשימה
settings-general-reply-button-summary = כפתור התשובה שליד כל הודעה עונה לכולם
settings-general-remote-images-summary = תמיד להציג את התמונות של כל הודעה
settings-general-sending-summary = ביטול שליחה: כמה זמן הודעה שנשלחה ממתינה, כדי שאפשר יהיה לבטל את שליחתה
settings-general-offline-summary = כמה ימים של דואר אחרון מורדים במלואם, לקריאה ללא חיבור
settings-general-notifications-summary = התראות על דואר חדש והצליל שלהן
settings-general-reset-cache-summary = מחיקת הדואר שהורד, תמונות השולחים ואינדקס החיפוש, והורדתם מחדש
settings-general-desktop-summary = הפעלת Katna בכניסה למערכת, הסמל במגש המערכת ומספר ההודעות שלא נקראו על סמל שורת המשימות
settings-accounts-accounts-summary = הוספה או הסרה של חשבון, או החלפת התמונה שלו
settings-appearance-density-summary = שורות ברירת מחדל או דחוסות ברשימה
settings-appearance-scaling-summary = הגדלה או הקטנה של הכול: טקסט, סמלים, ריווח וקווים מפרידים
settings-appearance-theme-summary = מערכת, בהירה או כהה
settings-appearance-sender-pictures-summary = לוגואים של חברות, מאותרים לפי הדומיין של השולח
settings-appearance-important-summary = סמן החשיבות ליד כל הודעה ברשימה
settings-appearance-mail-colors-summary = צבעים כהים לדואר HTML בערכת נושא כהה, או צבעי השולח
settings-appearance-attachment-previews-summary = תמונה קטנה של תוכן כל קובץ מצורף
settings-shortcuts-set-summary = התחלה מהמקשים של Gmail, Inbox by Gmail, Apple Mail, Outlook או Thunderbird
settings-shortcuts-single-summary = מקשים בלי Ctrl או Alt, כמו בדואר אינטרנט
settings-default-apps-pdf-summary = איפה נפתחים קבצים מצורפים מסוג PDF
settings-default-apps-pictures-summary = איפה נפתחים צילומים ותמונות
settings-default-apps-text-summary = איפה נפתחים טקסט פשוט, יומנים וקוד
settings-default-apps-sheets-summary = איפה נפתחים קובצי Excel, OpenDocument ו־CSV
settings-default-apps-documents-summary = איפה נפתחים Word, טקסט OpenDocument ומצגות
settings-default-apps-after-saving-summary = הצגת קבצים מצורפים שנשמרו בתיקייה שלהם
settings-compose-send-from-summary = החשבון שממנו יוצא דואר חדש: החשבון שבו נמצאים, או תמיד אותו חשבון
settings-compose-send-on-replies-summary = „שליחה” או „שליחה והעברה לארכיון” של השיחה, בתשובות ובהעברות
settings-compose-signatures-summary = נוספת מתחת להודעה, אחרי שורת „--”
settings-compose-for-new-mail-summary = החתימה שבה מתחיל דואר חדש
settings-compose-for-replies-summary = החתימה שבה מתחילות תשובות והעברות
settings-compose-format-summary = כתיבת דואר חדש בטקסט פשוט
settings-compose-spelling-summary = בדיקת איות בזמן הכתיבה, ושפת המילון
settings-compose-templates-summary = בקרוב: שמירת הודעות שכותבים לעיתים קרובות, והתחלת הודעה חדשה או תשובה מהן
settings-feedback-crash-reports-summary = שמירת דוחות קריסה במחשב הזה כש־Katna Mail או שירות הרקע שלה קורסים
settings-feedback-saved-summary = הצגה, העתקה או מחיקה של דוחות הקריסה שנשמרו במחשב הזה
settings-feedback-help-improve-summary = שליחת דוחות קריסה כדי לעזור לתקן את מה שהשתבש; כבוי אלא אם מפעילים אותו
settings-experimental-blur-summary = שולחן העבודה נראה מבעד לסרגל העליון, מטושטש, והתפריטים כמו זכוכית חלבית
settings-search-shortcut = קיצור מקלדת
settings-search-tab = כרטיסיית הגדרות
settings-search-none = אין הגדרות שתואמות את „{ $query }”.
settings-search-results = הגדרות שתואמות את „{ $query }”

## Settings: opening at login

settings-open-at-login-failed = לא ניתן היה לשנות את ההפעלה בכניסה למערכת: { $error }

## Settings > General > Time

settings-time = שעה
settings-clock-language = כפי שנהוג בשפה
settings-clock-12 = 12 שעות, למשל 2:05 PM
settings-clock-24 = 24 שעות, למשל 14:05
settings-time-summary = שעון של 12 או 24 שעות, או כפי שנהוג בשפה

## Settings > General > Default mail app, Settings > Compose > Grammar

settings-general-mail-app = אפליקציית דואר ברירת מחדל
settings-general-mail-app-detail = קישורי אימייל באפליקציות אחרות ובאתרים פותחים כאן הודעה חדשה.
mail-app-is-default = Katna Mail היא אפליקציית הדואר שלך כברירת מחדל.
mail-app-is-other = קישורי אימייל נפתחים באפליקציה אחרת.
mail-app-make-default = הגדרה כברירת מחדל
mail-app-make-default-failed = לא ניתן היה לשנות את אפליקציית הדואר שמוגדרת כברירת מחדל.
settings-general-mail-app-summary = פתיחת קישורי אימייל מאפליקציות ומאתרים אחרים ב־Katna Mail
settings-compose-grammar = דקדוק
settings-compose-grammar-detail = הבדיקה נעשית במחשב הזה עם Harper. בינתיים רק באנגלית: טקסט בשפות אחרות נשאר כמו שהוא.
settings-compose-grammar-check = בדיקת דקדוק
settings-compose-grammar-check-detail = סימון שגיאות דקדוק בקו תחתון בזמן הכתיבה, באנגלית
settings-compose-suggestions = הצעות כתיבה
settings-compose-suggestions-detail = נלמדות במחשב הזה מהדואר שנשלח ממנו ומהדואר שעונים עליו; שום דבר לא יוצא ממנו. יש ללחוץ על Tab כדי לקבל הצעה, או פשוט להמשיך להקליד.
settings-compose-suggestions-on = הצעות בזמן הכתיבה
settings-compose-suggestions-on-detail = הצגת ההמשך הסביר של הביטוי באפור בזמן ההקלדה
settings-compose-grammar-summary = סימון שגיאות דקדוק בקו תחתון בזמן הכתיבה, באנגלית
settings-compose-suggestions-summary = הצגת ההמשך הסביר של הביטוי באפור בזמן ההקלדה
