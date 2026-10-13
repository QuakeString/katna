# Katna Mail, Lao (ລາວ): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = ໜ້າວຽກໃໝ່
tasks-all = ໜ້າວຽກທັງໝົດ
tasks-today = ມື້ນີ້
tasks-upcoming = ທີ່ຈະມາເຖິງ
tasks-starred = ມີດາວ
tasks-completed-view = ສຳເລັດແລ້ວ
tasks-new-list = ສ້າງລາຍການໃໝ່
tasks-labels-heading = ປ້າຍກຳກັບ
tasks-on-this-computer = ໃນຄອມພິວເຕີເຄື່ອງນີ້
tasks-my-tasks = ໜ້າວຽກຂອງຂ້ອຍ
tasks-account-sign-in = ເຂົ້າສູ່ລະບົບອີກຄັ້ງເພື່ອສະແດງໜ້າວຽກ
tasks-account-signed-in = ເຂົ້າສູ່ລະບົບ { $address } ອີກຄັ້ງແລ້ວ. ກຳລັງດຶງໜ້າວຽກຂອງທ່ານ…
tasks-account-sign-in-refused = { $provider } ບໍ່ໃຫ້ Katna ເຂົ້າ. ລອງໃໝ່ ແລະ ອະນຸຍາດໃຫ້ເຂົ້າເຖິງໜ້າວຽກຂອງທ່ານ.
tasks-account-refused = ເຊີບເວີບໍ່ຍອມຮັບລະຫັດຜ່ານ. Yahoo, iCloud, Zoho ແລະ ອື່ນໆ ຕ້ອງການລະຫັດຜ່ານແອັບ.
tasks-account-change-password = ປ່ຽນລະຫັດຜ່ານ
tasks-account-change-password-tooltip = ພິມລະຫັດຜ່ານໃໝ່; Katna ຈະກວດມັນກັບເຊີບເວີ
tasks-account-not-enabled = ການເຂົ້າເຖິງໜ້າວຽກສຳລັບ Katna ຍັງບໍ່ໄດ້ເປີດເທື່ອ.
tasks-account-failed = ບໍ່ສາມາດອ່ານລາຍການໜ້າວຽກໄດ້.
tasks-account-error = ບໍ່ສາມາດອ່ານລາຍການໜ້າວຽກໄດ້: { $reason }
tasks-account-none = ບໍ່ພົບລາຍການໜ້າວຽກ
tasks-account-none-why = ບໍ່ພົບລາຍການໜ້າວຽກ: { $reason }
tasks-account-use-sign-in = { $provider } ສະແດງໜ້າວຽກສະເພາະໃຫ້ Katna ທີ່ເຂົ້າສູ່ລະບົບດ້ວຍ { $provider } ເທົ່ານັ້ນ.
tasks-account-sign-in-with = ເຂົ້າສູ່ລະບົບດ້ວຍ { $provider }
tasks-account-looking = ກຳລັງຊອກຫາລາຍການໜ້າວຽກ…
tasks-account-try-again = ລອງໃໝ່
tasks-account-try-again-tooltip = ກວດເບິ່ງໜ້າວຽກຂອງບັນຊີນີ້ອີກຄັ້ງດຽວນີ້
tasks-account-fixing = ກຳລັງດຳເນີນການ…
tasks-list-name-placeholder = ຊື່ລາຍການ

## Lists and tasks

tasks-loading = ກຳລັງອ່ານໜ້າວຽກຂອງເຈົ້າ…
tasks-no-lists = ລາຍການໜ້າວຽກຂອງເຈົ້າຈະສະແດງຢູ່ບ່ອນນີ້.
tasks-search = ຊອກຫາໜ້າວຽກ
tasks-search-none = ບໍ່ມີໜ້າວຽກທີ່ກົງກັບການຊອກຫາຂອງທ່ານ.
tasks-add = ເພີ່ມໜ້າວຽກ
tasks-title-placeholder = ຫົວຂໍ້
tasks-add-step = ເພີ່ມໜ້າວຽກຍ່ອຍ
tasks-empty = ຍັງບໍ່ມີໜ້າວຽກ. ເພີ່ມໄດ້ຢູ່ດ້ານເທິງ.
tasks-starred-empty = ໃສ່ດາວໃຫ້ໜ້າວຽກເພື່ອເບິ່ງຢູ່ບ່ອນນີ້.
tasks-label-empty = ບໍ່ມີໜ້າວຽກທີ່ຍັງເປີດຢູ່ທີ່ມີປ້າຍກຳກັບນີ້.
tasks-today-empty = ບໍ່ມີໜ້າວຽກທີ່ຮອດກຳນົດມື້ນີ້.
tasks-completed-empty = ໜ້າວຽກທີ່ທ່ານເຮັດສຳເລັດຈະສະແດງຢູ່ບ່ອນນີ້.
tasks-upcoming-add = ເພີ່ມໜ້າວຽກສຳລັບ { $day }
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = ຈາກອີເມວ
tasks-from-note-quiet = ຈາກບັນທຶກ
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = ເກີນກຳນົດ
tasks-completed = { $count ->
   *[other] ສຳເລັດແລ້ວ ({ $count })
}
tasks-list-options = ຕົວເລືອກລາຍການ
tasks-sort-by = ຈັດຮຽງຕາມ
tasks-sort-my-order = ລຳດັບຂອງຂ້ອຍ
tasks-sort-date = ວັນທີ
tasks-sort-starred = ຕິດດາວຫຼ້າສຸດ
tasks-sort-title = ຫົວຂໍ້
tasks-rename-list = ປ່ຽນຊື່ລາຍການ
tasks-delete-list = ລຶບລາຍການ
tasks-mark-done = ໝາຍວ່າສຳເລັດແລ້ວ
tasks-mark-open = ໝາຍວ່າຍັງບໍ່ສຳເລັດ
tasks-star = ໃສ່ດາວ
tasks-unstar = ເອົາດາວອອກ
tasks-edit-title = ແກ້ໄຂຫົວຂໍ້
tasks-details = ລາຍລະອຽດ
tasks-delete = ລຶບ
tasks-move-to = ຍ້າຍໄປ { $list }
tasks-from-mail = ອີເມວ
tasks-open-mail = ເປີດອີເມວ
tasks-from-note = ບັນທຶກ
tasks-open-note = ເປີດບັນທຶກ
tasks-note-gone = ບໍ່ມີບັນທຶກນັ້ນອີກແລ້ວ.
tasks-no-subject = (ບໍ່ມີຫົວຂໍ້)

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
   *[other] ເລືອກແລ້ວ { $count }
}
tasks-select-clear = ລ້າງການເລືອກ
tasks-select-move = ຍ້າຍໄປລາຍການ
tasks-select-date = ຕັ້ງວັນທີ
tasks-next-week = ອາທິດໜ້າ

## The details dialog

tasks-notes-placeholder = ເພີ່ມລາຍລະອຽດ
tasks-date = ວັນທີ
tasks-no-date = ບໍ່ມີວັນທີ
tasks-time-placeholder = ເພີ່ມເວລາ
tasks-repeat = ເຮັດຊ້ຳ
tasks-repeat-never = ບໍ່ເຮັດຊ້ຳ
tasks-repeat-daily = ທຸກມື້
tasks-repeat-weekly = ທຸກອາທິດ
tasks-repeat-monthly = ທຸກເດືອນ
tasks-repeat-yearly = ທຸກປີ
tasks-repeat-other = ກຳນົດເອງ
tasks-remind = ເຕືອນຂ້ອຍ
tasks-remind-off = ບໍ່ເຕືອນ
tasks-remind-on-time = ຕາມເວລາ
tasks-remind-morning = ໃນມື້ນັ້ນ { $time }
tasks-remind-hour-before = ກ່ອນ 1 ຊົ່ວໂມງ
tasks-remind-day-before = ກ່ອນ 1 ມື້
tasks-label-add = ເພີ່ມປ້າຍກຳກັບ
tasks-label-task = ຕິດປ້າຍກຳກັບໜ້າວຽກ
tasks-files-attach = ແນບໄຟລ໌
tasks-files-pick = ແນບ
tasks-file-open = ເປີດ
tasks-file-remove = ເອົາໄຟລ໌ອອກ
tasks-file-here = ສະເພາະໃນຄອມພິວເຕີເຄື່ອງນີ້
tasks-cancel = ຍົກເລີກ
tasks-save = ບັນທຶກ
tasks-not-a-time = “{ $text }” ບໍ່ແມ່ນເວລາ, ຕົວຢ່າງ { $example }.

## Due days

tasks-due-today = ມື້ນີ້
tasks-due-tomorrow = ມື້ອື່ນ
tasks-due-yesterday = ມື້ວານ
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = ວຽກສຳເລັດແລ້ວ
tasks-toast-next = ສຳເລັດແລ້ວ. ຄັ້ງຕໍ່ໄປວັນທີ { $date }
tasks-toast-deleted = ລຶບວຽກແລ້ວ
tasks-files-added = { $count ->
   *[other] ແນບ { $count } ໄຟລ໌ແລ້ວ
}
tasks-file-removed = ເອົາ “{ $name }” ອອກແລ້ວ
tasks-files-left-out = ບໍ່ໄດ້ແນບ: { $names }. ໜ້າວຽກຮັບໄຟລ໌ຂະໜາດບໍ່ເກີນ { $limit }, ບໍ່ຮັບໂຟນເດີ.
tasks-file-missing = ໄຟລ໌ນັ້ນບໍ່ຢູ່ທີ່ນີ້ແລ້ວ.
tasks-toast-added = { $count ->
   *[other] ເພີ່ມ { $count } ວຽກແລ້ວ
}
tasks-mail-gone = ບໍ່ມີອີເມວນັ້ນອີກແລ້ວ.
tasks-toast-list-deleted = ລຶບລາຍການແລ້ວ
tasks-toast-moved = ຍ້າຍໄປ { $list } ແລ້ວ
tasks-toast-placed = ຍ້າຍໜ້າວຽກແລ້ວ
tasks-toast-rescheduled = ປ່ຽນເວລາໜ້າວຽກແລ້ວ
tasks-toast-rescheduled-several = { $count ->
   *[other] ປ່ຽນເວລາ { $count } ໜ້າວຽກແລ້ວ
}
tasks-toast-done-several = { $count ->
   *[other] { $count } ໜ້າວຽກສຳເລັດແລ້ວ
}
tasks-toast-open-several = { $count ->
   *[other] ໝາຍ { $count } ໜ້າວຽກວ່າຍັງບໍ່ສຳເລັດແລ້ວ
}
tasks-toast-starred = { $count ->
   *[other] ຕິດດາວ { $count } ໜ້າວຽກແລ້ວ
}
tasks-toast-unstarred = { $count ->
   *[other] ເອົາດາວອອກຈາກ { $count } ໜ້າວຽກແລ້ວ
}
tasks-toast-deleted-several = { $count ->
   *[other] ລຶບ { $count } ໜ້າວຽກແລ້ວ
}
