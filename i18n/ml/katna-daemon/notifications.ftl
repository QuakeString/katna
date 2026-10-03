# Katna Mail, Malayalam (മലയാളം).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = { $count ->
    [one] { $count } പുതിയ ഇമെയിൽ
   *[other] { $count } പുതിയ ഇമെയിലുകൾ
}
notify-and-more = കൂടാതെ { $count } എണ്ണം കൂടി
notify-no-subject = (വിഷയമില്ല)
notify-unknown-sender = അജ്ഞാത അയച്ചയാൾ

## Reminders the user asked for (same buttons)

notify-snooze-back = സ്‌നൂസിൽ നിന്ന് തിരികെയെത്തി
notify-no-reply = ഇതുവരെ മറുപടിയില്ല
notify-no-reply-to = “{ $subject }” എന്നതിന് ആരും മറുപടി നൽകിയിട്ടില്ല.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } { $subject } തുറന്നു
notify-tracking-clicked = { $who } { $subject }-ലെ ഒരു ലിങ്കിൽ ക്ലിക്ക് ചെയ്തു

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail അപ്‌ഡേറ്റ് ചെയ്യാം
notify-update-ready-body = പതിപ്പ് { $version } ഡൗൺലോഡ് ചെയ്തു. അപ്‌ഡേറ്റ് അത് ഇൻസ്റ്റാൾ ചെയ്ത് Katna Mail റീസ്റ്റാർട്ട് ചെയ്യും.
notify-update = അപ്‌ഡേറ്റ്

## Reminders of calendar events

notify-event-now = ഇപ്പോൾ
notify-event-in-minutes = { $count ->
    [one] { $count } മിനിറ്റിനുള്ളിൽ
   *[other] { $count } മിനിറ്റിനുള്ളിൽ
}
notify-event-in-hours = { $count ->
    [one] { $count } മണിക്കൂറിനുള്ളിൽ
   *[other] { $count } മണിക്കൂറിനുള്ളിൽ
}
notify-event-in-days = { $count ->
    [1] നാളെ
    [one] { $count } ദിവസത്തിനുള്ളിൽ
   *[other] { $count } ദിവസത്തിനുള്ളിൽ
}
notify-event-all-day = ദിവസം മുഴുവൻ
notify-event-join = ചേരുക
notify-event-snooze = 5 മിനിറ്റ് സ്‌നൂസ് ചെയ്യുക
notify-task-done = പൂർത്തിയായതായി അടയാളപ്പെടുത്തുക

## The buttons of new-mail notifications and reminders

notify-open = തുറക്കുക
notify-peek = എത്തിനോക്കുക
notify-reply = മറുപടി നൽകുക
notify-reply-placeholder = { $name }-ന് മറുപടി നൽകുക…
notify-send = അയയ്ക്കുക
notify-reply-all = എല്ലാവർക്കും മറുപടി നൽകുക
notify-mark-read = വായിച്ചതായി അടയാളപ്പെടുത്തുക
notify-mark-all-read = എല്ലാം വായിച്ചതായി അടയാളപ്പെടുത്തുക
notify-archive = ആർക്കൈവ് ചെയ്യുക

## After Archive on a notification: a short note in the same place

notify-archived = ആർക്കൈവ് ചെയ്‌തു
notify-archived-count = { $count ->
    [one] { $count } സന്ദേശം ഇൻബോക്‌സിൽ നിന്ന് നീക്കി
   *[other] { $count } സന്ദേശങ്ങൾ ഇൻബോക്‌സിൽ നിന്ന് നീക്കി
}
notify-undo = പഴയപടിയാക്കുക

## it waits for the undo time

notify-reply-sent = { $name }-ന് മറുപടി അയച്ചു
notify-open-in-katna = Katna-യിൽ തുറക്കുക
