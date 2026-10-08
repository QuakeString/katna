# Katna Mail, Telugu (తెలుగు).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = { $count ->
    [one] { $count } కొత్త ఇమెయిల్
   *[other] { $count } కొత్త ఇమెయిల్‌లు
}
notify-and-more = మరో { $count }
notify-no-subject = (సబ్జెక్ట్ లేదు)
notify-unknown-sender = తెలియని పంపినవారు

## Reminders the user asked for (same buttons)

notify-snooze-back = స్నూజ్ నుండి తిరిగి వచ్చాయి
notify-no-reply = ఇంకా రిప్లయి లేదు
notify-no-reply-to = “{ $subject }”కి ఎవరూ రిప్లయి ఇవ్వలేదు.
notify-follow-up-sent = ఫాలో అప్ పంపబడింది
notify-follow-up-sent-to = “{ $subject }”కు ఎవరూ రిప్లయి ఇవ్వలేదు, కాబట్టి Katna ఫాలో అప్ పంపింది.
notify-follow-up-waiting = ఫాలో అప్ పంపబడలేదు
notify-follow-up-waiting-to = ఈ కంప్యూటర్ ఆఫ్‌లో ఉన్నప్పుడు దీని గడువు వచ్చింది. “{ $subject }” మీ ఇన్‌బాక్స్‌కు తిరిగి వచ్చింది.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } { $subject } తెరిచారు
notify-tracking-clicked = { $who } { $subject }లోని ఒక లింక్‌ను క్లిక్ చేశారు

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail-ని అప్‌డేట్ చేయవచ్చు
notify-update-ready-body = వెర్షన్ { $version } డౌన్‌లోడ్ అయింది. అప్‌డేట్ దాన్ని ఇన్‌స్టాల్ చేసి Katna Mail-ని రీస్టార్ట్ చేస్తుంది.
notify-update = అప్‌డేట్

## Something needs the user, shown once per problem

notify-signed-out = మళ్లీ సైన్ ఇన్ చేయండి
notify-signed-out-body = { $provider } Katnaను { $address } నుండి సైన్ అవుట్ చేసింది. మెయిల్ సింక్ ఆగిపోయింది.
notify-sign-in = సైన్ ఇన్ చేయండి
notify-password-refused = పాస్‌వర్డ్ తిరస్కరించబడింది
notify-password-refused-body = మెయిల్ సర్వర్ { $address } పాస్‌వర్డ్‌ను తిరస్కరించింది. అది మారి ఉండవచ్చు.
notify-new-password = కొత్త పాస్‌వర్డ్
notify-not-sent = “{ $subject }” పంపబడలేదు
notify-not-sent-no-subject = ఒక మెసేజ్ పంపబడలేదు
notify-not-sent-body = ఇది అవుట్‌బాక్స్‌లో ఉంది, ఎందుకో అక్కడ చూపబడుతుంది.
notify-open-outbox = అవుట్‌బాక్స్ తెరవండి

## Reminders of calendar events

notify-event-now = ఇప్పుడు
notify-event-in-minutes = { $count ->
    [one] { $count } నిమిషాల్లో
   *[other] { $count } నిమిషాల్లో
}
notify-event-in-hours = { $count ->
    [one] { $count } గంటల్లో
   *[other] { $count } గంటల్లో
}
notify-event-in-days = { $count ->
    [1] రేపు
    [one] { $count } రోజుల్లో
   *[other] { $count } రోజుల్లో
}
notify-event-all-day = రోజంతా
notify-event-join = చేరండి
notify-event-snooze = 5 నిమిషాలు స్నూజ్ చేయండి
notify-task-done = పూర్తయినట్లు గుర్తించండి

## The buttons of new-mail notifications and reminders

notify-open = తెరవండి
notify-peek = చూడండి
notify-reply = రిప్లయి
notify-reply-placeholder = { $name }కు రిప్లయి…
notify-send = పంపండి
notify-reply-quote-header = { $date }న, { $from } ఇలా రాశారు:
notify-reply-quote-header-no-date = { $from } ఇలా రాశారు:
notify-reply-all = అందరికీ రిప్లయి ఇవ్వండి
notify-mark-read = చదివినట్లు గుర్తు పెట్టండి
notify-mark-all-read = అన్నింటినీ చదివినట్లు గుర్తు పెట్టండి
notify-archive = ఆర్కైవ్ చేయండి
notify-snooze-hour = 1 గంట స్నూజ్ చేయండి
notify-snooze-tomorrow = రేపు
notify-copy-code = { $code } కాపీ చేయండి
notify-link-verify = { $domain }లో ధృవీకరించండి
notify-link-confirm = { $domain }లో నిర్ధారించండి
notify-link-activate = { $domain }లో యాక్టివేట్ చేయండి

## After Archive on a notification: a short note in the same place

notify-archived = ఆర్కైవ్ చేయబడింది
notify-archived-count = { $count ->
    [one] { $count } మెసేజ్ ఇన్‌బాక్స్ నుండి తరలించబడింది
   *[other] { $count } మెసేజ్‌లు ఇన్‌బాక్స్ నుండి తరలించబడ్డాయి
}
notify-undo = రద్దు చేయండి

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = కోడ్ కాపీ అయింది
notify-code-not-copied = కోడ్‌ను కాపీ చేయలేకపోయింది

## it waits for the undo time

notify-reply-sent = { $name }కు రిప్లయి పంపబడింది
notify-open-in-katna = Katnaలో తెరవండి
