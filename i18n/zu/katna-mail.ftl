# Katna Mail, Zulu (isiZulu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Ulimi: { $language }
language-tooltip-system = Ulimi: { $language }, kulandela isistimu
language-search = Sesha ulimi
language-system-default = Okuzenzakalelayo kwesistimu
language-system-now = Manje { $language }
language-no-match = Akukho ulimi oluhambisana nokuthi “{ $query }”
language-machine = Kuhunyushwe ngomshini. Siza ukuthuthukisa
language-setting = Ulimi
language-setting-detail = Ulimi lwamamenyu, izinkinobho nemilayezo, nefomethi yezinsuku nezinombolo. Okuzenzakalelayo kwesistimu kulandela ideskithophu.

## Dates and sizes

ago-just-now = manje nje
ago-minutes = { $count ->
    [one] umzuzu ongu-{ $count } odlule
   *[other] imizuzu engu-{ $count } edlule
}
ago-hours = { $count ->
    [one] ihora elingu-{ $count } eledlule
   *[other] amahora angu-{ $count } adlule
}
ago-days = { $count ->
    [one] usuku olungu-{ $count } oludlule
   *[other] izinsuku ezingu-{ $count } ezedlule
}
size-bytes = { $count ->
    [one] ibhayithi elingu-{ $count }
   *[other] amabhayithi angu-{ $count }
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = Fihla amafolda
folders-show = Bonisa amafolda
compose = Bhala
search = Sesha
search-mail = Sesha imeyili
search-settings = Sesha izilungiselelo
search-clear = Sula usesho
search-options-show = Bonisa okukhethwa kukho kosesho
settings = Izilungiselelo
account-add = Engeza i-akhawunti

## App rail (and the bottom bar on a phone)

rail-mail = Imeyili
rail-calendar = Ikhalenda
rail-contacts = Oxhumana nabo
rail-tasks = Imisebenzi
rail-notes = Amanothi
rail-feeds = Okuphakelayo

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Kuyeza maduze
app-calendar-promise = Amakhalenda akho e-CalDAV, izimemo zemihlangano ezivela kumeyili yakho nezikhumbuzi, eduze kwebhokisi lakho lokungenayo.
app-tasks-promise = Uhlu lwezinto okufanele zenziwe oluvumelana ne-CalDAV, nemisebenzi eyenziwe kusuka kumeyili.
app-notes-promise = Amanothi asheshayo, namanothi ngemeyili noma ngengxoxo ukuze uwasebenzise kamuva.
app-feeds-promise = Funda okuphakelayo kwe-RSS ne-Atom eduze kwemeyili yakho.

## Contacts page

app-contacts-loading = Kuqoqwa abantu kusuka kumeyili yakho…
app-contacts-empty = Abantu obhalelana nabo bazovela lapha.
app-contacts-count = { $count ->
    [one] Umuntu ongu-{ $count } ovela kumeyili yakho, obhalelana nabo kakhulu kuqala
   *[other] Abantu abangu-{ $count } abavela kumeyili yakho, obhalelana nabo kakhulu kuqala
}
app-contacts-top = { $count ->
    [one] Umuntu ophezulu ongu-{ $count } ovela kumeyili yakho, obhalelana nabo kakhulu kuqala
   *[other] Abantu abaphezulu abangu-{ $count } abavela kumeyili yakho, obhalelana nabo kakhulu kuqala
}
app-contacts-messages = { $count ->
    [one] umlayezo ongu-{ $count }
   *[other] imilayezo engu-{ $count }
}
app-contacts-last = okokugcina { $date }

## Navigation (the folders pane)

nav-labels = Amalebula
nav-folders = Amafolda
nav-label-new = Dala ilebula entsha
nav-folder-new = Dala ifolda entsha
nav-account-unnamed = I-akhawunti { $number }
nav-tab-new = { $count ->
    [one] { $count } okusha
   *[other] { $count } okusha
}

## Special folders (the user's own folders keep their names)

folder-inbox = Ibhokisi lokungenayo
folder-starred = Okunenkanyezi
folder-drafts = Okusalungiswa
folder-sent = Okuthunyelwe
folder-archive = Ingobo yomlando
folder-spam = Ugaxekile
folder-trash = Udoti
folder-all-mail = Wonke amameyili
folder-scheduled = Okuhleliwe

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Ilebula entsha
label-folder-new-title = Ifolda entsha
label-prompt = Sicela ufake igama elisha lelebula:
label-folder-prompt = Sicela ufake igama elisha lefolda:
label-name-hint = Igama lelebula
label-folder-name-hint = Igama lefolda
label-nest = Faka ilebula ngaphansi kwe:
label-folder-nest = Faka ifolda ngaphansi kwe:
label-cancel = Khansela
label-create = Dala
label-creating = Iyadala…
label-created = Ilebula ethi “{ $name }” idaliwe.
label-folder-created = Ifolda ethi “{ $name }” idaliwe.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Okuyinhloko
tab-promotions = Ukukhangisa
tab-social = Ezenhlalo
tab-updates = Izibuyekezo
tab-forums = Izinkundla
tab-focused = Okugxilile
tab-other = Okunye
tab-inbox = Ibhokisi lokungenayo
tab-newsletters = Izincwadi zezindaba
tab-notifications = Izaziso
tab-new = { $count } okusha
tab-provider-other = kuhlelwe yi-Katna

## Mail list: toolbar

list-select = Khetha
list-refresh = Vuselela
list-more = Okuningi
list-mark-read = Maka njengokufundiwe
list-mark-unread = Maka njengokungafundiwe
list-move-to = Hambisa ku-
list-archive = Faka kungobo yomlando
list-spam = Bika ugaxekile
list-delete = Susa
list-newer = Okusha
list-older = Okudala
list-range = { $first }–{ $last } kokungu-{ $total }
list-range-about = { $first }–{ $last } kokucishe kube ngu-{ $total }
list-results = Imiphumela ye-“{ $query }”
list-results-corrected = Kuboniswa imiphumela ye-“{ $query }”
list-search-instead = Esikhundleni salokho sesha u-“{ $query }”
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Konke
list-pick-none = Lutho
list-pick-read = Okufundiwe
list-pick-unread = Okungafundiwe
list-pick-starred = Okunenkanyezi
list-pick-unstarred = Okungenankanyezi

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo engu-{ $count } ikhethiwe.
       *[other] Zonke izingxoxo ezingu-{ $count } zikhethiwe.
    }
   *[message] { $count ->
        [one] Umlayezo ongu-{ $count } ukhethiwe.
       *[other] Yonke imilayezo engu-{ $count } ikhethiwe.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo engu-{ $count } ku-{ $folder } ikhethiwe.
       *[other] Zonke izingxoxo ezingu-{ $count } ku-{ $folder } zikhethiwe.
    }
   *[message] { $count ->
        [one] Umlayezo ongu-{ $count } ku-{ $folder } ukhethiwe.
       *[other] Yonke imilayezo engu-{ $count } ku-{ $folder } ikhethiwe.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo engu-{ $count } esesikrinini ikhethiwe.
       *[other] Zonke izingxoxo ezingu-{ $count } ezisesikrinini zikhethiwe.
    }
   *[message] { $count ->
        [one] Umlayezo ongu-{ $count } osesikrinini ukhethiwe.
       *[other] Yonke imilayezo engu-{ $count } esesikrinini ikhethiwe.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Khetha ingxoxo engu-{ $count }
       *[other] Khetha zonke izingxoxo ezingu-{ $count }
    }
   *[message] { $count ->
        [one] Khetha umlayezo ongu-{ $count }
       *[other] Khetha yonke imilayezo engu-{ $count }
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Khetha ingxoxo engu-{ $count } ku-{ $folder }
       *[other] Khetha zonke izingxoxo ezingu-{ $count } ku-{ $folder }
    }
   *[message] { $count ->
        [one] Khetha umlayezo ongu-{ $count } ku-{ $folder }
       *[other] Khetha yonke imilayezo engu-{ $count } ku-{ $folder }
    }
}
list-clear-selection = Sula okukhethiwe

## Mail list: empty states

list-empty-search = Ayikho imilayezo ehambisana nosesho lwakho.
list-empty-tab = Ayikho imeyili ku-{ $tab }.
list-empty-tab-unknown = Ayikho imeyili kule thebhu.
list-empty-folder = Ayikho imilayezo ku-{ $folder }.
list-empty-folder-unknown = Ayikho imilayezo kule folda.
list-first-sync = Kulandwa imeyili yakho…
list-first-sync-detail = Izovela lapha njengoba ifika.

## Mail list: lines

row-removed = Lo mlayezo ususiwe.
row-starred = Kunenkanyezi
row-not-starred = Akunankanyezi
row-important = Kubalulekile. Chofoza ukuze umake njengokungabalulekile.
row-mark-important = Maka njengokubalulekile
row-pinned = Kuphinwe phezulu
row-pin = Phina phezulu
row-unpin = Susa ukuphina

## Mail list: More menu and right-click menu

menu-reply = Phendula
menu-reply-all = Phendula bonke
menu-forward = Dlulisela
menu-archive = Faka kungobo yomlando
menu-delete = Susa
menu-spam = Bika ugaxekile
menu-mark-read = Maka njengokufundiwe
menu-mark-unread = Maka njengokungafundiwe
menu-mark-all-read = Maka konke njengokufundiwe
menu-star = Engeza inkanyezi
menu-unstar = Susa inkanyezi
menu-important = Maka njengokubalulekile
menu-not-important = Maka njengokungabalulekile
menu-pin = Phina phezulu
menu-unpin = Susa ukuphina
menu-print-all = Phrinta konke
menu-new-window = Vula ewindini elisha
menu-move-to = Hambisa ku-
menu-move-to-heading = Hambisa ku:
menu-find-from = Thola ama-imeyili avela ku-{ $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo ifakwe kungobo yomlando.
       *[other] Izingxoxo ezingu-{ $count } zifakwe kungobo yomlando.
    }
   *[message] { $count ->
        [one] Umlayezo ufakwe kungobo yomlando.
       *[other] Imilayezo engu-{ $count } ifakwe kungobo yomlando.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo ihanjiswe kudoti.
       *[other] Izingxoxo ezingu-{ $count } zihanjiswe kudoti.
    }
   *[message] { $count ->
        [one] Umlayezo uhanjiswe kudoti.
       *[other] Imilayezo engu-{ $count } ihanjiswe kudoti.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo ihanjisiwe.
       *[other] Izingxoxo ezingu-{ $count } zihanjisiwe.
    }
   *[message] { $count ->
        [one] Umlayezo uhanjisiwe.
       *[other] Imilayezo engu-{ $count } ihanjisiwe.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo ifakwe inkanyezi.
       *[other] Izingxoxo ezingu-{ $count } zifakwe inkanyezi.
    }
   *[message] { $count ->
        [one] Umlayezo ufakwe inkanyezi.
       *[other] Imilayezo engu-{ $count } ifakwe inkanyezi.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Inkanyezi isusiwe engxoxweni.
       *[other] Inkanyezi isusiwe ezingxoxweni ezingu-{ $count }.
    }
   *[message] { $count ->
        [one] Inkanyezi isusiwe emlayezweni.
       *[other] Inkanyezi isusiwe emilayezweni engu-{ $count }.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo imakwe njengebalulekile.
       *[other] Izingxoxo ezingu-{ $count } zimakwe njengezibalulekile.
    }
   *[message] { $count ->
        [one] Umlayezo umakwe njengobalulekile.
       *[other] Imilayezo engu-{ $count } imakwe njengebalulekile.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo imakwe njengengabalulekile.
       *[other] Izingxoxo ezingu-{ $count } zimakwe njengezingabalulekile.
    }
   *[message] { $count ->
        [one] Umlayezo umakwe njengongabalulekile.
       *[other] Imilayezo engu-{ $count } imakwe njengengabalulekile.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo iphinwe phezulu.
       *[other] Izingxoxo ezingu-{ $count } ziphinwe phezulu.
    }
   *[message] { $count ->
        [one] Umlayezo uphinwe phezulu.
       *[other] Imilayezo engu-{ $count } iphinwe phezulu.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Ukuphina kwengxoxo kususiwe.
       *[other] Ukuphina kwezingxoxo ezingu-{ $count } kususiwe.
    }
   *[message] { $count ->
        [one] Ukuphina komlayezo kususiwe.
       *[other] Ukuphina kwemilayezo engu-{ $count } kususiwe.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo ibikwe njengogaxekile.
       *[other] Izingxoxo ezingu-{ $count } zibikwe njengogaxekile.
    }
   *[message] { $count ->
        [one] Umlayezo ubikwe njengogaxekile.
       *[other] Imilayezo engu-{ $count } ibikwe njengogaxekile.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Ingxoxo isuswe unomphela.
       *[other] Izingxoxo ezingu-{ $count } zisuswe unomphela.
    }
   *[message] { $count ->
        [one] Umlayezo ususwe unomphela.
       *[other] Imilayezo engu-{ $count } isuswe unomphela.
    }
}
toast-undone = Isenzo sihlehlisiwe.
toast-undo = Hlehlisa
toast-no-spam-folder = Le akhawunti ayinayo ifolda kagaxekile.

## Reading pane: toolbar

reader-close = Vala
reader-back = Emuva
reader-mark-unread = Maka njengokungafundiwe
reader-move-to = Hambisa ku-
reader-more = Okuningi
reader-print-all = Phrinta konke
reader-new-window = Ewindini elisha
reader-position = { $position } kokungu-{ $total }
reader-newer = Okusha
reader-older = Okudala

## Reading pane: the conversation

reader-removed = Le ngxoxo isusiwe.
reader-no-subject = (asikho isihloko)
reader-collapse-all = Goqa konke
reader-expand-all = Nweba konke
reader-unknown-sender = (umthumeli ongaziwa)
reader-date-ago = { $date } ({ $ago })
reader-me = mina
reader-to = ku-{ $names }
reader-starred = Kunenkanyezi
reader-not-starred = Akunankanyezi
reader-too-long = Umlayezo mude kakhulu ukuthi uboniswe wonke.
reader-encrypted-images = Izithombe ezivela kuwebhu azilayishwa neze kumeyili ebethelwe.
reader-window-failed = Ayikwazanga ukuvula iwindi elisha.

## Reading pane: message details (opened from "to me")

reader-details-from = kusuka ku:
reader-details-to = ku:
reader-details-cc = cc:
reader-details-date = usuku:
reader-details-subject = isihloko:

## Reading pane: downloading a message

reader-downloading = Ilanda lo mlayezo kuseva…
reader-download-failed = Ayikwazanga ukulanda lo mlayezo.
reader-try-again = Zama futhi

## Reply row

reply-reply = Phendula
reply-reply-all = Phendula bonke
reply-forward = Dlulisela

## Encrypted and signed mail

security-decrypting = Iyasusa ukubethela…
security-checking = Ihlola isiginesha…
security-partly-encrypted = Yingxenye kuphela yalo mlayezo ebethelwe. Okunye kwengezwe ngaphandle kwesivikelo futhi kungavela kunoma ubani.
security-partly-signed = Yingxenye kuphela yalo mlayezo esayiniwe. Okunye kwengezwe ngaphandle kwesivikelo futhi kungavela kunoma ubani.
security-encrypted = Umlayezo obethelwe
security-encrypted-smime = Umlayezo obethelwe (S/MIME)
security-no-key = Ayikwazi ukususa ukubethela kulo mlayezo: ubethelwe ngokhiye ongenawo.
security-cancelled = Ukususa ukubethela kukhanseliwe.
security-damaged = Ayikwazi ukususa ukubethela kulo mlayezo: idatha ebethelwe yonakele noma ishintshiwe.
security-decrypt-unavailable = Ayikwazi ukususa ukubethela kulo mlayezo: faka i-{ $tool } ukuze ufunde imeyili ebethelwe.
security-decrypt-failed = Ayikwazi ukususa ukubethela kulo mlayezo: { $reason }
security-unknown-signer = umsayini ongaziwa
security-signed-verified = Kusayinwe ngu-{ $signer } · kuqinisekisiwe
security-signed-not-sender = Kusayinwe ngu-{ $signer }, ongesiye umthumeli
security-signed-untrusted = Kusayinwe ngu-{ $signer }, ngokhiye owumake njengongathembekile
security-signed-unverified = Kusayinwe ngu-{ $signer } · ukhiye awuqinisekisiwe
security-bad-signature = Isiginesha embi: lo mlayezo ushintshwe ngemva kokusayinwa, noma isiginesha ingumgunyathi.
security-signature-expired = Kusayinwe ngu-{ $signer } · isiginesha iphelelwe yisikhathi
security-key-expired = Kusayinwe ngu-{ $signer } · ukhiye usuphelelwe yisikhathi kusukela lapho
security-key-revoked = Kusayinwe ngu-{ $signer } ngokhiye ohoxisiwe
security-missing-key = Kusayinwe ngokhiye ongenawo, ngakho akukwazi ukuhlolwa
security-missing-key-id = Kusayinwe ngokhiye ongenawo ({ $key }), ngakho akukwazi ukuhlolwa
security-signature-unavailable = Kusayiniwe; faka i-{ $tool } ukuze uhlole isiginesha
security-signature-error = Isiginesha ayikwazanga ukuhlolwa.

## Remote images and pictures

remote-hidden = Izithombe kulo mlayezo zifihliwe.
remote-show = Bonisa izithombe
remote-always-show = Bonisa njalo kusuka kulo mthumeli
remote-picture-use = Sebenzisa
remote-picture-too-big = Khetha isithombe esingu-8 MB noma ngaphansi.
remote-picture-type = Khetha isithombe se-PNG, JPEG, GIF, WebP noma SVG.
remote-picture-read-failed = Ayikwazi ukufunda isithombe: { $error }
remote-picture-keep-failed = Ayikwazi ukugcina isithombe: { $error }
remote-picture-remove-failed = Ayikwazi ukususa isithombe: { $error }

## Attachments

attachment-count = { $count ->
    [one] Okunamathiselwe okukodwa
   *[other] Okunamathiselwe okungu-{ $count }
}
attachment-save = Londoloza
attachment-save-all = Londoloza konke
attachment-save-all-tooltip = Londoloza konke okunamathiselwe kufolda
attachment-save-here = Londoloza lapha
attachment-not-downloaded = Lo mlayezo awulandiwe.
attachment-not-found = Lokhu okunamathiselwe akutholakalanga emlayezweni.
attachment-read-failed = Ayikwazanga ukufunda i-{ $name }
attachment-numbered = okunamathiselwe { $number }
attachment-saved-all = { $count ->
    [one] Kulondolozwe ifayela elingu-{ $count } ku-{ $place }
   *[other] Kulondolozwe amafayela angu-{ $count } ku-{ $place }
}
attachment-saved-some = { $total ->
    [one] Kulondolozwe angu-{ $saved } kwifayela elingu-{ $total } ku-{ $place }. Ayikwazanga ukulondoloza i-{ $failed }
   *[other] Kulondolozwe angu-{ $saved } kwamafayela angu-{ $total } ku-{ $place }. Ayikwazanga ukulondoloza i-{ $failed }
}
attachment-saved-to = Kulondolozwe ku-{ $path }
attachment-save-failed = Ayikwazanga ukulondoloza i-{ $name }: { $error }
attachment-open-failed = Ayikwazanga ukuvula i-{ $name }: { $error }
attachment-risky = Leli fayela lingaqalisa uhlelo, ngakho i-Katna ayilivuli. Kunalokho lilondoloze.
attachment-encrypted-open = Leli fayela lifike libethelwe. Lilondoloze ukuze ulivule kwenye indawo.

## Printing

print-failed = Ayikwazanga ukuphrinta: { $error }
print-no-font = alikho ifonti elitholakele
print-opened-as-pdf = Kuvulwe njenge-PDF ukuze uphrinte ukusuka lapho.
print-not-downloaded = (Akukalandwa.)
print-encrypted = (Kubethelwe. Kuvule ku-Katna Mail ukuze uphrinte umbhalo wakho.)
print-to = Ku: { $addresses }
print-cc = Cc: { $addresses }
