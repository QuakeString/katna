# Katna Mail, Telugu (తెలుగు).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } కొత్త ఇమెయిల్
   *[other] { $count } కొత్త ఇమెయిల్‌లు
}
notify-and-more = మరో { $count }
notify-no-subject = (సబ్జెక్ట్ లేదు)
notify-unknown-sender = తెలియని పంపినవారు
notify-snooze-back = స్నూజ్ నుండి తిరిగి వచ్చాయి
notify-no-reply = ఇంకా రిప్లయి లేదు
notify-no-reply-to = “{ $subject }”కి ఎవరూ రిప్లయి ఇవ్వలేదు.
notify-tracking-opened = { $who } { $subject } తెరిచారు
notify-tracking-clicked = { $who } { $subject }లోని ఒక లింక్‌ను క్లిక్ చేశారు

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail-ని అప్‌డేట్ చేయవచ్చు
notify-update-ready-body = వెర్షన్ { $version } డౌన్‌లోడ్ అయింది. అప్‌డేట్ దాన్ని ఇన్‌స్టాల్ చేసి Katna Mail-ని రీస్టార్ట్ చేస్తుంది.
notify-update = అప్‌డేట్
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

## Its buttons

notify-open = తెరవండి
notify-reply-all = అందరికీ రిప్లయి ఇవ్వండి
notify-mark-read = చదివినట్లు గుర్తు పెట్టండి
notify-mark-all-read = అన్నింటినీ చదివినట్లు గుర్తు పెట్టండి
notify-archive = ఆర్కైవ్ చేయండి
