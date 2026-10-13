# Katna Mail, Turkish (Türkçe).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Etiketler
nav-folders = Klasörler
nav-label-new = Yeni etiket oluştur
nav-folder-new = Yeni klasör oluştur
nav-menu-check-mail = Yeni postaları denetle
nav-menu-check-inbox = Bu gelen kutusunu denetle
nav-unified-leave-out = Birleşik Gelen Kutusu'nun dışında bırak
nav-unified-bring-back = Birleşik Gelen Kutusu'na geri getir
nav-menu-sign-in-again = Yeniden oturum aç
nav-menu-new-mail = Bu hesaptan yeni posta
nav-menu-account-settings = Hesap ayarları
nav-account-checked = Eşitlendi · { $ago } denetlendi
nav-account-in-sync = Eşitlendi
nav-account-connecting = Bağlanılıyor…
nav-account-offline = Çevrimdışı, yeniden deneniyor
nav-account-signed-out = { $provider } oturumunun süresi doldu
nav-account-password-refused = Parola reddedildi
nav-account-storage = { $total } alanın { $used } kadarı kullanılıyor
nav-menu-new-subfolder = İçine yeni klasör
nav-menu-new-sublabel = İçine yeni etiket
nav-menu-rename = Yeniden adlandır
nav-menu-delete = Sil
nav-menu-empty-trash = Çöp Kutusu'nu boşalt
nav-account-unnamed = Hesap { $number }
nav-all-accounts = Tüm Hesaplar
nav-expand = Klasörleri göster
nav-collapse = Klasörleri gizle
storage-used = { $total } alanın %{ $percent } kadarı kullanılıyor
storage-used-detail = { $address }: { $total } alanın { $used } kadarı kullanılıyor

## Special folders (the user's own folders keep their names)

folder-inbox = Gelen Kutusu
folder-starred = Yıldızlı
folder-snoozed = Ertelenenler
folder-unread = Okunmamış
folder-important = Önemli
folder-drafts = Taslaklar
folder-sent = Gönderilmiş Postalar
folder-archive = Arşiv
folder-spam = Spam
folder-trash = Çöp Kutusu
folder-all-mail = Tüm Postalar
folder-scheduled = Planlanmış
folder-waiting = Yanıt bekleyenler
folder-waiting-short = Bekleyenler
folder-reminders = Hatırlatıcılar
folder-outbox = Giden Kutusu
folder-activity = Etkinlik
folder-not-on-account = Bu hesapta böyle bir klasör yok.

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
label-rename-title = Etiketi yeniden adlandır
label-folder-rename-title = Klasörü yeniden adlandır
label-rename = Yeniden adlandır
label-renaming = Yeniden adlandırılıyor…
label-renamed = Etiketin adı “{ $name }” olarak değiştirildi.
label-folder-renamed = Klasörün adı “{ $name }” olarak değiştirildi.
folder-delete-title = “{ $name }” silinsin mi?
folder-delete-body = { $count ->
    [0] İçinde posta yok. Klasör sunucudan kaldırılır, bu yüzden web postasından ve telefonunuzdan da kaybolur.
   *[other] { $kind ->
        [conversation] { $count ->
           *[other] İçindeki { $count } ileti dizisi Çöp Kutusu'na gider, yani yine de geri alabilirsiniz.
        }
       *[message] { $count ->
           *[other] İçindeki { $count } ileti Çöp Kutusu'na gider, yani yine de geri alabilirsiniz.
        }
    } Klasör sunucudan kaldırılır, bu yüzden web postasından ve telefonunuzdan da kaybolur.
}
folder-delete-forever-body = { $count ->
    [0] İçinde posta yok. Klasör sunucudan kaldırılır, bu yüzden web postasından ve telefonunuzdan da kaybolur.
   *[other] { $kind ->
        [conversation] { $count ->
           *[other] İçindeki { $count } ileti dizisi kalıcı olarak silinir; bu hesabın Çöp Kutusu yok.
        }
       *[message] { $count ->
           *[other] İçindeki { $count } ileti kalıcı olarak silinir; bu hesabın Çöp Kutusu yok.
        }
    } Klasör sunucudan kaldırılır, bu yüzden web postasından ve telefonunuzdan da kaybolur.
}
folder-delete-label-body = Etiket kaldırılır. Postaları Tüm Postalar'da ve diğer etiketlerinde kalır.
folder-delete-confirm = Klasörü sil
folder-delete-label-confirm = Etiketi sil
folder-deleted = “{ $name }” klasörü silindi
label-deleted = “{ $name }” etiketi silindi
