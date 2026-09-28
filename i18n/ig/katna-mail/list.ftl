# Katna Mail, Igbo (Igbo).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Isi
tab-promotions = Nkwalite
tab-social = Mmekọrịta ọha
tab-updates = Mmelite
tab-forums = Ọgbakọ
tab-focused = Lekwasịrị anya
tab-other = Ndị ọzọ
tab-inbox = Igbe ozi mbata
tab-newsletters = Akwụkwọ akụkọ
tab-notifications = Ọkwa
tab-new = { $count } ọhụrụ
tab-provider-other = Katna haziri ya

## Mail list: toolbar

list-select = Họrọ
list-refresh = Mee ọhụrụ
list-checking = Na-elele ozi ọhụrụ…
list-more = Ọzọ
list-mark-read = Kaa akara dị ka agụrụ
list-mark-unread = Kaa akara dị ka a gụghị
list-move-to = Bugharịa gaa
list-archive = Chekwaa
list-spam = Kọọ dị ka spam
list-delete = Hichapụ
list-snooze = Yigharịa
list-unsnooze = Kagbuo iyigharị
list-newer = Nke ọhụrụ
list-older = Nke ochie
list-range = { $first }–{ $last } n'ime { $total }
list-range-about = { $first }–{ $last } n'ime ihe dị ka { $total }
list-results = Nsonaazụ maka “{ $query }”
list-results-corrected = Na-egosi nsonaazụ maka “{ $query }”
list-search-instead = Kama nke ahụ, chọọ “{ $query }”
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Niile
list-pick-none = Ọ dịghị
list-pick-read = Agụrụ
list-pick-unread = A gụghị
list-pick-starred = Nwere kpakpando
list-pick-unstarred = Enweghị kpakpando

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] Ahọrọla mkparịta ụka { $count } niile.
   *[message] Ahọrọla ozi { $count } niile.
}
list-selected-all-in = { $kind ->
    [conversation] Ahọrọla mkparịta ụka { $count } niile dị na { $folder }.
   *[message] Ahọrọla ozi { $count } niile dị na { $folder }.
}
list-selected-screen = { $kind ->
    [conversation] Ahọrọla mkparịta ụka { $count } niile dị na ihuenyo.
   *[message] Ahọrọla ozi { $count } niile dị na ihuenyo.
}
list-select-all = { $kind ->
    [conversation] Họrọ mkparịta ụka { $count } niile
   *[message] Họrọ ozi { $count } niile
}
list-select-all-in = { $kind ->
    [conversation] Họrọ mkparịta ụka { $count } niile dị na { $folder }
   *[message] Họrọ ozi { $count } niile dị na { $folder }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] Ahọrọla mkparịta ụka { $count } niile agụrụ dị na ihuenyo.
       *[message] Ahọrọla ozi { $count } niile agụrụ dị na ihuenyo.
    }
   *[unread] { $kind ->
        [conversation] Ahọrọla mkparịta ụka { $count } niile a gụghị dị na ihuenyo.
       *[message] Ahọrọla ozi { $count } niile a gụghị dị na ihuenyo.
    }
    [starred] { $kind ->
        [conversation] Ahọrọla mkparịta ụka { $count } niile nwere kpakpando dị na ihuenyo.
       *[message] Ahọrọla ozi { $count } niile nwere kpakpando dị na ihuenyo.
    }
    [unstarred] { $kind ->
        [conversation] Ahọrọla mkparịta ụka { $count } niile enweghị kpakpando dị na ihuenyo.
       *[message] Ahọrọla ozi { $count } niile enweghị kpakpando dị na ihuenyo.
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] Họrọ mkparịta ụka { $count } niile agụrụ
       *[message] Họrọ ozi { $count } niile agụrụ
    }
   *[unread] { $kind ->
        [conversation] Họrọ mkparịta ụka { $count } niile a gụghị
       *[message] Họrọ ozi { $count } niile a gụghị
    }
    [starred] { $kind ->
        [conversation] Họrọ mkparịta ụka { $count } niile nwere kpakpando
       *[message] Họrọ ozi { $count } niile nwere kpakpando
    }
    [unstarred] { $kind ->
        [conversation] Họrọ mkparịta ụka { $count } niile enweghị kpakpando
       *[message] Họrọ ozi { $count } niile enweghị kpakpando
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] Họrọ mkparịta ụka { $count } niile agụrụ dị na { $folder }
       *[message] Họrọ ozi { $count } niile agụrụ dị na { $folder }
    }
   *[unread] { $kind ->
        [conversation] Họrọ mkparịta ụka { $count } niile a gụghị dị na { $folder }
       *[message] Họrọ ozi { $count } niile a gụghị dị na { $folder }
    }
    [starred] { $kind ->
        [conversation] Họrọ mkparịta ụka { $count } niile nwere kpakpando dị na { $folder }
       *[message] Họrọ ozi { $count } niile nwere kpakpando dị na { $folder }
    }
    [unstarred] { $kind ->
        [conversation] Họrọ mkparịta ụka { $count } niile enweghị kpakpando dị na { $folder }
       *[message] Họrọ ozi { $count } niile enweghị kpakpando dị na { $folder }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] Ahọrọla mkparịta ụka { $count } niile agụrụ.
       *[message] Ahọrọla ozi { $count } niile agụrụ.
    }
   *[unread] { $kind ->
        [conversation] Ahọrọla mkparịta ụka { $count } niile a gụghị.
       *[message] Ahọrọla ozi { $count } niile a gụghị.
    }
    [starred] { $kind ->
        [conversation] Ahọrọla mkparịta ụka { $count } niile nwere kpakpando.
       *[message] Ahọrọla ozi { $count } niile nwere kpakpando.
    }
    [unstarred] { $kind ->
        [conversation] Ahọrọla mkparịta ụka { $count } niile enweghị kpakpando.
       *[message] Ahọrọla ozi { $count } niile enweghị kpakpando.
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] Ahọrọla mkparịta ụka { $count } niile agụrụ dị na { $folder }.
       *[message] Ahọrọla ozi { $count } niile agụrụ dị na { $folder }.
    }
   *[unread] { $kind ->
        [conversation] Ahọrọla mkparịta ụka { $count } niile a gụghị dị na { $folder }.
       *[message] Ahọrọla ozi { $count } niile a gụghị dị na { $folder }.
    }
    [starred] { $kind ->
        [conversation] Ahọrọla mkparịta ụka { $count } niile nwere kpakpando dị na { $folder }.
       *[message] Ahọrọla ozi { $count } niile nwere kpakpando dị na { $folder }.
    }
    [unstarred] { $kind ->
        [conversation] Ahọrọla mkparịta ụka { $count } niile enweghị kpakpando dị na { $folder }.
       *[message] Ahọrọla ozi { $count } niile enweghị kpakpando dị na { $folder }.
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Enweghị mkparịta ụka agụrụ ebe a.
       *[message] Enweghị ozi agụrụ ebe a.
    }
   *[unread] { $kind ->
        [conversation] Enweghị mkparịta ụka a gụghị ebe a.
       *[message] Enweghị ozi a gụghị ebe a.
    }
    [starred] { $kind ->
        [conversation] Enweghị mkparịta ụka nwere kpakpando ebe a.
       *[message] Enweghị ozi nwere kpakpando ebe a.
    }
    [unstarred] { $kind ->
        [conversation] Enweghị mkparịta ụka enweghị kpakpando ebe a.
       *[message] Enweghị ozi enweghị kpakpando ebe a.
    }
}
list-clear-selection = Kpochapụ nhọrọ

## Mail list: empty states

list-empty-search = Ọ dịghị ozi dabara na ọchụchọ gị.
list-empty-tab = Ọ dịghị ozi dị na { $tab }.
list-empty-tab-unknown = Ọ dịghị ozi dị na taabụ a.
list-empty-folder = Ọ dịghị ozi dị na { $folder }.
list-empty-folder-unknown = Ọ dịghị ozi dị na folda a.
list-first-sync = Na-enweta ozi gị…
list-first-sync-detail = Ha ga-apụta ebe a ka ha na-abata.

## Mail list: lines

row-removed = Ewepụla ozi a.
row-starred = Nwere kpakpando
row-not-starred = Enweghị kpakpando
row-important = Dị mkpa. Pịa ka ị kaa akara dị ka ọ dịghị mkpa.
row-mark-important = Kaa akara dị ka ọ dị mkpa
row-pinned = Akwụnyere n'elu
row-tracking-none = A na-esochi ya. E megheghị ya ka
row-tracking-opened = Mmadụ { $opened } n'ime { $recipients } mepere ya
row-tracking-clicked = Mmadụ { $opened } n'ime { $recipients } mepere ya, mmadụ { $clicked } soro njikọ
row-pin = Kwụnye n'elu
row-unpin = Wepụ n'elu
row-snoozed-until = E yigharịrị ruo { $when }

## Mail list: More menu and right-click menu

menu-reply = Zaa
menu-reply-all = Zaa mmadụ niile
menu-forward = Zigaa
menu-archive = Chekwaa
menu-delete = Hichapụ
menu-delete-forever = Hichapụ ruo mgbe ebighị ebi
menu-move-to-inbox = Bugharịa gaa Igbe ozi mbata
menu-spam = Kọọ dị ka spam
menu-not-spam = Ọ bụghị spam
menu-mark-read = Kaa akara dị ka agụrụ
menu-mark-unread = Kaa akara dị ka a gụghị
menu-mark-all-read = Kaa akara na niile dị ka agụrụ
menu-star = Tinye kpakpando
menu-unstar = Wepụ kpakpando
menu-important = Kaa akara dị ka ọ dị mkpa
menu-not-important = Kaa akara dị ka ọ dịghị mkpa
menu-pin = Kwụnye n'elu
menu-unpin = Wepụ n'elu
menu-snooze = Yigharịa
menu-unsnooze = Kagbuo iyigharị
menu-print-all = Bipụta niile
menu-new-window = Mepee na windo ọhụrụ
menu-move-to = Bugharịa gaa
menu-move-to-heading = Bugharịa gaa:
menu-find-from = Chọta ozi-e si n'aka { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] Echekwala mkparịta ụka { $count }.
   *[message] Echekwala ozi { $count }.
}
toast-trashed = { $kind ->
    [conversation] Ebugara mkparịta ụka { $count } na Ihe mkpofu.
   *[message] Ebugara ozi { $count } na Ihe mkpofu.
}
toast-moved = { $kind ->
    [conversation] Ebugharịala mkparịta ụka { $count }.
   *[message] Ebugharịala ozi { $count }.
}
toast-starred = { $kind ->
    [conversation] Etinyela kpakpando na mkparịta ụka { $count }.
   *[message] Etinyela kpakpando na ozi { $count }.
}
toast-unstarred = { $kind ->
    [conversation] Ewepụla kpakpando na mkparịta ụka { $count }.
   *[message] Ewepụla kpakpando na ozi { $count }.
}
toast-important = { $kind ->
    [conversation] Akaala akara na mkparịta ụka { $count } dị ka ọ dị mkpa.
   *[message] Akaala akara na ozi { $count } dị ka ọ dị mkpa.
}
toast-not-important = { $kind ->
    [conversation] Akaala akara na mkparịta ụka { $count } dị ka ọ dịghị mkpa.
   *[message] Akaala akara na ozi { $count } dị ka ọ dịghị mkpa.
}
toast-pinned = { $kind ->
    [conversation] Akwụnyela mkparịta ụka { $count } n'elu.
   *[message] Akwụnyela ozi { $count } n'elu.
}
toast-unpinned = { $kind ->
    [conversation] Ewepụla mkparịta ụka { $count } n'elu.
   *[message] Ewepụla ozi { $count } n'elu.
}
toast-snoozed = { $kind ->
    [conversation] E yigharịla mkparịta ụka { $count } ruo { $when }.
   *[message] E yigharịla ozi { $count } ruo { $when }.
}
toast-unsnoozed = { $kind ->
    [conversation] Mkparịta ụka { $count } alọghachila n'Igbe ozi mbata.
   *[message] Ozi { $count } alọghachila n'Igbe ozi mbata.
}
toast-spam = { $kind ->
    [conversation] Akọọla mkparịta ụka { $count } dị ka spam.
   *[message] Akọọla ozi { $count } dị ka spam.
}
toast-not-spam = { $kind ->
    [conversation] Akaala mkparịta ụka { $count } dị ka ndị na-abụghị spam ma bugaa ha n'igbe ozi mbata.
   *[message] Akaala ozi { $count } dị ka ndị na-abụghị spam ma bugaa ha n'igbe ozi mbata.
}
toast-deleted-forever = { $kind ->
    [conversation] Ehichapụla mkparịta ụka { $count } ruo mgbe ebighị ebi.
   *[message] Ehichapụla ozi { $count } ruo mgbe ebighị ebi.
}
toast-marked-read = { $kind ->
    [conversation] Akaala mkparịta ụka { $count } akara dị ka agụrụ.
   *[message] Akaala ozi { $count } akara dị ka agụrụ.
}
toast-marked-unread = { $kind ->
    [conversation] Akaala mkparịta ụka { $count } akara dị ka a gụghị.
   *[message] Akaala ozi { $count } akara dị ka a gụghị.
}
toast-undone = Emegharịala omume ahụ.
toast-nothing-to-undo = Enweghị ihe a ga-emegharị.
toast-cannot-undo-delete-forever = Enweghị ike iweghachi ozi ehichapụrụ ruo mgbe ebighị ebi.
toast-send-undone = Emegharịala izipu.
toast-too-late-to-undo-send = Oge agafeela imegharị ya: ezigalarịrị ozi ahụ.
toast-undo = Megharịa
toast-close = Mechie
toast-no-spam-folder = Akaụntụ a enweghị folda spam.
