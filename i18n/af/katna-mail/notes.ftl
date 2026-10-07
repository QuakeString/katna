# Katna Mail, Afrikaans (Afrikaans): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Notas
notes-view-reminders = Herinneringe
notes-view-archive = Argief
notes-view-trash = Asblik
notes-edit-labels = Wysig etikette
notes-search = Soek notas
notes-loading = Jou notas word oopgemaak…

## Board

notes-take-a-note = Neem ’n nota…
notes-new-list = Nuwe lys
notes-new-note = Nuwe nota
notes-pinned = Vasgespeld
notes-others = Ander
notes-empty = Notas wat jy byvoeg, verskyn hier
notes-archive-empty = Jou geargiveerde notas verskyn hier
notes-trash-empty = Geen notas in die asblik nie
notes-none-found = Geen passende notas nie
notes-label-empty = Nog geen notas met hierdie etiket nie
notes-reminders-empty = Notas met komende herinneringe verskyn hier
notes-trash-note = Notas in die asblik word ná 7 dae uitgevee.
notes-empty-trash = Maak asblik leeg
notes-ticked = { $count ->
    [one] + { $count } gemerkte item
   *[other] + { $count } gemerkte items
}
notes-select = Kies nota
notes-selected = { $count ->
    [one] { $count } gekies
   *[other] { $count } gekies
}
notes-select-clear = Maak keuse skoon

## A note's buttons

notes-pin = Speld nota vas
notes-unpin = Maak nota los
notes-archive = Argiveer
notes-unarchive = Haal uit argief
notes-delete = Vee nota uit
notes-restore = Herstel
notes-delete-forever = Vee permanent uit
notes-color = Agtergrondkleur
notes-checkboxes = Wys of versteek merkblokkies
notes-labels = Etikette
notes-close = Maak toe
notes-more = Meer
notes-make-copy = Maak 'n kopie
notes-remind = Herinner my
notes-add-picture = Voeg prent by
notes-history = Weergawegeskiedenis
notes-ai = Help my skryf
notes-send-as-mail = Stuur as e-pos
notes-save-markdown = Stoor as Markdown
notes-save-pdf = Stoor as PDF

## The open note

notes-title = Titel
notes-edited = Gewysig: { $date }
notes-on-this-computer = Op hierdie rekenaar
notes-where = Waar hierdie nota gehou word
notes-untitled = Naamlose nota

## Pictures

notes-picture-choose = Voeg prente by
notes-picture-remove = Verwyder prent
notes-picture-too-big = Prente tot { $size } kan in 'n nota gaan
notes-picture-kind = Daardie lêer is nie 'n prent wat Katna kan wys nie
notes-picture-unreadable = Kon nie { $name } lees nie: { $error }

## Reminders

notes-remind-me = Herinner my
notes-remind-off = Verwyder herinnering
notes-remind-in-the-past = Kies 'n tyd wat nog nie verby is nie
notes-remind-today = Vandag, { $time }
notes-remind-tomorrow = Môre, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = Herinnering gestel vir { $when }
notes-reminder-off = Herinnering verwyder

## Links between notes

notes-link-note = Skakel 'n nota
notes-link-new = Nuwe nota "{ $title }"
notes-linked-from = Geskakel vanaf
notes-link-gone = Daardie nota is nie meer hier nie

## Version history

notes-versions = Weergawes
notes-version-now = Nou
notes-version-here = Jy, op hierdie rekenaar
notes-version-yesterday = Gister, { $time }
notes-version-changes = { $count ->
    [one] { $count } verandering
   *[other] { $count } veranderinge
}
notes-version-from = Van { $device }
notes-version-elsewhere = Van 'n ander toestel
notes-version-created = Geskep
notes-version-restore = Herstel hierdie weergawe
notes-version-restored = Weergawe herstel
notes-history-none = Nog geen vroeëre weergawes nie

## AI help

notes-ai-tidy = Maak die teks netjies
notes-ai-checklist = Maak dit 'n kontrolelys
notes-ai-summarise = Som op
notes-ai-empty = Skryf eers iets
notes-ai-tidied = Teks netjies gemaak. Ctrl+Z sit dit terug.
notes-ai-listed = 'n Kontrolelys gemaak. Ctrl+Z sit dit terug.
notes-ai-summarised = Opsomming bo bygevoeg

## Labels

notes-label-note = Plak etiket op nota
notes-label-name = Voer etiketnaam in
notes-label-create = Skep “{ $name }”
notes-label-remove = Verwyder etiket
notes-label-delete = Vee etiket uit
notes-labels-none = Nog geen etikette nie. Voeg een by vanaf 'n nota se etiketknoppie.
notes-labels-done = Klaar
notes-label-renamed = Etiket hernoem na “{ $name }”
notes-label-deleted = Etiket “{ $name }” uitgevee

## A note about a mail

notes-mail = E-pos
notes-open-mail = Maak die e-pos oop
notes-open-note = Maak die nota oop

## Meeting notes

notes-meeting-take = Neem vergadernotas
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = Genooide: { $names }
notes-meeting-notes = Notas
notes-meeting-actions = Aksiepunte
notes-event = Geleentheid
notes-open-event = Maak die geleentheid oop

## Formatting

notes-format = Formatering
notes-format-heading-1 = Opskrif 1
notes-format-heading-2 = Opskrif 2
notes-format-normal = Normale teks
notes-format-bold = Vetdruk
notes-format-italic = Kursief
notes-format-underline = Onderstreep
notes-format-quote = Aanhaling
notes-format-code = Kode
notes-format-divider = Skeidslyn
notes-format-clear = Vee formatering uit

## Tasks

notes-make-task = Maak dit 'n taak

## Colors (tooltips)

notes-color-none = Geen kleur
notes-color-coral = Koraal
notes-color-peach = Perske
notes-color-sand = Sand
notes-color-mint = Kruisment
notes-color-sage = Salie
notes-color-fog = Newel
notes-color-storm = Storm
notes-color-dusk = Skemer
notes-color-blossom = Blom
notes-color-clay = Klei
notes-color-chalk = Kryt

## Messages at the foot of the window

notes-archived = Nota geargiveer
notes-unarchived = Nota uit argief gehaal
notes-trashed = Nota na die asblik geskuif
notes-restored = Nota herstel
notes-saved = Nota gestoor
notes-pinned-count = { $count ->
    [one] Nota vasgespeld
   *[other] { $count } notas vasgespeld
}
notes-unpinned-count = { $count ->
    [one] Nota losgemaak
   *[other] { $count } notas losgemaak
}
notes-colored-count = { $count ->
    [one] Kleur verander
   *[other] Kleur verander op { $count } notas
}
notes-archived-count = { $count ->
    [one] Nota geargiveer
   *[other] { $count } notas geargiveer
}
notes-unarchived-count = { $count ->
    [one] Nota uit argief gehaal
   *[other] { $count } notas uit argief gehaal
}
notes-trashed-count = { $count ->
    [one] Nota na die Asblik geskuif
   *[other] { $count } notas na die Asblik geskuif
}
notes-restored-count = { $count ->
    [one] Nota herstel
   *[other] { $count } notas herstel
}
notes-copied-count = { $count ->
    [one] Kopie gemaak
   *[other] { $count } kopieë gemaak
}
notes-empty-discarded = Leë nota weggegooi
notes-mail-gone = Daardie e-pos is nie meer hier nie
notes-deleted-forever = { $count ->
    [one] Nota permanent uitgevee
   *[other] { $count } notas permanent uitgevee
}
