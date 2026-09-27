# Katna Mail, Malayalam (മലയാളം).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = ഭാഷ: { $language }
language-tooltip-system = ഭാഷ: { $language }, സിസ്റ്റം പിന്തുടരുന്നു
language-search = ഭാഷ തിരയുക
language-system-default = സിസ്റ്റം ഡിഫോൾട്ട്
language-system-now = ഇപ്പോൾ { $language }
language-no-match = “{ $query }” എന്നതുമായി പൊരുത്തപ്പെടുന്ന ഭാഷയൊന്നുമില്ല
language-machine = മെഷീൻ വിവർത്തനം. മെച്ചപ്പെടുത്താൻ സഹായിക്കൂ
language-setting = ഭാഷ
language-setting-detail = മെനുകൾ, ബട്ടണുകൾ, സന്ദേശങ്ങൾ എന്നിവയുടെ ഭാഷയും തീയതികളുടെയും നമ്പറുകളുടെയും ഫോർമാറ്റും. സിസ്റ്റം ഡിഫോൾട്ട് ഡെസ്‌ക്‌ടോപ്പിനെ പിന്തുടരുന്നു.

## Dates and sizes

ago-just-now = അൽപ്പം മുമ്പ്
ago-minutes = { $count ->
    [one] { $count } മിനിറ്റ് മുമ്പ്
   *[other] { $count } മിനിറ്റ് മുമ്പ്
}
ago-hours = { $count ->
    [one] { $count } മണിക്കൂർ മുമ്പ്
   *[other] { $count } മണിക്കൂർ മുമ്പ്
}
ago-days = { $count ->
    [one] { $count } ദിവസം മുമ്പ്
   *[other] { $count } ദിവസം മുമ്പ്
}
size-bytes = { $count ->
    [one] { $count } ബൈറ്റ്
   *[other] { $count } ബൈറ്റുകൾ
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = ഫോൾഡറുകൾ മറയ്ക്കുക
folders-show = ഫോൾഡറുകൾ കാണിക്കുക
compose = രചിക്കുക
search = തിരയുക
search-mail = മെയിലിൽ തിരയുക
search-settings = ക്രമീകരണത്തിൽ തിരയുക
search-clear = തിരയൽ മായ്‌ക്കുക
search-options-show = തിരയൽ ഓപ്ഷനുകൾ കാണിക്കുക
settings = ക്രമീകരണം
account-add = അക്കൗണ്ട് ചേർക്കുക

## App rail (and the bottom bar on a phone)

rail-mail = മെയിൽ
rail-calendar = കലണ്ടർ
rail-contacts = കോൺടാക്റ്റുകൾ
rail-tasks = ടാസ്‌ക്കുകൾ
rail-notes = കുറിപ്പുകൾ
rail-feeds = ഫീഡുകൾ

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = ഉടൻ വരുന്നു
app-calendar-promise = നിങ്ങളുടെ CalDAV കലണ്ടറുകളും മെയിലിൽ വരുന്ന മീറ്റിംഗ് ക്ഷണങ്ങളും ഓർമ്മപ്പെടുത്തലുകളും, ഇൻബോക്‌സിന് തൊട്ടടുത്ത്.
app-tasks-promise = CalDAV-യുമായി സമന്വയിപ്പിക്കുന്ന ചെയ്യേണ്ടവയുടെ ലിസ്റ്റുകളും മെയിലിൽ നിന്ന് ഉണ്ടാക്കിയ ടാസ്‌ക്കുകളും.
app-notes-promise = പെട്ടെന്നുള്ള കുറിപ്പുകളും, പിന്നീടത്തേക്കായി ഒരു മെയിലിനെയോ സംഭാഷണത്തെയോ കുറിച്ചുള്ള കുറിപ്പുകളും.
app-feeds-promise = നിങ്ങളുടെ മെയിലിനൊപ്പം RSS, Atom ഫീഡുകൾ വായിക്കുക.

## Contacts page

app-contacts-loading = നിങ്ങളുടെ മെയിലിൽ നിന്ന് ആളുകളെ ശേഖരിക്കുന്നു…
app-contacts-empty = നിങ്ങൾ മെയിൽ അയയ്ക്കുന്ന ആളുകൾ ഇവിടെ ദൃശ്യമാകും.
app-contacts-count = { $count ->
    [one] നിങ്ങളുടെ മെയിലിൽ നിന്ന് { $count } വ്യക്തി, ഏറ്റവും കൂടുതൽ മെയിൽ ചെയ്‌തവർ ആദ്യം
   *[other] നിങ്ങളുടെ മെയിലിൽ നിന്ന് { $count } ആളുകൾ, ഏറ്റവും കൂടുതൽ മെയിൽ ചെയ്‌തവർ ആദ്യം
}
app-contacts-top = { $count ->
    [one] നിങ്ങളുടെ മെയിലിൽ നിന്നുള്ള മുൻനിരയിലെ { $count } വ്യക്തി, ഏറ്റവും കൂടുതൽ മെയിൽ ചെയ്‌തവർ ആദ്യം
   *[other] നിങ്ങളുടെ മെയിലിൽ നിന്നുള്ള മുൻനിരയിലെ { $count } ആളുകൾ, ഏറ്റവും കൂടുതൽ മെയിൽ ചെയ്‌തവർ ആദ്യം
}
app-contacts-messages = { $count ->
    [one] { $count } സന്ദേശം
   *[other] { $count } സന്ദേശങ്ങൾ
}
app-contacts-last = അവസാനം { $date }

## Navigation (the folders pane)

nav-labels = ലേബലുകൾ
nav-folders = ഫോൾഡറുകൾ
nav-label-new = പുതിയ ലേബൽ സൃഷ്‌ടിക്കുക
nav-folder-new = പുതിയ ഫോൾഡർ സൃഷ്‌ടിക്കുക
nav-account-unnamed = അക്കൗണ്ട് { $number }
nav-tab-new = { $count ->
    [one] { $count } പുതിയത്
   *[other] { $count } പുതിയവ
}

## Special folders (the user's own folders keep their names)

folder-inbox = ഇൻബോക്‌സ്
folder-starred = നക്ഷത്രമിട്ടവ
folder-drafts = ഡ്രാഫ്റ്റുകൾ
folder-sent = അയച്ചവ
folder-archive = ആർക്കൈവ്
folder-spam = സ്‌പാം
folder-trash = ട്രാഷ്
folder-all-mail = എല്ലാ മെയിലുകളും
folder-scheduled = ഷെഡ്യൂൾ ചെയ്‌തവ

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = പുതിയ ലേബൽ
label-folder-new-title = പുതിയ ഫോൾഡർ
label-prompt = പുതിയ ലേബലിന്റെ പേര് നൽകുക:
label-folder-prompt = പുതിയ ഫോൾഡറിന്റെ പേര് നൽകുക:
label-name-hint = ലേബലിന്റെ പേര്
label-folder-name-hint = ഫോൾഡറിന്റെ പേര്
label-nest = ഇതിന് കീഴിൽ ലേബൽ ചേർക്കുക:
label-folder-nest = ഇതിന് കീഴിൽ ഫോൾഡർ ചേർക്കുക:
label-cancel = റദ്ദാക്കുക
label-create = സൃഷ്‌ടിക്കുക
label-creating = സൃഷ്‌ടിക്കുന്നു…
label-created = “{ $name }” ലേബൽ സൃഷ്‌ടിച്ചു.
label-folder-created = “{ $name }” ഫോൾഡർ സൃഷ്‌ടിച്ചു.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = പ്രാഥമികം
tab-promotions = പ്രമോഷനുകൾ
tab-social = സോഷ്യൽ
tab-updates = അപ്‌ഡേറ്റുകൾ
tab-forums = ഫോറങ്ങൾ
tab-focused = ഫോക്കസ് ചെയ്‌തവ
tab-other = മറ്റുള്ളവ
tab-inbox = ഇൻബോക്‌സ്
tab-newsletters = വാർത്താക്കുറിപ്പുകൾ
tab-notifications = അറിയിപ്പുകൾ
tab-new = { $count } പുതിയവ
tab-provider-other = Katna അടുക്കിയത്

## Mail list: toolbar

list-select = തിരഞ്ഞെടുക്കുക
list-refresh = പുതുക്കുക
list-more = കൂടുതൽ
list-mark-read = വായിച്ചതായി അടയാളപ്പെടുത്തുക
list-mark-unread = വായിക്കാത്തതായി അടയാളപ്പെടുത്തുക
list-move-to = ഇതിലേക്ക് നീക്കുക
list-archive = ആർക്കൈവ് ചെയ്യുക
list-spam = സ്‌പാം ആയി റിപ്പോർട്ട് ചെയ്യുക
list-delete = ഇല്ലാതാക്കുക
list-newer = പുതിയവ
list-older = പഴയവ
list-range = { $total }-ൽ { $first }–{ $last }
list-range-about = ഏകദേശം { $total }-ൽ { $first }–{ $last }
list-results = “{ $query }” എന്നതിനുള്ള ഫലങ്ങൾ
list-results-corrected = “{ $query }” എന്നതിനുള്ള ഫലങ്ങൾ കാണിക്കുന്നു
list-search-instead = പകരം “{ $query }” എന്ന് തിരയുക
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = എല്ലാം
list-pick-none = ഒന്നുമില്ല
list-pick-read = വായിച്ചവ
list-pick-unread = വായിക്കാത്തവ
list-pick-starred = നക്ഷത്രമിട്ടവ
list-pick-unstarred = നക്ഷത്രമിടാത്തവ

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } സംഭാഷണവും തിരഞ്ഞെടുത്തു.
       *[other] എല്ലാ { $count } സംഭാഷണങ്ങളും തിരഞ്ഞെടുത്തു.
    }
   *[message] { $count ->
        [one] { $count } സന്ദേശവും തിരഞ്ഞെടുത്തു.
       *[other] എല്ലാ { $count } സന്ദേശങ്ങളും തിരഞ്ഞെടുത്തു.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder }-ലെ { $count } സംഭാഷണവും തിരഞ്ഞെടുത്തു.
       *[other] { $folder }-ലെ എല്ലാ { $count } സംഭാഷണങ്ങളും തിരഞ്ഞെടുത്തു.
    }
   *[message] { $count ->
        [one] { $folder }-ലെ { $count } സന്ദേശവും തിരഞ്ഞെടുത്തു.
       *[other] { $folder }-ലെ എല്ലാ { $count } സന്ദേശങ്ങളും തിരഞ്ഞെടുത്തു.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] സ്ക്രീനിലെ { $count } സംഭാഷണവും തിരഞ്ഞെടുത്തു.
       *[other] സ്ക്രീനിലെ എല്ലാ { $count } സംഭാഷണങ്ങളും തിരഞ്ഞെടുത്തു.
    }
   *[message] { $count ->
        [one] സ്ക്രീനിലെ { $count } സന്ദേശവും തിരഞ്ഞെടുത്തു.
       *[other] സ്ക്രീനിലെ എല്ലാ { $count } സന്ദേശങ്ങളും തിരഞ്ഞെടുത്തു.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } സംഭാഷണവും തിരഞ്ഞെടുക്കുക
       *[other] എല്ലാ { $count } സംഭാഷണങ്ങളും തിരഞ്ഞെടുക്കുക
    }
   *[message] { $count ->
        [one] { $count } സന്ദേശവും തിരഞ്ഞെടുക്കുക
       *[other] എല്ലാ { $count } സന്ദേശങ്ങളും തിരഞ്ഞെടുക്കുക
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder }-ലെ { $count } സംഭാഷണവും തിരഞ്ഞെടുക്കുക
       *[other] { $folder }-ലെ എല്ലാ { $count } സംഭാഷണങ്ങളും തിരഞ്ഞെടുക്കുക
    }
   *[message] { $count ->
        [one] { $folder }-ലെ { $count } സന്ദേശവും തിരഞ്ഞെടുക്കുക
       *[other] { $folder }-ലെ എല്ലാ { $count } സന്ദേശങ്ങളും തിരഞ്ഞെടുക്കുക
    }
}
list-clear-selection = തിരഞ്ഞെടുക്കൽ മായ്‌ക്കുക

## Mail list: empty states

list-empty-search = നിങ്ങളുടെ തിരയലുമായി പൊരുത്തപ്പെടുന്ന സന്ദേശങ്ങളൊന്നുമില്ല.
list-empty-tab = { $tab }-ൽ മെയിലൊന്നുമില്ല.
list-empty-tab-unknown = ഈ ടാബിൽ മെയിലൊന്നുമില്ല.
list-empty-folder = { $folder }-ൽ സന്ദേശങ്ങളൊന്നുമില്ല.
list-empty-folder-unknown = ഈ ഫോൾഡറിൽ സന്ദേശങ്ങളൊന്നുമില്ല.
list-first-sync = നിങ്ങളുടെ മെയിൽ ലഭ്യമാക്കുന്നു…
list-first-sync-detail = എത്തുന്നതിനനുസരിച്ച് ഇവിടെ ദൃശ്യമാകും.

## Mail list: lines

row-removed = ഈ സന്ദേശം നീക്കം ചെയ്‌തു.
row-starred = നക്ഷത്രമിട്ടത്
row-not-starred = നക്ഷത്രമിട്ടിട്ടില്ല
row-important = പ്രധാനപ്പെട്ടത്. പ്രധാനപ്പെട്ടതല്ലെന്ന് അടയാളപ്പെടുത്താൻ ക്ലിക്ക് ചെയ്യുക.
row-mark-important = പ്രധാനപ്പെട്ടതായി അടയാളപ്പെടുത്തുക
row-pinned = മുകളിൽ പിൻ ചെയ്‌തു
row-pin = മുകളിൽ പിൻ ചെയ്യുക
row-unpin = അൺപിൻ ചെയ്യുക

## Mail list: More menu and right-click menu

menu-reply = മറുപടി നൽകുക
menu-reply-all = എല്ലാവർക്കും മറുപടി നൽകുക
menu-forward = ഫോർവേഡ് ചെയ്യുക
menu-archive = ആർക്കൈവ് ചെയ്യുക
menu-delete = ഇല്ലാതാക്കുക
menu-spam = സ്‌പാം ആയി റിപ്പോർട്ട് ചെയ്യുക
menu-mark-read = വായിച്ചതായി അടയാളപ്പെടുത്തുക
menu-mark-unread = വായിക്കാത്തതായി അടയാളപ്പെടുത്തുക
menu-mark-all-read = എല്ലാം വായിച്ചതായി അടയാളപ്പെടുത്തുക
menu-star = നക്ഷത്രമിടുക
menu-unstar = നക്ഷത്രം നീക്കം ചെയ്യുക
menu-important = പ്രധാനപ്പെട്ടതായി അടയാളപ്പെടുത്തുക
menu-not-important = പ്രധാനപ്പെട്ടതല്ലെന്ന് അടയാളപ്പെടുത്തുക
menu-pin = മുകളിൽ പിൻ ചെയ്യുക
menu-unpin = അൺപിൻ ചെയ്യുക
menu-print-all = എല്ലാം പ്രിന്റ് ചെയ്യുക
menu-new-window = പുതിയ വിൻഡോയിൽ തുറക്കുക
menu-move-to = ഇതിലേക്ക് നീക്കുക
menu-move-to-heading = ഇതിലേക്ക് നീക്കുക:
menu-find-from = { $name } അയച്ച ഇമെയിലുകൾ കണ്ടെത്തുക

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] സംഭാഷണം ആർക്കൈവ് ചെയ്‌തു.
       *[other] { $count } സംഭാഷണങ്ങൾ ആർക്കൈവ് ചെയ്‌തു.
    }
   *[message] { $count ->
        [one] സന്ദേശം ആർക്കൈവ് ചെയ്‌തു.
       *[other] { $count } സന്ദേശങ്ങൾ ആർക്കൈവ് ചെയ്‌തു.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] സംഭാഷണം ട്രാഷിലേക്ക് നീക്കി.
       *[other] { $count } സംഭാഷണങ്ങൾ ട്രാഷിലേക്ക് നീക്കി.
    }
   *[message] { $count ->
        [one] സന്ദേശം ട്രാഷിലേക്ക് നീക്കി.
       *[other] { $count } സന്ദേശങ്ങൾ ട്രാഷിലേക്ക് നീക്കി.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] സംഭാഷണം നീക്കി.
       *[other] { $count } സംഭാഷണങ്ങൾ നീക്കി.
    }
   *[message] { $count ->
        [one] സന്ദേശം നീക്കി.
       *[other] { $count } സന്ദേശങ്ങൾ നീക്കി.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] സംഭാഷണത്തിന് നക്ഷത്രമിട്ടു.
       *[other] { $count } സംഭാഷണങ്ങൾക്ക് നക്ഷത്രമിട്ടു.
    }
   *[message] { $count ->
        [one] സന്ദേശത്തിന് നക്ഷത്രമിട്ടു.
       *[other] { $count } സന്ദേശങ്ങൾക്ക് നക്ഷത്രമിട്ടു.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] സംഭാഷണത്തിൽ നിന്ന് നക്ഷത്രം നീക്കി.
       *[other] { $count } സംഭാഷണങ്ങളിൽ നിന്ന് നക്ഷത്രം നീക്കി.
    }
   *[message] { $count ->
        [one] സന്ദേശത്തിൽ നിന്ന് നക്ഷത്രം നീക്കി.
       *[other] { $count } സന്ദേശങ്ങളിൽ നിന്ന് നക്ഷത്രം നീക്കി.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] സംഭാഷണം പ്രധാനപ്പെട്ടതായി അടയാളപ്പെടുത്തി.
       *[other] { $count } സംഭാഷണങ്ങൾ പ്രധാനപ്പെട്ടതായി അടയാളപ്പെടുത്തി.
    }
   *[message] { $count ->
        [one] സന്ദേശം പ്രധാനപ്പെട്ടതായി അടയാളപ്പെടുത്തി.
       *[other] { $count } സന്ദേശങ്ങൾ പ്രധാനപ്പെട്ടതായി അടയാളപ്പെടുത്തി.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] സംഭാഷണം പ്രധാനപ്പെട്ടതല്ലെന്ന് അടയാളപ്പെടുത്തി.
       *[other] { $count } സംഭാഷണങ്ങൾ പ്രധാനപ്പെട്ടതല്ലെന്ന് അടയാളപ്പെടുത്തി.
    }
   *[message] { $count ->
        [one] സന്ദേശം പ്രധാനപ്പെട്ടതല്ലെന്ന് അടയാളപ്പെടുത്തി.
       *[other] { $count } സന്ദേശങ്ങൾ പ്രധാനപ്പെട്ടതല്ലെന്ന് അടയാളപ്പെടുത്തി.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] സംഭാഷണം മുകളിൽ പിൻ ചെയ്‌തു.
       *[other] { $count } സംഭാഷണങ്ങൾ മുകളിൽ പിൻ ചെയ്‌തു.
    }
   *[message] { $count ->
        [one] സന്ദേശം മുകളിൽ പിൻ ചെയ്‌തു.
       *[other] { $count } സന്ദേശങ്ങൾ മുകളിൽ പിൻ ചെയ്‌തു.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] സംഭാഷണം അൺപിൻ ചെയ്‌തു.
       *[other] { $count } സംഭാഷണങ്ങൾ അൺപിൻ ചെയ്‌തു.
    }
   *[message] { $count ->
        [one] സന്ദേശം അൺപിൻ ചെയ്‌തു.
       *[other] { $count } സന്ദേശങ്ങൾ അൺപിൻ ചെയ്‌തു.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] സംഭാഷണം സ്‌പാം ആയി റിപ്പോർട്ട് ചെയ്‌തു.
       *[other] { $count } സംഭാഷണങ്ങൾ സ്‌പാം ആയി റിപ്പോർട്ട് ചെയ്‌തു.
    }
   *[message] { $count ->
        [one] സന്ദേശം സ്‌പാം ആയി റിപ്പോർട്ട് ചെയ്‌തു.
       *[other] { $count } സന്ദേശങ്ങൾ സ്‌പാം ആയി റിപ്പോർട്ട് ചെയ്‌തു.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] സംഭാഷണം ശാശ്വതമായി ഇല്ലാതാക്കി.
       *[other] { $count } സംഭാഷണങ്ങൾ ശാശ്വതമായി ഇല്ലാതാക്കി.
    }
   *[message] { $count ->
        [one] സന്ദേശം ശാശ്വതമായി ഇല്ലാതാക്കി.
       *[other] { $count } സന്ദേശങ്ങൾ ശാശ്വതമായി ഇല്ലാതാക്കി.
    }
}
toast-undone = പ്രവർത്തനം പഴയപടിയാക്കി.
toast-undo = പഴയപടിയാക്കുക
toast-no-spam-folder = ഈ അക്കൗണ്ടിന് സ്‌പാം ഫോൾഡർ ഇല്ല.

## Reading pane: toolbar

reader-close = അടയ്ക്കുക
reader-back = മടങ്ങുക
reader-mark-unread = വായിക്കാത്തതായി അടയാളപ്പെടുത്തുക
reader-move-to = ഇതിലേക്ക് നീക്കുക
reader-more = കൂടുതൽ
reader-print-all = എല്ലാം പ്രിന്റ് ചെയ്യുക
reader-new-window = പുതിയ വിൻഡോയിൽ
reader-position = { $total }-ൽ { $position }
reader-newer = പുതിയത്
reader-older = പഴയത്

## Reading pane: the conversation

reader-removed = ഈ സംഭാഷണം നീക്കം ചെയ്‌തു.
reader-no-subject = (വിഷയമില്ല)
reader-collapse-all = എല്ലാം ചുരുക്കുക
reader-expand-all = എല്ലാം വികസിപ്പിക്കുക
reader-unknown-sender = (അജ്ഞാത അയച്ചയാൾ)
reader-date-ago = { $date } ({ $ago })
reader-me = ഞാൻ
reader-to = സ്വീകർത്താവ്: { $names }
reader-starred = നക്ഷത്രമിട്ടത്
reader-not-starred = നക്ഷത്രമിട്ടിട്ടില്ല
reader-too-long = സന്ദേശം പൂർണ്ണമായി കാണിക്കാൻ കഴിയാത്തത്ര ദൈർഘ്യമേറിയതാണ്.
reader-encrypted-images = എൻക്രിപ്റ്റ് ചെയ്‌ത മെയിലിൽ വെബിൽ നിന്നുള്ള ചിത്രങ്ങൾ ഒരിക്കലും ലോഡ് ചെയ്യില്ല.
reader-window-failed = പുതിയ വിൻഡോ തുറക്കാനായില്ല.

## Reading pane: message details (opened from "to me")

reader-details-from = അയച്ചയാൾ:
reader-details-to = സ്വീകർത്താവ്:
reader-details-cc = cc:
reader-details-date = തീയതി:
reader-details-subject = വിഷയം:

## Reading pane: downloading a message

reader-downloading = ഈ സന്ദേശം സെർവറിൽ നിന്ന് ഡൗൺലോഡ് ചെയ്യുന്നു…
reader-download-failed = ഈ സന്ദേശം ഡൗൺലോഡ് ചെയ്യാനായില്ല.
reader-try-again = വീണ്ടും ശ്രമിക്കുക

## Reply row

reply-reply = മറുപടി നൽകുക
reply-reply-all = എല്ലാവർക്കും മറുപടി നൽകുക
reply-forward = ഫോർവേഡ് ചെയ്യുക

## Encrypted and signed mail

security-decrypting = ഡീക്രിപ്റ്റ് ചെയ്യുന്നു…
security-checking = ഒപ്പ് പരിശോധിക്കുന്നു…
security-partly-encrypted = ഈ സന്ദേശത്തിന്റെ ഒരു ഭാഗം മാത്രമേ എൻക്രിപ്റ്റ് ചെയ്‌തിട്ടുള്ളൂ. ബാക്കി ഭാഗം പരിരക്ഷയ്ക്ക് പുറത്ത് ചേർത്തതാണ്, അത് ആരിൽ നിന്നും വരാം.
security-partly-signed = ഈ സന്ദേശത്തിന്റെ ഒരു ഭാഗത്തിൽ മാത്രമേ ഒപ്പുള്ളൂ. ബാക്കി ഭാഗം പരിരക്ഷയ്ക്ക് പുറത്ത് ചേർത്തതാണ്, അത് ആരിൽ നിന്നും വരാം.
security-encrypted = എൻക്രിപ്റ്റ് ചെയ്‌ത സന്ദേശം
security-encrypted-smime = എൻക്രിപ്റ്റ് ചെയ്‌ത സന്ദേശം (S/MIME)
security-no-key = ഈ സന്ദേശം ഡീക്രിപ്റ്റ് ചെയ്യാനാകില്ല: നിങ്ങളുടെ പക്കൽ ഇല്ലാത്ത ഒരു കീയ്ക്കായാണ് ഇത് എൻക്രിപ്റ്റ് ചെയ്‌തത്.
security-cancelled = ഡീക്രിപ്റ്റ് ചെയ്യൽ റദ്ദാക്കി.
security-damaged = ഈ സന്ദേശം ഡീക്രിപ്റ്റ് ചെയ്യാനാകില്ല: എൻക്രിപ്റ്റ് ചെയ്‌ത ഡാറ്റ കേടായതോ മാറ്റം വരുത്തിയതോ ആണ്.
security-decrypt-unavailable = ഈ സന്ദേശം ഡീക്രിപ്റ്റ് ചെയ്യാനാകില്ല: എൻക്രിപ്റ്റ് ചെയ്‌ത മെയിൽ വായിക്കാൻ { $tool } ഇൻസ്റ്റാൾ ചെയ്യുക.
security-decrypt-failed = ഈ സന്ദേശം ഡീക്രിപ്റ്റ് ചെയ്യാനാകില്ല: { $reason }
security-unknown-signer = അജ്ഞാതനായ ഒപ്പിട്ടയാൾ
security-signed-verified = { $signer } ഒപ്പിട്ടത് · പരിശോധിച്ചുറപ്പിച്ചു
security-signed-not-sender = { $signer } ഒപ്പിട്ടത്, അയച്ചയാളല്ല
security-signed-untrusted = { $signer } ഒപ്പിട്ടത്, നിങ്ങൾ വിശ്വസനീയമല്ലെന്ന് അടയാളപ്പെടുത്തിയ കീ ഉപയോഗിച്ച്
security-signed-unverified = { $signer } ഒപ്പിട്ടത് · കീ പരിശോധിച്ചുറപ്പിച്ചിട്ടില്ല
security-bad-signature = തെറ്റായ ഒപ്പ്: ഒപ്പിട്ട ശേഷം ഈ സന്ദേശത്തിൽ മാറ്റം വരുത്തി, അല്ലെങ്കിൽ ഒപ്പ് വ്യാജമാണ്.
security-signature-expired = { $signer } ഒപ്പിട്ടത് · ഒപ്പിന്റെ കാലാവധി കഴിഞ്ഞു
security-key-expired = { $signer } ഒപ്പിട്ടത് · അതിനുശേഷം കീയുടെ കാലാവധി കഴിഞ്ഞു
security-key-revoked = { $signer } ഒപ്പിട്ടത്, അസാധുവാക്കിയ ഒരു കീ ഉപയോഗിച്ച്
security-missing-key = നിങ്ങളുടെ പക്കൽ ഇല്ലാത്ത ഒരു കീ ഉപയോഗിച്ച് ഒപ്പിട്ടതിനാൽ പരിശോധിക്കാനാകില്ല
security-missing-key-id = നിങ്ങളുടെ പക്കൽ ഇല്ലാത്ത ഒരു കീ ({ $key }) ഉപയോഗിച്ച് ഒപ്പിട്ടതിനാൽ പരിശോധിക്കാനാകില്ല
security-signature-unavailable = ഒപ്പിട്ടത്; ഒപ്പ് പരിശോധിക്കാൻ { $tool } ഇൻസ്റ്റാൾ ചെയ്യുക
security-signature-error = ഒപ്പ് പരിശോധിക്കാനായില്ല.

## Remote images and pictures

remote-hidden = ഈ സന്ദേശത്തിലെ ചിത്രങ്ങൾ മറച്ചിരിക്കുന്നു.
remote-show = ചിത്രങ്ങൾ കാണിക്കുക
remote-always-show = ഈ അയച്ചയാളിൽ നിന്നുള്ളവ എപ്പോഴും കാണിക്കുക
remote-picture-use = ഉപയോഗിക്കുക
remote-picture-too-big = 8 MB-യോ അതിൽ കുറവോ ഉള്ള ചിത്രം തിരഞ്ഞെടുക്കുക.
remote-picture-type = PNG, JPEG, GIF, WebP അല്ലെങ്കിൽ SVG ചിത്രം തിരഞ്ഞെടുക്കുക.
remote-picture-read-failed = ചിത്രം വായിക്കാനാകില്ല: { $error }
remote-picture-keep-failed = ചിത്രം സൂക്ഷിക്കാനാകില്ല: { $error }
remote-picture-remove-failed = ചിത്രം നീക്കം ചെയ്യാനാകില്ല: { $error }

## Attachments

attachment-count = { $count ->
    [one] ഒരു അറ്റാച്ച്‌മെന്റ്
   *[other] { $count } അറ്റാച്ച്‌മെന്റുകൾ
}
attachment-save = സംരക്ഷിക്കുക
attachment-save-all = എല്ലാം സംരക്ഷിക്കുക
attachment-save-all-tooltip = എല്ലാ അറ്റാച്ച്‌മെന്റുകളും ഒരു ഫോൾഡറിലേക്ക് സംരക്ഷിക്കുക
attachment-save-here = ഇവിടെ സംരക്ഷിക്കുക
attachment-not-downloaded = ഈ സന്ദേശം ഡൗൺലോഡ് ചെയ്‌തിട്ടില്ല.
attachment-not-found = ഈ അറ്റാച്ച്‌മെന്റ് സന്ദേശത്തിൽ കണ്ടെത്താനായില്ല.
attachment-read-failed = { $name } വായിക്കാനായില്ല
attachment-numbered = അറ്റാച്ച്‌മെന്റ് { $number }
attachment-saved-all = { $count ->
    [one] { $count } ഫയൽ { $place }-ലേക്ക് സംരക്ഷിച്ചു
   *[other] { $count } ഫയലുകൾ { $place }-ലേക്ക് സംരക്ഷിച്ചു
}
attachment-saved-some = { $total ->
    [one] { $total } ഫയലിൽ { $saved } എണ്ണം { $place }-ലേക്ക് സംരക്ഷിച്ചു. { $failed } സംരക്ഷിക്കാനായില്ല
   *[other] { $total } ഫയലുകളിൽ { $saved } എണ്ണം { $place }-ലേക്ക് സംരക്ഷിച്ചു. { $failed } സംരക്ഷിക്കാനായില്ല
}
attachment-saved-to = { $path }-ലേക്ക് സംരക്ഷിച്ചു
attachment-save-failed = { $name } സംരക്ഷിക്കാനായില്ല: { $error }
attachment-open-failed = { $name } തുറക്കാനായില്ല: { $error }
attachment-risky = ഈ ഫയലിന് ഒരു പ്രോഗ്രാം റൺ ചെയ്യാനാകും, അതിനാൽ Katna ഇത് തുറക്കില്ല. പകരം ഇത് സംരക്ഷിക്കുക.
attachment-encrypted-open = ഈ ഫയൽ എൻക്രിപ്റ്റ് ചെയ്‌താണ് വന്നത്. മറ്റെവിടെയെങ്കിലും തുറക്കാൻ ഇത് സംരക്ഷിക്കുക.

## Printing

print-failed = പ്രിന്റ് ചെയ്യാനായില്ല: { $error }
print-no-font = ഫോണ്ടൊന്നും കണ്ടെത്തിയില്ല
print-opened-as-pdf = അവിടെ നിന്ന് പ്രിന്റ് ചെയ്യാൻ PDF ആയി തുറന്നു.
print-not-downloaded = (ഇതുവരെ ഡൗൺലോഡ് ചെയ്‌തിട്ടില്ല.)
print-encrypted = (എൻക്രിപ്റ്റ് ചെയ്‌തത്. ഇതിന്റെ ടെക്സ്റ്റ് പ്രിന്റ് ചെയ്യാൻ Katna Mail-ൽ തുറക്കുക.)
print-to = സ്വീകർത്താവ്: { $addresses }
print-cc = Cc: { $addresses }
