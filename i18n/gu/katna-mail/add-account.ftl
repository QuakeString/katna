# Katna Mail, Gujarati (ગુજરાતી).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = મેઇલ એકાઉન્ટ ઉમેરો
add-account-providers-intro = તમારો મેઇલ પ્રદાતા પસંદ કરો. બાકીનું Katna શોધી લેશે.
add-account-provider-other = અન્ય મેઇલ
add-account-provider-other-detail = કોઈપણ IMAP અથવા POP3 એકાઉન્ટ
add-account-provider-google-detail = Gmail અને Google Workspace
add-account-provider-microsoft-detail = Outlook અને Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = { $provider } માં સાઇન ઇન કરો
add-account-form-title-other = તમારું મેઇલ એકાઉન્ટ
add-account-form-intro = Katna તમારો પાસવર્ડ તમારી સિસ્ટમના કીરિંગમાં રાખે છે.
add-account-looking = { $address } ના મેઇલ સર્વર શોધી રહ્યાં છીએ…
add-account-address-intro = તમારું ઇમેઇલ સરનામું દાખલ કરો. Katna તમારા માટે સર્વર શોધી લેશે.
add-account-servers-title = સર્વર સેટિંગ
add-account-servers-intro = Katna { $address } માટે મેઇલ ક્યાંથી વાંચે છે અને મોકલે છે.
add-account-signing-in = સાઇન ઇન કરી રહ્યાં છીએ…
add-account-browser-title = તમારા બ્રાઉઝરમાં ચાલુ રાખો
add-account-browser-intro = Katna એ તમારા બ્રાઉઝરમાં { $provider } નું સાઇન-ઇન પેજ ખોલ્યું છે. ત્યાં સાઇન ઇન કરો અને Katna ને તમારી મેઇલ વાંચવા અને મોકલવાની મંજૂરી આપો, પછી અહીં પાછા આવો.
add-account-browser-hint = કોઈ પેજ ન ખૂલ્યું? તમારા બ્રાઉઝરની વિન્ડો તપાસો, અથવા પાછા જઈને ફરી પ્રયાસ કરો.
add-account-stage-browser = તમે બ્રાઉઝરમાં સાઇન ઇન કરો તેની રાહ જોઈ રહ્યાં છીએ…
add-account-stage-signing-in-at = { $server } પર સાઇન ઇન થઈ રહ્યું છે…
add-account-help-app-password-link = ઍપ પાસવર્ડ કેવી રીતે બનાવવો
add-account-help-turn-on-imap = { $provider } તેના વેબ મેઇલના સેટિંગમાં IMAP અને POP3 ઍક્સેસ ચાલુ કર્યા પછી જ મેઇલ ઍપને અંદર આવવા દે છે.
add-account-help-turn-on-imap-link = તેને કેવી રીતે ચાલુ કરવું

## Add a mail account: fields

add-account-field-address = ઇમેઇલ સરનામું
add-account-receive-with = મેઇલ આનાથી મેળવો
add-account-imap-about = IMAP તમારા મેઇલ અને ફોલ્ડર સર્વર પર રાખે છે, દરેક ડિવાઇસ પર એકસરખા. શક્ય હોય ત્યારે તે પસંદ કરો.
add-account-pop3-about = POP3 તમારા મેઇલ આ કમ્પ્યુટર પર ડાઉનલોડ કરે છે. અહીં તમે વાંચો કે ખસેડો તે મેઇલ સર્વર અને તમારાં અન્ય ડિવાઇસ પર જેમ છે તેમ રહે છે.
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
add-account-done = થઈ ગયું
add-account-another = બીજું એકાઉન્ટ ઉમેરો
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
add-account-smtp-not-found = Katna ને તમારા મેઇલ ક્યાંથી વાંચવા તે મળ્યું, પણ ક્યાંથી મોકલવા તે નહીં. આઉટગોઇંગ સર્વર દાખલ કરો.

## Add a mail account: the last step

add-account-done-title = તમારું એકાઉન્ટ તૈયાર છે
add-account-done-intro = Katna હવે તમારા મેઇલ મેળવી રહ્યું છે. નવા મેઇલ આવતાંની સાથે દેખાશે.
add-account-done-sign-in = સાઇન ઇન
add-account-done-signed-in-with = { $provider } વડે, તમારા બ્રાઉઝરમાં
add-account-done-receiving = મેઇલ મેળવવા
add-account-done-sending = મેઇલ મોકલવા
add-account-done-on-server = સર્વર પરના મેઇલ
add-account-done-kept = તમે Katna માં ડિલીટ કરો ત્યાં સુધી રાખવામાં આવે છે
add-account-done-pop3-hint = સર્વર પરના મેઇલનું શું થાય તે સેટિંગ > એકાઉન્ટમાં બદલો.
add-account-done-zoho-title = કાર્યો અને કૅલેન્ડર
add-account-done-zoho-about = Zoho આને મેઇલથી અલગ રાખે છે. તેમને Katna માં લાવવા એક વાર Zoho વડે સાઇન ઇન કરો.
add-account-done-linked = કાર્યો અને કૅલેન્ડર જોડાયાં

## The account menu (from the account button on the top bar)

add-account-menu-another = બીજું એકાઉન્ટ ઉમેરો
app-menu = મુખ્ય મેનૂ
app-menu-back = પાછળ
