# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Wika: { $language }
language-tooltip-system = Wika: { $language }, sinusunod ang system
language-search = Maghanap ng wika
language-system-default = Default ng system
language-system-now = Kasalukuyang { $language }
language-no-match = Walang wikang tumutugma sa “{ $query }”
language-machine = Isinalin ng makina. Tumulong na pahusayin ito
language-setting = Wika
language-setting-detail = Ang wika ng mga menu, button at mensahe, at ang format ng mga petsa at numero. Sinusunod ng default ng system ang desktop.

## Dates and sizes

ago-just-now = ngayon lang
ago-minutes = { $count ->
    [one] { $count } minuto ang nakalipas
   *[other] { $count } minuto ang nakalipas
}
ago-hours = { $count ->
    [one] { $count } oras ang nakalipas
   *[other] { $count } oras ang nakalipas
}
ago-days = { $count ->
    [one] { $count } araw ang nakalipas
   *[other] { $count } araw ang nakalipas
}
size-bytes = { $count ->
    [one] { $count } byte
   *[other] { $count } byte
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = Itago ang mga folder
folders-show = Ipakita ang mga folder
compose = Mag-compose
search = Maghanap
search-mail = Maghanap sa mail
search-settings = Maghanap sa mga setting
search-clear = I-clear ang paghahanap
search-options-show = Ipakita ang mga opsyon sa paghahanap
settings = Mga setting
account-add = Magdagdag ng account

## App rail (and the bottom bar on a phone)

rail-mail = Mail
rail-calendar = Kalendaryo
rail-contacts = Mga Contact
rail-tasks = Mga Gawain
rail-notes = Mga Tala
rail-feeds = Mga Feed

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Malapit na
app-calendar-promise = Ang iyong mga CalDAV na kalendaryo, mga imbitasyon sa pulong mula sa iyong mail at mga paalala, katabi ng iyong inbox.
app-tasks-promise = Mga listahan ng gagawin na naka-sync sa CalDAV, at mga gawaing ginawa mula sa mail.
app-notes-promise = Mabibilis na tala, at mga tala sa isang mail o pag-uusap para sa ibang pagkakataon.
app-feeds-promise = Magbasa ng mga RSS at Atom feed katabi ng iyong mail.

## Contacts page

app-contacts-loading = Kinukuha ang mga tao mula sa iyong mail…
app-contacts-empty = Lalabas dito ang mga taong kasulatan mo.
app-contacts-count = { $count ->
    [one] { $count } tao mula sa iyong mail, nauuna ang pinakamadalas mong kasulatan
   *[other] { $count } tao mula sa iyong mail, nauuna ang pinakamadalas mong kasulatan
}
app-contacts-top = { $count ->
    [one] Ang nangungunang { $count } tao mula sa iyong mail, nauuna ang pinakamadalas mong kasulatan
   *[other] Ang nangungunang { $count } tao mula sa iyong mail, nauuna ang pinakamadalas mong kasulatan
}
app-contacts-messages = { $count ->
    [one] { $count } mensahe
   *[other] { $count } mensahe
}
app-contacts-last = huli noong { $date }

## Navigation (the folders pane)

nav-labels = Mga Label
nav-folders = Mga Folder
nav-label-new = Gumawa ng bagong label
nav-folder-new = Gumawa ng bagong folder
nav-account-unnamed = Account { $number }
nav-tab-new = { $count ->
    [one] { $count } bago
   *[other] { $count } bago
}

## Special folders (the user's own folders keep their names)

folder-inbox = Inbox
folder-starred = Naka-star
folder-drafts = Mga Draft
folder-sent = Naipadala
folder-archive = Archive
folder-spam = Spam
folder-trash = Basurahan
folder-all-mail = Lahat ng Mail
folder-scheduled = Naka-iskedyul

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Bagong label
label-folder-new-title = Bagong folder
label-prompt = Maglagay ng pangalan ng bagong label:
label-folder-prompt = Maglagay ng pangalan ng bagong folder:
label-name-hint = Pangalan ng label
label-folder-name-hint = Pangalan ng folder
label-nest = Ilagay ang label sa ilalim ng:
label-folder-nest = Ilagay ang folder sa ilalim ng:
label-cancel = Kanselahin
label-create = Gumawa
label-creating = Ginagawa…
label-created = Nagawa ang label na “{ $name }”.
label-folder-created = Nagawa ang folder na “{ $name }”.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Pangunahin
tab-promotions = Mga Promosyon
tab-social = Social
tab-updates = Mga Update
tab-forums = Mga Forum
tab-focused = Naka-focus
tab-other = Iba pa
tab-inbox = Inbox
tab-newsletters = Mga Newsletter
tab-notifications = Mga Notification
tab-new = { $count } bago
tab-provider-other = inayos ng Katna

## Mail list: toolbar

list-select = Piliin
list-refresh = I-refresh
list-more = Higit pa
list-mark-read = Markahan bilang nabasa na
list-mark-unread = Markahan bilang hindi pa nabasa
list-move-to = Ilipat sa
list-archive = I-archive
list-spam = Iulat bilang spam
list-delete = I-delete
list-newer = Mas bago
list-older = Mas luma
list-range = { $first }–{ $last } ng { $total }
list-range-about = { $first }–{ $last } ng humigit-kumulang { $total }
list-results = Mga resulta para sa “{ $query }”
list-results-corrected = Ipinapakita ang mga resulta para sa “{ $query }”
list-search-instead = Hanapin na lang ang “{ $query }”
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Lahat
list-pick-none = Wala
list-pick-read = Nabasa na
list-pick-unread = Hindi pa nabasa
list-pick-starred = Naka-star
list-pick-unstarred = Walang star

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] Napili ang lahat ng { $count } pag-uusap.
       *[other] Napili ang lahat ng { $count } pag-uusap.
    }
   *[message] { $count ->
        [one] Napili ang lahat ng { $count } mensahe.
       *[other] Napili ang lahat ng { $count } mensahe.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] Napili ang lahat ng { $count } pag-uusap sa { $folder }.
       *[other] Napili ang lahat ng { $count } pag-uusap sa { $folder }.
    }
   *[message] { $count ->
        [one] Napili ang lahat ng { $count } mensahe sa { $folder }.
       *[other] Napili ang lahat ng { $count } mensahe sa { $folder }.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] Napili ang lahat ng { $count } pag-uusap sa screen.
       *[other] Napili ang lahat ng { $count } pag-uusap sa screen.
    }
   *[message] { $count ->
        [one] Napili ang lahat ng { $count } mensahe sa screen.
       *[other] Napili ang lahat ng { $count } mensahe sa screen.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Piliin ang lahat ng { $count } pag-uusap
       *[other] Piliin ang lahat ng { $count } pag-uusap
    }
   *[message] { $count ->
        [one] Piliin ang lahat ng { $count } mensahe
       *[other] Piliin ang lahat ng { $count } mensahe
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Piliin ang lahat ng { $count } pag-uusap sa { $folder }
       *[other] Piliin ang lahat ng { $count } pag-uusap sa { $folder }
    }
   *[message] { $count ->
        [one] Piliin ang lahat ng { $count } mensahe sa { $folder }
       *[other] Piliin ang lahat ng { $count } mensahe sa { $folder }
    }
}
list-clear-selection = I-clear ang pagpili

## Mail list: empty states

list-empty-search = Walang mensaheng tumugma sa iyong paghahanap.
list-empty-tab = Walang mail sa { $tab }.
list-empty-tab-unknown = Walang mail sa tab na ito.
list-empty-folder = Walang mensahe sa { $folder }.
list-empty-folder-unknown = Walang mensahe sa folder na ito.
list-first-sync = Kinukuha ang iyong mail…
list-first-sync-detail = Lalabas ito dito habang dumarating.

## Mail list: lines

row-removed = Inalis ang mensaheng ito.
row-starred = Naka-star
row-not-starred = Walang star
row-important = Mahalaga. I-click para markahan bilang hindi mahalaga.
row-mark-important = Markahan bilang mahalaga
row-pinned = Naka-pin sa itaas
row-pin = I-pin sa itaas
row-unpin = I-unpin

## Mail list: More menu and right-click menu

menu-reply = Sumagot
menu-reply-all = Sumagot sa lahat
menu-forward = Ipasa
menu-archive = I-archive
menu-delete = I-delete
menu-spam = Iulat bilang spam
menu-mark-read = Markahan bilang nabasa na
menu-mark-unread = Markahan bilang hindi pa nabasa
menu-mark-all-read = Markahan lahat bilang nabasa na
menu-star = Magdagdag ng star
menu-unstar = Alisin ang star
menu-important = Markahan bilang mahalaga
menu-not-important = Markahan bilang hindi mahalaga
menu-pin = I-pin sa itaas
menu-unpin = I-unpin
menu-print-all = I-print lahat
menu-new-window = Buksan sa bagong window
menu-move-to = Ilipat sa
menu-move-to-heading = Ilipat sa:
menu-find-from = Hanapin ang mga email mula kay { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Na-archive ang { $count } pag-uusap.
       *[other] Na-archive ang { $count } pag-uusap.
    }
   *[message] { $count ->
        [one] Na-archive ang { $count } mensahe.
       *[other] Na-archive ang { $count } mensahe.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Inilipat sa Basurahan ang { $count } pag-uusap.
       *[other] Inilipat sa Basurahan ang { $count } pag-uusap.
    }
   *[message] { $count ->
        [one] Inilipat sa Basurahan ang { $count } mensahe.
       *[other] Inilipat sa Basurahan ang { $count } mensahe.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Nailipat ang { $count } pag-uusap.
       *[other] Nailipat ang { $count } pag-uusap.
    }
   *[message] { $count ->
        [one] Nailipat ang { $count } mensahe.
       *[other] Nailipat ang { $count } mensahe.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Nilagyan ng star ang { $count } pag-uusap.
       *[other] Nilagyan ng star ang { $count } pag-uusap.
    }
   *[message] { $count ->
        [one] Nilagyan ng star ang { $count } mensahe.
       *[other] Nilagyan ng star ang { $count } mensahe.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Inalis ang star sa { $count } pag-uusap.
       *[other] Inalis ang star sa { $count } pag-uusap.
    }
   *[message] { $count ->
        [one] Inalis ang star sa { $count } mensahe.
       *[other] Inalis ang star sa { $count } mensahe.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Minarkahang mahalaga ang { $count } pag-uusap.
       *[other] Minarkahang mahalaga ang { $count } pag-uusap.
    }
   *[message] { $count ->
        [one] Minarkahang mahalaga ang { $count } mensahe.
       *[other] Minarkahang mahalaga ang { $count } mensahe.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Minarkahang hindi mahalaga ang { $count } pag-uusap.
       *[other] Minarkahang hindi mahalaga ang { $count } pag-uusap.
    }
   *[message] { $count ->
        [one] Minarkahang hindi mahalaga ang { $count } mensahe.
       *[other] Minarkahang hindi mahalaga ang { $count } mensahe.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Na-pin sa itaas ang { $count } pag-uusap.
       *[other] Na-pin sa itaas ang { $count } pag-uusap.
    }
   *[message] { $count ->
        [one] Na-pin sa itaas ang { $count } mensahe.
       *[other] Na-pin sa itaas ang { $count } mensahe.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Na-unpin ang { $count } pag-uusap.
       *[other] Na-unpin ang { $count } pag-uusap.
    }
   *[message] { $count ->
        [one] Na-unpin ang { $count } mensahe.
       *[other] Na-unpin ang { $count } mensahe.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Iniulat bilang spam ang { $count } pag-uusap.
       *[other] Iniulat bilang spam ang { $count } pag-uusap.
    }
   *[message] { $count ->
        [one] Iniulat bilang spam ang { $count } mensahe.
       *[other] Iniulat bilang spam ang { $count } mensahe.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Permanenteng na-delete ang { $count } pag-uusap.
       *[other] Permanenteng na-delete ang { $count } pag-uusap.
    }
   *[message] { $count ->
        [one] Permanenteng na-delete ang { $count } mensahe.
       *[other] Permanenteng na-delete ang { $count } mensahe.
    }
}
toast-undone = Na-undo ang aksyon.
toast-undo = I-undo
toast-no-spam-folder = Walang spam folder ang account na ito.

## Reading pane: toolbar

reader-close = Isara
reader-back = Bumalik
reader-mark-unread = Markahan bilang hindi pa nabasa
reader-move-to = Ilipat sa
reader-more = Higit pa
reader-print-all = I-print lahat
reader-new-window = Sa bagong window
reader-position = { $position } ng { $total }
reader-newer = Mas bago
reader-older = Mas luma

## Reading pane: the conversation

reader-removed = Inalis ang pag-uusap na ito.
reader-no-subject = (walang subject)
reader-collapse-all = I-collapse lahat
reader-expand-all = I-expand lahat
reader-unknown-sender = (hindi kilalang nagpadala)
reader-date-ago = { $date } ({ $ago })
reader-me = ako
reader-to = para kay { $names }
reader-starred = Naka-star
reader-not-starred = Walang star
reader-too-long = Masyadong mahaba ang mensahe para maipakita nang buo.
reader-encrypted-images = Hindi kailanman nilo-load ang mga larawan mula sa web sa naka-encrypt na mail.
reader-window-failed = Hindi mabuksan ang bagong window.

## Reading pane: message details (opened from "to me")

reader-details-from = mula kay:
reader-details-to = para kay:
reader-details-cc = cc:
reader-details-date = petsa:
reader-details-subject = subject:

## Reading pane: downloading a message

reader-downloading = Dina-download ang mensaheng ito mula sa server…
reader-download-failed = Hindi ma-download ang mensaheng ito.
reader-try-again = Subukang muli

## Reply row

reply-reply = Sumagot
reply-reply-all = Sumagot sa lahat
reply-forward = Ipasa

## Encrypted and signed mail

security-decrypting = Dine-decrypt…
security-checking = Sinusuri ang lagda…
security-partly-encrypted = Bahagi lang ng mensaheng ito ang naka-encrypt. Ang natitira ay idinagdag sa labas ng proteksyon at maaaring galing kahit kanino.
security-partly-signed = Bahagi lang ng mensaheng ito ang may lagda. Ang natitira ay idinagdag sa labas ng proteksyon at maaaring galing kahit kanino.
security-encrypted = Naka-encrypt na mensahe
security-encrypted-smime = Naka-encrypt na mensahe (S/MIME)
security-no-key = Hindi ma-decrypt ang mensaheng ito: na-encrypt ito para sa key na wala sa iyo.
security-cancelled = Kinansela ang pag-decrypt.
security-damaged = Hindi ma-decrypt ang mensaheng ito: sira o binago ang naka-encrypt na data.
security-decrypt-unavailable = Hindi ma-decrypt ang mensaheng ito: i-install ang { $tool } para mabasa ang naka-encrypt na mail.
security-decrypt-failed = Hindi ma-decrypt ang mensaheng ito: { $reason }
security-unknown-signer = isang hindi kilalang lumagda
security-signed-verified = Nilagdaan ni { $signer } · na-verify
security-signed-not-sender = Nilagdaan ni { $signer }, na hindi ang nagpadala
security-signed-untrusted = Nilagdaan ni { $signer }, gamit ang key na minarkahan mong hindi pinagkakatiwalaan
security-signed-unverified = Nilagdaan ni { $signer } · hindi pa na-verify ang key
security-bad-signature = Hindi wastong lagda: binago ang mensaheng ito matapos itong lagdaan, o peke ang lagda.
security-signature-expired = Nilagdaan ni { $signer } · nag-expire na ang lagda
security-key-expired = Nilagdaan ni { $signer } · nag-expire na ang key mula noon
security-key-revoked = Nilagdaan ni { $signer } gamit ang key na binawi na
security-missing-key = Nilagdaan gamit ang key na wala sa iyo, kaya hindi ito masuri
security-missing-key-id = Nilagdaan gamit ang key na wala sa iyo ({ $key }), kaya hindi ito masuri
security-signature-unavailable = May lagda; i-install ang { $tool } para masuri ang lagda
security-signature-error = Hindi masuri ang lagda.

## Remote images and pictures

remote-hidden = Nakatago ang mga larawan sa mensaheng ito.
remote-show = Ipakita ang mga larawan
remote-always-show = Palaging ipakita mula sa nagpadalang ito
remote-picture-use = Gamitin
remote-picture-too-big = Pumili ng larawang 8 MB o mas maliit.
remote-picture-type = Pumili ng larawang PNG, JPEG, GIF, WebP o SVG.
remote-picture-read-failed = Hindi mabasa ang larawan: { $error }
remote-picture-keep-failed = Hindi maitabi ang larawan: { $error }
remote-picture-remove-failed = Hindi maalis ang larawan: { $error }

## Attachments

attachment-count = { $count ->
    [one] { $count } attachment
   *[other] { $count } attachment
}
attachment-save = I-save
attachment-save-all = I-save lahat
attachment-save-all-tooltip = I-save ang bawat attachment sa isang folder
attachment-save-here = I-save dito
attachment-not-downloaded = Hindi na-download ang mensaheng ito.
attachment-not-found = Hindi makita ang attachment na ito sa mensahe.
attachment-read-failed = Hindi mabasa ang { $name }
attachment-numbered = attachment { $number }
attachment-saved-all = { $count ->
    [one] Na-save ang { $count } file sa { $place }
   *[other] Na-save ang { $count } file sa { $place }
}
attachment-saved-some = { $total ->
    [one] Na-save ang { $saved } sa { $total } file sa { $place }. Hindi ma-save ang { $failed }
   *[other] Na-save ang { $saved } sa { $total } file sa { $place }. Hindi ma-save ang { $failed }
}
attachment-saved-to = Na-save sa { $path }
attachment-save-failed = Hindi ma-save ang { $name }: { $error }
attachment-open-failed = Hindi mabuksan ang { $name }: { $error }
attachment-risky = Maaaring magpatakbo ng program ang file na ito, kaya hindi ito binubuksan ng Katna. I-save na lang ito.
attachment-encrypted-open = Dumating nang naka-encrypt ang file na ito. I-save ito para mabuksan sa ibang lugar.

## Printing

print-failed = Hindi makapag-print: { $error }
print-no-font = walang nakitang font
print-opened-as-pdf = Binuksan bilang PDF para i-print mula roon.
print-not-downloaded = (Hindi pa na-download.)
print-encrypted = (Naka-encrypt. Buksan ito sa Katna Mail para i-print ang text nito.)
print-to = Para kay: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Buksan ang mensaheng ito para mabasa ang mga attachment nito.
text-copy = Kopyahin
text-select-all = Piliin lahat

## Settings page: its tabs

settings-tab-general = Pangkalahatan
settings-tab-inbox = Inbox
settings-tab-accounts = Mga Account
settings-tab-subscriptions = Mga Subscription
settings-tab-appearance = Hitsura
settings-tab-shortcuts = Mga Shortcut
settings-tab-default-apps = Mga default na app
settings-tab-folders-rules = Mga folder at panuntunan
settings-tab-compose = Mag-compose
settings-tab-mcp-server = MCP server
settings-tab-feedback = Feedback ng user
settings-tab-experimental = Pang-eksperimento

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Tingnan ang mga newsletter at mailing list na natatanggap mo, at mag-unsubscribe sa isang click.
settings-tab-folders-rules-coming = Gumawa, mag-rename, maglipat at magtago ng mga folder at label, at piliin kung alin ang magsi-sync. Kusang inaayos, nilalagyan ng label, ipinapasa o dine-delete ng mga panuntunan ang bagong mail, ayon sa nagpadala, subject o mga salita.
settings-tab-mcp-server-coming = Payagan ang mga AI assistant sa computer na ito na maghanap, magbasa at mag-draft ng iyong mail, nang may pahintulot mo.

## Settings > General

settings-general-conversations = View ng pag-uusap
settings-general-conversations-group = Pagsamahin ang mga sagot sa iisang mail
settings-general-conversations-group-detail = Isang linya bawat pag-uusap sa listahan
settings-general-reading = Pagbabasa
settings-general-newest-first = Pinakabagong mensahe muna
settings-general-newest-first-detail = Nagsisimula ang pag-uusap sa pinakahuling sagot nito
settings-general-full-headers = Ipakita ang buong header
settings-general-full-headers-detail = Nakabukas ang mula kay, para kay, cc, petsa at subject sa bawat mensahe
settings-general-full-names = Buong pangalan ng mga tatanggap
settings-general-full-names-detail = “para sa akin, Ada Lovelace” sa halip na “para sa akin, Ada”
settings-general-mark-read = Markahan bilang nabasa na
settings-general-mark-read-now = Sa sandaling mabuksan ito
settings-general-mark-read-1s = Pagkatapos itong mabuksan nang 1 segundo
settings-general-mark-read-3s = Pagkatapos itong mabuksan nang 3 segundo
settings-general-mark-read-never = Kapag minarkahan ko lang itong nabasa na
settings-general-reply-button = Button na Sumagot
settings-general-reply-all = Sumagot sa lahat
settings-general-reply-all-detail = Sumasagot sa lahat ang button na sumagot sa tabi ng bawat mensahe, hindi lang sa nagpadala
settings-general-remote-images = Mga larawan mula sa web
settings-general-remote-images-detail = Kapag nilo-load ang mga larawan ng isang mensahe, nalalaman ng nagpadala na binuksan mo ito, kailan, at halos kung saan. Kapag naka-off, nagtatanong muna ang bawat mensahe, at palagi mong maipapakita ang mga larawan ng isang nagpadala.
settings-general-remote-images-always = Palaging ipakita ang mga larawan
settings-general-remote-images-always-detail = Sa bawat mensahe, hindi lang mula sa mga nagpadalang pinagkakatiwalaan mo
settings-general-sending = Pagpapadala
settings-general-sending-detail = Gaano katagal naghihintay ang naipadalang mensahe, para mabawi pa ito.
settings-general-offline = Offline na mail
settings-general-offline-detail = Buong dina-download ang kamakailang mail, para mabasa nang walang koneksyon. Dina-download ang mas lumang mail kapag binuksan mo ito.
settings-general-offline-days = { $count ->
    [one] { $count } araw
   *[other] { $count } araw
}
settings-general-offline-years = { $count ->
    [one] { $count } taon
   *[other] { $count } taon
}
settings-general-offline-all = Lahat ng mail
settings-general-offline-note = Kapag pumili ng mas kaunting araw, mananatili ang mail na na-download na. Walang nagbabago sa server.
settings-general-notifications = Mga Notification
settings-general-notifications-detail = Para sa bagong mail sa Inbox, kahit nakasara ang Katna Mail.
settings-general-new-mail = Abisuhan ako tungkol sa bagong mail
settings-general-new-mail-detail = May Sumagot sa lahat, Markahan bilang nabasa na at I-archive
settings-general-new-mail-sound = Magpatugtog ng tunog
settings-general-new-mail-sound-detail = Ang tunog ng bagong mail ng desktop
settings-general-desktop = Desktop
settings-general-open-at-login = Buksan ang Katna Mail sa pag-log in
settings-general-open-at-login-detail = Nagsi-sync pa rin ang mail sa pag-log in, habang tumatakbo ang serbisyo
settings-general-tray = Ipakita ang Katna sa system tray
settings-general-tray-detail = May bilang ng hindi pa nabasa at isang menu
settings-general-unread-badge = Bilang ng hindi pa nabasa sa icon sa taskbar
settings-general-unread-badge-detail = Ilang mensahe sa Inbox ang hindi pa nabasa

## Settings > Inbox

settings-inbox-tabs = Mga tab ng inbox
settings-inbox-tabs-detail = Ayusin ang inbox sa mga tab, gaya ng ginagawa ng website ng iyong mail provider.
settings-inbox-tabs-show = Ipakita ang mga tab ng inbox
settings-inbox-tabs-show-detail = Kapag naka-off, iisang listahan ang ipinapakita para sa bawat account
settings-inbox-no-accounts = Magdagdag ng account para mapili ang mga tab nito.
settings-inbox-tabs-automatic = Awtomatiko: { $tabs } ({ $provider })
settings-inbox-tabs-off = Walang tab
settings-inbox-tabs-gmail = Pangunahin, Mga Promosyon, Social, Mga Update, Mga Forum
settings-inbox-tabs-focused = Naka-focus at Iba pa
settings-inbox-tabs-zoho = Inbox, Mga Newsletter at Mga Notification
settings-inbox-tabs-shown = Mga tab na ipinapakita. Mananatili sa { $tab } ang mail ng tab na io-off mo.

## Settings > Appearance

settings-appearance-reading-pane = Pane ng pagbabasa
settings-appearance-reading-pane-detail = Kung saan ipinapakita ang nakabukas na pag-uusap.
settings-appearance-pane-right = Sa kanan ng listahan
settings-appearance-pane-none = Walang hati
settings-appearance-density = Density
settings-appearance-density-default = Default
settings-appearance-density-compact = Compact
settings-appearance-scaling = Scaling
settings-appearance-scaling-detail = Pinalalaki o pinaliliit ang lahat ng nasa Katna Mail, dagdag sa sariling scale ng desktop: text, mga icon, espasyo at mga divider. Pinapanatili ng mail na ipinapadala mo ang sarili nitong laki ng font. Maaaring mahirap i-click ang mga icon sa napakaliliit na laki.
settings-appearance-theme = Tema
settings-appearance-theme-system = Kapareho ng desktop
settings-appearance-theme-light = Maliwanag
settings-appearance-theme-dark = Madilim
settings-appearance-desktop-colors = Mga kulay ng desktop
settings-appearance-desktop-colors-use = Gamitin ang mga kulay ng desktop
settings-appearance-desktop-colors-use-detail = Ang color scheme at accent color ng desktop
settings-appearance-app-names = Mga pangalan ng app
settings-appearance-app-names-show = Ipakita ang mga pangalan ng app
settings-appearance-app-names-show-detail = Mga pangalan sa ilalim ng mga icon ng app sa dulong kaliwa
settings-appearance-sender-pictures = Mga larawan ng nagpadala
settings-appearance-sender-pictures-show = Ipakita ang mga logo ng kumpanya
settings-appearance-sender-pictures-show-detail = Hinahanap ayon sa domain ng nagpadala, hindi kailanman ayon sa mensahe, at itinatabi nang isang linggo
settings-appearance-important = Mga marker ng Mahalaga
settings-appearance-important-show = Ipakita ang mga marker ng Mahalaga
settings-appearance-important-show-detail = Sa tabi ng bawat mensahe sa listahan
settings-appearance-message-width = Lapad ng mensahe
settings-appearance-message-width-limit = Limitahan ang lapad ng mga mensahe
settings-appearance-message-width-limit-detail = Mas madaling basahin ang mahahabang linya sa malapad na window
settings-appearance-mail-colors = Mga kulay ng mail
settings-appearance-mail-colors-detail = Idinisenyo ang karamihan ng mail para sa puting pahina. Sa madilim na tema, pinapalitan ang mga kulay nito ng madidilim na kulay na madaling basahin; kapag naka-off, pinapanatili nito ang mga kulay ng nagpadala sa maliwanag na pahina.
settings-appearance-dark-mail = Madidilim na kulay rin para sa mail
settings-appearance-dark-mail-detail = Habang madilim lang ang tema
settings-appearance-attachment-previews = Mga preview ng attachment
settings-appearance-attachment-previews-show = Ipakita ang mga preview ng mga attachment
settings-appearance-attachment-previews-show-detail = Maliit na larawan ng nilalaman ng bawat file sa card nito

## Settings > Default apps

settings-default-apps-intro = Kung saan bumubukas ang mga attachment kapag kini-click mo ang mga ito. Palaging makakapagbukas din ang viewer ng file sa ibang app. Itinatakda ang mga default na app ng desktop sa sarili nitong mga setting.
settings-default-apps-pdf = Mga PDF file
settings-default-apps-pdf-detail = Mga pahina, may zoom.
settings-default-apps-pictures = Mga larawan
settings-default-apps-pictures-detail = Mga litrato (itinuwid), PNG, GIF, WebP, BMP, TIFF at SVG.
settings-default-apps-text = Mga text file
settings-default-apps-text-detail = Plain text, mga log, code at iba pang text.
settings-default-apps-sheets = Mga spreadsheet
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) at CSV.
settings-default-apps-documents = Mga dokumento
settings-default-apps-documents-detail = Word (docx) at OpenDocument text (odt).
settings-default-apps-katna = Viewer ng Katna Mail
settings-default-apps-system = Default na app ng desktop
settings-default-apps-ask = Itanong kung aling app sa bawat pagkakataon
settings-default-apps-after-saving = Pagkatapos mag-save
settings-default-apps-show-folder = Ipakita ang mga na-save na file sa folder nila
settings-default-apps-show-folder-detail = Binubuksan ang file manager na nakapili ang mga na-save na attachment

## Settings > Compose

settings-compose-send-from = Ipadala ang mga bagong mensahe mula sa
settings-compose-send-from-detail = Palaging ipinapadala ang mga sagot at pagpapasa mula sa account na kinaroroonan mo.
settings-compose-send-from-current = Ang account na kinaroroonan mo
settings-compose-send-on-replies = Ipadala sa mga sagot
settings-compose-send-on-replies-detail = Ang ginagawa ng Ipadala sa isang sagot o pagpapasa. Iniaalok ng menu sa tabi ng Ipadala ang isa pa.
settings-compose-send-plain = Ipadala
settings-compose-send-archive = Ipadala at i-archive
settings-compose-signatures = Mga lagda
settings-compose-signatures-detail = Idinaragdag sa ibaba ng iyong mensahe, pagkatapos ng linyang “--”. Pumili ng iba sa window ng pag-compose.
settings-compose-untitled = Walang pamagat
settings-compose-signature-name = Pangalan, gaya ng Trabaho
settings-compose-signature-first = Aking lagda
settings-compose-signature-numbered = Lagda { $number }
settings-compose-signature-delete = I-delete
settings-compose-signature-deleted = Na-delete ang lagda
settings-compose-signature-new = Gumawa ng bago
settings-compose-no-signatures = Wala pang lagda.
settings-compose-no-signature = Walang lagda
settings-compose-for-new-mail = Para sa bagong mail
settings-compose-for-replies = Para sa mga sagot at pagpapasa
settings-compose-for-replies-detail = Sa pag-uusap kung saan nilagdaan mo ang isang mensahe, sa lagdang iyon nagsisimula ang sagot.
settings-compose-format = Format
settings-compose-plain-text = Sumulat sa plain text
settings-compose-plain-text-detail = Nagsisimula ang bagong mail nang walang formatting; maaari itong palitan sa window ng pag-compose
settings-compose-spelling = Pagbaybay
settings-compose-spell-check = Suriin ang pagbaybay habang sumusulat ako
settings-compose-spell-check-detail = Sinasalungguhitan ang mga maling baybay na salita, may mga mungkahi sa right-click
settings-compose-spell-desktop = Wika ng desktop ({ $language })
settings-compose-templates = Mga template
settings-compose-templates-detail = I-save ang mail na madalas mong isinusulat, at magsimula ng bagong mail o sagot mula rito.

## Settings > Shortcuts

settings-shortcuts-set = Set ng shortcut
settings-shortcuts-set-detail = Magsimula sa mga key ng mail app na kilala mo. Ctrl ang Cmd dito. Nananatili ang sarili mong mga pagbabago sa ibabaw ng set, at ibinabalik ng Ibalik ang mga default ang mga key ng set.
settings-shortcuts-single = Mga shortcut na iisang key
settings-shortcuts-single-detail = Mga key na walang Ctrl o Alt, gaya sa webmail: nag-a-archive ang e, gumagalaw ang j at k, naghahanap ang /. Gumagana ang mga ito sa listahan at sa nakabukas na pag-uusap, hindi kailanman habang nagta-type.
settings-shortcuts-single-use = Gamitin ang mga shortcut na iisang key
settings-shortcuts-single-use-detail = Palaging gumagana ang mga Ctrl shortcut
settings-shortcuts-how = I-click ang isang key para palitan ito, o ang + para magdagdag, pagkatapos ay pindutin ang mga bagong key. Kinakansela ng Esc.
settings-shortcuts-restore = Ibalik ang mga default
settings-shortcuts-no-key = Walang key
settings-shortcuts-press = Pumindot ng mga key…
settings-shortcuts-then = { $keys } at pagkatapos…
settings-shortcuts-moved = Ginagawa na ngayon ng { $keys } ang “{ $action }” sa halip na “{ $previous }”.
settings-shortcuts-single-off = Naka-off ang mga shortcut na iisang key, kaya gagana ang key na ito kapag na-on na ang mga ito.
settings-shortcuts-restored = Nasa mga key na ulit ng set nito ang bawat shortcut.

## Settings search: the line under a result

settings-general-language-summary = Wika ng app, mga petsa at numero
settings-general-reading-summary = Pinakabagong mensahe muna, buong header, buong pangalan ng mga tatanggap
settings-general-mark-read-summary = Kailan minamarkahang nabasa na ang nakabukas na pag-uusap: kaagad, pagkatapos ng 1 o 3 segundo, o mano-mano
settings-general-reply-button-summary = Sumasagot sa lahat ang button na sumagot sa tabi ng bawat mensahe
settings-general-remote-images-summary = Palaging ipakita ang mga larawan ng bawat mensahe
settings-general-sending-summary = I-undo ang pagpapadala: gaano katagal naghihintay ang naipadalang mensahe, para mabawi pa ito
settings-general-offline-summary = Ilang araw ng kamakailang mail ang buong dina-download, para mabasa nang walang koneksyon
settings-general-notifications-summary = Mga notification ng bagong mail at ang tunog nito
settings-general-desktop-summary = Buksan ang Katna Mail sa pag-log in, ang icon sa system tray at ang bilang ng hindi pa nabasa sa icon sa taskbar
settings-accounts-accounts-summary = Magdagdag o mag-alis ng account, o palitan ang larawan nito
settings-appearance-density-summary = Default o compact na mga linya sa listahan
settings-appearance-scaling-summary = Palakihin o paliitin ang lahat: text, mga icon, espasyo at mga divider
settings-appearance-theme-summary = Kapareho ng desktop, maliwanag o madilim
settings-appearance-sender-pictures-summary = Mga logo ng kumpanya, hinahanap ayon sa domain ng nagpadala
settings-appearance-important-summary = Ang marker ng Mahalaga sa tabi ng bawat mensahe sa listahan
settings-appearance-mail-colors-summary = Madidilim na kulay para sa HTML mail sa madilim na tema, o ang mga kulay ng nagpadala nito
settings-appearance-attachment-previews-summary = Maliit na larawan ng nilalaman ng bawat attachment
settings-shortcuts-set-summary = Magsimula sa mga key ng Gmail, Inbox by Gmail, Apple Mail, Outlook o Thunderbird
settings-shortcuts-single-summary = Mga key na walang Ctrl o Alt, gaya sa webmail
settings-default-apps-pdf-summary = Kung saan bumubukas ang mga PDF attachment
settings-default-apps-pictures-summary = Kung saan bumubukas ang mga litrato at larawan
settings-default-apps-text-summary = Kung saan bumubukas ang plain text, mga log at code
settings-default-apps-sheets-summary = Kung saan bumubukas ang mga Excel, OpenDocument at CSV file
settings-default-apps-documents-summary = Kung saan bumubukas ang Word at OpenDocument text
settings-default-apps-after-saving-summary = Ipakita ang mga na-save na attachment sa folder nila
settings-compose-send-from-summary = Ang account na pinagpapadalhan ng bagong mail: ang kinaroroonan mo, o palaging iisa
settings-compose-send-on-replies-summary = Ipadala, o Ipadala at i-archive ang pag-uusap, sa mga sagot at pagpapasa
settings-compose-signatures-summary = Idinaragdag sa ibaba ng iyong mensahe, pagkatapos ng linyang “--”
settings-compose-for-new-mail-summary = Ang lagdang pinagsisimulan ng bagong mail
settings-compose-for-replies-summary = Ang lagdang pinagsisimulan ng mga sagot at pagpapasa
settings-compose-format-summary = Sumulat ng bagong mail sa plain text
settings-compose-spelling-summary = Suriin ang pagbaybay habang sumusulat, at ang wika ng diksyunaryo
settings-compose-templates-summary = Malapit na: i-save ang mail na madalas mong isinusulat, at magsimula ng bagong mail o sagot mula rito
settings-feedback-crash-reports-summary = Mag-save ng mga ulat ng pag-crash sa computer na ito kapag nag-crash ang Katna Mail o ang serbisyo nito sa background
settings-feedback-saved-summary = Tingnan, kopyahin o i-delete ang mga ulat ng pag-crash na naka-save sa computer na ito
settings-feedback-help-improve-summary = Magpadala ng mga ulat ng pag-crash para makatulong ayusin ang nagkaproblema; naka-off maliban kung i-on mo
settings-experimental-blur-summary = Tumatagos ang desktop sa itaas na bar, malabo, at parang frosted glass ang mga menu
settings-search-shortcut = Keyboard shortcut
settings-search-tab = Tab ng mga setting
settings-search-none = Walang setting na tumutugma sa “{ $query }”.
settings-search-results = Mga setting na tumutugma sa “{ $query }”

## Quick settings (the panel that slides in from the right)

quick-title = Mabilisang setting
quick-see-all = Tingnan lahat ng setting
quick-reading-pane = Pane ng pagbabasa
quick-pane-right = Sa kanan ng listahan
quick-pane-none = Walang hati
quick-density = Density
quick-density-default = Default
quick-density-compact = Compact
quick-theme = Tema
quick-theme-system = Kapareho ng desktop
quick-theme-light = Maliwanag
quick-theme-dark = Madilim
quick-desktop-colors = Mga kulay ng desktop
quick-desktop-colors-detail = Ang color scheme at accent color ng desktop
quick-app-names = Mga pangalan ng app
quick-app-names-detail = Mga pangalan sa ilalim ng mga icon ng app sa dulong kaliwa
quick-inbox-tabs = Mga tab ng inbox
quick-inbox-tabs-detail = Ang mga tab ng mail provider ng bawat account
quick-choose-tabs = Pumili ng mga tab
quick-choose-tabs-detail = Bawat account, sa Mga setting
quick-sending = Pagpapadala
quick-undo-send = I-undo ang pagpapadala
quick-undo-send-off = Naka-off
quick-undo-send-seconds = { $seconds } s
quick-signatures = Mga lagda
quick-signatures-none = Wala pa
quick-signatures-one = { $name }, ginagamit bilang default
quick-signatures-many = { $count ->
    [one] { $count } lagda; { $name } ang default
   *[other] { $count } lagda; { $name } ang default
}
quick-signatures-no-default = { $count ->
    [one] { $count }, walang default
   *[other] { $count }, walang default
}
quick-signature-untitled = Walang pamagat
quick-threading = Pag-thread ng email
quick-conversation-view = View ng pag-uusap
quick-conversation-view-detail = Pagsamahin ang mga sagot sa iisang mail
quick-help = Tulong
quick-tour = Mag-tour
quick-whats-new = Ano’ng bago
quick-about = Tungkol sa Katna

## Settings: opening at login

settings-open-at-login-failed = Hindi mabago ang pagbubukas sa pag-log in: { $error }

## Settings > Appearance > Scaling

scale-letter = A
scale-percent = { $percent }%
scale-reset = Ibalik sa { $percent }%

## Settings > Experimental > Look & Feel

look-intro = Mga feature na sinusubukan pa. Maaaring magbago o mawala ang mga ito.
look-heading = Hitsura at Dating
look-window-frame = Frame ng window
look-window-frame-detail = Kung sino ang gumuguhit ng title bar, mga button ng window, mga sulok at anino.
look-frame-native-kde = Native: ang frame ng KDE, sa iyong tema ng Plasma
look-frame-native = Native: ang frame ng desktop
look-frame-katna = Katna: nagiging title bar ang itaas na bar
look-frame-katna-note-named = Gumuguhit ang Katna ng mga bilugang sulok at sarili nitong anino. Hindi na sinusunod ng frame ang tema ng { $desktop }; umiiral pa rin ang mga panuntunan ng window.
look-frame-katna-note = Gumuguhit ang Katna ng mga bilugang sulok at sarili nitong anino. Hindi na sinusunod ng frame ang tema ng desktop; umiiral pa rin ang mga panuntunan ng window.
look-frame-client-side = Ipinauubaya ng iyong desktop ang frame sa bawat app, kaya sariling frame na ang iginuguhit ng Katna.
look-blurred-background = Malabong background
look-blurred-background-detail = Tumatagos ang desktop sa itaas na bar at sa mga folder, malabo, at parang frosted glass ang mga menu at popover.
look-blur = Palabuin ang nasa likod ng window
look-blur-detail = Nananatili ang mail sa mga solidong card, kaya malinaw pa rin ang text
look-blur-off-kde = Naka-off ang blur effect ng KDE. I-on ang Blur sa System Settings, Window Management, Desktop Effects, pagkatapos ay buksan muli ang Katna Mail.
look-blur-none-gnome = Hindi pinalalabo ng GNOME ang nasa likod ng mga window.
look-blur-none-x11 = Hindi pinalalabo ng iyong window manager ang nasa likod ng mga window.
look-blur-none-wayland = Hindi pinalalabo ng iyong compositor ang nasa likod ng mga window.

## Settings > User feedback (crash reports)

feedback-intro-sending = Ipinapadala ang mga bagong ulat ng pag-crash para makatulong ayusin ang nagkaproblema. Walang ibang lumalabas sa computer na ito.
feedback-intro-local = Walang ipinapadala ang Katna kahit saan. Nananatili sa computer na ito ang mga ulat ng pag-crash, para tingnan mo o i-attach sa isang ulat ng bug.
feedback-crash-reports = Mga ulat ng pag-crash
feedback-crash-reports-detail = Isinusulat kapag nag-crash ang Katna Mail o ang serbisyo nito sa background.
feedback-save = I-save ang mga ulat ng pag-crash sa computer na ito
feedback-save-detail = Hindi isinasama ang iyong home folder, mga pangalan ng user at computer, at mga email address
feedback-saved = Mga naka-save na ulat ng pag-crash
feedback-saved-detail = { $count ->
    [one] Itinatabi ang pinakabagong { $count }.
   *[other] Itinatabi ang pinakabagong { $count }.
}
feedback-help-improve = Tumulong na pahusayin ang Katna
feedback-help-improve-detail = Naka-off maliban kung i-on mo, at maaari mo itong i-off dito anumang oras.
feedback-send = Magpadala ng mga ulat ng pag-crash
feedback-send-detail = Ang naka-save na ulat, eksaktong gaya ng makikita mo rito, ay ipinapadala sa crash tracker ng Katna (Sentry, sa EU). Walang IP address, mensahe o email address
feedback-none-saved = Walang naka-save na ulat ng pag-crash.
feedback-delete-all = I-delete lahat
feedback-app-daemon = Serbisyo sa background
feedback-report-sent = { $date } · Naipadala
feedback-view = Tingnan
feedback-view-tooltip = Buksan ang ulat
feedback-copy-tooltip = Kopyahin para i-paste sa isang ulat ng bug
feedback-copied = Nakopya ang ulat ng pag-crash.
feedback-deleted-all = Na-delete ang mga ulat ng pag-crash.
feedback-read-failed = Hindi mabasa ang ulat ng pag-crash: { $error }
feedback-delete-failed = Hindi ma-delete ang ulat ng pag-crash: { $error }
feedback-delete-all-failed = Hindi ma-delete ang mga ulat ng pag-crash: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _File
desktop-menu-new-message = _Bagong Mensahe
desktop-menu-quit = _Umalis
desktop-menu-edit = _I-edit
desktop-menu-undo = I-_undo
desktop-menu-select-all = Piliin _Lahat
desktop-menu-select-none = Alisin ang _Pagpili
desktop-menu-find = _Hanapin…
desktop-menu-view = _Tingnan
desktop-menu-folder-list = Ipakita ang Listahan ng _Folder
desktop-menu-refresh = I-_refresh
desktop-menu-go = _Pumunta
desktop-menu-inbox = _Inbox
desktop-menu-starred = _Naka-star
desktop-menu-sent = Na_ipadala
desktop-menu-drafts = Mga _Draft
desktop-menu-all-mail = _Lahat ng Mail
desktop-menu-next = Susunod na _Pag-uusap
desktop-menu-previous = Na_karaang Pag-uusap
desktop-menu-message = _Mensahe
desktop-menu-open = _Buksan
desktop-menu-reply = _Sumagot
desktop-menu-reply-all = Sumagot sa _Lahat
desktop-menu-forward = I_pasa
desktop-menu-archive = I-_archive
desktop-menu-delete = I-_delete
desktop-menu-spam = Iulat bilang Spa_m
desktop-menu-move-to = I_lipat sa…
desktop-menu-mark-read = Markahan bilang Na_basa Na
desktop-menu-mark-unread = Markahan bilang _Hindi Pa Nabasa
desktop-menu-star = Lagyan ng S_tar
desktop-menu-important = Markahan bilang Ma_halaga
desktop-menu-not-important = Markahan bilang Hindi Mahala_ga
desktop-menu-settings = Mga _Setting
desktop-menu-quick-settings = _Mabilisang Setting
desktop-menu-configure = I-_configure ang Katna Mail…
desktop-menu-help = _Tulong
desktop-menu-shortcuts = Mga _Keyboard Shortcut
desktop-menu-whats-new = _Ano’ng Bago
desktop-menu-about = Tungkol sa _Katna

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Paggalaw
shortcut-group-actions = Mga aksyon
shortcut-group-go-to = Pumunta sa
shortcut-group-app = Application

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Susunod na pag-uusap
shortcut-previous = Nakaraang pag-uusap
shortcut-down = Bumaba sa listahan
shortcut-up = Umakyat sa listahan
shortcut-first = Una sa listahan
shortcut-last = Huli sa listahan
shortcut-page-down = Isang pahina pababa sa listahan
shortcut-page-up = Isang pahina pataas sa listahan
shortcut-open = Buksan ang pag-uusap
shortcut-back = Bumalik sa listahan
shortcut-scroll-down = Mag-scroll pababa
shortcut-scroll-up = Mag-scroll pataas
shortcut-scroll-page-down = Mag-scroll nang isang pahina pababa
shortcut-scroll-page-up = Mag-scroll nang isang pahina pataas
shortcut-compose = Mag-compose
shortcut-reply = Sumagot
shortcut-reply-all = Sumagot sa lahat
shortcut-forward = Ipasa
shortcut-archive = I-archive
shortcut-delete = I-delete
shortcut-spam = Iulat bilang spam
shortcut-move-to = Ilipat sa
shortcut-mark-read = Markahan bilang nabasa na
shortcut-mark-unread = Markahan bilang hindi pa nabasa
shortcut-star = Lagyan o alisan ng star
shortcut-important = Markahan bilang mahalaga
shortcut-not-important = Markahan bilang hindi mahalaga
shortcut-check = Lagyan ng tsek ang pag-uusap
shortcut-select-all = Lagyan ng tsek ang lahat ng pag-uusap
shortcut-select-none = Alisin ang tsek sa lahat ng pag-uusap
shortcut-undo = I-undo ang huling aksyon
shortcut-go-inbox = Inbox
shortcut-go-starred = Naka-star
shortcut-go-sent = Naipadala
shortcut-go-drafts = Mga Draft
shortcut-go-all = Lahat ng mail
shortcut-search = Maghanap sa mail
shortcut-navigation = Ipakita o itiklop ang menu
shortcut-quick-settings = Mabilisang setting
shortcut-settings = Lahat ng setting
shortcut-shortcuts = Mga keyboard shortcut
shortcut-reload = Tingnan kung may bagong mail
shortcut-quit = Umalis

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } at pagkatapos ay { $second }

## Settings > Accounts

accounts-folder-pane = Pane ng folder
accounts-folder-pane-detail = Kung aling mga folder ng account ang ipinapakita ng pane sa kaliwa.
accounts-shown-one = Isang account sa bawat pagkakataon; magpalit sa account card
accounts-shown-all = Lahat ng account, sunud-sunod
accounts-row = Mga Account
accounts-row-detail = Kapag nag-alis ng account, mabubura ang kopya ng Katna ng mail nito sa computer na ito. Mananatili ang mail sa server.
accounts-none = Wala pang account.
accounts-kind-imported = Na-import
accounts-picture-reset = Gamitin ang larawan ng desktop
accounts-picture-change = Palitan ang larawan
accounts-remove = Alisin
accounts-delete-all-row = I-delete ang lahat ng data
accounts-delete-all-row-detail = Magsimulang muli, gaya ng sa bagong install.
accounts-delete-all-about = Dine-delete mula sa computer na ito ang bawat account, lahat ng naka-save na mail, mga contact at kalendaryo, ang search index, ang iyong mga setting at mga naka-save na password. Walang nagbabago sa iyong mga mail server.
accounts-delete-all-open = I-delete ang lahat ng data ng Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = Inalis ang { $address } sa Katna.
accounts-removed = Inalis ang { $address } sa Katna. Nasa server pa rin ang mail nito.
accounts-all-deleted = Na-delete sa computer na ito ang lahat ng data ng Katna.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Alisin ang { $address }?
accounts-remove-confirm = Alisin ang account
accounts-removing = Inaalis…
accounts-remove-local-mail = { $folders ->
    [0] Lahat ng mail na na-import sa account na ito
    [1] Lahat ng mail na na-import sa account na ito, sa folder nito
   *[other] Lahat ng mail na na-import sa account na ito, sa { $folders } folder nito
}
accounts-remove-local-settings = Ang mga setting nito sa Katna
accounts-remove-mail = { $folders ->
    [0] Lahat ng mail ng account na ito na naka-save sa Katna
    [1] Lahat ng mail ng account na ito na naka-save sa Katna, sa folder nito
   *[other] Lahat ng mail ng account na ito na naka-save sa Katna, sa { $folders } folder nito
}
accounts-remove-outbox = Ang mga mensahe nitong naghihintay sa outbox
accounts-remove-settings = Ang naka-save nitong password at ang mga setting nito sa Katna
accounts-delete-all-title = I-delete ang lahat ng data ng Katna?
accounts-delete-all-confirm = I-delete lahat
accounts-deleting = Dine-delete…
accounts-delete-all-accounts = Bawat account, at lahat ng mail at attachment na naka-save sa Katna
accounts-delete-all-contacts = Mga contact, kalendaryo at ang search index
accounts-delete-all-settings = Lahat ng setting, lagda at keyboard shortcut
accounts-delete-all-passwords = Bawat naka-save na password
accounts-deleted-heading = Mabubura sa computer na ito:
accounts-cannot-undo = Hindi na ito maa-undo.
accounts-server-delete-all = Walang nagbabago sa iyong mga mail server: nananatili roon ang mail mo, at kapag idinagdag muli ang isang account, dina-download itong muli. Nasa Katna lang ang mail na na-import mula sa mga file; hindi ginagalaw ang mga file.
accounts-server-local = Na-import ang mail na ito mula sa mga file, kaya ang Katna lang ang may kopya. Hindi ginagalaw ang mga file na pinagmulan nito; i-import muli ang mga ito para maibalik ito.
accounts-server-remove = Walang nagbabago sa mail server: nananatili roon ang mail mo, at kapag idinagdag muli ang account, dina-download itong muli.
accounts-confirm-word = burahin
accounts-confirm-placeholder = I-type ang “{ accounts-confirm-word }”
accounts-confirm-prompt = Para kumpirmahin, i-type ang “{ accounts-confirm-word }”:
accounts-cancel = Kanselahin
