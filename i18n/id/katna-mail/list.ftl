# Katna Mail, Indonesian (Bahasa Indonesia).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
tab-provider-other = diurutkan oleh Katna

## Mail list: toolbar

list-select = Pilih
list-refresh = Muat ulang
list-back-to-top = Kembali ke atas
list-checking = Memeriksa email baru…
list-more = Lainnya
list-mark-read = Tandai sudah dibaca
list-mark-unread = Tandai belum dibaca
list-move-to = Pindahkan ke
list-archive = Arsipkan
list-spam = Laporkan spam
list-delete = Hapus
list-snooze = Tunda
list-unsnooze = Batalkan penundaan
list-newer = Lebih baru
list-older = Lebih lama
list-range = { $first }–{ $last } dari { $total }
list-range-about = { $first }–{ $last } dari sekitar { $total }
list-results = Hasil untuk “{ $query }”
list-results-corrected = Menampilkan hasil untuk “{ $query }”
list-search-instead = Telusuri “{ $query }” saja
list-search-no-index = Penelusuran belum siap: indeks belum dibuat.
list-search-not-ready = Penelusuran belum siap: { $error }
list-files-more = +{ $count }
list-replied = Anda sudah membalas

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
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] Semua { $count } percakapan yang sudah dibaca di layar dipilih.
       *[message] Semua { $count } pesan yang sudah dibaca di layar dipilih.
    }
   *[unread] { $kind ->
        [conversation] Semua { $count } percakapan yang belum dibaca di layar dipilih.
       *[message] Semua { $count } pesan yang belum dibaca di layar dipilih.
    }
    [starred] { $kind ->
        [conversation] Semua { $count } percakapan berbintang di layar dipilih.
       *[message] Semua { $count } pesan berbintang di layar dipilih.
    }
    [unstarred] { $kind ->
        [conversation] Semua { $count } percakapan tidak berbintang di layar dipilih.
       *[message] Semua { $count } pesan tidak berbintang di layar dipilih.
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] Pilih semua { $count } percakapan yang sudah dibaca
       *[message] Pilih semua { $count } pesan yang sudah dibaca
    }
   *[unread] { $kind ->
        [conversation] Pilih semua { $count } percakapan yang belum dibaca
       *[message] Pilih semua { $count } pesan yang belum dibaca
    }
    [starred] { $kind ->
        [conversation] Pilih semua { $count } percakapan berbintang
       *[message] Pilih semua { $count } pesan berbintang
    }
    [unstarred] { $kind ->
        [conversation] Pilih semua { $count } percakapan tidak berbintang
       *[message] Pilih semua { $count } pesan tidak berbintang
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] Pilih semua { $count } percakapan yang sudah dibaca di { $folder }
       *[message] Pilih semua { $count } pesan yang sudah dibaca di { $folder }
    }
   *[unread] { $kind ->
        [conversation] Pilih semua { $count } percakapan yang belum dibaca di { $folder }
       *[message] Pilih semua { $count } pesan yang belum dibaca di { $folder }
    }
    [starred] { $kind ->
        [conversation] Pilih semua { $count } percakapan berbintang di { $folder }
       *[message] Pilih semua { $count } pesan berbintang di { $folder }
    }
    [unstarred] { $kind ->
        [conversation] Pilih semua { $count } percakapan tidak berbintang di { $folder }
       *[message] Pilih semua { $count } pesan tidak berbintang di { $folder }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] Semua { $count } percakapan yang sudah dibaca dipilih.
       *[message] Semua { $count } pesan yang sudah dibaca dipilih.
    }
   *[unread] { $kind ->
        [conversation] Semua { $count } percakapan yang belum dibaca dipilih.
       *[message] Semua { $count } pesan yang belum dibaca dipilih.
    }
    [starred] { $kind ->
        [conversation] Semua { $count } percakapan berbintang dipilih.
       *[message] Semua { $count } pesan berbintang dipilih.
    }
    [unstarred] { $kind ->
        [conversation] Semua { $count } percakapan tidak berbintang dipilih.
       *[message] Semua { $count } pesan tidak berbintang dipilih.
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] Semua { $count } percakapan yang sudah dibaca di { $folder } dipilih.
       *[message] Semua { $count } pesan yang sudah dibaca di { $folder } dipilih.
    }
   *[unread] { $kind ->
        [conversation] Semua { $count } percakapan yang belum dibaca di { $folder } dipilih.
       *[message] Semua { $count } pesan yang belum dibaca di { $folder } dipilih.
    }
    [starred] { $kind ->
        [conversation] Semua { $count } percakapan berbintang di { $folder } dipilih.
       *[message] Semua { $count } pesan berbintang di { $folder } dipilih.
    }
    [unstarred] { $kind ->
        [conversation] Semua { $count } percakapan tidak berbintang di { $folder } dipilih.
       *[message] Semua { $count } pesan tidak berbintang di { $folder } dipilih.
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Tidak ada percakapan yang sudah dibaca di sini.
       *[message] Tidak ada pesan yang sudah dibaca di sini.
    }
   *[unread] { $kind ->
        [conversation] Tidak ada percakapan yang belum dibaca di sini.
       *[message] Tidak ada pesan yang belum dibaca di sini.
    }
    [starred] { $kind ->
        [conversation] Tidak ada percakapan berbintang di sini.
       *[message] Tidak ada pesan berbintang di sini.
    }
    [unstarred] { $kind ->
        [conversation] Tidak ada percakapan tidak berbintang di sini.
       *[message] Tidak ada pesan tidak berbintang di sini.
    }
}
list-clear-selection = Hapus pilihan

## Mail list: empty states

list-empty-search = Tidak ada pesan yang cocok dengan penelusuran Anda.
list-empty-tab = Tidak ada email di { $tab }.
list-empty-tab-unknown = Tidak ada email di tab ini.
list-empty-folder = Tidak ada pesan di { $folder }.
list-empty-folder-unknown = Tidak ada pesan di folder ini.
list-empty-waiting = Tidak ada yang menunggu balasan.
list-empty-reminders = Tidak ada pengingat. Tekan H pada email untuk menambahkannya.
list-first-sync = Mengambil email Anda…
list-first-sync-detail = Email akan muncul di sini saat tiba.
list-store-unreadable = Penyimpanan email tidak dapat dibuka

## Mail list: lines

row-no-subject = (tanpa subjek)
row-unknown-sender = (pengirim tidak dikenal)
row-to = Kepada:
row-no-recipients = (tanpa penerima)
row-removed = Pesan ini telah dihapus.
row-starred = Berbintang
row-not-starred = Tidak berbintang
row-important = Penting. Klik untuk menandai sebagai tidak penting.
row-mark-important = Tandai sebagai penting
row-pinned = Disematkan di atas
row-task = Tugas
row-task-open = Buka tugas: { $title }
row-tracking-none = Dilacak. Belum dibuka
row-tracking-opened = Dibuka oleh { $opened } dari { $recipients }
row-tracking-clicked = Dibuka oleh { $opened } dari { $recipients }, link diikuti oleh { $clicked }
row-pin = Sematkan di atas
row-unpin = Lepas sematan
row-snoozed-until = Ditunda sampai { $when }
row-snoozed-day-time = { $day } { $time }
snoozed-group-today = Hari ini
snoozed-group-tomorrow = Besok
snoozed-group-this-week = Minggu ini
snoozed-group-later = Nanti
row-follow-up-step = Tindak lanjut { $step } dari { $steps } · { $date }
row-follow-up-waiting = Tindak lanjut menunggu
row-reminder = Pengingat { $date }

## Mail list: More menu and right-click menu

menu-reply = Balas
menu-reply-all = Balas semua
menu-forward = Teruskan
menu-archive = Arsipkan
menu-delete = Hapus
menu-delete-forever = Hapus selamanya
menu-move-to-inbox = Pindahkan ke Kotak Masuk
menu-spam = Laporkan spam
menu-not-spam = Bukan spam
menu-mark-read = Tandai sudah dibaca
menu-mark-unread = Tandai belum dibaca
menu-mark-all-read = Tandai semua sudah dibaca
menu-star = Tambahkan bintang
menu-unstar = Hapus bintang
menu-important = Tandai sebagai penting
menu-not-important = Tandai sebagai tidak penting
menu-pin = Sematkan di atas
menu-unpin = Lepas sematan
menu-snooze = Tunda
menu-remind = Ingatkan saya
menu-unsnooze = Batalkan penundaan
menu-add-to-tasks = Tambahkan ke Tugas
menu-schedule-meeting = Jadwalkan rapat
menu-start-call = Mulai panggilan video
menu-add-note = Tambahkan catatan
menu-print-all = Cetak semua
menu-new-window = Buka di jendela baru
menu-move-to = Pindahkan ke
menu-follow-up = Tindak lanjut
menu-more = Lainnya
menu-move-to-heading = Pindahkan ke:
menu-move-to-search = Pindahkan ke…
menu-label-as = Beri label
menu-label-as-search = Beri label…
menu-no-folder = Tidak ada folder bernama “{ $name }”
menu-no-label = Tidak ada label bernama “{ $name }”
menu-create-folder = Buat “{ $name }”
menu-always-move = Selalu pindahkan email dari { $name } ke sini
toast-always-move-failed = Email sudah dipindahkan, tetapi aturannya tidak dibuat: { $error }
drag-mail = { $kind ->
    [conversation] { $count } percakapan
   *[message] { $count } pesan
}
menu-find-from = Cari email dari { $name }
menu-make-rule = Buat aturan…

## Snackbar after an action on mail in the list

toast-key-imported = Kunci diimpor
toast-key-updated = Anda sudah memiliki kunci ini; kini sudah diperbarui
toast-key-removed = Kunci dihapus
toast-key-not-removed = Kunci tidak dapat dihapus
toast-fingerprint-copied = Sidik jari disalin
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
toast-label-added = Label “{ $label }” ditambahkan.
toast-label-removed = Label “{ $label }” dihapus.
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
toast-snoozed = { $kind ->
    [conversation] { $count } percakapan ditunda sampai { $when }.
   *[message] { $count } pesan ditunda sampai { $when }.
}
toast-unsnoozed = { $kind ->
    [conversation] { $count } percakapan kembali ke Kotak Masuk.
   *[message] { $count } pesan kembali ke Kotak Masuk.
}
toast-spam = { $kind ->
    [conversation] { $count } percakapan dilaporkan sebagai spam.
   *[message] { $count } pesan dilaporkan sebagai spam.
}
toast-not-spam = { $kind ->
    [conversation] { $count } percakapan ditandai bukan spam dan dipindahkan ke kotak masuk.
   *[message] { $count } pesan ditandai bukan spam dan dipindahkan ke kotak masuk.
}
toast-deleted-forever = { $kind ->
    [conversation] { $count } percakapan dihapus selamanya.
   *[message] { $count } pesan dihapus selamanya.
}
toast-marked-read = { $kind ->
    [conversation] { $count } percakapan ditandai sudah dibaca.
   *[message] { $count } pesan ditandai sudah dibaca.
}
toast-marked-unread = { $kind ->
    [conversation] { $count } percakapan ditandai belum dibaca.
   *[message] { $count } pesan ditandai belum dibaca.
}
toast-undone = Tindakan diurungkan.
toast-nothing-to-undo = Tidak ada yang bisa diurungkan.
toast-cannot-undo-delete-forever = Email yang dihapus selamanya tidak bisa dikembalikan.
toast-send-undone = Pengiriman diurungkan.
toast-too-late-to-undo-send = Terlambat untuk mengurungkan: pesan sudah terkirim.
toast-undo = Urungkan
toast-close = Tutup
toast-no-spam-folder = Akun ini tidak memiliki folder spam.
