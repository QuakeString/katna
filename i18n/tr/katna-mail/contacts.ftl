# Katna Mail, Turkish (Türkçe): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Kişiler
contacts-frequent = Sık kullanılanlar
contacts-other = Diğer kişiler
contacts-other-about = Gmail’den e-posta gönderdiğiniz ancak kaydetmediğiniz kişiler
contacts-other-email = E-posta gönder
contacts-other-empty = Başka kişi yok. Gmail’den e-posta gönderdiğiniz ancak kaydetmediğiniz kişiler burada görünür.
contacts-other-allow = Diğer kişileri görmek için Gmail hesabınızda yeniden oturum açın ve Katna’nın onları görmesine izin verin.
contacts-labels = Etiketler
contacts-label-options = Etiket seçenekleri
contacts-label-rename = Etiketi yeniden adlandır
contacts-label-email = Herkese e-posta gönder
contacts-label-delete = Etiketi sil
contacts-label-new = Yeni etiket
contacts-label-name = Etiket adı
contacts-label-button = Etiket
contacts-label-menu = Şu etiketle:
contacts-label-added = { $name } etiketine eklendi
contacts-label-removed = { $name } etiketinden kaldırıldı
contacts-label-renamed = Etiket adı { $name } olarak değiştirildi
contacts-label-deleted = { $name } etiketi silindi
contacts-label-no-email = Bu etiketteki hiç kimsenin e-posta adresi yok
contacts-manage = Düzelt ve yönet
contacts-merge = Birleştir ve düzelt
contacts-merge-about = { $count ->
    [one] { $count } öneri: aynı kişiye benzeyen kişiler
   *[other] { $count } öneri: aynı kişiye benzeyen kişiler
}
contacts-merge-none = Yinelenen kişi yok. Aynı ada veya telefon numarasına sahip kişiler burada görünür.
contacts-merge-count = { $count ->
    [one] { $count } kişi
   *[other] { $count } kişi
}
contacts-merge-all = Tümünü birleştir
contacts-merge-button = Birleştir
contacts-merge-dismiss = Kapat
contacts-merged = { $count ->
    [1] Kişiler birleştirildi
    [one] { $count } birleştirme yapıldı
   *[other] { $count } birleştirme yapıldı
}
contacts-import = İçe aktar
contacts-export = Dışa aktar
contacts-import-file = Kişileri vCard veya CSV dosyasından içe aktar
contacts-imported = { $count ->
    [one] { $count } kişi { $place } konumuna aktarıldı
   *[other] { $count } kişi { $place } konumuna aktarıldı
}
contacts-imported-some = { $count ->
    [one] { $count } kişi { $place } konumuna aktarıldı; zaten kayıtlı olan { $skipped } kişi atlandı
   *[other] { $count } kişi { $place } konumuna aktarıldı; zaten kayıtlı olan { $skipped } kişi atlandı
}
contacts-import-none = { $name } içinde kişi bulunamadı
contacts-import-all-saved = { $name } içindeki herkes zaten kayıtlı
contacts-import-failed = { $name } okunamadı: { $error }
contacts-exported = { $count ->
    [one] { $count } kişi { $path } konumuna aktarıldı
   *[other] { $count } kişi { $path } konumuna aktarıldı
}
contacts-export-none = Dışa aktarılacak kişi yok
contacts-export-failed = Kişiler dışa aktarılamadı: { $error }
contacts-print = Yazdır
contacts-print-title = Kişiler
contacts-print-none = Yazdırılacak kişi yok
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = Doğum günü: { $day }
contacts-print-nickname = Takma ad: { $name }
contacts-create = Kişi oluştur

## Search and the list

contacts-search = Kişilerde ara
contacts-loading = Kişiler yükleniyor…
contacts-empty = Henüz kayıtlı kişi yok. Gmail’de, Outlook’ta veya e-posta hizmetinizde kaydettiğiniz kişiler burada görünür.
contacts-empty-no-books = Hesaplarınızdaki kişiler eşitlendikten sonra burada görünür.
contacts-none-found = Aramanızla eşleşen kişi yok.
contacts-starred = { $count ->
    [one] Yıldızlı kişi ({ $count })
   *[other] Yıldızlı kişiler ({ $count })
}
contacts-count = Kişiler ({ $count })
contacts-col-name = Ad
contacts-col-email = E-posta
contacts-col-phone = Telefon numarası
contacts-col-job = İş unvanı ve şirket
contacts-col-labels = Etiketler

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Katna’nın { $address } hesabının kişilerini okumasına izin ver.
contacts-allow-many = { $more ->
    [one] Katna’nın { $address } ve { $more } diğer hesabın kişilerini okumasına izin ver.
   *[other] Katna’nın { $address } ve { $more } diğer hesabın kişilerini okumasına izin ver.
}
contacts-allow-button = İzin ver

## A contact's page

contacts-back = Kişilere geri dön
contacts-edit = Düzenle
contacts-delete = Sil
contacts-qr = QR kodu olarak paylaş
contacts-qr-about = Kişiyi kaydetmek için bunu bir telefonun kamerasıyla tarayın.
contacts-qr-too-long = Bu kişinin bilgileri bir QR koduna sığmayacak kadar fazla.
contacts-qr-done = Bitti
contacts-deleted = { $name } silindi
contacts-added = { $name } kişilere eklendi
contacts-find-mail = Posta
contacts-details = Kişi bilgileri
contacts-saved-in = Kaydedildiği yer
contacts-notes = Notlar
contacts-birthday = Doğum günü
contacts-nickname = Takma ad
contacts-this-computer = Bu bilgisayar
contacts-kind-home = Ev
contacts-kind-work = İş
contacts-kind-mobile = Cep
contacts-kind-other = Diğer
contacts-source-google = Google Kişiler
contacts-source-microsoft = Outlook kişileri
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Kişi oluştur
contacts-edit-title = Kişiyi düzenle
contacts-edit-save = Kaydet
contacts-edit-saving = Kaydediliyor…
contacts-edit-cancel = İptal
contacts-saved = Kişi kaydedildi
contacts-edit-save-to = Kaydedilecek yer
contacts-edit-changes-go-to = Değişiklikler { $place } konumuna kaydedilir.
contacts-edit-given = Ad
contacts-edit-family = Soyadı
contacts-edit-company = Şirket
contacts-edit-job = İş unvanı
contacts-edit-email = E-posta
contacts-edit-phone = Telefon
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = E-posta ekle
contacts-edit-add-phone = Telefon ekle
contacts-edit-street = Sokak adresi
contacts-edit-city = Şehir
contacts-edit-postcode = Posta kodu
contacts-edit-country = Ülke
contacts-edit-birthday = Doğum günü (YYYY-MM-DD)
contacts-edit-empty = Önce bir ad, e-posta veya telefon numarası ekleyin.
