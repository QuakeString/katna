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

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } ເປີດ { $subject } ແລ້ວ
notify-tracking-clicked = { $who } ຄລິກລິ້ງໃນ { $subject } ແລ້ວ

## An update of Katna is downloaded and ready to install

notify-update-ready = ອັບເດດ Katna Mail ໄດ້ແລ້ວ
notify-update-ready-body = ດາວໂຫຼດເວີຊັນ { $version } ແລ້ວ. ກົດອັບເດດເພື່ອຕິດຕັ້ງ ແລະ ເລີ່ມ Katna Mail ໃໝ່.
notify-update = ອັບເດດ

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
notify-reply-all = ຕອບກັບທັງໝົດ
notify-mark-read = ໝາຍວ່າອ່ານແລ້ວ
notify-mark-all-read = ໝາຍທັງໝົດວ່າອ່ານແລ້ວ
notify-archive = ຈັດເກັບ

## After Archive on a notification: a short note in the same place

notify-archived = ຈັດເກັບແລ້ວ
notify-archived-count = { $count ->
   *[other] ຍ້າຍ { $count } ຂໍ້ຄວາມອອກຈາກກ່ອງຈົດໝາຍເຂົ້າແລ້ວ
}
notify-undo = ຍ້ອນກັບ

## it waits for the undo time

notify-reply-sent = ສົ່ງຄຳຕອບຫາ { $name } ແລ້ວ
notify-open-in-katna = ເປີດໃນ Katna
