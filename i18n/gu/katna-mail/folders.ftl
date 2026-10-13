# Katna Mail, Gujarati (ગુજરાતી).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = લેબલ
nav-folders = ફોલ્ડર
nav-label-new = નવું લેબલ બનાવો
nav-folder-new = નવું ફોલ્ડર બનાવો
nav-menu-check-mail = નવા મેઇલ તપાસો
nav-menu-check-inbox = આ ઇનબૉક્સ તપાસો
nav-unified-leave-out = એકીકૃત ઇનબૉક્સમાંથી બાકાત રાખો
nav-unified-bring-back = એકીકૃત ઇનબૉક્સમાં પાછું લાવો
nav-menu-sign-in-again = ફરી સાઇન ઇન કરો
nav-menu-new-mail = આ એકાઉન્ટમાંથી નવો મેઇલ
nav-menu-account-settings = એકાઉન્ટ સેટિંગ
nav-account-checked = સિંકમાં · { $ago } તપાસ્યું
nav-account-in-sync = સિંકમાં
nav-account-connecting = કનેક્ટ થઈ રહ્યું છે…
nav-account-offline = ઑફલાઇન, ફરી પ્રયાસ કરી રહ્યાં છીએ
nav-account-signed-out = { $provider } સાઇન ઇનની મુદત પૂરી થઈ
nav-account-password-refused = પાસવર્ડ નકાર્યો
nav-account-storage = { $total } માંથી { $used } વપરાયું
nav-menu-new-subfolder = અંદર નવું ફોલ્ડર
nav-menu-new-sublabel = અંદર નવું લેબલ
nav-menu-rename = નામ બદલો
nav-menu-delete = ડિલીટ કરો
nav-menu-empty-trash = કચરાપેટી ખાલી કરો
nav-account-unnamed = એકાઉન્ટ { $number }
nav-all-accounts = બધાં એકાઉન્ટ
nav-expand = ફોલ્ડર બતાવો
nav-collapse = ફોલ્ડર છુપાવો
storage-used = { $total }માંથી { $percent }% વપરાયું
storage-used-detail = { $address }: { $total }માંથી { $used } વપરાયું

## Special folders (the user's own folders keep their names)

folder-inbox = ઇનબૉક્સ
folder-starred = તારાંકિત
folder-snoozed = સ્નૂઝ કરેલા
folder-unread = નહીં વાંચેલા
folder-important = મહત્ત્વપૂર્ણ
folder-drafts = ડ્રાફ્ટ
folder-sent = મોકલેલા
folder-archive = આર્કાઇવ
folder-spam = સ્પામ
folder-trash = કચરાપેટી
folder-all-mail = બધા મેઇલ
folder-scheduled = શેડ્યૂલ કરેલા
folder-waiting = જવાબની રાહમાં
folder-waiting-short = રાહમાં
folder-reminders = રિમાઇન્ડર
folder-outbox = આઉટબૉક્સ
folder-activity = પ્રવૃત્તિ
folder-not-on-account = આ એકાઉન્ટમાં આવું કોઈ ફોલ્ડર નથી.

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = નવું લેબલ
label-folder-new-title = નવું ફોલ્ડર
label-prompt = કૃપા કરીને નવા લેબલનું નામ દાખલ કરો:
label-folder-prompt = કૃપા કરીને નવા ફોલ્ડરનું નામ દાખલ કરો:
label-name-hint = લેબલનું નામ
label-folder-name-hint = ફોલ્ડરનું નામ
label-nest = લેબલને આની અંદર રાખો:
label-folder-nest = ફોલ્ડરને આની અંદર રાખો:
label-cancel = રદ કરો
label-create = બનાવો
label-creating = બનાવી રહ્યાં છીએ…
label-created = “{ $name }” લેબલ બનાવ્યું.
label-folder-created = “{ $name }” ફોલ્ડર બનાવ્યું.
label-rename-title = લેબલનું નામ બદલો
label-folder-rename-title = ફોલ્ડરનું નામ બદલો
label-rename = નામ બદલો
label-renaming = નામ બદલી રહ્યાં છીએ…
label-renamed = લેબલનું નામ બદલીને “{ $name }” કર્યું.
label-folder-renamed = ફોલ્ડરનું નામ બદલીને “{ $name }” કર્યું.

## Deleting a folder or label (asked first)

folder-delete-title = “{ $name }” ડિલીટ કરવું છે?
folder-delete-body = { $count ->
    [0] તેમાં કોઈ મેઇલ નથી. ફોલ્ડર સર્વર પરથી દૂર થાય છે, તેથી વેબમેઇલ અને તમારા ફોનમાંથી પણ તે જતું રહેશે.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] તેનો { $count } વાર્તાલાપ કચરાપેટીમાં જાય છે, તેથી તમે તેને હજી પાછો મેળવી શકો છો.
           *[other] તેના { $count } વાર્તાલાપ કચરાપેટીમાં જાય છે, તેથી તમે તેમને હજી પાછા મેળવી શકો છો.
        }
       *[message] { $count ->
            [one] તેનો { $count } મેસેજ કચરાપેટીમાં જાય છે, તેથી તમે તેને હજી પાછો મેળવી શકો છો.
           *[other] તેના { $count } મેસેજ કચરાપેટીમાં જાય છે, તેથી તમે તેમને હજી પાછા મેળવી શકો છો.
        }
    } ફોલ્ડર સર્વર પરથી દૂર થાય છે, તેથી વેબમેઇલ અને તમારા ફોનમાંથી પણ તે જતું રહેશે.
}
folder-delete-forever-body = { $count ->
    [0] તેમાં કોઈ મેઇલ નથી. ફોલ્ડર સર્વર પરથી દૂર થાય છે, તેથી વેબમેઇલ અને તમારા ફોનમાંથી પણ તે જતું રહેશે.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] તેનો { $count } વાર્તાલાપ કાયમ માટે ડિલીટ થાય છે; આ એકાઉન્ટમાં કચરાપેટી નથી.
           *[other] તેના { $count } વાર્તાલાપ કાયમ માટે ડિલીટ થાય છે; આ એકાઉન્ટમાં કચરાપેટી નથી.
        }
       *[message] { $count ->
            [one] તેનો { $count } મેસેજ કાયમ માટે ડિલીટ થાય છે; આ એકાઉન્ટમાં કચરાપેટી નથી.
           *[other] તેના { $count } મેસેજ કાયમ માટે ડિલીટ થાય છે; આ એકાઉન્ટમાં કચરાપેટી નથી.
        }
    } ફોલ્ડર સર્વર પરથી દૂર થાય છે, તેથી વેબમેઇલ અને તમારા ફોનમાંથી પણ તે જતું રહેશે.
}
folder-delete-label-body = લેબલ દૂર થાય છે. તેના મેઇલ બધા મેઇલમાં અને તેનાં અન્ય લેબલમાં રહે છે.
folder-delete-confirm = ફોલ્ડર ડિલીટ કરો
folder-delete-label-confirm = લેબલ ડિલીટ કરો
folder-deleted = “{ $name }” ફોલ્ડર ડિલીટ કર્યું
label-deleted = “{ $name }” લેબલ ડિલીટ કર્યું
