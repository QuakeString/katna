# Katna Mail, Marathi (मराठी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = प्राथमिक
tab-promotions = जाहिराती
tab-social = सामाजिक
tab-updates = अपडेट
tab-forums = फोरम
tab-focused = फोकस्ड
tab-other = इतर
tab-inbox = इनबॉक्स
tab-newsletters = वृत्तपत्रे
tab-notifications = सूचना
tab-provider-other = Katna ने क्रमवारी लावलेले

## Mail list: toolbar

list-select = निवडा
list-refresh = रिफ्रेश करा
list-back-to-top = वर परत जा
list-checking = नवीन मेल तपासत आहे…
list-more = आणखी
list-mark-read = वाचलेले म्हणून खूण करा
list-mark-unread = न वाचलेले म्हणून खूण करा
list-move-to = येथे हलवा
list-archive = संग्रहित करा
list-spam = स्पॅमचा अहवाल द्या
list-delete = हटवा
list-snooze = स्नूझ करा
list-unsnooze = स्नूझ रद्द करा
list-newer = नवीन
list-older = जुने
list-range = { $total } पैकी { $first }–{ $last }
list-range-about = सुमारे { $total } पैकी { $first }–{ $last }
list-results = “{ $query }” साठी परिणाम
list-results-corrected = “{ $query }” साठी परिणाम दाखवत आहे
list-search-instead = त्याऐवजी “{ $query }” शोधा
list-files-more = +{ $count }
list-replied = तुम्ही उत्तर दिले

## Mail list: Select menu (which lines to tick)

list-pick-all = सर्व
list-pick-none = काहीही नाही
list-pick-read = वाचलेले
list-pick-unread = न वाचलेले
list-pick-starred = तारांकित
list-pick-unstarred = तारांकित नसलेले

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } संभाषण निवडले आहे.
       *[other] सर्व { $count } संभाषणे निवडली आहेत.
    }
   *[message] { $count ->
        [one] { $count } मेसेज निवडला आहे.
       *[other] सर्व { $count } मेसेज निवडले आहेत.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } मधील { $count } संभाषण निवडले आहे.
       *[other] { $folder } मधील सर्व { $count } संभाषणे निवडली आहेत.
    }
   *[message] { $count ->
        [one] { $folder } मधील { $count } मेसेज निवडला आहे.
       *[other] { $folder } मधील सर्व { $count } मेसेज निवडले आहेत.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] स्क्रीनवरील { $count } संभाषण निवडले आहे.
       *[other] स्क्रीनवरील सर्व { $count } संभाषणे निवडली आहेत.
    }
   *[message] { $count ->
        [one] स्क्रीनवरील { $count } मेसेज निवडला आहे.
       *[other] स्क्रीनवरील सर्व { $count } मेसेज निवडले आहेत.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } संभाषण निवडा
       *[other] सर्व { $count } संभाषणे निवडा
    }
   *[message] { $count ->
        [one] { $count } मेसेज निवडा
       *[other] सर्व { $count } मेसेज निवडा
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } मधील { $count } संभाषण निवडा
       *[other] { $folder } मधील सर्व { $count } संभाषणे निवडा
    }
   *[message] { $count ->
        [one] { $folder } मधील { $count } मेसेज निवडा
       *[other] { $folder } मधील सर्व { $count } मेसेज निवडा
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] स्क्रीनवरील { $count } वाचलेले संभाषण निवडले आहे.
           *[other] स्क्रीनवरील सर्व { $count } वाचलेली संभाषणे निवडली आहेत.
        }
       *[message] { $count ->
            [one] स्क्रीनवरील { $count } वाचलेला मेसेज निवडला आहे.
           *[other] स्क्रीनवरील सर्व { $count } वाचलेले मेसेज निवडले आहेत.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] स्क्रीनवरील { $count } न वाचलेले संभाषण निवडले आहे.
           *[other] स्क्रीनवरील सर्व { $count } न वाचलेली संभाषणे निवडली आहेत.
        }
       *[message] { $count ->
            [one] स्क्रीनवरील { $count } न वाचलेला मेसेज निवडला आहे.
           *[other] स्क्रीनवरील सर्व { $count } न वाचलेले मेसेज निवडले आहेत.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] स्क्रीनवरील { $count } तारांकित संभाषण निवडले आहे.
           *[other] स्क्रीनवरील सर्व { $count } तारांकित संभाषणे निवडली आहेत.
        }
       *[message] { $count ->
            [one] स्क्रीनवरील { $count } तारांकित मेसेज निवडला आहे.
           *[other] स्क्रीनवरील सर्व { $count } तारांकित मेसेज निवडले आहेत.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] स्क्रीनवरील { $count } तारांकित नसलेले संभाषण निवडले आहे.
           *[other] स्क्रीनवरील सर्व { $count } तारांकित नसलेली संभाषणे निवडली आहेत.
        }
       *[message] { $count ->
            [one] स्क्रीनवरील { $count } तारांकित नसलेला मेसेज निवडला आहे.
           *[other] स्क्रीनवरील सर्व { $count } तारांकित नसलेले मेसेज निवडले आहेत.
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } वाचलेले संभाषण निवडा
           *[other] सर्व { $count } वाचलेली संभाषणे निवडा
        }
       *[message] { $count ->
            [one] { $count } वाचलेला मेसेज निवडा
           *[other] सर्व { $count } वाचलेले मेसेज निवडा
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } न वाचलेले संभाषण निवडा
           *[other] सर्व { $count } न वाचलेली संभाषणे निवडा
        }
       *[message] { $count ->
            [one] { $count } न वाचलेला मेसेज निवडा
           *[other] सर्व { $count } न वाचलेले मेसेज निवडा
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } तारांकित संभाषण निवडा
           *[other] सर्व { $count } तारांकित संभाषणे निवडा
        }
       *[message] { $count ->
            [one] { $count } तारांकित मेसेज निवडा
           *[other] सर्व { $count } तारांकित मेसेज निवडा
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } तारांकित नसलेले संभाषण निवडा
           *[other] सर्व { $count } तारांकित नसलेली संभाषणे निवडा
        }
       *[message] { $count ->
            [one] { $count } तारांकित नसलेला मेसेज निवडा
           *[other] सर्व { $count } तारांकित नसलेले मेसेज निवडा
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder } मधील { $count } वाचलेले संभाषण निवडा
           *[other] { $folder } मधील सर्व { $count } वाचलेली संभाषणे निवडा
        }
       *[message] { $count ->
            [one] { $folder } मधील { $count } वाचलेला मेसेज निवडा
           *[other] { $folder } मधील सर्व { $count } वाचलेले मेसेज निवडा
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder } मधील { $count } न वाचलेले संभाषण निवडा
           *[other] { $folder } मधील सर्व { $count } न वाचलेली संभाषणे निवडा
        }
       *[message] { $count ->
            [one] { $folder } मधील { $count } न वाचलेला मेसेज निवडा
           *[other] { $folder } मधील सर्व { $count } न वाचलेले मेसेज निवडा
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } मधील { $count } तारांकित संभाषण निवडा
           *[other] { $folder } मधील सर्व { $count } तारांकित संभाषणे निवडा
        }
       *[message] { $count ->
            [one] { $folder } मधील { $count } तारांकित मेसेज निवडा
           *[other] { $folder } मधील सर्व { $count } तारांकित मेसेज निवडा
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } मधील { $count } तारांकित नसलेले संभाषण निवडा
           *[other] { $folder } मधील सर्व { $count } तारांकित नसलेली संभाषणे निवडा
        }
       *[message] { $count ->
            [one] { $folder } मधील { $count } तारांकित नसलेला मेसेज निवडा
           *[other] { $folder } मधील सर्व { $count } तारांकित नसलेले मेसेज निवडा
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } वाचलेले संभाषण निवडले आहे.
           *[other] सर्व { $count } वाचलेली संभाषणे निवडली आहेत.
        }
       *[message] { $count ->
            [one] { $count } वाचलेला मेसेज निवडला आहे.
           *[other] सर्व { $count } वाचलेले मेसेज निवडले आहेत.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } न वाचलेले संभाषण निवडले आहे.
           *[other] सर्व { $count } न वाचलेली संभाषणे निवडली आहेत.
        }
       *[message] { $count ->
            [one] { $count } न वाचलेला मेसेज निवडला आहे.
           *[other] सर्व { $count } न वाचलेले मेसेज निवडले आहेत.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } तारांकित संभाषण निवडले आहे.
           *[other] सर्व { $count } तारांकित संभाषणे निवडली आहेत.
        }
       *[message] { $count ->
            [one] { $count } तारांकित मेसेज निवडला आहे.
           *[other] सर्व { $count } तारांकित मेसेज निवडले आहेत.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } तारांकित नसलेले संभाषण निवडले आहे.
           *[other] सर्व { $count } तारांकित नसलेली संभाषणे निवडली आहेत.
        }
       *[message] { $count ->
            [one] { $count } तारांकित नसलेला मेसेज निवडला आहे.
           *[other] सर्व { $count } तारांकित नसलेले मेसेज निवडले आहेत.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder } मधील { $count } वाचलेले संभाषण निवडले आहे.
           *[other] { $folder } मधील सर्व { $count } वाचलेली संभाषणे निवडली आहेत.
        }
       *[message] { $count ->
            [one] { $folder } मधील { $count } वाचलेला मेसेज निवडला आहे.
           *[other] { $folder } मधील सर्व { $count } वाचलेले मेसेज निवडले आहेत.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder } मधील { $count } न वाचलेले संभाषण निवडले आहे.
           *[other] { $folder } मधील सर्व { $count } न वाचलेली संभाषणे निवडली आहेत.
        }
       *[message] { $count ->
            [one] { $folder } मधील { $count } न वाचलेला मेसेज निवडला आहे.
           *[other] { $folder } मधील सर्व { $count } न वाचलेले मेसेज निवडले आहेत.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } मधील { $count } तारांकित संभाषण निवडले आहे.
           *[other] { $folder } मधील सर्व { $count } तारांकित संभाषणे निवडली आहेत.
        }
       *[message] { $count ->
            [one] { $folder } मधील { $count } तारांकित मेसेज निवडला आहे.
           *[other] { $folder } मधील सर्व { $count } तारांकित मेसेज निवडले आहेत.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } मधील { $count } तारांकित नसलेले संभाषण निवडले आहे.
           *[other] { $folder } मधील सर्व { $count } तारांकित नसलेली संभाषणे निवडली आहेत.
        }
       *[message] { $count ->
            [one] { $folder } मधील { $count } तारांकित नसलेला मेसेज निवडला आहे.
           *[other] { $folder } मधील सर्व { $count } तारांकित नसलेले मेसेज निवडले आहेत.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] इथे वाचलेली संभाषणे नाहीत.
       *[message] इथे वाचलेले मेसेज नाहीत.
    }
   *[unread] { $kind ->
        [conversation] इथे न वाचलेली संभाषणे नाहीत.
       *[message] इथे न वाचलेले मेसेज नाहीत.
    }
    [starred] { $kind ->
        [conversation] इथे तारांकित संभाषणे नाहीत.
       *[message] इथे तारांकित मेसेज नाहीत.
    }
    [unstarred] { $kind ->
        [conversation] इथे तारांकित नसलेली संभाषणे नाहीत.
       *[message] इथे तारांकित नसलेले मेसेज नाहीत.
    }
}
list-clear-selection = निवड साफ करा

## Mail list: empty states

list-empty-search = तुमच्या शोधाशी कोणताही मेसेज जुळला नाही.
list-empty-tab = { $tab } मध्ये कोणताही मेल नाही.
list-empty-tab-unknown = या टॅबमध्ये कोणताही मेल नाही.
list-empty-folder = { $folder } मध्ये कोणताही मेसेज नाही.
list-empty-folder-unknown = या फोल्डरमध्ये कोणताही मेसेज नाही.
list-first-sync = तुमचा मेल आणत आहे…
list-first-sync-detail = मेल येईल तसा इथे दिसेल.

## Mail list: lines

row-removed = हा मेसेज काढून टाकला.
row-starred = तारांकित
row-not-starred = तारांकित नाही
row-important = महत्त्वाचे. महत्त्वाचे नाही म्हणून खूण करण्यासाठी क्लिक करा.
row-mark-important = महत्त्वाचे म्हणून खूण करा
row-pinned = सर्वात वर पिन केलेले
row-task = कार्य
row-task-open = कार्य उघडा: { $title }
row-tracking-none = ट्रॅक केलेले. अजून उघडलेले नाही
row-tracking-opened = { $recipients } पैकी { $opened } जणांनी उघडले
row-tracking-clicked = { $recipients } पैकी { $opened } जणांनी उघडले, { $clicked } जणांनी लिंक उघडली
row-pin = सर्वात वर पिन करा
row-unpin = अनपिन करा
row-snoozed-until = { $when } पर्यंत स्नूझ केले

## Mail list: More menu and right-click menu

menu-reply = उत्तर द्या
menu-reply-all = सर्वांना उत्तर द्या
menu-forward = फॉरवर्ड करा
menu-archive = संग्रहित करा
menu-delete = हटवा
menu-delete-forever = कायमचे हटवा
menu-move-to-inbox = इनबॉक्समध्ये हलवा
menu-spam = स्पॅमचा अहवाल द्या
menu-not-spam = स्पॅम नाही
menu-mark-read = वाचलेले म्हणून खूण करा
menu-mark-unread = न वाचलेले म्हणून खूण करा
menu-mark-all-read = सर्व वाचलेले म्हणून खूण करा
menu-star = तारांकित करा
menu-unstar = तारांकन काढा
menu-important = महत्त्वाचे म्हणून खूण करा
menu-not-important = महत्त्वाचे नाही म्हणून खूण करा
menu-pin = सर्वात वर पिन करा
menu-unpin = अनपिन करा
menu-snooze = स्नूझ करा
menu-unsnooze = स्नूझ रद्द करा
menu-add-to-tasks = कार्यांमध्ये जोडा
menu-schedule-meeting = मीटिंग शेड्यूल करा
menu-start-call = व्हिडिओ कॉल सुरू करा
menu-add-note = नोट जोडा
menu-print-all = सर्व प्रिंट करा
menu-new-window = नवीन विंडोमध्ये उघडा
menu-move-to = येथे हलवा
# Opens a submenu: Add to Tasks, Add a note, Schedule a meeting and Start a
# video call.
menu-follow-up = पाठपुरावा
# Opens a submenu of the rarer actions: Report spam, Mark as important and
# Pin to top.
menu-more = अधिक
menu-move-to-heading = येथे हलवा:
menu-find-from = { $name } कडून आलेले ईमेल शोधा

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] संभाषण संग्रहित केले.
       *[other] { $count } संभाषणे संग्रहित केली.
    }
   *[message] { $count ->
        [one] मेसेज संग्रहित केला.
       *[other] { $count } मेसेज संग्रहित केले.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] संभाषण कचरापेटीत हलवले.
       *[other] { $count } संभाषणे कचरापेटीत हलवली.
    }
   *[message] { $count ->
        [one] मेसेज कचरापेटीत हलवला.
       *[other] { $count } मेसेज कचरापेटीत हलवले.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] संभाषण हलवले.
       *[other] { $count } संभाषणे हलवली.
    }
   *[message] { $count ->
        [one] मेसेज हलवला.
       *[other] { $count } मेसेज हलवले.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] संभाषण तारांकित केले.
       *[other] { $count } संभाषणे तारांकित केली.
    }
   *[message] { $count ->
        [one] मेसेज तारांकित केला.
       *[other] { $count } मेसेज तारांकित केले.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] संभाषणावरील तारांकन काढले.
       *[other] { $count } संभाषणांवरील तारांकन काढले.
    }
   *[message] { $count ->
        [one] मेसेजवरील तारांकन काढले.
       *[other] { $count } मेसेजवरील तारांकन काढले.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] संभाषणावर महत्त्वाचे म्हणून खूण केली.
       *[other] { $count } संभाषणांवर महत्त्वाचे म्हणून खूण केली.
    }
   *[message] { $count ->
        [one] मेसेजवर महत्त्वाचे म्हणून खूण केली.
       *[other] { $count } मेसेजवर महत्त्वाचे म्हणून खूण केली.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] संभाषणावर महत्त्वाचे नाही म्हणून खूण केली.
       *[other] { $count } संभाषणांवर महत्त्वाचे नाही म्हणून खूण केली.
    }
   *[message] { $count ->
        [one] मेसेजवर महत्त्वाचे नाही म्हणून खूण केली.
       *[other] { $count } मेसेजवर महत्त्वाचे नाही म्हणून खूण केली.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] संभाषण सर्वात वर पिन केले.
       *[other] { $count } संभाषणे सर्वात वर पिन केली.
    }
   *[message] { $count ->
        [one] मेसेज सर्वात वर पिन केला.
       *[other] { $count } मेसेज सर्वात वर पिन केले.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] संभाषण अनपिन केले.
       *[other] { $count } संभाषणे अनपिन केली.
    }
   *[message] { $count ->
        [one] मेसेज अनपिन केला.
       *[other] { $count } मेसेज अनपिन केले.
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] संभाषण { $when } पर्यंत स्नूझ केले.
       *[other] { $count } संभाषणे { $when } पर्यंत स्नूझ केली.
    }
   *[message] { $count ->
        [one] मेसेज { $when } पर्यंत स्नूझ केला.
       *[other] { $count } मेसेज { $when } पर्यंत स्नूझ केले.
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] संभाषण इनबॉक्समध्ये परत आले.
       *[other] { $count } संभाषणे इनबॉक्समध्ये परत आली.
    }
   *[message] { $count ->
        [one] मेसेज इनबॉक्समध्ये परत आला.
       *[other] { $count } मेसेज इनबॉक्समध्ये परत आले.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] संभाषणाचा स्पॅम म्हणून अहवाल दिला.
       *[other] { $count } संभाषणांचा स्पॅम म्हणून अहवाल दिला.
    }
   *[message] { $count ->
        [one] मेसेजचा स्पॅम म्हणून अहवाल दिला.
       *[other] { $count } मेसेजचा स्पॅम म्हणून अहवाल दिला.
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] संभाषण स्पॅम नाही म्हणून चिन्हांकित करून इनबॉक्समध्ये हलवले.
       *[other] { $count } संभाषणे स्पॅम नाही म्हणून चिन्हांकित करून इनबॉक्समध्ये हलवली.
    }
   *[message] { $count ->
        [one] मेसेज स्पॅम नाही म्हणून चिन्हांकित करून इनबॉक्समध्ये हलवला.
       *[other] { $count } मेसेज स्पॅम नाही म्हणून चिन्हांकित करून इनबॉक्समध्ये हलवले.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] संभाषण कायमचे हटवले.
       *[other] { $count } संभाषणे कायमची हटवली.
    }
   *[message] { $count ->
        [one] मेसेज कायमचा हटवला.
       *[other] { $count } मेसेज कायमचे हटवले.
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] संभाषण वाचलेले म्हणून खूण केले.
       *[other] { $count } संभाषणे वाचलेली म्हणून खूण केली.
    }
   *[message] { $count ->
        [one] मेसेज वाचलेला म्हणून खूण केला.
       *[other] { $count } मेसेज वाचलेले म्हणून खूण केले.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] संभाषण न वाचलेले म्हणून खूण केले.
       *[other] { $count } संभाषणे न वाचलेली म्हणून खूण केली.
    }
   *[message] { $count ->
        [one] मेसेज न वाचलेला म्हणून खूण केला.
       *[other] { $count } मेसेज न वाचलेले म्हणून खूण केले.
    }
}
toast-undone = कृती पूर्ववत केली.
toast-nothing-to-undo = पूर्ववत करण्यासारखे काहीही नाही.
toast-cannot-undo-delete-forever = कायमचा हटवलेला मेल परत आणता येत नाही.
toast-send-undone = पाठवणे पूर्ववत केले.
toast-too-late-to-undo-send = पूर्ववत करायला उशीर झाला: मेसेज आधीच पाठवला गेला आहे.
toast-undo = पूर्ववत करा
toast-close = बंद करा
toast-no-spam-folder = या खात्यात स्पॅम फोल्डर नाही.
