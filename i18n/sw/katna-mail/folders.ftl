# Katna Mail, Swahili (Kiswahili).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Lebo
nav-folders = Folda
nav-label-new = Unda lebo mpya
nav-folder-new = Unda folda mpya
nav-menu-check-mail = Kagua barua mpya
nav-menu-check-inbox = Kagua kikasha hiki
nav-unified-leave-out = Iache nje ya Kikasha kilichounganishwa
nav-unified-bring-back = Irudishe kwenye Kikasha kilichounganishwa
nav-menu-sign-in-again = Ingia tena
nav-menu-new-mail = Barua mpya kutoka akaunti hii
nav-menu-account-settings = Mipangilio ya akaunti
nav-account-checked = Imesawazishwa · imekaguliwa { $ago }
nav-account-in-sync = Imesawazishwa
nav-account-connecting = Inaunganisha…
nav-account-offline = Nje ya mtandao, inajaribu tena
nav-account-signed-out = Kuingia kwa { $provider } kumeisha muda
nav-account-password-refused = Nenosiri limekataliwa
nav-account-storage = { $used } kati ya { $total } zimetumika
nav-menu-new-subfolder = Folda mpya ndani
nav-menu-new-sublabel = Lebo mpya ndani
nav-menu-rename = Badilisha jina
nav-menu-delete = Futa
nav-menu-empty-trash = Safisha Tupio
nav-account-unnamed = Akaunti { $number }
nav-all-accounts = Akaunti Zote
nav-expand = Onyesha folda
nav-collapse = Ficha folda
storage-used = Imetumika { $percent }% ya { $total }
storage-used-detail = { $address }: imetumika { $used } ya { $total }

## Special folders (the user's own folders keep their names)

folder-inbox = Kikasha
folder-starred = Zenye nyota
folder-snoozed = Zilizoahirishwa
folder-unread = Ambazo hazijasomwa
folder-important = Muhimu
folder-drafts = Rasimu
folder-sent = Zilizotumwa
folder-archive = Kumbukumbu
folder-spam = Taka
folder-trash = Tupio
folder-all-mail = Barua zote
folder-scheduled = Zilizoratibiwa
folder-waiting = Zinasubiri jibu
folder-waiting-short = Zinasubiri
folder-reminders = Vikumbusho
folder-outbox = Kikasha toezi
folder-activity = Shughuli
folder-not-on-account = Akaunti hii haina folda kama hiyo.

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
label-rename-title = Badilisha jina la lebo
label-folder-rename-title = Badilisha jina la folda
label-rename = Badilisha jina
label-renaming = Inabadilisha jina…
label-renamed = Lebo imepewa jina jipya “{ $name }”.
label-folder-renamed = Folda imepewa jina jipya “{ $name }”.

## Deleting a folder or label (asked first)

folder-delete-title = Futa “{ $name }”?
folder-delete-body = { $count ->
    [0] Haina barua. Folda inaondolewa kwenye seva, hivyo barua pepe ya wavuti na simu yako pia zinaipoteza.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Mazungumzo yake { $count } yanaenda kwenye Tupio, hivyo bado unaweza kuyarudisha.
           *[other] Mazungumzo yake { $count } yanaenda kwenye Tupio, hivyo bado unaweza kuyarudisha.
        }
       *[message] { $count ->
            [one] Ujumbe wake { $count } unaenda kwenye Tupio, hivyo bado unaweza kuurudisha.
           *[other] Jumbe zake { $count } zinaenda kwenye Tupio, hivyo bado unaweza kuzirudisha.
        }
    } Folda inaondolewa kwenye seva, hivyo barua pepe ya wavuti na simu yako pia zinaipoteza.
}
folder-delete-forever-body = { $count ->
    [0] Haina barua. Folda inaondolewa kwenye seva, hivyo barua pepe ya wavuti na simu yako pia zinaipoteza.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Mazungumzo yake { $count } yanafutwa kabisa; akaunti hii haina Tupio.
           *[other] Mazungumzo yake { $count } yanafutwa kabisa; akaunti hii haina Tupio.
        }
       *[message] { $count ->
            [one] Ujumbe wake { $count } unafutwa kabisa; akaunti hii haina Tupio.
           *[other] Jumbe zake { $count } zinafutwa kabisa; akaunti hii haina Tupio.
        }
    } Folda inaondolewa kwenye seva, hivyo barua pepe ya wavuti na simu yako pia zinaipoteza.
}
folder-delete-label-body = Lebo inaondolewa. Barua zake zinabaki kwenye Barua zote na kwenye lebo zake nyingine.
folder-delete-confirm = Futa folda
folder-delete-label-confirm = Futa lebo
folder-deleted = Folda “{ $name }” imefutwa
label-deleted = Lebo “{ $name }” imefutwa
