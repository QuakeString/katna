# Katna Mail, Hebrew (עברית).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.
compose-ai-rephrase-tip = ניסוח מחדש (Ctrl+J)
compose-ai-tone-clearer = ברור יותר
compose-ai-tone-shorter = קצר יותר
compose-ai-tone-friendlier = ידידותי יותר
compose-ai-tone-formal = רשמי
compose-ai-tone-grammar = תיקון דקדוק
compose-ai-tone-longer = ארוך יותר
compose-ai-custom = להסביר איך…
compose-ai-more = דרכים נוספות
compose-ai-replace = החלפה
compose-ai-again = ניסיון נוסף
compose-ai-below = הוספה מתחת
compose-ai-copy = העתקה
compose-ai-cancel = ביטול
compose-ai-rephrase = ניסוח מחדש
compose-ai-replaced = נוסח מחדש
compose-ai-added = נוסף מתחת
compose-ai-copied = הועתק
compose-ai-katna = Katna AI
compose-ai-own = שירות ה־AI שלך
compose-ai-trial-left = { $service } · { $days ->
    [one] נשאר יום אחד בחינם
    [two] נשארו יומיים בחינם
   *[other] נשארו { $days } ימים בחינם
}
compose-ai-encrypted = ההודעה הזו תוצפן. ניסוח מחדש שולח את הטקסט שנבחר אל { $service } ללא הצפנה. לנסח מחדש בכל זאת?
compose-ai-sign-in = Katna AI דורש חשבון Katna. אפשר להתחבר כדי להשתמש בו, או להשתמש במפתח משלך.
compose-ai-pay = החודש החינמי שלך ב־Katna AI הסתיים. המחיר הוא 5$ לחודש, או שאפשר להשתמש במפתח משלך.
compose-ai-too-many = יותר מדי בקשות כרגע. כדאי לנסות שוב בעוד זמן מה.
compose-ai-no-key = כדי לנסח מחדש, יש להוסיף את המפתח של { $service } בהגדרות.
compose-ai-bad-key = { $service } לא קיבל את המפתח שלך. כדאי לבדוק אותו בהגדרות.
compose-ai-off = עזרה בכתיבה עם AI כבויה בהגדרות.
compose-ai-failed = אין גישה אל { $service }. כדאי לנסות שוב.
compose-ai-try-again = ניסיון נוסף
compose-ai-open-settings = פתיחת ההגדרות
compose-ai-write-reply-tip = כתיבת תשובה (Ctrl+J)
compose-ai-write-note-tip = כתיבת הערה (Ctrl+J)
compose-ai-rephrase-empty-tip = יש להקליד משהו כדי לנסח אותו מחדש
compose-ai-write-reply = כתיבת תשובה
compose-ai-write-note = כתיבת הערה
compose-ai-write-from = { $count ->
    [one] מהודעה אחת
    [two] מ־{ $count } הודעות
   *[other] מ־{ $count } הודעות
}
compose-ai-write-ideas = רעיונות מהשיחה
compose-ai-write-own = או לכתוב מה התשובה צריכה לומר…
compose-ai-write-short = קצר
compose-ai-write-longer = ארוך יותר
compose-ai-write-friendly = ידידותי
compose-ai-write-formal = רשמי
compose-ai-write-insert = הוספה
compose-ai-write-back = רעיונות אחרים
compose-ai-written = הטיוטה נוספה
compose-ai-write-encrypted = השיחה הזו מוצפנת. כתיבת תשובה שולחת את ההודעות שלה אל { $service } ללא הצפנה. לכתוב בכל זאת?
compose-ai-write-anyway = כתיבה
compose-ai-write-encrypted-off = השיחה הזו מוצפנת, וההגדרות לא מאפשרות עזרה בכתיבה בדואר מוצפן.
compose-ai-subject-tip = ניסוח מחדש של הנושא
compose-ai-subject-title = דרכים אחרות לומר זאת
compose-ai-subject-done = הנושא שונה

## Summing up a conversation: the list's right-click menu, the reading

## pane's sparkle, the chat's strip and the card each opens.

summary-summarize = סיכום
summary-hide = הסתרת הסיכום
summary-close = סגירה
summary-fold = כיווץ
summary-title = סיכום
summary-mails = { $count ->
    [one] הודעה אחת
    [two] { $count } הודעות
   *[other] { $count } הודעות
}
summary-of-mails = { $count } מתוך { $total } הודעות
summary-peek-count = { $mails ->
    [one] הודעה אחת
    [two] { $mails } הודעות
   *[other] { $mails } הודעות
} · { $people ->
    [one] אדם אחד
    [two] { $people } אנשים
   *[other] { $people } אנשים
}
summary-catch-up = { $count ->
    [one] אחת חדשה מאז הקריאה האחרונה שלך
    [two] { $count } חדשות מאז הקריאה האחרונה שלך
   *[other] { $count } חדשות מאז הקריאה האחרונה שלך
}
summary-strip-newer = { $count ->
    [one] אחת חדשה מאז · { $gist }
    [two] { $count } חדשות מאז · { $gist }
   *[other] { $count } חדשות מאז · { $gist }
}
summary-add-new = { $count ->
    [one] הוספת החדשה
    [two] הוספת { $count } החדשות
   *[other] הוספת { $count } החדשות
}
summary-point-settled = הוחלט
summary-point-money = כסף
summary-point-dates = תאריכים
summary-point-next = הצעד הבא
summary-point-open = פתוח
summary-files = קבצים
summary-for-you = בשבילך
summary-from-mail = { $name }, { $date }
summary-you = אני
summary-made = { $service } · { $time }
summary-not-read = { $service } · לא סומן כנקרא
summary-copy = העתקה
summary-copied = הסיכום הועתק
summary-again = סיכום מחדש
summary-open = פתיחה
summary-open-tip = פתיחת השיחה
summary-reply = תשובה
summary-reply-tip = כתיבת תשובה עם AI
summary-reply-to = תשובה ל־{ $name }
summary-reply-summary = סיכום
summary-reply-send = שליחה
summary-reply-open = פתיחה
summary-asking = שואלים את { $service }…
summary-stop = עצירה
summary-cancel = ביטול
summary-send = שליחה וסיכום
summary-ask-short = ממתין לאישור שלך
summary-encrypted = השיחה הזו מוצפנת. סיכום שולח את הטקסט שלה אל { $service } ללא הצפנה.
summary-encrypted-off = השיחה הזו מוצפנת, וההגדרות לא מאפשרות עזרה בכתיבה בדואר מוצפן.
summary-try-again = ניסיון נוסף
summary-open-settings = פתיחת ההגדרות
