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

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } membuka { $subject }
notify-tracking-clicked = { $who } mengklik link di { $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail dapat diperbarui
notify-update-ready-body = Versi { $version } telah diunduh. Perbarui akan memasangnya dan memulai ulang Katna Mail.
notify-update = Perbarui

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
notify-reply-all = Balas semua
notify-mark-read = Tandai sudah dibaca
notify-mark-all-read = Tandai semua sudah dibaca
notify-archive = Arsipkan

## After Archive on a notification: a short note in the same place

notify-archived = Diarsipkan
notify-archived-count = { $count ->
   *[other] { $count } pesan dipindahkan dari kotak masuk
}
notify-undo = Urungkan

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Balasan terkirim ke { $name }
notify-open-in-katna = Buka di Katna
