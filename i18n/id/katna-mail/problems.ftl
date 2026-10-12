# Katna Mail, Indonesian (Bahasa Indonesia).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = Server email
problems-signed-out = { $provider } mengeluarkan Katna dari { $address }. Email berhenti disinkronkan.
problems-password-refused = { $provider } menolak sandi untuk { $address }. Mungkin sandinya sudah diubah.
problems-no-answer = { $provider } tidak merespons untuk { $address }. Katna terus mencoba.
problems-offline = Anda sedang offline. Email Anda tetap ada di sini, dan email yang Anda kirim menunggu sampai Anda kembali online.
problems-accounts-need-you = { $count ->
   *[other] { $count } akun memerlukan tindakan Anda
}
problems-show = Tampilkan
problems-later = Nanti
problems-new-password = Sandi baru
problems-try-again = Coba lagi

## The New password card

problems-password-title = Sandi baru
problems-password-detail = { $provider } menolak sandi tersimpan untuk { $address }. Ketik sandi yang baru; Katna memeriksanya sebelum menyimpannya.
problems-password-placeholder = Sandi
problems-password-show = Tampilkan sandi
problems-password-hide = Sembunyikan sandi
problems-password-cancel = Batal
problems-password-save = Simpan
problems-password-checking = Memeriksa…
problems-password-refused-again = { $provider } juga menolak sandi ini. Periksa lalu coba lagi.
problems-password-saved = Sandi untuk { $address } disimpan. Mengambil email Anda…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = Server email { $address } tidak menerima pemindahan { $count ->
    [1] sebuah pesan, jadi pesan itu kembali ke tempat semula.
   *[other] { $count } pesan, jadi pesan-pesan itu kembali ke tempat semula.
}
problems-refused-flags = Server email { $address } tidak menerima penandaan { $count ->
    [1] sebuah pesan (dibaca, berbintang…), jadi pesan itu kembali seperti semula.
   *[other] { $count } pesan (dibaca, berbintang…), jadi pesan-pesan itu kembali seperti semula.
}
problems-refused-label = Server email { $address } tidak menerima perubahan label { $count ->
    [1] sebuah pesan, jadi pesan itu kembali seperti semula.
   *[other] { $count } pesan, jadi pesan-pesan itu kembali seperti semula.
}
problems-refused-delete = Server email { $address } tidak menerima penghapusan { $count ->
    [1] sebuah pesan, jadi pesan itu kembali.
   *[other] { $count } pesan, jadi pesan-pesan itu kembali.
}
problems-refused-other = Server email { $address } tidak menerima { $count ->
    [1] sebuah perubahan, jadi Katna mengembalikannya seperti semula.
   *[other] { $count } perubahan, jadi Katna mengembalikannya seperti semula.
}
problems-details = Detail

## Katna's background service (katna-daemon) isn't running

service-starting = Memulai layanan latar belakang Katna…
service-failed = Layanan latar belakang Katna tidak mau berjalan, jadi email tidak disinkronkan.
service-start-again = Mulai lagi
service-started-again = Layanan latar belakang Katna berhenti dan sudah dimulai lagi.
service-details-title = Mengapa layanan tidak mau berjalan
service-details-body = Salin ini dan kirimkan bersama laporan Anda. Isinya tidak memuat email atau sandi.
service-details-copy = Salin
service-details-close = Tutup
service-not-running = Layanan latar belakang Katna tidak berjalan.
service-no-answer = Layanan latar belakang Katna tidak menjawab: { $error }
service-no-session = Tidak ada sesi D-Bus: { $error }

## Safe mode: an update left Katna's background service unable to start

safe-line = Katna berada dalam mode aman setelah ada masalah dengan pembaruan, jadi email tidak disinkronkan.
safe-try-again = Coba lagi
safe-restore = Pulihkan
safe-restoring = Memulihkan data Anda dari { $when }…
safe-restored = Data Anda dari { $when } telah dipulihkan. Isi sebelumnya disimpan di sebuah folder.
safe-show-folder = Tampilkan folder
safe-restore-failed = Tidak dapat memulihkan data Anda: { $error }
safe-restore-title = Pulihkan data Anda dari sebelum pembaruan?
safe-restore-body = Katna kembali ke salinan yang Anda pilih. Email yang tiba setelahnya diunduh lagi dari akun Anda.
safe-restore-none = Belum ada salinan. Katna membuatnya sebelum setiap pembaruan mengubah data Anda.
safe-restore-keep = Isi yang ada sekarang, termasuk email yang belum terkirim, draf, dan perubahan yang belum disinkronkan, disimpan dulu di sebuah folder, jadi tidak ada yang hilang.
safe-restore-cancel = Batal
safe-restore-mail = Email
safe-restore-pim = Akun dan kontak
safe-restore-blobs = Lampiran
safe-report-title = Laporan debug
safe-report-body = Salin ini dan lampirkan ke laporan bug Anda. Isinya tidak memuat email, alamat, atau sandi.
safe-report-restore = Pulihkan…
safe-report-copied = Laporan debug disalin
