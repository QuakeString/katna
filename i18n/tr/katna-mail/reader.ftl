# Katna Mail, Turkish (Türkçe).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Kapat
reader-back = Geri
reader-mark-unread = Okunmadı olarak işaretle
reader-move-to = Taşı
reader-snooze = Ertele
reader-remind = Bana hatırlat
reader-more = Diğer
reader-original-colors = Özgün renkleri göster
reader-dark-colors = Koyu renklerde göster
reader-print-all = Tümünü yazdır
reader-new-window = Yeni pencerede
reader-position = { $position } / { $total }
reader-newer = Daha yeni
reader-older = Daha eski

## Reading pane: the conversation

reader-removed = Bu ileti dizisi kaldırıldı.
reader-no-subject = (konu yok)
reader-collapse-all = Tümünü daralt
reader-expand-all = Tümünü genişlet
reader-unknown-sender = (bilinmeyen gönderen)
reader-date-ago = { $date } ({ $ago })
reader-sending = Gönderiliyor…
reader-me = ben
reader-to = alıcı: { $names }
reader-to-label = alıcı:
reader-tick-delivered = Teslim edildi: { $when }
reader-tick-no-bounce = Gönderildi: { $when }; geri dönme bildirimi gelmedi, bu yüzden büyük olasılıkla ulaştı
reader-tick-bounced = Teslim edilmedi: geri döndü, { $when }
reader-tick-read = Okundu: { $when } (okundu bilgisi)
reader-tick-opened = Açıldı, son olarak { $when } (açılma izleme)
reader-starred = Yıldızlı
reader-chip-remove = { $label } etiketini kaldır
reader-not-starred = Yıldızlı değil
reader-too-long = İleti tamamen gösterilemeyecek kadar uzun.
reader-encrypted-images = Şifreli postalarda web'deki resimler hiçbir zaman yüklenmez.
reader-window-failed = Yeni pencere açılamadı.

## Reading pane: message details (opened from "to me")

reader-details-from = kimden:
reader-details-to = kime:
reader-details-cc = bilgi:
reader-details-date = tarih:
reader-details-subject = konu:

## Reading pane: downloading a message

reader-downloading = Bu ileti sunucudan indiriliyor…
reader-download-failed = Bu ileti indirilemedi.
reader-download-failed-reason = Bu ileti indirilemedi. { $reason }
reader-download-offline = Bu hesap çevrimdışı. Bu iletiyi indirmek için çevrimiçi olun.
reader-try-again = Tekrar dene

## Reply row

reply-reply = Yanıtla
reply-reply-all = Tümünü yanıtla
reply-forward = Yönlendir

## Encrypted and signed mail

security-decrypting = Şifre çözülüyor…
security-checking = İmza denetleniyor…
security-partly-encrypted = Bu iletinin yalnızca bir kısmı şifreli. Geri kalanı korumanın dışında eklenmiş ve herhangi birinden gelmiş olabilir.
security-partly-signed = Bu iletinin yalnızca bir kısmı imzalı. Geri kalanı korumanın dışında eklenmiş ve herhangi birinden gelmiş olabilir.
security-encrypted = Şifreli ileti
security-encrypted-smime = Şifreli ileti (S/MIME)
security-no-key = Bu iletinin şifresi çözülemiyor: sizde olmayan bir anahtar için şifrelenmiş.
security-cancelled = Şifre çözme iptal edildi.
security-damaged = Bu iletinin şifresi çözülemiyor: şifreli veriler bozuk ya da değiştirilmiş.
security-decrypt-unavailable = Bu iletinin şifresi çözülemiyor: şifreli postaları okumak için { $tool } yükleyin.
security-decrypt-failed = Bu iletinin şifresi çözülemiyor: { $reason }
security-unknown-signer = bilinmeyen bir imzacı
security-signed-verified = { $signer } tarafından imzalandı · doğrulandı
security-signed-not-sender = { $signer } tarafından imzalandı; imzalayan gönderen değil
security-signed-untrusted = { $signer } tarafından, güvenilmez olarak işaretlediğiniz bir anahtarla imzalandı
security-signed-unverified = { $signer } tarafından imzalandı · anahtar doğrulanmadı
security-bad-signature = Geçersiz imza: bu ileti imzalandıktan sonra değiştirilmiş ya da imza sahte.
security-signature-expired = { $signer } tarafından imzalandı · imzanın süresi doldu
security-key-expired = { $signer } tarafından imzalandı · anahtarın süresi o zamandan beri doldu
security-key-revoked = { $signer } tarafından, iptal edilmiş bir anahtarla imzalandı
security-missing-key = Sizde olmayan bir anahtarla imzalanmış, bu yüzden denetlenemiyor
security-missing-key-id = Sizde olmayan bir anahtarla ({ $key }) imzalanmış, bu yüzden denetlenemiyor
security-signature-unavailable = İmzalı; imzayı denetlemek için { $tool } yükleyin
security-signature-error = İmza denetlenemedi.
security-look-up-key = Anahtarı ara
key-card-verified = Doğrulanmış imza
key-card-verified-detail = İmza geçerli ve bu anahtara güveniyorsunuz.
key-card-unverified = İmza doğrulanmadı
key-card-unverified-detail = İmza geçerli, ancak anahtarın gerçekten bu kişiye ait olduğunu doğrulayan bir şey yok. Parmak izini kendisiyle karşılaştırın, ardından anahtara GnuPG'de güvenin (Kleopatra veya gpg --edit-key).
key-card-not-sender = Başka biri tarafından imzalandı
key-card-not-sender-detail = İmza geçerli, ancak anahtar gönderene ait değil.
key-card-untrusted = Anahtara güvenilmiyor
key-card-untrusted-detail = Bu anahtarı GnuPG'de güvenilmez olarak işaretlediniz.
key-card-signature-expired = İmzanın süresi doldu
key-card-signature-expired-detail = İmza geçerliydi, ancak süresi doldu.
key-card-key-expired = Anahtarın süresi doldu
key-card-key-expired-detail = İmza geçerli, ancak anahtarın süresi o zamandan beri doldu.
key-card-key-revoked = Anahtar iptal edildi
key-card-key-revoked-detail = Sahibi bu anahtarı iptal etti, bu yüzden imzaya güvenilemez.
key-card-bad = Geçersiz imza
key-card-bad-detail = Bu ileti imzalandıktan sonra değiştirilmiş ya da imza sahte.
key-card-signed-by = İmzalayan
key-card-belongs-to = Sahibi
key-card-fingerprint = Parmak izi
key-card-signed = İmzalanma
key-card-key = Anahtar
key-card-kind = { $standard }, { $algorithm }
key-card-created = Oluşturulma
key-card-expires = Son geçerlilik
key-card-never = Hiçbir zaman
key-card-issued-by = Veren
key-card-found-in = Bulunduğu yer
key-card-keyring = GnuPG anahtarlığınız
key-card-copy = Parmak izini kopyala
key-card-import-title = Bu anahtar içe aktarılsın mı?
key-card-from-directory = { $domain } alan adının anahtar dizininde bulundu.
key-card-from-attachment = Ekten: { $name }.
key-card-import-note = Katna bundan sonra bu kişinin imzalarını denetleyebilir ve ona şifreli posta gönderebilir. Anahtara tam olarak güvenmek için parmak izini kendisiyle karşılaştırın.
key-card-cancel = İptal
key-card-import = Anahtarı içe aktar
key-card-looking-up = Anahtar aranıyor…
key-card-looking-up-detail = { $domain } alan adının anahtar dizinine soruluyor.
key-card-not-found = Anahtar bulunamadı
key-card-not-found-detail = { $domain } bu adres için bir anahtar yayımlamıyor. Gönderenden kendi anahtarını size göndermesini isteyin.
key-card-not-kept = Bulunan anahtar kullanılamıyor.
key-card-failed = Anahtar alınamadı
sender-failed-title = Bu posta { $domain } adresinden gelmemiş olabilir
sender-failed-body = { $provider } tarafından yapılan gönderen denetimlerinden geçemedi. Bağlantılara, eklere ve yanıtlara dikkat edin.
sender-provider-unknown = posta sağlayıcınız
sender-details = Ayrıntılar
sender-details-hide = Ayrıntıları gizle
sender-looks-safe = Güvenli görünüyor
sender-move-to-spam = Spam'e taşı
sender-checked-by = Denetleyen: { $provider }
sender-checked-by-server = Denetleyen: { $provider } ({ $server })
sender-dmarc = Gönderen alan adı (DMARC)
sender-dkim = İmza (DKIM)
sender-spf = Gönderen sunucu (SPF)
sender-result-pass = Geçti
sender-result-fail = Geçemedi
sender-result-unsure = Emin değil
sender-result-none = Yok
sender-result-missing = Denetlenmedi
sender-dmarc-pass = { $domain } bu göndereni doğruluyor.
sender-dmarc-fail = Posta, { $domain } alan adının postalarının nasıl gönderildiğine dair söylediğiyle uyuşmuyor.
sender-dmarc-none = { $domain } postaları için hiçbir kural yayımlamıyor.
sender-dkim-pass = { $domain } tarafından imzalandı.
sender-dkim-fail = { $domain } imzası postayla uyuşmuyor.
sender-dkim-none = İleti imzalanmamış.
sender-spf-pass = { $domain } alan adının listelediği bir sunucudan gönderildi.
sender-spf-fail = { $domain } alan adının listelemediği bir sunucudan gönderildi.
sender-spf-none = { $domain } sunucularını listelemiyor.
sender-check-unsure = Denetim net bir yanıt veremedi.
sender-unconfirmed = { $provider }, bunun { $domain } adresinden geldiğini doğrulayamadı. Herkes gönderen olarak herhangi bir şey yazabilir.
sender-link-title = Bu bağlantı açılsın mı?
sender-link-body = Bu posta gönderen denetimlerinden geçemedi. Bağlantının gittiği yer: { $host }
sender-link-cancel = İptal
sender-link-open = Aç
tracking-opened = { $who } iletiyi { $count ->
    [one] bir kez
   *[other] { $count } kez
} açtı, son olarak { $when }
tracking-opens-clicks = { $who } iletiyi { $opens ->
    [one] bir kez
   *[other] { $opens } kez
} açtı ve bir bağlantıyı { $clicks ->
    [one] bir kez
   *[other] { $clicks } kez
} izledi, son olarak { $when }
tracking-clicked = { $who } bir bağlantıyı { $clicks ->
    [one] bir kez
   *[other] { $clicks } kez
} izledi, son olarak { $when }
tracking-maybe-opened = { $who } iletiyi açmış olabilir (Apple Mail gizlilik için resimleri yükler)
tracking-seen-none = Henüz kimse iletiyi açmadı veya bir bağlantıyı izlemedi
tracking-receipt = { $who } okundu bilgisi gönderdi
tracking-receipt-read = { $who } okudu (okundu bilgisi), { $when }
tracking-receipt-displayed = Okundu bilgisi: { $who } iletinizi açtı
tracking-receipt-other = Okundu bilgisi: { $who } iletinizi açmadan sildi veya işledi

## Remote images and pictures

remote-hidden = Bu iletideki resimler gizlendi.
remote-hidden-unconfirmed = Resimler gizlendi: gönderen doğrulanamadı.
remote-hidden-failed = Resimler gizlendi: bu posta gönderen denetimlerinden geçemedi.
remote-show = Resimleri göster
remote-always-show = Bu gönderenden her zaman göster
remote-picture-use = Kullan
remote-picture-too-big = En fazla 8 MB boyutunda bir resim seçin.
remote-picture-type = PNG, JPEG, GIF, WebP veya SVG biçiminde bir resim seçin.
remote-picture-read-failed = Resim okunamıyor: { $error }
remote-picture-keep-failed = Resim saklanamıyor: { $error }
remote-picture-remove-failed = Resim kaldırılamıyor: { $error }

## Attachments

attachment-count = { $count ->
    [one] Bir ek
   *[other] { $count } ek
}
attachment-save = Kaydet
attachment-forward = İlet
attachment-save-all = Tümünü kaydet
attachment-save-all-tooltip = Tüm ekleri bir klasöre kaydet
attachment-save-here = Buraya kaydet
attachment-not-downloaded = Bu ileti indirilmedi.
attachment-not-found = Bu ek iletide bulunamadı.
attachment-read-failed = { $name } okunamadı
attachment-numbered = ek { $number }
attachment-saved-all = { $count ->
    [one] { $count } dosya { $place } konumuna kaydedildi
   *[other] { $count } dosya { $place } konumuna kaydedildi
}
attachment-saved-some = { $total ->
    [one] { $total } dosyadan { $saved } tanesi { $place } konumuna kaydedildi. { $failed } kaydedilemedi
   *[other] { $total } dosyadan { $saved } tanesi { $place } konumuna kaydedildi. { $failed } kaydedilemedi
}
attachment-saved-to = Kaydedildiği yer: { $path }
attachment-save-failed = { $name } kaydedilemedi: { $error }
attachment-open-failed = { $name } açılamadı: { $error }
attachment-risky = Bu dosya bir program çalıştırabilir, bu yüzden Katna onu açmaz. Bunun yerine dosyayı kaydedin.
attachment-encrypted-open = Bu dosya şifreli geldi. Başka bir yerde açmak için kaydedin.

## Printing

print-failed = Yazdırılamadı: { $error }
print-no-font = yazı tipi bulunamadı
print-opened-as-pdf = Oradan yazdırmak için PDF olarak açıldı.
print-preview-title = Baskı önizlemesi
print-preview-laying-out = Sayfalar düzenleniyor…
print-preview-pages = { $count ->
    [one] { $count } sayfa
   *[other] { $count } sayfa
}
print-preview-more = { $count ->
    [one] ve { $count } sayfa daha
   *[other] ve { $count } sayfa daha
}
print-preview-failed = sayfalar gösterilemedi
print-preview-paper = Kâğıt
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-layout = Düzen
print-preview-as-shown = Göründüğü gibi
print-preview-simple = Yalnızca metin
print-preview-backgrounds = Arka planlar
print-preview-cancel = İptal
print-preview-print = Yazdır
print-not-downloaded = (Henüz indirilmedi.)
print-encrypted = (Şifreli. Metnini yazdırmak için Katna Mail'de açın.)
print-to = Kime: { $addresses }
print-cc = Bilgi: { $addresses }
text-pin = En üste sabitle
text-copy-address = Adresi kopyala

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Eklerini okumak için bu iletiyi açın.
text-copy = Kopyala
text-select-all = Tümünü seç
