# Katna Mail, Gujarati (ગુજરાતી).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = ભાષા: { $language }
language-tooltip-system = ભાષા: { $language }, સિસ્ટમ મુજબ
language-search = ભાષા શોધો
language-system-default = સિસ્ટમ ડિફૉલ્ટ
language-system-now = હાલમાં { $language }
language-no-match = “{ $query }” સાથે મેળ ખાતી કોઈ ભાષા નથી
language-machine = મશીન દ્વારા અનુવાદિત. તેને બહેતર બનાવવામાં સહાય કરો
language-setting = ભાષા
language-setting-detail = મેનૂ, બટન અને મેસેજની ભાષા તેમજ તારીખો અને સંખ્યાઓનું ફૉર્મેટ. સિસ્ટમ ડિફૉલ્ટ ડેસ્કટૉપના સેટિંગને અનુસરે છે.

## Dates and sizes

ago-just-now = હમણાં જ
ago-minutes = { $count ->
    [one] { $count } મિનિટ પહેલાં
   *[other] { $count } મિનિટ પહેલાં
}
ago-hours = { $count ->
    [one] { $count } કલાક પહેલાં
   *[other] { $count } કલાક પહેલાં
}
ago-days = { $count ->
    [one] { $count } દિવસ પહેલાં
   *[other] { $count } દિવસ પહેલાં
}
size-bytes = { $count ->
    [one] { $count } બાઇટ
   *[other] { $count } બાઇટ
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = ફોલ્ડર છુપાવો
folders-show = ફોલ્ડર બતાવો
compose = લખો
search = શોધો
search-mail = મેઇલ શોધો
search-settings = સેટિંગ શોધો
search-clear = શોધ સાફ કરો
search-options-show = શોધના વિકલ્પો બતાવો
settings = સેટિંગ
account-add = એકાઉન્ટ ઉમેરો

## App rail (and the bottom bar on a phone)

rail-mail = મેઇલ
rail-calendar = કૅલેન્ડર
rail-contacts = સંપર્કો
rail-tasks = કાર્યો
rail-notes = નોંધો
rail-feeds = ફીડ

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = ટૂંક સમયમાં આવી રહ્યું છે
app-calendar-promise = તમારા CalDAV કૅલેન્ડર, મેઇલમાં આવેલાં મીટિંગનાં આમંત્રણો અને રિમાઇન્ડર, તમારા ઇનબૉક્સની બાજુમાં જ.
app-tasks-promise = CalDAV સાથે સિંક થતી કરવાનાં કામોની સૂચિઓ, અને મેઇલમાંથી બનાવેલાં કાર્યો.
app-notes-promise = ઝડપી નોંધો, અને પછી માટે કોઈ મેઇલ કે વાર્તાલાપ પર નોંધો.
app-feeds-promise = તમારા મેઇલની બાજુમાં જ RSS અને Atom ફીડ વાંચો.

## Contacts page

app-contacts-loading = તમારા મેઇલમાંથી લોકોને એકઠા કરી રહ્યાં છીએ…
app-contacts-empty = તમે જેમની સાથે મેઇલની આપ-લે કરો છો તે લોકો અહીં દેખાશે.
app-contacts-count = { $count ->
    [one] તમારા મેઇલમાંથી { $count } વ્યક્તિ, સૌથી વધુ સંપર્ક ધરાવનાર પહેલાં
   *[other] તમારા મેઇલમાંથી { $count } લોકો, સૌથી વધુ સંપર્ક ધરાવનાર પહેલાં
}
app-contacts-top = { $count ->
    [one] તમારા મેઇલમાંથી ટોચની { $count } વ્યક્તિ, સૌથી વધુ સંપર્ક ધરાવનાર પહેલાં
   *[other] તમારા મેઇલમાંથી ટોચના { $count } લોકો, સૌથી વધુ સંપર્ક ધરાવનાર પહેલાં
}
app-contacts-messages = { $count ->
    [one] { $count } મેસેજ
   *[other] { $count } મેસેજ
}
app-contacts-last = છેલ્લે { $date }

## Navigation (the folders pane)

nav-labels = લેબલ
nav-folders = ફોલ્ડર
nav-label-new = નવું લેબલ બનાવો
nav-folder-new = નવું ફોલ્ડર બનાવો
nav-account-unnamed = એકાઉન્ટ { $number }
nav-tab-new = { $count ->
    [one] { $count } નવો
   *[other] { $count } નવા
}

## Special folders (the user's own folders keep their names)

folder-inbox = ઇનબૉક્સ
folder-starred = તારાંકિત
folder-drafts = ડ્રાફ્ટ
folder-sent = મોકલેલા
folder-archive = આર્કાઇવ
folder-spam = સ્પામ
folder-trash = કચરાપેટી
folder-all-mail = બધા મેઇલ
folder-scheduled = શેડ્યૂલ કરેલા

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

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = પ્રાથમિક
tab-promotions = પ્રમોશન
tab-social = સામાજિક
tab-updates = અપડેટ
tab-forums = ફોરમ
tab-focused = ફોકસ્ડ
tab-other = અન્ય
tab-inbox = ઇનબૉક્સ
tab-newsletters = ન્યૂઝલેટર
tab-notifications = નોટિફિકેશન
tab-new = { $count } નવા
tab-provider-other = Katna દ્વારા ગોઠવેલા

## Mail list: toolbar

list-select = પસંદ કરો
list-refresh = રિફ્રેશ કરો
list-more = વધુ
list-mark-read = વાંચેલા તરીકે ચિહ્નિત કરો
list-mark-unread = નહીં વાંચેલા તરીકે ચિહ્નિત કરો
list-move-to = આમાં ખસેડો
list-archive = આર્કાઇવ કરો
list-spam = સ્પામની જાણ કરો
list-delete = ડિલીટ કરો
list-newer = નવા
list-older = જૂના
list-range = { $total } માંથી { $first }–{ $last }
list-range-about = આશરે { $total } માંથી { $first }–{ $last }
list-results = “{ $query }” માટેનાં પરિણામો
list-results-corrected = “{ $query }” માટેનાં પરિણામો બતાવી રહ્યાં છીએ
list-search-instead = તેના બદલે “{ $query }” શોધો
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = બધા
list-pick-none = એકપણ નહીં
list-pick-read = વાંચેલા
list-pick-unread = નહીં વાંચેલા
list-pick-starred = તારાંકિત
list-pick-unstarred = તારાંકિત નહીં

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } વાર્તાલાપ પસંદ કરેલ છે.
       *[other] બધા { $count } વાર્તાલાપ પસંદ કરેલા છે.
    }
   *[message] { $count ->
        [one] { $count } મેસેજ પસંદ કરેલ છે.
       *[other] બધા { $count } મેસેજ પસંદ કરેલા છે.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } માંનો { $count } વાર્તાલાપ પસંદ કરેલ છે.
       *[other] { $folder } માંના બધા { $count } વાર્તાલાપ પસંદ કરેલા છે.
    }
   *[message] { $count ->
        [one] { $folder } માંનો { $count } મેસેજ પસંદ કરેલ છે.
       *[other] { $folder } માંના બધા { $count } મેસેજ પસંદ કરેલા છે.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] સ્ક્રીન પરનો { $count } વાર્તાલાપ પસંદ કરેલ છે.
       *[other] સ્ક્રીન પરના બધા { $count } વાર્તાલાપ પસંદ કરેલા છે.
    }
   *[message] { $count ->
        [one] સ્ક્રીન પરનો { $count } મેસેજ પસંદ કરેલ છે.
       *[other] સ્ક્રીન પરના બધા { $count } મેસેજ પસંદ કરેલા છે.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } વાર્તાલાપ પસંદ કરો
       *[other] બધા { $count } વાર્તાલાપ પસંદ કરો
    }
   *[message] { $count ->
        [one] { $count } મેસેજ પસંદ કરો
       *[other] બધા { $count } મેસેજ પસંદ કરો
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $folder } માંનો { $count } વાર્તાલાપ પસંદ કરો
       *[other] { $folder } માંના બધા { $count } વાર્તાલાપ પસંદ કરો
    }
   *[message] { $count ->
        [one] { $folder } માંનો { $count } મેસેજ પસંદ કરો
       *[other] { $folder } માંના બધા { $count } મેસેજ પસંદ કરો
    }
}
list-clear-selection = પસંદગી સાફ કરો

## Mail list: empty states

list-empty-search = તમારી શોધ સાથે કોઈ મેસેજ મેળ ખાતો નથી.
list-empty-tab = { $tab } માં કોઈ મેઇલ નથી.
list-empty-tab-unknown = આ ટૅબમાં કોઈ મેઇલ નથી.
list-empty-folder = { $folder } માં કોઈ મેસેજ નથી.
list-empty-folder-unknown = આ ફોલ્ડરમાં કોઈ મેસેજ નથી.
list-first-sync = તમારા મેઇલ મેળવી રહ્યાં છીએ…
list-first-sync-detail = મેઇલ આવશે તેમ અહીં દેખાશે.

## Mail list: lines

row-removed = આ મેસેજ કાઢી નાખવામાં આવ્યો.
row-starred = તારાંકિત
row-not-starred = તારાંકિત નથી
row-important = મહત્ત્વપૂર્ણ. મહત્ત્વપૂર્ણ નથી તરીકે ચિહ્નિત કરવા ક્લિક કરો.
row-mark-important = મહત્ત્વપૂર્ણ તરીકે ચિહ્નિત કરો
row-pinned = સૌથી ઉપર પિન કરેલ
row-pin = સૌથી ઉપર પિન કરો
row-unpin = અનપિન કરો

## Mail list: More menu and right-click menu

menu-reply = જવાબ આપો
menu-reply-all = બધાને જવાબ આપો
menu-forward = ફૉરવર્ડ કરો
menu-archive = આર્કાઇવ કરો
menu-delete = ડિલીટ કરો
menu-spam = સ્પામની જાણ કરો
menu-mark-read = વાંચેલા તરીકે ચિહ્નિત કરો
menu-mark-unread = નહીં વાંચેલા તરીકે ચિહ્નિત કરો
menu-mark-all-read = બધાને વાંચેલા તરીકે ચિહ્નિત કરો
menu-star = તારો ઉમેરો
menu-unstar = તારો કાઢી નાખો
menu-important = મહત્ત્વપૂર્ણ તરીકે ચિહ્નિત કરો
menu-not-important = મહત્ત્વપૂર્ણ નથી તરીકે ચિહ્નિત કરો
menu-pin = સૌથી ઉપર પિન કરો
menu-unpin = અનપિન કરો
menu-print-all = બધું પ્રિન્ટ કરો
menu-new-window = નવી વિન્ડોમાં ખોલો
menu-move-to = આમાં ખસેડો
menu-move-to-heading = આમાં ખસેડો:
menu-find-from = { $name } તરફથી આવેલા ઇમેઇલ શોધો

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] વાર્તાલાપ આર્કાઇવ કર્યો.
       *[other] { $count } વાર્તાલાપ આર્કાઇવ કર્યા.
    }
   *[message] { $count ->
        [one] મેસેજ આર્કાઇવ કર્યો.
       *[other] { $count } મેસેજ આર્કાઇવ કર્યા.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] વાર્તાલાપ કચરાપેટીમાં ખસેડ્યો.
       *[other] { $count } વાર્તાલાપ કચરાપેટીમાં ખસેડ્યા.
    }
   *[message] { $count ->
        [one] મેસેજ કચરાપેટીમાં ખસેડ્યો.
       *[other] { $count } મેસેજ કચરાપેટીમાં ખસેડ્યા.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] વાર્તાલાપ ખસેડ્યો.
       *[other] { $count } વાર્તાલાપ ખસેડ્યા.
    }
   *[message] { $count ->
        [one] મેસેજ ખસેડ્યો.
       *[other] { $count } મેસેજ ખસેડ્યા.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] વાર્તાલાપ તારાંકિત કર્યો.
       *[other] { $count } વાર્તાલાપ તારાંકિત કર્યા.
    }
   *[message] { $count ->
        [one] મેસેજ તારાંકિત કર્યો.
       *[other] { $count } મેસેજ તારાંકિત કર્યા.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] વાર્તાલાપ પરથી તારો કાઢી નાખ્યો.
       *[other] { $count } વાર્તાલાપ પરથી તારો કાઢી નાખ્યો.
    }
   *[message] { $count ->
        [one] મેસેજ પરથી તારો કાઢી નાખ્યો.
       *[other] { $count } મેસેજ પરથી તારો કાઢી નાખ્યો.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] વાર્તાલાપને મહત્ત્વપૂર્ણ તરીકે ચિહ્નિત કર્યો.
       *[other] { $count } વાર્તાલાપને મહત્ત્વપૂર્ણ તરીકે ચિહ્નિત કર્યા.
    }
   *[message] { $count ->
        [one] મેસેજને મહત્ત્વપૂર્ણ તરીકે ચિહ્નિત કર્યો.
       *[other] { $count } મેસેજને મહત્ત્વપૂર્ણ તરીકે ચિહ્નિત કર્યા.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] વાર્તાલાપને મહત્ત્વપૂર્ણ નથી તરીકે ચિહ્નિત કર્યો.
       *[other] { $count } વાર્તાલાપને મહત્ત્વપૂર્ણ નથી તરીકે ચિહ્નિત કર્યા.
    }
   *[message] { $count ->
        [one] મેસેજને મહત્ત્વપૂર્ણ નથી તરીકે ચિહ્નિત કર્યો.
       *[other] { $count } મેસેજને મહત્ત્વપૂર્ણ નથી તરીકે ચિહ્નિત કર્યા.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] વાર્તાલાપ સૌથી ઉપર પિન કર્યો.
       *[other] { $count } વાર્તાલાપ સૌથી ઉપર પિન કર્યા.
    }
   *[message] { $count ->
        [one] મેસેજ સૌથી ઉપર પિન કર્યો.
       *[other] { $count } મેસેજ સૌથી ઉપર પિન કર્યા.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] વાર્તાલાપ અનપિન કર્યો.
       *[other] { $count } વાર્તાલાપ અનપિન કર્યા.
    }
   *[message] { $count ->
        [one] મેસેજ અનપિન કર્યો.
       *[other] { $count } મેસેજ અનપિન કર્યા.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] વાર્તાલાપની સ્પામ તરીકે જાણ કરી.
       *[other] { $count } વાર્તાલાપની સ્પામ તરીકે જાણ કરી.
    }
   *[message] { $count ->
        [one] મેસેજની સ્પામ તરીકે જાણ કરી.
       *[other] { $count } મેસેજની સ્પામ તરીકે જાણ કરી.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] વાર્તાલાપ કાયમ માટે ડિલીટ કર્યો.
       *[other] { $count } વાર્તાલાપ કાયમ માટે ડિલીટ કર્યા.
    }
   *[message] { $count ->
        [one] મેસેજ કાયમ માટે ડિલીટ કર્યો.
       *[other] { $count } મેસેજ કાયમ માટે ડિલીટ કર્યા.
    }
}
toast-undone = ક્રિયા પૂર્વવત્ કરી.
toast-undo = પૂર્વવત્ કરો
toast-no-spam-folder = આ એકાઉન્ટમાં કોઈ સ્પામ ફોલ્ડર નથી.

## Reading pane: toolbar

reader-close = બંધ કરો
reader-back = પાછળ
reader-mark-unread = નહીં વાંચેલા તરીકે ચિહ્નિત કરો
reader-move-to = આમાં ખસેડો
reader-more = વધુ
reader-print-all = બધું પ્રિન્ટ કરો
reader-new-window = નવી વિન્ડોમાં
reader-position = { $total } માંથી { $position }
reader-newer = નવો
reader-older = જૂનો

## Reading pane: the conversation

reader-removed = આ વાર્તાલાપ કાઢી નાખવામાં આવ્યો.
reader-no-subject = (કોઈ વિષય નથી)
reader-collapse-all = બધું સંકુચિત કરો
reader-expand-all = બધું વિસ્તૃત કરો
reader-unknown-sender = (અજાણ્યા મોકલનાર)
reader-date-ago = { $date } ({ $ago })
reader-me = મને
reader-to = પ્રતિ { $names }
reader-starred = તારાંકિત
reader-not-starred = તારાંકિત નથી
reader-too-long = મેસેજ એટલો લાંબો છે કે પૂરો બતાવી શકાતો નથી.
reader-encrypted-images = એન્ક્રિપ્ટ કરેલા મેઇલમાં વેબ પરથી છબીઓ ક્યારેય લોડ થતી નથી.
reader-window-failed = નવી વિન્ડો ખોલી શકાઈ નથી.

## Reading pane: message details (opened from "to me")

reader-details-from = મોકલનાર:
reader-details-to = પ્રતિ:
reader-details-cc = cc:
reader-details-date = તારીખ:
reader-details-subject = વિષય:

## Reading pane: downloading a message

reader-downloading = સર્વર પરથી આ મેસેજ ડાઉનલોડ કરી રહ્યાં છીએ…
reader-download-failed = આ મેસેજ ડાઉનલોડ કરી શકાયો નથી.
reader-try-again = ફરી પ્રયાસ કરો

## Reply row

reply-reply = જવાબ આપો
reply-reply-all = બધાને જવાબ આપો
reply-forward = ફૉરવર્ડ કરો

## Encrypted and signed mail

security-decrypting = ડિક્રિપ્ટ કરી રહ્યાં છીએ…
security-checking = હસ્તાક્ષર તપાસી રહ્યાં છીએ…
security-partly-encrypted = આ મેસેજનો માત્ર અમુક ભાગ એન્ક્રિપ્ટ કરેલો છે. બાકીનો ભાગ સુરક્ષાની બહાર ઉમેરાયો હતો અને કોઈ પણ વ્યક્તિએ મોકલ્યો હોઈ શકે છે.
security-partly-signed = આ મેસેજના માત્ર અમુક ભાગ પર હસ્તાક્ષર છે. બાકીનો ભાગ સુરક્ષાની બહાર ઉમેરાયો હતો અને કોઈ પણ વ્યક્તિએ મોકલ્યો હોઈ શકે છે.
security-encrypted = એન્ક્રિપ્ટ કરેલો મેસેજ
security-encrypted-smime = એન્ક્રિપ્ટ કરેલો મેસેજ (S/MIME)
security-no-key = આ મેસેજ ડિક્રિપ્ટ કરી શકાતો નથી: તે એવી કી માટે એન્ક્રિપ્ટ કરવામાં આવ્યો હતો જે તમારી પાસે નથી.
security-cancelled = ડિક્રિપ્ટ કરવાનું રદ કર્યું.
security-damaged = આ મેસેજ ડિક્રિપ્ટ કરી શકાતો નથી: એન્ક્રિપ્ટ કરેલો ડેટા બગડેલો છે અથવા બદલાયેલો છે.
security-decrypt-unavailable = આ મેસેજ ડિક્રિપ્ટ કરી શકાતો નથી: એન્ક્રિપ્ટ કરેલા મેઇલ વાંચવા માટે { $tool } ઇન્સ્ટૉલ કરો.
security-decrypt-failed = આ મેસેજ ડિક્રિપ્ટ કરી શકાતો નથી: { $reason }
security-unknown-signer = અજાણ્યા હસ્તાક્ષરકર્તા
security-signed-verified = { $signer } દ્વારા હસ્તાક્ષરિત · ચકાસાયેલ
security-signed-not-sender = { $signer } દ્વારા હસ્તાક્ષરિત, જેઓ મોકલનાર નથી
security-signed-untrusted = { $signer } દ્વારા હસ્તાક્ષરિત, એવી કી વડે જેને તમે વિશ્વસનીય નથી તરીકે ચિહ્નિત કરી છે
security-signed-unverified = { $signer } દ્વારા હસ્તાક્ષરિત · કી ચકાસાયેલ નથી
security-bad-signature = ખોટા હસ્તાક્ષર: હસ્તાક્ષર પછી આ મેસેજ બદલવામાં આવ્યો છે, અથવા હસ્તાક્ષર બનાવટી છે.
security-signature-expired = { $signer } દ્વારા હસ્તાક્ષરિત · હસ્તાક્ષરની મુદત પૂરી થઈ ગઈ છે
security-key-expired = { $signer } દ્વારા હસ્તાક્ષરિત · ત્યાર પછી કીની મુદત પૂરી થઈ ગઈ છે
security-key-revoked = { $signer } દ્વારા હસ્તાક્ષરિત, રદ કરાયેલી કી વડે
security-missing-key = તમારી પાસે ન હોય એવી કી વડે હસ્તાક્ષરિત, તેથી તપાસી શકાતું નથી
security-missing-key-id = તમારી પાસે ન હોય એવી કી ({ $key }) વડે હસ્તાક્ષરિત, તેથી તપાસી શકાતું નથી
security-signature-unavailable = હસ્તાક્ષરિત; હસ્તાક્ષર તપાસવા માટે { $tool } ઇન્સ્ટૉલ કરો
security-signature-error = હસ્તાક્ષર તપાસી શકાયા નથી.

## Remote images and pictures

remote-hidden = આ મેસેજમાંની છબીઓ છુપાવેલી છે.
remote-show = છબીઓ બતાવો
remote-always-show = આ મોકલનાર તરફથી હંમેશાં બતાવો
remote-picture-use = ઉપયોગ કરો
remote-picture-too-big = 8 MB કે તેથી નાનું ચિત્ર પસંદ કરો.
remote-picture-type = PNG, JPEG, GIF, WebP અથવા SVG ચિત્ર પસંદ કરો.
remote-picture-read-failed = ચિત્ર વાંચી શકાતું નથી: { $error }
remote-picture-keep-failed = ચિત્ર રાખી શકાતું નથી: { $error }
remote-picture-remove-failed = ચિત્ર કાઢી શકાતું નથી: { $error }

## Attachments

attachment-count = { $count ->
    [one] એક જોડાણ
   *[other] { $count } જોડાણ
}
attachment-save = સેવ કરો
attachment-save-all = બધા સેવ કરો
attachment-save-all-tooltip = બધાં જોડાણ એક ફોલ્ડરમાં સેવ કરો
attachment-save-here = અહીં સેવ કરો
attachment-not-downloaded = આ મેસેજ ડાઉનલોડ કરેલો નથી.
attachment-not-found = આ જોડાણ મેસેજમાં મળ્યું નથી.
attachment-read-failed = { $name } વાંચી શકાઈ નથી
attachment-numbered = જોડાણ { $number }
attachment-saved-all = { $count ->
    [one] { $count } ફાઇલ { $place } માં સેવ કરી
   *[other] { $count } ફાઇલો { $place } માં સેવ કરી
}
attachment-saved-some = { $total ->
    [one] { $total } માંથી { $saved } ફાઇલ { $place } માં સેવ કરી. { $failed } સેવ કરી શકાઈ નથી
   *[other] { $total } માંથી { $saved } ફાઇલો { $place } માં સેવ કરી. { $failed } સેવ કરી શકાઈ નથી
}
attachment-saved-to = { $path } માં સેવ કર્યું
attachment-save-failed = { $name } સેવ કરી શકાઈ નથી: { $error }
attachment-open-failed = { $name } ખોલી શકાઈ નથી: { $error }
attachment-risky = આ ફાઇલ કોઈ પ્રોગ્રામ ચલાવી શકે છે, તેથી Katna તેને ખોલતું નથી. તેના બદલે તેને સેવ કરો.
attachment-encrypted-open = આ ફાઇલ એન્ક્રિપ્ટ થઈને આવી હતી. તેને બીજે ક્યાંક ખોલવા માટે સેવ કરો.

## Printing

print-failed = પ્રિન્ટ કરી શકાયું નથી: { $error }
print-no-font = કોઈ ફૉન્ટ મળ્યો નથી
print-opened-as-pdf = ત્યાંથી પ્રિન્ટ કરવા માટે PDF તરીકે ખોલ્યું.
print-not-downloaded = (હજી ડાઉનલોડ થયું નથી.)
print-encrypted = (એન્ક્રિપ્ટ કરેલું. તેનું લખાણ પ્રિન્ટ કરવા માટે તેને Katna Mail માં ખોલો.)
print-to = પ્રતિ: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = આ મેસેજનાં જોડાણ વાંચવા માટે તેને ખોલો.
text-copy = કૉપિ કરો
text-select-all = બધું પસંદ કરો

## Settings page: its tabs

settings-tab-general = સામાન્ય
settings-tab-inbox = ઇનબૉક્સ
settings-tab-accounts = એકાઉન્ટ
settings-tab-subscriptions = સબ્સ્ક્રિપ્શન
settings-tab-appearance = દેખાવ
settings-tab-shortcuts = શૉર્ટકટ
settings-tab-default-apps = ડિફૉલ્ટ ઍપ
settings-tab-folders-rules = ફોલ્ડર અને નિયમો
settings-tab-compose = લખવું
settings-tab-mcp-server = MCP સર્વર
settings-tab-feedback = વપરાશકર્તા પ્રતિસાદ
settings-tab-experimental = પ્રાયોગિક

## Settings page: tabs still to come

settings-tab-subscriptions-coming = તમને મળતા ન્યૂઝલેટર અને મેઇલિંગ સૂચિઓ જુઓ, અને એક ક્લિકમાં અનસબ્સ્ક્રાઇબ કરો.
settings-tab-folders-rules-coming = ફોલ્ડર અને લેબલ બનાવો, તેમનું નામ બદલો, ખસેડો અને છુપાવો, તેમજ કયા સિંક થાય તે પસંદ કરો. નિયમો નવા મેઇલને મોકલનાર, વિષય કે શબ્દો મુજબ આપમેળે ગોઠવે છે, લેબલ લગાવે છે, ફૉરવર્ડ કરે છે અથવા ડિલીટ કરે છે.
settings-tab-mcp-server-coming = આ કમ્પ્યુટર પરના AI આસિસ્ટન્ટને તમારી મંજૂરીથી તમારા મેઇલ શોધવા, વાંચવા અને ડ્રાફ્ટ કરવા દો.

## Settings > General

settings-general-conversations = વાર્તાલાપ દૃશ્ય
settings-general-conversations-group = એક જ મેઇલના જવાબોને જૂથબદ્ધ કરો
settings-general-conversations-group-detail = સૂચિમાં દરેક વાર્તાલાપ માટે એક લાઇન
settings-general-reading = વાંચન
settings-general-newest-first = સૌથી નવો મેસેજ પહેલાં
settings-general-newest-first-detail = વાર્તાલાપ તેના છેલ્લા જવાબથી શરૂ થાય છે
settings-general-full-headers = સંપૂર્ણ હેડર બતાવો
settings-general-full-headers-detail = દરેક મેસેજ પર મોકલનાર, પ્રતિ, cc, તારીખ અને વિષય ખુલ્લા દેખાય છે
settings-general-full-names = પ્રાપ્તકર્તાઓનાં પૂરાં નામ
settings-general-full-names-detail = “પ્રતિ મને, Ada” ને બદલે “પ્રતિ મને, Ada Lovelace”
settings-general-mark-read = વાંચેલા તરીકે ચિહ્નિત કરો
settings-general-mark-read-now = ખૂલતાંની સાથે જ
settings-general-mark-read-1s = 1 સેકન્ડ ખુલ્લો રહ્યા પછી
settings-general-mark-read-3s = 3 સેકન્ડ ખુલ્લો રહ્યા પછી
settings-general-mark-read-never = હું વાંચેલા તરીકે ચિહ્નિત કરું ત્યારે જ
settings-general-reply-button = જવાબ બટન
settings-general-reply-all = બધાને જવાબ આપો
settings-general-reply-all-detail = દરેક મેસેજની બાજુનું જવાબ બટન માત્ર મોકલનારને નહીં, બધાને જવાબ આપે છે
settings-general-remote-images = વેબ પરની છબીઓ
settings-general-remote-images-detail = મેસેજની છબીઓ લોડ કરવાથી તેના મોકલનારને ખબર પડે છે કે તમે તે ખોલ્યો છે, ક્યારે અને આશરે ક્યાંથી. બંધ હોય ત્યારે દરેક મેસેજ પહેલાં પૂછે છે, અને તમે કોઈ મોકલનારની છબીઓ હંમેશાં બતાવી શકો છો.
settings-general-remote-images-always = હંમેશાં છબીઓ બતાવો
settings-general-remote-images-always-detail = દરેક મેસેજમાં, માત્ર તમે વિશ્વાસ કરતા હો તે મોકલનાર તરફથી જ નહીં
settings-general-sending = મોકલવું
settings-general-sending-detail = મોકલેલો મેસેજ કેટલો સમય રાહ જુએ, જેથી તેને પાછો ખેંચી શકાય.
settings-general-offline = ઑફલાઇન મેઇલ
settings-general-offline-detail = તાજેતરના મેઇલ કનેક્શન વગર વાંચવા માટે પૂરેપૂરા ડાઉનલોડ થાય છે. જૂના મેઇલ તમે ખોલો ત્યારે ડાઉનલોડ થાય છે.
settings-general-offline-days = { $count ->
    [one] { $count } દિવસ
   *[other] { $count } દિવસ
}
settings-general-offline-years = { $count ->
    [one] { $count } વર્ષ
   *[other] { $count } વર્ષ
}
settings-general-offline-all = બધા મેઇલ
settings-general-offline-note = ઓછા દિવસ પસંદ કરવાથી પહેલેથી ડાઉનલોડ થયેલા મેઇલ રહે છે. સર્વર પર કંઈ બદલાતું નથી.
settings-general-notifications = નોટિફિકેશન
settings-general-notifications-detail = ઇનબૉક્સમાં નવા મેઇલ માટે, Katna Mail બંધ હોય ત્યારે પણ.
settings-general-new-mail = નવા મેઇલ વિશે મને સૂચિત કરો
settings-general-new-mail-detail = બધાને જવાબ આપો, વાંચેલા તરીકે ચિહ્નિત કરો અને આર્કાઇવ કરો સાથે
settings-general-new-mail-sound = અવાજ વગાડો
settings-general-new-mail-sound-detail = ડેસ્કટૉપનો નવા મેઇલનો અવાજ
settings-general-desktop = ડેસ્કટૉપ
settings-general-open-at-login = લૉગિન વખતે Katna Mail ખોલો
settings-general-open-at-login-detail = સેવા ચાલુ હોય ત્યાં સુધી, લૉગિન વખતે મેઇલ કોઈ પણ રીતે સિંક થાય છે
settings-general-tray = સિસ્ટમ ટ્રેમાં Katna બતાવો
settings-general-tray-detail = નહીં વાંચેલાની સંખ્યા અને મેનૂ સાથે
settings-general-unread-badge = ટાસ્કબાર આઇકન પર નહીં વાંચેલાની સંખ્યા
settings-general-unread-badge-detail = ઇનબૉક્સના કેટલા મેસેજ નહીં વાંચેલા છે

## Settings > Inbox

settings-inbox-tabs = ઇનબૉક્સ ટૅબ
settings-inbox-tabs-detail = તમારા મેઇલ પ્રદાતાની વેબસાઇટની જેમ, ઇનબૉક્સને ટૅબમાં ગોઠવો.
settings-inbox-tabs-show = ઇનબૉક્સ ટૅબ બતાવો
settings-inbox-tabs-show-detail = બંધ હોય ત્યારે દરેક એકાઉન્ટ માટે એક જ સૂચિ બતાવે છે
settings-inbox-no-accounts = ટૅબ પસંદ કરવા માટે એકાઉન્ટ ઉમેરો.
settings-inbox-tabs-automatic = ઑટોમેટિક: { $tabs } ({ $provider })
settings-inbox-tabs-off = કોઈ ટૅબ નહીં
settings-inbox-tabs-gmail = પ્રાથમિક, પ્રમોશન, સામાજિક, અપડેટ, ફોરમ
settings-inbox-tabs-focused = ફોકસ્ડ અને અન્ય
settings-inbox-tabs-zoho = ઇનબૉક્સ, ન્યૂઝલેટર અને નોટિફિકેશન
settings-inbox-tabs-shown = બતાવેલા ટૅબ. તમે બંધ કરો તે ટૅબના મેઇલ { $tab } માં રહે છે.

## Settings > Appearance

settings-appearance-reading-pane = વાંચન પેન
settings-appearance-reading-pane-detail = ખોલેલો વાર્તાલાપ ક્યાં દેખાય.
settings-appearance-pane-right = સૂચિની જમણી બાજુએ
settings-appearance-pane-none = કોઈ વિભાજન નહીં
settings-appearance-density = ગીચતા
settings-appearance-density-default = ડિફૉલ્ટ
settings-appearance-density-compact = સઘન
settings-appearance-scaling = સ્કેલિંગ
settings-appearance-scaling-detail = ડેસ્કટૉપના પોતાના સ્કેલ ઉપરાંત, Katna Mail માંની દરેક વસ્તુને મોટી કે નાની બનાવે છે: લખાણ, આઇકન, અંતર અને વિભાજકો. તમે મોકલો તે મેઇલ તેનું પોતાનું ફૉન્ટ કદ જાળવે છે. બહુ નાનાં કદથી આઇકન પર ક્લિક કરવું મુશ્કેલ બની શકે છે.
settings-appearance-theme = થીમ
settings-appearance-theme-system = ડેસ્કટૉપ જેવી જ
settings-appearance-theme-light = આછી
settings-appearance-theme-dark = ઘેરી
settings-appearance-desktop-colors = ડેસ્કટૉપના રંગો
settings-appearance-desktop-colors-use = ડેસ્કટૉપના રંગોનો ઉપયોગ કરો
settings-appearance-desktop-colors-use-detail = ડેસ્કટૉપની રંગ યોજના અને ઍક્સેન્ટ રંગ
settings-appearance-app-names = ઍપનાં નામ
settings-appearance-app-names-show = ઍપનાં નામ બતાવો
settings-appearance-app-names-show-detail = સૌથી ડાબી બાજુના ઍપ આઇકનની નીચે નામ
settings-appearance-sender-pictures = મોકલનારનાં ચિત્રો
settings-appearance-sender-pictures-show = કંપનીના લોગો બતાવો
settings-appearance-sender-pictures-show-detail = મોકલનારના ડોમેન દ્વારા શોધાય છે, ક્યારેય મેસેજ દ્વારા નહીં, અને એક અઠવાડિયા સુધી રાખવામાં આવે છે
settings-appearance-important = મહત્ત્વપૂર્ણ માર્કર
settings-appearance-important-show = મહત્ત્વપૂર્ણ માર્કર બતાવો
settings-appearance-important-show-detail = સૂચિમાં દરેક મેસેજની બાજુમાં
settings-appearance-message-width = મેસેજની પહોળાઈ
settings-appearance-message-width-limit = મેસેજની પહોળાઈ મર્યાદિત કરો
settings-appearance-message-width-limit-detail = પહોળી વિન્ડોમાં લાંબી લાઇનો વાંચવી સરળ બને છે
settings-appearance-mail-colors = મેઇલના રંગો
settings-appearance-mail-colors-detail = મોટા ભાગના મેઇલ સફેદ પાના માટે બનાવાયેલા હોય છે. ઘેરી થીમમાં તેના રંગો સારી રીતે વાંચી શકાય તેવા ઘેરા રંગોમાં બદલાય છે; બંધ હોય ત્યારે, તે આછા પાના પર તેના મોકલનારના રંગો જાળવે છે.
settings-appearance-dark-mail = મેઇલ માટે પણ ઘેરા રંગો
settings-appearance-dark-mail-detail = માત્ર થીમ ઘેરી હોય ત્યારે
settings-appearance-attachment-previews = જોડાણનાં પ્રીવ્યૂ
settings-appearance-attachment-previews-show = જોડાણનાં પ્રીવ્યૂ બતાવો
settings-appearance-attachment-previews-show-detail = દરેક ફાઇલના કાર્ડ પર તેની સામગ્રીનું નાનું ચિત્ર

## Settings > Default apps

settings-default-apps-intro = તમે જોડાણ પર ક્લિક કરો ત્યારે તે ક્યાં ખૂલે. વ્યૂઅર હંમેશાં ફાઇલને બીજી ઍપમાં પણ ખોલી શકે છે. ડેસ્કટૉપની ડિફૉલ્ટ ઍપ તેના પોતાના સેટિંગમાં સેટ થાય છે.
settings-default-apps-pdf = PDF ફાઇલો
settings-default-apps-pdf-detail = પાનાં, ઝૂમ સાથે.
settings-default-apps-pictures = ચિત્રો
settings-default-apps-pictures-detail = ફોટા (સીધા કરેલા), PNG, GIF, WebP, BMP, TIFF અને SVG.
settings-default-apps-text = ટેક્સ્ટ ફાઇલો
settings-default-apps-text-detail = સાદું લખાણ, લૉગ, કોડ અને અન્ય લખાણ.
settings-default-apps-sheets = સ્પ્રેડશીટ
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) અને CSV.
settings-default-apps-documents = દસ્તાવેજો
settings-default-apps-documents-detail = Word (docx) અને OpenDocument ટેક્સ્ટ (odt).
settings-default-apps-katna = Katna Mail નું વ્યૂઅર
settings-default-apps-system = ડેસ્કટૉપની ડિફૉલ્ટ ઍપ
settings-default-apps-ask = દર વખતે કઈ ઍપ તે પૂછો
settings-default-apps-after-saving = સેવ કર્યા પછી
settings-default-apps-show-folder = સેવ કરેલી ફાઇલો તેમના ફોલ્ડરમાં બતાવો
settings-default-apps-show-folder-detail = સેવ કરેલાં જોડાણ પસંદ કરેલાં હોય એ રીતે ફાઇલ મેનેજર ખોલે છે

## Settings > Compose

settings-compose-send-from = નવા મેસેજ આમાંથી મોકલો
settings-compose-send-from-detail = જવાબ અને ફૉરવર્ડ હંમેશાં તમે જે એકાઉન્ટમાં હો તેમાંથી જાય છે.
settings-compose-send-from-current = તમે જે એકાઉન્ટમાં છો તે
settings-compose-send-on-replies = જવાબો પર મોકલો
settings-compose-send-on-replies-detail = જવાબ કે ફૉરવર્ડ પર મોકલો બટન શું કરે. મોકલો ની બાજુનું મેનૂ બીજો વિકલ્પ આપે છે.
settings-compose-send-plain = મોકલો
settings-compose-send-archive = મોકલો અને આર્કાઇવ કરો
settings-compose-signatures = હસ્તાક્ષર
settings-compose-signatures-detail = તમારા મેસેજની નીચે, “--” લાઇન પછી ઉમેરાય છે. લખવાની વિન્ડોમાં બીજો પસંદ કરો.
settings-compose-untitled = શીર્ષક વિનાનું
settings-compose-signature-name = નામ, જેમ કે કાર્ય
settings-compose-signature-first = મારા હસ્તાક્ષર
settings-compose-signature-numbered = હસ્તાક્ષર { $number }
settings-compose-signature-delete = ડિલીટ કરો
settings-compose-signature-deleted = હસ્તાક્ષર ડિલીટ કર્યા
settings-compose-signature-new = નવું બનાવો
settings-compose-no-signatures = હજી કોઈ હસ્તાક્ષર નથી.
settings-compose-no-signature = કોઈ હસ્તાક્ષર નહીં
settings-compose-for-new-mail = નવા મેઇલ માટે
settings-compose-for-replies = જવાબ અને ફૉરવર્ડ માટે
settings-compose-for-replies-detail = જે વાર્તાલાપમાં તમે કોઈ મેસેજ પર હસ્તાક્ષર કર્યા હોય, તેમાં જવાબ તેના બદલે એ હસ્તાક્ષરથી શરૂ થાય છે.
settings-compose-format = ફૉર્મેટ
settings-compose-plain-text = સાદા લખાણમાં લખો
settings-compose-plain-text-detail = નવા મેઇલ ફૉર્મેટિંગ વગર શરૂ થાય છે; લખવાની વિન્ડોમાં બદલી શકાય છે
settings-compose-spelling = જોડણી
settings-compose-spell-check = હું લખું ત્યારે જોડણી તપાસો
settings-compose-spell-check-detail = ખોટી જોડણીવાળા શબ્દો નીચે લીટી દોરાય છે, અને જમણું ક્લિક કરવાથી સૂચનો મળે છે
settings-compose-spell-desktop = ડેસ્કટૉપની ભાષા ({ $language })
settings-compose-templates = ટેમ્પ્લેટ
settings-compose-templates-detail = તમે વારંવાર લખો તે મેઇલ સેવ કરો, અને તેમાંથી નવો મેઇલ કે જવાબ શરૂ કરો.

## Settings > Shortcuts

settings-shortcuts-set = શૉર્ટકટ સેટ
settings-shortcuts-set-detail = તમે જાણતા હો તે મેઇલ ઍપની કીથી શરૂઆત કરો. અહીં Cmd એટલે Ctrl. તમારા પોતાના ફેરફારો સેટની ઉપર રહે છે, અને ડિફૉલ્ટ રિસ્ટોર કરો સેટની કી પર પાછું લાવે છે.
settings-shortcuts-single = એક-કી શૉર્ટકટ
settings-shortcuts-single-detail = વેબમેઇલની જેમ, Ctrl કે Alt વગરની કી: e આર્કાઇવ કરે છે, j અને k ખસેડે છે, / શોધે છે. તે સૂચિમાં અને ખુલ્લા વાર્તાલાપમાં કામ કરે છે, લખતી વખતે ક્યારેય નહીં.
settings-shortcuts-single-use = એક-કી શૉર્ટકટનો ઉપયોગ કરો
settings-shortcuts-single-use-detail = Ctrl શૉર્ટકટ હંમેશાં કામ કરે છે
settings-shortcuts-how = કી બદલવા માટે તેના પર ક્લિક કરો, અથવા નવી ઉમેરવા માટે + પર, પછી નવી કી દબાવો. Esc રદ કરે છે.
settings-shortcuts-restore = ડિફૉલ્ટ રિસ્ટોર કરો
settings-shortcuts-no-key = કોઈ કી નથી
settings-shortcuts-press = કી દબાવો…
settings-shortcuts-then = { $keys } પછી…
settings-shortcuts-moved = { $keys } હવે “{ $previous }” ને બદલે “{ $action }” કરે છે.
settings-shortcuts-single-off = એક-કી શૉર્ટકટ બંધ છે, તેથી તે ચાલુ થશે ત્યારે આ કી કામ કરશે.
settings-shortcuts-restored = દરેક શૉર્ટકટને ફરીથી તેના સેટની કી મળી ગઈ છે.

## Settings search: the line under a result

settings-general-language-summary = ઍપ, તારીખો અને સંખ્યાઓની ભાષા
settings-general-reading-summary = સૌથી નવો મેસેજ પહેલાં, સંપૂર્ણ હેડર, પ્રાપ્તકર્તાઓનાં પૂરાં નામ
settings-general-mark-read-summary = ખોલેલો વાર્તાલાપ ક્યારે વાંચેલો ચિહ્નિત થાય: તરત, 1 કે 3 સેકન્ડ પછી, અથવા જાતે
settings-general-reply-button-summary = દરેક મેસેજની બાજુનું જવાબ બટન બધાને જવાબ આપે છે
settings-general-remote-images-summary = દરેક મેસેજની છબીઓ હંમેશાં બતાવો
settings-general-sending-summary = મોકલવાનું પૂર્વવત્ કરો: મોકલેલો મેસેજ કેટલો સમય રાહ જુએ, જેથી તેને પાછો ખેંચી શકાય
settings-general-offline-summary = કનેક્શન વગર વાંચવા માટે કેટલા દિવસના તાજેતરના મેઇલ પૂરેપૂરા ડાઉનલોડ થાય
settings-general-notifications-summary = નવા મેઇલનાં નોટિફિકેશન અને તેમનો અવાજ
settings-general-desktop-summary = લૉગિન વખતે Katna Mail ખોલો, સિસ્ટમ ટ્રે આઇકન અને ટાસ્કબાર આઇકન પર નહીં વાંચેલાની સંખ્યા
settings-accounts-accounts-summary = એકાઉન્ટ ઉમેરો કે કાઢી નાખો, અથવા તેનું ચિત્ર બદલો
settings-appearance-density-summary = સૂચિમાં ડિફૉલ્ટ કે સઘન લાઇનો
settings-appearance-scaling-summary = બધું મોટું કે નાનું કરો: લખાણ, આઇકન, અંતર અને વિભાજકો
settings-appearance-theme-summary = ડેસ્કટૉપ જેવી જ, આછી કે ઘેરી
settings-appearance-sender-pictures-summary = કંપનીના લોગો, મોકલનારના ડોમેન દ્વારા શોધાયેલા
settings-appearance-important-summary = સૂચિમાં દરેક મેસેજની બાજુમાં મહત્ત્વપૂર્ણ માર્કર
settings-appearance-mail-colors-summary = ઘેરી થીમમાં HTML મેઇલ માટે ઘેરા રંગો, અથવા તેના મોકલનારના રંગો
settings-appearance-attachment-previews-summary = દરેક જોડાણની સામગ્રીનું નાનું ચિત્ર
settings-shortcuts-set-summary = Gmail, Inbox by Gmail, Apple Mail, Outlook કે Thunderbird ની કીથી શરૂઆત કરો
settings-shortcuts-single-summary = વેબમેઇલની જેમ, Ctrl કે Alt વગરની કી
settings-default-apps-pdf-summary = PDF જોડાણ ક્યાં ખૂલે
settings-default-apps-pictures-summary = ફોટા અને ચિત્રો ક્યાં ખૂલે
settings-default-apps-text-summary = સાદું લખાણ, લૉગ અને કોડ ક્યાં ખૂલે
settings-default-apps-sheets-summary = Excel, OpenDocument અને CSV ફાઇલો ક્યાં ખૂલે
settings-default-apps-documents-summary = Word અને OpenDocument ટેક્સ્ટ ક્યાં ખૂલે
settings-default-apps-after-saving-summary = સેવ કરેલાં જોડાણ તેમના ફોલ્ડરમાં બતાવો
settings-compose-send-from-summary = નવા મેઇલ જે એકાઉન્ટમાંથી જાય: તમે જેમાં હો તે, અથવા હંમેશાં એક જ
settings-compose-send-on-replies-summary = જવાબ અને ફૉરવર્ડ પર મોકલો, અથવા મોકલો અને વાર્તાલાપ આર્કાઇવ કરો
settings-compose-signatures-summary = તમારા મેસેજની નીચે, “--” લાઇન પછી ઉમેરાય છે
settings-compose-for-new-mail-summary = નવા મેઇલ જે હસ્તાક્ષરથી શરૂ થાય
settings-compose-for-replies-summary = જવાબ અને ફૉરવર્ડ જે હસ્તાક્ષરથી શરૂ થાય
settings-compose-format-summary = નવા મેઇલ સાદા લખાણમાં લખો
settings-compose-spelling-summary = લખતી વખતે જોડણી તપાસો, અને શબ્દકોશની ભાષા
settings-compose-templates-summary = ટૂંક સમયમાં: તમે વારંવાર લખો તે મેઇલ સેવ કરો, અને તેમાંથી નવો મેઇલ કે જવાબ શરૂ કરો
settings-feedback-crash-reports-summary = Katna Mail કે તેની બૅકગ્રાઉન્ડ સેવા ક્રેશ થાય ત્યારે ક્રેશ રિપોર્ટ આ કમ્પ્યુટર પર સેવ કરો
settings-feedback-saved-summary = આ કમ્પ્યુટર પર સેવ કરેલા ક્રેશ રિપોર્ટ જુઓ, કૉપિ કરો કે ડિલીટ કરો
settings-feedback-help-improve-summary = શું ખોટું થયું તે સુધારવામાં સહાય માટે ક્રેશ રિપોર્ટ મોકલો; તમે ચાલુ ન કરો ત્યાં સુધી બંધ
settings-experimental-blur-summary = ટોચના બાર પાછળથી ડેસ્કટૉપ ઝાંખું દેખાય છે, અને મેનૂ ધૂંધળા કાચ જેવાં હોય છે
settings-search-shortcut = કીબોર્ડ શૉર્ટકટ
settings-search-tab = સેટિંગ ટૅબ
settings-search-none = “{ $query }” સાથે મેળ ખાતું કોઈ સેટિંગ નથી.
settings-search-results = “{ $query }” સાથે મેળ ખાતાં સેટિંગ

## Quick settings (the panel that slides in from the right)

quick-title = ઝડપી સેટિંગ
quick-see-all = બધાં સેટિંગ જુઓ
quick-reading-pane = વાંચન પેન
quick-pane-right = સૂચિની જમણી બાજુએ
quick-pane-none = કોઈ વિભાજન નહીં
quick-density = ગીચતા
quick-density-default = ડિફૉલ્ટ
quick-density-compact = સઘન
quick-theme = થીમ
quick-theme-system = ડેસ્કટૉપ જેવી જ
quick-theme-light = આછી
quick-theme-dark = ઘેરી
quick-desktop-colors = ડેસ્કટૉપના રંગો
quick-desktop-colors-detail = ડેસ્કટૉપની રંગ યોજના અને ઍક્સેન્ટ રંગ
quick-app-names = ઍપનાં નામ
quick-app-names-detail = સૌથી ડાબી બાજુના ઍપ આઇકનની નીચે નામ
quick-inbox-tabs = ઇનબૉક્સ ટૅબ
quick-inbox-tabs-detail = દરેક એકાઉન્ટના મેઇલ પ્રદાતાના ટૅબ
quick-choose-tabs = ટૅબ પસંદ કરો
quick-choose-tabs-detail = દરેક એકાઉન્ટ માટે, સેટિંગમાં
quick-sending = મોકલવું
quick-undo-send = મોકલવાનું પૂર્વવત્ કરો
quick-undo-send-off = બંધ
quick-undo-send-seconds = { $seconds } સેકન્ડ
quick-signatures = હસ્તાક્ષર
quick-signatures-none = હજી કોઈ નહીં
quick-signatures-one = { $name }, ડિફૉલ્ટ તરીકે વપરાય છે
quick-signatures-many = { $count ->
    [one] { $count } હસ્તાક્ષર; ડિફૉલ્ટ { $name }
   *[other] { $count } હસ્તાક્ષર; ડિફૉલ્ટ { $name }
}
quick-signatures-no-default = { $count ->
    [one] { $count }, કોઈ ડિફૉલ્ટ નહીં
   *[other] { $count }, કોઈ ડિફૉલ્ટ નહીં
}
quick-signature-untitled = શીર્ષક વિનાનું
quick-threading = ઇમેઇલ થ્રેડિંગ
quick-conversation-view = વાર્તાલાપ દૃશ્ય
quick-conversation-view-detail = એક જ મેઇલના જવાબોને જૂથબદ્ધ કરો
quick-help = સહાય
quick-tour = પરિચય ટૂર લો
quick-whats-new = નવું શું છે
quick-about = Katna વિશે

## Settings: opening at login

settings-open-at-login-failed = લૉગિન વખતે ખોલવાનું બદલી શકાયું નથી: { $error }

## Settings > Appearance > Scaling

scale-letter = અ
scale-percent = { $percent }%
scale-reset = પાછું { $percent }% કરો

## Settings > Experimental > Look & Feel

look-intro = હજી અજમાવાઈ રહેલી સુવિધાઓ. તે બદલાઈ શકે છે અથવા દૂર થઈ શકે છે.
look-heading = દેખાવ અને અનુભવ
look-window-frame = વિન્ડો ફ્રેમ
look-window-frame-detail = શીર્ષક બાર, વિન્ડો બટન, ખૂણા અને પડછાયો કોણ દોરે.
look-frame-native-kde = મૂળ: KDE ની ફ્રેમ, તમારી Plasma થીમમાં
look-frame-native = મૂળ: ડેસ્કટૉપની ફ્રેમ
look-frame-katna = Katna: ટોચનો બાર શીર્ષક બાર બની જાય છે
look-frame-katna-note-named = Katna ગોળ ખૂણા અને પોતાનો પડછાયો દોરે છે. ફ્રેમ હવે { $desktop } થીમને અનુસરતી નથી; વિન્ડો નિયમો હજી લાગુ પડે છે.
look-frame-katna-note = Katna ગોળ ખૂણા અને પોતાનો પડછાયો દોરે છે. ફ્રેમ હવે ડેસ્કટૉપ થીમને અનુસરતી નથી; વિન્ડો નિયમો હજી લાગુ પડે છે.
look-frame-client-side = તમારું ડેસ્કટૉપ ફ્રેમ દરેક ઍપ પર છોડે છે, તેથી Katna પહેલેથી જ પોતાની ફ્રેમ દોરે છે.
look-blurred-background = ઝાંખું બૅકગ્રાઉન્ડ
look-blurred-background-detail = ટોચના બાર અને ફોલ્ડર પાછળથી ડેસ્કટૉપ ઝાંખું દેખાય છે, અને મેનૂ તથા પૉપઓવર ધૂંધળા કાચ જેવાં હોય છે.
look-blur = વિન્ડોની પાછળનું ઝાંખું કરો
look-blur-detail = મેઇલ નક્કર કાર્ડ પર રહે છે, તેથી લખાણનો કૉન્ટ્રાસ્ટ જળવાય છે
look-blur-off-kde = KDE ની બ્લર અસર બંધ છે. સિસ્ટમ સેટિંગ, વિન્ડો મેનેજમેન્ટ, ડેસ્કટૉપ ઇફેક્ટ્સમાં બ્લર ચાલુ કરો, પછી Katna Mail ફરી ખોલો.
look-blur-none-gnome = GNOME વિન્ડોની પાછળનું ઝાંખું કરતું નથી.
look-blur-none-x11 = તમારું વિન્ડો મેનેજર વિન્ડોની પાછળનું ઝાંખું કરતું નથી.
look-blur-none-wayland = તમારું કમ્પોઝિટર વિન્ડોની પાછળનું ઝાંખું કરતું નથી.

## Settings > User feedback (crash reports)

feedback-intro-sending = શું ખોટું થયું તે સુધારવામાં સહાય માટે નવા ક્રેશ રિપોર્ટ મોકલવામાં આવે છે. બીજું કંઈ આ કમ્પ્યુટરની બહાર જતું નથી.
feedback-intro-local = Katna ક્યાંય કંઈ મોકલતું નથી. ક્રેશ રિપોર્ટ આ કમ્પ્યુટર પર રહે છે, જેથી તમે તેને જોઈ શકો અથવા બગ રિપોર્ટ સાથે જોડી શકો.
feedback-crash-reports = ક્રેશ રિપોર્ટ
feedback-crash-reports-detail = Katna Mail કે તેની બૅકગ્રાઉન્ડ સેવા ક્રેશ થાય ત્યારે લખાય છે.
feedback-save = ક્રેશ રિપોર્ટ આ કમ્પ્યુટર પર સેવ કરો
feedback-save-detail = તમારું હોમ ફોલ્ડર, વપરાશકર્તા અને કમ્પ્યુટરનાં નામ તથા ઇમેઇલ ઍડ્રેસ બાકાત રાખવામાં આવે છે
feedback-saved = સેવ કરેલા ક્રેશ રિપોર્ટ
feedback-saved-detail = { $count ->
    [one] સૌથી નવો { $count } રાખવામાં આવે છે.
   *[other] સૌથી નવા { $count } રાખવામાં આવે છે.
}
feedback-help-improve = Katna ને બહેતર બનાવવામાં સહાય કરો
feedback-help-improve-detail = તમે ચાલુ ન કરો ત્યાં સુધી બંધ, અને તમે અહીં ગમે ત્યારે તેને બંધ કરી શકો છો.
feedback-send = ક્રેશ રિપોર્ટ મોકલો
feedback-send-detail = સેવ કરેલો રિપોર્ટ, બરાબર જેવો તમે અહીં જોઈ શકો છો તેવો જ, Katna ના ક્રેશ ટ્રેકર (Sentry, EU માં) પર જાય છે. કોઈ IP ઍડ્રેસ, મેસેજ કે ઇમેઇલ ઍડ્રેસ નહીં
feedback-none-saved = કોઈ ક્રેશ રિપોર્ટ સેવ કરેલા નથી.
feedback-delete-all = બધા ડિલીટ કરો
feedback-app-daemon = બૅકગ્રાઉન્ડ સેવા
feedback-report-sent = { $date } · મોકલ્યો
feedback-view = જુઓ
feedback-view-tooltip = રિપોર્ટ ખોલો
feedback-copy-tooltip = બગ રિપોર્ટમાં પેસ્ટ કરવા માટે તેને કૉપિ કરો
feedback-copied = ક્રેશ રિપોર્ટ કૉપિ કર્યો.
feedback-deleted-all = ક્રેશ રિપોર્ટ ડિલીટ કર્યા.
feedback-read-failed = ક્રેશ રિપોર્ટ વાંચી શકાયો નથી: { $error }
feedback-delete-failed = ક્રેશ રિપોર્ટ ડિલીટ કરી શકાયો નથી: { $error }
feedback-delete-all-failed = ક્રેશ રિપોર્ટ ડિલીટ કરી શકાયા નથી: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _ફાઇલ
desktop-menu-new-message = _નવો મેસેજ
desktop-menu-quit = _બહાર નીકળો
desktop-menu-edit = _ફેરફાર કરો
desktop-menu-undo = _પૂર્વવત્ કરો
desktop-menu-select-all = _બધું પસંદ કરો
desktop-menu-select-none = _કંઈ પસંદ ન કરો
desktop-menu-find = _શોધો…
desktop-menu-view = _જુઓ
desktop-menu-folder-list = _ફોલ્ડર સૂચિ બતાવો
desktop-menu-refresh = _રિફ્રેશ કરો
desktop-menu-go = _જાઓ
desktop-menu-inbox = _ઇનબૉક્સ
desktop-menu-starred = _તારાંકિત
desktop-menu-sent = _મોકલેલા
desktop-menu-drafts = _ડ્રાફ્ટ
desktop-menu-all-mail = _બધા મેઇલ
desktop-menu-next = _આગલો વાર્તાલાપ
desktop-menu-previous = _પાછલો વાર્તાલાપ
desktop-menu-message = _મેસેજ
desktop-menu-open = _ખોલો
desktop-menu-reply = _જવાબ આપો
desktop-menu-reply-all = _બધાને જવાબ આપો
desktop-menu-forward = _ફૉરવર્ડ કરો
desktop-menu-archive = _આર્કાઇવ કરો
desktop-menu-delete = _ડિલીટ કરો
desktop-menu-spam = _સ્પામની જાણ કરો
desktop-menu-move-to = _આમાં ખસેડો…
desktop-menu-mark-read = _વાંચેલા તરીકે ચિહ્નિત કરો
desktop-menu-mark-unread = _નહીં વાંચેલા તરીકે ચિહ્નિત કરો
desktop-menu-star = _તારો ઉમેરો
desktop-menu-important = _મહત્ત્વપૂર્ણ તરીકે ચિહ્નિત કરો
desktop-menu-not-important = _મહત્ત્વપૂર્ણ નથી તરીકે ચિહ્નિત કરો
desktop-menu-settings = _સેટિંગ
desktop-menu-quick-settings = _ઝડપી સેટિંગ
desktop-menu-configure = _Katna Mail ગોઠવો…
desktop-menu-help = _સહાય
desktop-menu-shortcuts = _કીબોર્ડ શૉર્ટકટ
desktop-menu-whats-new = _નવું શું છે
desktop-menu-about = _Katna વિશે

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = નેવિગેશન
shortcut-group-actions = ક્રિયાઓ
shortcut-group-go-to = આના પર જાઓ
shortcut-group-app = ઍપ્લિકેશન

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = આગલો વાર્તાલાપ
shortcut-previous = પાછલો વાર્તાલાપ
shortcut-down = સૂચિમાં નીચે જાઓ
shortcut-up = સૂચિમાં ઉપર જાઓ
shortcut-first = સૂચિમાં પહેલો
shortcut-last = સૂચિમાં છેલ્લો
shortcut-page-down = સૂચિમાં એક પાનું નીચે
shortcut-page-up = સૂચિમાં એક પાનું ઉપર
shortcut-open = વાર્તાલાપ ખોલો
shortcut-back = સૂચિ પર પાછા જાઓ
shortcut-scroll-down = નીચે સ્ક્રોલ કરો
shortcut-scroll-up = ઉપર સ્ક્રોલ કરો
shortcut-scroll-page-down = એક પાનું નીચે સ્ક્રોલ કરો
shortcut-scroll-page-up = એક પાનું ઉપર સ્ક્રોલ કરો
shortcut-compose = લખો
shortcut-reply = જવાબ આપો
shortcut-reply-all = બધાને જવાબ આપો
shortcut-forward = ફૉરવર્ડ કરો
shortcut-archive = આર્કાઇવ કરો
shortcut-delete = ડિલીટ કરો
shortcut-spam = સ્પામની જાણ કરો
shortcut-move-to = આમાં ખસેડો
shortcut-mark-read = વાંચેલા તરીકે ચિહ્નિત કરો
shortcut-mark-unread = નહીં વાંચેલા તરીકે ચિહ્નિત કરો
shortcut-star = તારો ઉમેરો કે કાઢી નાખો
shortcut-important = મહત્ત્વપૂર્ણ તરીકે ચિહ્નિત કરો
shortcut-not-important = મહત્ત્વપૂર્ણ નથી તરીકે ચિહ્નિત કરો
shortcut-check = વાર્તાલાપ પસંદ કરો
shortcut-select-all = બધા વાર્તાલાપ પસંદ કરો
shortcut-select-none = બધા વાર્તાલાપની પસંદગી રદ કરો
shortcut-undo = છેલ્લી ક્રિયા પૂર્વવત્ કરો
shortcut-go-inbox = ઇનબૉક્સ
shortcut-go-starred = તારાંકિત
shortcut-go-sent = મોકલેલા
shortcut-go-drafts = ડ્રાફ્ટ
shortcut-go-all = બધા મેઇલ
shortcut-search = મેઇલ શોધો
shortcut-navigation = મેનૂ બતાવો કે સંકુચિત કરો
shortcut-quick-settings = ઝડપી સેટિંગ
shortcut-settings = બધાં સેટિંગ
shortcut-shortcuts = કીબોર્ડ શૉર્ટકટ
shortcut-reload = નવા મેઇલ માટે તપાસો
shortcut-quit = બહાર નીકળો

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } પછી { $second }

## Settings > Accounts

accounts-folder-pane = ફોલ્ડર પેન
accounts-folder-pane-detail = ડાબી બાજુની પેન કયાં એકાઉન્ટનાં ફોલ્ડર બતાવે.
accounts-shown-one = એક સમયે એક એકાઉન્ટ; એકાઉન્ટ કાર્ડમાં બદલો
accounts-shown-all = બધાં એકાઉન્ટ, એક પછી એક
accounts-row = એકાઉન્ટ
accounts-row-detail = એકાઉન્ટ કાઢી નાખવાથી આ કમ્પ્યુટર પરની Katna ની તેના મેઇલની કૉપિ ડિલીટ થાય છે. મેઇલ સર્વર પર રહે છે.
accounts-none = હજી કોઈ એકાઉન્ટ નથી.
accounts-kind-imported = આયાત કરેલું
accounts-picture-reset = ડેસ્કટૉપ ચિત્રનો ઉપયોગ કરો
accounts-picture-change = ચિત્ર બદલો
accounts-remove = કાઢી નાખો
accounts-delete-all-row = બધો ડેટા ડિલીટ કરો
accounts-delete-all-row-detail = નવા ઇન્સ્ટૉલની જેમ, ફરીથી શરૂઆત કરો.
accounts-delete-all-about = આ કમ્પ્યુટર પરથી દરેક એકાઉન્ટ, બધા સ્ટોર કરેલા મેઇલ, સંપર્કો અને કૅલેન્ડર, શોધ ઇન્ડેક્સ, તમારાં સેટિંગ અને સેવ કરેલા પાસવર્ડ ડિલીટ કરે છે. તમારા મેઇલ સર્વર પર કંઈ બદલાતું નથી.
accounts-delete-all-open = Katna નો બધો ડેટા ડિલીટ કરો

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } ને Katna માંથી કાઢી નાખ્યું.
accounts-removed = { $address } ને Katna માંથી કાઢી નાખ્યું. તેના મેઇલ હજી સર્વર પર છે.
accounts-all-deleted = Katna નો બધો ડેટા આ કમ્પ્યુટર પરથી ડિલીટ કર્યો.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } કાઢી નાખીએ?
accounts-remove-confirm = એકાઉન્ટ કાઢી નાખો
accounts-removing = કાઢી રહ્યાં છીએ…
accounts-remove-local-mail = { $folders ->
    [0] આ એકાઉન્ટમાં આયાત કરેલા બધા મેઇલ
    [one] આ એકાઉન્ટમાં તેના ફોલ્ડરમાં આયાત કરેલા બધા મેઇલ
   *[other] આ એકાઉન્ટમાં તેના { $folders } ફોલ્ડરમાં આયાત કરેલા બધા મેઇલ
}
accounts-remove-local-settings = તેનાં Katna સેટિંગ
accounts-remove-mail = { $folders ->
    [0] Katna દ્વારા સ્ટોર કરેલા આ એકાઉન્ટના બધા મેઇલ
    [one] Katna દ્વારા તેના ફોલ્ડરમાં સ્ટોર કરેલા આ એકાઉન્ટના બધા મેઇલ
   *[other] Katna દ્વારા તેના { $folders } ફોલ્ડરમાં સ્ટોર કરેલા આ એકાઉન્ટના બધા મેઇલ
}
accounts-remove-outbox = આઉટબૉક્સમાં રાહ જોતા તેના મેસેજ
accounts-remove-settings = તેનો સેવ કરેલો પાસવર્ડ અને તેનાં Katna સેટિંગ
accounts-delete-all-title = Katna નો બધો ડેટા ડિલીટ કરીએ?
accounts-delete-all-confirm = બધું ડિલીટ કરો
accounts-deleting = ડિલીટ કરી રહ્યાં છીએ…
accounts-delete-all-accounts = દરેક એકાઉન્ટ, અને Katna દ્વારા સ્ટોર કરેલા બધા મેઇલ અને જોડાણ
accounts-delete-all-contacts = સંપર્કો, કૅલેન્ડર અને શોધ ઇન્ડેક્સ
accounts-delete-all-settings = બધાં સેટિંગ, હસ્તાક્ષર અને કીબોર્ડ શૉર્ટકટ
accounts-delete-all-passwords = દરેક સેવ કરેલો પાસવર્ડ
accounts-deleted-heading = આ કમ્પ્યુટર પરથી ડિલીટ થશે:
accounts-cannot-undo = આ પૂર્વવત્ કરી શકાતું નથી.
accounts-server-delete-all = તમારા મેઇલ સર્વર પર કંઈ બદલાતું નથી: તમારા મેઇલ ત્યાં જ રહે છે, અને ફરીથી એકાઉન્ટ ઉમેરવાથી તે ફરી ડાઉનલોડ થાય છે. ફાઇલોમાંથી આયાત કરેલા મેઇલ માત્ર Katna માં છે; ફાઇલોને કંઈ થતું નથી.
accounts-server-local = આ મેઇલ ફાઇલોમાંથી આયાત કરવામાં આવ્યા હતા, તેથી એકમાત્ર કૉપિ Katna પાસે છે. જે ફાઇલોમાંથી તે આવ્યા તેમને કંઈ થતું નથી; તેમને પાછા મેળવવા માટે ફરીથી આયાત કરો.
accounts-server-remove = મેઇલ સર્વર પર કંઈ બદલાતું નથી: તમારા મેઇલ ત્યાં જ રહે છે, અને ફરીથી એકાઉન્ટ ઉમેરવાથી તે ફરી ડાઉનલોડ થાય છે.
accounts-confirm-word = ડિલીટ
accounts-confirm-placeholder = “{ accounts-confirm-word }” લખો
accounts-confirm-prompt = ખાતરી કરવા માટે, “{ accounts-confirm-word }” લખો:
accounts-cancel = રદ કરો
