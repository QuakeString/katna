# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = मेल सर्वर

problems-signed-out = { $provider } ने Katna को { $address } से साइन आउट कर दिया। मेल सिंक होना बंद हो गया।
problems-password-refused = { $provider } ने { $address } का पासवर्ड अस्वीकार कर दिया। शायद यह बदल गया है।
problems-no-answer = { $provider } { $address } के लिए जवाब नहीं दे रहा। Katna कोशिश करता रहेगा।
problems-offline = आप ऑफ़लाइन हैं। आपका मेल अभी भी यहां है, और आपका भेजा मेल आपके वापस ऑनलाइन होने तक इंतज़ार करता है।
problems-accounts-need-you = { $count ->
    [one] 1 खाते को आपकी ज़रूरत है
   *[other] { $count } खातों को आपकी ज़रूरत है
}
problems-show = दिखाएं
problems-later = बाद में
problems-new-password = नया पासवर्ड
problems-try-again = फिर से कोशिश करें

## The New password card

problems-password-title = नया पासवर्ड
problems-password-detail = { $provider } ने { $address } का सेव किया गया पासवर्ड अस्वीकार कर दिया। नया पासवर्ड टाइप करें; Katna रखने से पहले उसे जाँचता है।
problems-password-placeholder = पासवर्ड
problems-password-show = पासवर्ड दिखाएं
problems-password-hide = पासवर्ड छिपाएं
problems-password-cancel = रद्द करें
problems-password-save = सेव करें
problems-password-checking = जाँचा जा रहा है…
problems-password-refused-again = { $provider } ने यह पासवर्ड भी अस्वीकार कर दिया। इसे जाँचें और फिर से कोशिश करें।
problems-password-saved = { $address } का पासवर्ड सेव किया गया। आपका मेल लाया जा रहा है…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $address } के मेल सर्वर ने { $count ->
    [one] एक मैसेज को ले जाना स्वीकार नहीं किया, इसलिए वह वापस अपनी जगह पर है।
   *[other] { $count } मैसेज को ले जाना स्वीकार नहीं किया, इसलिए वे वापस अपनी जगह पर हैं।
}
problems-refused-flags = { $address } के मेल सर्वर ने { $count ->
    [one] एक मैसेज को मार्क करना (पढ़ा गया, तारांकित…) स्वीकार नहीं किया, इसलिए वह पहले जैसा है।
   *[other] { $count } मैसेज को मार्क करना (पढ़ा गया, तारांकित…) स्वीकार नहीं किया, इसलिए वे पहले जैसे हैं।
}
problems-refused-label = { $address } के मेल सर्वर ने { $count ->
    [one] एक मैसेज के लेबल बदलना स्वीकार नहीं किया, इसलिए वह पहले जैसा है।
   *[other] { $count } मैसेज के लेबल बदलना स्वीकार नहीं किया, इसलिए वे पहले जैसे हैं।
}
problems-refused-delete = { $address } के मेल सर्वर ने { $count ->
    [one] एक मैसेज को मिटाना स्वीकार नहीं किया, इसलिए वह वापस आ गया है।
   *[other] { $count } मैसेज को मिटाना स्वीकार नहीं किया, इसलिए वे वापस आ गए हैं।
}
problems-refused-other = { $address } के मेल सर्वर ने { $count ->
    [one] एक बदलाव स्वीकार नहीं किया, इसलिए Katna ने उसे पहले जैसा कर दिया।
   *[other] { $count } बदलाव स्वीकार नहीं किए, इसलिए Katna ने उन्हें पहले जैसा कर दिया।
}
problems-details = विवरण

## Katna's background service (katna-daemon) isn't running

service-starting = Katna की बैकग्राउंड सेवा शुरू हो रही है…
service-failed = Katna की बैकग्राउंड सेवा शुरू नहीं हो रही, इसलिए मेल सिंक नहीं हो रहा।
service-start-again = फिर से शुरू करें
service-started-again = Katna की बैकग्राउंड सेवा बंद हो गई थी और उसे फिर से शुरू किया गया।
service-details-title = सेवा शुरू क्यों नहीं हो रही
service-details-body = इसे कॉपी करें और अपनी रिपोर्ट के साथ भेजें। इसमें कोई मेल या पासवर्ड नहीं है।
service-details-copy = कॉपी करें
service-details-close = बंद करें
service-not-running = Katna की बैकग्राउंड सेवा नहीं चल रही है।
service-no-answer = Katna की बैकग्राउंड सेवा ने जवाब नहीं दिया: { $error }
service-no-session = कोई D-Bus सेशन नहीं: { $error }

## Safe mode: an update left Katna's background service unable to start

safe-line = अपडेट में समस्या के बाद Katna सेफ़ मोड में है, इसलिए मेल सिंक नहीं हो रहा।
safe-try-again = फिर से कोशिश करें
safe-restore = वापस लाएं
safe-restoring = { $when } से आपका डेटा वापस लाया जा रहा है…
safe-restored = { $when } से आपका डेटा वापस लाया गया। पहले जो था वह एक फ़ोल्डर में रखा गया है।
safe-show-folder = फ़ोल्डर दिखाएं
safe-restore-failed = आपका डेटा वापस नहीं लाया जा सका: { $error }
safe-restore-title = अपडेट से पहले का अपना डेटा वापस लाएं?
safe-restore-body = Katna आपकी चुनी हुई कॉपी पर लौट जाता है। उसके बाद आया मेल आपके खातों से फिर से डाउनलोड होता है।
safe-restore-none = अभी कोई कॉपी नहीं है। हर अपडेट से पहले, जो आपका डेटा बदलता है, Katna एक कॉपी बनाता है।
safe-restore-keep = अभी जो है, जिसमें न भेजा गया मेल, ड्राफ़्ट और अभी सिंक न हुए बदलाव शामिल हैं, वह पहले एक फ़ोल्डर में रखा जाता है, इसलिए कुछ नहीं खोता।
safe-restore-cancel = रद्द करें
safe-restore-mail = मेल
safe-restore-pim = खाते और संपर्क
safe-restore-blobs = अटैचमेंट
safe-report-title = डीबग रिपोर्ट
safe-report-body = इसे कॉपी करें और अपनी बग रिपोर्ट के साथ जोड़ें। इसमें कोई मेल, पता या पासवर्ड नहीं है।
safe-report-restore = वापस लाएं…
safe-report-copied = डीबग रिपोर्ट कॉपी की गई
