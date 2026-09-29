# Katna Mail, Lao (ລາວ): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = ມື້ນີ້
calendar-today-tip = ໄປທີ່ມື້ນີ້
calendar-view-day = ມື້
calendar-view-week = ອາທິດ
calendar-view-month = ເດືອນ
calendar-view-year = ປີ
calendar-view-schedule = ຕາຕະລາງ
calendar-view-days =
    { $count ->
       *[other] { $count } ມື້
    }
calendar-options = ທາງເລືອກ
calendar-density = ຄວາມໜາແໜ້ນ
calendar-density-responsive = ປັບຕາມໜ້າຈໍຂອງທ່ານ
calendar-density-comfortable = ສະບາຍ
calendar-density-compact = ກະທັດຮັດ
calendar-custom-days = ມຸມມອງກຳນົດເອງ
calendar-second-zone = ເຂດເວລາທີສອງ
calendar-zone-none = ບໍ່ມີ
calendar-zone = { $zone } ({ $offset })
calendar-share-free = ແບ່ງປັນເວລາຫວ່າງ
calendar-free-subject = ເວລາທີ່ຂ້ອຍຫວ່າງ
calendar-free-intro = ນີ້ແມ່ນເວລາທີ່ຂ້ອຍຫວ່າງ ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = ຂ້ອຍບໍ່ມີເວລາຫວ່າງໃນສອງສາມວັນເຮັດວຽກຂ້າງໜ້າ.
calendar-previous-day = ມື້ກ່ອນ
calendar-next-day = ມື້ຕໍ່ໄປ
calendar-previous-week = ອາທິດກ່ອນ
calendar-next-week = ອາທິດຕໍ່ໄປ
calendar-previous-month = ເດືອນກ່ອນ
calendar-next-month = ເດືອນຕໍ່ໄປ
calendar-previous-year = ປີກ່ອນ
calendar-next-year = ປີຕໍ່ໄປ
calendar-previous-period = ກ່ອນໜ້າ
calendar-next-period = ຕໍ່ໄປ
calendar-title-months = { $first } – { $last }
calendar-loading = ກຳລັງໂຫຼດ…
calendar-read-failed = ບໍ່ສາມາດອ່ານປະຕິທິນໄດ້: { $error }
calendar-sets = ຊຸດປະຕິທິນ
calendar-set-add = ບັນທຶກປະຕິທິນທີ່ກຳລັງສະແດງເປັນຊຸດ
calendar-set-name = ຊື່ຂອງຊຸດ
calendar-set-remove = ລຶບຊຸດອອກ
calendar-local = ຄອມພິວເຕີເຄື່ອງນີ້
calendar-account-gone = ບັນຊີທີ່ຖືກລຶບອອກແລ້ວ
calendar-birthdays = ວັນເກີດ
calendar-birthday-of = ວັນເກີດຂອງ { $name }
calendar-empty-title = ຍັງບໍ່ມີປະຕິທິນ
calendar-empty-text = ປະຕິທິນຂອງບັນຊີ Google ແລະ Microsoft ຂອງທ່ານຈະສະແດງຢູ່ບ່ອນນີ້ເມື່ອຊິງຄ໌ແລ້ວ ພ້ອມທັງປະຕິທິນຈາກເຊີບເວີອື່ນທີ່ຮອງຮັບ CalDAV.
calendar-schedule-empty = ບໍ່ມີແຜນໃນ 2 ເດືອນຂ້າງໜ້າ.
calendar-no-title = (ບໍ່ມີຫົວຂໍ້)
calendar-all-day = ທັງມື້
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = ອີກ { $count } ລາຍການ
calendar-repeats = ເກີດຊ້ຳ
calendar-join = ເຂົ້າຮ່ວມ
calendar-email-guests = ສົ່ງອີເມວຫາແຂກ
calendar-running-late = ຈະໄປຊ້າ
calendar-late-subject = ຈະໄປຊ້າ: { $title }
calendar-late-body = ຂໍໂທດ, ຂ້ອຍຈະໄປຮອດ { $title } ຊ້າສອງສາມນາທີ. ຈະໄປເຖິງໄວໆນີ້.
calendar-guests =
    { $count ->
       *[other] ແຂກ { $count } ຄົນ
    }
calendar-guest-answers = ຕອບຮັບ { $yes }, ອາດຈະ { $maybe }, ປະຕິເສດ { $no }, ລໍຖ້າ { $waiting }
calendar-organizer = ຜູ້ຈັດ
calendar-optional = ທາງເລືອກ
calendar-open-web = ເປີດໃນບຣາວເຊີ
calendar-open-contact = ເປີດລາຍຊື່ຜູ້ຕິດຕໍ່
calendar-close = ປິດ

## Adding, changing and deleting events.

calendar-add-title = ເພີ່ມຫົວຂໍ້
calendar-add-location = ເພີ່ມສະຖານທີ່
calendar-add-notes = ເພີ່ມຄຳອະທິບາຍ
calendar-add-guests = ເພີ່ມແຂກ
calendar-remove-guest = ລຶບອອກ
calendar-add-meet = ເພີ່ມການປະຊຸມວິດີໂອ Google Meet
calendar-add-teams = ເພີ່ມການປະຊຸມ Teams
calendar-has-call = ເພີ່ມການໂທວິດີໂອແລ້ວ
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = ທັງມື້
calendar-more-options = ຕົວເລືອກເພີ່ມເຕີມ
calendar-save = ບັນທຶກ
calendar-saved = ບັນທຶກເຫດການແລ້ວ
calendar-deleted = ລຶບເຫດການແລ້ວ
calendar-discard = ຖິ້ມການປ່ຽນແປງ
calendar-edit = ແກ້ໄຂເຫດການ
calendar-delete = ລຶບເຫດການ
calendar-event-details = ລາຍລະອຽດເຫດການ
calendar-kind-event = ເຫດການ
calendar-kind-focus = ເວລາໂຟກັສ
calendar-kind-out-of-office = ບໍ່ຢູ່ຫ້ອງການ
calendar-kind-working-location = ສະຖານທີ່ເຮັດວຽກ
calendar-working-home = ເຮືອນ
calendar-busy = ບໍ່ຫວ່າງ
calendar-free = ຫວ່າງ
calendar-cancel = ຍົກເລີກ
calendar-ok = ຕົກລົງ
calendar-read-only = ເຈົ້າບໍ່ສາມາດປ່ຽນແປງເຫດການໃນປະຕິທິນນີ້ໄດ້
calendar-none-editable = ຍັງບໍ່ມີປະຕິທິນທີ່ເຈົ້າສາມາດເພີ່ມເຫດການໄດ້
calendar-no-such-time = ບໍ່ມີເວລານັ້ນໃນເຂດເວລາຂອງເຈົ້າ
calendar-end-before-start = ເຫດການສິ້ນສຸດກ່ອນທີ່ຈະເລີ່ມ
calendar-repeat-never = ບໍ່ເກີດຊ້ຳ
calendar-repeat-daily = ທຸກມື້
calendar-repeat-weekly = ທຸກອາທິດໃນ { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] ທຸກເດືອນໃນ { $weekday } ທຳອິດ
        [2] ທຸກເດືອນໃນ { $weekday } ທີສອງ
        [3] ທຸກເດືອນໃນ { $weekday } ທີສາມ
        [4] ທຸກເດືອນໃນ { $weekday } ທີສີ່
       *[other] ທຸກເດືອນໃນ { $weekday } ສຸດທ້າຍ
    }
calendar-repeat-yearly = ທຸກປີໃນ { $day }
calendar-repeat-weekdays = ທຸກວັນເຮັດວຽກ (ຈັນຫາສຸກ)
calendar-repeat-custom = ກຳນົດເອງ
calendar-reminder-none = ບໍ່ມີການແຈ້ງເຕືອນ
calendar-reminder-at-start = ເມື່ອເລີ່ມ
calendar-reminder-minutes =
    { $count ->
       *[other] ກ່ອນ { $count } ນາທີ
    }
calendar-reminder-hours =
    { $count ->
       *[other] ກ່ອນ { $count } ຊົ່ວໂມງ
    }
calendar-reminder-days =
    { $count ->
       *[other] ກ່ອນ { $count } ມື້
    }
calendar-scope-edit-title = ແກ້ໄຂເຫດການທີ່ເກີດຊ້ຳ
calendar-scope-delete-title = ລຶບເຫດການທີ່ເກີດຊ້ຳ
calendar-scope-this = ເຫດການນີ້
calendar-scope-following = ເຫດການນີ້ ແລະ ເຫດການຕໍ່ໄປ
calendar-scope-all = ທຸກເຫດການ
calendar-scope-respond-title = ຕອບເຫດການທີ່ເກີດຊ້ຳ
calendar-going = ຈະໄປບໍ?
calendar-answer-yes = ແມ່ນ
calendar-answer-no = ບໍ່
calendar-answer-maybe = ອາດຈະ
calendar-answered-yes = ເຈົ້າຈະໄປ
calendar-answered-no = ເຈົ້າຈະບໍ່ໄປ
calendar-answered-maybe = ເຈົ້າອາດຈະໄປ

## The card at the top of a mail with an invitation.

calendar-invite = ຄຳເຊີນ
calendar-invite-cancelled = ຍົກເລີກເຫດການແລ້ວ
calendar-invite-reply = { $name } ຕອບກັບແລ້ວ
calendar-invite-reply-yes = { $name } ຕອບຮັບແລ້ວ
calendar-invite-reply-no = { $name } ປະຕິເສດແລ້ວ
calendar-invite-reply-maybe = { $name } ອາດຈະໄປ
calendar-invite-organizer = ຈັດໂດຍ { $name }
calendar-invite-open = ເປີດໃນປະຕິທິນ
calendar-invite-not-yet = ຍັງບໍ່ຢູ່ໃນປະຕິທິນຂອງເຈົ້າ. ຕອບໄດ້ເມື່ອຊິ້ງຂໍ້ມູນແລ້ວ.
calendar-invite-by-mail = ບໍ່ຢູ່ໃນປະຕິທິນຂອງເຈົ້າ: ຄຳຕອບຂອງເຈົ້າຈະສົ່ງຫາຜູ້ຈັດທາງອີເມວ.
calendar-mail-yes = ຕອບຮັບແລ້ວ: { $title }
calendar-mail-yes-body = { $name } ຕອບຮັບຄຳເຊີນນີ້ແລ້ວ.
calendar-mail-no = ປະຕິເສດແລ້ວ: { $title }
calendar-mail-no-body = { $name } ປະຕິເສດຄຳເຊີນນີ້ແລ້ວ.
calendar-mail-maybe = ອາດຈະໄປ: { $title }
calendar-mail-maybe-body = { $name } ຕອບຮັບຄຳເຊີນນີ້ແບບຍັງບໍ່ແນ່ໃຈ.
calendar-invite-your-day = ມື້ຂອງເຈົ້າ
calendar-invite-clashes =
    { $count ->
       *[other] ຊ້ອນກັບ { $count } ເຫດການ
    }

## The day's agenda beside the mail.

agenda-show = ສະແດງຕາຕະລາງຂອງມື້
agenda-hide = ເຊື່ອງຕາຕະລາງ
agenda-today = ມື້ນີ້, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = ບໍ່ມີແຜນໃນມື້ນີ້.
