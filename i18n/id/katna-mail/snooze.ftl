# Katna Mail, Indonesian (Bahasa Indonesia).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = Tunda sampai…
snooze-later-today = Nanti hari ini
snooze-tomorrow = Besok
snooze-this-weekend = Akhir pekan ini
snooze-next-week = Minggu depan
snooze-pick = Pilih tanggal & waktu
snooze-back = Kembali ke pilihan waktu
snooze-type-placeholder = Ketik waktu
snooze-type-hint = Misalnya “tue 3pm”, “tomorrow”, atau “in 2 hours”
snooze-type-hint-unclear = Katna tidak dapat membacanya sebagai waktu
snooze-type-unclear = “{ $text }” bukan waktu yang dikenali Katna

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = Tunda
remind-tab = Ingatkan saya
snooze-says = Menyembunyikannya sampai waktu itu
remind-says = Membiarkannya di tempatnya dan memberi tahu Anda
remind-before-due = Sebelum jatuh tempo
remind-note = Catatan (opsional)
remind-note-placeholder = Subjeknya, jika dikosongkan
toast-remind-set = Pengingat diatur untuk { $date }
remind-chat-line = Pengingat { $date } · { $title }
remind-done = Selesai
toast-remind-done = Pengingat selesai
snooze-chat-line = Ditunda sampai { $date }
snooze-chat-change = Ubah

## The date and time picker

snooze-cancel = Batal
snooze-save = Simpan
snooze-in-the-past = Pilih waktu setelah sekarang.

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = Tindak lanjuti jika tidak ada balasan…
follow-up-title = Tindak lanjuti jika tidak ada balasan
follow-up-off = Nonaktif
follow-up-days = { $days ->
   *[other] { $days } hari
}
follow-up-weeks = { $weeks ->
   *[other] { $weeks } minggu
}
follow-up-pick = Pilih…
follow-up-pick-title = Tindak lanjuti jika tidak ada balasan sebelum
follow-up-remind = Ingatkan saya
follow-up-remind-note = Percakapan kembali ke atas Kotak Masuk Anda
follow-up-send = Kirimkan tindak lanjut untuk saya
follow-up-send-note = Ke orang yang sama, dalam percakapan yang sama
follow-up-send-encrypted = Tidak untuk email terenkripsi
follow-up-text-placeholder = Apa yang ingin ditulis
follow-up-text-named = Halo { $name }, saya hanya ingin memastikan Anda sudah melihat pesan saya di bawah.
follow-up-text = Halo, saya hanya ingin memastikan Anda sudah melihat pesan saya di bawah.
follow-up-template = Gunakan template
follow-up-signature = Tanda tangan Anda ditambahkan
follow-up-again = Jika masih tidak ada balasan, tindak lanjuti lagi setelah
follow-up-note = Berhenti begitu ada yang membalas dalam percakapan. Balasan otomatis tidak dihitung.
follow-up-note-send = Berhenti begitu ada yang membalas dalam percakapan. Dikirim pada hari kerja dari { $start } sampai { $end }, dan tidak pernah terlambat lebih dari sehari.
follow-up-cancel = Batal
follow-up-done = Selesai
follow-up-chip-send = Tindak lanjut dalam { $time }
follow-up-chip-remind = Pengingat dalam { $time }
follow-up-chip-send-on = Tindak lanjut { $date }
follow-up-chip-remind-on = Pengingat { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = Belum ada balasan
follow-up-card-title-waiting = Tindak lanjut Anda menunggu
follow-up-card-send = Katna mengirim tindak lanjut Anda pada { $date }. Berhenti saat ada yang membalas.
follow-up-card-send-twice = Katna mengirim tindak lanjut Anda pada { $date }, lalu sekali lagi nanti. Berhenti saat ada yang membalas.
follow-up-card-remind = Jika tidak ada yang membalas, percakapan ini kembali ke Kotak Masuk Anda pada { $date }.
follow-up-card-waiting = Jadwalnya tiba saat komputer Anda mati, jadi tidak dikirim terlambat. Kirim sekarang, pilih waktu baru, atau hentikan.
follow-up-card-edit = Edit
follow-up-card-edit-title = Tindak lanjuti pada
follow-up-card-send-now = Kirim sekarang
follow-up-card-stop = Hentikan
follow-up-chat-send = Tindak lanjut · { $date } jika tidak ada yang membalas
follow-up-chat-step = Tindak lanjut { $step } dari { $steps } · { $date } jika tidak ada yang membalas
follow-up-chat-waiting = Tindak lanjut menunggu · jadwalnya tiba saat komputer Anda mati
follow-up-chat-remind = Kembali ke Kotak Masuk { $date } jika tidak ada balasan
toast-follow-up-sent = Tindak lanjut terkirim
toast-follow-up-stopped = Tindak lanjut dihentikan
toast-follow-up-moved = Tindak lanjut dipindahkan ke { $date }
nudge-row = Dikirim { $days ->
   *[other] { $days } hari yang lalu
}. Tindak lanjuti?
nudge-row-tip = Tulis tindak lanjut ke semua orang di dalamnya
nudge-follow-up = Tindak lanjuti
nudge-dismiss = Abaikan
nudge-card-title = Belum ada balasan
nudge-card-text = Anda menanyakan sesuatu { $days ->
   *[other] { $days } hari yang lalu
} dan belum ada yang menjawab.
nudge-chat-line = Dikirim { $days ->
   *[other] { $days } hari yang lalu
}, belum ada balasan
toast-nudge-dismissed = Pengingat diabaikan
