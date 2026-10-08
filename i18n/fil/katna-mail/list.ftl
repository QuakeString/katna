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
tab-provider-other = inayos ng Katna

## Mail list: toolbar

list-select = Piliin
list-refresh = I-refresh
list-back-to-top = Bumalik sa itaas
list-checking = Tinitingnan kung may bagong mail…
list-more = Higit pa
list-mark-read = Markahan bilang nabasa na
list-mark-unread = Markahan bilang hindi pa nabasa
list-move-to = Ilipat sa
list-archive = I-archive
list-spam = Iulat bilang spam
list-delete = I-delete
list-snooze = I-snooze
list-unsnooze = I-unsnooze
list-newer = Mas bago
list-older = Mas luma
list-range = { $first }–{ $last } ng { $total }
list-range-about = { $first }–{ $last } ng humigit-kumulang { $total }
list-results = Mga resulta para sa “{ $query }”
list-results-corrected = Ipinapakita ang mga resulta para sa “{ $query }”
list-search-instead = Hanapin na lang ang “{ $query }”
list-files-more = +{ $count }
list-replied = Sumagot ka na

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
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Napili ang { $count } pag-uusap na nabasa na sa screen.
           *[other] Napili ang lahat ng { $count } pag-uusap na nabasa na sa screen.
        }
       *[message] { $count ->
            [one] Napili ang { $count } mensahe na nabasa na sa screen.
           *[other] Napili ang lahat ng { $count } mensahe na nabasa na sa screen.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Napili ang { $count } pag-uusap na hindi pa nabasa sa screen.
           *[other] Napili ang lahat ng { $count } pag-uusap na hindi pa nabasa sa screen.
        }
       *[message] { $count ->
            [one] Napili ang { $count } mensahe na hindi pa nabasa sa screen.
           *[other] Napili ang lahat ng { $count } mensahe na hindi pa nabasa sa screen.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Napili ang { $count } pag-uusap na naka-star sa screen.
           *[other] Napili ang lahat ng { $count } pag-uusap na naka-star sa screen.
        }
       *[message] { $count ->
            [one] Napili ang { $count } mensahe na naka-star sa screen.
           *[other] Napili ang lahat ng { $count } mensahe na naka-star sa screen.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Napili ang { $count } pag-uusap na walang star sa screen.
           *[other] Napili ang lahat ng { $count } pag-uusap na walang star sa screen.
        }
       *[message] { $count ->
            [one] Napili ang { $count } mensahe na walang star sa screen.
           *[other] Napili ang lahat ng { $count } mensahe na walang star sa screen.
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Piliin ang { $count } pag-uusap na nabasa na
           *[other] Piliin ang lahat ng { $count } pag-uusap na nabasa na
        }
       *[message] { $count ->
            [one] Piliin ang { $count } mensahe na nabasa na
           *[other] Piliin ang lahat ng { $count } mensahe na nabasa na
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Piliin ang { $count } pag-uusap na hindi pa nabasa
           *[other] Piliin ang lahat ng { $count } pag-uusap na hindi pa nabasa
        }
       *[message] { $count ->
            [one] Piliin ang { $count } mensahe na hindi pa nabasa
           *[other] Piliin ang lahat ng { $count } mensahe na hindi pa nabasa
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Piliin ang { $count } pag-uusap na naka-star
           *[other] Piliin ang lahat ng { $count } pag-uusap na naka-star
        }
       *[message] { $count ->
            [one] Piliin ang { $count } mensahe na naka-star
           *[other] Piliin ang lahat ng { $count } mensahe na naka-star
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Piliin ang { $count } pag-uusap na walang star
           *[other] Piliin ang lahat ng { $count } pag-uusap na walang star
        }
       *[message] { $count ->
            [one] Piliin ang { $count } mensahe na walang star
           *[other] Piliin ang lahat ng { $count } mensahe na walang star
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Piliin ang { $count } pag-uusap na nabasa na sa { $folder }
           *[other] Piliin ang lahat ng { $count } pag-uusap na nabasa na sa { $folder }
        }
       *[message] { $count ->
            [one] Piliin ang { $count } mensahe na nabasa na sa { $folder }
           *[other] Piliin ang lahat ng { $count } mensahe na nabasa na sa { $folder }
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Piliin ang { $count } pag-uusap na hindi pa nabasa sa { $folder }
           *[other] Piliin ang lahat ng { $count } pag-uusap na hindi pa nabasa sa { $folder }
        }
       *[message] { $count ->
            [one] Piliin ang { $count } mensahe na hindi pa nabasa sa { $folder }
           *[other] Piliin ang lahat ng { $count } mensahe na hindi pa nabasa sa { $folder }
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Piliin ang { $count } pag-uusap na naka-star sa { $folder }
           *[other] Piliin ang lahat ng { $count } pag-uusap na naka-star sa { $folder }
        }
       *[message] { $count ->
            [one] Piliin ang { $count } mensahe na naka-star sa { $folder }
           *[other] Piliin ang lahat ng { $count } mensahe na naka-star sa { $folder }
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Piliin ang { $count } pag-uusap na walang star sa { $folder }
           *[other] Piliin ang lahat ng { $count } pag-uusap na walang star sa { $folder }
        }
       *[message] { $count ->
            [one] Piliin ang { $count } mensahe na walang star sa { $folder }
           *[other] Piliin ang lahat ng { $count } mensahe na walang star sa { $folder }
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Napili ang { $count } pag-uusap na nabasa na.
           *[other] Napili ang lahat ng { $count } pag-uusap na nabasa na.
        }
       *[message] { $count ->
            [one] Napili ang { $count } mensahe na nabasa na.
           *[other] Napili ang lahat ng { $count } mensahe na nabasa na.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Napili ang { $count } pag-uusap na hindi pa nabasa.
           *[other] Napili ang lahat ng { $count } pag-uusap na hindi pa nabasa.
        }
       *[message] { $count ->
            [one] Napili ang { $count } mensahe na hindi pa nabasa.
           *[other] Napili ang lahat ng { $count } mensahe na hindi pa nabasa.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Napili ang { $count } pag-uusap na naka-star.
           *[other] Napili ang lahat ng { $count } pag-uusap na naka-star.
        }
       *[message] { $count ->
            [one] Napili ang { $count } mensahe na naka-star.
           *[other] Napili ang lahat ng { $count } mensahe na naka-star.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Napili ang { $count } pag-uusap na walang star.
           *[other] Napili ang lahat ng { $count } pag-uusap na walang star.
        }
       *[message] { $count ->
            [one] Napili ang { $count } mensahe na walang star.
           *[other] Napili ang lahat ng { $count } mensahe na walang star.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Napili ang { $count } pag-uusap na nabasa na sa { $folder }.
           *[other] Napili ang lahat ng { $count } pag-uusap na nabasa na sa { $folder }.
        }
       *[message] { $count ->
            [one] Napili ang { $count } mensahe na nabasa na sa { $folder }.
           *[other] Napili ang lahat ng { $count } mensahe na nabasa na sa { $folder }.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Napili ang { $count } pag-uusap na hindi pa nabasa sa { $folder }.
           *[other] Napili ang lahat ng { $count } pag-uusap na hindi pa nabasa sa { $folder }.
        }
       *[message] { $count ->
            [one] Napili ang { $count } mensahe na hindi pa nabasa sa { $folder }.
           *[other] Napili ang lahat ng { $count } mensahe na hindi pa nabasa sa { $folder }.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Napili ang { $count } pag-uusap na naka-star sa { $folder }.
           *[other] Napili ang lahat ng { $count } pag-uusap na naka-star sa { $folder }.
        }
       *[message] { $count ->
            [one] Napili ang { $count } mensahe na naka-star sa { $folder }.
           *[other] Napili ang lahat ng { $count } mensahe na naka-star sa { $folder }.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Napili ang { $count } pag-uusap na walang star sa { $folder }.
           *[other] Napili ang lahat ng { $count } pag-uusap na walang star sa { $folder }.
        }
       *[message] { $count ->
            [one] Napili ang { $count } mensahe na walang star sa { $folder }.
           *[other] Napili ang lahat ng { $count } mensahe na walang star sa { $folder }.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Walang pag-uusap dito na nabasa na.
       *[message] Walang mensahe dito na nabasa na.
    }
   *[unread] { $kind ->
        [conversation] Walang pag-uusap dito na hindi pa nabasa.
       *[message] Walang mensahe dito na hindi pa nabasa.
    }
    [starred] { $kind ->
        [conversation] Walang pag-uusap dito na naka-star.
       *[message] Walang mensahe dito na naka-star.
    }
    [unstarred] { $kind ->
        [conversation] Walang pag-uusap dito na walang star.
       *[message] Walang mensahe dito na walang star.
    }
}
list-clear-selection = I-clear ang pagpili

## Mail list: empty states

list-empty-search = Walang mensaheng tumugma sa iyong paghahanap.
list-empty-tab = Walang mail sa { $tab }.
list-empty-tab-unknown = Walang mail sa tab na ito.
list-empty-folder = Walang mensahe sa { $folder }.
list-empty-folder-unknown = Walang mensahe sa folder na ito.
list-empty-waiting = Walang naghihintay ng sagot.
list-empty-reminders = Walang paalala. Pindutin ang H sa isang mail para magdagdag.
list-first-sync = Kinukuha ang iyong mail…
list-first-sync-detail = Lalabas ito dito habang dumarating.

## Mail list: lines

row-removed = Inalis ang mensaheng ito.
row-starred = Naka-star
row-not-starred = Walang star
row-important = Mahalaga. I-click para markahan bilang hindi mahalaga.
row-mark-important = Markahan bilang mahalaga
row-pinned = Naka-pin sa itaas
row-task = Gawain
row-task-open = Buksan ang gawain: { $title }
row-tracking-none = Naka-track. Hindi pa nabubuksan
row-tracking-opened = Binuksan ng { $opened } sa { $recipients }
row-tracking-clicked = Binuksan ng { $opened } sa { $recipients }, may link na sinundan ng { $clicked }
row-pin = I-pin sa itaas
row-unpin = I-unpin
row-snoozed-until = Naka-snooze hanggang { $when }
row-snoozed-day-time = { $day } { $time }
snoozed-group-today = Ngayon
snoozed-group-tomorrow = Bukas
snoozed-group-this-week = Ngayong linggo
snoozed-group-later = Mamaya pa
row-follow-up-step = Follow-up { $step } sa { $steps } · { $date }
row-follow-up-waiting = Naghihintay ang follow-up
row-reminder = Paalala { $date }

## Mail list: More menu and right-click menu

menu-reply = Sumagot
menu-reply-all = Sumagot sa lahat
menu-forward = Ipasa
menu-archive = I-archive
menu-delete = I-delete
menu-delete-forever = I-delete nang permanente
menu-move-to-inbox = Ilipat sa Inbox
menu-spam = Iulat bilang spam
menu-not-spam = Hindi spam
menu-mark-read = Markahan bilang nabasa na
menu-mark-unread = Markahan bilang hindi pa nabasa
menu-mark-all-read = Markahan lahat bilang nabasa na
menu-star = Magdagdag ng star
menu-unstar = Alisin ang star
menu-important = Markahan bilang mahalaga
menu-not-important = Markahan bilang hindi mahalaga
menu-pin = I-pin sa itaas
menu-unpin = I-unpin
menu-snooze = I-snooze
menu-remind = Paalalahanan ako
menu-unsnooze = I-unsnooze
menu-add-to-tasks = Idagdag sa Mga Gawain
menu-schedule-meeting = Mag-iskedyul ng pulong
menu-start-call = Magsimula ng video call
menu-add-note = Magdagdag ng tala
menu-print-all = I-print lahat
menu-new-window = Buksan sa bagong window
menu-move-to = Ilipat sa
menu-follow-up = I-follow up
menu-more = Iba pa
menu-move-to-heading = Ilipat sa:
menu-move-to-search = Ilipat sa…
menu-label-as = Lagyan ng label
menu-label-as-search = Lagyan ng label…
menu-no-folder = Walang folder na “{ $name }”
menu-no-label = Walang label na “{ $name }”
menu-create-folder = Gumawa ng “{ $name }”
menu-always-move = Palaging ilipat dito ang mail mula kay { $name }
toast-always-move-failed = Nailipat ang mail, pero hindi nagawa ang panuntunan: { $error }
drag-mail = { $kind ->
    [conversation] { $count ->
        [one] { $count } pag-uusap
       *[other] { $count } pag-uusap
    }
   *[message] { $count ->
        [one] { $count } mensahe
       *[other] { $count } mensahe
    }
}
menu-find-from = Hanapin ang mga email mula kay { $name }
menu-make-rule = Gumawa ng panuntunan…

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
toast-label-added = Idinagdag ang label na “{ $label }”.
toast-label-removed = Inalis ang label na “{ $label }”.
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
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] Na-snooze ang { $count } pag-uusap hanggang { $when }.
       *[other] Na-snooze ang { $count } pag-uusap hanggang { $when }.
    }
   *[message] { $count ->
        [one] Na-snooze ang { $count } mensahe hanggang { $when }.
       *[other] Na-snooze ang { $count } mensahe hanggang { $when }.
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] Bumalik sa Inbox ang { $count } pag-uusap.
       *[other] Bumalik sa Inbox ang { $count } pag-uusap.
    }
   *[message] { $count ->
        [one] Bumalik sa Inbox ang { $count } mensahe.
       *[other] Bumalik sa Inbox ang { $count } mensahe.
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
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] Minarkahang hindi spam at inilipat sa inbox ang { $count } pag-uusap.
       *[other] Minarkahang hindi spam at inilipat sa inbox ang { $count } pag-uusap.
    }
   *[message] { $count ->
        [one] Minarkahang hindi spam at inilipat sa inbox ang { $count } mensahe.
       *[other] Minarkahang hindi spam at inilipat sa inbox ang { $count } mensahe.
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
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] Minarkahan ang pag-uusap bilang nabasa na.
       *[other] Minarkahan ang { $count } pag-uusap bilang nabasa na.
    }
   *[message] { $count ->
        [one] Minarkahan ang mensahe bilang nabasa na.
       *[other] Minarkahan ang { $count } mensahe bilang nabasa na.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] Minarkahan ang pag-uusap bilang hindi pa nabasa.
       *[other] Minarkahan ang { $count } pag-uusap bilang hindi pa nabasa.
    }
   *[message] { $count ->
        [one] Minarkahan ang mensahe bilang hindi pa nabasa.
       *[other] Minarkahan ang { $count } mensahe bilang hindi pa nabasa.
    }
}
toast-undone = Na-undo ang aksyon.
toast-nothing-to-undo = Walang ia-undo.
toast-cannot-undo-delete-forever = Hindi na maibabalik ang mail na permanenteng na-delete.
toast-send-undone = Na-undo ang pagpapadala.
toast-too-late-to-undo-send = Huli na para i-undo: naipadala na ang mensahe.
toast-undo = I-undo
toast-close = Isara
toast-no-spam-folder = Walang spam folder ang account na ito.
