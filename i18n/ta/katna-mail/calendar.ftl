# Katna Mail, Tamil (தமிழ்): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = இன்று
calendar-today-tip = இன்றைக்குச் செல்
calendar-view-day = நாள்
calendar-view-week = வாரம்
calendar-view-month = மாதம்
calendar-view-schedule = அட்டவணை
calendar-previous-day = முந்தைய நாள்
calendar-next-day = அடுத்த நாள்
calendar-previous-week = முந்தைய வாரம்
calendar-next-week = அடுத்த வாரம்
calendar-previous-month = முந்தைய மாதம்
calendar-next-month = அடுத்த மாதம்
calendar-previous-period = முன்னதாக
calendar-next-period = பின்னர்
calendar-title-months = { $first } – { $last }
calendar-loading = ஏற்றப்படுகிறது…
calendar-read-failed = கேலெண்டரைப் படிக்க முடியவில்லை: { $error }
calendar-local = இந்தக் கணினியில்
calendar-account-gone = அகற்றப்பட்ட கணக்கு
calendar-birthdays = பிறந்தநாள்கள்
calendar-birthday-of = { $name } பிறந்தநாள்
calendar-empty-title = இன்னும் கேலெண்டர்கள் இல்லை
calendar-empty-text = உங்கள் Google, Microsoft கணக்குகளின் கேலெண்டர்களும், CalDAV வழங்கும் பிற சேவையகங்களின் கேலெண்டர்களும் ஒத்திசைந்ததும் Katna அவற்றை இங்கே காட்டும்.
calendar-schedule-empty = அடுத்த இரண்டு மாதங்களுக்கு எதுவும் திட்டமிடப்படவில்லை.
calendar-no-title = (தலைப்பு இல்லை)
calendar-all-day = நாள் முழுவதும்
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = மேலும் { $count }
calendar-repeats = மீண்டும் நிகழும்
calendar-join = சேர்
calendar-email-guests = விருந்தினர்களுக்கு மின்னஞ்சல் அனுப்பு
calendar-running-late = தாமதமாகிறது
calendar-late-subject = தாமதமாகிறது: { $title }
calendar-late-body = மன்னிக்கவும், { $title } நிகழ்வுக்கு வர எனக்குச் சில நிமிடங்கள் தாமதமாகிறது. விரைவில் வந்துவிடுவேன்.
calendar-guests =
    { $count ->
        [one] { $count } விருந்தினர்
       *[other] { $count } விருந்தினர்கள்
    }
calendar-guest-answers = { $yes } ஆம், { $maybe } இருக்கலாம், { $no } இல்லை, { $waiting } காத்திருப்பு
calendar-organizer = அமைப்பாளர்
calendar-optional = விருப்பத்திற்குரியது
calendar-open-web = உலாவியில் திற
calendar-open-contact = தொடர்பைத் திற
calendar-close = மூடு

## Adding, changing and deleting events.

calendar-add-title = தலைப்பைச் சேர்
calendar-add-location = இருப்பிடத்தைச் சேர்
calendar-add-notes = விளக்கத்தைச் சேர்
calendar-add-guests = விருந்தினர்களைச் சேர்
calendar-remove-guest = அகற்று
calendar-add-meet = Google Meet வீடியோ அழைப்பைச் சேர்
calendar-add-teams = Teams சந்திப்பைச் சேர்
calendar-has-call = வீடியோ அழைப்பு சேர்க்கப்பட்டது
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = நாள் முழுவதும்
calendar-more-options = மேலும் விருப்பங்கள்
calendar-save = சேமி
calendar-saved = நிகழ்வு சேமிக்கப்பட்டது
calendar-deleted = நிகழ்வு நீக்கப்பட்டது
calendar-discard = மாற்றங்களை நிராகரி
calendar-edit = நிகழ்வைத் திருத்து
calendar-delete = நிகழ்வை நீக்கு
calendar-event-details = நிகழ்வு விவரங்கள்
calendar-kind-event = நிகழ்வு
calendar-kind-focus = ஃபோகஸ் நேரம்
calendar-kind-out-of-office = அலுவலகத்தில் இல்லை
calendar-kind-working-location = பணியிடம்
calendar-working-home = வீடு
calendar-busy = பிஸி
calendar-free = ஃப்ரீ
calendar-cancel = ரத்துசெய்
calendar-ok = சரி
calendar-read-only = இந்தக் கேலண்டரில் நிகழ்வுகளை மாற்ற முடியாது
calendar-none-editable = நிகழ்வுகளைச் சேர்க்கக்கூடிய கேலண்டர் இன்னும் இல்லை
calendar-no-such-time = உங்கள் நேர மண்டலத்தில் அந்த நேரம் இல்லை
calendar-end-before-start = நிகழ்வு தொடங்கும் முன்பே முடிகிறது
calendar-repeat-never = மீண்டும் நிகழாது
calendar-repeat-daily = தினமும்
calendar-repeat-weekly = வாராந்திரம்: { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] மாதந்தோறும்: முதல் { $weekday }
        [2] மாதந்தோறும்: இரண்டாவது { $weekday }
        [3] மாதந்தோறும்: மூன்றாவது { $weekday }
        [4] மாதந்தோறும்: நான்காவது { $weekday }
       *[other] மாதந்தோறும்: கடைசி { $weekday }
    }
calendar-repeat-yearly = ஆண்டுதோறும்: { $day }
calendar-repeat-weekdays = ஒவ்வொரு வார நாளும் (திங்கள் முதல் வெள்ளி வரை)
calendar-repeat-custom = தனிப்பயன்
calendar-reminder-none = அறிவிப்பு இல்லை
calendar-reminder-at-start = தொடக்கத்தில்
calendar-reminder-minutes =
    { $count ->
        [one] { $count } நிமிடத்திற்கு முன்
       *[other] { $count } நிமிடங்களுக்கு முன்
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } மணிநேரத்திற்கு முன்
       *[other] { $count } மணிநேரத்திற்கு முன்
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } நாளுக்கு முன்
       *[other] { $count } நாட்களுக்கு முன்
    }
calendar-scope-edit-title = தொடர் நிகழ்வைத் திருத்து
calendar-scope-delete-title = தொடர் நிகழ்வை நீக்கு
calendar-scope-this = இந்த நிகழ்வு
calendar-scope-following = இந்த மற்றும் அடுத்த நிகழ்வுகள்
calendar-scope-all = அனைத்து நிகழ்வுகள்
calendar-scope-respond-title = தொடர் நிகழ்வுக்கான பதில்
calendar-going = நீங்கள் செல்கிறீர்களா?
calendar-answer-yes = ஆம்
calendar-answer-no = இல்லை
calendar-answer-maybe = இருக்கலாம்
calendar-answered-yes = நீங்கள் செல்கிறீர்கள்
calendar-answered-no = நீங்கள் செல்லவில்லை
calendar-answered-maybe = நீங்கள் செல்லக்கூடும்

## The card at the top of a mail with an invitation.

calendar-invite = அழைப்பிதழ்
calendar-invite-cancelled = நிகழ்வு ரத்துசெய்யப்பட்டது
calendar-invite-reply = { $name } பதிலளித்துள்ளார்
calendar-invite-reply-yes = { $name } ஏற்றுக்கொண்டார்
calendar-invite-reply-no = { $name } நிராகரித்தார்
calendar-invite-reply-maybe = { $name } செல்லக்கூடும்
calendar-invite-organizer = ஏற்பாடு: { $name }
calendar-invite-open = கேலெண்டரில் திற
calendar-invite-not-yet = இன்னும் உங்கள் கேலெண்டரில் இல்லை. ஒத்திசைந்ததும் பதிலளிக்கலாம்.
calendar-invite-by-mail = உங்கள் கேலெண்டரில் இல்லை: உங்கள் பதில் அஞ்சல் மூலம் ஏற்பாட்டாளருக்குச் செல்லும்.
calendar-mail-yes = ஏற்கப்பட்டது: { $title }
calendar-mail-yes-body = { $name } இந்த அழைப்பை ஏற்றுக்கொண்டுள்ளனர்.
calendar-mail-no = நிராகரிக்கப்பட்டது: { $title }
calendar-mail-no-body = { $name } இந்த அழைப்பை நிராகரித்துள்ளனர்.
calendar-mail-maybe = தற்காலிகமாக ஏற்கப்பட்டது: { $title }
calendar-mail-maybe-body = { $name } இந்த அழைப்பைத் தற்காலிகமாக ஏற்றுக்கொண்டுள்ளனர்.
calendar-invite-your-day = உங்கள் நாள்
calendar-invite-clashes =
    { $count ->
        [one] { $count } நிகழ்வுடன் மோதுகிறது
       *[other] { $count } நிகழ்வுகளுடன் மோதுகிறது
    }

## The day's agenda beside the mail.

agenda-show = நாளின் நிகழ்ச்சி நிரலைக் காட்டு
agenda-hide = நிகழ்ச்சி நிரலை மறை
agenda-today = இன்று, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = இந்த நாளில் எதுவும் திட்டமிடப்படவில்லை.
