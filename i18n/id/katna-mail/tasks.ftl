# Katna Mail, Indonesian (Bahasa Indonesia): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Tugas baru
tasks-all = Semua tugas
tasks-today = Hari ini
tasks-upcoming = Mendatang
tasks-starred = Berbintang
tasks-completed-view = Selesai
tasks-new-list = Buat daftar baru
tasks-labels-heading = Label
tasks-on-this-computer = Di komputer ini
tasks-my-tasks = Tugas Saya
tasks-account-sign-in = Masuk lagi untuk menampilkan tugas
tasks-account-signed-in = Sudah masuk lagi ke { $address }. Mengambil tugas Anda…
tasks-account-sign-in-refused = { $provider } tidak mengizinkan Katna masuk. Coba lagi, dan izinkan akses ke tugas Anda.
tasks-account-refused = Server tidak menerima sandi. Yahoo, iCloud, Zoho, dan lainnya memerlukan sandi aplikasi.
tasks-account-change-password = Ubah sandi
tasks-account-change-password-tooltip = Ketik sandi baru; Katna memeriksanya dengan server
tasks-account-not-enabled = Akses tugas untuk Katna belum diaktifkan.
tasks-account-failed = Daftar tugas tidak dapat dibaca.
tasks-account-error = Daftar tugas tidak dapat dibaca: { $reason }
tasks-account-none = Tidak ada daftar tugas yang ditemukan
tasks-account-none-why = Tidak ada daftar tugas yang ditemukan: { $reason }
tasks-account-use-sign-in = { $provider } hanya menampilkan tugas ke Katna yang masuk dengan { $provider }.
tasks-account-sign-in-with = Masuk dengan { $provider }
tasks-account-looking = Mencari daftar tugas…
tasks-account-try-again = Coba lagi
tasks-account-try-again-tooltip = Periksa lagi tugas akun ini sekarang
tasks-account-fixing = Sedang dikerjakan…
tasks-list-name-placeholder = Nama daftar

## Lists and tasks

tasks-loading = Membaca tugas Anda…
tasks-no-lists = Daftar tugas Anda muncul di sini.
tasks-search = Telusuri tugas
tasks-search-none = Tidak ada tugas yang cocok dengan pencarian Anda.
tasks-add = Tambahkan tugas
tasks-title-placeholder = Judul
tasks-add-step = Tambahkan subtugas
tasks-empty = Belum ada tugas. Tambahkan satu di atas.
tasks-starred-empty = Beri bintang pada tugas untuk melihatnya di sini.
tasks-label-empty = Tidak ada tugas terbuka dengan label ini.
tasks-today-empty = Tidak ada yang jatuh tempo hari ini.
tasks-completed-empty = Tugas yang Anda selesaikan muncul di sini.
tasks-upcoming-add = Tambahkan tugas untuk { $day }
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = Dari email
tasks-from-note-quiet = Dari catatan
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Terlambat
tasks-completed = { $count ->
   *[other] Selesai ({ $count })
}
tasks-list-options = Opsi daftar
tasks-sort-by = Urutkan menurut
tasks-sort-my-order = Urutan saya
tasks-sort-date = Tanggal
tasks-sort-starred = Baru diberi bintang
tasks-sort-title = Judul
tasks-rename-list = Ganti nama daftar
tasks-delete-list = Hapus daftar
tasks-mark-done = Tandai selesai
tasks-mark-open = Tandai belum selesai
tasks-star = Beri bintang
tasks-unstar = Hapus bintang
tasks-edit-title = Edit judul
tasks-details = Detail
tasks-delete = Hapus
tasks-move-to = Pindahkan ke { $list }
tasks-from-mail = Email
tasks-open-mail = Buka email
tasks-from-note = Catatan
tasks-open-note = Buka catatan
tasks-note-gone = Catatan itu sudah tidak ada.
tasks-no-subject = (tanpa subjek)

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
   *[other] { $count } dipilih
}
tasks-select-clear = Hapus pilihan
tasks-select-move = Pindahkan ke daftar
tasks-select-date = Atur tanggal
tasks-next-week = Minggu depan

## The details dialog

tasks-notes-placeholder = Tambahkan detail
tasks-date = Tanggal
tasks-no-date = Tanpa tanggal
tasks-time-placeholder = Tambahkan waktu
tasks-repeat = Ulangi
tasks-repeat-never = Tidak berulang
tasks-repeat-daily = Harian
tasks-repeat-weekly = Mingguan
tasks-repeat-monthly = Bulanan
tasks-repeat-yearly = Tahunan
tasks-repeat-other = Khusus
tasks-remind = Ingatkan saya
tasks-remind-off = Jangan ingatkan
tasks-remind-on-time = Pada waktunya
tasks-remind-morning = Pada hari itu, { $time }
tasks-remind-hour-before = Satu jam sebelumnya
tasks-remind-day-before = Sehari sebelumnya
tasks-label-add = Tambahkan label
tasks-label-task = Beri label pada tugas
tasks-files-attach = Lampirkan file
tasks-files-pick = Lampirkan
tasks-file-open = Buka
tasks-file-remove = Hapus file
tasks-file-here = Hanya di komputer ini
tasks-cancel = Batal
tasks-save = Simpan
tasks-not-a-time = “{ $text }” bukan waktu, misalnya { $example }.

## Due days

tasks-due-today = Hari ini
tasks-due-tomorrow = Besok
tasks-due-yesterday = Kemarin
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Tugas selesai
tasks-toast-next = Selesai. Berikutnya pada { $date }
tasks-toast-deleted = Tugas dihapus
tasks-files-added = { $count ->
    [1] File dilampirkan
   *[other] { $count } file dilampirkan
}
tasks-file-removed = “{ $name }” dihapus
tasks-files-left-out = Tidak dilampirkan: { $names }. Tugas menerima file hingga { $limit }, bukan folder.
tasks-file-missing = File itu sudah tidak ada di sini.
tasks-toast-added = { $count ->
   *[other] { $count } tugas ditambahkan
}
tasks-mail-gone = Email itu sudah tidak ada di sini.
tasks-toast-list-deleted = Daftar dihapus
tasks-toast-moved = Dipindahkan ke { $list }
tasks-toast-placed = Tugas dipindahkan
tasks-toast-rescheduled = Tugas dijadwalkan ulang
tasks-toast-rescheduled-several = { $count ->
    [1] Tugas dijadwalkan ulang
   *[other] { $count } tugas dijadwalkan ulang
}
tasks-toast-done-several = { $count ->
    [1] Tugas selesai
   *[other] { $count } tugas selesai
}
tasks-toast-open-several = { $count ->
    [1] Tugas ditandai belum selesai
   *[other] { $count } tugas ditandai belum selesai
}
tasks-toast-starred = { $count ->
    [1] Tugas diberi bintang
   *[other] { $count } tugas diberi bintang
}
tasks-toast-unstarred = { $count ->
    [1] Bintang dihapus
   *[other] Bintang dihapus dari { $count } tugas
}
tasks-toast-deleted-several = { $count ->
    [1] Tugas dihapus
   *[other] { $count } tugas dihapus
}
