# Katna Mail, Malay (Bahasa Melayu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Tambah akaun mel
add-account-providers-intro = Pilih penyedia mel anda. Katna akan mencari selebihnya.
add-account-provider-other = Mel lain
add-account-provider-other-detail = Sebarang akaun IMAP atau POP3
add-account-provider-google-detail = Gmail dan Google Workspace
add-account-provider-microsoft-detail = Outlook dan Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = Log masuk ke { $provider }
add-account-form-title-other = Akaun mel anda
add-account-form-intro = Katna menyimpan kata laluan anda dalam gelang kunci sistem anda.
add-account-looking = Mencari pelayan mel untuk { $address }…
add-account-address-intro = Masukkan alamat e-mel anda. Katna akan mencari pelayan untuk anda.
add-account-servers-title = Tetapan pelayan
add-account-servers-intro = Tempat Katna membaca dan menghantar mel untuk { $address }.
add-account-signing-in = Log masuk…
add-account-browser-title = Teruskan dalam pelayar anda
add-account-browser-intro = Katna telah membuka halaman log masuk { $provider } dalam pelayar anda. Log masuk di sana dan benarkan Katna membaca dan menghantar mel anda, kemudian kembali ke sini.
add-account-browser-hint = Tiada halaman dibuka? Semak tetingkap pelayar anda, atau kembali dan cuba lagi.
add-account-stage-browser = Menunggu anda log masuk dalam pelayar…
add-account-stage-signing-in-at = Log masuk di { $server }…
add-account-help-app-password-link = Cara membuat kata laluan aplikasi
add-account-help-turn-on-imap = { $provider } hanya membenarkan apl mel masuk setelah akses IMAP dan POP3 dihidupkan dalam tetapan mel webnya.
add-account-help-turn-on-imap-link = Cara menghidupkannya

## Add a mail account: fields

add-account-field-address = Alamat e-mel
add-account-receive-with = Terima mel dengan
add-account-imap-about = IMAP menyimpan mel dan folder anda pada pelayan, sama pada setiap peranti. Pilih IMAP jika boleh.
add-account-pop3-about = POP3 memuat turun mel anda ke komputer ini. Mel yang anda baca atau alihkan di sini kekal seperti asal pada pelayan dan peranti anda yang lain.
add-account-incoming = Mel masuk ({ $protocol })
add-account-outgoing = Mel keluar ({ $protocol })
add-account-field-server = Pelayan
add-account-field-port = Port
add-account-security-none = Tiada
add-account-security-none-warning = Tidak disulitkan: kata laluan dan mel anda boleh dibaca semasa dalam perjalanan.
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

add-account-sign-in-with = Log masuk dengan { $provider }
add-account-sign-in-instead = Log masuk dengan { $provider } sahaja
add-account-servers-button = Tetapan pelayan
add-account-back = Kembali
add-account-add = Tambah akaun
add-account-done = Selesai
add-account-another = Tambah akaun lain
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
add-account-app-password-refused = { $provider } menolak kata laluan itu. Ia memerlukan kata laluan aplikasi, bukan kata laluan yang anda gunakan di web.
add-account-password-refused = Pelayan menolak kata laluan itu. Semak dan cuba lagi.
add-account-sign-in-refused = { $provider } tidak membenarkan Katna masuk. Cuba lagi, dan benarkan akses kepada mel anda.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Salinan Katna ini belum boleh log masuk ke akaun Microsoft.
    [Google] Salinan Katna ini belum boleh log masuk ke akaun Google.
   *[other] Penyedia ini hanya membenarkan log masuk di halamannya sendiri, dan Katna belum boleh melakukannya untuk penyedia ini.
}
add-account-smtp-not-found = Katna menemui tempat untuk membaca mel anda tetapi bukan tempat untuk menghantarnya. Masukkan pelayan keluar.

## Add a mail account: the last step

add-account-done-title = Akaun anda sudah sedia
add-account-done-intro = Katna sedang mendapatkan mel anda. Mel baharu dipaparkan sebaik sahaja tiba.
add-account-done-sign-in = Log masuk
add-account-done-signed-in-with = Dengan { $provider }, dalam pelayar anda
add-account-done-receiving = Menerima mel
add-account-done-sending = Menghantar mel
add-account-done-on-server = Mel pada pelayan
add-account-done-kept = Disimpan sehingga anda memadamnya dalam Katna
add-account-done-pop3-hint = Tukar apa yang berlaku kepada mel pada pelayan dalam Tetapan > Akaun.
add-account-done-zoho-title = Tugas dan kalendar
add-account-done-zoho-about = Zoho menyimpan kedua-duanya berasingan daripada mel. Log masuk dengan Zoho sekali untuk membawanya ke dalam Katna.
add-account-done-linked = Tugas dan kalendar disambungkan

## The account menu (from the account button on the top bar)

add-account-menu-another = Tambah akaun lain
app-menu = Menu utama
app-menu-back = Kembali
