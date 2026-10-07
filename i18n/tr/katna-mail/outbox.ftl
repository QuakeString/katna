# Katna Mail, Turkish (Türkçe).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = Gönderilmedi, çünkü { $reason }.
outbox-retrying = Henüz gönderilmedi, çünkü { $reason }. Katna kendiliğinden yeniden dener.
outbox-waiting-sign-in = { $address } hesabında yeniden oturum açmanız bekleniyor. Ardından gönderilir.
outbox-waiting-password = { $address } için yeni parola bekleniyor. Ardından gönderilir.
outbox-waiting-connection = Bağlantı bekleniyor. Yeniden çevrimiçi olduğunuzda gönderilir.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = hiç alıcısı yok
outbox-reason-address = gönderildiği adreslerden biri yok
outbox-reason-too-large = posta sunucusu için çok büyük
outbox-reason-blocked = posta sunucusu onu engelledi
outbox-reason-gone = bu bilgisayardaki kopyası kayboldu
outbox-reason-refused = posta sunucusu onu reddetti

## Buttons and notes

outbox-try-again = Yeniden dene
outbox-edit = Düzenle
outbox-delete = Sil
outbox-deleted = Giden Kutusu'ndan silindi
outbox-sending-again = Yeniden gönderiliyor…
outbox-snackbar-not-sent = “{ $subject }” gönderilmedi, çünkü { $reason }.
outbox-open = Giden Kutusu
