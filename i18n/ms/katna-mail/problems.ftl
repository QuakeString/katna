# Katna Mail, Malay (Bahasa Melayu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = Pelayan mel
problems-signed-out = { $provider } telah mengelog keluar Katna daripada { $address }. Mel berhenti disegerakkan.
problems-password-refused = { $provider } menolak kata laluan untuk { $address }. Kata laluan itu mungkin telah berubah.
problems-no-answer = { $provider } tidak menjawab untuk { $address }. Katna terus mencuba.
problems-offline = Anda di luar talian. Mel anda masih di sini, dan mel yang anda hantar menunggu sehingga anda kembali dalam talian.
problems-accounts-need-you = { $count ->
   *[other] { $count } akaun memerlukan perhatian anda
}
problems-show = Tunjukkan
problems-later = Nanti
problems-new-password = Kata laluan baharu
problems-try-again = Cuba lagi

## The New password card

problems-password-title = Kata laluan baharu
problems-password-detail = { $provider } menolak kata laluan yang disimpan untuk { $address }. Taip kata laluan baharu; Katna menyemaknya sebelum menyimpannya.
problems-password-placeholder = Kata laluan
problems-password-show = Tunjukkan kata laluan
problems-password-hide = Sembunyikan kata laluan
problems-password-cancel = Batal
problems-password-save = Simpan
problems-password-checking = Menyemak…
problems-password-refused-again = { $provider } juga menolak kata laluan ini. Semak dan cuba lagi.
problems-password-saved = Kata laluan disimpan untuk { $address }. Mendapatkan mel anda…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = Pelayan mel { $address } tidak menerima pengalihan { $count ->
   *[other] { $count } mesej, jadi mesej itu kembali ke tempat asalnya.
}
problems-refused-flags = Pelayan mel { $address } tidak menerima penandaan { $count ->
   *[other] { $count } mesej (dibaca, dibintangi…), jadi mesej itu kembali seperti asal.
}
problems-refused-label = Pelayan mel { $address } tidak menerima perubahan label { $count ->
   *[other] { $count } mesej, jadi mesej itu kembali seperti asal.
}
problems-refused-delete = Pelayan mel { $address } tidak menerima pemadaman { $count ->
   *[other] { $count } mesej, jadi mesej itu kembali.
}
problems-refused-other = Pelayan mel { $address } tidak menerima { $count ->
   *[other] { $count } perubahan, jadi Katna mengembalikannya seperti asal.
}
problems-details = Butiran

## Katna's background service (katna-daemon) isn't running

service-starting = Memulakan perkhidmatan latar belakang Katna…
service-failed = Perkhidmatan latar belakang Katna tidak dapat dimulakan, jadi mel tidak disegerakkan.
service-start-again = Mulakan semula
service-started-again = Perkhidmatan latar belakang Katna terhenti dan telah dimulakan semula.
service-details-title = Mengapa perkhidmatan tidak dapat dimulakan
service-details-body = Salin ini dan hantarkannya bersama laporan anda. Tiada mel atau kata laluan di dalamnya.
service-details-copy = Salin
service-details-close = Tutup
service-not-running = Perkhidmatan latar belakang Katna tidak berjalan.
service-no-answer = Perkhidmatan latar belakang Katna tidak menjawab: { $error }
service-no-session = Tiada sesi D-Bus: { $error }

## Safe mode: an update left Katna's background service unable to start

safe-line = Katna berada dalam mod selamat selepas masalah dengan kemas kini, jadi mel tidak disegerakkan.
safe-try-again = Cuba lagi
safe-restore = Pulihkan
safe-restoring = Memulihkan data anda daripada { $when }…
safe-restored = Data anda telah dipulihkan daripada { $when }. Apa yang ada sebelum ini disimpan dalam folder.
safe-show-folder = Tunjukkan folder
safe-restore-failed = Tidak dapat memulihkan data anda: { $error }
safe-restore-title = Pulihkan data anda daripada sebelum kemas kini?
safe-restore-body = Katna kembali kepada salinan yang anda pilih. Mel yang tiba selepasnya dimuat turun semula daripada akaun anda.
safe-restore-none = Belum ada salinan lagi. Katna membuat satu salinan sebelum setiap kemas kini mengubah data anda.
safe-restore-keep = Apa yang ada sekarang, termasuk mel yang belum dihantar, draf dan perubahan yang belum disegerakkan, disimpan dalam folder dahulu, jadi tiada apa yang hilang.
safe-restore-cancel = Batal
safe-restore-mail = Mel
safe-restore-pim = Akaun dan kenalan
safe-restore-blobs = Lampiran
safe-report-title = Laporan nyahpepijat
safe-report-body = Salin ini dan lampirkannya pada laporan pepijat anda. Tiada mel, alamat atau kata laluan di dalamnya.
safe-report-restore = Pulihkan…
safe-report-copied = Laporan nyahpepijat disalin
