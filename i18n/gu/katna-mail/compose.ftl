# Katna Mail, Gujarati (ગુજરાતી).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = નવો મેસેજ
compose-restore = મૂળ કદમાં લાવો
compose-minimize = નાનું કરો
compose-exit-full-screen = પૂર્ણ સ્ક્રીનમાંથી બહાર નીકળો
compose-open-window = નવી વિન્ડોમાં ખોલો
compose-save-close = સેવ કરીને બંધ કરો
compose-back-to-mail = મેઇલ વિન્ડો પર પાછા જાઓ
compose-pop-out-reply = જવાબ અલગ વિન્ડોમાં ખોલો
compose-edit-recipients = પ્રાપ્તકર્તાઓમાં ફેરફાર કરો
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Bcc: { $names }
compose-show-trimmed = ટૂંકાવેલી સામગ્રી બતાવો
compose-hide-trimmed = ટૂંકાવેલી સામગ્રી છુપાવો
compose-remove-trimmed = અવતરિત લખાણ દૂર કરો
compose-trimmed-removed = અવતરિત લખાણ દૂર કર્યું

## Recipients and subject

compose-to = પ્રતિ
compose-cc = Cc
compose-bcc = Bcc
compose-from = મોકલનાર
compose-from-choose = બીજા એકાઉન્ટમાંથી મોકલો
compose-recipients = પ્રાપ્તકર્તાઓ
compose-subject = વિષય

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = પહેલાં ખુલ્લો મેસેજ મોકલો અથવા કાઢી નાખો.
compose-bad-address = “{ $address }” ઇમેઇલ સરનામું નથી.
compose-no-recipients = ઓછામાં ઓછો એક પ્રાપ્તકર્તા ઉમેરો.
compose-attachments-too-large = જોડાણો { $size } છે; મેઇલ સર્વર વધુમાં વધુ { $limit } સ્વીકારે છે.
compose-no-account = મેઇલ મોકલવા માટે એક એકાઉન્ટ ઉમેરો.
compose-past-time = ભવિષ્યનો સમય પસંદ કરો.
compose-scheduling = શેડ્યૂલ થઈ રહ્યું છે…
compose-sending = મોકલાઈ રહ્યું છે…
compose-scheduled = { $when } માટે મોકલવાનું શેડ્યૂલ થયું
compose-sent-archived = મોકલ્યો અને આર્કાઇવ કર્યો
compose-sent = મેસેજ મોકલ્યો
compose-discarded = ડ્રાફ્ટ કાઢી નાખ્યો
compose-draft-saved = ડ્રાફ્ટ સેવ કર્યો
compose-draft-failed = ડ્રાફ્ટ સેવ કરી શકાયો નહીં: { $error }
compose-draft-not-opened = ડ્રાફ્ટ ખોલી શકાયો નહીં.

## Attachments

compose-picker-insert = દાખલ કરો
compose-picker-attach = જોડો
compose-file-too-large = { $name } ખૂબ મોટી છે: એક મેસેજમાં વધુમાં વધુ { $limit } સમાઈ શકે.
compose-attachment-size = ({ $size })
compose-remove-attachment = જોડાણ દૂર કરો
compose-attachments-total = { $count ->
    [one] { $count } ફાઇલ, { $size }
   *[other] { $count } ફાઇલ, { $size }
}
compose-drop-files = ફાઇલો અહીં મૂકો
compose-drop-here = અહીં મૂકો
compose-paste-keep-formatting = ફૉર્મેટિંગ રાખો
compose-paste-table = કોષ્ટક
compose-paste-picture = ચિત્ર
compose-paste-plain-text = સાદો ટેક્સ્ટ
compose-paste-inline = ટેક્સ્ટમાં
compose-paste-attachment = જોડાણ

## Encryption and signing (the toggles by the recipients)

compose-encrypt = એન્ક્રિપ્ટ કરો
compose-encrypted = એન્ક્રિપ્ટ કરેલો: માત્ર પ્રાપ્તકર્તાઓ જ તે વાંચી શકે
compose-sign = હસ્તાક્ષર કરો
compose-signed = હસ્તાક્ષરિત: પ્રાપ્તકર્તાઓ ચકાસી શકે કે તે તમારા તરફથી છે
compose-track = ખોલવાનું અને ક્લિક ટ્રૅક કરો
compose-tracked = ટ્રૅક થાય છે: દરેક પ્રાપ્તકર્તા તેને ક્યારે ખોલે છે કે લિંક ખોલે છે, તે તમે જોશો
compose-track-clicks = લિંક ક્લિક ટ્રૅક કરો (સાદા ટેક્સ્ટમાં ખોલવાનું બતાવી શકાતું નથી)
compose-tracked-clicks = ટ્રૅક થાય છે: દરેક પ્રાપ્તકર્તા ક્યારે લિંક ખોલે છે, તે તમે જોશો
compose-track-sign-in = ખોલવાનું અને ક્લિક ટ્રૅક કરવા Katna એકાઉન્ટમાં સાઇન ઇન કરો
compose-receipt = વાંચ્યાની રસીદ માગો
compose-receipt-on = વાંચ્યાની રસીદ માગી છે: પ્રાપ્તકર્તાની ઍપ તેમને રસીદ મોકલવા કહી શકે છે
compose-delivery = ડિલિવરી રસીદ માગો
compose-delivery-on = ડિલિવરી રસીદ માગી છે: દરેક પ્રાપ્તકર્તાનું સર્વર તેને સ્વીકારે ત્યારે તમારું મેઇલ સર્વર તમને ઇમેઇલ મોકલશે
compose-delivery-unavailable = તમારું મેઇલ સર્વર ડિલિવરી રસીદ મોકલતું નથી

## Spelling

spell-no-dictionary = { $language } માટે કોઈ જોડણી શબ્દકોશ ઇન્સ્ટૉલ કરેલો નથી (ઉદાહરણ તરીકે hunspell-en_us).
spell-dictionary-error = જોડણી શબ્દકોશ: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = “{ $words }” ઉમેરો
grammar-remove = “{ $words }” કાઢી નાખો
grammar-ignore = અવગણો

## Send checks (asked before a message goes out)

send-check-attachment-title = શું તમે ફાઇલો જોડવા માગતા હતા?
send-check-attachment-text = તમે જોડાણ વિશે લખ્યું છે, પણ કંઈ જોડેલું નથી.
send-check-attach = ફાઇલ જોડો
send-check-subject-title = વિષય વગર મોકલવો છે?
send-check-subject-text = આ મેસેજનો કોઈ વિષય નથી.
send-check-add-subject = વિષય ઉમેરો
send-check-send-anyway = તો પણ મોકલો
recipient-not-valid = માન્ય ઇમેઇલ સરનામું નથી
recipient-show-address = સરનામું બતાવો
recipient-remove = કાઢી નાખો
recipient-bad-title = સરનામું તપાસો
recipient-bad-text = “{ $address }” માન્ય ઇમેઇલ સરનામું નથી. મોકલતા પહેલાં તેને સુધારો અથવા કાઢી નાખો.
recipient-bad-fix = સુધારો
