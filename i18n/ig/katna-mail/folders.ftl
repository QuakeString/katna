# Katna Mail, Igbo (Igbo).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Leebụl
nav-folders = Folda
nav-label-new = Mepụta leebụl ọhụrụ
nav-folder-new = Mepụta folda ọhụrụ
nav-menu-check-mail = Lelee ozi ọhụrụ
nav-menu-check-inbox = Lelee igbe ozi mbata a
nav-unified-leave-out = Hapụ ya n'Igbe ozi mbata jikọrọ ọnụ
nav-unified-bring-back = Weghachi ya n'Igbe ozi mbata jikọrọ ọnụ
nav-menu-sign-in-again = Banye ọzọ
nav-menu-new-mail = Ozi ọhụrụ site n'akaụntụ a
nav-menu-account-settings = Ntọala akaụntụ
nav-account-checked = Emekọrịtala · elelere { $ago }
nav-account-in-sync = Emekọrịtala
nav-account-connecting = Na-ejikọ…
nav-account-offline = Enweghị njikọ, na-anwa ọzọ
nav-account-signed-out = Nbanye { $provider } agwụla
nav-account-password-refused = A jụrụ okwuntughe
nav-account-storage = Ejirila { $used } n'ime { $total }
nav-menu-new-subfolder = Folda ọhụrụ n'ime
nav-menu-new-sublabel = Leebụl ọhụrụ n'ime
nav-menu-rename = Gbanwee aha
nav-menu-delete = Hichapụ
nav-menu-empty-trash = Kpochapụ Ihe mkpofu
nav-account-unnamed = Akaụntụ { $number }
nav-all-accounts = Akaụntụ niile
nav-expand = Gosi folda
nav-collapse = Zoo folda
storage-used = Ejirila { $percent }% nke { $total }
storage-used-detail = { $address }: ejirila { $used } nke { $total }

## Special folders (the user's own folders keep their names)

folder-inbox = Igbe ozi mbata
folder-starred = Nwere kpakpando
folder-snoozed = Ndị e yigharịrị
folder-unread = A gụghị
folder-important = Dị mkpa
folder-drafts = Ndebiri
folder-sent = Ezigara
folder-archive = Ebe nchekwa
folder-spam = Spam
folder-trash = Ihe mkpofu
folder-all-mail = Ozi niile
folder-scheduled = Ahaziri ahazi
folder-waiting = Na-eche nzaghachi
folder-waiting-short = Na-eche
folder-reminders = Ncheta
folder-outbox = Igbe ozi mpụta
folder-activity = Ihe omume
folder-not-on-account = Akaụntụ a enweghị folda dị otú ahụ.

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Leebụl ọhụrụ
label-folder-new-title = Folda ọhụrụ
label-prompt = Biko tinye aha leebụl ọhụrụ:
label-folder-prompt = Biko tinye aha folda ọhụrụ:
label-name-hint = Aha leebụl
label-folder-name-hint = Aha folda
label-nest = Tinye leebụl n'okpuru:
label-folder-nest = Tinye folda n'okpuru:
label-cancel = Kagbuo
label-create = Mepụta
label-creating = Na-emepụta…
label-created = E mepụtala leebụl “{ $name }”.
label-folder-created = E mepụtala folda “{ $name }”.
label-rename-title = Gbanwee aha leebụl
label-folder-rename-title = Gbanwee aha folda
label-rename = Gbanwee aha
label-renaming = Na-agbanwe aha…
label-renamed = Agbanweela aha leebụl ka ọ bụrụ “{ $name }”.
label-folder-renamed = Agbanweela aha folda ka ọ bụrụ “{ $name }”.

## Deleting a folder or label (asked first)

folder-delete-title = Hichapụ “{ $name }”?
folder-delete-body = { $count ->
    [0] Ọ nweghị ozi ọ bụla. A na-ewepụ folda ahụ na sava, ya mere ozi weebụ na ekwentị gị ga-atụfukwa ya.
   *[other] { $kind ->
        [conversation] { $count ->
           *[other] Mkparịta ụka { $count } dị na ya na-aga n'Ihe mkpofu, ya mere ị ka nwere ike iweghachi ha.
        }
       *[message] { $count ->
           *[other] Ozi { $count } dị na ya na-aga n'Ihe mkpofu, ya mere ị ka nwere ike iweghachi ha.
        }
    } A na-ewepụ folda ahụ na sava, ya mere ozi weebụ na ekwentị gị ga-atụfukwa ya.
}
folder-delete-forever-body = { $count ->
    [0] Ọ nweghị ozi ọ bụla. A na-ewepụ folda ahụ na sava, ya mere ozi weebụ na ekwentị gị ga-atụfukwa ya.
   *[other] { $kind ->
        [conversation] { $count ->
           *[other] A na-ehichapụ mkparịta ụka { $count } dị na ya kpamkpam; akaụntụ a enweghị Ihe mkpofu.
        }
       *[message] { $count ->
           *[other] A na-ehichapụ ozi { $count } dị na ya kpamkpam; akaụntụ a enweghị Ihe mkpofu.
        }
    } A na-ewepụ folda ahụ na sava, ya mere ozi weebụ na ekwentị gị ga-atụfukwa ya.
}
folder-delete-label-body = A na-ewepụ leebụl ahụ. Ozi ya na-anọgide n'Ozi niile na na leebụl ya ndị ọzọ.
folder-delete-confirm = Hichapụ folda
folder-delete-label-confirm = Hichapụ leebụl
folder-deleted = Ehichapụla folda “{ $name }”
label-deleted = Ehichapụla leebụl “{ $name }”
