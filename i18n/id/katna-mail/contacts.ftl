# Katna Mail, Indonesian (Bahasa Indonesia): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Kontak
contacts-frequent = Sering
contacts-other = Kontak lainnya
contacts-other-about = Orang yang pernah Anda kirimi email dari Gmail tetapi belum disimpan
contacts-other-email = Kirim email
contacts-other-empty = Tidak ada kontak lainnya. Orang yang Anda kirimi email dari Gmail tetapi tidak Anda simpan akan muncul di sini.
contacts-other-allow = Untuk melihat kontak lainnya, masuk lagi ke akun Gmail Anda dan izinkan Katna melihatnya.
contacts-labels = Label
contacts-label-options = Opsi label
contacts-label-rename = Ganti nama label
contacts-label-email = Kirim email ke semua
contacts-label-delete = Hapus label
contacts-label-new = Label baru
contacts-label-name = Nama label
contacts-label-button = Label
contacts-label-menu = Beri label:
contacts-label-added = Ditambahkan ke { $name }
contacts-label-removed = Dihapus dari { $name }
contacts-label-renamed = Label diganti namanya menjadi { $name }
contacts-label-deleted = Label { $name } dihapus
contacts-label-no-email = Tidak ada orang di label ini yang memiliki alamat email
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = Akun
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = Masuk lagi untuk menampilkan kontak
contacts-account-signed-in = Sudah masuk lagi ke { $address }. Mengambil kontak Anda…
contacts-account-sign-in-refused = { $provider } tidak mengizinkan Katna masuk. Coba lagi, dan izinkan akses ke kontak Anda.
contacts-account-password = Server tidak menerima sandi. Yahoo, iCloud, Zoho, dan lainnya memerlukan sandi aplikasi.
contacts-account-change-password = Ubah sandi
contacts-account-change-password-tooltip = Buka Setelan > Akun
contacts-account-failed = Kontak tidak dapat dibaca.
# $reason is the server's own words, in English.
contacts-account-error = Kontak tidak dapat dibaca: { $reason }
contacts-account-none = Tidak ada buku alamat yang ditemukan
contacts-account-looking = Mencari kontak…
contacts-account-try-again = Coba lagi
contacts-account-try-again-tooltip = Periksa lagi kontak akun ini sekarang
contacts-account-fixing = Sedang dikerjakan…
contacts-manage = Perbaiki dan kelola
contacts-merge = Gabungkan dan perbaiki
contacts-merge-about = { $count ->
   *[other] { $count } saran: kontak yang tampak seperti orang yang sama
}
contacts-merge-none = Tidak ada duplikat. Kontak dengan nama atau nomor telepon yang sama akan muncul di sini.
contacts-merge-count = { $count ->
   *[other] { $count } kontak
}
contacts-merge-all = Gabungkan semua
contacts-merge-button = Gabungkan
contacts-merge-dismiss = Tutup
contacts-merged = { $count ->
    [1] Kontak digabungkan
   *[other] { $count } penggabungan selesai
}
contacts-import = Impor
contacts-export = Ekspor
contacts-import-file = Impor kontak dari file vCard atau CSV
contacts-imported = { $count ->
   *[other] { $count } kontak diimpor ke { $place }
}
contacts-imported-some = { $count ->
   *[other] { $count } kontak diimpor ke { $place }; { $skipped } sudah tersimpan, dilewati
}
contacts-import-none = Tidak ada kontak yang ditemukan di { $name }
contacts-import-all-saved = Semua orang di { $name } sudah tersimpan
contacts-import-failed = Tidak dapat membaca { $name }: { $error }
contacts-exported = { $count ->
   *[other] { $count } kontak diekspor ke { $path }
}
contacts-export-none = Tidak ada kontak untuk diekspor
contacts-export-failed = Tidak dapat mengekspor kontak: { $error }
contacts-print = Cetak
contacts-print-title = Kontak
contacts-print-none = Tidak ada kontak untuk dicetak
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = Ulang tahun: { $day }
contacts-print-nickname = Nama panggilan: { $name }
contacts-create = Buat kontak

## Search and the list

contacts-search = Telusuri kontak
contacts-loading = Memuat kontak…
contacts-empty = Belum ada kontak tersimpan. Kontak yang Anda simpan di Gmail, Outlook, atau layanan email Anda muncul di sini.
contacts-empty-no-books = Kontak dari akun Anda muncul di sini setelah disinkronkan.
contacts-none-found = Tidak ada kontak yang cocok dengan pencarian Anda.
contacts-starred = { $count ->
   *[other] Kontak berbintang ({ $count })
}
contacts-count = Kontak ({ $count })
contacts-col-name = Nama
contacts-col-email = Email
contacts-col-phone = Nomor telepon
contacts-col-job = Jabatan dan perusahaan
contacts-col-labels = Label

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Izinkan Katna membaca kontak dari { $address }.
contacts-allow-many = { $more ->
   *[other] Izinkan Katna membaca kontak dari { $address } dan { $more } akun lainnya.
}
contacts-allow-button = Izinkan

## A contact's page

contacts-back = Kembali ke kontak
contacts-edit = Edit
contacts-delete = Hapus
contacts-qr = Bagikan sebagai kode QR
contacts-qr-about = Pindai ini dengan kamera ponsel untuk menyimpan kontak.
contacts-qr-too-long = Kontak ini memiliki terlalu banyak detail untuk dimuat dalam kode QR.
contacts-qr-done = Selesai
contacts-deleted = { $name } dihapus
contacts-added = { $name } ditambahkan ke kontak
contacts-find-mail = Email
contacts-details = Detail kontak
contacts-saved-in = Disimpan di
contacts-notes = Catatan
contacts-birthday = Ulang tahun
contacts-nickname = Nama panggilan
contacts-this-computer = Komputer ini
contacts-kind-home = Rumah
contacts-kind-work = Kantor
contacts-kind-mobile = Seluler
contacts-kind-other = Lainnya
contacts-source-google = Google Kontak
contacts-source-microsoft = Kontak Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Buat kontak
contacts-edit-title = Edit kontak
contacts-edit-save = Simpan
contacts-edit-saving = Menyimpan…
contacts-edit-cancel = Batal
contacts-saved = Kontak disimpan
contacts-edit-save-to = Simpan ke
contacts-edit-changes-go-to = Perubahan disimpan ke { $place }.
contacts-edit-given = Nama depan
contacts-edit-family = Nama belakang
contacts-edit-company = Perusahaan
contacts-edit-job = Jabatan
contacts-edit-email = Email
contacts-edit-phone = Telepon
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = Tambahkan email
contacts-edit-add-phone = Tambahkan telepon
contacts-edit-street = Alamat jalan
contacts-edit-city = Kota
contacts-edit-postcode = Kode pos
contacts-edit-country = Negara
contacts-edit-birthday = Ulang tahun (YYYY-MM-DD)
contacts-edit-empty = Tambahkan nama, email, atau nomor telepon terlebih dahulu.
