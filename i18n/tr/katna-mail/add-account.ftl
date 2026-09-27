# Katna Mail, Turkish (Türkçe).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Posta hesabı ekle
add-account-looking = { $address } için posta sunucuları aranıyor…
add-account-address-intro = E-posta adresinizi girin. Katna sunucuları sizin için bulur.
add-account-servers-title = Sunucu ayarları
add-account-servers-intro = Katna'nın { $address } için postaları okuduğu ve gönderdiği yer.
add-account-password-title = Parolanızı girin
add-account-signing-in = Oturum açılıyor…

## Add a mail account: fields

add-account-field-address = E-posta adresi
add-account-incoming = Gelen posta ({ $protocol })
add-account-outgoing = Giden posta ({ $protocol })
add-account-field-server = Sunucu
add-account-field-port = Bağlantı noktası
add-account-security-none = Yok
add-account-field-username = Kullanıcı adı
add-account-field-password = Parola
add-account-show-password = Parolayı göster
add-account-app-password-hint = { $provider } burada web'de kullandığınız parolayı değil, bir uygulama parolasını ister. { $provider } hesabınızın güvenlik ayarlarından bir tane oluşturun.
add-account-field-name = Adınız (isteğe bağlı)
add-account-name-hint = Yazdığınız kişilere gösterilir.
add-account-servers-pair = { $imap } ve { $smtp }
add-account-servers-found = { $source ->
    [built-in] Sunucular: { $servers }, Katna'nın sağlayıcı listesinde bulundu.
    [provider] Sunucular: { $servers }, sağlayıcınızın ayarlarında bulundu.
    [ispdb] Sunucular: { $servers }, Thunderbird'ün sağlayıcı listesinde bulundu.
    [dns] Sunucular: { $servers }, alan adınızın DNS kayıtlarında bulundu.
   *[other] Sunucular: { $servers }, tahmin edildi; oturum açma başarısız olursa denetleyin.
}
add-account-servers-entered = Sunucular: { $servers }, girildiği gibi.

## Add a mail account: buttons

add-account-servers-button = Sunucu ayarları
add-account-back = Geri
add-account-add = Hesap ekle
add-account-next = İleri
add-account-cancel = İptal

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Gelen posta sunucusunu girin.
   *[outgoing] Giden posta sunucusunu girin.
}
add-account-server-space = { $kind ->
    [incoming] Gelen posta sunucusunun adında boşluk var.
   *[outgoing] Giden posta sunucusunun adında boşluk var.
}
add-account-port-invalid = { $kind ->
    [incoming] Gelen posta bağlantı noktası { $min } ile { $max } arasında bir sayı olmalıdır.
   *[outgoing] Giden posta bağlantı noktası { $min } ile { $max } arasında bir sayı olmalıdır.
}
add-account-address-empty = Bir e-posta adresi girin.
add-account-address-invalid = { $example } gibi bir e-posta adresi girin.
add-account-not-found = Katna, { $address } için sunucuları bulamadı, bu yüzden olağan adları doldurdu. Sağlayıcınızla denetleyin.
add-account-password-empty = Parolayı girin.
add-account-name-is-password = Ad, parolayla aynı. Oraya bunun yerine adınızı, insanların görmesi gereken biçimde yazın.
add-account-added = { $address } eklendi. Postalarınız alınıyor…
add-account-app-password-refused = { $provider } parolayı reddetti. Web'de kullandığınız parola değil, bir uygulama parolası gerekiyor.
add-account-password-refused = Sunucu parolayı reddetti. Denetleyip yeniden deneyin.

## The account menu (from the account button on the top bar)

add-account-menu-another = Başka bir hesap ekle
add-account-menu-manage = Hesapları yönet
