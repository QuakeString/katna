# Katna Mail, Yoruba (Yorùbá).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Àwọn àmì
nav-folders = Àwọn fódà
nav-label-new = Ṣẹ̀dá àmì tuntun
nav-folder-new = Ṣẹ̀dá fódà tuntun
nav-menu-check-mail = Ṣàyẹ̀wò lẹ́tà tuntun
nav-menu-check-inbox = Ṣàyẹ̀wò àpótí-ìwọlé yìí
nav-unified-leave-out = Yọ ọ́ kúrò nínú Àpótí-ìwọlé àpapọ̀
nav-unified-bring-back = Dá a padà sínú Àpótí-ìwọlé àpapọ̀
nav-menu-sign-in-again = Wọlé lẹ́ẹ̀kan sí i
nav-menu-new-mail = Lẹ́tà tuntun láti àkáǹtì yìí
nav-menu-account-settings = Ètò àkáǹtì
nav-account-checked = Wà ní ìbámu · a ṣàyẹ̀wò { $ago }
nav-account-in-sync = Wà ní ìbámu
nav-account-connecting = Ń so pọ̀…
nav-account-offline = Kò sí lórí ayélujára, ń gbìyànjú lẹ́ẹ̀kan sí i
nav-account-signed-out = Wíwọlé { $provider } ti parí
nav-account-password-refused = A kọ ọ̀rọ̀ aṣínà
nav-account-storage = A ti lo { $used } nínú { $total }
nav-menu-new-subfolder = Fódà tuntun ní inú
nav-menu-new-sublabel = Àmì tuntun ní inú
nav-menu-rename = Yí orúkọ padà
nav-menu-delete = Pa rẹ́
nav-menu-empty-trash = Sọ Ìdọ̀tí di òfo
nav-account-unnamed = Àkáǹtì { $number }
nav-all-accounts = Gbogbo àkáǹtì
nav-expand = Fi àwọn fódà hàn
nav-collapse = Fi àwọn fódà pamọ́
storage-used = A ti lo { $percent }% nínú { $total }
storage-used-detail = { $address }: a ti lo { $used } nínú { $total }

## Special folders (the user's own folders keep their names)

folder-inbox = Àpótí-ìwọlé
folder-starred = Oní ìràwọ̀
folder-snoozed = Tí a sún síwájú
folder-unread = Àìkà
folder-important = Pàtàkì
folder-drafts = Àwọn àkọ̀pamọ́
folder-sent = Tí a fi ránṣẹ́
folder-archive = Ibi ìpamọ́
folder-spam = Àwúrúju
folder-trash = Ìdọ̀tí
folder-all-mail = Gbogbo lẹ́tà
folder-scheduled = Tí a ṣètò
folder-waiting = Ń dúró de èsì
folder-waiting-short = Ń dúró
folder-reminders = Ìránnilétí
folder-outbox = Àpótí-ìjáde
folder-activity = Ìgbòkègbodò

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Àmì tuntun
label-folder-new-title = Fódà tuntun
label-prompt = Jọ̀wọ́ tẹ orúkọ àmì tuntun:
label-folder-prompt = Jọ̀wọ́ tẹ orúkọ fódà tuntun:
label-name-hint = Orúkọ àmì
label-folder-name-hint = Orúkọ fódà
label-nest = Fi àmì sí abẹ́:
label-folder-nest = Fi fódà sí abẹ́:
label-cancel = Fagilé
label-create = Ṣẹ̀dá
label-creating = Ó ń ṣẹ̀dá…
label-created = A ti ṣẹ̀dá àmì “{ $name }”.
label-folder-created = A ti ṣẹ̀dá fódà “{ $name }”.
label-rename-title = Yí orúkọ àmì padà
label-folder-rename-title = Yí orúkọ fódà padà
label-rename = Yí orúkọ padà
label-renaming = Ń yí orúkọ padà…
label-renamed = A ti yí orúkọ àmì padà sí “{ $name }”.
label-folder-renamed = A ti yí orúkọ fódà padà sí “{ $name }”.

## Deleting a folder or label (asked first)

folder-delete-title = Ṣé kí a pa “{ $name }” rẹ́?
folder-delete-body = { $count ->
    [0] Kò sí lẹ́tà kankan nínú rẹ̀. A ó yọ fódà náà kúrò lórí sáfà, nítorí náà lẹ́tà wẹ́ẹ̀bù àti fóònù rẹ yóò pàdánù rẹ̀ pẹ̀lú.
   *[other] { $kind ->
        [conversation] Ìjíròrò { $count } inú rẹ̀ yóò lọ sí Ìdọ̀tí, nítorí náà o ṣì lè gbà wọ́n padà.
       *[message] Ìfiránṣẹ́ { $count } inú rẹ̀ yóò lọ sí Ìdọ̀tí, nítorí náà o ṣì lè gbà wọ́n padà.
    } A ó yọ fódà náà kúrò lórí sáfà, nítorí náà lẹ́tà wẹ́ẹ̀bù àti fóònù rẹ yóò pàdánù rẹ̀ pẹ̀lú.
}
folder-delete-forever-body = { $count ->
    [0] Kò sí lẹ́tà kankan nínú rẹ̀. A ó yọ fódà náà kúrò lórí sáfà, nítorí náà lẹ́tà wẹ́ẹ̀bù àti fóònù rẹ yóò pàdánù rẹ̀ pẹ̀lú.
   *[other] { $kind ->
        [conversation] Ìjíròrò { $count } inú rẹ̀ yóò di píparẹ́ títí láé; àkáǹtì yìí kò ní Ìdọ̀tí.
       *[message] Ìfiránṣẹ́ { $count } inú rẹ̀ yóò di píparẹ́ títí láé; àkáǹtì yìí kò ní Ìdọ̀tí.
    } A ó yọ fódà náà kúrò lórí sáfà, nítorí náà lẹ́tà wẹ́ẹ̀bù àti fóònù rẹ yóò pàdánù rẹ̀ pẹ̀lú.
}
folder-delete-label-body = A ó yọ àmì náà kúrò. Lẹ́tà rẹ̀ yóò wà nínú Gbogbo lẹ́tà àti nínú àwọn àmì rẹ̀ mìíràn.
folder-delete-confirm = Pa fódà rẹ́
folder-delete-label-confirm = Pa àmì rẹ́
folder-deleted = A ti pa fódà “{ $name }” rẹ́
label-deleted = A ti pa àmì “{ $name }” rẹ́
