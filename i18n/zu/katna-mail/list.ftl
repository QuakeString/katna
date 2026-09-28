# Katna Mail, Zulu (isiZulu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Okuyinhloko
tab-promotions = Ukukhangisa
tab-social = Ezenhlalo
tab-updates = Izibuyekezo
tab-forums = Izinkundla
tab-focused = Okugxilile
tab-other = Okunye
tab-inbox = Ibhokisi lokungenayo
tab-newsletters = Izincwadi zezindaba
tab-notifications = Izaziso
tab-new = { $count } okusha
tab-provider-other = kuhlelwe yi-Katna

## Mail list: toolbar

list-select = Khetha
list-refresh = Vuselela
list-more = Okuningi
list-mark-read = Maka njengokufundiwe
list-mark-unread = Maka njengokungafundiwe
list-move-to = Hambisa ku-
list-archive = Faka kungobo yomlando
list-spam = Bika ugaxekile
list-delete = Susa
list-snooze = Libazisa
list-unsnooze = Yeka ukulibazisa
list-newer = Okusha
list-older = Okudala
list-range = { $first }–{ $last } kokungu-{ $total }
list-range-about = { $first }–{ $last } kokucishe kube ngu-{ $total }
list-results = Imiphumela ye-“{ $query }”
list-results-corrected = Kuboniswa imiphumela ye-“{ $query }”
list-search-instead = Esikhundleni salokho sesha u-“{ $query }”
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Konke
list-pick-none = Lutho
list-pick-read = Okufundiwe
list-pick-unread = Okungafundiwe
list-pick-starred = Okunenkanyezi
list-pick-unstarred = Okungenankanyezi

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo engu-{ $count } ikhethiwe.
       *[other] Zonke izingxoxo ezingu-{ $count } zikhethiwe.
    }
   *[message] { $count ->
        [one] Umlayezo ongu-{ $count } ukhethiwe.
       *[other] Yonke imilayezo engu-{ $count } ikhethiwe.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo engu-{ $count } ku-{ $folder } ikhethiwe.
       *[other] Zonke izingxoxo ezingu-{ $count } ku-{ $folder } zikhethiwe.
    }
   *[message] { $count ->
        [one] Umlayezo ongu-{ $count } ku-{ $folder } ukhethiwe.
       *[other] Yonke imilayezo engu-{ $count } ku-{ $folder } ikhethiwe.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo engu-{ $count } esesikrinini ikhethiwe.
       *[other] Zonke izingxoxo ezingu-{ $count } ezisesikrinini zikhethiwe.
    }
   *[message] { $count ->
        [one] Umlayezo ongu-{ $count } osesikrinini ukhethiwe.
       *[other] Yonke imilayezo engu-{ $count } esesikrinini ikhethiwe.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Khetha ingxoxo engu-{ $count }
       *[other] Khetha zonke izingxoxo ezingu-{ $count }
    }
   *[message] { $count ->
        [one] Khetha umlayezo ongu-{ $count }
       *[other] Khetha yonke imilayezo engu-{ $count }
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Khetha ingxoxo engu-{ $count } ku-{ $folder }
       *[other] Khetha zonke izingxoxo ezingu-{ $count } ku-{ $folder }
    }
   *[message] { $count ->
        [one] Khetha umlayezo ongu-{ $count } ku-{ $folder }
       *[other] Khetha yonke imilayezo engu-{ $count } ku-{ $folder }
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Ingxoxo efundiwe engu-{ $count } esesikrinini ikhethiwe.
           *[other] Zonke izingxoxo ezifundiwe ezingu-{ $count } ezisesikrinini zikhethiwe.
        }
       *[message] { $count ->
            [one] Umlayezo ofundiwe ongu-{ $count } osesikrinini ukhethiwe.
           *[other] Yonke imilayezo efundiwe engu-{ $count } esesikrinini ikhethiwe.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Ingxoxo engafundiwe engu-{ $count } esesikrinini ikhethiwe.
           *[other] Zonke izingxoxo ezingafundiwe ezingu-{ $count } ezisesikrinini zikhethiwe.
        }
       *[message] { $count ->
            [one] Umlayezo ongafundiwe ongu-{ $count } osesikrinini ukhethiwe.
           *[other] Yonke imilayezo engafundiwe engu-{ $count } esesikrinini ikhethiwe.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Ingxoxo enenkanyezi engu-{ $count } esesikrinini ikhethiwe.
           *[other] Zonke izingxoxo ezinenkanyezi ezingu-{ $count } ezisesikrinini zikhethiwe.
        }
       *[message] { $count ->
            [one] Umlayezo onenkanyezi ongu-{ $count } osesikrinini ukhethiwe.
           *[other] Yonke imilayezo enenkanyezi engu-{ $count } esesikrinini ikhethiwe.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Ingxoxo engenankanyezi engu-{ $count } esesikrinini ikhethiwe.
           *[other] Zonke izingxoxo ezingenankanyezi ezingu-{ $count } ezisesikrinini zikhethiwe.
        }
       *[message] { $count ->
            [one] Umlayezo ongenankanyezi ongu-{ $count } osesikrinini ukhethiwe.
           *[other] Yonke imilayezo engenankanyezi engu-{ $count } esesikrinini ikhethiwe.
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Khetha ingxoxo efundiwe engu-{ $count }
           *[other] Khetha zonke izingxoxo ezifundiwe ezingu-{ $count }
        }
       *[message] { $count ->
            [one] Khetha umlayezo ofundiwe ongu-{ $count }
           *[other] Khetha yonke imilayezo efundiwe engu-{ $count }
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Khetha ingxoxo engafundiwe engu-{ $count }
           *[other] Khetha zonke izingxoxo ezingafundiwe ezingu-{ $count }
        }
       *[message] { $count ->
            [one] Khetha umlayezo ongafundiwe ongu-{ $count }
           *[other] Khetha yonke imilayezo engafundiwe engu-{ $count }
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Khetha ingxoxo enenkanyezi engu-{ $count }
           *[other] Khetha zonke izingxoxo ezinenkanyezi ezingu-{ $count }
        }
       *[message] { $count ->
            [one] Khetha umlayezo onenkanyezi ongu-{ $count }
           *[other] Khetha yonke imilayezo enenkanyezi engu-{ $count }
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Khetha ingxoxo engenankanyezi engu-{ $count }
           *[other] Khetha zonke izingxoxo ezingenankanyezi ezingu-{ $count }
        }
       *[message] { $count ->
            [one] Khetha umlayezo ongenankanyezi ongu-{ $count }
           *[other] Khetha yonke imilayezo engenankanyezi engu-{ $count }
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Khetha ingxoxo efundiwe engu-{ $count } ku-{ $folder }
           *[other] Khetha zonke izingxoxo ezifundiwe ezingu-{ $count } ku-{ $folder }
        }
       *[message] { $count ->
            [one] Khetha umlayezo ofundiwe ongu-{ $count } ku-{ $folder }
           *[other] Khetha yonke imilayezo efundiwe engu-{ $count } ku-{ $folder }
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Khetha ingxoxo engafundiwe engu-{ $count } ku-{ $folder }
           *[other] Khetha zonke izingxoxo ezingafundiwe ezingu-{ $count } ku-{ $folder }
        }
       *[message] { $count ->
            [one] Khetha umlayezo ongafundiwe ongu-{ $count } ku-{ $folder }
           *[other] Khetha yonke imilayezo engafundiwe engu-{ $count } ku-{ $folder }
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Khetha ingxoxo enenkanyezi engu-{ $count } ku-{ $folder }
           *[other] Khetha zonke izingxoxo ezinenkanyezi ezingu-{ $count } ku-{ $folder }
        }
       *[message] { $count ->
            [one] Khetha umlayezo onenkanyezi ongu-{ $count } ku-{ $folder }
           *[other] Khetha yonke imilayezo enenkanyezi engu-{ $count } ku-{ $folder }
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Khetha ingxoxo engenankanyezi engu-{ $count } ku-{ $folder }
           *[other] Khetha zonke izingxoxo ezingenankanyezi ezingu-{ $count } ku-{ $folder }
        }
       *[message] { $count ->
            [one] Khetha umlayezo ongenankanyezi ongu-{ $count } ku-{ $folder }
           *[other] Khetha yonke imilayezo engenankanyezi engu-{ $count } ku-{ $folder }
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Ingxoxo efundiwe engu-{ $count } ikhethiwe.
           *[other] Zonke izingxoxo ezifundiwe ezingu-{ $count } zikhethiwe.
        }
       *[message] { $count ->
            [one] Umlayezo ofundiwe ongu-{ $count } ukhethiwe.
           *[other] Yonke imilayezo efundiwe engu-{ $count } ikhethiwe.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Ingxoxo engafundiwe engu-{ $count } ikhethiwe.
           *[other] Zonke izingxoxo ezingafundiwe ezingu-{ $count } zikhethiwe.
        }
       *[message] { $count ->
            [one] Umlayezo ongafundiwe ongu-{ $count } ukhethiwe.
           *[other] Yonke imilayezo engafundiwe engu-{ $count } ikhethiwe.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Ingxoxo enenkanyezi engu-{ $count } ikhethiwe.
           *[other] Zonke izingxoxo ezinenkanyezi ezingu-{ $count } zikhethiwe.
        }
       *[message] { $count ->
            [one] Umlayezo onenkanyezi ongu-{ $count } ukhethiwe.
           *[other] Yonke imilayezo enenkanyezi engu-{ $count } ikhethiwe.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Ingxoxo engenankanyezi engu-{ $count } ikhethiwe.
           *[other] Zonke izingxoxo ezingenankanyezi ezingu-{ $count } zikhethiwe.
        }
       *[message] { $count ->
            [one] Umlayezo ongenankanyezi ongu-{ $count } ukhethiwe.
           *[other] Yonke imilayezo engenankanyezi engu-{ $count } ikhethiwe.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Ingxoxo efundiwe engu-{ $count } ku-{ $folder } ikhethiwe.
           *[other] Zonke izingxoxo ezifundiwe ezingu-{ $count } ku-{ $folder } zikhethiwe.
        }
       *[message] { $count ->
            [one] Umlayezo ofundiwe ongu-{ $count } ku-{ $folder } ukhethiwe.
           *[other] Yonke imilayezo efundiwe engu-{ $count } ku-{ $folder } ikhethiwe.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Ingxoxo engafundiwe engu-{ $count } ku-{ $folder } ikhethiwe.
           *[other] Zonke izingxoxo ezingafundiwe ezingu-{ $count } ku-{ $folder } zikhethiwe.
        }
       *[message] { $count ->
            [one] Umlayezo ongafundiwe ongu-{ $count } ku-{ $folder } ukhethiwe.
           *[other] Yonke imilayezo engafundiwe engu-{ $count } ku-{ $folder } ikhethiwe.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Ingxoxo enenkanyezi engu-{ $count } ku-{ $folder } ikhethiwe.
           *[other] Zonke izingxoxo ezinenkanyezi ezingu-{ $count } ku-{ $folder } zikhethiwe.
        }
       *[message] { $count ->
            [one] Umlayezo onenkanyezi ongu-{ $count } ku-{ $folder } ukhethiwe.
           *[other] Yonke imilayezo enenkanyezi engu-{ $count } ku-{ $folder } ikhethiwe.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Ingxoxo engenankanyezi engu-{ $count } ku-{ $folder } ikhethiwe.
           *[other] Zonke izingxoxo ezingenankanyezi ezingu-{ $count } ku-{ $folder } zikhethiwe.
        }
       *[message] { $count ->
            [one] Umlayezo ongenankanyezi ongu-{ $count } ku-{ $folder } ukhethiwe.
           *[other] Yonke imilayezo engenankanyezi engu-{ $count } ku-{ $folder } ikhethiwe.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Azikho izingxoxo ezifundiwe lapha.
       *[message] Ayikho imilayezo efundiwe lapha.
    }
   *[unread] { $kind ->
        [conversation] Azikho izingxoxo ezingafundiwe lapha.
       *[message] Ayikho imilayezo engafundiwe lapha.
    }
    [starred] { $kind ->
        [conversation] Azikho izingxoxo ezinenkanyezi lapha.
       *[message] Ayikho imilayezo enenkanyezi lapha.
    }
    [unstarred] { $kind ->
        [conversation] Azikho izingxoxo ezingenankanyezi lapha.
       *[message] Ayikho imilayezo engenankanyezi lapha.
    }
}
list-clear-selection = Sula okukhethiwe

## Mail list: empty states

list-empty-search = Ayikho imilayezo ehambisana nosesho lwakho.
list-empty-tab = Ayikho imeyili ku-{ $tab }.
list-empty-tab-unknown = Ayikho imeyili kule thebhu.
list-empty-folder = Ayikho imilayezo ku-{ $folder }.
list-empty-folder-unknown = Ayikho imilayezo kule folda.
list-first-sync = Kulandwa imeyili yakho…
list-first-sync-detail = Izovela lapha njengoba ifika.

## Mail list: lines

row-removed = Lo mlayezo ususiwe.
row-starred = Kunenkanyezi
row-not-starred = Akunankanyezi
row-important = Kubalulekile. Chofoza ukuze umake njengokungabalulekile.
row-mark-important = Maka njengokubalulekile
row-pinned = Kuphinwe phezulu
row-tracking-none = Kuyalandelelwa. Akukavulwa
row-tracking-opened = Ivulwe ngu-{ $opened } kwabangu-{ $recipients }
row-tracking-clicked = Ivulwe ngu-{ $opened } kwabangu-{ $recipients }, isixhumanisi silandelwe ngu-{ $clicked }
row-pin = Phina phezulu
row-unpin = Susa ukuphina
row-snoozed-until = Kulibazisiwe kuze kube ngu-{ $when }

## Mail list: More menu and right-click menu

menu-reply = Phendula
menu-reply-all = Phendula bonke
menu-forward = Dlulisela
menu-archive = Faka kungobo yomlando
menu-delete = Susa
menu-delete-forever = Susa unomphela
menu-move-to-inbox = Hambisa kubhokisi lokungenayo
menu-spam = Bika ugaxekile
menu-not-spam = Akuyona ugaxekile
menu-mark-read = Maka njengokufundiwe
menu-mark-unread = Maka njengokungafundiwe
menu-mark-all-read = Maka konke njengokufundiwe
menu-star = Engeza inkanyezi
menu-unstar = Susa inkanyezi
menu-important = Maka njengokubalulekile
menu-not-important = Maka njengokungabalulekile
menu-pin = Phina phezulu
menu-unpin = Susa ukuphina
menu-snooze = Libazisa
menu-unsnooze = Yeka ukulibazisa
menu-print-all = Phrinta konke
menu-new-window = Vula ewindini elisha
menu-move-to = Hambisa ku-
menu-move-to-heading = Hambisa ku:
menu-find-from = Thola ama-imeyili avela ku-{ $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo ifakwe kungobo yomlando.
       *[other] Izingxoxo ezingu-{ $count } zifakwe kungobo yomlando.
    }
   *[message] { $count ->
        [one] Umlayezo ufakwe kungobo yomlando.
       *[other] Imilayezo engu-{ $count } ifakwe kungobo yomlando.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo ihanjiswe kudoti.
       *[other] Izingxoxo ezingu-{ $count } zihanjiswe kudoti.
    }
   *[message] { $count ->
        [one] Umlayezo uhanjiswe kudoti.
       *[other] Imilayezo engu-{ $count } ihanjiswe kudoti.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo ihanjisiwe.
       *[other] Izingxoxo ezingu-{ $count } zihanjisiwe.
    }
   *[message] { $count ->
        [one] Umlayezo uhanjisiwe.
       *[other] Imilayezo engu-{ $count } ihanjisiwe.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo ifakwe inkanyezi.
       *[other] Izingxoxo ezingu-{ $count } zifakwe inkanyezi.
    }
   *[message] { $count ->
        [one] Umlayezo ufakwe inkanyezi.
       *[other] Imilayezo engu-{ $count } ifakwe inkanyezi.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Inkanyezi isusiwe engxoxweni.
       *[other] Inkanyezi isusiwe ezingxoxweni ezingu-{ $count }.
    }
   *[message] { $count ->
        [one] Inkanyezi isusiwe emlayezweni.
       *[other] Inkanyezi isusiwe emilayezweni engu-{ $count }.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo imakwe njengebalulekile.
       *[other] Izingxoxo ezingu-{ $count } zimakwe njengezibalulekile.
    }
   *[message] { $count ->
        [one] Umlayezo umakwe njengobalulekile.
       *[other] Imilayezo engu-{ $count } imakwe njengebalulekile.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo imakwe njengengabalulekile.
       *[other] Izingxoxo ezingu-{ $count } zimakwe njengezingabalulekile.
    }
   *[message] { $count ->
        [one] Umlayezo umakwe njengongabalulekile.
       *[other] Imilayezo engu-{ $count } imakwe njengengabalulekile.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo iphinwe phezulu.
       *[other] Izingxoxo ezingu-{ $count } ziphinwe phezulu.
    }
   *[message] { $count ->
        [one] Umlayezo uphinwe phezulu.
       *[other] Imilayezo engu-{ $count } iphinwe phezulu.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Ukuphina kwengxoxo kususiwe.
       *[other] Ukuphina kwezingxoxo ezingu-{ $count } kususiwe.
    }
   *[message] { $count ->
        [one] Ukuphina komlayezo kususiwe.
       *[other] Ukuphina kwemilayezo engu-{ $count } kususiwe.
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo ilibazisiwe kuze kube ngu-{ $when }.
       *[other] Izingxoxo ezingu-{ $count } zilibazisiwe kuze kube ngu-{ $when }.
    }
   *[message] { $count ->
        [one] Umlayezo ulibazisiwe kuze kube ngu-{ $when }.
       *[other] Imilayezo engu-{ $count } ilibazisiwe kuze kube ngu-{ $when }.
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo ibuyele ebhokisini lokungenayo.
       *[other] Izingxoxo ezingu-{ $count } zibuyele ebhokisini lokungenayo.
    }
   *[message] { $count ->
        [one] Umlayezo ubuyele ebhokisini lokungenayo.
       *[other] Imilayezo engu-{ $count } ibuyele ebhokisini lokungenayo.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo ibikwe njengogaxekile.
       *[other] Izingxoxo ezingu-{ $count } zibikwe njengogaxekile.
    }
   *[message] { $count ->
        [one] Umlayezo ubikwe njengogaxekile.
       *[other] Imilayezo engu-{ $count } ibikwe njengogaxekile.
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo imakwe njengengeyona ugaxekile futhi yahanjiswa kubhokisi lokungenayo.
       *[other] Izingxoxo ezingu-{ $count } zimakwe njengezingeyona ugaxekile futhi zahanjiswa kubhokisi lokungenayo.
    }
   *[message] { $count ->
        [one] Umlayezo umakwe njengongeyona ugaxekile futhi wahanjiswa kubhokisi lokungenayo.
       *[other] Imilayezo engu-{ $count } imakwe njengengeyona ugaxekile futhi yahanjiswa kubhokisi lokungenayo.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo isuswe unomphela.
       *[other] Izingxoxo ezingu-{ $count } zisuswe unomphela.
    }
   *[message] { $count ->
        [one] Umlayezo ususwe unomphela.
       *[other] Imilayezo engu-{ $count } isuswe unomphela.
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo imakwe njengokufundiwe.
       *[other] Izingxoxo ezingu-{ $count } zimakwe njengokufundiwe.
    }
   *[message] { $count ->
        [one] Umlayezo umakwe njengokufundiwe.
       *[other] Imilayezo engu-{ $count } imakwe njengokufundiwe.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo imakwe njengokungafundiwe.
       *[other] Izingxoxo ezingu-{ $count } zimakwe njengokungafundiwe.
    }
   *[message] { $count ->
        [one] Umlayezo umakwe njengokungafundiwe.
       *[other] Imilayezo engu-{ $count } imakwe njengokungafundiwe.
    }
}
toast-undone = Isenzo sihlehlisiwe.
toast-nothing-to-undo = Akukho okungahlehliswa.
toast-cannot-undo-delete-forever = Imeyili esuswe unomphela ayikwazi ukubuyiswa.
toast-send-undone = Ukuthumela kuhlehlisiwe.
toast-too-late-to-undo-send = Sekwephuze kakhulu ukuhlehlisa: umlayezo usuthunyelwe.
toast-undo = Hlehlisa
toast-close = Vala
toast-no-spam-folder = Le akhawunti ayinayo ifolda kagaxekile.
