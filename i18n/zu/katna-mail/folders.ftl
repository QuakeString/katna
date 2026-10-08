# Katna Mail, Zulu (isiZulu).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Amalebula
nav-folders = Amafolda
nav-label-new = Dala ilebula entsha
nav-folder-new = Dala ifolda entsha
nav-menu-check-mail = Hlola imeyili entsha
nav-menu-check-inbox = Hlola leli bhokisi lokungenayo
nav-unified-leave-out = Ungafaki ebhokisini lokungenayo elihlanganisiwe
nav-unified-bring-back = Buyisela ebhokisini lokungenayo elihlanganisiwe
nav-menu-sign-in-again = Ngena futhi
nav-menu-new-mail = Imeyili entsha evela kule akhawunti
nav-menu-account-settings = Izilungiselelo ze-akhawunti
nav-account-checked = Kuvumelanisiwe · kuhlolwe { $ago }
nav-account-in-sync = Kuvumelanisiwe
nav-account-connecting = Kuyaxhunywa…
nav-account-offline = Akuxhunyiwe, kuzanywa futhi
nav-account-signed-out = Ukungena nge-{ $provider } kuphelelwe yisikhathi
nav-account-password-refused = Iphasiwedi yenqatshiwe
nav-account-storage = Kusetshenziswe { $used } kokungu-{ $total }
nav-menu-new-subfolder = Ifolda entsha ngaphakathi
nav-menu-new-sublabel = Ilebula entsha ngaphakathi
nav-menu-rename = Qamba kabusha
nav-menu-delete = Susa
nav-menu-empty-trash = Sula Udoti
nav-account-unnamed = I-akhawunti { $number }
nav-all-accounts = Wonke Ama-akhawunti
nav-expand = Bonisa amafolda
nav-collapse = Fihla amafolda
storage-used = Kusetshenziswe { $percent }% ku-{ $total }
storage-used-detail = { $address }: kusetshenziswe { $used } ku-{ $total }

## Special folders (the user's own folders keep their names)

folder-inbox = Ibhokisi lokungenayo
folder-starred = Okunenkanyezi
folder-snoozed = Okulibazisiwe
folder-unread = Okungafundiwe
folder-important = Okubalulekile
folder-drafts = Okusalungiswa
folder-sent = Okuthunyelwe
folder-archive = Ingobo yomlando
folder-spam = Ugaxekile
folder-trash = Udoti
folder-all-mail = Wonke amameyili
folder-scheduled = Okuhleliwe
folder-waiting = Kulindwe impendulo
folder-waiting-short = Kulindiwe
folder-reminders = Izikhumbuzo
folder-outbox = Ibhokisi eliphumayo
folder-activity = Umsebenzi

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Ilebula entsha
label-folder-new-title = Ifolda entsha
label-prompt = Sicela ufake igama elisha lelebula:
label-folder-prompt = Sicela ufake igama elisha lefolda:
label-name-hint = Igama lelebula
label-folder-name-hint = Igama lefolda
label-nest = Faka ilebula ngaphansi kwe:
label-folder-nest = Faka ifolda ngaphansi kwe:
label-cancel = Khansela
label-create = Dala
label-creating = Iyadala…
label-created = Ilebula ethi “{ $name }” idaliwe.
label-folder-created = Ifolda ethi “{ $name }” idaliwe.
label-rename-title = Qamba kabusha ilebula
label-folder-rename-title = Qamba kabusha ifolda
label-rename = Qamba kabusha
label-renaming = Iqamba kabusha…
label-renamed = Ilebula liqanjwe kabusha laba ngu-“{ $name }”.
label-folder-renamed = Ifolda iqanjwe kabusha yaba ngu-“{ $name }”.

## Deleting a folder or label (asked first)

folder-delete-title = Susa “{ $name }”?
folder-delete-body = { $count ->
    [0] Ayinayo imeyili. Ifolda isuswa kuseva, ngakho i-webmail nefoni yakho nakho kuyayilahlekelwa.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Ingxoxo yayo engu-{ $count } iya kudoti, ngakho usengayibuyisa.
           *[other] Izingxoxo zayo ezingu-{ $count } ziya kudoti, ngakho usengazibuyisa.
        }
       *[message] { $count ->
            [one] Umlayezo wayo ongu-{ $count } uya kudoti, ngakho usengawubuyisa.
           *[other] Imilayezo yayo engu-{ $count } iya kudoti, ngakho usengayibuyisa.
        }
    } Ifolda isuswa kuseva, ngakho i-webmail nefoni yakho nakho kuyayilahlekelwa.
}
folder-delete-forever-body = { $count ->
    [0] Ayinayo imeyili. Ifolda isuswa kuseva, ngakho i-webmail nefoni yakho nakho kuyayilahlekelwa.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Ingxoxo yayo engu-{ $count } isuswa unomphela; le akhawunti ayinaye udoti.
           *[other] Izingxoxo zayo ezingu-{ $count } zisuswa unomphela; le akhawunti ayinaye udoti.
        }
       *[message] { $count ->
            [one] Umlayezo wayo ongu-{ $count } ususwa unomphela; le akhawunti ayinaye udoti.
           *[other] Imilayezo yayo engu-{ $count } isuswa unomphela; le akhawunti ayinaye udoti.
        }
    } Ifolda isuswa kuseva, ngakho i-webmail nefoni yakho nakho kuyayilahlekelwa.
}
folder-delete-label-body = Ilebula liyasuswa. Imeyili yalo isala ku-Wonke amameyili nakwamanye amalebula ayo.
folder-delete-confirm = Susa ifolda
folder-delete-label-confirm = Susa ilebula
folder-deleted = Ifolda ethi “{ $name }” isusiwe
label-deleted = Ilebula elithi “{ $name }” lisusiwe
