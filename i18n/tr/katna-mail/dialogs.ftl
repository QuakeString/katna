# Katna Mail, Turkish (Türkçe).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Katna hakkında
about-tagline = Linux masaüstü için posta ve takvim
about-whats-new = Yenilikler
about-update-not-checked = Güncellemeler henüz denetlenmedi
about-update-checking = Güncellemeler denetleniyor…
about-update-up-to-date = Katna Mail güncel
about-update-check-failed = Güncellemeler denetlenemedi
about-update-available = { $version } sürümü kullanılabilir
about-update-downloading = { $version } sürümü indiriliyor… { $percent }%
about-update-download-failed = { $version } sürümünün indirilmesi tamamlanamadı
about-update-ready = { $version } sürümü kurulmaya hazır
about-update-ready-detail = Güncellemeyi tamamlamak için Katna Mail yeniden başlar.
about-update-confirm = { $version } sürümü kurulsun mu?
about-update-confirm-detail = Katna Mail kapanacak, güncellemeyi kuracak ve kaldığınız yerden yeniden açılacak. Bilgisayarınız parolanızı isteyecek.
about-update-installing = { $version } sürümü kuruluyor…
about-update-installing-detail = Açılan penceredeki parolanızı girin.
about-update-cancelled = Parola verilmediği için güncelleme kurulmadı.
about-update-failed = Güncelleme kurulamadı: { $error }
about-update-unsupported = Katna Mail'in bu kopyası paket yöneticiniz tarafından güncellenir.
about-update-restart-failed = Güncelleme kuruldu, ancak Katna Mail yeniden açılamadı ({ $error }). Kendiniz açın.
about-update-check = Güncellemeleri denetle
about-update-download = İndir
about-update-retry = Yeniden dene
about-update-button = Güncelle
about-update-restart = Güncelle ve yeniden başlat
about-update-cancel = Şimdi değil
about-changelog = Değişiklik günlüğü
about-source = Kaynak kodu
about-coffee = Bana bir kahve ısmarla
about-coming-soon = Çok yakında
about-follow-me = Beni takip edin:
about-love-title = Rust, KDE ve Linux sevgisiyle yapıldı
about-love-text = Rust, hızlı ve güvenli bir posta uygulaması yazmayı keyifli kılıyor: Katna'da hiç unsafe kod yok. KDE'nin Plasma masaüstü ve PIM paketi Katna'ya ilham verdi; Linux ve özgür yazılım topluluğu da üzerinde durduğu zemini inşa ediyor. Teşekkürler, aşağıdaki kütüphanelere de teşekkürler.
about-kde-text = KDE, Katna'nın kendini en çok evinde hissettiği masaüstünü geliştiriyor; gönüllüler tarafından yapılıyor ve sizin gibi insanlar tarafından destekleniyor. Plasma'yı veya KDE uygulamalarını seviyorsanız lütfen KDE'ye bağış yapmayı düşünün.
about-donate-kde = KDE'ye bağış yap
about-gpui-title = Zed projesinden GPUI üzerine kurulu
about-gpui-text = Katna Mail'in tüm arayüzü, Zed Industries'in Zed düzenleyicisi için geliştirdiği hızlı, GPU hızlandırmalı arayüz çatısı GPUI üzerine kuruludur. Gördüğünüz her piksel, animasyon ve pencere onunla çizilir. Onu açık şekilde geliştirdiğiniz için teşekkürler, Zed ekibi. Apache-2.0.
about-gpui-github = GitHub'da GPUI
about-personal-title = Kişisel bir proje
about-personal-text = Katna Mail yeni ya da devrimci olmaya çalışmıyor. Geliştiricisinin istediği posta uygulaması bu; özellikleri ve görünümü Gmail, Mailspring ve Thunderbird'den ödünç alındı. Ancak LLM'lerin bu kadar ilerlemesi sayesinde mümkün oldu.
about-built-on = ÖZGÜR YAZILIM ÜZERİNE KURULU
about-credit-pimalaya = IMAP, SMTP ve oturum açma (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = IMAP okuma ve yazma
about-credit-tantivy = Arama
about-credit-sqlite = Posta deposu
about-credit-rustls = Güvenli bağlantılar
about-credit-mail-parser = Posta okuma, Stalwart Labs'ten
about-credit-html5ever = HTML posta, Servo projesinden
about-credit-zbus = Masaüstüyle D-Bus ve portallar üzerinden iletişim
about-credit-oo7 = Masaüstü anahtarlığındaki parolalar
about-credit-hayro = PDF görüntüleme ve yazdırma
about-credit-calamine = Elektronik tablo önizlemeleri
about-credit-resvg = SVG resimleri
about-credit-jiff = Tarihler ve saat dilimleri
about-credit-spellbook = Yazım denetimi, Helix düzenleyicisinden
about-credit-smol = Aynı anda birçok iş yapma
about-all-libraries = Katna'nın kullandığı tüm kütüphaneler ({ $count })
about-library-authors = Yazarlar: { $authors }
about-license = Katna, GNU GPL sürüm 3 veya sonrası altında özgür yazılımdır.
about-close = Kapat

## What’s new (shown after an update)

whats-new-title = Katna Mail'deki yenilikler
whats-new-updated = { $version } sürümüne güncellendi
whats-new-version = Sürüm { $version }
whats-new-more = { $count ->
    [one] Tam değişiklik günlüğünde bir tane daha var.
   *[other] Tam değişiklik günlüğünde { $count } tane daha var.
}
whats-new-changelog = Tam değişiklik günlüğü
whats-new-got-it = Anladım

## First run: welcome page

onboarding-welcome-title = Katna Mail'e hoş geldiniz
onboarding-welcome-lead = Postalarınız kendi bilgisayarınızda: hızlı aranır, çevrimdışı okunur ve gizli kalır.
onboarding-fast-title = Çevrimdışıyken bile hızlı
onboarding-fast-text = Katna postalarınızın bir kopyasını burada tutar; böylece bağlantı olsun olmasın, açmak ve aramak anında gerçekleşir.
onboarding-providers-title = Postanızla çalışır
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud ve diğer tüm IMAP veya POP hesapları.
onboarding-private-title = Gizli
onboarding-private-text = Postalarınız sağlayıcınızdan doğrudan bu bilgisayara gelir. Hiçbir Katna sunucusu onları görmez.
onboarding-get-started = Başlayın

## First run: adding an account

onboarding-service-checking = Katna arka plan hizmeti denetleniyor…
onboarding-service-running = Katna arka plan hizmeti çalışıyor.
onboarding-service-missing = Katna arka plan hizmeti çalışmıyor
onboarding-service-start = Postalarınızı alır ve gönderir. Bir terminalden başlatın, sonra yeniden denetleyin:
onboarding-check-again = Yeniden denetle
onboarding-account-title = Posta hesabınızı ekleyin
onboarding-account-lead = E-posta adresinizi ve parolanızı yazın, Katna sunucu ayarlarını bulsun. Gmail, Yahoo ve iCloud, hesabınızın güvenlik ayarlarında oluşturulan bir uygulama parolası ister.
onboarding-add-account = Hesap ekle
onboarding-back = Geri

## First run: choosing the look

onboarding-look-title = Kendinize göre ayarlayın
onboarding-look-lead = Postanın nasıl açılacağını ve Katna'nın nasıl görüneceğini seçin. Bunları istediğiniz zaman hızlı ayarlardan değiştirebilirsiniz.
onboarding-reading-pane = Okuma bölmesi
onboarding-pane-right = Listenin sağında
onboarding-pane-none = Bölme yok
onboarding-theme = Tema
onboarding-theme-system = Sistem
onboarding-theme-light = Açık
onboarding-theme-dark = Koyu
onboarding-density = Yoğunluk
onboarding-density-default = Varsayılan
onboarding-density-compact = Sıkışık
onboarding-continue = Devam

## First run: done

onboarding-ready-title = Her şey hazır
onboarding-ready-lead = Katna postalarınızı alıyor. Geldikçe görünürler ve yeni postalar kendiliğinden belirir.
onboarding-ready-lead-address = Katna, { $address } adresinin postalarını alıyor. Geldikçe görünürler ve yeni postalar kendiliğinden belirir.
onboarding-ready-tour = Her şeyin nerede olduğunu görmek için bir dakikalık tura ne dersiniz?
onboarding-skip = Şimdilik atla
onboarding-take-tour = Turu başlat

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Katna'nın geliştirilmesine yardımcı olun
share-lead = Katna çöktüğünde bu bilgisayara bir rapor kaydeder. Bu raporları göndermek, sorunun düzeltilmesine yardımcı olur. Bunu istediğiniz zaman Ayarlar > Kullanıcı geri bildirimi bölümünden değiştirebilirsiniz.
share-sent = Ne gönderilir
share-sent-detail = Ayarlar'da görebileceğiniz haliyle çökme raporu: Katna'da neyin nerede çöktüğü, sürüm, Linux sisteminiz ve masaüstünüz ve Katna'nın posta klasörlerinin adlarını içerebilen son günlük satırları.
share-never-sent = Asla gönderilmeyenler
share-never-sent-detail = İletileriniz, kişileriniz, parolalarınız, IP adresiniz, kullanıcı adınız veya bilgisayar adınız. E-posta adresleri rapordan çıkarılır.
share-where = Nereye gider
share-where-detail = Katna'nın Sentry'deki çökme izleyicisine; AB'de saklanır. Raporları size bağlayan hiçbir kimlik yoktur.
share-dont-send = Gönderme
share-send = Çökme raporlarını gönder
share-sending = Çökme raporları gönderilecek. Teşekkürler.
share-local = Çökme raporları bu bilgisayarda kalır.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Katna Mail'e hoş geldiniz
tour-welcome-text = Bir dakikalık tur her şeyin nerede olduğunu gösterir.
tour-not-now = Şimdi değil
tour-start = Turu başlat
tour-close = Kapat
tour-skip = Turu atla
tour-back = Geri
tour-done = Bitti
tour-next = İleri
tour-step = { $step } / { $total }
tour-compose-title = İleti yazın
tour-compose-text = Oluştur, sağ altta yeni bir ileti açar; böylece yazarken okumaya devam edebilirsiniz.
tour-search-title = Tüm postalarınızda arayın
tour-search-text = Arama çevrimdışı da çalışır. Sağ uçtaki düğme filtre ekler: gönderen, alıcı, konu, tarihler ve ekler.
tour-menu-title = Klasörleri göster veya gizle
tour-menu-text = Bu düğme klasör listesini katlar. Gizliyken klasörleri görmek için işaretçiyi soldaki Posta'nın üzerinde tutun.
tour-apps-title = Uygulamalarınız
tour-apps-text = Posta artık burada. Takvim, Kişiler, Görevler, Notlar ve Akışlar da bu çubukta ona katılacak.
tour-tabs-title = Gelen Kutusu sekmeleri
tour-tabs-text = Yeni postalar Birincil, Tanıtımlar, Sosyal, Güncellemeler ve Forumlar olarak sıralanır. Sekmeleri hızlı ayarlardan kapatabilirsiniz.
tour-list-title = İletileriniz
tour-list-text = Okumak için bir iletiye tıklayın. Hızlı işlemler için üzerine gelin, daha fazlası için sağ tıklayın veya birlikte işlem yapmak için birkaçını işaretleyin.
tour-settings-title = Hızlı ayarlar
tour-settings-text = Okuma bölmesini, yoğunluğu ve temayı buradan değiştirin. Tur da oradan yeniden başlatılabilir.
tour-account-title = Hesabınız
tour-account-text = Hangi hesapta olduğunuzu görün ve başka bir hesap ekleyin.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Katna'nın arka plan hizmeti beklenmedik şekilde durdu.
    [one] Katna'nın arka plan hizmeti beklenmedik şekilde durdu. Bir çökme raporu daha kayıtlı.
   *[other] Katna'nın arka plan hizmeti beklenmedik şekilde durdu. { $more } çökme raporu daha kayıtlı.
}
crash-mail = { $more ->
    [0] Katna Mail geçen sefer beklenmedik şekilde kapandı.
    [one] Katna Mail geçen sefer beklenmedik şekilde kapandı. Bir çökme raporu daha kayıtlı.
   *[other] Katna Mail geçen sefer beklenmedik şekilde kapandı. { $more } çökme raporu daha kayıtlı.
}
crash-view = Raporu görüntüle
crash-view-tooltip = Bu bilgisayarda kayıtlı raporu aç
crash-copy = Raporu kopyala
crash-close = Kapat
sign-in-again-text = { $provider }, { $address } hesabında yeniden oturum açmanızı istiyor.
sign-in-again-button = Oturum aç
sign-in-again-tooltip = { $provider } oturum açma sayfasını tarayıcınızda aç
sign-in-again-waiting = Tarayıcınız bekleniyor…
sign-in-again-close = Kapat
sign-in-again-done = { $address } hesabında yeniden oturum açıldı. Postalarınız alınıyor…
delete-ask-title = { $kind ->
    [conversation] { $count ->
        [one] İleti dizisi Çöp Kutusu'na taşınsın mı?
       *[other] { $count } ileti dizisi Çöp Kutusu'na taşınsın mı?
    }
   *[message] { $count ->
        [one] İleti Çöp Kutusu'na taşınsın mı?
       *[other] { $count } ileti Çöp Kutusu'na taşınsın mı?
    }
}
delete-ask-body = { $count ->
    [one] Hemen ardından geri alabilir ya da daha sonra Çöp Kutusu'ndan geri getirebilirsiniz.
   *[other] Hemen ardından geri alabilir ya da daha sonra onları Çöp Kutusu'ndan geri getirebilirsiniz.
}
delete-ask-confirm = Çöp Kutusu'na taşı
delete-forever-title = { $kind ->
    [conversation] { $count ->
        [one] İleti dizisi kalıcı olarak silinsin mi?
       *[other] { $count } ileti dizisi kalıcı olarak silinsin mi?
    }
   *[message] { $count ->
        [one] İleti kalıcı olarak silinsin mi?
       *[other] { $count } ileti kalıcı olarak silinsin mi?
    }
}
delete-forever-body = { $count ->
    [one] Sunucudan da silinir. Bu geri alınamaz.
   *[other] Sunucudan da silinirler. Bu geri alınamaz.
}
delete-forever-confirm = Kalıcı olarak sil
delete-ask-dont-ask = Bir daha sorma
delete-ask-cancel = İptal
