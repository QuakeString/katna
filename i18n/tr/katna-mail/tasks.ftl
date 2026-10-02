# Katna Mail, Turkish (Türkçe): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Yeni görev
tasks-all = Tüm görevler
tasks-today = Bugün
tasks-starred = Yıldızlı
tasks-new-list = Yeni liste oluştur
tasks-on-this-computer = Bu bilgisayarda
tasks-my-tasks = Görevlerim
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Görevleri göstermek için yeniden oturum aç
tasks-account-signed-in = { $address } hesabında yeniden oturum açıldı. Görevleriniz alınıyor…
tasks-account-sign-in-refused = { $provider }, Katna'nın girişine izin vermedi. Tekrar deneyin ve görevlerinize erişime izin verin.
tasks-account-refused = Sunucu parolayı kabul etmedi. Yahoo, iCloud, Zoho ve diğerleri bir uygulama parolası gerektirir.
tasks-account-change-password = Parolayı değiştir
tasks-account-change-password-tooltip = Ayarlar > Hesaplar'ı aç
tasks-account-not-enabled = Katna için görev erişimi henüz açılmadı.
tasks-account-failed = Görev listeleri okunamadı.
# $reason is the server's own words, in English.
tasks-account-error = Görev listeleri okunamadı: { $reason }
tasks-account-none = Görev listesi bulunamadı
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = Görev listesi bulunamadı: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider }, görevleri yalnızca { $provider } ile oturum açmış Katna'ya gösterir.
tasks-account-sign-in-with = { $provider } ile oturum aç
tasks-account-looking = Görev listeleri aranıyor…
tasks-account-try-again = Tekrar dene
tasks-account-try-again-tooltip = Bu hesabın görevlerini şimdi yeniden denetle
tasks-account-fixing = Üzerinde çalışılıyor…
tasks-list-name-placeholder = Liste adı

## Lists and tasks

tasks-loading = Görevleriniz okunuyor…
tasks-no-lists = Görev listeleriniz burada görünür.
tasks-search = Görevlerde ara
tasks-search-none = Aramanızla eşleşen görev yok.
tasks-add = Görev ekle
tasks-title-placeholder = Başlık
tasks-add-step = Alt görev ekle
tasks-empty = Henüz görev yok. Yukarıdan bir tane ekleyin.
tasks-starred-empty = Burada görmek için bir göreve yıldız ekleyin.
tasks-today-empty = Bugün son tarihi gelen bir şey yok.
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Gecikmiş
tasks-completed = { $count ->
    [one] Tamamlananlar ({ $count })
   *[other] Tamamlananlar ({ $count })
}
tasks-list-options = Liste seçenekleri
tasks-rename-list = Listeyi yeniden adlandır
tasks-delete-list = Listeyi sil
tasks-mark-done = Tamamlandı olarak işaretle
tasks-mark-open = Tamamlanmadı olarak işaretle
tasks-star = Yıldız ekle
tasks-unstar = Yıldızı kaldır
tasks-edit-title = Başlığı düzenle
tasks-details = Ayrıntılar
tasks-delete = Sil
tasks-move-to = { $list } listesine taşı
tasks-from-mail = Posta
tasks-open-mail = E-postayı aç
tasks-from-note = Not
tasks-open-note = Notu aç
tasks-note-gone = O not artık burada değil.
tasks-no-subject = (konu yok)

## The details dialog

tasks-notes-placeholder = Ayrıntı ekle
tasks-date = Tarih
tasks-no-date = Tarih yok
tasks-time-placeholder = Saat ekle
tasks-repeat = Tekrarla
tasks-repeat-never = Tekrarlanmaz
tasks-repeat-daily = Günlük
tasks-repeat-weekly = Haftalık
tasks-repeat-monthly = Aylık
tasks-repeat-yearly = Yıllık
tasks-repeat-other = Özel
tasks-remind = Bana hatırlat
tasks-remind-off = Hatırlatma
tasks-remind-on-time = Görev zamanında
tasks-remind-morning = O gün, { $time }
tasks-remind-hour-before = Bir saat önce
tasks-remind-day-before = Bir gün önce
tasks-cancel = İptal
tasks-save = Kaydet
tasks-not-a-time = “{ $text }” bir saat değil, örneğin { $example }.

## Due days

tasks-due-today = Bugün
tasks-due-tomorrow = Yarın
tasks-due-yesterday = Dün
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Görev tamamlandı
tasks-toast-next = Bitti. Sonraki: { $date }
tasks-toast-deleted = Görev silindi
tasks-toast-added = { $count ->
    [one] Görevlere eklendi
   *[other] { $count } görev eklendi
}
tasks-mail-gone = Bu e-posta artık burada değil.
tasks-toast-list-deleted = Liste silindi
tasks-toast-moved = { $list } listesine taşındı
# A task dragged to another place in its own list.
tasks-toast-placed = Görev taşındı
tasks-toast-rescheduled = Görev yeniden zamanlandı
