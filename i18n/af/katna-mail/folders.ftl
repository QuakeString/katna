# Katna Mail, Afrikaans (Afrikaans).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Etikette
nav-folders = Vouers
nav-label-new = Skep nuwe etiket
nav-folder-new = Skep nuwe vouer
nav-menu-check-mail = Kyk vir nuwe e-pos
nav-menu-check-inbox = Kyk hierdie inkassie na
nav-unified-leave-out = Laat uit die verenigde inkassie
nav-unified-bring-back = Bring terug in die verenigde inkassie
nav-menu-sign-in-again = Meld weer aan
nav-menu-new-mail = Nuwe e-pos van hierdie rekening
nav-menu-account-settings = Rekeninginstellings
nav-account-checked = Gesinkroniseer · nagegaan { $ago }
nav-account-in-sync = Gesinkroniseer
nav-account-connecting = Koppel tans…
nav-account-offline = Vanlyn, probeer weer
nav-account-signed-out = { $provider }-aanmelding het verval
nav-account-password-refused = Wagwoord geweier
nav-account-storage = { $used } van { $total } gebruik
nav-menu-new-subfolder = Nuwe vouer binne-in
nav-menu-new-sublabel = Nuwe etiket binne-in
nav-menu-rename = Hernoem
nav-menu-delete = Vee uit
nav-menu-empty-trash = Maak Asblik leeg
nav-account-unnamed = Rekening { $number }
nav-all-accounts = Alle rekeninge
nav-expand = Wys vouers
nav-collapse = Versteek vouers
storage-used = { $percent }% van { $total } gebruik
storage-used-detail = { $address }: { $used } van { $total } gebruik

## Special folders (the user's own folders keep their names)

folder-inbox = Inkassie
folder-starred = Gester
folder-snoozed = Gesluimer
folder-unread = Ongelees
folder-important = Belangrik
folder-drafts = Konsepte
folder-sent = Gestuur
folder-archive = Argief
folder-spam = Strooipos
folder-trash = Asblik
folder-all-mail = Alle pos
folder-scheduled = Geskeduleer
folder-waiting = Wag vir antwoord
folder-waiting-short = Wag
folder-reminders = Herinneringe
folder-outbox = Uitkassie
folder-activity = Aktiwiteit
folder-not-on-account = Hierdie rekening het nie so ’n vouer nie.

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Nuwe etiket
label-folder-new-title = Nuwe vouer
label-prompt = Voer 'n nuwe etiketnaam in:
label-folder-prompt = Voer 'n nuwe vouernaam in:
label-name-hint = Etiketnaam
label-folder-name-hint = Vouernaam
label-nest = Nes etiket onder:
label-folder-nest = Nes vouer onder:
label-cancel = Kanselleer
label-create = Skep
label-creating = Skep tans…
label-created = Etiket “{ $name }” is geskep.
label-folder-created = Vouer “{ $name }” is geskep.
label-rename-title = Hernoem etiket
label-folder-rename-title = Hernoem vouer
label-rename = Hernoem
label-renaming = Hernoem tans…
label-renamed = Etiket hernoem na “{ $name }”.
label-folder-renamed = Vouer hernoem na “{ $name }”.

## Deleting a folder or label (asked first)

folder-delete-title = Vee “{ $name }” uit?
folder-delete-body = { $count ->
    [0] Dit bevat geen e-pos nie. Die vouer word van die bediener verwyder, so webpos en jou foon verloor dit ook.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Sy { $count } gesprek gaan na die Asblik, sodat jy dit steeds kan terugkry.
           *[other] Sy { $count } gesprekke gaan na die Asblik, sodat jy dit steeds kan terugkry.
        }
       *[message] { $count ->
            [one] Sy { $count } boodskap gaan na die Asblik, sodat jy dit steeds kan terugkry.
           *[other] Sy { $count } boodskappe gaan na die Asblik, sodat jy dit steeds kan terugkry.
        }
    } Die vouer word van die bediener verwyder, so webpos en jou foon verloor dit ook.
}
folder-delete-forever-body = { $count ->
    [0] Dit bevat geen e-pos nie. Die vouer word van die bediener verwyder, so webpos en jou foon verloor dit ook.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Sy { $count } gesprek word vir altyd uitgevee; hierdie rekening het geen Asblik nie.
           *[other] Sy { $count } gesprekke word vir altyd uitgevee; hierdie rekening het geen Asblik nie.
        }
       *[message] { $count ->
            [one] Sy { $count } boodskap word vir altyd uitgevee; hierdie rekening het geen Asblik nie.
           *[other] Sy { $count } boodskappe word vir altyd uitgevee; hierdie rekening het geen Asblik nie.
        }
    } Die vouer word van die bediener verwyder, so webpos en jou foon verloor dit ook.
}
folder-delete-label-body = Die etiket word verwyder. Sy e-pos bly in Alle pos en in sy ander etikette.
folder-delete-confirm = Vee vouer uit
folder-delete-label-confirm = Vee etiket uit
folder-deleted = Vouer “{ $name }” uitgevee
label-deleted = Etiket “{ $name }” uitgevee
