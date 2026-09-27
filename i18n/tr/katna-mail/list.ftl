# Katna Mail, Turkish (Türkçe).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } okunmuş ileti dizisi seçildi.
           *[other] { $count } okunmuş ileti dizisinin tümü seçildi.
        }
       *[message] { $count ->
            [one] { $count } okunmuş ileti seçildi.
           *[other] { $count } okunmuş iletinin tümü seçildi.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } okunmamış ileti dizisi seçildi.
           *[other] { $count } okunmamış ileti dizisinin tümü seçildi.
        }
       *[message] { $count ->
            [one] { $count } okunmamış ileti seçildi.
           *[other] { $count } okunmamış iletinin tümü seçildi.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } yıldızlı ileti dizisi seçildi.
           *[other] { $count } yıldızlı ileti dizisinin tümü seçildi.
        }
       *[message] { $count ->
            [one] { $count } yıldızlı ileti seçildi.
           *[other] { $count } yıldızlı iletinin tümü seçildi.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } yıldızsız ileti dizisi seçildi.
           *[other] { $count } yıldızsız ileti dizisinin tümü seçildi.
        }
       *[message] { $count ->
            [one] { $count } yıldızsız ileti seçildi.
           *[other] { $count } yıldızsız iletinin tümü seçildi.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder } klasöründeki { $count } okunmuş ileti dizisi seçildi.
           *[other] { $folder } klasöründeki { $count } okunmuş ileti dizisinin tümü seçildi.
        }
       *[message] { $count ->
            [one] { $folder } klasöründeki { $count } okunmuş ileti seçildi.
           *[other] { $folder } klasöründeki { $count } okunmuş iletinin tümü seçildi.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder } klasöründeki { $count } okunmamış ileti dizisi seçildi.
           *[other] { $folder } klasöründeki { $count } okunmamış ileti dizisinin tümü seçildi.
        }
       *[message] { $count ->
            [one] { $folder } klasöründeki { $count } okunmamış ileti seçildi.
           *[other] { $folder } klasöründeki { $count } okunmamış iletinin tümü seçildi.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } klasöründeki { $count } yıldızlı ileti dizisi seçildi.
           *[other] { $folder } klasöründeki { $count } yıldızlı ileti dizisinin tümü seçildi.
        }
       *[message] { $count ->
            [one] { $folder } klasöründeki { $count } yıldızlı ileti seçildi.
           *[other] { $folder } klasöründeki { $count } yıldızlı iletinin tümü seçildi.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder } klasöründeki { $count } yıldızsız ileti dizisi seçildi.
           *[other] { $folder } klasöründeki { $count } yıldızsız ileti dizisinin tümü seçildi.
        }
       *[message] { $count ->
            [one] { $folder } klasöründeki { $count } yıldızsız ileti seçildi.
           *[other] { $folder } klasöründeki { $count } yıldızsız iletinin tümü seçildi.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Burada okunmuş ileti dizisi yok.
       *[message] Burada okunmuş ileti yok.
    }
   *[unread] { $kind ->
        [conversation] Burada okunmamış ileti dizisi yok.
       *[message] Burada okunmamış ileti yok.
    }
    [starred] { $kind ->
        [conversation] Burada yıldızlı ileti dizisi yok.
       *[message] Burada yıldızlı ileti yok.
    }
    [unstarred] { $kind ->
        [conversation] Burada yıldızsız ileti dizisi yok.
       *[message] Burada yıldızsız ileti yok.
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
menu-delete-forever = Kalıcı olarak sil
menu-move-to-inbox = Gelen Kutusu'na taşı
menu-spam = Spam bildir
menu-not-spam = Spam değil
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
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] İleti dizisi spam değil olarak işaretlendi ve gelen kutusuna taşındı.
       *[other] { $count } ileti dizisi spam değil olarak işaretlendi ve gelen kutusuna taşındı.
    }
   *[message] { $count ->
        [one] İleti spam değil olarak işaretlendi ve gelen kutusuna taşındı.
       *[other] { $count } ileti spam değil olarak işaretlendi ve gelen kutusuna taşındı.
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
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] İleti dizisi okundu olarak işaretlendi.
       *[other] { $count } ileti dizisi okundu olarak işaretlendi.
    }
   *[message] { $count ->
        [one] İleti okundu olarak işaretlendi.
       *[other] { $count } ileti okundu olarak işaretlendi.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] İleti dizisi okunmadı olarak işaretlendi.
       *[other] { $count } ileti dizisi okunmadı olarak işaretlendi.
    }
   *[message] { $count ->
        [one] İleti okunmadı olarak işaretlendi.
       *[other] { $count } ileti okunmadı olarak işaretlendi.
    }
}
toast-undone = İşlem geri alındı.
toast-nothing-to-undo = Geri alınacak bir şey yok.
toast-cannot-undo-delete-forever = Kalıcı olarak silinen postalar geri getirilemez.
toast-send-undone = Gönderme geri alındı.
toast-too-late-to-undo-send = Geri almak için çok geç: ileti zaten gönderildi.
toast-undo = Geri al
toast-no-spam-folder = Bu hesabın spam klasörü yok.
