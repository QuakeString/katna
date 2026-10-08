# Katna Mail, Malay (Bahasa Melayu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = Tunda hingga…
snooze-later-today = Kemudian hari ini
snooze-tomorrow = Esok
snooze-this-weekend = Hujung minggu ini
snooze-next-week = Minggu depan
snooze-pick = Pilih tarikh & masa
snooze-back = Kembali ke senarai masa
snooze-type-placeholder = Taip masa
snooze-type-hint = Contohnya “sel 3ptg”, “esok” atau “dalam 2 jam”
snooze-type-hint-unclear = Katna tidak dapat membacanya sebagai masa
snooze-type-unclear = “{ $text }” bukan masa yang Katna kenali

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = Tunda
remind-tab = Ingatkan saya
snooze-says = Menyembunyikannya hingga masa itu
remind-says = Mengekalkannya di tempatnya dan memberitahu anda
remind-before-due = Sebelum tarikh akhir
remind-note = Nota (pilihan)
remind-note-placeholder = Subjek, jika dibiarkan kosong
toast-remind-set = Peringatan ditetapkan pada { $date }
remind-chat-line = Peringatan { $date } · { $title }
remind-done = Selesai
toast-remind-done = Peringatan selesai
snooze-chat-line = Ditunda hingga { $date }
snooze-chat-change = Tukar

## The date and time picker

snooze-cancel = Batal
snooze-save = Simpan
snooze-in-the-past = Pilih masa yang lebih lewat daripada sekarang.

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = Susuli jika tiada balasan…
follow-up-title = Susuli jika tiada balasan
follow-up-off = Mati
follow-up-days = { $days ->
   *[other] { $days } hari
}
follow-up-weeks = { $weeks ->
   *[other] { $weeks } minggu
}
follow-up-pick = Pilih…
follow-up-pick-title = Susuli jika tiada balasan menjelang
follow-up-remind = Ingatkan saya
follow-up-remind-note = Perbualan kembali ke atas Peti Masuk anda
follow-up-send = Hantar susulan bagi pihak saya
follow-up-send-note = Kepada orang yang sama, dalam perbualan yang sama
follow-up-send-encrypted = Bukan untuk mel yang disulitkan
follow-up-text-placeholder = Apa yang hendak ditulis
follow-up-text-named = Hai { $name }, sekadar bertanya sama ada anda telah melihat mesej saya di bawah.
follow-up-text = Hai, sekadar bertanya sama ada anda telah melihat mesej saya di bawah.
follow-up-template = Gunakan templat
follow-up-signature = Tandatangan anda ditambah
follow-up-again = Jika masih tiada balasan, susuli sekali lagi selepas
follow-up-note = Berhenti sebaik sahaja sesiapa dalam perbualan membalas. Balasan automatik tidak dikira.
follow-up-note-send = Berhenti sebaik sahaja sesiapa dalam perbualan membalas. Dihantar pada hari bekerja dari { $start } hingga { $end }, dan tidak pernah lewat lebih daripada sehari.
follow-up-cancel = Batal
follow-up-done = Selesai
follow-up-chip-send = Susulan dalam { $time }
follow-up-chip-remind = Peringatan dalam { $time }
follow-up-chip-send-on = Susulan { $date }
follow-up-chip-remind-on = Peringatan { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = Belum ada balasan
follow-up-card-title-waiting = Susulan anda sedang menunggu
follow-up-card-send = Katna menghantar susulan anda pada { $date }. Ia berhenti apabila sesiapa membalas.
follow-up-card-send-twice = Katna menghantar susulan anda pada { $date }, kemudian sekali lagi selepas itu. Ia berhenti apabila sesiapa membalas.
follow-up-card-remind = Jika tiada sesiapa membalas, perbualan ini kembali ke Peti Masuk anda pada { $date }.
follow-up-card-waiting = Masanya tiba semasa komputer anda dimatikan, jadi ia tidak dihantar lewat. Hantar sekarang, pilih masa baharu atau hentikannya.
follow-up-card-edit = Sunting
follow-up-card-edit-title = Susuli pada
follow-up-card-send-now = Hantar sekarang
follow-up-card-stop = Hentikan
follow-up-chat-send = Susulan · { $date } jika tiada sesiapa membalas
follow-up-chat-step = Susulan { $step } daripada { $steps } · { $date } jika tiada sesiapa membalas
follow-up-chat-waiting = Susulan menunggu · masanya tiba semasa komputer anda dimatikan
follow-up-chat-remind = Kembali ke Peti Masuk { $date } jika tiada balasan
toast-follow-up-sent = Susulan dihantar
toast-follow-up-stopped = Susulan dihentikan
toast-follow-up-moved = Susulan dialihkan ke { $date }
nudge-row = Dihantar { $days ->
   *[other] { $days } hari yang lalu
}. Susuli?
nudge-row-tip = Tulis susulan kepada semua orang di dalamnya
nudge-follow-up = Susuli
nudge-dismiss = Ketepikan
nudge-card-title = Belum ada balasan
nudge-card-text = Anda bertanyakan sesuatu { $days ->
   *[other] { $days } hari yang lalu
} dan tiada sesiapa menjawab.
nudge-chat-line = Dihantar { $days ->
   *[other] { $days } hari yang lalu
}, belum ada balasan
toast-nudge-dismissed = Dorongan diketepikan
