# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Labels
nav-folders = Mappen
nav-label-new = Nieuw label maken
nav-folder-new = Nieuwe map maken
nav-menu-check-mail = Controleren op nieuwe e-mail
nav-menu-check-inbox = Deze inbox controleren
nav-unified-leave-out = Weglaten uit gecombineerde inbox
nav-unified-bring-back = Terugzetten in gecombineerde inbox
nav-menu-sign-in-again = Opnieuw aanmelden
nav-menu-new-mail = Nieuwe e-mail vanuit dit account
nav-menu-account-settings = Accountinstellingen
nav-account-checked = Gesynchroniseerd · gecontroleerd { $ago }
nav-account-in-sync = Gesynchroniseerd
nav-account-connecting = Verbinden…
nav-account-offline = Offline, opnieuw proberen
nav-account-signed-out = Aanmelding bij { $provider } verlopen
nav-account-password-refused = Wachtwoord geweigerd
nav-account-storage = { $used } van { $total } gebruikt
nav-menu-new-subfolder = Nieuwe map erin
nav-menu-new-sublabel = Nieuw label erin
nav-menu-rename = Hernoemen
nav-menu-delete = Verwijderen
nav-menu-empty-trash = Prullenbak legen
nav-account-unnamed = Account { $number }
nav-all-accounts = Alle accounts
nav-expand = Mappen tonen
nav-collapse = Mappen verbergen
storage-used = { $percent }% van { $total } gebruikt
storage-used-detail = { $address }: { $used } van { $total } gebruikt

## Special folders (the user's own folders keep their names)

folder-inbox = Inbox
folder-starred = Met ster
folder-snoozed = Gesnoozed
folder-unread = Ongelezen
folder-important = Belangrijk
folder-drafts = Concepten
folder-sent = Verzonden
folder-archive = Archief
folder-spam = Spam
folder-trash = Prullenbak
folder-all-mail = Alle berichten
folder-scheduled = Gepland
folder-waiting = Wacht op antwoord
folder-waiting-short = Wachtend
folder-reminders = Herinneringen
folder-outbox = Postvak UIT
folder-activity = Activiteit
folder-not-on-account = Dit account heeft zo’n map niet.

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Nieuw label
label-folder-new-title = Nieuwe map
label-prompt = Geef een nieuwe labelnaam op:
label-folder-prompt = Geef een nieuwe mapnaam op:
label-name-hint = Labelnaam
label-folder-name-hint = Mapnaam
label-nest = Label nesten onder:
label-folder-nest = Map nesten onder:
label-cancel = Annuleren
label-create = Maken
label-creating = Maken…
label-created = Label ‘{ $name }’ gemaakt.
label-folder-created = Map ‘{ $name }’ gemaakt.
label-rename-title = Label hernoemen
label-folder-rename-title = Map hernoemen
label-rename = Hernoemen
label-renaming = Hernoemen…
label-renamed = Label hernoemd naar ‘{ $name }’.
label-folder-renamed = Map hernoemd naar ‘{ $name }’.

## Deleting a folder or label (asked first)

folder-delete-title = ‘{ $name }’ verwijderen?
folder-delete-body = { $count ->
    [0] Er staat geen e-mail in. De map wordt van de server verwijderd, dus ook webmail en je telefoon raken hem kwijt.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Het { $count } gesprek erin gaat naar de Prullenbak, zodat je het nog terug kunt halen.
           *[other] De { $count } gesprekken erin gaan naar de Prullenbak, zodat je ze nog terug kunt halen.
        }
       *[message] { $count ->
            [one] Het { $count } bericht erin gaat naar de Prullenbak, zodat je het nog terug kunt halen.
           *[other] De { $count } berichten erin gaan naar de Prullenbak, zodat je ze nog terug kunt halen.
        }
    } De map wordt van de server verwijderd, dus ook webmail en je telefoon raken hem kwijt.
}
folder-delete-forever-body = { $count ->
    [0] Er staat geen e-mail in. De map wordt van de server verwijderd, dus ook webmail en je telefoon raken hem kwijt.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Het { $count } gesprek erin wordt definitief verwijderd; dit account heeft geen Prullenbak.
           *[other] De { $count } gesprekken erin worden definitief verwijderd; dit account heeft geen Prullenbak.
        }
       *[message] { $count ->
            [one] Het { $count } bericht erin wordt definitief verwijderd; dit account heeft geen Prullenbak.
           *[other] De { $count } berichten erin worden definitief verwijderd; dit account heeft geen Prullenbak.
        }
    } De map wordt van de server verwijderd, dus ook webmail en je telefoon raken hem kwijt.
}
folder-delete-label-body = Het label wordt verwijderd. De e-mail ervan blijft in Alle berichten en in de andere labels.
folder-delete-confirm = Map verwijderen
folder-delete-label-confirm = Label verwijderen
folder-deleted = Map ‘{ $name }’ verwijderd
label-deleted = Label ‘{ $name }’ verwijderd
