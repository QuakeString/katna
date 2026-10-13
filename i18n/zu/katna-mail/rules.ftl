# Katna Mail, Zulu (isiZulu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Imithetho
settings-rules-summary = Hlunga, faka ilebula, dlulisela noma thulisa imeyili entsha ngokwayo
settings-rules-intro = Imithetho ihlunga imeyili entsha ngokwayo, ngale ndlela elandelanayo. Hudula ukuze ushintshe ukulandelana.
settings-rules-all-accounts = Wonke ama-akhawunti
settings-rules-new = Umthetho omusha
settings-rules-none = Ayikho imithetho okwamanje. Umthetho uhlunga imeyili entsha ngokwawo: ngomthumeli, isihloko noma amagama.
settings-rules-none-account = Ayikho imithetho yale akhawunti okwamanje.
settings-rules-drag = Hudula ukuze ushintshe ukulandelana
settings-rules-edit = Hlela umthetho
settings-rules-turn-off = Vala lo mthetho
settings-rules-turn-on = Vula lo mthetho

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = Imithetho yokuqala
settings-rules-starters-intro = Ivaliwe uze uvule owodwa. Isebenza kuwo wonke ama-akhawunti akho; hlela owodwa ukuze uwushintshe.
settings-rules-starter-turning-on = Kuvulwa “{ $name }”…
settings-rules-starter-failed = Akukwazekanga ukuvula “{ $name }”: { $error }
rules-starter-promotions = Thulisa ukukhangisa
rules-starter-newsletters = Izincwadi zezindaba ziye ku-Ukufunda
rules-starter-receipts = Amarisidi nama-invoyisi
rules-starter-deliveries = Okulethwayo
rules-starter-train = Amathikithi esitimela
rules-starter-flight = Amathikithi endiza
rules-starter-codes = Amakhodi esikhathi esisodwa
rules-starter-security = Izexwayiso zokuphepha
rules-starter-social = Imeyili yezenhlalo
rules-starter-invites = Izimemo zekhalenda
rules-starter-folder-reading = Ukufunda
rules-starter-folder-receipts = Amarisidi
rules-starter-folder-deliveries = Okulethwayo
rules-starter-folder-travel = Ukuhamba
rules-starter-folder-social = Ezenhlalo
rules-runs-katna = Isebenza ku-Katna
rules-runs-gmail = Isebenza ku-Gmail
rules-runs-sieve = Isebenza kuseva
rules-stopped = Imisiwe
rules-error-folder-gone = Ifolda esetshenziswa yilo mthetho ayisekho. Hlela umthetho ukuze ukhethe enye.
rules-error-no-archive = Le akhawunti ayinayo ifolda yengobo yomlando. Hlela umthetho ukuze wenze okunye.
rules-error-no-trash = Le akhawunti ayinayo ifolda kadoti. Hlela umthetho ukuze wenze okunye.
rules-error-cannot-send = Le akhawunti ayikwazi ukuthumela imeyili, ngakho umthetho awukwazi ukuyidlulisela.
rules-error-other = { $error }. Hlela umthetho bese uwuvula futhi.

settings-folders = Amafolda
settings-folders-summary = Izibalo ezingafundiwe kuphaneli yamafolda
settings-folders-unread-counts = Isibalo esingafundiwe kuyo yonke ifolda
settings-folders-unread-counts-detail = Kuvaliwe: yi-Ibhokisi lokungenayo kuphela elibonisa ukuthi mangaki angafundiwe

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } futhi { $next }
rules-summary-or = { $first } noma { $next }
rules-summary-more = { $count } ngaphezulu
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Kunokunamathiselwe
rules-summary-no-attachment = Akunakho okunamathiselwe
rules-summary-mailing-list = Kuvela ohlwini lwamakheli
rules-summary-not-mailing-list = Akuveli ohlwini lwamakheli
rules-summary-tab = Kuthebhu ethi { $tab }
rules-summary-not-tab = Akukho kuthebhu ethi { $tab }
rules-summary-move = hambisa ku-{ $folder }
rules-summary-archive = yeqa ibhokisi lokungenayo
rules-summary-trash = hambisa kudoti
rules-summary-mark-read = maka njengokufundiwe
rules-summary-star = faka inkanyezi
rules-summary-important = maka njengokubalulekile
rules-summary-label = faka ilebula { $label }
rules-summary-forward = dlulisela ku-{ $address }
rules-summary-dont-notify = ungazisi
rules-summary-read-after = { $count ->
    [one] maka njengokufundiwe emva kosuku olu-{ $count }
   *[other] maka njengokufundiwe emva kwezinsuku ezingu-{ $count }
}
rules-summary-folder-gone = ifolda engasekho

## The rule editor

rules-editor-new-title = Umthetho omusha
rules-editor-edit-title = Hlela umthetho
rules-editor-name-hint = Igama lomthetho
rules-editor-when = Uma imeyili entsha ihambisana
rules-editor-of-these = nalokhu:
rules-mode-all = nakho konke
rules-mode-any = nokunye kwakho
rules-field-from = Kusuka ku-
rules-field-to = Ku-
rules-field-cc = Cc
rules-field-any-recipient = Ku- noma Cc
rules-field-reply-to = Phendula ku-
rules-field-subject = Isihloko
rules-field-body = Umbhalo
rules-field-attachment-name = Igama lokunamathiselwe
rules-field-has-attachment = Kunokunamathiselwe
rules-field-mailing-list = Kuvela ohlwini lwamakheli
rules-field-tab = Ithebhu yebhokisi lokungenayo
rules-comparator-contains = kuqukethe
rules-comparator-not-contains = akuqukethe
rules-comparator-begins-with = kuqala ngo-
rules-comparator-ends-with = kuphela ngo-
rules-comparator-equals = kuyi-
rules-comparator-matches = kuhambisana nephethini
rules-has-yes = yebo
rules-has-no = cha
rules-editor-value-hint = Amagama noma ikheli
rules-editor-add-condition = Engeza umbandela
rules-editor-remove = Susa
rules-editor-then = Bese:
rules-action-move = Hambisa ku-
rules-action-archive = Yeqa ibhokisi lokungenayo (ingobo yomlando)
rules-action-trash = Hambisa kudoti
rules-action-mark-read = Maka njengokufundiwe
rules-action-star = Faka inkanyezi
rules-action-important = Maka njengokubalulekile
rules-action-label = Engeza ilebula
rules-action-forward = Dlulisela ku-
rules-action-dont-notify = Ungazisi
rules-action-read-after = Maka njengokufundiwe emva kwe-
rules-editor-choose-folder = Khetha ifolda
rules-editor-choose-label = Khetha ilebula
rules-editor-new-folder = Okusha: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = Ikheli le-imeyili
rules-editor-days = izinsuku
rules-editor-add-action = Engeza isenzo
rules-editor-stop = Yima lapha: imithetho elandelayo ayisebenzi kule meyili
rules-editor-accounts = Ama-akhawunti:
rules-editor-accounts-none = Khetha ama-akhawunti
rules-editor-accounts-many = { $count ->
    [one] I-akhawunti engu-{ $count }
   *[other] Ama-akhawunti angu-{ $count }
}
rules-editor-matches = Ihambisana no-{ $mails } ezinsukwini ezingu-{ $days } ezedlule
rules-editor-mails = { $count ->
    [one] imeyili engu-{ $count }
   *[other] amameyili angu-{ $count }
}
rules-editor-counting = Kubalwa imeyili ehambisana nawo…
rules-editor-show = Zibonise
rules-editor-also-apply = Sebenzisa nakulezi ezingu-{ $count }
rules-editor-runs-katna = Isebenza ku-Katna, ngesikhathi le khompyutha ivuliwe.
rules-editor-runs-gmail = Isebenza ku-Gmail, ngakho iyasebenza nasefonini yakho nalapho le khompyutha ivaliwe.
rules-editor-runs-sieve = Isebenza kuseva yakho yemeyili, ngakho iyasebenza nasefonini yakho nalapho le khompyutha ivaliwe.
rules-note-gmail-action = Isebenza ku-Katna: izihlungi ze-Gmail azikwazi ukwenza “{ $action }”.
rules-note-sieve-action = Isebenza ku-Katna: imithetho yeseva yakho yemeyili ayikwazi ukwenza “{ $action }”.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Isebenza ku-Katna: izihlungi ze-Gmail azikwazi ukuhlola “{ $test }” njengoba kwenza i-Katna.
rules-note-sieve-condition = Isebenza ku-Katna: imithetho yeseva yakho yemeyili ayikwazi ukuhlola “{ $test }” njengoba kwenza i-Katna.
rules-note-order = Isebenza ku-Katna, njengomthetho wangaphambili we-akhawunti: imithetho isebenza ngokulandelana kohlu.
rules-note-gmail-stop = Isebenza ku-Katna: izihlungi ze-Gmail azikwazi ukuvimba imithetho elandelayo ukuthi isebenze.
rules-note-gmail-forward = Isebenza ku-Katna: i-Gmail idlulisela kuphela kumakheli aqinisekisiwe ezilungiselelweni zayo, futhi u-{ $address } akalona elinjalo.
rules-note-gmail-folder = Isebenza ku-Katna: i-Gmail ayinayo ilebula yefolda esetshenziswa yilo mthetho.
rules-note-sieve-folder = Isebenza ku-Katna: iseva yakho yemeyili ayinayo ifolda esetshenziswa yilo mthetho.
rules-note-gmail-sign-in = Isebenza ku-Katna uze ungene ku-Google futhi bese uvumela i-Katna yenze izihlungi ze-Gmail.
rules-note-sieve-other-script = Isebenza ku-Katna: esinye isikripthi semithetho (“{ $name }”) siyasebenza kuseva yakho yemeyili.
rules-note-gmail-failed = Isebenza ku-Katna: i-Gmail ayiwamukelanga ({ $error }).
rules-note-sieve-failed = Isebenza ku-Katna: iseva yakho yemeyili ayiwamukelanga ({ $error }).
rules-editor-cancel = Khansela
rules-editor-save = Londoloza
rules-editor-saving = Iyalondoloza…
rules-editor-delete = Susa umthetho
rules-editor-delete-ask = Susa lo mthetho?
rules-editor-delete-keep = Wugcine
rules-editor-delete-confirm = Susa
rules-editor-needs-folder = Khetha ifolda ku-“Hambisa ku-” ngamunye kanye nelebula ku-“Engeza ilebula” ngamunye.
rules-editor-needs-days = “Maka njengokufundiwe emva kwe-” kudinga inani lezinsuku, kusuka ku-1 kuya ku-3650.
rules-saved = Umthetho ulondoloziwe
rules-saved-applied = { $count ->
    [one] Umthetho ulondoloziwe futhi wasetshenziswa emeyilini engu-{ $count }
   *[other] Umthetho ulondoloziwe futhi wasetshenziswa kumameyili angu-{ $count }
}
rules-apply-failed = Umthetho ulondoloziwe, kodwa ukuwusebenzisa kuhlulekile: { $error }
rules-deleted = Umthetho ususiwe
rules-delete-failed = Akukwazekanga ukususa umthetho: { $error }
rules-change-failed = Akukwazekanga ukushintsha imithetho: { $error }
