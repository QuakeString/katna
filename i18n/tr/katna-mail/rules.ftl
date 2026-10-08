# Katna Mail, Turkish (Türkçe).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Kurallar
settings-rules-summary = Yeni postaları kendiliğinden sırala, etiketle, ilet veya sessize al
settings-rules-intro = Kurallar yeni postaları bu sırayla kendiliğinden sıralar. Sırayı değiştirmek için sürükleyin.
settings-rules-all-accounts = Tüm hesaplar
settings-rules-new = Yeni kural
settings-rules-none = Henüz kural yok. Bir kural yeni postaları kendiliğinden sıralar: gönderene, konuya veya sözcüklere göre.
settings-rules-none-account = Bu hesap için henüz kural yok.
settings-rules-drag = Sırayı değiştirmek için sürükleyin
settings-rules-edit = Kuralı düzenle
settings-rules-turn-off = Bu kuralı kapat
settings-rules-turn-on = Bu kuralı aç

## Starter rules: offered under the user's own rules, switched off.

## Turning one on makes it one of the user's rules.

settings-rules-starters = Hazır kurallar
settings-rules-starters-intro = Siz açana kadar kapalıdırlar. Tüm hesaplarınız için çalışırlar; değiştirmek için birini düzenleyin.
settings-rules-starter-turning-on = “{ $name }” açılıyor…
settings-rules-starter-failed = “{ $name }” açılamadı: { $error }
rules-starter-promotions = Promosyonları sessize al
rules-starter-newsletters = Bültenler Okunacaklar'a
rules-starter-receipts = Makbuzlar ve faturalar
rules-starter-deliveries = Teslimatlar
rules-starter-train = Tren biletleri
rules-starter-flight = Uçak biletleri
rules-starter-codes = Tek kullanımlık kodlar
rules-starter-security = Güvenlik uyarıları
rules-starter-social = Sosyal postalar
rules-starter-invites = Takvim davetleri
rules-starter-folder-reading = Okunacaklar
rules-starter-folder-receipts = Makbuzlar
rules-starter-folder-deliveries = Teslimatlar
rules-starter-folder-travel = Seyahat
rules-starter-folder-social = Sosyal
rules-runs-katna = Katna'da çalışır
rules-runs-gmail = Gmail'de çalışır
rules-runs-sieve = Sunucuda çalışır
rules-stopped = Durduruldu
rules-error-folder-gone = Bu kuralın kullandığı klasör artık yok. Başka bir klasör seçmek için kuralı düzenleyin.
rules-error-no-archive = Bu hesabın arşiv klasörü yok. Başka bir şey yapması için kuralı düzenleyin.
rules-error-no-trash = Bu hesabın Çöp Kutusu klasörü yok. Başka bir şey yapması için kuralı düzenleyin.
rules-error-cannot-send = Bu hesap posta gönderemiyor, bu yüzden kural postayı iletemez.
rules-error-other = { $error }. Kuralı düzenleyin ve yeniden açın.
settings-folders = Klasörler
settings-folders-summary = Klasör bölmesindeki okunmamış sayıları
settings-folders-unread-counts = Her klasörde okunmamış sayısı
settings-folders-unread-counts-detail = Kapalı: yalnızca Gelen Kutusu kaç okunmamış olduğunu gösterir

## A rule in one line, on its row: "From contains substack.com → skip the

## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } ve { $next }
rules-summary-or = { $first } veya { $next }
rules-summary-more = { $count } tane daha
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Eki var
rules-summary-no-attachment = Eki yok
rules-summary-mailing-list = Bir posta listesinden
rules-summary-not-mailing-list = Bir posta listesinden değil
rules-summary-tab = { $tab } sekmesinde
rules-summary-not-tab = { $tab } sekmesinde değil
rules-summary-move = { $folder } klasörüne taşı
rules-summary-archive = gelen kutusunu atla
rules-summary-trash = çöp kutusuna taşı
rules-summary-mark-read = okundu olarak işaretle
rules-summary-star = yıldız ekle
rules-summary-important = önemli olarak işaretle
rules-summary-label = { $label } etiketini ekle
rules-summary-forward = { $address } adresine ilet
rules-summary-dont-notify = bildirme
rules-summary-read-after = { $count ->
   *[other] { $count } gün sonra okundu olarak işaretle
}
rules-summary-folder-gone = artık olmayan bir klasör

## The rule editor

rules-editor-new-title = Yeni kural
rules-editor-edit-title = Kuralı düzenle
rules-editor-name-hint = Kural adı
rules-editor-when = Yeni bir posta şunların
rules-editor-of-these = ile eşleştiğinde:
rules-mode-all = tümü
rules-mode-any = herhangi biri
rules-field-from = Kimden
rules-field-to = Kime
rules-field-cc = Bilgi
rules-field-any-recipient = Kime veya Bilgi
rules-field-reply-to = Yanıt adresi
rules-field-subject = Konu
rules-field-body = Metin
rules-field-attachment-name = Ek adı
rules-field-has-attachment = Eki var
rules-field-mailing-list = Bir posta listesinden
rules-field-tab = Gelen Kutusu sekmesi
rules-comparator-contains = şunu içeriyor
rules-comparator-not-contains = şunu içermiyor
rules-comparator-begins-with = şununla başlıyor
rules-comparator-ends-with = şununla bitiyor
rules-comparator-equals = tam olarak şu
rules-comparator-matches = şu kalıpla eşleşiyor
rules-has-yes = evet
rules-has-no = hayır
rules-editor-value-hint = Sözcükler veya bir adres
rules-editor-add-condition = Koşul ekle
rules-editor-remove = Kaldır
rules-editor-then = Sonra:
rules-action-move = Şuraya taşı
rules-action-archive = Gelen kutusunu atla (arşivle)
rules-action-trash = Çöp kutusuna taşı
rules-action-mark-read = Okundu olarak işaretle
rules-action-star = Yıldız ekle
rules-action-important = Önemli olarak işaretle
rules-action-label = Etiket ekle
rules-action-forward = Şuraya ilet
rules-action-dont-notify = Bildirme
rules-action-read-after = Şu kadar sonra okundu olarak işaretle
rules-editor-choose-folder = Bir klasör seçin
rules-editor-choose-label = Bir etiket seçin
rules-editor-new-folder = Yeni: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = E-posta adresi
rules-editor-days = gün
rules-editor-add-action = Eylem ekle
rules-editor-stop = Burada dur: sonraki kurallar bu postada çalışmaz
rules-editor-accounts = Hesaplar:
rules-editor-accounts-none = Hesap seçin
rules-editor-accounts-many = { $count ->
   *[other] { $count } hesap
}
rules-editor-matches = Son { $days } gündeki { $mails } ile eşleşiyor
rules-editor-mails = { $count ->
   *[other] { $count } posta
}
rules-editor-counting = Eşleştiği postalar sayılıyor…
rules-editor-show = Göster
rules-editor-also-apply = Bu { $count } postaya da uygula
rules-editor-runs-katna = Bu bilgisayar açıkken Katna'da çalışır.
rules-editor-runs-gmail = Gmail'de çalışır, böylece telefonunuzda ve bu bilgisayar kapalıyken de işler.
rules-editor-runs-sieve = Posta sunucunuzda çalışır, böylece telefonunuzda ve bu bilgisayar kapalıyken de işler.
rules-note-gmail-action = Katna'da çalışır: Gmail filtreleri “{ $action }” eylemini yapamaz.
rules-note-sieve-action = Katna'da çalışır: posta sunucunuzun kuralları “{ $action }” eylemini yapamaz.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Katna'da çalışır: Gmail filtreleri “{ $test }” koşulunu Katna gibi sınayamaz.
rules-note-sieve-condition = Katna'da çalışır: posta sunucunuzun kuralları “{ $test }” koşulunu Katna gibi sınayamaz.
rules-note-order = Hesabın önceki bir kuralı gibi Katna'da çalışır: kurallar liste sırasıyla çalışır.
rules-note-gmail-stop = Katna'da çalışır: Gmail filtreleri sonraki kuralların çalışmasını engelleyemez.
rules-note-gmail-forward = Katna'da çalışır: Gmail yalnızca ayarlarında doğrulanmış adreslere iletir ve { $address } bunlardan biri değil.
rules-note-gmail-folder = Katna'da çalışır: Gmail'de bu kuralın kullandığı klasör için bir etiket yok.
rules-note-sieve-folder = Katna'da çalışır: posta sunucunuzda bu kuralın kullandığı klasör yok.
rules-note-gmail-sign-in = Google'da yeniden oturum açıp Katna'nın Gmail filtreleri oluşturmasına izin verene kadar Katna'da çalışır.
rules-note-sieve-other-script = Katna'da çalışır: posta sunucunuzda başka bir kural betiği (“{ $name }”) etkin.
rules-note-gmail-failed = Katna'da çalışır: Gmail onu kabul etmedi ({ $error }).
rules-note-sieve-failed = Katna'da çalışır: posta sunucunuz onu kabul etmedi ({ $error }).
rules-editor-cancel = İptal
rules-editor-save = Kaydet
rules-editor-saving = Kaydediliyor…
rules-editor-delete = Kuralı sil
rules-editor-delete-ask = Bu kural silinsin mi?
rules-editor-delete-keep = Kalsın
rules-editor-delete-confirm = Sil
rules-editor-needs-folder = Her “Şuraya taşı” için bir klasör ve her “Etiket ekle” için bir etiket seçin.
rules-editor-needs-days = “Şu kadar sonra okundu olarak işaretle” 1 ile 3650 arasında bir gün sayısı ister.
rules-saved = Kural kaydedildi
rules-saved-applied = { $count ->
   *[other] Kural kaydedildi ve { $count } postaya uygulandı
}
rules-apply-failed = Kural kaydedildi ama uygulanamadı: { $error }
rules-deleted = Kural silindi
rules-delete-failed = Kural silinemedi: { $error }
rules-change-failed = Kurallar değiştirilemedi: { $error }
