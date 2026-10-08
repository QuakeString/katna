# Katna Mail, Malay (Bahasa Melayu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Tutup
reader-back = Kembali
reader-mark-unread = Tandai sebagai belum dibaca
reader-move-to = Alih ke
reader-snooze = Tunda
reader-remind = Ingatkan saya
reader-more = Lagi
reader-original-colors = Tunjukkan warna asal
reader-dark-colors = Tunjukkan dalam warna gelap
reader-print-all = Cetak semua
reader-new-window = Dalam tetingkap baharu
reader-position = { $position } daripada { $total }
reader-newer = Lebih baharu
reader-older = Lebih lama

## Reading pane: the conversation

reader-removed = Perbualan ini telah dialih keluar.
reader-no-subject = (tiada subjek)
reader-collapse-all = Runtuhkan semua
reader-expand-all = Kembangkan semua
reader-unknown-sender = (pengirim tidak diketahui)
reader-date-ago = { $date } ({ $ago })
reader-sending = Menghantar…
reader-me = saya
reader-to = kepada { $names }
reader-to-label = kepada
reader-tick-delivered = Sampai { $when }
reader-tick-no-bounce = Dihantar { $when }; tiada lantunan kembali, jadi kemungkinan besar ia sampai
reader-tick-bounced = Tidak sampai: melantun { $when }
reader-tick-read = Dibaca { $when } (resit baca)
reader-tick-opened = Dibuka, terakhir { $when } (penjejakan pembukaan)
reader-starred = Dibintangi
reader-chip-remove = Alih keluar { $label }
reader-not-starred = Tidak dibintangi
reader-too-long = Mesej ini terlalu panjang untuk dipaparkan sepenuhnya.
reader-encrypted-images = Imej dari web tidak sekali-kali dimuatkan dalam mel yang disulitkan.
reader-window-failed = Tidak dapat membuka tetingkap baharu.

## Reading pane: message details (opened from "to me")

reader-details-from = daripada:
reader-details-to = kepada:
reader-details-cc = sk:
reader-details-date = tarikh:
reader-details-subject = subjek:

## Reading pane: downloading a message

reader-downloading = Memuat turun mesej ini daripada pelayan…
reader-download-failed = Tidak dapat memuat turun mesej ini.
reader-download-failed-reason = Tidak dapat memuat turun mesej ini. { $reason }
reader-download-offline = Akaun ini di luar talian. Pergi ke dalam talian untuk memuat turun mesej ini.
reader-try-again = Cuba lagi

## Reply row

reply-reply = Balas
reply-reply-all = Balas semua
reply-forward = Majukan

## Encrypted and signed mail

security-decrypting = Menyahsulit…
security-checking = Menyemak tandatangan…
security-partly-encrypted = Hanya sebahagian mesej ini yang disulitkan. Selebihnya ditambah di luar perlindungan dan boleh datang daripada sesiapa sahaja.
security-partly-signed = Hanya sebahagian mesej ini yang ditandatangani. Selebihnya ditambah di luar perlindungan dan boleh datang daripada sesiapa sahaja.
security-encrypted = Mesej disulitkan
security-encrypted-smime = Mesej disulitkan (S/MIME)
security-no-key = Tidak dapat menyahsulit mesej ini: mesej ini disulitkan untuk kunci yang anda tidak miliki.
security-cancelled = Penyahsulitan dibatalkan.
security-damaged = Tidak dapat menyahsulit mesej ini: data yang disulitkan rosak atau telah diubah.
security-decrypt-unavailable = Tidak dapat menyahsulit mesej ini: pasang { $tool } untuk membaca mel yang disulitkan.
security-decrypt-failed = Tidak dapat menyahsulit mesej ini: { $reason }
security-unknown-signer = penandatangan yang tidak diketahui
security-signed-verified = Ditandatangani oleh { $signer } · disahkan
security-signed-not-sender = Ditandatangani oleh { $signer }, yang bukan pengirimnya
security-signed-untrusted = Ditandatangani oleh { $signer }, dengan kunci yang anda tandai sebagai tidak dipercayai
security-signed-unverified = Ditandatangani oleh { $signer } · kunci belum disahkan
security-bad-signature = Tandatangan tidak sah: mesej ini telah diubah selepas ditandatangani, atau tandatangannya dipalsukan.
security-signature-expired = Ditandatangani oleh { $signer } · tandatangan telah tamat tempoh
security-key-expired = Ditandatangani oleh { $signer } · kunci telah tamat tempoh sejak itu
security-key-revoked = Ditandatangani oleh { $signer } dengan kunci yang telah dibatalkan
security-missing-key = Ditandatangani dengan kunci yang anda tidak miliki, jadi tidak dapat disemak
security-missing-key-id = Ditandatangani dengan kunci yang anda tidak miliki ({ $key }), jadi tidak dapat disemak
security-signature-unavailable = Ditandatangani; pasang { $tool } untuk menyemak tandatangan
security-signature-error = Tandatangan tidak dapat disemak.
security-look-up-key = Cari kunci

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = Tandatangan disahkan
key-card-verified-detail = Tandatangan ini sah dan anda mempercayai kunci ini.
key-card-unverified = Tandatangan belum disahkan
key-card-unverified-detail = Tandatangan ini sah, tetapi tiada apa yang mengesahkan bahawa kunci ini milik mereka. Bandingkan cap jari dengan mereka, kemudian percayai kunci itu dalam GnuPG (Kleopatra atau gpg --edit-key).
key-card-not-sender = Ditandatangani oleh orang lain
key-card-not-sender-detail = Tandatangan ini sah, tetapi kunci itu bukan milik pengirim.
key-card-untrusted = Kunci tidak dipercayai
key-card-untrusted-detail = Anda telah menandai kunci ini sebagai tidak dipercayai dalam GnuPG.
key-card-signature-expired = Tandatangan telah tamat tempoh
key-card-signature-expired-detail = Tandatangan ini pernah sah, tetapi kini telah tamat tempoh.
key-card-key-expired = Kunci telah tamat tempoh
key-card-key-expired-detail = Tandatangan ini sah, tetapi kunci itu telah tamat tempoh sejak itu.
key-card-key-revoked = Kunci telah dibatalkan
key-card-key-revoked-detail = Pemiliknya telah membatalkan kunci ini, jadi tandatangan ini tidak boleh dipercayai.
key-card-bad = Tandatangan tidak sah
key-card-bad-detail = Mesej ini telah diubah selepas ditandatangani, atau tandatangannya dipalsukan.
key-card-signed-by = Ditandatangani oleh
key-card-belongs-to = Milik
key-card-fingerprint = Cap jari
key-card-signed = Ditandatangani
key-card-key = Kunci
key-card-kind = { $standard }, { $algorithm }
key-card-created = Dicipta
key-card-expires = Tamat tempoh
key-card-never = Tidak pernah
key-card-issued-by = Dikeluarkan oleh
key-card-found-in = Ditemui dalam
key-card-keyring = Gugusan kunci GnuPG anda
key-card-copy = Salin cap jari
key-card-import-title = Import kunci ini?
key-card-from-directory = Ditemui dalam direktori kunci { $domain }.
key-card-from-attachment = Daripada lampiran { $name }.
key-card-import-note = Katna kemudian boleh menyemak tandatangan orang ini dan menyulitkan mel kepadanya. Untuk mempercayai kunci ini sepenuhnya, bandingkan cap jari dengannya.
key-card-cancel = Batal
key-card-import = Import kunci
key-card-looking-up = Mencari kunci…
key-card-looking-up-detail = Bertanya kepada direktori kunci { $domain }.
key-card-not-found = Tiada kunci ditemui
key-card-not-found-detail = { $domain } tidak menerbitkan kunci untuk alamat ini. Minta pengirim menghantar kuncinya kepada anda.
key-card-not-kept = Kunci yang ditemui tidak boleh digunakan.
key-card-failed = Tidak dapat mendapatkan kunci

## Sender checks: the banner on mail that failed the checks the user's mail
## provider ran on its sender (DMARC, DKIM, SPF), and the "?" on the picture
## of a sender nothing confirmed

sender-failed-title = Ini mungkin bukan daripada { $domain }
sender-failed-body = Mel ini gagal dalam semakan pengirim { $provider }. Berhati-hati dengan pautan, lampiran dan balasan.
sender-provider-unknown = penyedia mel anda
sender-details = Butiran
sender-details-hide = Sembunyikan butiran
sender-looks-safe = Nampak selamat
sender-move-to-spam = Alih ke spam
sender-checked-by = Disemak oleh { $provider }
sender-checked-by-server = Disemak oleh { $provider } ({ $server })
sender-dmarc = Domain pengirim (DMARC)
sender-dkim = Tandatangan (DKIM)
sender-spf = Pelayan penghantar (SPF)
sender-result-pass = Lulus
sender-result-fail = Gagal
sender-result-unsure = Tidak pasti
sender-result-none = Tiada
sender-result-missing = Tidak disemak
sender-dmarc-pass = { $domain } mengesahkan pengirim ini.
sender-dmarc-fail = Mel ini tidak sepadan dengan cara { $domain } menyatakan melnya dihantar.
sender-dmarc-none = { $domain } tidak menerbitkan sebarang peraturan untuk melnya.
sender-dkim-pass = Ditandatangani oleh { $domain }.
sender-dkim-fail = Tandatangan daripada { $domain } tidak sepadan dengan mel ini.
sender-dkim-none = Mesej ini tidak ditandatangani.
sender-spf-pass = Dihantar dari pelayan yang disenaraikan oleh { $domain }.
sender-spf-fail = Dihantar dari pelayan yang tidak disenaraikan oleh { $domain }.
sender-spf-none = { $domain } tidak menyenaraikan pelayannya.
sender-check-unsure = Semakan tidak dapat memberikan jawapan yang jelas.
sender-unconfirmed = { $provider } tidak dapat mengesahkan bahawa mel ini datang daripada { $domain }. Sesiapa sahaja boleh menulis apa-apa nama pengirim.
sender-link-title = Buka pautan ini?
sender-link-body = Mel ini gagal dalam semakan pengirimnya. Pautan ini pergi ke { $host }:
sender-link-cancel = Batal
sender-link-open = Buka

## Open and click tracking and read receipts (the eye's popover beside a
## sent message's star, and the line above a read receipt)

tracking-opened = { $who } membukanya { $count } kali, terakhir { $when }
tracking-opens-clicks = { $who } membukanya { $opens } kali dan mengikuti pautan { $clicks } kali, terakhir { $when }
tracking-clicked = { $who } mengikuti pautan { $clicks } kali, terakhir { $when }
tracking-maybe-opened = { $who } mungkin telah membukanya (Apple Mail memuatkan gambar demi privasi)
tracking-seen-none = Belum ada sesiapa yang membukanya atau mengikuti pautan
tracking-receipt = { $who } menghantar resit baca
tracking-receipt-read = { $who } telah membacanya (resit baca), { $when }
tracking-receipt-displayed = Resit baca: { $who } membuka mesej anda
tracking-receipt-other = Resit baca: { $who } memadamkan atau menguruskan mesej anda tanpa membukanya

## Remote images and pictures

remote-hidden = Imej dalam mesej ini disembunyikan.
remote-hidden-unconfirmed = Imej disembunyikan: pengirim tidak dapat disahkan.
remote-hidden-failed = Imej disembunyikan: mel ini gagal dalam semakan pengirimnya.
remote-show = Tunjukkan imej
remote-always-show = Sentiasa tunjukkan daripada pengirim ini
remote-picture-use = Gunakan
remote-picture-too-big = Pilih gambar bersaiz 8 MB atau kurang.
remote-picture-type = Pilih gambar PNG, JPEG, GIF, WebP atau SVG.
remote-picture-read-failed = Tidak dapat membaca gambar: { $error }
remote-picture-keep-failed = Tidak dapat menyimpan gambar: { $error }
remote-picture-remove-failed = Tidak dapat mengalih keluar gambar: { $error }

## Attachments

attachment-count = { $count } lampiran
attachment-save = Simpan
attachment-forward = Majukan
attachment-save-all = Simpan semua
attachment-save-all-tooltip = Simpan setiap lampiran ke dalam folder
attachment-save-here = Simpan di sini
attachment-not-downloaded = Mesej ini tidak dimuat turun.
attachment-open-message = Buka mesej ini untuk membaca lampirannya.
attachment-not-found = Lampiran ini tidak ditemui dalam mesej.
attachment-read-failed = Tidak dapat membaca { $name }
attachment-numbered = lampiran { $number }
attachment-saved-all = { $count } fail disimpan ke { $place }
attachment-saved-some = { $saved } daripada { $total } fail disimpan ke { $place }. Tidak dapat menyimpan { $failed }
attachment-saved-to = Disimpan ke { $path }
attachment-save-failed = Tidak dapat menyimpan { $name }: { $error }
attachment-open-failed = Tidak dapat membuka { $name }: { $error }
attachment-risky = Fail ini boleh menjalankan program, jadi Katna tidak membukanya. Simpan fail ini sahaja.
attachment-encrypted-open = Fail ini diterima dalam bentuk disulitkan. Simpan fail ini untuk membukanya di tempat lain.

## Printing

print-failed = Tidak dapat mencetak: { $error }
print-no-font = tiada fon ditemui
print-opened-as-pdf = Dibuka sebagai PDF untuk dicetak dari situ.
print-preview-title = Pratonton cetakan
print-preview-laying-out = Menyusun halaman…
print-preview-pages = { $count } halaman
print-preview-more = dan { $count } halaman lagi
print-preview-failed = halaman tidak dapat ditunjukkan
print-preview-paper = Kertas
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-layout = Susun atur
print-preview-as-shown = Seperti dipaparkan
print-preview-simple = Teks sahaja
print-preview-backgrounds = Latar belakang
print-preview-cancel = Batal
print-preview-print = Cetak
print-not-downloaded = (Belum dimuat turun.)
print-encrypted = (Disulitkan. Buka dalam Katna Mail untuk mencetak teksnya.)
print-to = Kepada: { $addresses }
print-cc = Sk: { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = Sematkan di atas
text-copy-address = Salin alamat
text-copy = Salin
text-select-all = Pilih semua
