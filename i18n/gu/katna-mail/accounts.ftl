# Katna Mail, Gujarati (ગુજરાતી).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = ફોલ્ડર પેન
accounts-folder-pane-detail = ડાબી બાજુની પેન કયાં એકાઉન્ટનાં ફોલ્ડર બતાવે.
accounts-shown-one = એક સમયે એક એકાઉન્ટ; એકાઉન્ટ કાર્ડમાં બદલો
accounts-shown-all = બધાં એકાઉન્ટ, એક પછી એક
accounts-unified = એકીકૃત ઇનબૉક્સ
accounts-unified-switch = બધાં એકાઉન્ટના મેઇલ એકસાથે બતાવો
accounts-unified-switch-detail = “બધાં એકાઉન્ટ” ફોલ્ડર પેનમાં સૌથી ઉપર હોય છે, જેમાં દરેક એકાઉન્ટનાં ઇનબૉક્સ, મોકલેલા મેઇલ અને વધુ એક જ સૂચિમાં હોય છે. તેની નીચેનાં એકાઉન્ટ શરૂઆતમાં સંકેલાયેલાં હોય છે.
accounts-row = એકાઉન્ટ
accounts-row-detail = ફોલ્ડર પેન અને એકાઉન્ટ મેનૂ એકાઉન્ટને આ ક્રમમાં બતાવે છે; પહેલું ડિફૉલ્ટ છે. એકાઉન્ટ કાઢી નાખવાથી આ કમ્પ્યુટર પરની Katna ની તેના મેઇલની કૉપિ ડિલીટ થાય છે. મેઇલ સર્વર પર રહે છે.
accounts-none = હજી કોઈ એકાઉન્ટ નથી.
accounts-pop3-row = સર્વર પરના મેઇલ
accounts-pop3-row-detail = POP3 એકાઉન્ટ મેઇલને આ કમ્પ્યુટર પર ડાઉનલોડ કરે છે. પછી સર્વર પરની નકલનું શું થાય તે પસંદ કરો.
accounts-pop3-with-katna = હું Katna માં ડિલીટ કરું ત્યાં સુધી રાખો
accounts-pop3-at-once = ડાઉનલોડ થતાં જ ડિલીટ કરો
accounts-pop3-after-days = { $count ->
    [one] { $count } દિવસ પછી ડિલીટ કરો
   *[other] { $count } દિવસ પછી ડિલીટ કરો
}
accounts-pop3-never = ક્યારેય ડિલીટ ન કરો
accounts-pop3-days-less = ઓછા દિવસ
accounts-pop3-days-more = વધુ દિવસ
accounts-kind-imported = આયાત કરેલું
accounts-picture-reset = ડેસ્કટૉપ ચિત્રનો ઉપયોગ કરો
accounts-picture-change = ચિત્ર બદલો
accounts-picture-remove = ચિત્ર કાઢી નાખો
account-color-red = લાલ
account-color-pink = ગુલાબી
account-color-magenta = મજેન્ટા
account-color-brown = કથ્થઈ
account-color-olive = ઑલિવ
account-color-teal = ટીલ
account-color-indigo = ઘેરો વાદળી
account-color-slate = સ્લેટ
account-color-menu = રંગ
accounts-rename = નામ બદલો
accounts-name-save = સેવ કરો
accounts-name-cancel = રદ કરો
accounts-name-placeholder = તમારું નામ
accounts-rename-failed = એકાઉન્ટનું નામ બદલી શકાયું નહીં: { $error }
accounts-move-up = ઉપર ખસેડો
accounts-move-down = નીચે ખસેડો
accounts-drag = ક્રમ બદલવા માટે ખેંચો
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
reset-cache-about = Katna એ ડાઉનલોડ કરેલા મેઇલ અને જોડાણ, મોકલનારનાં ચિત્રો અને શોધ ઇન્ડેક્સ ડિલીટ કરે છે, પછી તાજેતરના મેઇલ ફરી ડાઉનલોડ કરે છે. એકાઉન્ટ, સેટિંગ અને ફક્ત આ કમ્પ્યુટર પર હોય એવા મેઇલ રહે છે.
reset-cache-button = કૅશ રીસેટ કરો
reset-cache-title = કૅશ રીસેટ કરીએ?
reset-cache-deleted = ડિલીટ થશે, પછી ફરી ડાઉનલોડ થશે:
reset-cache-mail = તમારા IMAP સર્વર પરથી ડાઉનલોડ કરેલા મેઇલ અને જોડાણ: તાજેતરના મેઇલ હમણાં ફરી ડાઉનલોડ થાય છે, જૂના મેઇલ તમે ખોલો ત્યારે
reset-cache-index = શોધ ઇન્ડેક્સ, જે તરત ફરી બને છે
reset-cache-pictures = મોકલનારનાં ચિત્રો
reset-cache-kept = રહે છે: તમારાં એકાઉન્ટ, પાસવર્ડ અને સેટિંગ; તારા, લેબલ, વાંચેલાનાં ચિહ્નો અને પિન; ડ્રાફ્ટ, આઉટબૉક્સ અને હજી સર્વર પર ન પહોંચેલા ફેરફારો; અને POP3 એકાઉન્ટ કે આયાત કરેલી ફાઇલોના મેઇલ, જેની કદાચ બીજી કોઈ નકલ ન હોય. તમારા મેઇલ સર્વર પર કંઈ બદલાતું નથી.
reset-cache-confirm = કૅશ રીસેટ કરો
reset-cache-busy = રીસેટ કરી રહ્યાં છીએ…
reset-cache-done = કૅશ રીસેટ થઈ ગઈ. તાજેતરના મેઇલ ફરી ડાઉનલોડ થઈ રહ્યા છે.
reset-cache-done-freed = કૅશ રીસેટ થઈ ગઈ અને { $size } ખાલી થઈ. તાજેતરના મેઇલ ફરી ડાઉનલોડ થઈ રહ્યા છે.
