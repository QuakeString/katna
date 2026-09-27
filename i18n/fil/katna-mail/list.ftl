# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
