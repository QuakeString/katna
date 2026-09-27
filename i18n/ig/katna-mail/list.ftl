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
list-more = Ọzọ
list-mark-read = Kaa akara dị ka agụrụ
list-mark-unread = Kaa akara dị ka a gụghị
list-move-to = Bugharịa gaa
list-archive = Chekwaa
list-spam = Kọọ dị ka spam
list-delete = Hichapụ
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
row-pin = Kwụnye n'elu
row-unpin = Wepụ n'elu

## Mail list: More menu and right-click menu

menu-reply = Zaa
menu-reply-all = Zaa mmadụ niile
menu-forward = Zigaa
menu-archive = Chekwaa
menu-delete = Hichapụ
menu-spam = Kọọ dị ka spam
menu-mark-read = Kaa akara dị ka agụrụ
menu-mark-unread = Kaa akara dị ka a gụghị
menu-mark-all-read = Kaa akara na niile dị ka agụrụ
menu-star = Tinye kpakpando
menu-unstar = Wepụ kpakpando
menu-important = Kaa akara dị ka ọ dị mkpa
menu-not-important = Kaa akara dị ka ọ dịghị mkpa
menu-pin = Kwụnye n'elu
menu-unpin = Wepụ n'elu
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
toast-spam = { $kind ->
    [conversation] Akọọla mkparịta ụka { $count } dị ka spam.
   *[message] Akọọla ozi { $count } dị ka spam.
}
toast-deleted-forever = { $kind ->
    [conversation] Ehichapụla mkparịta ụka { $count } ruo mgbe ebighị ebi.
   *[message] Ehichapụla ozi { $count } ruo mgbe ebighị ebi.
}
toast-undone = Emegharịala omume ahụ.
toast-undo = Megharịa
toast-no-spam-folder = Akaụntụ a enweghị folda spam.
