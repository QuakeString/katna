# Katna Mail, Indonesian (Bahasa Indonesia).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Aturan
settings-rules-summary = Sortir, beri label, teruskan, atau senyapkan email baru secara otomatis
settings-rules-intro = Aturan menyortir email baru secara otomatis, sesuai urutan ini. Seret untuk mengurutkan ulang.
settings-rules-all-accounts = Semua akun
settings-rules-new = Aturan baru
settings-rules-none = Belum ada aturan. Aturan menyortir email baru secara otomatis: berdasarkan pengirim, subjek, atau kata.
settings-rules-none-account = Belum ada aturan untuk akun ini.
settings-rules-drag = Seret untuk mengurutkan ulang
settings-rules-edit = Edit aturan
settings-rules-turn-off = Nonaktifkan aturan ini
settings-rules-turn-on = Aktifkan aturan ini

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = Aturan siap pakai
settings-rules-starters-intro = Nonaktif sampai Anda mengaktifkannya. Aturan ini berlaku untuk semua akun Anda; edit untuk mengubahnya.
settings-rules-starter-turning-on = Mengaktifkan “{ $name }”…
settings-rules-starter-failed = Tidak dapat mengaktifkan “{ $name }”: { $error }
rules-starter-promotions = Senyapkan promosi
rules-starter-newsletters = Buletin ke Bacaan
rules-starter-receipts = Tanda terima dan faktur
rules-starter-deliveries = Pengiriman paket
rules-starter-train = Tiket kereta
rules-starter-flight = Tiket pesawat
rules-starter-codes = Kode sekali pakai
rules-starter-security = Peringatan keamanan
rules-starter-social = Email sosial
rules-starter-invites = Undangan kalender
rules-starter-folder-reading = Bacaan
rules-starter-folder-receipts = Tanda terima
rules-starter-folder-deliveries = Pengiriman
rules-starter-folder-travel = Perjalanan
rules-starter-folder-social = Sosial
rules-runs-katna = Berjalan di Katna
rules-runs-gmail = Berjalan di Gmail
rules-runs-sieve = Berjalan di server
rules-stopped = Dihentikan
rules-error-folder-gone = Folder yang dipakai aturan ini sudah tidak ada. Edit aturan untuk memilih folder lain.
rules-error-no-archive = Akun ini tidak memiliki folder arsip. Edit aturan untuk melakukan hal lain.
rules-error-no-trash = Akun ini tidak memiliki folder Sampah. Edit aturan untuk melakukan hal lain.
rules-error-cannot-send = Akun ini tidak dapat mengirim email, jadi aturan tidak dapat meneruskannya.
rules-error-other = { $error }. Edit aturan lalu aktifkan lagi.
settings-folders = Folder
settings-folders-summary = Jumlah belum dibaca di panel folder
settings-folders-unread-counts = Jumlah belum dibaca di setiap folder
settings-folders-unread-counts-detail = Nonaktif: hanya Kotak Masuk yang menampilkan jumlah belum dibaca

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } dan { $next }
rules-summary-or = { $first } atau { $next }
rules-summary-more = { $count } lainnya
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Ada lampiran
rules-summary-no-attachment = Tanpa lampiran
rules-summary-mailing-list = Dari milis
rules-summary-not-mailing-list = Bukan dari milis
rules-summary-tab = Di tab { $tab }
rules-summary-not-tab = Tidak di tab { $tab }
rules-summary-move = pindahkan ke { $folder }
rules-summary-archive = lewati kotak masuk
rules-summary-trash = pindahkan ke sampah
rules-summary-mark-read = tandai sudah dibaca
rules-summary-star = beri bintang
rules-summary-important = tandai penting
rules-summary-label = beri label { $label }
rules-summary-forward = teruskan ke { $address }
rules-summary-dont-notify = jangan beri notifikasi
rules-summary-read-after = { $count ->
   *[other] tandai sudah dibaca setelah { $count } hari
}
rules-summary-folder-gone = folder yang sudah tidak ada

## The rule editor

rules-editor-new-title = Aturan baru
rules-editor-edit-title = Edit aturan
rules-editor-name-hint = Nama aturan
rules-editor-when = Saat email baru cocok dengan
rules-editor-of-these = kondisi ini:
rules-mode-all = semua
rules-mode-any = salah satu
rules-field-from = Dari
rules-field-to = Kepada
rules-field-cc = Cc
rules-field-any-recipient = Kepada atau Cc
rules-field-reply-to = Balas ke
rules-field-subject = Subjek
rules-field-body = Teks
rules-field-attachment-name = Nama lampiran
rules-field-has-attachment = Ada lampiran
rules-field-mailing-list = Dari milis
rules-field-tab = Tab kotak masuk
rules-comparator-contains = berisi
rules-comparator-not-contains = tidak berisi
rules-comparator-begins-with = diawali dengan
rules-comparator-ends-with = diakhiri dengan
rules-comparator-equals = sama persis dengan
rules-comparator-matches = cocok dengan pola
rules-has-yes = ya
rules-has-no = tidak
rules-editor-value-hint = Kata atau alamat
rules-editor-add-condition = Tambahkan kondisi
rules-editor-remove = Hapus
rules-editor-then = Lalu:
rules-action-move = Pindahkan ke
rules-action-archive = Lewati kotak masuk (arsipkan)
rules-action-trash = Pindahkan ke sampah
rules-action-mark-read = Tandai sudah dibaca
rules-action-star = Beri bintang
rules-action-important = Tandai penting
rules-action-label = Tambahkan label
rules-action-forward = Teruskan ke
rules-action-dont-notify = Jangan beri notifikasi
rules-action-read-after = Tandai sudah dibaca setelah
rules-editor-choose-folder = Pilih folder
rules-editor-choose-label = Pilih label
rules-editor-new-folder = Baru: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = Alamat email
rules-editor-days = hari
rules-editor-add-action = Tambahkan tindakan
rules-editor-stop = Berhenti di sini: aturan berikutnya tidak dijalankan pada email ini
rules-editor-accounts = Akun:
rules-editor-accounts-none = Pilih akun
rules-editor-accounts-many = { $count ->
   *[other] { $count } akun
}
rules-editor-matches = Cocok dengan { $mails } dari { $days } hari terakhir
rules-editor-mails = { $count ->
   *[other] { $count } email
}
rules-editor-counting = Menghitung email yang cocok…
rules-editor-show = Tampilkan
rules-editor-also-apply = Terapkan juga ke { $count } email ini
rules-editor-runs-katna = Berjalan di Katna, selama komputer ini menyala.
rules-editor-runs-gmail = Berjalan di Gmail, jadi juga berfungsi di ponsel Anda dan saat komputer ini mati.
rules-editor-runs-sieve = Berjalan di server email Anda, jadi juga berfungsi di ponsel Anda dan saat komputer ini mati.
rules-note-gmail-action = Berjalan di Katna: filter Gmail tidak dapat melakukan “{ $action }”.
rules-note-sieve-action = Berjalan di Katna: aturan server email Anda tidak dapat melakukan “{ $action }”.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Berjalan di Katna: filter Gmail tidak dapat memeriksa “{ $test }” seperti Katna.
rules-note-sieve-condition = Berjalan di Katna: aturan server email Anda tidak dapat memeriksa “{ $test }” seperti Katna.
rules-note-order = Berjalan di Katna, seperti aturan sebelumnya di akun ini: aturan dijalankan sesuai urutan daftar.
rules-note-gmail-stop = Berjalan di Katna: filter Gmail tidak dapat mencegah aturan berikutnya berjalan.
rules-note-gmail-forward = Berjalan di Katna: Gmail hanya meneruskan ke alamat yang diverifikasi di setelannya, dan { $address } bukan salah satunya.
rules-note-gmail-folder = Berjalan di Katna: Gmail tidak memiliki label untuk folder yang dipakai aturan ini.
rules-note-sieve-folder = Berjalan di Katna: server email Anda tidak memiliki folder yang dipakai aturan ini.
rules-note-gmail-sign-in = Berjalan di Katna sampai Anda masuk lagi ke Google dan mengizinkan Katna membuat filter Gmail.
rules-note-sieve-other-script = Berjalan di Katna: skrip aturan lain (“{ $name }”) aktif di server email Anda.
rules-note-gmail-failed = Berjalan di Katna: Gmail tidak menerimanya ({ $error }).
rules-note-sieve-failed = Berjalan di Katna: server email Anda tidak menerimanya ({ $error }).
rules-editor-cancel = Batal
rules-editor-save = Simpan
rules-editor-saving = Menyimpan…
rules-editor-delete = Hapus aturan
rules-editor-delete-ask = Hapus aturan ini?
rules-editor-delete-keep = Simpan saja
rules-editor-delete-confirm = Hapus
rules-editor-needs-folder = Pilih folder untuk setiap “Pindahkan ke” dan label untuk setiap “Tambahkan label”.
rules-editor-needs-days = “Tandai sudah dibaca setelah” memerlukan jumlah hari, dari 1 sampai 3650.
rules-saved = Aturan disimpan
rules-saved-applied = { $count ->
   *[other] Aturan disimpan dan diterapkan ke { $count } email
}
rules-apply-failed = Aturan disimpan, tetapi gagal diterapkan: { $error }
rules-deleted = Aturan dihapus
rules-delete-failed = Tidak dapat menghapus aturan: { $error }
rules-change-failed = Tidak dapat mengubah aturan: { $error }
