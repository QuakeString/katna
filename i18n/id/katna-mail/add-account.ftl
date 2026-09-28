# Katna Mail, Indonesian (Bahasa Indonesia).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Tambahkan akun email
add-account-looking = Mencari server email untuk { $address }…
add-account-address-intro = Masukkan alamat email Anda. Katna akan menemukan servernya untuk Anda.
add-account-servers-title = Setelan server
add-account-servers-intro = Tempat Katna membaca dan mengirim email untuk { $address }.
add-account-password-title = Masukkan sandi Anda
add-account-signing-in = Masuk…
add-account-browser-title = Lanjutkan di browser Anda
add-account-browser-intro = Katna membuka halaman masuk { $provider } di browser Anda. Masuklah di sana dan izinkan Katna membaca dan mengirim email Anda, lalu kembali ke sini.
add-account-browser-hint = Tidak ada halaman yang terbuka? Periksa jendela browser Anda, atau kembali dan coba lagi.

## Add a mail account: fields

add-account-field-address = Alamat email
add-account-incoming = Email masuk ({ $protocol })
add-account-outgoing = Email keluar ({ $protocol })
add-account-field-server = Server
add-account-field-port = Port
add-account-security-none = Tidak ada
add-account-field-username = Nama pengguna
add-account-field-password = Sandi
add-account-show-password = Tampilkan sandi
add-account-app-password-hint = { $provider } memerlukan sandi aplikasi di sini, bukan sandi yang Anda gunakan di web. Buat sandi aplikasi di setelan keamanan akun { $provider } Anda.
add-account-field-name = Nama Anda (opsional)
add-account-name-hint = Ditampilkan kepada orang yang Anda kirimi email.
add-account-servers-pair = { $imap } dan { $smtp }
add-account-servers-found = { $source ->
    [built-in] Server: { $servers }, ditemukan di daftar penyedia Katna.
    [provider] Server: { $servers }, ditemukan di setelan penyedia Anda.
    [ispdb] Server: { $servers }, ditemukan di daftar penyedia Thunderbird.
    [dns] Server: { $servers }, ditemukan di data DNS domain Anda.
   *[other] Server: { $servers }, hasil tebakan; periksa jika gagal masuk.
}
add-account-servers-entered = Server: { $servers }, sesuai yang dimasukkan.
add-account-or = atau
add-account-sign-in-with = Masuk dengan { $provider }
add-account-sign-in-instead = Masuk dengan { $provider } saja

## Add a mail account: buttons

add-account-servers-button = Setelan server
add-account-back = Kembali
add-account-add = Tambahkan akun
add-account-next = Berikutnya
add-account-cancel = Batal

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Masukkan server email masuk.
   *[outgoing] Masukkan server email keluar.
}
add-account-server-space = { $kind ->
    [incoming] Nama server email masuk mengandung spasi.
   *[outgoing] Nama server email keluar mengandung spasi.
}
add-account-port-invalid = { $kind ->
    [incoming] Port email masuk harus berupa angka dari { $min } hingga { $max }.
   *[outgoing] Port email keluar harus berupa angka dari { $min } hingga { $max }.
}
add-account-address-empty = Masukkan alamat email.
add-account-address-invalid = Masukkan alamat email seperti { $example }.
add-account-not-found = Katna tidak dapat menemukan server untuk { $address }, jadi Katna mengisi nama yang umum. Periksa dengan penyedia Anda.
add-account-password-empty = Masukkan sandi.
add-account-name-is-password = Nama sama dengan sandi. Ketik nama Anda di sana, seperti yang akan dilihat orang lain.
add-account-added = { $address } ditambahkan. Mengambil email Anda…
add-account-app-password-refused = { $provider } menolak sandi. Diperlukan sandi aplikasi, bukan sandi yang Anda gunakan di web.
add-account-password-refused = Server menolak sandi. Periksa sandi, lalu coba lagi.
add-account-sign-in-refused = { $provider } tidak mengizinkan Katna masuk. Coba lagi, dan izinkan akses ke email Anda.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Salinan Katna ini belum dapat masuk ke akun Microsoft.
    [Google] Salinan Katna ini belum dapat masuk ke akun Google.
   *[other] Penyedia ini hanya mengizinkan masuk di halamannya sendiri, yang belum dapat dilakukan Katna untuknya.
}
add-account-signed-in = Sudah masuk dengan { $provider }. Mengambil email Anda…

## The account menu (from the account button on the top bar)

add-account-menu-another = Tambahkan akun lain
add-account-menu-manage = Kelola akun
app-menu = Menu utama
