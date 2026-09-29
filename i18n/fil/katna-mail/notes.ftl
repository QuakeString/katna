# Katna Mail, Filipino (Filipino): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Mga Tala
notes-view-archive = Archive
notes-view-trash = Trash
notes-edit-labels = I-edit ang mga label
notes-search = Maghanap sa mga tala
notes-loading = Binubuksan ang iyong mga tala…

## Board

notes-take-a-note = Magtala…
notes-new-list = Bagong listahan
notes-pinned = Naka-pin
notes-others = Iba pa
notes-empty = Lalabas dito ang mga tala na idaragdag mo
notes-archive-empty = Lalabas dito ang iyong mga naka-archive na tala
notes-trash-empty = Walang tala sa Trash
notes-none-found = Walang tugmang tala
notes-label-empty = Wala pang mga tala na may ganitong label
notes-trash-note = Ang mga tala sa Trash ay made-delete pagkalipas ng 7 araw.
notes-empty-trash = Alisin ang laman ng Trash
notes-ticked = { $count ->
    [one] + { $count } naka-tick na item
   *[other] + { $count } naka-tick na item
}

## A note's buttons

notes-pin = I-pin ang tala
notes-unpin = I-unpin ang tala
notes-archive = I-archive
notes-unarchive = I-unarchive
notes-delete = I-delete ang tala
notes-restore = I-restore
notes-delete-forever = I-delete nang permanente
notes-color = Kulay ng background
notes-checkboxes = Ipakita o itago ang mga checkbox
notes-labels = Mga Label
notes-close = Isara

## The open note

notes-title = Pamagat
notes-edited = Na-edit { $date }
notes-on-this-computer = Sa computer na ito
notes-where = Kung saan itinatago ang talang ito

## Labels

notes-label-note = Lagyan ng label ang tala
notes-label-name = Ilagay ang pangalan ng label
notes-label-create = Gumawa ng “{ $name }”
notes-label-remove = Alisin ang label
notes-label-delete = Tanggalin ang label
notes-labels-none = Wala pang mga label. Magdagdag mula sa button ng label ng isang tala.
notes-labels-done = Tapos na
notes-label-renamed = Pinalitan ang pangalan ng label ng “{ $name }”
notes-label-deleted = Natanggal ang label na “{ $name }”

## A note about a mail

notes-mail = Mail
notes-open-mail = Buksan ang mail
notes-open-note = Buksan ang tala

## Meeting notes

notes-meeting-take = Gumawa ng tala ng pulong
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = Mga dadalo: { $names }
notes-meeting-notes = Mga tala
notes-meeting-actions = Mga aksyon
notes-event = Event
notes-open-event = Buksan ang event

## Tasks

notes-make-task = Gawing gawain

## Colors (tooltips)

notes-color-none = Walang kulay
notes-color-coral = Coral
notes-color-peach = Peach
notes-color-sand = Buhangin
notes-color-mint = Mint
notes-color-sage = Sage
notes-color-fog = Hamog
notes-color-storm = Bagyo
notes-color-dusk = Dapithapon
notes-color-blossom = Bulaklak
notes-color-clay = Luwad
notes-color-chalk = Tisa

## Messages at the foot of the window

notes-archived = Na-archive ang tala
notes-unarchived = Na-unarchive ang tala
notes-trashed = Inilipat ang tala sa Trash
notes-restored = Na-restore ang tala
notes-empty-discarded = Itinapon ang walang lamang tala
notes-mail-gone = Wala na rito ang mail na iyon
notes-deleted-forever = { $count ->
    [one] Permanenteng na-delete ang tala
   *[other] Permanenteng na-delete ang { $count } tala
}
