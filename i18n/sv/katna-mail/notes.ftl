# Katna Mail, Swedish (Svenska): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Anteckningar
notes-view-reminders = Påminnelser
notes-view-archive = Arkiv
notes-view-trash = Papperskorgen
notes-edit-labels = Redigera etiketter
notes-search = Sök i anteckningar
notes-loading = Öppnar dina anteckningar…

## Board

notes-take-a-note = Skriv en anteckning…
notes-new-list = Ny lista
notes-new-note = Ny anteckning
notes-pinned = Fästa
notes-others = Övriga
notes-empty = Anteckningar du lägger till visas här
notes-archive-empty = Dina arkiverade anteckningar visas här
notes-trash-empty = Inga anteckningar i papperskorgen
notes-none-found = Inga matchande anteckningar
notes-label-empty = Inga anteckningar med den här etiketten än
notes-reminders-empty = Anteckningar med kommande påminnelser visas här
notes-trash-note = Anteckningar i papperskorgen raderas efter 7 dagar.
notes-empty-trash = Töm papperskorgen
notes-ticked = { $count ->
    [one] + { $count } markerat objekt
   *[other] + { $count } markerade objekt
}
notes-select = Markera anteckning
notes-selected = { $count ->
    [one] { $count } markerad
   *[other] { $count } markerade
}
notes-select-clear = Rensa markering

## A note's buttons

notes-pin = Fäst anteckning
notes-unpin = Lossa anteckning
notes-archive = Arkivera
notes-unarchive = Avarkivera
notes-delete = Radera anteckning
notes-restore = Återställ
notes-delete-forever = Radera permanent
notes-color = Bakgrundsfärg
notes-checkboxes = Visa eller dölj kryssrutor
notes-labels = Etiketter
notes-close = Stäng
notes-more = Mer
notes-make-copy = Gör en kopia
notes-remind = Påminn mig
notes-add-picture = Lägg till bild
notes-history = Versionshistorik
notes-ai = Hjälp mig skriva
notes-send-as-mail = Skicka som e-post
notes-save-markdown = Spara som Markdown
notes-save-pdf = Spara som PDF

## The open note

notes-title = Rubrik
notes-edited = Redigerad { $date }
notes-on-this-computer = På den här datorn
notes-where = Var anteckningen sparas
notes-untitled = Namnlös anteckning

## Pictures

notes-picture-choose = Lägg till bilder
notes-picture-remove = Ta bort bild
notes-picture-too-big = Bilder på upp till { $size } kan läggas i en anteckning
notes-picture-kind = Filen är inte en bild som Katna kan visa
notes-picture-unreadable = Det gick inte att läsa { $name }: { $error }

## Reminders

notes-remind-me = Påminn mig
notes-remind-off = Ta bort påminnelse
notes-remind-in-the-past = Välj en tid som inte redan har passerat
notes-remind-today = Idag, { $time }
notes-remind-tomorrow = I morgon, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = Påminnelse inställd till { $when }
notes-reminder-off = Påminnelsen togs bort

## Links between notes

notes-link-note = Länka en anteckning
notes-link-new = Ny anteckning ”{ $title }”
notes-linked-from = Länkad från
notes-link-gone = Den anteckningen finns inte längre här

## Version history

notes-versions = Versioner
notes-version-now = Nu
notes-version-here = Du, på den här datorn
notes-version-yesterday = I går, { $time }
notes-version-changes = { $count ->
    [one] { $count } ändring
   *[other] { $count } ändringar
}
notes-version-from = Från { $device }
notes-version-elsewhere = Från en annan enhet
notes-version-created = Skapad
notes-version-restore = Återställ den här versionen
notes-version-restored = Versionen återställdes
notes-history-none = Inga tidigare versioner än

## AI help

notes-ai-tidy = Snygga till texten
notes-ai-checklist = Gör om till en checklista
notes-ai-summarise = Sammanfatta
notes-ai-empty = Skriv något först
notes-ai-tidied = Texten snyggades till. Ctrl+Z ångrar.
notes-ai-listed = Gjordes om till en checklista. Ctrl+Z ångrar.
notes-ai-summarised = Sammanfattning tillagd överst

## Labels

notes-label-note = Etikettera anteckning
notes-label-name = Ange etikettnamn
notes-label-create = Skapa ”{ $name }”
notes-label-remove = Ta bort etikett
notes-label-delete = Radera etikett
notes-labels-none = Inga etiketter än. Lägg till en från etikettknappen på en anteckning.
notes-labels-done = Klar
notes-label-renamed = Etiketten fick namnet ”{ $name }”
notes-label-deleted = Etiketten ”{ $name }” raderades

## A note about a mail

notes-mail = E-post
notes-open-mail = Öppna e-postmeddelandet
notes-open-note = Öppna anteckningen

## Meeting notes

notes-meeting-take = Anteckna mötet
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = Deltagare: { $names }
notes-meeting-notes = Anteckningar
notes-meeting-actions = Åtgärder
notes-event = Händelse
notes-open-event = Öppna händelsen

## Formatting

notes-format = Formatering
notes-format-heading-1 = Rubrik 1
notes-format-heading-2 = Rubrik 2
notes-format-normal = Normal text
notes-format-bold = Fet
notes-format-italic = Kursiv
notes-format-underline = Understruken
notes-format-quote = Citat
notes-format-code = Kod
notes-format-divider = Avdelare
notes-format-clear = Rensa formatering

## Tasks

notes-make-task = Gör till uppgift

## Colors (tooltips)

notes-color-none = Ingen färg
notes-color-coral = Korall
notes-color-peach = Persika
notes-color-sand = Sand
notes-color-mint = Mynta
notes-color-sage = Salvia
notes-color-fog = Dimma
notes-color-storm = Storm
notes-color-dusk = Skymning
notes-color-blossom = Blomma
notes-color-clay = Lera
notes-color-chalk = Krita

## Messages at the foot of the window

notes-archived = Anteckningen har arkiverats
notes-unarchived = Anteckningen har avarkiverats
notes-trashed = Anteckningen har flyttats till papperskorgen
notes-restored = Anteckningen har återställts
notes-saved = Anteckningen sparades
notes-pinned-count = { $count ->
    [one] Anteckningen fästes
   *[other] { $count } anteckningar fästes
}
notes-unpinned-count = { $count ->
    [one] Anteckningen lossades
   *[other] { $count } anteckningar lossades
}
notes-colored-count = { $count ->
    [one] Färgen ändrades
   *[other] Färgen ändrades på { $count } anteckningar
}
notes-archived-count = { $count ->
    [one] Anteckningen arkiverades
   *[other] { $count } anteckningar arkiverades
}
notes-unarchived-count = { $count ->
    [one] Anteckningen avarkiverades
   *[other] { $count } anteckningar avarkiverades
}
notes-trashed-count = { $count ->
    [one] Anteckningen flyttades till papperskorgen
   *[other] { $count } anteckningar flyttades till papperskorgen
}
notes-restored-count = { $count ->
    [one] Anteckningen återställdes
   *[other] { $count } anteckningar återställdes
}
notes-copied-count = { $count ->
    [one] En kopia skapades
   *[other] { $count } kopior skapades
}
notes-empty-discarded = Den tomma anteckningen har tagits bort
notes-mail-gone = Det e-postmeddelandet finns inte längre
notes-deleted-forever = { $count ->
    [one] Anteckningen har raderats permanent
   *[other] { $count } anteckningar har raderats permanent
}
