# Katna Mail, Turkish (Türkçe): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Notlar
notes-view-reminders = Hatırlatıcılar
notes-view-archive = Arşiv
notes-view-trash = Çöp kutusu
notes-edit-labels = Etiketleri düzenle
notes-search = Notlarda ara
notes-loading = Notlarınız açılıyor…

## Board

notes-take-a-note = Not al…
notes-new-list = Yeni liste
notes-new-note = Yeni not
notes-pinned = Sabitlenenler
notes-others = Diğerleri
notes-empty = Eklediğiniz notlar burada görünür
notes-archive-empty = Arşivlenen notlarınız burada görünür
notes-trash-empty = Çöp kutusunda not yok
notes-none-found = Eşleşen not yok
notes-label-empty = Bu etikete sahip not henüz yok
notes-reminders-empty = Yaklaşan hatırlatıcısı olan notlar burada görünür
notes-trash-note = Çöp kutusundaki notlar 7 gün sonra silinir.
notes-empty-trash = Çöp kutusunu boşalt
notes-ticked = { $count ->
    [one] + { $count } işaretli öğe
   *[other] + { $count } işaretli öğe
}
notes-select = Notu seç
notes-selected = { $count ->
   *[other] { $count } seçili
}
notes-select-clear = Seçimi temizle

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
notes-more = Diğer
notes-make-copy = Kopyasını oluştur
notes-remind = Bana hatırlat
notes-add-picture = Resim ekle
notes-history = Sürüm geçmişi
notes-ai = Yazmama yardım et
notes-send-as-mail = Posta olarak gönder
notes-save-markdown = Markdown olarak kaydet
notes-save-pdf = PDF olarak kaydet

## The open note

notes-title = Başlık
notes-edited = Düzenlenme: { $date }
notes-on-this-computer = Bu bilgisayarda
notes-where = Bu notun saklandığı yer
notes-untitled = Adsız not
notes-picture-choose = Resim ekle
notes-picture-remove = Resmi kaldır
notes-picture-too-big = Bir nota en fazla { $size } boyutunda resimler eklenebilir
notes-picture-kind = Bu dosya Katna'nın gösterebileceği bir resim değil
notes-picture-unreadable = { $name } okunamadı: { $error }
notes-remind-me = Bana hatırlat
notes-remind-off = Hatırlatıcıyı kaldır
notes-remind-in-the-past = Henüz geçmemiş bir zaman seçin
notes-remind-today = Bugün, { $time }
notes-remind-tomorrow = Yarın, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = Hatırlatıcı kuruldu: { $when }
notes-reminder-off = Hatırlatıcı kaldırıldı
notes-link-note = Not bağla
notes-link-new = Yeni not "{ $title }"
notes-linked-from = Şuradan bağlantı verilmiş
notes-link-gone = O not artık burada değil
notes-versions = Sürümler
notes-version-now = Şimdi
notes-version-here = Siz, bu bilgisayarda
notes-version-yesterday = Dün, { $time }
notes-version-changes = { $count ->
   *[other] { $count } değişiklik
}
notes-version-from = { $device } cihazından
notes-version-elsewhere = Başka bir cihazdan
notes-version-created = Oluşturuldu
notes-version-restore = Bu sürümü geri yükle
notes-version-restored = Sürüm geri yüklendi
notes-history-none = Henüz önceki sürüm yok
notes-ai-tidy = Metni düzenle
notes-ai-checklist = Yapılacaklar listesine dönüştür
notes-ai-summarise = Özetle
notes-ai-empty = Önce bir şey yazın
notes-ai-tidied = Metin düzenlendi. Ctrl+Z eski haline getirir.
notes-ai-listed = Yapılacaklar listesine dönüştürüldü. Ctrl+Z eski haline getirir.
notes-ai-summarised = Özet en üste eklendi

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

## Formatting

notes-format = Biçimlendirme
notes-format-heading-1 = Başlık 1
notes-format-heading-2 = Başlık 2
notes-format-normal = Normal metin
notes-format-bold = Kalın
notes-format-italic = İtalik
notes-format-underline = Altı çizili
notes-format-quote = Alıntı
notes-format-code = Kod
notes-format-divider = Ayırıcı
notes-format-clear = Biçimlendirmeyi temizle

## Tasks

notes-make-task = Göreve dönüştür

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
notes-saved = Not kaydedildi
notes-pinned-count = { $count ->
    [one] Not sabitlendi
   *[other] { $count } not sabitlendi
}
notes-unpinned-count = { $count ->
    [one] Notun sabitlemesi kaldırıldı
   *[other] { $count } notun sabitlemesi kaldırıldı
}
notes-colored-count = { $count ->
    [one] Renk değiştirildi
   *[other] { $count } notun rengi değiştirildi
}
notes-archived-count = { $count ->
    [one] Not arşivlendi
   *[other] { $count } not arşivlendi
}
notes-unarchived-count = { $count ->
    [one] Not arşivden çıkarıldı
   *[other] { $count } not arşivden çıkarıldı
}
notes-trashed-count = { $count ->
    [one] Not Çöp Kutusu'na taşındı
   *[other] { $count } not Çöp Kutusu'na taşındı
}
notes-restored-count = { $count ->
    [one] Not geri yüklendi
   *[other] { $count } not geri yüklendi
}
notes-copied-count = { $count ->
    [one] Kopya oluşturuldu
   *[other] { $count } kopya oluşturuldu
}
notes-empty-discarded = Boş not atıldı
notes-mail-gone = Bu e-posta artık burada değil
notes-deleted-forever = { $count ->
    [one] Not kalıcı olarak silindi
   *[other] { $count } not kalıcı olarak silindi
}
