# Katna Mail, Sinhala (සිංහල).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = නව ඊමේල් { $count }
notify-and-more = සහ තවත් { $count }
notify-no-subject = (විෂයක් නැත)
notify-unknown-sender = නොදන්නා යවන්නා

## Reminders the user asked for (same buttons)

notify-snooze-back = කල් දැමීමෙන් ආපසු
notify-no-reply = තවම පිළිතුරක් නැත
notify-no-reply-to = “{ $subject }” ට කිසිවෙකු පිළිතුරු දී නැත.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } { $subject } විවෘත කළා
notify-tracking-clicked = { $who } { $subject } හි සබැඳියක් ක්ලික් කළා

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail යාවත්කාලීන කළ හැක
notify-update-ready-body = අනුවාදය { $version } බාගත කර ඇත. යාවත්කාලීන කිරීම එය ස්ථාපනය කර Katna Mail යළි ඇරඹේ.
notify-update = යාවත්කාලීන කරන්න

## Reminders of calendar events

notify-event-now = දැන්
notify-event-in-minutes = { $count ->
    [one] විනාඩි { $count } කින්
   *[other] විනාඩි { $count } කින්
}
notify-event-in-hours = { $count ->
    [one] පැය { $count } කින්
   *[other] පැය { $count } කින්
}
notify-event-in-days = { $count ->
    [1] හෙට
    [one] දින { $count } කින්
   *[other] දින { $count } කින්
}
notify-event-all-day = දවස පුරා
notify-event-join = සම්බන්ධ වන්න
notify-event-snooze = විනාඩි 5ක් කල් දමන්න
notify-task-done = සම්පූර්ණ ලෙස සලකුණු කරන්න

## The buttons of new-mail notifications and reminders

notify-open = විවෘත කරන්න
notify-peek = බලන්න
notify-reply = පිළිතුරු දෙන්න
notify-reply-placeholder = { $name } ට පිළිතුරු දෙන්න…
notify-send = යවන්න
notify-reply-all = සියල්ලන්ට පිළිතුරු දෙන්න
notify-mark-read = කියවූ ලෙස සලකුණු කරන්න
notify-mark-all-read = සියල්ල කියවූ ලෙස සලකුණු කරන්න
notify-archive = සංරක්ෂණය කරන්න

## After Archive on a notification: a short note in the same place

notify-archived = සංරක්ෂණය කළා
notify-archived-count = { $count ->
    [one] පණිවිඩ { $count }ක් එන ලිපිවලින් ඉවත් කළා
   *[other] පණිවිඩ { $count }ක් එන ලිපිවලින් ඉවත් කළා
}
notify-undo = අහෝසි කරන්න

## it waits for the undo time

notify-reply-sent = පිළිතුර { $name } වෙත යැවුණි
notify-open-in-katna = Katna හි විවෘත කරන්න
