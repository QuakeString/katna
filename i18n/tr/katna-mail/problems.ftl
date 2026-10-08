# Katna Mail, Turkish (Türkçe).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = Posta sunucusu
problems-signed-out = { $provider }, Katna'nın { $address } oturumunu kapattı. Posta eşitlemesi durdu.
problems-password-refused = { $provider }, { $address } için parolayı reddetti. Parola değişmiş olabilir.
problems-no-answer = { $provider }, { $address } için yanıt vermiyor. Katna denemeye devam ediyor.
problems-offline = Çevrimdışısınız. Postalarınız hâlâ burada; gönderdiğiniz postalar siz geri dönene kadar bekler.
problems-accounts-need-you = { $count ->
    [one] 1 hesap sizi bekliyor
   *[other] { $count } hesap sizi bekliyor
}
problems-show = Göster
problems-later = Daha sonra
problems-new-password = Yeni parola
problems-try-again = Yeniden dene

## The New password card

problems-password-title = Yeni parola
problems-password-detail = { $provider }, { $address } için kayıtlı parolayı reddetti. Yenisini yazın; Katna saklamadan önce onu denetler.
problems-password-placeholder = Parola
problems-password-show = Parolayı göster
problems-password-hide = Parolayı gizle
problems-password-cancel = İptal
problems-password-save = Kaydet
problems-password-checking = Denetleniyor…
problems-password-refused-again = { $provider } bu parolayı da reddetti. Denetleyip yeniden deneyin.
problems-password-saved = { $address } için parola kaydedildi. Postalarınız alınıyor…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $address } posta sunucusu { $count ->
    [one] bir iletinin taşınmasını kabul etmedi, bu yüzden ileti eski yerine döndü.
   *[other] { $count } iletinin taşınmasını kabul etmedi, bu yüzden iletiler eski yerlerine döndü.
}
problems-refused-flags = { $address } posta sunucusu { $count ->
    [one] bir iletinin işaretlenmesini (okundu, yıldızlı…) kabul etmedi, bu yüzden ileti eski haline döndü.
   *[other] { $count } iletinin işaretlenmesini (okundu, yıldızlı…) kabul etmedi, bu yüzden iletiler eski hallerine döndü.
}
problems-refused-label = { $address } posta sunucusu { $count ->
    [one] bir iletinin etiketlerinin değiştirilmesini kabul etmedi, bu yüzden ileti eski haline döndü.
   *[other] { $count } iletinin etiketlerinin değiştirilmesini kabul etmedi, bu yüzden iletiler eski hallerine döndü.
}
problems-refused-delete = { $address } posta sunucusu { $count ->
    [one] bir iletinin silinmesini kabul etmedi, bu yüzden ileti geri geldi.
   *[other] { $count } iletinin silinmesini kabul etmedi, bu yüzden iletiler geri geldi.
}
problems-refused-other = { $address } posta sunucusu { $count ->
    [one] bir değişikliği kabul etmedi, bu yüzden Katna onu eski haline getirdi.
   *[other] { $count } değişikliği kabul etmedi, bu yüzden Katna onları eski haline getirdi.
}
problems-details = Ayrıntılar

## Katna's background service (katna-daemon) isn't running

service-starting = Katna arka plan hizmeti başlatılıyor…
service-failed = Katna arka plan hizmeti başlamıyor, bu yüzden postalar eşitlenmiyor.
service-start-again = Yeniden başlat
service-started-again = Katna arka plan hizmeti durdu ve yeniden başlatıldı.
service-details-title = Hizmet neden başlamıyor
service-details-body = Bunu kopyalayın ve raporunuzla birlikte gönderin. İçinde posta veya parola yok.
service-details-copy = Kopyala
service-details-close = Kapat
service-not-running = Katna arka plan hizmeti çalışmıyor.
service-no-answer = Katna arka plan hizmeti yanıt vermedi: { $error }
service-no-session = D-Bus oturumu yok: { $error }
safe-line = Katna, güncellemedeki bir sorundan sonra güvenli modda; bu yüzden posta eşitlenmiyor.
safe-try-again = Yeniden dene
safe-restore = Geri yükle
safe-restoring = Verileriniz { $when } tarihli kopyadan geri yükleniyor…
safe-restored = Verileriniz { $when } tarihli kopyadan geri yüklendi. Önceden orada olanlar bir klasörde saklanıyor.
safe-show-folder = Klasörü göster
safe-restore-failed = Verileriniz geri yüklenemedi: { $error }
safe-restore-title = Verileriniz bir güncellemeden önceki haline geri yüklensin mi?
safe-restore-body = Katna seçtiğiniz kopyaya geri döner. O kopyadan sonra gelen postalar hesaplarınızdan yeniden indirilir.
safe-restore-none = Henüz kopya yok. Katna, her güncelleme verilerinizi değiştirmeden önce bir kopya alır.
safe-restore-keep = Şu anda olanlar, gönderilmemiş postalar, taslaklar ve henüz eşitlenmemiş değişiklikler dahil, önce bir klasörde saklanır; böylece hiçbir şey kaybolmaz.
safe-restore-cancel = İptal
safe-restore-mail = Posta
safe-restore-pim = Hesaplar ve kişiler
safe-restore-blobs = Ekler
safe-report-title = Hata ayıklama raporu
safe-report-body = Bunu kopyalayıp hata raporunuza ekleyin. İçinde posta, adres veya parola yoktur.
safe-report-restore = Geri yükle…
safe-report-copied = Hata ayıklama raporu kopyalandı
