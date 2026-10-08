# Katna Mail, Lao (ລາວ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = ເຊີບເວີອີເມວ
problems-signed-out = { $provider } ອອກຈາກລະບົບ Katna ໃນ { $address } ແລ້ວ. ອີເມວຢຸດຊິງຄ໌.
problems-password-refused = { $provider } ປະຕິເສດລະຫັດຜ່ານຂອງ { $address }. ມັນອາດຖືກປ່ຽນແລ້ວ.
problems-no-answer = { $provider } ບໍ່ຕອບສະໜອງສຳລັບ { $address }. Katna ຈະລອງຕໍ່ໄປ.
problems-offline = ທ່ານອອບລາຍຢູ່. ອີເມວຂອງທ່ານຍັງຢູ່ທີ່ນີ້, ແລະ ອີເມວທີ່ທ່ານສົ່ງຈະລໍຖ້າຈົນກວ່າທ່ານຈະກັບມາ.
problems-accounts-need-you = { $count ->
   *[other] { $count } ບັນຊີຕ້ອງການທ່ານ
}
problems-show = ສະແດງ
problems-later = ພາຍຫຼັງ
problems-new-password = ລະຫັດຜ່ານໃໝ່
problems-try-again = ລອງໃໝ່

## The New password card

problems-password-title = ລະຫັດຜ່ານໃໝ່
problems-password-detail = { $provider } ປະຕິເສດລະຫັດຜ່ານທີ່ບັນທຶກໄວ້ຂອງ { $address }. ພິມລະຫັດຜ່ານໃໝ່; Katna ຈະກວດມັນກ່ອນເກັບໄວ້.
problems-password-placeholder = ລະຫັດຜ່ານ
problems-password-show = ສະແດງລະຫັດຜ່ານ
problems-password-hide = ເຊື່ອງລະຫັດຜ່ານ
problems-password-cancel = ຍົກເລີກ
problems-password-save = ບັນທຶກ
problems-password-checking = ກຳລັງກວດ…
problems-password-refused-again = { $provider } ປະຕິເສດລະຫັດຜ່ານນີ້ເຊັ່ນກັນ. ກວດເບິ່ງ ແລະ ລອງໃໝ່.
problems-password-saved = ບັນທຶກລະຫັດຜ່ານຂອງ { $address } ແລ້ວ. ກຳລັງດຶງອີເມວຂອງທ່ານ…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = ເຊີບເວີອີເມວຂອງ { $address } ບໍ່ຍອມຮັບການຍ້າຍ { $count ->
   *[other] { $count } ຂໍ້ຄວາມ, ສະນັ້ນພວກມັນກັບໄປຢູ່ບ່ອນເດີມແລ້ວ.
}
problems-refused-flags = ເຊີບເວີອີເມວຂອງ { $address } ບໍ່ຍອມຮັບການໝາຍ { $count ->
   *[other] { $count } ຂໍ້ຄວາມ (ອ່ານແລ້ວ, ຕິດດາວ…), ສະນັ້ນພວກມັນກັບເປັນຄືເດີມແລ້ວ.
}
problems-refused-label = ເຊີບເວີອີເມວຂອງ { $address } ບໍ່ຍອມຮັບການປ່ຽນປ້າຍກຳກັບຂອງ { $count ->
   *[other] { $count } ຂໍ້ຄວາມ, ສະນັ້ນພວກມັນກັບເປັນຄືເດີມແລ້ວ.
}
problems-refused-delete = ເຊີບເວີອີເມວຂອງ { $address } ບໍ່ຍອມຮັບການລຶບ { $count ->
   *[other] { $count } ຂໍ້ຄວາມ, ສະນັ້ນພວກມັນກັບມາແລ້ວ.
}
problems-refused-other = ເຊີບເວີອີເມວຂອງ { $address } ບໍ່ຍອມຮັບ { $count ->
   *[other] { $count } ການປ່ຽນແປງ, ສະນັ້ນ Katna ເອົາພວກມັນກັບຄືເດີມແລ້ວ.
}
problems-details = ລາຍລະອຽດ

## Katna's background service (katna-daemon) isn't running

service-starting = ກຳລັງເລີ່ມບໍລິການພື້ນຫຼັງຂອງ Katna…
service-failed = ບໍລິການພື້ນຫຼັງຂອງ Katna ບໍ່ຍອມເລີ່ມ, ສະນັ້ນອີເມວຈຶ່ງບໍ່ຊິງຄ໌.
service-start-again = ເລີ່ມອີກຄັ້ງ
service-started-again = ບໍລິການພື້ນຫຼັງຂອງ Katna ຢຸດເຮັດວຽກ ແລະ ຖືກເລີ່ມໃໝ່ແລ້ວ.
service-details-title = ເປັນຫຍັງບໍລິການຈຶ່ງບໍ່ເລີ່ມ
service-details-body = ສຳເນົາສິ່ງນີ້ ແລະ ສົ່ງມາພ້ອມກັບລາຍງານຂອງທ່ານ. ມັນບໍ່ມີອີເມວ ຫຼື ລະຫັດຜ່ານຢູ່ໃນນັ້ນ.
service-details-copy = ສຳເນົາ
service-details-close = ປິດ
service-not-running = ບໍລິການເບື້ອງຫຼັງຂອງ Katna ບໍ່ໄດ້ເຮັດວຽກຢູ່.
service-no-answer = ບໍລິການເບື້ອງຫຼັງຂອງ Katna ບໍ່ໄດ້ຕອບ: { $error }
service-no-session = ບໍ່ມີເຊດຊັນ D-Bus: { $error }

## Safe mode: an update left Katna's background service unable to start

safe-line = Katna ຢູ່ໃນໂໝດປອດໄພ ຫຼັງຈາກມີບັນຫາກັບການອັບເດດ, ສະນັ້ນອີເມວຈຶ່ງບໍ່ຊິງຄ໌.
safe-try-again = ລອງໃໝ່
safe-restore = ກູ້ຄືນ
safe-restoring = ກຳລັງກູ້ຄືນຂໍ້ມູນຂອງທ່ານຈາກ { $when }…
safe-restored = ກູ້ຄືນຂໍ້ມູນຂອງທ່ານຈາກ { $when } ແລ້ວ. ສິ່ງທີ່ມີຢູ່ກ່ອນໜ້ານັ້ນຖືກເກັບໄວ້ໃນໂຟນເດີ.
safe-show-folder = ສະແດງໂຟນເດີ
safe-restore-failed = ກູ້ຄືນຂໍ້ມູນຂອງທ່ານບໍ່ໄດ້: { $error }
safe-restore-title = ກູ້ຄືນຂໍ້ມູນຂອງທ່ານຈາກກ່ອນການອັບເດດບໍ?
safe-restore-body = Katna ຈະກັບໄປໃຊ້ສຳເນົາທີ່ທ່ານເລືອກ. ອີເມວທີ່ມາຮອດຫຼັງຈາກນັ້ນຈະດາວໂຫຼດຄືນຈາກບັນຊີຂອງທ່ານ.
safe-restore-none = ຍັງບໍ່ມີສຳເນົາເທື່ອ. Katna ຈະສ້າງສຳເນົາກ່ອນທີ່ການອັບເດດແຕ່ລະຄັ້ງຈະປ່ຽນຂໍ້ມູນຂອງທ່ານ.
safe-restore-keep = ສິ່ງທີ່ມີຢູ່ຕອນນີ້, ລວມທັງອີເມວທີ່ຍັງບໍ່ໄດ້ສົ່ງ, ສະບັບຮ່າງ ແລະ ການປ່ຽນແປງທີ່ຍັງບໍ່ໄດ້ຊິງຄ໌, ຈະຖືກເກັບໄວ້ໃນໂຟນເດີກ່ອນ, ສະນັ້ນບໍ່ມີຫຍັງເສຍ.
safe-restore-cancel = ຍົກເລີກ
safe-restore-mail = ອີເມວ
safe-restore-pim = ບັນຊີ ແລະ ລາຍຊື່ຜູ້ຕິດຕໍ່
safe-restore-blobs = ໄຟລ໌ແນບ
safe-report-title = ລາຍງານດີບັກ
safe-report-body = ສຳເນົາອັນນີ້ ແລ້ວແນບໃສ່ລາຍງານບັກຂອງທ່ານ. ມັນບໍ່ມີອີເມວ, ທີ່ຢູ່ ຫຼື ລະຫັດຜ່ານຢູ່ໃນນັ້ນ.
safe-report-restore = ກູ້ຄືນ…
safe-report-copied = ສຳເນົາລາຍງານດີບັກແລ້ວ
