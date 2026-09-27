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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Buɗe wannan saƙo don karanta abubuwan haɗawarsa.
text-copy = Kwafa
text-select-all = Zaɓi duka

## Settings page: its tabs

settings-tab-general = Gabaɗaya
settings-tab-inbox = Akwatin saƙo
settings-tab-accounts = Asusu
settings-tab-subscriptions = Rajista
settings-tab-appearance = Bayyanar
settings-tab-shortcuts = Gajerun hanyoyi
settings-tab-default-apps = Manhajojin asali
settings-tab-folders-rules = Folda da ƙa'idoji
settings-tab-compose = Rubutu
settings-tab-mcp-server = Sabar MCP
settings-tab-feedback = Ra'ayin mai amfani
settings-tab-experimental = Na gwaji

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Duba wasiƙun labarai da jerin wasiƙun da kuke samu, kuma ku daina karɓa da danna ɗaya.
settings-tab-folders-rules-coming = Ƙirƙira, sake suna, matsar da ɓoye folda da lakabai, kuma zaɓi waɗanda za su daidaita. Ƙa'idoji suna tsara, lakaftawa, turawa ko share sabbin wasiƙu da kansu, bisa mai aikawa, jigo ko kalmomi.
settings-tab-mcp-server-coming = Bari mataimakan AI da ke kan wannan kwamfuta su bincika, karanta da zayyana wasiƙunku, da izininku.

## Settings > General

settings-general-conversations = Duban tattaunawa
settings-general-conversations-group = Haɗa amsoshin wasiƙa ɗaya wuri ɗaya
settings-general-conversations-group-detail = Layi ɗaya ga kowace tattaunawa a cikin jerin
settings-general-reading = Karatu
settings-general-newest-first = Sabon saƙo farko
settings-general-newest-first-detail = Tattaunawa tana farawa da amsarta ta ƙarshe
settings-general-full-headers = Nuna cikakkun kanun saƙo
settings-general-full-headers-detail = Daga, zuwa, kwafi, kwanan wata da jigo suna buɗe a kowane saƙo
settings-general-full-names = Cikakkun sunayen masu karɓa
settings-general-full-names-detail = “zuwa gare ni, Ada Lovelace” maimakon “zuwa gare ni, Ada”
settings-general-mark-read = Yi alama an karanta
settings-general-mark-read-now = Da zarar ya buɗe
settings-general-mark-read-1s = Bayan ya kasance a buɗe na daƙiƙa 1
settings-general-mark-read-3s = Bayan ya kasance a buɗe na daƙiƙa 3
settings-general-mark-read-never = Sai lokacin da na yi masa alama an karanta
settings-general-reply-button = Maɓallin amsa
settings-general-reply-all = Amsa wa kowa
settings-general-reply-all-detail = Maɓallin amsa da ke kusa da kowane saƙo yana amsa wa kowa, ba mai aikawa kaɗai ba
settings-general-remote-images = Hotuna daga yanar gizo
settings-general-remote-images-detail = Loda hotunan saƙo yana sanar da mai aikawa cewa kun buɗe shi, da lokacin, da kusan inda kuke. Idan a kashe, kowane saƙo zai fara tambaya, kuma koyaushe kuna iya nuna hotunan mai aikawa.
settings-general-remote-images-always = Koyaushe nuna hotuna
settings-general-remote-images-always-detail = A kowane saƙo, ba daga masu aikawa da kuka amince da su kaɗai ba
settings-general-sending = Aikawa
settings-general-sending-detail = Tsawon lokacin da saƙon da aka aika zai jira, don a iya janye shi.
settings-general-offline = Wasiƙu ba tare da intanet ba
settings-general-offline-detail = Ana sauke sababbin wasiƙu gaba ɗaya, don karantawa ba tare da haɗi ba. Tsofaffin wasiƙu suna sauka lokacin da kuka buɗe su.
settings-general-offline-days = { $count ->
    [one] kwana { $count }
   *[other] kwanaki { $count }
}
settings-general-offline-years = { $count ->
    [one] shekara { $count }
   *[other] shekaru { $count }
}
settings-general-offline-all = Duk wasiƙu
settings-general-offline-note = Zaɓar ƙananan kwanaki yana ajiye wasiƙun da aka riga aka sauke. Babu abin da ke canzawa a sabar.
settings-general-notifications = Sanarwa
settings-general-notifications-detail = Don sababbin wasiƙu a Akwatin saƙo, ko da Katna Mail a rufe take.
settings-general-new-mail = Sanar da ni game da sababbin wasiƙu
settings-general-new-mail-detail = Tare da Amsa wa kowa, Yi alama an karanta da Adana a ma'ajiya
settings-general-new-mail-sound = Kunna sauti
settings-general-new-mail-sound-detail = Sautin sabuwar wasiƙa na tebur
settings-general-desktop = Tebur
settings-general-open-at-login = Buɗe Katna Mail lokacin shiga
settings-general-open-at-login-detail = Wasiƙu suna daidaitawa lokacin shiga ko ta yaya, muddin sabis ɗin yana aiki
settings-general-tray = Nuna Katna a cikin tiren tsarin
settings-general-tray-detail = Tare da adadin waɗanda ba a karanta ba da menu
settings-general-unread-badge = Adadin waɗanda ba a karanta ba a gunkin ma'ajin ayyuka
settings-general-unread-badge-detail = Yawan saƙonnin Akwatin saƙo da ba a karanta ba

## Settings > Inbox

settings-inbox-tabs = Shafukan akwatin saƙo
settings-inbox-tabs-detail = Raba akwatin saƙo zuwa shafuka, kamar yadda shafin yanar gizon mai ba ku sabis na wasiƙu yake yi.
settings-inbox-tabs-show = Nuna shafukan akwatin saƙo
settings-inbox-tabs-show-detail = Idan a kashe, jeri ɗaya ne ga kowane asusu
settings-inbox-no-accounts = Ƙara asusu don zaɓar shafukansa.
settings-inbox-tabs-automatic = Kai tsaye: { $tabs } ({ $provider })
settings-inbox-tabs-off = Babu shafuka
settings-inbox-tabs-gmail = Na farko, Tallace-tallace, Zamantakewa, Sabuntawa, Dandali
settings-inbox-tabs-focused = Mai da hankali da Sauran
settings-inbox-tabs-zoho = Akwatin saƙo, Wasiƙun labarai da Sanarwa
settings-inbox-tabs-shown = Shafukan da ake nunawa. Wasiƙun shafin da kuka kashe suna zama a { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Wurin karatu
settings-appearance-reading-pane-detail = Inda tattaunawar da aka buɗe take bayyana.
settings-appearance-pane-right = A dama da jerin
settings-appearance-pane-none = Babu rabuwa
settings-appearance-density = Matsatsi
settings-appearance-density-default = Na asali
settings-appearance-density-compact = Matsattse
settings-appearance-scaling = Girman nuni
settings-appearance-scaling-detail = Yana ƙara ko rage girman komai a Katna Mail, a kan girman nunin tebur: rubutu, gumaka, tazara da layukan rabuwa. Wasiƙun da kuke aikawa suna riƙe girman rubutunsu. Ƙananan girma sosai na iya sa gumaka su yi wuyar dannawa.
settings-appearance-theme = Jigon launi
settings-appearance-theme-system = Kamar tebur
settings-appearance-theme-light = Haske
settings-appearance-theme-dark = Duhu
settings-appearance-desktop-colors = Launukan tebur
settings-appearance-desktop-colors-use = Yi amfani da launukan tebur
settings-appearance-desktop-colors-use-detail = Tsarin launi da launin ƙawa na tebur
settings-appearance-app-names = Sunayen manhajoji
settings-appearance-app-names-show = Nuna sunayen manhajoji
settings-appearance-app-names-show-detail = Sunaye a ƙarƙashin gumakan manhajoji a can gefen hagu
settings-appearance-sender-pictures = Hotunan masu aikawa
settings-appearance-sender-pictures-show = Nuna tambarin kamfanoni
settings-appearance-sender-pictures-show-detail = Ana nemo su ta hanyar yankin mai aikawa, ba ta saƙo ba, kuma ana ajiye su na mako ɗaya
settings-appearance-important = Alamomin Muhimmi
settings-appearance-important-show = Nuna alamomin Muhimmi
settings-appearance-important-show-detail = Kusa da kowane saƙo a cikin jerin
settings-appearance-message-width = Faɗin saƙo
settings-appearance-message-width-limit = Iyakance faɗin saƙonni
settings-appearance-message-width-limit-detail = Dogayen layuka sun fi sauƙin karantawa a babbar taga
settings-appearance-mail-colors = Launukan wasiƙu
settings-appearance-mail-colors-detail = Yawancin wasiƙu an tsara su ne don farin shafi. Da jigon duhu, ana canza launukansu zuwa masu duhu da ake iya karantawa da kyau; idan a kashe, suna riƙe launukan mai aikawa a kan shafi mai haske.
settings-appearance-dark-mail = Launuka masu duhu ga wasiƙu ma
settings-appearance-dark-mail-detail = Sai lokacin da jigon yake duhu
settings-appearance-attachment-previews = Samfotin abubuwan haɗawa
settings-appearance-attachment-previews-show = Nuna samfotin abubuwan haɗawa
settings-appearance-attachment-previews-show-detail = Ƙaramin hoton abin da ke cikin kowane fayil a kan katinsa

## Settings > Default apps

settings-default-apps-intro = Inda abubuwan haɗawa suke buɗewa lokacin da kuka danna su. Mai dubawa koyaushe yana iya buɗe fayil a wata manhaja ma. Ana saita manhajojin asali na tebur a cikin saitunansa.
settings-default-apps-pdf = Fayilolin PDF
settings-default-apps-pdf-detail = Shafuka, tare da zuƙowa.
settings-default-apps-pictures = Hotuna
settings-default-apps-pictures-detail = Hotuna (an miƙe su tsaye), PNG, GIF, WebP, BMP, TIFF da SVG.
settings-default-apps-text = Fayilolin rubutu
settings-default-apps-text-detail = Rubutu mara ado, bayanan log, lamba da sauran rubutu.
settings-default-apps-sheets = Maƙunsar bayanai
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) da CSV.
settings-default-apps-documents = Takardu
settings-default-apps-documents-detail = Word (docx) da rubutun OpenDocument (odt).
settings-default-apps-katna = Mai dubawa na Katna Mail
settings-default-apps-system = Manhajar asali ta tebur
settings-default-apps-ask = Tambaye ni wace manhaja kowane lokaci
settings-default-apps-after-saving = Bayan ajiyewa
settings-default-apps-show-folder = Nuna fayilolin da aka ajiye a cikin foldarsu
settings-default-apps-show-folder-detail = Yana buɗe manajan fayiloli tare da zaɓaɓɓun abubuwan haɗawa da aka ajiye

## Settings > Compose

settings-compose-send-from = Aika sababbin saƙonni daga
settings-compose-send-from-detail = Amsoshi da turawa koyaushe suna fita daga asusun da kuke ciki.
settings-compose-send-from-current = Asusun da kuke ciki
settings-compose-send-on-replies = Aikawa a kan amsoshi
settings-compose-send-on-replies-detail = Abin da Aika ke yi a kan amsa ko turawa. Menu da ke kusa da Aika yana ba da ɗayan.
settings-compose-send-plain = Aika
settings-compose-send-archive = Aika kuma adana a ma'ajiya
settings-compose-signatures = Sa hannu
settings-compose-signatures-detail = Ana ƙara shi a ƙasan saƙonku, bayan layin “--”. Zaɓi wani a cikin tagar rubutu.
settings-compose-untitled = Babu suna
settings-compose-signature-name = Suna, kamar Aiki
settings-compose-signature-first = Sa hannuna
settings-compose-signature-numbered = Sa hannu { $number }
settings-compose-signature-delete = Share
settings-compose-signature-deleted = An share sa hannu
settings-compose-signature-new = Ƙirƙiri sabo
settings-compose-no-signatures = Babu sa hannu tukuna.
settings-compose-no-signature = Babu sa hannu
settings-compose-for-new-mail = Don sababbin wasiƙu
settings-compose-for-replies = Don amsoshi da turawa
settings-compose-for-replies-detail = A tattaunawar da kuka sa wa saƙo hannu, amsa tana farawa da wannan sa hannun maimakon haka.
settings-compose-format = Tsari
settings-compose-plain-text = Rubuta da rubutu mara ado
settings-compose-plain-text-detail = Sababbin wasiƙu suna farawa ba tare da tsari ba; tagar rubutu na iya sauyawa
settings-compose-spelling = Rubutun kalmomi
settings-compose-spell-check = Duba rubutun kalmomi yayin da nake rubutu
settings-compose-spell-check-detail = Ana ja layi a ƙarƙashin kalmomin da aka rubuta ba daidai ba, tare da shawarwari idan an danna dama
settings-compose-spell-desktop = Harshen tebur ({ $language })
settings-compose-templates = Samfura
settings-compose-templates-detail = Ajiye wasiƙun da kuke yawan rubutawa, kuma fara sabuwar wasiƙa ko amsa daga gare su.

## Settings > Shortcuts

settings-shortcuts-set = Saitin gajerun hanyoyi
settings-shortcuts-set-detail = Fara daga maɓallan manhajar wasiƙu da kuka sani. Cmd shi ne Ctrl a nan. Canje-canjenku suna zama a kan saitin, kuma Maido da na asali yana komawa ga maɓallan saitin.
settings-shortcuts-single = Gajerun hanyoyi na maɓalli ɗaya
settings-shortcuts-single-detail = Maɓallai ba tare da Ctrl ko Alt ba, kamar a wasiƙun yanar gizo: e yana adana a ma'ajiya, j da k suna motsawa, / yana bincike. Suna aiki a cikin jerin da tattaunawar da aka buɗe, ba yayin rubutu ba.
settings-shortcuts-single-use = Yi amfani da gajerun hanyoyi na maɓalli ɗaya
settings-shortcuts-single-use-detail = Gajerun hanyoyin Ctrl koyaushe suna aiki
settings-shortcuts-how = Danna maɓalli don canza shi, ko + don ƙara ɗaya, sannan danna sababbin maɓallan. Esc yana sokewa.
settings-shortcuts-restore = Maido da na asali
settings-shortcuts-no-key = Babu maɓalli
settings-shortcuts-press = Danna maɓallai…
settings-shortcuts-then = { $keys } sannan…
settings-shortcuts-moved = { $keys } yanzu yana yin “{ $action }” maimakon “{ $previous }”.
settings-shortcuts-single-off = Gajerun hanyoyi na maɓalli ɗaya a kashe suke, don haka wannan maɓalli zai yi aiki da zarar an kunna su.
settings-shortcuts-restored = Kowace gajeriyar hanya ta sake samun maɓallan saitinta.

## Settings search: the line under a result

settings-general-language-summary = Harshen manhaja, kwanan wata da lambobi
settings-general-reading-summary = Sabon saƙo farko, cikakkun kanun saƙo, cikakkun sunayen masu karɓa
settings-general-mark-read-summary = Lokacin da ake yi wa tattaunawar da aka buɗe alama an karanta: nan take, bayan daƙiƙa 1 ko 3, ko da hannu
settings-general-reply-button-summary = Maɓallin amsa da ke kusa da kowane saƙo yana amsa wa kowa
settings-general-remote-images-summary = Koyaushe nuna hotunan kowane saƙo
settings-general-sending-summary = Janye aikawa: tsawon lokacin da saƙon da aka aika zai jira, don a iya janye shi
settings-general-offline-summary = Kwanaki nawa na sababbin wasiƙu ake saukewa gaba ɗaya, don karantawa ba tare da haɗi ba
settings-general-notifications-summary = Sanarwar sababbin wasiƙu da sautinsu
settings-general-desktop-summary = Buɗe Katna Mail lokacin shiga, gunkin tiren tsarin da adadin waɗanda ba a karanta ba a gunkin ma'ajin ayyuka
settings-accounts-accounts-summary = Ƙara ko cire asusu, ko canza hotonsa
settings-appearance-density-summary = Layuka na asali ko matsattsu a cikin jerin
settings-appearance-scaling-summary = Ƙara ko rage girman komai: rubutu, gumaka, tazara da layukan rabuwa
settings-appearance-theme-summary = Kamar tebur, haske ko duhu
settings-appearance-sender-pictures-summary = Tambarin kamfanoni, ana nemo su ta hanyar yankin mai aikawa
settings-appearance-important-summary = Alamar Muhimmi kusa da kowane saƙo a cikin jerin
settings-appearance-mail-colors-summary = Launuka masu duhu ga wasiƙun HTML a jigon duhu, ko launukan mai aikawa
settings-appearance-attachment-previews-summary = Ƙaramin hoton abin da ke cikin kowane abin haɗawa
settings-shortcuts-set-summary = Fara daga maɓallan Gmail, Inbox by Gmail, Apple Mail, Outlook ko Thunderbird
settings-shortcuts-single-summary = Maɓallai ba tare da Ctrl ko Alt ba, kamar a wasiƙun yanar gizo
settings-default-apps-pdf-summary = Inda abubuwan haɗawa na PDF suke buɗewa
settings-default-apps-pictures-summary = Inda hotuna suke buɗewa
settings-default-apps-text-summary = Inda rubutu mara ado, bayanan log da lamba suke buɗewa
settings-default-apps-sheets-summary = Inda fayilolin Excel, OpenDocument da CSV suke buɗewa
settings-default-apps-documents-summary = Inda Word da rubutun OpenDocument suke buɗewa
settings-default-apps-after-saving-summary = Nuna abubuwan haɗawa da aka ajiye a cikin foldarsu
settings-compose-send-from-summary = Asusun da sababbin wasiƙu suke fita daga gare shi: wanda kuke ciki, ko koyaushe iri ɗaya
settings-compose-send-on-replies-summary = Aika, ko Aika kuma adana tattaunawar a ma'ajiya, a kan amsoshi da turawa
settings-compose-signatures-summary = Ana ƙara shi a ƙasan saƙonku, bayan layin “--”
settings-compose-for-new-mail-summary = Sa hannun da sababbin wasiƙu suke farawa da shi
settings-compose-for-replies-summary = Sa hannun da amsoshi da turawa suke farawa da shi
settings-compose-format-summary = Rubuta sababbin wasiƙu da rubutu mara ado
settings-compose-spelling-summary = Duba rubutun kalmomi yayin rubutu, da harshen ƙamus
settings-compose-templates-summary = Yana zuwa nan ba da daɗewa ba: ajiye wasiƙun da kuke yawan rubutawa, kuma fara sabuwar wasiƙa ko amsa daga gare su
settings-feedback-crash-reports-summary = Ajiye rahotannin faɗuwa a wannan kwamfuta lokacin da Katna Mail ko sabis ɗinta na bango ya faɗi
settings-feedback-saved-summary = Duba, kwafa ko share rahotannin faɗuwa da aka ajiye a wannan kwamfuta
settings-feedback-help-improve-summary = Aika rahotannin faɗuwa don taimakawa gyara abin da ya lalace; a kashe sai idan kun kunna
settings-experimental-blur-summary = Tebur yana bayyana ta sandar sama, a dushe, kuma menu suna kamar gilashi mai hazo
settings-search-shortcut = Gajeriyar hanyar madannai
settings-search-tab = Shafin saituna
settings-search-none = Babu saitunan da suka dace da “{ $query }”.
settings-search-results = Saitunan da suka dace da “{ $query }”
## Quick settings (the panel that slides in from the right)

quick-title = Saituna masu sauri
quick-see-all = Duba duk saituna
quick-reading-pane = Wurin karatu
quick-pane-right = A dama da jerin
quick-pane-none = Babu rabuwa
quick-density = Matsatsi
quick-density-default = Na asali
quick-density-compact = Matsattse
quick-theme = Jigon launi
quick-theme-system = Kamar tebur
quick-theme-light = Haske
quick-theme-dark = Duhu
quick-desktop-colors = Launukan tebur
quick-desktop-colors-detail = Tsarin launi da launin ƙawa na tebur
quick-app-names = Sunayen manhajoji
quick-app-names-detail = Sunaye a ƙarƙashin gumakan manhajoji a can gefen hagu
quick-inbox-tabs = Shafukan akwatin saƙo
quick-inbox-tabs-detail = Shafukan mai ba da sabis na wasiƙu na kowane asusu
quick-choose-tabs = Zaɓi shafuka
quick-choose-tabs-detail = Ga kowane asusu, a cikin Saituna
quick-sending = Aikawa
quick-undo-send = Janye aikawa
quick-undo-send-off = A kashe
quick-undo-send-seconds = { $seconds } s
quick-signatures = Sa hannu
quick-signatures-none = Babu tukuna
quick-signatures-one = { $name }, ana amfani da shi a matsayin na asali
quick-signatures-many = { $count ->
    [one] Sa hannu { $count }; { $name } shi ne na asali
   *[other] Sa hannu { $count }; { $name } shi ne na asali
}
quick-signatures-no-default = { $count ->
    [one] { $count }, babu na asali
   *[other] { $count }, babu na asali
}
quick-signature-untitled = Babu suna
quick-threading = Haɗa wasiƙu cikin tattaunawa
quick-conversation-view = Duban tattaunawa
quick-conversation-view-detail = Haɗa amsoshin wasiƙa ɗaya wuri ɗaya
quick-help = Taimako
quick-tour = Yi rangadi
quick-whats-new = Me ke sabo
quick-about = Game da Katna

## Settings: opening at login

settings-open-at-login-failed = Ba a iya canza buɗewa lokacin shiga ba: { $error }

## Settings > Appearance > Scaling

scale-letter = A
scale-percent = { $percent }%
scale-reset = Koma zuwa { $percent }%

## Settings > Experimental > Look & Feel

look-intro = Fasalolin da ake gwadawa har yanzu. Suna iya canzawa ko su ɓace.
look-heading = Kama da Ji
look-window-frame = Firam ɗin taga
look-window-frame-detail = Wanda ke zana sandar take, maɓallan taga, kusurwoyi da inuwa.
look-frame-native-kde = Na tsarin: firam ɗin KDE, a cikin jigon Plasma ɗinku
look-frame-native = Na tsarin: firam ɗin tebur
look-frame-katna = Katna: sandar sama ta zama sandar take
look-frame-katna-note-named = Katna tana zana kusurwoyi masu zagaye da inuwarta. Firam ɗin ba ya bin jigon { $desktop } kuma; ƙa'idojin taga suna aiki har yanzu.
look-frame-katna-note = Katna tana zana kusurwoyi masu zagaye da inuwarta. Firam ɗin ba ya bin jigon tebur kuma; ƙa'idojin taga suna aiki har yanzu.
look-frame-client-side = Teburinku yana bar wa kowace manhaja firam ɗinta, don haka Katna tana zana nata tuni.
look-blurred-background = Bango mai dushewa
look-blurred-background-detail = Tebur yana bayyana ta sandar sama da folda, a dushe, kuma menu da taga masu tasowa suna kamar gilashi mai hazo.
look-blur = Dushe abin da ke bayan taga
look-blur-detail = Wasiƙu suna zama a kan katuna masu ƙarfi, don haka rubutu yana riƙe bambancinsa
look-blur-off-kde = Tasirin Blur na KDE a kashe yake. Kunna Blur a System Settings, Window Management, Desktop Effects, sannan sake buɗe Katna Mail.
look-blur-none-gnome = GNOME ba ya dushe abin da ke bayan taga.
look-blur-none-x11 = Manajan taga ɗinku ba ya dushe abin da ke bayan taga.
look-blur-none-wayland = Compositor ɗinku ba ya dushe abin da ke bayan taga.

## Settings > User feedback (crash reports)

feedback-intro-sending = Ana aika sababbin rahotannin faɗuwa don taimakawa gyara abin da ya lalace. Babu wani abu kuma da ke barin wannan kwamfuta.
feedback-intro-local = Katna ba ta aika komai zuwa ko'ina. Rahotannin faɗuwa suna zama a wannan kwamfuta, don ku duba ko ku haɗa su da rahoton kwaro.
feedback-crash-reports = Rahotannin faɗuwa
feedback-crash-reports-detail = Ana rubuta su lokacin da Katna Mail ko sabis ɗinta na bango ya faɗi.
feedback-save = Ajiye rahotannin faɗuwa a wannan kwamfuta
feedback-save-detail = Ana cire foldar gidanku, sunayen mai amfani da na kwamfuta da adiresoshin imel
feedback-saved = Rahotannin faɗuwa da aka ajiye
feedback-saved-detail = { $count ->
    [one] Ana ajiye sabo { $count }.
   *[other] Ana ajiye sababbi { $count }.
}
feedback-help-improve = Taimaka a inganta Katna
feedback-help-improve-detail = A kashe sai idan kun kunna, kuma kuna iya kashe shi a nan a kowane lokaci.
feedback-send = Aika rahotannin faɗuwa
feedback-send-detail = Rahoton da aka ajiye, daidai yadda kuke iya duba shi a nan, yana zuwa mai bin diddigin faɗuwar Katna (Sentry, a EU). Babu adireshin IP, saƙonni ko adiresoshin imel
feedback-none-saved = Babu rahotannin faɗuwa da aka ajiye.
feedback-delete-all = Share duka
feedback-app-daemon = Sabis na bango
feedback-report-sent = { $date } · An aika
feedback-view = Duba
feedback-view-tooltip = Buɗe rahoton
feedback-copy-tooltip = Kwafa shi don liƙawa a cikin rahoton kwaro
feedback-copied = An kwafi rahoton faɗuwa.
feedback-deleted-all = An share rahotannin faɗuwa.
feedback-read-failed = Ba a iya karanta rahoton faɗuwa ba: { $error }
feedback-delete-failed = Ba a iya share rahoton faɗuwa ba: { $error }
feedback-delete-all-failed = Ba a iya share rahotannin faɗuwa ba: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _Fayil
desktop-menu-new-message = _Sabon saƙo
desktop-menu-quit = _Fita
desktop-menu-edit = _Gyara
desktop-menu-undo = _Janye
desktop-menu-select-all = Zaɓi _duka
desktop-menu-select-none = Kada a zaɓi _komai
desktop-menu-find = _Nemo…
desktop-menu-view = _Duba
desktop-menu-folder-list = Nuna jerin _folda
desktop-menu-refresh = Sa_bunta
desktop-menu-go = _Je zuwa
desktop-menu-inbox = _Akwatin saƙo
desktop-menu-starred = Masu _tauraro
desktop-menu-sent = _Waɗanda aka aika
desktop-menu-drafts = _Zayyanai
desktop-menu-all-mail = D_uk wasiƙu
desktop-menu-next = Tattaunawa ta _gaba
desktop-menu-previous = Tattaunawa ta _baya
desktop-menu-message = _Saƙo
desktop-menu-open = _Buɗe
desktop-menu-reply = _Amsa
desktop-menu-reply-all = Amsa wa _kowa
desktop-menu-forward = _Tura
desktop-menu-archive = Adana a _ma'ajiya
desktop-menu-delete = _Share
desktop-menu-spam = Rahoto saƙon _banza
desktop-menu-move-to = _Matsar zuwa…
desktop-menu-mark-read = Yi alama an _karanta
desktop-menu-mark-unread = Yi alama ba a karanta _ba
desktop-menu-star = Saka _tauraro
desktop-menu-important = Yi alama _muhimmi
desktop-menu-not-important = Yi alama ba muhimmi _ba
desktop-menu-settings = _Saituna
desktop-menu-quick-settings = Saituna masu _sauri
desktop-menu-configure = _Saita Katna Mail…
desktop-menu-help = _Taimako
desktop-menu-shortcuts = Gajerun hanyoyin _madannai
desktop-menu-whats-new = _Me ke sabo
desktop-menu-about = _Game da Katna
## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Zagayawa
shortcut-group-actions = Ayyuka
shortcut-group-go-to = Je zuwa
shortcut-group-app = Manhaja

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Tattaunawa ta gaba
shortcut-previous = Tattaunawa ta baya
shortcut-down = Sauka ƙasa a jerin
shortcut-up = Hau sama a jerin
shortcut-first = Na farko a jerin
shortcut-last = Na ƙarshe a jerin
shortcut-page-down = Shafi ƙasa a jerin
shortcut-page-up = Shafi sama a jerin
shortcut-open = Buɗe tattaunawa
shortcut-back = Koma jerin
shortcut-scroll-down = Gungura ƙasa
shortcut-scroll-up = Gungura sama
shortcut-scroll-page-down = Gungura shafi ɗaya ƙasa
shortcut-scroll-page-up = Gungura shafi ɗaya sama
shortcut-compose = Rubuta
shortcut-reply = Amsa
shortcut-reply-all = Amsa wa kowa
shortcut-forward = Tura
shortcut-archive = Adana a ma'ajiya
shortcut-delete = Share
shortcut-spam = Rahoto saƙon banza
shortcut-move-to = Matsar zuwa
shortcut-mark-read = Yi alama an karanta
shortcut-mark-unread = Yi alama ba a karanta ba
shortcut-star = Saka ko cire tauraro
shortcut-important = Yi alama muhimmi
shortcut-not-important = Yi alama ba muhimmi ba
shortcut-check = Yi wa tattaunawa alamar zaɓi
shortcut-select-all = Zaɓi dukkan tattaunawa
shortcut-select-none = Cire zaɓin dukkan tattaunawa
shortcut-undo = Janye aiki na ƙarshe
shortcut-go-inbox = Akwatin saƙo
shortcut-go-starred = Masu tauraro
shortcut-go-sent = Waɗanda aka aika
shortcut-go-drafts = Zayyanai
shortcut-go-all = Duk wasiƙu
shortcut-search = Bincika wasiƙu
shortcut-navigation = Nuna ko naɗe menu
shortcut-quick-settings = Saituna masu sauri
shortcut-settings = Duk saituna
shortcut-shortcuts = Gajerun hanyoyin madannai
shortcut-reload = Duba sababbin wasiƙu
shortcut-quit = Fita

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } sannan { $second }

## Settings > Accounts

accounts-folder-pane = Wurin folda
accounts-folder-pane-detail = Foldar waɗanne asusu ne wurin da ke hagu yake nunawa.
accounts-shown-one = Asusu ɗaya a lokaci guda; sauya a katin asusu
accounts-shown-all = Dukkan asusu, ɗaya bayan ɗaya
accounts-row = Asusu
accounts-row-detail = Cire asusu yana share kwafin wasiƙunsa na Katna a wannan kwamfuta. Wasiƙun suna zama a sabar.
accounts-none = Babu asusu tukuna.
accounts-kind-imported = An shigo da shi
accounts-picture-reset = Yi amfani da hoton tebur
accounts-picture-change = Canza hoto
accounts-remove = Cire
accounts-delete-all-row = Share duk bayanai
accounts-delete-all-row-detail = Fara daga farko, kamar sabon shigarwa.
accounts-delete-all-about = Yana share kowane asusu, duk wasiƙun da aka ajiye, lambobin sadarwa da kalandoji, fihirisar bincike, saitunanku da kalmomin sirri da aka ajiye daga wannan kwamfuta. Babu abin da ke canzawa a sabobin wasiƙunku.
accounts-delete-all-open = Share duk bayanan Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = An cire { $address } daga Katna.
accounts-removed = An cire { $address } daga Katna. Wasiƙunsa suna nan a sabar.
accounts-all-deleted = An share duk bayanan Katna daga wannan kwamfuta.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Cire { $address }?
accounts-remove-confirm = Cire asusu
accounts-removing = Ana cirewa…
accounts-remove-local-mail = { $folders ->
    [0] Duk wasiƙun da aka shigo da su cikin wannan asusu
    [one] Duk wasiƙun da aka shigo da su cikin wannan asusu a foldarsa
   *[other] Duk wasiƙun da aka shigo da su cikin wannan asusu a foldoji { $folders } nasa
}
accounts-remove-local-settings = Saitunansa na Katna
accounts-remove-mail = { $folders ->
    [0] Duk wasiƙun wannan asusu da Katna ta ajiye
    [one] Duk wasiƙun wannan asusu da Katna ta ajiye a foldarsa
   *[other] Duk wasiƙun wannan asusu da Katna ta ajiye a foldoji { $folders } nasa
}
accounts-remove-outbox = Saƙonninsa da ke jira a akwatin fita
accounts-remove-settings = Kalmar sirrinsa da aka ajiye da saitunansa na Katna
accounts-delete-all-title = Share duk bayanan Katna?
accounts-delete-all-confirm = Share komai
accounts-deleting = Ana sharewa…
accounts-delete-all-accounts = Kowane asusu, da duk wasiƙu da abubuwan haɗawa da Katna ta ajiye
accounts-delete-all-contacts = Lambobin sadarwa, kalandoji da fihirisar bincike
accounts-delete-all-settings = Duk saituna, sa hannu da gajerun hanyoyin madannai
accounts-delete-all-passwords = Kowace kalmar sirri da aka ajiye
accounts-deleted-heading = Za a share daga wannan kwamfuta:
accounts-cannot-undo = Ba za a iya janye wannan ba.
accounts-server-delete-all = Babu abin da ke canzawa a sabobin wasiƙunku: wasiƙunku suna zama a can, kuma sake ƙara asusu zai sake sauke su. Wasiƙun da aka shigo da su daga fayiloli suna cikin Katna kaɗai; ba a taɓa fayilolin ba.
accounts-server-local = An shigo da waɗannan wasiƙu daga fayiloli, don haka Katna ce kaɗai ke da kwafinsu. Ba a taɓa fayilolin da suka fito daga ciki ba; sake shigo da su don dawo da su.
accounts-server-remove = Babu abin da ke canzawa a sabar wasiƙu: wasiƙunku suna zama a can, kuma sake ƙara asusun zai sake sauke su.
accounts-confirm-word = share
accounts-confirm-placeholder = Rubuta “{ accounts-confirm-word }”
accounts-confirm-prompt = Don tabbatarwa, rubuta “{ accounts-confirm-word }”:
accounts-cancel = Soke
