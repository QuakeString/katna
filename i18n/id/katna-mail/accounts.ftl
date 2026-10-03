# Katna Mail, Indonesian (Bahasa Indonesia).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = Panel folder
accounts-folder-pane-detail = Folder akun mana yang ditampilkan di panel kiri.
accounts-shown-one = Satu akun dalam satu waktu; beralih di kartu akun
accounts-shown-all = Semua akun, satu per satu berurutan
accounts-unified = Kotak masuk gabungan
accounts-unified-switch = Tampilkan email semua akun sekaligus
accounts-unified-switch-detail = “Semua Akun” berada di bagian atas panel folder, dengan kotak masuk, email terkirim, dan lainnya dari setiap akun dalam satu daftar. Akun di bawahnya awalnya diciutkan.
accounts-row = Akun
accounts-row-detail = Panel folder dan menu akun menampilkan akun dengan urutan ini; yang pertama menjadi default. Menghapus akun akan menghapus salinan email akun tersebut milik Katna di komputer ini. Email tetap ada di server.
accounts-none = Belum ada akun.
accounts-pop3-row = Email di server
accounts-pop3-row-detail = Akun POP3 mendownload email ke komputer ini. Pilih apa yang terjadi selanjutnya pada salinan di server.
accounts-pop3-with-katna = Simpan sampai saya menghapusnya di Katna
accounts-pop3-at-once = Hapus setelah didownload
accounts-pop3-after-days = { $count ->
   *[other] Hapus setelah { $count } hari
}
accounts-pop3-never = Jangan pernah hapus
accounts-pop3-days-less = Lebih sedikit hari
accounts-pop3-days-more = Lebih banyak hari
accounts-kind-imported = Diimpor
accounts-picture-reset = Gunakan gambar desktop
accounts-picture-change = Ubah gambar
accounts-picture-remove = Hapus gambar
accounts-rename = Ganti nama
accounts-name-save = Simpan
accounts-name-cancel = Batal
accounts-name-placeholder = Nama Anda
accounts-rename-failed = Tidak dapat mengganti nama akun: { $error }
accounts-move-up = Pindahkan ke atas
accounts-move-down = Pindahkan ke bawah
accounts-drag = Seret untuk mengubah urutan
accounts-remove = Hapus
accounts-delete-all-row = Hapus semua data
accounts-delete-all-row-detail = Mulai dari awal, seperti pada instalasi baru.
accounts-delete-all-about = Menghapus setiap akun, semua email, kontak, dan kalender yang tersimpan, indeks penelusuran, setelan Anda, dan sandi yang tersimpan dari komputer ini. Tidak ada yang berubah di server email Anda.
accounts-delete-all-open = Hapus semua data Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } telah dihapus dari Katna.
accounts-removed = { $address } telah dihapus dari Katna. Emailnya masih ada di server.
accounts-all-deleted = Semua data Katna telah dihapus dari komputer ini.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Hapus { $address }?
accounts-remove-confirm = Hapus akun
accounts-removing = Menghapus…
accounts-remove-local-mail = { $folders ->
    [0] Semua email yang diimpor ke akun ini
   *[other] Semua email yang diimpor ke akun ini, di { $folders } foldernya
}
accounts-remove-local-settings = Setelan Katna akun ini
accounts-remove-mail = { $folders ->
    [0] Semua email akun ini yang disimpan oleh Katna
   *[other] Semua email akun ini yang disimpan oleh Katna, di { $folders } foldernya
}
accounts-remove-outbox = Pesan akun ini yang menunggu di kotak keluar
accounts-remove-settings = Sandi tersimpan dan setelan Katna akun ini
accounts-delete-all-title = Hapus semua data Katna?
accounts-delete-all-confirm = Hapus semuanya
accounts-deleting = Menghapus…
accounts-delete-all-accounts = Setiap akun, serta semua email dan lampiran yang disimpan oleh Katna
accounts-delete-all-contacts = Kontak, kalender, dan indeks penelusuran
accounts-delete-all-settings = Semua setelan, tanda tangan, dan pintasan keyboard
accounts-delete-all-passwords = Setiap sandi yang tersimpan
accounts-deleted-heading = Dihapus dari komputer ini:
accounts-cannot-undo = Tindakan ini tidak dapat diurungkan.
accounts-server-delete-all = Tidak ada yang berubah di server email Anda: email Anda tetap di sana, dan menambahkan akun lagi akan mendownloadnya kembali. Email yang diimpor dari file hanya ada di Katna; file-filenya tidak diubah.
accounts-server-local = Email ini diimpor dari file, jadi Katna memiliki satu-satunya salinan. File asalnya tidak diubah; impor lagi untuk mendapatkannya kembali.
accounts-server-remove = Tidak ada yang berubah di server email: email Anda tetap di sana, dan menambahkan akun lagi akan mendownloadnya kembali.
accounts-confirm-word = hapus
accounts-confirm-placeholder = Ketik “{ accounts-confirm-word }”
accounts-confirm-prompt = Untuk mengonfirmasi, ketik “{ accounts-confirm-word }”:
accounts-cancel = Batal

## Reset cache (Settings > General), in the same dialog

reset-cache-about = Menghapus email dan lampiran yang didownload Katna, gambar pengirim, dan indeks penelusuran, lalu mendownload ulang email terbaru. Akun, setelan, dan email yang hanya ada di komputer ini tetap disimpan.
reset-cache-button = Reset cache
reset-cache-title = Reset cache?
reset-cache-deleted = Dihapus, lalu didownload ulang:
reset-cache-mail = Email dan lampiran yang didownload dari server IMAP Anda: email terbaru didownload ulang sekarang, email lama saat Anda membukanya
reset-cache-index = Indeks penelusuran, yang langsung dibangun ulang
reset-cache-pictures = Gambar pengirim
reset-cache-kept = Tetap disimpan: akun, sandi, dan setelan Anda; bintang, label, tanda sudah dibaca, dan sematan; draf, kotak keluar, dan perubahan yang belum sampai ke server; serta email dari akun POP3 atau file yang diimpor, yang mungkin tidak punya salinan lain. Tidak ada yang berubah di server email Anda.
reset-cache-confirm = Reset cache
reset-cache-busy = Mereset…
reset-cache-done = Cache telah direset. Email terbaru sedang didownload ulang.
reset-cache-done-freed = Cache telah direset dan { $size } dibebaskan. Email terbaru sedang didownload ulang.
