# Katna Mail, Gujarati (ગુજરાતી): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = નવું કાર્ય
tasks-all = બધાં કાર્યો
tasks-today = આજે
tasks-upcoming = આવનારાં
tasks-starred = તારાંકિત
tasks-completed-view = પૂર્ણ
tasks-new-list = નવી સૂચિ બનાવો
tasks-labels-heading = લેબલ
tasks-on-this-computer = આ કમ્પ્યુટર પર
tasks-my-tasks = મારાં કાર્યો
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = કાર્યો બતાવવા માટે ફરી સાઇન ઇન કરો
tasks-account-signed-in = { $address } માં ફરી સાઇન ઇન થયું. તમારાં કાર્યો મેળવી રહ્યાં છીએ…
tasks-account-sign-in-refused = { $provider } એ Katna ને અંદર આવવા ન દીધું. ફરી પ્રયાસ કરો, અને તમારાં કાર્યોની ઍક્સેસની મંજૂરી આપો.
tasks-account-refused = સર્વરે પાસવર્ડ સ્વીકાર્યો નહીં. Yahoo, iCloud, Zoho અને અન્યને ઍપ પાસવર્ડની જરૂર છે.
tasks-account-change-password = પાસવર્ડ બદલો
tasks-account-change-password-tooltip = નવો પાસવર્ડ લખો; Katna તેને સર્વર સાથે તપાસે છે
tasks-account-not-enabled = Katna માટે કાર્ય ઍક્સેસ હજી ચાલુ નથી.
tasks-account-failed = કાર્ય સૂચિઓ વાંચી શકાઈ નથી.
# $reason is the server's own words, in English.
tasks-account-error = કાર્ય સૂચિઓ વાંચી શકાઈ નથી: { $reason }
tasks-account-none = કોઈ કાર્ય સૂચિ મળી નથી
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = કોઈ કાર્ય સૂચિ મળી નથી: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } કાર્યો ફક્ત { $provider } વડે સાઇન ઇન થયેલા Katna ને જ બતાવે છે.
tasks-account-sign-in-with = { $provider } વડે સાઇન ઇન કરો
tasks-account-looking = કાર્ય સૂચિઓ શોધી રહ્યાં છીએ…
tasks-account-try-again = ફરી પ્રયાસ કરો
tasks-account-try-again-tooltip = આ એકાઉન્ટનાં કાર્યો હમણાં ફરી તપાસો
tasks-account-fixing = કામ ચાલુ છે…
tasks-list-name-placeholder = સૂચિનું નામ

## Lists and tasks

tasks-loading = તમારાં કાર્યો વાંચી રહ્યા છીએ…
tasks-no-lists = તમારી કાર્ય સૂચિઓ અહીં દેખાશે.
tasks-search = કાર્યો શોધો
tasks-search-none = તમારી શોધ સાથે કોઈ કાર્ય મેળ ખાતું નથી.
tasks-add = કાર્ય ઉમેરો
tasks-title-placeholder = શીર્ષક
tasks-add-step = પેટા કાર્ય ઉમેરો
tasks-empty = હજી કોઈ કાર્ય નથી. ઉપર એક ઉમેરો.
tasks-starred-empty = અહીં જોવા માટે કાર્ય પર તારો ઉમેરો.
tasks-label-empty = આ લેબલવાળું કોઈ ખુલ્લું કાર્ય નથી.
tasks-today-empty = આજે કંઈ બાકી નથી.
tasks-completed-empty = તમે પૂર્ણ કરેલાં કાર્યો અહીં દેખાય છે.
tasks-upcoming-add = { $day } માટે કાર્ય ઉમેરો
tasks-upcoming-overdue-day = { $weekday } { $day }
tasks-from-mail-quiet = મેઇલમાંથી
tasks-from-note-quiet = નોંધમાંથી
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }, { $day }
tasks-overdue = મુદતવીતી
tasks-completed = { $count ->
    [one] પૂર્ણ થયેલાં ({ $count })
   *[other] પૂર્ણ થયેલાં ({ $count })
}
tasks-list-options = સૂચિ વિકલ્પો
tasks-sort-by = આ મુજબ ગોઠવો
tasks-sort-my-order = મારો ક્રમ
tasks-sort-date = તારીખ
tasks-sort-starred = તાજેતરમાં તારાંકિત
tasks-sort-title = શીર્ષક
tasks-rename-list = સૂચિનું નામ બદલો
tasks-delete-list = સૂચિ ડિલીટ કરો
tasks-mark-done = પૂર્ણ તરીકે ચિહ્નિત કરો
tasks-mark-open = અપૂર્ણ તરીકે ચિહ્નિત કરો
tasks-star = તારો ઉમેરો
tasks-unstar = તારો કાઢી નાખો
tasks-edit-title = શીર્ષક સંપાદિત કરો
tasks-details = વિગતો
tasks-delete = ડિલીટ કરો
tasks-move-to = { $list } માં ખસેડો
tasks-from-mail = મેઇલ
tasks-open-mail = મેઇલ ખોલો
tasks-from-note = નોંધ
tasks-open-note = નોંધ ખોલો
tasks-note-gone = તે નોંધ હવે અહીં નથી.
tasks-no-subject = (કોઈ વિષય નથી)

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
    [one] { $count } પસંદ કરેલ
   *[other] { $count } પસંદ કરેલ
}
tasks-select-clear = પસંદગી સાફ કરો
tasks-select-move = સૂચિમાં ખસેડો
tasks-select-date = તારીખ સેટ કરો
tasks-next-week = આવતા અઠવાડિયે

## The details dialog

tasks-notes-placeholder = વિગતો ઉમેરો
tasks-date = તારીખ
tasks-no-date = કોઈ તારીખ નથી
tasks-time-placeholder = સમય ઉમેરો
tasks-repeat = પુનરાવર્તન
tasks-repeat-never = પુનરાવર્તિત થતું નથી
tasks-repeat-daily = દરરોજ
tasks-repeat-weekly = દર અઠવાડિયે
tasks-repeat-monthly = દર મહિને
tasks-repeat-yearly = દર વર્ષે
tasks-repeat-other = કસ્ટમ
tasks-remind = મને યાદ અપાવો
tasks-remind-off = યાદ ન અપાવશો
tasks-remind-on-time = તે જ સમયે
tasks-remind-morning = તે દિવસે, { $time }
tasks-remind-hour-before = એક કલાક પહેલાં
tasks-remind-day-before = એક દિવસ પહેલાં
tasks-label-add = લેબલ ઉમેરો
tasks-label-task = કાર્યને લેબલ લગાવો
tasks-files-attach = ફાઇલો જોડો
tasks-files-pick = જોડો
tasks-file-open = ખોલો
tasks-file-remove = ફાઇલ દૂર કરો
tasks-file-here = ફક્ત આ કમ્પ્યુટર પર
tasks-cancel = રદ કરો
tasks-save = સેવ કરો
tasks-not-a-time = “{ $text }” સમય નથી, ઉદાહરણ તરીકે { $example }.

## Due days

tasks-due-today = આજે
tasks-due-tomorrow = આવતીકાલે
tasks-due-yesterday = ગઈકાલે
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = કાર્ય પૂર્ણ થયું
tasks-toast-next = થઈ ગયું. આગલું { $date } એ
tasks-toast-deleted = કાર્ય ડિલીટ થયું
tasks-files-added = { $count ->
    [one] ફાઇલ જોડી
   *[other] { $count } ફાઇલો જોડી
}
tasks-file-removed = “{ $name }” દૂર કરી
tasks-files-left-out = જોડાઈ નથી: { $names }. કાર્યમાં { $limit } સુધીની ફાઇલો જોડી શકાય છે, ફોલ્ડર નહીં.
tasks-file-missing = તે ફાઇલ હવે અહીં નથી.
tasks-toast-added = { $count ->
    [one] કાર્યોમાં ઉમેર્યું
   *[other] { $count } કાર્યો ઉમેર્યાં
}
tasks-mail-gone = તે મેઇલ હવે અહીં નથી.
tasks-toast-list-deleted = સૂચિ ડિલીટ થઈ
tasks-toast-moved = { $list } માં ખસેડ્યું
# A task dragged to another place in its own list.
tasks-toast-placed = કાર્ય ખસેડ્યું
tasks-toast-rescheduled = કાર્યનો સમય બદલ્યો
tasks-toast-rescheduled-several = { $count ->
    [one] કાર્યનો સમય બદલ્યો
   *[other] { $count } કાર્યોનો સમય બદલ્યો
}
tasks-toast-done-several = { $count ->
    [one] કાર્ય પૂર્ણ કર્યું
   *[other] { $count } કાર્યો પૂર્ણ કર્યાં
}
tasks-toast-open-several = { $count ->
    [one] કાર્ય અપૂર્ણ તરીકે ચિહ્નિત કર્યું
   *[other] { $count } કાર્યો અપૂર્ણ તરીકે ચિહ્નિત કર્યાં
}
tasks-toast-starred = { $count ->
    [one] કાર્યને તારો આપ્યો
   *[other] { $count } કાર્યોને તારો આપ્યો
}
tasks-toast-unstarred = { $count ->
    [one] તારો દૂર કર્યો
   *[other] { $count } કાર્યોમાંથી તારા દૂર કર્યા
}
tasks-toast-deleted-several = { $count ->
    [one] કાર્ય ડિલીટ કર્યું
   *[other] { $count } કાર્યો ડિલીટ કર્યાં
}
