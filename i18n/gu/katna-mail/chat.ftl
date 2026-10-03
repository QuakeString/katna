# Katna Mail, Gujarati (ગુજરાતી).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = વાંચન
chat-view = વાર્તાલાપ ચૅટ તરીકે
chat-view-detail = લોકો વચ્ચેની મેઇલ ગ્રૂપ ચૅટની જેમ વંચાય છે: દરેક મેઇલ માટે એક બબલ, જેમાં ફક્ત લખેલું હોય, અને તમારી પોતાની જમણી બાજુએ. ન્યૂઝલેટર સામાન્ય દૃશ્યમાં જ રહે છે.
chat-view-switch = વાર્તાલાપ ચૅટ તરીકે બતાવો
chat-view-switch-detail = અવતરિત મેઇલ અને હસ્તાક્ષર દરેક બબલમાં ··· પાછળ રહે છે

chat-switch-chat = ચૅટ
chat-switch-mail = મેઇલ
chat-people = { $names } અને તમે · { $count ->
    [one] { $count } મેઇલ
   *[other] { $count } મેઇલ
}
chat-people-heading = { $count ->
    [one] આ ચૅટમાં · { $count } વ્યક્તિ
   *[other] આ ચૅટમાં · { $count } વ્યક્તિ
}
chat-member-mails = { $count ->
    [0] કોઈ મેઇલ નથી
    [one] { $count } મેઇલ
   *[other] { $count } મેઇલ
}
chat-today = આજે
chat-yesterday = ગઈકાલે
chat-added = { $who } એ { $names } ને ઉમેર્યા
chat-renamed = { $who } એ વિષય બદલીને “{ $subject }” કર્યો
chat-you = તમે
chat-not-downloaded = હજી ડાઉનલોડ થયું નથી
chat-forwarded = ફૉરવર્ડ કરેલું
chat-show-quoted = અવતરિત મેઇલ અને હસ્તાક્ષર બતાવો
chat-hide-quoted = અવતરિત મેઇલ અને હસ્તાક્ષર છુપાવો
chat-hide-dots = ··· છુપાવો
chat-show-card = તેમનું કાર્ડ બતાવો
chat-reply-all = બધાને જવાબ આપો
chat-more = વધુ
chat-reply-only = ફક્ત { $name } ને જવાબ આપો
chat-forward = ફૉરવર્ડ કરો
chat-copy-text = લખાણ કૉપિ કરો
chat-show-as-mail = મેઇલ તરીકે બતાવો
chat-pin = ઉપર પિન કરો
chat-pin-file = ફાઇલ ઉપર પિન કરો
chat-unpin = અનપિન કરો
chat-unpin-file = ફાઇલ અનપિન કરો
chat-pinned-of = { $count } માંથી { $at } પિન કરેલું
chat-pins-all = બધી પિન
chat-pins-heading = પિન કરેલાં · { $most } માંથી { $count }
chat-pins-drag = ક્રમ બદલવા ખેંચો
chat-pin-from-mail = { $name } તરફથી મેઇલ · { $when }
chat-pin-from-file = { $name } તરફથી ફાઇલ · { $when }
chat-pin-from-text = { $name } તરફથી લખાણ · { $when }
chat-pins-full = આ ચૅટમાં પહેલેથી 5 પિન છે
chat-pins-replace-title = પિન બદલો
chat-pins-replace-hint = એક ચૅટમાં વધુમાં વધુ 5 પિન રહી શકે. કઈ હટાવવી તે પસંદ કરો.
chat-pins-replace = બદલો
chat-pins-cancel = રદ કરો
chat-undo = પૂર્વવત્ કરો

chat-reply-to = { $names } ને જવાબ આપો
chat-send = મોકલો (Ctrl+Enter)
chat-attach = જોડો
chat-attach-photo = ફોટો
chat-attach-file = ફાઇલ
chat-attach-library = ફાઇલોમાંથી
chat-attach-template = ટેમ્પ્લેટ
chat-attach-signature = હસ્તાક્ષર
chat-replying-to = { $name } ને જવાબ આપી રહ્યા છો
chat-reply-newest = સૌથી નવી મેઇલને જવાબ આપો

## The attach picker (paperclip > From Files)

picker-title = ફાઇલોમાંથી જોડો
picker-search = નામ, લોકો, વિષય શોધો
picker-search-drive = આ ડ્રાઇવમાં શોધો
picker-mail-files = મેઇલની ફાઇલો
picker-this-chat = આ વાર્તાલાપ
picker-this-computer = આ કમ્પ્યુટર…
picker-in-chat = આ વાર્તાલાપમાં
picker-recent = તાજેતરનાં
picker-preview = પૂર્વાવલોકન
picker-cancel = રદ કરો
picker-attach = જોડો
picker-attach-count = { $count } જોડો
picker-selected = { $count } પસંદ કર્યાં
picker-of-limit = { $limit } માંથી
picker-in-mail = મેઇલમાં { $size }
picker-drive-links = { $count ->
    [one] 1 Google Drive લિંક તરીકે
   *[other] { $count } Google Drive લિંક તરીકે
}
picker-onedrive-links = { $count ->
    [one] 1 OneDrive લિંક તરીકે
   *[other] { $count } OneDrive લિંક તરીકે
}
picker-over = { $size }, મેઇલમાં સમાઈ શકે તેવા { $limit } કરતાં વધુ
picker-getting = { $count ->
    [one] ડ્રાઇવમાંથી ફાઇલ મેળવી રહ્યા છીએ…
   *[other] ડ્રાઇવમાંથી { $count } ફાઇલો મેળવી રહ્યા છીએ…
}
picker-some-failed = { $count ->
    [one] એક ફાઇલ વાંચી શકાઈ નહીં
   *[other] { $count } ફાઇલો વાંચી શકાઈ નહીં
}
