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
