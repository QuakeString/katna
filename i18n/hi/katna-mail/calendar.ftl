# Katna Mail, Hindi (हिन्दी): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = आज
calendar-today-tip = आज पर जाएं
calendar-view-day = दिन
calendar-view-week = सप्ताह
calendar-view-month = महीना
calendar-view-year = वर्ष
calendar-view-schedule = शेड्यूल
calendar-view-days =
    { $count ->
        [one] { $count } दिन
       *[other] { $count } दिन
    }
calendar-options = विकल्प
calendar-density = घनत्व
calendar-density-responsive = आपकी स्क्रीन के अनुसार
calendar-density-comfortable = आरामदायक
calendar-density-compact = कॉम्पैक्ट
calendar-custom-days = कस्टम दृश्य
calendar-second-zone = दूसरा समय क्षेत्र
calendar-zone-none = कोई नहीं
calendar-zone = { $zone } ({ $offset })
calendar-share-free = खाली समय साझा करें
calendar-free-subject = मेरे खाली समय
calendar-free-intro = ये कुछ समय हैं जब मैं खाली हूँ ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = अगले कुछ कार्यदिवसों में मेरे पास कोई खाली समय नहीं है।
calendar-previous-day = पिछला दिन
calendar-next-day = अगला दिन
calendar-previous-week = पिछला सप्ताह
calendar-next-week = अगला सप्ताह
calendar-previous-month = पिछला महीना
calendar-next-month = अगला महीना
calendar-previous-year = पिछला वर्ष
calendar-next-year = अगला वर्ष
calendar-previous-period = पहले
calendar-next-period = बाद में
calendar-title-months = { $first } – { $last }
calendar-loading = लोड हो रहा है…
calendar-read-failed = कैलेंडर पढ़ा नहीं जा सका: { $error }
calendar-sets = कैलेंडर सेट
calendar-set-add = दिखाए जा रहे कैलेंडर को सेट के रूप में सहेजें
calendar-set-name = सेट का नाम
calendar-set-remove = सेट हटाएं
calendar-local = यह कंप्यूटर
calendar-account-gone = हटाया गया खाता
calendar-account-sign-in = कैलेंडर दिखाने के लिए फिर से साइन इन करें
calendar-account-signed-in = { $address } में फिर से साइन इन हो गया। आपके कैलेंडर लाए जा रहे हैं…
calendar-account-sign-in-refused = { $provider } ने Katna को अंदर नहीं आने दिया। फिर से कोशिश करें, और अपने कैलेंडर तक पहुँच की अनुमति दें।
calendar-account-refused = सर्वर ने पासवर्ड स्वीकार नहीं किया। Yahoo, iCloud, Zoho और दूसरों को ऐप पासवर्ड चाहिए।
calendar-account-change-password = पासवर्ड बदलें
calendar-account-change-password-tooltip = नया पासवर्ड टाइप करें; Katna सर्वर से इसकी जाँच करता है
calendar-account-not-enabled = Katna के लिए कैलेंडर पहुँच अभी चालू नहीं है।
calendar-account-failed = कैलेंडर पढ़े नहीं जा सके।
calendar-account-error = कैलेंडर पढ़े नहीं जा सके: { $reason }
calendar-account-none = कोई कैलेंडर नहीं मिला
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = कोई कैलेंडर नहीं मिला: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } कैलेंडर सिर्फ़ उसी Katna को दिखाता है जो { $provider } से साइन इन हो।
calendar-account-sign-in-with = { $provider } से साइन इन करें
calendar-account-looking = कैलेंडर खोजे जा रहे हैं…
calendar-account-try-again = फिर से कोशिश करें
calendar-account-try-again-tooltip = इस खाते के कैलेंडर अभी फिर से जाँचें
calendar-account-fixing = काम जारी है…
calendar-birthdays = जन्मदिन
calendar-tasks = टास्क
calendar-birthday-of = { $name } का जन्मदिन
calendar-empty-title = अभी कोई कैलेंडर नहीं
calendar-empty-text = Katna यहां आपके Google और Microsoft खातों के कैलेंडर, और CalDAV देने वाले अन्य सर्वरों के कैलेंडर सिंक होने के बाद दिखाता है।
calendar-schedule-empty = अगले दो महीनों में कुछ भी नियोजित नहीं है।
calendar-search = इवेंट खोजें
calendar-search-past = बीते इवेंट
calendar-search-none = आपकी खोज से कोई इवेंट मेल नहीं खाता।
calendar-no-title = (कोई शीर्षक नहीं)
calendar-all-day = पूरे दिन
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } और
calendar-peek-day = { $weekday }, { $day }
calendar-repeats = दोहराया जाता है
calendar-join = शामिल हों
calendar-join-with = { $service } से शामिल हों
calendar-email-guests = मेहमानों को ईमेल करें
calendar-running-late = देर हो रही है
calendar-late-subject = देर हो रही है: { $title }
calendar-late-body = क्षमा करें, { $title } के लिए मुझे कुछ मिनट की देर हो रही है। जल्द ही पहुंचना हो जाएगा।
calendar-guests =
    { $count ->
        [one] { $count } मेहमान
       *[other] { $count } मेहमान
    }
calendar-guest-answers = { $yes } हां, { $maybe } शायद, { $no } नहीं, { $waiting } प्रतीक्षित
calendar-organizer = आयोजक
calendar-optional = वैकल्पिक
calendar-open-web = ब्राउज़र में खोलें
calendar-open-mail = मेल खोलें
calendar-open-contact = संपर्क खोलें
calendar-close = बंद करें

## Adding, changing and deleting events.

calendar-add-title = शीर्षक जोड़ें
calendar-add-location = जगह जोड़ें
calendar-add-notes = ब्यौरा जोड़ें
calendar-add-guests = मेहमान जोड़ें
calendar-remove-guest = हटाएं
calendar-add-meet = Google Meet वीडियो कॉल जोड़ें
calendar-add-teams = Teams मीटिंग जोड़ें
calendar-has-call = वीडियो कॉल जोड़ी गई
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = पूरे दिन
calendar-more-options = ज़्यादा विकल्प
calendar-save = सेव करें
calendar-saved = इवेंट सेव हो गया
calendar-deleted = इवेंट मिटा दिया गया
calendar-discard = बदलाव खारिज करें
calendar-edit = इवेंट में बदलाव करें
calendar-delete = इवेंट मिटाएं
calendar-event-details = इवेंट की जानकारी
# Right-click menus on the calendar: on a free time or day, an event and
# a task.
calendar-menu-new-event = नया इवेंट
calendar-event-window-title = नया इवेंट
# Shows the day right-clicked on its own, in the Day view.
calendar-menu-open-day = दिन खोलें
calendar-menu-duplicate = डुप्लीकेट करें
calendar-menu-color = रंग
# The event takes its calendar's color.
calendar-menu-color-calendar = कैलेंडर का रंग
# A task's new due day, a week from today.
calendar-menu-in-a-week = एक सप्ताह में
# Event colors, by the names Google Calendar gives them.
calendar-color-tomato = टमाटर
calendar-color-flamingo = फ़्लेमिंगो
calendar-color-tangerine = संतरा
calendar-color-banana = केला
calendar-color-sage = सेज
calendar-color-basil = तुलसी
calendar-color-peacock = मोर
calendar-color-blueberry = ब्लूबेरी
calendar-color-lavender = लैवेंडर
calendar-color-grape = अंगूर
calendar-color-graphite = ग्रेफ़ाइट
calendar-menu-only-this = सिर्फ़ यह दिखाएं
calendar-menu-rename = नाम बदलें
calendar-menu-remove = सूची से हटाएं
calendar-menu-delete = मिटाएं
calendar-menu-new-calendar = नया कैलेंडर
calendar-menu-show-all = सभी दिखाएं
calendar-menu-hide-all = सभी छिपाएं
calendar-menu-account-settings = खाते की सेटिंग
calendar-why-main = मुख्य कैलेंडर
calendar-why-last = यहां सिर्फ़ एक है
calendar-why-owner = सिर्फ़ मालिक
calendar-why-contacts = संपर्कों से
calendar-why-unreached = पहुंच नहीं हुई
calendar-name-placeholder = कैलेंडर का नाम
calendar-toast-added = “{ $name }” जोड़ा गया
calendar-toast-renamed = कैलेंडर का नाम बदला गया
calendar-toast-recolored = कैलेंडर का रंग बदला गया
calendar-toast-deleted = “{ $name }” मिटाया गया
calendar-toast-removed = “{ $name }” आपकी सूची से हटाया गया
calendar-edit-failed = कैलेंडर नहीं बदला गया: { $reason }
calendar-delete-title = “{ $name }” मिटाएं?
calendar-delete-confirm = मिटाएं
calendar-deleting = मिटाया जा रहा है…
calendar-delete-heading = मिटाया जाएगा:
calendar-delete-events = कैलेंडर और उसके सभी इवेंट
calendar-delete-shared = उन सभी के लिए जिनके साथ यह शेयर किया गया है
calendar-delete-server = यह सिर्फ़ Katna में नहीं, मेल सेवा पर { $account } से भी मिट जाता है।
calendar-delete-local = यह इस कंप्यूटर से मिट जाता है।
calendar-remove-title = “{ $name }” को अपनी सूची से हटाएं?
calendar-remove-confirm = हटाएं
calendar-removing = हटाया जा रहा है…
calendar-remove-heading = क्या बदलता है:
calendar-remove-events = आपको इसके इवेंट न यहां दिखेंगे, न आपके दूसरे ऐप्स में
calendar-remove-server = कैलेंडर उसके मालिक के पास रहता है, जो इसे फिर से आपके साथ शेयर कर सकते हैं।
calendar-kind-event = इवेंट
calendar-kind-task = टास्क
calendar-kind-focus = फ़ोकस टाइम
calendar-kind-out-of-office = ऑफ़िस से बाहर
calendar-kind-working-location = काम करने की जगह
calendar-task-added = टास्क जोड़ा गया
calendar-task-added-to = { $list } में टास्क जोड़ा गया
calendar-task-list-local = इस कंप्यूटर पर
calendar-working-home = घर
calendar-busy = व्यस्त
calendar-free = खाली
calendar-cancel = रद्द करें
calendar-ok = ठीक है
calendar-read-only = आप इस कैलेंडर में इवेंट नहीं बदल सकते
calendar-none-editable = अभी कोई ऐसा कैलेंडर नहीं है जिसमें आप इवेंट जोड़ सकें
calendar-no-such-time = आपके टाइम ज़ोन में यह समय मौजूद नहीं है
calendar-end-before-start = इवेंट शुरू होने से पहले ही खत्म हो जाता है
calendar-repeat-never = दोहराया नहीं जाता
calendar-repeat-daily = रोज़ाना
calendar-repeat-weekly = हर सप्ताह { $weekday } को
calendar-repeat-monthly =
    { $nth ->
        [1] हर महीने के पहले { $weekday } को
        [2] हर महीने के दूसरे { $weekday } को
        [3] हर महीने के तीसरे { $weekday } को
        [4] हर महीने के चौथे { $weekday } को
       *[other] हर महीने के आखिरी { $weekday } को
    }
calendar-repeat-yearly = हर साल { $day } को
calendar-repeat-weekdays = हर कार्यदिवस (सोमवार से शुक्रवार)
calendar-repeat-custom = कस्टम
calendar-reminder-none = कोई सूचना नहीं
calendar-reminder-at-start = शुरू होने पर
calendar-reminder-minutes =
    { $count ->
        [one] { $count } मिनट पहले
       *[other] { $count } मिनट पहले
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } घंटा पहले
       *[other] { $count } घंटे पहले
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } दिन पहले
       *[other] { $count } दिन पहले
    }
calendar-scope-edit-title = दोहराए जाने वाले इवेंट में बदलाव करें
calendar-scope-delete-title = दोहराए जाने वाले इवेंट को मिटाएं
calendar-scope-this = यह इवेंट
calendar-scope-following = यह और इसके बाद के इवेंट
calendar-scope-all = सभी इवेंट
calendar-scope-respond-title = दोहराए जाने वाले इवेंट के लिए जवाब
calendar-going = क्या आप जाएंगे?
calendar-answer-yes = हां
calendar-answer-no = नहीं
calendar-answer-maybe = शायद
calendar-answered-yes = आप जा रहे हैं
calendar-answered-no = आप नहीं जा रहे हैं
calendar-answered-maybe = आप शायद जाएंगे

## The card at the top of a mail with an invitation.

calendar-invite = आमंत्रण
calendar-invite-cancelled = इवेंट रद्द किया गया
calendar-invite-reply = { $name } ने जवाब दिया
calendar-invite-reply-yes = { $name } ने स्वीकार किया
calendar-invite-reply-no = { $name } ने अस्वीकार किया
calendar-invite-reply-maybe = { $name } का जवाब: शायद
calendar-invite-organizer = { $name } द्वारा आयोजित
calendar-invite-open = कैलेंडर में खोलें
calendar-invite-not-yet = अभी आपके कैलेंडर में नहीं है। सिंक होने के बाद जवाब दिया जा सकेगा।
calendar-invite-by-mail = आपके कैलेंडर में नहीं है: आपका जवाब मेल से आयोजक को भेजा जाएगा।
calendar-mail-yes = स्वीकार किया गया: { $title }
calendar-mail-yes-body = { $name } ने इस आमंत्रण को स्वीकार कर लिया है।
calendar-mail-no = अस्वीकार किया गया: { $title }
calendar-mail-no-body = { $name } ने इस आमंत्रण को अस्वीकार कर दिया है।
calendar-mail-maybe = अस्थायी रूप से स्वीकार किया गया: { $title }
calendar-mail-maybe-body = { $name } ने इस आमंत्रण को अस्थायी रूप से स्वीकार किया है।
calendar-invite-your-day = आपका दिन
calendar-invite-clashes =
    { $count ->
        [one] { $count } इवेंट से टकराव
       *[other] { $count } इवेंट से टकराव
    }

## The day's agenda beside the mail.

agenda-show = दिन का एजेंडा दिखाएं
agenda-hide = एजेंडा छिपाएं
agenda-today = आज, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = इस दिन कुछ भी नियोजित नहीं है।
