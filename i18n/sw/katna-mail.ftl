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
