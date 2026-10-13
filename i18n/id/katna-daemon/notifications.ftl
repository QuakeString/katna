# Katna Mail, Indonesian (Bahasa Indonesia).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = { $count } email baru
notify-and-more = dan { $count } lainnya
notify-no-subject = (tanpa subjek)
notify-unknown-sender = Pengirim tidak dikenal

## Reminders the user asked for (same buttons)

notify-snooze-back = Kembali dari penundaan
notify-no-reply = Belum ada balasan
notify-no-reply-to = Belum ada yang membalas “{ $subject }”.
notify-follow-up-sent = Tindak lanjut terkirim
notify-follow-up-sent-to = Belum ada yang membalas “{ $subject }”, jadi Katna mengirim tindak lanjut.
notify-follow-up-waiting = Tindak lanjut tidak terkirim
notify-follow-up-waiting-to = Jadwalnya tiba saat komputer ini mati. “{ $subject }” kembali ke Kotak Masuk Anda.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } membuka { $subject }
notify-tracking-clicked = { $who } mengklik link di { $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail dapat diperbarui
notify-update-ready-body = Versi { $version } telah diunduh. Perbarui akan memasangnya dan memulai ulang Katna Mail.
notify-update = Perbarui

## Something needs the user, shown once per problem

notify-signed-out = Masuk lagi
notify-signed-out-body = { $provider } mengeluarkan Katna dari { $address }. Email berhenti disinkronkan.
notify-sign-in = Masuk
notify-password-refused = Sandi ditolak
notify-password-refused-body = Server email menolak sandi untuk { $address }. Mungkin sandinya sudah diubah.
notify-new-password = Sandi baru
notify-not-sent = “{ $subject }” tidak terkirim
notify-not-sent-no-subject = Sebuah pesan tidak terkirim
notify-not-sent-body = Pesan ada di Kotak Keluar, beserta alasannya.
notify-open-outbox = Buka Kotak Keluar

## Reminders of calendar events

notify-event-now = Sekarang
notify-event-in-minutes = { $count ->
   *[other] { $count } menit lagi
}
notify-event-in-hours = { $count ->
   *[other] { $count } jam lagi
}
notify-event-in-days = { $count ->
    [1] Besok
   *[other] { $count } hari lagi
}
notify-event-all-day = Sepanjang hari
notify-event-join = Gabung
notify-event-snooze = Tunda 5 mnt
notify-task-done = Tandai selesai

## The buttons of new-mail notifications and reminders

notify-open = Buka
notify-peek = Intip
notify-reply = Balas
notify-reply-placeholder = Balas ke { $name }…
notify-send = Kirim
notify-reply-quote-header = Pada { $date }, { $from } menulis:
notify-reply-quote-header-no-date = { $from } menulis:
notify-reply-all = Balas semua
notify-mark-read = Tandai sudah dibaca
notify-mark-all-read = Tandai semua sudah dibaca
notify-archive = Arsipkan
notify-snooze-hour = Tunda 1 jam
notify-snooze-tomorrow = Besok
notify-copy-code = Salin { $code }
notify-link-verify = Verifikasi di { $domain }
notify-link-confirm = Konfirmasi di { $domain }
notify-link-activate = Aktifkan di { $domain }

## After Archive on a notification: a short note in the same place

notify-archived = Diarsipkan
notify-archived-count = { $count ->
   *[other] { $count } pesan dipindahkan dari kotak masuk
}
notify-undo = Urungkan

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = Kode disalin
notify-code-not-copied = Tidak dapat menyalin kode

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Balasan terkirim ke { $name }
notify-open-in-katna = Buka di Katna
