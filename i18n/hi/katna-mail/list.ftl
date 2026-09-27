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
tab-new = { $count } नए
tab-provider-other = Katna ने छांटा

## Mail list: toolbar

list-select = चुनें
list-refresh = रीफ़्रेश करें
list-more = ज़्यादा
list-mark-read = पढ़ा गया के रूप में मार्क करें
list-mark-unread = नहीं पढ़ा गया के रूप में मार्क करें
list-move-to = इसमें ले जाएं
list-archive = संग्रह करें
list-spam = स्पैम की शिकायत करें
list-delete = मिटाएं
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
row-pin = सबसे ऊपर पिन करें
row-unpin = अनपिन करें

## Mail list: More menu and right-click menu

menu-reply = जवाब दें
menu-reply-all = सभी को जवाब दें
menu-forward = फ़ॉरवर्ड करें
menu-archive = संग्रह करें
menu-delete = मिटाएं
menu-spam = स्पैम की शिकायत करें
menu-mark-read = पढ़ा गया के रूप में मार्क करें
menu-mark-unread = नहीं पढ़ा गया के रूप में मार्क करें
menu-mark-all-read = सभी को पढ़ा गया के रूप में मार्क करें
menu-star = तारांकित करें
menu-unstar = तारांकन हटाएं
menu-important = ज़रूरी के रूप में मार्क करें
menu-not-important = ज़रूरी नहीं के रूप में मार्क करें
menu-pin = सबसे ऊपर पिन करें
menu-unpin = अनपिन करें
menu-print-all = सभी प्रिंट करें
menu-new-window = नई विंडो में खोलें
menu-move-to = इसमें ले जाएं
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
toast-undone = कार्रवाई पहले जैसी कर दी गई।
toast-undo = पहले जैसा करें
toast-no-spam-folder = इस खाते में कोई स्पैम फ़ोल्डर नहीं है।
