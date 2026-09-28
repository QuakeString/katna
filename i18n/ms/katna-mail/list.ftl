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
list-checking = Menyemak mel baharu…
list-more = Lagi
list-mark-read = Tandai sebagai dibaca
list-mark-unread = Tandai sebagai belum dibaca
list-move-to = Alih ke
list-archive = Arkibkan
list-spam = Laporkan spam
list-delete = Padam
list-snooze = Tunda
list-unsnooze = Nyahtunda
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
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] Semua { $count } perbualan dibaca pada skrin dipilih.
       *[message] Semua { $count } mesej dibaca pada skrin dipilih.
    }
   *[unread] { $kind ->
        [conversation] Semua { $count } perbualan belum dibaca pada skrin dipilih.
       *[message] Semua { $count } mesej belum dibaca pada skrin dipilih.
    }
    [starred] { $kind ->
        [conversation] Semua { $count } perbualan dibintangi pada skrin dipilih.
       *[message] Semua { $count } mesej dibintangi pada skrin dipilih.
    }
    [unstarred] { $kind ->
        [conversation] Semua { $count } perbualan tidak dibintangi pada skrin dipilih.
       *[message] Semua { $count } mesej tidak dibintangi pada skrin dipilih.
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] Pilih semua { $count } perbualan dibaca
       *[message] Pilih semua { $count } mesej dibaca
    }
   *[unread] { $kind ->
        [conversation] Pilih semua { $count } perbualan belum dibaca
       *[message] Pilih semua { $count } mesej belum dibaca
    }
    [starred] { $kind ->
        [conversation] Pilih semua { $count } perbualan dibintangi
       *[message] Pilih semua { $count } mesej dibintangi
    }
    [unstarred] { $kind ->
        [conversation] Pilih semua { $count } perbualan tidak dibintangi
       *[message] Pilih semua { $count } mesej tidak dibintangi
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] Pilih semua { $count } perbualan dibaca dalam { $folder }
       *[message] Pilih semua { $count } mesej dibaca dalam { $folder }
    }
   *[unread] { $kind ->
        [conversation] Pilih semua { $count } perbualan belum dibaca dalam { $folder }
       *[message] Pilih semua { $count } mesej belum dibaca dalam { $folder }
    }
    [starred] { $kind ->
        [conversation] Pilih semua { $count } perbualan dibintangi dalam { $folder }
       *[message] Pilih semua { $count } mesej dibintangi dalam { $folder }
    }
    [unstarred] { $kind ->
        [conversation] Pilih semua { $count } perbualan tidak dibintangi dalam { $folder }
       *[message] Pilih semua { $count } mesej tidak dibintangi dalam { $folder }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] Kesemua { $count } perbualan yang dibaca dipilih.
       *[message] Kesemua { $count } mesej yang dibaca dipilih.
    }
   *[unread] { $kind ->
        [conversation] Kesemua { $count } perbualan yang belum dibaca dipilih.
       *[message] Kesemua { $count } mesej yang belum dibaca dipilih.
    }
    [starred] { $kind ->
        [conversation] Kesemua { $count } perbualan yang dibintangi dipilih.
       *[message] Kesemua { $count } mesej yang dibintangi dipilih.
    }
    [unstarred] { $kind ->
        [conversation] Kesemua { $count } perbualan yang tidak dibintangi dipilih.
       *[message] Kesemua { $count } mesej yang tidak dibintangi dipilih.
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] Kesemua { $count } perbualan yang dibaca dalam { $folder } dipilih.
       *[message] Kesemua { $count } mesej yang dibaca dalam { $folder } dipilih.
    }
   *[unread] { $kind ->
        [conversation] Kesemua { $count } perbualan yang belum dibaca dalam { $folder } dipilih.
       *[message] Kesemua { $count } mesej yang belum dibaca dalam { $folder } dipilih.
    }
    [starred] { $kind ->
        [conversation] Kesemua { $count } perbualan yang dibintangi dalam { $folder } dipilih.
       *[message] Kesemua { $count } mesej yang dibintangi dalam { $folder } dipilih.
    }
    [unstarred] { $kind ->
        [conversation] Kesemua { $count } perbualan yang tidak dibintangi dalam { $folder } dipilih.
       *[message] Kesemua { $count } mesej yang tidak dibintangi dalam { $folder } dipilih.
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Tiada perbualan yang dibaca di sini.
       *[message] Tiada mesej yang dibaca di sini.
    }
   *[unread] { $kind ->
        [conversation] Tiada perbualan yang belum dibaca di sini.
       *[message] Tiada mesej yang belum dibaca di sini.
    }
    [starred] { $kind ->
        [conversation] Tiada perbualan yang dibintangi di sini.
       *[message] Tiada mesej yang dibintangi di sini.
    }
    [unstarred] { $kind ->
        [conversation] Tiada perbualan yang tidak dibintangi di sini.
       *[message] Tiada mesej yang tidak dibintangi di sini.
    }
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
row-tracking-none = Dijejaki. Belum dibuka
row-tracking-opened = Dibuka oleh { $opened } daripada { $recipients }
row-tracking-clicked = Dibuka oleh { $opened } daripada { $recipients }, pautan diikuti oleh { $clicked }
row-pin = Sematkan di atas
row-unpin = Nyahsemat
row-snoozed-until = Ditunda hingga { $when }

## Mail list: More menu and right-click menu

menu-reply = Balas
menu-reply-all = Balas semua
menu-forward = Majukan
menu-archive = Arkibkan
menu-delete = Padam
menu-delete-forever = Padam selama-lamanya
menu-move-to-inbox = Alih ke Peti Masuk
menu-spam = Laporkan spam
menu-not-spam = Bukan spam
menu-mark-read = Tandai sebagai dibaca
menu-mark-unread = Tandai sebagai belum dibaca
menu-mark-all-read = Tandai semua sebagai dibaca
menu-star = Tambah bintang
menu-unstar = Alih keluar bintang
menu-important = Tandai sebagai penting
menu-not-important = Tandai sebagai tidak penting
menu-pin = Sematkan di atas
menu-unpin = Nyahsemat
menu-snooze = Tunda
menu-unsnooze = Nyahtunda
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
toast-snoozed = { $kind ->
    [conversation] { $count } perbualan ditunda hingga { $when }.
   *[message] { $count } mesej ditunda hingga { $when }.
}
toast-unsnoozed = { $kind ->
    [conversation] { $count } perbualan kembali ke Peti Masuk.
   *[message] { $count } mesej kembali ke Peti Masuk.
}
toast-spam = { $kind ->
    [conversation] { $count } perbualan dilaporkan sebagai spam.
   *[message] { $count } mesej dilaporkan sebagai spam.
}
toast-not-spam = { $kind ->
    [conversation] { $count } perbualan ditandakan bukan spam dan dialihkan ke peti masuk.
   *[message] { $count } mesej ditandakan bukan spam dan dialihkan ke peti masuk.
}
toast-deleted-forever = { $kind ->
    [conversation] { $count } perbualan dipadamkan selama-lamanya.
   *[message] { $count } mesej dipadamkan selama-lamanya.
}
toast-marked-read = { $kind ->
    [conversation] { $count } perbualan ditandakan sebagai dibaca.
   *[message] { $count } mesej ditandakan sebagai dibaca.
}
toast-marked-unread = { $kind ->
    [conversation] { $count } perbualan ditandakan sebagai belum dibaca.
   *[message] { $count } mesej ditandakan sebagai belum dibaca.
}
toast-undone = Tindakan dibuat asal.
toast-nothing-to-undo = Tiada apa-apa untuk dibuat asal.
toast-cannot-undo-delete-forever = Mel yang dipadamkan selama-lamanya tidak boleh dikembalikan.
toast-send-undone = Penghantaran dibuat asal.
toast-too-late-to-undo-send = Sudah terlambat untuk membuat asal: mesej telah pun dihantar.
toast-undo = Buat asal
toast-close = Tutup
toast-no-spam-folder = Akaun ini tiada folder spam.
