# Katna Mail, Malay (Bahasa Melayu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Tambah akaun mel
add-account-looking = Mencari pelayan mel untuk { $address }…
add-account-address-intro = Masukkan alamat e-mel anda. Katna akan mencari pelayan untuk anda.
add-account-servers-title = Tetapan pelayan
add-account-servers-intro = Tempat Katna membaca dan menghantar mel untuk { $address }.
add-account-password-title = Masukkan kata laluan anda
add-account-signing-in = Log masuk…

## Add a mail account: fields

add-account-field-address = Alamat e-mel
add-account-incoming = Mel masuk ({ $protocol })
add-account-outgoing = Mel keluar ({ $protocol })
add-account-field-server = Pelayan
add-account-field-port = Port
add-account-security-none = Tiada
add-account-field-username = Nama pengguna
add-account-field-password = Kata laluan
add-account-show-password = Tunjukkan kata laluan
add-account-app-password-hint = { $provider } memerlukan kata laluan aplikasi di sini, bukan kata laluan yang anda gunakan di web. Buat satu dalam tetapan keselamatan akaun { $provider } anda.
add-account-field-name = Nama anda (pilihan)
add-account-name-hint = Ditunjukkan kepada orang yang anda tulis.
add-account-servers-pair = { $imap } dan { $smtp }
add-account-servers-found = { $source ->
    [built-in] Pelayan: { $servers }, ditemui dalam senarai penyedia Katna.
    [provider] Pelayan: { $servers }, ditemui dalam tetapan penyedia anda.
    [ispdb] Pelayan: { $servers }, ditemui dalam senarai penyedia Thunderbird.
    [dns] Pelayan: { $servers }, ditemui dalam rekod DNS domain anda.
   *[other] Pelayan: { $servers }, berdasarkan tekaan; semak jika log masuk gagal.
}
add-account-servers-entered = Pelayan: { $servers }, seperti yang dimasukkan.

## Add a mail account: buttons

add-account-servers-button = Tetapan pelayan
add-account-back = Kembali
add-account-add = Tambah akaun
add-account-next = Seterusnya
add-account-cancel = Batal

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Masukkan pelayan masuk.
   *[outgoing] Masukkan pelayan keluar.
}
add-account-server-space = { $kind ->
    [incoming] Nama pelayan masuk mengandungi ruang.
   *[outgoing] Nama pelayan keluar mengandungi ruang.
}
add-account-port-invalid = { $kind ->
    [incoming] Port masuk mestilah nombor dari { $min } hingga { $max }.
   *[outgoing] Port keluar mestilah nombor dari { $min } hingga { $max }.
}
add-account-address-empty = Masukkan alamat e-mel.
add-account-address-invalid = Masukkan alamat e-mel seperti { $example }.
add-account-not-found = Katna tidak dapat mencari pelayan untuk { $address }, jadi ia mengisi nama yang biasa. Semak dengan penyedia anda.
add-account-password-empty = Masukkan kata laluan.
add-account-name-is-password = Nama itu sama dengan kata laluan. Taipkan nama anda di situ, seperti yang patut dilihat oleh orang lain.
add-account-added = { $address } telah ditambah. Mengambil mel anda…
add-account-app-password-refused = { $provider } menolak kata laluan itu. Ia memerlukan kata laluan aplikasi, bukan kata laluan yang anda gunakan di web.
add-account-password-refused = Pelayan menolak kata laluan itu. Semak dan cuba lagi.

## The account menu (from the account button on the top bar)

add-account-menu-another = Tambah akaun lain
add-account-menu-manage = Urus akaun
