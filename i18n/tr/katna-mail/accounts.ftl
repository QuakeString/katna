# Katna Mail, Turkish (Türkçe).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = Klasör bölmesi
accounts-folder-pane-detail = Soldaki bölmenin hangi hesapların klasörlerini gösterdiği.
accounts-shown-one = Bir seferde bir hesap; hesap kartından geçiş yapın
accounts-shown-all = Tüm hesaplar, art arda
accounts-row = Hesaplar
accounts-row-detail = Bir hesabı kaldırmak, Katna'nın bu bilgisayardaki posta kopyasını siler. Postalar sunucuda kalır.
accounts-none = Henüz hesap yok.
accounts-kind-imported = İçe aktarılmış
accounts-picture-reset = Masaüstü resmini kullan
accounts-picture-change = Resmi değiştir
accounts-remove = Kaldır
accounts-delete-all-row = Tüm verileri sil
accounts-delete-all-row-detail = Yeni bir kurulumdaki gibi baştan başlayın.
accounts-delete-all-about = Tüm hesapları, saklanan tüm postaları, kişileri ve takvimleri, arama dizinini, ayarlarınızı ve kayıtlı şifreleri bu bilgisayardan siler. Posta sunucularınızda hiçbir şey değişmez.
accounts-delete-all-open = Tüm Katna verilerini sil

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } Katna'dan kaldırıldı.
accounts-removed = { $address } Katna'dan kaldırıldı. Postaları hâlâ sunucuda.
accounts-all-deleted = Tüm Katna verileri bu bilgisayardan silindi.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } kaldırılsın mı?
accounts-remove-confirm = Hesabı kaldır
accounts-removing = Kaldırılıyor…
accounts-remove-local-mail = { $folders ->
    [0] Bu hesaba içe aktarılan tüm postalar
    [one] Bu hesaba içe aktarılan ve klasöründeki tüm postalar
   *[other] Bu hesaba içe aktarılan ve { $folders } klasöründeki tüm postalar
}
accounts-remove-local-settings = Hesabın Katna ayarları
accounts-remove-mail = { $folders ->
    [0] Katna'nın sakladığı, bu hesaba ait tüm postalar
    [one] Katna'nın klasöründe sakladığı, bu hesaba ait tüm postalar
   *[other] Katna'nın { $folders } klasöründe sakladığı, bu hesaba ait tüm postalar
}
accounts-remove-outbox = Hesabın Giden Kutusu'nda bekleyen iletileri
accounts-remove-settings = Hesabın kayıtlı şifresi ve Katna ayarları
accounts-delete-all-title = Tüm Katna verileri silinsin mi?
accounts-delete-all-confirm = Her şeyi sil
accounts-deleting = Siliniyor…
accounts-delete-all-accounts = Tüm hesaplar ve Katna'nın sakladığı tüm postalar ve ekler
accounts-delete-all-contacts = Kişiler, takvimler ve arama dizini
accounts-delete-all-settings = Tüm ayarlar, imzalar ve klavye kısayolları
accounts-delete-all-passwords = Kayıtlı tüm şifreler
accounts-deleted-heading = Bu bilgisayardan silinenler:
accounts-cannot-undo = Bu işlem geri alınamaz.
accounts-server-delete-all = Posta sunucularınızda hiçbir şey değişmez: postalarınız orada kalır ve bir hesabı yeniden eklemek onları yeniden indirir. Dosyalardan içe aktarılan postalar yalnızca Katna'dadır; dosyalara dokunulmaz.
accounts-server-local = Bu postalar dosyalardan içe aktarıldı, bu yüzden tek kopya Katna'da. Geldikleri dosyalara dokunulmaz; geri almak için onları yeniden içe aktarın.
accounts-server-remove = Posta sunucusunda hiçbir şey değişmez: postalarınız orada kalır ve hesabı yeniden eklemek onları yeniden indirir.
accounts-confirm-word = sil
accounts-confirm-placeholder = “{ accounts-confirm-word }” yazın
accounts-confirm-prompt = Onaylamak için “{ accounts-confirm-word }” yazın:
accounts-cancel = İptal
