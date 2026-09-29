# Katna Mail, Telugu (తెలుగు): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = ఈరోజు
calendar-today-tip = ఈరోజుకు వెళ్లండి
calendar-view-day = రోజు
calendar-view-week = వారం
calendar-view-month = నెల
calendar-view-schedule = షెడ్యూల్
calendar-previous-day = మునుపటి రోజు
calendar-next-day = తదుపరి రోజు
calendar-previous-week = మునుపటి వారం
calendar-next-week = తదుపరి వారం
calendar-previous-month = మునుపటి నెల
calendar-next-month = తదుపరి నెల
calendar-previous-period = ముందు
calendar-next-period = తర్వాత
calendar-title-months = { $first } – { $last }
calendar-loading = లోడ్ అవుతోంది…
calendar-read-failed = క్యాలెండర్‌ను చదవడం సాధ్యం కాలేదు: { $error }
calendar-local = ఈ కంప్యూటర్‌లో
calendar-account-gone = తీసివేసిన ఖాతా
calendar-empty-title = ఇంకా క్యాలెండర్‌లు లేవు
calendar-empty-text = మీ Google, Microsoft ఖాతాల క్యాలెండర్‌లు, అలాగే CalDAV అందించే ఇతర సర్వర్‌ల క్యాలెండర్‌లు సింక్ అయిన తర్వాత Katna వాటిని ఇక్కడ చూపిస్తుంది.
calendar-schedule-empty = రాబోయే రెండు నెలల్లో ఏమీ ప్లాన్ చేయలేదు.
calendar-no-title = (శీర్షిక లేదు)
calendar-all-day = రోజంతా
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = ఇంకా { $count }
calendar-repeats = పునరావృతమవుతుంది
calendar-join = చేరండి
calendar-email-guests = అతిథులకు ఇమెయిల్ చేయండి
calendar-running-late = ఆలస్యమవుతోంది
calendar-late-subject = ఆలస్యమవుతోంది: { $title }
calendar-late-body = క్షమించండి, { $title } కోసం నాకు కొన్ని నిమిషాలు ఆలస్యమవుతోంది. నేను త్వరలోనే వస్తాను.
calendar-guests =
    { $count ->
        [one] { $count } అతిథి
       *[other] { $count } అతిథులు
    }
calendar-guest-answers = { $yes } అవును, { $maybe } కావచ్చు, { $no } కాదు, { $waiting } వేచి ఉన్నారు
calendar-organizer = నిర్వాహకులు
calendar-optional = ఐచ్ఛికం
calendar-open-web = బ్రౌజర్‌లో తెరవండి
calendar-close = మూసివేయండి

## Adding, changing and deleting events.

calendar-add-title = శీర్షికను జోడించండి
calendar-add-location = స్థానాన్ని జోడించండి
calendar-add-notes = వివరణను జోడించండి
calendar-add-guests = అతిథులను జోడించండి
calendar-remove-guest = తీసివేయండి
calendar-add-meet = Google Meet వీడియో కాల్‌ను జోడించండి
calendar-add-teams = Teams మీటింగ్‌ను జోడించండి
calendar-has-call = వీడియో కాల్ జోడించబడింది
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = రోజంతా
calendar-more-options = మరిన్ని ఎంపికలు
calendar-save = సేవ్ చేయండి
calendar-saved = ఈవెంట్ సేవ్ అయింది
calendar-deleted = ఈవెంట్ తొలగించబడింది
calendar-discard = మార్పులను విస్మరించండి
calendar-edit = ఈవెంట్‌ను ఎడిట్ చేయండి
calendar-delete = ఈవెంట్‌ను తొలగించండి
calendar-event-details = ఈవెంట్ వివరాలు
calendar-busy = బిజీ
calendar-free = ఖాళీ
calendar-cancel = రద్దు చేయండి
calendar-ok = సరే
calendar-read-only = ఈ క్యాలెండర్‌లోని ఈవెంట్‌లను మీరు మార్చలేరు
calendar-none-editable = ఈవెంట్‌లను జోడించగల క్యాలెండర్ ఇంకా లేదు
calendar-no-such-time = మీ టైమ్ జోన్‌లో ఆ సమయం లేదు
calendar-end-before-start = ఈవెంట్ ప్రారంభం కావడానికి ముందే ముగుస్తుంది
calendar-repeat-never = పునరావృతం కాదు
calendar-repeat-daily = ప్రతిరోజూ
calendar-repeat-weekly = వారానికోసారి: { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] నెలవారీ: మొదటి { $weekday }
        [2] నెలవారీ: రెండవ { $weekday }
        [3] నెలవారీ: మూడవ { $weekday }
        [4] నెలవారీ: నాలుగవ { $weekday }
       *[other] నెలవారీ: చివరి { $weekday }
    }
calendar-repeat-yearly = వార్షికంగా: { $day }
calendar-repeat-weekdays = ప్రతి వారం రోజు (సోమవారం నుండి శుక్రవారం వరకు)
calendar-repeat-custom = అనుకూలం
calendar-reminder-none = నోటిఫికేషన్ లేదు
calendar-reminder-at-start = ప్రారంభంలో
calendar-reminder-minutes =
    { $count ->
        [one] { $count } నిమిషం ముందు
       *[other] { $count } నిమిషాల ముందు
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } గంట ముందు
       *[other] { $count } గంటల ముందు
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } రోజు ముందు
       *[other] { $count } రోజుల ముందు
    }
calendar-scope-edit-title = పునరావృత ఈవెంట్‌ను ఎడిట్ చేయండి
calendar-scope-delete-title = పునరావృత ఈవెంట్‌ను తొలగించండి
calendar-scope-this = ఈ ఈవెంట్
calendar-scope-following = ఈ మరియు తర్వాతి ఈవెంట్‌లు
calendar-scope-all = అన్ని ఈవెంట్‌లు
calendar-scope-respond-title = పునరావృత ఈవెంట్‌కు సమాధానం
calendar-going = మీరు వెళ్తున్నారా?
calendar-answer-yes = అవును
calendar-answer-no = కాదు
calendar-answer-maybe = కావచ్చు
calendar-answered-yes = మీరు వెళ్తున్నారు
calendar-answered-no = మీరు వెళ్లడం లేదు
calendar-answered-maybe = మీరు వెళ్లవచ్చు

## The card at the top of a mail with an invitation.

calendar-invite = ఆహ్వానం
calendar-invite-cancelled = ఈవెంట్ రద్దు చేయబడింది
calendar-invite-reply = { $name } ప్రతిస్పందించారు
calendar-invite-reply-yes = { $name } అంగీకరించారు
calendar-invite-reply-no = { $name } తిరస్కరించారు
calendar-invite-reply-maybe = { $name } వెళ్లవచ్చు
calendar-invite-organizer = నిర్వాహకులు: { $name }
calendar-invite-open = క్యాలెండర్‌లో తెరవండి
calendar-invite-not-yet = ఇంకా మీ క్యాలెండర్‌లో లేదు. సింక్ అయ్యాక సమాధానం ఇవ్వవచ్చు.
calendar-invite-your-day = మీ రోజు
calendar-invite-clashes =
    { $count ->
        [one] { $count } ఈవెంట్‌తో ఘర్షణ
       *[other] { $count } ఈవెంట్‌లతో ఘర్షణ
    }

## The day's agenda beside the mail.

agenda-show = రోజు ఎజెండాను చూపండి
agenda-hide = ఎజెండాను దాచండి
agenda-today = ఈరోజు, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = ఈ రోజున ఏమీ ప్లాన్ చేయలేదు.
