# Katna Mail, Malay (Bahasa Melayu): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Kenalan
contacts-frequent = Kerap
contacts-other = Kenalan lain
contacts-other-about = Orang yang anda e-melkan daripada Gmail tetapi belum disimpan
contacts-other-email = Hantar e-mel
contacts-other-empty = Tiada kenalan lain. Orang yang anda e-melkan daripada Gmail tetapi tidak disimpan akan muncul di sini.
contacts-other-allow = Untuk melihat kenalan lain, log masuk semula ke akaun Gmail anda dan benarkan Katna melihatnya.
contacts-labels = Label
contacts-label-options = Pilihan label
contacts-label-rename = Namakan semula label
contacts-label-email = E-mel semua orang
contacts-label-delete = Padam label
contacts-label-new = Label baharu
contacts-label-name = Nama label
contacts-label-button = Label
contacts-label-menu = Labelkan sebagai:
contacts-label-added = Ditambahkan pada { $name }
contacts-label-removed = Dialih keluar daripada { $name }
contacts-label-renamed = Label dinamakan semula kepada { $name }
contacts-label-deleted = Label { $name } dipadam
contacts-label-no-email = Tiada sesiapa pada label ini yang mempunyai alamat e-mel
contacts-manage = Betulkan dan urus
contacts-merge = Gabung dan betulkan
contacts-merge-about = { $count ->
   *[other] { $count } cadangan: kenalan yang kelihatan seperti orang yang sama
}
contacts-merge-none = Tiada pendua. Kenalan dengan nama atau nombor telefon yang sama akan muncul di sini.
contacts-merge-count = { $count ->
   *[other] { $count } kenalan
}
contacts-merge-all = Gabungkan semua
contacts-merge-button = Gabung
contacts-merge-dismiss = Ketepikan
contacts-merged = { $count ->
    [1] Kenalan digabungkan
   *[other] { $count } penggabungan selesai
}
contacts-import = Import
contacts-export = Eksport
contacts-import-title = Import kenalan daripada fail vCard
contacts-imported = { $count ->
   *[other] { $count } kenalan diimport ke { $place }
}
contacts-imported-some = { $count ->
   *[other] { $count } kenalan diimport ke { $place }; { $skipped } yang sudah disimpan ditinggalkan
}
contacts-import-none = Tiada kenalan ditemui dalam { $name }
contacts-import-all-saved = Semua orang dalam { $name } sudah disimpan
contacts-import-failed = Tidak dapat membaca { $name }: { $error }
contacts-exported = { $count ->
   *[other] { $count } kenalan dieksport ke { $path }
}
contacts-export-none = Tiada kenalan untuk dieksport
contacts-export-failed = Tidak dapat mengeksport kenalan: { $error }
contacts-create = Cipta kenalan

## Search and the list

contacts-search = Cari kenalan
contacts-loading = Memuatkan kenalan…
contacts-empty = Belum ada kenalan disimpan. Kenalan yang anda simpan dalam Gmail, Outlook atau perkhidmatan mel anda muncul di sini.
contacts-empty-no-books = Kenalan daripada akaun anda muncul di sini selepas disegerakkan.
contacts-none-found = Tiada kenalan sepadan dengan carian anda.
contacts-starred = { $count ->
   *[other] Kenalan berbintang ({ $count })
}
contacts-count = Kenalan ({ $count })
contacts-col-name = Nama
contacts-col-email = E-mel
contacts-col-phone = Nombor telefon
contacts-col-job = Jawatan dan syarikat
contacts-col-labels = Label

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Benarkan Katna membaca kenalan { $address }.
contacts-allow-many = { $more ->
   *[other] Benarkan Katna membaca kenalan { $address } dan { $more } akaun lagi.
}
contacts-allow-button = Benarkan

## A contact's page

contacts-back = Kembali ke kenalan
contacts-edit = Edit
contacts-delete = Padam
contacts-deleted = { $name } dipadam
contacts-added = { $name } ditambahkan pada kenalan
contacts-find-mail = Mel
contacts-details = Butiran kenalan
contacts-saved-in = Disimpan dalam
contacts-notes = Nota
contacts-birthday = Hari lahir
contacts-nickname = Nama panggilan
contacts-this-computer = Komputer ini
contacts-kind-home = Rumah
contacts-kind-work = Kerja
contacts-kind-mobile = Mudah alih
contacts-kind-other = Lain-lain
contacts-source-google = Google Kenalan
contacts-source-microsoft = Kenalan Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Cipta kenalan
contacts-edit-title = Edit kenalan
contacts-edit-save = Simpan
contacts-edit-saving = Menyimpan…
contacts-edit-cancel = Batal
contacts-saved = Kenalan disimpan
contacts-edit-save-to = Simpan ke
contacts-edit-changes-go-to = Perubahan disimpan ke { $place }.
contacts-edit-given = Nama pertama
contacts-edit-family = Nama akhir
contacts-edit-company = Syarikat
contacts-edit-job = Jawatan
contacts-edit-email = E-mel
contacts-edit-phone = Telefon
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = Tambah e-mel
contacts-edit-add-phone = Tambah telefon
contacts-edit-street = Alamat jalan
contacts-edit-city = Bandar
contacts-edit-postcode = Poskod
contacts-edit-country = Negara
contacts-edit-birthday = Hari lahir (YYYY-MM-DD)
contacts-edit-empty = Tambah nama, e-mel atau nombor telefon dahulu.
