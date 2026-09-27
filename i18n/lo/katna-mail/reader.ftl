# Katna Mail, Lao (ລາວ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = ປິດ
reader-back = ກັບຄືນ
reader-mark-unread = ໝາຍວ່າຍັງບໍ່ໄດ້ອ່ານ
reader-move-to = ຍ້າຍໄປທີ່
reader-more = ເພີ່ມເຕີມ
reader-print-all = ພິມທັງໝົດ
reader-new-window = ເປີດໃນໜ້າຕ່າງໃໝ່
reader-position = { $position } ຈາກ { $total }
reader-newer = ໃໝ່ກວ່າ
reader-older = ເກົ່າກວ່າ

## Reading pane: the conversation

reader-removed = ການສົນທະນານີ້ຖືກລຶບອອກແລ້ວ.
reader-no-subject = (ບໍ່ມີຫົວຂໍ້)
reader-collapse-all = ຫຍໍ້ທັງໝົດ
reader-expand-all = ຂະຫຍາຍທັງໝົດ
reader-unknown-sender = (ບໍ່ຮູ້ຈັກຜູ້ສົ່ງ)
reader-date-ago = { $date } ({ $ago })
reader-me = ຂ້ອຍ
reader-to = ເຖິງ { $names }
reader-starred = ຕິດດາວແລ້ວ
reader-not-starred = ບໍ່ໄດ້ຕິດດາວ
reader-too-long = ຂໍ້ຄວາມຍາວເກີນໄປທີ່ຈະສະແດງທັງໝົດໄດ້.
reader-encrypted-images = ຮູບພາບຈາກເວັບຈະບໍ່ຖືກໂຫຼດໃນອີເມວທີ່ເຂົ້າລະຫັດ.
reader-window-failed = ບໍ່ສາມາດເປີດໜ້າຕ່າງໃໝ່ໄດ້.

## Reading pane: message details (opened from "to me")

reader-details-from = ຈາກ:
reader-details-to = ເຖິງ:
reader-details-cc = ສຳເນົາ:
reader-details-date = ວັນທີ:
reader-details-subject = ຫົວຂໍ້:

## Reading pane: downloading a message

reader-downloading = ກຳລັງດາວໂຫຼດຂໍ້ຄວາມນີ້ຈາກເຊີບເວີ…
reader-download-failed = ບໍ່ສາມາດດາວໂຫຼດຂໍ້ຄວາມນີ້ໄດ້.
reader-try-again = ລອງໃໝ່

## Reply row

reply-reply = ຕອບກັບ
reply-reply-all = ຕອບກັບທັງໝົດ
reply-forward = ສົ່ງຕໍ່

## Encrypted and signed mail

security-decrypting = ກຳລັງຖອດລະຫັດ…
security-checking = ກຳລັງກວດສອບລາຍເຊັນ…
security-partly-encrypted = ມີພຽງບາງສ່ວນຂອງຂໍ້ຄວາມນີ້ທີ່ຖືກເຂົ້າລະຫັດ. ສ່ວນທີ່ເຫຼືອຖືກເພີ່ມນອກການປົກປ້ອງ ແລະ ອາດມາຈາກໃຜກໍໄດ້.
security-partly-signed = ມີພຽງບາງສ່ວນຂອງຂໍ້ຄວາມນີ້ທີ່ຖືກເຊັນ. ສ່ວນທີ່ເຫຼືອຖືກເພີ່ມນອກການປົກປ້ອງ ແລະ ອາດມາຈາກໃຜກໍໄດ້.
security-encrypted = ຂໍ້ຄວາມທີ່ເຂົ້າລະຫັດ
security-encrypted-smime = ຂໍ້ຄວາມທີ່ເຂົ້າລະຫັດ (S/MIME)
security-no-key = ບໍ່ສາມາດຖອດລະຫັດຂໍ້ຄວາມນີ້ໄດ້: ມັນຖືກເຂົ້າລະຫັດສຳລັບກະແຈທີ່ທ່ານບໍ່ມີ.
security-cancelled = ຍົກເລີກການຖອດລະຫັດແລ້ວ.
security-damaged = ບໍ່ສາມາດຖອດລະຫັດຂໍ້ຄວາມນີ້ໄດ້: ຂໍ້ມູນທີ່ເຂົ້າລະຫັດເສຍຫາຍ ຫຼື ຖືກປ່ຽນແປງ.
security-decrypt-unavailable = ບໍ່ສາມາດຖອດລະຫັດຂໍ້ຄວາມນີ້ໄດ້: ຕິດຕັ້ງ { $tool } ເພື່ອອ່ານອີເມວທີ່ເຂົ້າລະຫັດ.
security-decrypt-failed = ບໍ່ສາມາດຖອດລະຫັດຂໍ້ຄວາມນີ້ໄດ້: { $reason }
security-unknown-signer = ຜູ້ເຊັນທີ່ບໍ່ຮູ້ຈັກ
security-signed-verified = ເຊັນໂດຍ { $signer } · ຢືນຢັນແລ້ວ
security-signed-not-sender = ເຊັນໂດຍ { $signer }, ເຊິ່ງບໍ່ແມ່ນຜູ້ສົ່ງ
security-signed-untrusted = ເຊັນໂດຍ { $signer }, ດ້ວຍກະແຈທີ່ທ່ານໝາຍວ່າບໍ່ໜ້າເຊື່ອຖື
security-signed-unverified = ເຊັນໂດຍ { $signer } · ກະແຈຍັງບໍ່ໄດ້ຢືນຢັນ
security-bad-signature = ລາຍເຊັນບໍ່ຖືກຕ້ອງ: ຂໍ້ຄວາມນີ້ຖືກປ່ຽນແປງຫຼັງຈາກເຊັນ, ຫຼື ລາຍເຊັນຖືກປອມແປງ.
security-signature-expired = ເຊັນໂດຍ { $signer } · ລາຍເຊັນໝົດອາຍຸແລ້ວ
security-key-expired = ເຊັນໂດຍ { $signer } · ກະແຈໝົດອາຍຸໄປແລ້ວຕັ້ງແຕ່ນັ້ນ
security-key-revoked = ເຊັນໂດຍ { $signer } ດ້ວຍກະແຈທີ່ຖືກຖອນຄືນແລ້ວ
security-missing-key = ເຊັນດ້ວຍກະແຈທີ່ທ່ານບໍ່ມີ, ຈຶ່ງກວດສອບບໍ່ໄດ້
security-missing-key-id = ເຊັນດ້ວຍກະແຈທີ່ທ່ານບໍ່ມີ ({ $key }), ຈຶ່ງກວດສອບບໍ່ໄດ້
security-signature-unavailable = ເຊັນແລ້ວ; ຕິດຕັ້ງ { $tool } ເພື່ອກວດສອບລາຍເຊັນ
security-signature-error = ບໍ່ສາມາດກວດສອບລາຍເຊັນໄດ້.
tracking-opened = { $who } ເປີດມັນ { $count } ເທື່ອ, ຫຼ້າສຸດ { $when }
tracking-opened-clicked = { $who } ເປີດມັນ ແລະ ຄລິກລິ້ງ { $count } ເທື່ອ, ຫຼ້າສຸດ { $when }
tracking-maybe-opened = { $who } ອາດຈະເປີດມັນແລ້ວ (Apple Mail ໂຫຼດຮູບພາບເພື່ອຄວາມເປັນສ່ວນຕົວ)
tracking-not-opened = { $who } ຍັງບໍ່ໄດ້ເປີດມັນ
tracking-receipt = { $who } ສົ່ງໃບຢືນຢັນການອ່ານແລ້ວ
tracking-receipt-displayed = ໃບຢືນຢັນການອ່ານ: { $who } ເປີດຂໍ້ຄວາມຂອງທ່ານແລ້ວ
tracking-receipt-other = ໃບຢືນຢັນການອ່ານ: { $who } ລຶບ ຫຼື ຈັດການຂໍ້ຄວາມຂອງທ່ານໂດຍບໍ່ໄດ້ເປີດມັນ

## Remote images and pictures

remote-hidden = ຮູບພາບໃນຂໍ້ຄວາມນີ້ຖືກເຊື່ອງໄວ້.
remote-show = ສະແດງຮູບພາບ
remote-always-show = ສະແດງຈາກຜູ້ສົ່ງນີ້ສະເໝີ
remote-picture-use = ໃຊ້
remote-picture-too-big = ເລືອກຮູບທີ່ມີຂະໜາດບໍ່ເກີນ 8 MB.
remote-picture-type = ເລືອກຮູບ PNG, JPEG, GIF, WebP ຫຼື SVG.
remote-picture-read-failed = ບໍ່ສາມາດອ່ານຮູບໄດ້: { $error }
remote-picture-keep-failed = ບໍ່ສາມາດເກັບຮູບໄວ້ໄດ້: { $error }
remote-picture-remove-failed = ບໍ່ສາມາດລຶບຮູບໄດ້: { $error }

## Attachments

attachment-count = ໄຟລ໌ແນບ { $count } ໄຟລ໌
attachment-save = ບັນທຶກ
attachment-save-all = ບັນທຶກທັງໝົດ
attachment-save-all-tooltip = ບັນທຶກໄຟລ໌ແນບທັງໝົດໄວ້ໃນໂຟນເດີ
attachment-save-here = ບັນທຶກໄວ້ບ່ອນນີ້
attachment-not-downloaded = ຂໍ້ຄວາມນີ້ບໍ່ໄດ້ຖືກດາວໂຫຼດ.
attachment-not-found = ບໍ່ພົບໄຟລ໌ແນບນີ້ໃນຂໍ້ຄວາມ.
attachment-read-failed = ບໍ່ສາມາດອ່ານ { $name } ໄດ້
attachment-numbered = ໄຟລ໌ແນບ { $number }
attachment-saved-all = ບັນທຶກ { $count } ໄຟລ໌ໄວ້ໃນ { $place } ແລ້ວ
attachment-saved-some = ບັນທຶກ { $saved } ຈາກ { $total } ໄຟລ໌ໄວ້ໃນ { $place } ແລ້ວ. ບໍ່ສາມາດບັນທຶກ { $failed }
attachment-saved-to = ບັນທຶກໄວ້ທີ່ { $path } ແລ້ວ
attachment-save-failed = ບໍ່ສາມາດບັນທຶກ { $name } ໄດ້: { $error }
attachment-open-failed = ບໍ່ສາມາດເປີດ { $name } ໄດ້: { $error }
attachment-risky = ໄຟລ໌ນີ້ອາດເປີດໂປຣແກຣມໃຫ້ເຮັດວຽກ, ສະນັ້ນ Katna ຈະບໍ່ເປີດມັນ. ໃຫ້ບັນທຶກມັນແທນ.
attachment-encrypted-open = ໄຟລ໌ນີ້ມາແບບເຂົ້າລະຫັດ. ບັນທຶກມັນເພື່ອເປີດຢູ່ບ່ອນອື່ນ.

## Printing

print-failed = ບໍ່ສາມາດພິມໄດ້: { $error }
print-no-font = ບໍ່ພົບຟອນ
print-opened-as-pdf = ເປີດເປັນ PDF ແລ້ວ ເພື່ອພິມຈາກບ່ອນນັ້ນ.
print-preview-title = ເບິ່ງຕົວຢ່າງກ່ອນພິມ
print-preview-laying-out = ກຳລັງຈັດໜ້າ…
print-preview-pages = { $count } ໜ້າ
print-preview-more = ແລະ ອີກ { $count } ໜ້າ
print-preview-failed = ບໍ່ສາມາດສະແດງໜ້າໄດ້
print-preview-paper = ເຈ້ຍ
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-cancel = ຍົກເລີກ
print-preview-print = ພິມ
print-not-downloaded = (ຍັງບໍ່ໄດ້ດາວໂຫຼດ.)
print-encrypted = (ເຂົ້າລະຫັດໄວ້. ເປີດໃນ Katna Mail ເພື່ອພິມຂໍ້ຄວາມ.)
print-to = ເຖິງ: { $addresses }
print-cc = ສຳເນົາ: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = ເປີດຂໍ້ຄວາມນີ້ເພື່ອເບິ່ງໄຟລ໌ແນບ.
text-copy = ສຳເນົາ
text-select-all = ເລືອກທັງໝົດ
