# Katna Mail, Malayalam (മലയാളം): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = സൃഷ്ടിക്കുക
tasks-all = എല്ലാ ടാസ്‌ക്കുകളും
tasks-today = ഇന്ന്
tasks-starred = നക്ഷത്രമിട്ടവ
tasks-new-list = പുതിയ ലിസ്റ്റ് സൃഷ്ടിക്കുക
tasks-on-this-computer = ഈ കമ്പ്യൂട്ടറിൽ
tasks-my-tasks = എന്റെ ടാസ്‌ക്കുകൾ
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = ടാസ്‌ക്കുകൾ കാണിക്കാൻ വീണ്ടും സൈൻ ഇൻ ചെയ്യുക
tasks-account-signed-in = { $address }-ൽ വീണ്ടും സൈൻ ഇൻ ചെയ്തു. നിങ്ങളുടെ ടാസ്‌ക്കുകൾ ലഭ്യമാക്കുന്നു…
tasks-account-sign-in-refused = { $provider } Katna-യെ അകത്ത് കയറ്റിയില്ല. വീണ്ടും ശ്രമിക്കുക, നിങ്ങളുടെ ടാസ്‌ക്കുകളിലേക്ക് ആക്‌സസ് അനുവദിക്കുക.
tasks-account-refused = സെർവർ പാസ്‌വേഡ് സ്വീകരിച്ചില്ല. Yahoo, iCloud, Zoho എന്നിവയ്ക്കും മറ്റുള്ളവയ്ക്കും ഒരു ആപ്പ് പാസ്‌വേഡ് വേണം.
tasks-account-change-password = പാസ്‌വേഡ് മാറ്റുക
tasks-account-change-password-tooltip = ക്രമീകരണം > അക്കൗണ്ടുകൾ തുറക്കുക
tasks-account-not-enabled = Katna-യ്ക്കുള്ള ടാസ്‌ക് ആക്‌സസ് ഇതുവരെ ഓണാക്കിയിട്ടില്ല.
tasks-account-failed = ടാസ്‌ക് ലിസ്റ്റുകൾ വായിക്കാൻ കഴിഞ്ഞില്ല.
# $reason is the server's own words, in English.
tasks-account-error = ടാസ്‌ക് ലിസ്റ്റുകൾ വായിക്കാൻ കഴിഞ്ഞില്ല: { $reason }
tasks-account-none = ടാസ്‌ക് ലിസ്റ്റുകളൊന്നും കണ്ടെത്തിയില്ല
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = ടാസ്‌ക് ലിസ്റ്റുകളൊന്നും കണ്ടെത്തിയില്ല: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } ഉപയോഗിച്ച് സൈൻ ഇൻ ചെയ്ത Katna-യ്ക്ക് മാത്രമേ { $provider } ടാസ്‌ക്കുകൾ കാണിക്കൂ.
tasks-account-sign-in-with = { $provider } ഉപയോഗിച്ച് സൈൻ ഇൻ ചെയ്യുക
tasks-account-looking = ടാസ്‌ക് ലിസ്റ്റുകൾ തിരയുന്നു…
tasks-account-try-again = വീണ്ടും ശ്രമിക്കുക
tasks-account-try-again-tooltip = ഈ അക്കൗണ്ടിന്റെ ടാസ്‌ക്കുകൾ ഇപ്പോൾ വീണ്ടും പരിശോധിക്കുക
tasks-account-fixing = പരിഹരിക്കുന്നു…
tasks-list-name-placeholder = ലിസ്റ്റിന്റെ പേര്

## Lists and tasks

tasks-loading = നിങ്ങളുടെ ടാസ്‌ക്കുകൾ വായിക്കുന്നു…
tasks-no-lists = നിങ്ങളുടെ ടാസ്‌ക് ലിസ്റ്റുകൾ ഇവിടെ കാണാം.
tasks-search = ടാസ്‌ക്കുകൾ തിരയുക
tasks-search-none = നിങ്ങളുടെ തിരയലുമായി പൊരുത്തപ്പെടുന്ന ടാസ്‌ക്കുകളൊന്നുമില്ല.
tasks-add = ടാസ്‌ക് ചേർക്കുക
tasks-title-placeholder = ശീർഷകം
tasks-add-step = ഉപടാസ്‌ക് ചേർക്കുക
tasks-empty = ടാസ്‌ക്കുകളൊന്നുമില്ല. മുകളിൽ ഒന്ന് ചേർക്കുക.
tasks-starred-empty = ഇവിടെ കാണാൻ ഒരു ടാസ്‌ക്കിന് നക്ഷത്രമിടുക.
tasks-today-empty = ഇന്നത്തേക്ക് ഒന്നുമില്ല.
tasks-today-date = { $weekday }, { $day }
tasks-overdue = കാലഹരണപ്പെട്ടത്
tasks-completed = { $count ->
    [one] പൂർത്തിയായവ ({ $count })
   *[other] പൂർത്തിയായവ ({ $count })
}
tasks-list-options = ലിസ്റ്റ് ഓപ്ഷനുകൾ
tasks-rename-list = ലിസ്റ്റിന്റെ പേര് മാറ്റുക
tasks-delete-list = ലിസ്റ്റ് ഇല്ലാതാക്കുക
tasks-mark-done = പൂർത്തിയായതായി അടയാളപ്പെടുത്തുക
tasks-mark-open = പൂർത്തിയാകാത്തതായി അടയാളപ്പെടുത്തുക
tasks-star = നക്ഷത്രമിടുക
tasks-unstar = നക്ഷത്രം നീക്കം ചെയ്യുക
tasks-edit-title = ശീർഷകം എഡിറ്റ് ചെയ്യുക
tasks-details = വിശദാംശങ്ങൾ
tasks-delete = ഇല്ലാതാക്കുക
tasks-move-to = { $list } എന്നതിലേക്ക് നീക്കുക
tasks-from-mail = മെയിൽ
tasks-open-mail = മെയിൽ തുറക്കുക
tasks-from-note = കുറിപ്പ്
tasks-open-note = കുറിപ്പ് തുറക്കുക
tasks-note-gone = ആ കുറിപ്പ് ഇപ്പോൾ ഇവിടെ ഇല്ല.
tasks-no-subject = (വിഷയമില്ല)

## The details dialog

tasks-notes-placeholder = വിശദാംശങ്ങൾ ചേർക്കുക
tasks-date = തീയതി
tasks-no-date = തീയതി ഇല്ല
tasks-time-placeholder = സമയം ചേർക്കുക
tasks-repeat = ആവർത്തിക്കുക
tasks-repeat-never = ആവർത്തിക്കുന്നില്ല
tasks-repeat-daily = ദിവസവും
tasks-repeat-weekly = ആഴ്ചതോറും
tasks-repeat-monthly = മാസംതോറും
tasks-repeat-yearly = വർഷംതോറും
tasks-repeat-other = ഇഷ്ടാനുസൃതം
tasks-remind = എന്നെ ഓർമ്മിപ്പിക്കുക
tasks-remind-off = ഓർമ്മിപ്പിക്കരുത്
tasks-remind-on-time = ആ സമയത്ത്
tasks-remind-morning = അന്നേ ദിവസം, { $time }
tasks-remind-hour-before = ഒരു മണിക്കൂർ മുമ്പ്
tasks-remind-day-before = ഒരു ദിവസം മുമ്പ്
tasks-cancel = റദ്ദാക്കുക
tasks-save = സംരക്ഷിക്കുക
tasks-not-a-time = “{ $text }” ഒരു സമയമല്ല, ഉദാഹരണത്തിന് { $example }.

## Due days

tasks-due-today = ഇന്ന്
tasks-due-tomorrow = നാളെ
tasks-due-yesterday = ഇന്നലെ
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = ടാസ്‌ക് പൂർത്തിയായി
tasks-toast-next = കഴിഞ്ഞു. അടുത്തത് { $date }-ന്
tasks-toast-deleted = ടാസ്‌ക് ഇല്ലാതാക്കി
tasks-toast-added = { $count ->
    [one] ടാസ്‌ക്കുകളിലേക്ക് ചേർത്തു
   *[other] { $count } ടാസ്‌ക്കുകൾ ചേർത്തു
}
tasks-mail-gone = ആ മെയിൽ ഇപ്പോൾ ഇവിടെ ഇല്ല.
tasks-toast-list-deleted = ലിസ്റ്റ് ഇല്ലാതാക്കി
tasks-toast-moved = { $list } എന്നതിലേക്ക് നീക്കി
tasks-toast-rescheduled = ടാസ്‌ക്കിന്റെ സമയം മാറ്റി
