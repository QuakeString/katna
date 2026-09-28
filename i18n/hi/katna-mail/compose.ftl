# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = नया मैसेज
compose-restore = पहले के साइज़ में लाएं
compose-minimize = छोटा करें
compose-exit-full-screen = फ़ुल स्क्रीन से बाहर निकलें
compose-open-window = नई विंडो में खोलें
compose-save-close = सेव करें और बंद करें
compose-back-to-mail = मेल विंडो पर वापस जाएं
compose-pop-out-reply = जवाब को अलग विंडो में खोलें
compose-edit-recipients = पाने वालों में बदलाव करें
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Bcc: { $names }
compose-show-trimmed = छिपा हुआ कॉन्टेंट दिखाएं
compose-hide-trimmed = छिपा हुआ कॉन्टेंट फिर से छिपाएं
compose-remove-trimmed = कोट किया गया टेक्स्ट हटाएं
compose-trimmed-removed = कोट किया गया टेक्स्ट हटाया गया

## Recipients and subject

compose-to = पाने वाले
compose-cc = Cc
compose-bcc = Bcc
compose-from = भेजने वाला
compose-from-choose = दूसरे खाते से भेजें
compose-recipients = पाने वाले
compose-subject = विषय

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = पहले खुले हुए मैसेज को भेजें या खारिज करें।
compose-bad-address = “{ $address }” ईमेल पता नहीं है।
compose-no-recipients = पाने वाला कम से कम एक व्यक्ति जोड़ें।
compose-attachments-too-large = अटैचमेंट { $size } के हैं; मेल सर्वर ज़्यादा से ज़्यादा { $limit } लेते हैं।
compose-no-account = मेल भेजने के लिए कोई खाता जोड़ें।
compose-past-time = आने वाले समय में से कोई समय चुनें।
compose-scheduling = शेड्यूल किया जा रहा है…
compose-sending = भेजा जा रहा है…
compose-scheduled = { $when } को भेजने के लिए शेड्यूल किया गया
compose-sent-archived = भेजा गया और संग्रह किया गया
compose-sent = मैसेज भेजा गया
compose-discarded = ड्राफ़्ट खारिज किया गया
compose-draft-saved = ड्राफ़्ट सेव किया गया
compose-draft-failed = ड्राफ़्ट सेव नहीं किया जा सका: { $error }
compose-draft-not-opened = ड्राफ़्ट खोला नहीं जा सका।

## Attachments

compose-picker-insert = डालें
compose-picker-attach = अटैच करें
compose-file-too-large = { $name } बहुत बड़ी है: एक मैसेज में ज़्यादा से ज़्यादा { $limit } भेजे जा सकते हैं।
compose-attachment-size = ({ $size })
compose-remove-attachment = अटैचमेंट हटाएं
compose-attachments-total = { $count ->
    [one] { $count } फ़ाइल, { $size }
   *[other] { $count } फ़ाइलें, { $size }
}
compose-drop-files = फ़ाइलें यहां छोड़ें
compose-drop-here = यहां छोड़ें
compose-paste-keep-formatting = फ़ॉर्मैटिंग रखें
compose-paste-table = टेबल
compose-paste-picture = तस्वीर
compose-paste-plain-text = सादा टेक्स्ट
compose-paste-inline = टेक्स्ट में
compose-paste-attachment = अटैचमेंट

## Encryption and signing (the toggles by the recipients)

compose-encrypt = एन्क्रिप्ट करें
compose-encrypted = एन्क्रिप्ट किया गया: सिर्फ़ पाने वाले इसे पढ़ सकते हैं
compose-sign = हस्ताक्षर करें
compose-signed = हस्ताक्षर किया गया: पाने वाले जांच सकते हैं कि यह आपकी ओर से है
compose-track = खोलना और क्लिक ट्रैक करें
compose-tracked = ट्रैक हो रहा है: आप देखेंगे कि हर पाने वाला इसे कब खोलता है या कोई लिंक खोलता है
compose-track-sign-in = खोलना और क्लिक ट्रैक करने के लिए Katna खाते में साइन इन करें
compose-receipt = पढ़ने की रसीद मांगें
compose-receipt-on = पढ़ने की रसीद मांगी गई: पाने वाले का ऐप उनसे इसे भेजने को कह सकता है
compose-delivery = डिलीवरी रसीद मांगें
compose-delivery-on = डिलीवरी रसीद मांगी गई: हर पाने वाले का सर्वर इसे स्वीकार करने पर आपका मेल सर्वर आपको ईमेल भेजेगा
compose-delivery-unavailable = आपका मेल सर्वर डिलीवरी रसीद नहीं भेजता

## Spelling

spell-no-dictionary = { $language } के लिए कोई वर्तनी शब्दकोश इंस्टॉल नहीं है (उदाहरण के लिए hunspell-en_us)।
spell-dictionary-error = वर्तनी शब्दकोश: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = “{ $words }” जोड़ें
grammar-remove = “{ $words }” हटाएं
grammar-ignore = अनदेखा करें

## Send checks (asked before a message goes out)

send-check-attachment-title = क्या आप फ़ाइलें अटैच करना चाहते थे?
send-check-attachment-text = आपने अटैचमेंट के बारे में लिखा है, लेकिन कुछ भी अटैच नहीं है।
send-check-attach = फ़ाइल अटैच करें
send-check-subject-title = बिना विषय के भेजें?
send-check-subject-text = इस मैसेज का कोई विषय नहीं है।
send-check-add-subject = विषय जोड़ें
send-check-send-anyway = फिर भी भेजें
recipient-not-valid = यह मान्य ईमेल पता नहीं है
recipient-show-address = पता दिखाएं
recipient-remove = हटाएं
recipient-bad-title = पता जांचें
recipient-bad-text = “{ $address }” मान्य ईमेल पता नहीं है। भेजने से पहले इसे ठीक करें या हटा दें।
recipient-bad-fix = ठीक करें
