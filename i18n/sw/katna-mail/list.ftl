# Katna Mail, Swahili (Kiswahili).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Msingi
tab-promotions = Matangazo
tab-social = Mitandao ya kijamii
tab-updates = Taarifa
tab-forums = Mijadala
tab-focused = Zilizolengwa
tab-other = Nyingine
tab-inbox = Kikasha
tab-newsletters = Majarida
tab-notifications = Arifa
tab-provider-other = zimepangwa na Katna

## Mail list: toolbar

list-select = Chagua
list-refresh = Onyesha upya
list-back-to-top = Rudi juu
list-checking = Inakagua barua mpya…
list-more = Zaidi
list-mark-read = Tia alama kuwa imesomwa
list-mark-unread = Tia alama kuwa haijasomwa
list-move-to = Hamishia
list-archive = Weka kwenye kumbukumbu
list-spam = Ripoti taka
list-delete = Futa
list-snooze = Ahirisha
list-unsnooze = Acha kuahirisha
list-newer = Mpya zaidi
list-older = Za zamani zaidi
list-range = { $first }–{ $last } kati ya { $total }
list-range-about = { $first }–{ $last } kati ya takriban { $total }
list-results = Matokeo ya “{ $query }”
list-results-corrected = Inaonyesha matokeo ya “{ $query }”
list-search-instead = Badala yake tafuta “{ $query }”
list-files-more = +{ $count }
list-replied = Ulijibu

## Mail list: Select menu (which lines to tick)

list-pick-all = Zote
list-pick-none = Hakuna
list-pick-read = Zilizosomwa
list-pick-unread = Ambazo hazijasomwa
list-pick-starred = Zenye nyota
list-pick-unstarred = Zisizo na nyota

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yote { $count } yamechaguliwa.
       *[other] Mazungumzo yote { $count } yamechaguliwa.
    }
   *[message] { $count ->
        [one] Ujumbe wote { $count } umechaguliwa.
       *[other] Jumbe zote { $count } zimechaguliwa.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yote { $count } katika { $folder } yamechaguliwa.
       *[other] Mazungumzo yote { $count } katika { $folder } yamechaguliwa.
    }
   *[message] { $count ->
        [one] Ujumbe wote { $count } katika { $folder } umechaguliwa.
       *[other] Jumbe zote { $count } katika { $folder } zimechaguliwa.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yote { $count } kwenye skrini yamechaguliwa.
       *[other] Mazungumzo yote { $count } kwenye skrini yamechaguliwa.
    }
   *[message] { $count ->
        [one] Ujumbe wote { $count } kwenye skrini umechaguliwa.
       *[other] Jumbe zote { $count } kwenye skrini zimechaguliwa.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Chagua mazungumzo yote { $count }
       *[other] Chagua mazungumzo yote { $count }
    }
   *[message] { $count ->
        [one] Chagua ujumbe wote { $count }
       *[other] Chagua jumbe zote { $count }
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Chagua mazungumzo yote { $count } katika { $folder }
       *[other] Chagua mazungumzo yote { $count } katika { $folder }
    }
   *[message] { $count ->
        [one] Chagua ujumbe wote { $count } katika { $folder }
       *[other] Chagua jumbe zote { $count } katika { $folder }
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Mazungumzo { $count } yaliyosomwa kwenye skrini yamechaguliwa.
           *[other] Mazungumzo yote { $count } yaliyosomwa kwenye skrini yamechaguliwa.
        }
       *[message] { $count ->
            [one] Ujumbe { $count } uliosomwa kwenye skrini umechaguliwa.
           *[other] Jumbe zote { $count } zilizosomwa kwenye skrini zimechaguliwa.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Mazungumzo { $count } ambayo hayajasomwa kwenye skrini yamechaguliwa.
           *[other] Mazungumzo yote { $count } ambayo hayajasomwa kwenye skrini yamechaguliwa.
        }
       *[message] { $count ->
            [one] Ujumbe { $count } ambao haujasomwa kwenye skrini umechaguliwa.
           *[other] Jumbe zote { $count } ambazo hazijasomwa kwenye skrini zimechaguliwa.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Mazungumzo { $count } yenye nyota kwenye skrini yamechaguliwa.
           *[other] Mazungumzo yote { $count } yenye nyota kwenye skrini yamechaguliwa.
        }
       *[message] { $count ->
            [one] Ujumbe { $count } wenye nyota kwenye skrini umechaguliwa.
           *[other] Jumbe zote { $count } zenye nyota kwenye skrini zimechaguliwa.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Mazungumzo { $count } yasiyo na nyota kwenye skrini yamechaguliwa.
           *[other] Mazungumzo yote { $count } yasiyo na nyota kwenye skrini yamechaguliwa.
        }
       *[message] { $count ->
            [one] Ujumbe { $count } usio na nyota kwenye skrini umechaguliwa.
           *[other] Jumbe zote { $count } zisizo na nyota kwenye skrini zimechaguliwa.
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Chagua mazungumzo { $count } yaliyosomwa
           *[other] Chagua mazungumzo yote { $count } yaliyosomwa
        }
       *[message] { $count ->
            [one] Chagua ujumbe { $count } uliosomwa
           *[other] Chagua jumbe zote { $count } zilizosomwa
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Chagua mazungumzo { $count } ambayo hayajasomwa
           *[other] Chagua mazungumzo yote { $count } ambayo hayajasomwa
        }
       *[message] { $count ->
            [one] Chagua ujumbe { $count } ambao haujasomwa
           *[other] Chagua jumbe zote { $count } ambazo hazijasomwa
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Chagua mazungumzo { $count } yenye nyota
           *[other] Chagua mazungumzo yote { $count } yenye nyota
        }
       *[message] { $count ->
            [one] Chagua ujumbe { $count } wenye nyota
           *[other] Chagua jumbe zote { $count } zenye nyota
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Chagua mazungumzo { $count } yasiyo na nyota
           *[other] Chagua mazungumzo yote { $count } yasiyo na nyota
        }
       *[message] { $count ->
            [one] Chagua ujumbe { $count } usio na nyota
           *[other] Chagua jumbe zote { $count } zisizo na nyota
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Chagua mazungumzo { $count } yaliyosomwa katika { $folder }
           *[other] Chagua mazungumzo yote { $count } yaliyosomwa katika { $folder }
        }
       *[message] { $count ->
            [one] Chagua ujumbe { $count } uliosomwa katika { $folder }
           *[other] Chagua jumbe zote { $count } zilizosomwa katika { $folder }
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Chagua mazungumzo { $count } ambayo hayajasomwa katika { $folder }
           *[other] Chagua mazungumzo yote { $count } ambayo hayajasomwa katika { $folder }
        }
       *[message] { $count ->
            [one] Chagua ujumbe { $count } ambao haujasomwa katika { $folder }
           *[other] Chagua jumbe zote { $count } ambazo hazijasomwa katika { $folder }
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Chagua mazungumzo { $count } yenye nyota katika { $folder }
           *[other] Chagua mazungumzo yote { $count } yenye nyota katika { $folder }
        }
       *[message] { $count ->
            [one] Chagua ujumbe { $count } wenye nyota katika { $folder }
           *[other] Chagua jumbe zote { $count } zenye nyota katika { $folder }
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Chagua mazungumzo { $count } yasiyo na nyota katika { $folder }
           *[other] Chagua mazungumzo yote { $count } yasiyo na nyota katika { $folder }
        }
       *[message] { $count ->
            [one] Chagua ujumbe { $count } usio na nyota katika { $folder }
           *[other] Chagua jumbe zote { $count } zisizo na nyota katika { $folder }
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Mazungumzo { $count } yaliyosomwa yamechaguliwa.
           *[other] Mazungumzo yote { $count } yaliyosomwa yamechaguliwa.
        }
       *[message] { $count ->
            [one] Ujumbe { $count } uliosomwa umechaguliwa.
           *[other] Jumbe zote { $count } zilizosomwa zimechaguliwa.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Mazungumzo { $count } ambayo hayajasomwa yamechaguliwa.
           *[other] Mazungumzo yote { $count } ambayo hayajasomwa yamechaguliwa.
        }
       *[message] { $count ->
            [one] Ujumbe { $count } ambao haujasomwa umechaguliwa.
           *[other] Jumbe zote { $count } ambazo hazijasomwa zimechaguliwa.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Mazungumzo { $count } yenye nyota yamechaguliwa.
           *[other] Mazungumzo yote { $count } yenye nyota yamechaguliwa.
        }
       *[message] { $count ->
            [one] Ujumbe { $count } wenye nyota umechaguliwa.
           *[other] Jumbe zote { $count } zenye nyota zimechaguliwa.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Mazungumzo { $count } yasiyo na nyota yamechaguliwa.
           *[other] Mazungumzo yote { $count } yasiyo na nyota yamechaguliwa.
        }
       *[message] { $count ->
            [one] Ujumbe { $count } usio na nyota umechaguliwa.
           *[other] Jumbe zote { $count } zisizo na nyota zimechaguliwa.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Mazungumzo { $count } yaliyosomwa katika { $folder } yamechaguliwa.
           *[other] Mazungumzo yote { $count } yaliyosomwa katika { $folder } yamechaguliwa.
        }
       *[message] { $count ->
            [one] Ujumbe { $count } uliosomwa katika { $folder } umechaguliwa.
           *[other] Jumbe zote { $count } zilizosomwa katika { $folder } zimechaguliwa.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Mazungumzo { $count } ambayo hayajasomwa katika { $folder } yamechaguliwa.
           *[other] Mazungumzo yote { $count } ambayo hayajasomwa katika { $folder } yamechaguliwa.
        }
       *[message] { $count ->
            [one] Ujumbe { $count } ambao haujasomwa katika { $folder } umechaguliwa.
           *[other] Jumbe zote { $count } ambazo hazijasomwa katika { $folder } zimechaguliwa.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Mazungumzo { $count } yenye nyota katika { $folder } yamechaguliwa.
           *[other] Mazungumzo yote { $count } yenye nyota katika { $folder } yamechaguliwa.
        }
       *[message] { $count ->
            [one] Ujumbe { $count } wenye nyota katika { $folder } umechaguliwa.
           *[other] Jumbe zote { $count } zenye nyota katika { $folder } zimechaguliwa.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Mazungumzo { $count } yasiyo na nyota katika { $folder } yamechaguliwa.
           *[other] Mazungumzo yote { $count } yasiyo na nyota katika { $folder } yamechaguliwa.
        }
       *[message] { $count ->
            [one] Ujumbe { $count } usio na nyota katika { $folder } umechaguliwa.
           *[other] Jumbe zote { $count } zisizo na nyota katika { $folder } zimechaguliwa.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Hakuna mazungumzo yaliyosomwa hapa.
       *[message] Hakuna jumbe zilizosomwa hapa.
    }
   *[unread] { $kind ->
        [conversation] Hakuna mazungumzo ambayo hayajasomwa hapa.
       *[message] Hakuna jumbe ambazo hazijasomwa hapa.
    }
    [starred] { $kind ->
        [conversation] Hakuna mazungumzo yenye nyota hapa.
       *[message] Hakuna jumbe zenye nyota hapa.
    }
    [unstarred] { $kind ->
        [conversation] Hakuna mazungumzo yasiyo na nyota hapa.
       *[message] Hakuna jumbe zisizo na nyota hapa.
    }
}
list-clear-selection = Futa uteuzi

## Mail list: empty states

list-empty-search = Hakuna ujumbe unaolingana na utafutaji wako.
list-empty-tab = Hakuna barua katika { $tab }.
list-empty-tab-unknown = Hakuna barua katika kichupo hiki.
list-empty-folder = Hakuna ujumbe katika { $folder }.
list-empty-folder-unknown = Hakuna ujumbe katika folda hii.
list-empty-waiting = Hakuna kinachosubiri jibu.
list-empty-reminders = Hakuna vikumbusho. Bonyeza H kwenye barua ili kuongeza.
list-first-sync = Inaleta barua zako…
list-first-sync-detail = Zitaonekana hapa zinapowasili.

## Mail list: lines

row-removed = Ujumbe huu uliondolewa.
row-starred = Ina nyota
row-not-starred = Haina nyota
row-important = Muhimu. Bofya ili kutia alama kuwa si muhimu.
row-mark-important = Tia alama kuwa muhimu
row-pinned = Imebandikwa juu
row-task = Jukumu
row-task-open = Fungua jukumu: { $title }
row-tracking-none = Unafuatiliwa. Bado haujafunguliwa
row-tracking-opened = Umefunguliwa na { $opened } kati ya { $recipients }
row-tracking-clicked = Umefunguliwa na { $opened } kati ya { $recipients }, kiungo kimefuatwa na { $clicked }
row-pin = Bandika juu
row-unpin = Bandua
row-snoozed-until = Imeahirishwa hadi { $when }
row-snoozed-day-time = { $day } { $time }
snoozed-group-today = Leo
snoozed-group-tomorrow = Kesho
snoozed-group-this-week = Wiki hii
snoozed-group-later = Baadaye
row-follow-up-step = Ufuatiliaji { $step } kati ya { $steps } · { $date }
row-follow-up-waiting = Ufuatiliaji unasubiri
row-reminder = Kikumbusho { $date }

## Mail list: More menu and right-click menu

menu-reply = Jibu
menu-reply-all = Jibu wote
menu-forward = Sambaza
menu-archive = Weka kwenye kumbukumbu
menu-delete = Futa
menu-delete-forever = Futa kabisa
menu-move-to-inbox = Hamishia Kikasha
menu-spam = Ripoti taka
menu-not-spam = Si taka
menu-mark-read = Tia alama kuwa imesomwa
menu-mark-unread = Tia alama kuwa haijasomwa
menu-mark-all-read = Tia alama zote kuwa zimesomwa
menu-star = Weka nyota
menu-unstar = Ondoa nyota
menu-important = Tia alama kuwa muhimu
menu-not-important = Tia alama kuwa si muhimu
menu-pin = Bandika juu
menu-unpin = Bandua
menu-snooze = Ahirisha
menu-remind = Nikumbushe
menu-unsnooze = Acha kuahirisha
menu-add-to-tasks = Ongeza kwenye Majukumu
menu-schedule-meeting = Panga mkutano
menu-start-call = Anzisha simu ya video
menu-add-note = Ongeza dokezo
menu-print-all = Chapisha zote
menu-new-window = Fungua katika dirisha jipya
menu-move-to = Hamishia
# Opens a submenu: Add to Tasks, Add a note, Schedule a meeting and Start a
# video call.
menu-follow-up = Fuatilia
# Opens a submenu of the rarer actions: Report spam, Mark as important and
# Pin to top.
menu-more = Zaidi
menu-move-to-heading = Hamishia:
menu-move-to-search = Hamishia…
menu-label-as = Weka lebo
menu-label-as-search = Weka lebo…
menu-no-folder = Hakuna folda inayoitwa “{ $name }”
menu-no-label = Hakuna lebo inayoitwa “{ $name }”
menu-create-folder = Unda “{ $name }”
menu-always-move = Hamishia hapa barua za { $name } kila mara
toast-always-move-failed = Barua imehamishwa, lakini sheria haikuundwa: { $error }
drag-mail = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo { $count }
       *[other] Mazungumzo { $count }
    }
   *[message] { $count ->
        [one] Ujumbe { $count }
       *[other] Jumbe { $count }
    }
}
menu-find-from = Tafuta barua pepe kutoka kwa { $name }
menu-make-rule = Unda sheria…

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yamewekwa kwenye kumbukumbu.
       *[other] Mazungumzo { $count } yamewekwa kwenye kumbukumbu.
    }
   *[message] { $count ->
        [one] Ujumbe umewekwa kwenye kumbukumbu.
       *[other] Jumbe { $count } zimewekwa kwenye kumbukumbu.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yamehamishiwa kwenye Tupio.
       *[other] Mazungumzo { $count } yamehamishiwa kwenye Tupio.
    }
   *[message] { $count ->
        [one] Ujumbe umehamishiwa kwenye Tupio.
       *[other] Jumbe { $count } zimehamishiwa kwenye Tupio.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yamehamishwa.
       *[other] Mazungumzo { $count } yamehamishwa.
    }
   *[message] { $count ->
        [one] Ujumbe umehamishwa.
       *[other] Jumbe { $count } zimehamishwa.
    }
}
toast-label-added = Lebo “{ $label }” imeongezwa.
toast-label-removed = Lebo “{ $label }” imeondolewa.
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yamewekewa nyota.
       *[other] Mazungumzo { $count } yamewekewa nyota.
    }
   *[message] { $count ->
        [one] Ujumbe umewekewa nyota.
       *[other] Jumbe { $count } zimewekewa nyota.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Nyota imeondolewa kwenye mazungumzo.
       *[other] Nyota imeondolewa kwenye mazungumzo { $count }.
    }
   *[message] { $count ->
        [one] Nyota imeondolewa kwenye ujumbe.
       *[other] Nyota imeondolewa kwenye jumbe { $count }.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yametiwa alama kuwa muhimu.
       *[other] Mazungumzo { $count } yametiwa alama kuwa muhimu.
    }
   *[message] { $count ->
        [one] Ujumbe umetiwa alama kuwa muhimu.
       *[other] Jumbe { $count } zimetiwa alama kuwa muhimu.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yametiwa alama kuwa si muhimu.
       *[other] Mazungumzo { $count } yametiwa alama kuwa si muhimu.
    }
   *[message] { $count ->
        [one] Ujumbe umetiwa alama kuwa si muhimu.
       *[other] Jumbe { $count } zimetiwa alama kuwa si muhimu.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yamebandikwa juu.
       *[other] Mazungumzo { $count } yamebandikwa juu.
    }
   *[message] { $count ->
        [one] Ujumbe umebandikwa juu.
       *[other] Jumbe { $count } zimebandikwa juu.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yamebanduliwa.
       *[other] Mazungumzo { $count } yamebanduliwa.
    }
   *[message] { $count ->
        [one] Ujumbe umebanduliwa.
       *[other] Jumbe { $count } zimebanduliwa.
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yameahirishwa hadi { $when }.
       *[other] Mazungumzo { $count } yameahirishwa hadi { $when }.
    }
   *[message] { $count ->
        [one] Ujumbe umeahirishwa hadi { $when }.
       *[other] Jumbe { $count } zimeahirishwa hadi { $when }.
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yamerudi kwenye Kikasha.
       *[other] Mazungumzo { $count } yamerudi kwenye Kikasha.
    }
   *[message] { $count ->
        [one] Ujumbe umerudi kwenye Kikasha.
       *[other] Jumbe { $count } zimerudi kwenye Kikasha.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yameripotiwa kuwa taka.
       *[other] Mazungumzo { $count } yameripotiwa kuwa taka.
    }
   *[message] { $count ->
        [one] Ujumbe umeripotiwa kuwa taka.
       *[other] Jumbe { $count } zimeripotiwa kuwa taka.
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yamewekwa alama kuwa si taka na kuhamishiwa kikasha.
       *[other] Mazungumzo { $count } yamewekwa alama kuwa si taka na kuhamishiwa kikasha.
    }
   *[message] { $count ->
        [one] Ujumbe umewekwa alama kuwa si taka na kuhamishiwa kikasha.
       *[other] Jumbe { $count } zimewekwa alama kuwa si taka na kuhamishiwa kikasha.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yamefutwa kabisa.
       *[other] Mazungumzo { $count } yamefutwa kabisa.
    }
   *[message] { $count ->
        [one] Ujumbe umefutwa kabisa.
       *[other] Jumbe { $count } zimefutwa kabisa.
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yamewekwa alama kuwa yamesomwa.
       *[other] Mazungumzo { $count } yamewekwa alama kuwa yamesomwa.
    }
   *[message] { $count ->
        [one] Ujumbe umewekwa alama kuwa umesomwa.
       *[other] Jumbe { $count } zimewekwa alama kuwa zimesomwa.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yamewekwa alama kuwa hayajasomwa.
       *[other] Mazungumzo { $count } yamewekwa alama kuwa hayajasomwa.
    }
   *[message] { $count ->
        [one] Ujumbe umewekwa alama kuwa haujasomwa.
       *[other] Jumbe { $count } zimewekwa alama kuwa hazijasomwa.
    }
}
toast-undone = Kitendo kimetenduliwa.
toast-nothing-to-undo = Hakuna cha kutendua.
toast-cannot-undo-delete-forever = Barua iliyofutwa kabisa haiwezi kurejeshwa.
toast-send-undone = Kutuma kumetenduliwa.
toast-too-late-to-undo-send = Imechelewa mno kutendua: ujumbe tayari umetumwa.
toast-undo = Tendua
toast-close = Funga
toast-no-spam-folder = Akaunti hii haina folda ya taka.
