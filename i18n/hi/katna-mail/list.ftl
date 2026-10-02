# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = मुख्य
tab-promotions = प्रमोशन
tab-social = सामाजिक
tab-updates = अपडेट
tab-forums = फ़ोरम
tab-focused = फ़ोकस्ड
tab-other = अन्य
tab-inbox = इनबॉक्स
tab-newsletters = न्यूज़लेटर
tab-notifications = सूचनाएं
tab-provider-other = Katna ने छांटा

## Mail list: toolbar

list-select = चुनें
list-refresh = रीफ़्रेश करें
list-back-to-top = ऊपर जाएं
list-checking = नए मेल की जाँच हो रही है…
list-more = ज़्यादा
list-mark-read = पढ़ा गया के रूप में मार्क करें
list-mark-unread = नहीं पढ़ा गया के रूप में मार्क करें
list-move-to = इसमें ले जाएं
list-archive = संग्रह करें
list-spam = स्पैम की शिकायत करें
list-delete = मिटाएं
list-snooze = स्नूज़ करें
list-unsnooze = स्नूज़ हटाएं
list-newer = नए
list-older = पुराने
list-range = { $total } में से { $first }–{ $last }
list-range-about = लगभग { $total } में से { $first }–{ $last }
list-results = “{ $query }” के नतीजे
list-results-corrected = “{ $query }” के नतीजे दिखाए जा रहे हैं
list-search-instead = इसके बजाय “{ $query }” खोजें
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = सभी
list-pick-none = कोई नहीं
list-pick-read = पढ़े गए
list-pick-unread = नहीं पढ़े गए
list-pick-starred = तारांकित
list-pick-unstarred = तारांकित नहीं

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } बातचीत चुनी गई है।
       *[other] सभी { $count } बातचीत चुनी गई हैं।
    }
   *[message] { $count ->
        [one] { $count } मैसेज चुना गया है।
       *[other] सभी { $count } मैसेज चुने गए हैं।
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } में { $count } बातचीत चुनी गई है।
       *[other] { $folder } में सभी { $count } बातचीत चुनी गई हैं।
    }
   *[message] { $count ->
        [one] { $folder } में { $count } मैसेज चुना गया है।
       *[other] { $folder } में सभी { $count } मैसेज चुने गए हैं।
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] स्क्रीन पर { $count } बातचीत चुनी गई है।
       *[other] स्क्रीन पर सभी { $count } बातचीत चुनी गई हैं।
    }
   *[message] { $count ->
        [one] स्क्रीन पर { $count } मैसेज चुना गया है।
       *[other] स्क्रीन पर सभी { $count } मैसेज चुने गए हैं।
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } बातचीत चुनें
       *[other] सभी { $count } बातचीत चुनें
    }
   *[message] { $count ->
        [one] { $count } मैसेज चुनें
       *[other] सभी { $count } मैसेज चुनें
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } में { $count } बातचीत चुनें
       *[other] { $folder } में सभी { $count } बातचीत चुनें
    }
   *[message] { $count ->
        [one] { $folder } में { $count } मैसेज चुनें
       *[other] { $folder } में सभी { $count } मैसेज चुनें
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] स्क्रीन पर { $count } पढ़ी गई बातचीत चुनी गई है।
           *[other] स्क्रीन पर सभी { $count } पढ़ी गई बातचीत चुनी गई हैं।
        }
       *[message] { $count ->
            [one] स्क्रीन पर { $count } पढ़ा गया मैसेज चुना गया है।
           *[other] स्क्रीन पर सभी { $count } पढ़े गए मैसेज चुने गए हैं।
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] स्क्रीन पर { $count } बिना पढ़ी बातचीत चुनी गई है।
           *[other] स्क्रीन पर सभी { $count } बिना पढ़ी बातचीत चुनी गई हैं।
        }
       *[message] { $count ->
            [one] स्क्रीन पर { $count } बिना पढ़ा मैसेज चुना गया है।
           *[other] स्क्रीन पर सभी { $count } बिना पढ़े मैसेज चुने गए हैं।
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] स्क्रीन पर { $count } तारांकित बातचीत चुनी गई है।
           *[other] स्क्रीन पर सभी { $count } तारांकित बातचीत चुनी गई हैं।
        }
       *[message] { $count ->
            [one] स्क्रीन पर { $count } तारांकित मैसेज चुना गया है।
           *[other] स्क्रीन पर सभी { $count } तारांकित मैसेज चुने गए हैं।
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] स्क्रीन पर { $count } तारांकित न की गई बातचीत चुनी गई है।
           *[other] स्क्रीन पर सभी { $count } तारांकित न की गई बातचीत चुनी गई हैं।
        }
       *[message] { $count ->
            [one] स्क्रीन पर { $count } तारांकित न किया गया मैसेज चुना गया है।
           *[other] स्क्रीन पर सभी { $count } तारांकित न किए गए मैसेज चुने गए हैं।
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } पढ़ी गई बातचीत चुनें
           *[other] सभी { $count } पढ़ी गई बातचीत चुनें
        }
       *[message] { $count ->
            [one] { $count } पढ़ा गया मैसेज चुनें
           *[other] सभी { $count } पढ़े गए मैसेज चुनें
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } बिना पढ़ी बातचीत चुनें
           *[other] सभी { $count } बिना पढ़ी बातचीत चुनें
        }
       *[message] { $count ->
            [one] { $count } बिना पढ़ा मैसेज चुनें
           *[other] सभी { $count } बिना पढ़े मैसेज चुनें
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } तारांकित बातचीत चुनें
           *[other] सभी { $count } तारांकित बातचीत चुनें
        }
       *[message] { $count ->
            [one] { $count } तारांकित मैसेज चुनें
           *[other] सभी { $count } तारांकित मैसेज चुनें
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } तारांकित न की गई बातचीत चुनें
           *[other] सभी { $count } तारांकित न की गई बातचीत चुनें
        }
       *[message] { $count ->
            [one] { $count } तारांकित न किया गया मैसेज चुनें
           *[other] सभी { $count } तारांकित न किए गए मैसेज चुनें
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder } में { $count } पढ़ी गई बातचीत चुनें
           *[other] { $folder } में सभी { $count } पढ़ी गई बातचीत चुनें
        }
       *[message] { $count ->
            [one] { $folder } में { $count } पढ़ा गया मैसेज चुनें
           *[other] { $folder } में सभी { $count } पढ़े गए मैसेज चुनें
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder } में { $count } बिना पढ़ी बातचीत चुनें
           *[other] { $folder } में सभी { $count } बिना पढ़ी बातचीत चुनें
        }
       *[message] { $count ->
            [one] { $folder } में { $count } बिना पढ़ा मैसेज चुनें
           *[other] { $folder } में सभी { $count } बिना पढ़े मैसेज चुनें
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } में { $count } तारांकित बातचीत चुनें
           *[other] { $folder } में सभी { $count } तारांकित बातचीत चुनें
        }
       *[message] { $count ->
            [one] { $folder } में { $count } तारांकित मैसेज चुनें
           *[other] { $folder } में सभी { $count } तारांकित मैसेज चुनें
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } में { $count } तारांकित न की गई बातचीत चुनें
           *[other] { $folder } में सभी { $count } तारांकित न की गई बातचीत चुनें
        }
       *[message] { $count ->
            [one] { $folder } में { $count } तारांकित न किया गया मैसेज चुनें
           *[other] { $folder } में सभी { $count } तारांकित न किए गए मैसेज चुनें
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } पढ़ी गई बातचीत चुनी गई है।
           *[other] सभी { $count } पढ़ी गई बातचीत चुनी गई हैं।
        }
       *[message] { $count ->
            [one] { $count } पढ़ा गया मैसेज चुना गया है।
           *[other] सभी { $count } पढ़े गए मैसेज चुने गए हैं।
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } बिना पढ़ी बातचीत चुनी गई है।
           *[other] सभी { $count } बिना पढ़ी बातचीत चुनी गई हैं।
        }
       *[message] { $count ->
            [one] { $count } बिना पढ़ा मैसेज चुना गया है।
           *[other] सभी { $count } बिना पढ़े मैसेज चुने गए हैं।
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } तारांकित बातचीत चुनी गई है।
           *[other] सभी { $count } तारांकित बातचीत चुनी गई हैं।
        }
       *[message] { $count ->
            [one] { $count } तारांकित मैसेज चुना गया है।
           *[other] सभी { $count } तारांकित मैसेज चुने गए हैं।
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } तारांकित न की गई बातचीत चुनी गई है।
           *[other] सभी { $count } तारांकित न की गई बातचीत चुनी गई हैं।
        }
       *[message] { $count ->
            [one] { $count } तारांकित न किया गया मैसेज चुना गया है।
           *[other] सभी { $count } तारांकित न किए गए मैसेज चुने गए हैं।
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder } में { $count } पढ़ी गई बातचीत चुनी गई है।
           *[other] { $folder } में सभी { $count } पढ़ी गई बातचीत चुनी गई हैं।
        }
       *[message] { $count ->
            [one] { $folder } में { $count } पढ़ा गया मैसेज चुना गया है।
           *[other] { $folder } में सभी { $count } पढ़े गए मैसेज चुने गए हैं।
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder } में { $count } बिना पढ़ी बातचीत चुनी गई है।
           *[other] { $folder } में सभी { $count } बिना पढ़ी बातचीत चुनी गई हैं।
        }
       *[message] { $count ->
            [one] { $folder } में { $count } बिना पढ़ा मैसेज चुना गया है।
           *[other] { $folder } में सभी { $count } बिना पढ़े मैसेज चुने गए हैं।
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } में { $count } तारांकित बातचीत चुनी गई है।
           *[other] { $folder } में सभी { $count } तारांकित बातचीत चुनी गई हैं।
        }
       *[message] { $count ->
            [one] { $folder } में { $count } तारांकित मैसेज चुना गया है।
           *[other] { $folder } में सभी { $count } तारांकित मैसेज चुने गए हैं।
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } में { $count } तारांकित न की गई बातचीत चुनी गई है।
           *[other] { $folder } में सभी { $count } तारांकित न की गई बातचीत चुनी गई हैं।
        }
       *[message] { $count ->
            [one] { $folder } में { $count } तारांकित न किया गया मैसेज चुना गया है।
           *[other] { $folder } में सभी { $count } तारांकित न किए गए मैसेज चुने गए हैं।
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] यहां कोई पढ़ी गई बातचीत नहीं है।
       *[message] यहां कोई पढ़ा गया मैसेज नहीं है।
    }
   *[unread] { $kind ->
        [conversation] यहां कोई बिना पढ़ी बातचीत नहीं है।
       *[message] यहां कोई बिना पढ़ा मैसेज नहीं है।
    }
    [starred] { $kind ->
        [conversation] यहां कोई तारांकित बातचीत नहीं है।
       *[message] यहां कोई तारांकित मैसेज नहीं है।
    }
    [unstarred] { $kind ->
        [conversation] यहां कोई तारांकित न की गई बातचीत नहीं है।
       *[message] यहां कोई तारांकित न किया गया मैसेज नहीं है।
    }
}
list-clear-selection = चुनाव हटाएं

## Mail list: empty states

list-empty-search = आपकी खोज से कोई मैसेज मेल नहीं खाता।
list-empty-tab = { $tab } में कोई मेल नहीं है।
list-empty-tab-unknown = इस टैब में कोई मेल नहीं है।
list-empty-folder = { $folder } में कोई मैसेज नहीं है।
list-empty-folder-unknown = इस फ़ोल्डर में कोई मैसेज नहीं है।
list-first-sync = आपका मेल लाया जा रहा है…
list-first-sync-detail = मेल आते ही यहां दिखेगा।

## Mail list: lines

row-removed = यह मैसेज हटा दिया गया।
row-starred = तारांकित
row-not-starred = तारांकित नहीं
row-important = ज़रूरी। ज़रूरी नहीं के रूप में मार्क करने के लिए क्लिक करें।
row-mark-important = ज़रूरी के रूप में मार्क करें
row-pinned = सबसे ऊपर पिन किया गया
row-task = टास्क
row-task-open = टास्क खोलें: { $title }
row-tracking-none = ट्रैक किया गया। अभी तक नहीं खोला गया
row-tracking-opened = { $recipients } में से { $opened } ने खोला
row-tracking-clicked = { $recipients } में से { $opened } ने खोला, { $clicked } ने लिंक खोला
row-pin = सबसे ऊपर पिन करें
row-unpin = अनपिन करें
row-snoozed-until = { $when } तक स्नूज़ किया गया

## Mail list: More menu and right-click menu

menu-reply = जवाब दें
menu-reply-all = सभी को जवाब दें
menu-forward = फ़ॉरवर्ड करें
menu-archive = संग्रह करें
menu-delete = मिटाएं
menu-delete-forever = हमेशा के लिए मिटाएं
menu-move-to-inbox = इनबॉक्स में ले जाएं
menu-spam = स्पैम की शिकायत करें
menu-not-spam = स्पैम नहीं है
menu-mark-read = पढ़ा गया के रूप में मार्क करें
menu-mark-unread = नहीं पढ़ा गया के रूप में मार्क करें
menu-mark-all-read = सभी को पढ़ा गया के रूप में मार्क करें
menu-star = तारांकित करें
menu-unstar = तारांकन हटाएं
menu-important = ज़रूरी के रूप में मार्क करें
menu-not-important = ज़रूरी नहीं के रूप में मार्क करें
menu-pin = सबसे ऊपर पिन करें
menu-unpin = अनपिन करें
menu-snooze = स्नूज़ करें
menu-unsnooze = स्नूज़ हटाएं
menu-add-to-tasks = टास्क में जोड़ें
menu-schedule-meeting = मीटिंग शेड्यूल करें
menu-start-call = वीडियो कॉल शुरू करें
menu-add-note = नोट जोड़ें
menu-print-all = सभी प्रिंट करें
menu-new-window = नई विंडो में खोलें
menu-move-to = इसमें ले जाएं
# Opens a submenu: Add to Tasks, Add a note, Schedule a meeting and Start a
# video call.
menu-follow-up = फ़ॉलो अप
# Opens a submenu of the rarer actions: Report spam, Mark as important and
# Pin to top.
menu-more = और
menu-move-to-heading = इसमें ले जाएं:
menu-find-from = { $name } से आए ईमेल ढूंढें

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] बातचीत संग्रहित की गई।
       *[other] { $count } बातचीत संग्रहित की गईं।
    }
   *[message] { $count ->
        [one] मैसेज संग्रहित किया गया।
       *[other] { $count } मैसेज संग्रहित किए गए।
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] बातचीत ट्रैश में ले जाई गई।
       *[other] { $count } बातचीत ट्रैश में ले जाई गईं।
    }
   *[message] { $count ->
        [one] मैसेज ट्रैश में ले जाया गया।
       *[other] { $count } मैसेज ट्रैश में ले जाए गए।
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] बातचीत ले जाई गई।
       *[other] { $count } बातचीत ले जाई गईं।
    }
   *[message] { $count ->
        [one] मैसेज ले जाया गया।
       *[other] { $count } मैसेज ले जाए गए।
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] बातचीत तारांकित की गई।
       *[other] { $count } बातचीत तारांकित की गईं।
    }
   *[message] { $count ->
        [one] मैसेज तारांकित किया गया।
       *[other] { $count } मैसेज तारांकित किए गए।
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] बातचीत से तारांकन हटाया गया।
       *[other] { $count } बातचीत से तारांकन हटाया गया।
    }
   *[message] { $count ->
        [one] मैसेज से तारांकन हटाया गया।
       *[other] { $count } मैसेज से तारांकन हटाया गया।
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] बातचीत ज़रूरी के रूप में मार्क की गई।
       *[other] { $count } बातचीत ज़रूरी के रूप में मार्क की गईं।
    }
   *[message] { $count ->
        [one] मैसेज ज़रूरी के रूप में मार्क किया गया।
       *[other] { $count } मैसेज ज़रूरी के रूप में मार्क किए गए।
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] बातचीत ज़रूरी नहीं के रूप में मार्क की गई।
       *[other] { $count } बातचीत ज़रूरी नहीं के रूप में मार्क की गईं।
    }
   *[message] { $count ->
        [one] मैसेज ज़रूरी नहीं के रूप में मार्क किया गया।
       *[other] { $count } मैसेज ज़रूरी नहीं के रूप में मार्क किए गए।
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] बातचीत सबसे ऊपर पिन की गई।
       *[other] { $count } बातचीत सबसे ऊपर पिन की गईं।
    }
   *[message] { $count ->
        [one] मैसेज सबसे ऊपर पिन किया गया।
       *[other] { $count } मैसेज सबसे ऊपर पिन किए गए।
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] बातचीत अनपिन की गई।
       *[other] { $count } बातचीत अनपिन की गईं।
    }
   *[message] { $count ->
        [one] मैसेज अनपिन किया गया।
       *[other] { $count } मैसेज अनपिन किए गए।
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] बातचीत { $when } तक स्नूज़ की गई।
       *[other] { $count } बातचीत { $when } तक स्नूज़ की गईं।
    }
   *[message] { $count ->
        [one] मैसेज { $when } तक स्नूज़ किया गया।
       *[other] { $count } मैसेज { $when } तक स्नूज़ किए गए।
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] बातचीत इनबॉक्स में वापस आ गई।
       *[other] { $count } बातचीत इनबॉक्स में वापस आ गईं।
    }
   *[message] { $count ->
        [one] मैसेज इनबॉक्स में वापस आ गया।
       *[other] { $count } मैसेज इनबॉक्स में वापस आ गए।
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] बातचीत की स्पैम के रूप में शिकायत की गई।
       *[other] { $count } बातचीत की स्पैम के रूप में शिकायत की गई।
    }
   *[message] { $count ->
        [one] मैसेज की स्पैम के रूप में शिकायत की गई।
       *[other] { $count } मैसेज की स्पैम के रूप में शिकायत की गई।
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] बातचीत को स्पैम नहीं के रूप में मार्क करके इनबॉक्स में ले जाया गया।
       *[other] { $count } बातचीत को स्पैम नहीं के रूप में मार्क करके इनबॉक्स में ले जाया गया।
    }
   *[message] { $count ->
        [one] मैसेज को स्पैम नहीं के रूप में मार्क करके इनबॉक्स में ले जाया गया।
       *[other] { $count } मैसेज को स्पैम नहीं के रूप में मार्क करके इनबॉक्स में ले जाया गया।
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] बातचीत हमेशा के लिए मिटा दी गई।
       *[other] { $count } बातचीत हमेशा के लिए मिटा दी गईं।
    }
   *[message] { $count ->
        [one] मैसेज हमेशा के लिए मिटा दिया गया।
       *[other] { $count } मैसेज हमेशा के लिए मिटा दिए गए।
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] बातचीत को पढ़ा गया के रूप में मार्क किया गया।
       *[other] { $count } बातचीत को पढ़ा गया के रूप में मार्क किया गया।
    }
   *[message] { $count ->
        [one] मैसेज को पढ़ा गया के रूप में मार्क किया गया।
       *[other] { $count } मैसेज को पढ़ा गया के रूप में मार्क किया गया।
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] बातचीत को नहीं पढ़ा गया के रूप में मार्क किया गया।
       *[other] { $count } बातचीत को नहीं पढ़ा गया के रूप में मार्क किया गया।
    }
   *[message] { $count ->
        [one] मैसेज को नहीं पढ़ा गया के रूप में मार्क किया गया।
       *[other] { $count } मैसेज को नहीं पढ़ा गया के रूप में मार्क किया गया।
    }
}
toast-undone = कार्रवाई पहले जैसी कर दी गई।
toast-nothing-to-undo = पहले जैसा करने के लिए कुछ नहीं है।
toast-cannot-undo-delete-forever = हमेशा के लिए मिटाया गया मेल वापस नहीं लाया जा सकता।
toast-send-undone = भेजना पहले जैसा कर दिया गया।
toast-too-late-to-undo-send = पहले जैसा करने में बहुत देर हो गई: मैसेज पहले ही भेजा जा चुका है।
toast-undo = पहले जैसा करें
toast-close = बंद करें
toast-no-spam-folder = इस खाते में कोई स्पैम फ़ोल्डर नहीं है।
