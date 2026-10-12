# Katna Mail, Hausa (Hausa).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Lakabai
nav-folders = Folda
nav-label-new = Ƙirƙiri sabon lakabi
nav-folder-new = Ƙirƙiri sabuwar folda
nav-menu-check-mail = Duba sabbin wasiƙu
nav-menu-check-inbox = Duba wannan akwatin saƙo
nav-unified-leave-out = Cire daga Haɗaɗɗen akwatin saƙo
nav-unified-bring-back = Mayar cikin Haɗaɗɗen akwatin saƙo
nav-menu-sign-in-again = Sake shiga
nav-menu-new-mail = Sabuwar wasiƙa daga wannan asusu
nav-menu-account-settings = Saitunan asusu
nav-account-checked = An daidaita · an duba { $ago }
nav-account-in-sync = An daidaita
nav-account-connecting = Ana haɗawa…
nav-account-offline = Babu intanet, ana sake gwadawa
nav-account-signed-out = Shigar { $provider } ta ƙare
nav-account-password-refused = An ƙi kalmar sirri
nav-account-storage = An yi amfani da { $used } daga { $total }
nav-menu-new-subfolder = Sabuwar folda a ciki
nav-menu-new-sublabel = Sabon lakabi a ciki
nav-menu-rename = Sake suna
nav-menu-delete = Share
nav-menu-empty-trash = Kwashe kwandon shara
nav-account-unnamed = Asusu { $number }
nav-all-accounts = Dukkan asusu
nav-expand = Nuna folda
nav-collapse = Ɓoye folda
storage-used = An yi amfani da { $percent }% na { $total }
storage-used-detail = { $address }: an yi amfani da { $used } na { $total }

## Special folders (the user's own folders keep their names)

folder-inbox = Akwatin saƙo
folder-starred = Masu tauraro
folder-snoozed = Waɗanda aka jinkirta
folder-unread = Ba a karanta ba
folder-important = Muhimmi
folder-drafts = Zayyanai
folder-sent = Waɗanda aka aika
folder-archive = Ma'ajiya
folder-spam = Saƙonnin banza
folder-trash = Kwandon shara
folder-all-mail = Duk wasiƙu
folder-scheduled = Waɗanda aka tsara
folder-waiting = Ana jiran amsa
folder-waiting-short = Ana jira
folder-reminders = Tunatarwa
folder-outbox = Akwatin fita
folder-activity = Ayyuka
folder-not-on-account = Wannan asusu ba shi da irin wannan folda.

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
label-rename-title = Sake sunan lakabi
label-folder-rename-title = Sake sunan folda
label-rename = Sake suna
label-renaming = Ana sake suna…
label-renamed = An sake wa lakabin suna zuwa “{ $name }”.
label-folder-renamed = An sake wa foldar suna zuwa “{ $name }”.

## Deleting a folder or label (asked first)

folder-delete-title = A share “{ $name }”?
folder-delete-body = { $count ->
    [0] Babu wasiƙa a cikinta. Ana cire foldar daga sabar, don haka wasiƙun yanar gizo da wayarku ma za su rasa ta.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Tattaunawarta { $count } tana zuwa Kwandon shara, don haka kuna iya dawo da ita.
           *[other] Tattaunawowinta { $count } suna zuwa Kwandon shara, don haka kuna iya dawo da su.
        }
       *[message] { $count ->
            [one] Saƙonta { $count } yana zuwa Kwandon shara, don haka kuna iya dawo da shi.
           *[other] Saƙonninta { $count } suna zuwa Kwandon shara, don haka kuna iya dawo da su.
        }
    } Ana cire foldar daga sabar, don haka wasiƙun yanar gizo da wayarku ma za su rasa ta.
}
folder-delete-forever-body = { $count ->
    [0] Babu wasiƙa a cikinta. Ana cire foldar daga sabar, don haka wasiƙun yanar gizo da wayarku ma za su rasa ta.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] Ana share tattaunawarta { $count } har abada; wannan asusun ba shi da Kwandon shara.
           *[other] Ana share tattaunawowinta { $count } har abada; wannan asusun ba shi da Kwandon shara.
        }
       *[message] { $count ->
            [one] Ana share saƙonta { $count } har abada; wannan asusun ba shi da Kwandon shara.
           *[other] Ana share saƙonninta { $count } har abada; wannan asusun ba shi da Kwandon shara.
        }
    } Ana cire foldar daga sabar, don haka wasiƙun yanar gizo da wayarku ma za su rasa ta.
}
folder-delete-label-body = Ana cire lakabin. Wasiƙunsa suna nan a All mail da sauran lakabobinsu.
folder-delete-confirm = Share folda
folder-delete-label-confirm = Share lakabi
folder-deleted = An share folda “{ $name }”
label-deleted = An share lakabi “{ $name }”
