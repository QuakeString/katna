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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Vula lo mlayezo ukuze ufunde okunamathiselwe kuwo.
text-copy = Kopisha
text-select-all = Khetha konke

## Settings page: its tabs

settings-tab-general = Okuvamile
settings-tab-inbox = Ibhokisi lokungenayo
settings-tab-accounts = Ama-akhawunti
settings-tab-subscriptions = Okubhaliselwe
settings-tab-appearance = Ukubukeka
settings-tab-shortcuts = Izinqamuleli
settings-tab-default-apps = Ama-app azenzakalelayo
settings-tab-folders-rules = Amafolda nemithetho
settings-tab-compose = Bhala
settings-tab-mcp-server = Iseva ye-MCP
settings-tab-feedback = Impendulo yomsebenzisi
settings-tab-experimental = Okokuhlola

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Bona izincwadi zezindaba nohlu lwamameyili ozitholayo, bese uzikhansela ngokuchofoza kanye.
settings-tab-folders-rules-coming = Dala, qamba kabusha, hambisa futhi ufihle amafolda namalebula, bese ukhetha ukuthi yimaphi avumelaniswayo. Imithetho ihlela, ifaka amalebula, idlulisela noma isuse imeyili entsha ngokwayo, ngokomthumeli, isihloko noma amagama.
settings-tab-mcp-server-coming = Vumela abasizi be-AI kule khompyutha ukuthi baseshe, bafunde futhi babhale okusalungiswa kwemeyili yakho, ngemvume yakho.

## Settings > General

settings-general-conversations = Ukubuka kwengxoxo
settings-general-conversations-group = Qoqa ndawonye izimpendulo zemeyili efanayo
settings-general-conversations-group-detail = Umugqa owodwa wengxoxo ngayinye ohlwini
settings-general-reading = Ukufunda
settings-general-newest-first = Umlayezo omusha kakhulu kuqala
settings-general-newest-first-detail = Ingxoxo iqala ngempendulo yayo yakamuva
settings-general-full-headers = Bonisa amakhanda aphelele
settings-general-full-headers-detail = Kusuka ku, ku, cc, usuku nesihloko kuvuleka kuwo wonke umlayezo
settings-general-full-names = Amagama aphelele abamukeli
settings-general-full-names-detail = “ku-mina, Ada Lovelace” esikhundleni sokuthi “ku-mina, Ada”
settings-general-mark-read = Maka njengokufundiwe
settings-general-mark-read-now = Ngokushesha nje uma uvuleka
settings-general-mark-read-1s = Ngemva kokuvuleka isekhondi elingu-1
settings-general-mark-read-3s = Ngemva kokuvuleka imizuzwana engu-3
settings-general-mark-read-never = Kuphela uma ngiwumaka njengofundiwe
settings-general-reply-button = Inkinobho yokuphendula
settings-general-reply-all = Phendula wonke umuntu
settings-general-reply-all-detail = Inkinobho yokuphendula eduze komlayezo ngamunye iphendula bonke, hhayi umthumeli kuphela
settings-general-remote-images = Izithombe ezivela kuwebhu
settings-general-remote-images-detail = Ukulayisha izithombe zomlayezo kutshela umthumeli wawo ukuthi uwuvulile, nini, futhi cishe ukuphi. Uma kuvaliwe, umlayezo ngamunye ubuza kuqala, futhi ungahlala ubonisa izithombe zomthumeli.
settings-general-remote-images-always = Bonisa izithombe njalo
settings-general-remote-images-always-detail = Kuyo yonke imilayezo, hhayi kuphela evela kubathumeli obathembayo
settings-general-sending = Ukuthumela
settings-general-sending-detail = Isikhathi umlayezo othunyelwe olinda ngaso, ukuze ukwazi ukuhoxiswa.
settings-general-offline = Imeyili yokungaxhunyiwe
settings-general-offline-detail = Imeyili yakamuva ilandwa iphelele, ukuze ifundwe ngaphandle koxhumano. Imeyili endala ilandwa uma uyivula.
settings-general-offline-days = { $count ->
    [one] Usuku olungu-{ $count }
   *[other] Izinsuku ezingu-{ $count }
}
settings-general-offline-years = { $count ->
    [one] Unyaka ongu-{ $count }
   *[other] Iminyaka engu-{ $count }
}
settings-general-offline-all = Yonke imeyili
settings-general-offline-note = Ukukhetha izinsuku ezimbalwa kugcina imeyili esivele ilandiwe. Akukho okushintshayo kuseva.
settings-general-notifications = Izaziso
settings-general-notifications-detail = Ngemeyili entsha ebhokisini lokungenayo, ngisho noma i-Katna Mail ivaliwe.
settings-general-new-mail = Ngazise ngemeyili entsha
settings-general-new-mail-detail = Nezinkinobho ezithi Phendula bonke, Maka njengokufundiwe nokuthi Faka kungobo yomlando
settings-general-new-mail-sound = Dlala umsindo
settings-general-new-mail-sound-detail = Umsindo wemeyili entsha wedeskithophu
settings-general-desktop = Ideskithophu
settings-general-open-at-login = Vula i-Katna Mail lapho ungena
settings-general-open-at-login-detail = Imeyili iyavumelanisa lapho ungena noma kunjalo, uma nje isevisi isebenza
settings-general-tray = Bonisa i-Katna kuthileyi yesistimu
settings-general-tray-detail = Nesibalo sokungafundiwe nemenyu
settings-general-unread-badge = Isibalo sokungafundiwe esithonjaneni sebha yemisebenzi
settings-general-unread-badge-detail = Ingakanani imilayezo yebhokisi lokungenayo engafundiwe

## Settings > Inbox

settings-inbox-tabs = Amathebhu ebhokisi lokungenayo
settings-inbox-tabs-detail = Hlela ibhokisi lokungenayo libe amathebhu, njengoba kwenza iwebhusayithi yomhlinzeki wakho wemeyili.
settings-inbox-tabs-show = Bonisa amathebhu ebhokisi lokungenayo
settings-inbox-tabs-show-detail = Uma kuvaliwe, kuboniswa uhlu olulodwa ku-akhawunti ngayinye
settings-inbox-no-accounts = Engeza i-akhawunti ukuze ukhethe amathebhu ayo.
settings-inbox-tabs-automatic = Ngokuzenzakalela: { $tabs } ({ $provider })
settings-inbox-tabs-off = Awekho amathebhu
settings-inbox-tabs-gmail = Okuyinhloko, Ukukhangisa, Ezenhlalo, Izibuyekezo, Izinkundla
settings-inbox-tabs-focused = Okugxilile nokunye
settings-inbox-tabs-zoho = Ibhokisi lokungenayo, Izincwadi zezindaba nezaziso
settings-inbox-tabs-shown = Amathebhu aboniswayo. Imeyili yethebhu oyivalayo ihlala ku-{ $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Iphaneli yokufunda
settings-appearance-reading-pane-detail = Lapho ingxoxo evuliwe iboniswa khona.
settings-appearance-pane-right = Ngakwesokudla sohlu
settings-appearance-pane-none = Akukho ukuhlukanisa
settings-appearance-density = Ukuminyana
settings-appearance-density-default = Okuzenzakalelayo
settings-appearance-density-compact = Okuminyene
settings-appearance-scaling = Ukukala
settings-appearance-scaling-detail = Yenza konke ku-Katna Mail kube kukhulu noma kube kuncane, ngaphezu kokukala kwedeskithophu uqobo: umbhalo, izithonjana, izikhala nemigqa ehlukanisayo. Imeyili oyithumelayo igcina usayizi wayo wefonti. Osayizi abancane kakhulu bangenza kube nzima ukuchofoza izithonjana.
settings-appearance-theme = Itimu
settings-appearance-theme-system = Kufana nedeskithophu
settings-appearance-theme-light = Ekhanyayo
settings-appearance-theme-dark = Emnyama
settings-appearance-desktop-colors = Imibala yedeskithophu
settings-appearance-desktop-colors-use = Sebenzisa imibala yedeskithophu
settings-appearance-desktop-colors-use-detail = Uhlelo lwemibala nombala wokugcizelela wedeskithophu
settings-appearance-app-names = Amagama ama-app
settings-appearance-app-names-show = Bonisa amagama ama-app
settings-appearance-app-names-show-detail = Amagama angaphansi kwezithonjana zama-app ngakwesobunxele kakhulu
settings-appearance-sender-pictures = Izithombe zabathumeli
settings-appearance-sender-pictures-show = Bonisa amalogo ezinkampani
settings-appearance-sender-pictures-show-detail = Kubhekwa ngesizinda somthumeli, hhayi neze ngomlayezo, futhi kugcinwa isonto
settings-appearance-important = Izimpawu zokubalulekile
settings-appearance-important-show = Bonisa izimpawu zokubalulekile
settings-appearance-important-show-detail = Eduze komlayezo ngamunye ohlwini
settings-appearance-message-width = Ububanzi bomlayezo
settings-appearance-message-width-limit = Khawulela ububanzi bemilayezo
settings-appearance-message-width-limit-detail = Imigqa emide kulula ukuyifunda ewindini elibanzi
settings-appearance-mail-colors = Imibala yemeyili
settings-appearance-mail-colors-detail = Iningi lemeyili lidizayinelwe ikhasi elimhlophe. Ngetimu emnyama imibala yayo ishintshwa ibe emnyama efundeka kahle; uma kuvaliwe, igcina imibala yomthumeli wayo ekhasini elikhanyayo.
settings-appearance-dark-mail = Imibala emnyama nakumeyili
settings-appearance-dark-mail-detail = Kuphela uma itimu imnyama
settings-appearance-attachment-previews = Ukubuka kuqala okunamathiselwe
settings-appearance-attachment-previews-show = Bonisa ukubuka kuqala kokunamathiselwe
settings-appearance-attachment-previews-show-detail = Isithombe esincane sokuqukethwe yifayela ngalinye ekhadini lalo

## Settings > Default apps

settings-default-apps-intro = Lapho okunamathiselwe kuvuleka khona uma ukuchofoza. Isibukeli singahlala sivula ifayela nakwenye i-app. Ama-app azenzakalelayo edeskithophu asethwa kuzilungiselelo zayo.
settings-default-apps-pdf = Amafayela e-PDF
settings-default-apps-pdf-detail = Amakhasi, nokusondeza.
settings-default-apps-pictures = Izithombe
settings-default-apps-pictures-detail = Izithombe zekhamera (eziqondisiwe), PNG, GIF, WebP, BMP, TIFF ne-SVG.
settings-default-apps-text = Amafayela ombhalo
settings-default-apps-text-detail = Umbhalo osobala, amalogi, ikhodi nomunye umbhalo.
settings-default-apps-sheets = Amaspredishithi
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) ne-CSV.
settings-default-apps-documents = Amadokhumenti
settings-default-apps-documents-detail = Word (docx) nombhalo we-OpenDocument (odt).
settings-default-apps-katna = Isibukeli se-Katna Mail
settings-default-apps-system = I-app ezenzakalelayo yedeskithophu
settings-default-apps-ask = Buza ukuthi yiyiphi i-app njalo
settings-default-apps-after-saving = Ngemva kokulondoloza
settings-default-apps-show-folder = Bonisa amafayela alondoloziwe kufolda yawo
settings-default-apps-show-folder-detail = Ivula isiphathi samafayela okunamathiselwe okulondoloziwe kukhethiwe

## Settings > Compose

settings-compose-send-from = Thumela imilayezo emisha ivela ku-
settings-compose-send-from-detail = Izimpendulo nokudluliselwayo kuhlala kuphuma ku-akhawunti okuyo.
settings-compose-send-from-current = I-akhawunti okuyo
settings-compose-send-on-replies = Thumela ezimpendulweni
settings-compose-send-on-replies-detail = Lokho okwenziwa ngu-Thumela empendulweni noma kokudluliselwayo. Imenyu eduze kuka-Thumela inikeza okunye.
settings-compose-send-plain = Thumela
settings-compose-send-archive = Thumela futhi ufake kungobo yomlando
settings-compose-signatures = Amasiginesha
settings-compose-signatures-detail = Kungezwa ngaphansi komlayezo wakho, ngemva komugqa othi “--”. Khetha elinye ewindini lokubhala.
settings-compose-untitled = Akunasihloko
settings-compose-signature-name = Igama, njengokuthi Umsebenzi
settings-compose-signature-first = Isiginesha yami
settings-compose-signature-numbered = Isiginesha { $number }
settings-compose-signature-delete = Susa
settings-compose-signature-deleted = Isiginesha isusiwe
settings-compose-signature-new = Dala entsha
settings-compose-no-signatures = Awekho amasiginesha okwamanje.
settings-compose-no-signature = Ayikho isiginesha
settings-compose-for-new-mail = Okwemeyili entsha
settings-compose-for-replies = Okwezimpendulo nokudluliselwayo
settings-compose-for-replies-detail = Engxoxweni lapho usayine khona umlayezo, impendulo iqala ngaleyo siginesha esikhundleni salokho.
settings-compose-format = Ifomethi
settings-compose-plain-text = Bhala ngombhalo osobala
settings-compose-plain-text-detail = Imeyili entsha iqala ngaphandle kokufometha; iwindi lokubhala lingashintsha
settings-compose-spelling = Ukupela
settings-compose-spell-check = Hlola ukupela ngenkathi ngibhala
settings-compose-spell-check-detail = Amagama apelwe kabi adwetshelwa, neziphakamiso uma uchofoza kwesokudla
settings-compose-spell-desktop = Ulimi lwedeskithophu ({ $language })
settings-compose-templates = Izifanekiso
settings-compose-templates-detail = Londoloza imeyili oyibhala kaningi, bese uqala imeyili entsha noma impendulo ngayo.

## Settings > Shortcuts

settings-shortcuts-set = Isethi yezinqamuleli
settings-shortcuts-set-detail = Qala ngokhiye be-app yemeyili oyaziyo. I-Cmd ngu-Ctrl lapha. Izinguquko zakho zihlala ngaphezu kwesethi, futhi u-Buyisela okuzenzakalelayo ubuyela kokhiye besethi.
settings-shortcuts-single = Izinqamuleli zokhiye oyedwa
settings-shortcuts-single-detail = Okhiye abangenayo i-Ctrl noma i-Alt, njengakuwebhumeyili: u-e ufaka kungobo yomlando, u-j no-k bayahambisa, u-/ uyasesha. Zisebenza ohlwini nasengxoxweni evuliwe, hhayi neze ngenkathi uthayipha.
settings-shortcuts-single-use = Sebenzisa izinqamuleli zokhiye oyedwa
settings-shortcuts-single-use-detail = Izinqamuleli ze-Ctrl zihlala zisebenza
settings-shortcuts-how = Chofoza ukhiye ukuze uwushintshe, noma u-+ ukuze wengeze omunye, bese ucindezela okhiye abasha. U-Esc uyakhansela.
settings-shortcuts-restore = Buyisela okuzenzakalelayo
settings-shortcuts-no-key = Akukho khiye
settings-shortcuts-press = Cindezela okhiye…
settings-shortcuts-then = { $keys } bese…
settings-shortcuts-moved = { $keys } manje wenza “{ $action }” esikhundleni sika-“{ $previous }”.
settings-shortcuts-single-off = Izinqamuleli zokhiye oyedwa zivaliwe, ngakho lo khiye uzosebenza uma sezivuliwe.
settings-shortcuts-restored = Isinqamuleli ngasinye sesinokhiye besethi yaso futhi.

## Settings search: the line under a result

settings-general-language-summary = Ulimi lwe-app, izinsuku nezinombolo
settings-general-reading-summary = Umlayezo omusha kakhulu kuqala, amakhanda aphelele, amagama aphelele abamukeli
settings-general-mark-read-summary = Lapho ingxoxo evuliwe imakwa njengefundiwe: ngokushesha, ngemva kwemizuzwana engu-1 noma engu-3, noma ngesandla
settings-general-reply-button-summary = Inkinobho yokuphendula eduze komlayezo ngamunye iphendula wonke umuntu
settings-general-remote-images-summary = Hlala ubonisa izithombe zawo wonke umlayezo
settings-general-sending-summary = Hlehlisa ukuthumela: isikhathi umlayezo othunyelwe olinda ngaso, ukuze ukwazi ukuhoxiswa
settings-general-offline-summary = Zingaki izinsuku zemeyili yakamuva ezilandwa ziphelele, ukuze ifundwe ngaphandle koxhumano
settings-general-notifications-summary = Izaziso zemeyili entsha nomsindo wazo
settings-general-desktop-summary = Vula i-Katna Mail lapho ungena, isithonjana sethileyi yesistimu nesibalo sokungafundiwe esithonjaneni sebha yemisebenzi
settings-accounts-accounts-summary = Engeza noma ususe i-akhawunti, noma ushintshe isithombe sayo
settings-appearance-density-summary = Imigqa ezenzakalelayo noma eminyene ohlwini
settings-appearance-scaling-summary = Yenza konke kube kukhulu noma kube kuncane: umbhalo, izithonjana, izikhala nemigqa ehlukanisayo
settings-appearance-theme-summary = Kufana nedeskithophu, ekhanyayo noma emnyama
settings-appearance-sender-pictures-summary = Amalogo ezinkampani, abhekwa ngesizinda somthumeli
settings-appearance-important-summary = Uphawu lokubalulekile eduze komlayezo ngamunye ohlwini
settings-appearance-mail-colors-summary = Imibala emnyama yemeyili ye-HTML etimini emnyama, noma imibala yomthumeli wayo
settings-appearance-attachment-previews-summary = Isithombe esincane sokuqukethwe yilokho okunamathiselwe ngakunye
settings-shortcuts-set-summary = Qala ngokhiye be-Gmail, Inbox by Gmail, Apple Mail, Outlook noma Thunderbird
settings-shortcuts-single-summary = Okhiye abangenayo i-Ctrl noma i-Alt, njengakuwebhumeyili
settings-default-apps-pdf-summary = Lapho okunamathiselwe kwe-PDF kuvuleka khona
settings-default-apps-pictures-summary = Lapho izithombe zekhamera nezinye izithombe zivuleka khona
settings-default-apps-text-summary = Lapho umbhalo osobala, amalogi nekhodi kuvuleka khona
settings-default-apps-sheets-summary = Lapho amafayela e-Excel, OpenDocument ne-CSV avuleka khona
settings-default-apps-documents-summary = Lapho umbhalo we-Word ne-OpenDocument uvuleka khona
settings-default-apps-after-saving-summary = Bonisa okunamathiselwe okulondoloziwe kufolda yakho
settings-compose-send-from-summary = I-akhawunti imeyili entsha ephuma kuyo: leyo okuyo, noma efanayo njalo
settings-compose-send-on-replies-summary = Thumela, noma Thumela futhi ufake ingxoxo kungobo yomlando, ezimpendulweni nakokudluliselwayo
settings-compose-signatures-summary = Kungezwa ngaphansi komlayezo wakho, ngemva komugqa othi “--”
settings-compose-for-new-mail-summary = Isiginesha imeyili entsha eqala ngayo
settings-compose-for-replies-summary = Isiginesha izimpendulo nokudluliselwayo okuqala ngayo
settings-compose-format-summary = Bhala imeyili entsha ngombhalo osobala
settings-compose-spelling-summary = Hlola ukupela ngenkathi ubhala, nolimi lwesichazamazwi
settings-compose-templates-summary = Kuyeza maduze: londoloza imeyili oyibhala kaningi, bese uqala imeyili entsha noma impendulo ngayo
settings-feedback-crash-reports-summary = Londoloza imibiko yokuphahlazeka kule khompyutha uma i-Katna Mail noma isevisi yayo yangemuva iphahlazeka
settings-feedback-saved-summary = Buka, kopisha noma susa imibiko yokuphahlazeka elondolozwe kule khompyutha
settings-feedback-help-improve-summary = Thumela imibiko yokuphahlazeka ukuze usize ukulungisa inkinga; kuvaliwe ngaphandle uma ukuvula
settings-experimental-blur-summary = Ideskithophu iyabonakala ngebha ephezulu, ifiphele, futhi amamenyu afana nengilazi efiphele
settings-search-shortcut = Isinqamuleli sekhibhodi
settings-search-tab = Ithebhu yezilungiselelo
settings-search-none = Azikho izilungiselelo ezihambisana nokuthi “{ $query }”.
settings-search-results = Izilungiselelo ezihambisana nokuthi “{ $query }”

## Quick settings (the panel that slides in from the right)

quick-title = Izilungiselelo ezisheshayo
quick-see-all = Bona zonke izilungiselelo
quick-reading-pane = Iphaneli yokufunda
quick-pane-right = Ngakwesokudla sohlu
quick-pane-none = Akukho ukuhlukanisa
quick-density = Ukuminyana
quick-density-default = Okuzenzakalelayo
quick-density-compact = Okuminyene
quick-theme = Itimu
quick-theme-system = Kufana nedeskithophu
quick-theme-light = Ekhanyayo
quick-theme-dark = Emnyama
quick-desktop-colors = Imibala yedeskithophu
quick-desktop-colors-detail = Uhlelo lwemibala nombala wokugcizelela wedeskithophu
quick-app-names = Amagama ama-app
quick-app-names-detail = Amagama angaphansi kwezithonjana zama-app ngakwesobunxele kakhulu
quick-inbox-tabs = Amathebhu ebhokisi lokungenayo
quick-inbox-tabs-detail = Amathebhu omhlinzeki wemeyili we-akhawunti ngayinye
quick-choose-tabs = Khetha amathebhu
quick-choose-tabs-detail = Nge-akhawunti ngayinye, kuzilungiselelo
quick-sending = Ukuthumela
quick-undo-send = Hlehlisa ukuthumela
quick-undo-send-off = Kuvaliwe
quick-undo-send-seconds = { $seconds } s
quick-signatures = Amasiginesha
quick-signatures-none = Awekho okwamanje
quick-signatures-one = { $name }, isetshenziswa ngokuzenzakalelayo
quick-signatures-many = { $count ->
    [one] Isiginesha engu-{ $count }; { $name } ngokuzenzakalelayo
   *[other] Amasiginesha angu-{ $count }; { $name } ngokuzenzakalelayo
}
quick-signatures-no-default = { $count ->
    [one] { $count }, ayikho ezenzakalelayo
   *[other] { $count }, ayikho ezenzakalelayo
}
quick-signature-untitled = Akunasihloko
quick-threading = Ukuqoqa imeyili ibe izingxoxo
quick-conversation-view = Ukubuka kwengxoxo
quick-conversation-view-detail = Qoqa ndawonye izimpendulo zemeyili efanayo
quick-help = Usizo
quick-tour = Thatha uhambo
quick-whats-new = Okusha
quick-about = Mayelana ne-Katna

## Settings: opening at login

settings-open-at-login-failed = Ayikwazanga ukushintsha ukuvula lapho ungena: { $error }

## Settings > Appearance > Scaling

scale-letter = A
scale-percent = { $percent }%
scale-reset = Buyela ku-{ $percent }%

## Settings > Experimental > Look & Feel

look-intro = Izici ezisazanywa. Zingashintsha noma zisuswe.
look-heading = Ukubukeka Nokuzwakala
look-window-frame = Uhlaka lwewindi
look-window-frame-detail = Ubani odweba ibha yesihloko, izinkinobho zewindi, amakhona nesithunzi.
look-frame-native-kde = Okwendabuko: uhlaka lwe-KDE, kutimu yakho ye-Plasma
look-frame-native = Okwendabuko: uhlaka lwedeskithophu
look-frame-katna = Katna: ibha ephezulu iba ibha yesihloko
look-frame-katna-note-named = I-Katna idweba amakhona ayindilinga nesithunzi sayo. Uhlaka alusalandeli itimu ye-{ $desktop }; imithetho yewindi isasebenza.
look-frame-katna-note = I-Katna idweba amakhona ayindilinga nesithunzi sayo. Uhlaka alusalandeli itimu yedeskithophu; imithetho yewindi isasebenza.
look-frame-client-side = Ideskithophu yakho ishiyela i-app ngayinye uhlaka lwayo, ngakho i-Katna isivele idweba olwayo.
look-blurred-background = Ingemuva elifiphele
look-blurred-background-detail = Ideskithophu iyabonakala ngebha ephezulu namafolda, ifiphele, futhi amamenyu nama-popover afana nengilazi efiphele.
look-blur = Fiphaza okungemuva kwewindi
look-blur-detail = Imeyili ihlala emakhadini aqinile, ngakho umbhalo ugcina ukugqama kwawo
look-blur-off-kde = Umphumela wokufiphaza we-KDE uvaliwe. Vula i-Blur ku-System Settings, Window Management, Desktop Effects, bese uvula i-Katna Mail futhi.
look-blur-none-gnome = I-GNOME ayikufiphazi okungemuva kwamawindi.
look-blur-none-x11 = Isiphathi sakho samawindi asikufiphazi okungemuva kwamawindi.
look-blur-none-wayland = I-compositor yakho ayikufiphazi okungemuva kwamawindi.

## Settings > User feedback (crash reports)

feedback-intro-sending = Imibiko emisha yokuphahlazeka iyathunyelwa ukuze isize ukulungisa inkinga. Akukho okunye okuphuma kule khompyutha.
feedback-intro-local = I-Katna ayithumeli lutho ndawo. Imibiko yokuphahlazeka ihlala kule khompyutha, ukuze uyibuke noma uyinamathisele embikweni wesiphazamisi.
feedback-crash-reports = Imibiko yokuphahlazeka
feedback-crash-reports-detail = Ibhalwa uma i-Katna Mail noma isevisi yayo yangemuva iphahlazeka.
feedback-save = Londoloza imibiko yokuphahlazeka kule khompyutha
feedback-save-detail = Ifolda yakho yasekhaya, amagama omsebenzisi nawekhompyutha kanye namakheli e-imeyili akufakwa
feedback-saved = Imibiko yokuphahlazeka elondoloziwe
feedback-saved-detail = { $count ->
    [one] Kugcinwa umbiko omusha kakhulu ongu-{ $count }.
   *[other] Kugcinwa imibiko emisha kakhulu engu-{ $count }.
}
feedback-help-improve = Siza ukuthuthukisa i-Katna
feedback-help-improve-detail = Kuvaliwe ngaphandle uma ukuvula, futhi ungakuvala lapha noma nini.
feedback-send = Thumela imibiko yokuphahlazeka
feedback-send-detail = Umbiko olondoloziwe, njengoba nje ungawubuka lapha, uya kusilandeleli sokuphahlazeka se-Katna (Sentry, e-EU). Alikho ikheli le-IP, imilayezo noma amakheli e-imeyili
feedback-none-saved = Ayikho imibiko yokuphahlazeka elondoloziwe.
feedback-delete-all = Susa konke
feedback-app-daemon = Isevisi yangemuva
feedback-report-sent = { $date } · Kuthunyelwe
feedback-view = Buka
feedback-view-tooltip = Vula umbiko
feedback-copy-tooltip = Kopisha ukuze uwunamathisele embikweni wesiphazamisi
feedback-copied = Umbiko wokuphahlazeka ukopishiwe.
feedback-deleted-all = Imibiko yokuphahlazeka isusiwe.
feedback-read-failed = Ayikwazanga ukufunda umbiko wokuphahlazeka: { $error }
feedback-delete-failed = Ayikwazanga ukususa umbiko wokuphahlazeka: { $error }
feedback-delete-all-failed = Ayikwazanga ukususa imibiko yokuphahlazeka: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _Ifayela
desktop-menu-new-message = Umlayezo _Omusha
desktop-menu-quit = _Phuma
desktop-menu-edit = _Hlela
desktop-menu-undo = H_lehlisa
desktop-menu-select-all = Khetha _Konke
desktop-menu-select-none = Ungakhethi _Lutho
desktop-menu-find = _Thola…
desktop-menu-view = _Buka
desktop-menu-folder-list = Bonisa Uhlu _Lwamafolda
desktop-menu-refresh = _Vuselela
desktop-menu-go = _Hamba
desktop-menu-inbox = _Ibhokisi Lokungenayo
desktop-menu-starred = Oku_nenkanyezi
desktop-menu-sent = Okuthun_yelwe
desktop-menu-drafts = Oku_salungiswa
desktop-menu-all-mail = _Wonke Amameyili
desktop-menu-next = Ingxoxo _Elandelayo
desktop-menu-previous = Ingxoxo E_dlule
desktop-menu-message = _Umlayezo
desktop-menu-open = _Vula
desktop-menu-reply = _Phendula
desktop-menu-reply-all = Phendula B_onke
desktop-menu-forward = _Dlulisela
desktop-menu-archive = Faka Kungobo _Yomlando
desktop-menu-delete = _Susa
desktop-menu-spam = Bika _Ugaxekile
desktop-menu-move-to = _Hambisa Ku…
desktop-menu-mark-read = Maka Njengoku_fundiwe
desktop-menu-mark-unread = _Maka Njengokungafundiwe
desktop-menu-star = Faka I_nkanyezi
desktop-menu-important = Maka Njengoku_balulekile
desktop-menu-not-important = Maka Njengokungabalu_lekile
desktop-menu-settings = _Izilungiselelo
desktop-menu-quick-settings = Izilungiselelo _Ezisheshayo
desktop-menu-configure = _Lungiselela i-Katna Mail…
desktop-menu-help = _Usizo
desktop-menu-shortcuts = Izinqamuleli Ze_khibhodi
desktop-menu-whats-new = _Okusha
desktop-menu-about = _Mayelana ne-Katna

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Ukuzulazula
shortcut-group-actions = Izenzo
shortcut-group-go-to = Iya ku-
shortcut-group-app = Uhlelo lokusebenza

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Ingxoxo elandelayo
shortcut-previous = Ingxoxo edlule
shortcut-down = Yehla ohlwini
shortcut-up = Khuphuka ohlwini
shortcut-first = Okokuqala ohlwini
shortcut-last = Okokugcina ohlwini
shortcut-page-down = Ikhasi elilodwa phansi ohlwini
shortcut-page-up = Ikhasi elilodwa phezulu ohlwini
shortcut-open = Vula ingxoxo
shortcut-back = Buyela ohlwini
shortcut-scroll-down = Skrolela phansi
shortcut-scroll-up = Skrolela phezulu
shortcut-scroll-page-down = Skrolela ikhasi phansi
shortcut-scroll-page-up = Skrolela ikhasi phezulu
shortcut-compose = Bhala
shortcut-reply = Phendula
shortcut-reply-all = Phendula bonke
shortcut-forward = Dlulisela
shortcut-archive = Faka kungobo yomlando
shortcut-delete = Susa
shortcut-spam = Bika ugaxekile
shortcut-move-to = Hambisa ku-
shortcut-mark-read = Maka njengokufundiwe
shortcut-mark-unread = Maka njengokungafundiwe
shortcut-star = Faka noma susa inkanyezi
shortcut-important = Maka njengokubalulekile
shortcut-not-important = Maka njengokungabalulekile
shortcut-check = Thikha ingxoxo
shortcut-select-all = Thikha zonke izingxoxo
shortcut-select-none = Susa ukuthikha kuzo zonke izingxoxo
shortcut-undo = Hlehlisa isenzo sokugcina
shortcut-go-inbox = Ibhokisi lokungenayo
shortcut-go-starred = Okunenkanyezi
shortcut-go-sent = Okuthunyelwe
shortcut-go-drafts = Okusalungiswa
shortcut-go-all = Wonke amameyili
shortcut-search = Sesha imeyili
shortcut-navigation = Bonisa noma goqa imenyu
shortcut-quick-settings = Izilungiselelo ezisheshayo
shortcut-settings = Zonke izilungiselelo
shortcut-shortcuts = Izinqamuleli zekhibhodi
shortcut-reload = Hlola imeyili entsha
shortcut-quit = Phuma

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } bese { $second }

## Settings > Accounts

accounts-folder-pane = Iphaneli yamafolda
accounts-folder-pane-detail = Amafolda ama-akhawunti aboniswa yiphaneli engakwesobunxele.
accounts-shown-one = I-akhawunti eyodwa ngesikhathi; shintsha ekhadini le-akhawunti
accounts-shown-all = Wonke ama-akhawunti, elinye emva kwelinye
accounts-row = Ama-akhawunti
accounts-row-detail = Ukususa i-akhawunti kususa ikhophi ye-Katna yemeyili yayo kule khompyutha. Imeyili ihlala kuseva.
accounts-none = Awekho ama-akhawunti okwamanje.
accounts-kind-imported = Ingenisiwe
accounts-picture-reset = Sebenzisa isithombe sedeskithophu
accounts-picture-change = Shintsha isithombe
accounts-remove = Susa
accounts-delete-all-row = Susa yonke idatha
accounts-delete-all-row-detail = Qala kabusha, njengokufakwa okusha.
accounts-delete-all-about = Isusa kule khompyutha yonke i-akhawunti, yonke imeyili egciniwe, oxhumana nabo namakhalenda, inkomba yosesho, izilungiselelo zakho namaphasiwedi alondoloziwe. Akukho okushintshayo kumaseva akho emeyili.
accounts-delete-all-open = Susa yonke idatha ye-Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = Ikheli { $address } lisusiwe ku-Katna.
accounts-removed = Ikheli { $address } lisusiwe ku-Katna. Imeyili yalo isesekhona kuseva.
accounts-all-deleted = Yonke idatha ye-Katna isusiwe kule khompyutha.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Susa i-akhawunti { $address }?
accounts-remove-confirm = Susa i-akhawunti
accounts-removing = Iyasusa…
accounts-remove-local-mail = { $folders ->
    [0] Yonke imeyili engeniswe kule akhawunti
    [one] Yonke imeyili engeniswe kule akhawunti, efolda yayo
   *[other] Yonke imeyili engeniswe kule akhawunti, emafolda ayo angu-{ $folders }
}
accounts-remove-local-settings = Izilungiselelo zayo ze-Katna
accounts-remove-mail = { $folders ->
    [0] Yonke imeyili yale akhawunti egcinwe yi-Katna
    [one] Yonke imeyili yale akhawunti egcinwe yi-Katna, efolda yayo
   *[other] Yonke imeyili yale akhawunti egcinwe yi-Katna, emafolda ayo angu-{ $folders }
}
accounts-remove-outbox = Imilayezo yayo elinde ebhokisini eliphumayo
accounts-remove-settings = Iphasiwedi yayo elondoloziwe nezilungiselelo zayo ze-Katna
accounts-delete-all-title = Susa yonke idatha ye-Katna?
accounts-delete-all-confirm = Susa konke
accounts-deleting = Iyasusa…
accounts-delete-all-accounts = Yonke i-akhawunti, nayo yonke imeyili nokunamathiselwe okugcinwe yi-Katna
accounts-delete-all-contacts = Oxhumana nabo, amakhalenda nenkomba yosesho
accounts-delete-all-settings = Zonke izilungiselelo, amasiginesha nezinqamuleli zekhibhodi
accounts-delete-all-passwords = Yonke iphasiwedi elondoloziwe
accounts-deleted-heading = Kuzosuswa kule khompyutha:
accounts-cannot-undo = Lokhu akukwazi ukuhlehliswa.
accounts-server-delete-all = Akukho okushintshayo kumaseva akho emeyili: imeyili yakho ihlala lapho, futhi ukwengeza i-akhawunti futhi kuyayilanda kabusha. Imeyili engeniswe kusuka kumafayela ise-Katna kuphela; amafayela awathintwa.
accounts-server-local = Le meyili ingeniswe kusuka kumafayela, ngakho i-Katna inekhophi okuwukuphela kwayo. Amafayela eyavela kuwo awathintwa; wangenise futhi ukuze uyibuyise.
accounts-server-remove = Akukho okushintshayo kuseva yemeyili: imeyili yakho ihlala lapho, futhi ukwengeza i-akhawunti futhi kuyayilanda kabusha.
accounts-confirm-word = susa
accounts-confirm-placeholder = Thayipha “{ accounts-confirm-word }”
accounts-confirm-prompt = Ukuze uqinisekise, thayipha “{ accounts-confirm-word }”:
accounts-cancel = Khansela
