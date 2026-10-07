# Katna Mail, Telugu (తెలుగు).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = ఎప్పటి వరకు స్నూజ్ చేయాలి…
snooze-later-today = ఈరోజు తర్వాత
snooze-tomorrow = రేపు
snooze-this-weekend = ఈ వారాంతం
snooze-next-week = వచ్చే వారం
snooze-pick = తేదీ, సమయాన్ని ఎంచుకోండి
snooze-back = సమయాలకు తిరిగి వెళ్లండి
snooze-type-placeholder = సమయాన్ని టైప్ చేయండి
snooze-type-hint = ఉదా. “tue 3pm”, “tomorrow” లేదా “in 2 hours”
snooze-type-hint-unclear = Katna దాన్ని సమయంగా అర్థం చేసుకోలేదు
snooze-type-unclear = “{ $text }” Katnaకు తెలిసిన సమయం కాదు

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = స్నూజ్
remind-tab = నాకు గుర్తు చేయండి
snooze-says = అప్పటి వరకు దాన్ని దాచుతుంది
remind-says = దాన్ని అక్కడే ఉంచి మీకు తెలియజేస్తుంది
remind-before-due = గడువుకు ముందు
remind-note = గమనిక (ఐచ్ఛికం)
remind-note-placeholder = ఖాళీగా వదిలితే సబ్జెక్ట్
toast-remind-set = { $date }కు రిమైండర్ సెట్ చేయబడింది
remind-chat-line = రిమైండర్ { $date } · { $title }
remind-done = పూర్తయింది
toast-remind-done = రిమైండర్ పూర్తయింది
snooze-chat-line = { $date } వరకు స్నూజ్ చేయబడింది
snooze-chat-change = మార్చండి

## The date and time picker

snooze-cancel = రద్దు చేయండి
snooze-save = సేవ్ చేయండి
snooze-in-the-past = ఇప్పటి కంటే తర్వాతి సమయాన్ని ఎంచుకోండి.

## beside Send

follow-up-menu = రిప్లయి రాకపోతే ఫాలో అప్…
follow-up-title = రిప్లయి రాకపోతే ఫాలో అప్
follow-up-off = ఆఫ్
follow-up-days = { $days ->
    [one] { $days } రోజు
   *[other] { $days } రోజులు
}
follow-up-weeks = { $weeks ->
    [one] { $weeks } వారం
   *[other] { $weeks } వారాలు
}
follow-up-pick = ఎంచుకోండి…
follow-up-pick-title = ఈ సమయానికి రిప్లయి రాకపోతే ఫాలో అప్
follow-up-remind = నాకు గుర్తు చేయండి
follow-up-remind-note = సంభాషణ మీ ఇన్‌బాక్స్ పైకి తిరిగి వస్తుంది
follow-up-send = నా తరఫున ఫాలో అప్ పంపండి
follow-up-send-note = అదే వ్యక్తులకు, అదే సంభాషణలో
follow-up-send-encrypted = ఎన్‌క్రిప్ట్ చేసిన మెయిల్‌కు కాదు
follow-up-text-placeholder = ఏం రాయాలి
follow-up-text-named = హాయ్ { $name }, కింద ఉన్న నా మెసేజ్ మీరు చూశారో లేదో తెలుసుకోవాలనుకున్నాను.
follow-up-text = హాయ్, కింద ఉన్న నా మెసేజ్ మీరు చూశారో లేదో తెలుసుకోవాలనుకున్నాను.
follow-up-template = టెంప్లేట్‌ను ఉపయోగించండి
follow-up-signature = మీ సంతకం జోడించబడుతుంది
follow-up-again = ఇంకా రిప్లయి రాకపోతే, ఇంత సమయం తర్వాత మళ్లీ ఫాలో అప్ చేయండి
follow-up-note = సంభాషణలో ఎవరైనా రిప్లయి ఇచ్చిన వెంటనే ఆగిపోతుంది. ఆటో-రిప్లయిలు లెక్కలోకి రావు.
follow-up-note-send = సంభాషణలో ఎవరైనా రిప్లయి ఇచ్చిన వెంటనే ఆగిపోతుంది. వారం రోజుల్లో { $start } నుండి { $end } వరకు వెళ్తుంది, ఒక రోజు కంటే ఎక్కువ ఆలస్యం కాదు.
follow-up-cancel = రద్దు చేయండి
follow-up-done = పూర్తయింది
follow-up-chip-send = { $time }లో ఫాలో అప్
follow-up-chip-remind = { $time }లో రిమైండర్
follow-up-chip-send-on = ఫాలో అప్ { $date }
follow-up-chip-remind-on = రిమైండర్ { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = ఇంకా రిప్లయి లేదు
follow-up-card-title-waiting = మీ ఫాలో అప్ వేచి ఉంది
follow-up-card-send = Katna మీ ఫాలో అప్‌ను { $date }న పంపుతుంది. ఎవరైనా రిప్లయి ఇస్తే ఆగిపోతుంది.
follow-up-card-send-twice = Katna మీ ఫాలో అప్‌ను { $date }న పంపి, ఆ తర్వాత మరోసారి పంపుతుంది. ఎవరైనా రిప్లయి ఇస్తే ఆగిపోతుంది.
follow-up-card-remind = ఎవరూ రిప్లయి ఇవ్వకపోతే, ఈ సంభాషణ { $date }న మీ ఇన్‌బాక్స్‌కు తిరిగి వస్తుంది.
follow-up-card-waiting = మీ కంప్యూటర్ ఆఫ్‌లో ఉన్నప్పుడు దీని గడువు వచ్చింది, కాబట్టి ఆలస్యంగా పంపలేదు. ఇప్పుడే పంపండి, కొత్త సమయాన్ని ఎంచుకోండి, లేదా ఆపివేయండి.
follow-up-card-edit = ఎడిట్ చేయండి
follow-up-card-edit-title = ఎప్పుడు ఫాలో అప్ చేయాలి
follow-up-card-send-now = ఇప్పుడే పంపండి
follow-up-card-stop = ఆపండి
follow-up-chat-send = ఫాలో అప్ · ఎవరూ రిప్లయి ఇవ్వకపోతే { $date }
follow-up-chat-step = ఫాలో అప్ { $steps }లో { $step } · ఎవరూ రిప్లయి ఇవ్వకపోతే { $date }
follow-up-chat-waiting = ఫాలో అప్ వేచి ఉంది · మీ కంప్యూటర్ ఆఫ్‌లో ఉన్నప్పుడు దీని గడువు వచ్చింది
follow-up-chat-remind = రిప్లయి రాకపోతే { $date }న ఇన్‌బాక్స్‌కు తిరిగి వస్తుంది
toast-follow-up-sent = ఫాలో అప్ పంపబడింది
toast-follow-up-stopped = ఫాలో అప్ ఆపివేయబడింది
toast-follow-up-moved = ఫాలో అప్ { $date }కు మార్చబడింది
nudge-row = { $days ->
    [one] { $days } రోజు క్రితం
   *[other] { $days } రోజుల క్రితం
} పంపారు. ఫాలో అప్ చేయాలా?
nudge-row-tip = ఇందులోని అందరికీ ఫాలో అప్ రాయండి
nudge-follow-up = ఫాలో అప్
nudge-dismiss = తీసివేయండి
nudge-card-title = ఇంకా రిప్లయి లేదు
nudge-card-text = మీరు { $days ->
    [one] { $days } రోజు క్రితం
   *[other] { $days } రోజుల క్రితం
} ఏదో అడిగారు, ఎవరూ జవాబు ఇవ్వలేదు.
nudge-chat-line = { $days ->
    [one] { $days } రోజు క్రితం
   *[other] { $days } రోజుల క్రితం
} పంపారు, ఇంకా రిప్లయి లేదు
toast-nudge-dismissed = గుర్తుచేత తీసివేయబడింది
