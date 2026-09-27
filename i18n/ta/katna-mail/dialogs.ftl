# Katna Mail, Tamil (தமிழ்).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Katna பற்றி
about-tagline = Linux டெஸ்க்டாப்புக்கான அஞ்சலும் கேலெண்டரும்
about-whats-new = புதிதாக என்ன உள்ளது
about-changelog = மாற்றப் பதிவு
about-source = மூலக் குறியீடு
about-coffee = எனக்கு ஒரு காபி வாங்கிக் கொடுங்கள்
about-coming-soon = விரைவில் வருகிறது
about-follow = ஆசிரியரைப் பின்தொடருங்கள்
about-love-title = Rust, KDE, Linux மீதான அன்புடன் உருவாக்கப்பட்டது
about-love-text = வேகமான, பாதுகாப்பான அஞ்சல் செயலியை எழுதுவதை Rust மகிழ்ச்சியாக்குகிறது: Katna இல் unsafe குறியீடு எதுவும் இல்லை. KDE இன் Plasma டெஸ்க்டாப்பும் அதன் PIM தொகுப்பும் Katna க்கு ஊக்கமளித்தன, Linux உம் கட்டற்ற மென்பொருள் சமூகமும் அது நிற்கும் அடித்தளத்தை அமைக்கின்றன. நன்றி, கீழே உள்ள நூலகங்களுக்கும் நன்றி.
about-kde-text = Katna மிகவும் வீட்டைப் போல உணரும் டெஸ்க்டாப்பை KDE உருவாக்குகிறது; அதைத் தன்னார்வலர்கள் உருவாக்குகிறார்கள், உங்களைப் போன்றவர்கள் நிதியளிக்கிறார்கள். Plasma அல்லது KDE இன் செயலிகளை நீங்கள் விரும்பினால், KDE க்கு நன்கொடை அளிப்பதைக் கருதுங்கள்.
about-donate-kde = KDE க்கு நன்கொடை அளி
about-gpui-title = Zed திட்டத்தின் GPUI மீது உருவாக்கப்பட்டது
about-gpui-text = Katna Mail இன் முழு இடைமுகமும் GPUI மீது உருவாக்கப்பட்டுள்ளது; இது Zed எடிட்டருக்காக Zed Industries உருவாக்கிய வேகமான, GPU-முடுக்கம் கொண்ட UI கட்டமைப்பு. நீங்கள் பார்க்கும் ஒவ்வொரு பிக்சலும், அனிமேஷனும், சாளரமும் அதனால் வரையப்படுகின்றன. இதை வெளிப்படையாக உருவாக்கியதற்கு நன்றி, Zed குழுவே. Apache-2.0.
about-gpui-github = GitHub இல் GPUI
about-personal-title = ஒரு தனிப்பட்ட திட்டம்
about-personal-text = Katna Mail புதியதாகவோ புரட்சிகரமானதாகவோ இருக்க முயலவில்லை. இது அதன் ஆசிரியர் விரும்பிய அஞ்சல் செயலி; அதன் அம்சங்களும் தோற்றமும் Gmail, Mailspring, Thunderbird ஆகியவற்றிலிருந்து எடுக்கப்பட்டவை. LLMகள் எவ்வளவு முன்னேறியுள்ளன என்பதால் மட்டுமே இது சாத்தியமானது.
about-built-on = கட்டற்ற மென்பொருள் மீது உருவாக்கப்பட்டது
about-credit-pimalaya = IMAP, SMTP, உள்நுழைவு (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = IMAP ஐப் படித்தல், எழுதுதல்
about-credit-tantivy = தேடல்
about-credit-sqlite = அஞ்சல் சேமிப்பகம்
about-credit-rustls = பாதுகாப்பான இணைப்புகள்
about-credit-mail-parser = அஞ்சலைப் படித்தல், Stalwart Labs இடமிருந்து
about-credit-html5ever = HTML அஞ்சல், Servo திட்டத்திலிருந்து
about-credit-zbus = D-Bus, portals வழியாக டெஸ்க்டாப்புடன் பேசுதல்
about-credit-oo7 = டெஸ்க்டாப்பின் கீரிங்கில் கடவுச்சொற்கள்
about-credit-hayro = PDFகளைப் பார்த்தல், அச்சிடுதல்
about-credit-calamine = விரிதாள் முன்னோட்டங்கள்
about-credit-resvg = SVG படங்கள்
about-credit-jiff = தேதிகளும் நேர மண்டலங்களும்
about-credit-spellbook = எழுத்துப்பிழை சரிபார்ப்பு, Helix எடிட்டரிலிருந்து
about-credit-smol = பல வேலைகளை ஒரே நேரத்தில் செய்தல்
about-all-libraries = Katna பயன்படுத்தும் எல்லா நூலகங்களும் ({ $count })
about-library-authors = உருவாக்கியவர்கள்: { $authors }
about-license = Katna என்பது GNU GPL பதிப்பு 3 அல்லது அதற்குப் பிந்தையதன் கீழ் உள்ள கட்டற்ற மென்பொருள்.
about-close = மூடு

## What’s new (shown after an update)

whats-new-title = Katna Mail இல் புதிதாக என்ன உள்ளது
whats-new-updated = பதிப்பு { $version } க்குப் புதுப்பிக்கப்பட்டது
whats-new-version = பதிப்பு { $version }
whats-new-more = { $count ->
    [one] முழு மாற்றப் பதிவில் இன்னும் ஒன்று உள்ளது.
   *[other] முழு மாற்றப் பதிவில் இன்னும் { $count } உள்ளன.
}
whats-new-changelog = முழு மாற்றப் பதிவு
whats-new-got-it = புரிந்தது

## First run: welcome page

onboarding-welcome-title = Katna Mail க்கு வரவேற்கிறோம்
onboarding-welcome-lead = உங்கள் அஞ்சல் உங்கள் சொந்தக் கணினியில்: விரைவாகத் தேடலாம், ஆஃப்லைனிலும் படிக்கலாம், தனிப்பட்டது.
onboarding-fast-title = வேகமானது, ஆஃப்லைனிலும் கூட
onboarding-fast-text = Katna உங்கள் அஞ்சலின் நகலை இங்கே வைத்திருக்கிறது, எனவே இணைப்பு இருந்தாலும் இல்லாவிட்டாலும் அதைத் திறப்பதும் தேடுவதும் உடனடியாக நடக்கும்.
onboarding-providers-title = உங்கள் அஞ்சலுடன் வேலை செய்யும்
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud மற்றும் வேறு எந்த IMAP அல்லது POP கணக்கும்.
onboarding-private-title = தனிப்பட்டது
onboarding-private-text = உங்கள் அஞ்சல் உங்கள் வழங்குநரிடமிருந்து நேராக இந்தக் கணினிக்கு வரும். எந்த Katna சர்வரும் அதைப் பார்ப்பதில்லை.
onboarding-get-started = தொடங்குங்கள்

## First run: adding an account

onboarding-service-checking = Katna பின்னணிச் சேவையைச் சரிபார்க்கிறது…
onboarding-service-running = Katna பின்னணிச் சேவை இயங்குகிறது.
onboarding-service-missing = Katna பின்னணிச் சேவை இயங்கவில்லை
onboarding-service-start = இது உங்கள் அஞ்சலைப் பெற்று அனுப்புகிறது. டெர்மினலில் இருந்து அதைத் தொடங்கி, மீண்டும் சரிபாருங்கள்:
onboarding-check-again = மீண்டும் சரிபார்
onboarding-account-title = உங்கள் அஞ்சல் கணக்கைச் சேருங்கள்
onboarding-account-lead = உங்கள் மின்னஞ்சல் முகவரியையும் கடவுச்சொல்லையும் உள்ளிடுங்கள், Katna சர்வர் அமைப்புகளைக் கண்டறியும். Gmail, Yahoo, iCloud ஆகியவற்றுக்கு ஆப் கடவுச்சொல் தேவை; அதை உங்கள் கணக்கின் பாதுகாப்பு அமைப்புகளில் உருவாக்கலாம்.
onboarding-add-account = கணக்கைச் சேர்
onboarding-back = பின்செல்

## First run: choosing the look

onboarding-look-title = உங்களுக்கு ஏற்றபடி மாற்றுங்கள்
onboarding-look-lead = அஞ்சல் எப்படித் திறக்கும், Katna எப்படித் தோன்றும் என்பதைத் தேர்ந்தெடுங்கள். இவற்றை விரைவு அமைப்புகளில் எப்போது வேண்டுமானாலும் மாற்றலாம்.
onboarding-reading-pane = படிக்கும் பலகம்
onboarding-pane-right = பட்டியலின் வலதுபுறம்
onboarding-pane-none = பிரிப்பு இல்லை
onboarding-theme = தீம்
onboarding-theme-system = டெஸ்க்டாப்பைப் போலவே
onboarding-theme-light = லைட்
onboarding-theme-dark = டார்க்
onboarding-density = அடர்த்தி
onboarding-density-default = இயல்புநிலை
onboarding-density-compact = கச்சிதமானது
onboarding-continue = தொடர்

## First run: done

onboarding-ready-title = எல்லாம் தயார்
onboarding-ready-lead = Katna உங்கள் அஞ்சலைப் பெறுகிறது. அவை வந்து சேரும்போதே தோன்றும், புதிய அஞ்சல் தானாகவே தோன்றும்.
onboarding-ready-lead-address = Katna { $address } இன் அஞ்சலைப் பெறுகிறது. அவை வந்து சேரும்போதே தோன்றும், புதிய அஞ்சல் தானாகவே தோன்றும்.
onboarding-ready-tour = எல்லாம் எங்கே உள்ளது என்று பார்க்க ஒரு நிமிட அறிமுகச் சுற்று வேண்டுமா?
onboarding-skip = இப்போதைக்குத் தவிர்
onboarding-take-tour = அறிமுகச் சுற்றைத் தொடங்கு

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Katna ஐ மேம்படுத்த உதவுங்கள்
share-lead = Katna செயலிழக்கும்போது, இந்தக் கணினியில் ஒரு அறிக்கையைச் சேமிக்கிறது. இந்த அறிக்கைகளை அனுப்புவது என்ன தவறு நடந்தது என்பதைச் சரிசெய்ய உதவுகிறது. இதை அமைப்புகள் > பயனர் கருத்து இல் எப்போது வேண்டுமானாலும் மாற்றலாம்.
share-sent = என்ன அனுப்பப்படும்
share-sent-detail = அமைப்புகளில் நீங்கள் பார்க்கக்கூடிய அதே செயலிழப்பு அறிக்கை: எது செயலிழந்தது, Katna இல் எங்கே, பதிப்பு, உங்கள் Linux அமைப்பும் டெஸ்க்டாப்பும், Katna இன் கடைசிப் பதிவு வரிகள் (அவை அஞ்சல் ஃபோல்டர்களின் பெயர்களைக் கொண்டிருக்கலாம்).
share-never-sent = ஒருபோதும் அனுப்பப்படாதவை
share-never-sent-detail = உங்கள் மெசேஜ்கள், தொடர்புகள், கடவுச்சொற்கள், IP முகவரி, பயனர் பெயர் அல்லது கணினிப் பெயர். மின்னஞ்சல் முகவரிகள் அறிக்கையிலிருந்து நீக்கப்படும்.
share-where = எங்கே செல்கிறது
share-where-detail = Sentry இல் உள்ள Katna இன் செயலிழப்பு டிராக்கர், EU இல் சேமிக்கப்படுகிறது. எந்த ID யும் அறிக்கைகளை உங்களுடன் இணைப்பதில்லை.
share-dont-send = அனுப்ப வேண்டாம்
share-send = செயலிழப்பு அறிக்கைகளை அனுப்பு
share-sending = செயலிழப்பு அறிக்கைகள் அனுப்பப்படும். நன்றி.
share-local = செயலிழப்பு அறிக்கைகள் இந்தக் கணினியிலேயே இருக்கும்.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Katna Mail க்கு வரவேற்கிறோம்
tour-welcome-text = எல்லாம் எங்கே உள்ளது என்பதை ஒரு நிமிட அறிமுகச் சுற்று காட்டும்.
tour-not-now = இப்போது வேண்டாம்
tour-start = அறிமுகச் சுற்றைத் தொடங்கு
tour-close = மூடு
tour-skip = சுற்றைத் தவிர்
tour-back = பின்செல்
tour-done = முடிந்தது
tour-next = அடுத்து
tour-step = { $total } இல் { $step }
tour-compose-title = மெசேஜ் எழுதுங்கள்
tour-compose-text = எழுது பொத்தான் கீழ் வலதுபுறத்தில் புதிய மெசேஜைத் திறக்கும், எனவே எழுதும்போதே படித்துக்கொண்டிருக்கலாம்.
tour-search-title = உங்கள் எல்லா அஞ்சலிலும் தேடுங்கள்
tour-search-text = தேடல் ஆஃப்லைனிலும் வேலை செய்யும். வலது முனையில் உள்ள பொத்தான் வடிப்பான்களைச் சேர்க்கும்: அனுப்புநர், பெறுநர், பொருள், தேதிகள், இணைப்புகள்.
tour-menu-title = ஃபோல்டர்களைக் காட்டு அல்லது மறை
tour-menu-text = இந்தப் பொத்தான் ஃபோல்டர் பட்டியலை மடக்கி மறைக்கும். அது மறைந்திருக்கும்போது, ஃபோல்டர்களைப் பார்க்க இடதுபுறம் உள்ள அஞ்சல் மீது சுட்டியை வையுங்கள்.
tour-apps-title = உங்கள் செயலிகள்
tour-apps-text = இப்போது அஞ்சல் இங்கே உள்ளது. கேலெண்டர், தொடர்புகள், பணிகள், குறிப்புகள், ஊட்டங்கள் ஆகியவை இந்தப் பட்டையில் அதனுடன் சேரும்.
tour-tabs-title = இன்பாக்ஸ் தாவல்கள்
tour-tabs-text = புதிய அஞ்சல் முதன்மை, விளம்பரங்கள், சமூகம், புதுப்பிப்புகள், மன்றங்கள் எனப் பிரிக்கப்படும். தாவல்களை விரைவு அமைப்புகளில் முடக்கலாம்.
tour-list-title = உங்கள் மெசேஜ்கள்
tour-list-text = ஒரு மெசேஜைப் படிக்க அதைக் கிளிக் செய்யுங்கள். விரைவுச் செயல்களுக்கு அதன் மீது சுட்டியை வையுங்கள், மேலும் பலவற்றுக்கு வலது கிளிக் செய்யுங்கள், அல்லது பலவற்றை ஒன்றாகக் கையாள அவற்றைத் தேர்வுசெய்யுங்கள்.
tour-settings-title = விரைவு அமைப்புகள்
tour-settings-text = படிக்கும் பலகம், அடர்த்தி, தீம் ஆகியவற்றை இங்கே மாற்றலாம். அறிமுகச் சுற்றையும் அங்கிருந்து மீண்டும் தொடங்கலாம்.
tour-account-title = உங்கள் கணக்கு
tour-account-text = நீங்கள் எந்தக் கணக்கில் உள்ளீர்கள் என்று பாருங்கள், இன்னொன்றையும் சேருங்கள்.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Katna இன் பின்னணிச் சேவை எதிர்பாராமல் நின்றுவிட்டது.
    [one] Katna இன் பின்னணிச் சேவை எதிர்பாராமல் நின்றுவிட்டது. இன்னும் ஒரு செயலிழப்பு அறிக்கை சேமிக்கப்பட்டுள்ளது.
   *[other] Katna இன் பின்னணிச் சேவை எதிர்பாராமல் நின்றுவிட்டது. இன்னும் { $more } செயலிழப்பு அறிக்கைகள் சேமிக்கப்பட்டுள்ளன.
}
crash-mail = { $more ->
    [0] கடந்த முறை Katna Mail எதிர்பாராமல் மூடப்பட்டது.
    [one] கடந்த முறை Katna Mail எதிர்பாராமல் மூடப்பட்டது. இன்னும் ஒரு செயலிழப்பு அறிக்கை சேமிக்கப்பட்டுள்ளது.
   *[other] கடந்த முறை Katna Mail எதிர்பாராமல் மூடப்பட்டது. இன்னும் { $more } செயலிழப்பு அறிக்கைகள் சேமிக்கப்பட்டுள்ளன.
}
crash-view = அறிக்கையைப் பார்
crash-view-tooltip = இந்தக் கணினியில் சேமித்த அறிக்கையைத் திற
crash-copy = அறிக்கையை நகலெடு
crash-close = மூடு
