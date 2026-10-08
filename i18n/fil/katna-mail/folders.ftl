# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Mga Label
nav-folders = Mga Folder
nav-label-new = Gumawa ng bagong label
nav-folder-new = Gumawa ng bagong folder
nav-menu-check-mail = Tingnan kung may bagong mail
nav-menu-check-inbox = Tingnan ang inbox na ito
nav-unified-leave-out = Huwag isama sa pinag-isang Inbox
nav-unified-bring-back = Ibalik sa pinag-isang Inbox
nav-menu-sign-in-again = Mag-sign in ulit
nav-menu-new-mail = Bagong mail mula sa account na ito
nav-menu-account-settings = Mga setting ng account
nav-account-checked = Naka-sync · tiningnan { $ago }
nav-account-in-sync = Naka-sync
nav-account-connecting = Kumokonekta…
nav-account-offline = Offline, sinusubukan ulit
nav-account-signed-out = Nag-expire ang pag-sign in sa { $provider }
nav-account-password-refused = Tinanggihan ang password
nav-account-storage = { $used } ng { $total } ang nagamit
nav-menu-new-subfolder = Bagong folder sa loob
nav-menu-new-sublabel = Bagong label sa loob
nav-menu-rename = I-rename
nav-menu-delete = I-delete
nav-menu-empty-trash = Alisin ang laman ng Basurahan
nav-account-unnamed = Account { $number }
nav-all-accounts = Lahat ng Account
nav-expand = Ipakita ang mga folder
nav-collapse = Itago ang mga folder
storage-used = { $percent }% ng { $total } ang nagamit
storage-used-detail = { $address }: { $used } ng { $total } ang nagamit

## Special folders (the user's own folders keep their names)

folder-inbox = Inbox
folder-starred = Naka-star
folder-snoozed = Naka-snooze
folder-unread = Hindi pa nabasa
folder-important = Mahalaga
folder-drafts = Mga Draft
folder-sent = Naipadala
folder-archive = Archive
folder-spam = Spam
folder-trash = Basurahan
folder-all-mail = Lahat ng Mail
folder-scheduled = Naka-iskedyul
folder-waiting = Naghihintay ng sagot
folder-waiting-short = Naghihintay
folder-reminders = Mga Paalala
folder-outbox = Outbox
folder-activity = Aktibidad

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
label-rename-title = I-rename ang label
label-folder-rename-title = I-rename ang folder
label-rename = I-rename
label-renaming = Nire-rename…
label-renamed = Na-rename ang label bilang “{ $name }”.
label-folder-renamed = Na-rename ang folder bilang “{ $name }”.

## Deleting a folder or label (asked first)

folder-delete-title = I-delete ang “{ $name }”?
folder-delete-body = { $count ->
    [0] Wala itong mail. Aalisin ang folder sa server, kaya mawawala rin ito sa webmail at sa iyong phone.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Mapupunta sa Basurahan ang { $count } pag-uusap nito, kaya maibabalik mo pa ito.
           *[other] Mapupunta sa Basurahan ang { $count } pag-uusap nito, kaya maibabalik mo pa ang mga ito.
        }
       *[message] { $count ->
            [one] Mapupunta sa Basurahan ang { $count } mensahe nito, kaya maibabalik mo pa ito.
           *[other] Mapupunta sa Basurahan ang { $count } mensahe nito, kaya maibabalik mo pa ang mga ito.
        }
    } Aalisin ang folder sa server, kaya mawawala rin ito sa webmail at sa iyong phone.
}
folder-delete-forever-body = { $count ->
    [0] Wala itong mail. Aalisin ang folder sa server, kaya mawawala rin ito sa webmail at sa iyong phone.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Permanenteng made-delete ang { $count } pag-uusap nito; walang Basurahan ang account na ito.
           *[other] Permanenteng made-delete ang { $count } pag-uusap nito; walang Basurahan ang account na ito.
        }
       *[message] { $count ->
            [one] Permanenteng made-delete ang { $count } mensahe nito; walang Basurahan ang account na ito.
           *[other] Permanenteng made-delete ang { $count } mensahe nito; walang Basurahan ang account na ito.
        }
    } Aalisin ang folder sa server, kaya mawawala rin ito sa webmail at sa iyong phone.
}
folder-delete-label-body = Aalisin ang label. Mananatili ang mail nito sa Lahat ng Mail at sa iba pa nitong label.
folder-delete-confirm = I-delete ang folder
folder-delete-label-confirm = I-delete ang label
folder-deleted = Na-delete ang folder na “{ $name }”
label-deleted = Na-delete ang label na “{ $name }”
