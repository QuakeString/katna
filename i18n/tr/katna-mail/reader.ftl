# Katna Mail, Turkish (Türkçe).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Kapat
reader-back = Geri
reader-mark-unread = Okunmadı olarak işaretle
reader-move-to = Taşı
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
reader-me = ben
reader-to = alıcı: { $names }
reader-to-label = alıcı:
reader-tick-delivered = Teslim edildi: { $when }
reader-tick-no-bounce = Gönderildi: { $when }; geri dönme bildirimi gelmedi, bu yüzden büyük olasılıkla ulaştı
reader-tick-bounced = Teslim edilmedi: geri döndü, { $when }
reader-tick-read = Okundu: { $when } (okundu bilgisi)
reader-tick-opened = Açıldı, son olarak { $when } (açılma izleme)
reader-starred = Yıldızlı
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
tracking-receipt-displayed = Okundu bilgisi: { $who } iletinizi açtı
tracking-receipt-other = Okundu bilgisi: { $who } iletinizi açmadan sildi veya işledi

## Remote images and pictures

remote-hidden = Bu iletideki resimler gizlendi.
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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Eklerini okumak için bu iletiyi açın.
text-copy = Kopyala
text-select-all = Tümünü seç
