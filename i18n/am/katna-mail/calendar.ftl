# Katna Mail, Amharic (አማርኛ): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = ዛሬ
calendar-today-tip = ወደ ዛሬ ሂድ
calendar-view-day = ቀን
calendar-view-week = ሳምንት
calendar-view-month = ወር
calendar-view-schedule = መርሐግብር
calendar-previous-day = ያለፈው ቀን
calendar-next-day = ቀጣዩ ቀን
calendar-previous-week = ያለፈው ሳምንት
calendar-next-week = ቀጣዩ ሳምንት
calendar-previous-month = ያለፈው ወር
calendar-next-month = ቀጣዩ ወር
calendar-previous-period = ቀደም ብሎ
calendar-next-period = በኋላ
calendar-title-months = { $first } – { $last }
calendar-loading = በመጫን ላይ…
calendar-read-failed = ቀን መቁጠሪያውን ማንበብ አልተቻለም፦ { $error }
calendar-local = ይህ ኮምፒውተር
calendar-account-gone = የተወገደ መለያ
calendar-empty-title = እስካሁን ምንም ቀን መቁጠሪያ የለም
calendar-empty-text = የGoogle እና የMicrosoft መለያዎችዎ ቀን መቁጠሪያዎች ከተመሳሰሉ በኋላ እዚህ ይታያሉ፤ CalDAV የሚደግፉ ሌሎች አገልጋዮችም እንዲሁ።
calendar-schedule-empty = በሚቀጥሉት ሁለት ወራት ውስጥ የታቀደ ምንም ነገር የለም።
calendar-no-title = (ርዕስ የለም)
calendar-all-day = ቀኑን ሙሉ
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }፣ { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } ተጨማሪ
calendar-repeats = ይደገማል
calendar-join = ተቀላቀል
calendar-guests =
    { $count ->
        [one] { $count } እንግዳ
       *[other] { $count } እንግዶች
    }
calendar-guest-answers = { $yes } አዎ፣ { $maybe } ምናልባት፣ { $no } አይ፣ { $waiting } በመጠባበቅ ላይ
calendar-organizer = አዘጋጅ
calendar-optional = አማራጭ
calendar-open-web = በአሳሽ ውስጥ ክፈት
calendar-close = ዝጋ

## Adding, changing and deleting events.

calendar-add-title = ርዕስ ያክሉ
calendar-add-location = ቦታ ያክሉ
calendar-add-notes = መግለጫ ያክሉ
calendar-add-guests = እንግዶችን ያክሉ
calendar-remove-guest = አስወግድ
calendar-add-meet = የGoogle Meet የቪዲዮ ኮንፈረንስ ያክሉ
calendar-add-teams = የTeams ስብሰባ ያክሉ
calendar-has-call = የቪዲዮ ጥሪ ተጨምሯል
calendar-weekday-day = { $weekday }፣ { $day }
calendar-all-day-box = ቀኑን ሙሉ
calendar-more-options = ተጨማሪ አማራጮች
calendar-save = አስቀምጥ
calendar-saved = ክስተቱ ተቀምጧል
calendar-deleted = ክስተቱ ተሰርዟል
calendar-discard = ለውጦችን አስወግድ
calendar-edit = ክስተት አርትዕ
calendar-delete = ክስተት ሰርዝ
calendar-event-details = የክስተት ዝርዝሮች
calendar-busy = ተይዟል
calendar-free = ነፃ
calendar-cancel = ይቅር
calendar-ok = እሺ
calendar-read-only = በዚህ የቀን መቁጠሪያ ውስጥ ያሉ ክስተቶችን መቀየር አይችሉም
calendar-none-editable = ክስተት ማከል የሚችሉበት የቀን መቁጠሪያ እስካሁን የለም
calendar-no-such-time = ያ ሰዓት በሰዓት ሰቅዎ ውስጥ የለም
calendar-end-before-start = ክስተቱ ከመጀመሩ በፊት ያበቃል
calendar-repeat-never = አይደገምም
calendar-repeat-daily = በየቀኑ
calendar-repeat-weekly = በየሳምንቱ { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] በየወሩ የመጀመሪያው { $weekday }
        [2] በየወሩ ሁለተኛው { $weekday }
        [3] በየወሩ ሦስተኛው { $weekday }
        [4] በየወሩ አራተኛው { $weekday }
       *[other] በየወሩ የመጨረሻው { $weekday }
    }
calendar-repeat-yearly = በየዓመቱ { $day }
calendar-repeat-weekdays = በየሳምንቱ የሥራ ቀናት (ሰኞ እስከ አርብ)
calendar-repeat-custom = ብጁ
calendar-reminder-none = ማሳወቂያ የለም
calendar-reminder-at-start = ሲጀምር
calendar-reminder-minutes =
    { $count ->
        [one] ከ{ $count } ደቂቃ በፊት
       *[other] ከ{ $count } ደቂቃ በፊት
    }
calendar-reminder-hours =
    { $count ->
        [one] ከ{ $count } ሰዓት በፊት
       *[other] ከ{ $count } ሰዓት በፊት
    }
calendar-reminder-days =
    { $count ->
        [one] ከ{ $count } ቀን በፊት
       *[other] ከ{ $count } ቀን በፊት
    }
calendar-scope-edit-title = ተደጋጋሚ ክስተት አርትዕ
calendar-scope-delete-title = ተደጋጋሚ ክስተት ሰርዝ
calendar-scope-this = ይህ ክስተት
calendar-scope-following = ይህ እና ተከታይ ክስተቶች
calendar-scope-all = ሁሉም ክስተቶች
calendar-scope-respond-title = ለተደጋጋሚ ክስተት መልስ
calendar-going = ይሄዳሉ?
calendar-answer-yes = አዎ
calendar-answer-no = አይ
calendar-answer-maybe = ምናልባት
calendar-answered-yes = ይሄዳሉ
calendar-answered-no = አይሄዱም
calendar-answered-maybe = ምናልባት ይሄዳሉ

## The day's agenda beside the mail.

agenda-show = የቀኑን አጀንዳ አሳይ
agenda-hide = አጀንዳውን ደብቅ
agenda-today = ዛሬ፣ { $date }
agenda-day = { $weekday }፣ { $date }
agenda-empty = በዚህ ቀን የታቀደ ምንም ነገር የለም።
