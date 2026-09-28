# Katna Mail, Yoruba (Yorùbá).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Àkọ́kọ́
tab-promotions = Ìpolówó
tab-social = Àwùjọ
tab-updates = Ìmúdójúìwọ̀n
tab-forums = Àpérò
tab-focused = Àfojúsùn
tab-other = Òmíràn
tab-inbox = Àpótí-ìwọlé
tab-newsletters = Ìwé ìròyìn
tab-notifications = Ìfitónilétí
tab-new = { $count } tuntun
tab-provider-other = Katna ló tò ó

## Mail list: toolbar

list-select = Yàn
list-refresh = Sọdọ̀tun
list-more = Síi
list-mark-read = Sàmì sí bí kíkà
list-mark-unread = Sàmì sí bí àìkà
list-move-to = Gbé lọ sí
list-archive = Fi pamọ́
list-spam = Jábọ̀ àwúrúju
list-delete = Pa rẹ́
list-snooze = Sún síwájú
list-unsnooze = Mú padà báyìí
list-newer = Tuntun
list-older = Àtijọ́
list-range = { $first }–{ $last } nínú { $total }
list-range-about = { $first }–{ $last } nínú bí { $total }
list-results = Àbájáde fún “{ $query }”
list-results-corrected = Ó ń fi àbájáde hàn fún “{ $query }”
list-search-instead = Ṣàwárí “{ $query }” dípò
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Gbogbo
list-pick-none = Kò sí
list-pick-read = Kíkà
list-pick-unread = Àìkà
list-pick-starred = Oní ìràwọ̀
list-pick-unstarred = Aláìní ìràwọ̀

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] A ti yan gbogbo ìjíròrò { $count }.
   *[message] A ti yan gbogbo ìfiránṣẹ́ { $count }.
}
list-selected-all-in = { $kind ->
    [conversation] A ti yan gbogbo ìjíròrò { $count } nínú { $folder }.
   *[message] A ti yan gbogbo ìfiránṣẹ́ { $count } nínú { $folder }.
}
list-selected-screen = { $kind ->
    [conversation] A ti yan gbogbo ìjíròrò { $count } lójú ìbòjú.
   *[message] A ti yan gbogbo ìfiránṣẹ́ { $count } lójú ìbòjú.
}
list-select-all = { $kind ->
    [conversation] Yan gbogbo ìjíròrò { $count }
   *[message] Yan gbogbo ìfiránṣẹ́ { $count }
}
list-select-all-in = { $kind ->
    [conversation] Yan gbogbo ìjíròrò { $count } nínú { $folder }
   *[message] Yan gbogbo ìfiránṣẹ́ { $count } nínú { $folder }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] A ti yan gbogbo ìjíròrò { $count } tí a ti kà lójú ìbòjú.
       *[message] A ti yan gbogbo ìfiránṣẹ́ { $count } tí a ti kà lójú ìbòjú.
    }
   *[unread] { $kind ->
        [conversation] A ti yan gbogbo ìjíròrò { $count } tí a kò tíì kà lójú ìbòjú.
       *[message] A ti yan gbogbo ìfiránṣẹ́ { $count } tí a kò tíì kà lójú ìbòjú.
    }
    [starred] { $kind ->
        [conversation] A ti yan gbogbo ìjíròrò { $count } oní ìràwọ̀ lójú ìbòjú.
       *[message] A ti yan gbogbo ìfiránṣẹ́ { $count } oní ìràwọ̀ lójú ìbòjú.
    }
    [unstarred] { $kind ->
        [conversation] A ti yan gbogbo ìjíròrò { $count } aláìní ìràwọ̀ lójú ìbòjú.
       *[message] A ti yan gbogbo ìfiránṣẹ́ { $count } aláìní ìràwọ̀ lójú ìbòjú.
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] Yan gbogbo ìjíròrò { $count } tí a ti kà
       *[message] Yan gbogbo ìfiránṣẹ́ { $count } tí a ti kà
    }
   *[unread] { $kind ->
        [conversation] Yan gbogbo ìjíròrò { $count } tí a kò tíì kà
       *[message] Yan gbogbo ìfiránṣẹ́ { $count } tí a kò tíì kà
    }
    [starred] { $kind ->
        [conversation] Yan gbogbo ìjíròrò { $count } oní ìràwọ̀
       *[message] Yan gbogbo ìfiránṣẹ́ { $count } oní ìràwọ̀
    }
    [unstarred] { $kind ->
        [conversation] Yan gbogbo ìjíròrò { $count } aláìní ìràwọ̀
       *[message] Yan gbogbo ìfiránṣẹ́ { $count } aláìní ìràwọ̀
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] Yan gbogbo ìjíròrò { $count } tí a ti kà nínú { $folder }
       *[message] Yan gbogbo ìfiránṣẹ́ { $count } tí a ti kà nínú { $folder }
    }
   *[unread] { $kind ->
        [conversation] Yan gbogbo ìjíròrò { $count } tí a kò tíì kà nínú { $folder }
       *[message] Yan gbogbo ìfiránṣẹ́ { $count } tí a kò tíì kà nínú { $folder }
    }
    [starred] { $kind ->
        [conversation] Yan gbogbo ìjíròrò { $count } oní ìràwọ̀ nínú { $folder }
       *[message] Yan gbogbo ìfiránṣẹ́ { $count } oní ìràwọ̀ nínú { $folder }
    }
    [unstarred] { $kind ->
        [conversation] Yan gbogbo ìjíròrò { $count } aláìní ìràwọ̀ nínú { $folder }
       *[message] Yan gbogbo ìfiránṣẹ́ { $count } aláìní ìràwọ̀ nínú { $folder }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] A ti yan gbogbo ìjíròrò { $count } tí a ti kà.
       *[message] A ti yan gbogbo ìfiránṣẹ́ { $count } tí a ti kà.
    }
   *[unread] { $kind ->
        [conversation] A ti yan gbogbo ìjíròrò { $count } tí a kò tíì kà.
       *[message] A ti yan gbogbo ìfiránṣẹ́ { $count } tí a kò tíì kà.
    }
    [starred] { $kind ->
        [conversation] A ti yan gbogbo ìjíròrò { $count } oní ìràwọ̀.
       *[message] A ti yan gbogbo ìfiránṣẹ́ { $count } oní ìràwọ̀.
    }
    [unstarred] { $kind ->
        [conversation] A ti yan gbogbo ìjíròrò { $count } aláìní ìràwọ̀.
       *[message] A ti yan gbogbo ìfiránṣẹ́ { $count } aláìní ìràwọ̀.
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] A ti yan gbogbo ìjíròrò { $count } tí a ti kà nínú { $folder }.
       *[message] A ti yan gbogbo ìfiránṣẹ́ { $count } tí a ti kà nínú { $folder }.
    }
   *[unread] { $kind ->
        [conversation] A ti yan gbogbo ìjíròrò { $count } tí a kò tíì kà nínú { $folder }.
       *[message] A ti yan gbogbo ìfiránṣẹ́ { $count } tí a kò tíì kà nínú { $folder }.
    }
    [starred] { $kind ->
        [conversation] A ti yan gbogbo ìjíròrò { $count } oní ìràwọ̀ nínú { $folder }.
       *[message] A ti yan gbogbo ìfiránṣẹ́ { $count } oní ìràwọ̀ nínú { $folder }.
    }
    [unstarred] { $kind ->
        [conversation] A ti yan gbogbo ìjíròrò { $count } aláìní ìràwọ̀ nínú { $folder }.
       *[message] A ti yan gbogbo ìfiránṣẹ́ { $count } aláìní ìràwọ̀ nínú { $folder }.
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Kò sí ìjíròrò tí a ti kà níbí.
       *[message] Kò sí ìfiránṣẹ́ tí a ti kà níbí.
    }
   *[unread] { $kind ->
        [conversation] Kò sí ìjíròrò tí a kò tíì kà níbí.
       *[message] Kò sí ìfiránṣẹ́ tí a kò tíì kà níbí.
    }
    [starred] { $kind ->
        [conversation] Kò sí ìjíròrò oní ìràwọ̀ níbí.
       *[message] Kò sí ìfiránṣẹ́ oní ìràwọ̀ níbí.
    }
    [unstarred] { $kind ->
        [conversation] Kò sí ìjíròrò aláìní ìràwọ̀ níbí.
       *[message] Kò sí ìfiránṣẹ́ aláìní ìràwọ̀ níbí.
    }
}
list-clear-selection = Pa àṣàyàn rẹ́

## Mail list: empty states

list-empty-search = Kò sí ìfiránṣẹ́ tó bá àwárí rẹ mu.
list-empty-tab = Kò sí lẹ́tà nínú { $tab }.
list-empty-tab-unknown = Kò sí lẹ́tà nínú táàbù yìí.
list-empty-folder = Kò sí ìfiránṣẹ́ nínú { $folder }.
list-empty-folder-unknown = Kò sí ìfiránṣẹ́ nínú fódà yìí.
list-first-sync = À ń gba lẹ́tà rẹ…
list-first-sync-detail = Wọn yóò hàn níbí bí wọ́n ṣe ń dé.

## Mail list: lines

row-removed = A ti yọ ìfiránṣẹ́ yìí kúrò.
row-starred = Oní ìràwọ̀
row-not-starred = Aláìní ìràwọ̀
row-important = Pàtàkì. Tẹ̀ ẹ́ láti sàmì sí bí kò ṣe pàtàkì.
row-mark-important = Sàmì sí bí pàtàkì
row-pinned = A ti lẹ̀ ẹ́ mọ́ òkè
row-tracking-none = A ń tọpa rẹ̀. Kò tíì sí ẹni tó ṣí i
row-tracking-opened = { $opened } nínú { $recipients } ló ṣí i
row-tracking-clicked = { $opened } nínú { $recipients } ló ṣí i, { $clicked } ló tẹ̀lé ìjápọ̀ kan
row-pin = Lẹ̀ mọ́ òkè
row-unpin = Yọ kúrò ní òkè
row-snoozed-until = A sún un síwájú di { $when }

## Mail list: More menu and right-click menu

menu-reply = Fèsì
menu-reply-all = Fèsì sí gbogbo
menu-forward = Fi ránṣẹ́ síwájú
menu-archive = Fi pamọ́
menu-delete = Pa rẹ́
menu-delete-forever = Pa rẹ́ títí láé
menu-move-to-inbox = Gbé lọ sí Àpótí-ìwọlé
menu-spam = Jábọ̀ àwúrúju
menu-not-spam = Kì í ṣe àwúrúju
menu-mark-read = Sàmì sí bí kíkà
menu-mark-unread = Sàmì sí bí àìkà
menu-mark-all-read = Sàmì sí gbogbo rẹ̀ bí kíkà
menu-star = Fi ìràwọ̀ sí
menu-unstar = Yọ ìràwọ̀ kúrò
menu-important = Sàmì sí bí pàtàkì
menu-not-important = Sàmì sí bí kò ṣe pàtàkì
menu-pin = Lẹ̀ mọ́ òkè
menu-unpin = Yọ kúrò ní òkè
menu-snooze = Sún síwájú
menu-unsnooze = Mú padà báyìí
menu-print-all = Tẹ gbogbo rẹ̀ jáde
menu-new-window = Ṣí ní fèrèsé tuntun
menu-move-to = Gbé lọ sí
menu-move-to-heading = Gbé lọ sí:
menu-find-from = Wá àwọn ímeèlì láti ọ̀dọ̀ { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] A ti fi ìjíròrò { $count } pamọ́.
   *[message] A ti fi ìfiránṣẹ́ { $count } pamọ́.
}
toast-trashed = { $kind ->
    [conversation] A ti gbé ìjíròrò { $count } lọ sí Ìdọ̀tí.
   *[message] A ti gbé ìfiránṣẹ́ { $count } lọ sí Ìdọ̀tí.
}
toast-moved = { $kind ->
    [conversation] A ti gbé ìjíròrò { $count } lọ.
   *[message] A ti gbé ìfiránṣẹ́ { $count } lọ.
}
toast-starred = { $kind ->
    [conversation] A ti fi ìràwọ̀ sí ìjíròrò { $count }.
   *[message] A ti fi ìràwọ̀ sí ìfiránṣẹ́ { $count }.
}
toast-unstarred = { $kind ->
    [conversation] A ti yọ ìràwọ̀ kúrò lára ìjíròrò { $count }.
   *[message] A ti yọ ìràwọ̀ kúrò lára ìfiránṣẹ́ { $count }.
}
toast-important = { $kind ->
    [conversation] A ti sàmì sí ìjíròrò { $count } bí pàtàkì.
   *[message] A ti sàmì sí ìfiránṣẹ́ { $count } bí pàtàkì.
}
toast-not-important = { $kind ->
    [conversation] A ti sàmì sí ìjíròrò { $count } bí kò ṣe pàtàkì.
   *[message] A ti sàmì sí ìfiránṣẹ́ { $count } bí kò ṣe pàtàkì.
}
toast-pinned = { $kind ->
    [conversation] A ti lẹ ìjíròrò { $count } mọ́ òkè.
   *[message] A ti lẹ ìfiránṣẹ́ { $count } mọ́ òkè.
}
toast-unpinned = { $kind ->
    [conversation] A ti yọ ìjíròrò { $count } kúrò ní òkè.
   *[message] A ti yọ ìfiránṣẹ́ { $count } kúrò ní òkè.
}
toast-snoozed = { $kind ->
    [conversation] A ti sún ìjíròrò { $count } síwájú di { $when }.
   *[message] A ti sún ìfiránṣẹ́ { $count } síwájú di { $when }.
}
toast-unsnoozed = { $kind ->
    [conversation] Ìjíròrò { $count } ti padà sí Àpótí-ìwọlé.
   *[message] Ìfiránṣẹ́ { $count } ti padà sí Àpótí-ìwọlé.
}
toast-spam = { $kind ->
    [conversation] A ti jábọ̀ ìjíròrò { $count } bí àwúrúju.
   *[message] A ti jábọ̀ ìfiránṣẹ́ { $count } bí àwúrúju.
}
toast-not-spam = { $kind ->
    [conversation] A ti sàmì sí ìjíròrò { $count } pé kì í ṣe àwúrúju, a sì ti gbé wọn lọ sí àpótí-ìwọlé.
   *[message] A ti sàmì sí ìfiránṣẹ́ { $count } pé kì í ṣe àwúrúju, a sì ti gbé wọn lọ sí àpótí-ìwọlé.
}
toast-deleted-forever = { $kind ->
    [conversation] A ti pa ìjíròrò { $count } rẹ́ títí láé.
   *[message] A ti pa ìfiránṣẹ́ { $count } rẹ́ títí láé.
}
toast-marked-read = { $kind ->
    [conversation] A ti sàmì sí ìjíròrò { $count } bí kíkà.
   *[message] A ti sàmì sí ìfiránṣẹ́ { $count } bí kíkà.
}
toast-marked-unread = { $kind ->
    [conversation] A ti sàmì sí ìjíròrò { $count } bí àìkà.
   *[message] A ti sàmì sí ìfiránṣẹ́ { $count } bí àìkà.
}
toast-undone = A ti dá ìgbésẹ̀ náà padà.
toast-nothing-to-undo = Kò sí nǹkan láti dá padà.
toast-cannot-undo-delete-forever = Lẹ́tà tí a ti pa rẹ́ títí láé kò ṣeé mú padà.
toast-send-undone = A ti dá fífiránṣẹ́ padà.
toast-too-late-to-undo-send = Ó ti pẹ́ jù láti dá a padà: a ti fi ìfiránṣẹ́ náà ránṣẹ́ tán.
toast-undo = Dá padà
toast-close = Pa á dé
toast-no-spam-folder = Àkáǹtì yìí kò ní fódà àwúrúju.
