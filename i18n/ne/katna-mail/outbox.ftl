# Katna Mail, Nepali (नेपाली).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = { $reason } भएकाले पठाइएन।
outbox-retrying = { $reason } भएकाले अझै पठाइएको छैन। Katna आफैँ फेरि प्रयास गर्छ।
outbox-waiting-sign-in = तपाईं { $address } मा फेरि साइन इन गर्नुहुने प्रतीक्षामा। त्यसपछि जान्छ।
outbox-waiting-password = { $address } को नयाँ पासवर्डको प्रतीक्षामा। त्यसपछि जान्छ।
outbox-waiting-connection = जडानको प्रतीक्षामा। तपाईं फेरि अनलाइन भएपछि जान्छ।

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = यसमा कुनै प्रापक नभएको
outbox-reason-address = पठाइएको एउटा ठेगाना अस्तित्वमा नभएको
outbox-reason-too-large = यो मेल सर्भरका लागि धेरै ठूलो
outbox-reason-blocked = मेल सर्भरले यसलाई रोकेको
outbox-reason-gone = यो कम्प्युटरमा भएको यसको प्रतिलिपि हराएको
outbox-reason-refused = मेल सर्भरले यसलाई अस्वीकार गरेको

## Buttons and notes

outbox-try-again = फेरि प्रयास गर्नुहोस्
outbox-edit = सम्पादन गर्नुहोस्
outbox-delete = मेटाउनुहोस्
outbox-deleted = आउटबक्सबाट मेटाइयो
outbox-sending-again = फेरि पठाउँदै…

outbox-snackbar-not-sent = { $reason } भएकाले “{ $subject }” पठाइएन।
outbox-open = आउटबक्स
