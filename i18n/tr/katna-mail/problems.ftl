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
