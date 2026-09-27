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
