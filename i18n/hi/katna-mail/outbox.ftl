# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = नहीं भेजा गया, क्योंकि { $reason }।
outbox-retrying = अभी नहीं भेजा गया, क्योंकि { $reason }। Katna अपने-आप फिर से कोशिश करता है।
outbox-waiting-sign-in = { $address } में आपके फिर से साइन इन करने का इंतज़ार है। तब यह चला जाएगा।
outbox-waiting-password = { $address } के नए पासवर्ड का इंतज़ार है। तब यह चला जाएगा।
outbox-waiting-connection = कनेक्शन का इंतज़ार है। आपके फिर से ऑनलाइन होने पर यह चला जाएगा।

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = इसमें कोई पाने वाला नहीं है
outbox-reason-address = जिस पते पर यह भेजा गया है, वह मौजूद नहीं है
outbox-reason-too-large = यह मेल सर्वर के लिए बहुत बड़ा है
outbox-reason-blocked = मेल सर्वर ने इसे ब्लॉक कर दिया
outbox-reason-gone = इस कंप्यूटर पर इसकी कॉपी नहीं रही
outbox-reason-refused = मेल सर्वर ने इसे अस्वीकार कर दिया

## Buttons and notes

outbox-try-again = फिर से कोशिश करें
outbox-edit = बदलाव करें
outbox-delete = मिटाएं
outbox-deleted = आउटबॉक्स से मिटाया गया
outbox-sending-again = फिर से भेजा जा रहा है…

outbox-snackbar-not-sent = “{ $subject }” नहीं भेजा गया, क्योंकि { $reason }।
outbox-open = आउटबॉक्स
