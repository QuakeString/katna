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
