# Katna Mail, Nepali (नेपाली).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = प्राथमिक
tab-promotions = प्रमोसनहरू
tab-social = सामाजिक
tab-updates = अपडेटहरू
tab-forums = फोरमहरू
tab-focused = केन्द्रित
tab-other = अन्य
tab-inbox = इनबक्स
tab-newsletters = न्यूजलेटरहरू
tab-notifications = सूचनाहरू
tab-new = { $count } नयाँ
tab-provider-other = Katna द्वारा क्रमबद्ध

## Mail list: toolbar

list-select = चयन गर्नुहोस्
list-refresh = रिफ्रेस गर्नुहोस्
list-checking = नयाँ मेल जाँच गर्दै…
list-more = थप
list-mark-read = पढिएको भनी चिन्ह लगाउनुहोस्
list-mark-unread = नपढिएको भनी चिन्ह लगाउनुहोस्
list-move-to = यहाँ सार्नुहोस्
list-archive = संग्रह गर्नुहोस्
list-spam = स्प्याम भनी रिपोर्ट गर्नुहोस्
list-delete = मेटाउनुहोस्
list-snooze = स्नुज गर्नुहोस्
list-unsnooze = स्नुज हटाउनुहोस्
list-newer = नयाँ
list-older = पुरानो
list-range = { $total } मध्ये { $first }–{ $last }
list-range-about = लगभग { $total } मध्ये { $first }–{ $last }
list-results = “{ $query }” का लागि नतिजाहरू
list-results-corrected = “{ $query }” का लागि नतिजाहरू देखाउँदै
list-search-instead = यसको सट्टा “{ $query }” खोज्नुहोस्
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = सबै
list-pick-none = कुनै पनि होइन
list-pick-read = पढिएको
list-pick-unread = नपढिएको
list-pick-starred = तारा लगाइएको
list-pick-unstarred = तारा नलगाइएको

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] सबै { $count } वार्तालाप चयन गरिएको छ।
       *[other] सबै { $count } वार्तालापहरू चयन गरिएका छन्।
    }
   *[message] { $count ->
        [one] सबै { $count } सन्देश चयन गरिएको छ।
       *[other] सबै { $count } सन्देशहरू चयन गरिएका छन्।
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } का सबै { $count } वार्तालाप चयन गरिएको छ।
       *[other] { $folder } का सबै { $count } वार्तालापहरू चयन गरिएका छन्।
    }
   *[message] { $count ->
        [one] { $folder } का सबै { $count } सन्देश चयन गरिएको छ।
       *[other] { $folder } का सबै { $count } सन्देशहरू चयन गरिएका छन्।
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] स्क्रिनमा भएका सबै { $count } वार्तालाप चयन गरिएको छ।
       *[other] स्क्रिनमा भएका सबै { $count } वार्तालापहरू चयन गरिएका छन्।
    }
   *[message] { $count ->
        [one] स्क्रिनमा भएका सबै { $count } सन्देश चयन गरिएको छ।
       *[other] स्क्रिनमा भएका सबै { $count } सन्देशहरू चयन गरिएका छन्।
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] सबै { $count } वार्तालाप चयन गर्नुहोस्
       *[other] सबै { $count } वार्तालापहरू चयन गर्नुहोस्
    }
   *[message] { $count ->
        [one] सबै { $count } सन्देश चयन गर्नुहोस्
       *[other] सबै { $count } सन्देशहरू चयन गर्नुहोस्
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } का सबै { $count } वार्तालाप चयन गर्नुहोस्
       *[other] { $folder } का सबै { $count } वार्तालापहरू चयन गर्नुहोस्
    }
   *[message] { $count ->
        [one] { $folder } का सबै { $count } सन्देश चयन गर्नुहोस्
       *[other] { $folder } का सबै { $count } सन्देशहरू चयन गर्नुहोस्
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] स्क्रिनमा भएको { $count } पढिएको वार्तालाप चयन गरिएको छ।
           *[other] स्क्रिनमा भएका सबै { $count } पढिएका वार्तालापहरू चयन गरिएका छन्।
        }
       *[message] { $count ->
            [one] स्क्रिनमा भएको { $count } पढिएको सन्देश चयन गरिएको छ।
           *[other] स्क्रिनमा भएका सबै { $count } पढिएका सन्देशहरू चयन गरिएका छन्।
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] स्क्रिनमा भएको { $count } नपढिएको वार्तालाप चयन गरिएको छ।
           *[other] स्क्रिनमा भएका सबै { $count } नपढिएका वार्तालापहरू चयन गरिएका छन्।
        }
       *[message] { $count ->
            [one] स्क्रिनमा भएको { $count } नपढिएको सन्देश चयन गरिएको छ।
           *[other] स्क्रिनमा भएका सबै { $count } नपढिएका सन्देशहरू चयन गरिएका छन्।
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] स्क्रिनमा भएको { $count } तारा लगाइएको वार्तालाप चयन गरिएको छ।
           *[other] स्क्रिनमा भएका सबै { $count } तारा लगाइएका वार्तालापहरू चयन गरिएका छन्।
        }
       *[message] { $count ->
            [one] स्क्रिनमा भएको { $count } तारा लगाइएको सन्देश चयन गरिएको छ।
           *[other] स्क्रिनमा भएका सबै { $count } तारा लगाइएका सन्देशहरू चयन गरिएका छन्।
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] स्क्रिनमा भएको { $count } तारा नलगाइएको वार्तालाप चयन गरिएको छ।
           *[other] स्क्रिनमा भएका सबै { $count } तारा नलगाइएका वार्तालापहरू चयन गरिएका छन्।
        }
       *[message] { $count ->
            [one] स्क्रिनमा भएको { $count } तारा नलगाइएको सन्देश चयन गरिएको छ।
           *[other] स्क्रिनमा भएका सबै { $count } तारा नलगाइएका सन्देशहरू चयन गरिएका छन्।
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } पढिएको वार्तालाप चयन गर्नुहोस्
           *[other] सबै { $count } पढिएका वार्तालापहरू चयन गर्नुहोस्
        }
       *[message] { $count ->
            [one] { $count } पढिएको सन्देश चयन गर्नुहोस्
           *[other] सबै { $count } पढिएका सन्देशहरू चयन गर्नुहोस्
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } नपढिएको वार्तालाप चयन गर्नुहोस्
           *[other] सबै { $count } नपढिएका वार्तालापहरू चयन गर्नुहोस्
        }
       *[message] { $count ->
            [one] { $count } नपढिएको सन्देश चयन गर्नुहोस्
           *[other] सबै { $count } नपढिएका सन्देशहरू चयन गर्नुहोस्
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } तारा लगाइएको वार्तालाप चयन गर्नुहोस्
           *[other] सबै { $count } तारा लगाइएका वार्तालापहरू चयन गर्नुहोस्
        }
       *[message] { $count ->
            [one] { $count } तारा लगाइएको सन्देश चयन गर्नुहोस्
           *[other] सबै { $count } तारा लगाइएका सन्देशहरू चयन गर्नुहोस्
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } तारा नलगाइएको वार्तालाप चयन गर्नुहोस्
           *[other] सबै { $count } तारा नलगाइएका वार्तालापहरू चयन गर्नुहोस्
        }
       *[message] { $count ->
            [one] { $count } तारा नलगाइएको सन्देश चयन गर्नुहोस्
           *[other] सबै { $count } तारा नलगाइएका सन्देशहरू चयन गर्नुहोस्
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder } का { $count } पढिएको वार्तालाप चयन गर्नुहोस्
           *[other] { $folder } का सबै { $count } पढिएका वार्तालापहरू चयन गर्नुहोस्
        }
       *[message] { $count ->
            [one] { $folder } का { $count } पढिएको सन्देश चयन गर्नुहोस्
           *[other] { $folder } का सबै { $count } पढिएका सन्देशहरू चयन गर्नुहोस्
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder } का { $count } नपढिएको वार्तालाप चयन गर्नुहोस्
           *[other] { $folder } का सबै { $count } नपढिएका वार्तालापहरू चयन गर्नुहोस्
        }
       *[message] { $count ->
            [one] { $folder } का { $count } नपढिएको सन्देश चयन गर्नुहोस्
           *[other] { $folder } का सबै { $count } नपढिएका सन्देशहरू चयन गर्नुहोस्
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } का { $count } तारा लगाइएको वार्तालाप चयन गर्नुहोस्
           *[other] { $folder } का सबै { $count } तारा लगाइएका वार्तालापहरू चयन गर्नुहोस्
        }
       *[message] { $count ->
            [one] { $folder } का { $count } तारा लगाइएको सन्देश चयन गर्नुहोस्
           *[other] { $folder } का सबै { $count } तारा लगाइएका सन्देशहरू चयन गर्नुहोस्
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } का { $count } तारा नलगाइएको वार्तालाप चयन गर्नुहोस्
           *[other] { $folder } का सबै { $count } तारा नलगाइएका वार्तालापहरू चयन गर्नुहोस्
        }
       *[message] { $count ->
            [one] { $folder } का { $count } तारा नलगाइएको सन्देश चयन गर्नुहोस्
           *[other] { $folder } का सबै { $count } तारा नलगाइएका सन्देशहरू चयन गर्नुहोस्
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } पढिएको वार्तालाप चयन गरियो।
           *[other] सबै { $count } पढिएका वार्तालापहरू चयन गरिए।
        }
       *[message] { $count ->
            [one] { $count } पढिएको सन्देश चयन गरियो।
           *[other] सबै { $count } पढिएका सन्देशहरू चयन गरिए।
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } नपढिएको वार्तालाप चयन गरियो।
           *[other] सबै { $count } नपढिएका वार्तालापहरू चयन गरिए।
        }
       *[message] { $count ->
            [one] { $count } नपढिएको सन्देश चयन गरियो।
           *[other] सबै { $count } नपढिएका सन्देशहरू चयन गरिए।
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } तारा लगाइएको वार्तालाप चयन गरियो।
           *[other] सबै { $count } तारा लगाइएका वार्तालापहरू चयन गरिए।
        }
       *[message] { $count ->
            [one] { $count } तारा लगाइएको सन्देश चयन गरियो।
           *[other] सबै { $count } तारा लगाइएका सन्देशहरू चयन गरिए।
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } तारा नलगाइएको वार्तालाप चयन गरियो।
           *[other] सबै { $count } तारा नलगाइएका वार्तालापहरू चयन गरिए।
        }
       *[message] { $count ->
            [one] { $count } तारा नलगाइएको सन्देश चयन गरियो।
           *[other] सबै { $count } तारा नलगाइएका सन्देशहरू चयन गरिए।
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder } को { $count } पढिएको वार्तालाप चयन गरियो।
           *[other] { $folder } का सबै { $count } पढिएका वार्तालापहरू चयन गरिए।
        }
       *[message] { $count ->
            [one] { $folder } को { $count } पढिएको सन्देश चयन गरियो।
           *[other] { $folder } का सबै { $count } पढिएका सन्देशहरू चयन गरिए।
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder } को { $count } नपढिएको वार्तालाप चयन गरियो।
           *[other] { $folder } का सबै { $count } नपढिएका वार्तालापहरू चयन गरिए।
        }
       *[message] { $count ->
            [one] { $folder } को { $count } नपढिएको सन्देश चयन गरियो।
           *[other] { $folder } का सबै { $count } नपढिएका सन्देशहरू चयन गरिए।
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } को { $count } तारा लगाइएको वार्तालाप चयन गरियो।
           *[other] { $folder } का सबै { $count } तारा लगाइएका वार्तालापहरू चयन गरिए।
        }
       *[message] { $count ->
            [one] { $folder } को { $count } तारा लगाइएको सन्देश चयन गरियो।
           *[other] { $folder } का सबै { $count } तारा लगाइएका सन्देशहरू चयन गरिए।
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } को { $count } तारा नलगाइएको वार्तालाप चयन गरियो।
           *[other] { $folder } का सबै { $count } तारा नलगाइएका वार्तालापहरू चयन गरिए।
        }
       *[message] { $count ->
            [one] { $folder } को { $count } तारा नलगाइएको सन्देश चयन गरियो।
           *[other] { $folder } का सबै { $count } तारा नलगाइएका सन्देशहरू चयन गरिए।
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] यहाँ कुनै पढिएका वार्तालापहरू छैनन्।
       *[message] यहाँ कुनै पढिएका सन्देशहरू छैनन्।
    }
   *[unread] { $kind ->
        [conversation] यहाँ कुनै नपढिएका वार्तालापहरू छैनन्।
       *[message] यहाँ कुनै नपढिएका सन्देशहरू छैनन्।
    }
    [starred] { $kind ->
        [conversation] यहाँ कुनै तारा लगाइएका वार्तालापहरू छैनन्।
       *[message] यहाँ कुनै तारा लगाइएका सन्देशहरू छैनन्।
    }
    [unstarred] { $kind ->
        [conversation] यहाँ कुनै तारा नलगाइएका वार्तालापहरू छैनन्।
       *[message] यहाँ कुनै तारा नलगाइएका सन्देशहरू छैनन्।
    }
}
list-clear-selection = चयन हटाउनुहोस्

## Mail list: empty states

list-empty-search = तपाईंको खोजसँग मिल्ने कुनै सन्देश भेटिएन।
list-empty-tab = { $tab } मा कुनै मेल छैन।
list-empty-tab-unknown = यो ट्याबमा कुनै मेल छैन।
list-empty-folder = { $folder } मा कुनै सन्देश छैन।
list-empty-folder-unknown = यो फोल्डरमा कुनै सन्देश छैन।
list-first-sync = तपाईंको मेल ल्याउँदै…
list-first-sync-detail = मेल आइपुग्दै गर्दा यहाँ देखिन्छ।

## Mail list: lines

row-removed = यो सन्देश हटाइयो।
row-starred = तारा लगाइएको
row-not-starred = तारा नलगाइएको
row-important = महत्त्वपूर्ण। महत्त्वपूर्ण होइन भनी चिन्ह लगाउन क्लिक गर्नुहोस्।
row-mark-important = महत्त्वपूर्ण भनी चिन्ह लगाउनुहोस्
row-pinned = माथि पिन गरिएको
row-tracking-none = ट्र्याक गरिएको। अझै खोलिएको छैन
row-tracking-opened = { $recipients } मध्ये { $opened } जनाले खोले
row-tracking-clicked = { $recipients } मध्ये { $opened } जनाले खोले, { $clicked } जनाले लिङ्क खोले
row-pin = माथि पिन गर्नुहोस्
row-unpin = अनपिन गर्नुहोस्
row-snoozed-until = { $when } सम्म स्नुज गरिएको

## Mail list: More menu and right-click menu

menu-reply = जवाफ दिनुहोस्
menu-reply-all = सबैलाई जवाफ दिनुहोस्
menu-forward = फर्वार्ड गर्नुहोस्
menu-archive = संग्रह गर्नुहोस्
menu-delete = मेटाउनुहोस्
menu-delete-forever = सधैँका लागि मेटाउनुहोस्
menu-move-to-inbox = इनबक्समा सार्नुहोस्
menu-spam = स्प्याम भनी रिपोर्ट गर्नुहोस्
menu-not-spam = स्प्याम होइन
menu-mark-read = पढिएको भनी चिन्ह लगाउनुहोस्
menu-mark-unread = नपढिएको भनी चिन्ह लगाउनुहोस्
menu-mark-all-read = सबैलाई पढिएको भनी चिन्ह लगाउनुहोस्
menu-star = तारा लगाउनुहोस्
menu-unstar = तारा हटाउनुहोस्
menu-important = महत्त्वपूर्ण भनी चिन्ह लगाउनुहोस्
menu-not-important = महत्त्वपूर्ण होइन भनी चिन्ह लगाउनुहोस्
menu-pin = माथि पिन गर्नुहोस्
menu-unpin = अनपिन गर्नुहोस्
menu-snooze = स्नुज गर्नुहोस्
menu-unsnooze = स्नुज हटाउनुहोस्
menu-add-to-tasks = कार्यमा थप्नुहोस्
menu-print-all = सबै प्रिन्ट गर्नुहोस्
menu-new-window = नयाँ विन्डोमा खोल्नुहोस्
menu-move-to = यहाँ सार्नुहोस्
menu-move-to-heading = यहाँ सार्नुहोस्:
menu-find-from = { $name } बाट आएका इमेलहरू खोज्नुहोस्

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] वार्तालाप संग्रह गरियो।
       *[other] { $count } वार्तालापहरू संग्रह गरिए।
    }
   *[message] { $count ->
        [one] सन्देश संग्रह गरियो।
       *[other] { $count } सन्देशहरू संग्रह गरिए।
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] वार्तालाप ट्र्यासमा सारियो।
       *[other] { $count } वार्तालापहरू ट्र्यासमा सारिए।
    }
   *[message] { $count ->
        [one] सन्देश ट्र्यासमा सारियो।
       *[other] { $count } सन्देशहरू ट्र्यासमा सारिए।
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] वार्तालाप सारियो।
       *[other] { $count } वार्तालापहरू सारिए।
    }
   *[message] { $count ->
        [one] सन्देश सारियो।
       *[other] { $count } सन्देशहरू सारिए।
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] वार्तालापमा तारा लगाइयो।
       *[other] { $count } वार्तालापहरूमा तारा लगाइयो।
    }
   *[message] { $count ->
        [one] सन्देशमा तारा लगाइयो।
       *[other] { $count } सन्देशहरूमा तारा लगाइयो।
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] वार्तालापबाट तारा हटाइयो।
       *[other] { $count } वार्तालापहरूबाट तारा हटाइयो।
    }
   *[message] { $count ->
        [one] सन्देशबाट तारा हटाइयो।
       *[other] { $count } सन्देशहरूबाट तारा हटाइयो।
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] वार्तालापलाई महत्त्वपूर्ण भनी चिन्ह लगाइयो।
       *[other] { $count } वार्तालापहरूलाई महत्त्वपूर्ण भनी चिन्ह लगाइयो।
    }
   *[message] { $count ->
        [one] सन्देशलाई महत्त्वपूर्ण भनी चिन्ह लगाइयो।
       *[other] { $count } सन्देशहरूलाई महत्त्वपूर्ण भनी चिन्ह लगाइयो।
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] वार्तालापलाई महत्त्वपूर्ण होइन भनी चिन्ह लगाइयो।
       *[other] { $count } वार्तालापहरूलाई महत्त्वपूर्ण होइन भनी चिन्ह लगाइयो।
    }
   *[message] { $count ->
        [one] सन्देशलाई महत्त्वपूर्ण होइन भनी चिन्ह लगाइयो।
       *[other] { $count } सन्देशहरूलाई महत्त्वपूर्ण होइन भनी चिन्ह लगाइयो।
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] वार्तालाप माथि पिन गरियो।
       *[other] { $count } वार्तालापहरू माथि पिन गरिए।
    }
   *[message] { $count ->
        [one] सन्देश माथि पिन गरियो।
       *[other] { $count } सन्देशहरू माथि पिन गरिए।
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] वार्तालाप अनपिन गरियो।
       *[other] { $count } वार्तालापहरू अनपिन गरिए।
    }
   *[message] { $count ->
        [one] सन्देश अनपिन गरियो।
       *[other] { $count } सन्देशहरू अनपिन गरिए।
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] वार्तालाप { $when } सम्म स्नुज गरियो।
       *[other] { $count } वार्तालापहरू { $when } सम्म स्नुज गरिए।
    }
   *[message] { $count ->
        [one] सन्देश { $when } सम्म स्नुज गरियो।
       *[other] { $count } सन्देशहरू { $when } सम्म स्नुज गरिए।
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] वार्तालाप इनबक्समा फर्कियो।
       *[other] { $count } वार्तालापहरू इनबक्समा फर्किए।
    }
   *[message] { $count ->
        [one] सन्देश इनबक्समा फर्कियो।
       *[other] { $count } सन्देशहरू इनबक्समा फर्किए।
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] वार्तालापलाई स्प्याम भनी रिपोर्ट गरियो।
       *[other] { $count } वार्तालापहरूलाई स्प्याम भनी रिपोर्ट गरियो।
    }
   *[message] { $count ->
        [one] सन्देशलाई स्प्याम भनी रिपोर्ट गरियो।
       *[other] { $count } सन्देशहरूलाई स्प्याम भनी रिपोर्ट गरियो।
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] वार्तालापलाई स्प्याम होइन भनी चिन्ह लगाएर इनबक्समा सारियो।
       *[other] { $count } वार्तालापहरूलाई स्प्याम होइन भनी चिन्ह लगाएर इनबक्समा सारियो।
    }
   *[message] { $count ->
        [one] सन्देशलाई स्प्याम होइन भनी चिन्ह लगाएर इनबक्समा सारियो।
       *[other] { $count } सन्देशहरूलाई स्प्याम होइन भनी चिन्ह लगाएर इनबक्समा सारियो।
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] वार्तालाप सधैँका लागि मेटाइयो।
       *[other] { $count } वार्तालापहरू सधैँका लागि मेटाइए।
    }
   *[message] { $count ->
        [one] सन्देश सधैँका लागि मेटाइयो।
       *[other] { $count } सन्देशहरू सधैँका लागि मेटाइए।
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] वार्तालाप पढिएको भनी चिन्ह लगाइयो।
       *[other] { $count } वार्तालापहरू पढिएको भनी चिन्ह लगाइए।
    }
   *[message] { $count ->
        [one] सन्देश पढिएको भनी चिन्ह लगाइयो।
       *[other] { $count } सन्देशहरू पढिएको भनी चिन्ह लगाइए।
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] वार्तालाप नपढिएको भनी चिन्ह लगाइयो।
       *[other] { $count } वार्तालापहरू नपढिएको भनी चिन्ह लगाइए।
    }
   *[message] { $count ->
        [one] सन्देश नपढिएको भनी चिन्ह लगाइयो।
       *[other] { $count } सन्देशहरू नपढिएको भनी चिन्ह लगाइए।
    }
}
toast-undone = कार्य पूर्ववत गरियो।
toast-nothing-to-undo = पूर्ववत गर्न केही छैन।
toast-cannot-undo-delete-forever = सधैँका लागि मेटाइएको मेल फिर्ता ल्याउन सकिँदैन।
toast-send-undone = पठाउने कार्य पूर्ववत गरियो।
toast-too-late-to-undo-send = पूर्ववत गर्न ढिलो भयो: सन्देश पहिल्यै पठाइसकिएको छ।
toast-undo = पूर्ववत गर्नुहोस्
toast-close = बन्द गर्नुहोस्
toast-no-spam-folder = यो खातामा स्प्याम फोल्डर छैन।
