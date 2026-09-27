# Katna Mail, Sinhala (සිංහල).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = ප්‍රාථමික
tab-promotions = ප්‍රවර්ධන
tab-social = සමාජ
tab-updates = යාවත්කාලීන
tab-forums = සංසද
tab-focused = අවධානය යොමු කළ
tab-other = වෙනත්
tab-inbox = එන ලිපි
tab-newsletters = පුවත් හසුන්
tab-notifications = දැනුම්දීම්
tab-new = නව { $count }
tab-provider-other = Katna විසින් වර්ග කළ

## Mail list: toolbar

list-select = තෝරන්න
list-refresh = නැවුම් කරන්න
list-more = තවත්
list-mark-read = කියවූ ලෙස සලකුණු කරන්න
list-mark-unread = නොකියවූ ලෙස සලකුණු කරන්න
list-move-to = වෙත ගෙන යන්න
list-archive = සංරක්ෂණය කරන්න
list-spam = අයාචිත තැපැල් ලෙස වාර්තා කරන්න
list-delete = මකන්න
list-snooze = කල් දමන්න
list-unsnooze = කල් දැමීම ඉවත් කරන්න
list-newer = අලුත්
list-older = පැරණි
list-range = { $total } න් { $first }–{ $last }
list-range-about = ආසන්න වශයෙන් { $total } න් { $first }–{ $last }
list-results = “{ $query }” සඳහා ප්‍රතිඵල
list-results-corrected = “{ $query }” සඳහා ප්‍රතිඵල පෙන්වමින්
list-search-instead = ඒ වෙනුවට “{ $query }” සොයන්න
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = සියල්ල
list-pick-none = කිසිවක් නැත
list-pick-read = කියවූ
list-pick-unread = නොකියවූ
list-pick-starred = තරු යෙදූ
list-pick-unstarred = තරු නොයෙදූ

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] සංවාද { $count } ම තෝරා ඇත.
       *[other] සංවාද { $count } ම තෝරා ඇත.
    }
   *[message] { $count ->
        [one] පණිවිඩ { $count } ම තෝරා ඇත.
       *[other] පණිවිඩ { $count } ම තෝරා ඇත.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } හි සංවාද { $count } ම තෝරා ඇත.
       *[other] { $folder } හි සංවාද { $count } ම තෝරා ඇත.
    }
   *[message] { $count ->
        [one] { $folder } හි පණිවිඩ { $count } ම තෝරා ඇත.
       *[other] { $folder } හි පණිවිඩ { $count } ම තෝරා ඇත.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] තිරයේ ඇති සංවාද { $count } ම තෝරා ඇත.
       *[other] තිරයේ ඇති සංවාද { $count } ම තෝරා ඇත.
    }
   *[message] { $count ->
        [one] තිරයේ ඇති පණිවිඩ { $count } ම තෝරා ඇත.
       *[other] තිරයේ ඇති පණිවිඩ { $count } ම තෝරා ඇත.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] සංවාද { $count } ම තෝරන්න
       *[other] සංවාද { $count } ම තෝරන්න
    }
   *[message] { $count ->
        [one] පණිවිඩ { $count } ම තෝරන්න
       *[other] පණිවිඩ { $count } ම තෝරන්න
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } හි සංවාද { $count } ම තෝරන්න
       *[other] { $folder } හි සංවාද { $count } ම තෝරන්න
    }
   *[message] { $count ->
        [one] { $folder } හි පණිවිඩ { $count } ම තෝරන්න
       *[other] { $folder } හි පණිවිඩ { $count } ම තෝරන්න
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] තිරයේ ඇති කියවූ සංවාද { $count } තෝරා ඇත.
           *[other] තිරයේ ඇති කියවූ සංවාද { $count } ම තෝරා ඇත.
        }
       *[message] { $count ->
            [one] තිරයේ ඇති කියවූ පණිවිඩ { $count } තෝරා ඇත.
           *[other] තිරයේ ඇති කියවූ පණිවිඩ { $count } ම තෝරා ඇත.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] තිරයේ ඇති නොකියවූ සංවාද { $count } තෝරා ඇත.
           *[other] තිරයේ ඇති නොකියවූ සංවාද { $count } ම තෝරා ඇත.
        }
       *[message] { $count ->
            [one] තිරයේ ඇති නොකියවූ පණිවිඩ { $count } තෝරා ඇත.
           *[other] තිරයේ ඇති නොකියවූ පණිවිඩ { $count } ම තෝරා ඇත.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] තිරයේ ඇති තරු යෙදූ සංවාද { $count } තෝරා ඇත.
           *[other] තිරයේ ඇති තරු යෙදූ සංවාද { $count } ම තෝරා ඇත.
        }
       *[message] { $count ->
            [one] තිරයේ ඇති තරු යෙදූ පණිවිඩ { $count } තෝරා ඇත.
           *[other] තිරයේ ඇති තරු යෙදූ පණිවිඩ { $count } ම තෝරා ඇත.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] තිරයේ ඇති තරු නොයෙදූ සංවාද { $count } තෝරා ඇත.
           *[other] තිරයේ ඇති තරු නොයෙදූ සංවාද { $count } ම තෝරා ඇත.
        }
       *[message] { $count ->
            [one] තිරයේ ඇති තරු නොයෙදූ පණිවිඩ { $count } තෝරා ඇත.
           *[other] තිරයේ ඇති තරු නොයෙදූ පණිවිඩ { $count } ම තෝරා ඇත.
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] කියවූ සංවාද { $count } තෝරන්න
           *[other] කියවූ සංවාද { $count } ම තෝරන්න
        }
       *[message] { $count ->
            [one] කියවූ පණිවිඩ { $count } තෝරන්න
           *[other] කියවූ පණිවිඩ { $count } ම තෝරන්න
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] නොකියවූ සංවාද { $count } තෝරන්න
           *[other] නොකියවූ සංවාද { $count } ම තෝරන්න
        }
       *[message] { $count ->
            [one] නොකියවූ පණිවිඩ { $count } තෝරන්න
           *[other] නොකියවූ පණිවිඩ { $count } ම තෝරන්න
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] තරු යෙදූ සංවාද { $count } තෝරන්න
           *[other] තරු යෙදූ සංවාද { $count } ම තෝරන්න
        }
       *[message] { $count ->
            [one] තරු යෙදූ පණිවිඩ { $count } තෝරන්න
           *[other] තරු යෙදූ පණිවිඩ { $count } ම තෝරන්න
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] තරු නොයෙදූ සංවාද { $count } තෝරන්න
           *[other] තරු නොයෙදූ සංවාද { $count } ම තෝරන්න
        }
       *[message] { $count ->
            [one] තරු නොයෙදූ පණිවිඩ { $count } තෝරන්න
           *[other] තරු නොයෙදූ පණිවිඩ { $count } ම තෝරන්න
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder } හි කියවූ සංවාද { $count } තෝරන්න
           *[other] { $folder } හි කියවූ සංවාද { $count } ම තෝරන්න
        }
       *[message] { $count ->
            [one] { $folder } හි කියවූ පණිවිඩ { $count } තෝරන්න
           *[other] { $folder } හි කියවූ පණිවිඩ { $count } ම තෝරන්න
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder } හි නොකියවූ සංවාද { $count } තෝරන්න
           *[other] { $folder } හි නොකියවූ සංවාද { $count } ම තෝරන්න
        }
       *[message] { $count ->
            [one] { $folder } හි නොකියවූ පණිවිඩ { $count } තෝරන්න
           *[other] { $folder } හි නොකියවූ පණිවිඩ { $count } ම තෝරන්න
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } හි තරු යෙදූ සංවාද { $count } තෝරන්න
           *[other] { $folder } හි තරු යෙදූ සංවාද { $count } ම තෝරන්න
        }
       *[message] { $count ->
            [one] { $folder } හි තරු යෙදූ පණිවිඩ { $count } තෝරන්න
           *[other] { $folder } හි තරු යෙදූ පණිවිඩ { $count } ම තෝරන්න
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } හි තරු නොයෙදූ සංවාද { $count } තෝරන්න
           *[other] { $folder } හි තරු නොයෙදූ සංවාද { $count } ම තෝරන්න
        }
       *[message] { $count ->
            [one] { $folder } හි තරු නොයෙදූ පණිවිඩ { $count } තෝරන්න
           *[other] { $folder } හි තරු නොයෙදූ පණිවිඩ { $count } ම තෝරන්න
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] කියවූ සංවාද { $count } ක් තෝරා ඇත.
           *[other] කියවූ සංවාද { $count } ම තෝරා ඇත.
        }
       *[message] { $count ->
            [one] කියවූ පණිවිඩ { $count } ක් තෝරා ඇත.
           *[other] කියවූ පණිවිඩ { $count } ම තෝරා ඇත.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] නොකියවූ සංවාද { $count } ක් තෝරා ඇත.
           *[other] නොකියවූ සංවාද { $count } ම තෝරා ඇත.
        }
       *[message] { $count ->
            [one] නොකියවූ පණිවිඩ { $count } ක් තෝරා ඇත.
           *[other] නොකියවූ පණිවිඩ { $count } ම තෝරා ඇත.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] තරු යෙදූ සංවාද { $count } ක් තෝරා ඇත.
           *[other] තරු යෙදූ සංවාද { $count } ම තෝරා ඇත.
        }
       *[message] { $count ->
            [one] තරු යෙදූ පණිවිඩ { $count } ක් තෝරා ඇත.
           *[other] තරු යෙදූ පණිවිඩ { $count } ම තෝරා ඇත.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] තරු නොයෙදූ සංවාද { $count } ක් තෝරා ඇත.
           *[other] තරු නොයෙදූ සංවාද { $count } ම තෝරා ඇත.
        }
       *[message] { $count ->
            [one] තරු නොයෙදූ පණිවිඩ { $count } ක් තෝරා ඇත.
           *[other] තරු නොයෙදූ පණිවිඩ { $count } ම තෝරා ඇත.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder } හි කියවූ සංවාද { $count } ක් තෝරා ඇත.
           *[other] { $folder } හි කියවූ සංවාද { $count } ම තෝරා ඇත.
        }
       *[message] { $count ->
            [one] { $folder } හි කියවූ පණිවිඩ { $count } ක් තෝරා ඇත.
           *[other] { $folder } හි කියවූ පණිවිඩ { $count } ම තෝරා ඇත.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder } හි නොකියවූ සංවාද { $count } ක් තෝරා ඇත.
           *[other] { $folder } හි නොකියවූ සංවාද { $count } ම තෝරා ඇත.
        }
       *[message] { $count ->
            [one] { $folder } හි නොකියවූ පණිවිඩ { $count } ක් තෝරා ඇත.
           *[other] { $folder } හි නොකියවූ පණිවිඩ { $count } ම තෝරා ඇත.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } හි තරු යෙදූ සංවාද { $count } ක් තෝරා ඇත.
           *[other] { $folder } හි තරු යෙදූ සංවාද { $count } ම තෝරා ඇත.
        }
       *[message] { $count ->
            [one] { $folder } හි තරු යෙදූ පණිවිඩ { $count } ක් තෝරා ඇත.
           *[other] { $folder } හි තරු යෙදූ පණිවිඩ { $count } ම තෝරා ඇත.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } හි තරු නොයෙදූ සංවාද { $count } ක් තෝරා ඇත.
           *[other] { $folder } හි තරු නොයෙදූ සංවාද { $count } ම තෝරා ඇත.
        }
       *[message] { $count ->
            [one] { $folder } හි තරු නොයෙදූ පණිවිඩ { $count } ක් තෝරා ඇත.
           *[other] { $folder } හි තරු නොයෙදූ පණිවිඩ { $count } ම තෝරා ඇත.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] මෙහි කියවූ සංවාද නැත.
       *[message] මෙහි කියවූ පණිවිඩ නැත.
    }
   *[unread] { $kind ->
        [conversation] මෙහි නොකියවූ සංවාද නැත.
       *[message] මෙහි නොකියවූ පණිවිඩ නැත.
    }
    [starred] { $kind ->
        [conversation] මෙහි තරු යෙදූ සංවාද නැත.
       *[message] මෙහි තරු යෙදූ පණිවිඩ නැත.
    }
    [unstarred] { $kind ->
        [conversation] මෙහි තරු නොයෙදූ සංවාද නැත.
       *[message] මෙහි තරු නොයෙදූ පණිවිඩ නැත.
    }
}
list-clear-selection = තේරීම හිස් කරන්න

## Mail list: empty states

list-empty-search = ඔබේ සෙවීමට ගැළපෙන පණිවිඩ නැත.
list-empty-tab = { $tab } හි තැපැල් නැත.
list-empty-tab-unknown = මෙම ටැබයේ තැපැල් නැත.
list-empty-folder = { $folder } හි පණිවිඩ නැත.
list-empty-folder-unknown = මෙම ෆෝල්ඩරයේ පණිවිඩ නැත.
list-first-sync = ඔබේ තැපැල් ලබා ගනිමින්…
list-first-sync-detail = ඒවා ලැබෙන විට මෙහි පෙන්වයි.

## Mail list: lines

row-removed = මෙම පණිවිඩය ඉවත් කරන ලදී.
row-starred = තරු යෙදූ
row-not-starred = තරු නොයෙදූ
row-important = වැදගත්. වැදගත් නොවන ලෙස සලකුණු කිරීමට ක්ලික් කරන්න.
row-mark-important = වැදගත් ලෙස සලකුණු කරන්න
row-pinned = ඉහළට ඇමිණූ
row-tracking-none = ලුහුබඳිනවා. තවම විවෘත කර නැත
row-tracking-opened = { $recipients } දෙනෙකුගෙන් { $opened } දෙනෙකු විවෘත කළා
row-tracking-clicked = { $recipients } දෙනෙකුගෙන් { $opened } දෙනෙකු විවෘත කළා, { $clicked } දෙනෙකු සබැඳියක් විවෘත කළා
row-pin = ඉහළට අමුණන්න
row-unpin = ඇමිණීම ඉවත් කරන්න
row-snoozed-until = { $when } දක්වා කල් දමා ඇත

## Mail list: More menu and right-click menu

menu-reply = පිළිතුරු දෙන්න
menu-reply-all = සියල්ලන්ට පිළිතුරු දෙන්න
menu-forward = ඉදිරියට යවන්න
menu-archive = සංරක්ෂණය කරන්න
menu-delete = මකන්න
menu-delete-forever = සදහටම මකන්න
menu-move-to-inbox = එන ලිපි වෙත ගෙන යන්න
menu-spam = අයාචිත තැපැල් ලෙස වාර්තා කරන්න
menu-not-spam = අයාචිත තැපැල් නොවේ
menu-mark-read = කියවූ ලෙස සලකුණු කරන්න
menu-mark-unread = නොකියවූ ලෙස සලකුණු කරන්න
menu-mark-all-read = සියල්ල කියවූ ලෙස සලකුණු කරන්න
menu-star = තරුවක් එක් කරන්න
menu-unstar = තරුව ඉවත් කරන්න
menu-important = වැදගත් ලෙස සලකුණු කරන්න
menu-not-important = වැදගත් නොවන ලෙස සලකුණු කරන්න
menu-pin = ඉහළට අමුණන්න
menu-unpin = ඇමිණීම ඉවත් කරන්න
menu-snooze = කල් දමන්න
menu-unsnooze = කල් දැමීම ඉවත් කරන්න
menu-print-all = සියල්ල මුද්‍රණය කරන්න
menu-new-window = නව කවුළුවක විවෘත කරන්න
menu-move-to = වෙත ගෙන යන්න
menu-move-to-heading = වෙත ගෙන යන්න:
menu-find-from = { $name } ගෙන් ලැබුණු ඊමේල් සොයන්න

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය සංරක්ෂණය කරන ලදී.
       *[other] සංවාද { $count } ක් සංරක්ෂණය කරන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය සංරක්ෂණය කරන ලදී.
       *[other] පණිවිඩ { $count } ක් සංරක්ෂණය කරන ලදී.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය කුණු කූඩයට ගෙන යන ලදී.
       *[other] සංවාද { $count } ක් කුණු කූඩයට ගෙන යන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය කුණු කූඩයට ගෙන යන ලදී.
       *[other] පණිවිඩ { $count } ක් කුණු කූඩයට ගෙන යන ලදී.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය ගෙන යන ලදී.
       *[other] සංවාද { $count } ක් ගෙන යන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය ගෙන යන ලදී.
       *[other] පණිවිඩ { $count } ක් ගෙන යන ලදී.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] සංවාදයට තරුවක් යොදන ලදී.
       *[other] සංවාද { $count } කට තරු යොදන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩයට තරුවක් යොදන ලදී.
       *[other] පණිවිඩ { $count } කට තරු යොදන ලදී.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] සංවාදයෙන් තරුව ඉවත් කරන ලදී.
       *[other] සංවාද { $count } කින් තරු ඉවත් කරන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩයෙන් තරුව ඉවත් කරන ලදී.
       *[other] පණිවිඩ { $count } කින් තරු ඉවත් කරන ලදී.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය වැදගත් ලෙස සලකුණු කරන ලදී.
       *[other] සංවාද { $count } ක් වැදගත් ලෙස සලකුණු කරන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය වැදගත් ලෙස සලකුණු කරන ලදී.
       *[other] පණිවිඩ { $count } ක් වැදගත් ලෙස සලකුණු කරන ලදී.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය වැදගත් නොවන ලෙස සලකුණු කරන ලදී.
       *[other] සංවාද { $count } ක් වැදගත් නොවන ලෙස සලකුණු කරන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය වැදගත් නොවන ලෙස සලකුණු කරන ලදී.
       *[other] පණිවිඩ { $count } ක් වැදගත් නොවන ලෙස සලකුණු කරන ලදී.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය ඉහළට අමුණන ලදී.
       *[other] සංවාද { $count } ක් ඉහළට අමුණන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය ඉහළට අමුණන ලදී.
       *[other] පණිවිඩ { $count } ක් ඉහළට අමුණන ලදී.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] සංවාදයේ ඇමිණීම ඉවත් කරන ලදී.
       *[other] සංවාද { $count } ක ඇමිණීම ඉවත් කරන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩයේ ඇමිණීම ඉවත් කරන ලදී.
       *[other] පණිවිඩ { $count } ක ඇමිණීම ඉවත් කරන ලදී.
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය { $when } දක්වා කල් දමන ලදී.
       *[other] සංවාද { $count } ක් { $when } දක්වා කල් දමන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය { $when } දක්වා කල් දමන ලදී.
       *[other] පණිවිඩ { $count } ක් { $when } දක්වා කල් දමන ලදී.
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය නැවත එන ලිපි වෙත පැමිණියේය.
       *[other] සංවාද { $count } ක් නැවත එන ලිපි වෙත පැමිණියේය.
    }
   *[message] { $count ->
        [one] පණිවිඩය නැවත එන ලිපි වෙත පැමිණියේය.
       *[other] පණිවිඩ { $count } ක් නැවත එන ලිපි වෙත පැමිණියේය.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය අයාචිත තැපැල් ලෙස වාර්තා කරන ලදී.
       *[other] සංවාද { $count } ක් අයාචිත තැපැල් ලෙස වාර්තා කරන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය අයාචිත තැපැල් ලෙස වාර්තා කරන ලදී.
       *[other] පණිවිඩ { $count } ක් අයාචිත තැපැල් ලෙස වාර්තා කරන ලදී.
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය අයාචිත තැපැල් නොවන ලෙස සලකුණු කර එන ලිපි වෙත ගෙන යන ලදී.
       *[other] සංවාද { $count } ක් අයාචිත තැපැල් නොවන ලෙස සලකුණු කර එන ලිපි වෙත ගෙන යන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය අයාචිත තැපැල් නොවන ලෙස සලකුණු කර එන ලිපි වෙත ගෙන යන ලදී.
       *[other] පණිවිඩ { $count } ක් අයාචිත තැපැල් නොවන ලෙස සලකුණු කර එන ලිපි වෙත ගෙන යන ලදී.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය සදහටම මකන ලදී.
       *[other] සංවාද { $count } ක් සදහටම මකන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය සදහටම මකන ලදී.
       *[other] පණිවිඩ { $count } ක් සදහටම මකන ලදී.
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය කියවූ ලෙස සලකුණු කරන ලදී.
       *[other] සංවාද { $count } ක් කියවූ ලෙස සලකුණු කරන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය කියවූ ලෙස සලකුණු කරන ලදී.
       *[other] පණිවිඩ { $count } ක් කියවූ ලෙස සලකුණු කරන ලදී.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] සංවාදය නොකියවූ ලෙස සලකුණු කරන ලදී.
       *[other] සංවාද { $count } ක් නොකියවූ ලෙස සලකුණු කරන ලදී.
    }
   *[message] { $count ->
        [one] පණිවිඩය නොකියවූ ලෙස සලකුණු කරන ලදී.
       *[other] පණිවිඩ { $count } ක් නොකියවූ ලෙස සලකුණු කරන ලදී.
    }
}
toast-undone = ක්‍රියාව අහෝසි කරන ලදී.
toast-nothing-to-undo = අහෝසි කිරීමට කිසිවක් නැත.
toast-cannot-undo-delete-forever = සදහටම මැකූ තැපැල් ආපසු ගෙන ආ නොහැක.
toast-send-undone = යැවීම අහෝසි කරන ලදී.
toast-too-late-to-undo-send = අහෝසි කිරීමට ප්‍රමාද වැඩියි: පණිවිඩය දැනටමත් යවා ඇත.
toast-undo = අහෝසි කරන්න
toast-no-spam-folder = මෙම ගිණුමට අයාචිත තැපැල් ෆෝල්ඩරයක් නැත.
