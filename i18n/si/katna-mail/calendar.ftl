# Katna Mail, Sinhala (සිංහල): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = අද
calendar-today-tip = අදට යන්න
calendar-view-day = දිනය
calendar-view-week = සතිය
calendar-view-month = මාසය
calendar-view-schedule = කාලසටහන
calendar-previous-day = පෙර දිනය
calendar-next-day = ඊළඟ දිනය
calendar-previous-week = පෙර සතිය
calendar-next-week = ඊළඟ සතිය
calendar-previous-month = පෙර මාසය
calendar-next-month = ඊළඟ මාසය
calendar-previous-period = කලින්
calendar-next-period = පසුව
calendar-title-months = { $first } – { $last }
calendar-loading = පූරණය වෙමින්…
calendar-read-failed = දින දර්ශනය කියවීමට නොහැකි විය: { $error }
calendar-local = මෙම පරිගණකයේ
calendar-account-gone = ඉවත් කළ ගිණුම
calendar-empty-title = තවම දින දර්ශන නැත
calendar-empty-text = ඔබේ Google සහ Microsoft ගිණුම්වල දින දර්ශන, සහ CalDAV සපයන වෙනත් සේවාදායකවල දින දර්ශන සමමුහූර්ත වූ පසු Katna ඒවා මෙහි පෙන්වයි.
calendar-schedule-empty = ඉදිරි මාස දෙකට කිසිවක් සැලසුම් කර නැත.
calendar-no-title = (මාතෘකාවක් නැත)
calendar-all-day = දවස පුරා
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = තවත් { $count }
calendar-repeats = පුනරාවර්තනය වේ
calendar-join = සම්බන්ධ වන්න
calendar-guests =
    { $count ->
        [one] අමුත්තන් { $count }
       *[other] අමුත්තන් { $count }
    }
calendar-guest-answers = { $yes } ඔව්, { $maybe } සමහරවිට, { $no } නැත, { $waiting } බලා සිටී
calendar-organizer = සංවිධායක
calendar-optional = විකල්ප
calendar-open-web = බ්‍රව්සරයේ විවෘත කරන්න
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

## The day's agenda beside the mail.

agenda-show = දිනයේ න්‍යාය පත්‍රය පෙන්වන්න
agenda-hide = න්‍යාය පත්‍රය සඟවන්න
agenda-today = අද, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = මෙම දිනයේ කිසිවක් සැලසුම් කර නැත.
