# Katna Mail, Malay (Bahasa Melayu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
