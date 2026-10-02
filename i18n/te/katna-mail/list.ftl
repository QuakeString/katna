# Katna Mail, Telugu (తెలుగు).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = ప్రాథమికం
tab-promotions = ప్రమోషన్‌లు
tab-social = సామాజికం
tab-updates = అప్‌డేట్‌లు
tab-forums = ఫోరమ్‌లు
tab-focused = ఫోకస్ చేసినవి
tab-other = ఇతరాలు
tab-inbox = ఇన్‌బాక్స్
tab-newsletters = న్యూస్‌లెటర్‌లు
tab-notifications = నోటిఫికేషన్‌లు
tab-provider-other = Katna క్రమబద్ధీకరించింది

## Mail list: toolbar

list-select = ఎంచుకోండి
list-refresh = రిఫ్రెష్ చేయండి
list-back-to-top = పైకి వెళ్లండి
list-checking = కొత్త మెయిల్ కోసం చెక్ చేస్తోంది…
list-more = మరిన్ని
list-mark-read = చదివినట్లు గుర్తు పెట్టండి
list-mark-unread = చదవనట్లు గుర్తు పెట్టండి
list-move-to = దీనికి తరలించండి
list-archive = ఆర్కైవ్ చేయండి
list-spam = స్పామ్‌గా రిపోర్ట్ చేయండి
list-delete = తొలగించండి
list-snooze = స్నూజ్ చేయండి
list-unsnooze = స్నూజ్ తీసివేయండి
list-newer = కొత్తవి
list-older = పాతవి
list-range = { $total }లో { $first }–{ $last }
list-range-about = సుమారు { $total }లో { $first }–{ $last }
list-results = “{ $query }” కోసం ఫలితాలు
list-results-corrected = “{ $query }” కోసం ఫలితాలను చూపుతోంది
list-search-instead = దానికి బదులుగా “{ $query }” కోసం వెతకండి
list-files-more = +{ $count }
list-replied = మీరు రిప్లయి ఇచ్చారు

## Mail list: Select menu (which lines to tick)

list-pick-all = అన్నీ
list-pick-none = ఏవీ వద్దు
list-pick-read = చదివినవి
list-pick-unread = చదవనివి
list-pick-starred = నక్షత్రం ఉంచినవి
list-pick-unstarred = నక్షత్రం ఉంచనివి

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } సంభాషణ ఎంచుకోబడింది.
       *[other] మొత్తం { $count } సంభాషణలు ఎంచుకోబడ్డాయి.
    }
   *[message] { $count ->
        [one] { $count } మెసేజ్ ఎంచుకోబడింది.
       *[other] మొత్తం { $count } మెసేజ్‌లు ఎంచుకోబడ్డాయి.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder }లోని { $count } సంభాషణ ఎంచుకోబడింది.
       *[other] { $folder }లోని మొత్తం { $count } సంభాషణలు ఎంచుకోబడ్డాయి.
    }
   *[message] { $count ->
        [one] { $folder }లోని { $count } మెసేజ్ ఎంచుకోబడింది.
       *[other] { $folder }లోని మొత్తం { $count } మెసేజ్‌లు ఎంచుకోబడ్డాయి.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] స్క్రీన్‌పై ఉన్న { $count } సంభాషణ ఎంచుకోబడింది.
       *[other] స్క్రీన్‌పై ఉన్న మొత్తం { $count } సంభాషణలు ఎంచుకోబడ్డాయి.
    }
   *[message] { $count ->
        [one] స్క్రీన్‌పై ఉన్న { $count } మెసేజ్ ఎంచుకోబడింది.
       *[other] స్క్రీన్‌పై ఉన్న మొత్తం { $count } మెసేజ్‌లు ఎంచుకోబడ్డాయి.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } సంభాషణను ఎంచుకోండి
       *[other] మొత్తం { $count } సంభాషణలను ఎంచుకోండి
    }
   *[message] { $count ->
        [one] { $count } మెసేజ్‌ను ఎంచుకోండి
       *[other] మొత్తం { $count } మెసేజ్‌లను ఎంచుకోండి
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder }లోని { $count } సంభాషణను ఎంచుకోండి
       *[other] { $folder }లోని మొత్తం { $count } సంభాషణలను ఎంచుకోండి
    }
   *[message] { $count ->
        [one] { $folder }లోని { $count } మెసేజ్‌ను ఎంచుకోండి
       *[other] { $folder }లోని మొత్తం { $count } మెసేజ్‌లను ఎంచుకోండి
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] స్క్రీన్‌పై ఉన్న { $count } చదివిన సంభాషణ ఎంచుకోబడింది.
           *[other] స్క్రీన్‌పై ఉన్న మొత్తం { $count } చదివిన సంభాషణలు ఎంచుకోబడ్డాయి.
        }
       *[message] { $count ->
            [one] స్క్రీన్‌పై ఉన్న { $count } చదివిన మెసేజ్ ఎంచుకోబడింది.
           *[other] స్క్రీన్‌పై ఉన్న మొత్తం { $count } చదివిన మెసేజ్‌లు ఎంచుకోబడ్డాయి.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] స్క్రీన్‌పై ఉన్న { $count } చదవని సంభాషణ ఎంచుకోబడింది.
           *[other] స్క్రీన్‌పై ఉన్న మొత్తం { $count } చదవని సంభాషణలు ఎంచుకోబడ్డాయి.
        }
       *[message] { $count ->
            [one] స్క్రీన్‌పై ఉన్న { $count } చదవని మెసేజ్ ఎంచుకోబడింది.
           *[other] స్క్రీన్‌పై ఉన్న మొత్తం { $count } చదవని మెసేజ్‌లు ఎంచుకోబడ్డాయి.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] స్క్రీన్‌పై ఉన్న { $count } నక్షత్రం ఉంచిన సంభాషణ ఎంచుకోబడింది.
           *[other] స్క్రీన్‌పై ఉన్న మొత్తం { $count } నక్షత్రం ఉంచిన సంభాషణలు ఎంచుకోబడ్డాయి.
        }
       *[message] { $count ->
            [one] స్క్రీన్‌పై ఉన్న { $count } నక్షత్రం ఉంచిన మెసేజ్ ఎంచుకోబడింది.
           *[other] స్క్రీన్‌పై ఉన్న మొత్తం { $count } నక్షత్రం ఉంచిన మెసేజ్‌లు ఎంచుకోబడ్డాయి.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] స్క్రీన్‌పై ఉన్న { $count } నక్షత్రం ఉంచని సంభాషణ ఎంచుకోబడింది.
           *[other] స్క్రీన్‌పై ఉన్న మొత్తం { $count } నక్షత్రం ఉంచని సంభాషణలు ఎంచుకోబడ్డాయి.
        }
       *[message] { $count ->
            [one] స్క్రీన్‌పై ఉన్న { $count } నక్షత్రం ఉంచని మెసేజ్ ఎంచుకోబడింది.
           *[other] స్క్రీన్‌పై ఉన్న మొత్తం { $count } నక్షత్రం ఉంచని మెసేజ్‌లు ఎంచుకోబడ్డాయి.
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } చదివిన సంభాషణను ఎంచుకోండి
           *[other] మొత్తం { $count } చదివిన సంభాషణలను ఎంచుకోండి
        }
       *[message] { $count ->
            [one] { $count } చదివిన మెసేజ్‌ను ఎంచుకోండి
           *[other] మొత్తం { $count } చదివిన మెసేజ్‌లను ఎంచుకోండి
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } చదవని సంభాషణను ఎంచుకోండి
           *[other] మొత్తం { $count } చదవని సంభాషణలను ఎంచుకోండి
        }
       *[message] { $count ->
            [one] { $count } చదవని మెసేజ్‌ను ఎంచుకోండి
           *[other] మొత్తం { $count } చదవని మెసేజ్‌లను ఎంచుకోండి
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } నక్షత్రం ఉంచిన సంభాషణను ఎంచుకోండి
           *[other] మొత్తం { $count } నక్షత్రం ఉంచిన సంభాషణలను ఎంచుకోండి
        }
       *[message] { $count ->
            [one] { $count } నక్షత్రం ఉంచిన మెసేజ్‌ను ఎంచుకోండి
           *[other] మొత్తం { $count } నక్షత్రం ఉంచిన మెసేజ్‌లను ఎంచుకోండి
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } నక్షత్రం ఉంచని సంభాషణను ఎంచుకోండి
           *[other] మొత్తం { $count } నక్షత్రం ఉంచని సంభాషణలను ఎంచుకోండి
        }
       *[message] { $count ->
            [one] { $count } నక్షత్రం ఉంచని మెసేజ్‌ను ఎంచుకోండి
           *[other] మొత్తం { $count } నక్షత్రం ఉంచని మెసేజ్‌లను ఎంచుకోండి
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder }లోని { $count } చదివిన సంభాషణను ఎంచుకోండి
           *[other] { $folder }లోని మొత్తం { $count } చదివిన సంభాషణలను ఎంచుకోండి
        }
       *[message] { $count ->
            [one] { $folder }లోని { $count } చదివిన మెసేజ్‌ను ఎంచుకోండి
           *[other] { $folder }లోని మొత్తం { $count } చదివిన మెసేజ్‌లను ఎంచుకోండి
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder }లోని { $count } చదవని సంభాషణను ఎంచుకోండి
           *[other] { $folder }లోని మొత్తం { $count } చదవని సంభాషణలను ఎంచుకోండి
        }
       *[message] { $count ->
            [one] { $folder }లోని { $count } చదవని మెసేజ్‌ను ఎంచుకోండి
           *[other] { $folder }లోని మొత్తం { $count } చదవని మెసేజ్‌లను ఎంచుకోండి
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder }లోని { $count } నక్షత్రం ఉంచిన సంభాషణను ఎంచుకోండి
           *[other] { $folder }లోని మొత్తం { $count } నక్షత్రం ఉంచిన సంభాషణలను ఎంచుకోండి
        }
       *[message] { $count ->
            [one] { $folder }లోని { $count } నక్షత్రం ఉంచిన మెసేజ్‌ను ఎంచుకోండి
           *[other] { $folder }లోని మొత్తం { $count } నక్షత్రం ఉంచిన మెసేజ్‌లను ఎంచుకోండి
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder }లోని { $count } నక్షత్రం ఉంచని సంభాషణను ఎంచుకోండి
           *[other] { $folder }లోని మొత్తం { $count } నక్షత్రం ఉంచని సంభాషణలను ఎంచుకోండి
        }
       *[message] { $count ->
            [one] { $folder }లోని { $count } నక్షత్రం ఉంచని మెసేజ్‌ను ఎంచుకోండి
           *[other] { $folder }లోని మొత్తం { $count } నక్షత్రం ఉంచని మెసేజ్‌లను ఎంచుకోండి
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } చదివిన సంభాషణ ఎంచుకోబడింది.
           *[other] మొత్తం { $count } చదివిన సంభాషణలు ఎంచుకోబడ్డాయి.
        }
       *[message] { $count ->
            [one] { $count } చదివిన మెసేజ్ ఎంచుకోబడింది.
           *[other] మొత్తం { $count } చదివిన మెసేజ్‌లు ఎంచుకోబడ్డాయి.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } చదవని సంభాషణ ఎంచుకోబడింది.
           *[other] మొత్తం { $count } చదవని సంభాషణలు ఎంచుకోబడ్డాయి.
        }
       *[message] { $count ->
            [one] { $count } చదవని మెసేజ్ ఎంచుకోబడింది.
           *[other] మొత్తం { $count } చదవని మెసేజ్‌లు ఎంచుకోబడ్డాయి.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } నక్షత్రం ఉంచిన సంభాషణ ఎంచుకోబడింది.
           *[other] మొత్తం { $count } నక్షత్రం ఉంచిన సంభాషణలు ఎంచుకోబడ్డాయి.
        }
       *[message] { $count ->
            [one] { $count } నక్షత్రం ఉంచిన మెసేజ్ ఎంచుకోబడింది.
           *[other] మొత్తం { $count } నక్షత్రం ఉంచిన మెసేజ్‌లు ఎంచుకోబడ్డాయి.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } నక్షత్రం ఉంచని సంభాషణ ఎంచుకోబడింది.
           *[other] మొత్తం { $count } నక్షత్రం ఉంచని సంభాషణలు ఎంచుకోబడ్డాయి.
        }
       *[message] { $count ->
            [one] { $count } నక్షత్రం ఉంచని మెసేజ్ ఎంచుకోబడింది.
           *[other] మొత్తం { $count } నక్షత్రం ఉంచని మెసేజ్‌లు ఎంచుకోబడ్డాయి.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder }లోని { $count } చదివిన సంభాషణ ఎంచుకోబడింది.
           *[other] { $folder }లోని మొత్తం { $count } చదివిన సంభాషణలు ఎంచుకోబడ్డాయి.
        }
       *[message] { $count ->
            [one] { $folder }లోని { $count } చదివిన మెసేజ్ ఎంచుకోబడింది.
           *[other] { $folder }లోని మొత్తం { $count } చదివిన మెసేజ్‌లు ఎంచుకోబడ్డాయి.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder }లోని { $count } చదవని సంభాషణ ఎంచుకోబడింది.
           *[other] { $folder }లోని మొత్తం { $count } చదవని సంభాషణలు ఎంచుకోబడ్డాయి.
        }
       *[message] { $count ->
            [one] { $folder }లోని { $count } చదవని మెసేజ్ ఎంచుకోబడింది.
           *[other] { $folder }లోని మొత్తం { $count } చదవని మెసేజ్‌లు ఎంచుకోబడ్డాయి.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder }లోని { $count } నక్షత్రం ఉంచిన సంభాషణ ఎంచుకోబడింది.
           *[other] { $folder }లోని మొత్తం { $count } నక్షత్రం ఉంచిన సంభాషణలు ఎంచుకోబడ్డాయి.
        }
       *[message] { $count ->
            [one] { $folder }లోని { $count } నక్షత్రం ఉంచిన మెసేజ్ ఎంచుకోబడింది.
           *[other] { $folder }లోని మొత్తం { $count } నక్షత్రం ఉంచిన మెసేజ్‌లు ఎంచుకోబడ్డాయి.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder }లోని { $count } నక్షత్రం ఉంచని సంభాషణ ఎంచుకోబడింది.
           *[other] { $folder }లోని మొత్తం { $count } నక్షత్రం ఉంచని సంభాషణలు ఎంచుకోబడ్డాయి.
        }
       *[message] { $count ->
            [one] { $folder }లోని { $count } నక్షత్రం ఉంచని మెసేజ్ ఎంచుకోబడింది.
           *[other] { $folder }లోని మొత్తం { $count } నక్షత్రం ఉంచని మెసేజ్‌లు ఎంచుకోబడ్డాయి.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] ఇక్కడ చదివిన సంభాషణలు ఏవీ లేవు.
       *[message] ఇక్కడ చదివిన మెసేజ్‌లు ఏవీ లేవు.
    }
   *[unread] { $kind ->
        [conversation] ఇక్కడ చదవని సంభాషణలు ఏవీ లేవు.
       *[message] ఇక్కడ చదవని మెసేజ్‌లు ఏవీ లేవు.
    }
    [starred] { $kind ->
        [conversation] ఇక్కడ నక్షత్రం ఉంచిన సంభాషణలు ఏవీ లేవు.
       *[message] ఇక్కడ నక్షత్రం ఉంచిన మెసేజ్‌లు ఏవీ లేవు.
    }
    [unstarred] { $kind ->
        [conversation] ఇక్కడ నక్షత్రం ఉంచని సంభాషణలు ఏవీ లేవు.
       *[message] ఇక్కడ నక్షత్రం ఉంచని మెసేజ్‌లు ఏవీ లేవు.
    }
}
list-clear-selection = ఎంపికను క్లియర్ చేయండి

## Mail list: empty states

list-empty-search = మీ సెర్చ్‌కు సరిపోలే మెసేజ్‌లు ఏవీ లేవు.
list-empty-tab = { $tab }లో మెయిల్ ఏదీ లేదు.
list-empty-tab-unknown = ఈ ట్యాబ్‌లో మెయిల్ ఏదీ లేదు.
list-empty-folder = { $folder }లో మెసేజ్‌లు ఏవీ లేవు.
list-empty-folder-unknown = ఈ ఫోల్డర్‌లో మెసేజ్‌లు ఏవీ లేవు.
list-first-sync = మీ మెయిల్‌ను పొందుతోంది…
list-first-sync-detail = మెయిల్ వచ్చే కొద్దీ ఇక్కడ కనిపిస్తుంది.

## Mail list: lines

row-removed = ఈ మెసేజ్ తీసివేయబడింది.
row-starred = నక్షత్రం ఉంచబడింది
row-not-starred = నక్షత్రం ఉంచలేదు
row-important = ముఖ్యమైనది. ముఖ్యమైనది కాదని గుర్తు పెట్టడానికి క్లిక్ చేయండి.
row-mark-important = ముఖ్యమైనదిగా గుర్తు పెట్టండి
row-pinned = పైన పిన్ చేయబడింది
row-task = టాస్క్
row-task-open = టాస్క్‌ను తెరవండి: { $title }
row-tracking-none = ట్రాక్ చేయబడుతోంది. ఇంకా తెరవలేదు
row-tracking-opened = { $recipients } మందిలో { $opened } మంది తెరిచారు
row-tracking-clicked = { $recipients } మందిలో { $opened } మంది తెరిచారు, { $clicked } మంది లింక్‌ను తెరిచారు
row-pin = పైన పిన్ చేయండి
row-unpin = అన్‌పిన్ చేయండి
row-snoozed-until = { $when } వరకు స్నూజ్ చేయబడింది

## Mail list: More menu and right-click menu

menu-reply = రిప్లయి ఇవ్వండి
menu-reply-all = అందరికీ రిప్లయి ఇవ్వండి
menu-forward = ఫార్వర్డ్ చేయండి
menu-archive = ఆర్కైవ్ చేయండి
menu-delete = తొలగించండి
menu-delete-forever = శాశ్వతంగా తొలగించండి
menu-move-to-inbox = ఇన్‌బాక్స్‌కు తరలించండి
menu-spam = స్పామ్‌గా రిపోర్ట్ చేయండి
menu-not-spam = స్పామ్ కాదు
menu-mark-read = చదివినట్లు గుర్తు పెట్టండి
menu-mark-unread = చదవనట్లు గుర్తు పెట్టండి
menu-mark-all-read = అన్నింటినీ చదివినట్లు గుర్తు పెట్టండి
menu-star = నక్షత్రం ఉంచండి
menu-unstar = నక్షత్రాన్ని తీసివేయండి
menu-important = ముఖ్యమైనదిగా గుర్తు పెట్టండి
menu-not-important = ముఖ్యమైనది కాదని గుర్తు పెట్టండి
menu-pin = పైన పిన్ చేయండి
menu-unpin = అన్‌పిన్ చేయండి
menu-snooze = స్నూజ్ చేయండి
menu-unsnooze = స్నూజ్ తీసివేయండి
menu-add-to-tasks = టాస్క్‌లకు జోడించండి
menu-schedule-meeting = సమావేశాన్ని షెడ్యూల్ చేయండి
menu-start-call = వీడియో కాల్ ప్రారంభించండి
menu-add-note = గమనికను జోడించండి
menu-print-all = అన్నీ ప్రింట్ చేయండి
menu-new-window = కొత్త విండోలో తెరవండి
menu-move-to = దీనికి తరలించండి
menu-follow-up = ఫాలో అప్
menu-more = మరిన్ని
menu-move-to-heading = దీనికి తరలించండి:
menu-find-from = { $name } నుండి వచ్చిన ఈమెయిల్స్‌ను కనుగొనండి

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ ఆర్కైవ్ చేయబడింది.
       *[other] { $count } సంభాషణలు ఆర్కైవ్ చేయబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ ఆర్కైవ్ చేయబడింది.
       *[other] { $count } మెసేజ్‌లు ఆర్కైవ్ చేయబడ్డాయి.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ ట్రాష్‌కు తరలించబడింది.
       *[other] { $count } సంభాషణలు ట్రాష్‌కు తరలించబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ ట్రాష్‌కు తరలించబడింది.
       *[other] { $count } మెసేజ్‌లు ట్రాష్‌కు తరలించబడ్డాయి.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ తరలించబడింది.
       *[other] { $count } సంభాషణలు తరలించబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ తరలించబడింది.
       *[other] { $count } మెసేజ్‌లు తరలించబడ్డాయి.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణకు నక్షత్రం ఉంచబడింది.
       *[other] { $count } సంభాషణలకు నక్షత్రం ఉంచబడింది.
    }
   *[message] { $count ->
        [one] మెసేజ్‌కు నక్షత్రం ఉంచబడింది.
       *[other] { $count } మెసేజ్‌లకు నక్షత్రం ఉంచబడింది.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ నుండి నక్షత్రం తీసివేయబడింది.
       *[other] { $count } సంభాషణల నుండి నక్షత్రం తీసివేయబడింది.
    }
   *[message] { $count ->
        [one] మెసేజ్ నుండి నక్షత్రం తీసివేయబడింది.
       *[other] { $count } మెసేజ్‌ల నుండి నక్షత్రం తీసివేయబడింది.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ ముఖ్యమైనదిగా గుర్తు పెట్టబడింది.
       *[other] { $count } సంభాషణలు ముఖ్యమైనవిగా గుర్తు పెట్టబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ ముఖ్యమైనదిగా గుర్తు పెట్టబడింది.
       *[other] { $count } మెసేజ్‌లు ముఖ్యమైనవిగా గుర్తు పెట్టబడ్డాయి.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ ముఖ్యమైనది కాదని గుర్తు పెట్టబడింది.
       *[other] { $count } సంభాషణలు ముఖ్యమైనవి కాదని గుర్తు పెట్టబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ ముఖ్యమైనది కాదని గుర్తు పెట్టబడింది.
       *[other] { $count } మెసేజ్‌లు ముఖ్యమైనవి కాదని గుర్తు పెట్టబడ్డాయి.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ పైన పిన్ చేయబడింది.
       *[other] { $count } సంభాషణలు పైన పిన్ చేయబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ పైన పిన్ చేయబడింది.
       *[other] { $count } మెసేజ్‌లు పైన పిన్ చేయబడ్డాయి.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ అన్‌పిన్ చేయబడింది.
       *[other] { $count } సంభాషణలు అన్‌పిన్ చేయబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ అన్‌పిన్ చేయబడింది.
       *[other] { $count } మెసేజ్‌లు అన్‌పిన్ చేయబడ్డాయి.
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ { $when } వరకు స్నూజ్ చేయబడింది.
       *[other] { $count } సంభాషణలు { $when } వరకు స్నూజ్ చేయబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ { $when } వరకు స్నూజ్ చేయబడింది.
       *[other] { $count } మెసేజ్‌లు { $when } వరకు స్నూజ్ చేయబడ్డాయి.
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ ఇన్‌బాక్స్‌కు తిరిగి వచ్చింది.
       *[other] { $count } సంభాషణలు ఇన్‌బాక్స్‌కు తిరిగి వచ్చాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ ఇన్‌బాక్స్‌కు తిరిగి వచ్చింది.
       *[other] { $count } మెసేజ్‌లు ఇన్‌బాక్స్‌కు తిరిగి వచ్చాయి.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ స్పామ్‌గా రిపోర్ట్ చేయబడింది.
       *[other] { $count } సంభాషణలు స్పామ్‌గా రిపోర్ట్ చేయబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ స్పామ్‌గా రిపోర్ట్ చేయబడింది.
       *[other] { $count } మెసేజ్‌లు స్పామ్‌గా రిపోర్ట్ చేయబడ్డాయి.
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ స్పామ్ కాదని గుర్తించబడి ఇన్‌బాక్స్‌కు తరలించబడింది.
       *[other] { $count } సంభాషణలు స్పామ్ కాదని గుర్తించబడి ఇన్‌బాక్స్‌కు తరలించబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ స్పామ్ కాదని గుర్తించబడి ఇన్‌బాక్స్‌కు తరలించబడింది.
       *[other] { $count } మెసేజ్‌లు స్పామ్ కాదని గుర్తించబడి ఇన్‌బాక్స్‌కు తరలించబడ్డాయి.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ శాశ్వతంగా తొలగించబడింది.
       *[other] { $count } సంభాషణలు శాశ్వతంగా తొలగించబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ శాశ్వతంగా తొలగించబడింది.
       *[other] { $count } మెసేజ్‌లు శాశ్వతంగా తొలగించబడ్డాయి.
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ చదివినట్లు గుర్తు పెట్టబడింది.
       *[other] { $count } సంభాషణలు చదివినట్లు గుర్తు పెట్టబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ చదివినట్లు గుర్తు పెట్టబడింది.
       *[other] { $count } మెసేజ్‌లు చదివినట్లు గుర్తు పెట్టబడ్డాయి.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ చదవనట్లు గుర్తు పెట్టబడింది.
       *[other] { $count } సంభాషణలు చదవనట్లు గుర్తు పెట్టబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ చదవనట్లు గుర్తు పెట్టబడింది.
       *[other] { $count } మెసేజ్‌లు చదవనట్లు గుర్తు పెట్టబడ్డాయి.
    }
}
toast-undone = చర్య రద్దు చేయబడింది.
toast-nothing-to-undo = అన్‌డూ చేయడానికి ఏమీ లేదు.
toast-cannot-undo-delete-forever = శాశ్వతంగా తొలగించిన మెయిల్‌ను తిరిగి తీసుకురాలేము.
toast-send-undone = పంపడం రద్దు చేయబడింది.
toast-too-late-to-undo-send = అన్‌డూ చేయడానికి ఆలస్యమైంది: మెసేజ్ ఇప్పటికే పంపబడింది.
toast-undo = చర్య రద్దు చేయండి
toast-close = మూసివేయండి
toast-no-spam-folder = ఈ ఖాతాలో స్పామ్ ఫోల్డర్ లేదు.
