# Katna Mail, Lao (ລາວ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = ບໍ່ໄດ້ສົ່ງ ເພາະ { $reason }.
outbox-retrying = ຍັງບໍ່ໄດ້ສົ່ງ ເພາະ { $reason }. Katna ຈະລອງໃໝ່ເອງ.
outbox-waiting-sign-in = ລໍຖ້າໃຫ້ທ່ານເຂົ້າສູ່ລະບົບ { $address } ອີກຄັ້ງ. ແລ້ວມັນຈະສົ່ງອອກ.
outbox-waiting-password = ລໍຖ້າລະຫັດຜ່ານໃໝ່ຂອງ { $address }. ແລ້ວມັນຈະສົ່ງອອກ.
outbox-waiting-connection = ລໍຖ້າການເຊື່ອມຕໍ່. ມັນຈະສົ່ງອອກເມື່ອທ່ານກັບມາອອນລາຍ.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = ມັນບໍ່ມີຜູ້ຮັບ
outbox-reason-address = ທີ່ຢູ່ທີ່ສົ່ງໄປບໍ່ມີຢູ່ຈິງ
outbox-reason-too-large = ມັນໃຫຍ່ເກີນໄປສຳລັບເຊີບເວີອີເມວ
outbox-reason-blocked = ເຊີບເວີອີເມວບລັອກມັນ
outbox-reason-gone = ສຳເນົາຂອງມັນໃນຄອມພິວເຕີເຄື່ອງນີ້ບໍ່ມີແລ້ວ
outbox-reason-refused = ເຊີບເວີອີເມວປະຕິເສດມັນ

## Buttons and notes

outbox-try-again = ລອງໃໝ່
outbox-edit = ແກ້ໄຂ
outbox-delete = ລຶບ
outbox-deleted = ລຶບອອກຈາກກ່ອງຈົດໝາຍອອກແລ້ວ
outbox-sending-again = ກຳລັງສົ່ງອີກຄັ້ງ…
outbox-snackbar-not-sent = “{ $subject }” ບໍ່ໄດ້ສົ່ງ ເພາະ { $reason }.
outbox-open = ກ່ອງຈົດໝາຍອອກ
