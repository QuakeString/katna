# Katna Mail, Swedish (Svenska): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Ny uppgift
tasks-all = Alla uppgifter
tasks-today = I dag
tasks-upcoming = Kommande
tasks-starred = Stjärnmärkta
tasks-completed-view = Slutförda
tasks-new-list = Skapa ny lista
tasks-labels-heading = Etiketter
tasks-on-this-computer = På den här datorn
tasks-my-tasks = Mina uppgifter
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Logga in igen för att visa uppgifter
tasks-account-signed-in = Inloggad på { $address } igen. Hämtar dina uppgifter…
tasks-account-sign-in-refused = { $provider } släppte inte in Katna. Försök igen och ge åtkomst till dina uppgifter.
tasks-account-refused = Servern godtog inte lösenordet. Yahoo, iCloud, Zoho och andra kräver ett applösenord.
tasks-account-change-password = Ändra lösenord
tasks-account-change-password-tooltip = Skriv det nya lösenordet; Katna kontrollerar det med servern
tasks-account-not-enabled = Uppgiftsåtkomst för Katna är inte påslagen än.
tasks-account-failed = Det gick inte att läsa uppgiftslistorna.
# $reason is the server's own words, in English.
tasks-account-error = Det gick inte att läsa uppgiftslistorna: { $reason }
tasks-account-none = Inga uppgiftslistor hittades
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = Inga uppgiftslistor hittades: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } visar uppgifter bara för Katna när den är inloggad med { $provider }.
tasks-account-sign-in-with = Logga in med { $provider }
tasks-account-looking = Letar efter uppgiftslistor…
tasks-account-try-again = Försök igen
tasks-account-try-again-tooltip = Kontrollera det här kontots uppgifter igen nu
tasks-account-fixing = Arbetar på det…
tasks-list-name-placeholder = Listnamn

## Lists and tasks

tasks-loading = Läser dina uppgifter…
tasks-no-lists = Dina uppgiftslistor visas här.
tasks-search = Sök bland uppgifter
tasks-search-none = Inga uppgifter matchar din sökning.
tasks-add = Lägg till en uppgift
tasks-title-placeholder = Titel
tasks-add-step = Lägg till en deluppgift
tasks-empty = Inga uppgifter än. Lägg till en ovan.
tasks-starred-empty = Stjärnmärk en uppgift för att se den här.
tasks-label-empty = Inga öppna uppgifter med den här etiketten.
tasks-today-empty = Inget förfaller i dag.
tasks-completed-empty = Uppgifter du slutför visas här.
tasks-upcoming-add = Lägg till en uppgift för { $day }
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = Från e-post
tasks-from-note-quiet = Från anteckning
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Försenade
tasks-completed = { $count ->
    [one] Slutförda ({ $count })
   *[other] Slutförda ({ $count })
}
tasks-list-options = Listalternativ
tasks-sort-by = Sortera efter
tasks-sort-my-order = Min ordning
tasks-sort-date = Datum
tasks-sort-starred = Nyligen stjärnmärkta
tasks-sort-title = Titel
tasks-rename-list = Byt namn på listan
tasks-delete-list = Radera listan
tasks-mark-done = Markera som slutförd
tasks-mark-open = Markera som ej slutförd
tasks-star = Stjärnmärk
tasks-unstar = Ta bort stjärnmärkning
tasks-edit-title = Redigera titel
tasks-details = Information
tasks-delete = Radera
tasks-move-to = Flytta till { $list }
tasks-from-mail = E-post
tasks-open-mail = Öppna e-postmeddelandet
tasks-from-note = Anteckning
tasks-open-note = Öppna anteckningen
tasks-note-gone = Den anteckningen finns inte längre.
tasks-no-subject = (inget ämne)

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
    [one] { $count } markerad
   *[other] { $count } markerade
}
tasks-select-clear = Rensa markering
tasks-select-move = Flytta till lista
tasks-select-date = Ange datum
tasks-next-week = Nästa vecka

## The details dialog

tasks-notes-placeholder = Lägg till information
tasks-date = Datum
tasks-no-date = Inget datum
tasks-time-placeholder = Lägg till tid
tasks-repeat = Upprepa
tasks-repeat-never = Upprepas inte
tasks-repeat-daily = Dagligen
tasks-repeat-weekly = Varje vecka
tasks-repeat-monthly = Varje månad
tasks-repeat-yearly = Varje år
tasks-repeat-other = Anpassad
tasks-remind = Påminn mig
tasks-remind-off = Påminn inte
tasks-remind-on-time = När det är dags
tasks-remind-morning = Samma dag, { $time }
tasks-remind-hour-before = En timme före
tasks-remind-day-before = Dagen före
tasks-label-add = Lägg till etikett
tasks-label-task = Etikettera uppgift
tasks-files-attach = Bifoga filer
tasks-files-pick = Bifoga
tasks-file-open = Öppna
tasks-file-remove = Ta bort fil
tasks-file-here = Bara på den här datorn
tasks-cancel = Avbryt
tasks-save = Spara
tasks-not-a-time = ”{ $text }” är ingen tid, till exempel { $example }.

## Due days

tasks-due-today = I dag
tasks-due-tomorrow = I morgon
tasks-due-yesterday = I går
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Uppgiften är slutförd
tasks-toast-next = Klart. Nästa är den { $date }
tasks-toast-deleted = Uppgiften har raderats
tasks-files-added = { $count ->
    [one] Filen har bifogats
   *[other] { $count } filer har bifogats
}
tasks-file-removed = ”{ $name }” har tagits bort
tasks-files-left-out = Inte bifogade: { $names }. En uppgift tar filer upp till { $limit }, inte mappar.
tasks-file-missing = Filen finns inte här längre.
tasks-toast-added = { $count ->
    [one] Har lagts till i Uppgifter
   *[other] { $count } uppgifter har lagts till
}
tasks-mail-gone = Det e-postmeddelandet finns inte längre.
tasks-toast-list-deleted = Listan har raderats
tasks-toast-moved = Flyttad till { $list }
# A task dragged to another place in its own list.
tasks-toast-placed = Uppgiften har flyttats
tasks-toast-rescheduled = Uppgift omplanerad
tasks-toast-rescheduled-several = { $count ->
    [one] Uppgiften har flyttats
   *[other] { $count } uppgifter har flyttats
}
tasks-toast-done-several = { $count ->
    [one] Uppgiften har slutförts
   *[other] { $count } uppgifter har slutförts
}
tasks-toast-open-several = { $count ->
    [one] Uppgiften har markerats som inte slutförd
   *[other] { $count } uppgifter har markerats som inte slutförda
}
tasks-toast-starred = { $count ->
    [one] Uppgiften har stjärnmärkts
   *[other] { $count } uppgifter har stjärnmärkts
}
tasks-toast-unstarred = { $count ->
    [one] Stjärnan har tagits bort
   *[other] Stjärnorna har tagits bort från { $count } uppgifter
}
tasks-toast-deleted-several = { $count ->
    [one] Uppgiften har raderats
   *[other] { $count } uppgifter har raderats
}
