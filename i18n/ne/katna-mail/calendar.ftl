# Katna Mail, Nepali (नेपाली): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = आज
calendar-today-tip = आजमा जानुहोस्
calendar-view-day = दिन
calendar-view-week = हप्ता
calendar-view-month = महिना
calendar-view-year = वर्ष
calendar-view-schedule = तालिका
calendar-view-days =
    { $count ->
        [one] { $count } दिन
       *[other] { $count } दिन
    }
calendar-options = विकल्पहरू
calendar-density = घनत्व
calendar-density-responsive = तपाईंको स्क्रिनअनुसार
calendar-density-comfortable = आरामदायी
calendar-density-compact = कम्प्याक्ट
calendar-custom-days = अनुकूल दृश्य
calendar-second-zone = दोस्रो समय क्षेत्र
calendar-zone-none = कुनै पनि होइन
calendar-zone = { $zone } ({ $offset })
calendar-share-free = खाली समय साझा गर्नुहोस्
calendar-free-subject = मेरो खाली समय
calendar-free-intro = यी केही समय हुन् जब म खाली छु ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = अर्का केही कार्यदिनमा मसँग खाली समय छैन।
calendar-previous-day = अघिल्लो दिन
calendar-next-day = अर्को दिन
calendar-previous-week = अघिल्लो हप्ता
calendar-next-week = अर्को हप्ता
calendar-previous-month = अघिल्लो महिना
calendar-next-month = अर्को महिना
calendar-previous-year = अघिल्लो वर्ष
calendar-next-year = अर्को वर्ष
calendar-previous-period = पहिले
calendar-next-period = पछि
calendar-title-months = { $first } – { $last }
calendar-loading = लोड हुँदैछ…
calendar-read-failed = पात्रो पढ्न सकिएन: { $error }
calendar-sets = पात्रो समूह
calendar-set-add = देखाइएका पात्रोलाई समूहका रूपमा सुरक्षित गर्नुहोस्
calendar-set-name = समूहको नाम
calendar-set-remove = समूह हटाउनुहोस्
calendar-local = यो कम्प्युटरमा
calendar-account-gone = हटाइएको खाता
calendar-account-sign-in = पात्रोहरू देखाउन फेरि साइन इन गर्नुहोस्
calendar-account-signed-in = { $address } मा फेरि साइन इन भयो। तपाईंका पात्रोहरू ल्याउँदै…
calendar-account-sign-in-refused = { $provider } ले Katna लाई भित्र आउन दिएन। फेरि प्रयास गर्नुहोस्, र आफ्ना पात्रोहरूमा पहुँच दिनुहोस्।
calendar-account-refused = सर्भरले पासवर्ड स्वीकार गरेन। Yahoo, iCloud, Zoho र अरूलाई एप पासवर्ड चाहिन्छ।
calendar-account-change-password = पासवर्ड बदल्नुहोस्
calendar-account-change-password-tooltip = सेटिङहरू > खाताहरू खोल्नुहोस्
calendar-account-not-enabled = Katna का लागि पात्रो पहुँच अझै सक्रिय गरिएको छैन।
calendar-account-failed = पात्रोहरू पढ्न सकिएन।
calendar-account-error = पात्रोहरू पढ्न सकिएन: { $reason }
calendar-account-none = कुनै पात्रो भेटिएन
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = कुनै पात्रो भेटिएन: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } ले { $provider } बाट साइन इन गरिएको Katna लाई मात्र पात्रोहरू देखाउँछ।
calendar-account-sign-in-with = { $provider } बाट साइन इन गर्नुहोस्
calendar-account-looking = पात्रोहरू खोज्दै…
calendar-account-try-again = फेरि प्रयास गर्नुहोस्
calendar-account-try-again-tooltip = यो खाताका पात्रोहरू अहिले फेरि जाँच गर्नुहोस्
calendar-account-fixing = काम हुँदैछ…
calendar-birthdays = जन्मदिन
calendar-birthday-of = { $name } को जन्मदिन
calendar-empty-title = अझै कुनै पात्रो छैन
calendar-empty-text = Katna ले तपाईंका Google र Microsoft खाताका पात्रो, र CalDAV दिने अन्य सर्भरका पात्रो सिङ्क भएपछि यहाँ देखाउँछ।
calendar-schedule-empty = अर्को दुई महिनामा केही तय गरिएको छैन।
calendar-search = कार्यक्रमहरू खोज्नुहोस्
calendar-search-past = विगतका कार्यक्रम
calendar-search-none = तपाईंको खोजसँग मिल्ने कुनै कार्यक्रम भेटिएन।
calendar-no-title = (शीर्षक छैन)
calendar-all-day = दिनभरि
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = थप { $count }
calendar-repeats = दोहोरिन्छ
calendar-join = सामेल हुनुहोस्
calendar-email-guests = पाहुनालाई इमेल गर्नुहोस्
calendar-running-late = ढिलो हुँदैछ
calendar-late-subject = ढिलो हुँदैछ: { $title }
calendar-late-body = माफ गर्नुहोस्, { $title } का लागि मलाई केही मिनेट ढिलो भइरहेको छ। म चाँडै आइपुग्नेछु।
calendar-guests =
    { $count ->
        [one] { $count } पाहुना
       *[other] { $count } पाहुना
    }
calendar-guest-answers = { $yes } हो, { $maybe } सायद, { $no } होइन, { $waiting } पर्खाइमा
calendar-organizer = आयोजक
calendar-optional = ऐच्छिक
calendar-open-web = ब्राउजरमा खोल्नुहोस्
calendar-open-contact = सम्पर्क खोल्नुहोस्
calendar-close = बन्द गर्नुहोस्

## Adding, changing and deleting events.

calendar-add-title = शीर्षक थप्नुहोस्
calendar-add-location = स्थान थप्नुहोस्
calendar-add-notes = विवरण थप्नुहोस्
calendar-add-guests = पाहुना थप्नुहोस्
calendar-remove-guest = हटाउनुहोस्
calendar-add-meet = Google Meet भिडियो कल थप्नुहोस्
calendar-add-teams = Teams बैठक थप्नुहोस्
calendar-has-call = भिडियो कल थपियो
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = दिनभरि
calendar-more-options = थप विकल्पहरू
calendar-save = सेभ गर्नुहोस्
calendar-saved = कार्यक्रम सेभ भयो
calendar-deleted = कार्यक्रम मेटियो
calendar-discard = परिवर्तन खारेज गर्नुहोस्
calendar-edit = कार्यक्रम सम्पादन गर्नुहोस्
calendar-delete = कार्यक्रम मेटाउनुहोस्
calendar-event-details = कार्यक्रमको विवरण
# Right-click menus on the calendar: on a free time or day, an event and
# a task.
calendar-menu-new-event = नयाँ कार्यक्रम
# Shows the day right-clicked on its own, in the Day view.
calendar-menu-open-day = दिन खोल्नुहोस्
calendar-menu-duplicate = प्रतिलिपि बनाउनुहोस्
calendar-menu-color = रङ
# The event takes its calendar's color.
calendar-menu-color-calendar = पात्रोको रङ
# A task's new due day, a week from today.
calendar-menu-in-a-week = एक हप्तापछि
# Event colors, by the names Google Calendar gives them.
calendar-color-tomato = टमाटर
calendar-color-flamingo = फ्लेमिङ्गो
calendar-color-tangerine = सुन्तला
calendar-color-banana = केरा
calendar-color-sage = सेज
calendar-color-basil = तुलसी
calendar-color-peacock = मयूर
calendar-color-blueberry = ब्लुबेरी
calendar-color-lavender = ल्याभेन्डर
calendar-color-grape = अङ्गुर
calendar-color-graphite = ग्राफाइट
calendar-kind-event = कार्यक्रम
calendar-kind-focus = फोकस समय
calendar-kind-out-of-office = कार्यालयबाहिर
calendar-kind-working-location = कार्यस्थल
calendar-working-home = घर
calendar-busy = व्यस्त
calendar-free = खाली
calendar-cancel = रद्द गर्नुहोस्
calendar-ok = ठिक छ
calendar-read-only = तपाईं यो पात्रोका कार्यक्रम परिवर्तन गर्न सक्नुहुन्न
calendar-none-editable = अहिलेसम्म तपाईंले कार्यक्रम थप्न सक्ने कुनै पात्रो छैन
calendar-no-such-time = तपाईंको समय क्षेत्रमा त्यो समय अस्तित्वमा छैन
calendar-end-before-start = कार्यक्रम सुरु हुनुअघि नै सकिन्छ
calendar-repeat-never = दोहोरिँदैन
calendar-repeat-daily = दैनिक
calendar-repeat-weekly = साप्ताहिक: { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] मासिक: पहिलो { $weekday }
        [2] मासिक: दोस्रो { $weekday }
        [3] मासिक: तेस्रो { $weekday }
        [4] मासिक: चौथो { $weekday }
       *[other] मासिक: अन्तिम { $weekday }
    }
calendar-repeat-yearly = वार्षिक: { $day }
calendar-repeat-weekdays = हरेक कार्यदिन (सोमबारदेखि शुक्रबारसम्म)
calendar-repeat-custom = अनुकूलन
calendar-reminder-none = कुनै सूचना छैन
calendar-reminder-at-start = सुरु हुँदा
calendar-reminder-minutes =
    { $count ->
        [one] { $count } मिनेट अघि
       *[other] { $count } मिनेट अघि
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } घण्टा अघि
       *[other] { $count } घण्टा अघि
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } दिन अघि
       *[other] { $count } दिन अघि
    }
calendar-scope-edit-title = दोहोरिने कार्यक्रम सम्पादन गर्नुहोस्
calendar-scope-delete-title = दोहोरिने कार्यक्रम मेटाउनुहोस्
calendar-scope-this = यो कार्यक्रम
calendar-scope-following = यो र यसपछिका कार्यक्रम
calendar-scope-all = सबै कार्यक्रम
calendar-scope-respond-title = दोहोरिने कार्यक्रमका लागि जवाफ
calendar-going = के तपाईं जाँदै हुनुहुन्छ?
calendar-answer-yes = हो
calendar-answer-no = होइन
calendar-answer-maybe = सायद
calendar-answered-yes = तपाईं जाँदै हुनुहुन्छ
calendar-answered-no = तपाईं जाँदै हुनुहुन्न
calendar-answered-maybe = तपाईं सायद जानुहुनेछ

## The card at the top of a mail with an invitation.

calendar-invite = निमन्त्रणा
calendar-invite-cancelled = कार्यक्रम रद्द गरियो
calendar-invite-reply = { $name } ले जवाफ दिनुभयो
calendar-invite-reply-yes = { $name } ले स्वीकार गर्नुभयो
calendar-invite-reply-no = { $name } ले अस्वीकार गर्नुभयो
calendar-invite-reply-maybe = { $name } सायद जानुहुनेछ
calendar-invite-organizer = { $name } द्वारा आयोजित
calendar-invite-open = पात्रोमा खोल्नुहोस्
calendar-invite-not-yet = अझै तपाईंको पात्रोमा छैन। सिंक भएपछि जवाफ दिन सकिन्छ।
calendar-invite-by-mail = तपाईंको पात्रोमा छैन: तपाईंको जवाफ मेलबाट आयोजकलाई जान्छ।
calendar-mail-yes = स्वीकार गरियो: { $title }
calendar-mail-yes-body = { $name } ले यो निमन्त्रणा स्वीकार गर्नुभएको छ।
calendar-mail-no = अस्वीकार गरियो: { $title }
calendar-mail-no-body = { $name } ले यो निमन्त्रणा अस्वीकार गर्नुभएको छ।
calendar-mail-maybe = अस्थायी रूपमा स्वीकार गरियो: { $title }
calendar-mail-maybe-body = { $name } ले यो निमन्त्रणा अस्थायी रूपमा स्वीकार गर्नुभएको छ।
calendar-invite-your-day = तपाईंको दिन
calendar-invite-clashes =
    { $count ->
        [one] { $count } कार्यक्रमसँग बाझिन्छ
       *[other] { $count } कार्यक्रमसँग बाझिन्छ
    }

## The day's agenda beside the mail.

agenda-show = दिनको कार्यसूची देखाउनुहोस्
agenda-hide = कार्यसूची लुकाउनुहोस्
agenda-today = आज, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = यो दिन केही तय गरिएको छैन।
