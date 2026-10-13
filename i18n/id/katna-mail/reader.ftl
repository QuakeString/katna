# Katna Mail, Indonesian (Bahasa Indonesia).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Tutup
reader-back = Kembali
reader-mark-unread = Tandai belum dibaca
reader-move-to = Pindahkan ke
reader-snooze = Tunda
reader-remind = Ingatkan saya
reader-more = Lainnya
reader-original-colors = Tampilkan warna asli
reader-dark-colors = Tampilkan dengan warna gelap
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
reader-sending = Mengirim…
reader-me = saya
reader-to = kepada { $names }
reader-to-label = kepada
reader-tick-delivered = Sudah sampai { $when }
reader-tick-no-bounce = Dikirim { $when }; tidak ada pemberitahuan gagal kirim yang kembali, jadi hampir pasti sudah sampai
reader-tick-bounced = Tidak sampai: gagal kirim { $when }
reader-tick-read = Dibaca { $when } (tanda terima baca)
reader-tick-opened = Dibuka, terakhir { $when } (pelacakan buka)
reader-starred = Berbintang
reader-chip-remove = Hapus { $label }
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
reader-download-failed-reason = Tidak dapat mengunduh pesan ini. { $reason }
reader-download-offline = Akun ini sedang offline. Hubungkan ke internet untuk mendownload pesan ini.
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
security-look-up-key = Cari kunci

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = Tanda tangan terverifikasi
key-card-verified-detail = Tanda tangannya valid dan Anda memercayai kunci ini.
key-card-unverified = Tanda tangan belum diverifikasi
key-card-unverified-detail = Tanda tangannya valid, tetapi tidak ada yang memastikan kunci ini milik mereka. Bandingkan sidik jarinya dengan mereka, lalu percayai kunci itu di GnuPG (Kleopatra atau gpg --edit-key).
key-card-not-sender = Ditandatangani oleh orang lain
key-card-not-sender-detail = Tanda tangannya valid, tetapi kuncinya bukan milik pengirim.
key-card-untrusted = Kunci tidak tepercaya
key-card-untrusted-detail = Anda menandai kunci ini tidak tepercaya di GnuPG.
key-card-signature-expired = Tanda tangan kedaluwarsa
key-card-signature-expired-detail = Tanda tangannya dulu valid, tetapi sudah kedaluwarsa.
key-card-key-expired = Kunci kedaluwarsa
key-card-key-expired-detail = Tanda tangannya valid, tetapi kuncinya telah kedaluwarsa sejak itu.
key-card-key-revoked = Kunci dicabut
key-card-key-revoked-detail = Pemiliknya telah mencabut kunci ini, jadi tanda tangannya tidak dapat dipercaya.
key-card-bad = Tanda tangan buruk
key-card-bad-detail = Pesan ini diubah setelah ditandatangani, atau tanda tangannya dipalsukan.
key-card-signed-by = Ditandatangani oleh
key-card-belongs-to = Milik
key-card-fingerprint = Sidik jari
key-card-signed = Ditandatangani
key-card-key = Kunci
key-card-kind = { $standard }, { $algorithm }
key-card-created = Dibuat
key-card-expires = Kedaluwarsa
key-card-never = Tidak pernah
key-card-issued-by = Diterbitkan oleh
key-card-found-in = Ditemukan di
key-card-keyring = Keyring GnuPG Anda
key-card-copy = Salin sidik jari
key-card-import-title = Impor kunci ini?
key-card-from-directory = Ditemukan di direktori kunci { $domain }.
key-card-from-attachment = Dari lampiran { $name }.
key-card-import-note = Katna kemudian dapat memeriksa tanda tangan orang ini dan mengenkripsi email untuknya. Untuk memercayai kunci ini sepenuhnya, bandingkan sidik jarinya dengan mereka.
key-card-cancel = Batal
key-card-import = Impor kunci
key-card-looking-up = Mencari kunci…
key-card-looking-up-detail = Menanyakan direktori kunci { $domain }.
key-card-not-found = Kunci tidak ditemukan
key-card-not-found-detail = { $domain } tidak memublikasikan kunci untuk alamat ini. Minta pengirim mengirimkan kuncinya kepada Anda.
key-card-not-kept = Kunci yang ditemukan tidak dapat digunakan.
key-card-failed = Tidak dapat mengambil kunci

## Sender checks: the banner on mail that failed the checks the user's mail
## provider ran on its sender (DMARC, DKIM, SPF), and the "?" on the picture
## of a sender nothing confirmed

sender-failed-title = Ini mungkin bukan dari { $domain }
sender-failed-body = Email ini gagal dalam pemeriksaan pengirim { $provider }. Berhati-hatilah dengan tautan, lampiran, dan balasan.
sender-provider-unknown = penyedia email Anda
sender-details = Detail
sender-details-hide = Sembunyikan detail
sender-looks-safe = Tampak aman
sender-move-to-spam = Pindahkan ke Spam
sender-checked-by = Diperiksa oleh { $provider }
sender-checked-by-server = Diperiksa oleh { $provider } ({ $server })
sender-dmarc = Domain pengirim (DMARC)
sender-dkim = Tanda tangan (DKIM)
sender-spf = Server pengirim (SPF)
sender-result-pass = Lolos
sender-result-fail = Gagal
sender-result-unsure = Tidak pasti
sender-result-none = Tidak ada
sender-result-missing = Tidak diperiksa
sender-dmarc-pass = { $domain } mengonfirmasi pengirim ini.
sender-dmarc-fail = Email ini tidak cocok dengan cara pengiriman email yang dinyatakan { $domain }.
sender-dmarc-none = { $domain } tidak menerbitkan aturan untuk emailnya.
sender-dkim-pass = Ditandatangani oleh { $domain }.
sender-dkim-fail = Tanda tangan dari { $domain } tidak cocok dengan email ini.
sender-dkim-none = Pesan ini tidak ditandatangani.
sender-spf-pass = Dikirim dari server yang terdaftar oleh { $domain }.
sender-spf-fail = Dikirim dari server yang tidak terdaftar oleh { $domain }.
sender-spf-none = { $domain } tidak mendaftarkan servernya.
sender-check-unsure = Pemeriksaan tidak dapat memberikan jawaban yang jelas.
sender-unconfirmed = Menurut { $provider }, email ini tidak dapat dipastikan berasal dari { $domain }. Siapa pun bisa menulis pengirim apa saja.
sender-link-title = Buka tautan ini?
sender-link-body = Email ini gagal dalam pemeriksaan pengirim. Tautan ini menuju ke { $host }:
sender-link-cancel = Batal
sender-link-open = Buka

## Open and click tracking and read receipts (the eye's popover beside a
## sent message's star, and the line above a read receipt)

tracking-opened = { $who } membukanya { $count } kali, terakhir { $when }
tracking-opens-clicks = { $who } membukanya { $opens } kali dan mengikuti link { $clicks } kali, terakhir { $when }
tracking-clicked = { $who } mengikuti link { $clicks } kali, terakhir { $when }
tracking-maybe-opened = { $who } mungkin sudah membukanya (Apple Mail memuat gambar demi privasi)
tracking-seen-none = Belum ada yang membukanya atau mengikuti link
tracking-receipt = { $who } mengirim tanda terima baca
tracking-receipt-read = { $who } sudah membacanya (tanda terima baca), { $when }
tracking-receipt-displayed = Tanda terima baca: { $who } membuka pesan Anda
tracking-receipt-other = Tanda terima baca: { $who } menghapus atau menangani pesan Anda tanpa membukanya

## Remote images and pictures

remote-hidden = Gambar dalam pesan ini disembunyikan.
remote-hidden-unconfirmed = Gambar disembunyikan: pengirim tidak dapat dikonfirmasi.
remote-hidden-failed = Gambar disembunyikan: email ini gagal dalam pemeriksaan pengirim.
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
attachment-forward = Teruskan
attachment-save-all = Simpan semua
attachment-save-all-tooltip = Simpan semua lampiran ke folder
attachment-save-here = Simpan di sini
attachment-not-downloaded = Pesan ini tidak didownload.
attachment-open-message = Buka pesan ini untuk membaca lampirannya.
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
print-preview-title = Pratinjau cetak
print-preview-laying-out = Menata halaman…
print-preview-pages = { $count } halaman
print-preview-more = dan { $count } halaman lagi
print-preview-failed = halaman tidak dapat ditampilkan
print-preview-paper = Kertas
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-layout = Tata letak
print-preview-as-shown = Seperti ditampilkan
print-preview-simple = Teks sederhana
print-preview-backgrounds = Latar belakang
print-preview-cancel = Batal
print-preview-print = Cetak
print-not-downloaded = (Belum didownload.)
print-encrypted = (Terenkripsi. Buka di Katna Mail untuk mencetak teksnya.)
print-to = Kepada: { $addresses }
print-cc = Cc: { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = Sematkan di atas
text-copy-address = Salin alamat
text-copy = Salin
text-select-all = Pilih semua
