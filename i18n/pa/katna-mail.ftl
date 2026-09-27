# Katna Mail, Punjabi (ਪੰਜਾਬੀ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = ਭਾਸ਼ਾ: { $language }
language-tooltip-system = ਭਾਸ਼ਾ: { $language }, ਸਿਸਟਮ ਮੁਤਾਬਕ
language-search = ਭਾਸ਼ਾ ਖੋਜੋ
language-system-default = ਸਿਸਟਮ ਪੂਰਵ-ਨਿਰਧਾਰਿਤ
language-system-now = ਹੁਣ { $language }
language-no-match = “{ $query }” ਨਾਲ ਮੇਲ ਖਾਂਦੀ ਕੋਈ ਭਾਸ਼ਾ ਨਹੀਂ
language-machine = ਮਸ਼ੀਨ ਵੱਲੋਂ ਅਨੁਵਾਦਿਤ। ਇਸਨੂੰ ਬਿਹਤਰ ਬਣਾਉਣ ਵਿੱਚ ਮਦਦ ਕਰੋ
language-setting = ਭਾਸ਼ਾ
language-setting-detail = ਮੀਨੂ, ਬਟਨਾਂ ਅਤੇ ਸੁਨੇਹਿਆਂ ਦੀ ਭਾਸ਼ਾ, ਅਤੇ ਤਾਰੀਖਾਂ ਤੇ ਨੰਬਰਾਂ ਦਾ ਫ਼ਾਰਮੈਟ। ਸਿਸਟਮ ਪੂਰਵ-ਨਿਰਧਾਰਿਤ ਡੈਸਕਟਾਪ ਦੀ ਸੈਟਿੰਗ ਮੁਤਾਬਕ ਚੱਲਦਾ ਹੈ।

## Dates and sizes

ago-just-now = ਹੁਣੇ ਹੀ
ago-minutes = { $count ->
    [one] { $count } ਮਿੰਟ ਪਹਿਲਾਂ
   *[other] { $count } ਮਿੰਟ ਪਹਿਲਾਂ
}
ago-hours = { $count ->
    [one] { $count } ਘੰਟਾ ਪਹਿਲਾਂ
   *[other] { $count } ਘੰਟੇ ਪਹਿਲਾਂ
}
ago-days = { $count ->
    [one] { $count } ਦਿਨ ਪਹਿਲਾਂ
   *[other] { $count } ਦਿਨ ਪਹਿਲਾਂ
}
size-bytes = { $count ->
    [one] { $count } ਬਾਈਟ
   *[other] { $count } ਬਾਈਟ
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = ਫੋਲਡਰ ਲੁਕਾਓ
folders-show = ਫੋਲਡਰ ਦਿਖਾਓ
compose = ਲਿਖੋ
search = ਖੋਜੋ
search-mail = ਮੇਲ ਖੋਜੋ
search-settings = ਸੈਟਿੰਗਾਂ ਖੋਜੋ
search-clear = ਖੋਜ ਸਾਫ਼ ਕਰੋ
search-options-show = ਖੋਜ ਵਿਕਲਪ ਦਿਖਾਓ
settings = ਸੈਟਿੰਗਾਂ
account-add = ਖਾਤਾ ਸ਼ਾਮਲ ਕਰੋ

## App rail (and the bottom bar on a phone)

rail-mail = ਮੇਲ
rail-calendar = ਕੈਲੰਡਰ
rail-contacts = ਸੰਪਰਕ
rail-tasks = ਕਾਰਜ
rail-notes = ਨੋਟ
rail-feeds = ਫ਼ੀਡਾਂ

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = ਜਲਦੀ ਆ ਰਿਹਾ ਹੈ
app-calendar-promise = ਤੁਹਾਡੇ CalDAV ਕੈਲੰਡਰ, ਤੁਹਾਡੀ ਮੇਲ ਵਿੱਚ ਆਏ ਮੀਟਿੰਗ ਸੱਦੇ ਅਤੇ ਰੀਮਾਈਂਡਰ, ਤੁਹਾਡੇ ਇਨਬਾਕਸ ਦੇ ਨਾਲ ਹੀ।
app-tasks-promise = CalDAV ਨਾਲ ਸਿੰਕ ਹੋਣ ਵਾਲੀਆਂ ਕਰਨਯੋਗ ਸੂਚੀਆਂ, ਅਤੇ ਮੇਲ ਤੋਂ ਬਣਾਏ ਕਾਰਜ।
app-notes-promise = ਝਟਪਟ ਨੋਟ, ਅਤੇ ਬਾਅਦ ਲਈ ਕਿਸੇ ਮੇਲ ਜਾਂ ਗੱਲਬਾਤ ਬਾਰੇ ਨੋਟ।
app-feeds-promise = ਆਪਣੀ ਮੇਲ ਦੇ ਨਾਲ ਹੀ RSS ਅਤੇ Atom ਫ਼ੀਡਾਂ ਪੜ੍ਹੋ।

## Contacts page

app-contacts-loading = ਤੁਹਾਡੀ ਮੇਲ ਵਿੱਚੋਂ ਲੋਕਾਂ ਨੂੰ ਇਕੱਠਾ ਕੀਤਾ ਜਾ ਰਿਹਾ ਹੈ…
app-contacts-empty = ਜਿਨ੍ਹਾਂ ਲੋਕਾਂ ਨਾਲ ਤੁਸੀਂ ਮੇਲ ਰਾਹੀਂ ਗੱਲ ਕਰਦੇ ਹੋ, ਉਹ ਇੱਥੇ ਦਿਖਾਈ ਦੇਣਗੇ।
app-contacts-count = { $count ->
    [one] ਤੁਹਾਡੀ ਮੇਲ ਵਿੱਚੋਂ { $count } ਵਿਅਕਤੀ, ਸਭ ਤੋਂ ਵੱਧ ਮੇਲ ਵਾਲੇ ਪਹਿਲਾਂ
   *[other] ਤੁਹਾਡੀ ਮੇਲ ਵਿੱਚੋਂ { $count } ਲੋਕ, ਸਭ ਤੋਂ ਵੱਧ ਮੇਲ ਵਾਲੇ ਪਹਿਲਾਂ
}
app-contacts-top = { $count ->
    [one] ਤੁਹਾਡੀ ਮੇਲ ਵਿੱਚੋਂ ਸਿਖਰਲਾ { $count } ਵਿਅਕਤੀ, ਸਭ ਤੋਂ ਵੱਧ ਮੇਲ ਵਾਲੇ ਪਹਿਲਾਂ
   *[other] ਤੁਹਾਡੀ ਮੇਲ ਵਿੱਚੋਂ ਸਿਖਰਲੇ { $count } ਲੋਕ, ਸਭ ਤੋਂ ਵੱਧ ਮੇਲ ਵਾਲੇ ਪਹਿਲਾਂ
}
app-contacts-messages = { $count ->
    [one] { $count } ਸੁਨੇਹਾ
   *[other] { $count } ਸੁਨੇਹੇ
}
app-contacts-last = ਆਖਰੀ ਵਾਰ { $date }

## Navigation (the folders pane)

nav-labels = ਲੇਬਲ
nav-folders = ਫੋਲਡਰ
nav-label-new = ਨਵਾਂ ਲੇਬਲ ਬਣਾਓ
nav-folder-new = ਨਵਾਂ ਫੋਲਡਰ ਬਣਾਓ
nav-account-unnamed = ਖਾਤਾ { $number }
nav-tab-new = { $count ->
    [one] { $count } ਨਵਾਂ
   *[other] { $count } ਨਵੇਂ
}

## Special folders (the user's own folders keep their names)

folder-inbox = ਇਨਬਾਕਸ
folder-starred = ਤਾਰਾਬੱਧ
folder-drafts = ਡਰਾਫਟ
folder-sent = ਭੇਜੀਆਂ ਗਈਆਂ
folder-archive = ਪੁਰਾਲੇਖ
folder-spam = ਸਪੈਮ
folder-trash = ਰੱਦੀ
folder-all-mail = ਸਾਰੀਆਂ ਮੇਲਾਂ
folder-scheduled = ਅਨੁਸੂਚਿਤ

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = ਨਵਾਂ ਲੇਬਲ
label-folder-new-title = ਨਵਾਂ ਫੋਲਡਰ
label-prompt = ਕਿਰਪਾ ਕਰਕੇ ਨਵੇਂ ਲੇਬਲ ਦਾ ਨਾਮ ਦਾਖਲ ਕਰੋ:
label-folder-prompt = ਕਿਰਪਾ ਕਰਕੇ ਨਵੇਂ ਫੋਲਡਰ ਦਾ ਨਾਮ ਦਾਖਲ ਕਰੋ:
label-name-hint = ਲੇਬਲ ਦਾ ਨਾਮ
label-folder-name-hint = ਫੋਲਡਰ ਦਾ ਨਾਮ
label-nest = ਲੇਬਲ ਨੂੰ ਇਸ ਹੇਠਾਂ ਰੱਖੋ:
label-folder-nest = ਫੋਲਡਰ ਨੂੰ ਇਸ ਹੇਠਾਂ ਰੱਖੋ:
label-cancel = ਰੱਦ ਕਰੋ
label-create = ਬਣਾਓ
label-creating = ਬਣਾਇਆ ਜਾ ਰਿਹਾ ਹੈ…
label-created = “{ $name }” ਲੇਬਲ ਬਣਾਇਆ ਗਿਆ।
label-folder-created = “{ $name }” ਫੋਲਡਰ ਬਣਾਇਆ ਗਿਆ।

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = ਮੁੱਖ
tab-promotions = ਪ੍ਰੋਮੋਸ਼ਨ
tab-social = ਸੋਸ਼ਲ
tab-updates = ਅੱਪਡੇਟ
tab-forums = ਫੋਰਮ
tab-focused = ਕੇਂਦਰਿਤ
tab-other = ਹੋਰ
tab-inbox = ਇਨਬਾਕਸ
tab-newsletters = ਨਿਊਜ਼ਲੈਟਰ
tab-notifications = ਸੂਚਨਾਵਾਂ
tab-new = { $count } ਨਵੇਂ
tab-provider-other = Katna ਵੱਲੋਂ ਛਾਂਟਿਆ ਗਿਆ

## Mail list: toolbar

list-select = ਚੁਣੋ
list-refresh = ਤਾਜ਼ਾ ਕਰੋ
list-more = ਹੋਰ
list-mark-read = ਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
list-mark-unread = ਅਣਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
list-move-to = ਇੱਥੇ ਭੇਜੋ
list-archive = ਪੁਰਾਲੇਖਬੱਧ ਕਰੋ
list-spam = ਸਪੈਮ ਦੀ ਰਿਪੋਰਟ ਕਰੋ
list-delete = ਮਿਟਾਓ
list-newer = ਨਵੀਆਂ
list-older = ਪੁਰਾਣੀਆਂ
list-range = { $total } ਵਿੱਚੋਂ { $first }–{ $last }
list-range-about = ਲਗਭਗ { $total } ਵਿੱਚੋਂ { $first }–{ $last }
list-results = “{ $query }” ਲਈ ਨਤੀਜੇ
list-results-corrected = “{ $query }” ਲਈ ਨਤੀਜੇ ਦਿਖਾਏ ਜਾ ਰਹੇ ਹਨ
list-search-instead = ਇਸਦੀ ਬਜਾਏ “{ $query }” ਲਈ ਖੋਜੋ
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = ਸਭ
list-pick-none = ਕੋਈ ਨਹੀਂ
list-pick-read = ਪੜ੍ਹੀਆਂ
list-pick-unread = ਅਣਪੜ੍ਹੀਆਂ
list-pick-starred = ਤਾਰਾਬੱਧ
list-pick-unstarred = ਤਾਰਾ-ਰਹਿਤ

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] ਸਾਰੀ { $count } ਗੱਲਬਾਤ ਚੁਣੀ ਗਈ ਹੈ।
       *[other] ਸਾਰੀਆਂ { $count } ਗੱਲਬਾਤਾਂ ਚੁਣੀਆਂ ਗਈਆਂ ਹਨ।
    }
   *[message] { $count ->
        [one] { $count } ਸੁਨੇਹਾ ਚੁਣਿਆ ਗਿਆ ਹੈ।
       *[other] ਸਾਰੇ { $count } ਸੁਨੇਹੇ ਚੁਣੇ ਗਏ ਹਨ।
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } ਵਿਚਲੀ { $count } ਗੱਲਬਾਤ ਚੁਣੀ ਗਈ ਹੈ।
       *[other] { $folder } ਵਿਚਲੀਆਂ ਸਾਰੀਆਂ { $count } ਗੱਲਬਾਤਾਂ ਚੁਣੀਆਂ ਗਈਆਂ ਹਨ।
    }
   *[message] { $count ->
        [one] { $folder } ਵਿਚਲਾ { $count } ਸੁਨੇਹਾ ਚੁਣਿਆ ਗਿਆ ਹੈ।
       *[other] { $folder } ਵਿਚਲੇ ਸਾਰੇ { $count } ਸੁਨੇਹੇ ਚੁਣੇ ਗਏ ਹਨ।
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] ਸਕ੍ਰੀਨ ਉੱਤੇ { $count } ਗੱਲਬਾਤ ਚੁਣੀ ਗਈ ਹੈ।
       *[other] ਸਕ੍ਰੀਨ ਉੱਤੇ ਸਾਰੀਆਂ { $count } ਗੱਲਬਾਤਾਂ ਚੁਣੀਆਂ ਗਈਆਂ ਹਨ।
    }
   *[message] { $count ->
        [one] ਸਕ੍ਰੀਨ ਉੱਤੇ { $count } ਸੁਨੇਹਾ ਚੁਣਿਆ ਗਿਆ ਹੈ।
       *[other] ਸਕ੍ਰੀਨ ਉੱਤੇ ਸਾਰੇ { $count } ਸੁਨੇਹੇ ਚੁਣੇ ਗਏ ਹਨ।
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } ਗੱਲਬਾਤ ਚੁਣੋ
       *[other] ਸਾਰੀਆਂ { $count } ਗੱਲਬਾਤਾਂ ਚੁਣੋ
    }
   *[message] { $count ->
        [one] { $count } ਸੁਨੇਹਾ ਚੁਣੋ
       *[other] ਸਾਰੇ { $count } ਸੁਨੇਹੇ ਚੁਣੋ
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } ਵਿਚਲੀ { $count } ਗੱਲਬਾਤ ਚੁਣੋ
       *[other] { $folder } ਵਿਚਲੀਆਂ ਸਾਰੀਆਂ { $count } ਗੱਲਬਾਤਾਂ ਚੁਣੋ
    }
   *[message] { $count ->
        [one] { $folder } ਵਿਚਲਾ { $count } ਸੁਨੇਹਾ ਚੁਣੋ
       *[other] { $folder } ਵਿਚਲੇ ਸਾਰੇ { $count } ਸੁਨੇਹੇ ਚੁਣੋ
    }
}
list-clear-selection = ਚੋਣ ਸਾਫ਼ ਕਰੋ

## Mail list: empty states

list-empty-search = ਤੁਹਾਡੀ ਖੋਜ ਨਾਲ ਕੋਈ ਸੁਨੇਹਾ ਮੇਲ ਨਹੀਂ ਖਾਂਦਾ।
list-empty-tab = { $tab } ਵਿੱਚ ਕੋਈ ਮੇਲ ਨਹੀਂ।
list-empty-tab-unknown = ਇਸ ਟੈਬ ਵਿੱਚ ਕੋਈ ਮੇਲ ਨਹੀਂ।
list-empty-folder = { $folder } ਵਿੱਚ ਕੋਈ ਸੁਨੇਹਾ ਨਹੀਂ।
list-empty-folder-unknown = ਇਸ ਫੋਲਡਰ ਵਿੱਚ ਕੋਈ ਸੁਨੇਹਾ ਨਹੀਂ।
list-first-sync = ਤੁਹਾਡੀ ਮੇਲ ਲਿਆਂਦੀ ਜਾ ਰਹੀ ਹੈ…
list-first-sync-detail = ਜਿਵੇਂ-ਜਿਵੇਂ ਇਹ ਆਵੇਗੀ, ਇੱਥੇ ਦਿਖਾਈ ਦੇਵੇਗੀ।

## Mail list: lines

row-removed = ਇਹ ਸੁਨੇਹਾ ਹਟਾ ਦਿੱਤਾ ਗਿਆ ਸੀ।
row-starred = ਤਾਰਾਬੱਧ
row-not-starred = ਤਾਰਾਬੱਧ ਨਹੀਂ
row-important = ਮਹੱਤਵਪੂਰਨ। ਗੈਰ-ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰਨ ਲਈ ਕਲਿੱਕ ਕਰੋ।
row-mark-important = ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
row-pinned = ਸਿਖਰ ’ਤੇ ਪਿੰਨ ਕੀਤਾ ਗਿਆ
row-pin = ਸਿਖਰ ’ਤੇ ਪਿੰਨ ਕਰੋ
row-unpin = ਅਣਪਿੰਨ ਕਰੋ

## Mail list: More menu and right-click menu

menu-reply = ਜਵਾਬ ਦਿਓ
menu-reply-all = ਸਭ ਨੂੰ ਜਵਾਬ ਦਿਓ
menu-forward = ਅੱਗੇ ਭੇਜੋ
menu-archive = ਪੁਰਾਲੇਖਬੱਧ ਕਰੋ
menu-delete = ਮਿਟਾਓ
menu-spam = ਸਪੈਮ ਦੀ ਰਿਪੋਰਟ ਕਰੋ
menu-mark-read = ਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
menu-mark-unread = ਅਣਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
menu-mark-all-read = ਸਭ ਨੂੰ ਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
menu-star = ਤਾਰਾ ਲਗਾਓ
menu-unstar = ਤਾਰਾ ਹਟਾਓ
menu-important = ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
menu-not-important = ਗੈਰ-ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
menu-pin = ਸਿਖਰ ’ਤੇ ਪਿੰਨ ਕਰੋ
menu-unpin = ਅਣਪਿੰਨ ਕਰੋ
menu-print-all = ਸਭ ਪ੍ਰਿੰਟ ਕਰੋ
menu-new-window = ਨਵੀਂ ਵਿੰਡੋ ਵਿੱਚ ਖੋਲ੍ਹੋ
menu-move-to = ਇੱਥੇ ਭੇਜੋ
menu-move-to-heading = ਇੱਥੇ ਭੇਜੋ:
menu-find-from = { $name } ਵੱਲੋਂ ਈਮੇਲਾਂ ਲੱਭੋ

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਪੁਰਾਲੇਖਬੱਧ ਕੀਤੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਪੁਰਾਲੇਖਬੱਧ ਕੀਤੀਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਪੁਰਾਲੇਖਬੱਧ ਕੀਤਾ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਪੁਰਾਲੇਖਬੱਧ ਕੀਤੇ ਗਏ।
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਰੱਦੀ ਵਿੱਚ ਭੇਜੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਰੱਦੀ ਵਿੱਚ ਭੇਜੀਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਰੱਦੀ ਵਿੱਚ ਭੇਜਿਆ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਰੱਦੀ ਵਿੱਚ ਭੇਜੇ ਗਏ।
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਭੇਜੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਭੇਜੀਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਭੇਜਿਆ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਭੇਜੇ ਗਏ।
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਨੂੰ ਤਾਰਾ ਲਗਾਇਆ ਗਿਆ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਨੂੰ ਤਾਰਾ ਲਗਾਇਆ ਗਿਆ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹੇ ਨੂੰ ਤਾਰਾ ਲਗਾਇਆ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹਿਆਂ ਨੂੰ ਤਾਰਾ ਲਗਾਇਆ ਗਿਆ।
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਤੋਂ ਤਾਰਾ ਹਟਾਇਆ ਗਿਆ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਤੋਂ ਤਾਰਾ ਹਟਾਇਆ ਗਿਆ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹੇ ਤੋਂ ਤਾਰਾ ਹਟਾਇਆ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹਿਆਂ ਤੋਂ ਤਾਰਾ ਹਟਾਇਆ ਗਿਆ।
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤੀਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤਾ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤੇ ਗਏ।
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਗੈਰ-ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਗੈਰ-ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤੀਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਗੈਰ-ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤਾ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਗੈਰ-ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤੇ ਗਏ।
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਸਿਖਰ ’ਤੇ ਪਿੰਨ ਕੀਤੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਸਿਖਰ ’ਤੇ ਪਿੰਨ ਕੀਤੀਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਸਿਖਰ ’ਤੇ ਪਿੰਨ ਕੀਤਾ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਸਿਖਰ ’ਤੇ ਪਿੰਨ ਕੀਤੇ ਗਏ।
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਅਣਪਿੰਨ ਕੀਤੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਅਣਪਿੰਨ ਕੀਤੀਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਅਣਪਿੰਨ ਕੀਤਾ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਅਣਪਿੰਨ ਕੀਤੇ ਗਏ।
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਦੀ ਸਪੈਮ ਵਜੋਂ ਰਿਪੋਰਟ ਕੀਤੀ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਦੀ ਸਪੈਮ ਵਜੋਂ ਰਿਪੋਰਟ ਕੀਤੀ ਗਈ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹੇ ਦੀ ਸਪੈਮ ਵਜੋਂ ਰਿਪੋਰਟ ਕੀਤੀ ਗਈ।
       *[other] { $count } ਸੁਨੇਹਿਆਂ ਦੀ ਸਪੈਮ ਵਜੋਂ ਰਿਪੋਰਟ ਕੀਤੀ ਗਈ।
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] ਗੱਲਬਾਤ ਹਮੇਸ਼ਾ ਲਈ ਮਿਟਾਈ ਗਈ।
       *[other] { $count } ਗੱਲਬਾਤਾਂ ਹਮੇਸ਼ਾ ਲਈ ਮਿਟਾਈਆਂ ਗਈਆਂ।
    }
   *[message] { $count ->
        [one] ਸੁਨੇਹਾ ਹਮੇਸ਼ਾ ਲਈ ਮਿਟਾਇਆ ਗਿਆ।
       *[other] { $count } ਸੁਨੇਹੇ ਹਮੇਸ਼ਾ ਲਈ ਮਿਟਾਏ ਗਏ।
    }
}
toast-undone = ਕਾਰਵਾਈ ਅਣਕੀਤੀ ਕੀਤੀ ਗਈ।
toast-undo = ਅਣਕੀਤਾ ਕਰੋ
toast-no-spam-folder = ਇਸ ਖਾਤੇ ਵਿੱਚ ਕੋਈ ਸਪੈਮ ਫੋਲਡਰ ਨਹੀਂ ਹੈ।

## Reading pane: toolbar

reader-close = ਬੰਦ ਕਰੋ
reader-back = ਪਿੱਛੇ
reader-mark-unread = ਅਣਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
reader-move-to = ਇੱਥੇ ਭੇਜੋ
reader-more = ਹੋਰ
reader-print-all = ਸਭ ਪ੍ਰਿੰਟ ਕਰੋ
reader-new-window = ਨਵੀਂ ਵਿੰਡੋ ਵਿੱਚ
reader-position = { $total } ਵਿੱਚੋਂ { $position }
reader-newer = ਨਵੀਂ
reader-older = ਪੁਰਾਣੀ

## Reading pane: the conversation

reader-removed = ਇਹ ਗੱਲਬਾਤ ਹਟਾ ਦਿੱਤੀ ਗਈ ਸੀ।
reader-no-subject = (ਕੋਈ ਵਿਸ਼ਾ ਨਹੀਂ)
reader-collapse-all = ਸਭ ਸਮੇਟੋ
reader-expand-all = ਸਭ ਫੈਲਾਓ
reader-unknown-sender = (ਅਗਿਆਤ ਭੇਜਣ ਵਾਲਾ)
reader-date-ago = { $date } ({ $ago })
reader-me = ਮੈਂ
reader-to = ਨੂੰ: { $names }
reader-starred = ਤਾਰਾਬੱਧ
reader-not-starred = ਤਾਰਾਬੱਧ ਨਹੀਂ
reader-too-long = ਸੁਨੇਹਾ ਪੂਰਾ ਦਿਖਾਉਣ ਲਈ ਬਹੁਤ ਲੰਮਾ ਹੈ।
reader-encrypted-images = ਇਨਕ੍ਰਿਪਟ ਕੀਤੀ ਮੇਲ ਵਿੱਚ ਵੈੱਬ ਤੋਂ ਚਿੱਤਰ ਕਦੇ ਲੋਡ ਨਹੀਂ ਕੀਤੇ ਜਾਂਦੇ।
reader-window-failed = ਨਵੀਂ ਵਿੰਡੋ ਨਹੀਂ ਖੋਲ੍ਹੀ ਜਾ ਸਕੀ।

## Reading pane: message details (opened from "to me")

reader-details-from = ਵੱਲੋਂ:
reader-details-to = ਨੂੰ:
reader-details-cc = cc:
reader-details-date = ਮਿਤੀ:
reader-details-subject = ਵਿਸ਼ਾ:

## Reading pane: downloading a message

reader-downloading = ਇਹ ਸੁਨੇਹਾ ਸਰਵਰ ਤੋਂ ਡਾਊਨਲੋਡ ਕੀਤਾ ਜਾ ਰਿਹਾ ਹੈ…
reader-download-failed = ਇਹ ਸੁਨੇਹਾ ਡਾਊਨਲੋਡ ਨਹੀਂ ਕੀਤਾ ਜਾ ਸਕਿਆ।
reader-try-again = ਦੁਬਾਰਾ ਕੋਸ਼ਿਸ਼ ਕਰੋ

## Reply row

reply-reply = ਜਵਾਬ ਦਿਓ
reply-reply-all = ਸਭ ਨੂੰ ਜਵਾਬ ਦਿਓ
reply-forward = ਅੱਗੇ ਭੇਜੋ

## Encrypted and signed mail

security-decrypting = ਡੀਕ੍ਰਿਪਟ ਕੀਤਾ ਜਾ ਰਿਹਾ ਹੈ…
security-checking = ਦਸਤਖ਼ਤ ਦੀ ਜਾਂਚ ਕੀਤੀ ਜਾ ਰਹੀ ਹੈ…
security-partly-encrypted = ਇਸ ਸੁਨੇਹੇ ਦਾ ਸਿਰਫ਼ ਇੱਕ ਹਿੱਸਾ ਇਨਕ੍ਰਿਪਟ ਕੀਤਾ ਹੋਇਆ ਹੈ। ਬਾਕੀ ਹਿੱਸਾ ਸੁਰੱਖਿਆ ਤੋਂ ਬਾਹਰ ਜੋੜਿਆ ਗਿਆ ਸੀ ਅਤੇ ਕਿਸੇ ਵੱਲੋਂ ਵੀ ਆਇਆ ਹੋ ਸਕਦਾ ਹੈ।
security-partly-signed = ਇਸ ਸੁਨੇਹੇ ਦੇ ਸਿਰਫ਼ ਇੱਕ ਹਿੱਸੇ ’ਤੇ ਦਸਤਖ਼ਤ ਹਨ। ਬਾਕੀ ਹਿੱਸਾ ਸੁਰੱਖਿਆ ਤੋਂ ਬਾਹਰ ਜੋੜਿਆ ਗਿਆ ਸੀ ਅਤੇ ਕਿਸੇ ਵੱਲੋਂ ਵੀ ਆਇਆ ਹੋ ਸਕਦਾ ਹੈ।
security-encrypted = ਇਨਕ੍ਰਿਪਟ ਕੀਤਾ ਸੁਨੇਹਾ
security-encrypted-smime = ਇਨਕ੍ਰਿਪਟ ਕੀਤਾ ਸੁਨੇਹਾ (S/MIME)
security-no-key = ਇਹ ਸੁਨੇਹਾ ਡੀਕ੍ਰਿਪਟ ਨਹੀਂ ਕੀਤਾ ਜਾ ਸਕਦਾ: ਇਹ ਅਜਿਹੀ ਕੁੰਜੀ ਲਈ ਇਨਕ੍ਰਿਪਟ ਕੀਤਾ ਗਿਆ ਸੀ ਜੋ ਤੁਹਾਡੇ ਕੋਲ ਨਹੀਂ ਹੈ।
security-cancelled = ਡੀਕ੍ਰਿਪਟ ਕਰਨਾ ਰੱਦ ਕੀਤਾ ਗਿਆ।
security-damaged = ਇਹ ਸੁਨੇਹਾ ਡੀਕ੍ਰਿਪਟ ਨਹੀਂ ਕੀਤਾ ਜਾ ਸਕਦਾ: ਇਨਕ੍ਰਿਪਟ ਕੀਤਾ ਡਾਟਾ ਖ਼ਰਾਬ ਹੈ ਜਾਂ ਬਦਲਿਆ ਗਿਆ ਸੀ।
security-decrypt-unavailable = ਇਹ ਸੁਨੇਹਾ ਡੀਕ੍ਰਿਪਟ ਨਹੀਂ ਕੀਤਾ ਜਾ ਸਕਦਾ: ਇਨਕ੍ਰਿਪਟ ਕੀਤੀ ਮੇਲ ਪੜ੍ਹਨ ਲਈ { $tool } ਸਥਾਪਤ ਕਰੋ।
security-decrypt-failed = ਇਹ ਸੁਨੇਹਾ ਡੀਕ੍ਰਿਪਟ ਨਹੀਂ ਕੀਤਾ ਜਾ ਸਕਦਾ: { $reason }
security-unknown-signer = ਇੱਕ ਅਗਿਆਤ ਦਸਤਖ਼ਤਕਰਤਾ
security-signed-verified = { $signer } ਵੱਲੋਂ ਦਸਤਖ਼ਤ ਕੀਤਾ · ਪੁਸ਼ਟੀ ਹੋਈ
security-signed-not-sender = { $signer } ਵੱਲੋਂ ਦਸਤਖ਼ਤ ਕੀਤਾ, ਜੋ ਭੇਜਣ ਵਾਲਾ ਨਹੀਂ ਹੈ
security-signed-untrusted = { $signer } ਵੱਲੋਂ ਦਸਤਖ਼ਤ ਕੀਤਾ, ਉਸ ਕੁੰਜੀ ਨਾਲ ਜਿਸਨੂੰ ਤੁਸੀਂ ਭਰੋਸੇਯੋਗ ਨਹੀਂ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕੀਤਾ ਸੀ
security-signed-unverified = { $signer } ਵੱਲੋਂ ਦਸਤਖ਼ਤ ਕੀਤਾ · ਕੁੰਜੀ ਦੀ ਪੁਸ਼ਟੀ ਨਹੀਂ ਹੋਈ
security-bad-signature = ਗਲਤ ਦਸਤਖ਼ਤ: ਇਹ ਸੁਨੇਹਾ ਦਸਤਖ਼ਤ ਕਰਨ ਤੋਂ ਬਾਅਦ ਬਦਲਿਆ ਗਿਆ ਸੀ, ਜਾਂ ਦਸਤਖ਼ਤ ਜਾਅਲੀ ਹਨ।
security-signature-expired = { $signer } ਵੱਲੋਂ ਦਸਤਖ਼ਤ ਕੀਤਾ · ਦਸਤਖ਼ਤ ਦੀ ਮਿਆਦ ਪੁੱਗ ਗਈ ਹੈ
security-key-expired = { $signer } ਵੱਲੋਂ ਦਸਤਖ਼ਤ ਕੀਤਾ · ਉਸ ਤੋਂ ਬਾਅਦ ਕੁੰਜੀ ਦੀ ਮਿਆਦ ਪੁੱਗ ਗਈ ਹੈ
security-key-revoked = { $signer } ਵੱਲੋਂ ਅਜਿਹੀ ਕੁੰਜੀ ਨਾਲ ਦਸਤਖ਼ਤ ਕੀਤਾ ਜੋ ਰੱਦ ਕੀਤੀ ਜਾ ਚੁੱਕੀ ਹੈ
security-missing-key = ਅਜਿਹੀ ਕੁੰਜੀ ਨਾਲ ਦਸਤਖ਼ਤ ਕੀਤਾ ਜੋ ਤੁਹਾਡੇ ਕੋਲ ਨਹੀਂ ਹੈ, ਇਸ ਲਈ ਜਾਂਚ ਨਹੀਂ ਕੀਤੀ ਜਾ ਸਕਦੀ
security-missing-key-id = ਅਜਿਹੀ ਕੁੰਜੀ ({ $key }) ਨਾਲ ਦਸਤਖ਼ਤ ਕੀਤਾ ਜੋ ਤੁਹਾਡੇ ਕੋਲ ਨਹੀਂ ਹੈ, ਇਸ ਲਈ ਜਾਂਚ ਨਹੀਂ ਕੀਤੀ ਜਾ ਸਕਦੀ
security-signature-unavailable = ਦਸਤਖ਼ਤ ਕੀਤਾ; ਦਸਤਖ਼ਤ ਦੀ ਜਾਂਚ ਲਈ { $tool } ਸਥਾਪਤ ਕਰੋ
security-signature-error = ਦਸਤਖ਼ਤ ਦੀ ਜਾਂਚ ਨਹੀਂ ਕੀਤੀ ਜਾ ਸਕੀ।

## Remote images and pictures

remote-hidden = ਇਸ ਸੁਨੇਹੇ ਵਿਚਲੇ ਚਿੱਤਰ ਲੁਕੇ ਹੋਏ ਹਨ।
remote-show = ਚਿੱਤਰ ਦਿਖਾਓ
remote-always-show = ਇਸ ਭੇਜਣ ਵਾਲੇ ਤੋਂ ਹਮੇਸ਼ਾ ਦਿਖਾਓ
remote-picture-use = ਵਰਤੋ
remote-picture-too-big = 8 MB ਜਾਂ ਇਸ ਤੋਂ ਛੋਟੀ ਤਸਵੀਰ ਚੁਣੋ।
remote-picture-type = PNG, JPEG, GIF, WebP ਜਾਂ SVG ਤਸਵੀਰ ਚੁਣੋ।
remote-picture-read-failed = ਤਸਵੀਰ ਪੜ੍ਹੀ ਨਹੀਂ ਜਾ ਸਕਦੀ: { $error }
remote-picture-keep-failed = ਤਸਵੀਰ ਰੱਖੀ ਨਹੀਂ ਜਾ ਸਕਦੀ: { $error }
remote-picture-remove-failed = ਤਸਵੀਰ ਹਟਾਈ ਨਹੀਂ ਜਾ ਸਕਦੀ: { $error }

## Attachments

attachment-count = { $count ->
    [one] ਇੱਕ ਅਟੈਚਮੈਂਟ
   *[other] { $count } ਅਟੈਚਮੈਂਟਾਂ
}
attachment-save = ਰੱਖਿਅਤ ਕਰੋ
attachment-save-all = ਸਭ ਰੱਖਿਅਤ ਕਰੋ
attachment-save-all-tooltip = ਸਾਰੀਆਂ ਅਟੈਚਮੈਂਟਾਂ ਇੱਕ ਫੋਲਡਰ ਵਿੱਚ ਰੱਖਿਅਤ ਕਰੋ
attachment-save-here = ਇੱਥੇ ਰੱਖਿਅਤ ਕਰੋ
attachment-not-downloaded = ਇਹ ਸੁਨੇਹਾ ਡਾਊਨਲੋਡ ਨਹੀਂ ਕੀਤਾ ਗਿਆ।
attachment-not-found = ਇਹ ਅਟੈਚਮੈਂਟ ਸੁਨੇਹੇ ਵਿੱਚ ਨਹੀਂ ਲੱਭੀ ਜਾ ਸਕੀ।
attachment-read-failed = { $name } ਪੜ੍ਹੀ ਨਹੀਂ ਜਾ ਸਕੀ
attachment-numbered = ਅਟੈਚਮੈਂਟ { $number }
attachment-saved-all = { $count ->
    [one] { $count } ਫ਼ਾਈਲ { $place } ਵਿੱਚ ਰੱਖਿਅਤ ਕੀਤੀ
   *[other] { $count } ਫ਼ਾਈਲਾਂ { $place } ਵਿੱਚ ਰੱਖਿਅਤ ਕੀਤੀਆਂ
}
attachment-saved-some = { $total ->
    [one] { $total } ਵਿੱਚੋਂ { $saved } ਫ਼ਾਈਲ { $place } ਵਿੱਚ ਰੱਖਿਅਤ ਕੀਤੀ। { $failed } ਰੱਖਿਅਤ ਨਹੀਂ ਕੀਤੀ ਜਾ ਸਕੀ
   *[other] { $total } ਵਿੱਚੋਂ { $saved } ਫ਼ਾਈਲਾਂ { $place } ਵਿੱਚ ਰੱਖਿਅਤ ਕੀਤੀਆਂ। { $failed } ਰੱਖਿਅਤ ਨਹੀਂ ਕੀਤੀ ਜਾ ਸਕੀ
}
attachment-saved-to = { $path } ਵਿੱਚ ਰੱਖਿਅਤ ਕੀਤੀ
attachment-save-failed = { $name } ਰੱਖਿਅਤ ਨਹੀਂ ਕੀਤੀ ਜਾ ਸਕੀ: { $error }
attachment-open-failed = { $name } ਖੋਲ੍ਹੀ ਨਹੀਂ ਜਾ ਸਕੀ: { $error }
attachment-risky = ਇਹ ਫ਼ਾਈਲ ਕੋਈ ਪ੍ਰੋਗਰਾਮ ਚਲਾ ਸਕਦੀ ਹੈ, ਇਸ ਲਈ Katna ਇਸਨੂੰ ਨਹੀਂ ਖੋਲ੍ਹਦਾ। ਇਸਦੀ ਬਜਾਏ ਇਸਨੂੰ ਰੱਖਿਅਤ ਕਰੋ।
attachment-encrypted-open = ਇਹ ਫ਼ਾਈਲ ਇਨਕ੍ਰਿਪਟ ਕੀਤੀ ਹੋਈ ਆਈ ਸੀ। ਇਸਨੂੰ ਕਿਤੇ ਹੋਰ ਖੋਲ੍ਹਣ ਲਈ ਰੱਖਿਅਤ ਕਰੋ।

## Printing

print-failed = ਪ੍ਰਿੰਟ ਨਹੀਂ ਕੀਤਾ ਜਾ ਸਕਿਆ: { $error }
print-no-font = ਕੋਈ ਫ਼ੌਂਟ ਨਹੀਂ ਲੱਭਿਆ
print-opened-as-pdf = ਉੱਥੋਂ ਪ੍ਰਿੰਟ ਕਰਨ ਲਈ PDF ਵਜੋਂ ਖੋਲ੍ਹਿਆ ਗਿਆ।
print-not-downloaded = (ਹਾਲੇ ਡਾਊਨਲੋਡ ਨਹੀਂ ਕੀਤਾ ਗਿਆ।)
print-encrypted = (ਇਨਕ੍ਰਿਪਟ ਕੀਤਾ ਹੋਇਆ। ਇਸਦੀ ਲਿਖਤ ਪ੍ਰਿੰਟ ਕਰਨ ਲਈ ਇਸਨੂੰ Katna Mail ਵਿੱਚ ਖੋਲ੍ਹੋ।)
print-to = ਨੂੰ: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = ਇਸ ਸੁਨੇਹੇ ਦੀਆਂ ਅਟੈਚਮੈਂਟਾਂ ਪੜ੍ਹਨ ਲਈ ਇਸਨੂੰ ਖੋਲ੍ਹੋ।
text-copy = ਕਾਪੀ ਕਰੋ
text-select-all = ਸਭ ਚੁਣੋ

## Settings page: its tabs

settings-tab-general = ਆਮ
settings-tab-inbox = ਇਨਬਾਕਸ
settings-tab-accounts = ਖਾਤੇ
settings-tab-subscriptions = ਸਬਸਕ੍ਰਿਪਸ਼ਨ
settings-tab-appearance = ਦਿੱਖ
settings-tab-shortcuts = ਸ਼ਾਰਟਕੱਟ
settings-tab-default-apps = ਪੂਰਵ-ਨਿਰਧਾਰਿਤ ਐਪਾਂ
settings-tab-folders-rules = ਫੋਲਡਰ ਅਤੇ ਨਿਯਮ
settings-tab-compose = ਲਿਖਣਾ
settings-tab-mcp-server = MCP ਸਰਵਰ
settings-tab-feedback = ਵਰਤੋਂਕਾਰ ਫੀਡਬੈਕ
settings-tab-experimental = ਪ੍ਰਯੋਗਾਤਮਕ

## Settings page: tabs still to come

settings-tab-subscriptions-coming = ਤੁਹਾਨੂੰ ਮਿਲਣ ਵਾਲੇ ਨਿਊਜ਼ਲੈਟਰ ਅਤੇ ਮੇਲਿੰਗ ਸੂਚੀਆਂ ਦੇਖੋ, ਅਤੇ ਇੱਕ ਕਲਿੱਕ ਨਾਲ ਅਣ-ਸਬਸਕ੍ਰਾਈਬ ਕਰੋ।
settings-tab-folders-rules-coming = ਫੋਲਡਰ ਅਤੇ ਲੇਬਲ ਬਣਾਓ, ਉਨ੍ਹਾਂ ਦਾ ਨਾਮ ਬਦਲੋ, ਹਿਲਾਓ ਅਤੇ ਲੁਕਾਓ, ਅਤੇ ਚੁਣੋ ਕਿ ਕਿਹੜੇ ਸਿੰਕ ਹੋਣ। ਨਿਯਮ ਨਵੀਂ ਮੇਲ ਨੂੰ ਭੇਜਣ ਵਾਲੇ, ਵਿਸ਼ੇ ਜਾਂ ਸ਼ਬਦਾਂ ਮੁਤਾਬਕ ਆਪਣੇ-ਆਪ ਛਾਂਟਦੇ, ਲੇਬਲ ਲਗਾਉਂਦੇ, ਅੱਗੇ ਭੇਜਦੇ ਜਾਂ ਮਿਟਾਉਂਦੇ ਹਨ।
settings-tab-mcp-server-coming = ਇਸ ਕੰਪਿਊਟਰ ’ਤੇ AI ਸਹਾਇਕਾਂ ਨੂੰ ਤੁਹਾਡੀ ਮਰਜ਼ੀ ਨਾਲ ਤੁਹਾਡੀ ਮੇਲ ਖੋਜਣ, ਪੜ੍ਹਨ ਅਤੇ ਡਰਾਫਟ ਕਰਨ ਦਿਓ।

## Settings > General

settings-general-conversations = ਗੱਲਬਾਤ ਦ੍ਰਿਸ਼
settings-general-conversations-group = ਇੱਕੋ ਮੇਲ ਦੇ ਜਵਾਬਾਂ ਨੂੰ ਸਮੂਹਬੱਧ ਕਰੋ
settings-general-conversations-group-detail = ਸੂਚੀ ਵਿੱਚ ਹਰ ਗੱਲਬਾਤ ਲਈ ਇੱਕ ਲਾਈਨ
settings-general-reading = ਪੜ੍ਹਨਾ
settings-general-newest-first = ਸਭ ਤੋਂ ਨਵਾਂ ਸੁਨੇਹਾ ਪਹਿਲਾਂ
settings-general-newest-first-detail = ਗੱਲਬਾਤ ਆਪਣੇ ਸਭ ਤੋਂ ਨਵੇਂ ਜਵਾਬ ਨਾਲ ਸ਼ੁਰੂ ਹੁੰਦੀ ਹੈ
settings-general-full-headers = ਪੂਰੇ ਹੈਡਰ ਦਿਖਾਓ
settings-general-full-headers-detail = ਹਰ ਸੁਨੇਹੇ ’ਤੇ ਵੱਲੋਂ, ਨੂੰ, cc, ਮਿਤੀ ਅਤੇ ਵਿਸ਼ਾ ਖੁੱਲ੍ਹੇ ਦਿਖਾਈ ਦਿੰਦੇ ਹਨ
settings-general-full-names = ਪ੍ਰਾਪਤਕਰਤਾਵਾਂ ਦੇ ਪੂਰੇ ਨਾਮ
settings-general-full-names-detail = “ਮੈਨੂੰ, Ada” ਦੀ ਬਜਾਏ “ਮੈਨੂੰ, Ada Lovelace”
settings-general-mark-read = ਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
settings-general-mark-read-now = ਖੁੱਲ੍ਹਦੇ ਹੀ
settings-general-mark-read-1s = 1 ਸਕਿੰਟ ਖੁੱਲ੍ਹਾ ਰਹਿਣ ਤੋਂ ਬਾਅਦ
settings-general-mark-read-3s = 3 ਸਕਿੰਟ ਖੁੱਲ੍ਹਾ ਰਹਿਣ ਤੋਂ ਬਾਅਦ
settings-general-mark-read-never = ਸਿਰਫ਼ ਜਦੋਂ ਮੈਂ ਇਸਨੂੰ ਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰਾਂ
settings-general-reply-button = ਜਵਾਬ ਬਟਨ
settings-general-reply-all = ਸਭ ਨੂੰ ਜਵਾਬ ਦਿਓ
settings-general-reply-all-detail = ਹਰ ਸੁਨੇਹੇ ਦੇ ਨਾਲ ਵਾਲਾ ਜਵਾਬ ਬਟਨ ਸਿਰਫ਼ ਭੇਜਣ ਵਾਲੇ ਨੂੰ ਨਹੀਂ, ਸਭ ਨੂੰ ਜਵਾਬ ਦਿੰਦਾ ਹੈ
settings-general-remote-images = ਵੈੱਬ ਤੋਂ ਚਿੱਤਰ
settings-general-remote-images-detail = ਕਿਸੇ ਸੁਨੇਹੇ ਦੇ ਚਿੱਤਰ ਲੋਡ ਕਰਨ ਨਾਲ ਉਸਦੇ ਭੇਜਣ ਵਾਲੇ ਨੂੰ ਪਤਾ ਲੱਗ ਜਾਂਦਾ ਹੈ ਕਿ ਤੁਸੀਂ ਉਸਨੂੰ ਖੋਲ੍ਹਿਆ, ਕਦੋਂ ਅਤੇ ਲਗਭਗ ਕਿੱਥੋਂ। ਬੰਦ ਹੋਣ ’ਤੇ, ਹਰ ਸੁਨੇਹਾ ਪਹਿਲਾਂ ਪੁੱਛਦਾ ਹੈ, ਅਤੇ ਤੁਸੀਂ ਕਿਸੇ ਭੇਜਣ ਵਾਲੇ ਦੇ ਚਿੱਤਰ ਹਮੇਸ਼ਾ ਦਿਖਾ ਸਕਦੇ ਹੋ।
settings-general-remote-images-always = ਚਿੱਤਰ ਹਮੇਸ਼ਾ ਦਿਖਾਓ
settings-general-remote-images-always-detail = ਹਰ ਸੁਨੇਹੇ ਵਿੱਚ, ਸਿਰਫ਼ ਉਨ੍ਹਾਂ ਭੇਜਣ ਵਾਲਿਆਂ ਤੋਂ ਨਹੀਂ ਜਿਨ੍ਹਾਂ ’ਤੇ ਤੁਸੀਂ ਭਰੋਸਾ ਕਰਦੇ ਹੋ
settings-general-sending = ਭੇਜਣਾ
settings-general-sending-detail = ਭੇਜਿਆ ਸੁਨੇਹਾ ਕਿੰਨੀ ਦੇਰ ਉਡੀਕ ਕਰੇ, ਤਾਂ ਜੋ ਉਸਨੂੰ ਵਾਪਸ ਲਿਆ ਜਾ ਸਕੇ।
settings-general-offline = ਆਫ਼ਲਾਈਨ ਮੇਲ
settings-general-offline-detail = ਹਾਲੀਆ ਮੇਲ ਕਨੈਕਸ਼ਨ ਤੋਂ ਬਿਨਾਂ ਪੜ੍ਹਨ ਲਈ ਪੂਰੀ ਡਾਊਨਲੋਡ ਕੀਤੀ ਜਾਂਦੀ ਹੈ। ਪੁਰਾਣੀ ਮੇਲ ਤੁਹਾਡੇ ਖੋਲ੍ਹਣ ’ਤੇ ਡਾਊਨਲੋਡ ਹੁੰਦੀ ਹੈ।
settings-general-offline-days = { $count ->
    [one] { $count } ਦਿਨ
   *[other] { $count } ਦਿਨ
}
settings-general-offline-years = { $count ->
    [one] { $count } ਸਾਲ
   *[other] { $count } ਸਾਲ
}
settings-general-offline-all = ਸਾਰੀਆਂ ਮੇਲਾਂ
settings-general-offline-note = ਘੱਟ ਦਿਨ ਚੁਣਨ ਨਾਲ ਪਹਿਲਾਂ ਤੋਂ ਡਾਊਨਲੋਡ ਕੀਤੀ ਮੇਲ ਰਹਿੰਦੀ ਹੈ। ਸਰਵਰ ’ਤੇ ਕੁਝ ਨਹੀਂ ਬਦਲਦਾ।
settings-general-notifications = ਸੂਚਨਾਵਾਂ
settings-general-notifications-detail = ਇਨਬਾਕਸ ਵਿੱਚ ਨਵੀਂ ਮੇਲ ਲਈ, ਭਾਵੇਂ Katna Mail ਬੰਦ ਹੋਵੇ।
settings-general-new-mail = ਨਵੀਂ ਮੇਲ ਬਾਰੇ ਮੈਨੂੰ ਸੂਚਿਤ ਕਰੋ
settings-general-new-mail-detail = ਸਭ ਨੂੰ ਜਵਾਬ ਦਿਓ, ਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ ਅਤੇ ਪੁਰਾਲੇਖਬੱਧ ਕਰੋ ਸਮੇਤ
settings-general-new-mail-sound = ਧੁਨੀ ਵਜਾਓ
settings-general-new-mail-sound-detail = ਡੈਸਕਟਾਪ ਦੀ ਨਵੀਂ ਮੇਲ ਵਾਲੀ ਧੁਨੀ
settings-general-desktop = ਡੈਸਕਟਾਪ
settings-general-open-at-login = ਲੌਗਇਨ ’ਤੇ Katna Mail ਖੋਲ੍ਹੋ
settings-general-open-at-login-detail = ਸੇਵਾ ਚੱਲਦੀ ਹੋਣ ਤੱਕ, ਲੌਗਇਨ ’ਤੇ ਮੇਲ ਹਰ ਹਾਲ ਵਿੱਚ ਸਿੰਕ ਹੁੰਦੀ ਹੈ
settings-general-tray = ਸਿਸਟਮ ਟ੍ਰੇ ਵਿੱਚ Katna ਦਿਖਾਓ
settings-general-tray-detail = ਅਣਪੜ੍ਹੀਆਂ ਦੀ ਗਿਣਤੀ ਅਤੇ ਇੱਕ ਮੀਨੂ ਨਾਲ
settings-general-unread-badge = ਟਾਸਕਬਾਰ ਆਈਕਨ ’ਤੇ ਅਣਪੜ੍ਹੀਆਂ ਦੀ ਗਿਣਤੀ
settings-general-unread-badge-detail = ਇਨਬਾਕਸ ਦੇ ਕਿੰਨੇ ਸੁਨੇਹੇ ਅਣਪੜ੍ਹੇ ਹਨ

## Settings > Inbox

settings-inbox-tabs = ਇਨਬਾਕਸ ਟੈਬਾਂ
settings-inbox-tabs-detail = ਆਪਣੇ ਮੇਲ ਪ੍ਰਦਾਤਾ ਦੀ ਵੈੱਬਸਾਈਟ ਵਾਂਗ, ਇਨਬਾਕਸ ਨੂੰ ਟੈਬਾਂ ਵਿੱਚ ਛਾਂਟੋ।
settings-inbox-tabs-show = ਇਨਬਾਕਸ ਟੈਬਾਂ ਦਿਖਾਓ
settings-inbox-tabs-show-detail = ਬੰਦ ਹੋਣ ’ਤੇ ਹਰ ਖਾਤੇ ਲਈ ਇੱਕ ਸੂਚੀ ਦਿਖਾਉਂਦਾ ਹੈ
settings-inbox-no-accounts = ਟੈਬਾਂ ਚੁਣਨ ਲਈ ਕੋਈ ਖਾਤਾ ਸ਼ਾਮਲ ਕਰੋ।
settings-inbox-tabs-automatic = ਸਵੈਚਲਿਤ: { $tabs } ({ $provider })
settings-inbox-tabs-off = ਕੋਈ ਟੈਬ ਨਹੀਂ
settings-inbox-tabs-gmail = ਮੁੱਖ, ਪ੍ਰੋਮੋਸ਼ਨ, ਸੋਸ਼ਲ, ਅੱਪਡੇਟ, ਫੋਰਮ
settings-inbox-tabs-focused = ਕੇਂਦਰਿਤ ਅਤੇ ਹੋਰ
settings-inbox-tabs-zoho = ਇਨਬਾਕਸ, ਨਿਊਜ਼ਲੈਟਰ ਅਤੇ ਸੂਚਨਾਵਾਂ
settings-inbox-tabs-shown = ਦਿਖਾਈਆਂ ਗਈਆਂ ਟੈਬਾਂ। ਜਿਸ ਟੈਬ ਨੂੰ ਤੁਸੀਂ ਬੰਦ ਕਰਦੇ ਹੋ, ਉਸਦੀ ਮੇਲ { $tab } ਵਿੱਚ ਰਹਿੰਦੀ ਹੈ।

## Settings > Appearance

settings-appearance-reading-pane = ਪੜ੍ਹਨ ਪੈਨ
settings-appearance-reading-pane-detail = ਖੋਲ੍ਹੀ ਗੱਲਬਾਤ ਕਿੱਥੇ ਦਿਖਾਈ ਦੇਵੇ।
settings-appearance-pane-right = ਸੂਚੀ ਦੇ ਸੱਜੇ ਪਾਸੇ
settings-appearance-pane-none = ਕੋਈ ਵੰਡ ਨਹੀਂ
settings-appearance-density = ਘਣਤਾ
settings-appearance-density-default = ਪੂਰਵ-ਨਿਰਧਾਰਿਤ
settings-appearance-density-compact = ਸੰਖੇਪ
settings-appearance-scaling = ਸਕੇਲਿੰਗ
settings-appearance-scaling-detail = ਡੈਸਕਟਾਪ ਦੇ ਆਪਣੇ ਸਕੇਲ ਤੋਂ ਇਲਾਵਾ, Katna Mail ਵਿੱਚ ਹਰ ਚੀਜ਼ ਨੂੰ ਵੱਡਾ ਜਾਂ ਛੋਟਾ ਕਰਦਾ ਹੈ: ਲਿਖਤ, ਆਈਕਨ, ਵਿੱਥ ਅਤੇ ਵਿਭਾਜਕ। ਤੁਹਾਡੇ ਵੱਲੋਂ ਭੇਜੀ ਮੇਲ ਆਪਣਾ ਫ਼ੌਂਟ ਆਕਾਰ ਬਰਕਰਾਰ ਰੱਖਦੀ ਹੈ। ਬਹੁਤ ਛੋਟੇ ਆਕਾਰਾਂ ਨਾਲ ਆਈਕਨਾਂ ’ਤੇ ਕਲਿੱਕ ਕਰਨਾ ਔਖਾ ਹੋ ਸਕਦਾ ਹੈ।
settings-appearance-theme = ਥੀਮ
settings-appearance-theme-system = ਡੈਸਕਟਾਪ ਵਾਲਾ ਹੀ
settings-appearance-theme-light = ਹਲਕਾ
settings-appearance-theme-dark = ਗੂੜ੍ਹਾ
settings-appearance-desktop-colors = ਡੈਸਕਟਾਪ ਦੇ ਰੰਗ
settings-appearance-desktop-colors-use = ਡੈਸਕਟਾਪ ਦੇ ਰੰਗ ਵਰਤੋ
settings-appearance-desktop-colors-use-detail = ਡੈਸਕਟਾਪ ਦੀ ਰੰਗ ਸਕੀਮ ਅਤੇ ਐਕਸੈਂਟ ਰੰਗ
settings-appearance-app-names = ਐਪ ਨਾਮ
settings-appearance-app-names-show = ਐਪ ਨਾਮ ਦਿਖਾਓ
settings-appearance-app-names-show-detail = ਬਿਲਕੁਲ ਖੱਬੇ ਪਾਸੇ ਐਪ ਆਈਕਨਾਂ ਹੇਠਾਂ ਨਾਮ
settings-appearance-sender-pictures = ਭੇਜਣ ਵਾਲੇ ਦੀਆਂ ਤਸਵੀਰਾਂ
settings-appearance-sender-pictures-show = ਕੰਪਨੀ ਲੋਗੋ ਦਿਖਾਓ
settings-appearance-sender-pictures-show-detail = ਭੇਜਣ ਵਾਲੇ ਦੇ ਡੋਮੇਨ ਰਾਹੀਂ ਲੱਭੇ ਜਾਂਦੇ ਹਨ, ਕਦੇ ਵੀ ਸੁਨੇਹੇ ਰਾਹੀਂ ਨਹੀਂ, ਅਤੇ ਇੱਕ ਹਫ਼ਤੇ ਲਈ ਰੱਖੇ ਜਾਂਦੇ ਹਨ
settings-appearance-important = ਮਹੱਤਵਪੂਰਨ ਨਿਸ਼ਾਨ
settings-appearance-important-show = ਮਹੱਤਵਪੂਰਨ ਨਿਸ਼ਾਨ ਦਿਖਾਓ
settings-appearance-important-show-detail = ਸੂਚੀ ਵਿੱਚ ਹਰ ਸੁਨੇਹੇ ਦੇ ਨਾਲ
settings-appearance-message-width = ਸੁਨੇਹੇ ਦੀ ਚੌੜਾਈ
settings-appearance-message-width-limit = ਸੁਨੇਹਿਆਂ ਦੀ ਚੌੜਾਈ ਸੀਮਤ ਕਰੋ
settings-appearance-message-width-limit-detail = ਚੌੜੀ ਵਿੰਡੋ ਵਿੱਚ ਲੰਮੀਆਂ ਲਾਈਨਾਂ ਪੜ੍ਹਨੀਆਂ ਸੌਖੀਆਂ ਹੁੰਦੀਆਂ ਹਨ
settings-appearance-mail-colors = ਮੇਲ ਦੇ ਰੰਗ
settings-appearance-mail-colors-detail = ਜ਼ਿਆਦਾਤਰ ਮੇਲ ਚਿੱਟੇ ਪੰਨੇ ਲਈ ਡਿਜ਼ਾਈਨ ਕੀਤੀ ਜਾਂਦੀ ਹੈ। ਗੂੜ੍ਹੇ ਥੀਮ ਵਿੱਚ ਇਸਦੇ ਰੰਗ ਅਜਿਹੇ ਗੂੜ੍ਹੇ ਰੰਗਾਂ ਵਿੱਚ ਬਦਲ ਦਿੱਤੇ ਜਾਂਦੇ ਹਨ ਜੋ ਚੰਗੀ ਤਰ੍ਹਾਂ ਪੜ੍ਹੇ ਜਾਂਦੇ ਹਨ; ਬੰਦ ਹੋਣ ’ਤੇ, ਇਹ ਹਲਕੇ ਪੰਨੇ ’ਤੇ ਆਪਣੇ ਭੇਜਣ ਵਾਲੇ ਦੇ ਰੰਗ ਰੱਖਦੀ ਹੈ।
settings-appearance-dark-mail = ਮੇਲ ਲਈ ਵੀ ਗੂੜ੍ਹੇ ਰੰਗ
settings-appearance-dark-mail-detail = ਸਿਰਫ਼ ਜਦੋਂ ਥੀਮ ਗੂੜ੍ਹਾ ਹੋਵੇ
settings-appearance-attachment-previews = ਅਟੈਚਮੈਂਟ ਝਲਕਾਂ
settings-appearance-attachment-previews-show = ਅਟੈਚਮੈਂਟਾਂ ਦੀਆਂ ਝਲਕਾਂ ਦਿਖਾਓ
settings-appearance-attachment-previews-show-detail = ਹਰ ਫ਼ਾਈਲ ਦੇ ਕਾਰਡ ’ਤੇ ਉਸਦੀ ਸਮੱਗਰੀ ਦੀ ਇੱਕ ਛੋਟੀ ਤਸਵੀਰ

## Settings > Default apps

settings-default-apps-intro = ਜਦੋਂ ਤੁਸੀਂ ਅਟੈਚਮੈਂਟਾਂ ’ਤੇ ਕਲਿੱਕ ਕਰਦੇ ਹੋ ਤਾਂ ਉਹ ਕਿੱਥੇ ਖੁੱਲ੍ਹਦੀਆਂ ਹਨ। ਦਰਸ਼ਕ ਹਮੇਸ਼ਾ ਕਿਸੇ ਫ਼ਾਈਲ ਨੂੰ ਕਿਸੇ ਹੋਰ ਐਪ ਵਿੱਚ ਵੀ ਖੋਲ੍ਹ ਸਕਦਾ ਹੈ। ਡੈਸਕਟਾਪ ਦੀਆਂ ਪੂਰਵ-ਨਿਰਧਾਰਿਤ ਐਪਾਂ ਉਸਦੀਆਂ ਆਪਣੀਆਂ ਸੈਟਿੰਗਾਂ ਵਿੱਚ ਸੈੱਟ ਹੁੰਦੀਆਂ ਹਨ।
settings-default-apps-pdf = PDF ਫ਼ਾਈਲਾਂ
settings-default-apps-pdf-detail = ਪੰਨੇ, ਜ਼ੂਮ ਨਾਲ।
settings-default-apps-pictures = ਤਸਵੀਰਾਂ
settings-default-apps-pictures-detail = ਫ਼ੋਟੋਆਂ (ਸਿੱਧੀਆਂ ਕੀਤੀਆਂ), PNG, GIF, WebP, BMP, TIFF ਅਤੇ SVG।
settings-default-apps-text = ਲਿਖਤ ਫ਼ਾਈਲਾਂ
settings-default-apps-text-detail = ਸਾਦੀ ਲਿਖਤ, ਲੌਗ, ਕੋਡ ਅਤੇ ਹੋਰ ਲਿਖਤ।
settings-default-apps-sheets = ਸਪ੍ਰੈਡਸ਼ੀਟਾਂ
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) ਅਤੇ CSV।
settings-default-apps-documents = ਦਸਤਾਵੇਜ਼
settings-default-apps-documents-detail = Word (docx) ਅਤੇ OpenDocument ਲਿਖਤ (odt)।
settings-default-apps-katna = Katna Mail ਦਾ ਦਰਸ਼ਕ
settings-default-apps-system = ਡੈਸਕਟਾਪ ਦੀ ਪੂਰਵ-ਨਿਰਧਾਰਿਤ ਐਪ
settings-default-apps-ask = ਹਰ ਵਾਰ ਪੁੱਛੋ ਕਿ ਕਿਹੜੀ ਐਪ
settings-default-apps-after-saving = ਰੱਖਿਅਤ ਕਰਨ ਤੋਂ ਬਾਅਦ
settings-default-apps-show-folder = ਰੱਖਿਅਤ ਕੀਤੀਆਂ ਫ਼ਾਈਲਾਂ ਉਨ੍ਹਾਂ ਦੇ ਫੋਲਡਰ ਵਿੱਚ ਦਿਖਾਓ
settings-default-apps-show-folder-detail = ਰੱਖਿਅਤ ਕੀਤੀਆਂ ਅਟੈਚਮੈਂਟਾਂ ਚੁਣੀਆਂ ਹੋਈਆਂ ਨਾਲ ਫ਼ਾਈਲ ਮੈਨੇਜਰ ਖੋਲ੍ਹਦਾ ਹੈ

## Settings > Compose

settings-compose-send-from = ਨਵੇਂ ਸੁਨੇਹੇ ਇਸ ਤੋਂ ਭੇਜੋ
settings-compose-send-from-detail = ਜਵਾਬ ਅਤੇ ਅੱਗੇ ਭੇਜੇ ਸੁਨੇਹੇ ਹਮੇਸ਼ਾ ਉਸ ਖਾਤੇ ਤੋਂ ਜਾਂਦੇ ਹਨ ਜਿਸ ਵਿੱਚ ਤੁਸੀਂ ਹੋ।
settings-compose-send-from-current = ਜਿਸ ਖਾਤੇ ਵਿੱਚ ਤੁਸੀਂ ਹੋ
settings-compose-send-on-replies = ਜਵਾਬਾਂ ’ਤੇ ਭੇਜੋ
settings-compose-send-on-replies-detail = ਜਵਾਬ ਜਾਂ ਅੱਗੇ ਭੇਜਣ ’ਤੇ ਭੇਜੋ ਕੀ ਕਰਦਾ ਹੈ। ਭੇਜੋ ਦੇ ਨਾਲ ਵਾਲਾ ਮੀਨੂ ਦੂਜਾ ਵਿਕਲਪ ਦਿੰਦਾ ਹੈ।
settings-compose-send-plain = ਭੇਜੋ
settings-compose-send-archive = ਭੇਜੋ ਅਤੇ ਪੁਰਾਲੇਖਬੱਧ ਕਰੋ
settings-compose-signatures = ਦਸਤਖ਼ਤ
settings-compose-signatures-detail = ਤੁਹਾਡੇ ਸੁਨੇਹੇ ਦੇ ਹੇਠਾਂ, “--” ਲਾਈਨ ਤੋਂ ਬਾਅਦ ਜੋੜੇ ਜਾਂਦੇ ਹਨ। ਲਿਖਣ ਵਾਲੀ ਵਿੰਡੋ ਵਿੱਚ ਕੋਈ ਹੋਰ ਚੁਣੋ।
settings-compose-untitled = ਬਿਨਾਂ ਸਿਰਲੇਖ
settings-compose-signature-name = ਨਾਮ, ਜਿਵੇਂ ਕੰਮ
settings-compose-signature-first = ਮੇਰੇ ਦਸਤਖ਼ਤ
settings-compose-signature-numbered = ਦਸਤਖ਼ਤ { $number }
settings-compose-signature-delete = ਮਿਟਾਓ
settings-compose-signature-deleted = ਦਸਤਖ਼ਤ ਮਿਟਾਏ ਗਏ
settings-compose-signature-new = ਨਵਾਂ ਬਣਾਓ
settings-compose-no-signatures = ਹਾਲੇ ਕੋਈ ਦਸਤਖ਼ਤ ਨਹੀਂ।
settings-compose-no-signature = ਕੋਈ ਦਸਤਖ਼ਤ ਨਹੀਂ
settings-compose-for-new-mail = ਨਵੀਂ ਮੇਲ ਲਈ
settings-compose-for-replies = ਜਵਾਬਾਂ ਅਤੇ ਅੱਗੇ ਭੇਜਣ ਲਈ
settings-compose-for-replies-detail = ਜਿਸ ਗੱਲਬਾਤ ਵਿੱਚ ਤੁਸੀਂ ਕਿਸੇ ਸੁਨੇਹੇ ’ਤੇ ਦਸਤਖ਼ਤ ਕੀਤੇ ਸਨ, ਉਸ ਵਿੱਚ ਜਵਾਬ ਇਸਦੀ ਬਜਾਏ ਉਨ੍ਹਾਂ ਦਸਤਖ਼ਤਾਂ ਨਾਲ ਸ਼ੁਰੂ ਹੁੰਦਾ ਹੈ।
settings-compose-format = ਫ਼ਾਰਮੈਟ
settings-compose-plain-text = ਸਾਦੀ ਲਿਖਤ ਵਿੱਚ ਲਿਖੋ
settings-compose-plain-text-detail = ਨਵੀਂ ਮੇਲ ਫ਼ਾਰਮੈਟਿੰਗ ਤੋਂ ਬਿਨਾਂ ਸ਼ੁਰੂ ਹੁੰਦੀ ਹੈ; ਲਿਖਣ ਵਾਲੀ ਵਿੰਡੋ ਵਿੱਚ ਬਦਲਿਆ ਜਾ ਸਕਦਾ ਹੈ
settings-compose-spelling = ਸ਼ਬਦ-ਜੋੜ
settings-compose-spell-check = ਲਿਖਦੇ ਸਮੇਂ ਸ਼ਬਦ-ਜੋੜ ਜਾਂਚੋ
settings-compose-spell-check-detail = ਗਲਤ ਸ਼ਬਦ-ਜੋੜ ਵਾਲੇ ਸ਼ਬਦਾਂ ਹੇਠਾਂ ਲਕੀਰ ਲੱਗਦੀ ਹੈ, ਸੱਜੇ-ਕਲਿੱਕ ’ਤੇ ਸੁਝਾਵਾਂ ਨਾਲ
settings-compose-spell-desktop = ਡੈਸਕਟਾਪ ਦੀ ਭਾਸ਼ਾ ({ $language })
settings-compose-templates = ਟੈਂਪਲੇਟ
settings-compose-templates-detail = ਜੋ ਮੇਲ ਤੁਸੀਂ ਅਕਸਰ ਲਿਖਦੇ ਹੋ ਉਸਨੂੰ ਰੱਖਿਅਤ ਕਰੋ, ਅਤੇ ਉਸ ਤੋਂ ਨਵੀਂ ਮੇਲ ਜਾਂ ਜਵਾਬ ਸ਼ੁਰੂ ਕਰੋ।

## Settings > Shortcuts

settings-shortcuts-set = ਸ਼ਾਰਟਕੱਟ ਸੈੱਟ
settings-shortcuts-set-detail = ਆਪਣੀ ਜਾਣੀ-ਪਛਾਣੀ ਮੇਲ ਐਪ ਦੀਆਂ ਕੁੰਜੀਆਂ ਤੋਂ ਸ਼ੁਰੂ ਕਰੋ। ਇੱਥੇ Cmd ਦਾ ਮਤਲਬ Ctrl ਹੈ। ਤੁਹਾਡੀਆਂ ਆਪਣੀਆਂ ਤਬਦੀਲੀਆਂ ਸੈੱਟ ਦੇ ਉੱਪਰ ਰਹਿੰਦੀਆਂ ਹਨ, ਅਤੇ ਪੂਰਵ-ਨਿਰਧਾਰਿਤ ਬਹਾਲ ਕਰੋ ਸੈੱਟ ਦੀਆਂ ਕੁੰਜੀਆਂ ’ਤੇ ਵਾਪਸ ਲੈ ਜਾਂਦਾ ਹੈ।
settings-shortcuts-single = ਇੱਕ-ਕੁੰਜੀ ਸ਼ਾਰਟਕੱਟ
settings-shortcuts-single-detail = ਵੈੱਬਮੇਲ ਵਾਂਗ, Ctrl ਜਾਂ Alt ਤੋਂ ਬਿਨਾਂ ਕੁੰਜੀਆਂ: e ਪੁਰਾਲੇਖਬੱਧ ਕਰਦਾ ਹੈ, j ਅਤੇ k ਉੱਪਰ-ਹੇਠਾਂ ਜਾਂਦੇ ਹਨ, / ਖੋਜਦਾ ਹੈ। ਇਹ ਸੂਚੀ ਅਤੇ ਖੁੱਲ੍ਹੀ ਗੱਲਬਾਤ ਵਿੱਚ ਕੰਮ ਕਰਦੇ ਹਨ, ਟਾਈਪ ਕਰਦੇ ਸਮੇਂ ਕਦੇ ਨਹੀਂ।
settings-shortcuts-single-use = ਇੱਕ-ਕੁੰਜੀ ਸ਼ਾਰਟਕੱਟ ਵਰਤੋ
settings-shortcuts-single-use-detail = Ctrl ਸ਼ਾਰਟਕੱਟ ਹਮੇਸ਼ਾ ਕੰਮ ਕਰਦੇ ਹਨ
settings-shortcuts-how = ਕੁੰਜੀ ਬਦਲਣ ਲਈ ਉਸ ’ਤੇ ਕਲਿੱਕ ਕਰੋ, ਜਾਂ ਨਵੀਂ ਜੋੜਨ ਲਈ + ’ਤੇ, ਫਿਰ ਨਵੀਆਂ ਕੁੰਜੀਆਂ ਦਬਾਓ। Esc ਰੱਦ ਕਰਦਾ ਹੈ।
settings-shortcuts-restore = ਪੂਰਵ-ਨਿਰਧਾਰਿਤ ਬਹਾਲ ਕਰੋ
settings-shortcuts-no-key = ਕੋਈ ਕੁੰਜੀ ਨਹੀਂ
settings-shortcuts-press = ਕੁੰਜੀਆਂ ਦਬਾਓ…
settings-shortcuts-then = { $keys } ਫਿਰ…
settings-shortcuts-moved = { $keys } ਹੁਣ “{ $previous }” ਦੀ ਬਜਾਏ “{ $action }” ਕਰਦਾ ਹੈ।
settings-shortcuts-single-off = ਇੱਕ-ਕੁੰਜੀ ਸ਼ਾਰਟਕੱਟ ਬੰਦ ਹਨ, ਇਸ ਲਈ ਇਹ ਕੁੰਜੀ ਉਨ੍ਹਾਂ ਦੇ ਚਾਲੂ ਹੋਣ ’ਤੇ ਕੰਮ ਕਰੇਗੀ।
settings-shortcuts-restored = ਹਰ ਸ਼ਾਰਟਕੱਟ ਨੂੰ ਫਿਰ ਤੋਂ ਉਸਦੇ ਸੈੱਟ ਦੀਆਂ ਕੁੰਜੀਆਂ ਮਿਲ ਗਈਆਂ ਹਨ।

## Settings search: the line under a result

settings-general-language-summary = ਐਪ, ਤਾਰੀਖਾਂ ਅਤੇ ਨੰਬਰਾਂ ਦੀ ਭਾਸ਼ਾ
settings-general-reading-summary = ਸਭ ਤੋਂ ਨਵਾਂ ਸੁਨੇਹਾ ਪਹਿਲਾਂ, ਪੂਰੇ ਹੈਡਰ, ਪ੍ਰਾਪਤਕਰਤਾਵਾਂ ਦੇ ਪੂਰੇ ਨਾਮ
settings-general-mark-read-summary = ਖੋਲ੍ਹੀ ਗੱਲਬਾਤ ਕਦੋਂ ਪੜ੍ਹੀ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਹੋਵੇ: ਤੁਰੰਤ, 1 ਜਾਂ 3 ਸਕਿੰਟਾਂ ਬਾਅਦ, ਜਾਂ ਹੱਥੀਂ
settings-general-reply-button-summary = ਹਰ ਸੁਨੇਹੇ ਦੇ ਨਾਲ ਵਾਲਾ ਜਵਾਬ ਬਟਨ ਸਭ ਨੂੰ ਜਵਾਬ ਦਿੰਦਾ ਹੈ
settings-general-remote-images-summary = ਹਰ ਸੁਨੇਹੇ ਦੇ ਚਿੱਤਰ ਹਮੇਸ਼ਾ ਦਿਖਾਓ
settings-general-sending-summary = ਭੇਜਣਾ ਅਣਕੀਤਾ ਕਰੋ: ਭੇਜਿਆ ਸੁਨੇਹਾ ਕਿੰਨੀ ਦੇਰ ਉਡੀਕ ਕਰੇ, ਤਾਂ ਜੋ ਉਸਨੂੰ ਵਾਪਸ ਲਿਆ ਜਾ ਸਕੇ
settings-general-offline-summary = ਕਨੈਕਸ਼ਨ ਤੋਂ ਬਿਨਾਂ ਪੜ੍ਹਨ ਲਈ ਕਿੰਨੇ ਦਿਨਾਂ ਦੀ ਹਾਲੀਆ ਮੇਲ ਪੂਰੀ ਡਾਊਨਲੋਡ ਹੋਵੇ
settings-general-notifications-summary = ਨਵੀਂ ਮੇਲ ਦੀਆਂ ਸੂਚਨਾਵਾਂ ਅਤੇ ਉਨ੍ਹਾਂ ਦੀ ਧੁਨੀ
settings-general-desktop-summary = ਲੌਗਇਨ ’ਤੇ Katna Mail ਖੋਲ੍ਹੋ, ਸਿਸਟਮ ਟ੍ਰੇ ਆਈਕਨ ਅਤੇ ਟਾਸਕਬਾਰ ਆਈਕਨ ’ਤੇ ਅਣਪੜ੍ਹੀਆਂ ਦੀ ਗਿਣਤੀ
settings-accounts-accounts-summary = ਖਾਤਾ ਸ਼ਾਮਲ ਕਰੋ ਜਾਂ ਹਟਾਓ, ਜਾਂ ਉਸਦੀ ਤਸਵੀਰ ਬਦਲੋ
settings-appearance-density-summary = ਸੂਚੀ ਵਿੱਚ ਪੂਰਵ-ਨਿਰਧਾਰਿਤ ਜਾਂ ਸੰਖੇਪ ਲਾਈਨਾਂ
settings-appearance-scaling-summary = ਹਰ ਚੀਜ਼ ਵੱਡੀ ਜਾਂ ਛੋਟੀ ਕਰੋ: ਲਿਖਤ, ਆਈਕਨ, ਵਿੱਥ ਅਤੇ ਵਿਭਾਜਕ
settings-appearance-theme-summary = ਡੈਸਕਟਾਪ ਵਾਲਾ ਹੀ, ਹਲਕਾ ਜਾਂ ਗੂੜ੍ਹਾ
settings-appearance-sender-pictures-summary = ਕੰਪਨੀ ਲੋਗੋ, ਭੇਜਣ ਵਾਲੇ ਦੇ ਡੋਮੇਨ ਰਾਹੀਂ ਲੱਭੇ ਗਏ
settings-appearance-important-summary = ਸੂਚੀ ਵਿੱਚ ਹਰ ਸੁਨੇਹੇ ਦੇ ਨਾਲ ਮਹੱਤਵਪੂਰਨ ਨਿਸ਼ਾਨ
settings-appearance-mail-colors-summary = ਗੂੜ੍ਹੇ ਥੀਮ ਵਿੱਚ HTML ਮੇਲ ਲਈ ਗੂੜ੍ਹੇ ਰੰਗ, ਜਾਂ ਇਸਦੇ ਭੇਜਣ ਵਾਲੇ ਦੇ ਰੰਗ
settings-appearance-attachment-previews-summary = ਹਰ ਅਟੈਚਮੈਂਟ ਦੀ ਸਮੱਗਰੀ ਦੀ ਇੱਕ ਛੋਟੀ ਤਸਵੀਰ
settings-shortcuts-set-summary = Gmail, Inbox by Gmail, Apple Mail, Outlook ਜਾਂ Thunderbird ਦੀਆਂ ਕੁੰਜੀਆਂ ਤੋਂ ਸ਼ੁਰੂ ਕਰੋ
settings-shortcuts-single-summary = ਵੈੱਬਮੇਲ ਵਾਂਗ, Ctrl ਜਾਂ Alt ਤੋਂ ਬਿਨਾਂ ਕੁੰਜੀਆਂ
settings-default-apps-pdf-summary = PDF ਅਟੈਚਮੈਂਟਾਂ ਕਿੱਥੇ ਖੁੱਲ੍ਹਣ
settings-default-apps-pictures-summary = ਫ਼ੋਟੋਆਂ ਅਤੇ ਤਸਵੀਰਾਂ ਕਿੱਥੇ ਖੁੱਲ੍ਹਣ
settings-default-apps-text-summary = ਸਾਦੀ ਲਿਖਤ, ਲੌਗ ਅਤੇ ਕੋਡ ਕਿੱਥੇ ਖੁੱਲ੍ਹਣ
settings-default-apps-sheets-summary = Excel, OpenDocument ਅਤੇ CSV ਫ਼ਾਈਲਾਂ ਕਿੱਥੇ ਖੁੱਲ੍ਹਣ
settings-default-apps-documents-summary = Word ਅਤੇ OpenDocument ਲਿਖਤ ਕਿੱਥੇ ਖੁੱਲ੍ਹੇ
settings-default-apps-after-saving-summary = ਰੱਖਿਅਤ ਕੀਤੀਆਂ ਅਟੈਚਮੈਂਟਾਂ ਉਨ੍ਹਾਂ ਦੇ ਫੋਲਡਰ ਵਿੱਚ ਦਿਖਾਓ
settings-compose-send-from-summary = ਨਵੀਂ ਮੇਲ ਜਿਸ ਖਾਤੇ ਤੋਂ ਜਾਂਦੀ ਹੈ: ਜਿਸ ਵਿੱਚ ਤੁਸੀਂ ਹੋ, ਜਾਂ ਹਮੇਸ਼ਾ ਇੱਕੋ
settings-compose-send-on-replies-summary = ਜਵਾਬਾਂ ਅਤੇ ਅੱਗੇ ਭੇਜਣ ’ਤੇ ਭੇਜੋ, ਜਾਂ ਭੇਜੋ ਅਤੇ ਗੱਲਬਾਤ ਪੁਰਾਲੇਖਬੱਧ ਕਰੋ
settings-compose-signatures-summary = ਤੁਹਾਡੇ ਸੁਨੇਹੇ ਦੇ ਹੇਠਾਂ, “--” ਲਾਈਨ ਤੋਂ ਬਾਅਦ ਜੋੜੇ ਜਾਂਦੇ ਹਨ
settings-compose-for-new-mail-summary = ਉਹ ਦਸਤਖ਼ਤ ਜਿਨ੍ਹਾਂ ਨਾਲ ਨਵੀਂ ਮੇਲ ਸ਼ੁਰੂ ਹੁੰਦੀ ਹੈ
settings-compose-for-replies-summary = ਉਹ ਦਸਤਖ਼ਤ ਜਿਨ੍ਹਾਂ ਨਾਲ ਜਵਾਬ ਅਤੇ ਅੱਗੇ ਭੇਜੇ ਸੁਨੇਹੇ ਸ਼ੁਰੂ ਹੁੰਦੇ ਹਨ
settings-compose-format-summary = ਨਵੀਂ ਮੇਲ ਸਾਦੀ ਲਿਖਤ ਵਿੱਚ ਲਿਖੋ
settings-compose-spelling-summary = ਲਿਖਦੇ ਸਮੇਂ ਸ਼ਬਦ-ਜੋੜ ਜਾਂਚੋ, ਅਤੇ ਸ਼ਬਦਕੋਸ਼ ਦੀ ਭਾਸ਼ਾ
settings-compose-templates-summary = ਜਲਦੀ ਆ ਰਿਹਾ ਹੈ: ਜੋ ਮੇਲ ਤੁਸੀਂ ਅਕਸਰ ਲਿਖਦੇ ਹੋ ਉਸਨੂੰ ਰੱਖਿਅਤ ਕਰੋ, ਅਤੇ ਉਸ ਤੋਂ ਨਵੀਂ ਮੇਲ ਜਾਂ ਜਵਾਬ ਸ਼ੁਰੂ ਕਰੋ
settings-feedback-crash-reports-summary = ਜਦੋਂ Katna Mail ਜਾਂ ਇਸਦੀ ਬੈਕਗ੍ਰਾਊਂਡ ਸੇਵਾ ਕ੍ਰੈਸ਼ ਹੋਵੇ ਤਾਂ ਕ੍ਰੈਸ਼ ਰਿਪੋਰਟਾਂ ਇਸ ਕੰਪਿਊਟਰ ’ਤੇ ਰੱਖਿਅਤ ਕਰੋ
settings-feedback-saved-summary = ਇਸ ਕੰਪਿਊਟਰ ’ਤੇ ਰੱਖਿਅਤ ਕੀਤੀਆਂ ਕ੍ਰੈਸ਼ ਰਿਪੋਰਟਾਂ ਦੇਖੋ, ਕਾਪੀ ਕਰੋ ਜਾਂ ਮਿਟਾਓ
settings-feedback-help-improve-summary = ਜੋ ਗਲਤ ਹੋਇਆ ਉਸਨੂੰ ਠੀਕ ਕਰਨ ਵਿੱਚ ਮਦਦ ਲਈ ਕ੍ਰੈਸ਼ ਰਿਪੋਰਟਾਂ ਭੇਜੋ; ਜਦ ਤੱਕ ਤੁਸੀਂ ਚਾਲੂ ਨਾ ਕਰੋ, ਬੰਦ
settings-experimental-blur-summary = ਸਿਖਰਲੀ ਪੱਟੀ ਵਿੱਚੋਂ ਡੈਸਕਟਾਪ ਧੁੰਦਲਾ ਦਿਖਾਈ ਦਿੰਦਾ ਹੈ, ਅਤੇ ਮੀਨੂ ਧੁੰਦਲੇ ਸ਼ੀਸ਼ੇ ਵਰਗੇ ਹੁੰਦੇ ਹਨ
settings-search-shortcut = ਕੀਬੋਰਡ ਸ਼ਾਰਟਕੱਟ
settings-search-tab = ਸੈਟਿੰਗਾਂ ਟੈਬ
settings-search-none = “{ $query }” ਨਾਲ ਮੇਲ ਖਾਂਦੀ ਕੋਈ ਸੈਟਿੰਗ ਨਹੀਂ।
settings-search-results = “{ $query }” ਨਾਲ ਮੇਲ ਖਾਂਦੀਆਂ ਸੈਟਿੰਗਾਂ

## Quick settings (the panel that slides in from the right)

quick-title = ਤਤਕਾਲ ਸੈਟਿੰਗਾਂ
quick-see-all = ਸਾਰੀਆਂ ਸੈਟਿੰਗਾਂ ਦੇਖੋ
quick-reading-pane = ਪੜ੍ਹਨ ਪੈਨ
quick-pane-right = ਸੂਚੀ ਦੇ ਸੱਜੇ ਪਾਸੇ
quick-pane-none = ਕੋਈ ਵੰਡ ਨਹੀਂ
quick-density = ਘਣਤਾ
quick-density-default = ਪੂਰਵ-ਨਿਰਧਾਰਿਤ
quick-density-compact = ਸੰਖੇਪ
quick-theme = ਥੀਮ
quick-theme-system = ਡੈਸਕਟਾਪ ਵਾਲਾ ਹੀ
quick-theme-light = ਹਲਕਾ
quick-theme-dark = ਗੂੜ੍ਹਾ
quick-desktop-colors = ਡੈਸਕਟਾਪ ਦੇ ਰੰਗ
quick-desktop-colors-detail = ਡੈਸਕਟਾਪ ਦੀ ਰੰਗ ਸਕੀਮ ਅਤੇ ਐਕਸੈਂਟ ਰੰਗ
quick-app-names = ਐਪ ਨਾਮ
quick-app-names-detail = ਬਿਲਕੁਲ ਖੱਬੇ ਪਾਸੇ ਐਪ ਆਈਕਨਾਂ ਹੇਠਾਂ ਨਾਮ
quick-inbox-tabs = ਇਨਬਾਕਸ ਟੈਬਾਂ
quick-inbox-tabs-detail = ਹਰ ਖਾਤੇ ਦੇ ਮੇਲ ਪ੍ਰਦਾਤਾ ਦੀਆਂ ਟੈਬਾਂ
quick-choose-tabs = ਟੈਬਾਂ ਚੁਣੋ
quick-choose-tabs-detail = ਹਰ ਖਾਤੇ ਲਈ, ਸੈਟਿੰਗਾਂ ਵਿੱਚ
quick-sending = ਭੇਜਣਾ
quick-undo-send = ਭੇਜਣਾ ਅਣਕੀਤਾ ਕਰੋ
quick-undo-send-off = ਬੰਦ
quick-undo-send-seconds = { $seconds } ਸਕਿੰਟ
quick-signatures = ਦਸਤਖ਼ਤ
quick-signatures-none = ਹਾਲੇ ਕੋਈ ਨਹੀਂ
quick-signatures-one = { $name }, ਪੂਰਵ-ਨਿਰਧਾਰਿਤ ਵਜੋਂ ਵਰਤੇ ਜਾਂਦੇ
quick-signatures-many = { $count ->
    [one] { $count } ਦਸਤਖ਼ਤ; ਪੂਰਵ-ਨਿਰਧਾਰਿਤ { $name }
   *[other] { $count } ਦਸਤਖ਼ਤ; ਪੂਰਵ-ਨਿਰਧਾਰਿਤ { $name }
}
quick-signatures-no-default = { $count ->
    [one] { $count }, ਕੋਈ ਪੂਰਵ-ਨਿਰਧਾਰਿਤ ਨਹੀਂ
   *[other] { $count }, ਕੋਈ ਪੂਰਵ-ਨਿਰਧਾਰਿਤ ਨਹੀਂ
}
quick-signature-untitled = ਬਿਨਾਂ ਸਿਰਲੇਖ
quick-threading = ਈਮੇਲ ਥ੍ਰੈਡਿੰਗ
quick-conversation-view = ਗੱਲਬਾਤ ਦ੍ਰਿਸ਼
quick-conversation-view-detail = ਇੱਕੋ ਮੇਲ ਦੇ ਜਵਾਬਾਂ ਨੂੰ ਸਮੂਹਬੱਧ ਕਰੋ
quick-help = ਮਦਦ
quick-tour = ਜਾਣ-ਪਛਾਣ ਟੂਰ ਲਓ
quick-whats-new = ਨਵਾਂ ਕੀ ਹੈ
quick-about = Katna ਬਾਰੇ

## Settings: opening at login

settings-open-at-login-failed = ਲੌਗਇਨ ’ਤੇ ਖੋਲ੍ਹਣਾ ਬਦਲਿਆ ਨਹੀਂ ਜਾ ਸਕਿਆ: { $error }

## Settings > Appearance > Scaling

scale-letter = ਅ
scale-percent = { $percent }%
scale-reset = ਵਾਪਸ { $percent }% ’ਤੇ

## Settings > Experimental > Look & Feel

look-intro = ਹਾਲੇ ਅਜ਼ਮਾਈਆਂ ਜਾ ਰਹੀਆਂ ਵਿਸ਼ੇਸ਼ਤਾਵਾਂ। ਇਹ ਬਦਲ ਸਕਦੀਆਂ ਹਨ ਜਾਂ ਹਟਾਈਆਂ ਜਾ ਸਕਦੀਆਂ ਹਨ।
look-heading = ਦਿੱਖ ਅਤੇ ਅਹਿਸਾਸ
look-window-frame = ਵਿੰਡੋ ਫ੍ਰੇਮ
look-window-frame-detail = ਸਿਰਲੇਖ ਪੱਟੀ, ਵਿੰਡੋ ਬਟਨ, ਕੋਨੇ ਅਤੇ ਪਰਛਾਵਾਂ ਕੌਣ ਬਣਾਉਂਦਾ ਹੈ।
look-frame-native-kde = ਮੂਲ: KDE ਦਾ ਫ੍ਰੇਮ, ਤੁਹਾਡੇ Plasma ਥੀਮ ਵਿੱਚ
look-frame-native = ਮੂਲ: ਡੈਸਕਟਾਪ ਦਾ ਫ੍ਰੇਮ
look-frame-katna = Katna: ਸਿਖਰਲੀ ਪੱਟੀ ਸਿਰਲੇਖ ਪੱਟੀ ਬਣ ਜਾਂਦੀ ਹੈ
look-frame-katna-note-named = Katna ਗੋਲ ਕੋਨੇ ਅਤੇ ਆਪਣਾ ਪਰਛਾਵਾਂ ਬਣਾਉਂਦਾ ਹੈ। ਫ੍ਰੇਮ ਹੁਣ { $desktop } ਥੀਮ ਦੀ ਪਾਲਣਾ ਨਹੀਂ ਕਰਦਾ; ਵਿੰਡੋ ਨਿਯਮ ਫਿਰ ਵੀ ਲਾਗੂ ਹੁੰਦੇ ਹਨ।
look-frame-katna-note = Katna ਗੋਲ ਕੋਨੇ ਅਤੇ ਆਪਣਾ ਪਰਛਾਵਾਂ ਬਣਾਉਂਦਾ ਹੈ। ਫ੍ਰੇਮ ਹੁਣ ਡੈਸਕਟਾਪ ਥੀਮ ਦੀ ਪਾਲਣਾ ਨਹੀਂ ਕਰਦਾ; ਵਿੰਡੋ ਨਿਯਮ ਫਿਰ ਵੀ ਲਾਗੂ ਹੁੰਦੇ ਹਨ।
look-frame-client-side = ਤੁਹਾਡਾ ਡੈਸਕਟਾਪ ਫ੍ਰੇਮ ਹਰ ਐਪ ’ਤੇ ਛੱਡਦਾ ਹੈ, ਇਸ ਲਈ Katna ਪਹਿਲਾਂ ਹੀ ਆਪਣਾ ਫ੍ਰੇਮ ਬਣਾਉਂਦਾ ਹੈ।
look-blurred-background = ਧੁੰਦਲਾ ਪਿਛੋਕੜ
look-blurred-background-detail = ਸਿਖਰਲੀ ਪੱਟੀ ਅਤੇ ਫੋਲਡਰਾਂ ਵਿੱਚੋਂ ਡੈਸਕਟਾਪ ਧੁੰਦਲਾ ਦਿਖਾਈ ਦਿੰਦਾ ਹੈ, ਅਤੇ ਮੀਨੂ ਤੇ ਪੌਪਓਵਰ ਧੁੰਦਲੇ ਸ਼ੀਸ਼ੇ ਵਰਗੇ ਹੁੰਦੇ ਹਨ।
look-blur = ਵਿੰਡੋ ਦੇ ਪਿੱਛੇ ਵਾਲੇ ਨੂੰ ਧੁੰਦਲਾ ਕਰੋ
look-blur-detail = ਮੇਲ ਠੋਸ ਕਾਰਡਾਂ ’ਤੇ ਰਹਿੰਦੀ ਹੈ, ਇਸ ਲਈ ਲਿਖਤ ਦਾ ਕੰਟ੍ਰਾਸਟ ਬਣਿਆ ਰਹਿੰਦਾ ਹੈ
look-blur-off-kde = KDE ਦਾ ਧੁੰਦਲਾ ਪ੍ਰਭਾਵ ਬੰਦ ਹੈ। ਸਿਸਟਮ ਸੈਟਿੰਗਾਂ, ਵਿੰਡੋ ਪ੍ਰਬੰਧਨ, ਡੈਸਕਟਾਪ ਪ੍ਰਭਾਵ ਵਿੱਚ ਧੁੰਦਲਾ ਚਾਲੂ ਕਰੋ, ਫਿਰ Katna Mail ਮੁੜ ਖੋਲ੍ਹੋ।
look-blur-none-gnome = GNOME ਵਿੰਡੋਆਂ ਦੇ ਪਿੱਛੇ ਵਾਲੇ ਨੂੰ ਧੁੰਦਲਾ ਨਹੀਂ ਕਰਦਾ।
look-blur-none-x11 = ਤੁਹਾਡਾ ਵਿੰਡੋ ਮੈਨੇਜਰ ਵਿੰਡੋਆਂ ਦੇ ਪਿੱਛੇ ਵਾਲੇ ਨੂੰ ਧੁੰਦਲਾ ਨਹੀਂ ਕਰਦਾ।
look-blur-none-wayland = ਤੁਹਾਡਾ ਕੰਪੋਜ਼ਿਟਰ ਵਿੰਡੋਆਂ ਦੇ ਪਿੱਛੇ ਵਾਲੇ ਨੂੰ ਧੁੰਦਲਾ ਨਹੀਂ ਕਰਦਾ।

## Settings > User feedback (crash reports)

feedback-intro-sending = ਜੋ ਗਲਤ ਹੋਇਆ ਉਸਨੂੰ ਠੀਕ ਕਰਨ ਵਿੱਚ ਮਦਦ ਲਈ ਨਵੀਆਂ ਕ੍ਰੈਸ਼ ਰਿਪੋਰਟਾਂ ਭੇਜੀਆਂ ਜਾਂਦੀਆਂ ਹਨ। ਇਸ ਤੋਂ ਇਲਾਵਾ ਕੁਝ ਵੀ ਇਸ ਕੰਪਿਊਟਰ ਤੋਂ ਬਾਹਰ ਨਹੀਂ ਜਾਂਦਾ।
feedback-intro-local = Katna ਕਿਤੇ ਵੀ ਕੁਝ ਨਹੀਂ ਭੇਜਦਾ। ਕ੍ਰੈਸ਼ ਰਿਪੋਰਟਾਂ ਇਸ ਕੰਪਿਊਟਰ ’ਤੇ ਰਹਿੰਦੀਆਂ ਹਨ, ਤਾਂ ਜੋ ਤੁਸੀਂ ਉਨ੍ਹਾਂ ਨੂੰ ਦੇਖ ਸਕੋ ਜਾਂ ਕਿਸੇ ਬੱਗ ਰਿਪੋਰਟ ਨਾਲ ਨੱਥੀ ਕਰ ਸਕੋ।
feedback-crash-reports = ਕ੍ਰੈਸ਼ ਰਿਪੋਰਟਾਂ
feedback-crash-reports-detail = ਜਦੋਂ Katna Mail ਜਾਂ ਇਸਦੀ ਬੈਕਗ੍ਰਾਊਂਡ ਸੇਵਾ ਕ੍ਰੈਸ਼ ਹੁੰਦੀ ਹੈ ਤਾਂ ਲਿਖੀਆਂ ਜਾਂਦੀਆਂ ਹਨ।
feedback-save = ਕ੍ਰੈਸ਼ ਰਿਪੋਰਟਾਂ ਇਸ ਕੰਪਿਊਟਰ ’ਤੇ ਰੱਖਿਅਤ ਕਰੋ
feedback-save-detail = ਤੁਹਾਡਾ ਹੋਮ ਫੋਲਡਰ, ਵਰਤੋਂਕਾਰ ਅਤੇ ਕੰਪਿਊਟਰ ਦੇ ਨਾਮ ਅਤੇ ਈਮੇਲ ਪਤੇ ਛੱਡ ਦਿੱਤੇ ਜਾਂਦੇ ਹਨ
feedback-saved = ਰੱਖਿਅਤ ਕੀਤੀਆਂ ਕ੍ਰੈਸ਼ ਰਿਪੋਰਟਾਂ
feedback-saved-detail = { $count ->
    [one] ਸਭ ਤੋਂ ਨਵੀਂ { $count } ਰੱਖੀ ਜਾਂਦੀ ਹੈ।
   *[other] ਸਭ ਤੋਂ ਨਵੀਆਂ { $count } ਰੱਖੀਆਂ ਜਾਂਦੀਆਂ ਹਨ।
}
feedback-help-improve = Katna ਨੂੰ ਬਿਹਤਰ ਬਣਾਉਣ ਵਿੱਚ ਮਦਦ ਕਰੋ
feedback-help-improve-detail = ਜਦ ਤੱਕ ਤੁਸੀਂ ਚਾਲੂ ਨਾ ਕਰੋ ਬੰਦ, ਅਤੇ ਤੁਸੀਂ ਇਸਨੂੰ ਇੱਥੇ ਕਿਸੇ ਵੀ ਸਮੇਂ ਬੰਦ ਕਰ ਸਕਦੇ ਹੋ।
feedback-send = ਕ੍ਰੈਸ਼ ਰਿਪੋਰਟਾਂ ਭੇਜੋ
feedback-send-detail = ਰੱਖਿਅਤ ਕੀਤੀ ਰਿਪੋਰਟ, ਬਿਲਕੁਲ ਉਵੇਂ ਜਿਵੇਂ ਤੁਸੀਂ ਇਸਨੂੰ ਇੱਥੇ ਦੇਖ ਸਕਦੇ ਹੋ, Katna ਦੇ ਕ੍ਰੈਸ਼ ਟ੍ਰੈਕਰ (Sentry, EU ਵਿੱਚ) ਨੂੰ ਜਾਂਦੀ ਹੈ। ਕੋਈ IP ਪਤਾ, ਸੁਨੇਹੇ ਜਾਂ ਈਮੇਲ ਪਤੇ ਨਹੀਂ
feedback-none-saved = ਕੋਈ ਕ੍ਰੈਸ਼ ਰਿਪੋਰਟ ਰੱਖਿਅਤ ਨਹੀਂ ਹੈ।
feedback-delete-all = ਸਭ ਮਿਟਾਓ
feedback-app-daemon = ਬੈਕਗ੍ਰਾਊਂਡ ਸੇਵਾ
feedback-report-sent = { $date } · ਭੇਜੀ ਗਈ
feedback-view = ਦੇਖੋ
feedback-view-tooltip = ਰਿਪੋਰਟ ਖੋਲ੍ਹੋ
feedback-copy-tooltip = ਬੱਗ ਰਿਪੋਰਟ ਵਿੱਚ ਪੇਸਟ ਕਰਨ ਲਈ ਇਸਨੂੰ ਕਾਪੀ ਕਰੋ
feedback-copied = ਕ੍ਰੈਸ਼ ਰਿਪੋਰਟ ਕਾਪੀ ਕੀਤੀ ਗਈ।
feedback-deleted-all = ਕ੍ਰੈਸ਼ ਰਿਪੋਰਟਾਂ ਮਿਟਾਈਆਂ ਗਈਆਂ।
feedback-read-failed = ਕ੍ਰੈਸ਼ ਰਿਪੋਰਟ ਪੜ੍ਹੀ ਨਹੀਂ ਜਾ ਸਕੀ: { $error }
feedback-delete-failed = ਕ੍ਰੈਸ਼ ਰਿਪੋਰਟ ਮਿਟਾਈ ਨਹੀਂ ਜਾ ਸਕੀ: { $error }
feedback-delete-all-failed = ਕ੍ਰੈਸ਼ ਰਿਪੋਰਟਾਂ ਮਿਟਾਈਆਂ ਨਹੀਂ ਜਾ ਸਕੀਆਂ: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _ਫ਼ਾਈਲ
desktop-menu-new-message = _ਨਵਾਂ ਸੁਨੇਹਾ
desktop-menu-quit = _ਬਾਹਰ ਜਾਓ
desktop-menu-edit = _ਸੋਧੋ
desktop-menu-undo = _ਅਣਕੀਤਾ ਕਰੋ
desktop-menu-select-all = _ਸਭ ਚੁਣੋ
desktop-menu-select-none = _ਕੋਈ ਨਾ ਚੁਣੋ
desktop-menu-find = _ਲੱਭੋ…
desktop-menu-view = _ਵੇਖੋ
desktop-menu-folder-list = _ਫੋਲਡਰ ਸੂਚੀ ਦਿਖਾਓ
desktop-menu-refresh = _ਤਾਜ਼ਾ ਕਰੋ
desktop-menu-go = _ਜਾਓ
desktop-menu-inbox = _ਇਨਬਾਕਸ
desktop-menu-starred = _ਤਾਰਾਬੱਧ
desktop-menu-sent = _ਭੇਜੀਆਂ ਗਈਆਂ
desktop-menu-drafts = _ਡਰਾਫਟ
desktop-menu-all-mail = _ਸਾਰੀਆਂ ਮੇਲਾਂ
desktop-menu-next = _ਅਗਲੀ ਗੱਲਬਾਤ
desktop-menu-previous = _ਪਿਛਲੀ ਗੱਲਬਾਤ
desktop-menu-message = _ਸੁਨੇਹਾ
desktop-menu-open = _ਖੋਲ੍ਹੋ
desktop-menu-reply = _ਜਵਾਬ ਦਿਓ
desktop-menu-reply-all = _ਸਭ ਨੂੰ ਜਵਾਬ ਦਿਓ
desktop-menu-forward = _ਅੱਗੇ ਭੇਜੋ
desktop-menu-archive = _ਪੁਰਾਲੇਖਬੱਧ ਕਰੋ
desktop-menu-delete = _ਮਿਟਾਓ
desktop-menu-spam = _ਸਪੈਮ ਦੀ ਰਿਪੋਰਟ ਕਰੋ
desktop-menu-move-to = _ਇੱਥੇ ਭੇਜੋ…
desktop-menu-mark-read = _ਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
desktop-menu-mark-unread = _ਅਣਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
desktop-menu-star = _ਤਾਰਾ ਲਗਾਓ
desktop-menu-important = _ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
desktop-menu-not-important = _ਗੈਰ-ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
desktop-menu-settings = _ਸੈਟਿੰਗਾਂ
desktop-menu-quick-settings = _ਤਤਕਾਲ ਸੈਟਿੰਗਾਂ
desktop-menu-configure = _Katna Mail ਦੀ ਸੰਰਚਨਾ ਕਰੋ…
desktop-menu-help = _ਮਦਦ
desktop-menu-shortcuts = _ਕੀਬੋਰਡ ਸ਼ਾਰਟਕੱਟ
desktop-menu-whats-new = _ਨਵਾਂ ਕੀ ਹੈ
desktop-menu-about = _Katna ਬਾਰੇ

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = ਨੈਵੀਗੇਸ਼ਨ
shortcut-group-actions = ਕਾਰਵਾਈਆਂ
shortcut-group-go-to = ਇੱਥੇ ਜਾਓ
shortcut-group-app = ਐਪਲੀਕੇਸ਼ਨ

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = ਅਗਲੀ ਗੱਲਬਾਤ
shortcut-previous = ਪਿਛਲੀ ਗੱਲਬਾਤ
shortcut-down = ਸੂਚੀ ਵਿੱਚ ਹੇਠਾਂ ਜਾਓ
shortcut-up = ਸੂਚੀ ਵਿੱਚ ਉੱਪਰ ਜਾਓ
shortcut-first = ਸੂਚੀ ਵਿੱਚ ਪਹਿਲੀ
shortcut-last = ਸੂਚੀ ਵਿੱਚ ਆਖਰੀ
shortcut-page-down = ਸੂਚੀ ਵਿੱਚ ਇੱਕ ਪੰਨਾ ਹੇਠਾਂ
shortcut-page-up = ਸੂਚੀ ਵਿੱਚ ਇੱਕ ਪੰਨਾ ਉੱਪਰ
shortcut-open = ਗੱਲਬਾਤ ਖੋਲ੍ਹੋ
shortcut-back = ਸੂਚੀ ’ਤੇ ਵਾਪਸ ਜਾਓ
shortcut-scroll-down = ਹੇਠਾਂ ਸਕ੍ਰੋਲ ਕਰੋ
shortcut-scroll-up = ਉੱਪਰ ਸਕ੍ਰੋਲ ਕਰੋ
shortcut-scroll-page-down = ਇੱਕ ਪੰਨਾ ਹੇਠਾਂ ਸਕ੍ਰੋਲ ਕਰੋ
shortcut-scroll-page-up = ਇੱਕ ਪੰਨਾ ਉੱਪਰ ਸਕ੍ਰੋਲ ਕਰੋ
shortcut-compose = ਲਿਖੋ
shortcut-reply = ਜਵਾਬ ਦਿਓ
shortcut-reply-all = ਸਭ ਨੂੰ ਜਵਾਬ ਦਿਓ
shortcut-forward = ਅੱਗੇ ਭੇਜੋ
shortcut-archive = ਪੁਰਾਲੇਖਬੱਧ ਕਰੋ
shortcut-delete = ਮਿਟਾਓ
shortcut-spam = ਸਪੈਮ ਦੀ ਰਿਪੋਰਟ ਕਰੋ
shortcut-move-to = ਇੱਥੇ ਭੇਜੋ
shortcut-mark-read = ਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
shortcut-mark-unread = ਅਣਪੜ੍ਹਿਆ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
shortcut-star = ਤਾਰਾ ਲਗਾਓ ਜਾਂ ਹਟਾਓ
shortcut-important = ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
shortcut-not-important = ਗੈਰ-ਮਹੱਤਵਪੂਰਨ ਵਜੋਂ ਨਿਸ਼ਾਨਦੇਹ ਕਰੋ
shortcut-check = ਗੱਲਬਾਤ ’ਤੇ ਨਿਸ਼ਾਨ ਲਗਾਓ
shortcut-select-all = ਸਾਰੀਆਂ ਗੱਲਬਾਤਾਂ ’ਤੇ ਨਿਸ਼ਾਨ ਲਗਾਓ
shortcut-select-none = ਸਾਰੀਆਂ ਗੱਲਬਾਤਾਂ ਤੋਂ ਨਿਸ਼ਾਨ ਹਟਾਓ
shortcut-undo = ਆਖਰੀ ਕਾਰਵਾਈ ਅਣਕੀਤੀ ਕਰੋ
shortcut-go-inbox = ਇਨਬਾਕਸ
shortcut-go-starred = ਤਾਰਾਬੱਧ
shortcut-go-sent = ਭੇਜੀਆਂ ਗਈਆਂ
shortcut-go-drafts = ਡਰਾਫਟ
shortcut-go-all = ਸਾਰੀਆਂ ਮੇਲਾਂ
shortcut-search = ਮੇਲ ਖੋਜੋ
shortcut-navigation = ਮੀਨੂ ਦਿਖਾਓ ਜਾਂ ਸਮੇਟੋ
shortcut-quick-settings = ਤਤਕਾਲ ਸੈਟਿੰਗਾਂ
shortcut-settings = ਸਾਰੀਆਂ ਸੈਟਿੰਗਾਂ
shortcut-shortcuts = ਕੀਬੋਰਡ ਸ਼ਾਰਟਕੱਟ
shortcut-reload = ਨਵੀਂ ਮੇਲ ਲਈ ਜਾਂਚ ਕਰੋ
shortcut-quit = ਬਾਹਰ ਜਾਓ

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } ਫਿਰ { $second }

## Settings > Accounts

accounts-folder-pane = ਫੋਲਡਰ ਪੈਨ
accounts-folder-pane-detail = ਖੱਬੇ ਪਾਸੇ ਵਾਲਾ ਪੈਨ ਕਿਹੜੇ ਖਾਤਿਆਂ ਦੇ ਫੋਲਡਰ ਦਿਖਾਉਂਦਾ ਹੈ।
accounts-shown-one = ਇੱਕ ਸਮੇਂ ਇੱਕ ਖਾਤਾ; ਖਾਤਾ ਕਾਰਡ ਵਿੱਚ ਬਦਲੋ
accounts-shown-all = ਸਾਰੇ ਖਾਤੇ, ਇੱਕ ਤੋਂ ਬਾਅਦ ਇੱਕ
accounts-row = ਖਾਤੇ
accounts-row-detail = ਖਾਤਾ ਹਟਾਉਣ ਨਾਲ ਇਸ ਕੰਪਿਊਟਰ ’ਤੇ ਉਸਦੀ ਮੇਲ ਦੀ Katna ਵਾਲੀ ਕਾਪੀ ਮਿਟ ਜਾਂਦੀ ਹੈ। ਮੇਲ ਸਰਵਰ ’ਤੇ ਰਹਿੰਦੀ ਹੈ।
accounts-none = ਹਾਲੇ ਕੋਈ ਖਾਤਾ ਨਹੀਂ।
accounts-kind-imported = ਆਯਾਤ ਕੀਤਾ
accounts-picture-reset = ਡੈਸਕਟਾਪ ਤਸਵੀਰ ਵਰਤੋ
accounts-picture-change = ਤਸਵੀਰ ਬਦਲੋ
accounts-remove = ਹਟਾਓ
accounts-delete-all-row = ਸਾਰਾ ਡਾਟਾ ਮਿਟਾਓ
accounts-delete-all-row-detail = ਨਵੀਂ ਸਥਾਪਨਾ ਵਾਂਗ, ਨਵੇਂ ਸਿਰੇ ਤੋਂ ਸ਼ੁਰੂ ਕਰੋ।
accounts-delete-all-about = ਇਸ ਕੰਪਿਊਟਰ ਤੋਂ ਹਰ ਖਾਤਾ, ਸਾਰੀ ਸਟੋਰ ਕੀਤੀ ਮੇਲ, ਸੰਪਰਕ ਅਤੇ ਕੈਲੰਡਰ, ਖੋਜ ਇੰਡੈਕਸ, ਤੁਹਾਡੀਆਂ ਸੈਟਿੰਗਾਂ ਅਤੇ ਰੱਖਿਅਤ ਕੀਤੇ ਪਾਸਵਰਡ ਮਿਟਾਉਂਦਾ ਹੈ। ਤੁਹਾਡੇ ਮੇਲ ਸਰਵਰਾਂ ’ਤੇ ਕੁਝ ਨਹੀਂ ਬਦਲਦਾ।
accounts-delete-all-open = Katna ਦਾ ਸਾਰਾ ਡਾਟਾ ਮਿਟਾਓ

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } ਨੂੰ Katna ਤੋਂ ਹਟਾ ਦਿੱਤਾ ਗਿਆ।
accounts-removed = { $address } ਨੂੰ Katna ਤੋਂ ਹਟਾ ਦਿੱਤਾ ਗਿਆ। ਇਸਦੀ ਮੇਲ ਹਾਲੇ ਵੀ ਸਰਵਰ ’ਤੇ ਹੈ।
accounts-all-deleted = Katna ਦਾ ਸਾਰਾ ਡਾਟਾ ਇਸ ਕੰਪਿਊਟਰ ਤੋਂ ਮਿਟਾ ਦਿੱਤਾ ਗਿਆ।

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = ਕੀ { $address } ਹਟਾਉਣਾ ਹੈ?
accounts-remove-confirm = ਖਾਤਾ ਹਟਾਓ
accounts-removing = ਹਟਾਇਆ ਜਾ ਰਿਹਾ ਹੈ…
accounts-remove-local-mail = { $folders ->
    [0] ਇਸ ਖਾਤੇ ਵਿੱਚ ਆਯਾਤ ਕੀਤੀ ਸਾਰੀ ਮੇਲ
    [one] ਇਸ ਖਾਤੇ ਵਿੱਚ ਇਸਦੇ ਫੋਲਡਰ ਵਿੱਚ ਆਯਾਤ ਕੀਤੀ ਸਾਰੀ ਮੇਲ
   *[other] ਇਸ ਖਾਤੇ ਵਿੱਚ ਇਸਦੇ { $folders } ਫੋਲਡਰਾਂ ਵਿੱਚ ਆਯਾਤ ਕੀਤੀ ਸਾਰੀ ਮੇਲ
}
accounts-remove-local-settings = ਇਸਦੀਆਂ Katna ਸੈਟਿੰਗਾਂ
accounts-remove-mail = { $folders ->
    [0] Katna ਵੱਲੋਂ ਸਟੋਰ ਕੀਤੀ ਇਸ ਖਾਤੇ ਦੀ ਸਾਰੀ ਮੇਲ
    [one] Katna ਵੱਲੋਂ ਇਸਦੇ ਫੋਲਡਰ ਵਿੱਚ ਸਟੋਰ ਕੀਤੀ ਇਸ ਖਾਤੇ ਦੀ ਸਾਰੀ ਮੇਲ
   *[other] Katna ਵੱਲੋਂ ਇਸਦੇ { $folders } ਫੋਲਡਰਾਂ ਵਿੱਚ ਸਟੋਰ ਕੀਤੀ ਇਸ ਖਾਤੇ ਦੀ ਸਾਰੀ ਮੇਲ
}
accounts-remove-outbox = ਆਊਟਬਾਕਸ ਵਿੱਚ ਉਡੀਕ ਰਹੇ ਇਸਦੇ ਸੁਨੇਹੇ
accounts-remove-settings = ਇਸਦਾ ਰੱਖਿਅਤ ਕੀਤਾ ਪਾਸਵਰਡ ਅਤੇ ਇਸਦੀਆਂ Katna ਸੈਟਿੰਗਾਂ
accounts-delete-all-title = ਕੀ Katna ਦਾ ਸਾਰਾ ਡਾਟਾ ਮਿਟਾਉਣਾ ਹੈ?
accounts-delete-all-confirm = ਸਭ ਕੁਝ ਮਿਟਾਓ
accounts-deleting = ਮਿਟਾਇਆ ਜਾ ਰਿਹਾ ਹੈ…
accounts-delete-all-accounts = ਹਰ ਖਾਤਾ, ਅਤੇ Katna ਵੱਲੋਂ ਸਟੋਰ ਕੀਤੀ ਸਾਰੀ ਮੇਲ ਅਤੇ ਅਟੈਚਮੈਂਟਾਂ
accounts-delete-all-contacts = ਸੰਪਰਕ, ਕੈਲੰਡਰ ਅਤੇ ਖੋਜ ਇੰਡੈਕਸ
accounts-delete-all-settings = ਸਾਰੀਆਂ ਸੈਟਿੰਗਾਂ, ਦਸਤਖ਼ਤ ਅਤੇ ਕੀਬੋਰਡ ਸ਼ਾਰਟਕੱਟ
accounts-delete-all-passwords = ਹਰ ਰੱਖਿਅਤ ਕੀਤਾ ਪਾਸਵਰਡ
accounts-deleted-heading = ਇਸ ਕੰਪਿਊਟਰ ਤੋਂ ਮਿਟਾਇਆ ਜਾਵੇਗਾ:
accounts-cannot-undo = ਇਸਨੂੰ ਅਣਕੀਤਾ ਨਹੀਂ ਕੀਤਾ ਜਾ ਸਕਦਾ।
accounts-server-delete-all = ਤੁਹਾਡੇ ਮੇਲ ਸਰਵਰਾਂ ’ਤੇ ਕੁਝ ਨਹੀਂ ਬਦਲਦਾ: ਤੁਹਾਡੀ ਮੇਲ ਉੱਥੇ ਹੀ ਰਹਿੰਦੀ ਹੈ, ਅਤੇ ਖਾਤਾ ਦੁਬਾਰਾ ਸ਼ਾਮਲ ਕਰਨ ’ਤੇ ਇਹ ਮੁੜ ਡਾਊਨਲੋਡ ਹੋ ਜਾਂਦੀ ਹੈ। ਫ਼ਾਈਲਾਂ ਤੋਂ ਆਯਾਤ ਕੀਤੀ ਮੇਲ ਸਿਰਫ਼ Katna ਵਿੱਚ ਹੈ; ਫ਼ਾਈਲਾਂ ਨੂੰ ਛੂਹਿਆ ਨਹੀਂ ਜਾਂਦਾ।
accounts-server-local = ਇਹ ਮੇਲ ਫ਼ਾਈਲਾਂ ਤੋਂ ਆਯਾਤ ਕੀਤੀ ਗਈ ਸੀ, ਇਸ ਲਈ ਇਸਦੀ ਇੱਕੋ-ਇੱਕ ਕਾਪੀ Katna ਕੋਲ ਹੈ। ਜਿਨ੍ਹਾਂ ਫ਼ਾਈਲਾਂ ਤੋਂ ਇਹ ਆਈ ਸੀ ਉਨ੍ਹਾਂ ਨੂੰ ਛੂਹਿਆ ਨਹੀਂ ਜਾਂਦਾ; ਇਸਨੂੰ ਵਾਪਸ ਲੈਣ ਲਈ ਉਨ੍ਹਾਂ ਨੂੰ ਦੁਬਾਰਾ ਆਯਾਤ ਕਰੋ।
accounts-server-remove = ਮੇਲ ਸਰਵਰ ’ਤੇ ਕੁਝ ਨਹੀਂ ਬਦਲਦਾ: ਤੁਹਾਡੀ ਮੇਲ ਉੱਥੇ ਹੀ ਰਹਿੰਦੀ ਹੈ, ਅਤੇ ਖਾਤਾ ਦੁਬਾਰਾ ਸ਼ਾਮਲ ਕਰਨ ’ਤੇ ਇਹ ਮੁੜ ਡਾਊਨਲੋਡ ਹੋ ਜਾਂਦੀ ਹੈ।
accounts-confirm-word = ਮਿਟਾਓ
accounts-confirm-placeholder = “{ accounts-confirm-word }” ਟਾਈਪ ਕਰੋ
accounts-confirm-prompt = ਪੁਸ਼ਟੀ ਕਰਨ ਲਈ, “{ accounts-confirm-word }” ਟਾਈਪ ਕਰੋ:
accounts-cancel = ਰੱਦ ਕਰੋ
