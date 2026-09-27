# Katna Mail, Malayalam (മലയാളം).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] വായിച്ച { $count } സംഭാഷണം തിരഞ്ഞെടുത്തു.
           *[other] വായിച്ച എല്ലാ { $count } സംഭാഷണങ്ങളും തിരഞ്ഞെടുത്തു.
        }
       *[message] { $count ->
            [one] വായിച്ച { $count } സന്ദേശം തിരഞ്ഞെടുത്തു.
           *[other] വായിച്ച എല്ലാ { $count } സന്ദേശങ്ങളും തിരഞ്ഞെടുത്തു.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] വായിക്കാത്ത { $count } സംഭാഷണം തിരഞ്ഞെടുത്തു.
           *[other] വായിക്കാത്ത എല്ലാ { $count } സംഭാഷണങ്ങളും തിരഞ്ഞെടുത്തു.
        }
       *[message] { $count ->
            [one] വായിക്കാത്ത { $count } സന്ദേശം തിരഞ്ഞെടുത്തു.
           *[other] വായിക്കാത്ത എല്ലാ { $count } സന്ദേശങ്ങളും തിരഞ്ഞെടുത്തു.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] നക്ഷത്രമിട്ട { $count } സംഭാഷണം തിരഞ്ഞെടുത്തു.
           *[other] നക്ഷത്രമിട്ട എല്ലാ { $count } സംഭാഷണങ്ങളും തിരഞ്ഞെടുത്തു.
        }
       *[message] { $count ->
            [one] നക്ഷത്രമിട്ട { $count } സന്ദേശം തിരഞ്ഞെടുത്തു.
           *[other] നക്ഷത്രമിട്ട എല്ലാ { $count } സന്ദേശങ്ങളും തിരഞ്ഞെടുത്തു.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] നക്ഷത്രമിടാത്ത { $count } സംഭാഷണം തിരഞ്ഞെടുത്തു.
           *[other] നക്ഷത്രമിടാത്ത എല്ലാ { $count } സംഭാഷണങ്ങളും തിരഞ്ഞെടുത്തു.
        }
       *[message] { $count ->
            [one] നക്ഷത്രമിടാത്ത { $count } സന്ദേശം തിരഞ്ഞെടുത്തു.
           *[other] നക്ഷത്രമിടാത്ത എല്ലാ { $count } സന്ദേശങ്ങളും തിരഞ്ഞെടുത്തു.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $folder }-ലെ വായിച്ച { $count } സംഭാഷണം തിരഞ്ഞെടുത്തു.
           *[other] { $folder }-ലെ വായിച്ച എല്ലാ { $count } സംഭാഷണങ്ങളും തിരഞ്ഞെടുത്തു.
        }
       *[message] { $count ->
            [one] { $folder }-ലെ വായിച്ച { $count } സന്ദേശം തിരഞ്ഞെടുത്തു.
           *[other] { $folder }-ലെ വായിച്ച എല്ലാ { $count } സന്ദേശങ്ങളും തിരഞ്ഞെടുത്തു.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $folder }-ലെ വായിക്കാത്ത { $count } സംഭാഷണം തിരഞ്ഞെടുത്തു.
           *[other] { $folder }-ലെ വായിക്കാത്ത എല്ലാ { $count } സംഭാഷണങ്ങളും തിരഞ്ഞെടുത്തു.
        }
       *[message] { $count ->
            [one] { $folder }-ലെ വായിക്കാത്ത { $count } സന്ദേശം തിരഞ്ഞെടുത്തു.
           *[other] { $folder }-ലെ വായിക്കാത്ത എല്ലാ { $count } സന്ദേശങ്ങളും തിരഞ്ഞെടുത്തു.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $folder }-ലെ നക്ഷത്രമിട്ട { $count } സംഭാഷണം തിരഞ്ഞെടുത്തു.
           *[other] { $folder }-ലെ നക്ഷത്രമിട്ട എല്ലാ { $count } സംഭാഷണങ്ങളും തിരഞ്ഞെടുത്തു.
        }
       *[message] { $count ->
            [one] { $folder }-ലെ നക്ഷത്രമിട്ട { $count } സന്ദേശം തിരഞ്ഞെടുത്തു.
           *[other] { $folder }-ലെ നക്ഷത്രമിട്ട എല്ലാ { $count } സന്ദേശങ്ങളും തിരഞ്ഞെടുത്തു.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $folder }-ലെ നക്ഷത്രമിടാത്ത { $count } സംഭാഷണം തിരഞ്ഞെടുത്തു.
           *[other] { $folder }-ലെ നക്ഷത്രമിടാത്ത എല്ലാ { $count } സംഭാഷണങ്ങളും തിരഞ്ഞെടുത്തു.
        }
       *[message] { $count ->
            [one] { $folder }-ലെ നക്ഷത്രമിടാത്ത { $count } സന്ദേശം തിരഞ്ഞെടുത്തു.
           *[other] { $folder }-ലെ നക്ഷത്രമിടാത്ത എല്ലാ { $count } സന്ദേശങ്ങളും തിരഞ്ഞെടുത്തു.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] വായിച്ച സംഭാഷണങ്ങളൊന്നും ഇവിടെയില്ല.
       *[message] വായിച്ച സന്ദേശങ്ങളൊന്നും ഇവിടെയില്ല.
    }
   *[unread] { $kind ->
        [conversation] വായിക്കാത്ത സംഭാഷണങ്ങളൊന്നും ഇവിടെയില്ല.
       *[message] വായിക്കാത്ത സന്ദേശങ്ങളൊന്നും ഇവിടെയില്ല.
    }
    [starred] { $kind ->
        [conversation] നക്ഷത്രമിട്ട സംഭാഷണങ്ങളൊന്നും ഇവിടെയില്ല.
       *[message] നക്ഷത്രമിട്ട സന്ദേശങ്ങളൊന്നും ഇവിടെയില്ല.
    }
    [unstarred] { $kind ->
        [conversation] നക്ഷത്രമിടാത്ത സംഭാഷണങ്ങളൊന്നും ഇവിടെയില്ല.
       *[message] നക്ഷത്രമിടാത്ത സന്ദേശങ്ങളൊന്നും ഇവിടെയില്ല.
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
menu-delete-forever = ശാശ്വതമായി ഇല്ലാതാക്കുക
menu-move-to-inbox = ഇൻബോക്‌സിലേക്ക് നീക്കുക
menu-spam = സ്‌പാം ആയി റിപ്പോർട്ട് ചെയ്യുക
menu-not-spam = സ്‌പാം അല്ല
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
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] സംഭാഷണം സ്‌പാം അല്ലെന്ന് അടയാളപ്പെടുത്തി ഇൻബോക്‌സിലേക്ക് നീക്കി.
       *[other] { $count } സംഭാഷണങ്ങൾ സ്‌പാം അല്ലെന്ന് അടയാളപ്പെടുത്തി ഇൻബോക്‌സിലേക്ക് നീക്കി.
    }
   *[message] { $count ->
        [one] സന്ദേശം സ്‌പാം അല്ലെന്ന് അടയാളപ്പെടുത്തി ഇൻബോക്‌സിലേക്ക് നീക്കി.
       *[other] { $count } സന്ദേശങ്ങൾ സ്‌പാം അല്ലെന്ന് അടയാളപ്പെടുത്തി ഇൻബോക്‌സിലേക്ക് നീക്കി.
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
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] സംഭാഷണം വായിച്ചതായി അടയാളപ്പെടുത്തി.
       *[other] { $count } സംഭാഷണങ്ങൾ വായിച്ചതായി അടയാളപ്പെടുത്തി.
    }
   *[message] { $count ->
        [one] സന്ദേശം വായിച്ചതായി അടയാളപ്പെടുത്തി.
       *[other] { $count } സന്ദേശങ്ങൾ വായിച്ചതായി അടയാളപ്പെടുത്തി.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] സംഭാഷണം വായിക്കാത്തതായി അടയാളപ്പെടുത്തി.
       *[other] { $count } സംഭാഷണങ്ങൾ വായിക്കാത്തതായി അടയാളപ്പെടുത്തി.
    }
   *[message] { $count ->
        [one] സന്ദേശം വായിക്കാത്തതായി അടയാളപ്പെടുത്തി.
       *[other] { $count } സന്ദേശങ്ങൾ വായിക്കാത്തതായി അടയാളപ്പെടുത്തി.
    }
}
toast-undone = പ്രവർത്തനം പഴയപടിയാക്കി.
toast-nothing-to-undo = പഴയപടിയാക്കാൻ ഒന്നുമില്ല.
toast-cannot-undo-delete-forever = ശാശ്വതമായി ഇല്ലാതാക്കിയ മെയിൽ തിരികെ കൊണ്ടുവരാനാകില്ല.
toast-send-undone = അയയ്ക്കൽ പഴയപടിയാക്കി.
toast-too-late-to-undo-send = പഴയപടിയാക്കാൻ വൈകിപ്പോയി: സന്ദേശം ഇതിനകം അയച്ചുകഴിഞ്ഞു.
toast-undo = പഴയപടിയാക്കുക
toast-no-spam-folder = ഈ അക്കൗണ്ടിന് സ്‌പാം ഫോൾഡർ ഇല്ല.
