# Katna Mail, Hindi (हिन्दी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Katna के बारे में
about-tagline = Linux डेस्कटॉप के लिए मेल और कैलेंडर
about-whats-new = नया क्या है
about-changelog = बदलावों की सूची
about-source = सोर्स कोड
about-coffee = मुझे एक कॉफ़ी पिलाएँ
about-coming-soon = जल्द आ रहा है
about-follow = लेखक को फ़ॉलो करें
about-love-title = Rust, KDE और Linux के लिए प्यार से बनाया गया
about-love-text = Rust की वजह से तेज़ और सुरक्षित मेल ऐप लिखना एक आनंद है: Katna में कोई unsafe कोड नहीं है। KDE के Plasma डेस्कटॉप और उसके PIM सूट ने Katna को प्रेरित किया, और Linux तथा फ़्री सॉफ़्टवेयर समुदाय वह ज़मीन बनाते हैं जिस पर यह खड़ा है। धन्यवाद, और नीचे दी गई लाइब्रेरी को भी धन्यवाद।
about-kde-text = KDE वह डेस्कटॉप बनाता है जिस पर Katna सबसे ज़्यादा घर जैसा महसूस करता है, और इसे स्वयंसेवक बनाते हैं और आप जैसे लोग इसका खर्च उठाते हैं। अगर आपको Plasma या KDE के ऐप पसंद हैं, तो कृपया KDE को दान देने के बारे में सोचें।
about-donate-kde = KDE को दान दें
about-gpui-title = Zed प्रोजेक्ट के GPUI पर बना
about-gpui-text = Katna Mail का पूरा इंटरफ़ेस GPUI पर बना है, जो तेज़, GPU-एक्सेलरेटेड UI फ़्रेमवर्क है और जिसे Zed Industries ने Zed एडिटर के लिए बनाया। आपको दिखने वाला हर पिक्सेल, एनिमेशन और विंडो इसी से बनता है। Zed टीम, इसे खुले में बनाने के लिए धन्यवाद। Apache-2.0.
about-gpui-github = GitHub पर GPUI
about-personal-title = एक निजी प्रोजेक्ट
about-personal-text = Katna Mail कुछ नया या क्रांतिकारी बनने की कोशिश नहीं करता। यह वह मेल ऐप है जो इसके लेखक चाहते थे, और इसकी सुविधाएँ और रूप Gmail, Mailspring और Thunderbird से लिए गए हैं। यह सिर्फ़ इसलिए संभव हो पाया क्योंकि LLM अब बहुत आगे आ चुके हैं।
about-built-on = फ़्री सॉफ़्टवेयर पर बना
about-credit-pimalaya = IMAP, SMTP और साइन इन (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = IMAP पढ़ना और लिखना
about-credit-tantivy = खोज
about-credit-sqlite = मेल स्टोर
about-credit-rustls = सुरक्षित कनेक्शन
about-credit-mail-parser = मेल पढ़ना, Stalwart Labs से
about-credit-html5ever = HTML मेल, Servo प्रोजेक्ट से
about-credit-zbus = D-Bus और पोर्टल के ज़रिए डेस्कटॉप से बातचीत
about-credit-oo7 = डेस्कटॉप के कीरिंग में पासवर्ड
about-credit-hayro = PDF देखना और प्रिंट करना
about-credit-calamine = स्प्रेडशीट प्रीव्यू
about-credit-resvg = SVG चित्र
about-credit-jiff = तारीख और टाइम ज़ोन
about-credit-spellbook = वर्तनी जाँच, Helix एडिटर से
about-credit-smol = एक साथ कई काम करना
about-all-libraries = Katna में इस्तेमाल हर लाइब्रेरी ({ $count })
about-library-authors = { $authors } द्वारा
about-license = Katna, GNU GPL संस्करण 3 या उसके बाद के संस्करण के तहत फ़्री सॉफ़्टवेयर है।
about-close = बंद करें

## What’s new (shown after an update)

whats-new-title = Katna Mail में नया क्या है
whats-new-updated = संस्करण { $version } में अपडेट किया गया
whats-new-version = संस्करण { $version }
whats-new-more = { $count ->
    [one] और { $count } बदलाव पूरी बदलाव सूची में है।
   *[other] और { $count } बदलाव पूरी बदलाव सूची में हैं।
}
whats-new-changelog = बदलावों की पूरी सूची
whats-new-got-it = समझ गया

## First run: welcome page

onboarding-welcome-title = Katna Mail में आपका स्वागत है
onboarding-welcome-lead = आपका मेल आपके अपने कंप्यूटर पर: खोजने में तेज़, ऑफ़लाइन पढ़ने लायक और निजी।
onboarding-fast-title = तेज़, ऑफ़लाइन भी
onboarding-fast-text = Katna आपके मेल की एक कॉपी यहाँ रखता है, इसलिए उसे खोलना और खोजना तुरंत होता है, कनेक्शन हो या न हो।
onboarding-providers-title = आपके मेल के साथ काम करता है
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud और कोई भी दूसरा IMAP या POP खाता।
onboarding-private-title = निजी
onboarding-private-text = आपका मेल सीधे आपके प्रोवाइडर से इस कंप्यूटर पर आता है। कोई Katna सर्वर उसे नहीं देखता।
onboarding-get-started = शुरू करें

## First run: adding an account

onboarding-service-checking = Katna की बैकग्राउंड सेवा जाँची जा रही है…
onboarding-service-running = Katna की बैकग्राउंड सेवा चल रही है।
onboarding-service-missing = Katna की बैकग्राउंड सेवा नहीं चल रही है
onboarding-service-start = यह आपका मेल लाती और भेजती है। इसे टर्मिनल से शुरू करें, फिर दोबारा जाँचें:
onboarding-check-again = दोबारा जाँचें
onboarding-account-title = अपना मेल खाता जोड़ें
onboarding-account-lead = अपना ईमेल पता और पासवर्ड टाइप करें, और Katna सर्वर सेटिंग ढूँढ लेगा। Gmail, Yahoo और iCloud के लिए ऐप पासवर्ड चाहिए, जो आपके खाते की सुरक्षा सेटिंग में बनता है।
onboarding-add-account = खाता जोड़ें
onboarding-back = वापस

## First run: choosing the look

onboarding-look-title = इसे अपना बनाएँ
onboarding-look-lead = चुनें कि मेल कैसे खुले और Katna कैसा दिखे। आप इन्हें क्विक सेटिंग में कभी भी बदल सकते हैं।
onboarding-reading-pane = रीडिंग पेन
onboarding-pane-right = सूची की दाईं ओर
onboarding-pane-none = कोई विभाजन नहीं
onboarding-theme = थीम
onboarding-theme-system = सिस्टम
onboarding-theme-light = लाइट
onboarding-theme-dark = डार्क
onboarding-density = डेंसिटी
onboarding-density-default = डिफ़ॉल्ट
onboarding-density-compact = कॉम्पैक्ट
onboarding-continue = जारी रखें

## First run: done

onboarding-ready-title = सब तैयार है
onboarding-ready-lead = Katna आपका मेल ला रहा है। मेल आते ही दिखने लगता है, और नया मेल अपने-आप दिखाई देगा।
onboarding-ready-lead-address = Katna, { $address } का मेल ला रहा है। मेल आते ही दिखने लगता है, और नया मेल अपने-आप दिखाई देगा।
onboarding-ready-tour = क्या आप एक मिनट का टूर लेकर देखना चाहेंगे कि सब कुछ कहाँ है?
onboarding-skip = अभी छोड़ें
onboarding-take-tour = टूर शुरू करें

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Katna को बेहतर बनाने में मदद करें
share-lead = जब Katna क्रैश होता है, तो वह इस कंप्यूटर पर एक रिपोर्ट सेव करता है। ये रिपोर्ट भेजने से गड़बड़ी ठीक करने में मदद मिलती है। आप इसे कभी भी सेटिंग > उपयोगकर्ता फ़ीडबैक में बदल सकते हैं।
share-sent = क्या भेजा जाता है
share-sent-detail = क्रैश रिपोर्ट, ठीक वैसी जैसी आप उसे सेटिंग में देख सकते हैं: क्या क्रैश हुआ और Katna में कहाँ, संस्करण, आपका Linux सिस्टम और डेस्कटॉप, और Katna के लॉग की आख़िरी पंक्तियाँ, जिनमें मेल फ़ोल्डर के नाम हो सकते हैं।
share-never-sent = क्या कभी नहीं भेजा जाता
share-never-sent-detail = आपके मैसेज, संपर्क, पासवर्ड, IP पता, यूज़र नाम या कंप्यूटर का नाम। ईमेल पते रिपोर्ट से हटा दिए जाते हैं।
share-where = यह कहाँ जाती है
share-where-detail = Sentry पर Katna के क्रैश ट्रैकर में, जो EU में संग्रहीत है। कोई भी ID रिपोर्ट को आपसे नहीं जोड़ती।
share-dont-send = न भेजें
share-send = क्रैश रिपोर्ट भेजें
share-sending = क्रैश रिपोर्ट भेजी जाएँगी। धन्यवाद।
share-local = क्रैश रिपोर्ट इसी कंप्यूटर पर रहेंगी।

## The tour (cards pointing at each part of the window)

tour-welcome-title = Katna Mail में आपका स्वागत है
tour-welcome-text = एक मिनट का टूर दिखाता है कि सब कुछ कहाँ है।
tour-not-now = अभी नहीं
tour-start = टूर शुरू करें
tour-close = बंद करें
tour-skip = टूर छोड़ें
tour-back = पीछे
tour-done = हो गया
tour-next = आगे
tour-step = { $total } में से { $step }
tour-compose-title = मैसेज लिखें
tour-compose-text = लिखें बटन नीचे दाईं ओर एक नया मैसेज खोलता है, ताकि आप लिखते हुए पढ़ना जारी रख सकें।
tour-search-title = अपना सारा मेल खोजें
tour-search-text = खोज ऑफ़लाइन भी काम करती है। दाएँ सिरे का बटन फ़िल्टर जोड़ता है: भेजने वाला, पाने वाला, विषय, तारीख और अटैचमेंट।
tour-menu-title = फ़ोल्डर दिखाएँ या छिपाएँ
tour-menu-text = यह बटन फ़ोल्डर सूची को समेट देता है। जब वह छिपी हो, तो फ़ोल्डर देखने के लिए पॉइंटर को बाईं ओर मेल पर रखें।
tour-apps-title = आपके ऐप
tour-apps-text = अभी यहाँ मेल है। कैलेंडर, संपर्क, टास्क, नोट और फ़ीड भी इस बार में जुड़ेंगे।
tour-tabs-title = इनबॉक्स टैब
tour-tabs-text = नया मेल मुख्य, प्रमोशन, सामाजिक, अपडेट और फ़ोरम में बँट जाता है। आप क्विक सेटिंग में टैब बंद कर सकते हैं।
tour-list-title = आपके मैसेज
tour-list-text = किसी मैसेज को पढ़ने के लिए उस पर क्लिक करें। क्विक ऐक्शन के लिए उस पर पॉइंटर रखें, ज़्यादा विकल्पों के लिए राइट-क्लिक करें, या कई मैसेज पर एक साथ काम करने के लिए उन्हें टिक करें।
tour-settings-title = क्विक सेटिंग
tour-settings-text = रीडिंग पेन, डेंसिटी और थीम यहाँ बदलें। टूर भी यहीं से दोबारा शुरू किया जा सकता है।
tour-account-title = आपका खाता
tour-account-text = देखें कि आप किस खाते में हैं, और दूसरा खाता जोड़ें।

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Katna की बैकग्राउंड सेवा अचानक रुक गई।
    [one] Katna की बैकग्राउंड सेवा अचानक रुक गई। { $more } और क्रैश रिपोर्ट सेव है।
   *[other] Katna की बैकग्राउंड सेवा अचानक रुक गई। { $more } और क्रैश रिपोर्ट सेव हैं।
}
crash-mail = { $more ->
    [0] पिछली बार Katna Mail अचानक बंद हो गया था।
    [one] पिछली बार Katna Mail अचानक बंद हो गया था। { $more } और क्रैश रिपोर्ट सेव है।
   *[other] पिछली बार Katna Mail अचानक बंद हो गया था। { $more } और क्रैश रिपोर्ट सेव हैं।
}
crash-view = रिपोर्ट देखें
crash-view-tooltip = इस कंप्यूटर पर सेव की गई रिपोर्ट खोलें
crash-copy = रिपोर्ट कॉपी करें
crash-close = बंद करें
sign-in-again-text = { $provider } चाहता है कि आप { $address } में फिर से साइन इन करें।
sign-in-again-button = साइन इन करें
sign-in-again-tooltip = अपने ब्राउज़र में { $provider } का साइन-इन पेज खोलें
sign-in-again-waiting = आपके ब्राउज़र का इंतज़ार है…
sign-in-again-close = बंद करें
sign-in-again-done = { $address } में फिर से साइन इन हो गया। आपका मेल लाया जा रहा है…
delete-ask-title = { $kind ->
    [conversation] { $count ->
        [one] इस बातचीत को ट्रैश में ले जाएं?
       *[other] { $count } बातचीत को ट्रैश में ले जाएं?
    }
   *[message] { $count ->
        [one] इस मैसेज को ट्रैश में ले जाएं?
       *[other] { $count } मैसेज को ट्रैश में ले जाएं?
    }
}
delete-ask-body = { $count ->
    [one] आप इसके तुरंत बाद पहले जैसा कर सकते हैं, या बाद में इसे ट्रैश से वापस ला सकते हैं।
   *[other] आप इसके तुरंत बाद पहले जैसा कर सकते हैं, या बाद में उन्हें ट्रैश से वापस ला सकते हैं।
}
delete-ask-confirm = ट्रैश में ले जाएं
delete-forever-title = { $kind ->
    [conversation] { $count ->
        [one] इस बातचीत को हमेशा के लिए मिटाएं?
       *[other] { $count } बातचीत को हमेशा के लिए मिटाएं?
    }
   *[message] { $count ->
        [one] इस मैसेज को हमेशा के लिए मिटाएं?
       *[other] { $count } मैसेज को हमेशा के लिए मिटाएं?
    }
}
delete-forever-body = { $count ->
    [one] यह सर्वर से भी मिट जाती है। इसे पहले जैसा नहीं किया जा सकता।
   *[other] ये सर्वर से भी मिट जाती हैं। इसे पहले जैसा नहीं किया जा सकता।
}
delete-forever-confirm = हमेशा के लिए मिटाएं
delete-ask-dont-ask = फिर से न पूछें
delete-ask-cancel = रद्द करें
