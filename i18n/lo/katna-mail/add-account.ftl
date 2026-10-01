# Katna Mail, Lao (ລາວ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = ເພີ່ມບັນຊີອີເມວ
add-account-looking = ກຳລັງຊອກຫາເຊີບເວີອີເມວຂອງ { $address }…
add-account-address-intro = ປ້ອນທີ່ຢູ່ອີເມວຂອງທ່ານ. Katna ຈະຊອກຫາເຊີບເວີໃຫ້ທ່ານ.
add-account-servers-title = ການຕັ້ງຄ່າເຊີບເວີ
add-account-servers-intro = ບ່ອນທີ່ Katna ອ່ານ ແລະ ສົ່ງອີເມວສຳລັບ { $address }.
add-account-signing-in = ກຳລັງເຂົ້າສູ່ລະບົບ…
add-account-browser-title = ສືບຕໍ່ໃນບຣາວເຊີຂອງທ່ານ
add-account-browser-intro = Katna ໄດ້ເປີດໜ້າເຂົ້າສູ່ລະບົບຂອງ { $provider } ໃນບຣາວເຊີຂອງທ່ານແລ້ວ. ເຂົ້າສູ່ລະບົບຢູ່ທີ່ນັ້ນ ແລະ ອະນຸຍາດໃຫ້ Katna ອ່ານ ແລະ ສົ່ງອີເມວຂອງທ່ານ, ແລ້ວກັບມາທີ່ນີ້.
add-account-browser-hint = ບໍ່ມີໜ້າໃດເປີດຂຶ້ນບໍ? ກວດເບິ່ງໜ້າຕ່າງຂອງບຣາວເຊີຂອງທ່ານ, ຫຼື ກັບຄືນ ແລະ ລອງໃໝ່.

## Add a mail account: fields

add-account-field-address = ທີ່ຢູ່ອີເມວ
add-account-incoming = ອີເມວຂາເຂົ້າ ({ $protocol })
add-account-outgoing = ອີເມວຂາອອກ ({ $protocol })
add-account-field-server = ເຊີບເວີ
add-account-field-port = ພອດ
add-account-security-none = ບໍ່ມີ
add-account-security-none-warning = ບໍ່ໄດ້ເຂົ້າລະຫັດ: ລະຫັດຜ່ານ ແລະ ອີເມວຂອງທ່ານອາດຖືກອ່ານໄດ້ລະຫວ່າງທາງ.
add-account-field-username = ຊື່ຜູ້ໃຊ້
add-account-field-password = ລະຫັດຜ່ານ
add-account-show-password = ສະແດງລະຫັດຜ່ານ
add-account-app-password-hint = { $provider } ຕ້ອງການລະຫັດຜ່ານແອັບຢູ່ທີ່ນີ້, ບໍ່ແມ່ນລະຫັດທີ່ທ່ານໃຊ້ໃນເວັບ. ສ້າງລະຫັດໜຶ່ງໃນການຕັ້ງຄ່າຄວາມປອດໄພຂອງບັນຊີ { $provider } ຂອງທ່ານ.
add-account-field-name = ຊື່ຂອງທ່ານ (ບໍ່ບັງຄັບ)
add-account-name-hint = ສະແດງໃຫ້ຄົນທີ່ທ່ານຂຽນຫາເຫັນ.
add-account-servers-pair = { $imap } ແລະ { $smtp }
add-account-servers-found = { $source ->
    [built-in] ເຊີບເວີ: { $servers }, ພົບໃນລາຍຊື່ຜູ້ໃຫ້ບໍລິການຂອງ Katna.
    [provider] ເຊີບເວີ: { $servers }, ພົບໃນການຕັ້ງຄ່າຂອງຜູ້ໃຫ້ບໍລິການຂອງທ່ານ.
    [ispdb] ເຊີບເວີ: { $servers }, ພົບໃນລາຍຊື່ຜູ້ໃຫ້ບໍລິການຂອງ Thunderbird.
    [dns] ເຊີບເວີ: { $servers }, ພົບໃນບັນທຶກ DNS ຂອງໂດເມນທ່ານ.
   *[other] ເຊີບເວີ: { $servers }, ຈາກການຄາດເດົາ; ກວດສອບຖ້າເຂົ້າສູ່ລະບົບບໍ່ສຳເລັດ.
}
add-account-servers-entered = ເຊີບເວີ: { $servers }, ຕາມທີ່ປ້ອນ.
add-account-sign-in-with = ເຂົ້າສູ່ລະບົບດ້ວຍ { $provider }
add-account-sign-in-instead = ເຂົ້າສູ່ລະບົບດ້ວຍ { $provider } ແທນ

## Add a mail account: buttons

add-account-servers-button = ການຕັ້ງຄ່າເຊີບເວີ
add-account-back = ກັບຄືນ
add-account-add = ເພີ່ມບັນຊີ
add-account-cancel = ຍົກເລີກ

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] ປ້ອນເຊີບເວີຂາເຂົ້າ.
   *[outgoing] ປ້ອນເຊີບເວີຂາອອກ.
}
add-account-server-space = { $kind ->
    [incoming] ຊື່ເຊີບເວີຂາເຂົ້າມີຍະຫວ່າງ.
   *[outgoing] ຊື່ເຊີບເວີຂາອອກມີຍະຫວ່າງ.
}
add-account-port-invalid = { $kind ->
    [incoming] ພອດຂາເຂົ້າຕ້ອງເປັນຕົວເລກຈາກ { $min } ຫາ { $max }.
   *[outgoing] ພອດຂາອອກຕ້ອງເປັນຕົວເລກຈາກ { $min } ຫາ { $max }.
}
add-account-address-empty = ປ້ອນທີ່ຢູ່ອີເມວ.
add-account-address-invalid = ປ້ອນທີ່ຢູ່ອີເມວເຊັ່ນ { $example }.
add-account-not-found = Katna ຊອກບໍ່ພົບເຊີບເວີຂອງ { $address }, ສະນັ້ນຈຶ່ງໃສ່ຊື່ທີ່ໃຊ້ທົ່ວໄປໄວ້. ກວດສອບກັບຜູ້ໃຫ້ບໍລິການຂອງທ່ານ.
add-account-password-empty = ປ້ອນລະຫັດຜ່ານ.
add-account-name-is-password = ຊື່ຄືກັນກັບລະຫັດຜ່ານ. ໃຫ້ພິມຊື່ຂອງທ່ານໃສ່ບ່ອນນັ້ນແທນ, ຕາມທີ່ຄົນອື່ນຄວນເຫັນ.
add-account-app-password-refused = { $provider } ປະຕິເສດລະຫັດຜ່ານ. ມັນຕ້ອງການລະຫັດຜ່ານແອັບ, ບໍ່ແມ່ນລະຫັດທີ່ທ່ານໃຊ້ໃນເວັບ.
add-account-password-refused = ເຊີບເວີປະຕິເສດລະຫັດຜ່ານ. ກວດສອບແລ້ວລອງອີກຄັ້ງ.
add-account-sign-in-refused = { $provider } ບໍ່ໃຫ້ Katna ເຂົ້າ. ລອງໃໝ່ ແລະ ອະນຸຍາດໃຫ້ເຂົ້າເຖິງອີເມວຂອງທ່ານ.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Katna ສະບັບນີ້ຍັງບໍ່ສາມາດເຂົ້າສູ່ລະບົບບັນຊີ Microsoft ໄດ້ເທື່ອ.
    [Google] Katna ສະບັບນີ້ຍັງບໍ່ສາມາດເຂົ້າສູ່ລະບົບບັນຊີ Google ໄດ້ເທື່ອ.
   *[other] ຜູ້ໃຫ້ບໍລິການນີ້ອະນຸຍາດໃຫ້ເຂົ້າສູ່ລະບົບໄດ້ສະເພາະໃນໜ້າຂອງຕົນເອງ, ເຊິ່ງ Katna ຍັງເຮັດໃຫ້ມັນບໍ່ໄດ້ເທື່ອ.
}

## The account menu (from the account button on the top bar)

add-account-menu-another = ເພີ່ມບັນຊີອື່ນ
app-menu = ເມນູຫຼັກ
app-menu-back = ກັບຄືນ
