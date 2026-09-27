# Katna Mail, Malay (Bahasa Melayu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Bahasa: { $language }
language-tooltip-system = Bahasa: { $language }, mengikut sistem
language-search = Cari bahasa
language-system-default = Lalai sistem
language-system-now = Sekarang { $language }
language-no-match = Tiada bahasa yang sepadan dengan “{ $query }”
language-machine = Diterjemahkan oleh mesin. Bantu perbaikinya
language-setting = Bahasa
language-setting-detail = Bahasa menu, butang dan mesej, serta format tarikh dan nombor. Lalai sistem mengikut desktop.

## Dates and sizes

ago-just-now = baru sahaja
ago-minutes = { $count } minit yang lalu
ago-hours = { $count } jam yang lalu
ago-days = { $count } hari yang lalu
size-bytes = { $count } bait
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = Sembunyikan folder
folders-show = Tunjukkan folder
compose = Karang
search = Cari
search-mail = Cari mel
search-settings = Cari tetapan
search-clear = Kosongkan carian
search-options-show = Tunjukkan pilihan carian
settings = Tetapan
account-add = Tambah akaun

## App rail (and the bottom bar on a phone)

rail-mail = Mel
rail-calendar = Kalendar
rail-contacts = Kenalan
rail-tasks = Tugas
rail-notes = Nota
rail-feeds = Suapan

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Akan datang
app-calendar-promise = Kalendar CalDAV anda, jemputan mesyuarat daripada mel anda dan peringatan, di sebelah peti masuk anda.
app-tasks-promise = Senarai tugasan yang disegerakkan dengan CalDAV, dan tugas yang dibuat daripada mel.
app-notes-promise = Nota ringkas, dan nota pada mel atau perbualan untuk kemudian.
app-feeds-promise = Baca suapan RSS dan Atom di sebelah mel anda.

## Contacts page

app-contacts-loading = Mengumpulkan orang daripada mel anda…
app-contacts-empty = Orang yang berutus mel dengan anda akan dipaparkan di sini.
app-contacts-count = { $count } orang daripada mel anda, yang paling kerap berutus mel dahulu
app-contacts-top = { $count } orang teratas daripada mel anda, yang paling kerap berutus mel dahulu
app-contacts-messages = { $count } mesej
app-contacts-last = terakhir { $date }

## Navigation (the folders pane)

nav-labels = Label
nav-folders = Folder
nav-label-new = Cipta label baharu
nav-folder-new = Cipta folder baharu
nav-account-unnamed = Akaun { $number }
nav-tab-new = { $count } baharu

## Special folders (the user's own folders keep their names)

folder-inbox = Peti Masuk
folder-starred = Dibintangi
folder-drafts = Draf
folder-sent = Dihantar
folder-archive = Arkib
folder-spam = Spam
folder-trash = Sampah
folder-all-mail = Semua Mel
folder-scheduled = Dijadualkan

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Label baharu
label-folder-new-title = Folder baharu
label-prompt = Sila masukkan nama label baharu:
label-folder-prompt = Sila masukkan nama folder baharu:
label-name-hint = Nama label
label-folder-name-hint = Nama folder
label-nest = Sarangkan label di bawah:
label-folder-nest = Sarangkan folder di bawah:
label-cancel = Batal
label-create = Cipta
label-creating = Mencipta…
label-created = Label “{ $name }” dicipta.
label-folder-created = Folder “{ $name }” dicipta.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Utama
tab-promotions = Promosi
tab-social = Sosial
tab-updates = Kemas kini
tab-forums = Forum
tab-focused = Difokuskan
tab-other = Lain-lain
tab-inbox = Peti Masuk
tab-newsletters = Surat berita
tab-notifications = Pemberitahuan
tab-new = { $count } baharu
tab-provider-other = diisih oleh Katna

## Mail list: toolbar

list-select = Pilih
list-refresh = Muat semula
list-more = Lagi
list-mark-read = Tandai sebagai dibaca
list-mark-unread = Tandai sebagai belum dibaca
list-move-to = Alih ke
list-archive = Arkibkan
list-spam = Laporkan spam
list-delete = Padam
list-newer = Lebih baharu
list-older = Lebih lama
list-range = { $first }–{ $last } daripada { $total }
list-range-about = { $first }–{ $last } daripada kira-kira { $total }
list-results = Hasil untuk “{ $query }”
list-results-corrected = Menunjukkan hasil untuk “{ $query }”
list-search-instead = Cari “{ $query }” sahaja
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Semua
list-pick-none = Tiada
list-pick-read = Dibaca
list-pick-unread = Belum dibaca
list-pick-starred = Dibintangi
list-pick-unstarred = Tidak dibintangi

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] Semua { $count } perbualan dipilih.
   *[message] Semua { $count } mesej dipilih.
}
list-selected-all-in = { $kind ->
    [conversation] Semua { $count } perbualan dalam { $folder } dipilih.
   *[message] Semua { $count } mesej dalam { $folder } dipilih.
}
list-selected-screen = { $kind ->
    [conversation] Semua { $count } perbualan pada skrin dipilih.
   *[message] Semua { $count } mesej pada skrin dipilih.
}
list-select-all = { $kind ->
    [conversation] Pilih semua { $count } perbualan
   *[message] Pilih semua { $count } mesej
}
list-select-all-in = { $kind ->
    [conversation] Pilih semua { $count } perbualan dalam { $folder }
   *[message] Pilih semua { $count } mesej dalam { $folder }
}
list-clear-selection = Kosongkan pilihan

## Mail list: empty states

list-empty-search = Tiada mesej yang sepadan dengan carian anda.
list-empty-tab = Tiada mel dalam { $tab }.
list-empty-tab-unknown = Tiada mel dalam tab ini.
list-empty-folder = Tiada mesej dalam { $folder }.
list-empty-folder-unknown = Tiada mesej dalam folder ini.
list-first-sync = Mendapatkan mel anda…
list-first-sync-detail = Mel akan dipaparkan di sini sebaik sahaja tiba.

## Mail list: lines

row-removed = Mesej ini telah dialih keluar.
row-starred = Dibintangi
row-not-starred = Tidak dibintangi
row-important = Penting. Klik untuk menandai sebagai tidak penting.
row-mark-important = Tandai sebagai penting
row-pinned = Disematkan di atas
row-pin = Sematkan di atas
row-unpin = Nyahsemat

## Mail list: More menu and right-click menu

menu-reply = Balas
menu-reply-all = Balas semua
menu-forward = Majukan
menu-archive = Arkibkan
menu-delete = Padam
menu-spam = Laporkan spam
menu-mark-read = Tandai sebagai dibaca
menu-mark-unread = Tandai sebagai belum dibaca
menu-mark-all-read = Tandai semua sebagai dibaca
menu-star = Tambah bintang
menu-unstar = Alih keluar bintang
menu-important = Tandai sebagai penting
menu-not-important = Tandai sebagai tidak penting
menu-pin = Sematkan di atas
menu-unpin = Nyahsemat
menu-print-all = Cetak semua
menu-new-window = Buka dalam tetingkap baharu
menu-move-to = Alih ke
menu-move-to-heading = Alih ke:
menu-find-from = Cari e-mel daripada { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count } perbualan diarkibkan.
   *[message] { $count } mesej diarkibkan.
}
toast-trashed = { $kind ->
    [conversation] { $count } perbualan dialihkan ke Sampah.
   *[message] { $count } mesej dialihkan ke Sampah.
}
toast-moved = { $kind ->
    [conversation] { $count } perbualan dialihkan.
   *[message] { $count } mesej dialihkan.
}
toast-starred = { $kind ->
    [conversation] { $count } perbualan dibintangi.
   *[message] { $count } mesej dibintangi.
}
toast-unstarred = { $kind ->
    [conversation] Bintang dialih keluar daripada { $count } perbualan.
   *[message] Bintang dialih keluar daripada { $count } mesej.
}
toast-important = { $kind ->
    [conversation] { $count } perbualan ditandai sebagai penting.
   *[message] { $count } mesej ditandai sebagai penting.
}
toast-not-important = { $kind ->
    [conversation] { $count } perbualan ditandai sebagai tidak penting.
   *[message] { $count } mesej ditandai sebagai tidak penting.
}
toast-pinned = { $kind ->
    [conversation] { $count } perbualan disematkan di atas.
   *[message] { $count } mesej disematkan di atas.
}
toast-unpinned = { $kind ->
    [conversation] { $count } perbualan dinyahsemat.
   *[message] { $count } mesej dinyahsemat.
}
toast-spam = { $kind ->
    [conversation] { $count } perbualan dilaporkan sebagai spam.
   *[message] { $count } mesej dilaporkan sebagai spam.
}
toast-deleted-forever = { $kind ->
    [conversation] { $count } perbualan dipadamkan selama-lamanya.
   *[message] { $count } mesej dipadamkan selama-lamanya.
}
toast-undone = Tindakan dibuat asal.
toast-undo = Buat asal
toast-no-spam-folder = Akaun ini tiada folder spam.

## Reading pane: toolbar

reader-close = Tutup
reader-back = Kembali
reader-mark-unread = Tandai sebagai belum dibaca
reader-move-to = Alih ke
reader-more = Lagi
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
reader-me = saya
reader-to = kepada { $names }
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
print-not-downloaded = (Belum dimuat turun.)
print-encrypted = (Disulitkan. Buka dalam Katna Mail untuk mencetak teksnya.)
print-to = Kepada: { $addresses }
print-cc = Sk: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Buka mesej ini untuk membaca lampirannya.
text-copy = Salin
text-select-all = Pilih semua

## Settings page: its tabs

settings-tab-general = Umum
settings-tab-inbox = Peti Masuk
settings-tab-accounts = Akaun
settings-tab-subscriptions = Langganan
settings-tab-appearance = Penampilan
settings-tab-shortcuts = Pintasan
settings-tab-default-apps = Apl lalai
settings-tab-folders-rules = Folder & peraturan
settings-tab-compose = Karang
settings-tab-mcp-server = Pelayan MCP
settings-tab-feedback = Maklum balas pengguna
settings-tab-experimental = Percubaan

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Lihat surat berita dan senarai mel yang anda terima, dan nyahlanggan dengan satu klik.
settings-tab-folders-rules-coming = Cipta, namakan semula, alihkan dan sembunyikan folder dan label, dan pilih yang mana disegerakkan. Peraturan mengisih, melabel, memajukan atau memadam mel baharu dengan sendirinya, mengikut pengirim, subjek atau perkataan.
settings-tab-mcp-server-coming = Benarkan pembantu AI pada komputer ini mencari, membaca dan mendraf mel anda, dengan kebenaran anda.

## Settings > General

settings-general-conversations = Paparan perbualan
settings-general-conversations-group = Kumpulkan balasan kepada mel yang sama
settings-general-conversations-group-detail = Satu baris bagi setiap perbualan dalam senarai
settings-general-reading = Bacaan
settings-general-newest-first = Mesej terbaharu dahulu
settings-general-newest-first-detail = Perbualan bermula dengan balasan terkininya
settings-general-full-headers = Tunjukkan pengepala penuh
settings-general-full-headers-detail = Daripada, kepada, sk, tarikh dan subjek dibuka pada setiap mesej
settings-general-full-names = Nama penuh penerima
settings-general-full-names-detail = “kepada saya, Ada Lovelace” dan bukannya “kepada saya, Ada”
settings-general-mark-read = Tandai sebagai dibaca
settings-general-mark-read-now = Sebaik sahaja dibuka
settings-general-mark-read-1s = Selepas dibuka selama 1 saat
settings-general-mark-read-3s = Selepas dibuka selama 3 saat
settings-general-mark-read-never = Hanya apabila saya menandainya sebagai dibaca
settings-general-reply-button = Butang balas
settings-general-reply-all = Balas kepada semua orang
settings-general-reply-all-detail = Butang balas di sebelah setiap mesej membalas kepada semua, bukan hanya pengirim
settings-general-remote-images = Imej dari web
settings-general-remote-images-detail = Memuatkan imej mesej memberitahu pengirimnya bahawa anda membukanya, bila, dan lebih kurang di mana. Jika dimatikan, setiap mesej bertanya dahulu, dan anda sentiasa boleh menunjukkan imej daripada seseorang pengirim.
settings-general-remote-images-always = Sentiasa tunjukkan imej
settings-general-remote-images-always-detail = Dalam setiap mesej, bukan hanya daripada pengirim yang anda percayai
settings-general-sending = Penghantaran
settings-general-sending-detail = Berapa lama mesej yang dihantar menunggu, supaya ia boleh ditarik balik.
settings-general-offline = Mel luar talian
settings-general-offline-detail = Mel terkini dimuat turun sepenuhnya, untuk dibaca tanpa sambungan. Mel yang lebih lama dimuat turun apabila anda membukanya.
settings-general-offline-days = { $count } hari
settings-general-offline-years = { $count } tahun
settings-general-offline-all = Semua mel
settings-general-offline-note = Memilih hari yang lebih sedikit mengekalkan mel yang sudah dimuat turun. Tiada apa-apa yang berubah pada pelayan.
settings-general-notifications = Pemberitahuan
settings-general-notifications-detail = Untuk mel baharu dalam Peti Masuk, walaupun semasa Katna Mail ditutup.
settings-general-new-mail = Beritahu saya tentang mel baharu
settings-general-new-mail-detail = Dengan Balas semua, Tandai sebagai dibaca dan Arkibkan
settings-general-new-mail-sound = Mainkan bunyi
settings-general-new-mail-sound-detail = Bunyi mel baharu desktop
settings-general-desktop = Desktop
settings-general-open-at-login = Buka Katna Mail semasa log masuk
settings-general-open-at-login-detail = Mel tetap disegerakkan semasa log masuk, selagi perkhidmatan berjalan
settings-general-tray = Tunjukkan Katna dalam dulang sistem
settings-general-tray-detail = Dengan kiraan belum dibaca dan menu
settings-general-unread-badge = Kiraan belum dibaca pada ikon bar tugas
settings-general-unread-badge-detail = Berapa banyak mesej Peti Masuk yang belum dibaca

## Settings > Inbox

settings-inbox-tabs = Tab peti masuk
settings-inbox-tabs-detail = Isih peti masuk kepada tab, seperti yang dilakukan oleh laman web pembekal mel anda.
settings-inbox-tabs-show = Tunjukkan tab peti masuk
settings-inbox-tabs-show-detail = Jika dimatikan, satu senarai ditunjukkan untuk setiap akaun
settings-inbox-no-accounts = Tambah akaun untuk memilih tabnya.
settings-inbox-tabs-automatic = Automatik: { $tabs } ({ $provider })
settings-inbox-tabs-off = Tiada tab
settings-inbox-tabs-gmail = Utama, Promosi, Sosial, Kemas kini, Forum
settings-inbox-tabs-focused = Difokuskan dan Lain-lain
settings-inbox-tabs-zoho = Peti Masuk, Surat berita dan Pemberitahuan
settings-inbox-tabs-shown = Tab yang ditunjukkan. Mel daripada tab yang anda matikan kekal dalam { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Anak tetingkap bacaan
settings-appearance-reading-pane-detail = Tempat perbualan yang dibuka dipaparkan.
settings-appearance-pane-right = Di kanan senarai
settings-appearance-pane-none = Tiada pemisahan
settings-appearance-density = Kepadatan
settings-appearance-density-default = Lalai
settings-appearance-density-compact = Padat
settings-appearance-scaling = Penskalaan
settings-appearance-scaling-detail = Menjadikan segala-galanya dalam Katna Mail lebih besar atau lebih kecil, di atas skala desktop sendiri: teks, ikon, jarak dan pembahagi. Mel yang anda hantar mengekalkan saiz fonnya sendiri. Saiz yang sangat kecil boleh menyukarkan ikon untuk diklik.
settings-appearance-theme = Tema
settings-appearance-theme-system = Sama seperti desktop
settings-appearance-theme-light = Cerah
settings-appearance-theme-dark = Gelap
settings-appearance-desktop-colors = Warna desktop
settings-appearance-desktop-colors-use = Gunakan warna desktop
settings-appearance-desktop-colors-use-detail = Skema warna dan warna aksen desktop
settings-appearance-app-names = Nama apl
settings-appearance-app-names-show = Tunjukkan nama apl
settings-appearance-app-names-show-detail = Nama di bawah ikon apl di sebelah paling kiri
settings-appearance-sender-pictures = Gambar pengirim
settings-appearance-sender-pictures-show = Tunjukkan logo syarikat
settings-appearance-sender-pictures-show-detail = Dicari mengikut domain pengirim, tidak sekali-kali mengikut mesej, dan disimpan selama seminggu
settings-appearance-important = Penanda Penting
settings-appearance-important-show = Tunjukkan penanda Penting
settings-appearance-important-show-detail = Di sebelah setiap mesej dalam senarai
settings-appearance-message-width = Lebar mesej
settings-appearance-message-width-limit = Hadkan lebar mesej
settings-appearance-message-width-limit-detail = Baris panjang lebih mudah dibaca dalam tetingkap yang lebar
settings-appearance-mail-colors = Warna mel
settings-appearance-mail-colors-detail = Kebanyakan mel direka untuk halaman putih. Dengan tema gelap, warnanya ditukar kepada warna gelap yang mudah dibaca; jika dimatikan, mel mengekalkan warna pengirimnya pada halaman cerah.
settings-appearance-dark-mail = Warna gelap untuk mel juga
settings-appearance-dark-mail-detail = Hanya semasa tema gelap
settings-appearance-attachment-previews = Pratonton lampiran
settings-appearance-attachment-previews-show = Tunjukkan pratonton lampiran
settings-appearance-attachment-previews-show-detail = Gambar kecil kandungan setiap fail pada kadnya

## Settings > Default apps

settings-default-apps-intro = Tempat lampiran dibuka apabila anda mengkliknya. Pemapar juga sentiasa boleh membuka fail dalam apl lain. Apl lalai desktop ditetapkan dalam tetapannya sendiri.
settings-default-apps-pdf = Fail PDF
settings-default-apps-pdf-detail = Halaman, dengan zum.
settings-default-apps-pictures = Gambar
settings-default-apps-pictures-detail = Foto (ditegakkan), PNG, GIF, WebP, BMP, TIFF dan SVG.
settings-default-apps-text = Fail teks
settings-default-apps-text-detail = Teks biasa, log, kod dan teks lain.
settings-default-apps-sheets = Hamparan
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) dan CSV.
settings-default-apps-documents = Dokumen
settings-default-apps-documents-detail = Word (docx) dan teks OpenDocument (odt).
settings-default-apps-katna = Pemapar Katna Mail
settings-default-apps-system = Apl lalai desktop
settings-default-apps-ask = Tanya apl mana setiap kali
settings-default-apps-after-saving = Selepas menyimpan
settings-default-apps-show-folder = Tunjukkan fail yang disimpan dalam foldernya
settings-default-apps-show-folder-detail = Membuka pengurus fail dengan lampiran yang disimpan dipilih

## Settings > Compose

settings-compose-send-from = Hantar mesej baharu daripada
settings-compose-send-from-detail = Balasan dan majuan sentiasa dihantar daripada akaun yang sedang anda gunakan.
settings-compose-send-from-current = Akaun yang sedang anda gunakan
settings-compose-send-on-replies = Hantar pada balasan
settings-compose-send-on-replies-detail = Apa yang dilakukan oleh Hantar pada balasan atau majuan. Menu di sebelah Hantar menawarkan pilihan yang satu lagi.
settings-compose-send-plain = Hantar
settings-compose-send-archive = Hantar dan arkibkan
settings-compose-signatures = Tandatangan
settings-compose-signatures-detail = Ditambah di bawah mesej anda, selepas baris “--”. Pilih yang lain dalam tetingkap karang.
settings-compose-untitled = Tanpa tajuk
settings-compose-signature-name = Nama, seperti Kerja
settings-compose-signature-first = Tandatangan saya
settings-compose-signature-numbered = Tandatangan { $number }
settings-compose-signature-delete = Padam
settings-compose-signature-deleted = Tandatangan dipadamkan
settings-compose-signature-new = Cipta baharu
settings-compose-no-signatures = Belum ada tandatangan.
settings-compose-no-signature = Tiada tandatangan
settings-compose-for-new-mail = Untuk mel baharu
settings-compose-for-replies = Untuk balasan dan majuan
settings-compose-for-replies-detail = Dalam perbualan di mana anda menandatangani sesuatu mesej, balasan bermula dengan tandatangan itu.
settings-compose-format = Format
settings-compose-plain-text = Tulis dalam teks biasa
settings-compose-plain-text-detail = Mel baharu bermula tanpa pemformatan; tetingkap karang boleh menukarnya
settings-compose-spelling = Ejaan
settings-compose-spell-check = Semak ejaan semasa saya menulis
settings-compose-spell-check-detail = Perkataan yang salah eja digariskan, dengan cadangan apabila klik kanan
settings-compose-spell-desktop = Bahasa desktop ({ $language })
settings-compose-templates = Templat
settings-compose-templates-detail = Simpan mel yang kerap anda tulis, dan mulakan mel baharu atau balasan daripadanya.

## Settings > Shortcuts

settings-shortcuts-set = Set pintasan
settings-shortcuts-set-detail = Mulakan daripada kekunci apl mel yang anda kenali. Cmd ialah Ctrl di sini. Perubahan anda sendiri kekal di atas set itu, dan Pulihkan lalai kembali kepada kekunci set tersebut.
settings-shortcuts-single = Pintasan satu kekunci
settings-shortcuts-single-detail = Kekunci tanpa Ctrl atau Alt, seperti dalam mel web: e mengarkibkan, j dan k bergerak, / mencari. Ia berfungsi dalam senarai dan perbualan yang dibuka, tidak sekali-kali semasa menaip.
settings-shortcuts-single-use = Gunakan pintasan satu kekunci
settings-shortcuts-single-use-detail = Pintasan Ctrl sentiasa berfungsi
settings-shortcuts-how = Klik kekunci untuk menukarnya, atau + untuk menambah satu, kemudian tekan kekunci baharu. Esc membatalkan.
settings-shortcuts-restore = Pulihkan lalai
settings-shortcuts-no-key = Tiada kekunci
settings-shortcuts-press = Tekan kekunci…
settings-shortcuts-then = { $keys } kemudian…
settings-shortcuts-moved = { $keys } kini melakukan “{ $action }” dan bukannya “{ $previous }”.
settings-shortcuts-single-off = Pintasan satu kekunci dimatikan, jadi kekunci ini berfungsi sebaik sahaja ia dihidupkan.
settings-shortcuts-restored = Setiap pintasan kembali menggunakan kekunci setnya.

## Settings search: the line under a result

settings-general-language-summary = Bahasa apl, tarikh dan nombor
settings-general-reading-summary = Mesej terbaharu dahulu, pengepala penuh, nama penuh penerima
settings-general-mark-read-summary = Bila perbualan yang dibuka ditandai sebagai dibaca: serta-merta, selepas 1 atau 3 saat, atau secara manual
settings-general-reply-button-summary = Butang balas di sebelah setiap mesej membalas kepada semua orang
settings-general-remote-images-summary = Sentiasa tunjukkan imej dalam setiap mesej
settings-general-sending-summary = Buat asal penghantaran: berapa lama mesej yang dihantar menunggu, supaya ia boleh ditarik balik
settings-general-offline-summary = Berapa hari mel terkini dimuat turun sepenuhnya, untuk dibaca tanpa sambungan
settings-general-notifications-summary = Pemberitahuan mel baharu dan bunyinya
settings-general-desktop-summary = Buka Katna Mail semasa log masuk, ikon dulang sistem dan kiraan belum dibaca pada ikon bar tugas
settings-accounts-accounts-summary = Tambah atau alih keluar akaun, atau tukar gambarnya
settings-appearance-density-summary = Baris lalai atau padat dalam senarai
settings-appearance-scaling-summary = Jadikan segala-galanya lebih besar atau lebih kecil: teks, ikon, jarak dan pembahagi
settings-appearance-theme-summary = Sama seperti desktop, cerah atau gelap
settings-appearance-sender-pictures-summary = Logo syarikat, dicari mengikut domain pengirim
settings-appearance-important-summary = Penanda Penting di sebelah setiap mesej dalam senarai
settings-appearance-mail-colors-summary = Warna gelap untuk mel HTML dalam tema gelap, atau warna pengirimnya
settings-appearance-attachment-previews-summary = Gambar kecil kandungan setiap lampiran
settings-shortcuts-set-summary = Mulakan daripada kekunci Gmail, Inbox by Gmail, Apple Mail, Outlook atau Thunderbird
settings-shortcuts-single-summary = Kekunci tanpa Ctrl atau Alt, seperti dalam mel web
settings-default-apps-pdf-summary = Tempat lampiran PDF dibuka
settings-default-apps-pictures-summary = Tempat foto dan gambar dibuka
settings-default-apps-text-summary = Tempat teks biasa, log dan kod dibuka
settings-default-apps-sheets-summary = Tempat fail Excel, OpenDocument dan CSV dibuka
settings-default-apps-documents-summary = Tempat teks Word dan OpenDocument dibuka
settings-default-apps-after-saving-summary = Tunjukkan lampiran yang disimpan dalam foldernya
settings-compose-send-from-summary = Akaun yang menghantar mel baharu: akaun yang sedang anda gunakan, atau sentiasa akaun yang sama
settings-compose-send-on-replies-summary = Hantar, atau Hantar dan arkibkan perbualan, pada balasan dan majuan
settings-compose-signatures-summary = Ditambah di bawah mesej anda, selepas baris “--”
settings-compose-for-new-mail-summary = Tandatangan yang memulakan mel baharu
settings-compose-for-replies-summary = Tandatangan yang memulakan balasan dan majuan
settings-compose-format-summary = Tulis mel baharu dalam teks biasa
settings-compose-spelling-summary = Semak ejaan semasa menulis, dan bahasa kamus
settings-compose-templates-summary = Akan datang: simpan mel yang kerap anda tulis, dan mulakan mel baharu atau balasan daripadanya
settings-feedback-crash-reports-summary = Simpan laporan ranap pada komputer ini apabila Katna Mail atau perkhidmatan latar belakangnya ranap
settings-feedback-saved-summary = Lihat, salin atau padam laporan ranap yang disimpan pada komputer ini
settings-feedback-help-improve-summary = Hantar laporan ranap untuk membantu membaiki masalah; dimatikan melainkan anda menghidupkannya
settings-experimental-blur-summary = Desktop kelihatan melalui bar atas, dikaburkan, dan menu kelihatan seperti kaca fros
settings-search-shortcut = Pintasan papan kekunci
settings-search-tab = Tab tetapan
settings-search-none = Tiada tetapan yang sepadan dengan “{ $query }”.
settings-search-results = Tetapan yang sepadan dengan “{ $query }”

## Quick settings (the panel that slides in from the right)

quick-title = Tetapan pantas
quick-see-all = Lihat semua tetapan
quick-reading-pane = Anak tetingkap bacaan
quick-pane-right = Di kanan senarai
quick-pane-none = Tiada pemisahan
quick-density = Kepadatan
quick-density-default = Lalai
quick-density-compact = Padat
quick-theme = Tema
quick-theme-system = Sama seperti desktop
quick-theme-light = Cerah
quick-theme-dark = Gelap
quick-desktop-colors = Warna desktop
quick-desktop-colors-detail = Skema warna dan warna aksen desktop
quick-app-names = Nama apl
quick-app-names-detail = Nama di bawah ikon apl di sebelah paling kiri
quick-inbox-tabs = Tab peti masuk
quick-inbox-tabs-detail = Tab pembekal mel setiap akaun
quick-choose-tabs = Pilih tab
quick-choose-tabs-detail = Bagi setiap akaun, dalam Tetapan
quick-sending = Penghantaran
quick-undo-send = Buat asal penghantaran
quick-undo-send-off = Mati
quick-undo-send-seconds = { $seconds } s
quick-signatures = Tandatangan
quick-signatures-none = Belum ada
quick-signatures-one = { $name }, digunakan secara lalai
quick-signatures-many = { $count } tandatangan; { $name } secara lalai
quick-signatures-no-default = { $count }, tiada yang lalai
quick-signature-untitled = Tanpa tajuk
quick-threading = Penjalinan e-mel
quick-conversation-view = Paparan perbualan
quick-conversation-view-detail = Kumpulkan balasan kepada mel yang sama
quick-help = Bantuan
quick-tour = Ikuti lawatan
quick-whats-new = Apa yang baharu
quick-about = Perihal Katna

## Settings: opening at login

settings-open-at-login-failed = Tidak dapat menukar pembukaan semasa log masuk: { $error }

## Settings > Appearance > Scaling

scale-letter = A
scale-percent = { $percent }%
scale-reset = Kembali ke { $percent }%

## Settings > Experimental > Look & Feel

look-intro = Ciri yang masih dicuba. Ciri ini mungkin berubah atau dibuang.
look-heading = Rupa & Rasa
look-window-frame = Bingkai tetingkap
look-window-frame-detail = Siapa yang melukis bar tajuk, butang tetingkap, bucu dan bayang.
look-frame-native-kde = Asli: bingkai KDE, dalam tema Plasma anda
look-frame-native = Asli: bingkai desktop
look-frame-katna = Katna: bar atas menjadi bar tajuk
look-frame-katna-note-named = Katna melukis bucu bulat dan bayangnya sendiri. Bingkai tidak lagi mengikut tema { $desktop }; peraturan tetingkap masih terpakai.
look-frame-katna-note = Katna melukis bucu bulat dan bayangnya sendiri. Bingkai tidak lagi mengikut tema desktop; peraturan tetingkap masih terpakai.
look-frame-client-side = Desktop anda menyerahkan bingkai kepada setiap apl, jadi Katna sudah melukis bingkainya sendiri.
look-blurred-background = Latar belakang kabur
look-blurred-background-detail = Desktop kelihatan melalui bar atas dan folder, dikaburkan, dan menu serta popover menjadi kaca fros.
look-blur = Kaburkan apa yang di belakang tetingkap
look-blur-detail = Mel kekal pada kad padu, jadi teks mengekalkan kontrasnya
look-blur-off-kde = Kesan kabur KDE dimatikan. Hidupkan Kabur dalam Tetapan Sistem, Pengurusan Tetingkap, Kesan Desktop, kemudian buka Katna Mail semula.
look-blur-none-gnome = GNOME tidak mengaburkan apa yang di belakang tetingkap.
look-blur-none-x11 = Pengurus tetingkap anda tidak mengaburkan apa yang di belakang tetingkap.
look-blur-none-wayland = Pengkomposit anda tidak mengaburkan apa yang di belakang tetingkap.

## Settings > User feedback (crash reports)

feedback-intro-sending = Laporan ranap baharu dihantar untuk membantu membaiki masalah. Tiada apa-apa lagi yang keluar dari komputer ini.
feedback-intro-local = Katna tidak menghantar apa-apa ke mana-mana. Laporan ranap kekal pada komputer ini, untuk anda lihat atau lampirkan pada laporan pepijat.
feedback-crash-reports = Laporan ranap
feedback-crash-reports-detail = Ditulis apabila Katna Mail atau perkhidmatan latar belakangnya ranap.
feedback-save = Simpan laporan ranap pada komputer ini
feedback-save-detail = Folder rumah, nama pengguna dan nama komputer serta alamat e-mel anda ditinggalkan
feedback-saved = Laporan ranap yang disimpan
feedback-saved-detail = { $count } laporan terbaharu disimpan.
feedback-help-improve = Bantu perbaiki Katna
feedback-help-improve-detail = Dimatikan melainkan anda menghidupkannya, dan anda boleh mematikannya di sini pada bila-bila masa.
feedback-send = Hantar laporan ranap
feedback-send-detail = Laporan yang disimpan, tepat seperti yang boleh anda lihat di sini, dihantar ke penjejak ranap Katna (Sentry, di EU). Tiada alamat IP, mesej atau alamat e-mel
feedback-none-saved = Tiada laporan ranap disimpan.
feedback-delete-all = Padam semua
feedback-app-daemon = Perkhidmatan latar belakang
feedback-report-sent = { $date } · Dihantar
feedback-view = Lihat
feedback-view-tooltip = Buka laporan
feedback-copy-tooltip = Salin untuk ditampal ke dalam laporan pepijat
feedback-copied = Laporan ranap disalin.
feedback-deleted-all = Laporan ranap dipadamkan.
feedback-read-failed = Tidak dapat membaca laporan ranap: { $error }
feedback-delete-failed = Tidak dapat memadam laporan ranap: { $error }
feedback-delete-all-failed = Tidak dapat memadam laporan ranap: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _Fail
desktop-menu-new-message = Mesej _Baharu
desktop-menu-quit = _Keluar
desktop-menu-edit = _Sunting
desktop-menu-undo = _Buat Asal
desktop-menu-select-all = Pilih _Semua
desktop-menu-select-none = Pilih _Tiada
desktop-menu-find = _Cari…
desktop-menu-view = _Lihat
desktop-menu-folder-list = Tunjukkan Senarai _Folder
desktop-menu-refresh = _Muat Semula
desktop-menu-go = _Pergi
desktop-menu-inbox = _Peti Masuk
desktop-menu-starred = _Dibintangi
desktop-menu-sent = Di_hantar
desktop-menu-drafts = D_raf
desktop-menu-all-mail = Semua _Mel
desktop-menu-next = Perbualan _Seterusnya
desktop-menu-previous = Perbualan Se_belumnya
desktop-menu-message = _Mesej
desktop-menu-open = _Buka
desktop-menu-reply = Ba_las
desktop-menu-reply-all = Balas _Semua
desktop-menu-forward = _Majukan
desktop-menu-archive = _Arkibkan
desktop-menu-delete = _Padam
desktop-menu-spam = Laporkan Spa_m
desktop-menu-move-to = Al_ih Ke…
desktop-menu-mark-read = Tandai sebagai _Dibaca
desktop-menu-mark-unread = Tandai sebagai Bel_um Dibaca
desktop-menu-star = Tambah Bin_tang
desktop-menu-important = Tandai sebagai Pe_nting
desktop-menu-not-important = Tandai sebagai Tidak Pentin_g
desktop-menu-settings = _Tetapan
desktop-menu-quick-settings = Tetapan _Pantas
desktop-menu-configure = _Konfigurasikan Katna Mail…
desktop-menu-help = _Bantuan
desktop-menu-shortcuts = Pintasan Papan _Kekunci
desktop-menu-whats-new = _Apa Yang Baharu
desktop-menu-about = Perihal Ka_tna

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Menavigasi
shortcut-group-actions = Tindakan
shortcut-group-go-to = Pergi ke
shortcut-group-app = Aplikasi

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Perbualan seterusnya
shortcut-previous = Perbualan sebelumnya
shortcut-down = Turun dalam senarai
shortcut-up = Naik dalam senarai
shortcut-first = Pertama dalam senarai
shortcut-last = Terakhir dalam senarai
shortcut-page-down = Satu halaman ke bawah dalam senarai
shortcut-page-up = Satu halaman ke atas dalam senarai
shortcut-open = Buka perbualan
shortcut-back = Kembali ke senarai
shortcut-scroll-down = Tatal ke bawah
shortcut-scroll-up = Tatal ke atas
shortcut-scroll-page-down = Tatal satu halaman ke bawah
shortcut-scroll-page-up = Tatal satu halaman ke atas
shortcut-compose = Karang
shortcut-reply = Balas
shortcut-reply-all = Balas semua
shortcut-forward = Majukan
shortcut-archive = Arkibkan
shortcut-delete = Padam
shortcut-spam = Laporkan spam
shortcut-move-to = Alih ke
shortcut-mark-read = Tandai sebagai dibaca
shortcut-mark-unread = Tandai sebagai belum dibaca
shortcut-star = Tambah atau alih keluar bintang
shortcut-important = Tandai sebagai penting
shortcut-not-important = Tandai sebagai tidak penting
shortcut-check = Tandakan perbualan
shortcut-select-all = Tandakan semua perbualan
shortcut-select-none = Nyahtanda semua perbualan
shortcut-undo = Buat asal tindakan terakhir
shortcut-go-inbox = Peti Masuk
shortcut-go-starred = Dibintangi
shortcut-go-sent = Dihantar
shortcut-go-drafts = Draf
shortcut-go-all = Semua mel
shortcut-search = Cari mel
shortcut-navigation = Tunjukkan atau lipat menu
shortcut-quick-settings = Tetapan pantas
shortcut-settings = Semua tetapan
shortcut-shortcuts = Pintasan papan kekunci
shortcut-reload = Semak mel baharu
shortcut-quit = Keluar

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } kemudian { $second }

## Settings > Accounts

accounts-folder-pane = Anak tetingkap folder
accounts-folder-pane-detail = Folder akaun mana yang ditunjukkan oleh anak tetingkap di sebelah kiri.
accounts-shown-one = Satu akaun pada satu masa; tukar dalam kad akaun
accounts-shown-all = Semua akaun, satu demi satu
accounts-row = Akaun
accounts-row-detail = Mengalih keluar akaun akan memadamkan salinan mel akaun itu yang disimpan oleh Katna pada komputer ini. Mel kekal pada pelayan.
accounts-none = Belum ada akaun.
accounts-kind-imported = Diimport
accounts-picture-reset = Gunakan gambar desktop
accounts-picture-change = Tukar gambar
accounts-remove = Alih keluar
accounts-delete-all-row = Padam semua data
accounts-delete-all-row-detail = Mulakan semula, seperti pada pemasangan baharu.
accounts-delete-all-about = Memadamkan setiap akaun, semua mel yang disimpan, kenalan dan kalendar, indeks carian, tetapan anda dan kata laluan yang disimpan daripada komputer ini. Tiada apa-apa yang berubah pada pelayan mel anda.
accounts-delete-all-open = Padam semua data Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } telah dialih keluar daripada Katna.
accounts-removed = { $address } telah dialih keluar daripada Katna. Melnya masih ada pada pelayan.
accounts-all-deleted = Semua data Katna telah dipadamkan daripada komputer ini.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Alih keluar { $address }?
accounts-remove-confirm = Alih keluar akaun
accounts-removing = Mengalih keluar…
accounts-remove-local-mail = { $folders ->
    [0] Semua mel yang diimport ke dalam akaun ini
   *[other] Semua mel yang diimport ke dalam akaun ini, dalam { $folders } folder
}
accounts-remove-local-settings = Tetapan Katna akaun ini
accounts-remove-mail = { $folders ->
    [0] Semua mel akaun ini yang disimpan oleh Katna
   *[other] Semua mel akaun ini yang disimpan oleh Katna, dalam { $folders } folder
}
accounts-remove-outbox = Mesej akaun ini yang menunggu dalam peti keluar
accounts-remove-settings = Kata laluan yang disimpan dan tetapan Katna akaun ini
accounts-delete-all-title = Padam semua data Katna?
accounts-delete-all-confirm = Padam semuanya
accounts-deleting = Memadam…
accounts-delete-all-accounts = Setiap akaun, dan semua mel serta lampiran yang disimpan oleh Katna
accounts-delete-all-contacts = Kenalan, kalendar dan indeks carian
accounts-delete-all-settings = Semua tetapan, tandatangan dan pintasan papan kekunci
accounts-delete-all-passwords = Setiap kata laluan yang disimpan
accounts-deleted-heading = Dipadamkan daripada komputer ini:
accounts-cannot-undo = Tindakan ini tidak boleh dibuat asal.
accounts-server-delete-all = Tiada apa-apa yang berubah pada pelayan mel anda: mel anda kekal di sana, dan menambah akaun sekali lagi akan memuat turunnya semula. Mel yang diimport daripada fail hanya ada dalam Katna; fail itu tidak disentuh.
accounts-server-local = Mel ini diimport daripada fail, jadi Katna mempunyai satu-satunya salinan. Fail asalnya tidak disentuh; import fail itu semula untuk mendapatkan mel ini kembali.
accounts-server-remove = Tiada apa-apa yang berubah pada pelayan mel: mel anda kekal di sana, dan menambah akaun ini sekali lagi akan memuat turunnya semula.
accounts-confirm-word = padam
accounts-confirm-placeholder = Taip “{ accounts-confirm-word }”
accounts-confirm-prompt = Untuk mengesahkan, taip “{ accounts-confirm-word }”:
accounts-cancel = Batal
