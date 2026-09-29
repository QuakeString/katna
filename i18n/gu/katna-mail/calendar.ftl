# Katna Mail, Gujarati (ગુજરાતી): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = આજે
calendar-today-tip = આજે જાઓ
calendar-view-day = દિવસ
calendar-view-week = અઠવાડિયું
calendar-view-month = મહિનો
calendar-view-year = વર્ષ
calendar-view-schedule = શેડ્યૂલ
calendar-view-days =
    { $count ->
        [one] { $count } દિવસ
       *[other] { $count } દિવસ
    }
calendar-options = વિકલ્પો
calendar-density = ઘનતા
calendar-density-responsive = તમારી સ્ક્રીન મુજબ
calendar-density-comfortable = આરામદાયક
calendar-density-compact = કોમ્પેક્ટ
calendar-custom-days = કસ્ટમ દૃશ્ય
calendar-second-zone = બીજો સમય ઝોન
calendar-zone-none = કોઈ નહીં
calendar-zone = { $zone } ({ $offset })
calendar-share-free = ખાલી સમય શેર કરો
calendar-free-subject = મારા ખાલી સમય
calendar-free-intro = અહીં કેટલાક સમય છે જ્યારે હું ખાલી છું ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = આગામી થોડા કાર્યદિવસોમાં મારી પાસે ખાલી સમય નથી.
calendar-previous-day = પાછલો દિવસ
calendar-next-day = આગલો દિવસ
calendar-previous-week = પાછલું અઠવાડિયું
calendar-next-week = આગલું અઠવાડિયું
calendar-previous-month = પાછલો મહિનો
calendar-next-month = આગલો મહિનો
calendar-previous-year = પાછલું વર્ષ
calendar-next-year = આગલું વર્ષ
calendar-previous-period = પહેલાં
calendar-next-period = પછી
calendar-title-months = { $first } – { $last }
calendar-loading = લોડ થઈ રહ્યું છે…
calendar-read-failed = કૅલેન્ડર વાંચી શકાયું નથી: { $error }
calendar-sets = કૅલેન્ડર સેટ
calendar-set-add = બતાવેલાં કૅલેન્ડરને સેટ તરીકે સાચવો
calendar-set-name = સેટનું નામ
calendar-set-remove = સેટ દૂર કરો
calendar-local = આ કમ્પ્યુટર પર
calendar-account-gone = દૂર કરેલું એકાઉન્ટ
calendar-account-sign-in = કૅલેન્ડર બતાવવા માટે ફરી સાઇન ઇન કરો
calendar-account-signed-in = { $address } માં ફરી સાઇન ઇન થયું. તમારા કૅલેન્ડર મેળવી રહ્યાં છીએ…
calendar-account-sign-in-refused = { $provider } એ Katna ને અંદર આવવા ન દીધું. ફરી પ્રયાસ કરો, અને તમારા કૅલેન્ડરની ઍક્સેસની મંજૂરી આપો.
calendar-account-refused = સર્વરે પાસવર્ડ સ્વીકાર્યો નહીં. Yahoo, iCloud, Zoho અને અન્યને ઍપ પાસવર્ડની જરૂર છે.
calendar-account-change-password = પાસવર્ડ બદલો
calendar-account-change-password-tooltip = સેટિંગ > એકાઉન્ટ ખોલો
calendar-account-not-enabled = Katna માટે કૅલેન્ડર ઍક્સેસ હજી ચાલુ નથી.
calendar-account-failed = કૅલેન્ડર વાંચી શકાયાં નથી.
calendar-account-error = કૅલેન્ડર વાંચી શકાયાં નથી: { $reason }
calendar-account-none = કોઈ કૅલેન્ડર મળ્યું નથી
calendar-account-looking = કૅલેન્ડર શોધી રહ્યાં છીએ…
calendar-account-try-again = ફરી પ્રયાસ કરો
calendar-account-try-again-tooltip = આ એકાઉન્ટના કૅલેન્ડર હમણાં ફરી તપાસો
calendar-account-fixing = કામ ચાલુ છે…
calendar-birthdays = જન્મદિવસ
calendar-birthday-of = { $name }નો જન્મદિવસ
calendar-empty-title = હજી કોઈ કૅલેન્ડર નથી
calendar-empty-text = Katna તમારા Google અને Microsoft એકાઉન્ટના કૅલેન્ડર, તેમજ CalDAV આપતા અન્ય સર્વરના કૅલેન્ડર સિંક થઈ જાય પછી અહીં બતાવે છે.
calendar-schedule-empty = આગામી બે મહિનામાં કંઈ આયોજિત નથી.
calendar-search = ઇવેન્ટ શોધો
calendar-search-past = પાછલા ઇવેન્ટ
calendar-search-none = તમારી શોધ સાથે કોઈ ઇવેન્ટ મેળ ખાતો નથી.
calendar-no-title = (કોઈ શીર્ષક નથી)
calendar-all-day = આખો દિવસ
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = વધુ { $count }
calendar-repeats = પુનરાવર્તન થાય છે
calendar-join = જોડાઓ
calendar-email-guests = મહેમાનોને ઇમેઇલ કરો
calendar-running-late = મોડું થઈ રહ્યું છે
calendar-late-subject = મોડું થઈ રહ્યું છે: { $title }
calendar-late-body = માફ કરશો, { $title } માટે મને થોડી મિનિટ મોડું થઈ રહ્યું છે. હું જલદી પહોંચીશ.
calendar-guests =
    { $count ->
        [one] { $count } મહેમાન
       *[other] { $count } મહેમાન
    }
calendar-guest-answers = { $yes } હા, { $maybe } કદાચ, { $no } ના, { $waiting } બાકી
calendar-organizer = આયોજક
calendar-optional = વૈકલ્પિક
calendar-open-web = બ્રાઉઝરમાં ખોલો
calendar-open-contact = સંપર્ક ખોલો
calendar-close = બંધ કરો

## Adding, changing and deleting events.

calendar-add-title = શીર્ષક ઉમેરો
calendar-add-location = સ્થાન ઉમેરો
calendar-add-notes = વર્ણન ઉમેરો
calendar-add-guests = મહેમાનો ઉમેરો
calendar-remove-guest = દૂર કરો
calendar-add-meet = Google Meet વીડિયો કૉલ ઉમેરો
calendar-add-teams = Teams મીટિંગ ઉમેરો
calendar-has-call = વીડિયો કૉલ ઉમેર્યો
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = આખો દિવસ
calendar-more-options = વધુ વિકલ્પો
calendar-save = સેવ કરો
calendar-saved = ઇવેન્ટ સેવ થયો
calendar-deleted = ઇવેન્ટ ડિલીટ થયો
calendar-discard = ફેરફારો કાઢી નાખો
calendar-edit = ઇવેન્ટમાં ફેરફાર કરો
calendar-delete = ઇવેન્ટ ડિલીટ કરો
calendar-event-details = ઇવેન્ટની વિગતો
calendar-kind-event = ઇવેન્ટ
calendar-kind-focus = ફોકસ સમય
calendar-kind-out-of-office = ઑફિસની બહાર
calendar-kind-working-location = કામ કરવાનું સ્થળ
calendar-working-home = ઘર
calendar-busy = વ્યસ્ત
calendar-free = ખાલી
calendar-cancel = રદ કરો
calendar-ok = ઠીક છે
calendar-read-only = તમે આ કૅલેન્ડરના ઇવેન્ટ બદલી શકતા નથી
calendar-none-editable = હજી એવું કોઈ કૅલેન્ડર નથી જેમાં તમે ઇવેન્ટ ઉમેરી શકો
calendar-no-such-time = તમારા ટાઇમ ઝોનમાં આ સમય અસ્તિત્વમાં નથી
calendar-end-before-start = ઇવેન્ટ શરૂ થાય તે પહેલાં પૂરો થાય છે
calendar-repeat-never = પુનરાવર્તન થતું નથી
calendar-repeat-daily = દરરોજ
calendar-repeat-weekly = સાપ્તાહિક: { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] માસિક: પહેલો { $weekday }
        [2] માસિક: બીજો { $weekday }
        [3] માસિક: ત્રીજો { $weekday }
        [4] માસિક: ચોથો { $weekday }
       *[other] માસિક: છેલ્લો { $weekday }
    }
calendar-repeat-yearly = વાર્ષિક: { $day }
calendar-repeat-weekdays = દરેક કાર્યદિવસ (સોમવારથી શુક્રવાર)
calendar-repeat-custom = કસ્ટમ
calendar-reminder-none = કોઈ સૂચના નહીં
calendar-reminder-at-start = શરૂઆતમાં
calendar-reminder-minutes =
    { $count ->
        [one] { $count } મિનિટ પહેલાં
       *[other] { $count } મિનિટ પહેલાં
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } કલાક પહેલાં
       *[other] { $count } કલાક પહેલાં
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } દિવસ પહેલાં
       *[other] { $count } દિવસ પહેલાં
    }
calendar-scope-edit-title = પુનરાવર્તિત ઇવેન્ટમાં ફેરફાર કરો
calendar-scope-delete-title = પુનરાવર્તિત ઇવેન્ટ ડિલીટ કરો
calendar-scope-this = આ ઇવેન્ટ
calendar-scope-following = આ અને પછીના ઇવેન્ટ
calendar-scope-all = બધા ઇવેન્ટ
calendar-scope-respond-title = પુનરાવર્તિત ઇવેન્ટ માટે જવાબ
calendar-going = તમે જવાના છો?
calendar-answer-yes = હા
calendar-answer-no = ના
calendar-answer-maybe = કદાચ
calendar-answered-yes = તમે જઈ રહ્યા છો
calendar-answered-no = તમે નથી જઈ રહ્યા
calendar-answered-maybe = તમે કદાચ જશો

## The card at the top of a mail with an invitation.

calendar-invite = આમંત્રણ
calendar-invite-cancelled = ઇવેન્ટ રદ કરી
calendar-invite-reply = { $name } એ જવાબ આપ્યો
calendar-invite-reply-yes = { $name } એ સ્વીકાર્યું
calendar-invite-reply-no = { $name } એ નકાર્યું
calendar-invite-reply-maybe = { $name }: કદાચ
calendar-invite-organizer = { $name } દ્વારા આયોજિત
calendar-invite-open = કૅલેન્ડરમાં ખોલો
calendar-invite-not-yet = હજી તમારા કૅલેન્ડરમાં નથી. સિંક થયા પછી જવાબ આપી શકાશે.
calendar-invite-by-mail = તમારા કૅલેન્ડરમાં નથી: તમારો જવાબ મેઇલ દ્વારા આયોજકને જશે.
calendar-mail-yes = સ્વીકાર્યું: { $title }
calendar-mail-yes-body = { $name } એ આ આમંત્રણ સ્વીકાર્યું છે.
calendar-mail-no = નકાર્યું: { $title }
calendar-mail-no-body = { $name } એ આ આમંત્રણ નકાર્યું છે.
calendar-mail-maybe = કામચલાઉ સ્વીકાર્યું: { $title }
calendar-mail-maybe-body = { $name } એ આ આમંત્રણ કામચલાઉ સ્વીકાર્યું છે.
calendar-invite-your-day = તમારો દિવસ
calendar-invite-clashes =
    { $count ->
        [one] { $count } ઇવેન્ટ સાથે ટકરાવ
       *[other] { $count } ઇવેન્ટ સાથે ટકરાવ
    }

## The day's agenda beside the mail.

agenda-show = દિવસનો એજન્ડા બતાવો
agenda-hide = એજન્ડા છુપાવો
agenda-today = આજે, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = આ દિવસે કંઈ આયોજિત નથી.
