# Katna Mail, Gujarati (ગુજરાતી).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = મેઇલ એકાઉન્ટ ઉમેરો
add-account-looking = { $address } ના મેઇલ સર્વર શોધી રહ્યાં છીએ…
add-account-address-intro = તમારું ઇમેઇલ સરનામું દાખલ કરો. Katna તમારા માટે સર્વર શોધી લેશે.
add-account-servers-title = સર્વર સેટિંગ
add-account-servers-intro = Katna { $address } માટે મેઇલ ક્યાંથી વાંચે છે અને મોકલે છે.
add-account-signing-in = સાઇન ઇન કરી રહ્યાં છીએ…
add-account-browser-title = તમારા બ્રાઉઝરમાં ચાલુ રાખો
add-account-browser-intro = Katna એ તમારા બ્રાઉઝરમાં { $provider } નું સાઇન-ઇન પેજ ખોલ્યું છે. ત્યાં સાઇન ઇન કરો અને Katna ને તમારી મેઇલ વાંચવા અને મોકલવાની મંજૂરી આપો, પછી અહીં પાછા આવો.
add-account-browser-hint = કોઈ પેજ ન ખૂલ્યું? તમારા બ્રાઉઝરની વિન્ડો તપાસો, અથવા પાછા જઈને ફરી પ્રયાસ કરો.

## Add a mail account: fields

add-account-field-address = ઇમેઇલ સરનામું
add-account-incoming = આવતી મેઇલ ({ $protocol })
add-account-outgoing = જતી મેઇલ ({ $protocol })
add-account-field-server = સર્વર
add-account-field-port = પોર્ટ
add-account-security-none = કોઈ નહીં
add-account-security-none-warning = એન્ક્રિપ્ટ કરેલું નથી: તમારો પાસવર્ડ અને મેઇલ રસ્તામાં વાંચી શકાય છે.
add-account-field-username = વપરાશકર્તા નામ
add-account-field-password = પાસવર્ડ
add-account-show-password = પાસવર્ડ બતાવો
add-account-app-password-hint = { $provider } ને અહીં ઍપ પાસવર્ડની જરૂર છે, તમે વેબ પર વાપરો છો તે નહીં. તમારા { $provider } એકાઉન્ટના સુરક્ષા સેટિંગમાં એક બનાવો.
add-account-field-name = તમારું નામ (વૈકલ્પિક)
add-account-name-hint = તમે જેમને લખો છો તે લોકોને બતાવાય છે.
add-account-servers-pair = { $imap } અને { $smtp }
add-account-servers-found = { $source ->
    [built-in] સર્વર: { $servers }, Katna ની પ્રદાતાઓની સૂચિમાં મળ્યા.
    [provider] સર્વર: { $servers }, તમારા પ્રદાતાના સેટિંગમાં મળ્યા.
    [ispdb] સર્વર: { $servers }, Thunderbird ની પ્રદાતાઓની સૂચિમાં મળ્યા.
    [dns] સર્વર: { $servers }, તમારા ડોમેનના DNS રેકોર્ડમાં મળ્યા.
   *[other] સર્વર: { $servers }, અનુમાનથી; સાઇન ઇન નિષ્ફળ જાય તો તેમને તપાસો.
}
add-account-servers-entered = સર્વર: { $servers }, દાખલ કર્યા મુજબ.
add-account-sign-in-with = { $provider } વડે સાઇન ઇન કરો
add-account-sign-in-instead = તેના બદલે { $provider } વડે સાઇન ઇન કરો

## Add a mail account: buttons

add-account-servers-button = સર્વર સેટિંગ
add-account-back = પાછળ
add-account-add = એકાઉન્ટ ઉમેરો
add-account-cancel = રદ કરો

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] આવતી મેઇલનું સર્વર દાખલ કરો.
   *[outgoing] જતી મેઇલનું સર્વર દાખલ કરો.
}
add-account-server-space = { $kind ->
    [incoming] આવતી મેઇલના સર્વરના નામમાં સ્પેસ છે.
   *[outgoing] જતી મેઇલના સર્વરના નામમાં સ્પેસ છે.
}
add-account-port-invalid = { $kind ->
    [incoming] આવતી મેઇલનો પોર્ટ { $min } થી { $max } સુધીની સંખ્યા હોવો જોઈએ.
   *[outgoing] જતી મેઇલનો પોર્ટ { $min } થી { $max } સુધીની સંખ્યા હોવો જોઈએ.
}
add-account-address-empty = ઇમેઇલ સરનામું દાખલ કરો.
add-account-address-invalid = { $example } જેવું ઇમેઇલ સરનામું દાખલ કરો.
add-account-not-found = Katna ને { $address } માટે સર્વર મળ્યા નહીં, તેથી તેણે સામાન્ય નામો ભરી દીધાં. તમારા પ્રદાતા પાસેથી તેમની ખાતરી કરો.
add-account-password-empty = પાસવર્ડ દાખલ કરો.
add-account-name-is-password = નામ પાસવર્ડ જેવું જ છે. તેના બદલે ત્યાં તમારું નામ લખો, જેમ લોકોએ તે જોવું જોઈએ.
add-account-app-password-refused = { $provider } એ પાસવર્ડ નકાર્યો. તેને ઍપ પાસવર્ડની જરૂર છે, તમે વેબ પર વાપરો છો તે નહીં.
add-account-password-refused = સર્વરે પાસવર્ડ નકાર્યો. તેને તપાસો અને ફરી પ્રયાસ કરો.
add-account-sign-in-refused = { $provider } એ Katna ને અંદર આવવા ન દીધું. ફરી પ્રયાસ કરો, અને તમારી મેઇલની ઍક્સેસની મંજૂરી આપો.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Katna ની આ નકલ હજી Microsoft એકાઉન્ટમાં સાઇન ઇન કરી શકતી નથી.
    [Google] Katna ની આ નકલ હજી Google એકાઉન્ટમાં સાઇન ઇન કરી શકતી નથી.
   *[other] આ પ્રદાતા ફક્ત પોતાના પેજ પર જ સાઇન ઇન કરવા દે છે, જે Katna તેના માટે હજી કરી શકતું નથી.
}

## The account menu (from the account button on the top bar)

add-account-menu-another = બીજું એકાઉન્ટ ઉમેરો
app-menu = મુખ્ય મેનૂ
app-menu-back = પાછળ
