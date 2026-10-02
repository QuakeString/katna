# Katna Mail, Lao (ລາວ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = ຫຼັກ
tab-promotions = ໂປຣໂມຊັນ
tab-social = ສັງຄົມ
tab-updates = ອັບເດດ
tab-forums = ຟໍຣັມ
tab-focused = ໂຟກັສ
tab-other = ອື່ນໆ
tab-inbox = ກ່ອງຈົດໝາຍເຂົ້າ
tab-newsletters = ຈົດໝາຍຂ່າວ
tab-notifications = ການແຈ້ງເຕືອນ
tab-provider-other = ຈັດຮຽງໂດຍ Katna

## Mail list: toolbar

list-select = ເລືອກ
list-refresh = ໂຫຼດຄືນໃໝ່
list-back-to-top = ກັບໄປເທິງສຸດ
list-checking = ກຳລັງກວດຫາອີເມວໃໝ່…
list-more = ເພີ່ມເຕີມ
list-mark-read = ໝາຍວ່າອ່ານແລ້ວ
list-mark-unread = ໝາຍວ່າຍັງບໍ່ໄດ້ອ່ານ
list-move-to = ຍ້າຍໄປທີ່
list-archive = ຈັດເກັບ
list-spam = ລາຍງານສະແປມ
list-delete = ລຶບ
list-snooze = ເລື່ອນເວລາ
list-unsnooze = ຍົກເລີກການເລື່ອນເວລາ
list-newer = ໃໝ່ກວ່າ
list-older = ເກົ່າກວ່າ
list-range = { $first }–{ $last } ຈາກ { $total }
list-range-about = { $first }–{ $last } ຈາກປະມານ { $total }
list-results = ຜົນການຊອກຫາ “{ $query }”
list-results-corrected = ກຳລັງສະແດງຜົນການຊອກຫາ “{ $query }”
list-search-instead = ຊອກຫາ “{ $query }” ແທນ
list-files-more = +{ $count }
list-replied = ທ່ານຕອບກັບແລ້ວ

## Mail list: Select menu (which lines to tick)

list-pick-all = ທັງໝົດ
list-pick-none = ບໍ່ເລືອກ
list-pick-read = ອ່ານແລ້ວ
list-pick-unread = ຍັງບໍ່ໄດ້ອ່ານ
list-pick-starred = ຕິດດາວແລ້ວ
list-pick-unstarred = ບໍ່ໄດ້ຕິດດາວ

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] ເລືອກການສົນທະນາທັງໝົດ { $count } ລາຍການແລ້ວ.
   *[message] ເລືອກຂໍ້ຄວາມທັງໝົດ { $count } ລາຍການແລ້ວ.
}
list-selected-all-in = { $kind ->
    [conversation] ເລືອກການສົນທະນາທັງໝົດ { $count } ລາຍການໃນ { $folder } ແລ້ວ.
   *[message] ເລືອກຂໍ້ຄວາມທັງໝົດ { $count } ລາຍການໃນ { $folder } ແລ້ວ.
}
list-selected-screen = { $kind ->
    [conversation] ເລືອກການສົນທະນາທັງໝົດ { $count } ລາຍການໃນໜ້ານີ້ແລ້ວ.
   *[message] ເລືອກຂໍ້ຄວາມທັງໝົດ { $count } ລາຍການໃນໜ້ານີ້ແລ້ວ.
}
list-select-all = { $kind ->
    [conversation] ເລືອກການສົນທະນາທັງໝົດ { $count } ລາຍການ
   *[message] ເລືອກຂໍ້ຄວາມທັງໝົດ { $count } ລາຍການ
}
list-select-all-in = { $kind ->
    [conversation] ເລືອກການສົນທະນາທັງໝົດ { $count } ລາຍການໃນ { $folder }
   *[message] ເລືອກຂໍ້ຄວາມທັງໝົດ { $count } ລາຍການໃນ { $folder }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ອ່ານແລ້ວທັງໝົດ { $count } ລາຍການໃນໜ້ານີ້ແລ້ວ.
       *[message] ເລືອກຂໍ້ຄວາມທີ່ອ່ານແລ້ວທັງໝົດ { $count } ລາຍການໃນໜ້ານີ້ແລ້ວ.
    }
   *[unread] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ຍັງບໍ່ໄດ້ອ່ານທັງໝົດ { $count } ລາຍການໃນໜ້ານີ້ແລ້ວ.
       *[message] ເລືອກຂໍ້ຄວາມທີ່ຍັງບໍ່ໄດ້ອ່ານທັງໝົດ { $count } ລາຍການໃນໜ້ານີ້ແລ້ວ.
    }
    [starred] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ຕິດດາວແລ້ວທັງໝົດ { $count } ລາຍການໃນໜ້ານີ້ແລ້ວ.
       *[message] ເລືອກຂໍ້ຄວາມທີ່ຕິດດາວແລ້ວທັງໝົດ { $count } ລາຍການໃນໜ້ານີ້ແລ້ວ.
    }
    [unstarred] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ບໍ່ໄດ້ຕິດດາວທັງໝົດ { $count } ລາຍການໃນໜ້ານີ້ແລ້ວ.
       *[message] ເລືອກຂໍ້ຄວາມທີ່ບໍ່ໄດ້ຕິດດາວທັງໝົດ { $count } ລາຍການໃນໜ້ານີ້ແລ້ວ.
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ອ່ານແລ້ວທັງໝົດ { $count } ລາຍການ
       *[message] ເລືອກຂໍ້ຄວາມທີ່ອ່ານແລ້ວທັງໝົດ { $count } ລາຍການ
    }
   *[unread] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ຍັງບໍ່ໄດ້ອ່ານທັງໝົດ { $count } ລາຍການ
       *[message] ເລືອກຂໍ້ຄວາມທີ່ຍັງບໍ່ໄດ້ອ່ານທັງໝົດ { $count } ລາຍການ
    }
    [starred] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ຕິດດາວແລ້ວທັງໝົດ { $count } ລາຍການ
       *[message] ເລືອກຂໍ້ຄວາມທີ່ຕິດດາວແລ້ວທັງໝົດ { $count } ລາຍການ
    }
    [unstarred] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ບໍ່ໄດ້ຕິດດາວທັງໝົດ { $count } ລາຍການ
       *[message] ເລືອກຂໍ້ຄວາມທີ່ບໍ່ໄດ້ຕິດດາວທັງໝົດ { $count } ລາຍການ
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ອ່ານແລ້ວທັງໝົດ { $count } ລາຍການໃນ { $folder }
       *[message] ເລືອກຂໍ້ຄວາມທີ່ອ່ານແລ້ວທັງໝົດ { $count } ລາຍການໃນ { $folder }
    }
   *[unread] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ຍັງບໍ່ໄດ້ອ່ານທັງໝົດ { $count } ລາຍການໃນ { $folder }
       *[message] ເລືອກຂໍ້ຄວາມທີ່ຍັງບໍ່ໄດ້ອ່ານທັງໝົດ { $count } ລາຍການໃນ { $folder }
    }
    [starred] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ຕິດດາວແລ້ວທັງໝົດ { $count } ລາຍການໃນ { $folder }
       *[message] ເລືອກຂໍ້ຄວາມທີ່ຕິດດາວແລ້ວທັງໝົດ { $count } ລາຍການໃນ { $folder }
    }
    [unstarred] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ບໍ່ໄດ້ຕິດດາວທັງໝົດ { $count } ລາຍການໃນ { $folder }
       *[message] ເລືອກຂໍ້ຄວາມທີ່ບໍ່ໄດ້ຕິດດາວທັງໝົດ { $count } ລາຍການໃນ { $folder }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ອ່ານແລ້ວທັງໝົດ { $count } ລາຍການແລ້ວ.
       *[message] ເລືອກຂໍ້ຄວາມທີ່ອ່ານແລ້ວທັງໝົດ { $count } ລາຍການແລ້ວ.
    }
   *[unread] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ຍັງບໍ່ໄດ້ອ່ານທັງໝົດ { $count } ລາຍການແລ້ວ.
       *[message] ເລືອກຂໍ້ຄວາມທີ່ຍັງບໍ່ໄດ້ອ່ານທັງໝົດ { $count } ລາຍການແລ້ວ.
    }
    [starred] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ຕິດດາວແລ້ວທັງໝົດ { $count } ລາຍການແລ້ວ.
       *[message] ເລືອກຂໍ້ຄວາມທີ່ຕິດດາວແລ້ວທັງໝົດ { $count } ລາຍການແລ້ວ.
    }
    [unstarred] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ບໍ່ໄດ້ຕິດດາວທັງໝົດ { $count } ລາຍການແລ້ວ.
       *[message] ເລືອກຂໍ້ຄວາມທີ່ບໍ່ໄດ້ຕິດດາວທັງໝົດ { $count } ລາຍການແລ້ວ.
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ອ່ານແລ້ວທັງໝົດ { $count } ລາຍການໃນ { $folder } ແລ້ວ.
       *[message] ເລືອກຂໍ້ຄວາມທີ່ອ່ານແລ້ວທັງໝົດ { $count } ລາຍການໃນ { $folder } ແລ້ວ.
    }
   *[unread] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ຍັງບໍ່ໄດ້ອ່ານທັງໝົດ { $count } ລາຍການໃນ { $folder } ແລ້ວ.
       *[message] ເລືອກຂໍ້ຄວາມທີ່ຍັງບໍ່ໄດ້ອ່ານທັງໝົດ { $count } ລາຍການໃນ { $folder } ແລ້ວ.
    }
    [starred] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ຕິດດາວແລ້ວທັງໝົດ { $count } ລາຍການໃນ { $folder } ແລ້ວ.
       *[message] ເລືອກຂໍ້ຄວາມທີ່ຕິດດາວແລ້ວທັງໝົດ { $count } ລາຍການໃນ { $folder } ແລ້ວ.
    }
    [unstarred] { $kind ->
        [conversation] ເລືອກການສົນທະນາທີ່ບໍ່ໄດ້ຕິດດາວທັງໝົດ { $count } ລາຍການໃນ { $folder } ແລ້ວ.
       *[message] ເລືອກຂໍ້ຄວາມທີ່ບໍ່ໄດ້ຕິດດາວທັງໝົດ { $count } ລາຍການໃນ { $folder } ແລ້ວ.
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] ບໍ່ມີການສົນທະນາທີ່ອ່ານແລ້ວຢູ່ບ່ອນນີ້.
       *[message] ບໍ່ມີຂໍ້ຄວາມທີ່ອ່ານແລ້ວຢູ່ບ່ອນນີ້.
    }
   *[unread] { $kind ->
        [conversation] ບໍ່ມີການສົນທະນາທີ່ຍັງບໍ່ໄດ້ອ່ານຢູ່ບ່ອນນີ້.
       *[message] ບໍ່ມີຂໍ້ຄວາມທີ່ຍັງບໍ່ໄດ້ອ່ານຢູ່ບ່ອນນີ້.
    }
    [starred] { $kind ->
        [conversation] ບໍ່ມີການສົນທະນາທີ່ຕິດດາວແລ້ວຢູ່ບ່ອນນີ້.
       *[message] ບໍ່ມີຂໍ້ຄວາມທີ່ຕິດດາວແລ້ວຢູ່ບ່ອນນີ້.
    }
    [unstarred] { $kind ->
        [conversation] ບໍ່ມີການສົນທະນາທີ່ບໍ່ໄດ້ຕິດດາວຢູ່ບ່ອນນີ້.
       *[message] ບໍ່ມີຂໍ້ຄວາມທີ່ບໍ່ໄດ້ຕິດດາວຢູ່ບ່ອນນີ້.
    }
}
list-clear-selection = ລຶບການເລືອກ

## Mail list: empty states

list-empty-search = ບໍ່ມີຂໍ້ຄວາມທີ່ກົງກັບການຊອກຫາຂອງທ່ານ.
list-empty-tab = ບໍ່ມີອີເມວໃນ { $tab }.
list-empty-tab-unknown = ບໍ່ມີອີເມວໃນແຖບນີ້.
list-empty-folder = ບໍ່ມີຂໍ້ຄວາມໃນ { $folder }.
list-empty-folder-unknown = ບໍ່ມີຂໍ້ຄວາມໃນໂຟນເດີນີ້.
list-first-sync = ກຳລັງດຶງອີເມວຂອງທ່ານ…
list-first-sync-detail = ອີເມວຈະສະແດງຢູ່ບ່ອນນີ້ເມື່ອມາຮອດ.

## Mail list: lines

row-removed = ຂໍ້ຄວາມນີ້ຖືກລຶບອອກແລ້ວ.
row-starred = ຕິດດາວແລ້ວ
row-not-starred = ບໍ່ໄດ້ຕິດດາວ
row-important = ສຳຄັນ. ຄລິກເພື່ອໝາຍວ່າບໍ່ສຳຄັນ.
row-mark-important = ໝາຍວ່າສຳຄັນ
row-pinned = ປັກໝຸດໄວ້ເທິງສຸດແລ້ວ
row-task = ໜ້າວຽກ
row-task-open = ເປີດໜ້າວຽກ: { $title }
row-tracking-none = ຕິດຕາມຢູ່. ຍັງບໍ່ໄດ້ເປີດ
row-tracking-opened = ເປີດໂດຍ { $opened } ຈາກ { $recipients } ຄົນ
row-tracking-clicked = ເປີດໂດຍ { $opened } ຈາກ { $recipients } ຄົນ, ຄລິກລິ້ງໂດຍ { $clicked } ຄົນ
row-pin = ປັກໝຸດໄວ້ເທິງສຸດ
row-unpin = ຖອນປັກໝຸດ
row-snoozed-until = ເລື່ອນເວລາຈົນຮອດ { $when }

## Mail list: More menu and right-click menu

menu-reply = ຕອບກັບ
menu-reply-all = ຕອບກັບທັງໝົດ
menu-forward = ສົ່ງຕໍ່
menu-archive = ຈັດເກັບ
menu-delete = ລຶບ
menu-delete-forever = ລຶບຖາວອນ
menu-move-to-inbox = ຍ້າຍໄປທີ່ກ່ອງຈົດໝາຍເຂົ້າ
menu-spam = ລາຍງານສະແປມ
menu-not-spam = ບໍ່ແມ່ນສະແປມ
menu-mark-read = ໝາຍວ່າອ່ານແລ້ວ
menu-mark-unread = ໝາຍວ່າຍັງບໍ່ໄດ້ອ່ານ
menu-mark-all-read = ໝາຍທັງໝົດວ່າອ່ານແລ້ວ
menu-star = ເພີ່ມດາວ
menu-unstar = ລຶບດາວອອກ
menu-important = ໝາຍວ່າສຳຄັນ
menu-not-important = ໝາຍວ່າບໍ່ສຳຄັນ
menu-pin = ປັກໝຸດໄວ້ເທິງສຸດ
menu-unpin = ຖອນປັກໝຸດ
menu-snooze = ເລື່ອນເວລາ
menu-unsnooze = ຍົກເລີກການເລື່ອນເວລາ
menu-add-to-tasks = ເພີ່ມໃສ່ວຽກ
menu-schedule-meeting = ນັດປະຊຸມ
menu-start-call = ເລີ່ມການປະຊຸມທາງວິດີໂອ
menu-add-note = ເພີ່ມບັນທຶກ
menu-print-all = ພິມທັງໝົດ
menu-new-window = ເປີດໃນໜ້າຕ່າງໃໝ່
menu-move-to = ຍ້າຍໄປທີ່
menu-follow-up = ຕິດຕາມ
menu-more = ເພີ່ມເຕີມ
menu-move-to-heading = ຍ້າຍໄປທີ່:
menu-find-from = ຊອກຫາອີເມວຈາກ { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] ຈັດເກັບການສົນທະນາ { $count } ລາຍການແລ້ວ.
   *[message] ຈັດເກັບຂໍ້ຄວາມ { $count } ລາຍການແລ້ວ.
}
toast-trashed = { $kind ->
    [conversation] ຍ້າຍການສົນທະນາ { $count } ລາຍການໄປໃສ່ຖັງຂີ້ເຫຍື້ອແລ້ວ.
   *[message] ຍ້າຍຂໍ້ຄວາມ { $count } ລາຍການໄປໃສ່ຖັງຂີ້ເຫຍື້ອແລ້ວ.
}
toast-moved = { $kind ->
    [conversation] ຍ້າຍການສົນທະນາ { $count } ລາຍການແລ້ວ.
   *[message] ຍ້າຍຂໍ້ຄວາມ { $count } ລາຍການແລ້ວ.
}
toast-starred = { $kind ->
    [conversation] ຕິດດາວໃຫ້ການສົນທະນາ { $count } ລາຍການແລ້ວ.
   *[message] ຕິດດາວໃຫ້ຂໍ້ຄວາມ { $count } ລາຍການແລ້ວ.
}
toast-unstarred = { $kind ->
    [conversation] ລຶບດາວອອກຈາກການສົນທະນາ { $count } ລາຍການແລ້ວ.
   *[message] ລຶບດາວອອກຈາກຂໍ້ຄວາມ { $count } ລາຍການແລ້ວ.
}
toast-important = { $kind ->
    [conversation] ໝາຍການສົນທະນາ { $count } ລາຍການວ່າສຳຄັນແລ້ວ.
   *[message] ໝາຍຂໍ້ຄວາມ { $count } ລາຍການວ່າສຳຄັນແລ້ວ.
}
toast-not-important = { $kind ->
    [conversation] ໝາຍການສົນທະນາ { $count } ລາຍການວ່າບໍ່ສຳຄັນແລ້ວ.
   *[message] ໝາຍຂໍ້ຄວາມ { $count } ລາຍການວ່າບໍ່ສຳຄັນແລ້ວ.
}
toast-pinned = { $kind ->
    [conversation] ປັກໝຸດການສົນທະນາ { $count } ລາຍການໄວ້ເທິງສຸດແລ້ວ.
   *[message] ປັກໝຸດຂໍ້ຄວາມ { $count } ລາຍການໄວ້ເທິງສຸດແລ້ວ.
}
toast-unpinned = { $kind ->
    [conversation] ຖອນປັກໝຸດການສົນທະນາ { $count } ລາຍການແລ້ວ.
   *[message] ຖອນປັກໝຸດຂໍ້ຄວາມ { $count } ລາຍການແລ້ວ.
}
toast-snoozed = { $kind ->
    [conversation] ເລື່ອນເວລາການສົນທະນາ { $count } ລາຍການຈົນຮອດ { $when } ແລ້ວ.
   *[message] ເລື່ອນເວລາຂໍ້ຄວາມ { $count } ລາຍການຈົນຮອດ { $when } ແລ້ວ.
}
toast-unsnoozed = { $kind ->
    [conversation] ການສົນທະນາ { $count } ລາຍການກັບມາຢູ່ກ່ອງຈົດໝາຍເຂົ້າແລ້ວ.
   *[message] ຂໍ້ຄວາມ { $count } ລາຍການກັບມາຢູ່ກ່ອງຈົດໝາຍເຂົ້າແລ້ວ.
}
toast-spam = { $kind ->
    [conversation] ລາຍງານການສົນທະນາ { $count } ລາຍການວ່າເປັນສະແປມແລ້ວ.
   *[message] ລາຍງານຂໍ້ຄວາມ { $count } ລາຍການວ່າເປັນສະແປມແລ້ວ.
}
toast-not-spam = { $kind ->
    [conversation] ໝາຍການສົນທະນາ { $count } ລາຍການວ່າບໍ່ແມ່ນສະແປມ ແລະ ຍ້າຍໄປກ່ອງຈົດໝາຍເຂົ້າແລ້ວ.
   *[message] ໝາຍຂໍ້ຄວາມ { $count } ລາຍການວ່າບໍ່ແມ່ນສະແປມ ແລະ ຍ້າຍໄປກ່ອງຈົດໝາຍເຂົ້າແລ້ວ.
}
toast-deleted-forever = { $kind ->
    [conversation] ລຶບການສົນທະນາ { $count } ລາຍການຖາວອນແລ້ວ.
   *[message] ລຶບຂໍ້ຄວາມ { $count } ລາຍການຖາວອນແລ້ວ.
}
toast-marked-read = { $kind ->
    [conversation] ໝາຍການສົນທະນາ { $count } ລາຍການວ່າອ່ານແລ້ວ.
   *[message] ໝາຍຂໍ້ຄວາມ { $count } ລາຍການວ່າອ່ານແລ້ວ.
}
toast-marked-unread = { $kind ->
    [conversation] ໝາຍການສົນທະນາ { $count } ລາຍການວ່າຍັງບໍ່ໄດ້ອ່ານແລ້ວ.
   *[message] ໝາຍຂໍ້ຄວາມ { $count } ລາຍການວ່າຍັງບໍ່ໄດ້ອ່ານແລ້ວ.
}
toast-undone = ຍ້ອນກັບການກະທຳແລ້ວ.
toast-nothing-to-undo = ບໍ່ມີຫຍັງໃຫ້ຍ້ອນກັບ.
toast-cannot-undo-delete-forever = ອີເມວທີ່ລຶບຖາວອນແລ້ວ ບໍ່ສາມາດກູ້ຄືນໄດ້.
toast-send-undone = ຍົກເລີກການສົ່ງແລ້ວ.
toast-too-late-to-undo-send = ສາຍເກີນໄປທີ່ຈະຍ້ອນກັບ: ຂໍ້ຄວາມຖືກສົ່ງໄປແລ້ວ.
toast-undo = ຍ້ອນກັບ
toast-close = ປິດ
toast-no-spam-folder = ບັນຊີນີ້ບໍ່ມີໂຟນເດີສະແປມ.
