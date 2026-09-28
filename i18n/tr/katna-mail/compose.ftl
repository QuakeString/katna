# Katna Mail, Turkish (Türkçe).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = Yeni İleti
compose-restore = Geri yükle
compose-minimize = Simge durumuna küçült
compose-exit-full-screen = Tam ekrandan çık
compose-open-window = Yeni pencerede aç
compose-save-close = Kaydet ve kapat
compose-back-to-mail = Posta penceresine dön
compose-pop-out-reply = Yanıtı ayrı aç
compose-edit-recipients = Alıcıları düzenle
compose-summary-cc = Bilgi: { $names }
compose-summary-bcc = Gizli: { $names }
compose-show-trimmed = Kırpılan içeriği göster
compose-hide-trimmed = Kırpılan içeriği gizle
compose-remove-trimmed = Alıntılanan metni kaldır
compose-trimmed-removed = Alıntılanan metin kaldırıldı

## Recipients and subject

compose-to = Kime
compose-cc = Bilgi
compose-bcc = Gizli
compose-from = Kimden
compose-from-choose = Başka bir hesaptan gönder
compose-recipients = Alıcılar
compose-subject = Konu

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = Önce açık iletiyi gönderin veya silin.
compose-bad-address = “{ $address }” bir e-posta adresi değil.
compose-no-recipients = En az bir alıcı ekleyin.
compose-attachments-too-large = Ekler { $size }; posta sunucuları en fazla { $limit } kabul eder.
compose-no-account = Posta göndermek için bir hesap ekleyin.
compose-past-time = Gelecekte bir zaman seçin.
compose-scheduling = Planlanıyor…
compose-sending = Gönderiliyor…
compose-scheduled = Gönderim { $when } için planlandı
compose-sent-archived = Gönderildi ve arşivlendi
compose-sent = İleti gönderildi
compose-discarded = Taslak silindi
compose-draft-saved = Taslak kaydedildi
compose-draft-failed = Taslak kaydedilemedi: { $error }
compose-draft-not-opened = Taslak açılamadı.

## Attachments

compose-picker-insert = Ekle
compose-picker-attach = Ekle
compose-file-too-large = { $name } çok büyük: bir ileti en fazla { $limit } taşıyabilir.
compose-attachment-size = ({ $size })
compose-remove-attachment = Eki kaldır
compose-attachments-total = { $count ->
    [one] { $count } dosya, { $size }
   *[other] { $count } dosya, { $size }
}
compose-drop-files = Dosyaları buraya bırakın
compose-drop-here = Buraya bırakın
compose-paste-keep-formatting = Biçimlendirmeyi koru
compose-paste-table = Tablo
compose-paste-picture = Resim
compose-paste-plain-text = Düz metin
compose-paste-inline = Metin içinde
compose-paste-attachment = Ek

## Encryption and signing (the toggles by the recipients)

compose-encrypt = Şifrele
compose-encrypted = Şifreli: yalnızca alıcılar okuyabilir
compose-sign = İmzala
compose-signed = İmzalı: alıcılar sizden geldiğini doğrulayabilir
compose-track = Açılmaları ve tıklamaları izle
compose-tracked = İzleniyor: her alıcının iletiyi ne zaman açtığını veya bir bağlantıyı izlediğini görürsünüz
compose-track-unavailable = İmzalı, şifreli ve düz metin postalar izlenemez
compose-track-sign-in = Açılmaları ve tıklamaları izlemek için bir Katna hesabında oturum açın
compose-receipt = Okundu bilgisi iste
compose-receipt-on = Okundu bilgisi istendi: alıcının uygulaması ondan bilgi göndermesini isteyebilir
compose-delivery = Teslim bilgisi iste
compose-delivery-on = Teslim bilgisi istendi: her alıcının sunucusu iletiyi kabul ettiğinde posta sunucunuz size e-posta gönderecek
compose-delivery-unavailable = Posta sunucunuz teslim bilgisi göndermiyor

## Spelling

spell-no-dictionary = { $language } için yazım sözlüğü yüklü değil (örneğin hunspell-en_us).
spell-dictionary-error = Yazım sözlüğü: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = “{ $words }” ekle
grammar-remove = “{ $words }” kaldır
grammar-ignore = Yok say

## Send checks (asked before a message goes out)

send-check-attachment-title = Dosya eklemek mi istediniz?
send-check-attachment-text = Bir ekten söz ettiniz ama hiçbir şey eklenmedi.
send-check-attach = Dosya ekle
send-check-subject-title = Konu olmadan gönderilsin mi?
send-check-subject-text = Bu iletinin konusu yok.
send-check-add-subject = Konu ekle
send-check-send-anyway = Yine de gönder
recipient-not-valid = Geçerli bir e-posta adresi değil
recipient-show-address = Adresi göster
recipient-remove = Kaldır
recipient-bad-title = Adresi kontrol edin
recipient-bad-text = “{ $address }” geçerli bir e-posta adresi değil. Göndermeden önce düzeltin veya kaldırın.
recipient-bad-fix = Düzelt
