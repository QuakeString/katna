# Katna Mail, Afrikaans (Afrikaans): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Skep
tasks-all = Alle take
tasks-today = Vandag
tasks-starred = Gester
tasks-new-list = Skep nuwe lys
tasks-on-this-computer = Op hierdie rekenaar
tasks-my-tasks = My take
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Meld weer aan om take te wys
tasks-account-signed-in = Weer aangemeld by { $address }. Haal tans jou take…
tasks-account-sign-in-refused = { $provider } het Katna nie ingelaat nie. Probeer weer, en gee toegang tot jou take.
tasks-account-refused = Die bediener het die wagwoord nie aanvaar nie. Yahoo, iCloud, Zoho en ander het 'n programwagwoord nodig.
tasks-account-change-password = Verander wagwoord
tasks-account-change-password-tooltip = Maak Instellings > Rekeninge oop
tasks-account-not-enabled = Taaktoegang vir Katna is nog nie aangeskakel nie.
tasks-account-failed = Die takelyste kon nie gelees word nie.
# $reason is the server's own words, in English.
tasks-account-error = Die takelyste kon nie gelees word nie: { $reason }
tasks-account-none = Geen takelyste gevind nie
tasks-account-looking = Soek tans takelyste…
tasks-account-try-again = Probeer weer
tasks-account-try-again-tooltip = Kyk nou weer na hierdie rekening se take
tasks-account-fixing = Besig daarmee…
tasks-list-name-placeholder = Lysnaam

## Lists and tasks

tasks-loading = Lees tans jou take…
tasks-no-lists = Jou takelyste verskyn hier.
tasks-search = Soek take
tasks-search-none = Geen take pas by jou soektog nie.
tasks-add = Voeg ’n taak by
tasks-title-placeholder = Titel
tasks-add-step = Voeg ’n subtaak by
tasks-empty = Nog geen take nie. Voeg een hierbo by.
tasks-starred-empty = Ster ’n taak om dit hier te sien.
tasks-today-empty = Niks is vandag verskuldig.
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Agterstallig
tasks-completed = { $count ->
    [one] Voltooi ({ $count })
   *[other] Voltooi ({ $count })
}
tasks-list-options = Lysopsies
tasks-rename-list = Hernoem lys
tasks-delete-list = Vee lys uit
tasks-mark-done = Merk as voltooi
tasks-mark-open = Merk as onvoltooid
tasks-star = Ster
tasks-unstar = Verwyder ster
tasks-edit-title = Wysig titel
tasks-details = Besonderhede
tasks-delete = Vee uit
tasks-move-to = Skuif na { $list }
tasks-from-mail = E-pos
tasks-open-mail = Maak die e-pos oop
tasks-from-note = Nota
tasks-open-note = Maak die nota oop
tasks-note-gone = Daardie nota is nie meer hier nie.
tasks-no-subject = (geen onderwerp)

## The details dialog

tasks-notes-placeholder = Voeg besonderhede by
tasks-date = Datum
tasks-no-date = Geen datum
tasks-time-placeholder = Voeg tyd by
tasks-repeat = Herhaal
tasks-repeat-never = Herhaal nie
tasks-repeat-daily = Daagliks
tasks-repeat-weekly = Weekliks
tasks-repeat-monthly = Maandeliks
tasks-repeat-yearly = Jaarliks
tasks-repeat-other = Pasgemaak
tasks-remind = Herinner my
tasks-remind-off = Moenie herinner nie
tasks-remind-on-time = Op die tydstip
tasks-remind-morning = Op die dag, { $time }
tasks-remind-hour-before = 'n Uur vooraf
tasks-remind-day-before = Die dag vooraf
tasks-cancel = Kanselleer
tasks-save = Stoor
tasks-not-a-time = “{ $text }” is nie ’n tyd nie, byvoorbeeld { $example }.

## Due days

tasks-due-today = Vandag
tasks-due-tomorrow = Môre
tasks-due-yesterday = Gister
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Taak voltooi
tasks-toast-next = Klaar. Volgende een op { $date }
tasks-toast-deleted = Taak uitgevee
tasks-toast-added = { $count ->
    [one] By Take gevoeg
   *[other] { $count } take bygevoeg
}
tasks-mail-gone = Daardie e-pos is nie meer hier nie.
tasks-toast-list-deleted = Lys uitgevee
tasks-toast-moved = Geskuif na { $list }
tasks-toast-rescheduled = Taak herskeduleer
