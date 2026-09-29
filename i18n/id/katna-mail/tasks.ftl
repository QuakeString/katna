# Katna Mail, Indonesian (Bahasa Indonesia): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Buat
tasks-all = Semua tugas
tasks-today = Hari ini
tasks-starred = Berbintang
tasks-new-list = Buat daftar baru
tasks-on-this-computer = Di komputer ini
tasks-my-tasks = Tugas Saya
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Masuk lagi untuk menampilkan tugas
tasks-account-signed-in = Sudah masuk lagi ke { $address }. Mengambil tugas Anda…
tasks-account-sign-in-refused = { $provider } tidak mengizinkan Katna masuk. Coba lagi, dan izinkan akses ke tugas Anda.
tasks-account-refused = Server tidak menerima sandi. Yahoo, iCloud, Zoho, dan lainnya memerlukan sandi aplikasi.
tasks-account-change-password = Ubah sandi
tasks-account-change-password-tooltip = Buka Setelan > Akun
tasks-account-not-enabled = Akses tugas untuk Katna belum diaktifkan.
tasks-account-failed = Daftar tugas tidak dapat dibaca.
# $reason is the server's own words, in English.
tasks-account-error = Daftar tugas tidak dapat dibaca: { $reason }
tasks-account-none = Tidak ada daftar tugas yang ditemukan
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = Tidak ada daftar tugas yang ditemukan: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
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
tasks-today-empty = Tidak ada yang jatuh tempo hari ini.
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Terlambat
tasks-completed = { $count ->
   *[other] Selesai ({ $count })
}
tasks-list-options = Opsi daftar
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
tasks-toast-added = { $count ->
   *[other] { $count } tugas ditambahkan
}
tasks-mail-gone = Email itu sudah tidak ada di sini.
tasks-toast-list-deleted = Daftar dihapus
tasks-toast-moved = Dipindahkan ke { $list }
tasks-toast-rescheduled = Tugas dijadwalkan ulang
