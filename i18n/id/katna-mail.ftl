# Katna Mail, Indonesian (Bahasa Indonesia).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Bahasa: { $language }
language-tooltip-system = Bahasa: { $language }, mengikuti sistem
language-search = Telusuri bahasa
language-system-default = Default sistem
language-system-now = Saat ini { $language }
language-no-match = Tidak ada bahasa yang cocok dengan “{ $query }”
language-machine = Diterjemahkan oleh mesin. Bantu tingkatkan
language-setting = Bahasa
language-setting-detail = Bahasa menu, tombol, dan pesan, serta format tanggal dan angka. Default sistem mengikuti desktop.

## Dates and sizes

ago-just-now = baru saja
ago-minutes = { $count } menit yang lalu
ago-hours = { $count } jam yang lalu
ago-days = { $count } hari yang lalu
size-bytes = { $count } byte
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = Sembunyikan folder
folders-show = Tampilkan folder
compose = Tulis
search = Telusuri
search-mail = Telusuri email
search-settings = Telusuri setelan
search-clear = Hapus penelusuran
search-options-show = Tampilkan opsi penelusuran
settings = Setelan
account-add = Tambahkan akun

## App rail (and the bottom bar on a phone)

rail-mail = Email
rail-calendar = Kalender
rail-contacts = Kontak
rail-tasks = Tugas
rail-notes = Catatan
rail-feeds = Feed

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Segera hadir
app-calendar-promise = Kalender CalDAV, undangan rapat dari email, dan pengingat Anda, di samping kotak masuk.
app-tasks-promise = Daftar tugas yang disinkronkan dengan CalDAV, dan tugas yang dibuat dari email.
app-notes-promise = Catatan singkat, dan catatan pada email atau percakapan untuk nanti.
app-feeds-promise = Baca feed RSS dan Atom di samping email Anda.

## Contacts page

app-contacts-loading = Mengumpulkan orang dari email Anda…
app-contacts-empty = Orang yang berkirim email dengan Anda akan muncul di sini.
app-contacts-count = { $count } orang dari email Anda, yang paling sering berkirim email ditampilkan lebih dulu
app-contacts-top = { $count } orang teratas dari email Anda, yang paling sering berkirim email ditampilkan lebih dulu
app-contacts-messages = { $count } pesan
app-contacts-last = terakhir { $date }

## Navigation (the folders pane)

nav-labels = Label
nav-folders = Folder
nav-label-new = Buat label baru
nav-folder-new = Buat folder baru
nav-account-unnamed = Akun { $number }
nav-tab-new = { $count } baru

## Special folders (the user's own folders keep their names)

folder-inbox = Kotak Masuk
folder-starred = Berbintang
folder-drafts = Draf
folder-sent = Terkirim
folder-archive = Arsip
folder-spam = Spam
folder-trash = Sampah
folder-all-mail = Semua Email
folder-scheduled = Terjadwal

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Label baru
label-folder-new-title = Folder baru
label-prompt = Masukkan nama label baru:
label-folder-prompt = Masukkan nama folder baru:
label-name-hint = Nama label
label-folder-name-hint = Nama folder
label-nest = Tempatkan label di bawah:
label-folder-nest = Tempatkan folder di bawah:
label-cancel = Batal
label-create = Buat
label-creating = Membuat…
label-created = Label “{ $name }” dibuat.
label-folder-created = Folder “{ $name }” dibuat.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Utama
tab-promotions = Promosi
tab-social = Sosial
tab-updates = Pembaruan
tab-forums = Forum
tab-focused = Prioritas
tab-other = Lainnya
tab-inbox = Kotak Masuk
tab-newsletters = Buletin
tab-notifications = Notifikasi
tab-new = { $count } baru
tab-provider-other = diurutkan oleh Katna

## Mail list: toolbar

list-select = Pilih
list-refresh = Muat ulang
list-more = Lainnya
list-mark-read = Tandai sudah dibaca
list-mark-unread = Tandai belum dibaca
list-move-to = Pindahkan ke
list-archive = Arsipkan
list-spam = Laporkan spam
list-delete = Hapus
list-newer = Lebih baru
list-older = Lebih lama
list-range = { $first }–{ $last } dari { $total }
list-range-about = { $first }–{ $last } dari sekitar { $total }
list-results = Hasil untuk “{ $query }”
list-results-corrected = Menampilkan hasil untuk “{ $query }”
list-search-instead = Telusuri “{ $query }” saja
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Semua
list-pick-none = Tidak ada
list-pick-read = Sudah dibaca
list-pick-unread = Belum dibaca
list-pick-starred = Berbintang
list-pick-unstarred = Tidak berbintang

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] Semua { $count } percakapan dipilih.
   *[message] Semua { $count } pesan dipilih.
}
list-selected-all-in = { $kind ->
    [conversation] Semua { $count } percakapan di { $folder } dipilih.
   *[message] Semua { $count } pesan di { $folder } dipilih.
}
list-selected-screen = { $kind ->
    [conversation] Semua { $count } percakapan di layar dipilih.
   *[message] Semua { $count } pesan di layar dipilih.
}
list-select-all = { $kind ->
    [conversation] Pilih semua { $count } percakapan
   *[message] Pilih semua { $count } pesan
}
list-select-all-in = { $kind ->
    [conversation] Pilih semua { $count } percakapan di { $folder }
   *[message] Pilih semua { $count } pesan di { $folder }
}
list-clear-selection = Hapus pilihan

## Mail list: empty states

list-empty-search = Tidak ada pesan yang cocok dengan penelusuran Anda.
list-empty-tab = Tidak ada email di { $tab }.
list-empty-tab-unknown = Tidak ada email di tab ini.
list-empty-folder = Tidak ada pesan di { $folder }.
list-empty-folder-unknown = Tidak ada pesan di folder ini.
list-first-sync = Mengambil email Anda…
list-first-sync-detail = Email akan muncul di sini saat tiba.

## Mail list: lines

row-removed = Pesan ini telah dihapus.
row-starred = Berbintang
row-not-starred = Tidak berbintang
row-important = Penting. Klik untuk menandai sebagai tidak penting.
row-mark-important = Tandai sebagai penting
row-pinned = Disematkan di atas
row-pin = Sematkan di atas
row-unpin = Lepas sematan

## Mail list: More menu and right-click menu

menu-reply = Balas
menu-reply-all = Balas semua
menu-forward = Teruskan
menu-archive = Arsipkan
menu-delete = Hapus
menu-spam = Laporkan spam
menu-mark-read = Tandai sudah dibaca
menu-mark-unread = Tandai belum dibaca
menu-mark-all-read = Tandai semua sudah dibaca
menu-star = Tambahkan bintang
menu-unstar = Hapus bintang
menu-important = Tandai sebagai penting
menu-not-important = Tandai sebagai tidak penting
menu-pin = Sematkan di atas
menu-unpin = Lepas sematan
menu-print-all = Cetak semua
menu-new-window = Buka di jendela baru
menu-move-to = Pindahkan ke
menu-move-to-heading = Pindahkan ke:
menu-find-from = Cari email dari { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count } percakapan diarsipkan.
   *[message] { $count } pesan diarsipkan.
}
toast-trashed = { $kind ->
    [conversation] { $count } percakapan dipindahkan ke Sampah.
   *[message] { $count } pesan dipindahkan ke Sampah.
}
toast-moved = { $kind ->
    [conversation] { $count } percakapan dipindahkan.
   *[message] { $count } pesan dipindahkan.
}
toast-starred = { $kind ->
    [conversation] { $count } percakapan diberi bintang.
   *[message] { $count } pesan diberi bintang.
}
toast-unstarred = { $kind ->
    [conversation] Bintang dihapus dari { $count } percakapan.
   *[message] Bintang dihapus dari { $count } pesan.
}
toast-important = { $kind ->
    [conversation] { $count } percakapan ditandai sebagai penting.
   *[message] { $count } pesan ditandai sebagai penting.
}
toast-not-important = { $kind ->
    [conversation] { $count } percakapan ditandai sebagai tidak penting.
   *[message] { $count } pesan ditandai sebagai tidak penting.
}
toast-pinned = { $kind ->
    [conversation] { $count } percakapan disematkan di atas.
   *[message] { $count } pesan disematkan di atas.
}
toast-unpinned = { $kind ->
    [conversation] Sematan { $count } percakapan dilepas.
   *[message] Sematan { $count } pesan dilepas.
}
toast-spam = { $kind ->
    [conversation] { $count } percakapan dilaporkan sebagai spam.
   *[message] { $count } pesan dilaporkan sebagai spam.
}
toast-deleted-forever = { $kind ->
    [conversation] { $count } percakapan dihapus selamanya.
   *[message] { $count } pesan dihapus selamanya.
}
toast-undone = Tindakan diurungkan.
toast-undo = Urungkan
toast-no-spam-folder = Akun ini tidak memiliki folder spam.

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
