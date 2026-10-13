# Katna Mail, Filipino (Filipino): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Bagong gawain
tasks-all = Lahat ng gawain
tasks-today = Ngayon
tasks-upcoming = Paparating
tasks-starred = Naka-star
tasks-completed-view = Tapos na
tasks-new-list = Gumawa ng bagong listahan
tasks-labels-heading = Mga Label
tasks-on-this-computer = Sa computer na ito
tasks-my-tasks = Aking Mga Gawain
tasks-account-sign-in = Mag-sign in muli para ipakita ang mga gawain
tasks-account-signed-in = Naka-sign in muli sa { $address }. Kinukuha ang iyong mga gawain…
tasks-account-sign-in-refused = Hindi pinapasok ng { $provider } ang Katna. Subukang muli, at payagan ang access sa iyong mga gawain.
tasks-account-refused = Hindi tinanggap ng server ang password. Kailangan ng Yahoo, iCloud, Zoho at iba pa ng app password.
tasks-account-change-password = Palitan ang password
tasks-account-change-password-tooltip = I-type ang bagong password; susuriin ito ng Katna sa server
tasks-account-not-enabled = Hindi pa naka-on ang access sa mga gawain para sa Katna.
tasks-account-failed = Hindi mabasa ang mga listahan ng gawain.
tasks-account-error = Hindi mabasa ang mga listahan ng gawain: { $reason }
tasks-account-none = Walang nakitang listahan ng gawain
tasks-account-none-why = Walang nakitang listahan ng gawain: { $reason }
tasks-account-use-sign-in = Ipinapakita lang ng { $provider } ang mga gawain sa Katna kapag naka-sign in gamit ang { $provider }.
tasks-account-sign-in-with = Mag-sign in gamit ang { $provider }
tasks-account-looking = Naghahanap ng mga listahan ng gawain…
tasks-account-try-again = Subukang muli
tasks-account-try-again-tooltip = Suriin muli ngayon ang mga gawain ng account na ito
tasks-account-fixing = Inaayos na…
tasks-list-name-placeholder = Pangalan ng listahan

## Lists and tasks

tasks-loading = Binabasa ang iyong mga gawain…
tasks-no-lists = Dito lalabas ang iyong mga listahan ng gawain.
tasks-search = Maghanap sa mga gawain
tasks-search-none = Walang gawain na tumutugma sa iyong paghahanap.
tasks-add = Magdagdag ng gawain
tasks-title-placeholder = Pamagat
tasks-add-step = Magdagdag ng subtask
tasks-empty = Wala pang gawain. Magdagdag ng isa sa itaas.
tasks-starred-empty = Lagyan ng star ang isang gawain para makita ito rito.
tasks-label-empty = Walang bukas na gawain na may ganitong label.
tasks-today-empty = Walang due ngayon.
tasks-completed-empty = Lalabas dito ang mga gawaing natapos mo.
tasks-upcoming-add = Magdagdag ng gawain para sa { $day }
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = Mula sa mail
tasks-from-note-quiet = Mula sa tala
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Lampas na sa takda
tasks-completed = { $count ->
    [one] Tapos na ({ $count })
   *[other] Tapos na ({ $count })
}
tasks-list-options = Mga opsyon sa listahan
tasks-sort-by = Ayusin ayon sa
tasks-sort-my-order = Aking pagkakasunod
tasks-sort-date = Petsa
tasks-sort-starred = Kamakailang naka-star
tasks-sort-title = Pamagat
tasks-rename-list = Palitan ang pangalan ng listahan
tasks-delete-list = I-delete ang listahan
tasks-mark-done = Markahang tapos na
tasks-mark-open = Markahang hindi pa tapos
tasks-star = I-star
tasks-unstar = Alisin ang star
tasks-edit-title = I-edit ang pamagat
tasks-details = Mga detalye
tasks-delete = I-delete
tasks-move-to = Ilipat sa { $list }
tasks-from-mail = Mail
tasks-open-mail = Buksan ang mail
tasks-from-note = Tala
tasks-open-note = Buksan ang tala
tasks-note-gone = Wala na rito ang talang iyon.
tasks-no-subject = (walang paksa)

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
    [one] { $count } ang napili
   *[other] { $count } ang napili
}
tasks-select-clear = I-clear ang pinili
tasks-select-move = Ilipat sa listahan
tasks-select-date = Itakda ang petsa
tasks-next-week = Sa susunod na linggo

## The details dialog

tasks-notes-placeholder = Magdagdag ng mga detalye
tasks-date = Petsa
tasks-no-date = Walang petsa
tasks-time-placeholder = Magdagdag ng oras
tasks-repeat = Ulitin
tasks-repeat-never = Hindi inuulit
tasks-repeat-daily = Araw-araw
tasks-repeat-weekly = Lingguhan
tasks-repeat-monthly = Buwan-buwan
tasks-repeat-yearly = Taon-taon
tasks-repeat-other = Custom
tasks-remind = Paalalahanan ako
tasks-remind-off = Huwag paalalahanan
tasks-remind-on-time = Sa oras mismo
tasks-remind-morning = Sa araw na iyon, { $time }
tasks-remind-hour-before = Isang oras bago
tasks-remind-day-before = Isang araw bago
tasks-label-add = Magdagdag ng label
tasks-label-task = Lagyan ng label ang gawain
tasks-files-attach = Mag-attach ng mga file
tasks-files-pick = I-attach
tasks-file-open = Buksan
tasks-file-remove = Alisin ang file
tasks-file-here = Nasa computer na ito lang
tasks-cancel = Kanselahin
tasks-save = I-save
tasks-not-a-time = Ang “{ $text }” ay hindi oras, halimbawa { $example }.

## Due days

tasks-due-today = Ngayon
tasks-due-tomorrow = Bukas
tasks-due-yesterday = Kahapon
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Tapos na ang gawain
tasks-toast-next = Tapos na. Susunod sa { $date }
tasks-toast-deleted = Na-delete ang gawain
tasks-files-added = { $count ->
    [one] Na-attach ang file
   *[other] Na-attach ang { $count } file
}
tasks-file-removed = Inalis ang “{ $name }”
tasks-files-left-out = Hindi na-attach: { $names }. Tumatanggap ang gawain ng mga file hanggang { $limit }, hindi ng mga folder.
tasks-file-missing = Wala na rito ang file na iyon.
tasks-toast-added = { $count ->
    [one] Naidagdag sa Mga Gawain
   *[other] Naidagdag ang { $count } gawain
}
tasks-mail-gone = Wala na rito ang mail na iyon.
tasks-toast-list-deleted = Na-delete ang listahan
tasks-toast-moved = Inilipat sa { $list }
tasks-toast-placed = Inilipat ang gawain
tasks-toast-rescheduled = Na-reschedule ang gawain
tasks-toast-rescheduled-several = { $count ->
    [one] Inilipat ang petsa ng gawain
   *[other] Inilipat ang petsa ng { $count } gawain
}
tasks-toast-done-several = { $count ->
    [one] Natapos ang gawain
   *[other] Natapos ang { $count } gawain
}
tasks-toast-open-several = { $count ->
    [one] Minarkahang hindi pa tapos ang gawain
   *[other] Minarkahang hindi pa tapos ang { $count } gawain
}
tasks-toast-starred = { $count ->
    [one] Na-star ang gawain
   *[other] Na-star ang { $count } gawain
}
tasks-toast-unstarred = { $count ->
    [one] Inalis ang star
   *[other] Inalis ang star sa { $count } gawain
}
tasks-toast-deleted-several = { $count ->
    [one] Na-delete ang gawain
   *[other] Na-delete ang { $count } gawain
}
