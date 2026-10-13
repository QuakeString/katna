# Katna Mail, Lao (ລາວ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = ອີເມວໃໝ່ { $count } ສະບັບ
notify-and-more = ແລະ ອີກ { $count }
notify-no-subject = (ບໍ່ມີຫົວຂໍ້)
notify-unknown-sender = ບໍ່ຮູ້ຈັກຜູ້ສົ່ງ

## Reminders the user asked for (same buttons)

notify-snooze-back = ກັບມາຈາກການເລື່ອນເວລາ
notify-no-reply = ຍັງບໍ່ມີການຕອບກັບ
notify-no-reply-to = ຍັງບໍ່ມີໃຜຕອບກັບ “{ $subject }”.
notify-follow-up-sent = ສົ່ງອີເມວຕິດຕາມແລ້ວ
notify-follow-up-sent-to = ບໍ່ມີໃຜຕອບກັບ “{ $subject }”, ສະນັ້ນ Katna ຈຶ່ງສົ່ງອີເມວຕິດຕາມແລ້ວ.
notify-follow-up-waiting = ບໍ່ໄດ້ສົ່ງອີເມວຕິດຕາມ
notify-follow-up-waiting-to = ມັນຮອດກຳນົດຂະນະທີ່ຄອມພິວເຕີເຄື່ອງນີ້ປິດຢູ່. “{ $subject }” ກັບມາຢູ່ໃນກ່ອງຈົດໝາຍເຂົ້າຂອງທ່ານແລ້ວ.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } ເປີດ { $subject } ແລ້ວ
notify-tracking-clicked = { $who } ຄລິກລິ້ງໃນ { $subject } ແລ້ວ

## An update of Katna is downloaded and ready to install

notify-update-ready = ອັບເດດ Katna Mail ໄດ້ແລ້ວ
notify-update-ready-body = ດາວໂຫຼດເວີຊັນ { $version } ແລ້ວ. ກົດອັບເດດເພື່ອຕິດຕັ້ງ ແລະ ເລີ່ມ Katna Mail ໃໝ່.
notify-update = ອັບເດດ

## Something needs the user, shown once per problem

notify-signed-out = ເຂົ້າສູ່ລະບົບອີກຄັ້ງ
notify-signed-out-body = { $provider } ອອກຈາກລະບົບ Katna ໃນ { $address } ແລ້ວ. ອີເມວຢຸດຊິງຄ໌.
notify-sign-in = ເຂົ້າສູ່ລະບົບ
notify-password-refused = ລະຫັດຜ່ານຖືກປະຕິເສດ
notify-password-refused-body = ເຊີບເວີອີເມວປະຕິເສດລະຫັດຜ່ານຂອງ { $address }. ມັນອາດຖືກປ່ຽນແລ້ວ.
notify-new-password = ລະຫັດຜ່ານໃໝ່
notify-not-sent = “{ $subject }” ບໍ່ໄດ້ຖືກສົ່ງ
notify-not-sent-no-subject = ມີຂໍ້ຄວາມໜຶ່ງບໍ່ໄດ້ຖືກສົ່ງ
notify-not-sent-body = ມັນຢູ່ໃນກ່ອງຈົດໝາຍອອກ, ເຊິ່ງບອກເຫດຜົນ.
notify-open-outbox = ເປີດກ່ອງຈົດໝາຍອອກ

## Reminders of calendar events

notify-event-now = ຕອນນີ້
notify-event-in-minutes = { $count ->
   *[other] ໃນອີກ { $count } ນາທີ
}
notify-event-in-hours = { $count ->
   *[other] ໃນອີກ { $count } ຊົ່ວໂມງ
}
notify-event-in-days = { $count ->
    [1] ມື້ອື່ນ
   *[other] ໃນອີກ { $count } ມື້
}
notify-event-all-day = ຕະຫຼອດມື້
notify-event-join = ເຂົ້າຮ່ວມ
notify-event-snooze = ເລື່ອນເວລາ 5 ນາທີ
notify-task-done = ໝາຍວ່າສຳເລັດ

## The buttons of new-mail notifications and reminders

notify-open = ເປີດ
notify-peek = ເບິ່ງໄວໆ
notify-reply = ຕອບກັບ
notify-reply-placeholder = ຕອບກັບ { $name }…
notify-send = ສົ່ງ
notify-reply-quote-header = ເມື່ອ { $date }, { $from } ໄດ້ຂຽນວ່າ:
notify-reply-quote-header-no-date = { $from } ໄດ້ຂຽນວ່າ:
notify-reply-all = ຕອບກັບທັງໝົດ
notify-mark-read = ໝາຍວ່າອ່ານແລ້ວ
notify-mark-all-read = ໝາຍທັງໝົດວ່າອ່ານແລ້ວ
notify-archive = ຈັດເກັບ
notify-snooze-hour = ເລື່ອນ 1 ຊົ່ວໂມງ
notify-snooze-tomorrow = ມື້ອື່ນ
notify-copy-code = ສຳເນົາ { $code }
notify-link-verify = ຢືນຢັນຕົວຕົນໃນ { $domain }
notify-link-confirm = ຢືນຢັນໃນ { $domain }
notify-link-activate = ເປີດໃຊ້ໃນ { $domain }

## After Archive on a notification: a short note in the same place

notify-archived = ຈັດເກັບແລ້ວ
notify-archived-count = { $count ->
   *[other] ຍ້າຍ { $count } ຂໍ້ຄວາມອອກຈາກກ່ອງຈົດໝາຍເຂົ້າແລ້ວ
}
notify-undo = ຍ້ອນກັບ

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = ສຳເນົາລະຫັດແລ້ວ
notify-code-not-copied = ສຳເນົາລະຫັດບໍ່ໄດ້

## it waits for the undo time

notify-reply-sent = ສົ່ງຄຳຕອບຫາ { $name } ແລ້ວ
notify-open-in-katna = ເປີດໃນ Katna
