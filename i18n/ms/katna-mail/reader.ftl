# Katna Mail, Malay (Bahasa Melayu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Tutup
reader-back = Kembali
reader-mark-unread = Tandai sebagai belum dibaca
reader-move-to = Alih ke
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
tracking-opened = { $who } membukanya { $count } kali, terakhir { $when }
tracking-opens-clicks = { $who } membukanya { $opens } kali dan mengikuti pautan { $clicks } kali, terakhir { $when }
tracking-clicked = { $who } mengikuti pautan { $clicks } kali, terakhir { $when }
tracking-maybe-opened = { $who } mungkin telah membukanya (Apple Mail memuatkan gambar demi privasi)
tracking-seen-none = Belum ada sesiapa yang membukanya atau mengikuti pautan
tracking-receipt = { $who } menghantar resit baca
tracking-receipt-displayed = Resit baca: { $who } membuka mesej anda
tracking-receipt-other = Resit baca: { $who } memadamkan atau menguruskan mesej anda tanpa membukanya

## Remote images and pictures

remote-hidden = Imej dalam mesej ini disembunyikan.
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
attachment-save-all = Simpan semua
attachment-save-all-tooltip = Simpan setiap lampiran ke dalam folder
attachment-save-here = Simpan di sini
attachment-not-downloaded = Mesej ini tidak dimuat turun.
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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Buka mesej ini untuk membaca lampirannya.
text-copy = Salin
text-select-all = Pilih semua
