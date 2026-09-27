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
