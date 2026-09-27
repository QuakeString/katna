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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Buka pesan ini untuk membaca lampirannya.
text-copy = Salin
text-select-all = Pilih semua

## Settings page: its tabs

settings-tab-general = Umum
settings-tab-inbox = Kotak Masuk
settings-tab-accounts = Akun
settings-tab-subscriptions = Langganan
settings-tab-appearance = Tampilan
settings-tab-shortcuts = Pintasan
settings-tab-default-apps = Aplikasi default
settings-tab-folders-rules = Folder & aturan
settings-tab-compose = Tulis
settings-tab-mcp-server = Server MCP
settings-tab-feedback = Masukan pengguna
settings-tab-experimental = Eksperimental

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Lihat buletin dan milis yang Anda terima, lalu berhenti berlangganan dengan satu klik.
settings-tab-folders-rules-coming = Buat, ganti nama, pindahkan, dan sembunyikan folder dan label, serta pilih mana yang disinkronkan. Aturan menyortir, memberi label, meneruskan, atau menghapus email baru secara otomatis, berdasarkan pengirim, subjek, atau kata.
settings-tab-mcp-server-coming = Izinkan asisten AI di komputer ini menelusuri, membaca, dan membuat draf email Anda, dengan persetujuan Anda.

## Settings > General

settings-general-conversations = Tampilan percakapan
settings-general-conversations-group = Kelompokkan balasan untuk email yang sama
settings-general-conversations-group-detail = Satu baris per percakapan di daftar
settings-general-reading = Membaca
settings-general-newest-first = Pesan terbaru di atas
settings-general-newest-first-detail = Percakapan dimulai dengan balasan terbarunya
settings-general-full-headers = Tampilkan header lengkap
settings-general-full-headers-detail = Dari, kepada, cc, tanggal, dan subjek terbuka di setiap pesan
settings-general-full-names = Nama lengkap penerima
settings-general-full-names-detail = “kepada saya, Ada Lovelace”, bukan “kepada saya, Ada”
settings-general-mark-read = Tandai sudah dibaca
settings-general-mark-read-now = Segera setelah dibuka
settings-general-mark-read-1s = Setelah terbuka selama 1 detik
settings-general-mark-read-3s = Setelah terbuka selama 3 detik
settings-general-mark-read-never = Hanya jika saya menandainya sudah dibaca
settings-general-reply-button = Tombol balas
settings-general-reply-all = Balas ke semua orang
settings-general-reply-all-detail = Tombol balas di samping setiap pesan membalas semua, bukan hanya pengirim
settings-general-remote-images = Gambar dari web
settings-general-remote-images-detail = Memuat gambar pesan memberi tahu pengirimnya bahwa Anda membukanya, kapan, dan kira-kira di mana. Jika nonaktif, setiap pesan bertanya dulu, dan Anda selalu dapat menampilkan gambar dari seorang pengirim.
settings-general-remote-images-always = Selalu tampilkan gambar
settings-general-remote-images-always-detail = Di setiap pesan, bukan hanya dari pengirim yang Anda percayai
settings-general-sending = Pengiriman
settings-general-sending-detail = Berapa lama pesan terkirim menunggu, agar bisa ditarik kembali.
settings-general-offline = Email offline
settings-general-offline-detail = Email terbaru didownload seluruhnya, untuk dibaca tanpa koneksi. Email lama didownload saat Anda membukanya.
settings-general-offline-days = { $count } hari
settings-general-offline-years = { $count } tahun
settings-general-offline-all = Semua email
settings-general-offline-note = Memilih lebih sedikit hari tetap menyimpan email yang sudah didownload. Tidak ada yang berubah di server.
settings-general-notifications = Notifikasi
settings-general-notifications-detail = Untuk email baru di Kotak Masuk, bahkan saat Katna Mail ditutup.
settings-general-new-mail = Beri tahu saya tentang email baru
settings-general-new-mail-detail = Dengan Balas semua, Tandai sudah dibaca, dan Arsipkan
settings-general-new-mail-sound = Putar suara
settings-general-new-mail-sound-detail = Suara email baru dari desktop
settings-general-desktop = Desktop
settings-general-open-at-login = Buka Katna Mail saat login
settings-general-open-at-login-detail = Email tetap disinkronkan saat login, selama layanan berjalan
settings-general-tray = Tampilkan Katna di baki sistem
settings-general-tray-detail = Dengan jumlah belum dibaca dan menu
settings-general-unread-badge = Jumlah belum dibaca di ikon taskbar
settings-general-unread-badge-detail = Berapa banyak pesan Kotak Masuk yang belum dibaca

## Settings > Inbox

settings-inbox-tabs = Tab kotak masuk
settings-inbox-tabs-detail = Urutkan kotak masuk ke dalam tab, seperti situs web penyedia email Anda.
settings-inbox-tabs-show = Tampilkan tab kotak masuk
settings-inbox-tabs-show-detail = Jika nonaktif, satu daftar ditampilkan untuk setiap akun
settings-inbox-no-accounts = Tambahkan akun untuk memilih tabnya.
settings-inbox-tabs-automatic = Otomatis: { $tabs } ({ $provider })
settings-inbox-tabs-off = Tanpa tab
settings-inbox-tabs-gmail = Utama, Promosi, Sosial, Pembaruan, Forum
settings-inbox-tabs-focused = Prioritas dan Lainnya
settings-inbox-tabs-zoho = Kotak Masuk, Buletin, dan Notifikasi
settings-inbox-tabs-shown = Tab yang ditampilkan. Email dari tab yang Anda nonaktifkan tetap berada di { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Panel baca
settings-appearance-reading-pane-detail = Tempat percakapan yang dibuka ditampilkan.
settings-appearance-pane-right = Di kanan daftar
settings-appearance-pane-none = Tanpa pemisahan
settings-appearance-density = Kepadatan
settings-appearance-density-default = Default
settings-appearance-density-compact = Ringkas
settings-appearance-scaling = Skala
settings-appearance-scaling-detail = Membuat semua yang ada di Katna Mail lebih besar atau lebih kecil, di atas skala desktop itu sendiri: teks, ikon, jarak, dan garis pemisah. Email yang Anda kirim tetap memakai ukuran fontnya sendiri. Ukuran yang sangat kecil dapat membuat ikon sulit diklik.
settings-appearance-theme = Tema
settings-appearance-theme-system = Sama seperti desktop
settings-appearance-theme-light = Terang
settings-appearance-theme-dark = Gelap
settings-appearance-desktop-colors = Warna desktop
settings-appearance-desktop-colors-use = Gunakan warna desktop
settings-appearance-desktop-colors-use-detail = Skema warna dan warna aksen desktop
settings-appearance-app-names = Nama aplikasi
settings-appearance-app-names-show = Tampilkan nama aplikasi
settings-appearance-app-names-show-detail = Nama di bawah ikon aplikasi di paling kiri
settings-appearance-sender-pictures = Gambar pengirim
settings-appearance-sender-pictures-show = Tampilkan logo perusahaan
settings-appearance-sender-pictures-show-detail = Dicari berdasarkan domain pengirim, tidak pernah berdasarkan pesan, dan disimpan selama seminggu
settings-appearance-important = Penanda penting
settings-appearance-important-show = Tampilkan penanda penting
settings-appearance-important-show-detail = Di samping setiap pesan di daftar
settings-appearance-message-width = Lebar pesan
settings-appearance-message-width-limit = Batasi lebar pesan
settings-appearance-message-width-limit-detail = Baris panjang lebih mudah dibaca di jendela yang lebar
settings-appearance-mail-colors = Warna email
settings-appearance-mail-colors-detail = Sebagian besar email dirancang untuk halaman putih. Dengan tema gelap, warnanya diubah menjadi warna gelap yang mudah dibaca; jika nonaktif, email tetap memakai warna pengirimnya di halaman terang.
settings-appearance-dark-mail = Warna gelap juga untuk email
settings-appearance-dark-mail-detail = Hanya saat tema gelap
settings-appearance-attachment-previews = Pratinjau lampiran
settings-appearance-attachment-previews-show = Tampilkan pratinjau lampiran
settings-appearance-attachment-previews-show-detail = Gambar kecil isi setiap file di kartunya

## Settings > Default apps

settings-default-apps-intro = Tempat lampiran dibuka saat Anda mengkliknya. Penampil juga selalu dapat membuka file di aplikasi lain. Aplikasi default desktop diatur di setelan desktop itu sendiri.
settings-default-apps-pdf = File PDF
settings-default-apps-pdf-detail = Halaman, dengan zoom.
settings-default-apps-pictures = Gambar
settings-default-apps-pictures-detail = Foto (diputar tegak), PNG, GIF, WebP, BMP, TIFF, dan SVG.
settings-default-apps-text = File teks
settings-default-apps-text-detail = Teks biasa, log, kode, dan teks lainnya.
settings-default-apps-sheets = Spreadsheet
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods), dan CSV.
settings-default-apps-documents = Dokumen
settings-default-apps-documents-detail = Word (docx) dan teks OpenDocument (odt).
settings-default-apps-katna = Penampil Katna Mail
settings-default-apps-system = Aplikasi default desktop
settings-default-apps-ask = Tanyakan aplikasi setiap kali
settings-default-apps-after-saving = Setelah menyimpan
settings-default-apps-show-folder = Tampilkan file yang disimpan di foldernya
settings-default-apps-show-folder-detail = Membuka pengelola file dengan lampiran yang disimpan sudah dipilih

## Settings > Compose

settings-compose-send-from = Kirim pesan baru dari
settings-compose-send-from-detail = Balasan dan penerusan selalu dikirim dari akun yang sedang Anda buka.
settings-compose-send-from-current = Akun yang sedang Anda buka
settings-compose-send-on-replies = Kirim pada balasan
settings-compose-send-on-replies-detail = Apa yang dilakukan Kirim pada balasan atau penerusan. Menu di samping Kirim menawarkan pilihan lainnya.
settings-compose-send-plain = Kirim
settings-compose-send-archive = Kirim dan arsipkan
settings-compose-signatures = Tanda tangan
settings-compose-signatures-detail = Ditambahkan di bawah pesan Anda, setelah baris “--”. Pilih tanda tangan lain di jendela tulis.
settings-compose-untitled = Tanpa judul
settings-compose-signature-name = Nama, misalnya Kantor
settings-compose-signature-first = Tanda tangan saya
settings-compose-signature-numbered = Tanda tangan { $number }
settings-compose-signature-delete = Hapus
settings-compose-signature-deleted = Tanda tangan dihapus
settings-compose-signature-new = Buat baru
settings-compose-no-signatures = Belum ada tanda tangan.
settings-compose-no-signature = Tanpa tanda tangan
settings-compose-for-new-mail = Untuk email baru
settings-compose-for-replies = Untuk balasan dan penerusan
settings-compose-for-replies-detail = Dalam percakapan tempat Anda menandatangani pesan, balasan dimulai dengan tanda tangan tersebut.
settings-compose-format = Format
settings-compose-plain-text = Tulis dalam teks biasa
settings-compose-plain-text-detail = Email baru dimulai tanpa format; jendela tulis dapat mengubahnya
settings-compose-spelling = Ejaan
settings-compose-spell-check = Periksa ejaan saat saya menulis
settings-compose-spell-check-detail = Kata yang salah eja digarisbawahi, dengan saran saat klik kanan
settings-compose-spell-desktop = Bahasa desktop ({ $language })
settings-compose-templates = Template
settings-compose-templates-detail = Simpan email yang sering Anda tulis, lalu mulai email baru atau balasan darinya.

## Settings > Shortcuts

settings-shortcuts-set = Set pintasan
settings-shortcuts-set-detail = Mulai dari tombol aplikasi email yang Anda kenal. Cmd di sini adalah Ctrl. Perubahan Anda sendiri tetap berlaku di atas set, dan Pulihkan default kembali ke tombol set.
settings-shortcuts-single = Pintasan satu tombol
settings-shortcuts-single-detail = Tombol tanpa Ctrl atau Alt, seperti di webmail: e mengarsipkan, j dan k berpindah, / menelusuri. Pintasan ini berfungsi di daftar dan percakapan yang terbuka, tidak pernah saat mengetik.
settings-shortcuts-single-use = Gunakan pintasan satu tombol
settings-shortcuts-single-use-detail = Pintasan Ctrl selalu berfungsi
settings-shortcuts-how = Klik tombol untuk mengubahnya, atau + untuk menambahkan, lalu tekan tombol baru. Esc untuk membatalkan.
settings-shortcuts-restore = Pulihkan default
settings-shortcuts-no-key = Tanpa tombol
settings-shortcuts-press = Tekan tombol…
settings-shortcuts-then = { $keys } lalu…
settings-shortcuts-moved = { $keys } kini menjalankan “{ $action }”, bukan “{ $previous }”.
settings-shortcuts-single-off = Pintasan satu tombol nonaktif, jadi tombol ini berfungsi setelah diaktifkan.
settings-shortcuts-restored = Semua pintasan kembali memakai tombol setnya.

## Settings search: the line under a result

settings-general-language-summary = Bahasa aplikasi, tanggal, dan angka
settings-general-reading-summary = Pesan terbaru di atas, header lengkap, nama lengkap penerima
settings-general-mark-read-summary = Kapan percakapan yang dibuka ditandai sudah dibaca: langsung, setelah 1 atau 3 detik, atau secara manual
settings-general-reply-button-summary = Tombol balas di samping setiap pesan membalas ke semua orang
settings-general-remote-images-summary = Selalu tampilkan gambar di setiap pesan
settings-general-sending-summary = Urungkan kirim: berapa lama pesan terkirim menunggu, agar bisa ditarik kembali
settings-general-offline-summary = Berapa hari email terbaru didownload seluruhnya, untuk dibaca tanpa koneksi
settings-general-notifications-summary = Notifikasi email baru dan suaranya
settings-general-desktop-summary = Buka Katna Mail saat login, ikon baki sistem, dan jumlah belum dibaca di ikon taskbar
settings-accounts-accounts-summary = Tambahkan atau hapus akun, atau ubah gambarnya
settings-appearance-density-summary = Baris default atau ringkas di daftar
settings-appearance-scaling-summary = Buat semuanya lebih besar atau lebih kecil: teks, ikon, jarak, dan garis pemisah
settings-appearance-theme-summary = Sama seperti desktop, terang, atau gelap
settings-appearance-sender-pictures-summary = Logo perusahaan, dicari berdasarkan domain pengirim
settings-appearance-important-summary = Penanda penting di samping setiap pesan di daftar
settings-appearance-mail-colors-summary = Warna gelap untuk email HTML dalam tema gelap, atau warna pengirimnya
settings-appearance-attachment-previews-summary = Gambar kecil isi setiap lampiran
settings-shortcuts-set-summary = Mulai dari tombol Gmail, Inbox by Gmail, Apple Mail, Outlook, atau Thunderbird
settings-shortcuts-single-summary = Tombol tanpa Ctrl atau Alt, seperti di webmail
settings-default-apps-pdf-summary = Tempat lampiran PDF dibuka
settings-default-apps-pictures-summary = Tempat foto dan gambar dibuka
settings-default-apps-text-summary = Tempat teks biasa, log, dan kode dibuka
settings-default-apps-sheets-summary = Tempat file Excel, OpenDocument, dan CSV dibuka
settings-default-apps-documents-summary = Tempat teks Word dan OpenDocument dibuka
settings-default-apps-after-saving-summary = Tampilkan lampiran yang disimpan di foldernya
settings-compose-send-from-summary = Akun pengirim email baru: akun yang sedang Anda buka, atau selalu akun yang sama
settings-compose-send-on-replies-summary = Kirim, atau Kirim dan arsipkan percakapan, pada balasan dan penerusan
settings-compose-signatures-summary = Ditambahkan di bawah pesan Anda, setelah baris “--”
settings-compose-for-new-mail-summary = Tanda tangan untuk email baru
settings-compose-for-replies-summary = Tanda tangan untuk balasan dan penerusan
settings-compose-format-summary = Tulis email baru dalam teks biasa
settings-compose-spelling-summary = Periksa ejaan saat menulis, dan bahasa kamus
settings-compose-templates-summary = Segera hadir: simpan email yang sering Anda tulis, lalu mulai email baru atau balasan darinya
settings-feedback-crash-reports-summary = Simpan laporan error di komputer ini saat Katna Mail atau layanan latar belakangnya error
settings-feedback-saved-summary = Lihat, salin, atau hapus laporan error yang disimpan di komputer ini
settings-feedback-help-improve-summary = Kirim laporan error untuk membantu memperbaiki masalah; nonaktif kecuali Anda mengaktifkannya
settings-experimental-blur-summary = Desktop terlihat samar melalui bilah atas, dan menu tampak seperti kaca buram
settings-search-shortcut = Pintasan keyboard
settings-search-tab = Tab setelan
settings-search-none = Tidak ada setelan yang cocok dengan “{ $query }”.
settings-search-results = Setelan yang cocok dengan “{ $query }”

## Quick settings (the panel that slides in from the right)

quick-title = Setelan cepat
quick-see-all = Lihat semua setelan
quick-reading-pane = Panel baca
quick-pane-right = Di kanan daftar
quick-pane-none = Tanpa pemisahan
quick-density = Kepadatan
quick-density-default = Default
quick-density-compact = Ringkas
quick-theme = Tema
quick-theme-system = Sama seperti desktop
quick-theme-light = Terang
quick-theme-dark = Gelap
quick-desktop-colors = Warna desktop
quick-desktop-colors-detail = Skema warna dan warna aksen desktop
quick-app-names = Nama aplikasi
quick-app-names-detail = Nama di bawah ikon aplikasi di paling kiri
quick-inbox-tabs = Tab kotak masuk
quick-inbox-tabs-detail = Tab dari penyedia email setiap akun
quick-choose-tabs = Pilih tab
quick-choose-tabs-detail = Per akun, di Setelan
quick-sending = Pengiriman
quick-undo-send = Urungkan kirim
quick-undo-send-off = Nonaktif
quick-undo-send-seconds = { $seconds } dtk
quick-signatures = Tanda tangan
quick-signatures-none = Belum ada
quick-signatures-one = { $name }, digunakan secara default
quick-signatures-many = { $count } tanda tangan; { $name } secara default
quick-signatures-no-default = { $count }, tanpa default
quick-signature-untitled = Tanpa judul
quick-threading = Rangkaian email
quick-conversation-view = Tampilan percakapan
quick-conversation-view-detail = Kelompokkan balasan untuk email yang sama
quick-help = Bantuan
quick-tour = Ikuti tur
quick-whats-new = Yang baru
quick-about = Tentang Katna

## Settings: opening at login

settings-open-at-login-failed = Tidak dapat mengubah pembukaan saat login: { $error }

## Settings > Appearance > Scaling

scale-letter = A
scale-percent = { $percent }%
scale-reset = Kembali ke { $percent }%

## Settings > Experimental > Look & Feel

look-intro = Fitur yang masih diuji coba. Fitur ini dapat berubah atau dihapus.
look-heading = Tampilan & Nuansa
look-window-frame = Bingkai jendela
look-window-frame-detail = Siapa yang menggambar bilah judul, tombol jendela, sudut, dan bayangan.
look-frame-native-kde = Bawaan: bingkai KDE, dalam tema Plasma Anda
look-frame-native = Bawaan: bingkai desktop
look-frame-katna = Katna: bilah atas menjadi bilah judul
look-frame-katna-note-named = Katna menggambar sudut membulat dan bayangannya sendiri. Bingkai tidak lagi mengikuti tema { $desktop }; aturan jendela tetap berlaku.
look-frame-katna-note = Katna menggambar sudut membulat dan bayangannya sendiri. Bingkai tidak lagi mengikuti tema desktop; aturan jendela tetap berlaku.
look-frame-client-side = Desktop Anda menyerahkan bingkai kepada setiap aplikasi, jadi Katna sudah menggambar bingkainya sendiri.
look-blurred-background = Latar belakang buram
look-blurred-background-detail = Desktop terlihat samar melalui bilah atas dan folder, sedangkan menu dan popover tampak seperti kaca buram.
look-blur = Buramkan yang ada di belakang jendela
look-blur-detail = Email tetap di kartu solid, sehingga teks tetap kontras
look-blur-off-kde = Efek buram KDE nonaktif. Aktifkan Blur di Pengaturan Sistem, Manajemen Jendela, Efek Desktop, lalu buka Katna Mail lagi.
look-blur-none-gnome = GNOME tidak memburamkan yang ada di belakang jendela.
look-blur-none-x11 = Pengelola jendela Anda tidak memburamkan yang ada di belakang jendela.
look-blur-none-wayland = Kompositor Anda tidak memburamkan yang ada di belakang jendela.

## Settings > User feedback (crash reports)

feedback-intro-sending = Laporan error baru dikirim untuk membantu memperbaiki masalah. Tidak ada hal lain yang keluar dari komputer ini.
feedback-intro-local = Katna tidak mengirim apa pun ke mana pun. Laporan error tetap di komputer ini, untuk Anda lihat atau lampirkan ke laporan bug.
feedback-crash-reports = Laporan error
feedback-crash-reports-detail = Ditulis saat Katna Mail atau layanan latar belakangnya error.
feedback-save = Simpan laporan error di komputer ini
feedback-save-detail = Folder utama, nama pengguna dan komputer, serta alamat email Anda tidak disertakan
feedback-saved = Laporan error tersimpan
feedback-saved-detail = { $count } laporan terbaru disimpan.
feedback-help-improve = Bantu tingkatkan Katna
feedback-help-improve-detail = Nonaktif kecuali Anda mengaktifkannya, dan Anda dapat menonaktifkannya di sini kapan saja.
feedback-send = Kirim laporan error
feedback-send-detail = Laporan yang disimpan, persis seperti yang dapat Anda lihat di sini, dikirim ke pelacak error Katna (Sentry, di UE). Tanpa alamat IP, pesan, atau alamat email
feedback-none-saved = Tidak ada laporan error yang disimpan.
feedback-delete-all = Hapus semua
feedback-app-daemon = Layanan latar belakang
feedback-report-sent = { $date } · Terkirim
feedback-view = Lihat
feedback-view-tooltip = Buka laporan
feedback-copy-tooltip = Salin untuk ditempel ke laporan bug
feedback-copied = Laporan error disalin.
feedback-deleted-all = Laporan error dihapus.
feedback-read-failed = Tidak dapat membaca laporan error: { $error }
feedback-delete-failed = Tidak dapat menghapus laporan error: { $error }
feedback-delete-all-failed = Tidak dapat menghapus laporan error: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _Berkas
desktop-menu-new-message = Pesan _Baru
desktop-menu-quit = _Keluar
desktop-menu-edit = _Sunting
desktop-menu-undo = _Urungkan
desktop-menu-select-all = Pilih _Semua
desktop-menu-select-none = _Batalkan Semua Pilihan
desktop-menu-find = _Cari…
desktop-menu-view = _Tampilan
desktop-menu-folder-list = Tampilkan Daftar _Folder
desktop-menu-refresh = _Muat Ulang
desktop-menu-go = _Pergi
desktop-menu-inbox = Kotak _Masuk
desktop-menu-starred = _Berbintang
desktop-menu-sent = _Terkirim
desktop-menu-drafts = _Draf
desktop-menu-all-mail = Semua _Email
desktop-menu-next = Percakapan _Berikutnya
desktop-menu-previous = Percakapan _Sebelumnya
desktop-menu-message = _Pesan
desktop-menu-open = _Buka
desktop-menu-reply = _Balas
desktop-menu-reply-all = Balas _Semua
desktop-menu-forward = _Teruskan
desktop-menu-archive = _Arsipkan
desktop-menu-delete = _Hapus
desktop-menu-spam = Laporkan S_pam
desktop-menu-move-to = Pi_ndahkan ke…
desktop-menu-mark-read = Tandai Sudah _Dibaca
desktop-menu-mark-unread = Tandai Belum Di_baca
desktop-menu-star = Beri B_intang
desktop-menu-important = Tandai _Penting
desktop-menu-not-important = Tandai Tidak Pe_nting
desktop-menu-settings = _Setelan
desktop-menu-quick-settings = Setelan _Cepat
desktop-menu-configure = _Konfigurasi Katna Mail…
desktop-menu-help = _Bantuan
desktop-menu-shortcuts = _Pintasan Keyboard
desktop-menu-whats-new = _Yang Baru
desktop-menu-about = _Tentang Katna

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Berpindah
shortcut-group-actions = Tindakan
shortcut-group-go-to = Buka
shortcut-group-app = Aplikasi

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Percakapan berikutnya
shortcut-previous = Percakapan sebelumnya
shortcut-down = Turun di daftar
shortcut-up = Naik di daftar
shortcut-first = Pertama di daftar
shortcut-last = Terakhir di daftar
shortcut-page-down = Satu halaman ke bawah di daftar
shortcut-page-up = Satu halaman ke atas di daftar
shortcut-open = Buka percakapan
shortcut-back = Kembali ke daftar
shortcut-scroll-down = Gulir ke bawah
shortcut-scroll-up = Gulir ke atas
shortcut-scroll-page-down = Gulir satu halaman ke bawah
shortcut-scroll-page-up = Gulir satu halaman ke atas
shortcut-compose = Tulis
shortcut-reply = Balas
shortcut-reply-all = Balas semua
shortcut-forward = Teruskan
shortcut-archive = Arsipkan
shortcut-delete = Hapus
shortcut-spam = Laporkan spam
shortcut-move-to = Pindahkan ke
shortcut-mark-read = Tandai sudah dibaca
shortcut-mark-unread = Tandai belum dibaca
shortcut-star = Beri atau hapus bintang
shortcut-important = Tandai sebagai penting
shortcut-not-important = Tandai sebagai tidak penting
shortcut-check = Centang percakapan
shortcut-select-all = Centang semua percakapan
shortcut-select-none = Hapus centang semua percakapan
shortcut-undo = Urungkan tindakan terakhir
shortcut-go-inbox = Kotak Masuk
shortcut-go-starred = Berbintang
shortcut-go-sent = Terkirim
shortcut-go-drafts = Draf
shortcut-go-all = Semua Email
shortcut-search = Telusuri email
shortcut-navigation = Tampilkan atau ciutkan menu
shortcut-quick-settings = Setelan cepat
shortcut-settings = Semua setelan
shortcut-shortcuts = Pintasan keyboard
shortcut-reload = Periksa email baru
shortcut-quit = Keluar

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } lalu { $second }

## Settings > Accounts

accounts-folder-pane = Panel folder
accounts-folder-pane-detail = Folder akun mana yang ditampilkan di panel kiri.
accounts-shown-one = Satu akun dalam satu waktu; beralih di kartu akun
accounts-shown-all = Semua akun, satu per satu berurutan
accounts-row = Akun
accounts-row-detail = Menghapus akun akan menghapus salinan email akun tersebut milik Katna di komputer ini. Email tetap ada di server.
accounts-none = Belum ada akun.
accounts-kind-imported = Diimpor
accounts-picture-reset = Gunakan gambar desktop
accounts-picture-change = Ubah gambar
accounts-remove = Hapus
accounts-delete-all-row = Hapus semua data
accounts-delete-all-row-detail = Mulai dari awal, seperti pada instalasi baru.
accounts-delete-all-about = Menghapus setiap akun, semua email, kontak, dan kalender yang tersimpan, indeks penelusuran, setelan Anda, dan sandi yang tersimpan dari komputer ini. Tidak ada yang berubah di server email Anda.
accounts-delete-all-open = Hapus semua data Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } telah dihapus dari Katna.
accounts-removed = { $address } telah dihapus dari Katna. Emailnya masih ada di server.
accounts-all-deleted = Semua data Katna telah dihapus dari komputer ini.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Hapus { $address }?
accounts-remove-confirm = Hapus akun
accounts-removing = Menghapus…
accounts-remove-local-mail = { $folders ->
    [0] Semua email yang diimpor ke akun ini
   *[other] Semua email yang diimpor ke akun ini, di { $folders } foldernya
}
accounts-remove-local-settings = Setelan Katna akun ini
accounts-remove-mail = { $folders ->
    [0] Semua email akun ini yang disimpan oleh Katna
   *[other] Semua email akun ini yang disimpan oleh Katna, di { $folders } foldernya
}
accounts-remove-outbox = Pesan akun ini yang menunggu di kotak keluar
accounts-remove-settings = Sandi tersimpan dan setelan Katna akun ini
accounts-delete-all-title = Hapus semua data Katna?
accounts-delete-all-confirm = Hapus semuanya
accounts-deleting = Menghapus…
accounts-delete-all-accounts = Setiap akun, serta semua email dan lampiran yang disimpan oleh Katna
accounts-delete-all-contacts = Kontak, kalender, dan indeks penelusuran
accounts-delete-all-settings = Semua setelan, tanda tangan, dan pintasan keyboard
accounts-delete-all-passwords = Setiap sandi yang tersimpan
accounts-deleted-heading = Dihapus dari komputer ini:
accounts-cannot-undo = Tindakan ini tidak dapat diurungkan.
accounts-server-delete-all = Tidak ada yang berubah di server email Anda: email Anda tetap di sana, dan menambahkan akun lagi akan mendownloadnya kembali. Email yang diimpor dari file hanya ada di Katna; file-filenya tidak diubah.
accounts-server-local = Email ini diimpor dari file, jadi Katna memiliki satu-satunya salinan. File asalnya tidak diubah; impor lagi untuk mendapatkannya kembali.
accounts-server-remove = Tidak ada yang berubah di server email: email Anda tetap di sana, dan menambahkan akun lagi akan mendownloadnya kembali.
accounts-confirm-word = hapus
accounts-confirm-placeholder = Ketik “{ accounts-confirm-word }”
accounts-confirm-prompt = Untuk mengonfirmasi, ketik “{ accounts-confirm-word }”:
accounts-cancel = Batal
