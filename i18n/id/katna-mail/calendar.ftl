# Katna Mail, Indonesian (Bahasa Indonesia): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Hari ini
calendar-today-tip = Buka hari ini
calendar-view-day = Hari
calendar-view-week = Minggu
calendar-view-month = Bulan
calendar-view-year = Tahun
calendar-view-schedule = Jadwal
calendar-view-days = { $count } hari
calendar-options = Opsi
calendar-density = Kepadatan
calendar-density-responsive = Responsif terhadap layar Anda
calendar-density-comfortable = Nyaman
calendar-density-compact = Ringkas
calendar-custom-days = Tampilan khusus
calendar-second-zone = Zona waktu kedua
calendar-zone-none = Tidak ada
calendar-zone = { $zone } ({ $offset })
calendar-share-free = Bagikan waktu luang
calendar-free-subject = Waktu saya luang
calendar-free-intro = Berikut beberapa waktu saya luang ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = Saya tidak punya waktu luang dalam beberapa hari kerja ke depan.
calendar-previous-day = Hari sebelumnya
calendar-next-day = Hari berikutnya
calendar-previous-week = Minggu sebelumnya
calendar-next-week = Minggu berikutnya
calendar-previous-month = Bulan sebelumnya
calendar-next-month = Bulan berikutnya
calendar-previous-year = Tahun sebelumnya
calendar-next-year = Tahun berikutnya
calendar-previous-period = Sebelumnya
calendar-next-period = Berikutnya
calendar-title-months = { $first } – { $last }
calendar-loading = Memuat…
calendar-read-failed = Kalender tidak dapat dibaca: { $error }
calendar-sets = Set kalender
calendar-set-add = Simpan kalender yang ditampilkan sebagai set
calendar-set-name = Nama set
calendar-set-remove = Hapus set
calendar-local = Di komputer ini
calendar-account-gone = Akun yang dihapus
calendar-account-sign-in = Masuk lagi untuk menampilkan kalender
calendar-account-signed-in = Sudah masuk lagi ke { $address }. Mengambil kalender Anda…
calendar-account-sign-in-refused = { $provider } tidak mengizinkan Katna masuk. Coba lagi, dan izinkan akses ke kalender Anda.
calendar-account-refused = Server tidak mengizinkan Katna mengakses kalender.
calendar-account-not-enabled = Akses kalender untuk Katna belum diaktifkan.
calendar-account-failed = Kalender tidak dapat dibaca.
calendar-account-error = Kalender tidak dapat dibaca: { $reason }
calendar-account-none = Tidak ada kalender yang ditemukan
calendar-account-looking = Mencari kalender…
calendar-account-try-again = Coba lagi
calendar-account-try-again-tooltip = Periksa lagi kalender akun ini sekarang
calendar-account-fixing = Sedang dikerjakan…
calendar-birthdays = Ulang tahun
calendar-birthday-of = Ulang tahun { $name }
calendar-empty-title = Belum ada kalender
calendar-empty-text = Katna menampilkan kalender akun Google dan Microsoft Anda di sini setelah disinkronkan, serta kalender dari server lain yang mendukung CalDAV.
calendar-schedule-empty = Tidak ada rencana untuk dua bulan ke depan.
calendar-search = Telusuri acara
calendar-search-past = Acara yang telah lewat
calendar-search-none = Tidak ada acara yang cocok dengan pencarian Anda.
calendar-no-title = (Tanpa judul)
calendar-all-day = Sepanjang hari
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } lainnya
calendar-repeats = Berulang
calendar-join = Gabung
calendar-email-guests = Kirim email ke tamu
calendar-running-late = Saya terlambat
calendar-late-subject = Terlambat: { $title }
calendar-late-body = Maaf, saya terlambat beberapa menit untuk { $title }. Saya akan segera tiba.
calendar-guests =
    { $count ->
       *[other] { $count } tamu
    }
calendar-guest-answers = { $yes } ya, { $maybe } mungkin, { $no } tidak, { $waiting } menunggu
calendar-organizer = Penyelenggara
calendar-optional = Opsional
calendar-open-web = Buka di browser
calendar-open-contact = Buka kontak
calendar-close = Tutup

## Adding, changing and deleting events.

calendar-add-title = Tambahkan judul
calendar-add-location = Tambahkan lokasi
calendar-add-notes = Tambahkan deskripsi
calendar-add-guests = Tambahkan tamu
calendar-remove-guest = Hapus
calendar-add-meet = Tambahkan konferensi video Google Meet
calendar-add-teams = Tambahkan rapat Teams
calendar-has-call = Konferensi video ditambahkan
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Sepanjang hari
calendar-more-options = Opsi lainnya
calendar-save = Simpan
calendar-saved = Acara disimpan
calendar-deleted = Acara dihapus
calendar-discard = Buang perubahan
calendar-edit = Edit acara
calendar-delete = Hapus acara
calendar-event-details = Detail acara
calendar-kind-event = Acara
calendar-kind-focus = Waktu fokus
calendar-kind-out-of-office = Di luar kantor
calendar-kind-working-location = Lokasi kerja
calendar-working-home = Rumah
calendar-busy = Sibuk
calendar-free = Tersedia
calendar-cancel = Batal
calendar-ok = OK
calendar-read-only = Anda tidak dapat mengubah acara di kalender ini
calendar-none-editable = Belum ada kalender tempat Anda dapat menambahkan acara
calendar-no-such-time = Waktu itu tidak ada di zona waktu Anda
calendar-end-before-start = Acara berakhir sebelum dimulai
calendar-repeat-never = Tidak berulang
calendar-repeat-daily = Harian
calendar-repeat-weekly = Mingguan pada hari { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Bulanan pada { $weekday } pertama
        [2] Bulanan pada { $weekday } kedua
        [3] Bulanan pada { $weekday } ketiga
        [4] Bulanan pada { $weekday } keempat
       *[other] Bulanan pada { $weekday } terakhir
    }
calendar-repeat-yearly = Tahunan pada { $day }
calendar-repeat-weekdays = Setiap hari kerja (Senin sampai Jumat)
calendar-repeat-custom = Khusus
calendar-reminder-none = Tidak ada notifikasi
calendar-reminder-at-start = Saat dimulai
calendar-reminder-minutes =
    { $count ->
       *[other] { $count } menit sebelumnya
    }
calendar-reminder-hours =
    { $count ->
       *[other] { $count } jam sebelumnya
    }
calendar-reminder-days =
    { $count ->
       *[other] { $count } hari sebelumnya
    }
calendar-scope-edit-title = Edit acara berulang
calendar-scope-delete-title = Hapus acara berulang
calendar-scope-this = Acara ini
calendar-scope-following = Acara ini dan acara berikutnya
calendar-scope-all = Semua acara
calendar-scope-respond-title = Jawaban untuk acara berulang
calendar-going = Hadir?
calendar-answer-yes = Ya
calendar-answer-no = Tidak
calendar-answer-maybe = Mungkin
calendar-answered-yes = Anda akan hadir
calendar-answered-no = Anda tidak akan hadir
calendar-answered-maybe = Anda mungkin hadir

## The card at the top of a mail with an invitation.

calendar-invite = Undangan
calendar-invite-cancelled = Acara dibatalkan
calendar-invite-reply = { $name } menjawab
calendar-invite-reply-yes = { $name } menerima
calendar-invite-reply-no = { $name } menolak
calendar-invite-reply-maybe = { $name } mungkin hadir
calendar-invite-organizer = Diselenggarakan oleh { $name }
calendar-invite-open = Buka di Kalender
calendar-invite-not-yet = Belum ada di kalender Anda. Anda dapat menjawab setelah disinkronkan.
calendar-invite-by-mail = Tidak ada di kalender Anda: jawaban Anda dikirim ke penyelenggara lewat email.
calendar-mail-yes = Diterima: { $title }
calendar-mail-yes-body = { $name } menerima undangan ini.
calendar-mail-no = Ditolak: { $title }
calendar-mail-no-body = { $name } menolak undangan ini.
calendar-mail-maybe = Diterima sementara: { $title }
calendar-mail-maybe-body = { $name } menerima undangan ini secara sementara.
calendar-invite-your-day = Hari Anda
calendar-invite-clashes =
    { $count ->
       *[other] Bentrok dengan { $count } acara
    }

## The day's agenda beside the mail.

agenda-show = Tampilkan agenda hari ini
agenda-hide = Sembunyikan agenda
agenda-today = Hari ini, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = Tidak ada rencana pada hari ini.
