# Katna Mail, Malay (Bahasa Melayu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = { $count } e-mel baharu
notify-and-more = dan { $count } lagi
notify-no-subject = (tiada subjek)
notify-unknown-sender = Pengirim tidak diketahui

## Reminders the user asked for (same buttons)

notify-snooze-back = Kembali daripada tunda
notify-no-reply = Belum ada balasan
notify-no-reply-to = Tiada sesiapa yang membalas “{ $subject }”.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } membuka { $subject }
notify-tracking-clicked = { $who } mengklik pautan dalam { $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail boleh dikemas kini
notify-update-ready-body = Versi { $version } telah dimuat turun. Kemas kini akan memasangnya dan memulakan semula Katna Mail.
notify-update = Kemas kini

## Reminders of calendar events

notify-event-now = Sekarang
notify-event-in-minutes = { $count ->
   *[other] Dalam { $count } minit
}
notify-event-in-hours = { $count ->
   *[other] Dalam { $count } jam
}
notify-event-in-days = { $count ->
    [1] Esok
   *[other] Dalam { $count } hari
}
notify-event-all-day = Sepanjang hari
notify-event-join = Sertai
notify-event-snooze = Tunda 5 minit
notify-task-done = Tandakan sebagai selesai

## The buttons of new-mail notifications and reminders

notify-open = Buka
notify-peek = Intai
notify-reply = Balas
notify-reply-placeholder = Balas kepada { $name }…
notify-send = Hantar
notify-reply-all = Balas semua
notify-mark-read = Tandai sebagai dibaca
notify-mark-all-read = Tandai semua sebagai dibaca
notify-archive = Arkibkan

## After Archive on a notification: a short note in the same place

notify-archived = Diarkibkan
notify-archived-count = { $count ->
   *[other] { $count } mesej dialihkan keluar dari peti masuk
}
notify-undo = Buat asal

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Balasan dihantar kepada { $name }
notify-open-in-katna = Buka dalam Katna
