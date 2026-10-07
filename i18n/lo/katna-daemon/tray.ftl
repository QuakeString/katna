# Katna Mail, Lao (ລາວ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = _ເປີດກ່ອງຈົດໝາຍເຂົ້າ
tray-new-message = _ຂໍ້ຄວາມໃໝ່
tray-new-task = _ໜ້າວຽກໃໝ່
tray-new-note = _ບັນທຶກໃໝ່
tray-preferences = _ການຕັ້ງຄ່າ
tray-quit = _ອອກ

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] ບໍ່ມີອີເມວທີ່ຍັງບໍ່ໄດ້ອ່ານ
   *[other] ຂໍ້ຄວາມທີ່ຍັງບໍ່ໄດ້ອ່ານ { $count } ສະບັບ
}
tray-password-refused = ຕ້ອງການລະຫັດຜ່ານໃໝ່ສຳລັບ { $address }
tray-signed-out = ເຂົ້າສູ່ລະບົບ { $address } ອີກຄັ້ງ
tray-accounts-need-you = { $count } ບັນຊີຕ້ອງການທ່ານ
tray-not-sent = { $count ->
   *[other] { $count } ຂໍ້ຄວາມບໍ່ໄດ້ຖືກສົ່ງ
}
