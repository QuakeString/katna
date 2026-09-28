# Katna Mail, Malay (Bahasa Melayu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = Mesej Baharu
compose-restore = Pulihkan
compose-minimize = Minimumkan
compose-exit-full-screen = Keluar skrin penuh
compose-open-window = Buka dalam tetingkap baharu
compose-save-close = Simpan dan tutup
compose-back-to-mail = Kembali ke tetingkap mel
compose-pop-out-reply = Buka balasan dalam tetingkap sendiri
compose-edit-recipients = Sunting penerima
compose-summary-cc = Sk: { $names }
compose-summary-bcc = Skt: { $names }
compose-more-recipients = { $count } lagi
compose-show-trimmed = Tunjukkan kandungan yang dipangkas
compose-hide-trimmed = Sembunyikan kandungan yang dipangkas
compose-remove-trimmed = Alih keluar teks petikan
compose-trimmed-removed = Teks petikan dialih keluar

## Recipients and subject

compose-to = Kepada
compose-cc = Sk
compose-bcc = Skt
compose-from = Daripada
compose-from-choose = Hantar daripada akaun lain
compose-recipients = Penerima
compose-subject = Subjek

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = Hantar atau buang mesej yang terbuka dahulu.
compose-bad-address = “{ $address }” bukan alamat e-mel.
compose-no-recipients = Tambah sekurang-kurangnya seorang penerima.
compose-attachments-too-large = Saiz lampiran ialah { $size }; pelayan mel menerima sehingga { $limit }.
compose-no-account = Tambah akaun untuk menghantar mel.
compose-past-time = Pilih masa pada masa hadapan.
compose-scheduling = Menjadualkan…
compose-sending = Menghantar…
compose-scheduled = Penghantaran dijadualkan pada { $when }
compose-sent-archived = Dihantar dan diarkibkan
compose-sent = Mesej dihantar
compose-discarded = Draf dibuang
compose-draft-saved = Draf disimpan
compose-draft-failed = Draf tidak dapat disimpan: { $error }
compose-draft-not-opened = Draf tidak dapat dibuka.

## Attachments

compose-picker-insert = Sisipkan
compose-picker-attach = Lampirkan
compose-file-too-large = { $name } terlalu besar: mesej boleh membawa sehingga { $limit }.
compose-attachment-size = ({ $size })
compose-remove-attachment = Alih keluar lampiran
compose-attachments-total = { $count } fail, { $size }
compose-drive-note = { $name } melebihi { $limit }, jadi fail itu dihantar ke Google Drive anda dan mesej membawa pautan.
compose-drive-tip = Dalam Google Drive anda; mesej membawa pautan
compose-drive-uploading = Memuat naik { $percent }%
compose-drive-allow = Benarkan Drive
compose-drive-allow-tip = Log masuk dengan Google sekali lagi supaya Katna boleh meletakkan fail besar dalam Drive anda
compose-drive-retry = Cuba lagi
compose-drive-sends-when-uploaded = Dihantar selepas { $name } dimuat naik
compose-drive-not-uploaded = { $name } belum lagi dalam Google Drive
compose-drive-share-failed = Tidak dapat berkongsi fail dalam Google Drive: { $error }
compose-drive-share-title = Kongsi fail dengan semua orang?
compose-drive-share-text = { $count ->
   *[other] Google Drive tidak dapat berkongsi fail dengan { $addresses }, yang tidak mempunyai akaun Google. Sebaliknya, sesiapa yang mempunyai pautan boleh membukanya.
}
compose-drive-share-link = Kongsi dengan pautan
compose-drive-send-without = Hantar tanpa berkongsi
compose-drive-share-cancel = Batal
compose-drive-card-detail = { $size } · Google Drive
compose-onedrive-note = { $name } melebihi { $limit }, jadi fail itu dihantar ke OneDrive anda dan mesej membawa pautan.
compose-onedrive-tip = Dalam OneDrive anda; mesej membawa pautan
compose-onedrive-allow = Benarkan OneDrive
compose-onedrive-allow-tip = Log masuk dengan Microsoft sekali lagi supaya Katna boleh meletakkan fail besar dalam OneDrive anda
compose-onedrive-not-uploaded = { $name } belum lagi dalam OneDrive
compose-onedrive-share-failed = Tidak dapat berkongsi fail dalam OneDrive: { $error }
compose-onedrive-share-text = { $count ->
   *[other] OneDrive tidak dapat berkongsi fail dengan { $addresses }. Sebaliknya, sesiapa yang mempunyai pautan boleh membukanya.
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-drop-files = Lepaskan fail di sini
compose-drop-here = Lepaskan di sini
compose-paste-keep-formatting = Kekalkan pemformatan
compose-paste-table = Jadual
compose-paste-picture = Gambar
compose-paste-plain-text = Teks biasa
compose-paste-inline = Dalam teks
compose-paste-attachment = Lampiran

## Encryption and signing (the toggles by the recipients)

compose-encrypt = Sulitkan
compose-encrypted = Disulitkan: hanya penerima boleh membacanya
compose-sign = Tandatangani
compose-signed = Ditandatangani: penerima boleh menyemak bahawa ia daripada anda
compose-track = Jejaki pembukaan dan klik
compose-tracked = Dijejaki: anda nampak bila setiap penerima membukanya atau mengikuti pautan
compose-track-clicks = Jejaki klik pautan (teks biasa tidak dapat menunjukkan pembukaan)
compose-tracked-clicks = Dijejaki: anda nampak bila setiap penerima mengikuti pautan
compose-track-sign-in = Log masuk ke akaun Katna untuk menjejaki pembukaan dan klik
compose-receipt = Minta resit baca
compose-receipt-on = Resit baca diminta: apl penerima mungkin meminta mereka menghantarnya
compose-delivery = Minta resit penghantaran
compose-delivery-on = Resit penghantaran diminta: pelayan mel anda akan menghantar e-mel kepada anda apabila pelayan setiap penerima menerimanya
compose-delivery-unavailable = Pelayan mel anda tidak menghantar resit penghantaran

## Spelling

spell-no-dictionary = Tiada kamus ejaan untuk { $language } dipasang (contohnya hunspell-en_us).
spell-dictionary-error = Kamus ejaan: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = Tambah “{ $words }”
grammar-remove = Alih keluar “{ $words }”
grammar-ignore = Abaikan

## Send checks (asked before a message goes out)

send-check-attachment-title = Adakah anda mahu melampirkan fail?
send-check-attachment-text = Anda menyebut tentang lampiran, tetapi tiada apa-apa yang dilampirkan.
send-check-attach = Lampirkan fail
send-check-subject-title = Hantar tanpa subjek?
send-check-subject-text = Mesej ini tiada subjek.
send-check-add-subject = Tambah subjek
send-check-send-anyway = Hantar juga
recipient-not-valid = Bukan alamat e-mel yang sah
recipient-show-address = Tunjukkan alamat
recipient-remove = Alih keluar
recipient-bad-title = Semak alamat
recipient-bad-text = “{ $address }” bukan alamat e-mel yang sah. Betulkan atau alih keluarnya sebelum menghantar.
recipient-bad-fix = Betulkan
