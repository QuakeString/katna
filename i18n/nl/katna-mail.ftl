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
