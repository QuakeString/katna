# Katna Mail, Yoruba (Yorùbá): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Iṣẹ́ tuntun
tasks-all = Gbogbo iṣẹ́
tasks-today = Òní
tasks-upcoming = Tó ń bọ̀
tasks-starred = Àwọn tí a fi ìràwọ̀ sàmì sí
tasks-completed-view = Tí a parí
tasks-new-list = Ṣẹ̀dá àtòjọ tuntun
tasks-labels-heading = Àwọn àmì
tasks-on-this-computer = Lórí kọ̀ǹpútà yìí
tasks-my-tasks = Àwọn Iṣẹ́ Mi
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Wọlé lẹ́ẹ̀kan sí i láti fi àwọn iṣẹ́ hàn
tasks-account-signed-in = O ti wọlé sí { $address } lẹ́ẹ̀kan sí i. À ń mú àwọn iṣẹ́ rẹ wá…
tasks-account-sign-in-refused = { $provider } kò jẹ́ kí Katna wọlé. Gbìyànjú lẹ́ẹ̀kan sí i, kí o sì gba ààyè sí àwọn iṣẹ́ rẹ láàyè.
tasks-account-refused = Sáfà kò gba ọ̀rọ̀ aṣínà náà. Yahoo, iCloud, Zoho àti àwọn mìíràn nílò ọ̀rọ̀ aṣínà áàpù.
tasks-account-change-password = Yí ọ̀rọ̀ aṣínà padà
tasks-account-change-password-tooltip = Tẹ ọ̀rọ̀ aṣínà tuntun; Katna yóò ṣàyẹ̀wò rẹ̀ pẹ̀lú sáfà
tasks-account-not-enabled = A kò tíì tan ààyè iṣẹ́ fún Katna.
tasks-account-failed = A kò lè ka àwọn àtòjọ iṣẹ́.
# $reason is the server's own words, in English.
tasks-account-error = A kò lè ka àwọn àtòjọ iṣẹ́: { $reason }
tasks-account-none = A kò rí àtòjọ iṣẹ́ kankan
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = A kò rí àtòjọ iṣẹ́ kankan: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } máa ń fi iṣẹ́ hàn fún Katna tí ó wọlé pẹ̀lú { $provider } nìkan.
tasks-account-sign-in-with = Wọlé pẹ̀lú { $provider }
tasks-account-looking = À ń wá àwọn àtòjọ iṣẹ́…
tasks-account-try-again = Gbìyànjú lẹ́ẹ̀kan sí i
tasks-account-try-again-tooltip = Ṣàyẹ̀wò àwọn iṣẹ́ àkáǹtì yìí lẹ́ẹ̀kan sí i báyìí
tasks-account-fixing = À ń ṣiṣẹ́ lé e lórí…
tasks-list-name-placeholder = Orúkọ àtòjọ

## Lists and tasks

tasks-loading = À ń ka àwọn iṣẹ́ rẹ…
tasks-no-lists = Àwọn àtòjọ iṣẹ́ rẹ yóò hàn níbí.
tasks-search = Ṣàwárí iṣẹ́
tasks-search-none = Kò sí iṣẹ́ tó bá ìwádìí rẹ mu.
tasks-add = Fi iṣẹ́ kún un
tasks-title-placeholder = Àkọlé
tasks-add-step = Fi iṣẹ́ kékeré kún un
tasks-empty = Kò sí iṣẹ́ kankan síbẹ̀. Fi ọ̀kan kún un lókè.
tasks-starred-empty = Fi ìràwọ̀ sàmì sí iṣẹ́ kan láti rí i níbí.
tasks-label-empty = Kò sí iṣẹ́ tí a kò tíì parí pẹ̀lú àmì yìí.
tasks-today-empty = Kò sí ohun tó yẹ kí a ṣe lónìí.
tasks-completed-empty = Àwọn iṣẹ́ tí o bá parí yóò hàn síbí.
tasks-upcoming-add = Fi iṣẹ́ kan kún un fún { $day }
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = Láti inú lẹ́tà
tasks-from-note-quiet = Láti inú àkọsílẹ̀
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Tí ó ti kọjá àkókò
tasks-completed = { $count ->
   *[other] Tí a parí ({ $count })
}
tasks-list-options = Àwọn àṣàyàn àtòjọ
tasks-sort-by = Ṣètò nípa
tasks-sort-my-order = Ìtò mi
tasks-sort-date = Ọjọ́
tasks-sort-starred = Oní ìràwọ̀ láìpẹ́
tasks-sort-title = Àkọlé
tasks-rename-list = Tún orúkọ àtòjọ ṣe
tasks-delete-list = Pa àtòjọ rẹ́
tasks-mark-done = Ṣàmì sí i pé ó parí
tasks-mark-open = Ṣàmì sí i pé kò parí
tasks-star = Fi ìràwọ̀ sàmì sí i
tasks-unstar = Yọ ìràwọ̀ kúrò
tasks-edit-title = Ṣàtúnṣe àkọlé
tasks-details = Àlàyé
tasks-delete = Pa rẹ́
tasks-move-to = Gbé lọ sí { $list }
tasks-from-mail = Lẹ́tà
tasks-open-mail = Ṣí lẹ́tà
tasks-from-note = Àkọsílẹ̀
tasks-open-note = Ṣí àkọsílẹ̀ náà
tasks-note-gone = Àkọsílẹ̀ yẹn kò sí níbí mọ́.
tasks-no-subject = (kò sí àkọlé)

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
   *[other] A ti yan { $count }
}
tasks-select-clear = Pa àṣàyàn rẹ́
tasks-select-move = Gbé lọ sí àkójọ
tasks-select-date = Ṣètò ọjọ́
tasks-next-week = Ọ̀sẹ̀ tó ń bọ̀

## The details dialog

tasks-notes-placeholder = Fi àlàyé kún un
tasks-date = Ọjọ́
tasks-no-date = Kò sí ọjọ́
tasks-time-placeholder = Fi àkókò kún un
tasks-repeat = Tún ṣe
tasks-repeat-never = Kì í tún ṣe
tasks-repeat-daily = Lójoojúmọ́
tasks-repeat-weekly = Ní ọ̀sẹ̀ kọ̀ọ̀kan
tasks-repeat-monthly = Ní oṣù kọ̀ọ̀kan
tasks-repeat-yearly = Ní ọdún kọ̀ọ̀kan
tasks-repeat-other = Àdáni
tasks-remind = Rán mi létí
tasks-remind-off = Má ṣe rán mi létí
tasks-remind-on-time = Ní àkókò náà
tasks-remind-morning = Ní ọjọ́ náà, { $time }
tasks-remind-hour-before = Wákàtí kan ṣáájú
tasks-remind-day-before = Ọjọ́ kan ṣáájú
tasks-label-add = Fi àmì kún un
tasks-label-task = Fi àmì sí iṣẹ́
tasks-files-attach = So àwọn fáìlì mọ́ ọn
tasks-files-pick = So mọ́ ọn
tasks-file-open = Ṣí i
tasks-file-remove = Yọ fáìlì kúrò
tasks-file-here = Lórí kọ̀ǹpútà yìí nìkan
tasks-cancel = Fagilé
tasks-save = Fi pamọ́
tasks-not-a-time = “{ $text }” kì í ṣe àkókò, fún àpẹẹrẹ { $example }.

## Due days

tasks-due-today = Òní
tasks-due-tomorrow = Ọ̀la
tasks-due-yesterday = Àná
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Iṣẹ́ parí
tasks-toast-next = Ó parí. Tí ó kàn ni { $date }
tasks-toast-deleted = A pa iṣẹ́ náà rẹ́
tasks-files-added = { $count ->
   *[other] A ti so fáìlì { $count } mọ́ ọn
}
tasks-file-removed = A ti yọ “{ $name }” kúrò
tasks-files-left-out = A kò so wọ́n mọ́ ọn: { $names }. Iṣẹ́ kan ń gba fáìlì tí kò ju { $limit } lọ, kì í ṣe fódà.
tasks-file-missing = Fáìlì yẹn kò sí níbí mọ́.
tasks-toast-added = { $count ->
   *[other] A fi iṣẹ́ { $count } kún
}
tasks-mail-gone = Lẹ́tà yẹn kò sí níbí mọ́.
tasks-toast-list-deleted = A pa àtòjọ náà rẹ́
tasks-toast-moved = A gbé e lọ sí { $list }
# A task dragged to another place in its own list.
tasks-toast-placed = A gbé iṣẹ́ náà lọ
tasks-toast-rescheduled = A ti yí àkókò iṣẹ́ padà
tasks-toast-rescheduled-several = { $count ->
   *[other] A ti yí ọjọ́ iṣẹ́ { $count } padà
}
tasks-toast-done-several = { $count ->
   *[other] A ti parí iṣẹ́ { $count }
}
tasks-toast-open-several = { $count ->
   *[other] A ti sàmì sí iṣẹ́ { $count } bí èyí tí a kò tíì parí
}
tasks-toast-starred = { $count ->
   *[other] A ti fi ìràwọ̀ sí iṣẹ́ { $count }
}
tasks-toast-unstarred = { $count ->
   *[other] A ti yọ ìràwọ̀ kúrò lára iṣẹ́ { $count }
}
tasks-toast-deleted-several = { $count ->
   *[other] A ti pa iṣẹ́ { $count } rẹ́
}
