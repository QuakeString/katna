# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Etiketter
nav-folders = Mappar
nav-label-new = Skapa ny etikett
nav-folder-new = Skapa ny mapp
nav-menu-check-mail = Sök efter ny e-post
nav-menu-check-inbox = Sök efter ny e-post i den här inkorgen
nav-unified-leave-out = Utelämna från gemensam inkorg
nav-unified-bring-back = Ta tillbaka till gemensam inkorg
nav-menu-sign-in-again = Logga in igen
nav-menu-new-mail = Ny e-post från det här kontot
nav-menu-account-settings = Kontoinställningar
nav-account-checked = Synkroniserat · kontrollerat { $ago }
nav-account-in-sync = Synkroniserat
nav-account-connecting = Ansluter…
nav-account-offline = Offline, försöker igen
nav-account-signed-out = Inloggningen hos { $provider } har gått ut
nav-account-password-refused = Lösenordet nekades
nav-account-storage = { $used } av { $total } används
nav-menu-new-subfolder = Ny mapp inuti
nav-menu-new-sublabel = Ny etikett inuti
nav-menu-rename = Byt namn
nav-menu-delete = Radera
nav-menu-empty-trash = Töm papperskorgen
nav-account-unnamed = Konto { $number }
nav-all-accounts = Alla konton
nav-expand = Visa mappar
nav-collapse = Dölj mappar
storage-used = { $percent } % av { $total } används
storage-used-detail = { $address }: { $used } av { $total } används

## Special folders (the user's own folders keep their names)

folder-inbox = Inkorgen
folder-starred = Stjärnmärkt
folder-snoozed = Snoozade
folder-unread = Olästa
folder-important = Viktigt
folder-drafts = Utkast
folder-sent = Skickat
folder-archive = Arkiv
folder-spam = Skräppost
folder-trash = Papperskorgen
folder-all-mail = Alla mail
folder-scheduled = Schemalagt
folder-waiting = Väntar på svar
folder-waiting-short = Väntar
folder-reminders = Påminnelser
folder-outbox = Utkorgen
folder-activity = Aktivitet
folder-not-on-account = Det här kontot har ingen sådan mapp.

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Ny etikett
label-folder-new-title = Ny mapp
label-prompt = Ange ett nytt etikettnamn:
label-folder-prompt = Ange ett nytt mappnamn:
label-name-hint = Etikettnamn
label-folder-name-hint = Mappnamn
label-nest = Kapsla etikett under:
label-folder-nest = Kapsla mapp under:
label-cancel = Avbryt
label-create = Skapa
label-creating = Skapar…
label-created = Etiketten ”{ $name }” har skapats.
label-folder-created = Mappen ”{ $name }” har skapats.
label-rename-title = Byt namn på etikett
label-folder-rename-title = Byt namn på mapp
label-rename = Byt namn
label-renaming = Byter namn…
label-renamed = Etiketten fick namnet ”{ $name }”.
label-folder-renamed = Mappen fick namnet ”{ $name }”.

## Deleting a folder or label (asked first)

folder-delete-title = Radera ”{ $name }”?
folder-delete-body = { $count ->
    [0] Den innehåller ingen e-post. Mappen tas bort från servern, så den försvinner även i webbmejlen och på din telefon.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Dess { $count } konversation flyttas till papperskorgen, så du kan fortfarande få tillbaka den.
           *[other] Dess { $count } konversationer flyttas till papperskorgen, så du kan fortfarande få tillbaka dem.
        }
       *[message] { $count ->
            [one] Dess { $count } meddelande flyttas till papperskorgen, så du kan fortfarande få tillbaka det.
           *[other] Dess { $count } meddelanden flyttas till papperskorgen, så du kan fortfarande få tillbaka dem.
        }
    } Mappen tas bort från servern, så den försvinner även i webbmejlen och på din telefon.
}
folder-delete-forever-body = { $count ->
    [0] Den innehåller ingen e-post. Mappen tas bort från servern, så den försvinner även i webbmejlen och på din telefon.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Dess { $count } konversation raderas permanent; det här kontot har ingen papperskorg.
           *[other] Dess { $count } konversationer raderas permanent; det här kontot har ingen papperskorg.
        }
       *[message] { $count ->
            [one] Dess { $count } meddelande raderas permanent; det här kontot har ingen papperskorg.
           *[other] Dess { $count } meddelanden raderas permanent; det här kontot har ingen papperskorg.
        }
    } Mappen tas bort från servern, så den försvinner även i webbmejlen och på din telefon.
}
folder-delete-label-body = Etiketten tas bort. Dess e-post finns kvar i Alla mail och under sina andra etiketter.
folder-delete-confirm = Radera mapp
folder-delete-label-confirm = Radera etikett
folder-deleted = Mappen ”{ $name }” raderades
label-deleted = Etiketten ”{ $name }” raderades
