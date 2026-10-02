# Katna Mail, Turkish (Türkçe).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Dosyalarda ara

## Left side (and chips on a phone)

files-all = Tüm dosyalar
files-pictures = Resimler
files-pdfs = PDF'ler
files-documents = Belgeler
files-sheets = Elektronik tablolar
files-slides = Sunumlar
files-other = Diğer
files-accounts = Hesaplar
files-drives = Sürücüler
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Benimle paylaşılanlar
files-shown = Gösterilen
files-received = Alınanlar
files-sent = Gönderdiklerim

## Over the files

files-count = { $count ->
    [one] { $count } dosya · { $size }
   *[other] { $count } dosya · { $size }
}
files-anyone = Herkes
files-from-person = { $name } kişisinden
files-time-any = Herhangi bir zaman
files-time-today = Bugün
files-time-yesterday = Dün
files-time-this-week = Bu hafta
files-time-last-week = Geçen hafta
files-time-this-month = Bu ay
files-time-last-month = Geçen ay
files-time-between = { $first } – { $last }
files-time-hint = Bir güne tıklayın ya da günler boyunca sürükleyin
files-time-summary = { $count ->
    [one] { $days } · { $count } dosya
   *[other] { $days } · { $count } dosya
}
files-time-clear = Temizle
files-time-month-back = Önceki ay
files-time-month-on = Sonraki ay
files-time-wheel = Uzunluğunu koruyarak bu tarihleri kaydırmak için tekerleği çevirin
files-sort-newest = Önce en yeni
files-sort-oldest = Önce en eski
files-sort-largest = Önce en büyük
files-sort-name = Ada göre
files-grid = Kartlar
files-list = Liste
files-this-week = Bu hafta
files-undated = Tarih yok
files-me = Ben
files-no-subject = (konu yok)
files-loading = Postalarınızdaki dosyalar toplanıyor…
files-empty = Postalarınızdaki dosyalar burada görünür.
files-none-match = Eşleşen dosya yok.
files-load-failed = Dosyalar okunamadı: { $error }

## A file's menu and buttons

files-open = Aç
files-open-with = Birlikte aç…
files-save = Kaydet…
files-show-mail = Postayı göster
files-mail-window = Postayı yeni pencerede aç
files-forward = Dosyayı ilet
files-from-them = { $name } kişisinden gelen dosyalar
files-copy-name = Dosya adını kopyala
files-name-copied = Dosya adı kopyalandı
files-downloading = Posta indiriliyor…
files-download-failed = Bu posta indirilemedi.

## A cloud drive in place of the mail files

files-drive-mine = Drive'ım
files-drive-mine-onedrive = Dosyalarım
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 dosya
       *[other] { $files } dosya
    }
    [one] 1 klasör · { $files ->
        [one] 1 dosya
       *[other] { $files } dosya
    }
   *[other] { $folders } klasör · { $files ->
        [one] 1 dosya
       *[other] { $files } dosya
    }
}
files-drive-folders = Klasörler
files-drive-files = Dosyalar
files-drive-folder = Klasör
files-drive-meta = { $what } · { $date } tarihinde düzenlendi
files-drive-as-link = { $what } · bağlantı olarak
files-drive-google-doc = Google Dokümanı
files-drive-google-sheet = Google E-Tablosu
files-drive-google-slides = Google Slaytları
files-drive-google-drawing = Google Çizimi
files-drive-fetching = Alınıyor…
files-drive-loading = Sürücü açılıyor…
files-drive-empty = Bu klasör boş.
files-drive-unreachable = { $drive } hizmetine ulaşılamıyor.
files-drive-try-again = Yeniden dene
files-drive-needs-permission = Katna'nın bu sürücüyü göstermek için bir kez izninize ihtiyacı var. Yeniden oturum açın ve Katna'nın dosyalarınızı görmesine izin verin.
files-drive-allow = İzin ver
files-drive-allow-failed = Oturum açma tamamlanmadı, bu yüzden sürücü kapalı kalıyor.
files-drive-attach = Ekle
files-drive-more = Diğer
files-drive-download = İndir…
files-drive-open-web = { $drive } içinde aç
files-drive-copy-link = Bağlantıyı kopyala
files-drive-link-copied = Bağlantı kopyalandı
files-drive-share = Paylaş…
files-drive-rename = Yeniden adlandır
files-drive-trash = Çöp kutusuna taşı
files-drive-trashed = “{ $name }”, { $drive } çöp kutusunda
files-drive-renamed = “{ $name }” olarak yeniden adlandırıldı
files-drive-getting = { $name }, { $drive } hizmetinden alınıyor…
files-drive-get-failed = { $name } alınamadı: { $error }
files-drive-upload = Yükle
files-drive-upload-files = Dosya yükle
files-drive-upload-folder = Klasör yükle
files-drive-upload-failed = { $name } yüklenemedi: { $error }
files-drive-upload-needs = Yüklemek için Katna'nın bir kez izninize ihtiyacı var: Ayarlar › Varsayılan uygulamalar › Dosyalar sayfası'nda İzin ver'e basın.

## The Share dialog of a drive file or folder

files-share-title = “{ $name }” paylaş
files-share-add = Ad veya adresle kişi ekleyin
files-share-not-address = “{ $text }” bir e-posta adresi değil
files-share-notify = { $drive } onlara e-posta da göndersin
files-share-people = Erişimi olan kişiler
files-share-general = Genel erişim
files-share-loading = Kimin erişimi olduğu okunuyor…
files-share-restricted = Kısıtlı
files-share-restricted-about = Yalnızca erişimi olan kişiler bağlantıyla açabilir
files-share-anyone = Bağlantıya sahip olan herkes
files-share-anyone-can = { $role ->
    [editor] Bağlantıya sahip olan herkes düzenleyebilir
    [commenter] Bağlantıya sahip olan herkes yorum yapabilir
   *[viewer] Bağlantıya sahip olan herkes görüntüleyebilir
}
files-share-anyone-about = { $role ->
    [editor] İnternette bağlantıya sahip olan herkes düzenleyebilir
    [commenter] İnternette bağlantıya sahip olan herkes yorum yapabilir
   *[viewer] İnternette bağlantıya sahip olan herkes görüntüleyebilir
}
files-share-role-owner = Sahip
files-share-role-editor = Düzenleyen
files-share-role-commenter = Yorumcu
files-share-role-viewer = Görüntüleyen
files-share-you = { $name } (siz)
files-share-domain = { $domain } alanındaki herkes
files-share-inherited = İçinde bulunduğu klasörden gelen erişim
files-share-remove = Erişimi kaldır
files-share-copy-link = Bağlantıyı kopyala
files-share-share = Paylaş
files-share-done = Bitti
files-share-sharing = Paylaşılıyor…
files-share-shared = { $count ->
    [one] 1 kişiyle paylaşıldı
   *[other] { $count } kişiyle paylaşıldı
}
files-share-refused = { $drive }, { $addresses } ile paylaşamadı
files-share-failed = Paylaşım değiştirilemedi: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] 1 öge yükleniyor
   *[other] { $count } öge yükleniyor
}
files-tray-done = { $count ->
    [one] 1 yükleme tamamlandı
   *[other] { $count } yükleme tamamlandı
}
files-tray-some-failed = { $done } yüklendi, { $failed } başarısız oldu
files-tray-minutes-left = { $minutes ->
    [one] Yaklaşık bir dakika kaldı
   *[other] Yaklaşık { $minutes } dakika kaldı
}
files-tray-seconds-left = Bir dakikadan az kaldı
files-tray-starting = Başlatılıyor…
files-tray-cancel-all = Tümünü iptal et
files-tray-cancel = İptal
files-tray-fold = Listeyi gizle
files-tray-unfold = Listeyi göster
files-tray-close = Kapat
files-tray-progress = { $place } · { $size } boyutunun { $sent } kadarı
files-tray-in = { $place } içinde
files-tray-cancelled = İptal edildi
