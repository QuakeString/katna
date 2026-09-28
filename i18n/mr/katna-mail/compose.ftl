# Katna Mail, Marathi (मराठी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = नवीन मेसेज
compose-restore = पूर्वस्थितीत आणा
compose-minimize = लहान करा
compose-exit-full-screen = पूर्ण स्क्रीनमधून बाहेर पडा
compose-open-window = नवीन विंडोमध्ये उघडा
compose-save-close = सेव्ह करा आणि बंद करा
compose-back-to-mail = मेल विंडोवर परत जा
compose-pop-out-reply = उत्तर वेगळ्या विंडोमध्ये उघडा
compose-show-trimmed = कापलेला मजकूर दाखवा

## Recipients and subject

compose-to = प्रति
compose-cc = Cc
compose-bcc = Bcc
compose-from = प्रेषक
compose-from-choose = दुसऱ्या खात्यावरून पाठवा
compose-recipients = प्राप्तकर्ते
compose-subject = विषय

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = आधी उघडा असलेला मेसेज पाठवा किंवा टाकून द्या.
compose-bad-address = “{ $address }” हा ईमेल पत्ता नाही.
compose-no-recipients = किमान एक प्राप्तकर्ता जोडा.
compose-attachments-too-large = अटॅचमेंट { $size } आहेत; मेल सर्व्हर जास्तीत जास्त { $limit } स्वीकारतात.
compose-no-account = मेल पाठवण्यासाठी एखादे खाते जोडा.
compose-past-time = भविष्यातील वेळ निवडा.
compose-scheduling = शेड्यूल करत आहे…
compose-sending = पाठवत आहे…
compose-scheduled = { $when } ला पाठवणे शेड्यूल केले
compose-sent-archived = पाठवले आणि संग्रहित केले
compose-sent = मेसेज पाठवला
compose-discarded = मसुदा टाकून दिला
compose-draft-saved = मसुदा सेव्ह केला
compose-draft-failed = मसुदा सेव्ह करता आला नाही: { $error }
compose-draft-not-opened = मसुदा उघडता आला नाही.

## Attachments

compose-picker-insert = घाला
compose-picker-attach = अटॅच करा
compose-file-too-large = { $name } खूप मोठी आहे: एका मेसेजमध्ये जास्तीत जास्त { $limit } पाठवता येते.
compose-attachment-size = ({ $size })
compose-remove-attachment = अटॅचमेंट काढा
compose-attachments-total = { $count ->
    [one] { $count } फाइल, { $size }
   *[other] { $count } फाइल्स, { $size }
}
compose-drop-files = फाइल्स येथे सोडा
compose-drop-here = येथे सोडा
compose-paste-keep-formatting = फॉरमॅटिंग ठेवा
compose-paste-table = टेबल
compose-paste-picture = चित्र
compose-paste-plain-text = साधा मजकूर
compose-paste-inline = मजकुरात
compose-paste-attachment = अटॅचमेंट

## Encryption and signing (the toggles by the recipients)

compose-encrypt = एन्क्रिप्ट करा
compose-encrypted = एन्क्रिप्ट केलेले: फक्त प्राप्तकर्तेच वाचू शकतात
compose-sign = स्वाक्षरी करा
compose-signed = स्वाक्षरी केलेले: हा मेसेज तुमच्याकडूनच आहे हे प्राप्तकर्ते तपासू शकतात
compose-track = उघडणे आणि क्लिक ट्रॅक करा
compose-tracked = ट्रॅक होत आहे: प्रत्येक प्राप्तकर्ता तो कधी उघडतो किंवा लिंक उघडतो ते तुम्हाला दिसेल
compose-track-unavailable = स्वाक्षरी केलेले, एन्क्रिप्ट केलेले आणि साध्या मजकुरातील मेल ट्रॅक करता येत नाहीत
compose-track-sign-in = उघडणे आणि क्लिक ट्रॅक करण्यासाठी Katna खात्यात साइन इन करा
compose-receipt = वाचल्याची पावती मागवा
compose-receipt-on = वाचल्याची पावती मागवली: प्राप्तकर्त्याचे ॲप त्यांना ती पाठवण्यास सांगू शकते
compose-delivery = पोहोचल्याची पावती मागवा
compose-delivery-on = पोहोचल्याची पावती मागवली: प्रत्येक प्राप्तकर्त्याचा सर्व्हर मेसेज स्वीकारतो तेव्हा तुमचा मेल सर्व्हर तुम्हाला ईमेल पाठवेल
compose-delivery-unavailable = तुमचा मेल सर्व्हर पोहोचल्याच्या पावत्या पाठवत नाही

## Spelling

spell-no-dictionary = { $language } साठी शुद्धलेखन शब्दकोश इंस्टॉल केलेला नाही (उदाहरणार्थ hunspell-en_us).
spell-dictionary-error = शुद्धलेखन शब्दकोश: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = “{ $words }” जोडा
grammar-remove = “{ $words }” काढा
grammar-ignore = दुर्लक्ष करा

## Send checks (asked before a message goes out)

send-check-attachment-title = तुम्हाला फाइल्स अटॅच करायच्या होत्या का?
send-check-attachment-text = तुम्ही अटॅचमेंटचा उल्लेख केला, पण काहीही अटॅच केलेले नाही.
send-check-attach = फाइल अटॅच करा
send-check-subject-title = विषयाशिवाय पाठवायचा?
send-check-subject-text = या मेसेजला विषय नाही.
send-check-add-subject = विषय जोडा
send-check-send-anyway = तरीही पाठवा
recipient-not-valid = वैध ईमेल पत्ता नाही
recipient-show-address = पत्ता दाखवा
recipient-remove = काढा
recipient-bad-title = पत्ता तपासा
recipient-bad-text = “{ $address }” हा वैध ईमेल पत्ता नाही. पाठवण्यापूर्वी तो दुरुस्त करा किंवा काढून टाका.
recipient-bad-fix = दुरुस्त करा
