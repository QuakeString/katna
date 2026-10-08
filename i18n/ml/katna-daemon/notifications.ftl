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
notify-follow-up-sent = തുടർസന്ദേശം അയച്ചു
notify-follow-up-sent-to = “{ $subject }” എന്നതിന് ആരും മറുപടി നൽകിയില്ല, അതിനാൽ Katna തുടർസന്ദേശം അയച്ചു.
notify-follow-up-waiting = തുടർസന്ദേശം അയച്ചില്ല
notify-follow-up-waiting-to = ഈ കമ്പ്യൂട്ടർ ഓഫായിരുന്നപ്പോഴാണ് സമയമായത്. “{ $subject }” നിങ്ങളുടെ ഇൻബോക്‌സിൽ തിരികെയെത്തി.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } { $subject } തുറന്നു
notify-tracking-clicked = { $who } { $subject }-ലെ ഒരു ലിങ്കിൽ ക്ലിക്ക് ചെയ്തു

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail അപ്‌ഡേറ്റ് ചെയ്യാം
notify-update-ready-body = പതിപ്പ് { $version } ഡൗൺലോഡ് ചെയ്തു. അപ്‌ഡേറ്റ് അത് ഇൻസ്റ്റാൾ ചെയ്ത് Katna Mail റീസ്റ്റാർട്ട് ചെയ്യും.
notify-update = അപ്‌ഡേറ്റ്

## Something needs the user, shown once per problem

notify-signed-out = വീണ്ടും സൈൻ ഇൻ ചെയ്യുക
notify-signed-out-body = { $provider } { $address }-ൽ നിന്ന് Katna-യെ സൈൻ ഔട്ട് ചെയ്‌തു. മെയിൽ സമന്വയം നിലച്ചു.
notify-sign-in = സൈൻ ഇൻ ചെയ്യുക
notify-password-refused = പാസ്‌വേഡ് നിരസിച്ചു
notify-password-refused-body = മെയിൽ സെർവർ { $address }-ന്റെ പാസ്‌വേഡ് നിരസിച്ചു. അത് മാറിയിരിക്കാം.
notify-new-password = പുതിയ പാസ്‌വേഡ്
notify-not-sent = “{ $subject }” അയച്ചില്ല
notify-not-sent-no-subject = ഒരു സന്ദേശം അയച്ചില്ല
notify-not-sent-body = അത് ഔട്ട്‌ബോക്‌സിലുണ്ട്, കാരണം അവിടെ കാണാം.
notify-open-outbox = ഔട്ട്‌ബോക്‌സ് തുറക്കുക

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
notify-reply-quote-header = { $date }-ന്, { $from } എഴുതി:
notify-reply-quote-header-no-date = { $from } എഴുതി:
notify-reply-all = എല്ലാവർക്കും മറുപടി നൽകുക
notify-mark-read = വായിച്ചതായി അടയാളപ്പെടുത്തുക
notify-mark-all-read = എല്ലാം വായിച്ചതായി അടയാളപ്പെടുത്തുക
notify-archive = ആർക്കൈവ് ചെയ്യുക
notify-snooze-hour = 1 മണിക്കൂർ സ്‌നൂസ് ചെയ്യുക
notify-snooze-tomorrow = നാളെ
notify-copy-code = { $code } പകർത്തുക
notify-link-verify = { $domain }-ൽ സ്ഥിരീകരിക്കുക
notify-link-confirm = { $domain }-ൽ ഉറപ്പാക്കുക
notify-link-activate = { $domain }-ൽ സജീവമാക്കുക

## After Archive on a notification: a short note in the same place

notify-archived = ആർക്കൈവ് ചെയ്‌തു
notify-archived-count = { $count ->
    [one] { $count } സന്ദേശം ഇൻബോക്‌സിൽ നിന്ന് നീക്കി
   *[other] { $count } സന്ദേശങ്ങൾ ഇൻബോക്‌സിൽ നിന്ന് നീക്കി
}
notify-undo = പഴയപടിയാക്കുക

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = കോഡ് പകർത്തി
notify-code-not-copied = കോഡ് പകർത്താനായില്ല

## it waits for the undo time

notify-reply-sent = { $name }-ന് മറുപടി അയച്ചു
notify-open-in-katna = Katna-യിൽ തുറക്കുക
