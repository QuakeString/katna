# Katna Mail, Swahili (Kiswahili).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Lugha: { $language }
language-tooltip-system = Lugha: { $language }, kulingana na mfumo
language-search = Tafuta lugha
language-system-default = Chaguomsingi la mfumo
language-system-now = Sasa ni { $language }
language-no-match = Hakuna lugha inayolingana na “{ $query }”
language-machine = Imetafsiriwa na mashine. Saidia kuiboresha
language-setting = Lugha
language-setting-detail = Lugha ya menyu, vitufe na ujumbe, pamoja na muundo wa tarehe na nambari. Chaguomsingi la mfumo hufuata kompyuta ya mezani.

## Dates and sizes

ago-just-now = sasa hivi
ago-minutes = { $count ->
    [one] dakika { $count } iliyopita
   *[other] dakika { $count } zilizopita
}
ago-hours = { $count ->
    [one] saa { $count } iliyopita
   *[other] saa { $count } zilizopita
}
ago-days = { $count ->
    [one] siku { $count } iliyopita
   *[other] siku { $count } zilizopita
}
size-bytes = { $count ->
    [one] baiti { $count }
   *[other] baiti { $count }
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = Ficha folda
folders-show = Onyesha folda
compose = Tunga
search = Tafuta
search-mail = Tafuta barua
search-settings = Tafuta mipangilio
search-clear = Futa utafutaji
search-options-show = Onyesha chaguo za utafutaji
settings = Mipangilio
account-add = Ongeza akaunti

## App rail (and the bottom bar on a phone)

rail-mail = Barua
rail-calendar = Kalenda
rail-contacts = Anwani
rail-tasks = Majukumu
rail-notes = Madokezo
rail-feeds = Mipasho

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Inakuja hivi karibuni
app-calendar-promise = Kalenda zako za CalDAV, mialiko ya mikutano kutoka kwenye barua zako na vikumbusho, kando ya kikasha chako.
app-tasks-promise = Orodha za mambo ya kufanya zinazosawazishwa na CalDAV, na majukumu yanayotokana na barua.
app-notes-promise = Madokezo ya haraka, na madokezo kuhusu barua au mazungumzo kwa ajili ya baadaye.
app-feeds-promise = Soma mipasho ya RSS na Atom kando ya barua zako.

## Contacts page

app-contacts-loading = Inakusanya watu kutoka kwenye barua zako…
app-contacts-empty = Watu unaoandikiana nao wataonekana hapa.
app-contacts-count = { $count ->
    [one] Mtu { $count } kutoka kwenye barua zako, unaoandikiana nao zaidi kwanza
   *[other] Watu { $count } kutoka kwenye barua zako, unaoandikiana nao zaidi kwanza
}
app-contacts-top = { $count ->
    [one] Mtu { $count } wa juu kutoka kwenye barua zako, unaoandikiana nao zaidi kwanza
   *[other] Watu { $count } wa juu kutoka kwenye barua zako, unaoandikiana nao zaidi kwanza
}
app-contacts-messages = { $count ->
    [one] ujumbe { $count }
   *[other] jumbe { $count }
}
app-contacts-last = mara ya mwisho { $date }

## Navigation (the folders pane)

nav-labels = Lebo
nav-folders = Folda
nav-label-new = Unda lebo mpya
nav-folder-new = Unda folda mpya
nav-account-unnamed = Akaunti { $number }
nav-tab-new = { $count ->
    [one] { $count } mpya
   *[other] { $count } mpya
}

## Special folders (the user's own folders keep their names)

folder-inbox = Kikasha
folder-starred = Zenye nyota
folder-drafts = Rasimu
folder-sent = Zilizotumwa
folder-archive = Kumbukumbu
folder-spam = Taka
folder-trash = Tupio
folder-all-mail = Barua zote
folder-scheduled = Zilizoratibiwa

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Lebo mpya
label-folder-new-title = Folda mpya
label-prompt = Tafadhali weka jina jipya la lebo:
label-folder-prompt = Tafadhali weka jina jipya la folda:
label-name-hint = Jina la lebo
label-folder-name-hint = Jina la folda
label-nest = Weka lebo chini ya:
label-folder-nest = Weka folda chini ya:
label-cancel = Ghairi
label-create = Unda
label-creating = Inaunda…
label-created = Lebo “{ $name }” imeundwa.
label-folder-created = Folda “{ $name }” imeundwa.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Msingi
tab-promotions = Matangazo
tab-social = Mitandao ya kijamii
tab-updates = Taarifa
tab-forums = Mijadala
tab-focused = Zilizolengwa
tab-other = Nyingine
tab-inbox = Kikasha
tab-newsletters = Majarida
tab-notifications = Arifa
tab-new = { $count } mpya
tab-provider-other = zimepangwa na Katna

## Mail list: toolbar

list-select = Chagua
list-refresh = Onyesha upya
list-more = Zaidi
list-mark-read = Tia alama kuwa imesomwa
list-mark-unread = Tia alama kuwa haijasomwa
list-move-to = Hamishia
list-archive = Weka kwenye kumbukumbu
list-spam = Ripoti taka
list-delete = Futa
list-newer = Mpya zaidi
list-older = Za zamani zaidi
list-range = { $first }–{ $last } kati ya { $total }
list-range-about = { $first }–{ $last } kati ya takriban { $total }
list-results = Matokeo ya “{ $query }”
list-results-corrected = Inaonyesha matokeo ya “{ $query }”
list-search-instead = Badala yake tafuta “{ $query }”
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Zote
list-pick-none = Hakuna
list-pick-read = Zilizosomwa
list-pick-unread = Ambazo hazijasomwa
list-pick-starred = Zenye nyota
list-pick-unstarred = Zisizo na nyota

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yote { $count } yamechaguliwa.
       *[other] Mazungumzo yote { $count } yamechaguliwa.
    }
   *[message] { $count ->
        [one] Ujumbe wote { $count } umechaguliwa.
       *[other] Jumbe zote { $count } zimechaguliwa.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yote { $count } katika { $folder } yamechaguliwa.
       *[other] Mazungumzo yote { $count } katika { $folder } yamechaguliwa.
    }
   *[message] { $count ->
        [one] Ujumbe wote { $count } katika { $folder } umechaguliwa.
       *[other] Jumbe zote { $count } katika { $folder } zimechaguliwa.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yote { $count } kwenye skrini yamechaguliwa.
       *[other] Mazungumzo yote { $count } kwenye skrini yamechaguliwa.
    }
   *[message] { $count ->
        [one] Ujumbe wote { $count } kwenye skrini umechaguliwa.
       *[other] Jumbe zote { $count } kwenye skrini zimechaguliwa.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Chagua mazungumzo yote { $count }
       *[other] Chagua mazungumzo yote { $count }
    }
   *[message] { $count ->
        [one] Chagua ujumbe wote { $count }
       *[other] Chagua jumbe zote { $count }
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Chagua mazungumzo yote { $count } katika { $folder }
       *[other] Chagua mazungumzo yote { $count } katika { $folder }
    }
   *[message] { $count ->
        [one] Chagua ujumbe wote { $count } katika { $folder }
       *[other] Chagua jumbe zote { $count } katika { $folder }
    }
}
list-clear-selection = Futa uteuzi

## Mail list: empty states

list-empty-search = Hakuna ujumbe unaolingana na utafutaji wako.
list-empty-tab = Hakuna barua katika { $tab }.
list-empty-tab-unknown = Hakuna barua katika kichupo hiki.
list-empty-folder = Hakuna ujumbe katika { $folder }.
list-empty-folder-unknown = Hakuna ujumbe katika folda hii.
list-first-sync = Inaleta barua zako…
list-first-sync-detail = Zitaonekana hapa zinapowasili.

## Mail list: lines

row-removed = Ujumbe huu uliondolewa.
row-starred = Ina nyota
row-not-starred = Haina nyota
row-important = Muhimu. Bofya ili kutia alama kuwa si muhimu.
row-mark-important = Tia alama kuwa muhimu
row-pinned = Imebandikwa juu
row-pin = Bandika juu
row-unpin = Bandua

## Mail list: More menu and right-click menu

menu-reply = Jibu
menu-reply-all = Jibu wote
menu-forward = Sambaza
menu-archive = Weka kwenye kumbukumbu
menu-delete = Futa
menu-spam = Ripoti taka
menu-mark-read = Tia alama kuwa imesomwa
menu-mark-unread = Tia alama kuwa haijasomwa
menu-mark-all-read = Tia alama zote kuwa zimesomwa
menu-star = Weka nyota
menu-unstar = Ondoa nyota
menu-important = Tia alama kuwa muhimu
menu-not-important = Tia alama kuwa si muhimu
menu-pin = Bandika juu
menu-unpin = Bandua
menu-print-all = Chapisha zote
menu-new-window = Fungua katika dirisha jipya
menu-move-to = Hamishia
menu-move-to-heading = Hamishia:
menu-find-from = Tafuta barua pepe kutoka kwa { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yamewekwa kwenye kumbukumbu.
       *[other] Mazungumzo { $count } yamewekwa kwenye kumbukumbu.
    }
   *[message] { $count ->
        [one] Ujumbe umewekwa kwenye kumbukumbu.
       *[other] Jumbe { $count } zimewekwa kwenye kumbukumbu.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yamehamishiwa kwenye Tupio.
       *[other] Mazungumzo { $count } yamehamishiwa kwenye Tupio.
    }
   *[message] { $count ->
        [one] Ujumbe umehamishiwa kwenye Tupio.
       *[other] Jumbe { $count } zimehamishiwa kwenye Tupio.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yamehamishwa.
       *[other] Mazungumzo { $count } yamehamishwa.
    }
   *[message] { $count ->
        [one] Ujumbe umehamishwa.
       *[other] Jumbe { $count } zimehamishwa.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yamewekewa nyota.
       *[other] Mazungumzo { $count } yamewekewa nyota.
    }
   *[message] { $count ->
        [one] Ujumbe umewekewa nyota.
       *[other] Jumbe { $count } zimewekewa nyota.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Nyota imeondolewa kwenye mazungumzo.
       *[other] Nyota imeondolewa kwenye mazungumzo { $count }.
    }
   *[message] { $count ->
        [one] Nyota imeondolewa kwenye ujumbe.
       *[other] Nyota imeondolewa kwenye jumbe { $count }.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yametiwa alama kuwa muhimu.
       *[other] Mazungumzo { $count } yametiwa alama kuwa muhimu.
    }
   *[message] { $count ->
        [one] Ujumbe umetiwa alama kuwa muhimu.
       *[other] Jumbe { $count } zimetiwa alama kuwa muhimu.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yametiwa alama kuwa si muhimu.
       *[other] Mazungumzo { $count } yametiwa alama kuwa si muhimu.
    }
   *[message] { $count ->
        [one] Ujumbe umetiwa alama kuwa si muhimu.
       *[other] Jumbe { $count } zimetiwa alama kuwa si muhimu.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yamebandikwa juu.
       *[other] Mazungumzo { $count } yamebandikwa juu.
    }
   *[message] { $count ->
        [one] Ujumbe umebandikwa juu.
       *[other] Jumbe { $count } zimebandikwa juu.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yamebanduliwa.
       *[other] Mazungumzo { $count } yamebanduliwa.
    }
   *[message] { $count ->
        [one] Ujumbe umebanduliwa.
       *[other] Jumbe { $count } zimebanduliwa.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yameripotiwa kuwa taka.
       *[other] Mazungumzo { $count } yameripotiwa kuwa taka.
    }
   *[message] { $count ->
        [one] Ujumbe umeripotiwa kuwa taka.
       *[other] Jumbe { $count } zimeripotiwa kuwa taka.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Mazungumzo yamefutwa kabisa.
       *[other] Mazungumzo { $count } yamefutwa kabisa.
    }
   *[message] { $count ->
        [one] Ujumbe umefutwa kabisa.
       *[other] Jumbe { $count } zimefutwa kabisa.
    }
}
toast-undone = Kitendo kimetenduliwa.
toast-undo = Tendua
toast-no-spam-folder = Akaunti hii haina folda ya taka.

## Reading pane: toolbar

reader-close = Funga
reader-back = Rudi
reader-mark-unread = Tia alama kuwa haijasomwa
reader-move-to = Hamishia
reader-more = Zaidi
reader-print-all = Chapisha zote
reader-new-window = Katika dirisha jipya
reader-position = { $position } kati ya { $total }
reader-newer = Mpya zaidi
reader-older = Ya zamani zaidi

## Reading pane: the conversation

reader-removed = Mazungumzo haya yaliondolewa.
reader-no-subject = (hakuna mada)
reader-collapse-all = Kunja zote
reader-expand-all = Panua zote
reader-unknown-sender = (mtumaji asiyejulikana)
reader-date-ago = { $date } ({ $ago })
reader-me = mimi
reader-to = kwa { $names }
reader-starred = Ina nyota
reader-not-starred = Haina nyota
reader-too-long = Ujumbe ni mrefu mno kuonyeshwa wote.
reader-encrypted-images = Picha kutoka kwenye wavuti hazipakiwi kamwe katika barua iliyosimbwa.
reader-window-failed = Imeshindwa kufungua dirisha jipya.

## Reading pane: message details (opened from "to me")

reader-details-from = kutoka:
reader-details-to = kwa:
reader-details-cc = nakala:
reader-details-date = tarehe:
reader-details-subject = mada:

## Reading pane: downloading a message

reader-downloading = Inapakua ujumbe huu kutoka kwenye seva…
reader-download-failed = Imeshindwa kupakua ujumbe huu.
reader-try-again = Jaribu tena

## Reply row

reply-reply = Jibu
reply-reply-all = Jibu wote
reply-forward = Sambaza

## Encrypted and signed mail

security-decrypting = Inasimbua…
security-checking = Inakagua sahihi…
security-partly-encrypted = Ni sehemu tu ya ujumbe huu iliyosimbwa. Sehemu iliyobaki iliongezwa nje ya ulinzi na inaweza kutoka kwa mtu yeyote.
security-partly-signed = Ni sehemu tu ya ujumbe huu iliyotiwa sahihi. Sehemu iliyobaki iliongezwa nje ya ulinzi na inaweza kutoka kwa mtu yeyote.
security-encrypted = Ujumbe uliosimbwa
security-encrypted-smime = Ujumbe uliosimbwa (S/MIME)
security-no-key = Haiwezi kusimbua ujumbe huu: ulisimbwa kwa ufunguo usio nao.
security-cancelled = Usimbuaji ulighairiwa.
security-damaged = Haiwezi kusimbua ujumbe huu: data iliyosimbwa imeharibika au ilibadilishwa.
security-decrypt-unavailable = Haiwezi kusimbua ujumbe huu: sakinisha { $tool } ili usome barua zilizosimbwa.
security-decrypt-failed = Haiwezi kusimbua ujumbe huu: { $reason }
security-unknown-signer = mtia sahihi asiyejulikana
security-signed-verified = Imetiwa sahihi na { $signer } · imethibitishwa
security-signed-not-sender = Imetiwa sahihi na { $signer }, ambaye si mtumaji
security-signed-untrusted = Imetiwa sahihi na { $signer }, kwa ufunguo uliotia alama kuwa hauaminiki
security-signed-unverified = Imetiwa sahihi na { $signer } · ufunguo haujathibitishwa
security-bad-signature = Sahihi mbaya: ujumbe huu ulibadilishwa baada ya kutiwa sahihi, au sahihi ni ya kughushi.
security-signature-expired = Imetiwa sahihi na { $signer } · muda wa sahihi umeisha
security-key-expired = Imetiwa sahihi na { $signer } · muda wa ufunguo umeisha tangu hapo
security-key-revoked = Imetiwa sahihi na { $signer } kwa ufunguo uliobatilishwa
security-missing-key = Imetiwa sahihi kwa ufunguo usio nao, kwa hivyo haiwezi kukaguliwa
security-missing-key-id = Imetiwa sahihi kwa ufunguo usio nao ({ $key }), kwa hivyo haiwezi kukaguliwa
security-signature-unavailable = Imetiwa sahihi; sakinisha { $tool } ili kukagua sahihi
security-signature-error = Sahihi haikuweza kukaguliwa.

## Remote images and pictures

remote-hidden = Picha katika ujumbe huu zimefichwa.
remote-show = Onyesha picha
remote-always-show = Onyesha kila wakati kutoka kwa mtumaji huyu
remote-picture-use = Tumia
remote-picture-too-big = Chagua picha ya MB 8 au chini.
remote-picture-type = Chagua picha ya PNG, JPEG, GIF, WebP au SVG.
remote-picture-read-failed = Haiwezi kusoma picha: { $error }
remote-picture-keep-failed = Haiwezi kuhifadhi picha: { $error }
remote-picture-remove-failed = Haiwezi kuondoa picha: { $error }

## Attachments

attachment-count = { $count ->
    [one] Kiambatisho kimoja
   *[other] Viambatisho { $count }
}
attachment-save = Hifadhi
attachment-save-all = Hifadhi vyote
attachment-save-all-tooltip = Hifadhi kila kiambatisho kwenye folda
attachment-save-here = Hifadhi hapa
attachment-not-downloaded = Ujumbe huu haujapakuliwa.
attachment-not-found = Kiambatisho hiki hakikupatikana katika ujumbe.
attachment-read-failed = Imeshindwa kusoma { $name }
attachment-numbered = kiambatisho { $number }
attachment-saved-all = { $count ->
    [one] Faili { $count } imehifadhiwa kwenye { $place }
   *[other] Faili { $count } zimehifadhiwa kwenye { $place }
}
attachment-saved-some = { $total ->
    [one] Faili { $saved } kati ya { $total } imehifadhiwa kwenye { $place }. Imeshindwa kuhifadhi { $failed }
   *[other] Faili { $saved } kati ya { $total } zimehifadhiwa kwenye { $place }. Imeshindwa kuhifadhi { $failed }
}
attachment-saved-to = Imehifadhiwa kwenye { $path }
attachment-save-failed = Imeshindwa kuhifadhi { $name }: { $error }
attachment-open-failed = Imeshindwa kufungua { $name }: { $error }
attachment-risky = Faili hii inaweza kuendesha programu, kwa hivyo Katna haiifungui. Ihifadhi badala yake.
attachment-encrypted-open = Faili hii ilikuja ikiwa imesimbwa. Ihifadhi ili uifungue mahali pengine.

## Printing

print-failed = Imeshindwa kuchapisha: { $error }
print-no-font = hakuna fonti iliyopatikana
print-opened-as-pdf = Imefunguliwa kama PDF ili uchapishe kutoka hapo.
print-not-downloaded = (Bado haujapakuliwa.)
print-encrypted = (Umesimbwa. Ufungue katika Katna Mail ili uchapishe maandishi yake.)
print-to = Kwa: { $addresses }
print-cc = Nakala: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Fungua ujumbe huu ili usome viambatisho vyake.
text-copy = Nakili
text-select-all = Chagua zote

## Settings page: its tabs

settings-tab-general = Jumla
settings-tab-inbox = Kikasha
settings-tab-accounts = Akaunti
settings-tab-subscriptions = Usajili
settings-tab-appearance = Mwonekano
settings-tab-shortcuts = Njia za mkato
settings-tab-default-apps = Programu chaguomsingi
settings-tab-folders-rules = Folda na sheria
settings-tab-compose = Tunga
settings-tab-mcp-server = Seva ya MCP
settings-tab-feedback = Maoni ya mtumiaji
settings-tab-experimental = Majaribio

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Angalia majarida na orodha za barua unazopokea, na ujiondoe kwa mbofyo mmoja.
settings-tab-folders-rules-coming = Unda, badilisha jina, hamisha na ficha folda na lebo, na uchague zipi zisawazishwe. Sheria hupanga, huweka lebo, husambaza au hufuta barua mpya zenyewe, kulingana na mtumaji, mada au maneno.
settings-tab-mcp-server-coming = Ruhusu wasaidizi wa AI kwenye kompyuta hii watafute, wasome na waandike rasimu za barua zako, kwa idhini yako.

## Settings > General

settings-general-conversations = Mwonekano wa mazungumzo
settings-general-conversations-group = Panga pamoja majibu ya barua ileile
settings-general-conversations-group-detail = Mstari mmoja kwa kila mazungumzo katika orodha
settings-general-reading = Kusoma
settings-general-newest-first = Ujumbe mpya zaidi kwanza
settings-general-newest-first-detail = Mazungumzo huanza na jibu lake la hivi karibuni
settings-general-full-headers = Onyesha vichwa kamili
settings-general-full-headers-detail = Kutoka, kwa, nakala, tarehe na mada hufunguka kwenye kila ujumbe
settings-general-full-names = Majina kamili ya wapokeaji
settings-general-full-names-detail = “kwa mimi, Ada Lovelace” badala ya “kwa mimi, Ada”
settings-general-mark-read = Tia alama kuwa imesomwa
settings-general-mark-read-now = Mara tu unapofunguliwa
settings-general-mark-read-1s = Baada ya kufunguliwa kwa sekunde 1
settings-general-mark-read-3s = Baada ya kufunguliwa kwa sekunde 3
settings-general-mark-read-never = Pale tu ninapotia alama kuwa umesomwa
settings-general-reply-button = Kitufe cha kujibu
settings-general-reply-all = Jibu kila mtu
settings-general-reply-all-detail = Kitufe cha kujibu kando ya kila ujumbe huwajibu wote, si mtumaji pekee
settings-general-remote-images = Picha kutoka kwenye wavuti
settings-general-remote-images-detail = Kupakia picha za ujumbe humjulisha mtumaji wake kwamba uliufungua, lini, na takriban ukiwa wapi. Kikiwa kimezimwa, kila ujumbe huuliza kwanza, na unaweza kuonyesha picha za mtumaji wakati wowote.
settings-general-remote-images-always = Onyesha picha kila wakati
settings-general-remote-images-always-detail = Katika kila ujumbe, si tu kutoka kwa watumaji unaowaamini
settings-general-sending = Kutuma
settings-general-sending-detail = Muda ambao ujumbe uliotumwa husubiri, ili uweze kurudishwa.
settings-general-offline = Barua nje ya mtandao
settings-general-offline-detail = Barua za hivi karibuni hupakuliwa kikamilifu, ili zisomwe bila muunganisho. Barua za zamani zaidi hupakuliwa unapozifungua.
settings-general-offline-days = { $count ->
    [one] Siku { $count }
   *[other] Siku { $count }
}
settings-general-offline-years = { $count ->
    [one] Mwaka { $count }
   *[other] Miaka { $count }
}
settings-general-offline-all = Barua zote
settings-general-offline-note = Kuchagua siku chache zaidi huhifadhi barua ambazo tayari zimepakuliwa. Hakuna kinachobadilika kwenye seva.
settings-general-notifications = Arifa
settings-general-notifications-detail = Kwa barua mpya katika Kikasha, hata wakati Katna Mail imefungwa.
settings-general-new-mail = Niarifu kuhusu barua mpya
settings-general-new-mail-detail = Pamoja na Jibu wote, Tia alama kuwa imesomwa na Weka kwenye kumbukumbu
settings-general-new-mail-sound = Cheza sauti
settings-general-new-mail-sound-detail = Sauti ya barua mpya ya kompyuta ya mezani
settings-general-desktop = Kompyuta ya mezani
settings-general-open-at-login = Fungua Katna Mail wakati wa kuingia
settings-general-open-at-login-detail = Barua husawazishwa wakati wa kuingia kwa vyovyote vile, huduma ikiwa inaendeshwa
settings-general-tray = Onyesha Katna kwenye trei ya mfumo
settings-general-tray-detail = Pamoja na idadi ya ambazo hazijasomwa na menyu
settings-general-unread-badge = Idadi ya ambazo hazijasomwa kwenye aikoni ya upau wa kazi
settings-general-unread-badge-detail = Idadi ya jumbe za Kikasha ambazo hazijasomwa

## Settings > Inbox

settings-inbox-tabs = Vichupo vya kikasha
settings-inbox-tabs-detail = Panga kikasha katika vichupo, kama tovuti ya mtoa huduma wako wa barua inavyofanya.
settings-inbox-tabs-show = Onyesha vichupo vya kikasha
settings-inbox-tabs-show-detail = Kikiwa kimezimwa, huonyesha orodha moja kwa kila akaunti
settings-inbox-no-accounts = Ongeza akaunti ili uchague vichupo vyake.
settings-inbox-tabs-automatic = Kiotomatiki: { $tabs } ({ $provider })
settings-inbox-tabs-off = Hakuna vichupo
settings-inbox-tabs-gmail = Msingi, Matangazo, Mitandao ya kijamii, Taarifa, Mijadala
settings-inbox-tabs-focused = Zilizolengwa na Nyingine
settings-inbox-tabs-zoho = Kikasha, Majarida na Arifa
settings-inbox-tabs-shown = Vichupo vinavyoonyeshwa. Barua za kichupo unachozima hubaki katika { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Kidirisha cha kusoma
settings-appearance-reading-pane-detail = Mahali mazungumzo yaliyofunguliwa yanapoonyeshwa.
settings-appearance-pane-right = Kulia mwa orodha
settings-appearance-pane-none = Bila kugawanya
settings-appearance-density = Msongamano
settings-appearance-density-default = Chaguomsingi
settings-appearance-density-compact = Iliyobanwa
settings-appearance-scaling = Kiwango cha ukubwa
settings-appearance-scaling-detail = Hufanya kila kitu katika Katna Mail kiwe kikubwa au kidogo zaidi, juu ya kipimo cha kompyuta ya mezani yenyewe: maandishi, aikoni, nafasi na vigawanyaji. Barua unazotuma huhifadhi ukubwa wake wa fonti. Ukubwa mdogo sana unaweza kufanya aikoni kuwa vigumu kubofya.
settings-appearance-theme = Mandhari
settings-appearance-theme-system = Sawa na kompyuta ya mezani
settings-appearance-theme-light = Angavu
settings-appearance-theme-dark = Meusi
settings-appearance-desktop-colors = Rangi za kompyuta ya mezani
settings-appearance-desktop-colors-use = Tumia rangi za kompyuta ya mezani
settings-appearance-desktop-colors-use-detail = Mpangilio wa rangi na rangi ya msisitizo ya kompyuta ya mezani
settings-appearance-app-names = Majina ya programu
settings-appearance-app-names-show = Onyesha majina ya programu
settings-appearance-app-names-show-detail = Majina chini ya aikoni za programu upande wa kushoto kabisa
settings-appearance-sender-pictures = Picha za watumaji
settings-appearance-sender-pictures-show = Onyesha nembo za kampuni
settings-appearance-sender-pictures-show-detail = Hutafutwa kwa kikoa cha mtumaji, kamwe si kwa ujumbe, na huhifadhiwa kwa wiki moja
settings-appearance-important = Alama za Muhimu
settings-appearance-important-show = Onyesha alama za Muhimu
settings-appearance-important-show-detail = Kando ya kila ujumbe katika orodha
settings-appearance-message-width = Upana wa ujumbe
settings-appearance-message-width-limit = Weka kikomo cha upana wa jumbe
settings-appearance-message-width-limit-detail = Mistari mirefu ni rahisi kusoma katika dirisha pana
settings-appearance-mail-colors = Rangi za barua
settings-appearance-mail-colors-detail = Barua nyingi zimeundwa kwa ukurasa mweupe. Katika mandhari meusi rangi zake hubadilishwa kuwa nyeusi zinazosomeka vizuri; kikiwa kimezimwa, barua huhifadhi rangi za mtumaji wake kwenye ukurasa angavu.
settings-appearance-dark-mail = Rangi nyeusi kwa barua pia
settings-appearance-dark-mail-detail = Wakati tu mandhari ni meusi
settings-appearance-attachment-previews = Onyesho la kukagua viambatisho
settings-appearance-attachment-previews-show = Onyesha onyesho la kukagua viambatisho
settings-appearance-attachment-previews-show-detail = Picha ndogo ya maudhui ya kila faili kwenye kadi yake

## Settings > Default apps

settings-default-apps-intro = Mahali viambatisho vinapofunguka unapovibofya. Kitazamaji kinaweza pia kufungua faili katika programu nyingine wakati wowote. Programu chaguomsingi za kompyuta ya mezani huwekwa katika mipangilio yake yenyewe.
settings-default-apps-pdf = Faili za PDF
settings-default-apps-pdf-detail = Kurasa, pamoja na kukuza.
settings-default-apps-pictures = Picha
settings-default-apps-pictures-detail = Picha za kamera (zilizonyooshwa), PNG, GIF, WebP, BMP, TIFF na SVG.
settings-default-apps-text = Faili za maandishi
settings-default-apps-text-detail = Maandishi matupu, logi, msimbo na maandishi mengine.
settings-default-apps-sheets = Lahajedwali
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) na CSV.
settings-default-apps-documents = Hati
settings-default-apps-documents-detail = Word (docx) na maandishi ya OpenDocument (odt).
settings-default-apps-katna = Kitazamaji cha Katna Mail
settings-default-apps-system = Programu chaguomsingi ya kompyuta ya mezani
settings-default-apps-ask = Uliza programu kila mara
settings-default-apps-after-saving = Baada ya kuhifadhi
settings-default-apps-show-folder = Onyesha faili zilizohifadhiwa katika folda yake
settings-default-apps-show-folder-detail = Hufungua kidhibiti faili huku viambatisho vilivyohifadhiwa vikiwa vimechaguliwa

## Settings > Compose

settings-compose-send-from = Tuma jumbe mpya kutoka
settings-compose-send-from-detail = Majibu na barua zinazosambazwa hutumwa kila wakati kutoka kwa akaunti uliyomo.
settings-compose-send-from-current = Akaunti uliyomo
settings-compose-send-on-replies = Kutuma kwenye majibu
settings-compose-send-on-replies-detail = Kile ambacho Tuma hufanya kwenye jibu au barua inayosambazwa. Menyu iliyo kando ya Tuma hutoa chaguo jingine.
settings-compose-send-plain = Tuma
settings-compose-send-archive = Tuma na uweke kwenye kumbukumbu
settings-compose-signatures = Sahihi
settings-compose-signatures-detail = Huongezwa chini ya ujumbe wako, baada ya mstari wa “--”. Chagua nyingine katika dirisha la kutunga.
settings-compose-untitled = Bila jina
settings-compose-signature-name = Jina, kama vile Kazini
settings-compose-signature-first = Sahihi yangu
settings-compose-signature-numbered = Sahihi { $number }
settings-compose-signature-delete = Futa
settings-compose-signature-deleted = Sahihi imefutwa
settings-compose-signature-new = Unda mpya
settings-compose-no-signatures = Bado hakuna sahihi.
settings-compose-no-signature = Hakuna sahihi
settings-compose-for-new-mail = Kwa barua mpya
settings-compose-for-replies = Kwa majibu na barua zinazosambazwa
settings-compose-for-replies-detail = Katika mazungumzo ambapo uliweka sahihi kwenye ujumbe, jibu huanza na sahihi hiyo badala yake.
settings-compose-format = Muundo
settings-compose-plain-text = Andika kwa maandishi matupu
settings-compose-plain-text-detail = Barua mpya huanza bila uumbizaji; dirisha la kutunga linaweza kubadilisha
settings-compose-spelling = Tahajia
settings-compose-spell-check = Kagua tahajia ninapoandika
settings-compose-spell-check-detail = Maneno yenye makosa ya tahajia hupigiwa mstari, pamoja na mapendekezo kwa kubofya kulia
settings-compose-spell-desktop = Lugha ya kompyuta ya mezani ({ $language })
settings-compose-templates = Violezo
settings-compose-templates-detail = Hifadhi barua unazoandika mara kwa mara, na uanze barua mpya au jibu kutoka kwayo.

## Settings > Shortcuts

settings-shortcuts-set = Seti ya njia za mkato
settings-shortcuts-set-detail = Anza na vitufe vya programu ya barua unayoifahamu. Cmd ni Ctrl hapa. Mabadiliko yako mwenyewe hubaki juu ya seti, na Rejesha chaguomsingi hurudi kwenye vitufe vya seti.
settings-shortcuts-single = Njia za mkato za kitufe kimoja
settings-shortcuts-single-detail = Vitufe bila Ctrl au Alt, kama katika barua pepe ya wavuti: e huweka kwenye kumbukumbu, j na k husogeza, / hutafuta. Hufanya kazi katika orodha na mazungumzo yaliyofunguliwa, kamwe si wakati wa kuandika.
settings-shortcuts-single-use = Tumia njia za mkato za kitufe kimoja
settings-shortcuts-single-use-detail = Njia za mkato za Ctrl hufanya kazi kila wakati
settings-shortcuts-how = Bofya kitufe ili ukibadilishe, au + ili uongeze kimoja, kisha ubonyeze vitufe vipya. Esc hughairi.
settings-shortcuts-restore = Rejesha chaguomsingi
settings-shortcuts-no-key = Hakuna kitufe
settings-shortcuts-press = Bonyeza vitufe…
settings-shortcuts-then = { $keys } kisha…
settings-shortcuts-moved = { $keys } sasa hufanya “{ $action }” badala ya “{ $previous }”.
settings-shortcuts-single-off = Njia za mkato za kitufe kimoja zimezimwa, kwa hivyo kitufe hiki kitafanya kazi zitakapowashwa.
settings-shortcuts-restored = Kila njia ya mkato ina vitufe vya seti yake tena.

## Settings search: the line under a result

settings-general-language-summary = Lugha ya programu, tarehe na nambari
settings-general-reading-summary = Ujumbe mpya zaidi kwanza, vichwa kamili, majina kamili ya wapokeaji
settings-general-mark-read-summary = Wakati mazungumzo yaliyofunguliwa yanapotiwa alama kuwa yamesomwa: papo hapo, baada ya sekunde 1 au 3, au kwa mkono
settings-general-reply-button-summary = Kitufe cha kujibu kando ya kila ujumbe humjibu kila mtu
settings-general-remote-images-summary = Onyesha picha za kila ujumbe kila wakati
settings-general-sending-summary = Tendua kutuma: muda ambao ujumbe uliotumwa husubiri, ili uweze kurudishwa
settings-general-offline-summary = Siku ngapi za barua za hivi karibuni hupakuliwa kikamilifu, ili zisomwe bila muunganisho
settings-general-notifications-summary = Arifa za barua mpya na sauti yake
settings-general-desktop-summary = Fungua Katna Mail wakati wa kuingia, aikoni ya trei ya mfumo na idadi ya ambazo hazijasomwa kwenye aikoni ya upau wa kazi
settings-accounts-accounts-summary = Ongeza au ondoa akaunti, au ubadilishe picha yake
settings-appearance-density-summary = Mistari chaguomsingi au iliyobanwa katika orodha
settings-appearance-scaling-summary = Fanya kila kitu kiwe kikubwa au kidogo zaidi: maandishi, aikoni, nafasi na vigawanyaji
settings-appearance-theme-summary = Sawa na kompyuta ya mezani, angavu au meusi
settings-appearance-sender-pictures-summary = Nembo za kampuni, zinazotafutwa kwa kikoa cha mtumaji
settings-appearance-important-summary = Alama ya Muhimu kando ya kila ujumbe katika orodha
settings-appearance-mail-colors-summary = Rangi nyeusi kwa barua za HTML katika mandhari meusi, au rangi za mtumaji wake
settings-appearance-attachment-previews-summary = Picha ndogo ya maudhui ya kila kiambatisho
settings-shortcuts-set-summary = Anza na vitufe vya Gmail, Inbox by Gmail, Apple Mail, Outlook au Thunderbird
settings-shortcuts-single-summary = Vitufe bila Ctrl au Alt, kama katika barua pepe ya wavuti
settings-default-apps-pdf-summary = Mahali viambatisho vya PDF vinapofunguka
settings-default-apps-pictures-summary = Mahali picha za kamera na picha nyingine zinapofunguka
settings-default-apps-text-summary = Mahali maandishi matupu, logi na msimbo vinapofunguka
settings-default-apps-sheets-summary = Mahali faili za Excel, OpenDocument na CSV zinapofunguka
settings-default-apps-documents-summary = Mahali maandishi ya Word na OpenDocument yanapofunguka
settings-default-apps-after-saving-summary = Onyesha viambatisho vilivyohifadhiwa katika folda yake
settings-compose-send-from-summary = Akaunti ambayo barua mpya hutumwa kutoka kwayo: ile uliyomo, au ileile kila wakati
settings-compose-send-on-replies-summary = Tuma, au Tuma na uweke mazungumzo kwenye kumbukumbu, kwenye majibu na barua zinazosambazwa
settings-compose-signatures-summary = Huongezwa chini ya ujumbe wako, baada ya mstari wa “--”
settings-compose-for-new-mail-summary = Sahihi ambayo barua mpya huanza nayo
settings-compose-for-replies-summary = Sahihi ambayo majibu na barua zinazosambazwa huanza nayo
settings-compose-format-summary = Andika barua mpya kwa maandishi matupu
settings-compose-spelling-summary = Kagua tahajia wakati wa kuandika, na lugha ya kamusi
settings-compose-templates-summary = Inakuja hivi karibuni: hifadhi barua unazoandika mara kwa mara, na uanze barua mpya au jibu kutoka kwayo
settings-feedback-crash-reports-summary = Hifadhi ripoti za kuacha kufanya kazi kwenye kompyuta hii Katna Mail au huduma yake ya chinichini inapoacha kufanya kazi
settings-feedback-saved-summary = Tazama, nakili au futa ripoti za kuacha kufanya kazi zilizohifadhiwa kwenye kompyuta hii
settings-feedback-help-improve-summary = Tuma ripoti za kuacha kufanya kazi ili kusaidia kurekebisha tatizo; imezimwa isipokuwa ukiiwasha
settings-experimental-blur-summary = Kompyuta ya mezani huonekana kupitia upau wa juu, ikiwa na ukungu, na menyu huonekana kama kioo chenye ukungu
settings-search-shortcut = Njia ya mkato ya kibodi
settings-search-tab = Kichupo cha mipangilio
settings-search-none = Hakuna mipangilio inayolingana na “{ $query }”.
settings-search-results = Mipangilio inayolingana na “{ $query }”

## Quick settings (the panel that slides in from the right)

quick-title = Mipangilio ya haraka
quick-see-all = Angalia mipangilio yote
quick-reading-pane = Kidirisha cha kusoma
quick-pane-right = Kulia mwa orodha
quick-pane-none = Bila kugawanya
quick-density = Msongamano
quick-density-default = Chaguomsingi
quick-density-compact = Iliyobanwa
quick-theme = Mandhari
quick-theme-system = Sawa na kompyuta ya mezani
quick-theme-light = Angavu
quick-theme-dark = Meusi
quick-desktop-colors = Rangi za kompyuta ya mezani
quick-desktop-colors-detail = Mpangilio wa rangi na rangi ya msisitizo ya kompyuta ya mezani
quick-app-names = Majina ya programu
quick-app-names-detail = Majina chini ya aikoni za programu upande wa kushoto kabisa
quick-inbox-tabs = Vichupo vya kikasha
quick-inbox-tabs-detail = Vichupo vya mtoa huduma wa barua wa kila akaunti
quick-choose-tabs = Chagua vichupo
quick-choose-tabs-detail = Kwa kila akaunti, katika Mipangilio
quick-sending = Kutuma
quick-undo-send = Tendua kutuma
quick-undo-send-off = Imezimwa
quick-undo-send-seconds = { $seconds } s
quick-signatures = Sahihi
quick-signatures-none = Bado hakuna
quick-signatures-one = { $name }, hutumika kwa chaguomsingi
quick-signatures-many = { $count ->
    [one] Sahihi { $count }; { $name } kwa chaguomsingi
   *[other] Sahihi { $count }; { $name } kwa chaguomsingi
}
quick-signatures-no-default = { $count ->
    [one] { $count }, hakuna chaguomsingi
   *[other] { $count }, hakuna chaguomsingi
}
quick-signature-untitled = Bila jina
quick-threading = Kupanga barua pepe kwa mazungumzo
quick-conversation-view = Mwonekano wa mazungumzo
quick-conversation-view-detail = Panga pamoja majibu ya barua ileile
quick-help = Usaidizi
quick-tour = Anza ziara
quick-whats-new = Kilicho kipya
quick-about = Kuhusu Katna

## Settings: opening at login

settings-open-at-login-failed = Imeshindwa kubadilisha kufungua wakati wa kuingia: { $error }

## Settings > Appearance > Scaling

scale-letter = A
scale-percent = { $percent }%
scale-reset = Rudi kwenye { $percent }%

## Settings > Experimental > Look & Feel

look-intro = Vipengele ambavyo bado vinajaribiwa. Vinaweza kubadilika au kuondolewa.
look-heading = Mwonekano na Hisia
look-window-frame = Fremu ya dirisha
look-window-frame-detail = Nani huchora upau wa kichwa, vitufe vya dirisha, pembe na kivuli.
look-frame-native-kde = Asili: fremu ya KDE, katika mandhari yako ya Plasma
look-frame-native = Asili: fremu ya kompyuta ya mezani
look-frame-katna = Katna: upau wa juu unakuwa upau wa kichwa
look-frame-katna-note-named = Katna huchora pembe za mviringo na kivuli chake yenyewe. Fremu haifuati tena mandhari ya { $desktop }; sheria za dirisha bado zinatumika.
look-frame-katna-note = Katna huchora pembe za mviringo na kivuli chake yenyewe. Fremu haifuati tena mandhari ya kompyuta ya mezani; sheria za dirisha bado zinatumika.
look-frame-client-side = Kompyuta yako ya mezani huachia kila programu ichore fremu yake, kwa hivyo Katna tayari huchora yake yenyewe.
look-blurred-background = Mandharinyuma yenye ukungu
look-blurred-background-detail = Kompyuta ya mezani huonekana kupitia upau wa juu na folda, ikiwa na ukungu, na menyu na madirisha ibukizi huwa kama kioo chenye ukungu.
look-blur = Tia ukungu kilicho nyuma ya dirisha
look-blur-detail = Barua hubaki kwenye kadi thabiti, kwa hivyo maandishi hubaki na utofautishaji wake
look-blur-off-kde = Athari ya ukungu ya KDE imezimwa. Washa Blur katika System Settings, Window Management, Desktop Effects, kisha ufungue Katna Mail tena.
look-blur-none-gnome = GNOME haitii ukungu kilicho nyuma ya madirisha.
look-blur-none-x11 = Kidhibiti chako cha madirisha hakitii ukungu kilicho nyuma ya madirisha.
look-blur-none-wayland = Kiunganishaji chako cha michoro (compositor) hakitii ukungu kilicho nyuma ya madirisha.

## Settings > User feedback (crash reports)

feedback-intro-sending = Ripoti mpya za kuacha kufanya kazi hutumwa ili kusaidia kurekebisha tatizo. Hakuna kitu kingine kinachotoka kwenye kompyuta hii.
feedback-intro-local = Katna haitumi chochote popote. Ripoti za kuacha kufanya kazi hubaki kwenye kompyuta hii, ili uzitazame au uziambatishe kwenye ripoti ya hitilafu.
feedback-crash-reports = Ripoti za kuacha kufanya kazi
feedback-crash-reports-detail = Huandikwa Katna Mail au huduma yake ya chinichini inapoacha kufanya kazi.
feedback-save = Hifadhi ripoti za kuacha kufanya kazi kwenye kompyuta hii
feedback-save-detail = Folda yako ya nyumbani, majina ya mtumiaji na kompyuta, na anwani za barua pepe huachwa nje
feedback-saved = Ripoti zilizohifadhiwa
feedback-saved-detail = { $count ->
    [one] Ripoti { $count } ya hivi karibuni zaidi huhifadhiwa.
   *[other] Ripoti { $count } za hivi karibuni zaidi huhifadhiwa.
}
feedback-help-improve = Saidia kuboresha Katna
feedback-help-improve-detail = Imezimwa isipokuwa ukiiwasha, na unaweza kuizima hapa wakati wowote.
feedback-send = Tuma ripoti za kuacha kufanya kazi
feedback-send-detail = Ripoti iliyohifadhiwa, kama unavyoweza kuiona hapa, hutumwa kwa kifuatiliaji cha Katna cha kuacha kufanya kazi (Sentry, katika EU). Hakuna anwani ya IP, jumbe wala anwani za barua pepe
feedback-none-saved = Hakuna ripoti za kuacha kufanya kazi zilizohifadhiwa.
feedback-delete-all = Futa zote
feedback-app-daemon = Huduma ya chinichini
feedback-report-sent = { $date } · Imetumwa
feedback-view = Tazama
feedback-view-tooltip = Fungua ripoti
feedback-copy-tooltip = Nakili ili uibandike kwenye ripoti ya hitilafu
feedback-copied = Ripoti ya kuacha kufanya kazi imenakiliwa.
feedback-deleted-all = Ripoti za kuacha kufanya kazi zimefutwa.
feedback-read-failed = Imeshindwa kusoma ripoti ya kuacha kufanya kazi: { $error }
feedback-delete-failed = Imeshindwa kufuta ripoti ya kuacha kufanya kazi: { $error }
feedback-delete-all-failed = Imeshindwa kufuta ripoti za kuacha kufanya kazi: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _Faili
desktop-menu-new-message = _Ujumbe Mpya
desktop-menu-quit = _Ondoka
desktop-menu-edit = _Hariri
desktop-menu-undo = _Tendua
desktop-menu-select-all = Chagua _Zote
desktop-menu-select-none = _Usichague Yoyote
desktop-menu-find = Ta_futa…
desktop-menu-view = _Onyesha
desktop-menu-folder-list = Onyesha Orodha ya _Folda
desktop-menu-refresh = Onyesha _Upya
desktop-menu-go = _Nenda
desktop-menu-inbox = _Kikasha
desktop-menu-starred = Zenye _Nyota
desktop-menu-sent = Zilizo_tumwa
desktop-menu-drafts = _Rasimu
desktop-menu-all-mail = _Barua Zote
desktop-menu-next = Mazungumzo _Yanayofuata
desktop-menu-previous = Mazungumzo Ya_liyotangulia
desktop-menu-message = _Ujumbe
desktop-menu-open = _Fungua
desktop-menu-reply = _Jibu
desktop-menu-reply-all = Jibu _Wote
desktop-menu-forward = _Sambaza
desktop-menu-archive = Weka kwenye _Kumbukumbu
desktop-menu-delete = F_uta
desktop-menu-spam = Ripoti _Taka
desktop-menu-move-to = _Hamishia…
desktop-menu-mark-read = Tia Alama Kuwa I_mesomwa
desktop-menu-mark-unread = Tia Alama Kuwa H_aijasomwa
desktop-menu-star = Weka N_yota
desktop-menu-important = Tia Alama Kuwa Muh_imu
desktop-menu-not-important = Tia Alama Kuwa _Si Muhimu
desktop-menu-settings = _Mipangilio
desktop-menu-quick-settings = Mipangilio ya _Haraka
desktop-menu-configure = _Sanidi Katna Mail…
desktop-menu-help = _Usaidizi
desktop-menu-shortcuts = Njia za Mkato za _Kibodi
desktop-menu-whats-new = Kilicho _Kipya
desktop-menu-about = Kuhusu K_atna

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Kusogea
shortcut-group-actions = Vitendo
shortcut-group-go-to = Nenda kwenye
shortcut-group-app = Programu

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Mazungumzo yanayofuata
shortcut-previous = Mazungumzo yaliyotangulia
shortcut-down = Shuka chini kwenye orodha
shortcut-up = Panda juu kwenye orodha
shortcut-first = Ya kwanza katika orodha
shortcut-last = Ya mwisho katika orodha
shortcut-page-down = Ukurasa mmoja chini katika orodha
shortcut-page-up = Ukurasa mmoja juu katika orodha
shortcut-open = Fungua mazungumzo
shortcut-back = Rudi kwenye orodha
shortcut-scroll-down = Sogeza chini
shortcut-scroll-up = Sogeza juu
shortcut-scroll-page-down = Sogeza ukurasa mmoja chini
shortcut-scroll-page-up = Sogeza ukurasa mmoja juu
shortcut-compose = Tunga
shortcut-reply = Jibu
shortcut-reply-all = Jibu wote
shortcut-forward = Sambaza
shortcut-archive = Weka kwenye kumbukumbu
shortcut-delete = Futa
shortcut-spam = Ripoti taka
shortcut-move-to = Hamishia
shortcut-mark-read = Tia alama kuwa imesomwa
shortcut-mark-unread = Tia alama kuwa haijasomwa
shortcut-star = Weka au ondoa nyota
shortcut-important = Tia alama kuwa muhimu
shortcut-not-important = Tia alama kuwa si muhimu
shortcut-check = Weka tiki kwenye mazungumzo
shortcut-select-all = Weka tiki kwenye mazungumzo yote
shortcut-select-none = Ondoa tiki kwenye mazungumzo yote
shortcut-undo = Tendua kitendo cha mwisho
shortcut-go-inbox = Kikasha
shortcut-go-starred = Zenye nyota
shortcut-go-sent = Zilizotumwa
shortcut-go-drafts = Rasimu
shortcut-go-all = Barua zote
shortcut-search = Tafuta barua
shortcut-navigation = Onyesha au kunja menyu
shortcut-quick-settings = Mipangilio ya haraka
shortcut-settings = Mipangilio yote
shortcut-shortcuts = Njia za mkato za kibodi
shortcut-reload = Angalia barua mpya
shortcut-quit = Ondoka

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } kisha { $second }

## Settings > Accounts

accounts-folder-pane = Kidirisha cha folda
accounts-folder-pane-detail = Folda za akaunti zipi zinaonyeshwa kwenye kidirisha cha kushoto.
accounts-shown-one = Akaunti moja kwa wakati; badilisha kwenye kadi ya akaunti
accounts-shown-all = Akaunti zote, moja baada ya nyingine
accounts-row = Akaunti
accounts-row-detail = Kuondoa akaunti hufuta nakala ya Katna ya barua zake kwenye kompyuta hii. Barua hubaki kwenye seva.
accounts-none = Bado hakuna akaunti.
accounts-kind-imported = Iliyoingizwa
accounts-picture-reset = Tumia picha ya kompyuta ya mezani
accounts-picture-change = Badilisha picha
accounts-remove = Ondoa
accounts-delete-all-row = Futa data yote
accounts-delete-all-row-detail = Anza upya, kama kwenye usakinishaji mpya.
accounts-delete-all-about = Hufuta kila akaunti, barua zote zilizohifadhiwa, anwani na kalenda, faharasa ya utafutaji, mipangilio yako na manenosiri yaliyohifadhiwa kwenye kompyuta hii. Hakuna kinachobadilika kwenye seva zako za barua.
accounts-delete-all-open = Futa data yote ya Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } imeondolewa kwenye Katna.
accounts-removed = { $address } imeondolewa kwenye Katna. Barua zake bado ziko kwenye seva.
accounts-all-deleted = Data yote ya Katna imefutwa kwenye kompyuta hii.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Ungependa kuondoa { $address }?
accounts-remove-confirm = Ondoa akaunti
accounts-removing = Inaondoa…
accounts-remove-local-mail = { $folders ->
    [0] Barua zote zilizoingizwa kwenye akaunti hii
    [one] Barua zote zilizoingizwa kwenye akaunti hii katika folda yake
   *[other] Barua zote zilizoingizwa kwenye akaunti hii katika folda zake { $folders }
}
accounts-remove-local-settings = Mipangilio yake ya Katna
accounts-remove-mail = { $folders ->
    [0] Barua zote za akaunti hii zilizohifadhiwa na Katna
    [one] Barua zote za akaunti hii zilizohifadhiwa na Katna katika folda yake
   *[other] Barua zote za akaunti hii zilizohifadhiwa na Katna katika folda zake { $folders }
}
accounts-remove-outbox = Jumbe zake zinazosubiri kwenye kikasha toezi
accounts-remove-settings = Nenosiri lake lililohifadhiwa na mipangilio yake ya Katna
accounts-delete-all-title = Ungependa kufuta data yote ya Katna?
accounts-delete-all-confirm = Futa kila kitu
accounts-deleting = Inafuta…
accounts-delete-all-accounts = Kila akaunti, na barua zote na viambatisho vilivyohifadhiwa na Katna
accounts-delete-all-contacts = Anwani, kalenda na faharasa ya utafutaji
accounts-delete-all-settings = Mipangilio yote, sahihi na njia za mkato za kibodi
accounts-delete-all-passwords = Kila nenosiri lililohifadhiwa
accounts-deleted-heading = Vitafutwa kwenye kompyuta hii:
accounts-cannot-undo = Hatua hii haiwezi kutenduliwa.
accounts-server-delete-all = Hakuna kinachobadilika kwenye seva zako za barua: barua zako hubaki huko, na kuongeza akaunti tena huzipakua tena. Barua zilizoingizwa kutoka kwenye faili ziko katika Katna pekee; faili haziguswi.
accounts-server-local = Barua hizi ziliingizwa kutoka kwenye faili, kwa hivyo Katna ina nakala pekee. Faili zilikotoka haziguswi; ziingize tena ili uzirejeshe.
accounts-server-remove = Hakuna kinachobadilika kwenye seva ya barua: barua zako hubaki huko, na kuongeza akaunti tena huzipakua tena.
accounts-confirm-word = futa
accounts-confirm-placeholder = Andika “{ accounts-confirm-word }”
accounts-confirm-prompt = Ili kuthibitisha, andika “{ accounts-confirm-word }”:
accounts-cancel = Ghairi
