# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Główne
tab-promotions = Oferty
tab-social = Społeczności
tab-updates = Powiadomienia
tab-forums = Fora
tab-focused = Priorytetowe
tab-other = Inne
tab-inbox = Odebrane
tab-newsletters = Biuletyny
tab-notifications = Powiadomienia
tab-new = { $count ->
    [one] { $count } nowa
    [few] { $count } nowe
    [many] { $count } nowych
   *[other] { $count } nowej
}
tab-provider-other = sortuje Katna

## Mail list: toolbar

list-select = Zaznacz
list-refresh = Odśwież
list-more = Więcej
list-mark-read = Oznacz jako przeczytane
list-mark-unread = Oznacz jako nieprzeczytane
list-move-to = Przenieś do
list-archive = Archiwizuj
list-spam = Zgłoś spam
list-delete = Usuń
list-snooze = Odłóż
list-unsnooze = Anuluj odłożenie
list-newer = Nowsze
list-older = Starsze
list-range = { $first }–{ $last } z { $total }
list-range-about = { $first }–{ $last } z około { $total }
list-results = Wyniki dla „{ $query }”
list-results-corrected = Wyświetlane są wyniki dla „{ $query }”
list-search-instead = Zamiast tego szukaj „{ $query }”
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Wszystkie
list-pick-none = Żadne
list-pick-read = Przeczytane
list-pick-unread = Nieprzeczytane
list-pick-starred = Oznaczone gwiazdką
list-pick-unstarred = Bez gwiazdki

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] Zaznaczono { $count } wątek.
        [few] Zaznaczono wszystkie { $count } wątki.
        [many] Zaznaczono wszystkie { $count } wątków.
       *[other] Zaznaczono wszystkie wątki ({ $count }).
    }
   *[message] { $count ->
        [one] Zaznaczono { $count } wiadomość.
        [few] Zaznaczono wszystkie { $count } wiadomości.
        [many] Zaznaczono wszystkie { $count } wiadomości.
       *[other] Zaznaczono wszystkie wiadomości ({ $count }).
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] Zaznaczono { $count } wątek w folderze { $folder }.
        [few] Zaznaczono wszystkie { $count } wątki w folderze { $folder }.
        [many] Zaznaczono wszystkie { $count } wątków w folderze { $folder }.
       *[other] Zaznaczono wszystkie wątki ({ $count }) w folderze { $folder }.
    }
   *[message] { $count ->
        [one] Zaznaczono { $count } wiadomość w folderze { $folder }.
        [few] Zaznaczono wszystkie { $count } wiadomości w folderze { $folder }.
        [many] Zaznaczono wszystkie { $count } wiadomości w folderze { $folder }.
       *[other] Zaznaczono wszystkie wiadomości ({ $count }) w folderze { $folder }.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] Zaznaczono { $count } wątek na tej stronie.
        [few] Zaznaczono wszystkie { $count } wątki na tej stronie.
        [many] Zaznaczono wszystkie { $count } wątków na tej stronie.
       *[other] Zaznaczono wszystkie wątki ({ $count }) na tej stronie.
    }
   *[message] { $count ->
        [one] Zaznaczono { $count } wiadomość na tej stronie.
        [few] Zaznaczono wszystkie { $count } wiadomości na tej stronie.
        [many] Zaznaczono wszystkie { $count } wiadomości na tej stronie.
       *[other] Zaznaczono wszystkie wiadomości ({ $count }) na tej stronie.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Zaznacz { $count } wątek
        [few] Zaznacz wszystkie { $count } wątki
        [many] Zaznacz wszystkie { $count } wątków
       *[other] Zaznacz wszystkie wątki ({ $count })
    }
   *[message] { $count ->
        [one] Zaznacz { $count } wiadomość
        [few] Zaznacz wszystkie { $count } wiadomości
        [many] Zaznacz wszystkie { $count } wiadomości
       *[other] Zaznacz wszystkie wiadomości ({ $count })
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Zaznacz { $count } wątek w folderze { $folder }
        [few] Zaznacz wszystkie { $count } wątki w folderze { $folder }
        [many] Zaznacz wszystkie { $count } wątków w folderze { $folder }
       *[other] Zaznacz wszystkie wątki ({ $count }) w folderze { $folder }
    }
   *[message] { $count ->
        [one] Zaznacz { $count } wiadomość w folderze { $folder }
        [few] Zaznacz wszystkie { $count } wiadomości w folderze { $folder }
        [many] Zaznacz wszystkie { $count } wiadomości w folderze { $folder }
       *[other] Zaznacz wszystkie wiadomości ({ $count }) w folderze { $folder }
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Zaznaczono { $count } przeczytany wątek na tej stronie.
            [few] Zaznaczono wszystkie { $count } przeczytane wątki na tej stronie.
            [many] Zaznaczono wszystkie { $count } przeczytanych wątków na tej stronie.
           *[other] Zaznaczono wszystkie przeczytane wątki ({ $count }) na tej stronie.
        }
       *[message] { $count ->
            [one] Zaznaczono { $count } przeczytaną wiadomość na tej stronie.
            [few] Zaznaczono wszystkie { $count } przeczytane wiadomości na tej stronie.
            [many] Zaznaczono wszystkie { $count } przeczytanych wiadomości na tej stronie.
           *[other] Zaznaczono wszystkie przeczytane wiadomości ({ $count }) na tej stronie.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Zaznaczono { $count } nieprzeczytany wątek na tej stronie.
            [few] Zaznaczono wszystkie { $count } nieprzeczytane wątki na tej stronie.
            [many] Zaznaczono wszystkie { $count } nieprzeczytanych wątków na tej stronie.
           *[other] Zaznaczono wszystkie nieprzeczytane wątki ({ $count }) na tej stronie.
        }
       *[message] { $count ->
            [one] Zaznaczono { $count } nieprzeczytaną wiadomość na tej stronie.
            [few] Zaznaczono wszystkie { $count } nieprzeczytane wiadomości na tej stronie.
            [many] Zaznaczono wszystkie { $count } nieprzeczytanych wiadomości na tej stronie.
           *[other] Zaznaczono wszystkie nieprzeczytane wiadomości ({ $count }) na tej stronie.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Zaznaczono { $count } wątek oznaczony gwiazdką na tej stronie.
            [few] Zaznaczono wszystkie { $count } wątki oznaczone gwiazdką na tej stronie.
            [many] Zaznaczono wszystkie { $count } wątków oznaczonych gwiazdką na tej stronie.
           *[other] Zaznaczono wszystkie wątki oznaczone gwiazdką ({ $count }) na tej stronie.
        }
       *[message] { $count ->
            [one] Zaznaczono { $count } wiadomość oznaczoną gwiazdką na tej stronie.
            [few] Zaznaczono wszystkie { $count } wiadomości oznaczone gwiazdką na tej stronie.
            [many] Zaznaczono wszystkie { $count } wiadomości oznaczonych gwiazdką na tej stronie.
           *[other] Zaznaczono wszystkie wiadomości oznaczone gwiazdką ({ $count }) na tej stronie.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Zaznaczono { $count } wątek bez gwiazdki na tej stronie.
            [few] Zaznaczono wszystkie { $count } wątki bez gwiazdki na tej stronie.
            [many] Zaznaczono wszystkie { $count } wątków bez gwiazdki na tej stronie.
           *[other] Zaznaczono wszystkie wątki bez gwiazdki ({ $count }) na tej stronie.
        }
       *[message] { $count ->
            [one] Zaznaczono { $count } wiadomość bez gwiazdki na tej stronie.
            [few] Zaznaczono wszystkie { $count } wiadomości bez gwiazdki na tej stronie.
            [many] Zaznaczono wszystkie { $count } wiadomości bez gwiazdki na tej stronie.
           *[other] Zaznaczono wszystkie wiadomości bez gwiazdki ({ $count }) na tej stronie.
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Zaznacz { $count } przeczytany wątek
            [few] Zaznacz wszystkie { $count } przeczytane wątki
            [many] Zaznacz wszystkie { $count } przeczytanych wątków
           *[other] Zaznacz wszystkie przeczytane wątki ({ $count })
        }
       *[message] { $count ->
            [one] Zaznacz { $count } przeczytaną wiadomość
            [few] Zaznacz wszystkie { $count } przeczytane wiadomości
            [many] Zaznacz wszystkie { $count } przeczytanych wiadomości
           *[other] Zaznacz wszystkie przeczytane wiadomości ({ $count })
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Zaznacz { $count } nieprzeczytany wątek
            [few] Zaznacz wszystkie { $count } nieprzeczytane wątki
            [many] Zaznacz wszystkie { $count } nieprzeczytanych wątków
           *[other] Zaznacz wszystkie nieprzeczytane wątki ({ $count })
        }
       *[message] { $count ->
            [one] Zaznacz { $count } nieprzeczytaną wiadomość
            [few] Zaznacz wszystkie { $count } nieprzeczytane wiadomości
            [many] Zaznacz wszystkie { $count } nieprzeczytanych wiadomości
           *[other] Zaznacz wszystkie nieprzeczytane wiadomości ({ $count })
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Zaznacz { $count } wątek oznaczony gwiazdką
            [few] Zaznacz wszystkie { $count } wątki oznaczone gwiazdką
            [many] Zaznacz wszystkie { $count } wątków oznaczonych gwiazdką
           *[other] Zaznacz wszystkie wątki oznaczone gwiazdką ({ $count })
        }
       *[message] { $count ->
            [one] Zaznacz { $count } wiadomość oznaczoną gwiazdką
            [few] Zaznacz wszystkie { $count } wiadomości oznaczone gwiazdką
            [many] Zaznacz wszystkie { $count } wiadomości oznaczonych gwiazdką
           *[other] Zaznacz wszystkie wiadomości oznaczone gwiazdką ({ $count })
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Zaznacz { $count } wątek bez gwiazdki
            [few] Zaznacz wszystkie { $count } wątki bez gwiazdki
            [many] Zaznacz wszystkie { $count } wątków bez gwiazdki
           *[other] Zaznacz wszystkie wątki bez gwiazdki ({ $count })
        }
       *[message] { $count ->
            [one] Zaznacz { $count } wiadomość bez gwiazdki
            [few] Zaznacz wszystkie { $count } wiadomości bez gwiazdki
            [many] Zaznacz wszystkie { $count } wiadomości bez gwiazdki
           *[other] Zaznacz wszystkie wiadomości bez gwiazdki ({ $count })
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Zaznacz { $count } przeczytany wątek w folderze { $folder }
            [few] Zaznacz wszystkie { $count } przeczytane wątki w folderze { $folder }
            [many] Zaznacz wszystkie { $count } przeczytanych wątków w folderze { $folder }
           *[other] Zaznacz wszystkie przeczytane wątki ({ $count }) w folderze { $folder }
        }
       *[message] { $count ->
            [one] Zaznacz { $count } przeczytaną wiadomość w folderze { $folder }
            [few] Zaznacz wszystkie { $count } przeczytane wiadomości w folderze { $folder }
            [many] Zaznacz wszystkie { $count } przeczytanych wiadomości w folderze { $folder }
           *[other] Zaznacz wszystkie przeczytane wiadomości ({ $count }) w folderze { $folder }
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Zaznacz { $count } nieprzeczytany wątek w folderze { $folder }
            [few] Zaznacz wszystkie { $count } nieprzeczytane wątki w folderze { $folder }
            [many] Zaznacz wszystkie { $count } nieprzeczytanych wątków w folderze { $folder }
           *[other] Zaznacz wszystkie nieprzeczytane wątki ({ $count }) w folderze { $folder }
        }
       *[message] { $count ->
            [one] Zaznacz { $count } nieprzeczytaną wiadomość w folderze { $folder }
            [few] Zaznacz wszystkie { $count } nieprzeczytane wiadomości w folderze { $folder }
            [many] Zaznacz wszystkie { $count } nieprzeczytanych wiadomości w folderze { $folder }
           *[other] Zaznacz wszystkie nieprzeczytane wiadomości ({ $count }) w folderze { $folder }
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Zaznacz { $count } wątek oznaczony gwiazdką w folderze { $folder }
            [few] Zaznacz wszystkie { $count } wątki oznaczone gwiazdką w folderze { $folder }
            [many] Zaznacz wszystkie { $count } wątków oznaczonych gwiazdką w folderze { $folder }
           *[other] Zaznacz wszystkie wątki oznaczone gwiazdką ({ $count }) w folderze { $folder }
        }
       *[message] { $count ->
            [one] Zaznacz { $count } wiadomość oznaczoną gwiazdką w folderze { $folder }
            [few] Zaznacz wszystkie { $count } wiadomości oznaczone gwiazdką w folderze { $folder }
            [many] Zaznacz wszystkie { $count } wiadomości oznaczonych gwiazdką w folderze { $folder }
           *[other] Zaznacz wszystkie wiadomości oznaczone gwiazdką ({ $count }) w folderze { $folder }
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Zaznacz { $count } wątek bez gwiazdki w folderze { $folder }
            [few] Zaznacz wszystkie { $count } wątki bez gwiazdki w folderze { $folder }
            [many] Zaznacz wszystkie { $count } wątków bez gwiazdki w folderze { $folder }
           *[other] Zaznacz wszystkie wątki bez gwiazdki ({ $count }) w folderze { $folder }
        }
       *[message] { $count ->
            [one] Zaznacz { $count } wiadomość bez gwiazdki w folderze { $folder }
            [few] Zaznacz wszystkie { $count } wiadomości bez gwiazdki w folderze { $folder }
            [many] Zaznacz wszystkie { $count } wiadomości bez gwiazdki w folderze { $folder }
           *[other] Zaznacz wszystkie wiadomości bez gwiazdki ({ $count }) w folderze { $folder }
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Zaznaczono { $count } przeczytany wątek.
            [few] Zaznaczono wszystkie { $count } przeczytane wątki.
            [many] Zaznaczono wszystkie { $count } przeczytanych wątków.
           *[other] Zaznaczono wszystkie przeczytane wątki ({ $count }).
        }
       *[message] { $count ->
            [one] Zaznaczono { $count } przeczytaną wiadomość.
            [few] Zaznaczono wszystkie { $count } przeczytane wiadomości.
            [many] Zaznaczono wszystkie { $count } przeczytanych wiadomości.
           *[other] Zaznaczono wszystkie przeczytane wiadomości ({ $count }).
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Zaznaczono { $count } nieprzeczytany wątek.
            [few] Zaznaczono wszystkie { $count } nieprzeczytane wątki.
            [many] Zaznaczono wszystkie { $count } nieprzeczytanych wątków.
           *[other] Zaznaczono wszystkie nieprzeczytane wątki ({ $count }).
        }
       *[message] { $count ->
            [one] Zaznaczono { $count } nieprzeczytaną wiadomość.
            [few] Zaznaczono wszystkie { $count } nieprzeczytane wiadomości.
            [many] Zaznaczono wszystkie { $count } nieprzeczytanych wiadomości.
           *[other] Zaznaczono wszystkie nieprzeczytane wiadomości ({ $count }).
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Zaznaczono { $count } wątek oznaczony gwiazdką.
            [few] Zaznaczono wszystkie { $count } wątki oznaczone gwiazdką.
            [many] Zaznaczono wszystkie { $count } wątków oznaczonych gwiazdką.
           *[other] Zaznaczono wszystkie wątki oznaczone gwiazdką ({ $count }).
        }
       *[message] { $count ->
            [one] Zaznaczono { $count } wiadomość oznaczoną gwiazdką.
            [few] Zaznaczono wszystkie { $count } wiadomości oznaczone gwiazdką.
            [many] Zaznaczono wszystkie { $count } wiadomości oznaczonych gwiazdką.
           *[other] Zaznaczono wszystkie wiadomości oznaczone gwiazdką ({ $count }).
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Zaznaczono { $count } wątek bez gwiazdki.
            [few] Zaznaczono wszystkie { $count } wątki bez gwiazdki.
            [many] Zaznaczono wszystkie { $count } wątków bez gwiazdki.
           *[other] Zaznaczono wszystkie wątki bez gwiazdki ({ $count }).
        }
       *[message] { $count ->
            [one] Zaznaczono { $count } wiadomość bez gwiazdki.
            [few] Zaznaczono wszystkie { $count } wiadomości bez gwiazdki.
            [many] Zaznaczono wszystkie { $count } wiadomości bez gwiazdki.
           *[other] Zaznaczono wszystkie wiadomości bez gwiazdki ({ $count }).
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Zaznaczono { $count } przeczytany wątek w folderze { $folder }.
            [few] Zaznaczono wszystkie { $count } przeczytane wątki w folderze { $folder }.
            [many] Zaznaczono wszystkie { $count } przeczytanych wątków w folderze { $folder }.
           *[other] Zaznaczono wszystkie przeczytane wątki ({ $count }) w folderze { $folder }.
        }
       *[message] { $count ->
            [one] Zaznaczono { $count } przeczytaną wiadomość w folderze { $folder }.
            [few] Zaznaczono wszystkie { $count } przeczytane wiadomości w folderze { $folder }.
            [many] Zaznaczono wszystkie { $count } przeczytanych wiadomości w folderze { $folder }.
           *[other] Zaznaczono wszystkie przeczytane wiadomości ({ $count }) w folderze { $folder }.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Zaznaczono { $count } nieprzeczytany wątek w folderze { $folder }.
            [few] Zaznaczono wszystkie { $count } nieprzeczytane wątki w folderze { $folder }.
            [many] Zaznaczono wszystkie { $count } nieprzeczytanych wątków w folderze { $folder }.
           *[other] Zaznaczono wszystkie nieprzeczytane wątki ({ $count }) w folderze { $folder }.
        }
       *[message] { $count ->
            [one] Zaznaczono { $count } nieprzeczytaną wiadomość w folderze { $folder }.
            [few] Zaznaczono wszystkie { $count } nieprzeczytane wiadomości w folderze { $folder }.
            [many] Zaznaczono wszystkie { $count } nieprzeczytanych wiadomości w folderze { $folder }.
           *[other] Zaznaczono wszystkie nieprzeczytane wiadomości ({ $count }) w folderze { $folder }.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Zaznaczono { $count } wątek oznaczony gwiazdką w folderze { $folder }.
            [few] Zaznaczono wszystkie { $count } wątki oznaczone gwiazdką w folderze { $folder }.
            [many] Zaznaczono wszystkie { $count } wątków oznaczonych gwiazdką w folderze { $folder }.
           *[other] Zaznaczono wszystkie wątki oznaczone gwiazdką ({ $count }) w folderze { $folder }.
        }
       *[message] { $count ->
            [one] Zaznaczono { $count } wiadomość oznaczoną gwiazdką w folderze { $folder }.
            [few] Zaznaczono wszystkie { $count } wiadomości oznaczone gwiazdką w folderze { $folder }.
            [many] Zaznaczono wszystkie { $count } wiadomości oznaczonych gwiazdką w folderze { $folder }.
           *[other] Zaznaczono wszystkie wiadomości oznaczone gwiazdką ({ $count }) w folderze { $folder }.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Zaznaczono { $count } wątek bez gwiazdki w folderze { $folder }.
            [few] Zaznaczono wszystkie { $count } wątki bez gwiazdki w folderze { $folder }.
            [many] Zaznaczono wszystkie { $count } wątków bez gwiazdki w folderze { $folder }.
           *[other] Zaznaczono wszystkie wątki bez gwiazdki ({ $count }) w folderze { $folder }.
        }
       *[message] { $count ->
            [one] Zaznaczono { $count } wiadomość bez gwiazdki w folderze { $folder }.
            [few] Zaznaczono wszystkie { $count } wiadomości bez gwiazdki w folderze { $folder }.
            [many] Zaznaczono wszystkie { $count } wiadomości bez gwiazdki w folderze { $folder }.
           *[other] Zaznaczono wszystkie wiadomości bez gwiazdki ({ $count }) w folderze { $folder }.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Brak tu przeczytanych wątków.
       *[message] Brak tu przeczytanych wiadomości.
    }
   *[unread] { $kind ->
        [conversation] Brak tu nieprzeczytanych wątków.
       *[message] Brak tu nieprzeczytanych wiadomości.
    }
    [starred] { $kind ->
        [conversation] Brak tu wątków oznaczonych gwiazdką.
       *[message] Brak tu wiadomości oznaczonych gwiazdką.
    }
    [unstarred] { $kind ->
        [conversation] Brak tu wątków bez gwiazdki.
       *[message] Brak tu wiadomości bez gwiazdki.
    }
}
list-clear-selection = Wyczyść zaznaczenie

## Mail list: empty states

list-empty-search = Brak wiadomości pasujących do wyszukiwania.
list-empty-tab = Brak poczty na karcie { $tab }.
list-empty-tab-unknown = Brak poczty na tej karcie.
list-empty-folder = Brak wiadomości w folderze { $folder }.
list-empty-folder-unknown = Brak wiadomości w tym folderze.
list-first-sync = Pobieranie poczty…
list-first-sync-detail = Wiadomości pojawią się tutaj, gdy tylko dotrą.

## Mail list: lines

row-removed = Ta wiadomość została usunięta.
row-starred = Oznaczone gwiazdką
row-not-starred = Bez gwiazdki
row-important = Ważne. Kliknij, aby oznaczyć jako nieważne.
row-mark-important = Oznacz jako ważne
row-pinned = Przypięte na górze
row-pin = Przypnij na górze
row-unpin = Odepnij
row-snoozed-until = Odłożone do { $when }

## Mail list: More menu and right-click menu

menu-reply = Odpowiedz
menu-reply-all = Odpowiedz wszystkim
menu-forward = Przekaż dalej
menu-archive = Archiwizuj
menu-delete = Usuń
menu-delete-forever = Usuń trwale
menu-move-to-inbox = Przenieś do Odebranych
menu-spam = Zgłoś spam
menu-not-spam = To nie spam
menu-mark-read = Oznacz jako przeczytane
menu-mark-unread = Oznacz jako nieprzeczytane
menu-mark-all-read = Oznacz wszystkie jako przeczytane
menu-star = Dodaj gwiazdkę
menu-unstar = Usuń gwiazdkę
menu-important = Oznacz jako ważne
menu-not-important = Oznacz jako nieważne
menu-pin = Przypnij na górze
menu-unpin = Odepnij
menu-snooze = Odłóż
menu-unsnooze = Anuluj odłożenie
menu-print-all = Drukuj wszystko
menu-new-window = Otwórz w nowym oknie
menu-move-to = Przenieś do
menu-move-to-heading = Przenieś do:
menu-find-from = Znajdź e-maile od { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Wątek zarchiwizowany.
        [few] Zarchiwizowano { $count } wątki.
        [many] Zarchiwizowano { $count } wątków.
       *[other] Zarchiwizowano { $count } wątku.
    }
   *[message] { $count ->
        [one] Wiadomość zarchiwizowana.
        [few] Zarchiwizowano { $count } wiadomości.
        [many] Zarchiwizowano { $count } wiadomości.
       *[other] Zarchiwizowano { $count } wiadomości.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Wątek przeniesiony do kosza.
        [few] Przeniesiono { $count } wątki do kosza.
        [many] Przeniesiono { $count } wątków do kosza.
       *[other] Przeniesiono { $count } wątku do kosza.
    }
   *[message] { $count ->
        [one] Wiadomość przeniesiona do kosza.
        [few] Przeniesiono { $count } wiadomości do kosza.
        [many] Przeniesiono { $count } wiadomości do kosza.
       *[other] Przeniesiono { $count } wiadomości do kosza.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Wątek przeniesiony.
        [few] Przeniesiono { $count } wątki.
        [many] Przeniesiono { $count } wątków.
       *[other] Przeniesiono { $count } wątku.
    }
   *[message] { $count ->
        [one] Wiadomość przeniesiona.
        [few] Przeniesiono { $count } wiadomości.
        [many] Przeniesiono { $count } wiadomości.
       *[other] Przeniesiono { $count } wiadomości.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Wątek oznaczony gwiazdką.
        [few] Oznaczono gwiazdką { $count } wątki.
        [many] Oznaczono gwiazdką { $count } wątków.
       *[other] Oznaczono gwiazdką { $count } wątku.
    }
   *[message] { $count ->
        [one] Wiadomość oznaczona gwiazdką.
        [few] Oznaczono gwiazdką { $count } wiadomości.
        [many] Oznaczono gwiazdką { $count } wiadomości.
       *[other] Oznaczono gwiazdką { $count } wiadomości.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Usunięto gwiazdkę z wątku.
        [few] Usunięto gwiazdkę z { $count } wątków.
        [many] Usunięto gwiazdkę z { $count } wątków.
       *[other] Usunięto gwiazdkę z { $count } wątku.
    }
   *[message] { $count ->
        [one] Usunięto gwiazdkę z wiadomości.
        [few] Usunięto gwiazdkę z { $count } wiadomości.
        [many] Usunięto gwiazdkę z { $count } wiadomości.
       *[other] Usunięto gwiazdkę z { $count } wiadomości.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Wątek oznaczony jako ważny.
        [few] Oznaczono { $count } wątki jako ważne.
        [many] Oznaczono { $count } wątków jako ważne.
       *[other] Oznaczono { $count } wątku jako ważne.
    }
   *[message] { $count ->
        [one] Wiadomość oznaczona jako ważna.
        [few] Oznaczono { $count } wiadomości jako ważne.
        [many] Oznaczono { $count } wiadomości jako ważne.
       *[other] Oznaczono { $count } wiadomości jako ważne.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Wątek oznaczony jako nieważny.
        [few] Oznaczono { $count } wątki jako nieważne.
        [many] Oznaczono { $count } wątków jako nieważne.
       *[other] Oznaczono { $count } wątku jako nieważne.
    }
   *[message] { $count ->
        [one] Wiadomość oznaczona jako nieważna.
        [few] Oznaczono { $count } wiadomości jako nieważne.
        [many] Oznaczono { $count } wiadomości jako nieważne.
       *[other] Oznaczono { $count } wiadomości jako nieważne.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Wątek przypięty na górze.
        [few] Przypięto { $count } wątki na górze.
        [many] Przypięto { $count } wątków na górze.
       *[other] Przypięto { $count } wątku na górze.
    }
   *[message] { $count ->
        [one] Wiadomość przypięta na górze.
        [few] Przypięto { $count } wiadomości na górze.
        [many] Przypięto { $count } wiadomości na górze.
       *[other] Przypięto { $count } wiadomości na górze.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Wątek odpięty.
        [few] Odpięto { $count } wątki.
        [many] Odpięto { $count } wątków.
       *[other] Odpięto { $count } wątku.
    }
   *[message] { $count ->
        [one] Wiadomość odpięta.
        [few] Odpięto { $count } wiadomości.
        [many] Odpięto { $count } wiadomości.
       *[other] Odpięto { $count } wiadomości.
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] Wątek odłożony do { $when }.
        [few] Odłożono { $count } wątki do { $when }.
        [many] Odłożono { $count } wątków do { $when }.
       *[other] Odłożono { $count } wątku do { $when }.
    }
   *[message] { $count ->
        [one] Wiadomość odłożona do { $when }.
        [few] Odłożono { $count } wiadomości do { $when }.
        [many] Odłożono { $count } wiadomości do { $when }.
       *[other] Odłożono { $count } wiadomości do { $when }.
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] Wątek wrócił do Odebranych.
        [few] Przywrócono { $count } wątki do Odebranych.
        [many] Przywrócono { $count } wątków do Odebranych.
       *[other] Przywrócono { $count } wątku do Odebranych.
    }
   *[message] { $count ->
        [one] Wiadomość wróciła do Odebranych.
        [few] Przywrócono { $count } wiadomości do Odebranych.
        [many] Przywrócono { $count } wiadomości do Odebranych.
       *[other] Przywrócono { $count } wiadomości do Odebranych.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Wątek zgłoszony jako spam.
        [few] Zgłoszono { $count } wątki jako spam.
        [many] Zgłoszono { $count } wątków jako spam.
       *[other] Zgłoszono { $count } wątku jako spam.
    }
   *[message] { $count ->
        [one] Wiadomość zgłoszona jako spam.
        [few] Zgłoszono { $count } wiadomości jako spam.
        [many] Zgłoszono { $count } wiadomości jako spam.
       *[other] Zgłoszono { $count } wiadomości jako spam.
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] Wątek oznaczony jako niebędący spamem i przeniesiony do Odebranych.
        [few] Oznaczono { $count } wątki jako niebędące spamem i przeniesiono do Odebranych.
        [many] Oznaczono { $count } wątków jako niebędące spamem i przeniesiono do Odebranych.
       *[other] Oznaczono { $count } wątku jako niebędące spamem i przeniesiono do Odebranych.
    }
   *[message] { $count ->
        [one] Wiadomość oznaczona jako niebędąca spamem i przeniesiona do Odebranych.
        [few] Oznaczono { $count } wiadomości jako niebędące spamem i przeniesiono do Odebranych.
        [many] Oznaczono { $count } wiadomości jako niebędące spamem i przeniesiono do Odebranych.
       *[other] Oznaczono { $count } wiadomości jako niebędące spamem i przeniesiono do Odebranych.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Wątek trwale usunięty.
        [few] Trwale usunięto { $count } wątki.
        [many] Trwale usunięto { $count } wątków.
       *[other] Trwale usunięto { $count } wątku.
    }
   *[message] { $count ->
        [one] Wiadomość trwale usunięta.
        [few] Trwale usunięto { $count } wiadomości.
        [many] Trwale usunięto { $count } wiadomości.
       *[other] Trwale usunięto { $count } wiadomości.
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] Wątek oznaczony jako przeczytany.
        [few] Oznaczono { $count } wątki jako przeczytane.
        [many] Oznaczono { $count } wątków jako przeczytane.
       *[other] Oznaczono { $count } wątku jako przeczytane.
    }
   *[message] { $count ->
        [one] Wiadomość oznaczona jako przeczytana.
        [few] Oznaczono { $count } wiadomości jako przeczytane.
        [many] Oznaczono { $count } wiadomości jako przeczytane.
       *[other] Oznaczono { $count } wiadomości jako przeczytane.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] Wątek oznaczony jako nieprzeczytany.
        [few] Oznaczono { $count } wątki jako nieprzeczytane.
        [many] Oznaczono { $count } wątków jako nieprzeczytane.
       *[other] Oznaczono { $count } wątku jako nieprzeczytane.
    }
   *[message] { $count ->
        [one] Wiadomość oznaczona jako nieprzeczytana.
        [few] Oznaczono { $count } wiadomości jako nieprzeczytane.
        [many] Oznaczono { $count } wiadomości jako nieprzeczytane.
       *[other] Oznaczono { $count } wiadomości jako nieprzeczytane.
    }
}
toast-undone = Cofnięto działanie.
toast-nothing-to-undo = Nie ma nic do cofnięcia.
toast-cannot-undo-delete-forever = Trwale usuniętej poczty nie da się przywrócić.
toast-send-undone = Cofnięto wysłanie.
toast-too-late-to-undo-send = Za późno na cofnięcie: wiadomość została już wysłana.
toast-undo = Cofnij
toast-no-spam-folder = To konto nie ma folderu spamu.
