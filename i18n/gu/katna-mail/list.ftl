# Katna Mail, Gujarati (ગુજરાતી).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
