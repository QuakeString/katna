# Katna Mail, Malay (Bahasa Melayu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = Tidak dihantar kerana { $reason }.
outbox-retrying = Belum dihantar kerana { $reason }. Katna akan mencuba lagi dengan sendirinya.
outbox-waiting-sign-in = Menunggu anda log masuk ke { $address } semula. Mesej akan dihantar selepas itu.
outbox-waiting-password = Menunggu kata laluan baharu untuk { $address }. Mesej akan dihantar selepas itu.
outbox-waiting-connection = Menunggu sambungan. Mesej akan dihantar apabila anda kembali dalam talian.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = ia tiada penerima
outbox-reason-address = alamat penerimanya tidak wujud
outbox-reason-too-large = ia terlalu besar untuk pelayan mel
outbox-reason-blocked = pelayan mel menyekatnya
outbox-reason-gone = salinannya pada komputer ini sudah tiada
outbox-reason-refused = pelayan mel menolaknya

## Buttons and notes

outbox-try-again = Cuba lagi
outbox-edit = Sunting
outbox-delete = Padam
outbox-deleted = Dipadam daripada Peti Keluar
outbox-sending-again = Menghantar semula…
outbox-snackbar-not-sent = “{ $subject }” tidak dihantar kerana { $reason }.
outbox-open = Peti Keluar
