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
tab-new = { $count } mpya
tab-provider-other = zimepangwa na Katna

## Mail list: toolbar

list-select = Chagua
list-refresh = Onyesha upya
list-more = Zaidi
list-mark-read = Tia alama kuwa imesomwa
list-mark-unread = Tia alama kuwa haijasomwa
list-move-to = Hamishia
list-archive = Weka kwenye kumbukumbu
list-spam = Ripoti taka
list-delete = Futa
list-newer = Mpya zaidi
list-older = Za zamani zaidi
list-range = { $first }–{ $last } kati ya { $total }
list-range-about = { $first }–{ $last } kati ya takriban { $total }
list-results = Matokeo ya “{ $query }”
list-results-corrected = Inaonyesha matokeo ya “{ $query }”
list-search-instead = Badala yake tafuta “{ $query }”
list-files-more = +{ $count }

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
list-clear-selection = Futa uteuzi

## Mail list: empty states

list-empty-search = Hakuna ujumbe unaolingana na utafutaji wako.
list-empty-tab = Hakuna barua katika { $tab }.
list-empty-tab-unknown = Hakuna barua katika kichupo hiki.
list-empty-folder = Hakuna ujumbe katika { $folder }.
list-empty-folder-unknown = Hakuna ujumbe katika folda hii.
list-first-sync = Inaleta barua zako…
list-first-sync-detail = Zitaonekana hapa zinapowasili.

## Mail list: lines

row-removed = Ujumbe huu uliondolewa.
row-starred = Ina nyota
row-not-starred = Haina nyota
row-important = Muhimu. Bofya ili kutia alama kuwa si muhimu.
row-mark-important = Tia alama kuwa muhimu
row-pinned = Imebandikwa juu
row-pin = Bandika juu
row-unpin = Bandua

## Mail list: More menu and right-click menu

menu-reply = Jibu
menu-reply-all = Jibu wote
menu-forward = Sambaza
menu-archive = Weka kwenye kumbukumbu
menu-delete = Futa
menu-spam = Ripoti taka
menu-mark-read = Tia alama kuwa imesomwa
menu-mark-unread = Tia alama kuwa haijasomwa
menu-mark-all-read = Tia alama zote kuwa zimesomwa
menu-star = Weka nyota
menu-unstar = Ondoa nyota
menu-important = Tia alama kuwa muhimu
menu-not-important = Tia alama kuwa si muhimu
menu-pin = Bandika juu
menu-unpin = Bandua
menu-print-all = Chapisha zote
menu-new-window = Fungua katika dirisha jipya
menu-move-to = Hamishia
menu-move-to-heading = Hamishia:
menu-find-from = Tafuta barua pepe kutoka kwa { $name }

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
toast-undone = Kitendo kimetenduliwa.
toast-undo = Tendua
toast-no-spam-folder = Akaunti hii haina folda ya taka.
