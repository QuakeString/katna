# Katna Mail, Indonesian (Bahasa Indonesia).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Telusuri file

## Left side (and chips on a phone)

files-all = Semua file
files-pictures = Gambar
files-pdfs = PDF
files-documents = Dokumen
files-sheets = Spreadsheet
files-slides = Slide
files-other = Lainnya
files-accounts = Akun
files-drives = Drive
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Dibagikan kepada saya
files-shown = Ditampilkan
files-received = Diterima
files-sent = Dikirim oleh saya

## Over the files

files-count = { $count ->
   *[other] { $count } file · { $size }
}
files-anyone = Siapa saja
files-from-person = Dari { $name }
files-time-any = Kapan saja
files-time-today = Hari ini
files-time-yesterday = Kemarin
files-time-this-week = Minggu ini
files-time-last-week = Minggu lalu
files-time-this-month = Bulan ini
files-time-last-month = Bulan lalu
files-time-between = { $first } – { $last }
files-time-hint = Klik satu hari, atau seret melintasi beberapa hari
files-time-summary = { $count ->
   *[other] { $days } · { $count } file
}
files-time-clear = Hapus
files-time-month-back = Bulan sebelumnya
files-time-month-on = Bulan berikutnya
files-time-wheel = Gulir untuk menggeser tanggal ini, dengan panjang yang sama
files-sort-newest = Terbaru dulu
files-sort-oldest = Terlama dulu
files-sort-largest = Terbesar dulu
files-sort-name = Menurut nama
files-grid = Kartu
files-list = Daftar
files-this-week = Minggu ini
files-undated = Tanpa tanggal
files-me = Saya
files-no-subject = (tanpa subjek)
files-loading = Mengumpulkan file dari email Anda…
files-empty = File dari email Anda akan muncul di sini.
files-none-match = Tidak ada file yang cocok.
files-load-failed = Gagal membaca file: { $error }

## A file's menu and buttons

files-open = Buka
files-open-with = Buka dengan…
files-save = Simpan…
files-show-mail = Tampilkan email
files-mail-window = Buka email di jendela baru
files-forward = Teruskan file
files-from-them = File dari { $name }
files-copy-name = Salin nama file
files-name-copied = Nama file disalin
files-downloading = Mendownload email…
files-download-failed = Tidak dapat mendownload email ini.

## A cloud drive in place of the mail files

files-drive-mine = Drive Saya
files-drive-mine-onedrive = File saya
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
       *[other] { $files } file
    }
   *[other] { $folders } folder · { $files ->
       *[other] { $files } file
    }
}
files-drive-folders = Folder
files-drive-files = File
files-drive-folder = Folder
files-drive-meta = { $what } · Diedit { $date }
files-drive-as-link = { $what } · sebagai link
files-drive-google-doc = Google Dokumen
files-drive-google-sheet = Google Spreadsheet
files-drive-google-slides = Google Slide
files-drive-google-drawing = Google Gambar
files-drive-fetching = Mengambil…
files-drive-loading = Membuka drive…
files-drive-empty = Folder ini kosong.
files-drive-unreachable = Tidak dapat menjangkau { $drive }.
files-drive-try-again = Coba lagi
files-drive-needs-permission = Katna perlu izin Anda sekali untuk menampilkan drive ini. Masuk lagi dan izinkan Katna melihat file Anda.
files-drive-allow = Izinkan
files-drive-allow-failed = Proses masuk tidak selesai, jadi drive tetap tertutup.
files-drive-attach = Lampirkan
files-drive-more = Lainnya
files-drive-download = Download…
files-drive-open-web = Buka di { $drive }
files-drive-copy-link = Salin link
files-drive-link-copied = Link disalin
files-drive-share = Bagikan…
files-drive-rename = Ganti nama
files-drive-trash = Pindahkan ke sampah
files-drive-trashed = “{ $name }” ada di sampah { $drive }
files-drive-renamed = Nama diubah menjadi “{ $name }”
files-drive-getting = Mengambil { $name } dari { $drive }…
files-drive-get-failed = Tidak dapat mengambil { $name }: { $error }
files-drive-upload = Upload
files-drive-upload-files = Upload file
files-drive-upload-folder = Upload folder
files-drive-upload-failed = Tidak dapat mengupload { $name }: { $error }
files-drive-upload-needs = Untuk mengupload, Katna perlu izin Anda sekali: tekan Izinkan di Setelan › Aplikasi default › Halaman File.

## The Share dialog of a drive file or folder

files-share-title = Bagikan “{ $name }”
files-share-add = Tambahkan orang berdasarkan nama atau alamat
files-share-not-address = “{ $text }” bukan alamat email
files-share-notify = Biarkan { $drive } juga mengirimi mereka email
files-share-people = Orang yang memiliki akses
files-share-general = Akses umum
files-share-loading = Membaca siapa yang memiliki akses…
files-share-restricted = Dibatasi
files-share-restricted-about = Hanya orang yang memiliki akses yang dapat membukanya dengan link
files-share-anyone = Siapa saja yang memiliki link
files-share-anyone-can = { $role ->
    [editor] Siapa saja yang memiliki link dapat mengedit
    [commenter] Siapa saja yang memiliki link dapat berkomentar
   *[viewer] Siapa saja yang memiliki link dapat melihat
}
files-share-anyone-about = { $role ->
    [editor] Siapa saja di internet yang memiliki link dapat mengedit
    [commenter] Siapa saja di internet yang memiliki link dapat berkomentar
   *[viewer] Siapa saja di internet yang memiliki link dapat melihat
}
files-share-role-owner = Pemilik
files-share-role-editor = Editor
files-share-role-commenter = Pemberi komentar
files-share-role-viewer = Pelihat
files-share-you = { $name } (Anda)
files-share-domain = Semua orang di { $domain }
files-share-inherited = Akses dari folder tempatnya berada
files-share-remove = Hapus akses
files-share-copy-link = Salin link
files-share-share = Bagikan
files-share-done = Selesai
files-share-close = Tutup
files-share-sharing = Membagikan…
files-share-shared = { $count ->
   *[other] Dibagikan kepada { $count } orang
}
files-share-refused = { $drive } tidak dapat berbagi dengan { $addresses }
files-share-failed = Tidak dapat mengubah setelan berbagi: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
   *[other] Mengupload { $count } item
}
files-tray-done = { $count ->
   *[other] { $count } upload selesai
}
files-tray-some-failed = { $done } diupload, { $failed } gagal
files-tray-minutes-left = { $minutes ->
   *[other] Sekitar { $minutes } menit lagi
}
files-tray-seconds-left = Kurang dari satu menit lagi
files-tray-starting = Memulai…
files-tray-cancel-all = Batalkan semua
files-tray-cancel = Batal
files-tray-fold = Sembunyikan daftar
files-tray-unfold = Tampilkan daftar
files-tray-close = Tutup
files-tray-progress = { $place } · { $sent } dari { $size }
files-tray-in = Di { $place }
files-tray-cancelled = Dibatalkan
