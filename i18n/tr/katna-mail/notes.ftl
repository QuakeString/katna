# Katna Mail, Turkish (Türkçe): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Notlar
notes-view-archive = Arşiv
notes-view-trash = Çöp kutusu
notes-edit-labels = Etiketleri düzenle
notes-search = Notlarda ara
notes-loading = Notlarınız açılıyor…

## Board

notes-take-a-note = Not al…
notes-new-list = Yeni liste
notes-pinned = Sabitlenenler
notes-others = Diğerleri
notes-empty = Eklediğiniz notlar burada görünür
notes-archive-empty = Arşivlenen notlarınız burada görünür
notes-trash-empty = Çöp kutusunda not yok
notes-none-found = Eşleşen not yok
notes-label-empty = Bu etikete sahip not henüz yok
notes-trash-note = Çöp kutusundaki notlar 7 gün sonra silinir.
notes-empty-trash = Çöp kutusunu boşalt
notes-ticked = { $count ->
    [one] + { $count } işaretli öğe
   *[other] + { $count } işaretli öğe
}

## A note's buttons

notes-pin = Notu sabitle
notes-unpin = Notun sabitlemesini kaldır
notes-archive = Arşivle
notes-unarchive = Arşivden çıkar
notes-delete = Notu sil
notes-restore = Geri yükle
notes-delete-forever = Kalıcı olarak sil
notes-color = Arka plan rengi
notes-checkboxes = Onay kutularını göster veya gizle
notes-labels = Etiketler
notes-close = Kapat

## The open note

notes-title = Başlık
notes-edited = Düzenlenme: { $date }
notes-on-this-computer = Bu bilgisayarda
notes-where = Bu notun saklandığı yer

## Labels

notes-label-note = Nota etiket ekle
notes-label-name = Etiket adını girin
notes-label-create = “{ $name }” oluştur
notes-label-remove = Etiketi kaldır
notes-label-delete = Etiketi sil
notes-labels-none = Henüz etiket yok. Bir notun etiket düğmesinden ekleyin.
notes-labels-done = Bitti
notes-label-renamed = Etiket adı “{ $name }” olarak değiştirildi
notes-label-deleted = “{ $name }” etiketi silindi

## A note about a mail

notes-mail = Posta
notes-open-mail = E-postayı aç
notes-open-note = Notu aç

## Meeting notes

notes-meeting-take = Toplantı notları al
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = Katılımcılar: { $names }
notes-meeting-notes = Notlar
notes-meeting-actions = Eylem öğeleri
notes-event = Etkinlik
notes-open-event = Etkinliği aç

## Colors (tooltips)

notes-color-none = Renk yok
notes-color-coral = Mercan
notes-color-peach = Şeftali
notes-color-sand = Kum
notes-color-mint = Nane
notes-color-sage = Adaçayı
notes-color-fog = Sis
notes-color-storm = Fırtına
notes-color-dusk = Alacakaranlık
notes-color-blossom = Çiçek
notes-color-clay = Kil
notes-color-chalk = Tebeşir

## Messages at the foot of the window

notes-archived = Not arşivlendi
notes-unarchived = Not arşivden çıkarıldı
notes-trashed = Not çöp kutusuna taşındı
notes-restored = Not geri yüklendi
notes-empty-discarded = Boş not atıldı
notes-mail-gone = Bu e-posta artık burada değil
notes-deleted-forever = { $count ->
    [one] Not kalıcı olarak silindi
   *[other] { $count } not kalıcı olarak silindi
}
