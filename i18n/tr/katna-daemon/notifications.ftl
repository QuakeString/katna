# Katna Mail, Turkish (Türkçe).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count } yeni e-posta
notify-and-more = ve { $count } tane daha
notify-no-subject = (konu yok)
notify-unknown-sender = Bilinmeyen gönderen
notify-snooze-back = Ertelemeden dönenler
notify-no-reply = Henüz yanıt yok
notify-no-reply-to = “{ $subject }” iletisine kimse yanıt vermedi.
notify-tracking-opened = { $who }, { $subject } iletisini açtı
notify-tracking-clicked = { $who }, { $subject } iletisindeki bir bağlantıya tıkladı

## Its buttons

notify-update-ready = Katna Mail güncellenebilir
notify-update-ready-body = { $version } sürümü indirildi. Güncelleme onu kurar ve Katna Mail'i yeniden başlatır.
notify-update = Güncelle
notify-event-now = Şimdi
notify-event-in-minutes = { $count ->
   *[other] { $count } dakika sonra
}
notify-event-in-hours = { $count ->
   *[other] { $count } saat sonra
}
notify-event-in-days = { $count ->
    [1] Yarın
   *[other] { $count } gün sonra
}
notify-event-all-day = Tüm gün
notify-event-join = Katıl
notify-event-snooze = 5 dk ertele
notify-task-done = Tamamlandı olarak işaretle
notify-open = Aç
notify-peek = Göz at
notify-reply = Yanıtla
notify-reply-placeholder = { $name } kişisine yanıt yazın…
notify-send = Gönder
notify-reply-all = Tümünü yanıtla
notify-mark-read = Okundu olarak işaretle
notify-mark-all-read = Tümünü okundu olarak işaretle
notify-archive = Arşivle
notify-archived = Arşivlendi
notify-archived-count = { $count ->
    [one] { $count } ileti gelen kutusundan çıkarıldı
   *[other] { $count } ileti gelen kutusundan çıkarıldı
}
notify-undo = Geri al
notify-reply-sent = Yanıt { $name } kişisine gönderildi
notify-open-in-katna = Katna'da aç
