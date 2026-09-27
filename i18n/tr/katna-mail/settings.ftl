# Katna Mail, Turkish (Türkçe).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings page: its tabs

settings-tab-general = Genel
settings-tab-inbox = Gelen Kutusu
settings-tab-accounts = Hesaplar
settings-tab-subscriptions = Abonelikler
settings-tab-appearance = Görünüm
settings-tab-shortcuts = Kısayollar
settings-tab-default-apps = Varsayılan uygulamalar
settings-tab-folders-rules = Klasörler ve kurallar
settings-tab-compose = Oluşturma
settings-tab-mcp-server = MCP sunucusu
settings-tab-feedback = Kullanıcı geri bildirimi
settings-tab-experimental = Deneysel

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Aldığınız bültenleri ve e-posta listelerini görün, tek tıklamayla abonelikten çıkın.
settings-tab-folders-rules-coming = Klasörleri ve etiketleri oluşturun, yeniden adlandırın, taşıyın ve gizleyin; hangilerinin senkronize edileceğini seçin. Kurallar yeni postaları gönderene, konuya veya kelimelere göre kendiliğinden sıralar, etiketler, yönlendirir ya da siler.
settings-tab-mcp-server-coming = Bu bilgisayardaki yapay zekâ asistanlarının, sizin onayınızla postalarınızda arama yapmasına, onları okumasına ve taslak yazmasına izin verin.

## Settings > General

settings-general-conversations = İleti dizisi görünümü
settings-general-conversations-group = Aynı postaya verilen yanıtları grupla
settings-general-conversations-group-detail = Listede her ileti dizisi için bir satır
settings-general-reading = Okuma
settings-general-newest-first = Önce en yeni ileti
settings-general-newest-first-detail = İleti dizisi en son yanıtla başlar
settings-general-full-headers = Tüm üst bilgileri göster
settings-general-full-headers-detail = Kimden, kime, bilgi, tarih ve konu her iletide açık olur
settings-general-full-names = Alıcıların tam adları
settings-general-full-names-detail = “alıcı: ben, Ada” yerine “alıcı: ben, Ada Lovelace”
settings-general-mark-read = Okundu olarak işaretle
settings-general-mark-read-now = Açılır açılmaz
settings-general-mark-read-1s = 1 saniye açık kaldıktan sonra
settings-general-mark-read-3s = 3 saniye açık kaldıktan sonra
settings-general-mark-read-never = Yalnızca ben okundu olarak işaretlediğimde
settings-general-reply-button = Yanıtla düğmesi
settings-general-reply-all = Herkesi yanıtla
settings-general-reply-all-detail = Her iletinin yanındaki yanıtla düğmesi yalnızca gönderene değil, herkese yanıt verir
settings-general-remote-images = Web'deki resimler
settings-general-remote-images-detail = Bir iletinin resimlerini yüklemek, gönderene iletiyi açtığınızı, ne zaman ve yaklaşık nerede açtığınızı bildirir. Kapalıyken her ileti önce sorar; bir gönderenin resimlerini her zaman gösterebilirsiniz.
settings-general-remote-images-always = Resimleri her zaman göster
settings-general-remote-images-always-detail = Yalnızca güvendiğiniz gönderenlerden gelenlerde değil, her iletide
settings-general-sending = Gönderme
settings-general-sending-detail = Gönderilen bir iletinin geri alınabilmesi için ne kadar bekleyeceği.
settings-general-offline = Çevrimdışı posta
settings-general-offline-detail = Son postalar, bağlantı olmadan okunabilmesi için tamamen indirilir. Daha eski postalar açtığınızda indirilir.
settings-general-offline-days = { $count ->
    [one] { $count } gün
   *[other] { $count } gün
}
settings-general-offline-years = { $count ->
    [one] { $count } yıl
   *[other] { $count } yıl
}
settings-general-offline-all = Tüm postalar
settings-general-offline-note = Daha az gün seçmek, önceden indirilmiş postaları korur. Sunucuda hiçbir şey değişmez.
settings-general-notifications = Bildirimler
settings-general-notifications-detail = Gelen Kutusu'ndaki yeni postalar için, Katna Mail kapalıyken bile.
settings-general-new-mail = Yeni postalar için bana bildir
settings-general-new-mail-detail = Tümünü yanıtla, Okundu olarak işaretle ve Arşivle düğmeleriyle
settings-general-new-mail-sound = Ses çal
settings-general-new-mail-sound-detail = Masaüstünün yeni posta sesi
settings-general-desktop = Masaüstü
settings-general-start-at-login = Oturum açıldığında Katna'yı başlat
settings-general-start-at-login-detail = Pencereyi açmadan postaları eşitler, yeni posta bildirimlerini ve tepsi simgesini gösterir
settings-general-login-window = Katna Mail penceresini de aç
settings-general-login-window-detail = Pencere de oturum açıldığında açılır
settings-general-tray = Katna'yı sistem tepsisinde göster
settings-general-tray-detail = Okunmamış sayısı ve bir menüyle
settings-general-unread-badge = Görev çubuğu simgesinde okunmamış sayısı
settings-general-unread-badge-detail = Gelen Kutusu'nda kaç iletinin okunmadığı

## Settings > Inbox

settings-inbox-tabs = Gelen Kutusu sekmeleri
settings-inbox-tabs-detail = Gelen kutusunu, posta sağlayıcınızın web sitesinin yaptığı gibi sekmelere ayırın.
settings-inbox-tabs-show = Gelen Kutusu sekmelerini göster
settings-inbox-tabs-show-detail = Kapalıyken her hesap için tek bir liste gösterilir
settings-inbox-no-accounts = Sekmelerini seçmek için bir hesap ekleyin.
settings-inbox-tabs-automatic = Otomatik: { $tabs } ({ $provider })
settings-inbox-tabs-off = Sekme yok
settings-inbox-tabs-gmail = Birincil, Tanıtımlar, Sosyal, Güncellemeler, Forumlar
settings-inbox-tabs-focused = Odaklanmış ve Diğer
settings-inbox-tabs-zoho = Gelen Kutusu, Bültenler ve Bildirimler
settings-inbox-tabs-shown = Gösterilen sekmeler. Kapattığınız bir sekmenin postaları { $tab } sekmesinde kalır.

## Settings > Appearance

settings-appearance-reading-pane = Okuma bölmesi
settings-appearance-reading-pane-detail = Açılan ileti dizisinin gösterildiği yer.
settings-appearance-pane-right = Listenin sağında
settings-appearance-pane-none = Bölme yok
settings-appearance-density = Yoğunluk
settings-appearance-density-default = Varsayılan
settings-appearance-density-compact = Sıkışık
settings-appearance-scaling = Ölçekleme
settings-appearance-scaling-detail = Masaüstünün kendi ölçeğinin üzerine Katna Mail'deki her şeyi büyütür veya küçültür: metin, simgeler, boşluklar ve ayırıcılar. Gönderdiğiniz postalar kendi yazı tipi boyutunu korur. Çok küçük boyutlar simgelere tıklamayı zorlaştırabilir.
settings-appearance-theme = Tema
settings-appearance-theme-system = Sistem
settings-appearance-theme-light = Açık
settings-appearance-theme-dark = Koyu
settings-appearance-desktop-colors = Masaüstü renkleri
settings-appearance-desktop-colors-use = Masaüstünün renklerini kullan
settings-appearance-desktop-colors-use-detail = Masaüstünün renk şeması ve vurgu rengi
settings-appearance-app-names = Uygulama adları
settings-appearance-app-names-show = Uygulama adlarını göster
settings-appearance-app-names-show-detail = En soldaki uygulama simgelerinin altında adlar
settings-appearance-sender-pictures = Gönderen resimleri
settings-appearance-sender-pictures-show = Şirket logolarını göster
settings-appearance-sender-pictures-show-detail = İletiye göre değil, yalnızca gönderenin alan adına göre aranır ve bir hafta saklanır
settings-appearance-important = Önemli işaretçileri
settings-appearance-important-show = Önemli işaretçilerini göster
settings-appearance-important-show-detail = Listede her iletinin yanında
settings-appearance-message-width = İleti genişliği
settings-appearance-message-width-limit = İletilerin genişliğini sınırla
settings-appearance-message-width-limit-detail = Geniş bir pencerede uzun satırlar böylece daha kolay okunur
settings-appearance-mail-colors = Posta renkleri
settings-appearance-mail-colors-detail = Çoğu posta beyaz bir sayfa için tasarlanır. Koyu temada renkleri, iyi okunan koyu renklerle değiştirilir; kapalıyken posta, gönderenin renklerini açık bir sayfada korur.
settings-appearance-dark-mail = Postalar için de koyu renkler
settings-appearance-dark-mail-detail = Yalnızca tema koyuyken
settings-appearance-attachment-previews = Ek önizlemeleri
settings-appearance-attachment-previews-show = Eklerin önizlemelerini göster
settings-appearance-attachment-previews-show-detail = Kartında her dosyanın içeriğinin küçük bir resmi

## Settings > Default apps

settings-default-apps-intro = Eklere tıkladığınızda açıldıkları yer. Görüntüleyici bir dosyayı her zaman başka bir uygulamada da açabilir. Masaüstünün varsayılan uygulamaları kendi ayarlarında belirlenir.
settings-default-apps-pdf = PDF dosyaları
settings-default-apps-pdf-detail = Yakınlaştırmalı sayfalar.
settings-default-apps-pictures = Resimler
settings-default-apps-pictures-detail = Fotoğraflar (dik çevrilmiş), PNG, GIF, WebP, BMP, TIFF ve SVG.
settings-default-apps-text = Metin dosyaları
settings-default-apps-text-detail = Düz metin, günlükler, kod ve diğer metinler.
settings-default-apps-sheets = E-tablolar
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) ve CSV.
settings-default-apps-documents = Belgeler
settings-default-apps-documents-detail = Word (docx, doc), OpenDocument metni (odt) ve slaytlar (pptx, ppt, odp).
settings-default-apps-katna = Katna Mail görüntüleyicisi
settings-default-apps-system = Masaüstünün varsayılan uygulaması
settings-default-apps-ask = Her seferinde hangi uygulamanın kullanılacağını sor
settings-default-apps-after-saving = Kaydettikten sonra
settings-default-apps-show-folder = Kaydedilen dosyaları klasörlerinde göster
settings-default-apps-show-folder-detail = Dosya yöneticisini kaydedilen ekler seçili olarak açar

## Settings > Compose

settings-compose-send-from = Yeni iletileri şu hesaptan gönder
settings-compose-send-from-detail = Yanıtlar ve yönlendirmeler her zaman içinde bulunduğunuz hesaptan gönderilir.
settings-compose-send-from-current = İçinde bulunduğunuz hesap
settings-compose-send-on-replies = Yanıtlarda gönderme
settings-compose-send-on-replies-detail = Bir yanıtta veya yönlendirmede Gönder düğmesinin ne yaptığı. Gönder'in yanındaki menü diğerini sunar.
settings-compose-send-plain = Gönder
settings-compose-send-archive = Gönder ve arşivle
settings-compose-signatures = İmzalar
settings-compose-signatures-detail = İletinizin altına, “--” satırından sonra eklenir. Oluşturma penceresinde başka bir imza seçebilirsiniz.
settings-compose-untitled = Adsız
settings-compose-signature-name = Ad, örneğin İş
settings-compose-signature-first = İmzam
settings-compose-signature-numbered = İmza { $number }
settings-compose-signature-delete = Sil
settings-compose-signature-deleted = İmza silindi
settings-compose-signature-new = Yeni oluştur
settings-compose-no-signatures = Henüz imza yok.
settings-compose-no-signature = İmza yok
settings-compose-for-new-mail = Yeni postalar için
settings-compose-for-replies = Yanıtlar ve yönlendirmeler için
settings-compose-for-replies-detail = Bir iletiyi imzaladığınız bir ileti dizisinde yanıt, bunun yerine o imzayla başlar.
settings-compose-format = Biçim
settings-compose-plain-text = Düz metin olarak yaz
settings-compose-plain-text-detail = Yeni postalar biçimlendirme olmadan başlar; oluşturma penceresinde değiştirilebilir
settings-compose-spelling = Yazım
settings-compose-spell-check = Yazarken yazımı denetle
settings-compose-spell-check-detail = Yanlış yazılan kelimelerin altı çizilir, sağ tıklayınca öneriler sunulur
settings-compose-spell-desktop = Masaüstünün dili ({ $language })
settings-compose-templates = Şablonlar
settings-compose-templates-detail = Sık yazdığınız postaları kaydedin ve yeni bir postaya veya yanıta onunla başlayın.

## Settings > Shortcuts

settings-shortcuts-set = Kısayol seti
settings-shortcuts-set-detail = Bildiğiniz bir posta uygulamasının tuşlarıyla başlayın. Burada Cmd, Ctrl'dir. Kendi değişiklikleriniz setin üzerinde kalır; Varsayılanları geri yükle, setin tuşlarına döner.
settings-shortcuts-single = Tek tuşlu kısayollar
settings-shortcuts-single-detail = Web postasındaki gibi Ctrl veya Alt olmadan tuşlar: e arşivler, j ve k gezinir, / arar. Listede ve açık ileti dizisinde çalışır, yazarken asla çalışmaz.
settings-shortcuts-single-use = Tek tuşlu kısayolları kullan
settings-shortcuts-single-use-detail = Ctrl kısayolları her zaman çalışır
settings-shortcuts-how = Değiştirmek için bir tuşa veya eklemek için + simgesine tıklayın, ardından yeni tuşlara basın. Esc iptal eder.
settings-shortcuts-restore = Varsayılanları geri yükle
settings-shortcuts-no-key = Tuş yok
settings-shortcuts-press = Tuşlara basın…
settings-shortcuts-then = { $keys } ardından…
settings-shortcuts-moved = { $keys } artık “{ $previous }” yerine “{ $action }” işlemini yapıyor.
settings-shortcuts-single-off = Tek tuşlu kısayollar kapalı; bu tuş, kısayollar açıldığında çalışır.
settings-shortcuts-restored = Tüm kısayollar yeniden setlerinin tuşlarına döndü.

## Settings search: the line under a result

settings-general-language-summary = Uygulamanın, tarihlerin ve sayıların dili
settings-general-reading-summary = Önce en yeni ileti, tüm üst bilgiler, alıcıların tam adları
settings-general-mark-read-summary = Açılan ileti dizisinin ne zaman okundu olarak işaretleneceği: hemen, 1 veya 3 saniye sonra ya da elle
settings-general-reply-button-summary = Her iletinin yanındaki yanıtla düğmesi herkese yanıt verir
settings-general-remote-images-summary = Her iletinin resimlerini her zaman göster
settings-general-sending-summary = Göndermeyi geri al: gönderilen bir iletinin geri alınabilmesi için ne kadar bekleyeceği
settings-general-offline-summary = Bağlantı olmadan okumak için son kaç günün postalarının tamamen indirileceği
settings-general-notifications-summary = Yeni posta bildirimleri ve sesleri
settings-general-desktop-summary = Oturum açıldığında Katna'yı başlatma, sistem tepsisi simgesi ve görev çubuğu simgesindeki okunmamış sayısı
settings-accounts-accounts-summary = Hesap ekleyin veya kaldırın ya da hesabın resmini değiştirin
settings-appearance-density-summary = Listede varsayılan veya sıkışık satırlar
settings-appearance-scaling-summary = Her şeyi büyütün veya küçültün: metin, simgeler, boşluklar ve ayırıcılar
settings-appearance-theme-summary = Sistem, açık veya koyu
settings-appearance-sender-pictures-summary = Gönderenin alan adına göre aranan şirket logoları
settings-appearance-important-summary = Listede her iletinin yanındaki Önemli işaretçisi
settings-appearance-mail-colors-summary = Koyu temada HTML postalar için koyu renkler ya da gönderenin renkleri
settings-appearance-attachment-previews-summary = Her ekin içeriğinin küçük bir resmi
settings-shortcuts-set-summary = Gmail, Inbox by Gmail, Apple Mail, Outlook veya Thunderbird tuşlarıyla başlayın
settings-shortcuts-single-summary = Web postasındaki gibi Ctrl veya Alt olmadan tuşlar
settings-default-apps-pdf-summary = PDF eklerinin açıldığı yer
settings-default-apps-pictures-summary = Fotoğrafların ve resimlerin açıldığı yer
settings-default-apps-text-summary = Düz metin, günlükler ve kodun açıldığı yer
settings-default-apps-sheets-summary = Excel, OpenDocument ve CSV dosyalarının açıldığı yer
settings-default-apps-documents-summary = Word ve OpenDocument metinlerinin ve slaytların açıldığı yer
settings-default-apps-after-saving-summary = Kaydedilen ekleri klasörlerinde göster
settings-compose-send-from-summary = Yeni postaların gönderildiği hesap: içinde bulunduğunuz hesap ya da her zaman aynı hesap
settings-compose-send-on-replies-summary = Yanıtlarda ve yönlendirmelerde Gönder ya da Gönder ve ileti dizisini arşivle
settings-compose-signatures-summary = İletinizin altına, “--” satırından sonra eklenir
settings-compose-for-new-mail-summary = Yeni postaların başladığı imza
settings-compose-for-replies-summary = Yanıtların ve yönlendirmelerin başladığı imza
settings-compose-format-summary = Yeni postaları düz metin olarak yaz
settings-compose-spelling-summary = Yazarken yazım denetimi ve sözlüğün dili
settings-compose-templates-summary = Çok yakında: sık yazdığınız postaları kaydedin ve yeni bir postaya veya yanıta onunla başlayın
settings-feedback-crash-reports-summary = Katna Mail veya arka plan hizmeti çöktüğünde çökme raporlarını bu bilgisayara kaydet
settings-feedback-saved-summary = Bu bilgisayara kaydedilen çökme raporlarını görüntüleyin, kopyalayın veya silin
settings-feedback-help-improve-summary = Sorunun düzeltilmesine yardımcı olmak için çökme raporları gönderin; siz açmadıkça kapalı
settings-experimental-blur-summary = Masaüstü üst çubuğun arkasından bulanık görünür ve menüler buzlu camdır
settings-search-shortcut = Klavye kısayolu
settings-search-tab = Ayarlar sekmesi
settings-search-none = “{ $query }” ile eşleşen ayar yok.
settings-search-results = “{ $query }” ile eşleşen ayarlar

## Settings: opening at login

settings-open-at-login-failed = Oturum açıldığında başlatma ayarı değiştirilemedi: { $error }

## Settings > General > Time

settings-time = Saat
settings-clock-language = Dilin yazdığı gibi
settings-clock-12 = 12 saatlik, örneğin 2:05 ÖS
settings-clock-24 = 24 saatlik, örneğin 14:05
settings-time-summary = 12 veya 24 saatlik biçim ya da dilin yazdığı gibi

## Settings > General > Default mail app, Settings > Compose > Grammar

settings-general-mail-app = Varsayılan e-posta uygulaması
settings-general-mail-app-detail = Diğer uygulamalardaki ve web sitelerindeki e-posta bağlantıları burada yeni bir ileti açar.
mail-app-is-default = Katna Mail varsayılan e-posta uygulamanız.
mail-app-is-other = E-posta bağlantıları başka bir uygulamada açılır.
mail-app-make-default = Varsayılan yap
mail-app-make-default-failed = Varsayılan e-posta uygulaması değiştirilemedi.
settings-general-mail-app-summary = Diğer uygulamalardan ve web sitelerinden gelen e-posta bağlantılarını Katna Mail'de aç
settings-compose-grammar = Dil bilgisi
settings-compose-grammar-detail = Bu bilgisayarda Harper ile denetlenir. Şimdilik yalnızca İngilizce: başka dillerdeki metinlere dokunulmaz.
settings-compose-grammar-check = Dil bilgisini denetle
settings-compose-grammar-check-detail = Yazarken dil bilgisi hatalarının altını çiz, İngilizce
settings-compose-suggestions = Yazma önerileri
settings-compose-suggestions-detail = Bu bilgisayarda, gönderdiğiniz ve yanıtladığınız postalardan öğrenilir; hiçbir şey bilgisayarın dışına çıkmaz. Bir öneriyi almak için Tab tuşuna basın veya yazmaya devam edin.
settings-compose-suggestions-on = Yazarken öner
settings-compose-suggestions-on-detail = Yazarken bir ifadenin olası devamını gri renkte göster
settings-compose-grammar-summary = Yazarken dil bilgisi hatalarının altını çiz, İngilizce
settings-compose-suggestions-summary = Yazarken bir ifadenin olası devamını gri renkte göster
