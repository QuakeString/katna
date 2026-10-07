# Katna Mail, Dutch (Nederlands): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Notities
notes-view-reminders = Herinneringen
notes-view-archive = Archief
notes-view-trash = Prullenbak
notes-edit-labels = Labels bewerken
notes-search = Notities zoeken
notes-loading = Je notities worden geopend…

## Board

notes-take-a-note = Notitie maken…
notes-new-list = Nieuwe lijst
notes-new-note = Nieuwe notitie
notes-pinned = Vastgemaakt
notes-others = Overig
notes-empty = Notities die je toevoegt, worden hier weergegeven
notes-archive-empty = Je gearchiveerde notities worden hier weergegeven
notes-trash-empty = Geen notities in de prullenbak
notes-none-found = Geen overeenkomende notities
notes-label-empty = Nog geen notities met dit label
notes-reminders-empty = Notities met komende herinneringen worden hier weergegeven
notes-trash-note = Notities in de prullenbak worden na 7 dagen verwijderd.
notes-empty-trash = Prullenbak leegmaken
notes-ticked = { $count ->
    [one] + { $count } aangevinkt item
   *[other] + { $count } aangevinkte items
}
notes-select = Notitie selecteren
notes-selected = { $count ->
    [one] { $count } geselecteerd
   *[other] { $count } geselecteerd
}
notes-select-clear = Selectie wissen

## A note's buttons

notes-pin = Notitie vastmaken
notes-unpin = Notitie losmaken
notes-archive = Archiveren
notes-unarchive = Uit archief halen
notes-delete = Notitie verwijderen
notes-restore = Herstellen
notes-delete-forever = Definitief verwijderen
notes-color = Achtergrondkleur
notes-checkboxes = Selectievakjes tonen of verbergen
notes-labels = Labels
notes-close = Sluiten
notes-more = Meer
notes-make-copy = Kopie maken
notes-remind = Herinner mij
notes-add-picture = Afbeelding toevoegen
notes-history = Versiegeschiedenis
notes-ai = Help me schrijven
notes-send-as-mail = Versturen als e-mail
notes-save-markdown = Opslaan als Markdown
notes-save-pdf = Opslaan als pdf

## The open note

notes-title = Titel
notes-edited = Bewerkt: { $date }
notes-on-this-computer = Op deze computer
notes-where = Waar deze notitie wordt bewaard
notes-untitled = Naamloze notitie

## Pictures

notes-picture-choose = Afbeeldingen toevoegen
notes-picture-remove = Afbeelding verwijderen
notes-picture-too-big = Afbeeldingen tot { $size } kunnen in een notitie
notes-picture-kind = Dat bestand is geen afbeelding die Katna kan tonen
notes-picture-unreadable = Kan { $name } niet lezen: { $error }

## Reminders

notes-remind-me = Herinner mij
notes-remind-off = Herinnering verwijderen
notes-remind-in-the-past = Kies een tijd die nog niet voorbij is
notes-remind-today = Vandaag, { $time }
notes-remind-tomorrow = Morgen, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = Herinnering ingesteld voor { $when }
notes-reminder-off = Herinnering verwijderd

## Links between notes

notes-link-note = Notitie koppelen
notes-link-new = Nieuwe notitie ‘{ $title }’
notes-linked-from = Gekoppeld vanuit
notes-link-gone = Die notitie is er niet meer

## Version history

notes-versions = Versies
notes-version-now = Nu
notes-version-here = Jij, op deze computer
notes-version-yesterday = Gisteren, { $time }
notes-version-changes = { $count ->
    [one] { $count } wijziging
   *[other] { $count } wijzigingen
}
notes-version-from = Vanaf { $device }
notes-version-elsewhere = Vanaf een ander apparaat
notes-version-created = Gemaakt
notes-version-restore = Deze versie herstellen
notes-version-restored = Versie hersteld
notes-history-none = Nog geen eerdere versies

## AI help

notes-ai-tidy = De tekst opschonen
notes-ai-checklist = Er een checklist van maken
notes-ai-summarise = Samenvatten
notes-ai-empty = Schrijf eerst iets
notes-ai-tidied = Tekst opgeschoond. Ctrl+Z zet hem terug.
notes-ai-listed = Er is een checklist van gemaakt. Ctrl+Z zet het terug.
notes-ai-summarised = Samenvatting bovenaan toegevoegd

## Labels

notes-label-note = Label aan notitie toevoegen
notes-label-name = Labelnaam invoeren
notes-label-create = ‘{ $name }’ maken
notes-label-remove = Label verwijderen
notes-label-delete = Label verwijderen
notes-labels-none = Nog geen labels. Voeg er een toe via de labelknop van een notitie.
notes-labels-done = Klaar
notes-label-renamed = Label hernoemd naar ‘{ $name }’
notes-label-deleted = Label ‘{ $name }’ verwijderd

## A note about a mail

notes-mail = E-mail
notes-open-mail = De e-mail openen
notes-open-note = De notitie openen

## Meeting notes

notes-meeting-take = Vergadernotities maken
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = Deelnemers: { $names }
notes-meeting-notes = Notities
notes-meeting-actions = Actiepunten
notes-event = Afspraak
notes-open-event = De afspraak openen

## Formatting

notes-format = Opmaak
notes-format-heading-1 = Kop 1
notes-format-heading-2 = Kop 2
notes-format-normal = Normale tekst
notes-format-bold = Vet
notes-format-italic = Cursief
notes-format-underline = Onderstrepen
notes-format-quote = Citaat
notes-format-code = Code
notes-format-divider = Scheidingslijn
notes-format-clear = Opmaak wissen

## Tasks

notes-make-task = Er een taak van maken

## Colors (tooltips)

notes-color-none = Geen kleur
notes-color-coral = Koraal
notes-color-peach = Perzik
notes-color-sand = Zand
notes-color-mint = Munt
notes-color-sage = Salie
notes-color-fog = Mist
notes-color-storm = Storm
notes-color-dusk = Schemering
notes-color-blossom = Bloesem
notes-color-clay = Klei
notes-color-chalk = Krijt

## Messages at the foot of the window

notes-archived = Notitie gearchiveerd
notes-unarchived = Notitie uit archief gehaald
notes-trashed = Notitie naar de prullenbak verplaatst
notes-restored = Notitie hersteld
notes-saved = Notitie opgeslagen
notes-pinned-count = { $count ->
    [one] Notitie vastgemaakt
   *[other] { $count } notities vastgemaakt
}
notes-unpinned-count = { $count ->
    [one] Notitie losgemaakt
   *[other] { $count } notities losgemaakt
}
notes-colored-count = { $count ->
    [one] Kleur gewijzigd
   *[other] Kleur gewijzigd van { $count } notities
}
notes-archived-count = { $count ->
    [one] Notitie gearchiveerd
   *[other] { $count } notities gearchiveerd
}
notes-unarchived-count = { $count ->
    [one] Notitie uit archief gehaald
   *[other] { $count } notities uit archief gehaald
}
notes-trashed-count = { $count ->
    [one] Notitie naar de prullenbak verplaatst
   *[other] { $count } notities naar de prullenbak verplaatst
}
notes-restored-count = { $count ->
    [one] Notitie hersteld
   *[other] { $count } notities hersteld
}
notes-copied-count = { $count ->
    [one] Kopie gemaakt
   *[other] { $count } kopieën gemaakt
}
notes-empty-discarded = Lege notitie verwijderd
notes-mail-gone = Die e-mail is er niet meer
notes-deleted-forever = { $count ->
    [one] Notitie definitief verwijderd
   *[other] { $count } notities definitief verwijderd
}
