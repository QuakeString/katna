# Katna Mail, Marathi (मराठी).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Katna बद्दल
about-tagline = Linux डेस्कटॉपसाठी मेल आणि कॅलेंडर
about-whats-new = नवीन काय आहे
about-changelog = बदलांची नोंद
about-source = सोर्स कोड
about-coffee = मला एक कॉफी पाजा
about-coming-soon = लवकरच येत आहे
about-coffee-scan = किंवा तुमच्या फोनने कोड स्कॅन करा.
about-follow = लेखकाला फॉलो करा
about-love-title = Rust, KDE आणि Linux साठी प्रेमाने बनवलेले
about-love-text = Rust मुळे वेगवान आणि सुरक्षित मेल ॲप लिहिणे आनंददायी होते: Katna मध्ये कोणताही unsafe कोड नाही. KDE च्या Plasma डेस्कटॉपने आणि त्याच्या PIM संचाने Katna ला प्रेरणा दिली, आणि Linux व मुक्त सॉफ्टवेअर समुदायाने ते ज्या पायावर उभे आहे तो पाया घडवला. धन्यवाद, आणि खालील लायब्ररींनाही धन्यवाद.
about-kde-text = Katna ला सर्वात घरच्यासारखे वाटते तो डेस्कटॉप KDE बनवते, आणि तो स्वयंसेवक बनवतात व तुमच्यासारखे लोक त्याला निधी देतात. तुम्हाला Plasma किंवा KDE ची ॲप्स आवडत असल्यास, कृपया KDE ला देणगी देण्याचा विचार करा.
about-donate-kde = KDE ला देणगी द्या
about-gpui-title = Zed प्रकल्पाच्या GPUI वर बनवलेले
about-gpui-text = Katna Mail चा संपूर्ण इंटरफेस GPUI वर बनवलेला आहे, हे Zed Industries ने Zed एडिटरसाठी बनवलेले वेगवान, GPU-प्रवेगित UI फ्रेमवर्क आहे. तुम्ही पाहता तो प्रत्येक पिक्सेल, ॲनिमेशन आणि विंडो तेच रेखाटते. Zed टीम, ते खुलेपणाने बनवल्याबद्दल धन्यवाद. Apache-2.0.
about-gpui-github = GitHub वर GPUI
about-personal-title = एक वैयक्तिक प्रकल्प
about-personal-text = Katna Mail नवीन किंवा क्रांतिकारी होण्याचा प्रयत्न करत नाही. हे त्याच्या लेखकाला हवे असलेले मेल ॲप आहे, आणि त्याची वैशिष्ट्ये व रूप Gmail, Mailspring आणि Thunderbird कडून घेतले आहे. LLM इतके पुढे आल्यामुळेच हे शक्य झाले.
about-built-on = मुक्त सॉफ्टवेअरवर बनवलेले
about-credit-pimalaya = IMAP, SMTP आणि साइन इन (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = IMAP वाचणे आणि लिहिणे
about-credit-tantivy = शोध
about-credit-sqlite = मेल संग्रह
about-credit-rustls = सुरक्षित कनेक्शन
about-credit-mail-parser = मेल वाचणे, Stalwart Labs कडून
about-credit-html5ever = HTML मेल, Servo प्रकल्पाकडून
about-credit-zbus = D-Bus आणि पोर्टलद्वारे डेस्कटॉपशी संवाद
about-credit-oo7 = डेस्कटॉपच्या कीरिंगमधील पासवर्ड
about-credit-hayro = PDF पाहणे आणि छापणे
about-credit-calamine = स्प्रेडशीटचे पूर्वावलोकन
about-credit-resvg = SVG चित्रे
about-credit-jiff = तारखा आणि टाइम झोन
about-credit-spellbook = स्पेल चेक, Helix एडिटरकडून
about-credit-smol = एकाच वेळी अनेक कामे
about-all-libraries = Katna वापरत असलेल्या सर्व लायब्ररी ({ $count })
about-library-authors = { $authors } यांच्याद्वारे
about-license = Katna हे GNU GPL, आवृत्ती 3 किंवा नंतरच्या अंतर्गत मुक्त सॉफ्टवेअर आहे.
about-close = बंद करा

## What’s new (shown after an update)

whats-new-title = Katna Mail मध्ये नवीन काय आहे
whats-new-updated = आवृत्ती { $version } वर अपडेट केले
whats-new-version = आवृत्ती { $version }
whats-new-more = { $count ->
    [one] आणि संपूर्ण बदल नोंदीत आणखी एक.
   *[other] आणि संपूर्ण बदल नोंदीत आणखी { $count }.
}
whats-new-changelog = संपूर्ण बदल नोंद
whats-new-got-it = समजले

## First run: welcome page

onboarding-welcome-title = Katna Mail मध्ये स्वागत आहे
onboarding-welcome-lead = तुमचा मेल तुमच्याच कॉम्प्युटरवर: शोधायला जलद, ऑफलाइन वाचता येणारा आणि खाजगी.
onboarding-fast-title = जलद, ऑफलाइनसुद्धा
onboarding-fast-text = Katna तुमच्या मेलची एक प्रत इथे ठेवते, त्यामुळे कनेक्शन असो वा नसो, तो उघडणे आणि शोधणे झटपट होते.
onboarding-providers-title = तुमच्या मेलसोबत चालते
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud आणि इतर कोणतेही IMAP किंवा POP खाते.
onboarding-private-title = खाजगी
onboarding-private-text = तुमचा मेल थेट तुमच्या प्रदात्याकडून या कॉम्प्युटरवर येतो. कोणताही Katna सर्व्हर तो पाहत नाही.
onboarding-get-started = सुरू करा

## First run: adding an account

onboarding-service-checking = Katna ची बॅकग्राउंड सेवा तपासत आहे…
onboarding-service-running = Katna ची बॅकग्राउंड सेवा चालू आहे.
onboarding-service-missing = Katna ची बॅकग्राउंड सेवा चालू नाही
onboarding-service-start = ती तुमचा मेल आणते आणि पाठवते. ती टर्मिनलमधून सुरू करा, नंतर पुन्हा तपासा:
onboarding-check-again = पुन्हा तपासा
onboarding-account-title = तुमचे मेल खाते जोडा
onboarding-account-lead = तुमचा ईमेल पत्ता आणि पासवर्ड टाइप करा, आणि Katna सर्व्हर सेटिंग्ज शोधते. Gmail, Yahoo आणि iCloud ला ॲप पासवर्ड लागतो, जो तुमच्या खात्याच्या सुरक्षा सेटिंग्जमध्ये बनवता येतो.
onboarding-add-account = खाते जोडा
onboarding-back = मागे

## First run: choosing the look

onboarding-look-title = ते तुमचे बनवा
onboarding-look-lead = मेल कसा उघडतो आणि Katna कसे दिसते ते निवडा. तुम्ही हे झटपट सेटिंग्जमध्ये कधीही बदलू शकता.
onboarding-reading-pane = वाचन पेन
onboarding-pane-right = यादीच्या उजवीकडे
onboarding-pane-none = विभाजन नाही
onboarding-theme = थीम
onboarding-theme-system = सिस्टम
onboarding-theme-light = लाइट
onboarding-theme-dark = डार्क
onboarding-density = घनता
onboarding-density-default = डीफॉल्ट
onboarding-density-compact = कॉम्पॅक्ट
onboarding-continue = पुढे चला

## First run: done

onboarding-ready-title = सर्व तयार आहे
onboarding-ready-lead = Katna तुमचा मेल आणत आहे. तो येईल तसा दिसतो, आणि नवीन मेल आपोआप दिसतो.
onboarding-ready-lead-address = Katna { $address } चा मेल आणत आहे. तो येईल तसा दिसतो, आणि नवीन मेल आपोआप दिसतो.
onboarding-ready-tour = सर्व काही कुठे आहे ते पाहण्यासाठी एक मिनिटाची सफर करायची?
onboarding-skip = आत्ता वगळा
onboarding-take-tour = सफर करा

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Katna सुधारण्यात मदत करा
share-lead = Katna क्रॅश झाल्यावर ते या कॉम्प्युटरवर एक अहवाल सेव्ह करते. हे अहवाल पाठवल्याने काय चुकले ते दुरुस्त करायला मदत होते. तुम्ही हे सेटिंग्ज > वापरकर्ता अभिप्राय मध्ये कधीही बदलू शकता.
share-sent = काय पाठवले जाते
share-sent-detail = सेटिंग्जमध्ये तुम्ही पाहू शकता तसाच क्रॅश अहवाल: काय क्रॅश झाले आणि Katna मध्ये कुठे, आवृत्ती, तुमची Linux प्रणाली आणि डेस्कटॉप, आणि Katna च्या लॉगमधील शेवटच्या ओळी, ज्यात मेल फोल्डरची नावे असू शकतात.
share-never-sent = काय कधीच पाठवले जात नाही
share-never-sent-detail = तुमचे मेसेज, संपर्क, पासवर्ड, IP पत्ता, वापरकर्ता नाव किंवा कॉम्प्युटरचे नाव. ईमेल पत्ते अहवालातून काढून टाकले जातात.
share-where = ते कुठे जाते
share-where-detail = Sentry वरील Katna चा क्रॅश ट्रॅकर, EU मध्ये साठवलेला. कोणताही ID अहवालांना तुमच्याशी जोडत नाही.
share-dont-send = पाठवू नका
share-send = क्रॅश अहवाल पाठवा
share-sending = क्रॅश अहवाल पाठवले जातील. धन्यवाद.
share-local = क्रॅश अहवाल या कॉम्प्युटरवरच राहतात.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Katna Mail मध्ये स्वागत आहे
tour-welcome-text = एक मिनिटाची सफर सर्व काही कुठे आहे ते दाखवते.
tour-not-now = आत्ता नको
tour-start = सफर करा
tour-close = बंद करा
tour-skip = सफर वगळा
tour-back = मागे
tour-done = झाले
tour-next = पुढे
tour-step = { $total } पैकी { $step }
tour-compose-title = मेसेज लिहा
tour-compose-text = लिहा हे बटण नवीन मेसेज खाली उजवीकडे उघडते, त्यामुळे लिहिताना तुम्ही वाचत राहू शकता.
tour-search-title = तुमचा सर्व मेल शोधा
tour-search-text = शोध ऑफलाइनसुद्धा चालतो. उजव्या टोकावरील बटण फिल्टर जोडते: पाठवणारा, प्राप्तकर्ता, विषय, तारखा आणि अटॅचमेंट.
tour-menu-title = फोल्डर दाखवा किंवा लपवा
tour-menu-text = हे बटण फोल्डर यादी दुमडून बाजूला ठेवते. ती लपलेली असताना, फोल्डर पाहण्यासाठी डावीकडील मेल वर पॉइंटर ठेवा.
tour-apps-title = तुमची ॲप्स
tour-apps-text = मेल आता इथे राहतो. कॅलेंडर, संपर्क, कार्ये, नोट्स आणि फीड या पट्टीत त्याच्यासोबत येतील.
tour-tabs-title = इनबॉक्स टॅब
tour-tabs-text = नवीन मेल प्राथमिक, जाहिराती, सामाजिक, अपडेट आणि फोरम मध्ये वर्गीकृत होतो. तुम्ही झटपट सेटिंग्जमध्ये टॅब बंद करू शकता.
tour-list-title = तुमचे मेसेज
tour-list-text = मेसेज वाचण्यासाठी त्यावर क्लिक करा. झटपट क्रियांसाठी त्यावर पॉइंटर न्या, आणखी पर्यायांसाठी राइट-क्लिक करा, किंवा अनेकांवर एकत्र कृती करण्यासाठी ते टिक करा.
tour-settings-title = झटपट सेटिंग्ज
tour-settings-text = वाचन पेन, घनता आणि थीम इथे बदला. सफरसुद्धा तिथून पुन्हा सुरू करता येते.
tour-account-title = तुमचे खाते
tour-account-text = तुम्ही कोणत्या खात्यात आहात ते पाहा, आणि आणखी एक जोडा.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Katna ची बॅकग्राउंड सेवा अनपेक्षितपणे थांबली.
    [one] Katna ची बॅकग्राउंड सेवा अनपेक्षितपणे थांबली. आणखी एक क्रॅश अहवाल सेव्ह केलेला आहे.
   *[other] Katna ची बॅकग्राउंड सेवा अनपेक्षितपणे थांबली. आणखी { $more } क्रॅश अहवाल सेव्ह केलेले आहेत.
}
crash-mail = { $more ->
    [0] मागच्या वेळी Katna Mail अनपेक्षितपणे बंद झाले.
    [one] मागच्या वेळी Katna Mail अनपेक्षितपणे बंद झाले. आणखी एक क्रॅश अहवाल सेव्ह केलेला आहे.
   *[other] मागच्या वेळी Katna Mail अनपेक्षितपणे बंद झाले. आणखी { $more } क्रॅश अहवाल सेव्ह केलेले आहेत.
}
crash-view = अहवाल पाहा
crash-view-tooltip = या कॉम्प्युटरवर सेव्ह केलेला अहवाल उघडा
crash-copy = अहवाल कॉपी करा
crash-close = बंद करा
