# Katna Mail, Malay (Bahasa Melayu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Cari fail

## Left side (and chips on a phone)

files-all = Semua fail
files-pictures = Gambar
files-pdfs = PDF
files-documents = Dokumen
files-sheets = Hamparan
files-slides = Slaid
files-other = Lain-lain
files-accounts = Akaun
files-drives = Pemacu
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Dikongsi dengan saya
files-shown = Ditunjukkan
files-received = Diterima
files-sent = Dihantar oleh saya

## Over the files

files-count = { $count ->
   *[other] { $count } fail · { $size }
}
files-anyone = Sesiapa sahaja
files-from-person = Daripada { $name }
files-time-any = Bila-bila masa
files-time-today = Hari ini
files-time-yesterday = Semalam
files-time-this-week = Minggu ini
files-time-last-week = Minggu lepas
files-time-this-month = Bulan ini
files-time-last-month = Bulan lepas
files-time-between = { $first } – { $last }
files-time-hint = Klik satu hari, atau seret merentasi beberapa hari
files-time-summary = { $count ->
   *[other] { $days } · { $count } fail
}
files-time-clear = Kosongkan
files-time-month-back = Bulan sebelumnya
files-time-month-on = Bulan seterusnya
files-time-wheel = Tatal untuk mengalihkan tarikh ini, dengan tempoh yang sama
files-sort-newest = Terbaharu dahulu
files-sort-oldest = Terlama dahulu
files-sort-largest = Terbesar dahulu
files-sort-name = Mengikut nama
files-grid = Kad
files-list = Senarai
files-this-week = Minggu ini
files-undated = Tiada tarikh
files-me = Saya
files-no-subject = (tiada subjek)
files-loading = Mengumpul fail daripada mel anda…
files-empty = Fail daripada mel anda dipaparkan di sini.
files-none-match = Tiada fail yang sepadan.
files-load-failed = Gagal membaca fail: { $error }

## A file's menu and buttons

files-open = Buka
files-open-with = Buka dengan…
files-save = Simpan…
files-show-mail = Tunjukkan mel
files-mail-window = Buka mel dalam tetingkap baharu
files-forward = Majukan fail
files-from-them = Fail daripada { $name }
files-copy-name = Salin nama fail
files-name-copied = Nama fail disalin
files-downloading = Memuat turun mel…
files-download-failed = Tidak dapat memuat turun mel ini.

## A cloud drive in place of the mail files

files-drive-mine = Drive Saya
files-drive-mine-onedrive = Fail saya
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
       *[other] { $files } fail
    }
   *[other] { $folders } folder · { $files ->
       *[other] { $files } fail
    }
}
files-drive-folders = Folder
files-drive-files = Fail
files-drive-folder = Folder
files-drive-meta = { $what } · Disunting { $date }
files-drive-as-link = { $what } · sebagai pautan
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = Mendapatkannya…
files-drive-loading = Membuka pemacu…
files-drive-empty = Folder ini kosong.
files-drive-unreachable = Tidak dapat mencapai { $drive }.
files-drive-try-again = Cuba lagi
files-drive-needs-permission = Katna memerlukan kebenaran anda sekali untuk menunjukkan pemacu ini. Log masuk semula dan benarkan Katna melihat fail anda.
files-drive-allow = Benarkan
files-drive-allow-failed = Log masuk tidak selesai, jadi pemacu kekal tertutup.
files-drive-attach = Lampirkan
files-drive-more = Lagi
files-drive-download = Muat turun…
files-drive-open-web = Buka dalam { $drive }
files-drive-copy-link = Salin pautan
files-drive-link-copied = Pautan disalin
files-drive-share = Kongsi…
files-drive-rename = Namakan semula
files-drive-trash = Alih ke tong sampah
files-drive-trashed = “{ $name }” berada dalam tong sampah { $drive }
files-drive-renamed = Dinamakan semula kepada “{ $name }”
files-drive-getting = Mendapatkan { $name } daripada { $drive }…
files-drive-get-failed = Tidak dapat mendapatkan { $name }: { $error }
files-drive-upload = Muat naik
files-drive-upload-files = Muat naik fail
files-drive-upload-folder = Muat naik folder
files-drive-upload-failed = Tidak dapat memuat naik { $name }: { $error }
files-drive-upload-needs = Untuk memuat naik, Katna memerlukan kebenaran anda sekali: tekan Benarkan dalam Tetapan › Apl lalai › Halaman Fail.

## The Share dialog of a drive file or folder

files-share-title = Kongsi “{ $name }”
files-share-add = Tambah orang mengikut nama atau alamat
files-share-not-address = “{ $text }” bukan alamat e-mel
files-share-notify = Benarkan { $drive } menghantar e-mel kepada mereka juga
files-share-people = Orang yang mempunyai akses
files-share-general = Akses umum
files-share-loading = Membaca siapa yang mempunyai akses…
files-share-restricted = Terhad
files-share-restricted-about = Hanya orang yang mempunyai akses boleh membukanya dengan pautan
files-share-anyone = Sesiapa yang mempunyai pautan
files-share-anyone-can = { $role ->
    [editor] Sesiapa yang mempunyai pautan boleh menyunting
    [commenter] Sesiapa yang mempunyai pautan boleh mengulas
   *[viewer] Sesiapa yang mempunyai pautan boleh melihat
}
files-share-anyone-about = { $role ->
    [editor] Sesiapa di internet yang mempunyai pautan boleh menyunting
    [commenter] Sesiapa di internet yang mempunyai pautan boleh mengulas
   *[viewer] Sesiapa di internet yang mempunyai pautan boleh melihat
}
files-share-role-owner = Pemilik
files-share-role-editor = Penyunting
files-share-role-commenter = Pengulas
files-share-role-viewer = Pelihat
files-share-you = { $name } (anda)
files-share-domain = Semua orang di { $domain }
files-share-inherited = Akses daripada folder yang mengandunginya
files-share-remove = Alih keluar akses
files-share-copy-link = Salin pautan
files-share-share = Kongsi
files-share-done = Selesai
files-share-close = Tutup
files-share-sharing = Berkongsi…
files-share-shared = { $count ->
   *[other] Dikongsi dengan { $count } orang
}
files-share-refused = { $drive } tidak dapat berkongsi dengan { $addresses }
files-share-failed = Tidak dapat menukar perkongsian: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
   *[other] Memuat naik { $count } item
}
files-tray-done = { $count ->
   *[other] { $count } muat naik selesai
}
files-tray-some-failed = { $done } dimuat naik, { $failed } gagal
files-tray-minutes-left = { $minutes ->
   *[other] Kira-kira { $minutes } minit lagi
}
files-tray-seconds-left = Kurang daripada seminit lagi
files-tray-starting = Bermula…
files-tray-cancel-all = Batalkan semua
files-tray-cancel = Batal
files-tray-fold = Sembunyikan senarai
files-tray-unfold = Tunjukkan senarai
files-tray-close = Tutup
files-tray-progress = { $place } · { $sent } daripada { $size }
files-tray-in = Dalam { $place }
files-tray-cancelled = Dibatalkan
