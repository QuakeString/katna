# Katna Mail, Turkish (Türkçe).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Dil: { $language }
language-tooltip-system = Dil: { $language }, sisteme göre
language-search = Dil ara
language-system-default = Sistem varsayılanı
language-system-now = Şu an: { $language }
language-no-match = “{ $query }” ile eşleşen dil yok
language-machine = Makine çevirisi. Geliştirmeye yardım edin
language-setting = Dil
language-setting-detail = Menülerin, düğmelerin ve mesajların dili ile tarih ve sayıların biçimi. Sistem varsayılanı masaüstünü izler.

## Dates and sizes

ago-just-now = az önce
ago-minutes = { $count ->
    [one] { $count } dakika önce
   *[other] { $count } dakika önce
}
ago-hours = { $count ->
    [one] { $count } saat önce
   *[other] { $count } saat önce
}
ago-days = { $count ->
    [one] { $count } gün önce
   *[other] { $count } gün önce
}
size-bytes = { $count ->
    [one] { $count } bayt
   *[other] { $count } bayt
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = Klasörleri gizle
folders-show = Klasörleri göster
compose = Oluştur
search = Ara
search-mail = Postalarda ara
search-settings = Ayarlarda ara
search-clear = Aramayı temizle
search-options-show = Arama seçeneklerini göster
settings = Ayarlar
account-add = Hesap ekle

## App rail (and the bottom bar on a phone)

rail-mail = Posta
rail-calendar = Takvim
rail-contacts = Kişiler
rail-tasks = Görevler
rail-notes = Notlar
rail-feeds = Akışlar

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Çok yakında
app-calendar-promise = CalDAV takvimleriniz, postalarınızdaki toplantı davetleri ve hatırlatıcılar, gelen kutunuzun hemen yanında.
app-tasks-promise = CalDAV ile senkronize olan yapılacaklar listeleri ve postalardan oluşturulan görevler.
app-notes-promise = Hızlı notlar ve daha sonrası için bir posta ya da ileti dizisi hakkında notlar.
app-feeds-promise = RSS ve Atom akışlarını postalarınızın yanında okuyun.

## Contacts page

app-contacts-loading = Postalarınızdaki kişiler toplanıyor…
app-contacts-empty = Yazıştığınız kişiler burada görünür.
app-contacts-count = { $count ->
    [one] Postalarınızdan { $count } kişi, en çok yazışılan önce
   *[other] Postalarınızdan { $count } kişi, en çok yazışılan önce
}
app-contacts-top = { $count ->
    [one] Postalarınızda en çok yazıştığınız { $count } kişi
   *[other] Postalarınızda en çok yazıştığınız { $count } kişi
}
app-contacts-messages = { $count ->
    [one] { $count } ileti
   *[other] { $count } ileti
}
app-contacts-last = son: { $date }

## Navigation (the folders pane)

nav-labels = Etiketler
nav-folders = Klasörler
nav-label-new = Yeni etiket oluştur
nav-folder-new = Yeni klasör oluştur
nav-account-unnamed = Hesap { $number }
nav-tab-new = { $count ->
    [one] { $count } yeni
   *[other] { $count } yeni
}

## Special folders (the user's own folders keep their names)

folder-inbox = Gelen Kutusu
folder-starred = Yıldızlı
folder-drafts = Taslaklar
folder-sent = Gönderilmiş Postalar
folder-archive = Arşiv
folder-spam = Spam
folder-trash = Çöp Kutusu
folder-all-mail = Tüm Postalar
folder-scheduled = Planlanmış

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Yeni etiket
label-folder-new-title = Yeni klasör
label-prompt = Lütfen yeni bir etiket adı girin:
label-folder-prompt = Lütfen yeni bir klasör adı girin:
label-name-hint = Etiket adı
label-folder-name-hint = Klasör adı
label-nest = Etiketi şunun altına yerleştir:
label-folder-nest = Klasörü şunun altına yerleştir:
label-cancel = İptal
label-create = Oluştur
label-creating = Oluşturuluyor…
label-created = “{ $name }” etiketi oluşturuldu.
label-folder-created = “{ $name }” klasörü oluşturuldu.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Birincil
tab-promotions = Tanıtımlar
tab-social = Sosyal
tab-updates = Güncellemeler
tab-forums = Forumlar
tab-focused = Odaklanmış
tab-other = Diğer
tab-inbox = Gelen Kutusu
tab-newsletters = Bültenler
tab-notifications = Bildirimler
tab-new = { $count } yeni
tab-provider-other = Katna tarafından sıralanır

## Mail list: toolbar

list-select = Seç
list-refresh = Yenile
list-more = Diğer
list-mark-read = Okundu olarak işaretle
list-mark-unread = Okunmadı olarak işaretle
list-move-to = Taşı
list-archive = Arşivle
list-spam = Spam bildir
list-delete = Sil
list-newer = Daha yeni
list-older = Daha eski
list-range = { $first }–{ $last } / { $total }
list-range-about = { $first }–{ $last } / yaklaşık { $total }
list-results = “{ $query }” için sonuçlar
list-results-corrected = “{ $query }” için sonuçlar gösteriliyor
list-search-instead = Bunun yerine “{ $query }” için ara
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Tümü
list-pick-none = Hiçbiri
list-pick-read = Okunmuş
list-pick-unread = Okunmamış
list-pick-starred = Yıldızlı
list-pick-unstarred = Yıldızsız

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } ileti dizisinin tümü seçildi.
       *[other] { $count } ileti dizisinin tümü seçildi.
    }
   *[message] { $count ->
        [one] { $count } iletinin tümü seçildi.
       *[other] { $count } iletinin tümü seçildi.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } klasöründeki { $count } ileti dizisinin tümü seçildi.
       *[other] { $folder } klasöründeki { $count } ileti dizisinin tümü seçildi.
    }
   *[message] { $count ->
        [one] { $folder } klasöründeki { $count } iletinin tümü seçildi.
       *[other] { $folder } klasöründeki { $count } iletinin tümü seçildi.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] Bu sayfadaki { $count } ileti dizisinin tümü seçildi.
       *[other] Bu sayfadaki { $count } ileti dizisinin tümü seçildi.
    }
   *[message] { $count ->
        [one] Bu sayfadaki { $count } iletinin tümü seçildi.
       *[other] Bu sayfadaki { $count } iletinin tümü seçildi.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } ileti dizisinin tümünü seç
       *[other] { $count } ileti dizisinin tümünü seç
    }
   *[message] { $count ->
        [one] { $count } iletinin tümünü seç
       *[other] { $count } iletinin tümünü seç
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } klasöründeki { $count } ileti dizisinin tümünü seç
       *[other] { $folder } klasöründeki { $count } ileti dizisinin tümünü seç
    }
   *[message] { $count ->
        [one] { $folder } klasöründeki { $count } iletinin tümünü seç
       *[other] { $folder } klasöründeki { $count } iletinin tümünü seç
    }
}
list-clear-selection = Seçimi temizle

## Mail list: empty states

list-empty-search = Aramanızla eşleşen ileti yok.
list-empty-tab = { $tab } sekmesinde posta yok.
list-empty-tab-unknown = Bu sekmede posta yok.
list-empty-folder = { $folder } klasöründe ileti yok.
list-empty-folder-unknown = Bu klasörde ileti yok.
list-first-sync = Postalarınız alınıyor…
list-first-sync-detail = Geldikçe burada görünecekler.

## Mail list: lines

row-removed = Bu ileti kaldırıldı.
row-starred = Yıldızlı
row-not-starred = Yıldızlı değil
row-important = Önemli. Önemli değil olarak işaretlemek için tıklayın.
row-mark-important = Önemli olarak işaretle
row-pinned = En üste sabitlendi
row-pin = En üste sabitle
row-unpin = Sabitlemeyi kaldır

## Mail list: More menu and right-click menu

menu-reply = Yanıtla
menu-reply-all = Tümünü yanıtla
menu-forward = Yönlendir
menu-archive = Arşivle
menu-delete = Sil
menu-spam = Spam bildir
menu-mark-read = Okundu olarak işaretle
menu-mark-unread = Okunmadı olarak işaretle
menu-mark-all-read = Tümünü okundu olarak işaretle
menu-star = Yıldız ekle
menu-unstar = Yıldızı kaldır
menu-important = Önemli olarak işaretle
menu-not-important = Önemli değil olarak işaretle
menu-pin = En üste sabitle
menu-unpin = Sabitlemeyi kaldır
menu-print-all = Tümünü yazdır
menu-new-window = Yeni pencerede aç
menu-move-to = Taşı
menu-move-to-heading = Şuraya taşı:
menu-find-from = { $name } tarafından gönderilen e-postaları bul

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] İleti dizisi arşivlendi.
       *[other] { $count } ileti dizisi arşivlendi.
    }
   *[message] { $count ->
        [one] İleti arşivlendi.
       *[other] { $count } ileti arşivlendi.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] İleti dizisi Çöp Kutusu'na taşındı.
       *[other] { $count } ileti dizisi Çöp Kutusu'na taşındı.
    }
   *[message] { $count ->
        [one] İleti Çöp Kutusu'na taşındı.
       *[other] { $count } ileti Çöp Kutusu'na taşındı.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] İleti dizisi taşındı.
       *[other] { $count } ileti dizisi taşındı.
    }
   *[message] { $count ->
        [one] İleti taşındı.
       *[other] { $count } ileti taşındı.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] İleti dizisine yıldız eklendi.
       *[other] { $count } ileti dizisine yıldız eklendi.
    }
   *[message] { $count ->
        [one] İletiye yıldız eklendi.
       *[other] { $count } iletiye yıldız eklendi.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] İleti dizisinin yıldızı kaldırıldı.
       *[other] { $count } ileti dizisinin yıldızı kaldırıldı.
    }
   *[message] { $count ->
        [one] İletinin yıldızı kaldırıldı.
       *[other] { $count } iletinin yıldızı kaldırıldı.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] İleti dizisi önemli olarak işaretlendi.
       *[other] { $count } ileti dizisi önemli olarak işaretlendi.
    }
   *[message] { $count ->
        [one] İleti önemli olarak işaretlendi.
       *[other] { $count } ileti önemli olarak işaretlendi.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] İleti dizisi önemli değil olarak işaretlendi.
       *[other] { $count } ileti dizisi önemli değil olarak işaretlendi.
    }
   *[message] { $count ->
        [one] İleti önemli değil olarak işaretlendi.
       *[other] { $count } ileti önemli değil olarak işaretlendi.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] İleti dizisi en üste sabitlendi.
       *[other] { $count } ileti dizisi en üste sabitlendi.
    }
   *[message] { $count ->
        [one] İleti en üste sabitlendi.
       *[other] { $count } ileti en üste sabitlendi.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] İleti dizisinin sabitlemesi kaldırıldı.
       *[other] { $count } ileti dizisinin sabitlemesi kaldırıldı.
    }
   *[message] { $count ->
        [one] İletinin sabitlemesi kaldırıldı.
       *[other] { $count } iletinin sabitlemesi kaldırıldı.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] İleti dizisi spam olarak bildirildi.
       *[other] { $count } ileti dizisi spam olarak bildirildi.
    }
   *[message] { $count ->
        [one] İleti spam olarak bildirildi.
       *[other] { $count } ileti spam olarak bildirildi.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] İleti dizisi kalıcı olarak silindi.
       *[other] { $count } ileti dizisi kalıcı olarak silindi.
    }
   *[message] { $count ->
        [one] İleti kalıcı olarak silindi.
       *[other] { $count } ileti kalıcı olarak silindi.
    }
}
toast-undone = İşlem geri alındı.
toast-undo = Geri al
toast-no-spam-folder = Bu hesabın spam klasörü yok.

## Reading pane: toolbar

reader-close = Kapat
reader-back = Geri
reader-mark-unread = Okunmadı olarak işaretle
reader-move-to = Taşı
reader-more = Diğer
reader-print-all = Tümünü yazdır
reader-new-window = Yeni pencerede
reader-position = { $position } / { $total }
reader-newer = Daha yeni
reader-older = Daha eski

## Reading pane: the conversation

reader-removed = Bu ileti dizisi kaldırıldı.
reader-no-subject = (konu yok)
reader-collapse-all = Tümünü daralt
reader-expand-all = Tümünü genişlet
reader-unknown-sender = (bilinmeyen gönderen)
reader-date-ago = { $date } ({ $ago })
reader-me = ben
reader-to = alıcı: { $names }
reader-starred = Yıldızlı
reader-not-starred = Yıldızlı değil
reader-too-long = İleti tamamen gösterilemeyecek kadar uzun.
reader-encrypted-images = Şifreli postalarda web'deki resimler hiçbir zaman yüklenmez.
reader-window-failed = Yeni pencere açılamadı.

## Reading pane: message details (opened from "to me")

reader-details-from = kimden:
reader-details-to = kime:
reader-details-cc = bilgi:
reader-details-date = tarih:
reader-details-subject = konu:

## Reading pane: downloading a message

reader-downloading = Bu ileti sunucudan indiriliyor…
reader-download-failed = Bu ileti indirilemedi.
reader-try-again = Tekrar dene

## Reply row

reply-reply = Yanıtla
reply-reply-all = Tümünü yanıtla
reply-forward = Yönlendir

## Encrypted and signed mail

security-decrypting = Şifre çözülüyor…
security-checking = İmza denetleniyor…
security-partly-encrypted = Bu iletinin yalnızca bir kısmı şifreli. Geri kalanı korumanın dışında eklenmiş ve herhangi birinden gelmiş olabilir.
security-partly-signed = Bu iletinin yalnızca bir kısmı imzalı. Geri kalanı korumanın dışında eklenmiş ve herhangi birinden gelmiş olabilir.
security-encrypted = Şifreli ileti
security-encrypted-smime = Şifreli ileti (S/MIME)
security-no-key = Bu iletinin şifresi çözülemiyor: sizde olmayan bir anahtar için şifrelenmiş.
security-cancelled = Şifre çözme iptal edildi.
security-damaged = Bu iletinin şifresi çözülemiyor: şifreli veriler bozuk ya da değiştirilmiş.
security-decrypt-unavailable = Bu iletinin şifresi çözülemiyor: şifreli postaları okumak için { $tool } yükleyin.
security-decrypt-failed = Bu iletinin şifresi çözülemiyor: { $reason }
security-unknown-signer = bilinmeyen bir imzacı
security-signed-verified = { $signer } tarafından imzalandı · doğrulandı
security-signed-not-sender = { $signer } tarafından imzalandı; imzalayan gönderen değil
security-signed-untrusted = { $signer } tarafından, güvenilmez olarak işaretlediğiniz bir anahtarla imzalandı
security-signed-unverified = { $signer } tarafından imzalandı · anahtar doğrulanmadı
security-bad-signature = Geçersiz imza: bu ileti imzalandıktan sonra değiştirilmiş ya da imza sahte.
security-signature-expired = { $signer } tarafından imzalandı · imzanın süresi doldu
security-key-expired = { $signer } tarafından imzalandı · anahtarın süresi o zamandan beri doldu
security-key-revoked = { $signer } tarafından, iptal edilmiş bir anahtarla imzalandı
security-missing-key = Sizde olmayan bir anahtarla imzalanmış, bu yüzden denetlenemiyor
security-missing-key-id = Sizde olmayan bir anahtarla ({ $key }) imzalanmış, bu yüzden denetlenemiyor
security-signature-unavailable = İmzalı; imzayı denetlemek için { $tool } yükleyin
security-signature-error = İmza denetlenemedi.

## Remote images and pictures

remote-hidden = Bu iletideki resimler gizlendi.
remote-show = Resimleri göster
remote-always-show = Bu gönderenden her zaman göster
remote-picture-use = Kullan
remote-picture-too-big = En fazla 8 MB boyutunda bir resim seçin.
remote-picture-type = PNG, JPEG, GIF, WebP veya SVG biçiminde bir resim seçin.
remote-picture-read-failed = Resim okunamıyor: { $error }
remote-picture-keep-failed = Resim saklanamıyor: { $error }
remote-picture-remove-failed = Resim kaldırılamıyor: { $error }

## Attachments

attachment-count = { $count ->
    [one] Bir ek
   *[other] { $count } ek
}
attachment-save = Kaydet
attachment-save-all = Tümünü kaydet
attachment-save-all-tooltip = Tüm ekleri bir klasöre kaydet
attachment-save-here = Buraya kaydet
attachment-not-downloaded = Bu ileti indirilmedi.
attachment-not-found = Bu ek iletide bulunamadı.
attachment-read-failed = { $name } okunamadı
attachment-numbered = ek { $number }
attachment-saved-all = { $count ->
    [one] { $count } dosya { $place } konumuna kaydedildi
   *[other] { $count } dosya { $place } konumuna kaydedildi
}
attachment-saved-some = { $total ->
    [one] { $total } dosyadan { $saved } tanesi { $place } konumuna kaydedildi. { $failed } kaydedilemedi
   *[other] { $total } dosyadan { $saved } tanesi { $place } konumuna kaydedildi. { $failed } kaydedilemedi
}
attachment-saved-to = Kaydedildiği yer: { $path }
attachment-save-failed = { $name } kaydedilemedi: { $error }
attachment-open-failed = { $name } açılamadı: { $error }
attachment-risky = Bu dosya bir program çalıştırabilir, bu yüzden Katna onu açmaz. Bunun yerine dosyayı kaydedin.
attachment-encrypted-open = Bu dosya şifreli geldi. Başka bir yerde açmak için kaydedin.

## Printing

print-failed = Yazdırılamadı: { $error }
print-no-font = yazı tipi bulunamadı
print-opened-as-pdf = Oradan yazdırmak için PDF olarak açıldı.
print-not-downloaded = (Henüz indirilmedi.)
print-encrypted = (Şifreli. Metnini yazdırmak için Katna Mail'de açın.)
print-to = Kime: { $addresses }
print-cc = Bilgi: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Eklerini okumak için bu iletiyi açın.
text-copy = Kopyala
text-select-all = Tümünü seç

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
settings-general-open-at-login = Oturum açıldığında Katna Mail'i aç
settings-general-open-at-login-detail = Hizmet çalıştığı sürece postalar oturum açılışında her durumda senkronize edilir
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
settings-appearance-theme-system = Masaüstüyle aynı
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
settings-default-apps-documents-detail = Word (docx) ve OpenDocument metni (odt).
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
settings-general-desktop-summary = Oturum açıldığında Katna Mail'i açma, sistem tepsisi simgesi ve görev çubuğu simgesindeki okunmamış sayısı
settings-accounts-accounts-summary = Hesap ekleyin veya kaldırın ya da hesabın resmini değiştirin
settings-appearance-density-summary = Listede varsayılan veya sıkışık satırlar
settings-appearance-scaling-summary = Her şeyi büyütün veya küçültün: metin, simgeler, boşluklar ve ayırıcılar
settings-appearance-theme-summary = Masaüstüyle aynı, açık veya koyu
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
settings-default-apps-documents-summary = Word ve OpenDocument metinlerinin açıldığı yer
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

## Quick settings (the panel that slides in from the right)

quick-title = Hızlı ayarlar
quick-see-all = Tüm ayarları görün
quick-reading-pane = Okuma bölmesi
quick-pane-right = Listenin sağında
quick-pane-none = Bölme yok
quick-density = Yoğunluk
quick-density-default = Varsayılan
quick-density-compact = Sıkışık
quick-theme = Tema
quick-theme-system = Masaüstüyle aynı
quick-theme-light = Açık
quick-theme-dark = Koyu
quick-desktop-colors = Masaüstü renkleri
quick-desktop-colors-detail = Masaüstünün renk şeması ve vurgu rengi
quick-app-names = Uygulama adları
quick-app-names-detail = En soldaki uygulama simgelerinin altında adlar
quick-inbox-tabs = Gelen Kutusu sekmeleri
quick-inbox-tabs-detail = Her hesabın posta sağlayıcısının sekmeleri
quick-choose-tabs = Sekmeleri seç
quick-choose-tabs-detail = Hesap başına, Ayarlar'da
quick-sending = Gönderme
quick-undo-send = Göndermeyi geri al
quick-undo-send-off = Kapalı
quick-undo-send-seconds = { $seconds } sn
quick-signatures = İmzalar
quick-signatures-none = Henüz yok
quick-signatures-one = { $name }, varsayılan olarak kullanılır
quick-signatures-many = { $count ->
    [one] { $count } imza; varsayılan: { $name }
   *[other] { $count } imza; varsayılan: { $name }
}
quick-signatures-no-default = { $count ->
    [one] { $count } imza, varsayılan yok
   *[other] { $count } imza, varsayılan yok
}
quick-signature-untitled = Adsız
quick-threading = E-posta ileti dizileri
quick-conversation-view = İleti dizisi görünümü
quick-conversation-view-detail = Aynı postaya verilen yanıtları grupla
quick-help = Yardım
quick-tour = Turu başlat
quick-whats-new = Yenilikler
quick-about = Katna hakkında

## Settings: opening at login

settings-open-at-login-failed = Oturum açılışında açma ayarı değiştirilemedi: { $error }

## Settings > Appearance > Scaling

scale-letter = A
scale-percent = %{ $percent }
scale-reset = %{ $percent } değerine dön

## Settings > Experimental > Look & Feel

look-intro = Hâlâ denenmekte olan özellikler. Değişebilir veya kaldırılabilirler.
look-heading = Görünüm ve Davranış
look-window-frame = Pencere çerçevesi
look-window-frame-detail = Başlık çubuğunu, pencere düğmelerini, köşeleri ve gölgeyi kimin çizdiği.
look-frame-native-kde = Yerel: Plasma temanızdaki KDE çerçevesi
look-frame-native = Yerel: masaüstünün çerçevesi
look-frame-katna = Katna: üst çubuk başlık çubuğu olur
look-frame-katna-note-named = Katna yuvarlak köşeleri ve kendi gölgesini çizer. Çerçeve artık { $desktop } temasını izlemez; pencere kuralları yine de geçerlidir.
look-frame-katna-note = Katna yuvarlak köşeleri ve kendi gölgesini çizer. Çerçeve artık masaüstü temasını izlemez; pencere kuralları yine de geçerlidir.
look-frame-client-side = Masaüstünüz çerçeveyi her uygulamaya bırakıyor, bu yüzden Katna zaten kendi çerçevesini çiziyor.
look-blurred-background = Bulanık arka plan
look-blurred-background-detail = Masaüstü üst çubuğun ve klasörlerin arkasından bulanık görünür; menüler ve açılır pencereler buzlu camdır.
look-blur = Pencerenin arkasındakini bulanıklaştır
look-blur-detail = Postalar düz kartlarda kalır, böylece metin karşıtlığını korur
look-blur-off-kde = KDE'nin bulanıklık efekti kapalı. Sistem Ayarları > Pencere Yönetimi > Masaüstü Efektleri'nde Bulanıklaştır'ı açın, ardından Katna Mail'i yeniden açın.
look-blur-none-gnome = GNOME pencerelerin arkasındakini bulanıklaştırmaz.
look-blur-none-x11 = Pencere yöneticiniz pencerelerin arkasındakini bulanıklaştırmaz.
look-blur-none-wayland = Bileşikleştiriciniz pencerelerin arkasındakini bulanıklaştırmaz.

## Settings > User feedback (crash reports)

feedback-intro-sending = Yeni çökme raporları, sorunun düzeltilmesine yardımcı olmak için gönderilir. Bu bilgisayardan başka hiçbir şey çıkmaz.
feedback-intro-local = Katna hiçbir yere hiçbir şey göndermez. Çökme raporları bu bilgisayarda kalır; onlara bakabilir veya bir hata raporuna ekleyebilirsiniz.
feedback-crash-reports = Çökme raporları
feedback-crash-reports-detail = Katna Mail veya arka plan hizmeti çöktüğünde yazılır.
feedback-save = Çökme raporlarını bu bilgisayara kaydet
feedback-save-detail = Ana klasörünüz, kullanıcı ve bilgisayar adlarınız ve e-posta adresleri çıkarılır
feedback-saved = Kaydedilen çökme raporları
feedback-saved-detail = { $count ->
    [one] En yeni { $count } rapor saklanır.
   *[other] En yeni { $count } rapor saklanır.
}
feedback-help-improve = Katna'nın geliştirilmesine yardımcı olun
feedback-help-improve-detail = Siz açmadıkça kapalıdır ve burada istediğiniz zaman kapatabilirsiniz.
feedback-send = Çökme raporlarını gönder
feedback-send-detail = Kaydedilen rapor, tam olarak burada görebileceğiniz haliyle Katna'nın çökme izleyicisine (Sentry, AB'de) gider. IP adresi, ileti veya e-posta adresi gönderilmez
feedback-none-saved = Kaydedilmiş çökme raporu yok.
feedback-delete-all = Tümünü sil
feedback-app-daemon = Arka plan hizmeti
feedback-report-sent = { $date } · Gönderildi
feedback-view = Görüntüle
feedback-view-tooltip = Raporu aç
feedback-copy-tooltip = Bir hata raporuna yapıştırmak için kopyala
feedback-copied = Çökme raporu kopyalandı.
feedback-deleted-all = Çökme raporları silindi.
feedback-read-failed = Çökme raporu okunamadı: { $error }
feedback-delete-failed = Çökme raporu silinemedi: { $error }
feedback-delete-all-failed = Çökme raporları silinemedi: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _Dosya
desktop-menu-new-message = _Yeni İleti
desktop-menu-quit = _Çık
desktop-menu-edit = Dü_zen
desktop-menu-undo = _Geri Al
desktop-menu-select-all = _Tümünü Seç
desktop-menu-select-none = _Hiçbirini Seçme
desktop-menu-find = _Bul…
desktop-menu-view = _Görünüm
desktop-menu-folder-list = _Klasör Listesini Göster
desktop-menu-refresh = _Yenile
desktop-menu-go = G_it
desktop-menu-inbox = _Gelen Kutusu
desktop-menu-starred = _Yıldızlı
desktop-menu-sent = Gönderilmiş _Postalar
desktop-menu-drafts = _Taslaklar
desktop-menu-all-mail = Tüm P_ostalar
desktop-menu-next = _Sonraki İleti Dizisi
desktop-menu-previous = _Önceki İleti Dizisi
desktop-menu-message = _İleti
desktop-menu-open = _Aç
desktop-menu-reply = _Yanıtla
desktop-menu-reply-all = _Tümünü Yanıtla
desktop-menu-forward = Yö_nlendir
desktop-menu-archive = A_rşivle
desktop-menu-delete = _Sil
desktop-menu-spam = Spa_m Bildir
desktop-menu-move-to = Ta_şı…
desktop-menu-mark-read = _Okundu Olarak İşaretle
desktop-menu-mark-unread = Okun_madı Olarak İşaretle
desktop-menu-star = Yıldı_z Ekle
desktop-menu-important = Ö_nemli Olarak İşaretle
desktop-menu-not-important = Önemli _Değil Olarak İşaretle
desktop-menu-settings = _Ayarlar
desktop-menu-quick-settings = _Hızlı Ayarlar
desktop-menu-configure = Katna Mail'i _Yapılandır…
desktop-menu-help = _Yardım
desktop-menu-shortcuts = _Klavye Kısayolları
desktop-menu-whats-new = Y_enilikler
desktop-menu-about = Katna _Hakkında

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Gezinme
shortcut-group-actions = İşlemler
shortcut-group-go-to = Git
shortcut-group-app = Uygulama

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Sonraki ileti dizisi
shortcut-previous = Önceki ileti dizisi
shortcut-down = Listede aşağı git
shortcut-up = Listede yukarı git
shortcut-first = Listede ilke git
shortcut-last = Listede sona git
shortcut-page-down = Listede bir sayfa aşağı git
shortcut-page-up = Listede bir sayfa yukarı git
shortcut-open = İleti dizisini aç
shortcut-back = Listeye dön
shortcut-scroll-down = Aşağı kaydır
shortcut-scroll-up = Yukarı kaydır
shortcut-scroll-page-down = Bir sayfa aşağı kaydır
shortcut-scroll-page-up = Bir sayfa yukarı kaydır
shortcut-compose = Oluştur
shortcut-reply = Yanıtla
shortcut-reply-all = Tümünü yanıtla
shortcut-forward = Yönlendir
shortcut-archive = Arşivle
shortcut-delete = Sil
shortcut-spam = Spam bildir
shortcut-move-to = Taşı
shortcut-mark-read = Okundu olarak işaretle
shortcut-mark-unread = Okunmadı olarak işaretle
shortcut-star = Yıldız ekle veya kaldır
shortcut-important = Önemli olarak işaretle
shortcut-not-important = Önemli değil olarak işaretle
shortcut-check = İleti dizisini seç
shortcut-select-all = Tüm ileti dizilerini seç
shortcut-select-none = Tüm ileti dizilerinin seçimini kaldır
shortcut-undo = Son işlemi geri al
shortcut-go-inbox = Gelen Kutusu
shortcut-go-starred = Yıldızlı
shortcut-go-sent = Gönderilmiş Postalar
shortcut-go-drafts = Taslaklar
shortcut-go-all = Tüm Postalar
shortcut-search = Postalarda ara
shortcut-navigation = Menüyü göster veya daralt
shortcut-quick-settings = Hızlı ayarlar
shortcut-settings = Tüm ayarlar
shortcut-shortcuts = Klavye kısayolları
shortcut-reload = Yeni postaları denetle
shortcut-quit = Çık

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } ardından { $second }

## Settings > Accounts

accounts-folder-pane = Klasör bölmesi
accounts-folder-pane-detail = Soldaki bölmenin hangi hesapların klasörlerini gösterdiği.
accounts-shown-one = Bir seferde bir hesap; hesap kartından geçiş yapın
accounts-shown-all = Tüm hesaplar, art arda
accounts-row = Hesaplar
accounts-row-detail = Bir hesabı kaldırmak, Katna'nın bu bilgisayardaki posta kopyasını siler. Postalar sunucuda kalır.
accounts-none = Henüz hesap yok.
accounts-kind-imported = İçe aktarılmış
accounts-picture-reset = Masaüstü resmini kullan
accounts-picture-change = Resmi değiştir
accounts-remove = Kaldır
accounts-delete-all-row = Tüm verileri sil
accounts-delete-all-row-detail = Yeni bir kurulumdaki gibi baştan başlayın.
accounts-delete-all-about = Tüm hesapları, saklanan tüm postaları, kişileri ve takvimleri, arama dizinini, ayarlarınızı ve kayıtlı şifreleri bu bilgisayardan siler. Posta sunucularınızda hiçbir şey değişmez.
accounts-delete-all-open = Tüm Katna verilerini sil

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } Katna'dan kaldırıldı.
accounts-removed = { $address } Katna'dan kaldırıldı. Postaları hâlâ sunucuda.
accounts-all-deleted = Tüm Katna verileri bu bilgisayardan silindi.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } kaldırılsın mı?
accounts-remove-confirm = Hesabı kaldır
accounts-removing = Kaldırılıyor…
accounts-remove-local-mail = { $folders ->
    [0] Bu hesaba içe aktarılan tüm postalar
    [one] Bu hesaba içe aktarılan ve klasöründeki tüm postalar
   *[other] Bu hesaba içe aktarılan ve { $folders } klasöründeki tüm postalar
}
accounts-remove-local-settings = Hesabın Katna ayarları
accounts-remove-mail = { $folders ->
    [0] Katna'nın sakladığı, bu hesaba ait tüm postalar
    [one] Katna'nın klasöründe sakladığı, bu hesaba ait tüm postalar
   *[other] Katna'nın { $folders } klasöründe sakladığı, bu hesaba ait tüm postalar
}
accounts-remove-outbox = Hesabın Giden Kutusu'nda bekleyen iletileri
accounts-remove-settings = Hesabın kayıtlı şifresi ve Katna ayarları
accounts-delete-all-title = Tüm Katna verileri silinsin mi?
accounts-delete-all-confirm = Her şeyi sil
accounts-deleting = Siliniyor…
accounts-delete-all-accounts = Tüm hesaplar ve Katna'nın sakladığı tüm postalar ve ekler
accounts-delete-all-contacts = Kişiler, takvimler ve arama dizini
accounts-delete-all-settings = Tüm ayarlar, imzalar ve klavye kısayolları
accounts-delete-all-passwords = Kayıtlı tüm şifreler
accounts-deleted-heading = Bu bilgisayardan silinenler:
accounts-cannot-undo = Bu işlem geri alınamaz.
accounts-server-delete-all = Posta sunucularınızda hiçbir şey değişmez: postalarınız orada kalır ve bir hesabı yeniden eklemek onları yeniden indirir. Dosyalardan içe aktarılan postalar yalnızca Katna'dadır; dosyalara dokunulmaz.
accounts-server-local = Bu postalar dosyalardan içe aktarıldı, bu yüzden tek kopya Katna'da. Geldikleri dosyalara dokunulmaz; geri almak için onları yeniden içe aktarın.
accounts-server-remove = Posta sunucusunda hiçbir şey değişmez: postalarınız orada kalır ve hesabı yeniden eklemek onları yeniden indirir.
accounts-confirm-word = sil
accounts-confirm-placeholder = “{ accounts-confirm-word }” yazın
accounts-confirm-prompt = Onaylamak için “{ accounts-confirm-word }” yazın:
accounts-cancel = İptal
