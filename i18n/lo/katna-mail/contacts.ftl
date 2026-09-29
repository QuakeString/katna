# Katna Mail, Lao (ລາວ): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = ລາຍຊື່ຜູ້ຕິດຕໍ່
contacts-frequent = ຕິດຕໍ່ເລື້ອຍໆ
contacts-other = ລາຍຊື່ຜູ້ຕິດຕໍ່ອື່ນໆ
contacts-other-about = ຄົນທີ່ເຈົ້າເຄີຍສົ່ງອີເມວຫາຈາກ Gmail ແຕ່ບໍ່ໄດ້ບັນທຶກໄວ້
contacts-other-email = ສົ່ງອີເມວ
contacts-other-empty = ບໍ່ມີລາຍຊື່ຜູ້ຕິດຕໍ່ອື່ນໆ. ຄົນທີ່ເຈົ້າສົ່ງອີເມວຫາຈາກ Gmail ແຕ່ບໍ່ໄດ້ບັນທຶກໄວ້ຈະສະແດງຢູ່ບ່ອນນີ້.
contacts-other-allow = ເພື່ອເບິ່ງລາຍຊື່ຜູ້ຕິດຕໍ່ອື່ນໆ, ໃຫ້ເຂົ້າສູ່ລະບົບບັນຊີ Gmail ຂອງເຈົ້າອີກຄັ້ງ ແລ້ວອະນຸຍາດໃຫ້ Katna ເບິ່ງລາຍຊື່ເຫຼົ່ານັ້ນ.
contacts-labels = ປ້າຍກຳກັບ
contacts-label-options = ຕົວເລືອກປ້າຍກຳກັບ
contacts-label-rename = ປ່ຽນຊື່ປ້າຍກຳກັບ
contacts-label-email = ສົ່ງອີເມວຫາທຸກຄົນ
contacts-label-delete = ລຶບປ້າຍກຳກັບ
contacts-label-new = ປ້າຍກຳກັບໃໝ່
contacts-label-name = ຊື່ປ້າຍກຳກັບ
contacts-label-button = ປ້າຍກຳກັບ
contacts-label-menu = ໃສ່ປ້າຍກຳກັບເປັນ:
contacts-label-added = ເພີ່ມໃສ່ { $name } ແລ້ວ
contacts-label-removed = ເອົາອອກຈາກ { $name } ແລ້ວ
contacts-label-renamed = ປ່ຽນຊື່ປ້າຍກຳກັບເປັນ { $name } ແລ້ວ
contacts-label-deleted = ລຶບປ້າຍກຳກັບ { $name } ແລ້ວ
contacts-label-no-email = ບໍ່ມີໃຜໃນປ້າຍກຳກັບນີ້ທີ່ມີທີ່ຢູ່ອີເມວ
contacts-manage = ແກ້ໄຂ ແລະ ຈັດການ
contacts-merge = ລວມ ແລະ ແກ້ໄຂ
contacts-merge-about = { $count ->
   *[other] ຄຳແນະນຳ { $count } ລາຍການ: ລາຍຊື່ຜູ້ຕິດຕໍ່ທີ່ເບິ່ງຄືຄົນດຽວກັນ
}
contacts-merge-none = ບໍ່ມີລາຍການຊ້ຳກັນ. ລາຍຊື່ຜູ້ຕິດຕໍ່ທີ່ມີຊື່ ຫຼື ເບີໂທລະສັບດຽວກັນຈະສະແດງຢູ່ບ່ອນນີ້.
contacts-merge-count = { $count ->
   *[other] ລາຍຊື່ຜູ້ຕິດຕໍ່ { $count } ລາຍການ
}
contacts-merge-all = ລວມທັງໝົດ
contacts-merge-button = ລວມ
contacts-merge-dismiss = ປິດ
contacts-merged = { $count ->
    [1] ລວມລາຍຊື່ຜູ້ຕິດຕໍ່ແລ້ວ
   *[other] ລວມແລ້ວ { $count } ລາຍການ
}
contacts-import = ນຳເຂົ້າ
contacts-export = ສົ່ງອອກ
contacts-import-file = ນຳເຂົ້າລາຍຊື່ຜູ້ຕິດຕໍ່ຈາກໄຟລ໌ vCard ຫຼື CSV
contacts-imported = { $count ->
   *[other] ນຳເຂົ້າລາຍຊື່ຜູ້ຕິດຕໍ່ { $count } ລາຍການໄປຍັງ { $place } ແລ້ວ
}
contacts-imported-some = { $count ->
   *[other] ນຳເຂົ້າລາຍຊື່ຜູ້ຕິດຕໍ່ { $count } ລາຍການໄປຍັງ { $place } ແລ້ວ ຂ້າມ { $skipped } ລາຍການທີ່ບັນທຶກໄວ້ແລ້ວ
}
contacts-import-none = ບໍ່ພົບລາຍຊື່ຜູ້ຕິດຕໍ່ໃນ { $name }
contacts-import-all-saved = ທຸກຄົນໃນ { $name } ຖືກບັນທຶກໄວ້ແລ້ວ
contacts-import-failed = ອ່ານ { $name } ບໍ່ໄດ້: { $error }
contacts-exported = { $count ->
   *[other] ສົ່ງອອກລາຍຊື່ຜູ້ຕິດຕໍ່ { $count } ລາຍການໄປຍັງ { $path } ແລ້ວ
}
contacts-export-none = ບໍ່ມີລາຍຊື່ຜູ້ຕິດຕໍ່ໃຫ້ສົ່ງອອກ
contacts-export-failed = ສົ່ງອອກລາຍຊື່ຜູ້ຕິດຕໍ່ບໍ່ໄດ້: { $error }
contacts-print = ພິມ
contacts-print-title = ລາຍຊື່ຜູ້ຕິດຕໍ່
contacts-print-none = ບໍ່ມີລາຍຊື່ຜູ້ຕິດຕໍ່ທີ່ຈະພິມ
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = ວັນເກີດ: { $day }
contacts-print-nickname = ຊື່ຫຼິ້ນ: { $name }
contacts-create = ສ້າງລາຍຊື່ຜູ້ຕິດຕໍ່

## Search and the list

contacts-search = ຊອກຫາລາຍຊື່ຜູ້ຕິດຕໍ່
contacts-loading = ກຳລັງໂຫຼດລາຍຊື່ຜູ້ຕິດຕໍ່…
contacts-empty = ຍັງບໍ່ມີລາຍຊື່ຜູ້ຕິດຕໍ່ທີ່ບັນທຶກໄວ້. ລາຍຊື່ຜູ້ຕິດຕໍ່ທີ່ທ່ານບັນທຶກໃນ Gmail, Outlook ຫຼືບໍລິການອີເມວຂອງທ່ານຈະສະແດງຢູ່ບ່ອນນີ້.
contacts-empty-no-books = ລາຍຊື່ຜູ້ຕິດຕໍ່ຈາກບັນຊີຂອງທ່ານຈະສະແດງຢູ່ບ່ອນນີ້ເມື່ອຊິງຄ໌ແລ້ວ.
contacts-none-found = ບໍ່ມີລາຍຊື່ຜູ້ຕິດຕໍ່ທີ່ກົງກັບການຊອກຫາ.
contacts-starred = { $count ->
   *[other] ລາຍຊື່ຜູ້ຕິດຕໍ່ທີ່ຕິດດາວ ({ $count })
}
contacts-count = ລາຍຊື່ຜູ້ຕິດຕໍ່ ({ $count })
contacts-col-name = ຊື່
contacts-col-email = ອີເມວ
contacts-col-phone = ເບີໂທລະສັບ
contacts-col-job = ຕຳແໜ່ງວຽກ ແລະ ບໍລິສັດ
contacts-col-labels = ປ້າຍກຳກັບ

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = ອະນຸຍາດໃຫ້ Katna ອ່ານລາຍຊື່ຜູ້ຕິດຕໍ່ຂອງ { $address }.
contacts-allow-many = { $more ->
   *[other] ອະນຸຍາດໃຫ້ Katna ອ່ານລາຍຊື່ຜູ້ຕິດຕໍ່ຂອງ { $address } ແລະອີກ { $more } ບັນຊີ.
}
contacts-allow-button = ອະນຸຍາດ

## A contact's page

contacts-back = ກັບໄປລາຍຊື່ຜູ້ຕິດຕໍ່
contacts-edit = ແກ້ໄຂ
contacts-delete = ລຶບ
contacts-qr = ແບ່ງປັນເປັນລະຫັດ QR
contacts-qr-about = ສະແກນດ້ວຍກ້ອງຂອງໂທລະສັບເພື່ອບັນທຶກລາຍຊື່ຜູ້ຕິດຕໍ່.
contacts-qr-too-long = ລາຍຊື່ຜູ້ຕິດຕໍ່ນີ້ມີລາຍລະອຽດຫຼາຍເກີນໄປທີ່ຈະໃສ່ໃນລະຫັດ QR ໄດ້.
contacts-qr-done = ສຳເລັດ
contacts-deleted = ລຶບ { $name } ແລ້ວ
contacts-added = ເພີ່ມ { $name } ໃສ່ລາຍຊື່ຜູ້ຕິດຕໍ່ແລ້ວ
contacts-find-mail = ອີເມວ
contacts-details = ລາຍລະອຽດຜູ້ຕິດຕໍ່
contacts-saved-in = ບັນທຶກໄວ້ໃນ
contacts-notes = ບັນທຶກ
contacts-birthday = ວັນເກີດ
contacts-nickname = ຊື່ຫຼິ້ນ
contacts-this-computer = ຄອມພິວເຕີເຄື່ອງນີ້
contacts-kind-home = ເຮືອນ
contacts-kind-work = ບ່ອນເຮັດວຽກ
contacts-kind-mobile = ມືຖື
contacts-kind-other = ອື່ນໆ
contacts-source-google = Google ລາຍຊື່ຜູ້ຕິດຕໍ່
contacts-source-microsoft = ລາຍຊື່ຜູ້ຕິດຕໍ່ Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = ສ້າງລາຍຊື່ຜູ້ຕິດຕໍ່
contacts-edit-title = ແກ້ໄຂລາຍຊື່ຜູ້ຕິດຕໍ່
contacts-edit-save = ບັນທຶກ
contacts-edit-saving = ກຳລັງບັນທຶກ…
contacts-edit-cancel = ຍົກເລີກ
contacts-saved = ບັນທຶກລາຍຊື່ຜູ້ຕິດຕໍ່ແລ້ວ
contacts-edit-save-to = ບັນທຶກໄປຫາ
contacts-edit-changes-go-to = ການປ່ຽນແປງຈະຖືກບັນທຶກໄປຫາ { $place }.
contacts-edit-given = ຊື່
contacts-edit-family = ນາມສະກຸນ
contacts-edit-company = ບໍລິສັດ
contacts-edit-job = ຕຳແໜ່ງວຽກ
contacts-edit-email = ອີເມວ
contacts-edit-phone = ໂທລະສັບ
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = ເພີ່ມອີເມວ
contacts-edit-add-phone = ເພີ່ມເບີໂທລະສັບ
contacts-edit-street = ທີ່ຢູ່ຖະໜົນ
contacts-edit-city = ເມືອງ
contacts-edit-postcode = ລະຫັດໄປສະນີ
contacts-edit-country = ປະເທດ
contacts-edit-birthday = ວັນເກີດ (YYYY-MM-DD)
contacts-edit-empty = ເພີ່ມຊື່, ອີເມວ ຫຼື ເບີໂທລະສັບກ່ອນ.
