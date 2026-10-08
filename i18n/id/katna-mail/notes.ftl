# Katna Mail, Indonesian (Bahasa Indonesia): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Catatan
notes-view-reminders = Pengingat
notes-view-archive = Arsip
notes-view-trash = Sampah
notes-edit-labels = Edit label
notes-search = Telusuri catatan
notes-loading = Membuka catatan Anda…

## Board

notes-take-a-note = Buat catatan…
notes-new-list = Daftar baru
notes-new-note = Catatan baru
notes-pinned = Disematkan
notes-others = Lainnya
notes-empty = Catatan yang Anda tambahkan akan muncul di sini
notes-archive-empty = Catatan yang diarsipkan akan muncul di sini
notes-trash-empty = Tidak ada catatan di Sampah
notes-none-found = Tidak ada catatan yang cocok
notes-label-empty = Belum ada catatan dengan label ini
notes-reminders-empty = Catatan dengan pengingat mendatang muncul di sini
notes-trash-note = Catatan di Sampah akan dihapus setelah 7 hari.
notes-empty-trash = Kosongkan sampah
notes-ticked = { $count ->
   *[other] + { $count } item dicentang
}
notes-select = Pilih catatan
notes-selected = { $count ->
   *[other] { $count } dipilih
}
notes-select-clear = Hapus pilihan

## A note's buttons

notes-pin = Sematkan catatan
notes-unpin = Lepas sematan catatan
notes-archive = Arsipkan
notes-unarchive = Batal arsipkan
notes-delete = Hapus catatan
notes-restore = Pulihkan
notes-delete-forever = Hapus permanen
notes-color = Warna latar belakang
notes-checkboxes = Tampilkan atau sembunyikan kotak centang
notes-labels = Label
notes-close = Tutup
notes-more = Lainnya
notes-make-copy = Buat salinan
notes-remind = Ingatkan saya
notes-add-picture = Tambahkan gambar
notes-history = Riwayat versi
notes-ai = Bantu saya menulis
notes-send-as-mail = Kirim sebagai email
notes-save-markdown = Simpan sebagai Markdown
notes-save-pdf = Simpan sebagai PDF

## The open note

notes-title = Judul
notes-edited = Diedit { $date }
notes-on-this-computer = Di komputer ini
notes-where = Tempat catatan ini disimpan
notes-untitled = Catatan tanpa judul

## Pictures

notes-picture-choose = Tambahkan gambar
notes-picture-remove = Hapus gambar
notes-picture-too-big = Gambar hingga { $size } dapat dimasukkan ke catatan
notes-picture-kind = File itu bukan gambar yang dapat ditampilkan Katna
notes-picture-unreadable = Tidak dapat membaca { $name }: { $error }

## Reminders

notes-remind-me = Ingatkan saya
notes-remind-off = Hapus pengingat
notes-remind-in-the-past = Pilih waktu yang belum lewat
notes-remind-today = Hari ini, { $time }
notes-remind-tomorrow = Besok, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = Pengingat diatur untuk { $when }
notes-reminder-off = Pengingat dihapus

## Links between notes

notes-link-note = Tautkan catatan
notes-link-new = Catatan baru “{ $title }”
notes-linked-from = Ditautkan dari
notes-link-gone = Catatan itu sudah tidak ada di sini

## Version history

notes-versions = Versi
notes-version-now = Sekarang
notes-version-here = Anda, di komputer ini
notes-version-yesterday = Kemarin, { $time }
notes-version-changes = { $count ->
   *[other] { $count } perubahan
}
notes-version-from = Dari { $device }
notes-version-elsewhere = Dari perangkat lain
notes-version-created = Dibuat
notes-version-restore = Pulihkan versi ini
notes-version-restored = Versi dipulihkan
notes-history-none = Belum ada versi sebelumnya

## AI help

notes-ai-tidy = Rapikan teks
notes-ai-checklist = Ubah menjadi daftar periksa
notes-ai-summarise = Ringkas
notes-ai-empty = Tulis sesuatu terlebih dahulu
notes-ai-tidied = Teks dirapikan. Ctrl+Z mengembalikannya.
notes-ai-listed = Diubah menjadi daftar periksa. Ctrl+Z mengembalikannya.
notes-ai-summarised = Ringkasan ditambahkan di atas

## Labels

notes-label-note = Beri label pada catatan
notes-label-name = Masukkan nama label
notes-label-create = Buat “{ $name }”
notes-label-remove = Hapus label dari catatan
notes-label-delete = Hapus label
notes-labels-none = Belum ada label. Tambahkan dari tombol label pada catatan.
notes-labels-done = Selesai
notes-label-renamed = Label diganti namanya menjadi “{ $name }”
notes-label-deleted = Label “{ $name }” dihapus

## A note about a mail

notes-mail = Email
notes-open-mail = Buka email
notes-open-note = Buka catatan

## Meeting notes

notes-meeting-take = Buat catatan rapat
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = Peserta: { $names }
notes-meeting-notes = Catatan
notes-meeting-actions = Item tindakan
notes-event = Acara
notes-open-event = Buka acara

## Formatting

notes-format = Pemformatan
notes-format-heading-1 = Judul 1
notes-format-heading-2 = Judul 2
notes-format-normal = Teks normal
notes-format-bold = Tebal
notes-format-italic = Miring
notes-format-underline = Garis bawah
notes-format-quote = Kutipan
notes-format-code = Kode
notes-format-divider = Pemisah
notes-format-clear = Hapus pemformatan

## Tasks

notes-make-task = Jadikan tugas

## Colors (tooltips)

notes-color-none = Tanpa warna
notes-color-coral = Koral
notes-color-peach = Persik
notes-color-sand = Pasir
notes-color-mint = Mint
notes-color-sage = Sage
notes-color-fog = Kabut
notes-color-storm = Badai
notes-color-dusk = Senja
notes-color-blossom = Bunga
notes-color-clay = Tanah liat
notes-color-chalk = Kapur

## Messages at the foot of the window

notes-archived = Catatan diarsipkan
notes-unarchived = Arsip catatan dibatalkan
notes-trashed = Catatan dipindahkan ke Sampah
notes-restored = Catatan dipulihkan
notes-saved = Catatan disimpan
notes-pinned-count = { $count ->
    [1] Catatan disematkan
   *[other] { $count } catatan disematkan
}
notes-unpinned-count = { $count ->
    [1] Sematan catatan dilepas
   *[other] Sematan { $count } catatan dilepas
}
notes-colored-count = { $count ->
    [1] Warna diubah
   *[other] Warna { $count } catatan diubah
}
notes-archived-count = { $count ->
    [1] Catatan diarsipkan
   *[other] { $count } catatan diarsipkan
}
notes-unarchived-count = { $count ->
    [1] Arsip catatan dibatalkan
   *[other] Arsip { $count } catatan dibatalkan
}
notes-trashed-count = { $count ->
    [1] Catatan dipindahkan ke Sampah
   *[other] { $count } catatan dipindahkan ke Sampah
}
notes-restored-count = { $count ->
    [1] Catatan dipulihkan
   *[other] { $count } catatan dipulihkan
}
notes-copied-count = { $count ->
    [1] Salinan dibuat
   *[other] { $count } salinan dibuat
}
notes-empty-discarded = Catatan kosong dibuang
notes-mail-gone = Email itu sudah tidak ada di sini
notes-deleted-forever = { $count ->
   *[other] { $count } catatan dihapus permanen
}
