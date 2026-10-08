# Katna Mail, Zulu (isiZulu): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Amanothi
notes-view-reminders = Izikhumbuzo
notes-view-archive = Ingobo yomlando
notes-view-trash = Udoti
notes-edit-labels = Hlela amalebula
notes-search = Sesha amanothi
notes-loading = Kuvulwa amanothi akho…

## Board

notes-take-a-note = Thatha inothi…
notes-new-list = Uhlu olusha
notes-new-note = Inothi elisha
notes-pinned = Okuphiniwe
notes-others = Okunye
notes-empty = Amanothi owengezayo avela lapha
notes-archive-empty = Amanothi akho agcinwe kungobo yomlando avela lapha
notes-trash-empty = Awekho amanothi kudoti
notes-none-found = Awekho amanothi ahambisanayo
notes-label-empty = Awekho amanothi analo ilebula okwamanje
notes-reminders-empty = Amanothi anezikhumbuzo ezizayo avela lapha
notes-trash-note = Amanothi asudotini asuswa ngemva kwezinsuku ezingu-7.
notes-empty-trash = Khipha udoti
notes-ticked = { $count ->
    [one] + into e-{ $count } eyaphawulwe
   *[other] + izinto ezingu-{ $count } ezaphawulwe
}
notes-select = Khetha inothi
notes-selected = { $count ->
    [one] { $count } likhethiwe
   *[other] { $count } akhethiwe
}
notes-select-clear = Sula ukukhetha

## A note's buttons

notes-pin = Phina inothi
notes-unpin = Susa ukuphina inothi
notes-archive = Faka kungobo yomlando
notes-unarchive = Khipha kungobo yomlando
notes-delete = Susa inothi
notes-restore = Buyisela
notes-delete-forever = Susa unomphela
notes-color = Umbala wangemuva
notes-checkboxes = Bonisa noma fihla amabhokisi okuphawula
notes-labels = Amalebula
notes-close = Vala
notes-more = Okwengeziwe
notes-make-copy = Yenza ikhophi
notes-remind = Ngikhumbuze
notes-add-picture = Engeza isithombe
notes-history = Umlando wezinguqulo
notes-ai = Ngisize ngibhale
notes-send-as-mail = Thumela njengemeyili
notes-save-markdown = Londoloza njenge-Markdown
notes-save-pdf = Londoloza njenge-PDF

## The open note

notes-title = Isihloko
notes-edited = Kuhlelwe: { $date }
notes-on-this-computer = Kule khompyutha
notes-where = Lapho leli nothi ligcinwe khona
notes-untitled = Inothi elingenasihloko

## Pictures

notes-picture-choose = Engeza izithombe
notes-picture-remove = Susa isithombe
notes-picture-too-big = Izithombe ezifika ku-{ $size } zingangena enothini
notes-picture-kind = Lelo fayela akusona isithombe i-Katna engasibonisa
notes-picture-unreadable = Akukwazekanga ukufunda u-{ $name }: { $error }

## Reminders

notes-remind-me = Ngikhumbuze
notes-remind-off = Susa isikhumbuzo
notes-remind-in-the-past = Khetha isikhathi esingakadluli
notes-remind-today = Namuhla, { $time }
notes-remind-tomorrow = Kusasa, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = Isikhumbuzo sisethelwe u-{ $when }
notes-reminder-off = Isikhumbuzo sisusiwe

## Links between notes

notes-link-note = Xhumanisa inothi
notes-link-new = Inothi elisha elithi "{ $title }"
notes-linked-from = Kuxhunywe kusuka ku-
notes-link-gone = Lelo nothi alisekho lapha
notes-new-note-gone = Inothi elisha alisekho.

## Version history

notes-versions = Izinguqulo
notes-version-now = Manje
notes-version-here = Wena, kule khompyutha
notes-version-yesterday = Izolo, { $time }
notes-version-changes = { $count ->
    [one] Uguquko olu-{ $count }
   *[other] Izinguquko ezingu-{ $count }
}
notes-version-from = Kusuka ku-{ $device }
notes-version-elsewhere = Kusuka kwenye idivayisi
notes-version-created = Kudaliwe
notes-version-restore = Buyisela le nguqulo
notes-version-restored = Inguqulo ibuyiselwe
notes-history-none = Azikho izinguqulo zangaphambili okwamanje

## AI help

notes-ai-tidy = Hlela umbhalo kahle
notes-ai-checklist = Kwenze kube uhlu lokuhlola
notes-ai-summarise = Fingqa
notes-ai-empty = Qala ubhale okuthile
notes-ai-tidied = Umbhalo uhlelwe kahle. U-Ctrl+Z uyawubuyisela.
notes-ai-listed = Kwenziwe kwaba uhlu lokuhlola. U-Ctrl+Z uyakubuyisela.
notes-ai-summarised = Isifinyezo sengezwe phezulu

## Labels

notes-label-note = Faka ilebula kunothi
notes-label-name = Faka igama lelebula
notes-label-create = Dala “{ $name }”
notes-label-remove = Susa ilebula
notes-label-delete = Sula ilebula
notes-labels-none = Awekho amalebula okwamanje. Engeza elilodwa ngenkinobho yelebula yenothi.
notes-labels-done = Kwenziwe
notes-label-renamed = Ilebula iqanjwe kabusha ngokuthi “{ $name }”
notes-label-deleted = Ilebula ethi “{ $name }” isuliwe

## A note about a mail

notes-mail = Imeyili
notes-open-mail = Vula imeyili
notes-open-note = Vula inothi

## Meeting notes

notes-meeting-take = Thatha amanothi omhlangano
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = Abakhona: { $names }
notes-meeting-notes = Amanothi
notes-meeting-actions = Izinto zokwenza
notes-event = Umcimbi
notes-open-event = Vula umcimbi

## Formatting

notes-format = Ukufometha
notes-format-heading-1 = Isihloko 1
notes-format-heading-2 = Isihloko 2
notes-format-normal = Umbhalo ojwayelekile
notes-format-bold = Okugqamile
notes-format-italic = Okutshekile
notes-format-underline = Dwebela
notes-format-quote = Isicaphuno
notes-format-code = Ikhodi
notes-format-divider = Umehlukanisi
notes-format-clear = Sula ukufometha

## Tasks

notes-make-task = Yenze umsebenzi

## Colors (tooltips)

notes-color-none = Awukho umbala
notes-color-coral = I-Coral
notes-color-peach = I-Peach
notes-color-sand = Isihlabathi
notes-color-mint = Uphelepele
notes-color-sage = I-Sage
notes-color-fog = Inkungu
notes-color-storm = Isiphepho
notes-color-dusk = Ukuhwalala
notes-color-blossom = Imbali
notes-color-clay = Ubumba
notes-color-chalk = Ushoki

## Messages at the foot of the window

notes-archived = Inothi ligcinwe kungobo yomlando
notes-unarchived = Inothi likhishwe kungobo yomlando
notes-trashed = Inothi lidluliselwe kudoti
notes-restored = Inothi libuyiselwe
notes-saved = Inothi lilondoloziwe
notes-pinned-count = { $count ->
    [one] Inothi liphiniwe
   *[other] Amanothi angu-{ $count } aphiniwe
}
notes-unpinned-count = { $count ->
    [one] Inothi lisusiwe ukuphina
   *[other] Amanothi angu-{ $count } asusiwe ukuphina
}
notes-colored-count = { $count ->
    [one] Umbala ushintshiwe
   *[other] Umbala ushintshiwe emanothini angu-{ $count }
}
notes-archived-count = { $count ->
    [one] Inothi ligcinwe kungobo yomlando
   *[other] Amanothi angu-{ $count } agcinwe kungobo yomlando
}
notes-unarchived-count = { $count ->
    [one] Inothi likhishwe kungobo yomlando
   *[other] Amanothi angu-{ $count } akhishwe kungobo yomlando
}
notes-trashed-count = { $count ->
    [one] Inothi lidluliselwe kudoti
   *[other] Amanothi angu-{ $count } adluliselwe kudoti
}
notes-restored-count = { $count ->
    [one] Inothi libuyiselwe
   *[other] Amanothi angu-{ $count } abuyiselwe
}
notes-copied-count = { $count ->
    [one] Ikhophi yenziwe
   *[other] Amakhophi angu-{ $count } enziwe
}
notes-empty-discarded = Inothi elingenalutho lilahliwe
notes-mail-gone = Leyo meyili ayisekho lapha
notes-deleted-forever = { $count ->
    [one] Inothi lisusiwe unomphela
   *[other] Amanothi angu-{ $count } asusiwe unomphela
}
