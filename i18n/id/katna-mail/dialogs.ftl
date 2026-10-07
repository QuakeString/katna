# Katna Mail, Indonesian (Bahasa Indonesia).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Tentang Katna
about-tagline = Email dan kalender untuk desktop Linux
about-whats-new = Yang baru

## Updates, in a box under the version in About (only in packages that
## update themselves). $version is a version such as 0.0.0.r236.g1a2b3c4.

about-update-not-checked = Pembaruan belum diperiksa
about-update-checking = Memeriksa pembaruan…
about-update-up-to-date = Katna Mail sudah versi terbaru
about-update-check-failed = Tidak dapat memeriksa pembaruan
about-update-available = Versi { $version } tersedia
about-update-downloading = Mengunduh versi { $version }… { $percent }%
about-update-download-failed = Pengunduhan versi { $version } tidak selesai
about-update-ready = Versi { $version } siap dipasang
about-update-ready-detail = Katna Mail akan memulai ulang untuk menyelesaikan pembaruan.
about-update-confirm = Pasang versi { $version }?
about-update-confirm-detail = Katna Mail akan menutup, memasang pembaruan, lalu terbuka lagi seperti sebelumnya. Komputer Anda akan meminta sandi Anda.
about-update-installing = Memasang versi { $version }…
about-update-installing-detail = Masukkan sandi Anda di jendela yang terbuka.
about-update-cancelled = Pembaruan tidak dipasang, karena sandi tidak diberikan.
about-update-failed = Pembaruan tidak dapat dipasang: { $error }
about-update-unsupported = Salinan Katna Mail ini diperbarui oleh pengelola paket Anda.
about-update-restart-failed = Pembaruan telah dipasang, tetapi Katna Mail tidak dapat terbuka lagi ({ $error }). Buka sendiri.
about-update-check = Periksa pembaruan
about-update-download = Unduh
about-update-retry = Coba lagi
about-update-button = Perbarui
about-update-restart = Perbarui dan mulai ulang
about-update-cancel = Nanti saja
about-changelog = Catatan perubahan
about-source = Kode sumber
about-coffee = Traktir saya kopi
about-coffee-coffee = Kopi?
about-coffee-tea = Teh?
about-coffee-pizza = Pizza?
about-coffee-nothing = Tidak ada? Sama sekali?
about-coffee-water = Aku bertahan hidup dengan air putih!!
about-coffee-thanks = Terima kasih telah menggunakan Katna
about-coming-soon = Segera hadir
about-follow-me = Ikuti saya di
about-love-title = Dibuat dengan cinta untuk Rust, KDE, dan Linux
about-love-text = Rust membuat penulisan aplikasi email yang cepat dan aman terasa menyenangkan: Katna tidak memiliki kode unsafe. Desktop Plasma dari KDE dan suite PIM-nya menginspirasi Katna, sedangkan Linux dan komunitas perangkat lunak bebas membangun fondasi tempatnya berdiri. Terima kasih, dan terima kasih juga untuk pustaka di bawah ini.
about-kde-text = KDE membangun desktop tempat Katna paling terasa di rumah, dan KDE dibuat oleh para sukarelawan serta didanai oleh orang-orang seperti Anda. Jika Anda menyukai Plasma atau aplikasi KDE, pertimbangkan untuk berdonasi ke KDE.
about-donate-kde = Donasi ke KDE
about-gpui-title = Dibangun di atas GPUI, dari proyek Zed
about-gpui-text = Seluruh antarmuka Katna Mail dibangun di atas GPUI, framework UI cepat dengan akselerasi GPU yang dibuat Zed Industries untuk editor Zed. Setiap piksel, animasi, dan jendela yang Anda lihat digambar olehnya. Terima kasih, tim Zed, telah membangunnya secara terbuka. Apache-2.0.
about-gpui-github = GPUI di GitHub
about-personal-title = Proyek pribadi
about-personal-text = Katna Mail tidak berusaha menjadi sesuatu yang baru atau revolusioner. Ini adalah aplikasi email yang diinginkan pembuatnya, dengan fitur dan tampilan yang dipinjam dari Gmail, Mailspring, dan Thunderbird. Aplikasi ini hanya mungkin dibuat berkat kemajuan LLM sejauh ini.
about-built-on = DIBANGUN DENGAN PERANGKAT LUNAK BEBAS
about-credit-pimalaya = IMAP, SMTP, dan proses masuk (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = Membaca dan menulis IMAP
about-credit-tantivy = Penelusuran
about-credit-sqlite = Penyimpanan email
about-credit-rustls = Koneksi aman
about-credit-mail-parser = Membaca email, dari Stalwart Labs
about-credit-html5ever = Email HTML, dari proyek Servo
about-credit-zbus = Berkomunikasi dengan desktop melalui D-Bus dan portal
about-credit-oo7 = Sandi di keyring desktop
about-credit-hayro = Melihat dan mencetak PDF
about-credit-calamine = Pratinjau spreadsheet
about-credit-resvg = Gambar SVG
about-credit-jiff = Tanggal dan zona waktu
about-credit-spellbook = Pemeriksa ejaan, dari editor Helix
about-credit-smol = Mengerjakan banyak hal sekaligus
about-credit-color-schemes = Palet skema warna bawaan
about-all-libraries = Semua pustaka yang digunakan Katna ({ $count })
about-library-authors = oleh { $authors }
about-license = Katna adalah perangkat lunak bebas di bawah GNU GPL, versi 3 atau yang lebih baru.
about-close = Tutup

## What’s new (shown after an update)

whats-new-title = Yang baru di Katna Mail
whats-new-updated = Diperbarui ke versi { $version }
whats-new-version = Versi { $version }
whats-new-more = Dan { $count } lainnya di catatan perubahan lengkap.
whats-new-changelog = Catatan perubahan lengkap
whats-new-got-it = Mengerti

## First run: welcome page

onboarding-welcome-title = Selamat datang di Katna Mail
onboarding-welcome-lead = Email Anda di komputer Anda sendiri: cepat ditelusuri, bisa dibaca secara offline, dan tetap privat.
onboarding-fast-title = Cepat, bahkan saat offline
onboarding-fast-text = Katna menyimpan salinan email Anda di sini, sehingga membuka dan menelusurinya langsung terjadi, dengan atau tanpa koneksi.
onboarding-providers-title = Berfungsi dengan email Anda
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud, dan akun IMAP atau POP lainnya.
onboarding-private-title = Privat
onboarding-private-text = Email Anda langsung dikirim dari penyedia Anda ke komputer ini. Tidak ada server Katna yang melihatnya.
onboarding-get-started = Mulai

## First run: adding an account

onboarding-service-checking = Memeriksa layanan latar belakang Katna…
onboarding-service-running = Layanan latar belakang Katna sedang berjalan.
onboarding-service-missing = Layanan latar belakang Katna tidak berjalan
onboarding-service-start = Layanan ini mengambil dan mengirim email Anda. Jalankan dari terminal, lalu periksa lagi:
onboarding-check-again = Periksa lagi
onboarding-account-title = Tambahkan akun email Anda
onboarding-account-lead = Ketik alamat email dan sandi Anda, lalu Katna akan menemukan setelan servernya. Gmail, Yahoo, dan iCloud memerlukan sandi aplikasi, yang dibuat di setelan keamanan akun Anda.
onboarding-add-account = Tambahkan akun
onboarding-back = Kembali

## First run: choosing the look

onboarding-look-title = Sesuaikan dengan selera Anda
onboarding-look-lead = Pilih cara email dibuka dan tampilan Katna. Anda dapat mengubahnya kapan saja di setelan cepat.
onboarding-reading-pane = Panel baca
onboarding-pane-right = Di kanan daftar
onboarding-pane-none = Tanpa pemisahan
onboarding-theme = Tema
onboarding-theme-system = Sistem
onboarding-theme-light = Terang
onboarding-theme-dark = Gelap
onboarding-density = Kepadatan
onboarding-density-default = Default
onboarding-density-compact = Ringkas
onboarding-continue = Lanjutkan

## First start: the Katna account page. A Katna account is an account on
## Katna's own server, not a mail account; see katna-account.ftl.

onboarding-katna-title = Dapatkan lebih banyak dengan akun Katna
onboarding-katna-lead = Ini opsional. Akun ini mengaktifkan fitur online Katna, dan Anda dapat membuatnya nanti di Setelan > Langganan.
onboarding-katna-receipts-title = Tanda terima baca
onboarding-katna-receipts-text = Lihat kapan orang membuka email yang Anda kirim.
onboarding-katna-links-title = Pelacakan link
onboarding-katna-links-text = Lihat link mana di email Anda yang diklik.
onboarding-katna-activity-title = Aktivitas
onboarding-katna-activity-text = Pembukaan dan klik untuk semua yang Anda kirim, di satu tempat.
onboarding-katna-translate-title = Terjemahan otomatis
onboarding-katna-translate-text = Baca email yang ditulis dalam bahasa lain dalam bahasa Anda sendiri.
onboarding-katna-private = Akun ini memiliki sandinya sendiri. Info masuk email Anda tidak pernah keluar dari komputer ini.

## First run: done

onboarding-ready-title = Semua sudah siap
onboarding-ready-lead = Katna sedang mengambil email Anda. Email muncul saat tiba, dan email baru akan muncul dengan sendirinya.
onboarding-ready-lead-address = Katna sedang mengambil email { $address }. Email muncul saat tiba, dan email baru akan muncul dengan sendirinya.
onboarding-ready-tour = Ikuti tur satu menit untuk melihat letak semuanya?
onboarding-skip = Lewati untuk sekarang
onboarding-take-tour = Ikuti tur

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Bantu tingkatkan Katna
share-lead = Saat Katna mengalami error, Katna menyimpan laporan di komputer ini. Mengirim laporan ini membantu memperbaiki masalahnya. Anda dapat mengubah ini kapan saja di Setelan > Masukan pengguna.
share-sent = Apa yang dikirim
share-sent-detail = Laporan error seperti yang dapat Anda lihat di Setelan: apa yang error dan di bagian mana dari Katna, versinya, sistem Linux dan desktop Anda, serta baris log terakhir Katna, yang dapat menyebutkan nama folder email.
share-never-sent = Apa yang tidak pernah dikirim
share-never-sent-detail = Pesan, kontak, sandi, alamat IP, nama pengguna, atau nama komputer Anda. Alamat email dihapus dari laporan.
share-where = Ke mana laporan dikirim
share-where-detail = Pelacak error Katna di Sentry, disimpan di Uni Eropa. Tidak ada ID yang mengaitkan laporan dengan Anda.
share-dont-send = Jangan kirim
share-send = Kirim laporan error
share-sending = Laporan error akan dikirim. Terima kasih.
share-local = Laporan error tetap di komputer ini.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Selamat datang di Katna Mail
tour-welcome-text = Tur satu menit menunjukkan letak semuanya.
tour-not-now = Nanti saja
tour-start = Ikuti tur
tour-close = Tutup
tour-skip = Lewati tur
tour-back = Kembali
tour-done = Selesai
tour-next = Berikutnya
tour-step = { $step } dari { $total }
tour-compose-title = Tulis pesan
tour-compose-text = Tulis membuka pesan baru di kanan bawah, sehingga Anda dapat terus membaca sambil menulis.
tour-search-title = Telusuri semua email Anda
tour-search-text = Penelusuran juga berfungsi secara offline. Tombol di ujung kanan menambahkan filter: pengirim, penerima, subjek, tanggal, dan lampiran.
tour-menu-title = Tampilkan atau sembunyikan folder
tour-menu-text = Tombol ini melipat daftar folder. Saat daftar tersembunyi, arahkan penunjuk ke Email di sebelah kiri untuk melihat folder.
tour-apps-title = Aplikasi Anda
tour-apps-text = Email ada di sini, di samping Kalender, Kontak, Tugas, Catatan, dan File.
tour-tabs-title = Tab Kotak Masuk
tour-tabs-text = Email baru dipilah ke Utama, Promosi, Sosial, Pembaruan, dan Forum. Anda dapat menonaktifkan tab di setelan cepat.
tour-list-title = Pesan Anda
tour-list-text = Klik pesan untuk membacanya. Arahkan penunjuk ke pesan untuk tindakan cepat, klik kanan untuk opsi lainnya, atau centang beberapa pesan untuk menanganinya sekaligus.
tour-settings-title = Setelan cepat
tour-settings-text = Ubah panel baca, kepadatan, dan tema di sini. Tur juga dapat dimulai lagi dari sini.
tour-account-title = Akun Anda
tour-account-text = Lihat akun yang sedang Anda gunakan, dan tambahkan akun lain.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Layanan latar belakang Katna berhenti secara tiba-tiba.
   *[other] Layanan latar belakang Katna berhenti secara tiba-tiba. { $more } laporan error lainnya tersimpan.
}
crash-mail = { $more ->
    [0] Katna Mail tertutup secara tiba-tiba terakhir kali.
   *[other] Katna Mail tertutup secara tiba-tiba terakhir kali. { $more } laporan error lainnya tersimpan.
}
crash-view = Lihat laporan
crash-view-tooltip = Buka laporan, yang tersimpan di komputer ini
crash-copy = Salin laporan
crash-close = Tutup

## Sign in again (a bar at the bottom when Google or Microsoft stopped
## letting an account in; $provider: Google or Microsoft)

sign-in-again-button = Masuk
sign-in-again-tooltip = Buka halaman masuk { $provider } di browser Anda
sign-in-again-waiting = Menunggu browser Anda…
google-api-off = { $api } dinonaktifkan di project Google Cloud milik Katna.
google-api-turn-on = Aktifkan
google-api-turn-on-tooltip = Buka Google Cloud untuk mengaktifkan { $api }, lalu tekan Coba lagi
sign-in-again-done = Sudah masuk lagi ke { $address }. Mengambil email Anda…

## Before deleting several conversations, or deleting for good

delete-ask-title = { $kind ->
    [conversation] Pindahkan { $count } percakapan ke Sampah?
   *[message] Pindahkan { $count } pesan ke Sampah?
}
delete-ask-body = { $count ->
   *[other] Anda bisa mengurungkannya sesaat setelahnya, atau mengembalikannya dari Sampah nanti.
}
delete-ask-confirm = Pindahkan ke Sampah
delete-forever-title = { $kind ->
    [conversation] Hapus { $count } percakapan selamanya?
   *[message] Hapus { $count } pesan selamanya?
}
delete-forever-body = { $count ->
   *[other] Email ini juga dihapus di server. Ini tidak bisa diurungkan.
}
delete-forever-confirm = Hapus selamanya
delete-ask-dont-ask = Jangan tanya lagi
delete-ask-cancel = Batal
