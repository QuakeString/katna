# Katna Mail, Afrikaans (Afrikaans).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Primêr
tab-promotions = Promosies
tab-social = Sosiaal
tab-updates = Opdaterings
tab-forums = Forums
tab-focused = Gefokus
tab-other = Ander
tab-inbox = Inkassie
tab-newsletters = Nuusbriewe
tab-notifications = Kennisgewings
tab-new = { $count } nuut
tab-provider-other = gesorteer deur Katna

## Mail list: toolbar

list-select = Kies
list-refresh = Herlaai
list-more = Meer
list-mark-read = Merk as gelees
list-mark-unread = Merk as ongelees
list-move-to = Skuif na
list-archive = Argiveer
list-spam = Rapporteer strooipos
list-delete = Vee uit
list-newer = Nuwer
list-older = Ouer
list-range = { $first }–{ $last } van { $total }
list-range-about = { $first }–{ $last } van ongeveer { $total }
list-results = Resultate vir “{ $query }”
list-results-corrected = Wys resultate vir “{ $query }”
list-search-instead = Soek eerder vir “{ $query }”
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Alles
list-pick-none = Geen
list-pick-read = Gelees
list-pick-unread = Ongelees
list-pick-starred = Gester
list-pick-unstarred = Ongester

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } gesprek is gekies.
       *[other] Al { $count } gesprekke is gekies.
    }
   *[message] { $count ->
        [one] { $count } boodskap is gekies.
       *[other] Al { $count } boodskappe is gekies.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $count } gesprek in { $folder } is gekies.
       *[other] Al { $count } gesprekke in { $folder } is gekies.
    }
   *[message] { $count ->
        [one] { $count } boodskap in { $folder } is gekies.
       *[other] Al { $count } boodskappe in { $folder } is gekies.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] { $count } gesprek op hierdie bladsy is gekies.
       *[other] Al { $count } gesprekke op hierdie bladsy is gekies.
    }
   *[message] { $count ->
        [one] { $count } boodskap op hierdie bladsy is gekies.
       *[other] Al { $count } boodskappe op hierdie bladsy is gekies.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Kies { $count } gesprek
       *[other] Kies al { $count } gesprekke
    }
   *[message] { $count ->
        [one] Kies { $count } boodskap
       *[other] Kies al { $count } boodskappe
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Kies { $count } gesprek in { $folder }
       *[other] Kies al { $count } gesprekke in { $folder }
    }
   *[message] { $count ->
        [one] Kies { $count } boodskap in { $folder }
       *[other] Kies al { $count } boodskappe in { $folder }
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } gelese gesprek op hierdie bladsy is gekies.
           *[other] Al { $count } gelese gesprekke op hierdie bladsy is gekies.
        }
       *[message] { $count ->
            [one] { $count } gelese boodskap op hierdie bladsy is gekies.
           *[other] Al { $count } gelese boodskappe op hierdie bladsy is gekies.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } ongelese gesprek op hierdie bladsy is gekies.
           *[other] Al { $count } ongelese gesprekke op hierdie bladsy is gekies.
        }
       *[message] { $count ->
            [one] { $count } ongelese boodskap op hierdie bladsy is gekies.
           *[other] Al { $count } ongelese boodskappe op hierdie bladsy is gekies.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } gesterde gesprek op hierdie bladsy is gekies.
           *[other] Al { $count } gesterde gesprekke op hierdie bladsy is gekies.
        }
       *[message] { $count ->
            [one] { $count } gesterde boodskap op hierdie bladsy is gekies.
           *[other] Al { $count } gesterde boodskappe op hierdie bladsy is gekies.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } ongesterde gesprek op hierdie bladsy is gekies.
           *[other] Al { $count } ongesterde gesprekke op hierdie bladsy is gekies.
        }
       *[message] { $count ->
            [one] { $count } ongesterde boodskap op hierdie bladsy is gekies.
           *[other] Al { $count } ongesterde boodskappe op hierdie bladsy is gekies.
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Kies { $count } gelese gesprek
           *[other] Kies al { $count } gelese gesprekke
        }
       *[message] { $count ->
            [one] Kies { $count } gelese boodskap
           *[other] Kies al { $count } gelese boodskappe
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Kies { $count } ongelese gesprek
           *[other] Kies al { $count } ongelese gesprekke
        }
       *[message] { $count ->
            [one] Kies { $count } ongelese boodskap
           *[other] Kies al { $count } ongelese boodskappe
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Kies { $count } gesterde gesprek
           *[other] Kies al { $count } gesterde gesprekke
        }
       *[message] { $count ->
            [one] Kies { $count } gesterde boodskap
           *[other] Kies al { $count } gesterde boodskappe
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Kies { $count } ongesterde gesprek
           *[other] Kies al { $count } ongesterde gesprekke
        }
       *[message] { $count ->
            [one] Kies { $count } ongesterde boodskap
           *[other] Kies al { $count } ongesterde boodskappe
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Kies { $count } gelese gesprek in { $folder }
           *[other] Kies al { $count } gelese gesprekke in { $folder }
        }
       *[message] { $count ->
            [one] Kies { $count } gelese boodskap in { $folder }
           *[other] Kies al { $count } gelese boodskappe in { $folder }
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Kies { $count } ongelese gesprek in { $folder }
           *[other] Kies al { $count } ongelese gesprekke in { $folder }
        }
       *[message] { $count ->
            [one] Kies { $count } ongelese boodskap in { $folder }
           *[other] Kies al { $count } ongelese boodskappe in { $folder }
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Kies { $count } gesterde gesprek in { $folder }
           *[other] Kies al { $count } gesterde gesprekke in { $folder }
        }
       *[message] { $count ->
            [one] Kies { $count } gesterde boodskap in { $folder }
           *[other] Kies al { $count } gesterde boodskappe in { $folder }
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Kies { $count } ongesterde gesprek in { $folder }
           *[other] Kies al { $count } ongesterde gesprekke in { $folder }
        }
       *[message] { $count ->
            [one] Kies { $count } ongesterde boodskap in { $folder }
           *[other] Kies al { $count } ongesterde boodskappe in { $folder }
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } gelese gesprek is gekies.
           *[other] Al { $count } gelese gesprekke is gekies.
        }
       *[message] { $count ->
            [one] { $count } gelese boodskap is gekies.
           *[other] Al { $count } gelese boodskappe is gekies.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } ongelese gesprek is gekies.
           *[other] Al { $count } ongelese gesprekke is gekies.
        }
       *[message] { $count ->
            [one] { $count } ongelese boodskap is gekies.
           *[other] Al { $count } ongelese boodskappe is gekies.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } gesterde gesprek is gekies.
           *[other] Al { $count } gesterde gesprekke is gekies.
        }
       *[message] { $count ->
            [one] { $count } gesterde boodskap is gekies.
           *[other] Al { $count } gesterde boodskappe is gekies.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } ongesterde gesprek is gekies.
           *[other] Al { $count } ongesterde gesprekke is gekies.
        }
       *[message] { $count ->
            [one] { $count } ongesterde boodskap is gekies.
           *[other] Al { $count } ongesterde boodskappe is gekies.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } gelese gesprek in { $folder } is gekies.
           *[other] Al { $count } gelese gesprekke in { $folder } is gekies.
        }
       *[message] { $count ->
            [one] { $count } gelese boodskap in { $folder } is gekies.
           *[other] Al { $count } gelese boodskappe in { $folder } is gekies.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } ongelese gesprek in { $folder } is gekies.
           *[other] Al { $count } ongelese gesprekke in { $folder } is gekies.
        }
       *[message] { $count ->
            [one] { $count } ongelese boodskap in { $folder } is gekies.
           *[other] Al { $count } ongelese boodskappe in { $folder } is gekies.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } gesterde gesprek in { $folder } is gekies.
           *[other] Al { $count } gesterde gesprekke in { $folder } is gekies.
        }
       *[message] { $count ->
            [one] { $count } gesterde boodskap in { $folder } is gekies.
           *[other] Al { $count } gesterde boodskappe in { $folder } is gekies.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } ongesterde gesprek in { $folder } is gekies.
           *[other] Al { $count } ongesterde gesprekke in { $folder } is gekies.
        }
       *[message] { $count ->
            [one] { $count } ongesterde boodskap in { $folder } is gekies.
           *[other] Al { $count } ongesterde boodskappe in { $folder } is gekies.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Geen gelese gesprekke hier nie.
       *[message] Geen gelese boodskappe hier nie.
    }
   *[unread] { $kind ->
        [conversation] Geen ongelese gesprekke hier nie.
       *[message] Geen ongelese boodskappe hier nie.
    }
    [starred] { $kind ->
        [conversation] Geen gesterde gesprekke hier nie.
       *[message] Geen gesterde boodskappe hier nie.
    }
    [unstarred] { $kind ->
        [conversation] Geen ongesterde gesprekke hier nie.
       *[message] Geen ongesterde boodskappe hier nie.
    }
}
list-clear-selection = Vee keuse uit

## Mail list: empty states

list-empty-search = Geen boodskappe pas by jou soektog nie.
list-empty-tab = Geen e-pos in { $tab } nie.
list-empty-tab-unknown = Geen e-pos in hierdie oortjie nie.
list-empty-folder = Geen boodskappe in { $folder } nie.
list-empty-folder-unknown = Geen boodskappe in hierdie vouer nie.
list-first-sync = Kry tans jou e-pos…
list-first-sync-detail = Dit verskyn hier soos dit aankom.

## Mail list: lines

row-removed = Hierdie boodskap is verwyder.
row-starred = Gester
row-not-starred = Nie gester nie
row-important = Belangrik. Klik om as nie belangrik nie te merk.
row-mark-important = Merk as belangrik
row-pinned = Bo vasgespeld
row-pin = Speld bo vas
row-unpin = Ontspeld

## Mail list: More menu and right-click menu

menu-reply = Antwoord
menu-reply-all = Antwoord almal
menu-forward = Stuur aan
menu-archive = Argiveer
menu-delete = Vee uit
menu-delete-forever = Vee permanent uit
menu-move-to-inbox = Skuif na Inkassie
menu-spam = Rapporteer strooipos
menu-not-spam = Nie strooipos nie
menu-mark-read = Merk as gelees
menu-mark-unread = Merk as ongelees
menu-mark-all-read = Merk almal as gelees
menu-star = Voeg ster by
menu-unstar = Verwyder ster
menu-important = Merk as belangrik
menu-not-important = Merk as nie belangrik nie
menu-pin = Speld bo vas
menu-unpin = Ontspeld
menu-print-all = Druk alles
menu-new-window = Maak oop in nuwe venster
menu-move-to = Skuif na
menu-move-to-heading = Skuif na:
menu-find-from = Vind e-posse van { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Gesprek geargiveer.
       *[other] { $count } gesprekke geargiveer.
    }
   *[message] { $count ->
        [one] Boodskap geargiveer.
       *[other] { $count } boodskappe geargiveer.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Gesprek na die asblik geskuif.
       *[other] { $count } gesprekke na die asblik geskuif.
    }
   *[message] { $count ->
        [one] Boodskap na die asblik geskuif.
       *[other] { $count } boodskappe na die asblik geskuif.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Gesprek geskuif.
       *[other] { $count } gesprekke geskuif.
    }
   *[message] { $count ->
        [one] Boodskap geskuif.
       *[other] { $count } boodskappe geskuif.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Gesprek gester.
       *[other] { $count } gesprekke gester.
    }
   *[message] { $count ->
        [one] Boodskap gester.
       *[other] { $count } boodskappe gester.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Ster van gesprek verwyder.
       *[other] Ster van { $count } gesprekke verwyder.
    }
   *[message] { $count ->
        [one] Ster van boodskap verwyder.
       *[other] Ster van { $count } boodskappe verwyder.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Gesprek as belangrik gemerk.
       *[other] { $count } gesprekke as belangrik gemerk.
    }
   *[message] { $count ->
        [one] Boodskap as belangrik gemerk.
       *[other] { $count } boodskappe as belangrik gemerk.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Gesprek as nie belangrik nie gemerk.
       *[other] { $count } gesprekke as nie belangrik nie gemerk.
    }
   *[message] { $count ->
        [one] Boodskap as nie belangrik nie gemerk.
       *[other] { $count } boodskappe as nie belangrik nie gemerk.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Gesprek bo vasgespeld.
       *[other] { $count } gesprekke bo vasgespeld.
    }
   *[message] { $count ->
        [one] Boodskap bo vasgespeld.
       *[other] { $count } boodskappe bo vasgespeld.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Gesprek ontspeld.
       *[other] { $count } gesprekke ontspeld.
    }
   *[message] { $count ->
        [one] Boodskap ontspeld.
       *[other] { $count } boodskappe ontspeld.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Gesprek as strooipos gerapporteer.
       *[other] { $count } gesprekke as strooipos gerapporteer.
    }
   *[message] { $count ->
        [one] Boodskap as strooipos gerapporteer.
       *[other] { $count } boodskappe as strooipos gerapporteer.
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] Gesprek as nie-strooipos gemerk en na die inkassie geskuif.
       *[other] { $count } gesprekke as nie-strooipos gemerk en na die inkassie geskuif.
    }
   *[message] { $count ->
        [one] Boodskap as nie-strooipos gemerk en na die inkassie geskuif.
       *[other] { $count } boodskappe as nie-strooipos gemerk en na die inkassie geskuif.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Gesprek permanent uitgevee.
       *[other] { $count } gesprekke permanent uitgevee.
    }
   *[message] { $count ->
        [one] Boodskap permanent uitgevee.
       *[other] { $count } boodskappe permanent uitgevee.
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] Gesprek as gelees gemerk.
       *[other] { $count } gesprekke as gelees gemerk.
    }
   *[message] { $count ->
        [one] Boodskap as gelees gemerk.
       *[other] { $count } boodskappe as gelees gemerk.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] Gesprek as ongelees gemerk.
       *[other] { $count } gesprekke as ongelees gemerk.
    }
   *[message] { $count ->
        [one] Boodskap as ongelees gemerk.
       *[other] { $count } boodskappe as ongelees gemerk.
    }
}
toast-undone = Aksie ontdoen.
toast-nothing-to-undo = Niks om te ontdoen nie.
toast-cannot-undo-delete-forever = E-pos wat permanent uitgevee is, kan nie teruggebring word nie.
toast-send-undone = Stuur ontdoen.
toast-too-late-to-undo-send = Te laat om te ontdoen: die boodskap is reeds gestuur.
toast-undo = Ontdoen
toast-no-spam-folder = Hierdie rekening het geen strooiposvouer nie.
