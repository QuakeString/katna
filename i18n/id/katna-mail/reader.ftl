# Katna Mail, Indonesian (Bahasa Indonesia).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Tutup
reader-back = Kembali
reader-mark-unread = Tandai belum dibaca
reader-move-to = Pindahkan ke
reader-more = Lainnya
reader-print-all = Cetak semua
reader-new-window = Di jendela baru
reader-position = { $position } dari { $total }
reader-newer = Lebih baru
reader-older = Lebih lama

## Reading pane: the conversation

reader-removed = Percakapan ini telah dihapus.
reader-no-subject = (tanpa subjek)
reader-collapse-all = Ciutkan semua
reader-expand-all = Luaskan semua
reader-unknown-sender = (pengirim tidak dikenal)
reader-date-ago = { $date } ({ $ago })
reader-me = saya
reader-to = kepada { $names }
reader-starred = Berbintang
reader-not-starred = Tidak berbintang
reader-too-long = Pesan terlalu panjang untuk ditampilkan seluruhnya.
reader-encrypted-images = Gambar dari web tidak pernah dimuat dalam email terenkripsi.
reader-window-failed = Tidak dapat membuka jendela baru.

## Reading pane: message details (opened from "to me")

reader-details-from = dari:
reader-details-to = kepada:
reader-details-cc = cc:
reader-details-date = tanggal:
reader-details-subject = subjek:

## Reading pane: downloading a message

reader-downloading = Mendownload pesan ini dari server…
reader-download-failed = Tidak dapat mendownload pesan ini.
reader-try-again = Coba lagi

## Reply row

reply-reply = Balas
reply-reply-all = Balas semua
reply-forward = Teruskan

## Encrypted and signed mail

security-decrypting = Mendekripsi…
security-checking = Memeriksa tanda tangan…
security-partly-encrypted = Hanya sebagian pesan ini yang dienkripsi. Sisanya ditambahkan di luar perlindungan dan bisa berasal dari siapa saja.
security-partly-signed = Hanya sebagian pesan ini yang ditandatangani. Sisanya ditambahkan di luar perlindungan dan bisa berasal dari siapa saja.
security-encrypted = Pesan terenkripsi
security-encrypted-smime = Pesan terenkripsi (S/MIME)
security-no-key = Tidak dapat mendekripsi pesan ini: pesan dienkripsi untuk kunci yang tidak Anda miliki.
security-cancelled = Dekripsi dibatalkan.
security-damaged = Tidak dapat mendekripsi pesan ini: data terenkripsi rusak atau telah diubah.
security-decrypt-unavailable = Tidak dapat mendekripsi pesan ini: instal { $tool } untuk membaca email terenkripsi.
security-decrypt-failed = Tidak dapat mendekripsi pesan ini: { $reason }
security-unknown-signer = penanda tangan tidak dikenal
security-signed-verified = Ditandatangani oleh { $signer } · terverifikasi
security-signed-not-sender = Ditandatangani oleh { $signer }, yang bukan pengirimnya
security-signed-untrusted = Ditandatangani oleh { $signer }, dengan kunci yang Anda tandai tidak tepercaya
security-signed-unverified = Ditandatangani oleh { $signer } · kunci belum diverifikasi
security-bad-signature = Tanda tangan buruk: pesan ini diubah setelah ditandatangani, atau tanda tangannya dipalsukan.
security-signature-expired = Ditandatangani oleh { $signer } · tanda tangan telah kedaluwarsa
security-key-expired = Ditandatangani oleh { $signer } · kunci telah kedaluwarsa sejak itu
security-key-revoked = Ditandatangani oleh { $signer } dengan kunci yang telah dicabut
security-missing-key = Ditandatangani dengan kunci yang tidak Anda miliki, jadi tidak dapat diperiksa
security-missing-key-id = Ditandatangani dengan kunci yang tidak Anda miliki ({ $key }), jadi tidak dapat diperiksa
security-signature-unavailable = Ditandatangani; instal { $tool } untuk memeriksa tanda tangan
security-signature-error = Tanda tangan tidak dapat diperiksa.

## Remote images and pictures

remote-hidden = Gambar dalam pesan ini disembunyikan.
remote-show = Tampilkan gambar
remote-always-show = Selalu tampilkan dari pengirim ini
remote-picture-use = Gunakan
remote-picture-too-big = Pilih gambar berukuran 8 MB atau kurang.
remote-picture-type = Pilih gambar PNG, JPEG, GIF, WebP, atau SVG.
remote-picture-read-failed = Tidak dapat membaca gambar: { $error }
remote-picture-keep-failed = Tidak dapat menyimpan gambar: { $error }
remote-picture-remove-failed = Tidak dapat menghapus gambar: { $error }

## Attachments

attachment-count = { $count } lampiran
attachment-save = Simpan
attachment-save-all = Simpan semua
attachment-save-all-tooltip = Simpan semua lampiran ke folder
attachment-save-here = Simpan di sini
attachment-not-downloaded = Pesan ini tidak didownload.
attachment-not-found = Lampiran ini tidak ditemukan di dalam pesan.
attachment-read-failed = Tidak dapat membaca { $name }
attachment-numbered = lampiran { $number }
attachment-saved-all = { $count } file disimpan ke { $place }
attachment-saved-some = { $saved } dari { $total } file disimpan ke { $place }. Tidak dapat menyimpan { $failed }
attachment-saved-to = Disimpan ke { $path }
attachment-save-failed = Tidak dapat menyimpan { $name }: { $error }
attachment-open-failed = Tidak dapat membuka { $name }: { $error }
attachment-risky = File ini dapat menjalankan program, jadi Katna tidak membukanya. Simpan saja file ini.
attachment-encrypted-open = File ini dikirim terenkripsi. Simpan file ini untuk membukanya di aplikasi lain.

## Printing

print-failed = Tidak dapat mencetak: { $error }
print-no-font = font tidak ditemukan
print-opened-as-pdf = Dibuka sebagai PDF untuk dicetak dari sana.
print-not-downloaded = (Belum didownload.)
print-encrypted = (Terenkripsi. Buka di Katna Mail untuk mencetak teksnya.)
print-to = Kepada: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Buka pesan ini untuk membaca lampirannya.
text-copy = Salin
text-select-all = Pilih semua
