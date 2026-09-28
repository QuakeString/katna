# Katna Mail, Hebrew (עברית).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = ראשי
tab-promotions = קידומי מכירות
tab-social = רשתות חברתיות
tab-updates = עדכונים
tab-forums = פורומים
tab-focused = ממוקד
tab-other = אחר
tab-inbox = דואר נכנס
tab-newsletters = ניוזלטרים
tab-notifications = התראות
tab-new = { $count } חדשות
tab-provider-other = ממוין על ידי Katna

## Mail list: toolbar

list-select = בחירה
list-refresh = רענון
list-more = עוד
list-mark-read = סימון כנקראו
list-mark-unread = סימון כלא נקראו
list-move-to = העברה אל
list-archive = העברה לארכיון
list-spam = דיווח על ספאם
list-delete = מחיקה
list-snooze = השהיה
list-unsnooze = ביטול ההשהיה
list-newer = חדשות יותר
list-older = ישנות יותר
list-range = { $first }–{ $last } מתוך { $total }
list-range-about = { $first }–{ $last } מתוך כ־{ $total }
list-results = תוצאות עבור „{ $query }”
list-results-corrected = מוצגות תוצאות עבור „{ $query }”
list-search-instead = חיפוש „{ $query }” במקום זאת
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = הכול
list-pick-none = ללא
list-pick-read = נקראו
list-pick-unread = לא נקראו
list-pick-starred = מסומנות בכוכב
list-pick-unstarred = לא מסומנות בכוכב

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] נבחרה שיחה אחת.
        [two] כל { $count } השיחות נבחרו.
       *[other] כל { $count } השיחות נבחרו.
    }
   *[message] { $count ->
        [one] נבחרה הודעה אחת.
        [two] כל { $count } ההודעות נבחרו.
       *[other] כל { $count } ההודעות נבחרו.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] נבחרה שיחה אחת ב־{ $folder }.
        [two] כל { $count } השיחות ב־{ $folder } נבחרו.
       *[other] כל { $count } השיחות ב־{ $folder } נבחרו.
    }
   *[message] { $count ->
        [one] נבחרה הודעה אחת ב־{ $folder }.
        [two] כל { $count } ההודעות ב־{ $folder } נבחרו.
       *[other] כל { $count } ההודעות ב־{ $folder } נבחרו.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] נבחרה השיחה היחידה שבמסך.
        [two] כל { $count } השיחות שבמסך נבחרו.
       *[other] כל { $count } השיחות שבמסך נבחרו.
    }
   *[message] { $count ->
        [one] נבחרה ההודעה היחידה שבמסך.
        [two] כל { $count } ההודעות שבמסך נבחרו.
       *[other] כל { $count } ההודעות שבמסך נבחרו.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] בחירת שיחה אחת
        [two] בחירת כל { $count } השיחות
       *[other] בחירת כל { $count } השיחות
    }
   *[message] { $count ->
        [one] בחירת הודעה אחת
        [two] בחירת כל { $count } ההודעות
       *[other] בחירת כל { $count } ההודעות
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] בחירת שיחה אחת ב־{ $folder }
        [two] בחירת כל { $count } השיחות ב־{ $folder }
       *[other] בחירת כל { $count } השיחות ב־{ $folder }
    }
   *[message] { $count ->
        [one] בחירת הודעה אחת ב־{ $folder }
        [two] בחירת כל { $count } ההודעות ב־{ $folder }
       *[other] בחירת כל { $count } ההודעות ב־{ $folder }
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] במסך נבחרה שיחה אחת שנקראה.
            [two] במסך נבחרו כל { $count } השיחות שנקראו.
           *[other] במסך נבחרו כל { $count } השיחות שנקראו.
        }
       *[message] { $count ->
            [one] במסך נבחרה הודעה אחת שנקראה.
            [two] במסך נבחרו כל { $count } ההודעות שנקראו.
           *[other] במסך נבחרו כל { $count } ההודעות שנקראו.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] במסך נבחרה שיחה אחת שלא נקראה.
            [two] במסך נבחרו כל { $count } השיחות שלא נקראו.
           *[other] במסך נבחרו כל { $count } השיחות שלא נקראו.
        }
       *[message] { $count ->
            [one] במסך נבחרה הודעה אחת שלא נקראה.
            [two] במסך נבחרו כל { $count } ההודעות שלא נקראו.
           *[other] במסך נבחרו כל { $count } ההודעות שלא נקראו.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] במסך נבחרה שיחה אחת המסומנת בכוכב.
            [two] במסך נבחרו כל { $count } השיחות המסומנות בכוכב.
           *[other] במסך נבחרו כל { $count } השיחות המסומנות בכוכב.
        }
       *[message] { $count ->
            [one] במסך נבחרה הודעה אחת המסומנת בכוכב.
            [two] במסך נבחרו כל { $count } ההודעות המסומנות בכוכב.
           *[other] במסך נבחרו כל { $count } ההודעות המסומנות בכוכב.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] במסך נבחרה שיחה אחת שאינה מסומנת בכוכב.
            [two] במסך נבחרו כל { $count } השיחות שאינן מסומנות בכוכב.
           *[other] במסך נבחרו כל { $count } השיחות שאינן מסומנות בכוכב.
        }
       *[message] { $count ->
            [one] במסך נבחרה הודעה אחת שאינה מסומנת בכוכב.
            [two] במסך נבחרו כל { $count } ההודעות שאינן מסומנות בכוכב.
           *[other] במסך נבחרו כל { $count } ההודעות שאינן מסומנות בכוכב.
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] בחירת שיחה אחת שנקראה
            [two] בחירת כל { $count } השיחות שנקראו
           *[other] בחירת כל { $count } השיחות שנקראו
        }
       *[message] { $count ->
            [one] בחירת הודעה אחת שנקראה
            [two] בחירת כל { $count } ההודעות שנקראו
           *[other] בחירת כל { $count } ההודעות שנקראו
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] בחירת שיחה אחת שלא נקראה
            [two] בחירת כל { $count } השיחות שלא נקראו
           *[other] בחירת כל { $count } השיחות שלא נקראו
        }
       *[message] { $count ->
            [one] בחירת הודעה אחת שלא נקראה
            [two] בחירת כל { $count } ההודעות שלא נקראו
           *[other] בחירת כל { $count } ההודעות שלא נקראו
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] בחירת שיחה אחת המסומנת בכוכב
            [two] בחירת כל { $count } השיחות המסומנות בכוכב
           *[other] בחירת כל { $count } השיחות המסומנות בכוכב
        }
       *[message] { $count ->
            [one] בחירת הודעה אחת המסומנת בכוכב
            [two] בחירת כל { $count } ההודעות המסומנות בכוכב
           *[other] בחירת כל { $count } ההודעות המסומנות בכוכב
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] בחירת שיחה אחת שאינה מסומנת בכוכב
            [two] בחירת כל { $count } השיחות שאינן מסומנות בכוכב
           *[other] בחירת כל { $count } השיחות שאינן מסומנות בכוכב
        }
       *[message] { $count ->
            [one] בחירת הודעה אחת שאינה מסומנת בכוכב
            [two] בחירת כל { $count } ההודעות שאינן מסומנות בכוכב
           *[other] בחירת כל { $count } ההודעות שאינן מסומנות בכוכב
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] בחירת שיחה אחת שנקראה ב־{ $folder }
            [two] בחירת כל { $count } השיחות שנקראו ב־{ $folder }
           *[other] בחירת כל { $count } השיחות שנקראו ב־{ $folder }
        }
       *[message] { $count ->
            [one] בחירת הודעה אחת שנקראה ב־{ $folder }
            [two] בחירת כל { $count } ההודעות שנקראו ב־{ $folder }
           *[other] בחירת כל { $count } ההודעות שנקראו ב־{ $folder }
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] בחירת שיחה אחת שלא נקראה ב־{ $folder }
            [two] בחירת כל { $count } השיחות שלא נקראו ב־{ $folder }
           *[other] בחירת כל { $count } השיחות שלא נקראו ב־{ $folder }
        }
       *[message] { $count ->
            [one] בחירת הודעה אחת שלא נקראה ב־{ $folder }
            [two] בחירת כל { $count } ההודעות שלא נקראו ב־{ $folder }
           *[other] בחירת כל { $count } ההודעות שלא נקראו ב־{ $folder }
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] בחירת שיחה אחת המסומנת בכוכב ב־{ $folder }
            [two] בחירת כל { $count } השיחות המסומנות בכוכב ב־{ $folder }
           *[other] בחירת כל { $count } השיחות המסומנות בכוכב ב־{ $folder }
        }
       *[message] { $count ->
            [one] בחירת הודעה אחת המסומנת בכוכב ב־{ $folder }
            [two] בחירת כל { $count } ההודעות המסומנות בכוכב ב־{ $folder }
           *[other] בחירת כל { $count } ההודעות המסומנות בכוכב ב־{ $folder }
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] בחירת שיחה אחת שאינה מסומנת בכוכב ב־{ $folder }
            [two] בחירת כל { $count } השיחות שאינן מסומנות בכוכב ב־{ $folder }
           *[other] בחירת כל { $count } השיחות שאינן מסומנות בכוכב ב־{ $folder }
        }
       *[message] { $count ->
            [one] בחירת הודעה אחת שאינה מסומנת בכוכב ב־{ $folder }
            [two] בחירת כל { $count } ההודעות שאינן מסומנות בכוכב ב־{ $folder }
           *[other] בחירת כל { $count } ההודעות שאינן מסומנות בכוכב ב־{ $folder }
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] נבחרה שיחה אחת שנקראה.
            [two] כל { $count } השיחות שנקראו נבחרו.
           *[other] כל { $count } השיחות שנקראו נבחרו.
        }
       *[message] { $count ->
            [one] נבחרה הודעה אחת שנקראה.
            [two] כל { $count } ההודעות שנקראו נבחרו.
           *[other] כל { $count } ההודעות שנקראו נבחרו.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] נבחרה שיחה אחת שלא נקראה.
            [two] כל { $count } השיחות שלא נקראו נבחרו.
           *[other] כל { $count } השיחות שלא נקראו נבחרו.
        }
       *[message] { $count ->
            [one] נבחרה הודעה אחת שלא נקראה.
            [two] כל { $count } ההודעות שלא נקראו נבחרו.
           *[other] כל { $count } ההודעות שלא נקראו נבחרו.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] נבחרה שיחה אחת המסומנת בכוכב.
            [two] כל { $count } השיחות המסומנות בכוכב נבחרו.
           *[other] כל { $count } השיחות המסומנות בכוכב נבחרו.
        }
       *[message] { $count ->
            [one] נבחרה הודעה אחת המסומנת בכוכב.
            [two] כל { $count } ההודעות המסומנות בכוכב נבחרו.
           *[other] כל { $count } ההודעות המסומנות בכוכב נבחרו.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] נבחרה שיחה אחת שאינה מסומנת בכוכב.
            [two] כל { $count } השיחות שאינן מסומנות בכוכב נבחרו.
           *[other] כל { $count } השיחות שאינן מסומנות בכוכב נבחרו.
        }
       *[message] { $count ->
            [one] נבחרה הודעה אחת שאינה מסומנת בכוכב.
            [two] כל { $count } ההודעות שאינן מסומנות בכוכב נבחרו.
           *[other] כל { $count } ההודעות שאינן מסומנות בכוכב נבחרו.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] נבחרה שיחה אחת שנקראה ב־{ $folder }.
            [two] כל { $count } השיחות שנקראו ב־{ $folder } נבחרו.
           *[other] כל { $count } השיחות שנקראו ב־{ $folder } נבחרו.
        }
       *[message] { $count ->
            [one] נבחרה הודעה אחת שנקראה ב־{ $folder }.
            [two] כל { $count } ההודעות שנקראו ב־{ $folder } נבחרו.
           *[other] כל { $count } ההודעות שנקראו ב־{ $folder } נבחרו.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] נבחרה שיחה אחת שלא נקראה ב־{ $folder }.
            [two] כל { $count } השיחות שלא נקראו ב־{ $folder } נבחרו.
           *[other] כל { $count } השיחות שלא נקראו ב־{ $folder } נבחרו.
        }
       *[message] { $count ->
            [one] נבחרה הודעה אחת שלא נקראה ב־{ $folder }.
            [two] כל { $count } ההודעות שלא נקראו ב־{ $folder } נבחרו.
           *[other] כל { $count } ההודעות שלא נקראו ב־{ $folder } נבחרו.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] נבחרה שיחה אחת המסומנת בכוכב ב־{ $folder }.
            [two] כל { $count } השיחות המסומנות בכוכב ב־{ $folder } נבחרו.
           *[other] כל { $count } השיחות המסומנות בכוכב ב־{ $folder } נבחרו.
        }
       *[message] { $count ->
            [one] נבחרה הודעה אחת המסומנת בכוכב ב־{ $folder }.
            [two] כל { $count } ההודעות המסומנות בכוכב ב־{ $folder } נבחרו.
           *[other] כל { $count } ההודעות המסומנות בכוכב ב־{ $folder } נבחרו.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] נבחרה שיחה אחת שאינה מסומנת בכוכב ב־{ $folder }.
            [two] כל { $count } השיחות שאינן מסומנות בכוכב ב־{ $folder } נבחרו.
           *[other] כל { $count } השיחות שאינן מסומנות בכוכב ב־{ $folder } נבחרו.
        }
       *[message] { $count ->
            [one] נבחרה הודעה אחת שאינה מסומנת בכוכב ב־{ $folder }.
            [two] כל { $count } ההודעות שאינן מסומנות בכוכב ב־{ $folder } נבחרו.
           *[other] כל { $count } ההודעות שאינן מסומנות בכוכב ב־{ $folder } נבחרו.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] אין כאן שיחות שנקראו.
       *[message] אין כאן הודעות שנקראו.
    }
   *[unread] { $kind ->
        [conversation] אין כאן שיחות שלא נקראו.
       *[message] אין כאן הודעות שלא נקראו.
    }
    [starred] { $kind ->
        [conversation] אין כאן שיחות מסומנות בכוכב.
       *[message] אין כאן הודעות מסומנות בכוכב.
    }
    [unstarred] { $kind ->
        [conversation] אין כאן שיחות שאינן מסומנות בכוכב.
       *[message] אין כאן הודעות שאינן מסומנות בכוכב.
    }
}
list-clear-selection = ניקוי הבחירה

## Mail list: empty states

list-empty-search = אין הודעות שתואמות לחיפוש.
list-empty-tab = אין דואר ב־{ $tab }.
list-empty-tab-unknown = אין דואר בכרטיסייה הזו.
list-empty-folder = אין הודעות ב־{ $folder }.
list-empty-folder-unknown = אין הודעות בתיקייה הזו.
list-first-sync = מביאים את הדואר שלך…
list-first-sync-detail = הוא יופיע כאן כשיגיע.

## Mail list: lines

row-removed = ההודעה הזו הוסרה.
row-starred = מסומנת בכוכב
row-not-starred = לא מסומנת בכוכב
row-important = חשובה. יש ללחוץ כדי לסמן כלא חשובה.
row-mark-important = סימון כחשובה
row-pinned = מוצמדת למעלה
row-tracking-none = במעקב. עדיין לא נפתחה
row-tracking-opened = נפתחה אצל { $opened } מתוך { $recipients }
row-tracking-clicked = נפתחה אצל { $opened } מתוך { $recipients }, קישור נפתח אצל { $clicked }
row-pin = הצמדה למעלה
row-unpin = ביטול ההצמדה
row-snoozed-until = מושהית עד { $when }

## Mail list: More menu and right-click menu

menu-reply = תשובה
menu-reply-all = תשובה לכולם
menu-forward = העברה
menu-archive = העברה לארכיון
menu-delete = מחיקה
menu-delete-forever = מחיקה לצמיתות
menu-move-to-inbox = העברה לדואר הנכנס
menu-spam = דיווח על ספאם
menu-not-spam = לא ספאם
menu-mark-read = סימון כנקראו
menu-mark-unread = סימון כלא נקראו
menu-mark-all-read = סימון של הכול כנקרא
menu-star = הוספת כוכב
menu-unstar = הסרת הכוכב
menu-important = סימון כחשובה
menu-not-important = סימון כלא חשובה
menu-pin = הצמדה למעלה
menu-unpin = ביטול ההצמדה
menu-snooze = השהיה
menu-unsnooze = ביטול ההשהיה
menu-print-all = הדפסת הכול
menu-new-window = פתיחה בחלון חדש
menu-move-to = העברה אל
menu-move-to-heading = העברה אל:
menu-find-from = חיפוש הודעות מאת { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] השיחה הועברה לארכיון.
        [two] { $count } שיחות הועברו לארכיון.
       *[other] { $count } שיחות הועברו לארכיון.
    }
   *[message] { $count ->
        [one] ההודעה הועברה לארכיון.
        [two] { $count } הודעות הועברו לארכיון.
       *[other] { $count } הודעות הועברו לארכיון.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] השיחה הועברה לאשפה.
        [two] { $count } שיחות הועברו לאשפה.
       *[other] { $count } שיחות הועברו לאשפה.
    }
   *[message] { $count ->
        [one] ההודעה הועברה לאשפה.
        [two] { $count } הודעות הועברו לאשפה.
       *[other] { $count } הודעות הועברו לאשפה.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] השיחה הועברה.
        [two] { $count } שיחות הועברו.
       *[other] { $count } שיחות הועברו.
    }
   *[message] { $count ->
        [one] ההודעה הועברה.
        [two] { $count } הודעות הועברו.
       *[other] { $count } הודעות הועברו.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] השיחה סומנה בכוכב.
        [two] { $count } שיחות סומנו בכוכב.
       *[other] { $count } שיחות סומנו בכוכב.
    }
   *[message] { $count ->
        [one] ההודעה סומנה בכוכב.
        [two] { $count } הודעות סומנו בכוכב.
       *[other] { $count } הודעות סומנו בכוכב.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] הכוכב הוסר מהשיחה.
        [two] הכוכב הוסר מ־{ $count } שיחות.
       *[other] הכוכב הוסר מ־{ $count } שיחות.
    }
   *[message] { $count ->
        [one] הכוכב הוסר מההודעה.
        [two] הכוכב הוסר מ־{ $count } הודעות.
       *[other] הכוכב הוסר מ־{ $count } הודעות.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] השיחה סומנה כחשובה.
        [two] { $count } שיחות סומנו כחשובות.
       *[other] { $count } שיחות סומנו כחשובות.
    }
   *[message] { $count ->
        [one] ההודעה סומנה כחשובה.
        [two] { $count } הודעות סומנו כחשובות.
       *[other] { $count } הודעות סומנו כחשובות.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] השיחה סומנה כלא חשובה.
        [two] { $count } שיחות סומנו כלא חשובות.
       *[other] { $count } שיחות סומנו כלא חשובות.
    }
   *[message] { $count ->
        [one] ההודעה סומנה כלא חשובה.
        [two] { $count } הודעות סומנו כלא חשובות.
       *[other] { $count } הודעות סומנו כלא חשובות.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] השיחה הוצמדה למעלה.
        [two] { $count } שיחות הוצמדו למעלה.
       *[other] { $count } שיחות הוצמדו למעלה.
    }
   *[message] { $count ->
        [one] ההודעה הוצמדה למעלה.
        [two] { $count } הודעות הוצמדו למעלה.
       *[other] { $count } הודעות הוצמדו למעלה.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] הצמדת השיחה בוטלה.
        [two] ההצמדה של { $count } שיחות בוטלה.
       *[other] ההצמדה של { $count } שיחות בוטלה.
    }
   *[message] { $count ->
        [one] הצמדת ההודעה בוטלה.
        [two] ההצמדה של { $count } הודעות בוטלה.
       *[other] ההצמדה של { $count } הודעות בוטלה.
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] השיחה הושהתה עד { $when }.
        [two] { $count } שיחות הושהו עד { $when }.
       *[other] { $count } שיחות הושהו עד { $when }.
    }
   *[message] { $count ->
        [one] ההודעה הושהתה עד { $when }.
        [two] { $count } הודעות הושהו עד { $when }.
       *[other] { $count } הודעות הושהו עד { $when }.
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] השיחה חזרה לדואר הנכנס.
        [two] { $count } שיחות חזרו לדואר הנכנס.
       *[other] { $count } שיחות חזרו לדואר הנכנס.
    }
   *[message] { $count ->
        [one] ההודעה חזרה לדואר הנכנס.
        [two] { $count } הודעות חזרו לדואר הנכנס.
       *[other] { $count } הודעות חזרו לדואר הנכנס.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] השיחה דווחה כספאם.
        [two] { $count } שיחות דווחו כספאם.
       *[other] { $count } שיחות דווחו כספאם.
    }
   *[message] { $count ->
        [one] ההודעה דווחה כספאם.
        [two] { $count } הודעות דווחו כספאם.
       *[other] { $count } הודעות דווחו כספאם.
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] השיחה סומנה כלא ספאם והועברה לדואר הנכנס.
        [two] { $count } שיחות סומנו כלא ספאם והועברו לדואר הנכנס.
       *[other] { $count } שיחות סומנו כלא ספאם והועברו לדואר הנכנס.
    }
   *[message] { $count ->
        [one] ההודעה סומנה כלא ספאם והועברה לדואר הנכנס.
        [two] { $count } הודעות סומנו כלא ספאם והועברו לדואר הנכנס.
       *[other] { $count } הודעות סומנו כלא ספאם והועברו לדואר הנכנס.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] השיחה נמחקה לצמיתות.
        [two] { $count } שיחות נמחקו לצמיתות.
       *[other] { $count } שיחות נמחקו לצמיתות.
    }
   *[message] { $count ->
        [one] ההודעה נמחקה לצמיתות.
        [two] { $count } הודעות נמחקו לצמיתות.
       *[other] { $count } הודעות נמחקו לצמיתות.
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] השיחה סומנה כנקראה.
        [two] { $count } שיחות סומנו כנקראו.
       *[other] { $count } שיחות סומנו כנקראו.
    }
   *[message] { $count ->
        [one] ההודעה סומנה כנקראה.
        [two] { $count } הודעות סומנו כנקראו.
       *[other] { $count } הודעות סומנו כנקראו.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] השיחה סומנה כלא נקראה.
        [two] { $count } שיחות סומנו כלא נקראו.
       *[other] { $count } שיחות סומנו כלא נקראו.
    }
   *[message] { $count ->
        [one] ההודעה סומנה כלא נקראה.
        [two] { $count } הודעות סומנו כלא נקראו.
       *[other] { $count } הודעות סומנו כלא נקראו.
    }
}
toast-undone = הפעולה בוטלה.
toast-nothing-to-undo = אין מה לבטל.
toast-cannot-undo-delete-forever = אי אפשר לשחזר דואר שנמחק לצמיתות.
toast-send-undone = השליחה בוטלה.
toast-too-late-to-undo-send = מאוחר מדי לבטל: ההודעה כבר נשלחה.
toast-undo = ביטול
toast-close = סגירה
toast-no-spam-folder = אין לחשבון הזה תיקיית ספאם.
