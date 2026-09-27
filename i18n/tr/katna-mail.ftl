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
