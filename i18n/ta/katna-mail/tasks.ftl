# Katna Mail, Tamil (தமிழ்): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = உருவாக்கு
tasks-all = அனைத்துப் பணிகளும்
tasks-today = இன்று
tasks-starred = நட்சத்திரமிட்டவை
tasks-new-list = புதிய பட்டியலை உருவாக்கு
tasks-on-this-computer = இந்தக் கணினியில்
tasks-my-tasks = எனது பணிகள்
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = பணிகளைக் காட்ட மீண்டும் உள்நுழை
tasks-account-signed-in = { $address } இல் மீண்டும் உள்நுழைந்தது. உங்கள் பணிகளைப் பெறுகிறது…
tasks-account-sign-in-refused = { $provider } Katna-வை உள்ளே அனுமதிக்கவில்லை. மீண்டும் முயன்று, உங்கள் பணிகளுக்கான அணுகலை அனுமதிக்கவும்.
tasks-account-refused = சர்வர் கடவுச்சொல்லை ஏற்கவில்லை. Yahoo, iCloud, Zoho போன்றவற்றுக்கு ஆப் கடவுச்சொல் தேவை.
tasks-account-change-password = கடவுச்சொல்லை மாற்று
tasks-account-change-password-tooltip = அமைப்புகள் > கணக்குகள் என்பதைத் திற
tasks-account-not-enabled = Katna-வுக்கான பணி அணுகல் இன்னும் இயக்கப்படவில்லை.
tasks-account-failed = பணிப் பட்டியல்களைப் படிக்க முடியவில்லை.
# $reason is the server's own words, in English.
tasks-account-error = பணிப் பட்டியல்களைப் படிக்க முடியவில்லை: { $reason }
tasks-account-none = பணிப் பட்டியல்கள் எதுவும் இல்லை
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = பணிப் பட்டியல்கள் எதுவும் இல்லை: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } மூலம் உள்நுழைந்த Katna-வுக்கு மட்டுமே { $provider } பணிகளைக் காட்டும்.
tasks-account-sign-in-with = { $provider } மூலம் உள்நுழை
tasks-account-looking = பணிப் பட்டியல்களைத் தேடுகிறது…
tasks-account-try-again = மீண்டும் முயல்க
tasks-account-try-again-tooltip = இந்தக் கணக்கின் பணிகளை இப்போது மீண்டும் சரிபார்
tasks-account-fixing = சரிசெய்கிறது…
tasks-list-name-placeholder = பட்டியலின் பெயர்

## Lists and tasks

tasks-loading = உங்கள் பணிகளைப் படிக்கிறது…
tasks-no-lists = உங்கள் பணிப் பட்டியல்கள் இங்கே தோன்றும்.
tasks-search = பணிகளைத் தேடு
tasks-search-none = உங்கள் தேடலுக்குப் பொருந்தும் பணிகள் இல்லை.
tasks-add = பணியைச் சேர்
tasks-title-placeholder = தலைப்பு
tasks-add-step = துணைப் பணியைச் சேர்
tasks-empty = இன்னும் பணிகள் இல்லை. மேலே ஒன்றைச் சேருங்கள்.
tasks-starred-empty = இங்கே காண ஒரு பணிக்கு நட்சத்திரமிடுங்கள்.
tasks-today-empty = இன்றைக்கு எதுவும் இல்லை.
tasks-today-date = { $weekday }, { $day }
tasks-overdue = காலாவதியானவை
tasks-completed = { $count ->
    [one] முடிந்தவை ({ $count })
   *[other] முடிந்தவை ({ $count })
}
tasks-list-options = பட்டியல் விருப்பங்கள்
tasks-rename-list = பட்டியலுக்கு மறுபெயரிடு
tasks-delete-list = பட்டியலை நீக்கு
tasks-mark-done = முடிந்ததாகக் குறி
tasks-mark-open = முடிக்கப்படாததாகக் குறி
tasks-star = நட்சத்திரமிடு
tasks-unstar = நட்சத்திரத்தை அகற்று
tasks-edit-title = தலைப்பைத் திருத்து
tasks-details = விவரங்கள்
tasks-delete = நீக்கு
tasks-move-to = { $list } பட்டியலுக்கு நகர்த்து
tasks-from-mail = அஞ்சல்
tasks-open-mail = அஞ்சலைத் திற
tasks-from-note = குறிப்பு
tasks-open-note = குறிப்பைத் திற
tasks-note-gone = அந்தக் குறிப்பு இனி இங்கு இல்லை.
tasks-no-subject = (பொருள் இல்லை)

## The details dialog

tasks-notes-placeholder = விவரங்களைச் சேர்
tasks-date = தேதி
tasks-no-date = தேதி இல்லை
tasks-time-placeholder = நேரத்தைச் சேர்
tasks-repeat = மீண்டும் செய்
tasks-repeat-never = மீண்டும் நிகழாது
tasks-repeat-daily = தினமும்
tasks-repeat-weekly = வாரந்தோறும்
tasks-repeat-monthly = மாதந்தோறும்
tasks-repeat-yearly = ஆண்டுதோறும்
tasks-repeat-other = தனிப்பயன்
tasks-remind = எனக்கு நினைவூட்டு
tasks-remind-off = நினைவூட்ட வேண்டாம்
tasks-remind-on-time = அந்த நேரத்தில்
tasks-remind-morning = அன்றைய தினம், { $time }
tasks-remind-hour-before = ஒரு மணிநேரத்திற்கு முன்
tasks-remind-day-before = ஒரு நாளுக்கு முன்
tasks-cancel = ரத்துசெய்
tasks-save = சேமி
tasks-not-a-time = “{ $text }” என்பது நேரம் அல்ல, எடுத்துக்காட்டாக { $example }.

## Due days

tasks-due-today = இன்று
tasks-due-tomorrow = நாளை
tasks-due-yesterday = நேற்று
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = பணி முடிந்தது
tasks-toast-next = முடிந்தது. அடுத்தது { $date } அன்று
tasks-toast-deleted = பணி நீக்கப்பட்டது
tasks-toast-added = { $count ->
    [one] பணிகளில் சேர்க்கப்பட்டது
   *[other] { $count } பணிகள் சேர்க்கப்பட்டன
}
tasks-mail-gone = அந்த அஞ்சல் இனி இங்கு இல்லை.
tasks-toast-list-deleted = பட்டியல் நீக்கப்பட்டது
tasks-toast-moved = { $list } பட்டியலுக்கு நகர்த்தப்பட்டது
tasks-toast-rescheduled = பணியின் நேரம் மாற்றப்பட்டது
