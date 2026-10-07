# Katna Mail, Gujarati (ગુજરાતી).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = મેઇલ સર્વર

problems-signed-out = { $provider } એ Katna ને { $address } માંથી સાઇન આઉટ કર્યું. મેઇલ સિંક થવાનું બંધ થયું.
problems-password-refused = { $provider } એ { $address } નો પાસવર્ડ નકાર્યો. કદાચ તે બદલાયો હશે.
problems-no-answer = { $provider } { $address } માટે જવાબ આપતું નથી. Katna પ્રયાસ ચાલુ રાખે છે.
problems-offline = તમે ઑફલાઇન છો. તમારા મેઇલ હજી અહીં છે, અને તમે મોકલો તે મેઇલ તમે પાછા આવો ત્યાં સુધી રાહ જુએ છે.
problems-accounts-need-you = { $count ->
    [one] 1 એકાઉન્ટને તમારી જરૂર છે
   *[other] { $count } એકાઉન્ટને તમારી જરૂર છે
}
problems-show = બતાવો
problems-later = પછી
problems-new-password = નવો પાસવર્ડ
problems-try-again = ફરી પ્રયાસ કરો

## The New password card

problems-password-title = નવો પાસવર્ડ
problems-password-detail = { $provider } એ { $address } નો સેવ કરેલો પાસવર્ડ નકાર્યો. નવો લખો; Katna તેને રાખતા પહેલાં તપાસે છે.
problems-password-placeholder = પાસવર્ડ
problems-password-show = પાસવર્ડ બતાવો
problems-password-hide = પાસવર્ડ છુપાવો
problems-password-cancel = રદ કરો
problems-password-save = સેવ કરો
problems-password-checking = તપાસી રહ્યાં છીએ…
problems-password-refused-again = { $provider } એ આ પાસવર્ડ પણ નકાર્યો. તેને તપાસો અને ફરી પ્રયાસ કરો.
problems-password-saved = { $address } માટે પાસવર્ડ સેવ થયો. તમારા મેઇલ મેળવી રહ્યાં છીએ…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $address } ના મેઇલ સર્વરે { $count ->
    [one] એક મેસેજ ખસેડવાનું સ્વીકાર્યું નહીં, તેથી તે પાછો જ્યાં હતો ત્યાં છે.
   *[other] { $count } મેસેજ ખસેડવાનું સ્વીકાર્યું નહીં, તેથી તે પાછા જ્યાં હતા ત્યાં છે.
}
problems-refused-flags = { $address } ના મેઇલ સર્વરે { $count ->
    [one] એક મેસેજને ચિહ્નિત કરવાનું (વાંચેલો, તારાંકિત…) સ્વીકાર્યું નહીં, તેથી તે પહેલાં જેવો હતો તેવો છે.
   *[other] { $count } મેસેજને ચિહ્નિત કરવાનું (વાંચેલા, તારાંકિત…) સ્વીકાર્યું નહીં, તેથી તે પહેલાં જેવા હતા તેવા છે.
}
problems-refused-label = { $address } ના મેઇલ સર્વરે { $count ->
    [one] એક મેસેજનાં લેબલ બદલવાનું સ્વીકાર્યું નહીં, તેથી તે પહેલાં જેવો હતો તેવો છે.
   *[other] { $count } મેસેજનાં લેબલ બદલવાનું સ્વીકાર્યું નહીં, તેથી તે પહેલાં જેવા હતા તેવા છે.
}
problems-refused-delete = { $address } ના મેઇલ સર્વરે { $count ->
    [one] એક મેસેજ ડિલીટ કરવાનું સ્વીકાર્યું નહીં, તેથી તે પાછો આવ્યો છે.
   *[other] { $count } મેસેજ ડિલીટ કરવાનું સ્વીકાર્યું નહીં, તેથી તે પાછા આવ્યા છે.
}
problems-refused-other = { $address } ના મેઇલ સર્વરે { $count ->
    [one] એક ફેરફાર સ્વીકાર્યો નહીં, તેથી Katna એ તેને પહેલાં જેવો હતો તેવો કરી દીધો.
   *[other] { $count } ફેરફાર સ્વીકાર્યા નહીં, તેથી Katna એ તેમને પહેલાં જેવા હતા તેવા કરી દીધા.
}
problems-details = વિગતો

## Katna's background service (katna-daemon) isn't running

service-starting = Katna ની બૅકગ્રાઉન્ડ સેવા શરૂ થઈ રહી છે…
service-failed = Katna ની બૅકગ્રાઉન્ડ સેવા શરૂ થતી નથી, તેથી મેઇલ સિંક થતા નથી.
service-start-again = ફરી શરૂ કરો
service-started-again = Katna ની બૅકગ્રાઉન્ડ સેવા બંધ થઈ ગઈ હતી અને ફરી શરૂ કરવામાં આવી.
service-details-title = સેવા શા માટે શરૂ થતી નથી
service-details-body = આ કૉપિ કરો અને તમારા રિપોર્ટ સાથે મોકલો. તેમાં કોઈ મેઇલ કે પાસવર્ડ નથી.
service-details-copy = કૉપિ કરો
service-details-close = બંધ કરો
