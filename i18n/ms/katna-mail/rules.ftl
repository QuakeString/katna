# Katna Mail, Malay (Bahasa Melayu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Peraturan
settings-rules-summary = Isih, labelkan, majukan atau senyapkan mel baharu dengan sendirinya
settings-rules-intro = Peraturan mengisih mel baharu dengan sendirinya, mengikut susunan ini. Seret untuk menyusun semula.
settings-rules-all-accounts = Semua akaun
settings-rules-new = Peraturan baharu
settings-rules-none = Belum ada peraturan. Peraturan mengisih mel baharu dengan sendirinya: mengikut pengirim, subjek atau perkataan.
settings-rules-none-account = Belum ada peraturan untuk akaun ini.
settings-rules-drag = Seret untuk menyusun semula
settings-rules-edit = Sunting peraturan
settings-rules-turn-off = Matikan peraturan ini
settings-rules-turn-on = Hidupkan peraturan ini

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = Peraturan permulaan
settings-rules-starters-intro = Mati sehingga anda menghidupkannya. Peraturan ini berfungsi untuk semua akaun anda; sunting satu untuk mengubahnya.
settings-rules-starter-turning-on = Menghidupkan “{ $name }”…
settings-rules-starter-failed = Tidak dapat menghidupkan “{ $name }”: { $error }
rules-starter-promotions = Senyapkan promosi
rules-starter-newsletters = Surat berita ke Bacaan
rules-starter-receipts = Resit dan invois
rules-starter-deliveries = Penghantaran barang
rules-starter-train = Tiket kereta api
rules-starter-flight = Tiket penerbangan
rules-starter-codes = Kod sekali guna
rules-starter-security = Amaran keselamatan
rules-starter-social = Mel sosial
rules-starter-invites = Jemputan kalendar
rules-starter-folder-reading = Bacaan
rules-starter-folder-receipts = Resit
rules-starter-folder-deliveries = Penghantaran barang
rules-starter-folder-travel = Perjalanan
rules-starter-folder-social = Sosial
rules-runs-katna = Berjalan dalam Katna
rules-runs-gmail = Berjalan pada Gmail
rules-runs-sieve = Berjalan pada pelayan
rules-stopped = Dihentikan
rules-error-folder-gone = Folder yang digunakan oleh peraturan ini sudah tiada. Sunting peraturan untuk memilih folder lain.
rules-error-no-archive = Akaun ini tiada folder arkib. Sunting peraturan untuk melakukan perkara lain.
rules-error-no-trash = Akaun ini tiada folder Sampah. Sunting peraturan untuk melakukan perkara lain.
rules-error-cannot-send = Akaun ini tidak boleh menghantar mel, jadi peraturan tidak dapat memajukannya.
rules-error-other = { $error }. Sunting peraturan dan hidupkannya semula.
settings-folders = Folder
settings-folders-summary = Kiraan belum dibaca dalam anak tetingkap folder
settings-folders-unread-counts = Kiraan belum dibaca pada setiap folder
settings-folders-unread-counts-detail = Mati: hanya Peti Masuk menunjukkan bilangan yang belum dibaca

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } dan { $next }
rules-summary-or = { $first } atau { $next }
rules-summary-more = { $count } lagi
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Ada lampiran
rules-summary-no-attachment = Tiada lampiran
rules-summary-mailing-list = Daripada senarai mel
rules-summary-not-mailing-list = Bukan daripada senarai mel
rules-summary-tab = Dalam tab { $tab }
rules-summary-not-tab = Bukan dalam tab { $tab }
rules-summary-move = alih ke { $folder }
rules-summary-archive = langkau peti masuk
rules-summary-trash = alih ke sampah
rules-summary-mark-read = tandai dibaca
rules-summary-star = bintangkan
rules-summary-important = tandai penting
rules-summary-label = labelkan { $label }
rules-summary-forward = majukan kepada { $address }
rules-summary-dont-notify = jangan beritahu
rules-summary-read-after = { $count ->
   *[other] tandai dibaca selepas { $count } hari
}
rules-summary-folder-gone = folder yang sudah tiada

## The rule editor

rules-editor-new-title = Peraturan baharu
rules-editor-edit-title = Sunting peraturan
rules-editor-name-hint = Nama peraturan
rules-editor-when = Apabila mel baharu sepadan dengan
rules-editor-of-these = syarat ini:
rules-mode-all = semua
rules-mode-any = mana-mana
rules-field-from = Daripada
rules-field-to = Kepada
rules-field-cc = Sk
rules-field-any-recipient = Kepada atau Sk
rules-field-reply-to = Balas-kepada
rules-field-subject = Subjek
rules-field-body = Teks
rules-field-attachment-name = Nama lampiran
rules-field-has-attachment = Ada lampiran
rules-field-mailing-list = Daripada senarai mel
rules-field-tab = Tab peti masuk
rules-comparator-contains = mengandungi
rules-comparator-not-contains = tidak mengandungi
rules-comparator-begins-with = bermula dengan
rules-comparator-ends-with = berakhir dengan
rules-comparator-equals = tepat sama dengan
rules-comparator-matches = sepadan dengan corak
rules-has-yes = ya
rules-has-no = tidak
rules-editor-value-hint = Perkataan atau alamat
rules-editor-add-condition = Tambah syarat
rules-editor-remove = Alih keluar
rules-editor-then = Kemudian:
rules-action-move = Alih ke
rules-action-archive = Langkau peti masuk (arkibkan)
rules-action-trash = Alih ke sampah
rules-action-mark-read = Tandai dibaca
rules-action-star = Bintangkan
rules-action-important = Tandai penting
rules-action-label = Tambah label
rules-action-forward = Majukan kepada
rules-action-dont-notify = Jangan beritahu
rules-action-read-after = Tandai dibaca selepas
rules-editor-choose-folder = Pilih folder
rules-editor-choose-label = Pilih label
rules-editor-new-folder = Baharu: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = Alamat e-mel
rules-editor-days = hari
rules-editor-add-action = Tambah tindakan
rules-editor-stop = Berhenti di sini: peraturan seterusnya tidak dijalankan pada mel ini
rules-editor-accounts = Akaun:
rules-editor-accounts-none = Pilih akaun
rules-editor-accounts-many = { $count ->
   *[other] { $count } akaun
}
rules-editor-matches = Sepadan dengan { $mails } dalam { $days } hari lepas
rules-editor-mails = { $count ->
   *[other] { $count } mel
}
rules-editor-counting = Mengira mel yang sepadan…
rules-editor-show = Tunjukkan
rules-editor-also-apply = Gunakan juga pada { $count } mel ini
rules-editor-runs-katna = Berjalan dalam Katna, semasa komputer ini hidup.
rules-editor-runs-gmail = Berjalan pada Gmail, jadi ia juga berfungsi pada telefon anda dan semasa komputer ini dimatikan.
rules-editor-runs-sieve = Berjalan pada pelayan mel anda, jadi ia juga berfungsi pada telefon anda dan semasa komputer ini dimatikan.
rules-note-gmail-action = Berjalan dalam Katna: penapis Gmail tidak boleh melakukan “{ $action }”.
rules-note-sieve-action = Berjalan dalam Katna: peraturan pelayan mel anda tidak boleh melakukan “{ $action }”.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Berjalan dalam Katna: penapis Gmail tidak boleh menguji “{ $test }” seperti yang dilakukan oleh Katna.
rules-note-sieve-condition = Berjalan dalam Katna: peraturan pelayan mel anda tidak boleh menguji “{ $test }” seperti yang dilakukan oleh Katna.
rules-note-order = Berjalan dalam Katna, seperti peraturan akaun yang lebih awal: peraturan berjalan mengikut susunan senarai.
rules-note-gmail-stop = Berjalan dalam Katna: penapis Gmail tidak boleh menghalang peraturan seterusnya daripada berjalan.
rules-note-gmail-forward = Berjalan dalam Katna: Gmail hanya memajukan kepada alamat yang disahkan dalam tetapannya, dan { $address } bukan salah satu daripadanya.
rules-note-gmail-folder = Berjalan dalam Katna: Gmail tiada label untuk folder yang digunakan oleh peraturan ini.
rules-note-sieve-folder = Berjalan dalam Katna: pelayan mel anda tiada folder yang digunakan oleh peraturan ini.
rules-note-gmail-sign-in = Berjalan dalam Katna sehingga anda log masuk ke Google semula dan membenarkan Katna membuat penapis Gmail.
rules-note-sieve-other-script = Berjalan dalam Katna: skrip peraturan lain (“{ $name }”) aktif pada pelayan mel anda.
rules-note-gmail-failed = Berjalan dalam Katna: Gmail tidak menerimanya ({ $error }).
rules-note-sieve-failed = Berjalan dalam Katna: pelayan mel anda tidak menerimanya ({ $error }).
rules-editor-cancel = Batal
rules-editor-save = Simpan
rules-editor-saving = Menyimpan…
rules-editor-delete = Padam peraturan
rules-editor-delete-ask = Padam peraturan ini?
rules-editor-delete-keep = Kekalkan
rules-editor-delete-confirm = Padam
rules-editor-needs-folder = Pilih folder untuk setiap “Alih ke” dan label untuk setiap “Tambah label”.
rules-editor-needs-days = “Tandai dibaca selepas” memerlukan bilangan hari, dari 1 hingga 3650.
rules-saved = Peraturan disimpan
rules-saved-applied = { $count ->
   *[other] Peraturan disimpan dan digunakan pada { $count } mel
}
rules-apply-failed = Peraturan disimpan, tetapi gagal menggunakannya: { $error }
rules-deleted = Peraturan dipadam
rules-delete-failed = Tidak dapat memadam peraturan: { $error }
rules-change-failed = Tidak dapat mengubah peraturan: { $error }
