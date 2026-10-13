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
notify-follow-up-sent = පසු විපරම යැව්වා
notify-follow-up-sent-to = “{ $subject }” ට කිසිවෙකු පිළිතුරු නොදුන් නිසා, Katna පසු විපරම් කළා.
notify-follow-up-waiting = පසු විපරම යැවුණේ නැත
notify-follow-up-waiting-to = මෙම පරිගණකය අක්‍රියව තිබියදී එහි වේලාව පැමිණියා. “{ $subject }” නැවත ඔබේ එන ලිපිවල ඇත.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } { $subject } විවෘත කළා
notify-tracking-clicked = { $who } { $subject } හි සබැඳියක් ක්ලික් කළා

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail යාවත්කාලීන කළ හැක
notify-update-ready-body = අනුවාදය { $version } බාගත කර ඇත. යාවත්කාලීන කිරීම එය ස්ථාපනය කර Katna Mail යළි ඇරඹේ.
notify-update = යාවත්කාලීන කරන්න

## Something needs the user, shown once per problem

notify-signed-out = නැවත පුරනය වන්න
notify-signed-out-body = { $provider } විසින් Katna, { $address } වෙතින් ඉවත් කළා. තැපැල් සමමුහුර්ත වීම නැවතුණා.
notify-sign-in = පුරනය වන්න
notify-password-refused = මුරපදය ප්‍රතික්ෂේප විය
notify-password-refused-body = තැපැල් සේවාදායකය { $address } සඳහා මුරපදය ප්‍රතික්ෂේප කළා. එය වෙනස් වී ඇති විය හැක.
notify-new-password = නව මුරපදය
notify-not-sent = “{ $subject }” යැවුණේ නැත
notify-not-sent-no-subject = පණිවිඩයක් යැවුණේ නැත
notify-not-sent-body = එය පිටතට යන ලිපි තුළ ඇත, එහි හේතුව සඳහන් වේ.
notify-open-outbox = පිටතට යන ලිපි විවෘත කරන්න

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
notify-reply-quote-header = { $date } දින, { $from } ලිවීය:
notify-reply-quote-header-no-date = { $from } ලිවීය:
notify-reply-all = සියල්ලන්ට පිළිතුරු දෙන්න
notify-mark-read = කියවූ ලෙස සලකුණු කරන්න
notify-mark-all-read = සියල්ල කියවූ ලෙස සලකුණු කරන්න
notify-archive = සංරක්ෂණය කරන්න
notify-snooze-hour = පැය 1ක් කල් දමන්න
notify-snooze-tomorrow = හෙට
notify-copy-code = { $code } පිටපත් කරන්න
notify-link-verify = { $domain } මත තහවුරු කරන්න
notify-link-confirm = { $domain } මත සනාථ කරන්න
notify-link-activate = { $domain } මත සක්‍රිය කරන්න

## After Archive on a notification: a short note in the same place

notify-archived = සංරක්ෂණය කළා
notify-archived-count = { $count ->
    [one] පණිවිඩ { $count }ක් එන ලිපිවලින් ඉවත් කළා
   *[other] පණිවිඩ { $count }ක් එන ලිපිවලින් ඉවත් කළා
}
notify-undo = අහෝසි කරන්න

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = කේතය පිටපත් කළා
notify-code-not-copied = කේතය පිටපත් කළ නොහැකි විය

## it waits for the undo time

notify-reply-sent = පිළිතුර { $name } වෙත යැවුණි
notify-open-in-katna = Katna හි විවෘත කරන්න
