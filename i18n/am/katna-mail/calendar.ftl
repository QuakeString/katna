# Katna Mail, Amharic (አማርኛ): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = ዛሬ
calendar-today-tip = ወደ ዛሬ ሂድ
calendar-view-day = ቀን
calendar-view-week = ሳምንት
calendar-view-month = ወር
calendar-view-year = ዓመት
calendar-view-schedule = መርሐግብር
calendar-view-days =
    { $count ->
        [one] { $count } ቀን
       *[other] { $count } ቀናት
    }
calendar-options = አማራጮች
calendar-density = ጥግግት
calendar-density-responsive = ለማያ ገጽዎ ምላሽ ሰጪ
calendar-density-comfortable = ምቹ
calendar-density-compact = የታመቀ
calendar-custom-days = ብጁ እይታ
calendar-second-zone = ሁለተኛ የሰዓት ሰቅ
calendar-zone-none = ምንም
calendar-zone = { $zone } ({ $offset })
calendar-share-free = ክፍት ጊዜዎችን አጋራ
calendar-free-subject = ክፍት የሆንኩባቸው ጊዜዎች
calendar-free-intro = ክፍት የምሆንባቸው አንዳንድ ጊዜዎች እነሆ ({ $zone })፦
calendar-free-day = { $weekday } { $date }፦ { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = በሚቀጥሉት ጥቂት የሥራ ቀናት ውስጥ ክፍት ጊዜ የለኝም።
calendar-previous-day = ያለፈው ቀን
calendar-next-day = ቀጣዩ ቀን
calendar-previous-week = ያለፈው ሳምንት
calendar-next-week = ቀጣዩ ሳምንት
calendar-previous-month = ያለፈው ወር
calendar-next-month = ቀጣዩ ወር
calendar-previous-year = ያለፈው ዓመት
calendar-next-year = ቀጣዩ ዓመት
calendar-previous-period = ቀደም ብሎ
calendar-next-period = በኋላ
calendar-title-months = { $first } – { $last }
calendar-loading = በመጫን ላይ…
calendar-read-failed = ቀን መቁጠሪያውን ማንበብ አልተቻለም፦ { $error }
calendar-sets = የቀን መቁጠሪያ ስብስቦች
calendar-set-add = የሚታዩትን ቀን መቁጠሪያዎች እንደ ስብስብ አስቀምጥ
calendar-set-name = የስብስቡ ስም
calendar-set-remove = ስብስብ አስወግድ
calendar-local = ይህ ኮምፒውተር
calendar-account-gone = የተወገደ መለያ
calendar-birthdays = ልደቶች
calendar-birthday-of = የ{ $name } ልደት
calendar-empty-title = እስካሁን ምንም ቀን መቁጠሪያ የለም
calendar-empty-text = የGoogle እና የMicrosoft መለያዎችዎ ቀን መቁጠሪያዎች ከተመሳሰሉ በኋላ እዚህ ይታያሉ፤ CalDAV የሚደግፉ ሌሎች አገልጋዮችም እንዲሁ።
calendar-schedule-empty = በሚቀጥሉት ሁለት ወራት ውስጥ የታቀደ ምንም ነገር የለም።
calendar-search = ክስተቶችን ፈልግ
calendar-search-past = ያለፉ ክስተቶች
calendar-search-none = ከፍለጋዎ ጋር የሚዛመድ ክስተት የለም።
calendar-no-title = (ርዕስ የለም)
calendar-all-day = ቀኑን ሙሉ
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }፣ { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } ተጨማሪ
calendar-repeats = ይደገማል
calendar-join = ተቀላቀል
calendar-email-guests = ለእንግዶች ደብዳቤ ላክ
calendar-running-late = እዘገያለሁ
calendar-late-subject = እዘገያለሁ፦ { $title }
calendar-late-body = ይቅርታ፣ ለ{ $title } ጥቂት ደቂቃዎች እዘገያለሁ። በቅርቡ እደርሳለሁ።
calendar-guests =
    { $count ->
        [one] { $count } እንግዳ
       *[other] { $count } እንግዶች
    }
calendar-guest-answers = { $yes } አዎ፣ { $maybe } ምናልባት፣ { $no } አይ፣ { $waiting } በመጠባበቅ ላይ
calendar-organizer = አዘጋጅ
calendar-optional = አማራጭ
calendar-open-web = በአሳሽ ውስጥ ክፈት
calendar-open-contact = እውቂያ ክፈት
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
calendar-kind-event = ክስተት
calendar-kind-focus = የትኩረት ጊዜ
calendar-kind-out-of-office = ከቢሮ ውጭ
calendar-kind-working-location = የሥራ ቦታ
calendar-working-home = ቤት
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

## The card at the top of a mail with an invitation.

calendar-invite = ግብዣ
calendar-invite-cancelled = ክስተቱ ተሰርዟል
calendar-invite-reply = { $name }፦ መልስ ሰጥተዋል
calendar-invite-reply-yes = { $name }፦ ተቀብለዋል
calendar-invite-reply-no = { $name }፦ ውድቅ አድርገዋል
calendar-invite-reply-maybe = { $name }፦ ምናልባት ይሄዳሉ
calendar-invite-organizer = አዘጋጅ፦ { $name }
calendar-invite-open = በቀን መቁጠሪያ ውስጥ ክፈት
calendar-invite-not-yet = ገና በቀን መቁጠሪያዎ ውስጥ የለም። ከተመሳሰለ በኋላ መልስ መስጠት ይቻላል።
calendar-invite-by-mail = በቀን መቁጠሪያዎ ውስጥ የለም፦ መልስዎ በኢሜይል ወደ አዘጋጁ ይላካል።
calendar-mail-yes = ተቀብለዋል፦ { $title }
calendar-mail-yes-body = { $name } ይህን ግብዣ ተቀብለዋል።
calendar-mail-no = ውድቅ አድርገዋል፦ { $title }
calendar-mail-no-body = { $name } ይህን ግብዣ ውድቅ አድርገዋል።
calendar-mail-maybe = ምናልባት፦ { $title }
calendar-mail-maybe-body = { $name } ይህን ግብዣ በጊዜያዊነት ተቀብለዋል።
calendar-invite-your-day = የእርስዎ ቀን
calendar-invite-clashes =
    { $count ->
        [one] ከ{ $count } ክስተት ጋር ይጋጫል
       *[other] ከ{ $count } ክስተቶች ጋር ይጋጫል
    }

## The day's agenda beside the mail.

agenda-show = የቀኑን አጀንዳ አሳይ
agenda-hide = አጀንዳውን ደብቅ
agenda-today = ዛሬ፣ { $date }
agenda-day = { $weekday }፣ { $date }
agenda-empty = በዚህ ቀን የታቀደ ምንም ነገር የለም።
