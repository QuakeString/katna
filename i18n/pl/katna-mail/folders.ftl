# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Etykiety
nav-folders = Foldery
nav-label-new = Utwórz nową etykietę
nav-folder-new = Utwórz nowy folder
nav-menu-check-mail = Sprawdź nową pocztę
nav-menu-check-inbox = Sprawdź te Odebrane
nav-unified-leave-out = Pomiń we wspólnych Odebranych
nav-unified-bring-back = Przywróć do wspólnych Odebranych
nav-menu-sign-in-again = Zaloguj się ponownie
nav-menu-new-mail = Nowa wiadomość z tego konta
nav-menu-account-settings = Ustawienia konta
nav-account-checked = Zsynchronizowane · sprawdzono { $ago }
nav-account-in-sync = Zsynchronizowane
nav-account-connecting = Łączenie…
nav-account-offline = Offline, ponawianie próby
nav-account-signed-out = Logowanie { $provider } wygasło
nav-account-password-refused = Hasło odrzucone
nav-account-storage = Użyto { $used } z { $total }
nav-menu-new-subfolder = Nowy folder w środku
nav-menu-new-sublabel = Nowa etykieta w środku
nav-menu-rename = Zmień nazwę
nav-menu-delete = Usuń
nav-menu-empty-trash = Opróżnij kosz
nav-account-unnamed = Konto { $number }
nav-all-accounts = Wszystkie konta
nav-expand = Pokaż foldery
nav-collapse = Ukryj foldery
storage-used = Wykorzystano { $percent }% z { $total }
storage-used-detail = { $address }: wykorzystano { $used } z { $total }

## Special folders (the user's own folders keep their names)

folder-inbox = Odebrane
folder-starred = Oznaczone gwiazdką
folder-snoozed = Odłożone
folder-unread = Nieprzeczytane
folder-important = Ważne
folder-drafts = Wersje robocze
folder-sent = Wysłane
folder-archive = Archiwum
folder-spam = Spam
folder-trash = Kosz
folder-all-mail = Wszystkie
folder-scheduled = Zaplanowane
folder-waiting = Czeka na odpowiedź
folder-waiting-short = Oczekujące
folder-reminders = Przypomnienia
folder-outbox = Skrzynka nadawcza
folder-activity = Aktywność
folder-not-on-account = To konto nie ma takiego folderu.

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Nowa etykieta
label-folder-new-title = Nowy folder
label-prompt = Wpisz nazwę nowej etykiety:
label-folder-prompt = Wpisz nazwę nowego folderu:
label-name-hint = Nazwa etykiety
label-folder-name-hint = Nazwa folderu
label-nest = Zagnieźdź etykietę pod:
label-folder-nest = Zagnieźdź folder pod:
label-cancel = Anuluj
label-create = Utwórz
label-creating = Tworzenie…
label-created = Utworzono etykietę „{ $name }”.
label-folder-created = Utworzono folder „{ $name }”.
label-rename-title = Zmień nazwę etykiety
label-folder-rename-title = Zmień nazwę folderu
label-rename = Zmień nazwę
label-renaming = Zmienianie nazwy…
label-renamed = Zmieniono nazwę etykiety na „{ $name }”.
label-folder-renamed = Zmieniono nazwę folderu na „{ $name }”.

## Deleting a folder or label (asked first)

folder-delete-title = Usunąć „{ $name }”?
folder-delete-body = { $count ->
    [0] Nie zawiera poczty. Folder zostanie usunięty z serwera, więc zniknie też z poczty w przeglądarce i z telefonu.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] { $count } wątek z tego folderu trafi do Kosza, więc nadal możesz go odzyskać.
            [few] { $count } wątki z tego folderu trafią do Kosza, więc nadal możesz je odzyskać.
            [many] { $count } wątków z tego folderu trafi do Kosza, więc nadal możesz je odzyskać.
           *[other] { $count } wątku z tego folderu trafi do Kosza, więc nadal możesz je odzyskać.
        }
       *[message] { $count ->
            [one] { $count } wiadomość z tego folderu trafi do Kosza, więc nadal możesz ją odzyskać.
            [few] { $count } wiadomości z tego folderu trafią do Kosza, więc nadal możesz je odzyskać.
            [many] { $count } wiadomości z tego folderu trafi do Kosza, więc nadal możesz je odzyskać.
           *[other] { $count } wiadomości z tego folderu trafi do Kosza, więc nadal możesz je odzyskać.
        }
    } Folder zostanie usunięty z serwera, więc zniknie też z poczty w przeglądarce i z telefonu.
}
folder-delete-forever-body = { $count ->
    [0] Nie zawiera poczty. Folder zostanie usunięty z serwera, więc zniknie też z poczty w przeglądarce i z telefonu.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] { $count } wątek z tego folderu zostanie usunięty na zawsze; to konto nie ma Kosza.
            [few] { $count } wątki z tego folderu zostaną usunięte na zawsze; to konto nie ma Kosza.
            [many] { $count } wątków z tego folderu zostanie usuniętych na zawsze; to konto nie ma Kosza.
           *[other] { $count } wątku z tego folderu zostanie usuniętych na zawsze; to konto nie ma Kosza.
        }
       *[message] { $count ->
            [one] { $count } wiadomość z tego folderu zostanie usunięta na zawsze; to konto nie ma Kosza.
            [few] { $count } wiadomości z tego folderu zostaną usunięte na zawsze; to konto nie ma Kosza.
            [many] { $count } wiadomości z tego folderu zostanie usuniętych na zawsze; to konto nie ma Kosza.
           *[other] { $count } wiadomości z tego folderu zostanie usuniętych na zawsze; to konto nie ma Kosza.
        }
    } Folder zostanie usunięty z serwera, więc zniknie też z poczty w przeglądarce i z telefonu.
}
folder-delete-label-body = Etykieta zostanie usunięta. Jej poczta zostaje w folderze Wszystkie i w pozostałych etykietach.
folder-delete-confirm = Usuń folder
folder-delete-label-confirm = Usuń etykietę
folder-deleted = Usunięto folder „{ $name }”
label-deleted = Usunięto etykietę „{ $name }”
