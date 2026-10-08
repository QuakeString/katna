# Katna Mail, Filipino (Filipino): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Mga Tala
notes-view-reminders = Mga Paalala
notes-view-archive = Archive
notes-view-trash = Trash
notes-edit-labels = I-edit ang mga label
notes-search = Maghanap sa mga tala
notes-loading = Binubuksan ang iyong mga tala…

## Board

notes-take-a-note = Magtala…
notes-new-list = Bagong listahan
notes-new-note = Bagong tala
notes-pinned = Naka-pin
notes-others = Iba pa
notes-empty = Lalabas dito ang mga tala na idaragdag mo
notes-archive-empty = Lalabas dito ang iyong mga naka-archive na tala
notes-trash-empty = Walang tala sa Trash
notes-none-found = Walang tugmang tala
notes-label-empty = Wala pang mga tala na may ganitong label
notes-reminders-empty = Lalabas dito ang mga talang may paparating na paalala
notes-trash-note = Ang mga tala sa Trash ay made-delete pagkalipas ng 7 araw.
notes-empty-trash = Alisin ang laman ng Trash
notes-ticked = { $count ->
    [one] + { $count } naka-tick na item
   *[other] + { $count } naka-tick na item
}
notes-select = Piliin ang tala
notes-selected = { $count ->
    [one] { $count } ang napili
   *[other] { $count } ang napili
}
notes-select-clear = I-clear ang pinili

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
notes-more = Iba pa
notes-make-copy = Gumawa ng kopya
notes-remind = Paalalahanan ako
notes-add-picture = Magdagdag ng larawan
notes-history = History ng bersyon
notes-ai = Tulungan akong magsulat
notes-send-as-mail = Ipadala bilang mail
notes-save-markdown = I-save bilang Markdown
notes-save-pdf = I-save bilang PDF

## The open note

notes-title = Pamagat
notes-edited = Na-edit { $date }
notes-on-this-computer = Sa computer na ito
notes-where = Kung saan itinatago ang talang ito
notes-untitled = Talang walang pamagat

## Pictures

notes-picture-choose = Magdagdag ng mga larawan
notes-picture-remove = Alisin ang larawan
notes-picture-too-big = Puwedeng ilagay sa tala ang mga larawang hanggang { $size }
notes-picture-kind = Hindi larawang kayang ipakita ng Katna ang file na iyan
notes-picture-unreadable = Hindi mabasa ang { $name }: { $error }

## Reminders

notes-remind-me = Paalalahanan ako
notes-remind-off = Alisin ang paalala
notes-remind-in-the-past = Pumili ng oras na hindi pa lumilipas
notes-remind-today = Ngayon, { $time }
notes-remind-tomorrow = Bukas, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = Nakatakda ang paalala sa { $when }
notes-reminder-off = Inalis ang paalala

## Links between notes

notes-link-note = Mag-link ng tala
notes-link-new = Bagong tala “{ $title }”
notes-linked-from = Naka-link mula sa
notes-link-gone = Wala na rito ang talang iyon
notes-new-note-gone = Nawala na ang bagong tala.

## Version history

notes-versions = Mga bersyon
notes-version-now = Ngayon
notes-version-here = Ikaw, sa computer na ito
notes-version-yesterday = Kahapon, { $time }
notes-version-changes = { $count ->
    [one] { $count } pagbabago
   *[other] { $count } pagbabago
}
notes-version-from = Mula sa { $device }
notes-version-elsewhere = Mula sa ibang device
notes-version-created = Ginawa
notes-version-restore = Ibalik ang bersyong ito
notes-version-restored = Naibalik ang bersyon
notes-history-none = Wala pang mas naunang bersyon

## AI help

notes-ai-tidy = Ayusin ang teksto
notes-ai-checklist = Gawin itong checklist
notes-ai-summarise = Ibuod
notes-ai-empty = Magsulat muna ng kahit ano
notes-ai-tidied = Naayos ang teksto. Ibinabalik ito ng Ctrl+Z.
notes-ai-listed = Ginawang checklist. Ibinabalik ito ng Ctrl+Z.
notes-ai-summarised = Idinagdag ang buod sa itaas

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

## Formatting

notes-format = Pag-format
notes-format-heading-1 = Heading 1
notes-format-heading-2 = Heading 2
notes-format-normal = Normal na text
notes-format-bold = Bold
notes-format-italic = Italic
notes-format-underline = Salungguhitan
notes-format-quote = Sipi
notes-format-code = Code
notes-format-divider = Divider
notes-format-clear = I-clear ang pag-format

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
notes-saved = Na-save ang tala
notes-pinned-count = { $count ->
    [one] Na-pin ang tala
   *[other] Na-pin ang { $count } tala
}
notes-unpinned-count = { $count ->
    [one] Na-unpin ang tala
   *[other] Na-unpin ang { $count } tala
}
notes-colored-count = { $count ->
    [one] Pinalitan ang kulay
   *[other] Pinalitan ang kulay ng { $count } tala
}
notes-archived-count = { $count ->
    [one] Na-archive ang tala
   *[other] Na-archive ang { $count } tala
}
notes-unarchived-count = { $count ->
    [one] Inalis sa archive ang tala
   *[other] Inalis sa archive ang { $count } tala
}
notes-trashed-count = { $count ->
    [one] Inilipat sa Trash ang tala
   *[other] Inilipat sa Trash ang { $count } tala
}
notes-restored-count = { $count ->
    [one] Naibalik ang tala
   *[other] Naibalik ang { $count } tala
}
notes-copied-count = { $count ->
    [one] Nagawa ang kopya
   *[other] Nagawa ang { $count } kopya
}
notes-empty-discarded = Itinapon ang walang lamang tala
notes-mail-gone = Wala na rito ang mail na iyon
notes-deleted-forever = { $count ->
    [one] Permanenteng na-delete ang tala
   *[other] Permanenteng na-delete ang { $count } tala
}
