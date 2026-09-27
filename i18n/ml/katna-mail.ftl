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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = അറ്റാച്ച്‌മെന്റുകൾ വായിക്കാൻ ഈ സന്ദേശം തുറക്കുക.
text-copy = പകർത്തുക
text-select-all = എല്ലാം തിരഞ്ഞെടുക്കുക

## Settings page: its tabs

settings-tab-general = പൊതുവായത്
settings-tab-inbox = ഇൻബോക്‌സ്
settings-tab-accounts = അക്കൗണ്ടുകൾ
settings-tab-subscriptions = സബ്‌സ്‌ക്രിപ്ഷനുകൾ
settings-tab-appearance = രൂപഭാവം
settings-tab-shortcuts = കുറുക്കുവഴികൾ
settings-tab-default-apps = ഡിഫോൾട്ട് ആപ്പുകൾ
settings-tab-folders-rules = ഫോൾഡറുകളും നിയമങ്ങളും
settings-tab-compose = രചന
settings-tab-mcp-server = MCP സെർവർ
settings-tab-feedback = ഉപയോക്തൃ ഫീഡ്‌ബാക്ക്
settings-tab-experimental = പരീക്ഷണാത്മകം

## Settings page: tabs still to come

settings-tab-subscriptions-coming = നിങ്ങൾക്ക് ലഭിക്കുന്ന വാർത്താക്കുറിപ്പുകളും മെയിലിംഗ് ലിസ്റ്റുകളും കാണുക, ഒറ്റ ക്ലിക്കിൽ അൺസബ്‌സ്‌ക്രൈബ് ചെയ്യുക.
settings-tab-folders-rules-coming = ഫോൾഡറുകളും ലേബലുകളും സൃഷ്‌ടിക്കുക, പേരുമാറ്റുക, നീക്കുക, മറയ്ക്കുക, ഏതൊക്കെ സമന്വയിപ്പിക്കണമെന്ന് തിരഞ്ഞെടുക്കുക. നിയമങ്ങൾ പുതിയ മെയിലിനെ അയച്ചയാൾ, വിഷയം അല്ലെങ്കിൽ വാക്കുകൾ അനുസരിച്ച് സ്വയമേവ അടുക്കുകയോ ലേബൽ ചെയ്യുകയോ ഫോർവേഡ് ചെയ്യുകയോ ഇല്ലാതാക്കുകയോ ചെയ്യും.
settings-tab-mcp-server-coming = ഈ കമ്പ്യൂട്ടറിലെ AI അസിസ്റ്റന്റുകളെ നിങ്ങളുടെ അനുമതിയോടെ നിങ്ങളുടെ മെയിൽ തിരയാനും വായിക്കാനും ഡ്രാഫ്റ്റ് ചെയ്യാനും അനുവദിക്കുക.

## Settings > General

settings-general-conversations = സംഭാഷണ കാഴ്‌ച
settings-general-conversations-group = ഒരേ മെയിലിനുള്ള മറുപടികൾ ഗ്രൂപ്പ് ചെയ്യുക
settings-general-conversations-group-detail = ലിസ്റ്റിൽ ഓരോ സംഭാഷണത്തിനും ഒരു വരി
settings-general-reading = വായന
settings-general-newest-first = ഏറ്റവും പുതിയ സന്ദേശം ആദ്യം
settings-general-newest-first-detail = സംഭാഷണം അതിന്റെ ഏറ്റവും പുതിയ മറുപടിയിൽ തുടങ്ങുന്നു
settings-general-full-headers = പൂർണ്ണ ഹെഡറുകൾ കാണിക്കുക
settings-general-full-headers-detail = എല്ലാ സന്ദേശത്തിലും അയച്ചയാൾ, സ്വീകർത്താവ്, cc, തീയതി, വിഷയം എന്നിവ തുറന്നിരിക്കും
settings-general-full-names = സ്വീകർത്താക്കളുടെ മുഴുവൻ പേരുകൾ
settings-general-full-names-detail = “ഞാൻ, Ada” എന്നതിന് പകരം “ഞാൻ, Ada Lovelace”
settings-general-mark-read = വായിച്ചതായി അടയാളപ്പെടുത്തൽ
settings-general-mark-read-now = തുറന്നയുടൻ
settings-general-mark-read-1s = 1 സെക്കൻഡ് തുറന്നിരുന്ന ശേഷം
settings-general-mark-read-3s = 3 സെക്കൻഡ് തുറന്നിരുന്ന ശേഷം
settings-general-mark-read-never = ഞാൻ അടയാളപ്പെടുത്തുമ്പോൾ മാത്രം
settings-general-reply-button = മറുപടി ബട്ടൺ
settings-general-reply-all = എല്ലാവർക്കും മറുപടി നൽകുക
settings-general-reply-all-detail = ഓരോ സന്ദേശത്തിനും അടുത്തുള്ള മറുപടി ബട്ടൺ അയച്ചയാൾക്ക് മാത്രമല്ല, എല്ലാവർക്കും മറുപടി നൽകും
settings-general-remote-images = വെബിൽ നിന്നുള്ള ചിത്രങ്ങൾ
settings-general-remote-images-detail = ഒരു സന്ദേശത്തിലെ ചിത്രങ്ങൾ ലോഡ് ചെയ്‌താൽ, നിങ്ങൾ അത് തുറന്നുവെന്നും എപ്പോഴെന്നും ഏകദേശം എവിടെ നിന്നാണെന്നും അയച്ചയാൾ അറിയും. ഓഫാണെങ്കിൽ, ഓരോ സന്ദേശവും ആദ്യം ചോദിക്കും, ഒരു അയച്ചയാളുടെ ചിത്രങ്ങൾ നിങ്ങൾക്ക് എപ്പോഴും കാണിക്കാം.
settings-general-remote-images-always = ചിത്രങ്ങൾ എപ്പോഴും കാണിക്കുക
settings-general-remote-images-always-detail = നിങ്ങൾ വിശ്വസിക്കുന്ന അയച്ചവരിൽ നിന്നുള്ളവയിൽ മാത്രമല്ല, എല്ലാ സന്ദേശത്തിലും
settings-general-sending = അയയ്ക്കൽ
settings-general-sending-detail = അയച്ച സന്ദേശം തിരിച്ചെടുക്കാൻ കഴിയുന്ന തരത്തിൽ, അത് എത്ര നേരം കാത്തിരിക്കണം.
settings-general-offline = ഓഫ്‌ലൈൻ മെയിൽ
settings-general-offline-detail = കണക്ഷനില്ലാതെ വായിക്കാൻ, സമീപകാല മെയിൽ പൂർണ്ണമായി ഡൗൺലോഡ് ചെയ്യും. പഴയ മെയിൽ നിങ്ങൾ തുറക്കുമ്പോൾ ഡൗൺലോഡ് ചെയ്യും.
settings-general-offline-days = { $count ->
    [one] { $count } ദിവസം
   *[other] { $count } ദിവസം
}
settings-general-offline-years = { $count ->
    [one] { $count } വർഷം
   *[other] { $count } വർഷം
}
settings-general-offline-all = എല്ലാ മെയിലുകളും
settings-general-offline-note = കുറച്ച് ദിവസങ്ങൾ തിരഞ്ഞെടുത്താലും, ഇതിനകം ഡൗൺലോഡ് ചെയ്‌ത മെയിൽ അതുപോലെ നിലനിൽക്കും. സെർവറിൽ ഒന്നും മാറില്ല.
settings-general-notifications = അറിയിപ്പുകൾ
settings-general-notifications-detail = ഇൻബോക്‌സിലെ പുതിയ മെയിലിന്, Katna Mail അടച്ചിരിക്കുമ്പോഴും.
settings-general-new-mail = പുതിയ മെയിലിനെക്കുറിച്ച് എന്നെ അറിയിക്കുക
settings-general-new-mail-detail = എല്ലാവർക്കും മറുപടി, വായിച്ചതായി അടയാളപ്പെടുത്തുക, ആർക്കൈവ് എന്നിവയോടൊപ്പം
settings-general-new-mail-sound = ശബ്‌ദം പ്ലേ ചെയ്യുക
settings-general-new-mail-sound-detail = ഡെസ്‌ക്‌ടോപ്പിന്റെ പുതിയ മെയിൽ ശബ്‌ദം
settings-general-desktop = ഡെസ്‌ക്‌ടോപ്പ്
settings-general-open-at-login = ലോഗിൻ ചെയ്യുമ്പോൾ Katna Mail തുറക്കുക
settings-general-open-at-login-detail = സേവനം പ്രവർത്തിക്കുന്നിടത്തോളം, ഏതായാലും ലോഗിൻ ചെയ്യുമ്പോൾ മെയിൽ സമന്വയിപ്പിക്കും
settings-general-tray = സിസ്റ്റം ട്രേയിൽ Katna കാണിക്കുക
settings-general-tray-detail = വായിക്കാത്തവയുടെ എണ്ണവും ഒരു മെനുവും സഹിതം
settings-general-unread-badge = ടാസ്‌ക്‌ബാർ ഐക്കണിൽ വായിക്കാത്തവയുടെ എണ്ണം
settings-general-unread-badge-detail = ഇൻബോക്‌സിലെ എത്ര സന്ദേശങ്ങൾ വായിച്ചിട്ടില്ല

## Settings > Inbox

settings-inbox-tabs = ഇൻബോക്‌സ് ടാബുകൾ
settings-inbox-tabs-detail = നിങ്ങളുടെ മെയിൽ ദാതാവിന്റെ വെബ്‌സൈറ്റിലെന്നപോലെ, ഇൻബോക്‌സിനെ ടാബുകളായി തരംതിരിക്കുക.
settings-inbox-tabs-show = ഇൻബോക്‌സ് ടാബുകൾ കാണിക്കുക
settings-inbox-tabs-show-detail = ഓഫാണെങ്കിൽ, ഓരോ അക്കൗണ്ടിനും ഒരൊറ്റ ലിസ്റ്റ്
settings-inbox-no-accounts = ടാബുകൾ തിരഞ്ഞെടുക്കാൻ ഒരു അക്കൗണ്ട് ചേർക്കുക.
settings-inbox-tabs-automatic = സ്വയമേവ: { $tabs } ({ $provider })
settings-inbox-tabs-off = ടാബുകളില്ല
settings-inbox-tabs-gmail = പ്രാഥമികം, പ്രമോഷനുകൾ, സോഷ്യൽ, അപ്‌ഡേറ്റുകൾ, ഫോറങ്ങൾ
settings-inbox-tabs-focused = ഫോക്കസ് ചെയ്‌തവയും മറ്റുള്ളവയും
settings-inbox-tabs-zoho = ഇൻബോക്‌സ്, വാർത്താക്കുറിപ്പുകൾ, അറിയിപ്പുകൾ
settings-inbox-tabs-shown = കാണിക്കുന്ന ടാബുകൾ. നിങ്ങൾ ഓഫാക്കുന്ന ടാബിലെ മെയിൽ { $tab }-ൽ തുടരും.

## Settings > Appearance

settings-appearance-reading-pane = വായനാ പാളി
settings-appearance-reading-pane-detail = തുറന്ന സംഭാഷണം എവിടെ കാണിക്കുന്നു.
settings-appearance-pane-right = ലിസ്റ്റിന്റെ വലതുവശത്ത്
settings-appearance-pane-none = വിഭജനമില്ല
settings-appearance-density = സാന്ദ്രത
settings-appearance-density-default = ഡിഫോൾട്ട്
settings-appearance-density-compact = ഒതുക്കമുള്ളത്
settings-appearance-scaling = സ്കെയിലിംഗ്
settings-appearance-scaling-detail = ഡെസ്‌ക്‌ടോപ്പിന്റെ സ്വന്തം സ്കെയിലിന് പുറമെ, Katna Mail-ലെ എല്ലാം വലുതോ ചെറുതോ ആക്കുന്നു: ടെക്സ്റ്റ്, ഐക്കണുകൾ, സ്പേസിംഗ്, ഡിവൈഡറുകൾ. നിങ്ങൾ അയയ്ക്കുന്ന മെയിൽ അതിന്റെ സ്വന്തം ഫോണ്ട് വലുപ്പം നിലനിർത്തും. വളരെ ചെറിയ വലുപ്പങ്ങളിൽ ഐക്കണുകളിൽ ക്ലിക്ക് ചെയ്യാൻ ബുദ്ധിമുട്ടായേക്കാം.
settings-appearance-theme = തീം
settings-appearance-theme-system = ഡെസ്‌ക്‌ടോപ്പിലേതുപോലെ
settings-appearance-theme-light = ലൈറ്റ്
settings-appearance-theme-dark = ഡാർക്ക്
settings-appearance-desktop-colors = ഡെസ്‌ക്‌ടോപ്പ് നിറങ്ങൾ
settings-appearance-desktop-colors-use = ഡെസ്‌ക്‌ടോപ്പിന്റെ നിറങ്ങൾ ഉപയോഗിക്കുക
settings-appearance-desktop-colors-use-detail = ഡെസ്‌ക്‌ടോപ്പിന്റെ വർണ്ണ സ്കീമും ആക്സന്റ് നിറവും
settings-appearance-app-names = ആപ്പ് പേരുകൾ
settings-appearance-app-names-show = ആപ്പ് പേരുകൾ കാണിക്കുക
settings-appearance-app-names-show-detail = ഇടത്തേ അറ്റത്തുള്ള ആപ്പ് ഐക്കണുകൾക്ക് താഴെ പേരുകൾ
settings-appearance-sender-pictures = അയച്ചയാളുടെ ചിത്രങ്ങൾ
settings-appearance-sender-pictures-show = കമ്പനി ലോഗോകൾ കാണിക്കുക
settings-appearance-sender-pictures-show-detail = ഒരിക്കലും സന്ദേശം വഴിയല്ല, അയച്ചയാളുടെ ഡൊമെയ്‌ൻ വഴി കണ്ടെത്തി, ഒരാഴ്‌ചത്തേക്ക് സൂക്ഷിക്കുന്നു
settings-appearance-important = പ്രധാനപ്പെട്ടത് മാർക്കറുകൾ
settings-appearance-important-show = പ്രധാനപ്പെട്ടത് മാർക്കറുകൾ കാണിക്കുക
settings-appearance-important-show-detail = ലിസ്റ്റിലെ ഓരോ സന്ദേശത്തിനും അടുത്ത്
settings-appearance-message-width = സന്ദേശത്തിന്റെ വീതി
settings-appearance-message-width-limit = സന്ദേശങ്ങളുടെ വീതി പരിമിതപ്പെടുത്തുക
settings-appearance-message-width-limit-detail = വീതിയുള്ള വിൻഡോയിൽ നീളമുള്ള വരികൾ വായിക്കാൻ എളുപ്പമാകും
settings-appearance-mail-colors = മെയിൽ നിറങ്ങൾ
settings-appearance-mail-colors-detail = മിക്ക മെയിലുകളും വെള്ള പേജിനായി രൂപകൽപ്പന ചെയ്‌തവയാണ്. ഡാർക്ക് തീമിൽ അവയുടെ നിറങ്ങൾ നന്നായി വായിക്കാവുന്ന ഇരുണ്ട നിറങ്ങളിലേക്ക് മാറ്റും; ഓഫാണെങ്കിൽ, ഇളം പേജിൽ അയച്ചയാളുടെ നിറങ്ങൾ തന്നെ നിലനിൽക്കും.
settings-appearance-dark-mail = മെയിലിനും ഇരുണ്ട നിറങ്ങൾ
settings-appearance-dark-mail-detail = തീം ഡാർക്ക് ആയിരിക്കുമ്പോൾ മാത്രം
settings-appearance-attachment-previews = അറ്റാച്ച്‌മെന്റ് പ്രിവ്യൂകൾ
settings-appearance-attachment-previews-show = അറ്റാച്ച്‌മെന്റുകളുടെ പ്രിവ്യൂകൾ കാണിക്കുക
settings-appearance-attachment-previews-show-detail = ഓരോ ഫയലിന്റെയും ഉള്ളടക്കത്തിന്റെ ചെറിയ ചിത്രം, അതിന്റെ കാർഡിൽ

## Settings > Default apps

settings-default-apps-intro = അറ്റാച്ച്‌മെന്റുകളിൽ ക്ലിക്ക് ചെയ്യുമ്പോൾ അവ എവിടെ തുറക്കണം. വ്യൂവറിൽ നിന്ന് ഒരു ഫയൽ എപ്പോഴും മറ്റൊരു ആപ്പിലും തുറക്കാം. ഡെസ്‌ക്‌ടോപ്പിന്റെ ഡിഫോൾട്ട് ആപ്പുകൾ അതിന്റെ സ്വന്തം ക്രമീകരണത്തിലാണ് സജ്ജീകരിക്കുന്നത്.
settings-default-apps-pdf = PDF ഫയലുകൾ
settings-default-apps-pdf-detail = പേജുകൾ, സൂം സഹിതം.
settings-default-apps-pictures = ചിത്രങ്ങൾ
settings-default-apps-pictures-detail = ഫോട്ടോകൾ (നേരെയാക്കിയവ), PNG, GIF, WebP, BMP, TIFF, SVG.
settings-default-apps-text = ടെക്സ്റ്റ് ഫയലുകൾ
settings-default-apps-text-detail = പ്ലെയിൻ ടെക്സ്റ്റ്, ലോഗുകൾ, കോഡ്, മറ്റ് ടെക്സ്റ്റ്.
settings-default-apps-sheets = സ്‌പ്രെഡ്‌ഷീറ്റുകൾ
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods), CSV.
settings-default-apps-documents = ഡോക്യുമെന്റുകൾ
settings-default-apps-documents-detail = Word (docx), OpenDocument ടെക്സ്റ്റ് (odt).
settings-default-apps-katna = Katna Mail-ന്റെ വ്യൂവർ
settings-default-apps-system = ഡെസ്‌ക്‌ടോപ്പിന്റെ ഡിഫോൾട്ട് ആപ്പ്
settings-default-apps-ask = ഓരോ തവണയും ഏത് ആപ്പ് എന്ന് ചോദിക്കുക
settings-default-apps-after-saving = സംരക്ഷിച്ച ശേഷം
settings-default-apps-show-folder = സംരക്ഷിച്ച ഫയലുകൾ അവയുടെ ഫോൾഡറിൽ കാണിക്കുക
settings-default-apps-show-folder-detail = സംരക്ഷിച്ച അറ്റാച്ച്‌മെന്റുകൾ തിരഞ്ഞെടുത്ത നിലയിൽ ഫയൽ മാനേജർ തുറക്കുന്നു

## Settings > Compose

settings-compose-send-from = പുതിയ സന്ദേശങ്ങൾ ഇതിൽ നിന്ന് അയയ്ക്കുക
settings-compose-send-from-detail = മറുപടികളും ഫോർവേഡുകളും എപ്പോഴും നിങ്ങൾ ഉള്ള അക്കൗണ്ടിൽ നിന്നാണ് പോകുന്നത്.
settings-compose-send-from-current = നിങ്ങൾ ഉള്ള അക്കൗണ്ട്
settings-compose-send-on-replies = മറുപടികളിലെ അയയ്ക്കുക ബട്ടൺ
settings-compose-send-on-replies-detail = മറുപടിയിലോ ഫോർവേഡിലോ അയയ്ക്കുക ബട്ടൺ എന്ത് ചെയ്യുന്നു. അയയ്ക്കുക ബട്ടണിന് അടുത്തുള്ള മെനുവിൽ മറ്റേത് ലഭ്യമാണ്.
settings-compose-send-plain = അയയ്ക്കുക
settings-compose-send-archive = അയച്ച് ആർക്കൈവ് ചെയ്യുക
settings-compose-signatures = ഒപ്പുകൾ
settings-compose-signatures-detail = നിങ്ങളുടെ സന്ദേശത്തിന് താഴെ, ഒരു “--” വരിക്ക് ശേഷം ചേർക്കുന്നു. രചനാ വിൻഡോയിൽ മറ്റൊന്ന് തിരഞ്ഞെടുക്കാം.
settings-compose-untitled = പേരില്ലാത്തത്
settings-compose-signature-name = പേര്, ഉദാ. ജോലി
settings-compose-signature-first = എന്റെ ഒപ്പ്
settings-compose-signature-numbered = ഒപ്പ് { $number }
settings-compose-signature-delete = ഇല്ലാതാക്കുക
settings-compose-signature-deleted = ഒപ്പ് ഇല്ലാതാക്കി
settings-compose-signature-new = പുതിയത് സൃഷ്‌ടിക്കുക
settings-compose-no-signatures = ഇതുവരെ ഒപ്പുകളൊന്നുമില്ല.
settings-compose-no-signature = ഒപ്പില്ല
settings-compose-for-new-mail = പുതിയ മെയിലിന്
settings-compose-for-replies = മറുപടികൾക്കും ഫോർവേഡുകൾക്കും
settings-compose-for-replies-detail = നിങ്ങൾ ഒരു സന്ദേശത്തിൽ ഒപ്പിട്ട സംഭാഷണത്തിൽ, മറുപടി പകരം ആ ഒപ്പോടെ തുടങ്ങും.
settings-compose-format = ഫോർമാറ്റ്
settings-compose-plain-text = പ്ലെയിൻ ടെക്സ്റ്റിൽ എഴുതുക
settings-compose-plain-text-detail = പുതിയ മെയിൽ ഫോർമാറ്റിംഗ് ഇല്ലാതെ തുടങ്ങും; രചനാ വിൻഡോയിൽ മാറ്റാം
settings-compose-spelling = അക്ഷരത്തെറ്റ്
settings-compose-spell-check = ഞാൻ എഴുതുമ്പോൾ അക്ഷരത്തെറ്റ് പരിശോധിക്കുക
settings-compose-spell-check-detail = തെറ്റായ വാക്കുകൾക്ക് അടിവരയിടും, റൈറ്റ് ക്ലിക്കിൽ നിർദ്ദേശങ്ങളോടെ
settings-compose-spell-desktop = ഡെസ്‌ക്‌ടോപ്പിന്റെ ഭാഷ ({ $language })
settings-compose-templates = ടെംപ്ലേറ്റുകൾ
settings-compose-templates-detail = നിങ്ങൾ പതിവായി എഴുതുന്ന മെയിൽ സംരക്ഷിച്ച്, അതിൽ നിന്ന് പുതിയ മെയിലോ മറുപടിയോ തുടങ്ങുക.

## Settings > Shortcuts

settings-shortcuts-set = കുറുക്കുവഴി സെറ്റ്
settings-shortcuts-set-detail = നിങ്ങൾക്ക് പരിചയമുള്ള മെയിൽ ആപ്പിന്റെ കീകളിൽ നിന്ന് തുടങ്ങുക. ഇവിടെ Cmd എന്നാൽ Ctrl ആണ്. നിങ്ങളുടെ സ്വന്തം മാറ്റങ്ങൾ സെറ്റിന് മുകളിൽ നിലനിൽക്കും, ഡിഫോൾട്ടുകൾ പുനഃസ്ഥാപിക്കുക സെറ്റിന്റെ കീകളിലേക്ക് മടങ്ങും.
settings-shortcuts-single = ഒറ്റ-കീ കുറുക്കുവഴികൾ
settings-shortcuts-single-detail = വെബ്‌മെയിലിലെന്നപോലെ, Ctrl അല്ലെങ്കിൽ Alt ഇല്ലാത്ത കീകൾ: e ആർക്കൈവ് ചെയ്യുന്നു, j, k നീക്കുന്നു, / തിരയുന്നു. ഇവ ലിസ്റ്റിലും തുറന്ന സംഭാഷണത്തിലും പ്രവർത്തിക്കും, ടൈപ്പ് ചെയ്യുമ്പോൾ ഒരിക്കലുമില്ല.
settings-shortcuts-single-use = ഒറ്റ-കീ കുറുക്കുവഴികൾ ഉപയോഗിക്കുക
settings-shortcuts-single-use-detail = Ctrl കുറുക്കുവഴികൾ എപ്പോഴും പ്രവർത്തിക്കും
settings-shortcuts-how = ഒരു കീ മാറ്റാൻ അതിൽ ക്ലിക്ക് ചെയ്യുക, അല്ലെങ്കിൽ പുതിയത് ചേർക്കാൻ + ക്ലിക്ക് ചെയ്യുക, തുടർന്ന് പുതിയ കീകൾ അമർത്തുക. Esc റദ്ദാക്കുന്നു.
settings-shortcuts-restore = ഡിഫോൾട്ടുകൾ പുനഃസ്ഥാപിക്കുക
settings-shortcuts-no-key = കീ ഇല്ല
settings-shortcuts-press = കീകൾ അമർത്തുക…
settings-shortcuts-then = { $keys } തുടർന്ന്…
settings-shortcuts-moved = { $keys } ഇപ്പോൾ “{ $previous }” എന്നതിന് പകരം “{ $action }” ചെയ്യുന്നു.
settings-shortcuts-single-off = ഒറ്റ-കീ കുറുക്കുവഴികൾ ഓഫാണ്, അതിനാൽ അവ ഓണാക്കിയ ശേഷമേ ഈ കീ പ്രവർത്തിക്കൂ.
settings-shortcuts-restored = ഓരോ കുറുക്കുവഴിക്കും വീണ്ടും അതിന്റെ സെറ്റിലെ കീകൾ ലഭിച്ചു.

## Settings search: the line under a result

settings-general-language-summary = ആപ്പിന്റെയും തീയതികളുടെയും നമ്പറുകളുടെയും ഭാഷ
settings-general-reading-summary = ഏറ്റവും പുതിയ സന്ദേശം ആദ്യം, പൂർണ്ണ ഹെഡറുകൾ, സ്വീകർത്താക്കളുടെ മുഴുവൻ പേരുകൾ
settings-general-mark-read-summary = തുറന്ന സംഭാഷണം എപ്പോൾ വായിച്ചതായി അടയാളപ്പെടുത്തുന്നു: ഉടൻ, 1 അല്ലെങ്കിൽ 3 സെക്കൻഡിന് ശേഷം, അല്ലെങ്കിൽ സ്വമേധയാ
settings-general-reply-button-summary = ഓരോ സന്ദേശത്തിനും അടുത്തുള്ള മറുപടി ബട്ടൺ എല്ലാവർക്കും മറുപടി നൽകുന്നു
settings-general-remote-images-summary = എല്ലാ സന്ദേശത്തിലെയും ചിത്രങ്ങൾ എപ്പോഴും കാണിക്കുക
settings-general-sending-summary = അയയ്ക്കൽ പഴയപടിയാക്കുക: അയച്ച സന്ദേശം തിരിച്ചെടുക്കാൻ കഴിയുന്ന തരത്തിൽ അത് എത്ര നേരം കാത്തിരിക്കണം
settings-general-offline-summary = കണക്ഷനില്ലാതെ വായിക്കാൻ, എത്ര ദിവസത്തെ സമീപകാല മെയിൽ പൂർണ്ണമായി ഡൗൺലോഡ് ചെയ്യണം
settings-general-notifications-summary = പുതിയ മെയിൽ അറിയിപ്പുകളും അവയുടെ ശബ്‌ദവും
settings-general-desktop-summary = ലോഗിൻ ചെയ്യുമ്പോൾ Katna Mail തുറക്കൽ, സിസ്റ്റം ട്രേ ഐക്കൺ, ടാസ്‌ക്‌ബാർ ഐക്കണിലെ വായിക്കാത്തവയുടെ എണ്ണം
settings-accounts-accounts-summary = ഒരു അക്കൗണ്ട് ചേർക്കുകയോ നീക്കം ചെയ്യുകയോ അതിന്റെ ചിത്രം മാറ്റുകയോ ചെയ്യുക
settings-appearance-density-summary = ലിസ്റ്റിൽ ഡിഫോൾട്ട് അല്ലെങ്കിൽ ഒതുക്കമുള്ള വരികൾ
settings-appearance-scaling-summary = എല്ലാം വലുതോ ചെറുതോ ആക്കുക: ടെക്സ്റ്റ്, ഐക്കണുകൾ, സ്പേസിംഗ്, ഡിവൈഡറുകൾ
settings-appearance-theme-summary = ഡെസ്‌ക്‌ടോപ്പിലേതുപോലെ, ലൈറ്റ് അല്ലെങ്കിൽ ഡാർക്ക്
settings-appearance-sender-pictures-summary = അയച്ചയാളുടെ ഡൊമെയ്‌ൻ വഴി കണ്ടെത്തുന്ന കമ്പനി ലോഗോകൾ
settings-appearance-important-summary = ലിസ്റ്റിലെ ഓരോ സന്ദേശത്തിനും അടുത്തുള്ള പ്രധാനപ്പെട്ടത് മാർക്കർ
settings-appearance-mail-colors-summary = ഡാർക്ക് തീമിൽ HTML മെയിലിന് ഇരുണ്ട നിറങ്ങൾ, അല്ലെങ്കിൽ അയച്ചയാളുടെ നിറങ്ങൾ
settings-appearance-attachment-previews-summary = ഓരോ അറ്റാച്ച്‌മെന്റിന്റെയും ഉള്ളടക്കത്തിന്റെ ചെറിയ ചിത്രം
settings-shortcuts-set-summary = Gmail, Inbox by Gmail, Apple Mail, Outlook അല്ലെങ്കിൽ Thunderbird-ന്റെ കീകളിൽ നിന്ന് തുടങ്ങുക
settings-shortcuts-single-summary = വെബ്‌മെയിലിലെന്നപോലെ, Ctrl അല്ലെങ്കിൽ Alt ഇല്ലാത്ത കീകൾ
settings-default-apps-pdf-summary = PDF അറ്റാച്ച്‌മെന്റുകൾ എവിടെ തുറക്കണം
settings-default-apps-pictures-summary = ഫോട്ടോകളും ചിത്രങ്ങളും എവിടെ തുറക്കണം
settings-default-apps-text-summary = പ്ലെയിൻ ടെക്സ്റ്റ്, ലോഗുകൾ, കോഡ് എന്നിവ എവിടെ തുറക്കണം
settings-default-apps-sheets-summary = Excel, OpenDocument, CSV ഫയലുകൾ എവിടെ തുറക്കണം
settings-default-apps-documents-summary = Word, OpenDocument ടെക്സ്റ്റ് എന്നിവ എവിടെ തുറക്കണം
settings-default-apps-after-saving-summary = സംരക്ഷിച്ച അറ്റാച്ച്‌മെന്റുകൾ അവയുടെ ഫോൾഡറിൽ കാണിക്കുക
settings-compose-send-from-summary = പുതിയ മെയിൽ പോകുന്ന അക്കൗണ്ട്: നിങ്ങൾ ഉള്ളത്, അല്ലെങ്കിൽ എപ്പോഴും ഒരേ അക്കൗണ്ട്
settings-compose-send-on-replies-summary = മറുപടികളിലും ഫോർവേഡുകളിലും അയയ്ക്കുക, അല്ലെങ്കിൽ അയച്ച് സംഭാഷണം ആർക്കൈവ് ചെയ്യുക
settings-compose-signatures-summary = നിങ്ങളുടെ സന്ദേശത്തിന് താഴെ, ഒരു “--” വരിക്ക് ശേഷം ചേർക്കുന്നു
settings-compose-for-new-mail-summary = പുതിയ മെയിൽ തുടങ്ങുന്ന ഒപ്പ്
settings-compose-for-replies-summary = മറുപടികളും ഫോർവേഡുകളും തുടങ്ങുന്ന ഒപ്പ്
settings-compose-format-summary = പുതിയ മെയിൽ പ്ലെയിൻ ടെക്സ്റ്റിൽ എഴുതുക
settings-compose-spelling-summary = എഴുതുമ്പോൾ അക്ഷരത്തെറ്റ് പരിശോധന, നിഘണ്ടുവിന്റെ ഭാഷ
settings-compose-templates-summary = ഉടൻ വരുന്നു: പതിവായി എഴുതുന്ന മെയിൽ സംരക്ഷിച്ച്, അതിൽ നിന്ന് പുതിയ മെയിലോ മറുപടിയോ തുടങ്ങുക
settings-feedback-crash-reports-summary = Katna Mail അല്ലെങ്കിൽ അതിന്റെ പശ്ചാത്തല സേവനം ക്രാഷ് ആകുമ്പോൾ ക്രാഷ് റിപ്പോർട്ടുകൾ ഈ കമ്പ്യൂട്ടറിൽ സംരക്ഷിക്കുക
settings-feedback-saved-summary = ഈ കമ്പ്യൂട്ടറിൽ സംരക്ഷിച്ച ക്രാഷ് റിപ്പോർട്ടുകൾ കാണുക, പകർത്തുക അല്ലെങ്കിൽ ഇല്ലാതാക്കുക
settings-feedback-help-improve-summary = എന്താണ് തെറ്റിയതെന്ന് പരിഹരിക്കാൻ സഹായിക്കാൻ ക്രാഷ് റിപ്പോർട്ടുകൾ അയയ്ക്കുക; നിങ്ങൾ ഓണാക്കിയില്ലെങ്കിൽ ഓഫായിരിക്കും
settings-experimental-blur-summary = മുകളിലെ ബാറിലൂടെ ഡെസ്‌ക്‌ടോപ്പ് മങ്ങിയതായി കാണാം, മെനുകൾ മഞ്ഞുഗ്ലാസ് പോലെയാകും
settings-search-shortcut = കീബോർഡ് കുറുക്കുവഴി
settings-search-tab = ക്രമീകരണ ടാബ്
settings-search-none = “{ $query }” എന്നതുമായി പൊരുത്തപ്പെടുന്ന ക്രമീകരണങ്ങളൊന്നുമില്ല.
settings-search-results = “{ $query }” എന്നതുമായി പൊരുത്തപ്പെടുന്ന ക്രമീകരണങ്ങൾ

## Quick settings (the panel that slides in from the right)

quick-title = ദ്രുത ക്രമീകരണം
quick-see-all = എല്ലാ ക്രമീകരണങ്ങളും കാണുക
quick-reading-pane = വായനാ പാളി
quick-pane-right = ലിസ്റ്റിന്റെ വലതുവശത്ത്
quick-pane-none = വിഭജനമില്ല
quick-density = സാന്ദ്രത
quick-density-default = ഡിഫോൾട്ട്
quick-density-compact = ഒതുക്കമുള്ളത്
quick-theme = തീം
quick-theme-system = ഡെസ്‌ക്‌ടോപ്പിലേതുപോലെ
quick-theme-light = ലൈറ്റ്
quick-theme-dark = ഡാർക്ക്
quick-desktop-colors = ഡെസ്‌ക്‌ടോപ്പ് നിറങ്ങൾ
quick-desktop-colors-detail = ഡെസ്‌ക്‌ടോപ്പിന്റെ വർണ്ണ സ്കീമും ആക്സന്റ് നിറവും
quick-app-names = ആപ്പ് പേരുകൾ
quick-app-names-detail = ഇടത്തേ അറ്റത്തുള്ള ആപ്പ് ഐക്കണുകൾക്ക് താഴെ പേരുകൾ
quick-inbox-tabs = ഇൻബോക്‌സ് ടാബുകൾ
quick-inbox-tabs-detail = ഓരോ അക്കൗണ്ടിന്റെയും മെയിൽ ദാതാവിന്റെ ടാബുകൾ
quick-choose-tabs = ടാബുകൾ തിരഞ്ഞെടുക്കുക
quick-choose-tabs-detail = ഓരോ അക്കൗണ്ടിനും, ക്രമീകരണത്തിൽ
quick-sending = അയയ്ക്കൽ
quick-undo-send = അയയ്ക്കൽ പഴയപടിയാക്കുക
quick-undo-send-off = ഓഫ്
quick-undo-send-seconds = { $seconds } സെ.
quick-signatures = ഒപ്പുകൾ
quick-signatures-none = ഇതുവരെ ഒന്നുമില്ല
quick-signatures-one = { $name }, ഡിഫോൾട്ടായി ഉപയോഗിക്കുന്നു
quick-signatures-many = { $count ->
    [one] { $count } ഒപ്പ്; ഡിഫോൾട്ടായി { $name }
   *[other] { $count } ഒപ്പുകൾ; ഡിഫോൾട്ടായി { $name }
}
quick-signatures-no-default = { $count ->
    [one] { $count }, ഡിഫോൾട്ടായി ഒന്നുമില്ല
   *[other] { $count }, ഡിഫോൾട്ടായി ഒന്നുമില്ല
}
quick-signature-untitled = പേരില്ലാത്തത്
quick-threading = ഇമെയിൽ ത്രെഡിംഗ്
quick-conversation-view = സംഭാഷണ കാഴ്‌ച
quick-conversation-view-detail = ഒരേ മെയിലിനുള്ള മറുപടികൾ ഗ്രൂപ്പ് ചെയ്യുക
quick-help = സഹായം
quick-tour = ടൂർ ആരംഭിക്കുക
quick-whats-new = പുതിയതെന്താണ്
quick-about = Katna-യെക്കുറിച്ച്

## Settings: opening at login

settings-open-at-login-failed = ലോഗിൻ ചെയ്യുമ്പോൾ തുറക്കുന്നത് മാറ്റാനായില്ല: { $error }

## Settings > Appearance > Scaling

scale-letter = അ
scale-percent = { $percent }%
scale-reset = { $percent }%-ലേക്ക് മടങ്ങുക

## Settings > Experimental > Look & Feel

look-intro = ഇപ്പോഴും പരീക്ഷിച്ചുകൊണ്ടിരിക്കുന്ന ഫീച്ചറുകൾ. ഇവ മാറുകയോ ഇല്ലാതാകുകയോ ചെയ്‌തേക്കാം.
look-heading = രൂപവും ഭാവവും
look-window-frame = വിൻഡോ ഫ്രെയിം
look-window-frame-detail = ടൈറ്റിൽ ബാർ, വിൻഡോ ബട്ടണുകൾ, കോണുകൾ, നിഴൽ എന്നിവ ആരാണ് വരയ്ക്കുന്നത്.
look-frame-native-kde = നേറ്റീവ്: നിങ്ങളുടെ Plasma തീമിൽ KDE-യുടെ ഫ്രെയിം
look-frame-native = നേറ്റീവ്: ഡെസ്‌ക്‌ടോപ്പിന്റെ ഫ്രെയിം
look-frame-katna = Katna: മുകളിലെ ബാർ ടൈറ്റിൽ ബാറാകുന്നു
look-frame-katna-note-named = Katna ഉരുണ്ട കോണുകളും സ്വന്തം നിഴലും വരയ്ക്കുന്നു. ഫ്രെയിം ഇനി { $desktop } തീം പിന്തുടരില്ല; വിൻഡോ നിയമങ്ങൾ തുടർന്നും ബാധകമാണ്.
look-frame-katna-note = Katna ഉരുണ്ട കോണുകളും സ്വന്തം നിഴലും വരയ്ക്കുന്നു. ഫ്രെയിം ഇനി ഡെസ്‌ക്‌ടോപ്പ് തീം പിന്തുടരില്ല; വിൻഡോ നിയമങ്ങൾ തുടർന്നും ബാധകമാണ്.
look-frame-client-side = നിങ്ങളുടെ ഡെസ്‌ക്‌ടോപ്പ് ഫ്രെയിം ഓരോ ആപ്പിനും വിട്ടുകൊടുക്കുന്നു, അതിനാൽ Katna ഇതിനകം സ്വന്തം ഫ്രെയിം വരയ്ക്കുന്നു.
look-blurred-background = മങ്ങിയ പശ്ചാത്തലം
look-blurred-background-detail = മുകളിലെ ബാറിലൂടെയും ഫോൾഡറുകളിലൂടെയും ഡെസ്‌ക്‌ടോപ്പ് മങ്ങിയതായി കാണാം, മെനുകളും പോപ്പ്ഓവറുകളും മഞ്ഞുഗ്ലാസ് പോലെയാകും.
look-blur = വിൻഡോയ്ക്ക് പിന്നിലുള്ളത് മങ്ങിക്കുക
look-blur-detail = മെയിൽ ഉറച്ച കാർഡുകളിൽ തന്നെ നിൽക്കും, അതിനാൽ ടെക്സ്റ്റിന്റെ കോൺട്രാസ്റ്റ് കുറയില്ല
look-blur-off-kde = KDE-യുടെ മങ്ങൽ ഇഫക്റ്റ് ഓഫാണ്. സിസ്റ്റം ക്രമീകരണം, വിൻഡോ മാനേജ്‌മെന്റ്, ഡെസ്‌ക്‌ടോപ്പ് ഇഫക്റ്റുകൾ എന്നതിൽ മങ്ങൽ ഓണാക്കി, തുടർന്ന് Katna Mail വീണ്ടും തുറക്കുക.
look-blur-none-gnome = GNOME വിൻഡോകൾക്ക് പിന്നിലുള്ളത് മങ്ങിക്കുന്നില്ല.
look-blur-none-x11 = നിങ്ങളുടെ വിൻഡോ മാനേജർ വിൻഡോകൾക്ക് പിന്നിലുള്ളത് മങ്ങിക്കുന്നില്ല.
look-blur-none-wayland = നിങ്ങളുടെ കമ്പോസിറ്റർ വിൻഡോകൾക്ക് പിന്നിലുള്ളത് മങ്ങിക്കുന്നില്ല.

## Settings > User feedback (crash reports)

feedback-intro-sending = എന്താണ് തെറ്റിയതെന്ന് പരിഹരിക്കാൻ സഹായിക്കാൻ പുതിയ ക്രാഷ് റിപ്പോർട്ടുകൾ അയയ്ക്കുന്നു. മറ്റൊന്നും ഈ കമ്പ്യൂട്ടറിൽ നിന്ന് പുറത്തുപോകില്ല.
feedback-intro-local = Katna ഒന്നും എവിടേക്കും അയയ്ക്കുന്നില്ല. ക്രാഷ് റിപ്പോർട്ടുകൾ ഈ കമ്പ്യൂട്ടറിൽ തന്നെ നിലനിൽക്കും, നിങ്ങൾക്ക് കാണാനോ ബഗ് റിപ്പോർട്ടിൽ അറ്റാച്ച് ചെയ്യാനോ.
feedback-crash-reports = ക്രാഷ് റിപ്പോർട്ടുകൾ
feedback-crash-reports-detail = Katna Mail അല്ലെങ്കിൽ അതിന്റെ പശ്ചാത്തല സേവനം ക്രാഷ് ആകുമ്പോൾ എഴുതുന്നു.
feedback-save = ക്രാഷ് റിപ്പോർട്ടുകൾ ഈ കമ്പ്യൂട്ടറിൽ സംരക്ഷിക്കുക
feedback-save-detail = നിങ്ങളുടെ ഹോം ഫോൾഡർ, ഉപയോക്തൃ, കമ്പ്യൂട്ടർ പേരുകൾ, ഇമെയിൽ വിലാസങ്ങൾ എന്നിവ ഒഴിവാക്കുന്നു
feedback-saved = സംരക്ഷിച്ച ക്രാഷ് റിപ്പോർട്ടുകൾ
feedback-saved-detail = { $count ->
    [one] ഏറ്റവും പുതിയ { $count } റിപ്പോർട്ട് സൂക്ഷിക്കുന്നു.
   *[other] ഏറ്റവും പുതിയ { $count } റിപ്പോർട്ടുകൾ സൂക്ഷിക്കുന്നു.
}
feedback-help-improve = Katna മെച്ചപ്പെടുത്താൻ സഹായിക്കുക
feedback-help-improve-detail = നിങ്ങൾ ഓണാക്കിയില്ലെങ്കിൽ ഓഫായിരിക്കും, എപ്പോൾ വേണമെങ്കിലും ഇവിടെ ഓഫാക്കാം.
feedback-send = ക്രാഷ് റിപ്പോർട്ടുകൾ അയയ്ക്കുക
feedback-send-detail = സംരക്ഷിച്ച റിപ്പോർട്ട്, നിങ്ങൾക്ക് ഇവിടെ കാണാനാകുന്നതുപോലെ തന്നെ, Katna-യുടെ ക്രാഷ് ട്രാക്കറിലേക്ക് (Sentry, EU-ൽ) പോകുന്നു. IP വിലാസമോ സന്ദേശങ്ങളോ ഇമെയിൽ വിലാസങ്ങളോ ഇല്ല
feedback-none-saved = ക്രാഷ് റിപ്പോർട്ടുകളൊന്നും സംരക്ഷിച്ചിട്ടില്ല.
feedback-delete-all = എല്ലാം ഇല്ലാതാക്കുക
feedback-app-daemon = പശ്ചാത്തല സേവനം
feedback-report-sent = { $date } · അയച്ചു
feedback-view = കാണുക
feedback-view-tooltip = റിപ്പോർട്ട് തുറക്കുക
feedback-copy-tooltip = ബഗ് റിപ്പോർട്ടിൽ ഒട്ടിക്കാൻ ഇത് പകർത്തുക
feedback-copied = ക്രാഷ് റിപ്പോർട്ട് പകർത്തി.
feedback-deleted-all = ക്രാഷ് റിപ്പോർട്ടുകൾ ഇല്ലാതാക്കി.
feedback-read-failed = ക്രാഷ് റിപ്പോർട്ട് വായിക്കാനായില്ല: { $error }
feedback-delete-failed = ക്രാഷ് റിപ്പോർട്ട് ഇല്ലാതാക്കാനായില്ല: { $error }
feedback-delete-all-failed = ക്രാഷ് റിപ്പോർട്ടുകൾ ഇല്ലാതാക്കാനായില്ല: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _ഫയൽ
desktop-menu-new-message = _പുതിയ സന്ദേശം
desktop-menu-quit = _പുറത്തുകടക്കുക
desktop-menu-edit = _എഡിറ്റ്
desktop-menu-undo = _പഴയപടിയാക്കുക
desktop-menu-select-all = _എല്ലാം തിരഞ്ഞെടുക്കുക
desktop-menu-select-none = _ഒന്നും തിരഞ്ഞെടുക്കരുത്
desktop-menu-find = _കണ്ടെത്തുക…
desktop-menu-view = _കാഴ്‌ച
desktop-menu-folder-list = _ഫോൾഡർ ലിസ്റ്റ് കാണിക്കുക
desktop-menu-refresh = _പുതുക്കുക
desktop-menu-go = _പോകുക
desktop-menu-inbox = _ഇൻബോക്‌സ്
desktop-menu-starred = _നക്ഷത്രമിട്ടവ
desktop-menu-sent = _അയച്ചവ
desktop-menu-drafts = _ഡ്രാഫ്റ്റുകൾ
desktop-menu-all-mail = _എല്ലാ മെയിലുകളും
desktop-menu-next = _അടുത്ത സംഭാഷണം
desktop-menu-previous = _മുമ്പത്തെ സംഭാഷണം
desktop-menu-message = _സന്ദേശം
desktop-menu-open = _തുറക്കുക
desktop-menu-reply = _മറുപടി നൽകുക
desktop-menu-reply-all = _എല്ലാവർക്കും മറുപടി നൽകുക
desktop-menu-forward = _ഫോർവേഡ് ചെയ്യുക
desktop-menu-archive = _ആർക്കൈവ് ചെയ്യുക
desktop-menu-delete = _ഇല്ലാതാക്കുക
desktop-menu-spam = _സ്‌പാം ആയി റിപ്പോർട്ട് ചെയ്യുക
desktop-menu-move-to = _ഇതിലേക്ക് നീക്കുക…
desktop-menu-mark-read = _വായിച്ചതായി അടയാളപ്പെടുത്തുക
desktop-menu-mark-unread = _വായിക്കാത്തതായി അടയാളപ്പെടുത്തുക
desktop-menu-star = _നക്ഷത്രമിടുക
desktop-menu-important = _പ്രധാനപ്പെട്ടതായി അടയാളപ്പെടുത്തുക
desktop-menu-not-important = _പ്രധാനപ്പെട്ടതല്ലെന്ന് അടയാളപ്പെടുത്തുക
desktop-menu-settings = _ക്രമീകരണം
desktop-menu-quick-settings = _ദ്രുത ക്രമീകരണം
desktop-menu-configure = _Katna Mail കോൺഫിഗർ ചെയ്യുക…
desktop-menu-help = _സഹായം
desktop-menu-shortcuts = _കീബോർഡ് കുറുക്കുവഴികൾ
desktop-menu-whats-new = _പുതിയതെന്താണ്
desktop-menu-about = _Katna-യെക്കുറിച്ച്

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = നീങ്ങൽ
shortcut-group-actions = പ്രവർത്തനങ്ങൾ
shortcut-group-go-to = ഇതിലേക്ക് പോകുക
shortcut-group-app = ആപ്ലിക്കേഷൻ

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = അടുത്ത സംഭാഷണം
shortcut-previous = മുമ്പത്തെ സംഭാഷണം
shortcut-down = ലിസ്റ്റിൽ താഴേക്ക് നീങ്ങുക
shortcut-up = ലിസ്റ്റിൽ മുകളിലേക്ക് നീങ്ങുക
shortcut-first = ലിസ്റ്റിലെ ആദ്യത്തേത്
shortcut-last = ലിസ്റ്റിലെ അവസാനത്തേത്
shortcut-page-down = ലിസ്റ്റിൽ ഒരു പേജ് താഴേക്ക്
shortcut-page-up = ലിസ്റ്റിൽ ഒരു പേജ് മുകളിലേക്ക്
shortcut-open = സംഭാഷണം തുറക്കുക
shortcut-back = ലിസ്റ്റിലേക്ക് മടങ്ങുക
shortcut-scroll-down = താഴേക്ക് സ്ക്രോൾ ചെയ്യുക
shortcut-scroll-up = മുകളിലേക്ക് സ്ക്രോൾ ചെയ്യുക
shortcut-scroll-page-down = ഒരു പേജ് താഴേക്ക് സ്ക്രോൾ ചെയ്യുക
shortcut-scroll-page-up = ഒരു പേജ് മുകളിലേക്ക് സ്ക്രോൾ ചെയ്യുക
shortcut-compose = രചിക്കുക
shortcut-reply = മറുപടി നൽകുക
shortcut-reply-all = എല്ലാവർക്കും മറുപടി നൽകുക
shortcut-forward = ഫോർവേഡ് ചെയ്യുക
shortcut-archive = ആർക്കൈവ് ചെയ്യുക
shortcut-delete = ഇല്ലാതാക്കുക
shortcut-spam = സ്‌പാം ആയി റിപ്പോർട്ട് ചെയ്യുക
shortcut-move-to = ഇതിലേക്ക് നീക്കുക
shortcut-mark-read = വായിച്ചതായി അടയാളപ്പെടുത്തുക
shortcut-mark-unread = വായിക്കാത്തതായി അടയാളപ്പെടുത്തുക
shortcut-star = നക്ഷത്രമിടുക അല്ലെങ്കിൽ നീക്കുക
shortcut-important = പ്രധാനപ്പെട്ടതായി അടയാളപ്പെടുത്തുക
shortcut-not-important = പ്രധാനപ്പെട്ടതല്ലെന്ന് അടയാളപ്പെടുത്തുക
shortcut-check = സംഭാഷണം തിരഞ്ഞെടുക്കുക
shortcut-select-all = എല്ലാ സംഭാഷണങ്ങളും തിരഞ്ഞെടുക്കുക
shortcut-select-none = എല്ലാ സംഭാഷണങ്ങളുടെയും തിരഞ്ഞെടുപ്പ് മാറ്റുക
shortcut-undo = അവസാന പ്രവർത്തനം പഴയപടിയാക്കുക
shortcut-go-inbox = ഇൻബോക്‌സ്
shortcut-go-starred = നക്ഷത്രമിട്ടവ
shortcut-go-sent = അയച്ചവ
shortcut-go-drafts = ഡ്രാഫ്റ്റുകൾ
shortcut-go-all = എല്ലാ മെയിലുകളും
shortcut-search = മെയിലിൽ തിരയുക
shortcut-navigation = മെനു കാണിക്കുക അല്ലെങ്കിൽ ചുരുക്കുക
shortcut-quick-settings = ദ്രുത ക്രമീകരണം
shortcut-settings = എല്ലാ ക്രമീകരണങ്ങളും
shortcut-shortcuts = കീബോർഡ് കുറുക്കുവഴികൾ
shortcut-reload = പുതിയ മെയിൽ പരിശോധിക്കുക
shortcut-quit = പുറത്തുകടക്കുക

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } തുടർന്ന് { $second }

## Settings > Accounts

accounts-folder-pane = ഫോൾഡർ പാളി
accounts-folder-pane-detail = ഇടതുവശത്തുള്ള പാളി ഏതൊക്കെ അക്കൗണ്ടുകളുടെ ഫോൾഡറുകൾ കാണിക്കുന്നു.
accounts-shown-one = ഒരു സമയം ഒരു അക്കൗണ്ട്; അക്കൗണ്ട് കാർഡിൽ മാറുക
accounts-shown-all = എല്ലാ അക്കൗണ്ടുകളും, ഒന്നിനുപിറകെ ഒന്നായി
accounts-row = അക്കൗണ്ടുകൾ
accounts-row-detail = ഒരു അക്കൗണ്ട് നീക്കം ചെയ്‌താൽ ഈ കമ്പ്യൂട്ടറിലുള്ള അതിന്റെ മെയിലിന്റെ Katna പകർപ്പ് ഇല്ലാതാക്കും. മെയിൽ സെർവറിൽ തുടരും.
accounts-none = ഇതുവരെ അക്കൗണ്ടുകളൊന്നുമില്ല.
accounts-kind-imported = ഇമ്പോർട്ട് ചെയ്‌തത്
accounts-picture-reset = ഡെസ്‌ക്‌ടോപ്പ് ചിത്രം ഉപയോഗിക്കുക
accounts-picture-change = ചിത്രം മാറ്റുക
accounts-remove = നീക്കം ചെയ്യുക
accounts-delete-all-row = എല്ലാ ഡാറ്റയും ഇല്ലാതാക്കുക
accounts-delete-all-row-detail = പുതിയ ഇൻസ്റ്റാളിലെന്നപോലെ, വീണ്ടും തുടങ്ങുക.
accounts-delete-all-about = എല്ലാ അക്കൗണ്ടുകളും, സംഭരിച്ച എല്ലാ മെയിലുകളും കോൺടാക്റ്റുകളും കലണ്ടറുകളും, തിരയൽ സൂചികയും, നിങ്ങളുടെ ക്രമീകരണവും സംരക്ഷിച്ച പാസ്‌വേഡുകളും ഈ കമ്പ്യൂട്ടറിൽ നിന്ന് ഇല്ലാതാക്കുന്നു. നിങ്ങളുടെ മെയിൽ സെർവറുകളിൽ ഒന്നും മാറില്ല.
accounts-delete-all-open = എല്ലാ Katna ഡാറ്റയും ഇല്ലാതാക്കുക

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } Katna-യിൽ നിന്ന് നീക്കം ചെയ്‌തു.
accounts-removed = { $address } Katna-യിൽ നിന്ന് നീക്കം ചെയ്‌തു. അതിന്റെ മെയിൽ ഇപ്പോഴും സെർവറിലുണ്ട്.
accounts-all-deleted = എല്ലാ Katna ഡാറ്റയും ഈ കമ്പ്യൂട്ടറിൽ നിന്ന് ഇല്ലാതാക്കി.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } നീക്കം ചെയ്യണോ?
accounts-remove-confirm = അക്കൗണ്ട് നീക്കം ചെയ്യുക
accounts-removing = നീക്കം ചെയ്യുന്നു…
accounts-remove-local-mail = { $folders ->
    [0] ഈ അക്കൗണ്ടിലേക്ക് ഇമ്പോർട്ട് ചെയ്‌ത എല്ലാ മെയിലുകളും
    [one] ഈ അക്കൗണ്ടിലേക്ക് ഇമ്പോർട്ട് ചെയ്‌ത, അതിന്റെ ഫോൾഡറിലുള്ള എല്ലാ മെയിലുകളും
   *[other] ഈ അക്കൗണ്ടിലേക്ക് ഇമ്പോർട്ട് ചെയ്‌ത, അതിന്റെ { $folders } ഫോൾഡറുകളിലുള്ള എല്ലാ മെയിലുകളും
}
accounts-remove-local-settings = അതിന്റെ Katna ക്രമീകരണം
accounts-remove-mail = { $folders ->
    [0] Katna സംഭരിച്ച ഈ അക്കൗണ്ടിന്റെ എല്ലാ മെയിലുകളും
    [one] Katna അതിന്റെ ഫോൾഡറിൽ സംഭരിച്ച ഈ അക്കൗണ്ടിന്റെ എല്ലാ മെയിലുകളും
   *[other] Katna അതിന്റെ { $folders } ഫോൾഡറുകളിൽ സംഭരിച്ച ഈ അക്കൗണ്ടിന്റെ എല്ലാ മെയിലുകളും
}
accounts-remove-outbox = ഔട്ട്‌ബോക്‌സിൽ കാത്തിരിക്കുന്ന അതിന്റെ സന്ദേശങ്ങൾ
accounts-remove-settings = അതിന്റെ സംരക്ഷിച്ച പാസ്‌വേഡും Katna ക്രമീകരണവും
accounts-delete-all-title = എല്ലാ Katna ഡാറ്റയും ഇല്ലാതാക്കണോ?
accounts-delete-all-confirm = എല്ലാം ഇല്ലാതാക്കുക
accounts-deleting = ഇല്ലാതാക്കുന്നു…
accounts-delete-all-accounts = എല്ലാ അക്കൗണ്ടുകളും, Katna സംഭരിച്ച എല്ലാ മെയിലുകളും അറ്റാച്ച്‌മെന്റുകളും
accounts-delete-all-contacts = കോൺടാക്റ്റുകൾ, കലണ്ടറുകൾ, തിരയൽ സൂചിക
accounts-delete-all-settings = എല്ലാ ക്രമീകരണങ്ങളും ഒപ്പുകളും കീബോർഡ് കുറുക്കുവഴികളും
accounts-delete-all-passwords = സംരക്ഷിച്ച എല്ലാ പാസ്‌വേഡുകളും
accounts-deleted-heading = ഈ കമ്പ്യൂട്ടറിൽ നിന്ന് ഇല്ലാതാക്കുന്നവ:
accounts-cannot-undo = ഇത് പഴയപടിയാക്കാനാകില്ല.
accounts-server-delete-all = നിങ്ങളുടെ മെയിൽ സെർവറുകളിൽ ഒന്നും മാറില്ല: നിങ്ങളുടെ മെയിൽ അവിടെത്തന്നെ നിലനിൽക്കും, അക്കൗണ്ട് വീണ്ടും ചേർത്താൽ അത് വീണ്ടും ഡൗൺലോഡ് ചെയ്യും. ഫയലുകളിൽ നിന്ന് ഇമ്പോർട്ട് ചെയ്‌ത മെയിൽ Katna-യിൽ മാത്രമേയുള്ളൂ; ആ ഫയലുകളെ തൊടില്ല.
accounts-server-local = ഈ മെയിൽ ഫയലുകളിൽ നിന്ന് ഇമ്പോർട്ട് ചെയ്‌തതാണ്, അതിനാൽ അതിന്റെ ഒരേയൊരു പകർപ്പ് Katna-യുടെ പക്കലാണ്. അത് വന്ന ഫയലുകളെ തൊടില്ല; അത് തിരികെ ലഭിക്കാൻ അവ വീണ്ടും ഇമ്പോർട്ട് ചെയ്യുക.
accounts-server-remove = മെയിൽ സെർവറിൽ ഒന്നും മാറില്ല: നിങ്ങളുടെ മെയിൽ അവിടെത്തന്നെ നിലനിൽക്കും, അക്കൗണ്ട് വീണ്ടും ചേർത്താൽ അത് വീണ്ടും ഡൗൺലോഡ് ചെയ്യും.
accounts-confirm-word = ഇല്ലാതാക്കുക
accounts-confirm-placeholder = “{ accounts-confirm-word }” എന്ന് ടൈപ്പ് ചെയ്യുക
accounts-confirm-prompt = സ്ഥിരീകരിക്കാൻ, “{ accounts-confirm-word }” എന്ന് ടൈപ്പ് ചെയ്യുക:
accounts-cancel = റദ്ദാക്കുക
