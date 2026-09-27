# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Taal: { $language }
language-tooltip-system = Taal: { $language }, volgens het systeem
language-search = Taal zoeken
language-system-default = Systeemstandaard
language-system-now = Momenteel { $language }
language-no-match = Geen taal gevonden voor ‘{ $query }’
language-machine = Machinaal vertaald. Help mee dit te verbeteren
language-setting = Taal
language-setting-detail = De taal van menu’s, knoppen en berichten, en de notatie van datums en getallen. ‘Systeemstandaard’ volgt de desktop.

## Dates and sizes

ago-just-now = zojuist
ago-minutes = { $count ->
    [one] { $count } minuut geleden
   *[other] { $count } minuten geleden
}
ago-hours = { $count ->
    [one] { $count } uur geleden
   *[other] { $count } uur geleden
}
ago-days = { $count ->
    [one] { $count } dag geleden
   *[other] { $count } dagen geleden
}
size-bytes = { $count ->
    [one] { $count } byte
   *[other] { $count } bytes
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = Mappen verbergen
folders-show = Mappen tonen
compose = Opstellen
search = Zoeken
search-mail = Zoeken in e-mail
search-settings = Zoeken in instellingen
search-clear = Zoekopdracht wissen
search-options-show = Zoekopties tonen
settings = Instellingen
account-add = Account toevoegen

## App rail (and the bottom bar on a phone)

rail-mail = E-mail
rail-calendar = Agenda
rail-contacts = Contacten
rail-tasks = Taken
rail-notes = Notities
rail-feeds = Feeds

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Binnenkort beschikbaar
app-calendar-promise = Je CalDAV-agenda’s, vergaderuitnodigingen uit je e-mail en herinneringen, naast je inbox.
app-tasks-promise = Takenlijsten die synchroniseren met CalDAV, en taken gemaakt van e-mail.
app-notes-promise = Snelle notities, en notities bij een e-mail of gesprek voor later.
app-feeds-promise = Lees RSS- en Atom-feeds naast je e-mail.

## Contacts page

app-contacts-loading = Mensen uit je e-mail verzamelen…
app-contacts-empty = Mensen met wie je mailt, verschijnen hier.
app-contacts-count = { $count ->
    [one] { $count } persoon uit je e-mail, meest gemaild eerst
   *[other] { $count } mensen uit je e-mail, meest gemaild eerst
}
app-contacts-top = { $count ->
    [one] De top { $count } persoon uit je e-mail, meest gemaild eerst
   *[other] De top { $count } mensen uit je e-mail, meest gemaild eerst
}
app-contacts-messages = { $count ->
    [one] { $count } bericht
   *[other] { $count } berichten
}
app-contacts-last = laatst { $date }

## Navigation (the folders pane)

nav-labels = Labels
nav-folders = Mappen
nav-label-new = Nieuw label maken
nav-folder-new = Nieuwe map maken
nav-account-unnamed = Account { $number }
nav-tab-new = { $count ->
    [one] { $count } nieuw
   *[other] { $count } nieuw
}

## Special folders (the user's own folders keep their names)

folder-inbox = Inbox
folder-starred = Met ster
folder-drafts = Concepten
folder-sent = Verzonden
folder-archive = Archief
folder-spam = Spam
folder-trash = Prullenbak
folder-all-mail = Alle berichten
folder-scheduled = Gepland

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

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Primair
tab-promotions = Reclame
tab-social = Sociaal
tab-updates = Updates
tab-forums = Forums
tab-focused = Prioriteit
tab-other = Overige
tab-inbox = Inbox
tab-newsletters = Nieuwsbrieven
tab-notifications = Meldingen
tab-new = { $count } nieuw
tab-provider-other = gesorteerd door Katna

## Mail list: toolbar

list-select = Selecteren
list-refresh = Vernieuwen
list-more = Meer
list-mark-read = Markeren als gelezen
list-mark-unread = Markeren als ongelezen
list-move-to = Verplaatsen naar
list-archive = Archiveren
list-spam = Spam melden
list-delete = Verwijderen
list-newer = Nieuwer
list-older = Ouder
list-range = { $first }–{ $last } van { $total }
list-range-about = { $first }–{ $last } van ongeveer { $total }
list-results = Resultaten voor ‘{ $query }’
list-results-corrected = Resultaten weergegeven voor ‘{ $query }’
list-search-instead = In plaats daarvan zoeken naar ‘{ $query }’
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Alle
list-pick-none = Geen
list-pick-read = Gelezen
list-pick-unread = Ongelezen
list-pick-starred = Met ster
list-pick-unstarred = Zonder ster

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } gesprek is geselecteerd.
       *[other] Alle { $count } gesprekken zijn geselecteerd.
    }
   *[message] { $count ->
        [one] { $count } bericht is geselecteerd.
       *[other] Alle { $count } berichten zijn geselecteerd.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $count } gesprek in { $folder } is geselecteerd.
       *[other] Alle { $count } gesprekken in { $folder } zijn geselecteerd.
    }
   *[message] { $count ->
        [one] { $count } bericht in { $folder } is geselecteerd.
       *[other] Alle { $count } berichten in { $folder } zijn geselecteerd.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] { $count } gesprek op deze pagina is geselecteerd.
       *[other] Alle { $count } gesprekken op deze pagina zijn geselecteerd.
    }
   *[message] { $count ->
        [one] { $count } bericht op deze pagina is geselecteerd.
       *[other] Alle { $count } berichten op deze pagina zijn geselecteerd.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } gesprek selecteren
       *[other] Alle { $count } gesprekken selecteren
    }
   *[message] { $count ->
        [one] { $count } bericht selecteren
       *[other] Alle { $count } berichten selecteren
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $count } gesprek in { $folder } selecteren
       *[other] Alle { $count } gesprekken in { $folder } selecteren
    }
   *[message] { $count ->
        [one] { $count } bericht in { $folder } selecteren
       *[other] Alle { $count } berichten in { $folder } selecteren
    }
}
list-clear-selection = Selectie wissen

## Mail list: empty states

list-empty-search = Er zijn geen berichten die overeenkomen met je zoekopdracht.
list-empty-tab = Geen e-mail in { $tab }.
list-empty-tab-unknown = Geen e-mail op dit tabblad.
list-empty-folder = Geen berichten in { $folder }.
list-empty-folder-unknown = Geen berichten in deze map.
list-first-sync = Je e-mail ophalen…
list-first-sync-detail = Berichten verschijnen hier zodra ze binnenkomen.

## Mail list: lines

row-removed = Dit bericht is verwijderd.
row-starred = Met ster
row-not-starred = Zonder ster
row-important = Belangrijk. Klik om als niet belangrijk te markeren.
row-mark-important = Markeren als belangrijk
row-pinned = Bovenaan vastgezet
row-pin = Bovenaan vastzetten
row-unpin = Losmaken

## Mail list: More menu and right-click menu

menu-reply = Beantwoorden
menu-reply-all = Allen beantwoorden
menu-forward = Doorsturen
menu-archive = Archiveren
menu-delete = Verwijderen
menu-spam = Spam melden
menu-mark-read = Markeren als gelezen
menu-mark-unread = Markeren als ongelezen
menu-mark-all-read = Alles markeren als gelezen
menu-star = Ster toevoegen
menu-unstar = Ster verwijderen
menu-important = Markeren als belangrijk
menu-not-important = Markeren als niet belangrijk
menu-pin = Bovenaan vastzetten
menu-unpin = Losmaken
menu-print-all = Alles afdrukken
menu-new-window = Openen in nieuw venster
menu-move-to = Verplaatsen naar
menu-move-to-heading = Verplaatsen naar:
menu-find-from = E-mails van { $name } zoeken

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Gesprek gearchiveerd.
       *[other] { $count } gesprekken gearchiveerd.
    }
   *[message] { $count ->
        [one] Bericht gearchiveerd.
       *[other] { $count } berichten gearchiveerd.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Gesprek naar Prullenbak verplaatst.
       *[other] { $count } gesprekken naar Prullenbak verplaatst.
    }
   *[message] { $count ->
        [one] Bericht naar Prullenbak verplaatst.
       *[other] { $count } berichten naar Prullenbak verplaatst.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Gesprek verplaatst.
       *[other] { $count } gesprekken verplaatst.
    }
   *[message] { $count ->
        [one] Bericht verplaatst.
       *[other] { $count } berichten verplaatst.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Ster toegevoegd aan gesprek.
       *[other] Ster toegevoegd aan { $count } gesprekken.
    }
   *[message] { $count ->
        [one] Ster toegevoegd aan bericht.
       *[other] Ster toegevoegd aan { $count } berichten.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Ster verwijderd van gesprek.
       *[other] Ster verwijderd van { $count } gesprekken.
    }
   *[message] { $count ->
        [one] Ster verwijderd van bericht.
       *[other] Ster verwijderd van { $count } berichten.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Gesprek gemarkeerd als belangrijk.
       *[other] { $count } gesprekken gemarkeerd als belangrijk.
    }
   *[message] { $count ->
        [one] Bericht gemarkeerd als belangrijk.
       *[other] { $count } berichten gemarkeerd als belangrijk.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Gesprek gemarkeerd als niet belangrijk.
       *[other] { $count } gesprekken gemarkeerd als niet belangrijk.
    }
   *[message] { $count ->
        [one] Bericht gemarkeerd als niet belangrijk.
       *[other] { $count } berichten gemarkeerd als niet belangrijk.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Gesprek bovenaan vastgezet.
       *[other] { $count } gesprekken bovenaan vastgezet.
    }
   *[message] { $count ->
        [one] Bericht bovenaan vastgezet.
       *[other] { $count } berichten bovenaan vastgezet.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Gesprek losgemaakt.
       *[other] { $count } gesprekken losgemaakt.
    }
   *[message] { $count ->
        [one] Bericht losgemaakt.
       *[other] { $count } berichten losgemaakt.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Gesprek gemeld als spam.
       *[other] { $count } gesprekken gemeld als spam.
    }
   *[message] { $count ->
        [one] Bericht gemeld als spam.
       *[other] { $count } berichten gemeld als spam.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Gesprek definitief verwijderd.
       *[other] { $count } gesprekken definitief verwijderd.
    }
   *[message] { $count ->
        [one] Bericht definitief verwijderd.
       *[other] { $count } berichten definitief verwijderd.
    }
}
toast-undone = Actie ongedaan gemaakt.
toast-undo = Ongedaan maken
toast-no-spam-folder = Dit account heeft geen spammap.

## Reading pane: toolbar

reader-close = Sluiten
reader-back = Terug
reader-mark-unread = Markeren als ongelezen
reader-move-to = Verplaatsen naar
reader-more = Meer
reader-print-all = Alles afdrukken
reader-new-window = In nieuw venster
reader-position = { $position } van { $total }
reader-newer = Nieuwer
reader-older = Ouder

## Reading pane: the conversation

reader-removed = Dit gesprek is verwijderd.
reader-no-subject = (geen onderwerp)
reader-collapse-all = Alles samenvouwen
reader-expand-all = Alles uitvouwen
reader-unknown-sender = (onbekende afzender)
reader-date-ago = { $date } ({ $ago })
reader-me = mij
reader-to = aan { $names }
reader-starred = Met ster
reader-not-starred = Zonder ster
reader-too-long = Het bericht is te lang om volledig te tonen.
reader-encrypted-images = Afbeeldingen van internet worden nooit geladen in versleutelde e-mail.
reader-window-failed = Kan geen nieuw venster openen.

## Reading pane: message details (opened from "to me")

reader-details-from = van:
reader-details-to = aan:
reader-details-cc = cc:
reader-details-date = datum:
reader-details-subject = onderwerp:

## Reading pane: downloading a message

reader-downloading = Dit bericht downloaden van de server…
reader-download-failed = Kan dit bericht niet downloaden.
reader-try-again = Opnieuw proberen

## Reply row

reply-reply = Beantwoorden
reply-reply-all = Allen beantwoorden
reply-forward = Doorsturen

## Encrypted and signed mail

security-decrypting = Ontsleutelen…
security-checking = Handtekening controleren…
security-partly-encrypted = Slechts een deel van dit bericht is versleuteld. De rest is buiten de beveiliging toegevoegd en kan van iedereen afkomstig zijn.
security-partly-signed = Slechts een deel van dit bericht is ondertekend. De rest is buiten de beveiliging toegevoegd en kan van iedereen afkomstig zijn.
security-encrypted = Versleuteld bericht
security-encrypted-smime = Versleuteld bericht (S/MIME)
security-no-key = Kan dit bericht niet ontsleutelen: het is versleuteld voor een sleutel die je niet hebt.
security-cancelled = Ontsleutelen is geannuleerd.
security-damaged = Kan dit bericht niet ontsleutelen: de versleutelde gegevens zijn beschadigd of gewijzigd.
security-decrypt-unavailable = Kan dit bericht niet ontsleutelen: installeer { $tool } om versleutelde e-mail te lezen.
security-decrypt-failed = Kan dit bericht niet ontsleutelen: { $reason }
security-unknown-signer = een onbekende ondertekenaar
security-signed-verified = Ondertekend door { $signer } · geverifieerd
security-signed-not-sender = Ondertekend door { $signer }, die niet de afzender is
security-signed-untrusted = Ondertekend door { $signer }, met een sleutel die je als niet vertrouwd hebt gemarkeerd
security-signed-unverified = Ondertekend door { $signer } · de sleutel is niet geverifieerd
security-bad-signature = Ongeldige handtekening: dit bericht is na ondertekening gewijzigd, of de handtekening is vervalst.
security-signature-expired = Ondertekend door { $signer } · de handtekening is verlopen
security-key-expired = Ondertekend door { $signer } · de sleutel is inmiddels verlopen
security-key-revoked = Ondertekend door { $signer } met een sleutel die is ingetrokken
security-missing-key = Ondertekend met een sleutel die je niet hebt, dus kan niet worden gecontroleerd
security-missing-key-id = Ondertekend met een sleutel die je niet hebt ({ $key }), dus kan niet worden gecontroleerd
security-signature-unavailable = Ondertekend; installeer { $tool } om de handtekening te controleren
security-signature-error = De handtekening kan niet worden gecontroleerd.

## Remote images and pictures

remote-hidden = Afbeeldingen in dit bericht zijn verborgen.
remote-show = Afbeeldingen tonen
remote-always-show = Altijd tonen van deze afzender
remote-picture-use = Gebruiken
remote-picture-too-big = Kies een afbeelding van maximaal 8 MB.
remote-picture-type = Kies een PNG-, JPEG-, GIF-, WebP- of SVG-afbeelding.
remote-picture-read-failed = Kan de afbeelding niet lezen: { $error }
remote-picture-keep-failed = Kan de afbeelding niet bewaren: { $error }
remote-picture-remove-failed = Kan de afbeelding niet verwijderen: { $error }

## Attachments

attachment-count = { $count ->
    [one] Eén bijlage
   *[other] { $count } bijlagen
}
attachment-save = Opslaan
attachment-save-all = Alles opslaan
attachment-save-all-tooltip = Alle bijlagen opslaan in een map
attachment-save-here = Hier opslaan
attachment-not-downloaded = Dit bericht is niet gedownload.
attachment-not-found = Deze bijlage is niet gevonden in het bericht.
attachment-read-failed = Kan { $name } niet lezen
attachment-numbered = bijlage { $number }
attachment-saved-all = { $count ->
    [one] { $count } bestand opgeslagen in { $place }
   *[other] { $count } bestanden opgeslagen in { $place }
}
attachment-saved-some = { $total ->
    [one] { $saved } van { $total } bestand opgeslagen in { $place }. Kan { $failed } niet opslaan
   *[other] { $saved } van { $total } bestanden opgeslagen in { $place }. Kan { $failed } niet opslaan
}
attachment-saved-to = Opgeslagen in { $path }
attachment-save-failed = Kan { $name } niet opslaan: { $error }
attachment-open-failed = Kan { $name } niet openen: { $error }
attachment-risky = Dit bestand kan een programma uitvoeren, dus Katna opent het niet. Sla het in plaats daarvan op.
attachment-encrypted-open = Dit bestand is versleuteld ontvangen. Sla het op om het elders te openen.

## Printing

print-failed = Kan niet afdrukken: { $error }
print-no-font = er is geen lettertype gevonden
print-opened-as-pdf = Geopend als pdf om vanaf daar af te drukken.
print-not-downloaded = (Nog niet gedownload.)
print-encrypted = (Versleuteld. Open het in Katna Mail om de tekst af te drukken.)
print-to = Aan: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Open dit bericht om de bijlagen te lezen.
text-copy = Kopiëren
text-select-all = Alles selecteren

## Settings page: its tabs

settings-tab-general = Algemeen
settings-tab-inbox = Inbox
settings-tab-accounts = Accounts
settings-tab-subscriptions = Abonnementen
settings-tab-appearance = Weergave
settings-tab-shortcuts = Sneltoetsen
settings-tab-default-apps = Standaardapps
settings-tab-folders-rules = Mappen en regels
settings-tab-compose = Opstellen
settings-tab-mcp-server = MCP-server
settings-tab-feedback = Feedback
settings-tab-experimental = Experimenteel

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Bekijk de nieuwsbrieven en mailinglijsten die je ontvangt, en meld je met één klik af.
settings-tab-folders-rules-coming = Maak, hernoem, verplaats en verberg mappen en labels, en kies welke worden gesynchroniseerd. Met regels wordt nieuwe e-mail vanzelf gesorteerd, gelabeld, doorgestuurd of verwijderd, op afzender, onderwerp of woorden.
settings-tab-mcp-server-coming = Laat AI-assistenten op deze computer je e-mail doorzoeken, lezen en concepten opstellen, met jouw toestemming.

## Settings > General

settings-general-conversations = Gespreksweergave
settings-general-conversations-group = Antwoorden op dezelfde e-mail groeperen
settings-general-conversations-group-detail = Eén regel per gesprek in de lijst
settings-general-reading = Lezen
settings-general-newest-first = Nieuwste bericht eerst
settings-general-newest-first-detail = Een gesprek begint met het laatste antwoord
settings-general-full-headers = Volledige headers tonen
settings-general-full-headers-detail = Van, aan, cc, datum en onderwerp open bij elk bericht
settings-general-full-names = Volledige namen van ontvangers
settings-general-full-names-detail = ‘aan mij, Ada Lovelace’ in plaats van ‘aan mij, Ada’
settings-general-mark-read = Markeren als gelezen
settings-general-mark-read-now = Zodra het wordt geopend
settings-general-mark-read-1s = Nadat het 1 seconde open is
settings-general-mark-read-3s = Nadat het 3 seconden open is
settings-general-mark-read-never = Alleen als ik het als gelezen markeer
settings-general-reply-button = Antwoordknop
settings-general-reply-all = Iedereen beantwoorden
settings-general-reply-all-detail = De antwoordknop naast elk bericht beantwoordt iedereen, niet alleen de afzender
settings-general-remote-images = Afbeeldingen van internet
settings-general-remote-images-detail = Als de afbeeldingen van een bericht worden geladen, weet de afzender dat je het hebt geopend, wanneer en ongeveer waar. Staat dit uit, dan vraagt elk bericht het eerst, en je kunt de afbeeldingen van een afzender altijd tonen.
settings-general-remote-images-always = Afbeeldingen altijd tonen
settings-general-remote-images-always-detail = In elk bericht, niet alleen van afzenders die je vertrouwt
settings-general-sending = Verzenden
settings-general-sending-detail = Hoelang een verzonden bericht wacht, zodat je het nog kunt terughalen.
settings-general-offline = Offline e-mail
settings-general-offline-detail = Recente e-mail wordt volledig gedownload, om zonder verbinding te lezen. Oudere e-mail wordt gedownload als je die opent.
settings-general-offline-days = { $count ->
    [one] { $count } dag
   *[other] { $count } dagen
}
settings-general-offline-years = { $count ->
    [one] { $count } jaar
   *[other] { $count } jaar
}
settings-general-offline-all = Alle e-mail
settings-general-offline-note = Als je minder dagen kiest, blijft e-mail die al is gedownload bewaard. Er verandert niets op de server.
settings-general-notifications = Meldingen
settings-general-notifications-detail = Voor nieuwe e-mail in de inbox, ook als Katna Mail gesloten is.
settings-general-new-mail = Meldingen voor nieuwe e-mail
settings-general-new-mail-detail = Met Allen beantwoorden, Markeren als gelezen en Archiveren
settings-general-new-mail-sound = Geluid afspelen
settings-general-new-mail-sound-detail = Het geluid van de desktop voor nieuwe e-mail
settings-general-desktop = Desktop
settings-general-open-at-login = Katna Mail openen bij inloggen
settings-general-open-at-login-detail = E-mail wordt hoe dan ook gesynchroniseerd bij inloggen, zolang de service draait
settings-general-tray = Katna tonen in het systeemvak
settings-general-tray-detail = Met het aantal ongelezen berichten en een menu
settings-general-unread-badge = Aantal ongelezen op het taakbalkpictogram
settings-general-unread-badge-detail = Hoeveel berichten in de inbox ongelezen zijn

## Settings > Inbox

settings-inbox-tabs = Inbox-tabbladen
settings-inbox-tabs-detail = Sorteer de inbox in tabbladen, zoals de website van je e-mailprovider dat doet.
settings-inbox-tabs-show = Inbox-tabbladen tonen
settings-inbox-tabs-show-detail = Uit toont één lijst voor elk account
settings-inbox-no-accounts = Voeg een account toe om de tabbladen ervan te kiezen.
settings-inbox-tabs-automatic = Automatisch: { $tabs } ({ $provider })
settings-inbox-tabs-off = Geen tabbladen
settings-inbox-tabs-gmail = Primair, Reclame, Sociaal, Updates, Forums
settings-inbox-tabs-focused = Prioriteit en Overige
settings-inbox-tabs-zoho = Inbox, Nieuwsbrieven en Meldingen
settings-inbox-tabs-shown = Getoonde tabbladen. E-mail van een tabblad dat je uitzet, blijft in { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Leesvenster
settings-appearance-reading-pane-detail = Waar een geopend gesprek wordt getoond.
settings-appearance-pane-right = Rechts van de lijst
settings-appearance-pane-none = Geen splitsing
settings-appearance-density = Dichtheid
settings-appearance-density-default = Standaard
settings-appearance-density-compact = Compact
settings-appearance-scaling = Schaal
settings-appearance-scaling-detail = Maakt alles in Katna Mail groter of kleiner, bovenop de schaal van de desktop zelf: tekst, pictogrammen, witruimte en scheidingslijnen. E-mail die je verstuurt, houdt zijn eigen lettergrootte. Bij erg kleine formaten zijn pictogrammen lastig aan te klikken.
settings-appearance-theme = Thema
settings-appearance-theme-system = Zelfde als de desktop
settings-appearance-theme-light = Licht
settings-appearance-theme-dark = Donker
settings-appearance-desktop-colors = Desktopkleuren
settings-appearance-desktop-colors-use = Kleuren van de desktop gebruiken
settings-appearance-desktop-colors-use-detail = Het kleurenschema en de accentkleur van de desktop
settings-appearance-app-names = Appnamen
settings-appearance-app-names-show = Appnamen tonen
settings-appearance-app-names-show-detail = Namen onder de app-pictogrammen helemaal links
settings-appearance-sender-pictures = Afzenderafbeeldingen
settings-appearance-sender-pictures-show = Bedrijfslogo’s tonen
settings-appearance-sender-pictures-show-detail = Opgezocht op het domein van de afzender, nooit per bericht, en een week bewaard
settings-appearance-important = Belangrijk-markeringen
settings-appearance-important-show = Belangrijk-markeringen tonen
settings-appearance-important-show-detail = Naast elk bericht in de lijst
settings-appearance-message-width = Berichtbreedte
settings-appearance-message-width-limit = Breedte van berichten beperken
settings-appearance-message-width-limit-detail = Lange regels lezen makkelijker in een breed venster
settings-appearance-mail-colors = E-mailkleuren
settings-appearance-mail-colors-detail = De meeste e-mail is ontworpen voor een witte pagina. Met een donker thema worden de kleuren vervangen door donkere die goed leesbaar zijn; staat dit uit, dan houdt de e-mail de kleuren van de afzender op een lichte pagina.
settings-appearance-dark-mail = Ook donkere kleuren voor e-mail
settings-appearance-dark-mail-detail = Alleen als het thema donker is
settings-appearance-attachment-previews = Voorbeelden van bijlagen
settings-appearance-attachment-previews-show = Voorbeelden van bijlagen tonen
settings-appearance-attachment-previews-show-detail = Een kleine afbeelding van de inhoud van elk bestand op de kaart ervan

## Settings > Default apps

settings-default-apps-intro = Waar bijlagen worden geopend als je erop klikt. De viewer kan een bestand ook altijd in een andere app openen. De standaardapps van de desktop stel je in bij de eigen instellingen van de desktop.
settings-default-apps-pdf = Pdf-bestanden
settings-default-apps-pdf-detail = Pagina’s, met zoomen.
settings-default-apps-pictures = Afbeeldingen
settings-default-apps-pictures-detail = Foto’s (rechtop gedraaid), PNG, GIF, WebP, BMP, TIFF en SVG.
settings-default-apps-text = Tekstbestanden
settings-default-apps-text-detail = Platte tekst, logboeken, code en andere tekst.
settings-default-apps-sheets = Spreadsheets
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) en CSV.
settings-default-apps-documents = Documenten
settings-default-apps-documents-detail = Word (docx) en OpenDocument-tekst (odt).
settings-default-apps-katna = Viewer van Katna Mail
settings-default-apps-system = De standaardapp van de desktop
settings-default-apps-ask = Elke keer vragen welke app
settings-default-apps-after-saving = Na opslaan
settings-default-apps-show-folder = Opgeslagen bestanden tonen in hun map
settings-default-apps-show-folder-detail = Opent de bestandsbeheerder met de opgeslagen bijlagen geselecteerd

## Settings > Compose

settings-compose-send-from = Nieuwe berichten verzenden vanaf
settings-compose-send-from-detail = Antwoorden en doorgestuurde berichten gaan altijd uit vanaf het account waarin je bent.
settings-compose-send-from-current = Het account waarin je bent
settings-compose-send-on-replies = Verzenden bij antwoorden
settings-compose-send-on-replies-detail = Wat Verzenden doet bij een antwoord of doorgestuurd bericht. Het menu naast Verzenden biedt de andere keuze.
settings-compose-send-plain = Verzenden
settings-compose-send-archive = Verzenden en archiveren
settings-compose-signatures = Handtekeningen
settings-compose-signatures-detail = Toegevoegd onder je bericht, na een regel ‘--’. Kies een andere in het opstelvenster.
settings-compose-untitled = Naamloos
settings-compose-signature-name = Naam, zoals Werk
settings-compose-signature-first = Mijn handtekening
settings-compose-signature-numbered = Handtekening { $number }
settings-compose-signature-delete = Verwijderen
settings-compose-signature-deleted = Handtekening verwijderd
settings-compose-signature-new = Nieuwe maken
settings-compose-no-signatures = Nog geen handtekeningen.
settings-compose-no-signature = Geen handtekening
settings-compose-for-new-mail = Voor nieuwe e-mail
settings-compose-for-replies = Voor antwoorden en doorsturen
settings-compose-for-replies-detail = In een gesprek waarin je een bericht hebt ondertekend, begint een antwoord in plaats daarvan met die handtekening.
settings-compose-format = Opmaak
settings-compose-plain-text = Schrijven in platte tekst
settings-compose-plain-text-detail = Nieuwe e-mail begint zonder opmaak; het opstelvenster kan overschakelen
settings-compose-spelling = Spelling
settings-compose-spell-check = Spelling controleren tijdens het schrijven
settings-compose-spell-check-detail = Verkeerd gespelde woorden worden onderstreept, met suggesties via rechtsklikken
settings-compose-spell-desktop = Taal van de desktop ({ $language })
settings-compose-templates = Sjablonen
settings-compose-templates-detail = Sla e-mail op die je vaak schrijft, en begin daarmee een nieuwe e-mail of een antwoord.

## Settings > Shortcuts

settings-shortcuts-set = Sneltoetsenset
settings-shortcuts-set-detail = Begin met de toetsen van een e-mailapp die je kent. Cmd is hier Ctrl. Je eigen wijzigingen blijven boven op de set, en Standaardwaarden herstellen gaat terug naar de toetsen van de set.
settings-shortcuts-single = Sneltoetsen met één toets
settings-shortcuts-single-detail = Toetsen zonder Ctrl of Alt, zoals in webmail: e archiveert, j en k gaan verder en terug, / zoekt. Ze werken in de lijst en in het geopende gesprek, nooit tijdens het typen.
settings-shortcuts-single-use = Sneltoetsen met één toets gebruiken
settings-shortcuts-single-use-detail = Sneltoetsen met Ctrl werken altijd
settings-shortcuts-how = Klik op een toets om die te wijzigen, of op + om er een toe te voegen, en druk dan de nieuwe toetsen in. Esc annuleert.
settings-shortcuts-restore = Standaardwaarden herstellen
settings-shortcuts-no-key = Geen toets
settings-shortcuts-press = Druk op toetsen…
settings-shortcuts-then = { $keys } en dan…
settings-shortcuts-moved = { $keys } doet nu ‘{ $action }’ in plaats van ‘{ $previous }’.
settings-shortcuts-single-off = Sneltoetsen met één toets staan uit, dus deze toets werkt zodra ze aanstaan.
settings-shortcuts-restored = Elke sneltoets heeft weer de toetsen van de set.

## Settings search: the line under a result

settings-general-language-summary = Taal van de app, datums en getallen
settings-general-reading-summary = Nieuwste bericht eerst, volledige headers, volledige namen van ontvangers
settings-general-mark-read-summary = Wanneer een geopend gesprek als gelezen wordt gemarkeerd: meteen, na 1 of 3 seconden, of met de hand
settings-general-reply-button-summary = De antwoordknop naast elk bericht beantwoordt iedereen
settings-general-remote-images-summary = De afbeeldingen van elk bericht altijd tonen
settings-general-sending-summary = Verzenden ongedaan maken: hoelang een verzonden bericht wacht, zodat je het nog kunt terughalen
settings-general-offline-summary = Hoeveel dagen recente e-mail volledig worden gedownload, om zonder verbinding te lezen
settings-general-notifications-summary = Meldingen voor nieuwe e-mail en het geluid ervan
settings-general-desktop-summary = Katna Mail openen bij inloggen, het pictogram in het systeemvak en het aantal ongelezen op het taakbalkpictogram
settings-accounts-accounts-summary = Een account toevoegen of verwijderen, of de afbeelding ervan wijzigen
settings-appearance-density-summary = Standaard of compacte regels in de lijst
settings-appearance-scaling-summary = Alles groter of kleiner maken: tekst, pictogrammen, witruimte en scheidingslijnen
settings-appearance-theme-summary = Zelfde als de desktop, licht of donker
settings-appearance-sender-pictures-summary = Bedrijfslogo’s, opgezocht op het domein van de afzender
settings-appearance-important-summary = De Belangrijk-markering naast elk bericht in de lijst
settings-appearance-mail-colors-summary = Donkere kleuren voor HTML-e-mail in een donker thema, of de kleuren van de afzender
settings-appearance-attachment-previews-summary = Een kleine afbeelding van de inhoud van elke bijlage
settings-shortcuts-set-summary = Begin met de toetsen van Gmail, Inbox by Gmail, Apple Mail, Outlook of Thunderbird
settings-shortcuts-single-summary = Toetsen zonder Ctrl of Alt, zoals in webmail
settings-default-apps-pdf-summary = Waar pdf-bijlagen worden geopend
settings-default-apps-pictures-summary = Waar foto’s en afbeeldingen worden geopend
settings-default-apps-text-summary = Waar platte tekst, logboeken en code worden geopend
settings-default-apps-sheets-summary = Waar Excel-, OpenDocument- en CSV-bestanden worden geopend
settings-default-apps-documents-summary = Waar Word- en OpenDocument-tekst worden geopend
settings-default-apps-after-saving-summary = Opgeslagen bijlagen tonen in hun map
settings-compose-send-from-summary = Het account waarvandaan nieuwe e-mail uitgaat: het account waarin je bent, of altijd hetzelfde
settings-compose-send-on-replies-summary = Verzenden, of Verzenden en het gesprek archiveren, bij antwoorden en doorsturen
settings-compose-signatures-summary = Toegevoegd onder je bericht, na een regel ‘--’
settings-compose-for-new-mail-summary = De handtekening waarmee nieuwe e-mail begint
settings-compose-for-replies-summary = De handtekening waarmee antwoorden en doorgestuurde berichten beginnen
settings-compose-format-summary = Nieuwe e-mail schrijven in platte tekst
settings-compose-spelling-summary = Spelling controleren tijdens het schrijven, en de taal van het woordenboek
settings-compose-templates-summary = Binnenkort: sla e-mail op die je vaak schrijft, en begin daarmee een nieuwe e-mail of een antwoord
settings-feedback-crash-reports-summary = Crashrapporten op deze computer bewaren als Katna Mail of de achtergrondservice crasht
settings-feedback-saved-summary = De crashrapporten die op deze computer zijn bewaard bekijken, kopiëren of verwijderen
settings-feedback-help-improve-summary = Crashrapporten versturen om te helpen oplossen wat er misging; uit tenzij je het aanzet
settings-experimental-blur-summary = De desktop schijnt wazig door de bovenbalk heen, en menu’s zijn van matglas
settings-search-shortcut = Sneltoets
settings-search-tab = Tabblad van Instellingen
settings-search-none = Geen instellingen gevonden voor ‘{ $query }’.
settings-search-results = Instellingen die overeenkomen met ‘{ $query }’
## Quick settings (the panel that slides in from the right)

quick-title = Snelle instellingen
quick-see-all = Alle instellingen bekijken
quick-reading-pane = Leesvenster
quick-pane-right = Rechts van de lijst
quick-pane-none = Geen splitsing
quick-density = Dichtheid
quick-density-default = Standaard
quick-density-compact = Compact
quick-theme = Thema
quick-theme-system = Zelfde als de desktop
quick-theme-light = Licht
quick-theme-dark = Donker
quick-desktop-colors = Desktopkleuren
quick-desktop-colors-detail = Het kleurenschema en de accentkleur van de desktop
quick-app-names = Appnamen
quick-app-names-detail = Namen onder de app-pictogrammen helemaal links
quick-inbox-tabs = Inbox-tabbladen
quick-inbox-tabs-detail = De tabbladen van de e-mailprovider van elk account
quick-choose-tabs = Tabbladen kiezen
quick-choose-tabs-detail = Per account, in Instellingen
quick-sending = Verzenden
quick-undo-send = Verzenden ongedaan maken
quick-undo-send-off = Uit
quick-undo-send-seconds = { $seconds } s
quick-signatures = Handtekeningen
quick-signatures-none = Nog geen
quick-signatures-one = { $name }, standaard gebruikt
quick-signatures-many = { $count ->
    [one] { $count } handtekening; { $name } standaard
   *[other] { $count } handtekeningen; { $name } standaard
}
quick-signatures-no-default = { $count ->
    [one] { $count }, geen standaard
   *[other] { $count }, geen standaard
}
quick-signature-untitled = Naamloos
quick-threading = E-mailgesprekken
quick-conversation-view = Gespreksweergave
quick-conversation-view-detail = Antwoorden op dezelfde e-mail groeperen
quick-help = Help
quick-tour = Rondleiding volgen
quick-whats-new = Wat is er nieuw
quick-about = Over Katna

## Settings: opening at login

settings-open-at-login-failed = Kan openen bij inloggen niet wijzigen: { $error }

## Settings > Appearance > Scaling

scale-letter = A
scale-percent = { $percent }%
scale-reset = Terug naar { $percent }%

## Settings > Experimental > Look & Feel

look-intro = Functies die nog worden uitgeprobeerd. Ze kunnen veranderen of verdwijnen.
look-heading = Uiterlijk
look-window-frame = Vensterrand
look-window-frame-detail = Wie de titelbalk, de vensterknoppen, de hoeken en de schaduw tekent.
look-frame-native-kde = Systeemeigen: de rand van KDE, in je Plasma-thema
look-frame-native = Systeemeigen: de rand van de desktop
look-frame-katna = Katna: de bovenbalk wordt de titelbalk
look-frame-katna-note-named = Katna tekent afgeronde hoeken en een eigen schaduw. De rand volgt het { $desktop }-thema niet meer; vensterregels blijven gelden.
look-frame-katna-note = Katna tekent afgeronde hoeken en een eigen schaduw. De rand volgt het desktopthema niet meer; vensterregels blijven gelden.
look-frame-client-side = Je desktop laat de rand over aan elke app, dus Katna tekent al een eigen rand.
look-blurred-background = Wazige achtergrond
look-blurred-background-detail = De desktop schijnt wazig door de bovenbalk en de mappen heen, en menu’s en pop-ups zijn van matglas.
look-blur = Wat achter het venster zit vervagen
look-blur-detail = E-mail blijft op ondoorzichtige kaarten, zodat tekst goed leesbaar blijft
look-blur-off-kde = Het vervagingseffect van KDE staat uit. Zet Vervagen aan in Systeeminstellingen, Vensterbeheer, Bureaubladeffecten en open Katna Mail daarna opnieuw.
look-blur-none-gnome = GNOME vervaagt niet wat achter vensters zit.
look-blur-none-x11 = Je vensterbeheerder vervaagt niet wat achter vensters zit.
look-blur-none-wayland = Je compositor vervaagt niet wat achter vensters zit.

## Settings > User feedback (crash reports)

feedback-intro-sending = Nieuwe crashrapporten worden verstuurd om te helpen oplossen wat er misging. Verder verlaat niets deze computer.
feedback-intro-local = Katna verstuurt nergens iets naartoe. Crashrapporten blijven op deze computer, zodat je ze kunt bekijken of bij een bugmelding kunt voegen.
feedback-crash-reports = Crashrapporten
feedback-crash-reports-detail = Aangemaakt als Katna Mail of de achtergrondservice crasht.
feedback-save = Crashrapporten op deze computer bewaren
feedback-save-detail = Je persoonlijke map, gebruikers- en computernamen en e-mailadressen worden weggelaten
feedback-saved = Bewaarde crashrapporten
feedback-saved-detail = { $count ->
    [one] Het nieuwste rapport wordt bewaard.
   *[other] De nieuwste { $count } worden bewaard.
}
feedback-help-improve = Help Katna te verbeteren
feedback-help-improve-detail = Uit tenzij je het aanzet, en je kunt het hier altijd weer uitzetten.
feedback-send = Crashrapporten versturen
feedback-send-detail = Het bewaarde rapport, precies zoals je het hier kunt bekijken, gaat naar de crashtracker van Katna (Sentry, in de EU). Geen IP-adres, berichten of e-mailadressen
feedback-none-saved = Er zijn geen crashrapporten bewaard.
feedback-delete-all = Alles verwijderen
feedback-app-daemon = Achtergrondservice
feedback-report-sent = { $date } · Verstuurd
feedback-view = Bekijken
feedback-view-tooltip = Het rapport openen
feedback-copy-tooltip = Kopiëren om in een bugmelding te plakken
feedback-copied = Crashrapport gekopieerd.
feedback-deleted-all = Crashrapporten verwijderd.
feedback-read-failed = Kan het crashrapport niet lezen: { $error }
feedback-delete-failed = Kan het crashrapport niet verwijderen: { $error }
feedback-delete-all-failed = Kan de crashrapporten niet verwijderen: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _Bestand
desktop-menu-new-message = _Nieuw bericht
desktop-menu-quit = A_fsluiten
desktop-menu-edit = Be_werken
desktop-menu-undo = _Ongedaan maken
desktop-menu-select-all = _Alles selecteren
desktop-menu-select-none = _Niets selecteren
desktop-menu-find = _Zoeken…
desktop-menu-view = Beel_d
desktop-menu-folder-list = _Mappenlijst tonen
desktop-menu-refresh = _Vernieuwen
desktop-menu-go = _Ga
desktop-menu-inbox = _Inbox
desktop-menu-starred = _Met ster
desktop-menu-sent = _Verzonden
desktop-menu-drafts = _Concepten
desktop-menu-all-mail = _Alle berichten
desktop-menu-next = V_olgend gesprek
desktop-menu-previous = Vo_rig gesprek
desktop-menu-message = _Bericht
desktop-menu-open = _Openen
desktop-menu-reply = _Beantwoorden
desktop-menu-reply-all = _Allen beantwoorden
desktop-menu-forward = _Doorsturen
desktop-menu-archive = A_rchiveren
desktop-menu-delete = _Verwijderen
desktop-menu-spam = _Spam melden
desktop-menu-move-to = Ver_plaatsen naar…
desktop-menu-mark-read = Markeren als _gelezen
desktop-menu-mark-unread = Markeren als _ongelezen
desktop-menu-star = S_ter
desktop-menu-important = Markeren als bel_angrijk
desktop-menu-not-important = Markeren als _niet belangrijk
desktop-menu-settings = _Instellingen
desktop-menu-quick-settings = _Snelle instellingen
desktop-menu-configure = Katna Mail _instellen…
desktop-menu-help = _Help
desktop-menu-shortcuts = _Sneltoetsen
desktop-menu-whats-new = _Wat is er nieuw
desktop-menu-about = _Over Katna
## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Navigeren
shortcut-group-actions = Acties
shortcut-group-go-to = Ga naar
shortcut-group-app = Applicatie

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Volgend gesprek
shortcut-previous = Vorig gesprek
shortcut-down = Omlaag in de lijst
shortcut-up = Omhoog in de lijst
shortcut-first = Eerste in de lijst
shortcut-last = Laatste in de lijst
shortcut-page-down = Pagina omlaag in de lijst
shortcut-page-up = Pagina omhoog in de lijst
shortcut-open = Gesprek openen
shortcut-back = Terug naar de lijst
shortcut-scroll-down = Omlaag scrollen
shortcut-scroll-up = Omhoog scrollen
shortcut-scroll-page-down = Een pagina omlaag scrollen
shortcut-scroll-page-up = Een pagina omhoog scrollen
shortcut-compose = Opstellen
shortcut-reply = Beantwoorden
shortcut-reply-all = Allen beantwoorden
shortcut-forward = Doorsturen
shortcut-archive = Archiveren
shortcut-delete = Verwijderen
shortcut-spam = Spam melden
shortcut-move-to = Verplaatsen naar
shortcut-mark-read = Markeren als gelezen
shortcut-mark-unread = Markeren als ongelezen
shortcut-star = Ster toevoegen of verwijderen
shortcut-important = Markeren als belangrijk
shortcut-not-important = Markeren als niet belangrijk
shortcut-check = Gesprek aanvinken
shortcut-select-all = Alle gesprekken aanvinken
shortcut-select-none = Alle gesprekken uitvinken
shortcut-undo = Laatste actie ongedaan maken
shortcut-go-inbox = Inbox
shortcut-go-starred = Met ster
shortcut-go-sent = Verzonden
shortcut-go-drafts = Concepten
shortcut-go-all = Alle berichten
shortcut-search = Zoeken in e-mail
shortcut-navigation = Menu tonen of inklappen
shortcut-quick-settings = Snelle instellingen
shortcut-settings = Alle instellingen
shortcut-shortcuts = Sneltoetsen
shortcut-reload = Nieuwe e-mail ophalen
shortcut-quit = Afsluiten

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } en dan { $second }

## Settings > Accounts

accounts-folder-pane = Mappenvenster
accounts-folder-pane-detail = Van welke accounts het venster links de mappen toont.
accounts-shown-one = Eén account tegelijk; wissel via de accountkaart
accounts-shown-all = Alle accounts, na elkaar
accounts-row = Accounts
accounts-row-detail = Als je een account verwijdert, wordt de kopie van de e-mail die Katna op deze computer heeft verwijderd. De e-mail blijft op de server.
accounts-none = Nog geen accounts.
accounts-kind-imported = Geïmporteerd
accounts-picture-reset = Desktopafbeelding gebruiken
accounts-picture-change = Afbeelding wijzigen
accounts-remove = Verwijderen
accounts-delete-all-row = Alle gegevens verwijderen
accounts-delete-all-row-detail = Opnieuw beginnen, zoals bij een nieuwe installatie.
accounts-delete-all-about = Verwijdert van deze computer elk account, alle opgeslagen e-mail, contacten en agenda’s, de zoekindex, je instellingen en opgeslagen wachtwoorden. Er verandert niets op je mailservers.
accounts-delete-all-open = Alle Katna-gegevens verwijderen

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } is verwijderd uit Katna.
accounts-removed = { $address } is verwijderd uit Katna. De e-mail staat nog op de server.
accounts-all-deleted = Alle Katna-gegevens zijn van deze computer verwijderd.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } verwijderen?
accounts-remove-confirm = Account verwijderen
accounts-removing = Verwijderen…
accounts-remove-local-mail = { $folders ->
    [0] Alle e-mail die in dit account is geïmporteerd
    [one] Alle e-mail die in dit account is geïmporteerd, in de map ervan
   *[other] Alle e-mail die in dit account is geïmporteerd, in de { $folders } mappen ervan
}
accounts-remove-local-settings = De Katna-instellingen ervan
accounts-remove-mail = { $folders ->
    [0] Alle e-mail van dit account die Katna heeft opgeslagen
    [one] Alle e-mail van dit account die Katna heeft opgeslagen, in de map ervan
   *[other] Alle e-mail van dit account die Katna heeft opgeslagen, in de { $folders } mappen ervan
}
accounts-remove-outbox = De berichten ervan die in het postvak UIT wachten
accounts-remove-settings = Het opgeslagen wachtwoord en de Katna-instellingen ervan
accounts-delete-all-title = Alle Katna-gegevens verwijderen?
accounts-delete-all-confirm = Alles verwijderen
accounts-deleting = Verwijderen…
accounts-delete-all-accounts = Elk account, en alle e-mail en bijlagen die Katna heeft opgeslagen
accounts-delete-all-contacts = Contacten, agenda’s en de zoekindex
accounts-delete-all-settings = Alle instellingen, handtekeningen en sneltoetsen
accounts-delete-all-passwords = Elk opgeslagen wachtwoord
accounts-deleted-heading = Verwijderd van deze computer:
accounts-cannot-undo = Dit kan niet ongedaan worden gemaakt.
accounts-server-delete-all = Er verandert niets op je mailservers: je e-mail blijft daar, en als je een account opnieuw toevoegt, wordt die opnieuw gedownload. E-mail die uit bestanden is geïmporteerd, staat alleen in Katna; de bestanden blijven onaangeroerd.
accounts-server-local = Deze e-mail is uit bestanden geïmporteerd, dus Katna heeft de enige kopie. De bestanden waar die vandaan komt, blijven onaangeroerd; importeer ze opnieuw om de e-mail terug te krijgen.
accounts-server-remove = Er verandert niets op de mailserver: je e-mail blijft daar, en als je het account opnieuw toevoegt, wordt die opnieuw gedownload.
accounts-confirm-word = verwijderen
accounts-confirm-placeholder = Typ ‘{ accounts-confirm-word }’
accounts-confirm-prompt = Typ ‘{ accounts-confirm-word }’ om te bevestigen:
accounts-cancel = Annuleren
