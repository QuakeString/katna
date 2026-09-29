# Katna Mail, Malay (Bahasa Melayu): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Hari ini
calendar-today-tip = Pergi ke hari ini
calendar-view-day = Hari
calendar-view-week = Minggu
calendar-view-month = Bulan
calendar-view-year = Tahun
calendar-view-schedule = Jadual
calendar-view-days =
    { $count ->
       *[other] { $count } hari
    }
calendar-options = Pilihan
calendar-density = Ketumpatan
calendar-density-responsive = Responsif kepada skrin anda
calendar-density-comfortable = Selesa
calendar-density-compact = Padat
calendar-custom-days = Paparan tersuai
calendar-second-zone = Zon waktu kedua
calendar-zone-none = Tiada
calendar-zone = { $zone } ({ $offset })
calendar-share-free = Kongsi masa terluang
calendar-free-subject = Masa saya terluang
calendar-free-intro = Berikut ialah beberapa masa saya terluang ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = Saya tiada masa terluang dalam beberapa hari bekerja yang akan datang.
calendar-previous-day = Hari sebelumnya
calendar-next-day = Hari seterusnya
calendar-previous-week = Minggu sebelumnya
calendar-next-week = Minggu seterusnya
calendar-previous-month = Bulan sebelumnya
calendar-next-month = Bulan seterusnya
calendar-previous-year = Tahun sebelumnya
calendar-next-year = Tahun seterusnya
calendar-previous-period = Lebih awal
calendar-next-period = Lebih lewat
calendar-title-months = { $first } – { $last }
calendar-loading = Memuatkan…
calendar-read-failed = Kalendar tidak dapat dibaca: { $error }
calendar-sets = Set kalendar
calendar-set-add = Simpan kalendar yang dipaparkan sebagai set
calendar-set-name = Nama set
calendar-set-remove = Alih keluar set
calendar-local = Pada komputer ini
calendar-account-gone = Akaun yang dialih keluar
calendar-account-sign-in = Log masuk semula untuk menunjukkan kalendar
calendar-account-signed-in = Telah log masuk ke { $address } semula. Mendapatkan kalendar anda…
calendar-account-sign-in-refused = { $provider } tidak membenarkan Katna masuk. Cuba lagi, dan benarkan akses kepada kalendar anda.
calendar-account-refused = Pelayan tidak menerima kata laluan. Yahoo, iCloud, Zoho dan lain-lain memerlukan kata laluan aplikasi.
calendar-account-change-password = Tukar kata laluan
calendar-account-change-password-tooltip = Buka Tetapan > Akaun
calendar-account-not-enabled = Akses kalendar untuk Katna belum dihidupkan lagi.
calendar-account-failed = Kalendar tidak dapat dibaca.
calendar-account-error = Kalendar tidak dapat dibaca: { $reason }
calendar-account-none = Tiada kalendar ditemui
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = Tiada kalendar ditemui: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } hanya menunjukkan kalendar kepada Katna yang log masuk dengan { $provider }.
calendar-account-sign-in-with = Log masuk dengan { $provider }
calendar-account-looking = Mencari kalendar…
calendar-account-try-again = Cuba lagi
calendar-account-try-again-tooltip = Semak kalendar akaun ini semula sekarang
calendar-account-fixing = Sedang diusahakan…
calendar-birthdays = Hari lahir
calendar-birthday-of = Hari lahir { $name }
calendar-empty-title = Belum ada kalendar
calendar-empty-text = Katna memaparkan kalendar akaun Google dan Microsoft anda di sini setelah disegerakkan, serta kalendar daripada pelayan lain yang menawarkan CalDAV.
calendar-schedule-empty = Tiada rancangan untuk dua bulan akan datang.
calendar-search = Cari acara
calendar-search-past = Acara lalu
calendar-search-none = Tiada acara yang sepadan dengan carian anda.
calendar-no-title = (Tiada tajuk)
calendar-all-day = Sepanjang hari
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } lagi
calendar-repeats = Berulang
calendar-join = Sertai
calendar-email-guests = E-mel tetamu
calendar-running-late = Saya lewat
calendar-late-subject = Lewat: { $title }
calendar-late-body = Maaf, saya akan lewat beberapa minit untuk { $title }. Saya akan sampai tidak lama lagi.
calendar-guests =
    { $count ->
       *[other] { $count } tetamu
    }
calendar-guest-answers = { $yes } ya, { $maybe } mungkin, { $no } tidak, { $waiting } menunggu
calendar-organizer = Penganjur
calendar-optional = Tidak wajib
calendar-open-web = Buka dalam pelayar
calendar-open-contact = Buka kenalan
calendar-close = Tutup

## Adding, changing and deleting events.

calendar-add-title = Tambah tajuk
calendar-add-location = Tambah lokasi
calendar-add-notes = Tambah penerangan
calendar-add-guests = Tambah tetamu
calendar-remove-guest = Alih keluar
calendar-add-meet = Tambah persidangan video Google Meet
calendar-add-teams = Tambah mesyuarat Teams
calendar-has-call = Persidangan video ditambahkan
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Sepanjang hari
calendar-more-options = Lagi pilihan
calendar-save = Simpan
calendar-saved = Acara disimpan
calendar-deleted = Acara dipadam
calendar-discard = Buang perubahan
calendar-edit = Edit acara
calendar-delete = Padam acara
calendar-event-details = Butiran acara
# Right-click menus on the calendar: on a free time or day, an event and
# a task.
calendar-menu-new-event = Acara baharu
# Shows the day right-clicked on its own, in the Day view.
calendar-menu-open-day = Buka hari
calendar-menu-duplicate = Pendua
calendar-menu-color = Warna
# The event takes its calendar's color.
calendar-menu-color-calendar = Warna kalendar
# A task's new due day, a week from today.
calendar-menu-in-a-week = Dalam seminggu
# Event colors, by the names Google Calendar gives them.
calendar-color-tomato = Tomato
calendar-color-flamingo = Flamingo
calendar-color-tangerine = Tangerin
calendar-color-banana = Pisang
calendar-color-sage = Sage
calendar-color-basil = Selasih
calendar-color-peacock = Merak
calendar-color-blueberry = Beri biru
calendar-color-lavender = Lavender
calendar-color-grape = Anggur
calendar-color-graphite = Grafit
calendar-kind-event = Acara
calendar-kind-task = Tugasan
calendar-kind-focus = Masa fokus
calendar-kind-out-of-office = Di luar pejabat
calendar-kind-working-location = Lokasi kerja
calendar-task-added = Tugasan ditambah
calendar-task-added-to = Tugasan ditambah ke { $list }
calendar-task-list-local = Pada komputer ini
calendar-working-home = Rumah
calendar-busy = Sibuk
calendar-free = Terluang
calendar-cancel = Batal
calendar-ok = OK
calendar-read-only = Anda tidak boleh mengubah acara dalam kalendar ini
calendar-none-editable = Belum ada kalendar yang boleh anda tambahkan acara
calendar-no-such-time = Masa itu tidak wujud dalam zon waktu anda
calendar-end-before-start = Acara tamat sebelum bermula
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
calendar-repeat-weekdays = Setiap hari bekerja (Isnin hingga Jumaat)
calendar-repeat-custom = Tersuai
calendar-reminder-none = Tiada pemberitahuan
calendar-reminder-at-start = Semasa bermula
calendar-reminder-minutes =
    { $count ->
       *[other] { $count } minit sebelum
    }
calendar-reminder-hours =
    { $count ->
       *[other] { $count } jam sebelum
    }
calendar-reminder-days =
    { $count ->
       *[other] { $count } hari sebelum
    }
calendar-scope-edit-title = Edit acara berulang
calendar-scope-delete-title = Padam acara berulang
calendar-scope-this = Acara ini
calendar-scope-following = Acara ini dan acara seterusnya
calendar-scope-all = Semua acara
calendar-scope-respond-title = Jawapan untuk acara berulang
calendar-going = Hadir?
calendar-answer-yes = Ya
calendar-answer-no = Tidak
calendar-answer-maybe = Mungkin
calendar-answered-yes = Anda akan hadir
calendar-answered-no = Anda tidak akan hadir
calendar-answered-maybe = Anda mungkin hadir

## The card at the top of a mail with an invitation.

calendar-invite = Jemputan
calendar-invite-cancelled = Acara dibatalkan
calendar-invite-reply = { $name } menjawab
calendar-invite-reply-yes = { $name } menerima
calendar-invite-reply-no = { $name } menolak
calendar-invite-reply-maybe = { $name } mungkin hadir
calendar-invite-organizer = Dianjurkan oleh { $name }
calendar-invite-open = Buka dalam Kalendar
calendar-invite-not-yet = Belum ada dalam kalendar anda. Anda boleh menjawab selepas disegerakkan.
calendar-invite-by-mail = Tiada dalam kalendar anda: jawapan anda dihantar kepada penganjur melalui e-mel.
calendar-mail-yes = Diterima: { $title }
calendar-mail-yes-body = { $name } telah menerima jemputan ini.
calendar-mail-no = Ditolak: { $title }
calendar-mail-no-body = { $name } telah menolak jemputan ini.
calendar-mail-maybe = Diterima secara tentatif: { $title }
calendar-mail-maybe-body = { $name } telah menerima jemputan ini secara tentatif.
calendar-invite-your-day = Hari anda
calendar-invite-clashes =
    { $count ->
       *[other] Bertindih dengan { $count } acara
    }

## The day's agenda beside the mail.

agenda-show = Papar agenda hari ini
agenda-hide = Sembunyikan agenda
agenda-today = Hari ini, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = Tiada rancangan pada hari ini.
