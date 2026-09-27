# Katna Mail, Hausa (Hausa).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Na farko
tab-promotions = Tallace-tallace
tab-social = Zamantakewa
tab-updates = Sabuntawa
tab-forums = Dandali
tab-focused = Mai da hankali
tab-other = Sauran
tab-inbox = Akwatin saƙo
tab-newsletters = Wasiƙun labarai
tab-notifications = Sanarwa
tab-new = { $count } sababbi
tab-provider-other = Katna ne ya tsara

## Mail list: toolbar

list-select = Zaɓi
list-refresh = Sabunta
list-more = Ƙari
list-mark-read = Yi alama an karanta
list-mark-unread = Yi alama ba a karanta ba
list-move-to = Matsar zuwa
list-archive = Adana a ma'ajiya
list-spam = Rahoto saƙon banza
list-delete = Share
list-newer = Sababbi
list-older = Tsofaffi
list-range = { $first }–{ $last } cikin { $total }
list-range-about = { $first }–{ $last } cikin kusan { $total }
list-results = Sakamakon “{ $query }”
list-results-corrected = Ana nuna sakamakon “{ $query }”
list-search-instead = Maimakon haka bincika “{ $query }”
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Duka
list-pick-none = Babu
list-pick-read = An karanta
list-pick-unread = Ba a karanta ba
list-pick-starred = Masu tauraro
list-pick-unstarred = Marasa tauraro

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] An zaɓi tattaunawa { $count }.
       *[other] An zaɓi dukkan tattaunawa { $count }.
    }
   *[message] { $count ->
        [one] An zaɓi saƙo { $count }.
       *[other] An zaɓi dukkan saƙonni { $count }.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] An zaɓi tattaunawa { $count } da ke cikin { $folder }.
       *[other] An zaɓi dukkan tattaunawa { $count } da ke cikin { $folder }.
    }
   *[message] { $count ->
        [one] An zaɓi saƙo { $count } da ke cikin { $folder }.
       *[other] An zaɓi dukkan saƙonni { $count } da ke cikin { $folder }.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] An zaɓi tattaunawa { $count } da ke kan allo.
       *[other] An zaɓi dukkan tattaunawa { $count } da ke kan allo.
    }
   *[message] { $count ->
        [one] An zaɓi saƙo { $count } da ke kan allo.
       *[other] An zaɓi dukkan saƙonni { $count } da ke kan allo.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Zaɓi tattaunawa { $count }
       *[other] Zaɓi dukkan tattaunawa { $count }
    }
   *[message] { $count ->
        [one] Zaɓi saƙo { $count }
       *[other] Zaɓi dukkan saƙonni { $count }
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Zaɓi tattaunawa { $count } da ke cikin { $folder }
       *[other] Zaɓi dukkan tattaunawa { $count } da ke cikin { $folder }
    }
   *[message] { $count ->
        [one] Zaɓi saƙo { $count } da ke cikin { $folder }
       *[other] Zaɓi dukkan saƙonni { $count } da ke cikin { $folder }
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] An zaɓi tattaunawa { $count } da aka karanta da ke kan allo.
           *[other] An zaɓi dukkan tattaunawa { $count } da aka karanta da ke kan allo.
        }
       *[message] { $count ->
            [one] An zaɓi saƙo { $count } da aka karanta da ke kan allo.
           *[other] An zaɓi dukkan saƙonni { $count } da aka karanta da ke kan allo.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] An zaɓi tattaunawa { $count } da ba a karanta ba da ke kan allo.
           *[other] An zaɓi dukkan tattaunawa { $count } da ba a karanta ba da ke kan allo.
        }
       *[message] { $count ->
            [one] An zaɓi saƙo { $count } da ba a karanta ba da ke kan allo.
           *[other] An zaɓi dukkan saƙonni { $count } da ba a karanta ba da ke kan allo.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] An zaɓi tattaunawa { $count } mai tauraro da ke kan allo.
           *[other] An zaɓi dukkan tattaunawa { $count } masu tauraro da ke kan allo.
        }
       *[message] { $count ->
            [one] An zaɓi saƙo { $count } mai tauraro da ke kan allo.
           *[other] An zaɓi dukkan saƙonni { $count } masu tauraro da ke kan allo.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] An zaɓi tattaunawa { $count } mara tauraro da ke kan allo.
           *[other] An zaɓi dukkan tattaunawa { $count } marasa tauraro da ke kan allo.
        }
       *[message] { $count ->
            [one] An zaɓi saƙo { $count } mara tauraro da ke kan allo.
           *[other] An zaɓi dukkan saƙonni { $count } marasa tauraro da ke kan allo.
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Zaɓi tattaunawa { $count } da aka karanta
           *[other] Zaɓi dukkan tattaunawa { $count } da aka karanta
        }
       *[message] { $count ->
            [one] Zaɓi saƙo { $count } da aka karanta
           *[other] Zaɓi dukkan saƙonni { $count } da aka karanta
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Zaɓi tattaunawa { $count } da ba a karanta ba
           *[other] Zaɓi dukkan tattaunawa { $count } da ba a karanta ba
        }
       *[message] { $count ->
            [one] Zaɓi saƙo { $count } da ba a karanta ba
           *[other] Zaɓi dukkan saƙonni { $count } da ba a karanta ba
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Zaɓi tattaunawa { $count } mai tauraro
           *[other] Zaɓi dukkan tattaunawa { $count } masu tauraro
        }
       *[message] { $count ->
            [one] Zaɓi saƙo { $count } mai tauraro
           *[other] Zaɓi dukkan saƙonni { $count } masu tauraro
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Zaɓi tattaunawa { $count } mara tauraro
           *[other] Zaɓi dukkan tattaunawa { $count } marasa tauraro
        }
       *[message] { $count ->
            [one] Zaɓi saƙo { $count } mara tauraro
           *[other] Zaɓi dukkan saƙonni { $count } marasa tauraro
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Zaɓi tattaunawa { $count } da aka karanta da ke cikin { $folder }
           *[other] Zaɓi dukkan tattaunawa { $count } da aka karanta da ke cikin { $folder }
        }
       *[message] { $count ->
            [one] Zaɓi saƙo { $count } da aka karanta da ke cikin { $folder }
           *[other] Zaɓi dukkan saƙonni { $count } da aka karanta da ke cikin { $folder }
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Zaɓi tattaunawa { $count } da ba a karanta ba da ke cikin { $folder }
           *[other] Zaɓi dukkan tattaunawa { $count } da ba a karanta ba da ke cikin { $folder }
        }
       *[message] { $count ->
            [one] Zaɓi saƙo { $count } da ba a karanta ba da ke cikin { $folder }
           *[other] Zaɓi dukkan saƙonni { $count } da ba a karanta ba da ke cikin { $folder }
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Zaɓi tattaunawa { $count } mai tauraro da ke cikin { $folder }
           *[other] Zaɓi dukkan tattaunawa { $count } masu tauraro da ke cikin { $folder }
        }
       *[message] { $count ->
            [one] Zaɓi saƙo { $count } mai tauraro da ke cikin { $folder }
           *[other] Zaɓi dukkan saƙonni { $count } masu tauraro da ke cikin { $folder }
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Zaɓi tattaunawa { $count } mara tauraro da ke cikin { $folder }
           *[other] Zaɓi dukkan tattaunawa { $count } marasa tauraro da ke cikin { $folder }
        }
       *[message] { $count ->
            [one] Zaɓi saƙo { $count } mara tauraro da ke cikin { $folder }
           *[other] Zaɓi dukkan saƙonni { $count } marasa tauraro da ke cikin { $folder }
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] An zaɓi tattaunawa { $count } da aka karanta.
           *[other] An zaɓi dukkan tattaunawa { $count } da aka karanta.
        }
       *[message] { $count ->
            [one] An zaɓi saƙo { $count } da aka karanta.
           *[other] An zaɓi dukkan saƙonni { $count } da aka karanta.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] An zaɓi tattaunawa { $count } da ba a karanta ba.
           *[other] An zaɓi dukkan tattaunawa { $count } da ba a karanta ba.
        }
       *[message] { $count ->
            [one] An zaɓi saƙo { $count } da ba a karanta ba.
           *[other] An zaɓi dukkan saƙonni { $count } da ba a karanta ba.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] An zaɓi tattaunawa { $count } mai tauraro.
           *[other] An zaɓi dukkan tattaunawa { $count } masu tauraro.
        }
       *[message] { $count ->
            [one] An zaɓi saƙo { $count } mai tauraro.
           *[other] An zaɓi dukkan saƙonni { $count } masu tauraro.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] An zaɓi tattaunawa { $count } mara tauraro.
           *[other] An zaɓi dukkan tattaunawa { $count } marasa tauraro.
        }
       *[message] { $count ->
            [one] An zaɓi saƙo { $count } mara tauraro.
           *[other] An zaɓi dukkan saƙonni { $count } marasa tauraro.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] An zaɓi tattaunawa { $count } da aka karanta a cikin { $folder }.
           *[other] An zaɓi dukkan tattaunawa { $count } da aka karanta a cikin { $folder }.
        }
       *[message] { $count ->
            [one] An zaɓi saƙo { $count } da aka karanta a cikin { $folder }.
           *[other] An zaɓi dukkan saƙonni { $count } da aka karanta a cikin { $folder }.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] An zaɓi tattaunawa { $count } da ba a karanta ba a cikin { $folder }.
           *[other] An zaɓi dukkan tattaunawa { $count } da ba a karanta ba a cikin { $folder }.
        }
       *[message] { $count ->
            [one] An zaɓi saƙo { $count } da ba a karanta ba a cikin { $folder }.
           *[other] An zaɓi dukkan saƙonni { $count } da ba a karanta ba a cikin { $folder }.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] An zaɓi tattaunawa { $count } mai tauraro a cikin { $folder }.
           *[other] An zaɓi dukkan tattaunawa { $count } masu tauraro a cikin { $folder }.
        }
       *[message] { $count ->
            [one] An zaɓi saƙo { $count } mai tauraro a cikin { $folder }.
           *[other] An zaɓi dukkan saƙonni { $count } masu tauraro a cikin { $folder }.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] An zaɓi tattaunawa { $count } mara tauraro a cikin { $folder }.
           *[other] An zaɓi dukkan tattaunawa { $count } marasa tauraro a cikin { $folder }.
        }
       *[message] { $count ->
            [one] An zaɓi saƙo { $count } mara tauraro a cikin { $folder }.
           *[other] An zaɓi dukkan saƙonni { $count } marasa tauraro a cikin { $folder }.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Babu tattaunawa da aka karanta a nan.
       *[message] Babu saƙonni da aka karanta a nan.
    }
   *[unread] { $kind ->
        [conversation] Babu tattaunawa da ba a karanta ba a nan.
       *[message] Babu saƙonni da ba a karanta ba a nan.
    }
    [starred] { $kind ->
        [conversation] Babu tattaunawa masu tauraro a nan.
       *[message] Babu saƙonni masu tauraro a nan.
    }
    [unstarred] { $kind ->
        [conversation] Babu tattaunawa marasa tauraro a nan.
       *[message] Babu saƙonni marasa tauraro a nan.
    }
}
list-clear-selection = Share zaɓi

## Mail list: empty states

list-empty-search = Babu saƙonnin da suka dace da bincikenku.
list-empty-tab = Babu wasiƙu a cikin { $tab }.
list-empty-tab-unknown = Babu wasiƙu a cikin wannan shafi.
list-empty-folder = Babu saƙonni a cikin { $folder }.
list-empty-folder-unknown = Babu saƙonni a cikin wannan folda.
list-first-sync = Ana samo wasiƙunku…
list-first-sync-detail = Za su bayyana a nan yayin da suke isowa.

## Mail list: lines

row-removed = An cire wannan saƙo.
row-starred = Mai tauraro
row-not-starred = Babu tauraro
row-important = Muhimmi. Danna don yin alama ba muhimmi ba.
row-mark-important = Yi alama muhimmi
row-pinned = An maƙala a sama
row-pin = Maƙala a sama
row-unpin = Cire maƙalawa

## Mail list: More menu and right-click menu

menu-reply = Amsa
menu-reply-all = Amsa wa kowa
menu-forward = Tura
menu-archive = Adana a ma'ajiya
menu-delete = Share
menu-delete-forever = Share har abada
menu-move-to-inbox = Matsar zuwa Akwatin saƙo
menu-spam = Rahoto saƙon banza
menu-not-spam = Ba saƙon banza ba ne
menu-mark-read = Yi alama an karanta
menu-mark-unread = Yi alama ba a karanta ba
menu-mark-all-read = Yi wa duka alama an karanta
menu-star = Saka tauraro
menu-unstar = Cire tauraro
menu-important = Yi alama muhimmi
menu-not-important = Yi alama ba muhimmi ba
menu-pin = Maƙala a sama
menu-unpin = Cire maƙalawa
menu-print-all = Buga duka
menu-new-window = Buɗe a sabuwar taga
menu-move-to = Matsar zuwa
menu-move-to-heading = Matsar zuwa:
menu-find-from = Nemo imel daga { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] An adana tattaunawa a ma'ajiya.
       *[other] An adana tattaunawa { $count } a ma'ajiya.
    }
   *[message] { $count ->
        [one] An adana saƙo a ma'ajiya.
       *[other] An adana saƙonni { $count } a ma'ajiya.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] An matsar da tattaunawa zuwa Kwandon shara.
       *[other] An matsar da tattaunawa { $count } zuwa Kwandon shara.
    }
   *[message] { $count ->
        [one] An matsar da saƙo zuwa Kwandon shara.
       *[other] An matsar da saƙonni { $count } zuwa Kwandon shara.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] An matsar da tattaunawa.
       *[other] An matsar da tattaunawa { $count }.
    }
   *[message] { $count ->
        [one] An matsar da saƙo.
       *[other] An matsar da saƙonni { $count }.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] An saka wa tattaunawa tauraro.
       *[other] An saka wa tattaunawa { $count } tauraro.
    }
   *[message] { $count ->
        [one] An saka wa saƙo tauraro.
       *[other] An saka wa saƙonni { $count } tauraro.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] An cire tauraro daga tattaunawa.
       *[other] An cire tauraro daga tattaunawa { $count }.
    }
   *[message] { $count ->
        [one] An cire tauraro daga saƙo.
       *[other] An cire tauraro daga saƙonni { $count }.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] An yi wa tattaunawa alama muhimmiya.
       *[other] An yi wa tattaunawa { $count } alama muhimmai.
    }
   *[message] { $count ->
        [one] An yi wa saƙo alama muhimmi.
       *[other] An yi wa saƙonni { $count } alama muhimmai.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] An yi wa tattaunawa alama ba muhimmiya ba.
       *[other] An yi wa tattaunawa { $count } alama ba muhimmai ba.
    }
   *[message] { $count ->
        [one] An yi wa saƙo alama ba muhimmi ba.
       *[other] An yi wa saƙonni { $count } alama ba muhimmai ba.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] An maƙala tattaunawa a sama.
       *[other] An maƙala tattaunawa { $count } a sama.
    }
   *[message] { $count ->
        [one] An maƙala saƙo a sama.
       *[other] An maƙala saƙonni { $count } a sama.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] An cire maƙalawar tattaunawa.
       *[other] An cire maƙalawar tattaunawa { $count }.
    }
   *[message] { $count ->
        [one] An cire maƙalawar saƙo.
       *[other] An cire maƙalawar saƙonni { $count }.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] An kai rahoton tattaunawa a matsayin saƙon banza.
       *[other] An kai rahoton tattaunawa { $count } a matsayin saƙonnin banza.
    }
   *[message] { $count ->
        [one] An kai rahoton saƙo a matsayin saƙon banza.
       *[other] An kai rahoton saƙonni { $count } a matsayin saƙonnin banza.
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] An yi wa tattaunawa alama cewa ba saƙon banza ba ce kuma an matsar da ita zuwa akwatin saƙo.
       *[other] An yi wa tattaunawa { $count } alama cewa ba saƙonnin banza ba ne kuma an matsar da su zuwa akwatin saƙo.
    }
   *[message] { $count ->
        [one] An yi wa saƙo alama cewa ba saƙon banza ba ne kuma an matsar da shi zuwa akwatin saƙo.
       *[other] An yi wa saƙonni { $count } alama cewa ba saƙonnin banza ba ne kuma an matsar da su zuwa akwatin saƙo.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] An share tattaunawa har abada.
       *[other] An share tattaunawa { $count } har abada.
    }
   *[message] { $count ->
        [one] An share saƙo har abada.
       *[other] An share saƙonni { $count } har abada.
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] An yi wa tattaunawa alama an karanta.
       *[other] An yi wa tattaunawa { $count } alama an karanta.
    }
   *[message] { $count ->
        [one] An yi wa saƙo alama an karanta.
       *[other] An yi wa saƙonni { $count } alama an karanta.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] An yi wa tattaunawa alama ba a karanta ba.
       *[other] An yi wa tattaunawa { $count } alama ba a karanta ba.
    }
   *[message] { $count ->
        [one] An yi wa saƙo alama ba a karanta ba.
       *[other] An yi wa saƙonni { $count } alama ba a karanta ba.
    }
}
toast-undone = An janye aikin.
toast-nothing-to-undo = Babu abin da za a janye.
toast-cannot-undo-delete-forever = Ba za a iya dawo da wasiƙun da aka share har abada ba.
toast-send-undone = An janye aikawa.
toast-too-late-to-undo-send = Lokacin janyewa ya wuce: an riga an aika saƙon.
toast-undo = Janye
toast-no-spam-folder = Wannan asusun ba shi da foldar saƙonnin banza.
