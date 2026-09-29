# Katna Mail, Turkish (Türkçe): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Bugün
calendar-today-tip = Bugüne git
calendar-view-day = Gün
calendar-view-week = Hafta
calendar-view-month = Ay
calendar-view-year = Yıl
calendar-view-schedule = Program
calendar-view-days =
    { $count ->
        [one] { $count } gün
       *[other] { $count } gün
    }
calendar-options = Seçenekler
calendar-density = Yoğunluk
calendar-density-responsive = Ekranınıza duyarlı
calendar-density-comfortable = Rahat
calendar-density-compact = Kompakt
calendar-custom-days = Özel görünüm
calendar-second-zone = İkinci saat dilimi
calendar-zone-none = Yok
calendar-zone = { $zone } ({ $offset })
calendar-share-free = Boş zamanları paylaş
calendar-free-subject = Boş olduğum zamanlar
calendar-free-intro = İşte boş olduğum bazı zamanlar ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = Önümüzdeki birkaç iş gününde boş zamanım yok.
calendar-previous-day = Önceki gün
calendar-next-day = Sonraki gün
calendar-previous-week = Önceki hafta
calendar-next-week = Sonraki hafta
calendar-previous-month = Önceki ay
calendar-next-month = Sonraki ay
calendar-previous-year = Önceki yıl
calendar-next-year = Sonraki yıl
calendar-previous-period = Daha önce
calendar-next-period = Daha sonra
calendar-title-months = { $first } – { $last }
calendar-loading = Yükleniyor…
calendar-read-failed = Takvim okunamadı: { $error }
calendar-sets = Takvim kümeleri
calendar-set-add = Gösterilen takvimleri küme olarak kaydet
calendar-set-name = Kümenin adı
calendar-set-remove = Kümeyi kaldır
calendar-local = Bu bilgisayar
calendar-account-gone = Kaldırılan hesap
calendar-birthdays = Doğum günleri
calendar-birthday-of = { $name } doğum günü
calendar-empty-title = Henüz takvim yok
calendar-empty-text = Katna, Google ve Microsoft hesaplarınızın takvimlerini eşitlendikten sonra, ayrıca CalDAV sunan diğer sunucuların takvimlerini burada gösterir.
calendar-schedule-empty = Önümüzdeki iki ay için planlanmış bir şey yok.
calendar-search = Etkinliklerde ara
calendar-search-past = Geçmiş etkinlikler
calendar-search-none = Aramanızla eşleşen etkinlik yok.
calendar-no-title = (Başlık yok)
calendar-all-day = Tüm gün
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } tane daha
calendar-repeats = Tekrarlanır
calendar-join = Katıl
calendar-email-guests = Konuklara posta gönder
calendar-running-late = Geç kalıyorum
calendar-late-subject = Geç kalıyorum: { $title }
calendar-late-body = Kusura bakmayın, { $title } için birkaç dakika geç kalacağım. Birazdan orada olacağım.
calendar-guests =
    { $count ->
        [one] { $count } konuk
       *[other] { $count } konuk
    }
calendar-guest-answers = { $yes } evet, { $maybe } belki, { $no } hayır, { $waiting } yanıt bekliyor
calendar-organizer = Organizatör
calendar-optional = İsteğe bağlı
calendar-open-web = Tarayıcıda aç
calendar-open-contact = Kişiyi aç
calendar-close = Kapat

## Adding, changing and deleting events.

calendar-add-title = Başlık ekle
calendar-add-location = Konum ekle
calendar-add-notes = Açıklama ekle
calendar-add-guests = Konuk ekle
calendar-remove-guest = Kaldır
calendar-add-meet = Google Meet görüntülü görüşmesi ekle
calendar-add-teams = Teams toplantısı ekle
calendar-has-call = Görüntülü görüşme eklendi
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Tüm gün
calendar-more-options = Diğer seçenekler
calendar-save = Kaydet
calendar-saved = Etkinlik kaydedildi
calendar-deleted = Etkinlik silindi
calendar-discard = Değişiklikleri at
calendar-edit = Etkinliği düzenle
calendar-delete = Etkinliği sil
calendar-event-details = Etkinlik ayrıntıları
calendar-kind-event = Etkinlik
calendar-kind-focus = Odaklanma zamanı
calendar-kind-out-of-office = Ofis dışında
calendar-kind-working-location = Çalışma konumu
calendar-working-home = Ev
calendar-busy = Meşgul
calendar-free = Müsait
calendar-cancel = İptal
calendar-ok = Tamam
calendar-read-only = Bu takvimdeki etkinlikleri değiştiremezsiniz
calendar-none-editable = Henüz etkinlik ekleyebileceğiniz bir takvim yok
calendar-no-such-time = Bu saat saat diliminizde yok
calendar-end-before-start = Etkinlik başlamadan önce bitiyor
calendar-repeat-never = Tekrarlanmaz
calendar-repeat-daily = Günlük
calendar-repeat-weekly = Haftalık: { $weekday } günleri
calendar-repeat-monthly =
    { $nth ->
        [1] Aylık: ayın ilk { $weekday } günü
        [2] Aylık: ayın ikinci { $weekday } günü
        [3] Aylık: ayın üçüncü { $weekday } günü
        [4] Aylık: ayın dördüncü { $weekday } günü
       *[other] Aylık: ayın son { $weekday } günü
    }
calendar-repeat-yearly = Yıllık: { $day }
calendar-repeat-weekdays = Her hafta içi gün (Pazartesi - Cuma)
calendar-repeat-custom = Özel
calendar-reminder-none = Bildirim yok
calendar-reminder-at-start = Başlangıçta
calendar-reminder-minutes =
    { $count ->
        [one] { $count } dakika önce
       *[other] { $count } dakika önce
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } saat önce
       *[other] { $count } saat önce
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } gün önce
       *[other] { $count } gün önce
    }
calendar-scope-edit-title = Yinelenen etkinliği düzenle
calendar-scope-delete-title = Yinelenen etkinliği sil
calendar-scope-this = Bu etkinlik
calendar-scope-following = Bu ve sonraki etkinlikler
calendar-scope-all = Tüm etkinlikler
calendar-scope-respond-title = Yinelenen etkinlik için yanıt
calendar-going = Katılıyor musunuz?
calendar-answer-yes = Evet
calendar-answer-no = Hayır
calendar-answer-maybe = Belki
calendar-answered-yes = Katılıyorsunuz
calendar-answered-no = Katılmıyorsunuz
calendar-answered-maybe = Belki katılırsınız

## The card at the top of a mail with an invitation.

calendar-invite = Davet
calendar-invite-cancelled = Etkinlik iptal edildi
calendar-invite-reply = { $name } yanıtladı
calendar-invite-reply-yes = { $name } kabul etti
calendar-invite-reply-no = { $name } reddetti
calendar-invite-reply-maybe = { $name } belki katılacak
calendar-invite-organizer = Organizatör: { $name }
calendar-invite-open = Takvim’de aç
calendar-invite-not-yet = Henüz takviminizde değil. Eşitlendikten sonra yanıt verebilirsiniz.
calendar-invite-by-mail = Takviminizde yok: yanıtınız düzenleyiciye e-postayla gönderilir.
calendar-mail-yes = Kabul edildi: { $title }
calendar-mail-yes-body = { $name } bu daveti kabul etti.
calendar-mail-no = Reddedildi: { $title }
calendar-mail-no-body = { $name } bu daveti reddetti.
calendar-mail-maybe = Geçici olarak kabul edildi: { $title }
calendar-mail-maybe-body = { $name } bu daveti geçici olarak kabul etti.
calendar-invite-your-day = Gününüz
calendar-invite-clashes =
    { $count ->
        [one] { $count } etkinlikle çakışıyor
       *[other] { $count } etkinlikle çakışıyor
    }

## The day's agenda beside the mail.

agenda-show = Günün ajandasını göster
agenda-hide = Ajandayı gizle
agenda-today = Bugün, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = Bu gün için planlanmış bir şey yok.
