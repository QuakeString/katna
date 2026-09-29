# Katna Mail, Sinhala (සිංහල): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = අද
calendar-today-tip = අදට යන්න
calendar-view-day = දිනය
calendar-view-week = සතිය
calendar-view-month = මාසය
calendar-view-year = වසර
calendar-view-schedule = කාලසටහන
calendar-view-days =
    { $count ->
        [one] දින { $count }
       *[other] දින { $count }
    }
calendar-options = විකල්ප
calendar-density = ඝනත්වය
calendar-density-responsive = ඔබේ තිරයට ප්‍රතිචාර දක්වයි
calendar-density-comfortable = සුවපහසු
calendar-density-compact = සංයුක්ත
calendar-custom-days = අභිරුචි දසුන
calendar-second-zone = දෙවන වේලා කලාපය
calendar-zone-none = කිසිවක් නැත
calendar-zone = { $zone } ({ $offset })
calendar-share-free = නිදහස් වේලාවන් බෙදාගන්න
calendar-free-subject = මට නිදහස් වේලාවන්
calendar-free-intro = මා නිදහස් වේලාවන් කිහිපයක් මෙන්න ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = ඉදිරි වැඩ දින කිහිපයේ මට නිදහස් වේලාවක් නැත.
calendar-previous-day = පෙර දිනය
calendar-next-day = ඊළඟ දිනය
calendar-previous-week = පෙර සතිය
calendar-next-week = ඊළඟ සතිය
calendar-previous-month = පෙර මාසය
calendar-next-month = ඊළඟ මාසය
calendar-previous-year = පෙර වසර
calendar-next-year = ඊළඟ වසර
calendar-previous-period = කලින්
calendar-next-period = පසුව
calendar-title-months = { $first } – { $last }
calendar-loading = පූරණය වෙමින්…
calendar-read-failed = දින දර්ශනය කියවීමට නොහැකි විය: { $error }
calendar-sets = දින දර්ශන කට්ටල
calendar-set-add = පෙන්වන දින දර්ශන කට්ටලයක් ලෙස සුරකින්න
calendar-set-name = කට්ටලයේ නම
calendar-set-remove = කට්ටලය ඉවත් කරන්න
calendar-local = මෙම පරිගණකයේ
calendar-account-gone = ඉවත් කළ ගිණුම
calendar-account-sign-in = දින දර්ශන පෙන්වීමට නැවත පුරනය වන්න
calendar-account-signed-in = { $address } වෙත නැවත පුරනය විය. ඔබේ දින දර්ශන ලබා ගනිමින්…
calendar-account-sign-in-refused = { $provider } Katna ට ඇතුළු වීමට ඉඩ දුන්නේ නැත. නැවත උත්සාහ කර, ඔබේ දින දර්ශනවලට ප්‍රවේශය ඉඩ දෙන්න.
calendar-account-refused = සේවාදායකය මුරපදය පිළිගත්තේ නැත. Yahoo, iCloud, Zoho සහ වෙනත් ඒවාට යෙදුම් මුරපදයක් අවශ්‍යයි.
calendar-account-change-password = මුරපදය වෙනස් කරන්න
calendar-account-change-password-tooltip = සැකසීම් > ගිණුම් විවෘත කරන්න
calendar-account-not-enabled = Katna සඳහා දින දර්ශන ප්‍රවේශය තවම සක්‍රිය කර නැත.
calendar-account-failed = දින දර්ශන කියවිය නොහැකි විය.
calendar-account-error = දින දර්ශන කියවිය නොහැකි විය: { $reason }
calendar-account-none = දින දර්ශන හමු නොවීය
calendar-account-looking = දින දර්ශන සොයමින්…
calendar-account-try-again = නැවත උත්සාහ කරන්න
calendar-account-try-again-tooltip = මෙම ගිණුමේ දින දර්ශන දැන් නැවත පරීක්ෂා කරන්න
calendar-account-fixing = ඒ මත වැඩ කරමින්…
calendar-birthdays = උපන්දින
calendar-birthday-of = { $name }ගේ උපන්දිනය
calendar-empty-title = තවම දින දර්ශන නැත
calendar-empty-text = ඔබේ Google සහ Microsoft ගිණුම්වල දින දර්ශන, සහ CalDAV සපයන වෙනත් සේවාදායකවල දින දර්ශන සමමුහූර්ත වූ පසු Katna ඒවා මෙහි පෙන්වයි.
calendar-schedule-empty = ඉදිරි මාස දෙකට කිසිවක් සැලසුම් කර නැත.
calendar-search = සිදුවීම් සොයන්න
calendar-search-past = පසුගිය සිදුවීම්
calendar-search-none = ඔබේ සෙවුමට ගැළපෙන සිදුවීම් නැත.
calendar-no-title = (මාතෘකාවක් නැත)
calendar-all-day = දවස පුරා
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = තවත් { $count }
calendar-repeats = පුනරාවර්තනය වේ
calendar-join = සම්බන්ධ වන්න
calendar-email-guests = අමුත්තන්ට ඊමේල් කරන්න
calendar-running-late = ප්‍රමාද වෙමින්
calendar-late-subject = ප්‍රමාද වෙමින්: { $title }
calendar-late-body = සමාවන්න, { $title } සඳහා පැමිණීමට මට මිනිත්තු කිහිපයක් ප්‍රමාද වෙනවා. මම ඉක්මනින් එන්නම්.
calendar-guests =
    { $count ->
        [one] අමුත්තන් { $count }
       *[other] අමුත්තන් { $count }
    }
calendar-guest-answers = { $yes } ඔව්, { $maybe } සමහරවිට, { $no } නැත, { $waiting } බලා සිටී
calendar-organizer = සංවිධායක
calendar-optional = විකල්ප
calendar-open-web = බ්‍රව්සරයේ විවෘත කරන්න
calendar-open-contact = සම්බන්ධතාව විවෘත කරන්න
calendar-close = වසන්න

## Adding, changing and deleting events.

calendar-add-title = මාතෘකාව එක් කරන්න
calendar-add-location = ස්ථානය එක් කරන්න
calendar-add-notes = විස්තරය එක් කරන්න
calendar-add-guests = අමුත්තන් එක් කරන්න
calendar-remove-guest = ඉවත් කරන්න
calendar-add-meet = Google Meet වීඩියෝ ඇමතුමක් එක් කරන්න
calendar-add-teams = Teams රැස්වීමක් එක් කරන්න
calendar-has-call = වීඩියෝ ඇමතුම එක් කළා
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = දවස පුරා
calendar-more-options = තවත් විකල්ප
calendar-save = සුරකින්න
calendar-saved = සිදුවීම සුරැකිණි
calendar-deleted = සිදුවීම මකා දැමිණි
calendar-discard = වෙනස්කම් ඉවත ලන්න
calendar-edit = සිදුවීම සංස්කරණය කරන්න
calendar-delete = සිදුවීම මකන්න
calendar-event-details = සිදුවීමේ විස්තර
calendar-kind-event = සිදුවීම
calendar-kind-focus = අවධාන කාලය
calendar-kind-out-of-office = කාර්යාලයෙන් පිටත
calendar-kind-working-location = වැඩ කරන ස්ථානය
calendar-working-home = ගෙදර
calendar-busy = කාර්යබහුලයි
calendar-free = නිදහස්
calendar-cancel = අවලංගු කරන්න
calendar-ok = හරි
calendar-read-only = මෙම දින දර්ශනයේ සිදුවීම් ඔබට වෙනස් කළ නොහැක
calendar-none-editable = සිදුවීම් එක් කළ හැකි දින දර්ශනයක් තවම නැත
calendar-no-such-time = ඔබේ වේලා කලාපයේ ඒ වේලාව නැත
calendar-end-before-start = සිදුවීම ආරම්භ වීමට පෙර අවසන් වේ
calendar-repeat-never = නැවත සිදු නොවේ
calendar-repeat-daily = දිනපතා
calendar-repeat-weekly = සතිපතා: { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] මාසිකව: පළමු { $weekday }
        [2] මාසිකව: දෙවන { $weekday }
        [3] මාසිකව: තුන්වන { $weekday }
        [4] මාසිකව: හතරවන { $weekday }
       *[other] මාසිකව: අවසාන { $weekday }
    }
calendar-repeat-yearly = වාර්ෂිකව: { $day }
calendar-repeat-weekdays = සෑම වැඩ දිනකම (සඳුදා සිට සිකුරාදා දක්වා)
calendar-repeat-custom = අභිරුචි
calendar-reminder-none = දැනුම්දීමක් නැත
calendar-reminder-at-start = ආරම්භයේදී
calendar-reminder-minutes =
    { $count ->
        [one] මිනිත්තු { $count } කට පෙර
       *[other] මිනිත්තු { $count } කට පෙර
    }
calendar-reminder-hours =
    { $count ->
        [one] පැය { $count } කට පෙර
       *[other] පැය { $count } කට පෙර
    }
calendar-reminder-days =
    { $count ->
        [one] දින { $count } කට පෙර
       *[other] දින { $count } කට පෙර
    }
calendar-scope-edit-title = පුනරාවර්තන සිදුවීම සංස්කරණය කරන්න
calendar-scope-delete-title = පුනරාවර්තන සිදුවීම මකන්න
calendar-scope-this = මෙම සිදුවීම
calendar-scope-following = මෙම සහ ඉදිරි සිදුවීම්
calendar-scope-all = සියලු සිදුවීම්
calendar-scope-respond-title = පුනරාවර්තන සිදුවීමකට පිළිතුර
calendar-going = ඔබ යනවාද?
calendar-answer-yes = ඔව්
calendar-answer-no = නැත
calendar-answer-maybe = සමහරවිට
calendar-answered-yes = ඔබ යනවා
calendar-answered-no = ඔබ නොයයි
calendar-answered-maybe = ඔබ සමහරවිට යයි

## The card at the top of a mail with an invitation.

calendar-invite = ආරාධනාව
calendar-invite-cancelled = සිදුවීම අවලංගු කරන ලදී
calendar-invite-reply = { $name } පිළිතුරු දුන්නා
calendar-invite-reply-yes = { $name } පිළිගත්තා
calendar-invite-reply-no = { $name } ප්‍රතික්ෂේප කළා
calendar-invite-reply-maybe = { $name } සමහරවිට යයි
calendar-invite-organizer = සංවිධානය කළේ { $name }
calendar-invite-open = දින දර්ශනයේ විවෘත කරන්න
calendar-invite-not-yet = තවම ඔබේ දින දර්ශනයේ නැත. සමමුහුර්ත වූ පසු පිළිතුරු දිය හැක.
calendar-invite-by-mail = ඔබේ දින දර්ශනයේ නැත: ඔබේ පිළිතුර තැපෑලෙන් සංවිධායකයා වෙත යයි.
calendar-mail-yes = පිළිගත්තා: { $title }
calendar-mail-yes-body = { $name } මෙම ආරාධනාව පිළිගත්තා.
calendar-mail-no = ප්‍රතික්ෂේප කළා: { $title }
calendar-mail-no-body = { $name } මෙම ආරාධනාව ප්‍රතික්ෂේප කළා.
calendar-mail-maybe = තාවකාලිකව පිළිගත්තා: { $title }
calendar-mail-maybe-body = { $name } මෙම ආරාධනාව තාවකාලිකව පිළිගත්තා.
calendar-invite-your-day = ඔබේ දිනය
calendar-invite-clashes =
    { $count ->
        [one] සිදුවීම { $count } සමඟ ගැටේ
       *[other] සිදුවීම් { $count } සමඟ ගැටේ
    }

## The day's agenda beside the mail.

agenda-show = දිනයේ න්‍යාය පත්‍රය පෙන්වන්න
agenda-hide = න්‍යාය පත්‍රය සඟවන්න
agenda-today = අද, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = මෙම දිනයේ කිසිවක් සැලසුම් කර නැත.
