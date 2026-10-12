# Katna Mail, Lao (ລາວ): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = ບັນທຶກ
notes-view-reminders = ການແຈ້ງເຕືອນ
notes-view-archive = ເກັບຖາວອນ
notes-view-trash = ຖັງຂີ້ເຫຍື້ອ
notes-edit-labels = ແກ້ໄຂປ້າຍກຳກັບ
notes-search = ຊອກຫາບັນທຶກ
notes-loading = ກຳລັງເປີດບັນທຶກຂອງທ່ານ…

## Board

notes-take-a-note = ຈົດບັນທຶກ…
notes-new-list = ລາຍການໃໝ່
notes-new-note = ບັນທຶກໃໝ່
notes-pinned = ປັກໝຸດແລ້ວ
notes-others = ອື່ນໆ
notes-empty = ບັນທຶກທີ່ທ່ານເພີ່ມຈະສະແດງຢູ່ບ່ອນນີ້
notes-archive-empty = ບັນທຶກທີ່ເກັບຖາວອນຈະສະແດງຢູ່ບ່ອນນີ້
notes-trash-empty = ບໍ່ມີບັນທຶກໃນຖັງຂີ້ເຫຍື້ອ
notes-none-found = ບໍ່ພົບບັນທຶກທີ່ກົງກັນ
notes-label-empty = ຍັງບໍ່ມີບັນທຶກທີ່ມີປ້າຍກຳກັບນີ້
notes-reminders-empty = ບັນທຶກທີ່ມີການແຈ້ງເຕືອນທີ່ຈະມາເຖິງ ຈະສະແດງຢູ່ບ່ອນນີ້
notes-trash-note = ບັນທຶກໃນຖັງຂີ້ເຫຍື້ອຈະຖືກລຶບຫຼັງຈາກ 7 ວັນ.
notes-empty-trash = ລ້າງຖັງຂີ້ເຫຍື້ອ
notes-ticked = { $count ->
   *[other] + ລາຍການທີ່ໝາຍແລ້ວ { $count } ລາຍການ
}
notes-select = ເລືອກບັນທຶກ
notes-selected = { $count ->
   *[other] ເລືອກແລ້ວ { $count }
}
notes-select-clear = ລ້າງການເລືອກ

## A note's buttons

notes-pin = ປັກໝຸດບັນທຶກ
notes-unpin = ເອົາໝຸດອອກຈາກບັນທຶກ
notes-archive = ເກັບຖາວອນ
notes-unarchive = ຍົກເລີກການເກັບຖາວອນ
notes-delete = ລຶບບັນທຶກ
notes-restore = ກູ້ຄືນ
notes-delete-forever = ລຶບຖາວອນ
notes-color = ຕົວເລືອກພື້ນຫຼັງ
notes-checkboxes = ສະແດງ/ເຊື່ອງຊ່ອງໝາຍ
notes-labels = ປ້າຍກຳກັບ
notes-close = ປິດ
notes-more = ເພີ່ມເຕີມ
notes-make-copy = ສ້າງສຳເນົາ
notes-remind = ແຈ້ງເຕືອນຂ້ອຍ
notes-add-picture = ເພີ່ມຮູບພາບ
notes-history = ປະຫວັດເວີຊັນ
notes-ai = ຊ່ວຍຂ້ອຍຂຽນ
notes-send-as-mail = ສົ່ງເປັນອີເມວ
notes-save-markdown = ບັນທຶກເປັນ Markdown
notes-save-pdf = ບັນທຶກເປັນ PDF

## The open note

notes-title = ຫົວຂໍ້
notes-edited = ແກ້ໄຂເມື່ອ { $date }
notes-on-this-computer = ໃນຄອມພິວເຕີເຄື່ອງນີ້
notes-where = ບ່ອນເກັບບັນທຶກນີ້
notes-untitled = ບັນທຶກທີ່ບໍ່ມີຊື່

## Pictures

notes-picture-choose = ເພີ່ມຮູບພາບ
notes-picture-remove = ລຶບຮູບພາບອອກ
notes-picture-too-big = ຮູບພາບຂະໜາດບໍ່ເກີນ { $size } ໃສ່ໃນບັນທຶກໄດ້
notes-picture-kind = ໄຟລ໌ນັ້ນບໍ່ແມ່ນຮູບພາບທີ່ Katna ສະແດງໄດ້
notes-picture-unreadable = ອ່ານ { $name } ບໍ່ໄດ້: { $error }

## Reminders

notes-remind-me = ແຈ້ງເຕືອນຂ້ອຍ
notes-remind-off = ລຶບການແຈ້ງເຕືອນ
notes-remind-in-the-past = ເລືອກເວລາທີ່ຍັງບໍ່ທັນຜ່ານໄປ
notes-remind-today = ມື້ນີ້, { $time }
notes-remind-tomorrow = ມື້ອື່ນ, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = ຕັ້ງການແຈ້ງເຕືອນໄວ້ { $when }
notes-reminder-off = ລຶບການແຈ້ງເຕືອນແລ້ວ

## Links between notes

notes-link-note = ລິ້ງບັນທຶກ
notes-link-new = ບັນທຶກໃໝ່ “{ $title }”
notes-linked-from = ລິ້ງມາຈາກ
notes-link-gone = ບັນທຶກນັ້ນບໍ່ມີຢູ່ແລ້ວ
notes-new-note-gone = ບັນທຶກໃໝ່ບໍ່ມີແລ້ວ.

## Version history

notes-versions = ເວີຊັນ
notes-version-now = ຕອນນີ້
notes-version-here = ທ່ານ, ໃນຄອມພິວເຕີເຄື່ອງນີ້
notes-version-yesterday = ມື້ວານ, { $time }
notes-version-changes = { $count ->
   *[other] { $count } ການປ່ຽນແປງ
}
notes-version-from = ຈາກ { $device }
notes-version-elsewhere = ຈາກອຸປະກອນອື່ນ
notes-version-created = ສ້າງແລ້ວ
notes-version-restore = ກູ້ຄືນເວີຊັນນີ້
notes-version-restored = ກູ້ຄືນເວີຊັນແລ້ວ
notes-history-none = ຍັງບໍ່ມີເວີຊັນກ່ອນໜ້າ

## AI help

notes-ai-tidy = ຈັດຂໍ້ຄວາມໃຫ້ຮຽບຮ້ອຍ
notes-ai-checklist = ປ່ຽນເປັນລາຍການກວດ
notes-ai-summarise = ສະຫຼຸບ
notes-ai-empty = ຂຽນບາງຢ່າງກ່ອນ
notes-ai-tidied = ຈັດຂໍ້ຄວາມແລ້ວ. Ctrl+Z ເພື່ອເອົາຄືນ.
notes-ai-listed = ປ່ຽນເປັນລາຍການກວດແລ້ວ. Ctrl+Z ເພື່ອເອົາຄືນ.
notes-ai-summarised = ເພີ່ມບົດສະຫຼຸບໄວ້ເທິງສຸດແລ້ວ

## Labels

notes-label-note = ໃສ່ປ້າຍກຳກັບໃຫ້ບັນທຶກ
notes-label-name = ໃສ່ຊື່ປ້າຍກຳກັບ
notes-label-create = ສ້າງ “{ $name }”
notes-label-remove = ເອົາປ້າຍກຳກັບອອກ
notes-label-delete = ລຶບປ້າຍກຳກັບ
notes-labels-none = ຍັງບໍ່ມີປ້າຍກຳກັບ. ເພີ່ມໄດ້ຈາກປຸ່ມປ້າຍກຳກັບຂອງບັນທຶກ.
notes-labels-done = ສຳເລັດ
notes-label-renamed = ປ່ຽນຊື່ປ້າຍກຳກັບເປັນ “{ $name }” ແລ້ວ
notes-label-deleted = ລຶບປ້າຍກຳກັບ “{ $name }” ແລ້ວ

## A note about a mail

notes-mail = ອີເມວ
notes-open-mail = ເປີດອີເມວ
notes-open-note = ເປີດບັນທຶກ

## Meeting notes

notes-meeting-take = ບັນທຶກການປະຊຸມ
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = ຜູ້ເຂົ້າຮ່ວມ: { $names }
notes-meeting-notes = ບັນທຶກ
notes-meeting-actions = ລາຍການທີ່ຕ້ອງເຮັດ
notes-event = ເຫດການ
notes-open-event = ເປີດເຫດການ

## Formatting

notes-format = ການຈັດຮູບແບບ
notes-format-heading-1 = ຫົວເລື່ອງ 1
notes-format-heading-2 = ຫົວເລື່ອງ 2
notes-format-normal = ຂໍ້ຄວາມທຳມະດາ
notes-format-bold = ຕົວໜາ
notes-format-italic = ຕົວເອນ
notes-format-underline = ຂີດກ້ອງ
notes-format-quote = ຄຳອ້າງອີງ
notes-format-code = ໂຄ້ດ
notes-format-divider = ເສັ້ນແບ່ງ
notes-format-clear = ລ້າງການຈັດຮູບແບບ

## Tasks

notes-make-task = ເຮັດເປັນວຽກ

## Colors (tooltips)

notes-color-none = ບໍ່ມີສີ
notes-color-coral = ສີປະການັງ
notes-color-peach = ສີພີຊ
notes-color-sand = ສີຊາຍ
notes-color-mint = ສີມິນ
notes-color-sage = ສີເຊດ
notes-color-fog = ສີໝອກ
notes-color-storm = ສີພະຍຸ
notes-color-dusk = ສີຄ່ຳ
notes-color-blossom = ສີດອກໄມ້
notes-color-clay = ສີດິນໜຽວ
notes-color-chalk = ສີສໍຂາວ

## Messages at the foot of the window

notes-archived = ເກັບບັນທຶກຖາວອນແລ້ວ
notes-unarchived = ຍົກເລີກການເກັບບັນທຶກຖາວອນແລ້ວ
notes-trashed = ຍ້າຍບັນທຶກໄປຖັງຂີ້ເຫຍື້ອແລ້ວ
notes-restored = ກູ້ຄືນບັນທຶກແລ້ວ
notes-saved = ບັນທຶກບັນທຶກແລ້ວ
notes-pinned-count = { $count ->
   *[other] ປັກໝຸດ { $count } ບັນທຶກແລ້ວ
}
notes-unpinned-count = { $count ->
   *[other] ເອົາໝຸດອອກຈາກ { $count } ບັນທຶກແລ້ວ
}
notes-colored-count = { $count ->
   *[other] ປ່ຽນສີຂອງ { $count } ບັນທຶກແລ້ວ
}
notes-archived-count = { $count ->
   *[other] ເກັບຖາວອນ { $count } ບັນທຶກແລ້ວ
}
notes-unarchived-count = { $count ->
   *[other] ເອົາ { $count } ບັນທຶກອອກຈາກບ່ອນເກັບຖາວອນແລ້ວ
}
notes-trashed-count = { $count ->
   *[other] ຍ້າຍ { $count } ບັນທຶກໄປທີ່ຖັງຂີ້ເຫຍື້ອແລ້ວ
}
notes-restored-count = { $count ->
   *[other] ກູ້ຄືນ { $count } ບັນທຶກແລ້ວ
}
notes-copied-count = { $count ->
   *[other] ສ້າງ { $count } ສຳເນົາແລ້ວ
}
notes-empty-discarded = ຖິ້ມບັນທຶກເປົ່າແລ້ວ
notes-mail-gone = ບໍ່ມີອີເມວນັ້ນອີກແລ້ວ
notes-deleted-forever = { $count ->
   *[other] ລຶບບັນທຶກ { $count } ລາຍການຖາວອນແລ້ວ
}
