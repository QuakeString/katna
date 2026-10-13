# Katna Mail, Indonesian (Bahasa Indonesia).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = Tidak terkirim karena { $reason }.
outbox-retrying = Belum terkirim karena { $reason }. Katna akan mencoba lagi dengan sendirinya.
outbox-waiting-sign-in = Menunggu Anda masuk lagi ke { $address }. Pesan akan dikirim setelahnya.
outbox-waiting-password = Menunggu sandi baru untuk { $address }. Pesan akan dikirim setelahnya.
outbox-waiting-connection = Menunggu koneksi. Pesan akan dikirim saat Anda kembali online.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = tidak ada penerimanya
outbox-reason-address = salah satu alamat tujuannya tidak ada
outbox-reason-too-large = terlalu besar untuk server email
outbox-reason-blocked = server email memblokirnya
outbox-reason-gone = salinannya di komputer ini sudah tidak ada
outbox-reason-refused = server email menolaknya

## Buttons and notes

outbox-try-again = Coba lagi
outbox-edit = Edit
outbox-delete = Hapus
outbox-deleted = Dihapus dari Kotak Keluar
outbox-sending-again = Mengirim lagi…
outbox-snackbar-not-sent = “{ $subject }” tidak terkirim karena { $reason }.
outbox-open = Kotak Keluar
