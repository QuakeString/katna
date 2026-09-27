# Katna Mail, Hausa (Hausa).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Harshe: { $language }
language-tooltip-system = Harshe: { $language }, yana bin tsarin
language-search = Bincika harshe
language-system-default = Na asali na tsarin
language-system-now = Yanzu { $language }
language-no-match = Babu harshen da ya dace da “{ $query }”
language-machine = Na'ura ce ta fassara. Taimaka a inganta shi
language-setting = Harshe
language-setting-detail = Harshen menu, maɓallai da saƙonni, da tsarin kwanan wata da lambobi. Na asali na tsarin yana bin tebur.

## Dates and sizes

ago-just-now = yanzu yanzu
ago-minutes = { $count ->
    [one] minti { $count } da ya wuce
   *[other] mintuna { $count } da suka wuce
}
ago-hours = { $count ->
    [one] awa { $count } da ya wuce
   *[other] awanni { $count } da suka wuce
}
ago-days = { $count ->
    [one] kwana { $count } da ya wuce
   *[other] kwanaki { $count } da suka wuce
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

folders-hide = Ɓoye folda
folders-show = Nuna folda
compose = Rubuta
search = Bincika
search-mail = Bincika wasiƙu
search-settings = Bincika saituna
search-clear = Share bincike
search-options-show = Nuna zaɓuɓɓukan bincike
settings = Saituna
account-add = Ƙara asusu

## App rail (and the bottom bar on a phone)

rail-mail = Wasiƙu
rail-calendar = Kalanda
rail-contacts = Lambobin sadarwa
rail-tasks = Ayyuka
rail-notes = Bayanai
rail-feeds = Ciyarwa

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Yana zuwa nan ba da daɗewa ba
app-calendar-promise = Kalandarku na CalDAV, gayyatar taro daga wasiƙunku da tunatarwa, kusa da akwatin saƙonku.
app-tasks-promise = Jerin abubuwan yi da ke daidaitawa da CalDAV, da ayyukan da aka yi daga wasiƙu.
app-notes-promise = Bayanai na gaggawa, da bayanai kan wasiƙa ko tattaunawa don nan gaba.
app-feeds-promise = Karanta ciyarwar RSS da Atom kusa da wasiƙunku.

## Contacts page

app-contacts-loading = Ana tattara mutane daga wasiƙunku…
app-contacts-empty = Mutanen da kuke rubuta wa juna za su bayyana a nan.
app-contacts-count = { $count ->
    [one] Mutum { $count } daga wasiƙunku, waɗanda kuka fi rubuta wa farko
   *[other] Mutane { $count } daga wasiƙunku, waɗanda kuka fi rubuta wa farko
}
app-contacts-top = { $count ->
    [one] Babban mutum { $count } daga wasiƙunku, waɗanda kuka fi rubuta wa farko
   *[other] Manyan mutane { $count } daga wasiƙunku, waɗanda kuka fi rubuta wa farko
}
app-contacts-messages = { $count ->
    [one] saƙo { $count }
   *[other] saƙonni { $count }
}
app-contacts-last = na ƙarshe { $date }

## Navigation (the folders pane)

nav-labels = Lakabai
nav-folders = Folda
nav-label-new = Ƙirƙiri sabon lakabi
nav-folder-new = Ƙirƙiri sabuwar folda
nav-account-unnamed = Asusu { $number }
nav-tab-new = { $count ->
    [one] { $count } sabo
   *[other] { $count } sababbi
}

## Special folders (the user's own folders keep their names)

folder-inbox = Akwatin saƙo
folder-starred = Masu tauraro
folder-drafts = Zayyanai
folder-sent = Waɗanda aka aika
folder-archive = Ma'ajiya
folder-spam = Saƙonnin banza
folder-trash = Kwandon shara
folder-all-mail = Duk wasiƙu
folder-scheduled = Waɗanda aka tsara

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Sabon lakabi
label-folder-new-title = Sabuwar folda
label-prompt = Da fatan za a shigar da sabon sunan lakabi:
label-folder-prompt = Da fatan za a shigar da sabon sunan folda:
label-name-hint = Sunan lakabi
label-folder-name-hint = Sunan folda
label-nest = Sanya lakabi a ƙarƙashin:
label-folder-nest = Sanya folda a ƙarƙashin:
label-cancel = Soke
label-create = Ƙirƙira
label-creating = Ana ƙirƙira…
label-created = An ƙirƙiri lakabi “{ $name }”.
label-folder-created = An ƙirƙiri folda “{ $name }”.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Na farko
tab-promotions = Tallace-tallace
tab-social = Zamantakewa
tab-updates = Sabuntawa
tab-forums = Dandali
tab-focused = Mai da hankali
tab-other = Sauran
tab-inbox = Akwatin saƙo
tab-newsletters = Wasiƙun labarai
tab-notifications = Sanarwa
tab-new = { $count } sababbi
tab-provider-other = Katna ne ya tsara

## Mail list: toolbar

list-select = Zaɓi
list-refresh = Sabunta
list-more = Ƙari
list-mark-read = Yi alama an karanta
list-mark-unread = Yi alama ba a karanta ba
list-move-to = Matsar zuwa
list-archive = Adana a ma'ajiya
list-spam = Rahoto saƙon banza
list-delete = Share
list-newer = Sababbi
list-older = Tsofaffi
list-range = { $first }–{ $last } cikin { $total }
list-range-about = { $first }–{ $last } cikin kusan { $total }
list-results = Sakamakon “{ $query }”
list-results-corrected = Ana nuna sakamakon “{ $query }”
list-search-instead = Maimakon haka bincika “{ $query }”
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Duka
list-pick-none = Babu
list-pick-read = An karanta
list-pick-unread = Ba a karanta ba
list-pick-starred = Masu tauraro
list-pick-unstarred = Marasa tauraro

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] An zaɓi tattaunawa { $count }.
       *[other] An zaɓi dukkan tattaunawa { $count }.
    }
   *[message] { $count ->
        [one] An zaɓi saƙo { $count }.
       *[other] An zaɓi dukkan saƙonni { $count }.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] An zaɓi tattaunawa { $count } da ke cikin { $folder }.
       *[other] An zaɓi dukkan tattaunawa { $count } da ke cikin { $folder }.
    }
   *[message] { $count ->
        [one] An zaɓi saƙo { $count } da ke cikin { $folder }.
       *[other] An zaɓi dukkan saƙonni { $count } da ke cikin { $folder }.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] An zaɓi tattaunawa { $count } da ke kan allo.
       *[other] An zaɓi dukkan tattaunawa { $count } da ke kan allo.
    }
   *[message] { $count ->
        [one] An zaɓi saƙo { $count } da ke kan allo.
       *[other] An zaɓi dukkan saƙonni { $count } da ke kan allo.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Zaɓi tattaunawa { $count }
       *[other] Zaɓi dukkan tattaunawa { $count }
    }
   *[message] { $count ->
        [one] Zaɓi saƙo { $count }
       *[other] Zaɓi dukkan saƙonni { $count }
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Zaɓi tattaunawa { $count } da ke cikin { $folder }
       *[other] Zaɓi dukkan tattaunawa { $count } da ke cikin { $folder }
    }
   *[message] { $count ->
        [one] Zaɓi saƙo { $count } da ke cikin { $folder }
       *[other] Zaɓi dukkan saƙonni { $count } da ke cikin { $folder }
    }
}
list-clear-selection = Share zaɓi

## Mail list: empty states

list-empty-search = Babu saƙonnin da suka dace da bincikenku.
list-empty-tab = Babu wasiƙu a cikin { $tab }.
list-empty-tab-unknown = Babu wasiƙu a cikin wannan shafi.
list-empty-folder = Babu saƙonni a cikin { $folder }.
list-empty-folder-unknown = Babu saƙonni a cikin wannan folda.
list-first-sync = Ana samo wasiƙunku…
list-first-sync-detail = Za su bayyana a nan yayin da suke isowa.

## Mail list: lines

row-removed = An cire wannan saƙo.
row-starred = Mai tauraro
row-not-starred = Babu tauraro
row-important = Muhimmi. Danna don yin alama ba muhimmi ba.
row-mark-important = Yi alama muhimmi
row-pinned = An maƙala a sama
row-pin = Maƙala a sama
row-unpin = Cire maƙalawa

## Mail list: More menu and right-click menu

menu-reply = Amsa
menu-reply-all = Amsa wa kowa
menu-forward = Tura
menu-archive = Adana a ma'ajiya
menu-delete = Share
menu-spam = Rahoto saƙon banza
menu-mark-read = Yi alama an karanta
menu-mark-unread = Yi alama ba a karanta ba
menu-mark-all-read = Yi wa duka alama an karanta
menu-star = Saka tauraro
menu-unstar = Cire tauraro
menu-important = Yi alama muhimmi
menu-not-important = Yi alama ba muhimmi ba
menu-pin = Maƙala a sama
menu-unpin = Cire maƙalawa
menu-print-all = Buga duka
menu-new-window = Buɗe a sabuwar taga
menu-move-to = Matsar zuwa
menu-move-to-heading = Matsar zuwa:
menu-find-from = Nemo imel daga { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] An adana tattaunawa a ma'ajiya.
       *[other] An adana tattaunawa { $count } a ma'ajiya.
    }
   *[message] { $count ->
        [one] An adana saƙo a ma'ajiya.
       *[other] An adana saƙonni { $count } a ma'ajiya.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] An matsar da tattaunawa zuwa Kwandon shara.
       *[other] An matsar da tattaunawa { $count } zuwa Kwandon shara.
    }
   *[message] { $count ->
        [one] An matsar da saƙo zuwa Kwandon shara.
       *[other] An matsar da saƙonni { $count } zuwa Kwandon shara.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] An matsar da tattaunawa.
       *[other] An matsar da tattaunawa { $count }.
    }
   *[message] { $count ->
        [one] An matsar da saƙo.
       *[other] An matsar da saƙonni { $count }.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] An saka wa tattaunawa tauraro.
       *[other] An saka wa tattaunawa { $count } tauraro.
    }
   *[message] { $count ->
        [one] An saka wa saƙo tauraro.
       *[other] An saka wa saƙonni { $count } tauraro.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] An cire tauraro daga tattaunawa.
       *[other] An cire tauraro daga tattaunawa { $count }.
    }
   *[message] { $count ->
        [one] An cire tauraro daga saƙo.
       *[other] An cire tauraro daga saƙonni { $count }.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] An yi wa tattaunawa alama muhimmiya.
       *[other] An yi wa tattaunawa { $count } alama muhimmai.
    }
   *[message] { $count ->
        [one] An yi wa saƙo alama muhimmi.
       *[other] An yi wa saƙonni { $count } alama muhimmai.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] An yi wa tattaunawa alama ba muhimmiya ba.
       *[other] An yi wa tattaunawa { $count } alama ba muhimmai ba.
    }
   *[message] { $count ->
        [one] An yi wa saƙo alama ba muhimmi ba.
       *[other] An yi wa saƙonni { $count } alama ba muhimmai ba.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] An maƙala tattaunawa a sama.
       *[other] An maƙala tattaunawa { $count } a sama.
    }
   *[message] { $count ->
        [one] An maƙala saƙo a sama.
       *[other] An maƙala saƙonni { $count } a sama.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] An cire maƙalawar tattaunawa.
       *[other] An cire maƙalawar tattaunawa { $count }.
    }
   *[message] { $count ->
        [one] An cire maƙalawar saƙo.
       *[other] An cire maƙalawar saƙonni { $count }.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] An kai rahoton tattaunawa a matsayin saƙon banza.
       *[other] An kai rahoton tattaunawa { $count } a matsayin saƙonnin banza.
    }
   *[message] { $count ->
        [one] An kai rahoton saƙo a matsayin saƙon banza.
       *[other] An kai rahoton saƙonni { $count } a matsayin saƙonnin banza.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] An share tattaunawa har abada.
       *[other] An share tattaunawa { $count } har abada.
    }
   *[message] { $count ->
        [one] An share saƙo har abada.
       *[other] An share saƙonni { $count } har abada.
    }
}
toast-undone = An janye aikin.
toast-undo = Janye
toast-no-spam-folder = Wannan asusun ba shi da foldar saƙonnin banza.

## Reading pane: toolbar

reader-close = Rufe
reader-back = Koma baya
reader-mark-unread = Yi alama ba a karanta ba
reader-move-to = Matsar zuwa
reader-more = Ƙari
reader-print-all = Buga duka
reader-new-window = A sabuwar taga
reader-position = { $position } cikin { $total }
reader-newer = Sabuwa
reader-older = Tsohuwa

## Reading pane: the conversation

reader-removed = An cire wannan tattaunawa.
reader-no-subject = (babu jigo)
reader-collapse-all = Naɗe duka
reader-expand-all = Buɗe duka
reader-unknown-sender = (mai aikawa da ba a sani ba)
reader-date-ago = { $date } ({ $ago })
reader-me = ni
reader-to = zuwa ga { $names }
reader-starred = Mai tauraro
reader-not-starred = Babu tauraro
reader-too-long = Saƙon ya yi tsayi da yawa don a nuna shi gaba ɗaya.
reader-encrypted-images = Ba a taɓa loda hotuna daga yanar gizo a cikin wasiƙar da aka ɓoye.
reader-window-failed = Ba a iya buɗe sabuwar taga ba.

## Reading pane: message details (opened from "to me")

reader-details-from = daga:
reader-details-to = zuwa:
reader-details-cc = kwafi:
reader-details-date = kwanan wata:
reader-details-subject = jigo:

## Reading pane: downloading a message

reader-downloading = Ana sauke wannan saƙo daga sabar…
reader-download-failed = Ba a iya sauke wannan saƙo ba.
reader-try-again = Sake gwadawa

## Reply row

reply-reply = Amsa
reply-reply-all = Amsa wa kowa
reply-forward = Tura

## Encrypted and signed mail

security-decrypting = Ana warware ɓoyewa…
security-checking = Ana duba sa hannu…
security-partly-encrypted = Wani ɓangare ne kawai na wannan saƙo aka ɓoye. An ƙara sauran a wajen kariyar kuma yana iya fitowa daga kowa.
security-partly-signed = Wani ɓangare ne kawai na wannan saƙo aka sa wa hannu. An ƙara sauran a wajen kariyar kuma yana iya fitowa daga kowa.
security-encrypted = Saƙon da aka ɓoye
security-encrypted-smime = Saƙon da aka ɓoye (S/MIME)
security-no-key = Ba za a iya warware wannan saƙo ba: an ɓoye shi da maɓallin da ba ku da shi.
security-cancelled = An soke warware ɓoyewa.
security-damaged = Ba za a iya warware wannan saƙo ba: bayanan da aka ɓoye sun lalace ko an canza su.
security-decrypt-unavailable = Ba za a iya warware wannan saƙo ba: shigar da { $tool } don karanta wasiƙun da aka ɓoye.
security-decrypt-failed = Ba za a iya warware wannan saƙo ba: { $reason }
security-unknown-signer = wanda ba a san shi ba
security-signed-verified = { $signer } ne ya sa hannu · an tabbatar
security-signed-not-sender = { $signer } ne ya sa hannu, wanda ba shi ne mai aikawa ba
security-signed-untrusted = { $signer } ne ya sa hannu, da maɓallin da kuka yi wa alama ba amintacce ba
security-signed-unverified = { $signer } ne ya sa hannu · ba a tabbatar da maɓallin ba
security-bad-signature = Sa hannu mara kyau: an canza wannan saƙo bayan an sa masa hannu, ko sa hannun na jabu ne.
security-signature-expired = { $signer } ne ya sa hannu · wa'adin sa hannun ya ƙare
security-key-expired = { $signer } ne ya sa hannu · wa'adin maɓallin ya ƙare tun daga lokacin
security-key-revoked = { $signer } ne ya sa hannu da maɓallin da aka soke
security-missing-key = An sa hannu da maɓallin da ba ku da shi, don haka ba za a iya duba shi ba
security-missing-key-id = An sa hannu da maɓallin da ba ku da shi ({ $key }), don haka ba za a iya duba shi ba
security-signature-unavailable = An sa hannu; shigar da { $tool } don duba sa hannun
security-signature-error = Ba a iya duba sa hannun ba.

## Remote images and pictures

remote-hidden = An ɓoye hotunan da ke cikin wannan saƙo.
remote-show = Nuna hotuna
remote-always-show = Koyaushe nuna daga wannan mai aikawa
remote-picture-use = Yi amfani
remote-picture-too-big = Zaɓi hoto mai girman 8 MB ko ƙasa da haka.
remote-picture-type = Zaɓi hoton PNG, JPEG, GIF, WebP ko SVG.
remote-picture-read-failed = Ba za a iya karanta hoton ba: { $error }
remote-picture-keep-failed = Ba za a iya ajiye hoton ba: { $error }
remote-picture-remove-failed = Ba za a iya cire hoton ba: { $error }

## Attachments

attachment-count = { $count ->
    [one] Abin haɗawa ɗaya
   *[other] Abubuwan haɗawa { $count }
}
attachment-save = Ajiye
attachment-save-all = Ajiye duka
attachment-save-all-tooltip = Ajiye kowane abin haɗawa a cikin folda
attachment-save-here = Ajiye a nan
attachment-not-downloaded = Ba a sauke wannan saƙo ba.
attachment-not-found = Ba a sami wannan abin haɗawa a cikin saƙon ba.
attachment-read-failed = Ba a iya karanta { $name } ba
attachment-numbered = abin haɗawa { $number }
attachment-saved-all = { $count ->
    [one] An ajiye fayil { $count } a { $place }
   *[other] An ajiye fayiloli { $count } a { $place }
}
attachment-saved-some = { $total ->
    [one] An ajiye { $saved } cikin fayil { $total } a { $place }. Ba a iya ajiye { $failed } ba
   *[other] An ajiye { $saved } cikin fayiloli { $total } a { $place }. Ba a iya ajiye { $failed } ba
}
attachment-saved-to = An ajiye a { $path }
attachment-save-failed = Ba a iya ajiye { $name } ba: { $error }
attachment-open-failed = Ba a iya buɗe { $name } ba: { $error }
attachment-risky = Wannan fayil na iya gudanar da shiri, don haka Katna ba ta buɗe shi. Ajiye shi maimakon haka.
attachment-encrypted-open = Wannan fayil ya zo a ɓoye. Ajiye shi don buɗe shi a wani wuri.

## Printing

print-failed = Ba a iya bugawa ba: { $error }
print-no-font = ba a sami rubutun haruffa ba
print-opened-as-pdf = An buɗe shi a matsayin PDF don bugawa daga can.
print-not-downloaded = (Har yanzu ba a sauke shi ba.)
print-encrypted = (An ɓoye shi. Buɗe shi a cikin Katna Mail don buga rubutunsa.)
print-to = Zuwa: { $addresses }
print-cc = Kwafi: { $addresses }
