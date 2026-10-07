# Katna Mail, Hausa (Hausa): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Sabon aiki
tasks-all = Duk ayyuka
tasks-today = Yau
tasks-upcoming = Masu zuwa
tasks-starred = Masu tauraro
tasks-completed-view = An kammala
tasks-new-list = Ƙirƙiri sabon jeri
tasks-labels-heading = Lakabobi
tasks-on-this-computer = A kan wannan kwamfuta
tasks-my-tasks = Ayyukana
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Sake shiga don nuna ayyuka
tasks-account-signed-in = An sake shiga { $address }. Ana samo ayyukanku…
tasks-account-sign-in-refused = { $provider } bai bar Katna ya shiga ba. Ku sake gwadawa, kuma ku ba da izinin shiga ayyukanku.
tasks-account-refused = Sabar ba ta karɓi kalmar sirrin ba. Yahoo, iCloud, Zoho da wasu suna buƙatar kalmar sirrin manhaja.
tasks-account-change-password = Canza kalmar sirri
tasks-account-change-password-tooltip = Rubuta sabuwar kalmar sirri; Katna za ta duba ta da sabar
tasks-account-not-enabled = Ba a kunna damar shiga ayyuka don Katna ba tukuna.
tasks-account-failed = Ba a iya karanta jerin ayyukan ba.
# $reason is the server's own words, in English.
tasks-account-error = Ba a iya karanta jerin ayyukan ba: { $reason }
tasks-account-none = Ba a sami jerin ayyuka ba
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = Ba a sami jerin ayyuka ba: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } yana nuna ayyuka ga Katna ne kawai idan ya shiga da { $provider }.
tasks-account-sign-in-with = Shiga da { $provider }
tasks-account-looking = Ana neman jerin ayyuka…
tasks-account-try-again = Sake gwadawa
tasks-account-try-again-tooltip = Sake duba ayyukan wannan asusun yanzu
tasks-account-fixing = Ana aiki a kai…
tasks-list-name-placeholder = Sunan jeri

## Lists and tasks

tasks-loading = Ana karanta ayyukanku…
tasks-no-lists = Jerin ayyukanku za su bayyana a nan.
tasks-search = Bincika ayyuka
tasks-search-none = Babu ayyukan da suka dace da bincikenka.
tasks-add = Ƙara aiki
tasks-title-placeholder = Take
tasks-add-step = Ƙara ƙaramin aiki
tasks-empty = Babu ayyuka tukuna. Ƙara ɗaya a sama.
tasks-starred-empty = Sanya tauraro a kan aiki don ganinsa a nan.
tasks-label-empty = Babu ayyukan da ba a kammala ba masu wannan lakabi.
tasks-today-empty = Babu abin da ya kamata a yi yau.
tasks-completed-empty = Ayyukan da kuka kammala suna bayyana a nan.
tasks-upcoming-add = Ƙara aiki don { $day }
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = Daga wasiƙa
tasks-from-note-quiet = Daga bayani
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Waɗanda suka wuce lokaci
tasks-completed = { $count ->
    [one] An kammala ({ $count })
   *[other] An kammala ({ $count })
}
tasks-list-options = Zaɓuɓɓukan jeri
tasks-sort-by = Tsara bisa
tasks-sort-my-order = Tsarina
tasks-sort-date = Kwanan wata
tasks-sort-starred = Masu tauraro na kwanan nan
tasks-sort-title = Take
tasks-rename-list = Sake wa jeri suna
tasks-delete-list = Share jeri
tasks-mark-done = Yi alamar an kammala
tasks-mark-open = Yi alamar ba a kammala ba
tasks-star = Sanya tauraro
tasks-unstar = Cire tauraro
tasks-edit-title = Gyara take
tasks-details = Bayanai
tasks-delete = Share
tasks-move-to = Mayar zuwa { $list }
tasks-from-mail = Wasiƙu
tasks-open-mail = Buɗe wasiƙar
tasks-from-note = Bayani
tasks-open-note = Buɗe bayanin
tasks-note-gone = Wannan bayanin ba ya nan kuma.
tasks-no-subject = (babu jigo)

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
    [one] An zaɓi { $count }
   *[other] An zaɓi { $count }
}
tasks-select-clear = Share zaɓi
tasks-select-move = Matsar zuwa jeri
tasks-select-date = Saita kwanan wata
tasks-next-week = Mako mai zuwa

## The details dialog

tasks-notes-placeholder = Ƙara bayanai
tasks-date = Kwanan wata
tasks-no-date = Babu kwanan wata
tasks-time-placeholder = Ƙara lokaci
tasks-repeat = Maimaita
tasks-repeat-never = Ba ya maimaituwa
tasks-repeat-daily = Kullum
tasks-repeat-weekly = Kowane mako
tasks-repeat-monthly = Kowane wata
tasks-repeat-yearly = Kowace shekara
tasks-repeat-other = Na musamman
tasks-remind = Tunatar da ni
tasks-remind-off = Kada a tunatar
tasks-remind-on-time = A lokacin
tasks-remind-morning = A ranar, { $time }
tasks-remind-hour-before = Awa ɗaya kafin lokaci
tasks-remind-day-before = Kwana ɗaya kafin lokaci
tasks-label-add = Ƙara lakabi
tasks-label-task = Sanya wa aiki lakabi
tasks-files-attach = Haɗa fayiloli
tasks-files-pick = Haɗa
tasks-file-open = Buɗe
tasks-file-remove = Cire fayil
tasks-file-here = A wannan kwamfuta kaɗai
tasks-cancel = Soke
tasks-save = Ajiye
tasks-not-a-time = “{ $text }” ba lokaci ba ne, misali { $example }.

## Due days

tasks-due-today = Yau
tasks-due-tomorrow = Gobe
tasks-due-yesterday = Jiya
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = An kammala aikin
tasks-toast-next = An gama. Na gaba shi ne { $date }
tasks-toast-deleted = An share aikin
tasks-files-added = { $count ->
    [one] An haɗa fayil
   *[other] An haɗa fayiloli { $count }
}
tasks-file-removed = An cire “{ $name }”
tasks-files-left-out = Ba a haɗa ba: { $names }. Aiki yana karɓar fayiloli har zuwa { $limit }, ba folda ba.
tasks-file-missing = Wannan fayil ba ya nan kuma.
tasks-toast-added = { $count ->
    [one] An ƙara a Ayyuka
   *[other] An ƙara ayyuka { $count }
}
tasks-mail-gone = Wannan wasiƙar ba ta nan kuma.
tasks-toast-list-deleted = An share jerin
tasks-toast-moved = An mayar zuwa { $list }
# A task dragged to another place in its own list.
tasks-toast-placed = An matsar da aikin
tasks-toast-rescheduled = An sake tsara lokacin aikin
tasks-toast-rescheduled-several = { $count ->
    [one] An canza ranar aiki
   *[other] An canza ranar ayyuka { $count }
}
tasks-toast-done-several = { $count ->
    [one] An kammala aiki
   *[other] An kammala ayyuka { $count }
}
tasks-toast-open-several = { $count ->
    [one] An yi wa aiki alamar ba a kammala ba
   *[other] An yi wa ayyuka { $count } alamar ba a kammala ba
}
tasks-toast-starred = { $count ->
    [one] An sanya wa aiki tauraro
   *[other] An sanya wa ayyuka { $count } tauraro
}
tasks-toast-unstarred = { $count ->
    [one] An cire tauraro
   *[other] An cire tauraro daga ayyuka { $count }
}
tasks-toast-deleted-several = { $count ->
    [one] An share aiki
   *[other] An share ayyuka { $count }
}
