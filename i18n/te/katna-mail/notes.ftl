# Katna Mail, Telugu (తెలుగు): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = నోట్స్
notes-view-reminders = రిమైండర్‌లు
notes-view-archive = ఆర్కైవ్
notes-view-trash = ట్రాష్
notes-edit-labels = లేబుల్‌లను ఎడిట్ చేయండి
notes-search = నోట్స్‌ను శోధించండి
notes-loading = మీ నోట్స్‌ను తెరుస్తోంది…

## Board

notes-take-a-note = గమనిక రాయండి…
notes-new-list = కొత్త జాబితా
notes-new-note = కొత్త నోట్
notes-pinned = పిన్ చేసినవి
notes-others = ఇతరాలు
notes-empty = మీరు జోడించిన గమనికలు ఇక్కడ కనిపిస్తాయి
notes-archive-empty = మీరు ఆర్కైవ్ చేసిన గమనికలు ఇక్కడ కనిపిస్తాయి
notes-trash-empty = ట్రాష్‌లో గమనికలు లేవు
notes-none-found = సరిపోలే గమనికలు లేవు
notes-label-empty = ఈ లేబుల్ ఉన్న గమనికలు ఇంకా లేవు
notes-reminders-empty = రాబోయే రిమైండర్‌లు ఉన్న గమనికలు ఇక్కడ కనిపిస్తాయి
notes-trash-note = ట్రాష్‌లోని గమనికలు 7 రోజుల తర్వాత తొలగించబడతాయి.
notes-empty-trash = ట్రాష్‌ను ఖాళీ చేయండి
notes-ticked = { $count ->
    [one] + { $count } టిక్ చేసిన అంశం
   *[other] + { $count } టిక్ చేసిన అంశాలు
}
notes-select = గమనికను ఎంచుకోండి
notes-selected = { $count ->
    [one] { $count } ఎంచుకోబడింది
   *[other] { $count } ఎంచుకోబడ్డాయి
}
notes-select-clear = ఎంపికను క్లియర్ చేయండి

## A note's buttons

notes-pin = గమనికను పిన్ చేయండి
notes-unpin = గమనికను అన్‌పిన్ చేయండి
notes-archive = ఆర్కైవ్ చేయండి
notes-unarchive = ఆర్కైవ్ నుండి తీసివేయండి
notes-delete = గమనికను తొలగించండి
notes-restore = పునరుద్ధరించండి
notes-delete-forever = శాశ్వతంగా తొలగించండి
notes-color = నేపథ్య రంగు
notes-checkboxes = చెక్‌బాక్స్‌లను చూపండి లేదా దాచండి
notes-labels = లేబుల్‌లు
notes-close = మూసివేయండి
notes-more = మరిన్ని
notes-make-copy = కాపీ చేయండి
notes-remind = నాకు గుర్తు చేయండి
notes-add-picture = చిత్రాన్ని జోడించండి
notes-history = వెర్షన్ చరిత్ర
notes-ai = రాయడంలో సహాయం చేయండి
notes-send-as-mail = మెయిల్‌గా పంపండి
notes-save-markdown = Markdownగా సేవ్ చేయండి
notes-save-pdf = PDFగా సేవ్ చేయండి

## The open note

notes-title = శీర్షిక
notes-edited = సవరించినది: { $date }
notes-on-this-computer = ఈ కంప్యూటర్‌లో
notes-where = ఈ గమనిక ఎక్కడ ఉంచబడింది
notes-untitled = శీర్షిక లేని గమనిక

## Pictures

notes-picture-choose = చిత్రాలను జోడించండి
notes-picture-remove = చిత్రాన్ని తీసివేయండి
notes-picture-too-big = గమనికలో { $size } వరకు ఉన్న చిత్రాలను ఉంచవచ్చు
notes-picture-kind = ఆ ఫైల్ Katna చూపగల చిత్రం కాదు
notes-picture-unreadable = { $name }ను చదవలేకపోయింది: { $error }

## Reminders

notes-remind-me = నాకు గుర్తు చేయండి
notes-remind-off = రిమైండర్‌ను తీసివేయండి
notes-remind-in-the-past = ఇంకా గడవని సమయాన్ని ఎంచుకోండి
notes-remind-today = ఈరోజు, { $time }
notes-remind-tomorrow = రేపు, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = { $when }కు రిమైండర్ సెట్ చేయబడింది
notes-reminder-off = రిమైండర్ తీసివేయబడింది

## Links between notes

notes-link-note = గమనికను లింక్ చేయండి
notes-link-new = కొత్త గమనిక “{ $title }”
notes-linked-from = వీటి నుండి లింక్ చేయబడింది
notes-link-gone = ఆ గమనిక ఇప్పుడు ఇక్కడ లేదు
notes-new-note-gone = కొత్త నోట్ కనిపించడం లేదు.

## Version history

notes-versions = వెర్షన్‌లు
notes-version-now = ఇప్పుడు
notes-version-here = మీరు, ఈ కంప్యూటర్‌లో
notes-version-yesterday = నిన్న, { $time }
notes-version-changes = { $count ->
    [one] { $count } మార్పు
   *[other] { $count } మార్పులు
}
notes-version-from = { $device } నుండి
notes-version-elsewhere = మరొక పరికరం నుండి
notes-version-created = క్రియేట్ చేయబడింది
notes-version-restore = ఈ వెర్షన్‌ను పునరుద్ధరించండి
notes-version-restored = వెర్షన్ పునరుద్ధరించబడింది
notes-history-none = ఇంకా మునుపటి వెర్షన్‌లు లేవు

## AI help

notes-ai-tidy = టెక్స్ట్‌ను చక్కదిద్దండి
notes-ai-checklist = చెక్‌లిస్ట్‌గా మార్చండి
notes-ai-summarise = సారాంశం
notes-ai-empty = ముందుగా ఏదైనా రాయండి
notes-ai-tidied = టెక్స్ట్ చక్కదిద్దబడింది. Ctrl+Z పాతదాన్ని తిరిగి తెస్తుంది.
notes-ai-listed = చెక్‌లిస్ట్‌గా మార్చబడింది. Ctrl+Z పాతదాన్ని తిరిగి తెస్తుంది.
notes-ai-summarised = సారాంశం పైన జోడించబడింది

## Labels

notes-label-note = గమనికకు లేబుల్ ఇవ్వండి
notes-label-name = లేబుల్ పేరును నమోదు చేయండి
notes-label-create = “{ $name }” క్రియేట్ చేయండి
notes-label-remove = లేబుల్‌ను తీసివేయండి
notes-label-delete = లేబుల్‌ను తొలగించండి
notes-labels-none = ఇంకా లేబుల్‌లు లేవు. గమనిక లేబుల్ బటన్ నుండి ఒకదాన్ని జోడించండి.
notes-labels-done = పూర్తయింది
notes-label-renamed = లేబుల్ పేరు “{ $name }”గా మార్చబడింది
notes-label-deleted = లేబుల్ “{ $name }” తొలగించబడింది

## A note about a mail

notes-mail = మెయిల్
notes-open-mail = మెయిల్‌ను తెరవండి
notes-open-note = గమనికను తెరవండి

## Meeting notes

notes-meeting-take = మీటింగ్ గమనిక రాయండి
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = హాజరయ్యేవారు: { $names }
notes-meeting-notes = గమనికలు
notes-meeting-actions = చర్య అంశాలు
notes-event = ఈవెంట్
notes-open-event = ఈవెంట్‌ను తెరవండి

## Formatting

notes-format = ఫార్మాటింగ్
notes-format-heading-1 = శీర్షిక 1
notes-format-heading-2 = శీర్షిక 2
notes-format-normal = సాధారణ టెక్స్ట్
notes-format-bold = బోల్డ్
notes-format-italic = ఇటాలిక్
notes-format-underline = అండర్‌లైన్
notes-format-quote = కోట్
notes-format-code = కోడ్
notes-format-divider = డివైడర్
notes-format-clear = ఫార్మాటింగ్‌ను క్లియర్ చేయండి

## Tasks

notes-make-task = టాస్క్‌గా చేయండి

## Colors (tooltips)

notes-color-none = రంగు లేదు
notes-color-coral = పగడం
notes-color-peach = పీచ్
notes-color-sand = ఇసుక
notes-color-mint = పుదీనా
notes-color-sage = సేజ్
notes-color-fog = పొగమంచు
notes-color-storm = తుఫాను
notes-color-dusk = సంధ్య
notes-color-blossom = పుష్పం
notes-color-clay = మట్టి
notes-color-chalk = సుద్ద

## Messages at the foot of the window

notes-archived = గమనిక ఆర్కైవ్ చేయబడింది
notes-unarchived = గమనిక ఆర్కైవ్ నుండి తీసివేయబడింది
notes-trashed = గమనిక ట్రాష్‌కు తరలించబడింది
notes-restored = గమనిక పునరుద్ధరించబడింది
notes-saved = గమనిక సేవ్ చేయబడింది
notes-pinned-count = { $count ->
    [one] గమనిక పిన్ చేయబడింది
   *[other] { $count } గమనికలు పిన్ చేయబడ్డాయి
}
notes-unpinned-count = { $count ->
    [one] గమనిక అన్‌పిన్ చేయబడింది
   *[other] { $count } గమనికలు అన్‌పిన్ చేయబడ్డాయి
}
notes-colored-count = { $count ->
    [one] రంగు మార్చబడింది
   *[other] { $count } గమనికల రంగు మార్చబడింది
}
notes-archived-count = { $count ->
    [one] గమనిక ఆర్కైవ్ చేయబడింది
   *[other] { $count } గమనికలు ఆర్కైవ్ చేయబడ్డాయి
}
notes-unarchived-count = { $count ->
    [one] గమనిక ఆర్కైవ్ నుండి తీసివేయబడింది
   *[other] { $count } గమనికలు ఆర్కైవ్ నుండి తీసివేయబడ్డాయి
}
notes-trashed-count = { $count ->
    [one] గమనిక ట్రాష్‌కు తరలించబడింది
   *[other] { $count } గమనికలు ట్రాష్‌కు తరలించబడ్డాయి
}
notes-restored-count = { $count ->
    [one] గమనిక పునరుద్ధరించబడింది
   *[other] { $count } గమనికలు పునరుద్ధరించబడ్డాయి
}
notes-copied-count = { $count ->
    [one] కాపీ చేయబడింది
   *[other] { $count } కాపీలు చేయబడ్డాయి
}
notes-empty-discarded = ఖాళీ గమనిక విస్మరించబడింది
notes-mail-gone = ఆ మెయిల్ ఇకపై ఇక్కడ లేదు
notes-deleted-forever = { $count ->
    [one] గమనిక శాశ్వతంగా తొలగించబడింది
   *[other] { $count } గమనికలు శాశ్వతంగా తొలగించబడ్డాయి
}
