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

## Mail list: More menu and right-click menu

menu-reply = Odpowiedz
menu-reply-all = Odpowiedz wszystkim
menu-forward = Przekaż dalej
menu-archive = Archiwizuj
menu-delete = Usuń
menu-spam = Zgłoś spam
menu-mark-read = Oznacz jako przeczytane
menu-mark-unread = Oznacz jako nieprzeczytane
menu-mark-all-read = Oznacz wszystkie jako przeczytane
menu-star = Dodaj gwiazdkę
menu-unstar = Usuń gwiazdkę
menu-important = Oznacz jako ważne
menu-not-important = Oznacz jako nieważne
menu-pin = Przypnij na górze
menu-unpin = Odepnij
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
toast-undone = Cofnięto działanie.
toast-undo = Cofnij
toast-no-spam-folder = To konto nie ma folderu spamu.
