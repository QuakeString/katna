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
