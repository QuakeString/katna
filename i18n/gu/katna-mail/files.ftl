# Katna Mail, Gujarati (ગુજરાતી).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = ફાઇલો શોધો

## Left side (and chips on a phone)

files-all = બધી ફાઇલો
files-pictures = ચિત્રો
files-pdfs = PDF
files-documents = દસ્તાવેજો
files-sheets = સ્પ્રેડશીટ
files-slides = સ્લાઇડ
files-other = અન્ય
files-accounts = એકાઉન્ટ
files-drives = ડ્રાઇવ
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = મારી સાથે શેર કરેલી
files-shown = બતાવેલી
files-received = મળેલી
files-sent = મેં મોકલેલી

## Over the files

files-count = { $count ->
    [one] { $count } ફાઇલ · { $size }
   *[other] { $count } ફાઇલો · { $size }
}
files-anyone = કોઈપણ
files-from-person = { $name } તરફથી
files-time-any = કોઈપણ સમય
files-time-today = આજે
files-time-yesterday = ગઈકાલે
files-time-this-week = આ અઠવાડિયે
files-time-last-week = ગયા અઠવાડિયે
files-time-this-month = આ મહિને
files-time-last-month = ગયા મહિને
files-time-between = { $first } – { $last }
files-time-hint = કોઈ દિવસ પર ક્લિક કરો, અથવા દિવસો પર ખેંચો
files-time-summary = { $count ->
    [one] { $days } · { $count } ફાઇલ
   *[other] { $days } · { $count } ફાઇલો
}
files-time-clear = સાફ કરો
files-time-month-back = પાછલો મહિનો
files-time-month-on = આગલો મહિનો
files-time-wheel = આ તારીખો ખસેડવા સ્ક્રોલ કરો, તેમની લંબાઈ એ જ રહેશે
files-sort-newest = સૌથી નવી પહેલાં
files-sort-oldest = સૌથી જૂની પહેલાં
files-sort-largest = સૌથી મોટી પહેલાં
files-sort-name = નામ મુજબ
files-grid = કાર્ડ
files-list = સૂચિ
files-this-week = આ અઠવાડિયે
files-undated = તારીખ નથી
files-me = હું
files-no-subject = (કોઈ વિષય નથી)
files-loading = તમારી મેઇલમાંથી ફાઇલો એકઠી થઈ રહી છે…
files-empty = તમારી મેઇલની ફાઇલો અહીં દેખાય છે.
files-none-match = કોઈ ફાઇલ મેળ ખાતી નથી.
files-load-failed = ફાઇલો વાંચી શકાઈ નહીં: { $error }

## A file's menu and buttons

files-open = ખોલો
files-open-with = આના વડે ખોલો…
files-save = સેવ કરો…
files-show-mail = મેઇલ બતાવો
files-mail-window = મેઇલ નવી વિન્ડોમાં ખોલો
files-forward = ફાઇલ ફૉરવર્ડ કરો
files-from-them = { $name } તરફથી ફાઇલો
files-copy-name = ફાઇલનું નામ કૉપિ કરો
files-name-copied = ફાઇલનું નામ કૉપિ કર્યું
files-downloading = મેઇલ ડાઉનલોડ થઈ રહી છે…
files-download-failed = આ મેઇલ ડાઉનલોડ કરી શકાઈ નહીં.

## A cloud drive in place of the mail files

files-drive-mine = મારી ડ્રાઇવ
files-drive-mine-onedrive = મારી ફાઇલો
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 ફાઇલ
       *[other] { $files } ફાઇલો
    }
    [one] 1 ફોલ્ડર · { $files ->
        [one] 1 ફાઇલ
       *[other] { $files } ફાઇલો
    }
   *[other] { $folders } ફોલ્ડર · { $files ->
        [one] 1 ફાઇલ
       *[other] { $files } ફાઇલો
    }
}
files-drive-folders = ફોલ્ડર
files-drive-files = ફાઇલો
files-drive-folder = ફોલ્ડર
files-drive-meta = { $what } · { $date } એ બદલાયું
files-drive-as-link = { $what } · લિંક તરીકે
files-drive-google-doc = Google Doc
files-drive-google-sheet = Google Sheet
files-drive-google-slides = Google Slides
files-drive-google-drawing = Google Drawing
files-drive-fetching = મેળવી રહ્યા છીએ…
files-drive-loading = ડ્રાઇવ ખૂલી રહી છે…
files-drive-empty = આ ફોલ્ડર ખાલી છે.
files-drive-unreachable = { $drive } સુધી પહોંચી શકાતું નથી.
files-drive-try-again = ફરી પ્રયાસ કરો
files-drive-needs-permission = આ ડ્રાઇવ બતાવવા Katna ને એક વાર તમારી મંજૂરી જોઈએ છે. ફરી સાઇન ઇન કરો અને Katna ને તમારી ફાઇલો જોવાની મંજૂરી આપો.
files-drive-allow = મંજૂરી આપો
files-drive-allow-failed = સાઇન ઇન પૂરું થયું નહીં, તેથી ડ્રાઇવ બંધ રહે છે.
files-drive-attach = જોડો
files-drive-more = વધુ
files-drive-download = ડાઉનલોડ કરો…
files-drive-open-web = { $drive } માં ખોલો
files-drive-copy-link = લિંક કૉપિ કરો
files-drive-link-copied = લિંક કૉપિ કરી
files-drive-share = શેર કરો…
files-drive-rename = નામ બદલો
files-drive-trash = કચરાપેટીમાં ખસેડો
files-drive-trashed = “{ $name }” { $drive } ની કચરાપેટીમાં છે
files-drive-renamed = નામ બદલીને “{ $name }” કર્યું
files-drive-getting = { $drive } માંથી { $name } મેળવી રહ્યા છીએ…
files-drive-get-failed = { $name } મેળવી શકાઈ નહીં: { $error }
files-drive-upload = અપલોડ કરો
files-drive-upload-files = ફાઇલો અપલોડ કરો
files-drive-upload-folder = ફોલ્ડર અપલોડ કરો
files-drive-upload-failed = { $name } અપલોડ કરી શકાઈ નહીં: { $error }
files-drive-upload-needs = અપલોડ કરવા માટે Katna ને એક વાર તમારી મંજૂરી જોઈએ છે: સેટિંગ › ડિફૉલ્ટ ઍપ › ફાઇલો પેજમાં મંજૂરી આપો દબાવો.

## The Share dialog of a drive file or folder

files-share-title = “{ $name }” શેર કરો
files-share-add = નામ કે સરનામાથી લોકો ઉમેરો
files-share-not-address = “{ $text }” ઇમેઇલ સરનામું નથી
files-share-notify = { $drive } ને પણ તેમને ઇમેઇલ કરવા દો
files-share-people = ઍક્સેસ ધરાવતા લોકો
files-share-general = સામાન્ય ઍક્સેસ
files-share-loading = કોની પાસે ઍક્સેસ છે તે વાંચી રહ્યા છીએ…
files-share-restricted = પ્રતિબંધિત
files-share-restricted-about = ફક્ત ઍક્સેસ ધરાવતા લોકો જ લિંક વડે તેને ખોલી શકે છે
files-share-anyone = લિંક ધરાવનાર કોઈપણ
files-share-anyone-can = { $role ->
    [editor] લિંક ધરાવનાર કોઈપણ ફેરફાર કરી શકે છે
    [commenter] લિંક ધરાવનાર કોઈપણ ટિપ્પણી કરી શકે છે
   *[viewer] લિંક ધરાવનાર કોઈપણ જોઈ શકે છે
}
files-share-anyone-about = { $role ->
    [editor] ઇન્ટરનેટ પર લિંક ધરાવનાર કોઈપણ ફેરફાર કરી શકે છે
    [commenter] ઇન્ટરનેટ પર લિંક ધરાવનાર કોઈપણ ટિપ્પણી કરી શકે છે
   *[viewer] ઇન્ટરનેટ પર લિંક ધરાવનાર કોઈપણ જોઈ શકે છે
}
files-share-role-owner = માલિક
files-share-role-editor = સંપાદક
files-share-role-commenter = ટિપ્પણીકર્તા
files-share-role-viewer = દર્શક
files-share-you = { $name } (તમે)
files-share-domain = { $domain } પરના બધા
files-share-inherited = જે ફોલ્ડરમાં છે તેમાંથી મળેલી ઍક્સેસ
files-share-remove = ઍક્સેસ દૂર કરો
files-share-copy-link = લિંક કૉપિ કરો
files-share-share = શેર કરો
files-share-done = થઈ ગયું
files-share-close = બંધ કરો
files-share-sharing = શેર થઈ રહ્યું છે…
files-share-shared = { $count ->
    [one] 1 વ્યક્તિ સાથે શેર કર્યું
   *[other] { $count } લોકો સાથે શેર કર્યું
}
files-share-refused = { $drive } { $addresses } સાથે શેર કરી શક્યું નહીં
files-share-failed = શેરિંગ બદલી શકાયું નહીં: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] 1 આઇટમ અપલોડ થઈ રહી છે
   *[other] { $count } આઇટમ અપલોડ થઈ રહી છે
}
files-tray-done = { $count ->
    [one] 1 અપલોડ પૂર્ણ
   *[other] { $count } અપલોડ પૂર્ણ
}
files-tray-some-failed = { $done } અપલોડ થઈ, { $failed } નિષ્ફળ
files-tray-minutes-left = { $minutes ->
    [one] લગભગ એક મિનિટ બાકી
   *[other] લગભગ { $minutes } મિનિટ બાકી
}
files-tray-seconds-left = એક મિનિટથી ઓછો સમય બાકી
files-tray-starting = શરૂ થઈ રહ્યું છે…
files-tray-cancel-all = બધું રદ કરો
files-tray-cancel = રદ કરો
files-tray-fold = સૂચિ છુપાવો
files-tray-unfold = સૂચિ બતાવો
files-tray-close = બંધ કરો
files-tray-progress = { $place } · { $size } માંથી { $sent }
files-tray-in = { $place } માં
files-tray-cancelled = રદ કર્યું
