# Katna Mail, Telugu (తెలుగు).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = భాష: { $language }
language-tooltip-system = భాష: { $language }, సిస్టమ్‌ను అనుసరిస్తోంది
language-search = భాషను వెతకండి
language-system-default = సిస్టమ్ డిఫాల్ట్
language-system-now = ప్రస్తుతం { $language }
language-no-match = “{ $query }”కు సరిపోలే భాష ఏదీ లేదు
language-machine = మెషిన్ అనువాదం. మెరుగుపరచడంలో సహాయపడండి
language-setting = భాష
language-setting-detail = మెనూలు, బటన్‌లు, మెసేజ్‌ల భాష, అలాగే తేదీలు, సంఖ్యల ఫార్మాట్. సిస్టమ్ డిఫాల్ట్ డెస్క్‌టాప్‌ను అనుసరిస్తుంది.

## Dates and sizes

ago-just-now = ఇప్పుడే
ago-minutes = { $count ->
    [one] { $count } నిమిషం క్రితం
   *[other] { $count } నిమిషాల క్రితం
}
ago-hours = { $count ->
    [one] { $count } గంట క్రితం
   *[other] { $count } గంటల క్రితం
}
ago-days = { $count ->
    [one] { $count } రోజు క్రితం
   *[other] { $count } రోజుల క్రితం
}
size-bytes = { $count ->
    [one] { $count } బైట్
   *[other] { $count } బైట్‌లు
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = ఫోల్డర్‌లను దాచండి
folders-show = ఫోల్డర్‌లను చూపండి
compose = కంపోజ్ చేయండి
search = వెతకండి
search-mail = మెయిల్‌లో వెతకండి
search-settings = సెట్టింగ్‌లలో వెతకండి
search-clear = సెర్చ్‌ను క్లియర్ చేయండి
search-options-show = సెర్చ్ ఆప్షన్‌లను చూపండి
settings = సెట్టింగ్‌లు
account-add = ఖాతాను జోడించండి

## App rail (and the bottom bar on a phone)

rail-mail = మెయిల్
rail-calendar = క్యాలెండర్
rail-contacts = కాంటాక్ట్‌లు
rail-tasks = టాస్క్‌లు
rail-notes = నోట్స్
rail-feeds = ఫీడ్‌లు

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = త్వరలో వస్తోంది
app-calendar-promise = మీ CalDAV క్యాలెండర్‌లు, మెయిల్‌లో వచ్చిన మీటింగ్ ఆహ్వానాలు, రిమైండర్‌లు, మీ ఇన్‌బాక్స్ పక్కనే.
app-tasks-promise = CalDAVతో సింక్ అయ్యే చేయవలసిన పనుల లిస్ట్‌లు, మెయిల్ నుండి రూపొందించిన టాస్క్‌లు.
app-notes-promise = త్వరిత నోట్స్, తర్వాత చూసుకోవడానికి ఒక మెయిల్ లేదా సంభాషణపై నోట్స్.
app-feeds-promise = మీ మెయిల్ పక్కనే RSS, Atom ఫీడ్‌లను చదవండి.

## Contacts page

app-contacts-loading = మీ మెయిల్ నుండి వ్యక్తులను సేకరిస్తోంది…
app-contacts-empty = మీరు మెయిల్ చేసే వ్యక్తులు ఇక్కడ కనిపిస్తారు.
app-contacts-count = { $count ->
    [one] మీ మెయిల్ నుండి { $count } వ్యక్తి, ఎక్కువగా మెయిల్ చేసినవారు ముందు
   *[other] మీ మెయిల్ నుండి { $count } వ్యక్తులు, ఎక్కువగా మెయిల్ చేసినవారు ముందు
}
app-contacts-top = { $count ->
    [one] మీ మెయిల్ నుండి టాప్ { $count } వ్యక్తి, ఎక్కువగా మెయిల్ చేసినవారు ముందు
   *[other] మీ మెయిల్ నుండి టాప్ { $count } వ్యక్తులు, ఎక్కువగా మెయిల్ చేసినవారు ముందు
}
app-contacts-messages = { $count ->
    [one] { $count } మెసేజ్
   *[other] { $count } మెసేజ్‌లు
}
app-contacts-last = చివరిగా { $date }

## Navigation (the folders pane)

nav-labels = లేబుల్‌లు
nav-folders = ఫోల్డర్‌లు
nav-label-new = కొత్త లేబుల్‌ను క్రియేట్ చేయండి
nav-folder-new = కొత్త ఫోల్డర్‌ను క్రియేట్ చేయండి
nav-account-unnamed = ఖాతా { $number }
nav-tab-new = { $count ->
    [one] { $count } కొత్తది
   *[other] { $count } కొత్తవి
}

## Special folders (the user's own folders keep their names)

folder-inbox = ఇన్‌బాక్స్
folder-starred = నక్షత్రం ఉంచినవి
folder-drafts = డ్రాఫ్ట్‌లు
folder-sent = పంపినవి
folder-archive = ఆర్కైవ్
folder-spam = స్పామ్
folder-trash = ట్రాష్
folder-all-mail = అన్ని మెయిల్స్
folder-scheduled = షెడ్యూల్ చేసినవి

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = కొత్త లేబుల్
label-folder-new-title = కొత్త ఫోల్డర్
label-prompt = దయచేసి కొత్త లేబుల్ పేరును ఎంటర్ చేయండి:
label-folder-prompt = దయచేసి కొత్త ఫోల్డర్ పేరును ఎంటర్ చేయండి:
label-name-hint = లేబుల్ పేరు
label-folder-name-hint = ఫోల్డర్ పేరు
label-nest = లేబుల్‌ను దీని కింద ఉంచండి:
label-folder-nest = ఫోల్డర్‌ను దీని కింద ఉంచండి:
label-cancel = రద్దు చేయండి
label-create = క్రియేట్ చేయండి
label-creating = క్రియేట్ చేస్తోంది…
label-created = “{ $name }” లేబుల్ క్రియేట్ చేయబడింది.
label-folder-created = “{ $name }” ఫోల్డర్ క్రియేట్ చేయబడింది.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = ప్రాథమికం
tab-promotions = ప్రమోషన్‌లు
tab-social = సామాజికం
tab-updates = అప్‌డేట్‌లు
tab-forums = ఫోరమ్‌లు
tab-focused = ఫోకస్ చేసినవి
tab-other = ఇతరాలు
tab-inbox = ఇన్‌బాక్స్
tab-newsletters = న్యూస్‌లెటర్‌లు
tab-notifications = నోటిఫికేషన్‌లు
tab-new = { $count } కొత్తవి
tab-provider-other = Katna క్రమబద్ధీకరించింది

## Mail list: toolbar

list-select = ఎంచుకోండి
list-refresh = రిఫ్రెష్ చేయండి
list-more = మరిన్ని
list-mark-read = చదివినట్లు గుర్తు పెట్టండి
list-mark-unread = చదవనట్లు గుర్తు పెట్టండి
list-move-to = దీనికి తరలించండి
list-archive = ఆర్కైవ్ చేయండి
list-spam = స్పామ్‌గా రిపోర్ట్ చేయండి
list-delete = తొలగించండి
list-newer = కొత్తవి
list-older = పాతవి
list-range = { $total }లో { $first }–{ $last }
list-range-about = సుమారు { $total }లో { $first }–{ $last }
list-results = “{ $query }” కోసం ఫలితాలు
list-results-corrected = “{ $query }” కోసం ఫలితాలను చూపుతోంది
list-search-instead = దానికి బదులుగా “{ $query }” కోసం వెతకండి
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = అన్నీ
list-pick-none = ఏవీ వద్దు
list-pick-read = చదివినవి
list-pick-unread = చదవనివి
list-pick-starred = నక్షత్రం ఉంచినవి
list-pick-unstarred = నక్షత్రం ఉంచనివి

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } సంభాషణ ఎంచుకోబడింది.
       *[other] మొత్తం { $count } సంభాషణలు ఎంచుకోబడ్డాయి.
    }
   *[message] { $count ->
        [one] { $count } మెసేజ్ ఎంచుకోబడింది.
       *[other] మొత్తం { $count } మెసేజ్‌లు ఎంచుకోబడ్డాయి.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder }లోని { $count } సంభాషణ ఎంచుకోబడింది.
       *[other] { $folder }లోని మొత్తం { $count } సంభాషణలు ఎంచుకోబడ్డాయి.
    }
   *[message] { $count ->
        [one] { $folder }లోని { $count } మెసేజ్ ఎంచుకోబడింది.
       *[other] { $folder }లోని మొత్తం { $count } మెసేజ్‌లు ఎంచుకోబడ్డాయి.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] స్క్రీన్‌పై ఉన్న { $count } సంభాషణ ఎంచుకోబడింది.
       *[other] స్క్రీన్‌పై ఉన్న మొత్తం { $count } సంభాషణలు ఎంచుకోబడ్డాయి.
    }
   *[message] { $count ->
        [one] స్క్రీన్‌పై ఉన్న { $count } మెసేజ్ ఎంచుకోబడింది.
       *[other] స్క్రీన్‌పై ఉన్న మొత్తం { $count } మెసేజ్‌లు ఎంచుకోబడ్డాయి.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } సంభాషణను ఎంచుకోండి
       *[other] మొత్తం { $count } సంభాషణలను ఎంచుకోండి
    }
   *[message] { $count ->
        [one] { $count } మెసేజ్‌ను ఎంచుకోండి
       *[other] మొత్తం { $count } మెసేజ్‌లను ఎంచుకోండి
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder }లోని { $count } సంభాషణను ఎంచుకోండి
       *[other] { $folder }లోని మొత్తం { $count } సంభాషణలను ఎంచుకోండి
    }
   *[message] { $count ->
        [one] { $folder }లోని { $count } మెసేజ్‌ను ఎంచుకోండి
       *[other] { $folder }లోని మొత్తం { $count } మెసేజ్‌లను ఎంచుకోండి
    }
}
list-clear-selection = ఎంపికను క్లియర్ చేయండి

## Mail list: empty states

list-empty-search = మీ సెర్చ్‌కు సరిపోలే మెసేజ్‌లు ఏవీ లేవు.
list-empty-tab = { $tab }లో మెయిల్ ఏదీ లేదు.
list-empty-tab-unknown = ఈ ట్యాబ్‌లో మెయిల్ ఏదీ లేదు.
list-empty-folder = { $folder }లో మెసేజ్‌లు ఏవీ లేవు.
list-empty-folder-unknown = ఈ ఫోల్డర్‌లో మెసేజ్‌లు ఏవీ లేవు.
list-first-sync = మీ మెయిల్‌ను పొందుతోంది…
list-first-sync-detail = మెయిల్ వచ్చే కొద్దీ ఇక్కడ కనిపిస్తుంది.

## Mail list: lines

row-removed = ఈ మెసేజ్ తీసివేయబడింది.
row-starred = నక్షత్రం ఉంచబడింది
row-not-starred = నక్షత్రం ఉంచలేదు
row-important = ముఖ్యమైనది. ముఖ్యమైనది కాదని గుర్తు పెట్టడానికి క్లిక్ చేయండి.
row-mark-important = ముఖ్యమైనదిగా గుర్తు పెట్టండి
row-pinned = పైన పిన్ చేయబడింది
row-pin = పైన పిన్ చేయండి
row-unpin = అన్‌పిన్ చేయండి

## Mail list: More menu and right-click menu

menu-reply = రిప్లయి ఇవ్వండి
menu-reply-all = అందరికీ రిప్లయి ఇవ్వండి
menu-forward = ఫార్వర్డ్ చేయండి
menu-archive = ఆర్కైవ్ చేయండి
menu-delete = తొలగించండి
menu-spam = స్పామ్‌గా రిపోర్ట్ చేయండి
menu-mark-read = చదివినట్లు గుర్తు పెట్టండి
menu-mark-unread = చదవనట్లు గుర్తు పెట్టండి
menu-mark-all-read = అన్నింటినీ చదివినట్లు గుర్తు పెట్టండి
menu-star = నక్షత్రం ఉంచండి
menu-unstar = నక్షత్రాన్ని తీసివేయండి
menu-important = ముఖ్యమైనదిగా గుర్తు పెట్టండి
menu-not-important = ముఖ్యమైనది కాదని గుర్తు పెట్టండి
menu-pin = పైన పిన్ చేయండి
menu-unpin = అన్‌పిన్ చేయండి
menu-print-all = అన్నీ ప్రింట్ చేయండి
menu-new-window = కొత్త విండోలో తెరవండి
menu-move-to = దీనికి తరలించండి
menu-move-to-heading = దీనికి తరలించండి:
menu-find-from = { $name } నుండి వచ్చిన ఈమెయిల్స్‌ను కనుగొనండి

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ ఆర్కైవ్ చేయబడింది.
       *[other] { $count } సంభాషణలు ఆర్కైవ్ చేయబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ ఆర్కైవ్ చేయబడింది.
       *[other] { $count } మెసేజ్‌లు ఆర్కైవ్ చేయబడ్డాయి.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ ట్రాష్‌కు తరలించబడింది.
       *[other] { $count } సంభాషణలు ట్రాష్‌కు తరలించబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ ట్రాష్‌కు తరలించబడింది.
       *[other] { $count } మెసేజ్‌లు ట్రాష్‌కు తరలించబడ్డాయి.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ తరలించబడింది.
       *[other] { $count } సంభాషణలు తరలించబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ తరలించబడింది.
       *[other] { $count } మెసేజ్‌లు తరలించబడ్డాయి.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణకు నక్షత్రం ఉంచబడింది.
       *[other] { $count } సంభాషణలకు నక్షత్రం ఉంచబడింది.
    }
   *[message] { $count ->
        [one] మెసేజ్‌కు నక్షత్రం ఉంచబడింది.
       *[other] { $count } మెసేజ్‌లకు నక్షత్రం ఉంచబడింది.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ నుండి నక్షత్రం తీసివేయబడింది.
       *[other] { $count } సంభాషణల నుండి నక్షత్రం తీసివేయబడింది.
    }
   *[message] { $count ->
        [one] మెసేజ్ నుండి నక్షత్రం తీసివేయబడింది.
       *[other] { $count } మెసేజ్‌ల నుండి నక్షత్రం తీసివేయబడింది.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ ముఖ్యమైనదిగా గుర్తు పెట్టబడింది.
       *[other] { $count } సంభాషణలు ముఖ్యమైనవిగా గుర్తు పెట్టబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ ముఖ్యమైనదిగా గుర్తు పెట్టబడింది.
       *[other] { $count } మెసేజ్‌లు ముఖ్యమైనవిగా గుర్తు పెట్టబడ్డాయి.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ ముఖ్యమైనది కాదని గుర్తు పెట్టబడింది.
       *[other] { $count } సంభాషణలు ముఖ్యమైనవి కాదని గుర్తు పెట్టబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ ముఖ్యమైనది కాదని గుర్తు పెట్టబడింది.
       *[other] { $count } మెసేజ్‌లు ముఖ్యమైనవి కాదని గుర్తు పెట్టబడ్డాయి.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ పైన పిన్ చేయబడింది.
       *[other] { $count } సంభాషణలు పైన పిన్ చేయబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ పైన పిన్ చేయబడింది.
       *[other] { $count } మెసేజ్‌లు పైన పిన్ చేయబడ్డాయి.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ అన్‌పిన్ చేయబడింది.
       *[other] { $count } సంభాషణలు అన్‌పిన్ చేయబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ అన్‌పిన్ చేయబడింది.
       *[other] { $count } మెసేజ్‌లు అన్‌పిన్ చేయబడ్డాయి.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ స్పామ్‌గా రిపోర్ట్ చేయబడింది.
       *[other] { $count } సంభాషణలు స్పామ్‌గా రిపోర్ట్ చేయబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ స్పామ్‌గా రిపోర్ట్ చేయబడింది.
       *[other] { $count } మెసేజ్‌లు స్పామ్‌గా రిపోర్ట్ చేయబడ్డాయి.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] సంభాషణ శాశ్వతంగా తొలగించబడింది.
       *[other] { $count } సంభాషణలు శాశ్వతంగా తొలగించబడ్డాయి.
    }
   *[message] { $count ->
        [one] మెసేజ్ శాశ్వతంగా తొలగించబడింది.
       *[other] { $count } మెసేజ్‌లు శాశ్వతంగా తొలగించబడ్డాయి.
    }
}
toast-undone = చర్య రద్దు చేయబడింది.
toast-undo = చర్య రద్దు చేయండి
toast-no-spam-folder = ఈ ఖాతాలో స్పామ్ ఫోల్డర్ లేదు.

## Reading pane: toolbar

reader-close = మూసివేయండి
reader-back = వెనుకకు
reader-mark-unread = చదవనట్లు గుర్తు పెట్టండి
reader-move-to = దీనికి తరలించండి
reader-more = మరిన్ని
reader-print-all = అన్నీ ప్రింట్ చేయండి
reader-new-window = కొత్త విండోలో
reader-position = { $total }లో { $position }
reader-newer = కొత్తది
reader-older = పాతది

## Reading pane: the conversation

reader-removed = ఈ సంభాషణ తీసివేయబడింది.
reader-no-subject = (సబ్జెక్ట్ లేదు)
reader-collapse-all = అన్నింటినీ కుదించండి
reader-expand-all = అన్నింటినీ విస్తరించండి
reader-unknown-sender = (తెలియని పంపినవారు)
reader-date-ago = { $date } ({ $ago })
reader-me = నాకు
reader-to = స్వీకర్తలు: { $names }
reader-starred = నక్షత్రం ఉంచబడింది
reader-not-starred = నక్షత్రం ఉంచలేదు
reader-too-long = మెసేజ్ చాలా పొడవుగా ఉంది, పూర్తిగా చూపడం సాధ్యం కాదు.
reader-encrypted-images = ఎన్‌క్రిప్ట్ చేసిన మెయిల్‌లో వెబ్ నుండి ఇమేజ్‌లు ఎప్పుడూ లోడ్ కావు.
reader-window-failed = కొత్త విండోను తెరవడం సాధ్యం కాలేదు.

## Reading pane: message details (opened from "to me")

reader-details-from = పంపినవారు:
reader-details-to = స్వీకర్త:
reader-details-cc = cc:
reader-details-date = తేదీ:
reader-details-subject = సబ్జెక్ట్:

## Reading pane: downloading a message

reader-downloading = సర్వర్ నుండి ఈ మెసేజ్‌ను డౌన్‌లోడ్ చేస్తోంది…
reader-download-failed = ఈ మెసేజ్‌ను డౌన్‌లోడ్ చేయడం సాధ్యం కాలేదు.
reader-try-again = మళ్లీ ట్రై చేయండి

## Reply row

reply-reply = రిప్లయి ఇవ్వండి
reply-reply-all = అందరికీ రిప్లయి ఇవ్వండి
reply-forward = ఫార్వర్డ్ చేయండి

## Encrypted and signed mail

security-decrypting = డీక్రిప్ట్ చేస్తోంది…
security-checking = సంతకాన్ని చెక్ చేస్తోంది…
security-partly-encrypted = ఈ మెసేజ్‌లో ఒక భాగం మాత్రమే ఎన్‌క్రిప్ట్ చేయబడింది. మిగతాది రక్షణ వెలుపల జోడించబడింది, ఎవరైనా పంపి ఉండవచ్చు.
security-partly-signed = ఈ మెసేజ్‌లో ఒక భాగంపై మాత్రమే సంతకం ఉంది. మిగతాది రక్షణ వెలుపల జోడించబడింది, ఎవరైనా పంపి ఉండవచ్చు.
security-encrypted = ఎన్‌క్రిప్ట్ చేసిన మెసేజ్
security-encrypted-smime = ఎన్‌క్రిప్ట్ చేసిన మెసేజ్ (S/MIME)
security-no-key = ఈ మెసేజ్‌ను డీక్రిప్ట్ చేయడం సాధ్యం కాదు: మీ వద్ద లేని కీ కోసం ఇది ఎన్‌క్రిప్ట్ చేయబడింది.
security-cancelled = డీక్రిప్ట్ చేయడం రద్దు చేయబడింది.
security-damaged = ఈ మెసేజ్‌ను డీక్రిప్ట్ చేయడం సాధ్యం కాదు: ఎన్‌క్రిప్ట్ చేసిన డేటా పాడైంది లేదా మార్చబడింది.
security-decrypt-unavailable = ఈ మెసేజ్‌ను డీక్రిప్ట్ చేయడం సాధ్యం కాదు: ఎన్‌క్రిప్ట్ చేసిన మెయిల్‌ను చదవడానికి { $tool }ను ఇన్‌స్టాల్ చేయండి.
security-decrypt-failed = ఈ మెసేజ్‌ను డీక్రిప్ట్ చేయడం సాధ్యం కాదు: { $reason }
security-unknown-signer = తెలియని సంతకందారు
security-signed-verified = { $signer } సంతకం చేశారు · వెరిఫై చేయబడింది
security-signed-not-sender = { $signer } సంతకం చేశారు, వీరు పంపినవారు కాదు
security-signed-untrusted = { $signer } సంతకం చేశారు, మీరు విశ్వసనీయం కాదని గుర్తు పెట్టిన కీతో
security-signed-unverified = { $signer } సంతకం చేశారు · కీ వెరిఫై చేయబడలేదు
security-bad-signature = చెడ్డ సంతకం: సంతకం చేసిన తర్వాత ఈ మెసేజ్ మార్చబడింది, లేదా సంతకం నకిలీది.
security-signature-expired = { $signer } సంతకం చేశారు · సంతకం గడువు ముగిసింది
security-key-expired = { $signer } సంతకం చేశారు · అప్పటి నుండి కీ గడువు ముగిసింది
security-key-revoked = { $signer } సంతకం చేశారు, ఉపసంహరించబడిన కీతో
security-missing-key = మీ వద్ద లేని కీతో సంతకం చేయబడింది, కాబట్టి చెక్ చేయడం సాధ్యం కాదు
security-missing-key-id = మీ వద్ద లేని కీతో ({ $key }) సంతకం చేయబడింది, కాబట్టి చెక్ చేయడం సాధ్యం కాదు
security-signature-unavailable = సంతకం చేయబడింది; సంతకాన్ని చెక్ చేయడానికి { $tool }ను ఇన్‌స్టాల్ చేయండి
security-signature-error = సంతకాన్ని చెక్ చేయడం సాధ్యం కాలేదు.

## Remote images and pictures

remote-hidden = ఈ మెసేజ్‌లోని ఇమేజ్‌లు దాచబడ్డాయి.
remote-show = ఇమేజ్‌లను చూపండి
remote-always-show = ఈ పంపినవారి నుండి ఎల్లప్పుడూ చూపండి
remote-picture-use = ఉపయోగించండి
remote-picture-too-big = 8 MB లేదా అంతకంటే తక్కువ ఉన్న చిత్రాన్ని ఎంచుకోండి.
remote-picture-type = PNG, JPEG, GIF, WebP లేదా SVG చిత్రాన్ని ఎంచుకోండి.
remote-picture-read-failed = చిత్రాన్ని చదవడం సాధ్యం కాదు: { $error }
remote-picture-keep-failed = చిత్రాన్ని సేవ్ చేయడం సాధ్యం కాదు: { $error }
remote-picture-remove-failed = చిత్రాన్ని తీసివేయడం సాధ్యం కాదు: { $error }

## Attachments

attachment-count = { $count ->
    [one] ఒక అటాచ్‌మెంట్
   *[other] { $count } అటాచ్‌మెంట్‌లు
}
attachment-save = సేవ్ చేయండి
attachment-save-all = అన్నీ సేవ్ చేయండి
attachment-save-all-tooltip = అన్ని అటాచ్‌మెంట్‌లను ఒక ఫోల్డర్‌లో సేవ్ చేయండి
attachment-save-here = ఇక్కడ సేవ్ చేయండి
attachment-not-downloaded = ఈ మెసేజ్ డౌన్‌లోడ్ చేయబడలేదు.
attachment-not-found = ఈ అటాచ్‌మెంట్ మెసేజ్‌లో కనుగొనబడలేదు.
attachment-read-failed = { $name }ను చదవడం సాధ్యం కాలేదు
attachment-numbered = అటాచ్‌మెంట్ { $number }
attachment-saved-all = { $count ->
    [one] { $count } ఫైల్ { $place }లో సేవ్ చేయబడింది
   *[other] { $count } ఫైల్‌లు { $place }లో సేవ్ చేయబడ్డాయి
}
attachment-saved-some = { $total ->
    [one] { $total } ఫైల్‌లో { $saved } { $place }లో సేవ్ చేయబడింది. { $failed }ను సేవ్ చేయడం సాధ్యం కాలేదు
   *[other] { $total } ఫైల్‌లలో { $saved } { $place }లో సేవ్ చేయబడ్డాయి. { $failed }ను సేవ్ చేయడం సాధ్యం కాలేదు
}
attachment-saved-to = { $path }లో సేవ్ చేయబడింది
attachment-save-failed = { $name }ను సేవ్ చేయడం సాధ్యం కాలేదు: { $error }
attachment-open-failed = { $name }ను తెరవడం సాధ్యం కాలేదు: { $error }
attachment-risky = ఈ ఫైల్ ఏదైనా ప్రోగ్రామ్‌ను రన్ చేయగలదు, కాబట్టి Katna దీన్ని తెరవదు. బదులుగా దీన్ని సేవ్ చేయండి.
attachment-encrypted-open = ఈ ఫైల్ ఎన్‌క్రిప్ట్ చేయబడి వచ్చింది. వేరే చోట తెరవడానికి దీన్ని సేవ్ చేయండి.

## Printing

print-failed = ప్రింట్ చేయడం సాధ్యం కాలేదు: { $error }
print-no-font = ఏ ఫాంట్ కనుగొనబడలేదు
print-opened-as-pdf = అక్కడి నుండి ప్రింట్ చేయడానికి PDFగా తెరవబడింది.
print-not-downloaded = (ఇంకా డౌన్‌లోడ్ చేయబడలేదు.)
print-encrypted = (ఎన్‌క్రిప్ట్ చేయబడింది. దీని టెక్స్ట్‌ను ప్రింట్ చేయడానికి Katna Mailలో తెరవండి.)
print-to = స్వీకర్తలు: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = అటాచ్‌మెంట్‌లను చదవడానికి ఈ మెసేజ్‌ను తెరవండి.
text-copy = కాపీ చేయండి
text-select-all = అన్నీ ఎంచుకోండి

## Settings page: its tabs

settings-tab-general = సాధారణం
settings-tab-inbox = ఇన్‌బాక్స్
settings-tab-accounts = ఖాతాలు
settings-tab-subscriptions = సబ్‌స్క్రిప్షన్‌లు
settings-tab-appearance = రూపురేఖలు
settings-tab-shortcuts = షార్ట్‌కట్‌లు
settings-tab-default-apps = డిఫాల్ట్ యాప్‌లు
settings-tab-folders-rules = ఫోల్డర్‌లు & నియమాలు
settings-tab-compose = కంపోజ్
settings-tab-mcp-server = MCP సర్వర్
settings-tab-feedback = యూజర్ ఫీడ్‌బ్యాక్
settings-tab-experimental = ప్రయోగాత్మకం

## Settings page: tabs still to come

settings-tab-subscriptions-coming = మీకు వచ్చే న్యూస్‌లెటర్‌లు, మెయిలింగ్ లిస్ట్‌లను చూసి, ఒక్క క్లిక్‌తో అన్‌సబ్‌స్క్రయిబ్ చేయండి.
settings-tab-folders-rules-coming = ఫోల్డర్‌లు, లేబుల్‌లను క్రియేట్ చేయండి, పేరు మార్చండి, తరలించండి, దాచండి, ఏవి సింక్ కావాలో ఎంచుకోండి. నియమాలు కొత్త మెయిల్‌ను పంపినవారు, సబ్జెక్ట్ లేదా పదాల ఆధారంగా ఆటోమేటిక్‌గా క్రమబద్ధీకరిస్తాయి, లేబుల్ చేస్తాయి, ఫార్వర్డ్ చేస్తాయి లేదా తొలగిస్తాయి.
settings-tab-mcp-server-coming = ఈ కంప్యూటర్‌లోని AI అసిస్టెంట్‌లు మీ అనుమతితో మీ మెయిల్‌ను వెతకడానికి, చదవడానికి, డ్రాఫ్ట్ చేయడానికి అనుమతించండి.

## Settings > General

settings-general-conversations = సంభాషణ వీక్షణ
settings-general-conversations-group = ఒకే మెయిల్‌కు వచ్చిన రిప్లయిలను గ్రూప్ చేయండి
settings-general-conversations-group-detail = లిస్ట్‌లో ఒక్కో సంభాషణకు ఒక లైన్
settings-general-reading = చదవడం
settings-general-newest-first = కొత్త మెసేజ్ ముందు
settings-general-newest-first-detail = సంభాషణ దాని తాజా రిప్లయితో మొదలవుతుంది
settings-general-full-headers = పూర్తి హెడర్‌లను చూపండి
settings-general-full-headers-detail = ప్రతి మెసేజ్‌లో పంపినవారు, స్వీకర్త, cc, తేదీ, సబ్జెక్ట్ తెరిచే ఉంటాయి
settings-general-full-names = స్వీకర్తల పూర్తి పేర్లు
settings-general-full-names-detail = “నాకు, Ada” బదులుగా “నాకు, Ada Lovelace”
settings-general-mark-read = చదివినట్లు గుర్తు పెట్టడం
settings-general-mark-read-now = తెరిచిన వెంటనే
settings-general-mark-read-1s = 1 సెకను తెరిచి ఉన్న తర్వాత
settings-general-mark-read-3s = 3 సెకన్లు తెరిచి ఉన్న తర్వాత
settings-general-mark-read-never = నేను గుర్తు పెట్టినప్పుడు మాత్రమే
settings-general-reply-button = రిప్లయి బటన్
settings-general-reply-all = అందరికీ రిప్లయి ఇవ్వండి
settings-general-reply-all-detail = ప్రతి మెసేజ్ పక్కన ఉన్న రిప్లయి బటన్ పంపినవారికి మాత్రమే కాకుండా అందరికీ రిప్లయి ఇస్తుంది
settings-general-remote-images = వెబ్ నుండి ఇమేజ్‌లు
settings-general-remote-images-detail = మెసేజ్‌లోని ఇమేజ్‌లను లోడ్ చేస్తే, మీరు దాన్ని తెరిచారని, ఎప్పుడు, సుమారుగా ఎక్కడి నుండి అనేది పంపినవారికి తెలుస్తుంది. ఆఫ్‌లో ఉంటే, ప్రతి మెసేజ్ ముందుగా అడుగుతుంది, మీరు ఎప్పుడైనా పంపినవారి ఇమేజ్‌లను చూపవచ్చు.
settings-general-remote-images-always = ఇమేజ్‌లను ఎల్లప్పుడూ చూపండి
settings-general-remote-images-always-detail = మీరు విశ్వసించే పంపినవారి నుండి మాత్రమే కాకుండా, ప్రతి మెసేజ్‌లో
settings-general-sending = పంపడం
settings-general-sending-detail = పంపిన మెసేజ్‌ను వెనక్కి తీసుకోగలిగేలా, అది ఎంతసేపు వేచి ఉండాలి.
settings-general-offline = ఆఫ్‌లైన్ మెయిల్
settings-general-offline-detail = కనెక్షన్ లేకుండా చదవడానికి, ఇటీవలి మెయిల్ పూర్తిగా డౌన్‌లోడ్ చేయబడుతుంది. పాత మెయిల్ మీరు తెరిచినప్పుడు డౌన్‌లోడ్ అవుతుంది.
settings-general-offline-days = { $count ->
    [one] { $count } రోజు
   *[other] { $count } రోజులు
}
settings-general-offline-years = { $count ->
    [one] { $count } సంవత్సరం
   *[other] { $count } సంవత్సరాలు
}
settings-general-offline-all = అన్ని మెయిల్స్
settings-general-offline-note = తక్కువ రోజులను ఎంచుకున్నా, ఇప్పటికే డౌన్‌లోడ్ చేసిన మెయిల్ అలాగే ఉంటుంది. సర్వర్‌లో ఏదీ మారదు.
settings-general-notifications = నోటిఫికేషన్‌లు
settings-general-notifications-detail = ఇన్‌బాక్స్‌లోని కొత్త మెయిల్ కోసం, Katna Mail మూసి ఉన్నప్పుడు కూడా.
settings-general-new-mail = కొత్త మెయిల్ గురించి నాకు తెలియజేయండి
settings-general-new-mail-detail = అందరికీ రిప్లయి, చదివినట్లు గుర్తు పెట్టడం, ఆర్కైవ్‌లతో
settings-general-new-mail-sound = సౌండ్ ప్లే చేయండి
settings-general-new-mail-sound-detail = డెస్క్‌టాప్ కొత్త మెయిల్ సౌండ్
settings-general-desktop = డెస్క్‌టాప్
settings-general-open-at-login = లాగిన్ అయినప్పుడు Katna Mailను తెరవండి
settings-general-open-at-login-detail = సర్వీస్ రన్ అవుతున్నంత వరకు, ఏదేమైనా లాగిన్ అయినప్పుడు మెయిల్ సింక్ అవుతుంది
settings-general-tray = సిస్టమ్ ట్రేలో Katnaను చూపండి
settings-general-tray-detail = చదవని వాటి సంఖ్య, ఒక మెనూతో
settings-general-unread-badge = టాస్క్‌బార్ ఐకాన్‌పై చదవని వాటి సంఖ్య
settings-general-unread-badge-detail = ఇన్‌బాక్స్‌లో ఎన్ని మెసేజ్‌లు చదవలేదు

## Settings > Inbox

settings-inbox-tabs = ఇన్‌బాక్స్ ట్యాబ్‌లు
settings-inbox-tabs-detail = మీ మెయిల్ ప్రొవైడర్ వెబ్‌సైట్‌లో లాగానే, ఇన్‌బాక్స్‌ను ట్యాబ్‌లుగా విభజించండి.
settings-inbox-tabs-show = ఇన్‌బాక్స్ ట్యాబ్‌లను చూపండి
settings-inbox-tabs-show-detail = ఆఫ్‌లో ఉంటే, ప్రతి ఖాతాకు ఒకే లిస్ట్
settings-inbox-no-accounts = ట్యాబ్‌లను ఎంచుకోవడానికి ఖాతాను జోడించండి.
settings-inbox-tabs-automatic = ఆటోమేటిక్: { $tabs } ({ $provider })
settings-inbox-tabs-off = ట్యాబ్‌లు లేవు
settings-inbox-tabs-gmail = ప్రాథమికం, ప్రమోషన్‌లు, సామాజికం, అప్‌డేట్‌లు, ఫోరమ్‌లు
settings-inbox-tabs-focused = ఫోకస్ చేసినవి, ఇతరాలు
settings-inbox-tabs-zoho = ఇన్‌బాక్స్, న్యూస్‌లెటర్‌లు, నోటిఫికేషన్‌లు
settings-inbox-tabs-shown = చూపబడే ట్యాబ్‌లు. మీరు ఆఫ్ చేసిన ట్యాబ్‌లోని మెయిల్ { $tab }లో ఉంటుంది.

## Settings > Appearance

settings-appearance-reading-pane = రీడింగ్ పేన్
settings-appearance-reading-pane-detail = తెరిచిన సంభాషణ ఎక్కడ కనిపిస్తుంది.
settings-appearance-pane-right = లిస్ట్‌కు కుడి వైపు
settings-appearance-pane-none = విభజన లేదు
settings-appearance-density = సాంద్రత
settings-appearance-density-default = డిఫాల్ట్
settings-appearance-density-compact = కాంపాక్ట్
settings-appearance-scaling = స్కేలింగ్
settings-appearance-scaling-detail = డెస్క్‌టాప్ సొంత స్కేల్‌పై అదనంగా, Katna Mailలోని ప్రతిదాన్నీ పెద్దదిగా లేదా చిన్నదిగా చేస్తుంది: టెక్స్ట్, ఐకాన్‌లు, స్పేసింగ్, డివైడర్‌లు. మీరు పంపే మెయిల్ దాని సొంత ఫాంట్ సైజ్‌ను అలాగే ఉంచుకుంటుంది. చాలా చిన్న సైజ్‌లలో ఐకాన్‌లను క్లిక్ చేయడం కష్టం కావచ్చు.
settings-appearance-theme = థీమ్
settings-appearance-theme-system = డెస్క్‌టాప్ లాగానే
settings-appearance-theme-light = లైట్
settings-appearance-theme-dark = డార్క్
settings-appearance-desktop-colors = డెస్క్‌టాప్ రంగులు
settings-appearance-desktop-colors-use = డెస్క్‌టాప్ రంగులను ఉపయోగించండి
settings-appearance-desktop-colors-use-detail = డెస్క్‌టాప్ కలర్ స్కీమ్, యాక్సెంట్ రంగు
settings-appearance-app-names = యాప్ పేర్లు
settings-appearance-app-names-show = యాప్ పేర్లను చూపండి
settings-appearance-app-names-show-detail = ఎడమ అంచున ఉన్న యాప్ ఐకాన్‌ల కింద పేర్లు
settings-appearance-sender-pictures = పంపినవారి చిత్రాలు
settings-appearance-sender-pictures-show = కంపెనీ లోగోలను చూపండి
settings-appearance-sender-pictures-show-detail = మెసేజ్ ఆధారంగా ఎప్పుడూ కాదు, పంపినవారి డొమైన్ ఆధారంగా వెతికి, ఒక వారం పాటు ఉంచబడతాయి
settings-appearance-important = ముఖ్యమైనవి మార్కర్‌లు
settings-appearance-important-show = ముఖ్యమైనవి మార్కర్‌లను చూపండి
settings-appearance-important-show-detail = లిస్ట్‌లో ప్రతి మెసేజ్ పక్కన
settings-appearance-message-width = మెసేజ్ వెడల్పు
settings-appearance-message-width-limit = మెసేజ్‌ల వెడల్పును పరిమితం చేయండి
settings-appearance-message-width-limit-detail = వెడల్పైన విండోలో పొడవైన లైన్‌లను చదవడం సులభం అవుతుంది
settings-appearance-mail-colors = మెయిల్ రంగులు
settings-appearance-mail-colors-detail = చాలా మెయిల్స్ తెల్లని పేజీ కోసం డిజైన్ చేయబడతాయి. డార్క్ థీమ్‌లో వాటి రంగులు బాగా చదవగలిగే ముదురు రంగులకు మార్చబడతాయి; ఆఫ్‌లో ఉంటే, లేత పేజీపై పంపినవారి రంగులే ఉంటాయి.
settings-appearance-dark-mail = మెయిల్‌కు కూడా ముదురు రంగులు
settings-appearance-dark-mail-detail = థీమ్ డార్క్‌గా ఉన్నప్పుడు మాత్రమే
settings-appearance-attachment-previews = అటాచ్‌మెంట్ ప్రివ్యూలు
settings-appearance-attachment-previews-show = అటాచ్‌మెంట్‌ల ప్రివ్యూలను చూపండి
settings-appearance-attachment-previews-show-detail = ప్రతి ఫైల్ కంటెంట్ యొక్క చిన్న చిత్రం, దాని కార్డ్‌పై

## Settings > Default apps

settings-default-apps-intro = అటాచ్‌మెంట్‌లను క్లిక్ చేసినప్పుడు అవి ఎక్కడ తెరవబడతాయి. వ్యూయర్ నుండి ఫైల్‌ను ఎప్పుడైనా వేరే యాప్‌లో కూడా తెరవవచ్చు. డెస్క్‌టాప్ డిఫాల్ట్ యాప్‌లు దాని సొంత సెట్టింగ్‌లలో సెట్ చేయబడతాయి.
settings-default-apps-pdf = PDF ఫైల్‌లు
settings-default-apps-pdf-detail = పేజీలు, జూమ్‌తో.
settings-default-apps-pictures = చిత్రాలు
settings-default-apps-pictures-detail = ఫోటోలు (నిటారుగా తిప్పబడినవి), PNG, GIF, WebP, BMP, TIFF, SVG.
settings-default-apps-text = టెక్స్ట్ ఫైల్‌లు
settings-default-apps-text-detail = సాధారణ టెక్స్ట్, లాగ్‌లు, కోడ్, ఇతర టెక్స్ట్.
settings-default-apps-sheets = స్ప్రెడ్‌షీట్‌లు
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods), CSV.
settings-default-apps-documents = డాక్యుమెంట్‌లు
settings-default-apps-documents-detail = Word (docx), OpenDocument టెక్స్ట్ (odt).
settings-default-apps-katna = Katna Mail వ్యూయర్
settings-default-apps-system = డెస్క్‌టాప్ డిఫాల్ట్ యాప్
settings-default-apps-ask = ప్రతిసారీ ఏ యాప్ అని అడగండి
settings-default-apps-after-saving = సేవ్ చేసిన తర్వాత
settings-default-apps-show-folder = సేవ్ చేసిన ఫైల్‌లను వాటి ఫోల్డర్‌లో చూపండి
settings-default-apps-show-folder-detail = సేవ్ చేసిన అటాచ్‌మెంట్‌లు ఎంచుకోబడిన స్థితిలో ఫైల్ మేనేజర్‌ను తెరుస్తుంది

## Settings > Compose

settings-compose-send-from = కొత్త మెసేజ్‌లను దీని నుండి పంపండి
settings-compose-send-from-detail = రిప్లయిలు, ఫార్వర్డ్‌లు ఎల్లప్పుడూ మీరు ఉన్న ఖాతా నుండే వెళ్తాయి.
settings-compose-send-from-current = మీరు ఉన్న ఖాతా
settings-compose-send-on-replies = రిప్లయిలపై పంపు బటన్
settings-compose-send-on-replies-detail = రిప్లయి లేదా ఫార్వర్డ్‌లో పంపు బటన్ ఏమి చేస్తుంది. పంపు పక్కన ఉన్న మెనూలో మరొకటి ఉంటుంది.
settings-compose-send-plain = పంపండి
settings-compose-send-archive = పంపి ఆర్కైవ్ చేయండి
settings-compose-signatures = సంతకాలు
settings-compose-signatures-detail = మీ మెసేజ్ కింద, “--” లైన్ తర్వాత జోడించబడుతుంది. కంపోజ్ విండోలో మరొకదాన్ని ఎంచుకోవచ్చు.
settings-compose-untitled = పేరు లేనిది
settings-compose-signature-name = పేరు, ఉదా. ఆఫీస్
settings-compose-signature-first = నా సంతకం
settings-compose-signature-numbered = సంతకం { $number }
settings-compose-signature-delete = తొలగించండి
settings-compose-signature-deleted = సంతకం తొలగించబడింది
settings-compose-signature-new = కొత్తది క్రియేట్ చేయండి
settings-compose-no-signatures = ఇంకా సంతకాలు లేవు.
settings-compose-no-signature = సంతకం లేదు
settings-compose-for-new-mail = కొత్త మెయిల్ కోసం
settings-compose-for-replies = రిప్లయిలు, ఫార్వర్డ్‌ల కోసం
settings-compose-for-replies-detail = మీరు ఒక మెసేజ్‌పై సంతకం చేసిన సంభాషణలో, రిప్లయి దానికి బదులుగా ఆ సంతకంతో మొదలవుతుంది.
settings-compose-format = ఫార్మాట్
settings-compose-plain-text = సాధారణ టెక్స్ట్‌లో రాయండి
settings-compose-plain-text-detail = కొత్త మెయిల్ ఫార్మాటింగ్ లేకుండా మొదలవుతుంది; కంపోజ్ విండోలో మార్చవచ్చు
settings-compose-spelling = స్పెల్లింగ్
settings-compose-spell-check = నేను రాస్తున్నప్పుడు స్పెల్లింగ్ చెక్ చేయండి
settings-compose-spell-check-detail = తప్పు పదాల కింద గీత పడుతుంది, కుడి క్లిక్‌పై సూచనలతో
settings-compose-spell-desktop = డెస్క్‌టాప్ భాష ({ $language })
settings-compose-templates = టెంప్లేట్‌లు
settings-compose-templates-detail = మీరు తరచుగా రాసే మెయిల్‌ను సేవ్ చేసి, దాని నుండి కొత్త మెయిల్ లేదా రిప్లయిని మొదలుపెట్టండి.

## Settings > Shortcuts

settings-shortcuts-set = షార్ట్‌కట్ సెట్
settings-shortcuts-set-detail = మీకు తెలిసిన మెయిల్ యాప్ కీలతో మొదలుపెట్టండి. ఇక్కడ Cmd అంటే Ctrl. మీ సొంత మార్పులు సెట్ పైన ఉంటాయి, డిఫాల్ట్‌లను రీస్టోర్ చేయండి సెట్ కీలకు తిరిగి వెళ్తుంది.
settings-shortcuts-single = సింగిల్-కీ షార్ట్‌కట్‌లు
settings-shortcuts-single-detail = వెబ్‌మెయిల్‌లో లాగా, Ctrl లేదా Alt లేని కీలు: e ఆర్కైవ్ చేస్తుంది, j, k కదుపుతాయి, / వెతుకుతుంది. ఇవి లిస్ట్‌లో, తెరిచిన సంభాషణలో పని చేస్తాయి, టైప్ చేస్తున్నప్పుడు ఎప్పుడూ కాదు.
settings-shortcuts-single-use = సింగిల్-కీ షార్ట్‌కట్‌లను ఉపయోగించండి
settings-shortcuts-single-use-detail = Ctrl షార్ట్‌కట్‌లు ఎల్లప్పుడూ పని చేస్తాయి
settings-shortcuts-how = కీని మార్చడానికి దాన్ని క్లిక్ చేయండి, లేదా కొత్తది జోడించడానికి + క్లిక్ చేయండి, ఆపై కొత్త కీలను నొక్కండి. Esc రద్దు చేస్తుంది.
settings-shortcuts-restore = డిఫాల్ట్‌లను రీస్టోర్ చేయండి
settings-shortcuts-no-key = కీ లేదు
settings-shortcuts-press = కీలను నొక్కండి…
settings-shortcuts-then = { $keys } తర్వాత…
settings-shortcuts-moved = { $keys } ఇప్పుడు “{ $previous }” బదులుగా “{ $action }” చేస్తుంది.
settings-shortcuts-single-off = సింగిల్-కీ షార్ట్‌కట్‌లు ఆఫ్‌లో ఉన్నాయి, కాబట్టి వాటిని ఆన్ చేసిన తర్వాతే ఈ కీ పని చేస్తుంది.
settings-shortcuts-restored = ప్రతి షార్ట్‌కట్‌కు మళ్లీ దాని సెట్ కీలు వచ్చాయి.

## Settings search: the line under a result

settings-general-language-summary = యాప్, తేదీలు, సంఖ్యల భాష
settings-general-reading-summary = కొత్త మెసేజ్ ముందు, పూర్తి హెడర్‌లు, స్వీకర్తల పూర్తి పేర్లు
settings-general-mark-read-summary = తెరిచిన సంభాషణ ఎప్పుడు చదివినట్లు గుర్తు పెట్టబడుతుంది: వెంటనే, 1 లేదా 3 సెకన్ల తర్వాత, లేదా మాన్యువల్‌గా
settings-general-reply-button-summary = ప్రతి మెసేజ్ పక్కన ఉన్న రిప్లయి బటన్ అందరికీ రిప్లయి ఇస్తుంది
settings-general-remote-images-summary = ప్రతి మెసేజ్‌లోని ఇమేజ్‌లను ఎల్లప్పుడూ చూపండి
settings-general-sending-summary = పంపడాన్ని రద్దు చేయండి: పంపిన మెసేజ్‌ను వెనక్కి తీసుకోగలిగేలా అది ఎంతసేపు వేచి ఉండాలి
settings-general-offline-summary = కనెక్షన్ లేకుండా చదవడానికి, ఎన్ని రోజుల ఇటీవలి మెయిల్ పూర్తిగా డౌన్‌లోడ్ చేయబడుతుంది
settings-general-notifications-summary = కొత్త మెయిల్ నోటిఫికేషన్‌లు, వాటి సౌండ్
settings-general-desktop-summary = లాగిన్ అయినప్పుడు Katna Mailను తెరవడం, సిస్టమ్ ట్రే ఐకాన్, టాస్క్‌బార్ ఐకాన్‌పై చదవని వాటి సంఖ్య
settings-accounts-accounts-summary = ఖాతాను జోడించండి లేదా తీసివేయండి, లేదా దాని చిత్రాన్ని మార్చండి
settings-appearance-density-summary = లిస్ట్‌లో డిఫాల్ట్ లేదా కాంపాక్ట్ లైన్‌లు
settings-appearance-scaling-summary = ప్రతిదాన్నీ పెద్దదిగా లేదా చిన్నదిగా చేయండి: టెక్స్ట్, ఐకాన్‌లు, స్పేసింగ్, డివైడర్‌లు
settings-appearance-theme-summary = డెస్క్‌టాప్ లాగానే, లైట్ లేదా డార్క్
settings-appearance-sender-pictures-summary = పంపినవారి డొమైన్ ఆధారంగా వెతికిన కంపెనీ లోగోలు
settings-appearance-important-summary = లిస్ట్‌లో ప్రతి మెసేజ్ పక్కన ఉన్న ముఖ్యమైనవి మార్కర్
settings-appearance-mail-colors-summary = డార్క్ థీమ్‌లో HTML మెయిల్‌కు ముదురు రంగులు, లేదా పంపినవారి రంగులు
settings-appearance-attachment-previews-summary = ప్రతి అటాచ్‌మెంట్ కంటెంట్ యొక్క చిన్న చిత్రం
settings-shortcuts-set-summary = Gmail, Inbox by Gmail, Apple Mail, Outlook లేదా Thunderbird కీలతో మొదలుపెట్టండి
settings-shortcuts-single-summary = వెబ్‌మెయిల్‌లో లాగా, Ctrl లేదా Alt లేని కీలు
settings-default-apps-pdf-summary = PDF అటాచ్‌మెంట్‌లు ఎక్కడ తెరవబడతాయి
settings-default-apps-pictures-summary = ఫోటోలు, చిత్రాలు ఎక్కడ తెరవబడతాయి
settings-default-apps-text-summary = సాధారణ టెక్స్ట్, లాగ్‌లు, కోడ్ ఎక్కడ తెరవబడతాయి
settings-default-apps-sheets-summary = Excel, OpenDocument, CSV ఫైల్‌లు ఎక్కడ తెరవబడతాయి
settings-default-apps-documents-summary = Word, OpenDocument టెక్స్ట్ ఎక్కడ తెరవబడతాయి
settings-default-apps-after-saving-summary = సేవ్ చేసిన అటాచ్‌మెంట్‌లను వాటి ఫోల్డర్‌లో చూపండి
settings-compose-send-from-summary = కొత్త మెయిల్ వెళ్లే ఖాతా: మీరు ఉన్న ఖాతా, లేదా ఎల్లప్పుడూ ఒకే ఖాతా
settings-compose-send-on-replies-summary = రిప్లయిలు, ఫార్వర్డ్‌లపై పంపండి, లేదా పంపి సంభాషణను ఆర్కైవ్ చేయండి
settings-compose-signatures-summary = మీ మెసేజ్ కింద, “--” లైన్ తర్వాత జోడించబడుతుంది
settings-compose-for-new-mail-summary = కొత్త మెయిల్ మొదలయ్యే సంతకం
settings-compose-for-replies-summary = రిప్లయిలు, ఫార్వర్డ్‌లు మొదలయ్యే సంతకం
settings-compose-format-summary = కొత్త మెయిల్‌ను సాధారణ టెక్స్ట్‌లో రాయండి
settings-compose-spelling-summary = రాస్తున్నప్పుడు స్పెల్లింగ్ చెక్, డిక్షనరీ భాష
settings-compose-templates-summary = త్వరలో: మీరు తరచుగా రాసే మెయిల్‌ను సేవ్ చేసి, దాని నుండి కొత్త మెయిల్ లేదా రిప్లయిని మొదలుపెట్టండి
settings-feedback-crash-reports-summary = Katna Mail లేదా దాని బ్యాక్‌గ్రౌండ్ సర్వీస్ క్రాష్ అయినప్పుడు క్రాష్ రిపోర్ట్‌లను ఈ కంప్యూటర్‌లో సేవ్ చేయండి
settings-feedback-saved-summary = ఈ కంప్యూటర్‌లో సేవ్ చేసిన క్రాష్ రిపోర్ట్‌లను చూడండి, కాపీ చేయండి లేదా తొలగించండి
settings-feedback-help-improve-summary = ఏమి తప్పు జరిగిందో సరిచేయడంలో సహాయపడటానికి క్రాష్ రిపోర్ట్‌లను పంపండి; మీరు ఆన్ చేస్తే తప్ప ఆఫ్‌లో ఉంటుంది
settings-experimental-blur-summary = టాప్ బార్ ద్వారా డెస్క్‌టాప్ బ్లర్‌గా కనిపిస్తుంది, మెనూలు ఫ్రాస్టెడ్ గ్లాస్‌లా ఉంటాయి
settings-search-shortcut = కీబోర్డ్ షార్ట్‌కట్
settings-search-tab = సెట్టింగ్‌ల ట్యాబ్
settings-search-none = “{ $query }”కు సరిపోలే సెట్టింగ్‌లు ఏవీ లేవు.
settings-search-results = “{ $query }”కు సరిపోలే సెట్టింగ్‌లు

## Quick settings (the panel that slides in from the right)

quick-title = త్వరిత సెట్టింగ్‌లు
quick-see-all = అన్ని సెట్టింగ్‌లను చూడండి
quick-reading-pane = రీడింగ్ పేన్
quick-pane-right = లిస్ట్‌కు కుడి వైపు
quick-pane-none = విభజన లేదు
quick-density = సాంద్రత
quick-density-default = డిఫాల్ట్
quick-density-compact = కాంపాక్ట్
quick-theme = థీమ్
quick-theme-system = డెస్క్‌టాప్ లాగానే
quick-theme-light = లైట్
quick-theme-dark = డార్క్
quick-desktop-colors = డెస్క్‌టాప్ రంగులు
quick-desktop-colors-detail = డెస్క్‌టాప్ కలర్ స్కీమ్, యాక్సెంట్ రంగు
quick-app-names = యాప్ పేర్లు
quick-app-names-detail = ఎడమ అంచున ఉన్న యాప్ ఐకాన్‌ల కింద పేర్లు
quick-inbox-tabs = ఇన్‌బాక్స్ ట్యాబ్‌లు
quick-inbox-tabs-detail = ప్రతి ఖాతా మెయిల్ ప్రొవైడర్ ట్యాబ్‌లు
quick-choose-tabs = ట్యాబ్‌లను ఎంచుకోండి
quick-choose-tabs-detail = ఒక్కో ఖాతాకు, సెట్టింగ్‌లలో
quick-sending = పంపడం
quick-undo-send = పంపడాన్ని రద్దు చేయండి
quick-undo-send-off = ఆఫ్
quick-undo-send-seconds = { $seconds } సె.
quick-signatures = సంతకాలు
quick-signatures-none = ఇంకా ఏవీ లేవు
quick-signatures-one = { $name }, డిఫాల్ట్‌గా ఉపయోగించబడుతుంది
quick-signatures-many = { $count ->
    [one] { $count } సంతకం; డిఫాల్ట్‌గా { $name }
   *[other] { $count } సంతకాలు; డిఫాల్ట్‌గా { $name }
}
quick-signatures-no-default = { $count ->
    [one] { $count }, డిఫాల్ట్‌గా ఏదీ లేదు
   *[other] { $count }, డిఫాల్ట్‌గా ఏదీ లేదు
}
quick-signature-untitled = పేరు లేనిది
quick-threading = ఈమెయిల్ థ్రెడింగ్
quick-conversation-view = సంభాషణ వీక్షణ
quick-conversation-view-detail = ఒకే మెయిల్‌కు వచ్చిన రిప్లయిలను గ్రూప్ చేయండి
quick-help = సహాయం
quick-tour = టూర్ చేయండి
quick-whats-new = కొత్తగా ఏమి ఉన్నాయి
quick-about = Katna గురించి

## Settings: opening at login

settings-open-at-login-failed = లాగిన్ అయినప్పుడు తెరవడాన్ని మార్చడం సాధ్యం కాలేదు: { $error }

## Settings > Appearance > Scaling

scale-letter = అ
scale-percent = { $percent }%
scale-reset = తిరిగి { $percent }%కు

## Settings > Experimental > Look & Feel

look-intro = ఇంకా పరీక్షిస్తున్న ఫీచర్‌లు. ఇవి మారవచ్చు లేదా తీసివేయబడవచ్చు.
look-heading = లుక్ & ఫీల్
look-window-frame = విండో ఫ్రేమ్
look-window-frame-detail = టైటిల్ బార్, విండో బటన్‌లు, మూలలు, నీడను ఎవరు గీస్తారు.
look-frame-native-kde = నేటివ్: మీ Plasma థీమ్‌లో KDE ఫ్రేమ్
look-frame-native = నేటివ్: డెస్క్‌టాప్ ఫ్రేమ్
look-frame-katna = Katna: టాప్ బార్ టైటిల్ బార్ అవుతుంది
look-frame-katna-note-named = Katna గుండ్రని మూలలను, దాని సొంత నీడను గీస్తుంది. ఫ్రేమ్ ఇకపై { $desktop } థీమ్‌ను అనుసరించదు; విండో నియమాలు ఇప్పటికీ వర్తిస్తాయి.
look-frame-katna-note = Katna గుండ్రని మూలలను, దాని సొంత నీడను గీస్తుంది. ఫ్రేమ్ ఇకపై డెస్క్‌టాప్ థీమ్‌ను అనుసరించదు; విండో నియమాలు ఇప్పటికీ వర్తిస్తాయి.
look-frame-client-side = మీ డెస్క్‌టాప్ ఫ్రేమ్‌ను ప్రతి యాప్‌కే వదిలేస్తుంది, కాబట్టి Katna ఇప్పటికే తన సొంత ఫ్రేమ్‌ను గీస్తోంది.
look-blurred-background = బ్లర్ చేసిన బ్యాక్‌గ్రౌండ్
look-blurred-background-detail = టాప్ బార్, ఫోల్డర్‌ల ద్వారా డెస్క్‌టాప్ బ్లర్‌గా కనిపిస్తుంది, మెనూలు, పాప్‌ఓవర్‌లు ఫ్రాస్టెడ్ గ్లాస్‌లా ఉంటాయి.
look-blur = విండో వెనుక ఉన్నదాన్ని బ్లర్ చేయండి
look-blur-detail = మెయిల్ దృఢమైన కార్డ్‌లపైనే ఉంటుంది, కాబట్టి టెక్స్ట్ కాంట్రాస్ట్ తగ్గదు
look-blur-off-kde = KDE బ్లర్ ఎఫెక్ట్ ఆఫ్‌లో ఉంది. సిస్టమ్ సెట్టింగ్‌లు, విండో మేనేజ్‌మెంట్, డెస్క్‌టాప్ ఎఫెక్ట్‌లలో బ్లర్‌ను ఆన్ చేసి, ఆపై Katna Mailను మళ్లీ తెరవండి.
look-blur-none-gnome = GNOME విండోల వెనుక ఉన్నదాన్ని బ్లర్ చేయదు.
look-blur-none-x11 = మీ విండో మేనేజర్ విండోల వెనుక ఉన్నదాన్ని బ్లర్ చేయదు.
look-blur-none-wayland = మీ కంపోజిటర్ విండోల వెనుక ఉన్నదాన్ని బ్లర్ చేయదు.

## Settings > User feedback (crash reports)

feedback-intro-sending = ఏమి తప్పు జరిగిందో సరిచేయడంలో సహాయపడటానికి కొత్త క్రాష్ రిపోర్ట్‌లు పంపబడతాయి. ఇంకేదీ ఈ కంప్యూటర్‌ను దాటి వెళ్లదు.
feedback-intro-local = Katna దేనినీ ఎక్కడికీ పంపదు. క్రాష్ రిపోర్ట్‌లు ఈ కంప్యూటర్‌లోనే ఉంటాయి, మీరు చూడటానికి లేదా బగ్ రిపోర్ట్‌కు జోడించడానికి.
feedback-crash-reports = క్రాష్ రిపోర్ట్‌లు
feedback-crash-reports-detail = Katna Mail లేదా దాని బ్యాక్‌గ్రౌండ్ సర్వీస్ క్రాష్ అయినప్పుడు రాయబడతాయి.
feedback-save = క్రాష్ రిపోర్ట్‌లను ఈ కంప్యూటర్‌లో సేవ్ చేయండి
feedback-save-detail = మీ హోమ్ ఫోల్డర్, యూజర్, కంప్యూటర్ పేర్లు, ఈమెయిల్ అడ్రస్‌లు చేర్చబడవు
feedback-saved = సేవ్ చేసిన క్రాష్ రిపోర్ట్‌లు
feedback-saved-detail = { $count ->
    [one] తాజా { $count } రిపోర్ట్ ఉంచబడుతుంది.
   *[other] తాజా { $count } రిపోర్ట్‌లు ఉంచబడతాయి.
}
feedback-help-improve = Katnaను మెరుగుపరచడంలో సహాయపడండి
feedback-help-improve-detail = మీరు ఆన్ చేస్తే తప్ప ఆఫ్‌లో ఉంటుంది, మీరు ఎప్పుడైనా ఇక్కడ ఆఫ్ చేయవచ్చు.
feedback-send = క్రాష్ రిపోర్ట్‌లను పంపండి
feedback-send-detail = సేవ్ చేసిన రిపోర్ట్, మీరు ఇక్కడ చూడగలిగినట్లే, Katna క్రాష్ ట్రాకర్‌కు (Sentry, EUలో) వెళ్తుంది. IP అడ్రస్, మెసేజ్‌లు లేదా ఈమెయిల్ అడ్రస్‌లు ఉండవు
feedback-none-saved = క్రాష్ రిపోర్ట్‌లు ఏవీ సేవ్ చేయబడలేదు.
feedback-delete-all = అన్నీ తొలగించండి
feedback-app-daemon = బ్యాక్‌గ్రౌండ్ సర్వీస్
feedback-report-sent = { $date } · పంపబడింది
feedback-view = చూడండి
feedback-view-tooltip = రిపోర్ట్‌ను తెరవండి
feedback-copy-tooltip = బగ్ రిపోర్ట్‌లో పేస్ట్ చేయడానికి దీన్ని కాపీ చేయండి
feedback-copied = క్రాష్ రిపోర్ట్ కాపీ చేయబడింది.
feedback-deleted-all = క్రాష్ రిపోర్ట్‌లు తొలగించబడ్డాయి.
feedback-read-failed = క్రాష్ రిపోర్ట్‌ను చదవడం సాధ్యం కాలేదు: { $error }
feedback-delete-failed = క్రాష్ రిపోర్ట్‌ను తొలగించడం సాధ్యం కాలేదు: { $error }
feedback-delete-all-failed = క్రాష్ రిపోర్ట్‌లను తొలగించడం సాధ్యం కాలేదు: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _ఫైల్
desktop-menu-new-message = _కొత్త మెసేజ్
desktop-menu-quit = _నిష్క్రమించండి
desktop-menu-edit = _ఎడిట్
desktop-menu-undo = _చర్య రద్దు చేయండి
desktop-menu-select-all = _అన్నీ ఎంచుకోండి
desktop-menu-select-none = _ఏదీ ఎంచుకోవద్దు
desktop-menu-find = _కనుగొనండి…
desktop-menu-view = _వీక్షణ
desktop-menu-folder-list = _ఫోల్డర్ లిస్ట్‌ను చూపండి
desktop-menu-refresh = _రిఫ్రెష్ చేయండి
desktop-menu-go = _వెళ్లండి
desktop-menu-inbox = _ఇన్‌బాక్స్
desktop-menu-starred = _నక్షత్రం ఉంచినవి
desktop-menu-sent = _పంపినవి
desktop-menu-drafts = _డ్రాఫ్ట్‌లు
desktop-menu-all-mail = _అన్ని మెయిల్స్
desktop-menu-next = _తర్వాతి సంభాషణ
desktop-menu-previous = _మునుపటి సంభాషణ
desktop-menu-message = _మెసేజ్
desktop-menu-open = _తెరవండి
desktop-menu-reply = _రిప్లయి ఇవ్వండి
desktop-menu-reply-all = _అందరికీ రిప్లయి ఇవ్వండి
desktop-menu-forward = _ఫార్వర్డ్ చేయండి
desktop-menu-archive = _ఆర్కైవ్ చేయండి
desktop-menu-delete = _తొలగించండి
desktop-menu-spam = _స్పామ్‌గా రిపోర్ట్ చేయండి
desktop-menu-move-to = _దీనికి తరలించండి…
desktop-menu-mark-read = _చదివినట్లు గుర్తు పెట్టండి
desktop-menu-mark-unread = _చదవనట్లు గుర్తు పెట్టండి
desktop-menu-star = _నక్షత్రం ఉంచండి
desktop-menu-important = _ముఖ్యమైనదిగా గుర్తు పెట్టండి
desktop-menu-not-important = _ముఖ్యమైనది కాదని గుర్తు పెట్టండి
desktop-menu-settings = _సెట్టింగ్‌లు
desktop-menu-quick-settings = _త్వరిత సెట్టింగ్‌లు
desktop-menu-configure = _Katna Mailను కాన్ఫిగర్ చేయండి…
desktop-menu-help = _సహాయం
desktop-menu-shortcuts = _కీబోర్డ్ షార్ట్‌కట్‌లు
desktop-menu-whats-new = _కొత్తగా ఏమి ఉన్నాయి
desktop-menu-about = _Katna గురించి

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = కదలడం
shortcut-group-actions = చర్యలు
shortcut-group-go-to = దీనికి వెళ్లండి
shortcut-group-app = అప్లికేషన్

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = తర్వాతి సంభాషణ
shortcut-previous = మునుపటి సంభాషణ
shortcut-down = లిస్ట్‌లో కిందికి వెళ్లండి
shortcut-up = లిస్ట్‌లో పైకి వెళ్లండి
shortcut-first = లిస్ట్‌లో మొదటిది
shortcut-last = లిస్ట్‌లో చివరిది
shortcut-page-down = లిస్ట్‌లో ఒక పేజీ కిందికి
shortcut-page-up = లిస్ట్‌లో ఒక పేజీ పైకి
shortcut-open = సంభాషణను తెరవండి
shortcut-back = లిస్ట్‌కు తిరిగి వెళ్లండి
shortcut-scroll-down = కిందికి స్క్రోల్ చేయండి
shortcut-scroll-up = పైకి స్క్రోల్ చేయండి
shortcut-scroll-page-down = ఒక పేజీ కిందికి స్క్రోల్ చేయండి
shortcut-scroll-page-up = ఒక పేజీ పైకి స్క్రోల్ చేయండి
shortcut-compose = కంపోజ్ చేయండి
shortcut-reply = రిప్లయి ఇవ్వండి
shortcut-reply-all = అందరికీ రిప్లయి ఇవ్వండి
shortcut-forward = ఫార్వర్డ్ చేయండి
shortcut-archive = ఆర్కైవ్ చేయండి
shortcut-delete = తొలగించండి
shortcut-spam = స్పామ్‌గా రిపోర్ట్ చేయండి
shortcut-move-to = దీనికి తరలించండి
shortcut-mark-read = చదివినట్లు గుర్తు పెట్టండి
shortcut-mark-unread = చదవనట్లు గుర్తు పెట్టండి
shortcut-star = నక్షత్రం ఉంచండి లేదా తీసివేయండి
shortcut-important = ముఖ్యమైనదిగా గుర్తు పెట్టండి
shortcut-not-important = ముఖ్యమైనది కాదని గుర్తు పెట్టండి
shortcut-check = సంభాషణను ఎంచుకోండి
shortcut-select-all = అన్ని సంభాషణలను ఎంచుకోండి
shortcut-select-none = అన్ని సంభాషణల ఎంపికను తీసివేయండి
shortcut-undo = చివరి చర్యను రద్దు చేయండి
shortcut-go-inbox = ఇన్‌బాక్స్
shortcut-go-starred = నక్షత్రం ఉంచినవి
shortcut-go-sent = పంపినవి
shortcut-go-drafts = డ్రాఫ్ట్‌లు
shortcut-go-all = అన్ని మెయిల్స్
shortcut-search = మెయిల్‌లో వెతకండి
shortcut-navigation = మెనూను చూపండి లేదా కుదించండి
shortcut-quick-settings = త్వరిత సెట్టింగ్‌లు
shortcut-settings = అన్ని సెట్టింగ్‌లు
shortcut-shortcuts = కీబోర్డ్ షార్ట్‌కట్‌లు
shortcut-reload = కొత్త మెయిల్ కోసం చెక్ చేయండి
shortcut-quit = నిష్క్రమించండి

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } తర్వాత { $second }

## Settings > Accounts

accounts-folder-pane = ఫోల్డర్ పేన్
accounts-folder-pane-detail = ఎడమ వైపు ఉన్న పేన్ ఏ ఖాతాల ఫోల్డర్‌లను చూపుతుంది.
accounts-shown-one = ఒకసారి ఒక ఖాతా; ఖాతా కార్డ్‌లో మార్చండి
accounts-shown-all = అన్ని ఖాతాలు, ఒకదాని తర్వాత ఒకటి
accounts-row = ఖాతాలు
accounts-row-detail = ఖాతాను తీసివేస్తే, ఈ కంప్యూటర్‌లోని దాని మెయిల్ యొక్క Katna కాపీ తొలగించబడుతుంది. మెయిల్ సర్వర్‌లో అలాగే ఉంటుంది.
accounts-none = ఇంకా ఖాతాలు లేవు.
accounts-kind-imported = ఇంపోర్ట్ చేయబడింది
accounts-picture-reset = డెస్క్‌టాప్ చిత్రాన్ని ఉపయోగించండి
accounts-picture-change = చిత్రాన్ని మార్చండి
accounts-remove = తీసివేయండి
accounts-delete-all-row = మొత్తం డేటాను తొలగించండి
accounts-delete-all-row-detail = కొత్తగా ఇన్‌స్టాల్ చేసినట్లుగా, మళ్లీ మొదలుపెట్టండి.
accounts-delete-all-about = ప్రతి ఖాతా, సేవ్ చేసిన అన్ని మెయిల్స్, కాంటాక్ట్‌లు, క్యాలెండర్‌లు, సెర్చ్ ఇండెక్స్, మీ సెట్టింగ్‌లు, సేవ్ చేసిన పాస్‌వర్డ్‌లను ఈ కంప్యూటర్ నుండి తొలగిస్తుంది. మీ మెయిల్ సర్వర్‌లలో ఏదీ మారదు.
accounts-delete-all-open = మొత్తం Katna డేటాను తొలగించండి

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } Katna నుండి తీసివేయబడింది.
accounts-removed = { $address } Katna నుండి తీసివేయబడింది. దాని మెయిల్ ఇప్పటికీ సర్వర్‌లో ఉంది.
accounts-all-deleted = మొత్తం Katna డేటా ఈ కంప్యూటర్ నుండి తొలగించబడింది.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address }ను తీసివేయాలా?
accounts-remove-confirm = ఖాతాను తీసివేయండి
accounts-removing = తీసివేస్తోంది…
accounts-remove-local-mail = { $folders ->
    [0] ఈ ఖాతాలోకి ఇంపోర్ట్ చేసిన అన్ని మెయిల్స్
    [one] ఈ ఖాతాలోకి ఇంపోర్ట్ చేసిన, దాని ఫోల్డర్‌లోని అన్ని మెయిల్స్
   *[other] ఈ ఖాతాలోకి ఇంపోర్ట్ చేసిన, దాని { $folders } ఫోల్డర్‌లలోని అన్ని మెయిల్స్
}
accounts-remove-local-settings = దాని Katna సెట్టింగ్‌లు
accounts-remove-mail = { $folders ->
    [0] Katna సేవ్ చేసిన ఈ ఖాతా యొక్క అన్ని మెయిల్స్
    [one] Katna దాని ఫోల్డర్‌లో సేవ్ చేసిన ఈ ఖాతా యొక్క అన్ని మెయిల్స్
   *[other] Katna దాని { $folders } ఫోల్డర్‌లలో సేవ్ చేసిన ఈ ఖాతా యొక్క అన్ని మెయిల్స్
}
accounts-remove-outbox = అవుట్‌బాక్స్‌లో వేచి ఉన్న దాని మెసేజ్‌లు
accounts-remove-settings = దాని సేవ్ చేసిన పాస్‌వర్డ్, దాని Katna సెట్టింగ్‌లు
accounts-delete-all-title = మొత్తం Katna డేటాను తొలగించాలా?
accounts-delete-all-confirm = అన్నీ తొలగించండి
accounts-deleting = తొలగిస్తోంది…
accounts-delete-all-accounts = ప్రతి ఖాతా, Katna సేవ్ చేసిన అన్ని మెయిల్స్, అటాచ్‌మెంట్‌లు
accounts-delete-all-contacts = కాంటాక్ట్‌లు, క్యాలెండర్‌లు, సెర్చ్ ఇండెక్స్
accounts-delete-all-settings = అన్ని సెట్టింగ్‌లు, సంతకాలు, కీబోర్డ్ షార్ట్‌కట్‌లు
accounts-delete-all-passwords = సేవ్ చేసిన ప్రతి పాస్‌వర్డ్
accounts-deleted-heading = ఈ కంప్యూటర్ నుండి తొలగించబడేవి:
accounts-cannot-undo = దీన్ని రద్దు చేయడం సాధ్యం కాదు.
accounts-server-delete-all = మీ మెయిల్ సర్వర్‌లలో ఏదీ మారదు: మీ మెయిల్ అక్కడే ఉంటుంది, ఖాతాను మళ్లీ జోడిస్తే అది మళ్లీ డౌన్‌లోడ్ అవుతుంది. ఫైల్‌ల నుండి ఇంపోర్ట్ చేసిన మెయిల్ Katnaలో మాత్రమే ఉంది; ఆ ఫైల్‌లను తాకరు.
accounts-server-local = ఈ మెయిల్ ఫైల్‌ల నుండి ఇంపోర్ట్ చేయబడింది, కాబట్టి దాని ఏకైక కాపీ Katna వద్దే ఉంది. అది వచ్చిన ఫైల్‌లను తాకరు; దాన్ని తిరిగి పొందడానికి వాటిని మళ్లీ ఇంపోర్ట్ చేయండి.
accounts-server-remove = మెయిల్ సర్వర్‌లో ఏదీ మారదు: మీ మెయిల్ అక్కడే ఉంటుంది, ఖాతాను మళ్లీ జోడిస్తే అది మళ్లీ డౌన్‌లోడ్ అవుతుంది.
accounts-confirm-word = తొలగించు
accounts-confirm-placeholder = “{ accounts-confirm-word }” అని టైప్ చేయండి
accounts-confirm-prompt = నిర్ధారించడానికి, “{ accounts-confirm-word }” అని టైప్ చేయండి:
accounts-cancel = రద్దు చేయండి
