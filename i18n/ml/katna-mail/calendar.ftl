# Katna Mail, Malayalam (മലയാളം): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = ഇന്ന്
calendar-today-tip = ഇന്നത്തേക്ക് പോകുക
calendar-view-day = ദിവസം
calendar-view-week = ആഴ്ച
calendar-view-month = മാസം
calendar-view-year = വർഷം
calendar-view-schedule = ഷെഡ്യൂൾ
calendar-view-days =
    { $count ->
        [one] { $count } ദിവസം
       *[other] { $count } ദിവസം
    }
calendar-options = ഓപ്ഷനുകൾ
calendar-density = സാന്ദ്രത
calendar-density-responsive = നിങ്ങളുടെ സ്ക്രീനിന് അനുസരിച്ച്
calendar-density-comfortable = സൗകര്യപ്രദം
calendar-density-compact = കോംപാക്റ്റ്
calendar-custom-days = ഇഷ്‌ടാനുസൃത വ്യൂ
calendar-second-zone = രണ്ടാമത്തെ സമയമേഖല
calendar-zone-none = ഒന്നുമില്ല
calendar-zone = { $zone } ({ $offset })
calendar-share-free = ഒഴിവുസമയങ്ങൾ പങ്കിടുക
calendar-free-subject = എനിക്ക് ഒഴിവുള്ള സമയങ്ങൾ
calendar-free-intro = എനിക്ക് ഒഴിവുള്ള ചില സമയങ്ങൾ ഇതാ ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = അടുത്ത കുറച്ച് പ്രവൃത്തി ദിവസങ്ങളിൽ എനിക്ക് ഒഴിവുസമയമില്ല.
calendar-previous-day = മുമ്പത്തെ ദിവസം
calendar-next-day = അടുത്ത ദിവസം
calendar-previous-week = മുമ്പത്തെ ആഴ്ച
calendar-next-week = അടുത്ത ആഴ്ച
calendar-previous-month = മുമ്പത്തെ മാസം
calendar-next-month = അടുത്ത മാസം
calendar-previous-year = മുമ്പത്തെ വർഷം
calendar-next-year = അടുത്ത വർഷം
calendar-previous-period = മുമ്പ്
calendar-next-period = പിന്നീട്
calendar-title-months = { $first } – { $last }
calendar-loading = ലോഡ് ചെയ്യുന്നു…
calendar-read-failed = കലണ്ടർ വായിക്കാൻ കഴിഞ്ഞില്ല: { $error }
calendar-sets = കലണ്ടർ സെറ്റുകൾ
calendar-set-add = കാണിക്കുന്ന കലണ്ടറുകൾ ഒരു സെറ്റായി സംരക്ഷിക്കുക
calendar-set-name = സെറ്റിന്റെ പേര്
calendar-set-remove = സെറ്റ് നീക്കം ചെയ്യുക
calendar-local = ഈ കമ്പ്യൂട്ടറിൽ
calendar-account-gone = നീക്കം ചെയ്ത അക്കൗണ്ട്
calendar-account-sign-in = കലണ്ടറുകൾ കാണിക്കാൻ വീണ്ടും സൈൻ ഇൻ ചെയ്യുക
calendar-account-signed-in = { $address }-ൽ വീണ്ടും സൈൻ ഇൻ ചെയ്തു. നിങ്ങളുടെ കലണ്ടറുകൾ ലഭ്യമാക്കുന്നു…
calendar-account-sign-in-refused = { $provider } Katna-യെ അകത്ത് കയറ്റിയില്ല. വീണ്ടും ശ്രമിക്കുക, നിങ്ങളുടെ കലണ്ടറുകളിലേക്ക് ആക്‌സസ് അനുവദിക്കുക.
calendar-account-refused = സെർവർ പാസ്‌വേഡ് സ്വീകരിച്ചില്ല. Yahoo, iCloud, Zoho എന്നിവയ്ക്കും മറ്റുള്ളവയ്ക്കും ഒരു ആപ്പ് പാസ്‌വേഡ് വേണം.
calendar-account-change-password = പാസ്‌വേഡ് മാറ്റുക
calendar-account-change-password-tooltip = ക്രമീകരണം > അക്കൗണ്ടുകൾ തുറക്കുക
calendar-account-not-enabled = Katna-യ്ക്കുള്ള കലണ്ടർ ആക്‌സസ് ഇതുവരെ ഓണാക്കിയിട്ടില്ല.
calendar-account-failed = കലണ്ടറുകൾ വായിക്കാൻ കഴിഞ്ഞില്ല.
calendar-account-error = കലണ്ടറുകൾ വായിക്കാൻ കഴിഞ്ഞില്ല: { $reason }
calendar-account-none = കലണ്ടറുകളൊന്നും കണ്ടെത്തിയില്ല
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = കലണ്ടറുകളൊന്നും കണ്ടെത്തിയില്ല: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } ഉപയോഗിച്ച് സൈൻ ഇൻ ചെയ്ത Katna-യ്ക്ക് മാത്രമേ { $provider } കലണ്ടറുകൾ കാണിക്കൂ.
calendar-account-sign-in-with = { $provider } ഉപയോഗിച്ച് സൈൻ ഇൻ ചെയ്യുക
calendar-account-looking = കലണ്ടറുകൾ തിരയുന്നു…
calendar-account-try-again = വീണ്ടും ശ്രമിക്കുക
calendar-account-try-again-tooltip = ഈ അക്കൗണ്ടിന്റെ കലണ്ടറുകൾ ഇപ്പോൾ വീണ്ടും പരിശോധിക്കുക
calendar-account-fixing = പരിഹരിക്കുന്നു…
calendar-birthdays = ജന്മദിനങ്ങൾ
calendar-birthday-of = { $name }-ന്റെ ജന്മദിനം
calendar-empty-title = ഇതുവരെ കലണ്ടറുകളില്ല
calendar-empty-text = നിങ്ങളുടെ Google, Microsoft അക്കൗണ്ടുകളുടെ കലണ്ടറുകളും CalDAV നൽകുന്ന മറ്റ് സെർവറുകളുടെ കലണ്ടറുകളും സമന്വയിപ്പിച്ചാൽ Katna അവ ഇവിടെ കാണിക്കും.
calendar-schedule-empty = അടുത്ത രണ്ട് മാസത്തേക്ക് ഒന്നും ആസൂത്രണം ചെയ്തിട്ടില്ല.
calendar-search = ഇവന്റുകൾ തിരയുക
calendar-search-past = കഴിഞ്ഞ ഇവന്റുകൾ
calendar-search-none = നിങ്ങളുടെ തിരയലുമായി പൊരുത്തപ്പെടുന്ന ഇവന്റുകളൊന്നുമില്ല.
calendar-no-title = (ശീർഷകമില്ല)
calendar-all-day = ദിവസം മുഴുവൻ
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } എണ്ണം കൂടി
calendar-repeats = ആവർത്തിക്കുന്നു
calendar-join = ചേരുക
calendar-email-guests = അതിഥികൾക്ക് ഇമെയിൽ ചെയ്യുക
calendar-running-late = വൈകുന്നു
calendar-late-subject = വൈകുന്നു: { $title }
calendar-late-body = ക്ഷമിക്കണം, { $title }-ന് എത്താൻ എനിക്ക് കുറച്ച് മിനിറ്റ് വൈകും. ഞാൻ ഉടൻ എത്താം.
calendar-guests =
    { $count ->
        [one] { $count } അതിഥി
       *[other] { $count } അതിഥികൾ
    }
calendar-guest-answers = { $yes } അതെ, { $maybe } ആയിരിക്കാം, { $no } ഇല്ല, { $waiting } കാത്തിരിക്കുന്നു
calendar-organizer = സംഘാടകൻ
calendar-optional = ഓപ്ഷണൽ
calendar-open-web = ബ്രൗസറിൽ തുറക്കുക
calendar-open-contact = കോൺടാക്റ്റ് തുറക്കുക
calendar-close = അടയ്ക്കുക

## Adding, changing and deleting events.

calendar-add-title = തലക്കെട്ട് ചേർക്കുക
calendar-add-location = സ്ഥലം ചേർക്കുക
calendar-add-notes = വിവരണം ചേർക്കുക
calendar-add-guests = അതിഥികളെ ചേർക്കുക
calendar-remove-guest = നീക്കം ചെയ്യുക
calendar-add-meet = Google Meet വീഡിയോ കോൾ ചേർക്കുക
calendar-add-teams = Teams മീറ്റിംഗ് ചേർക്കുക
calendar-has-call = വീഡിയോ കോൾ ചേർത്തു
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = ദിവസം മുഴുവൻ
calendar-more-options = കൂടുതൽ ഓപ്ഷനുകൾ
calendar-save = സംരക്ഷിക്കുക
calendar-saved = ഇവന്റ് സംരക്ഷിച്ചു
calendar-deleted = ഇവന്റ് ഇല്ലാതാക്കി
calendar-discard = മാറ്റങ്ങൾ നിരസിക്കുക
calendar-edit = ഇവന്റ് എഡിറ്റ് ചെയ്യുക
calendar-delete = ഇവന്റ് ഇല്ലാതാക്കുക
calendar-event-details = ഇവന്റ് വിശദാംശങ്ങൾ
# Right-click menus on the calendar: on a free time or day, an event and
# a task.
calendar-menu-new-event = പുതിയ ഇവന്റ്
# Shows the day right-clicked on its own, in the Day view.
calendar-menu-open-day = ദിവസം തുറക്കുക
calendar-menu-duplicate = ഡ്യൂപ്ലിക്കേറ്റ് ചെയ്യുക
calendar-menu-color = നിറം
# The event takes its calendar's color.
calendar-menu-color-calendar = കലണ്ടർ നിറം
# A task's new due day, a week from today.
calendar-menu-in-a-week = ഒരാഴ്ചയ്ക്കുള്ളിൽ
# Event colors, by the names Google Calendar gives them.
calendar-color-tomato = തക്കാളി
calendar-color-flamingo = ഫ്ലമിംഗോ
calendar-color-tangerine = ടാൻജറിൻ
calendar-color-banana = വാഴപ്പഴം
calendar-color-sage = സേജ്
calendar-color-basil = തുളസി
calendar-color-peacock = മയിൽ
calendar-color-blueberry = ബ്ലൂബെറി
calendar-color-lavender = ലാവെൻഡർ
calendar-color-grape = മുന്തിരി
calendar-color-graphite = ഗ്രാഫൈറ്റ്
calendar-kind-event = ഇവന്റ്
calendar-kind-focus = ഫോക്കസ് സമയം
calendar-kind-out-of-office = ഓഫീസിന് പുറത്ത്
calendar-kind-working-location = ജോലി സ്ഥലം
calendar-working-home = വീട്
calendar-busy = തിരക്കിലാണ്
calendar-free = ഒഴിവാണ്
calendar-cancel = റദ്ദാക്കുക
calendar-ok = ശരി
calendar-read-only = ഈ കലണ്ടറിലെ ഇവന്റുകൾ നിങ്ങൾക്ക് മാറ്റാനാകില്ല
calendar-none-editable = ഇവന്റുകൾ ചേർക്കാനാകുന്ന കലണ്ടർ ഇതുവരെ ഇല്ല
calendar-no-such-time = നിങ്ങളുടെ ടൈം സോണിൽ ആ സമയം ഇല്ല
calendar-end-before-start = ഇവന്റ് തുടങ്ങുന്നതിന് മുമ്പേ അവസാനിക്കുന്നു
calendar-repeat-never = ആവർത്തിക്കുന്നില്ല
calendar-repeat-daily = ദിവസവും
calendar-repeat-weekly = പ്രതിവാരം: { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] പ്രതിമാസം: ആദ്യത്തെ { $weekday }
        [2] പ്രതിമാസം: രണ്ടാമത്തെ { $weekday }
        [3] പ്രതിമാസം: മൂന്നാമത്തെ { $weekday }
        [4] പ്രതിമാസം: നാലാമത്തെ { $weekday }
       *[other] പ്രതിമാസം: അവസാനത്തെ { $weekday }
    }
calendar-repeat-yearly = വാർഷികം: { $day }
calendar-repeat-weekdays = എല്ലാ പ്രവൃത്തി ദിവസവും (തിങ്കൾ മുതൽ വെള്ളി വരെ)
calendar-repeat-custom = ഇഷ്ടാനുസൃതം
calendar-reminder-none = അറിയിപ്പില്ല
calendar-reminder-at-start = ആരംഭത്തിൽ
calendar-reminder-minutes =
    { $count ->
        [one] { $count } മിനിറ്റ് മുമ്പ്
       *[other] { $count } മിനിറ്റ് മുമ്പ്
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } മണിക്കൂർ മുമ്പ്
       *[other] { $count } മണിക്കൂർ മുമ്പ്
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } ദിവസം മുമ്പ്
       *[other] { $count } ദിവസം മുമ്പ്
    }
calendar-scope-edit-title = ആവർത്തിക്കുന്ന ഇവന്റ് എഡിറ്റ് ചെയ്യുക
calendar-scope-delete-title = ആവർത്തിക്കുന്ന ഇവന്റ് ഇല്ലാതാക്കുക
calendar-scope-this = ഈ ഇവന്റ്
calendar-scope-following = ഈ ഇവന്റും തുടർന്നുള്ളവയും
calendar-scope-all = എല്ലാ ഇവന്റുകളും
calendar-scope-respond-title = ആവർത്തിക്കുന്ന ഇവന്റിനുള്ള മറുപടി
calendar-going = നിങ്ങൾ പോകുന്നുണ്ടോ?
calendar-answer-yes = അതെ
calendar-answer-no = ഇല്ല
calendar-answer-maybe = ആയിരിക്കാം
calendar-answered-yes = നിങ്ങൾ പോകുന്നു
calendar-answered-no = നിങ്ങൾ പോകുന്നില്ല
calendar-answered-maybe = നിങ്ങൾ പോയേക്കാം

## The card at the top of a mail with an invitation.

calendar-invite = ക്ഷണം
calendar-invite-cancelled = ഇവന്റ് റദ്ദാക്കി
calendar-invite-reply = { $name } മറുപടി നൽകി
calendar-invite-reply-yes = { $name } സ്വീകരിച്ചു
calendar-invite-reply-no = { $name } നിരസിച്ചു
calendar-invite-reply-maybe = { $name } പോയേക്കാം
calendar-invite-organizer = സംഘടിപ്പിച്ചത്: { $name }
calendar-invite-open = കലണ്ടറിൽ തുറക്കുക
calendar-invite-not-yet = ഇതുവരെ നിങ്ങളുടെ കലണ്ടറിൽ ഇല്ല. സമന്വയിപ്പിച്ച ശേഷം മറുപടി നൽകാം.
calendar-invite-by-mail = നിങ്ങളുടെ കലണ്ടറിൽ ഇല്ല: നിങ്ങളുടെ മറുപടി മെയിൽ വഴി സംഘാടകന് പോകും.
calendar-mail-yes = സ്വീകരിച്ചു: { $title }
calendar-mail-yes-body = { $name } ഈ ക്ഷണം സ്വീകരിച്ചു.
calendar-mail-no = നിരസിച്ചു: { $title }
calendar-mail-no-body = { $name } ഈ ക്ഷണം നിരസിച്ചു.
calendar-mail-maybe = താൽക്കാലികമായി സ്വീകരിച്ചു: { $title }
calendar-mail-maybe-body = { $name } ഈ ക്ഷണം താൽക്കാലികമായി സ്വീകരിച്ചു.
calendar-invite-your-day = നിങ്ങളുടെ ദിവസം
calendar-invite-clashes =
    { $count ->
        [one] { $count } ഇവന്റുമായി ഏറ്റുമുട്ടുന്നു
       *[other] { $count } ഇവന്റുകളുമായി ഏറ്റുമുട്ടുന്നു
    }

## The day's agenda beside the mail.

agenda-show = ദിവസത്തെ അജണ്ട കാണിക്കുക
agenda-hide = അജണ്ട മറയ്ക്കുക
agenda-today = ഇന്ന്, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = ഈ ദിവസം ഒന്നും ആസൂത്രണം ചെയ്തിട്ടില്ല.
