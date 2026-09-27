# Katna Mail, Amharic (አማርኛ).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = ስለ Katna
about-tagline = ለLinux ዴስክቶፕ ደብዳቤ እና ቀን መቁጠሪያ
about-whats-new = ምን አዲስ ነገር አለ
about-changelog = የለውጥ መዝገብ
about-source = የምንጭ ኮድ
about-coffee = ቡና ይጋብዙኝ
about-coming-soon = በቅርቡ ይመጣል
about-follow = ደራሲውን ይከተሉ
about-love-title = ለRust፣ ለKDE እና ለLinux በፍቅር የተሠራ
about-love-text = Rust ፈጣን እና ደህንነቱ የተጠበቀ የደብዳቤ መተግበሪያ መጻፍን አስደሳች ያደርጋል፦ Katna ምንም unsafe ኮድ የለውም። የKDE Plasma ዴስክቶፕ እና የPIM ስብስቡ Katnaን አነሳስተዋል፣ Linux እና የነፃ ሶፍትዌር ማህበረሰብ ደግሞ የቆመበትን መሠረት ይገነባሉ። እናመሰግናለን፣ ከታች ላሉት ቤተ-መጻሕፍትም እናመሰግናለን።
about-kde-text = KDE Katna በጣም ቤቱ እንደሆነ የሚሰማውን ዴስክቶፕ ይገነባል፣ የሚሠራውም በበጎ ፈቃደኞች ሲሆን የሚደገፈውም እንደ እርስዎ ባሉ ሰዎች ነው። Plasmaን ወይም የKDE መተግበሪያዎችን ከወደዱ፣ እባክዎ ለKDE መለገስን ያስቡበት።
about-donate-kde = ለKDE ይለግሱ
about-gpui-title = በGPUI ላይ የተገነባ፣ ከZed ፕሮጀክት
about-gpui-text = የKatna Mail በይነገጽ በሙሉ የተገነባው Zed Industries ለZed አርታዒ በሠራው ፈጣን፣ በGPU የተፋጠነ የUI ማዕቀፍ በሆነው በGPUI ላይ ነው። የሚያዩት እያንዳንዱ ፒክሰል፣ እንቅስቃሴ እና መስኮት የሚሳለው በእሱ ነው። የZed ቡድን፣ በግልጽ ስለገነቡት እናመሰግናለን። Apache-2.0.
about-gpui-github = GPUI በGitHub ላይ
about-personal-title = የግል ፕሮጀክት
about-personal-text = Katna Mail አዲስ ወይም አብዮታዊ ለመሆን አይሞክርም። ደራሲው የፈለገው የደብዳቤ መተግበሪያ ነው፣ ባህሪያቱ እና መልኩም ከGmail፣ ከMailspring እና ከThunderbird የተዋሱ ናቸው። ሊሠራ የቻለውም LLMዎች ምን ያህል ርቀት ስለሄዱ ብቻ ነው።
about-built-on = በነፃ ሶፍትዌር ላይ የተገነባ
about-credit-pimalaya = IMAP፣ SMTP እና መግባት (io-imap፣ io-smtp፣ io-sasl)
about-credit-imap-codec = IMAPን ማንበብ እና መጻፍ
about-credit-tantivy = ፍለጋ
about-credit-sqlite = የደብዳቤ ማከማቻው
about-credit-rustls = ደህንነታቸው የተጠበቁ ግንኙነቶች
about-credit-mail-parser = ደብዳቤ ማንበብ፣ ከStalwart Labs
about-credit-html5ever = HTML ደብዳቤ፣ ከServo ፕሮጀክት
about-credit-zbus = በD-Bus እና በፖርታሎች ከዴስክቶፑ ጋር መነጋገር
about-credit-oo7 = በዴስክቶፑ ቁልፍ ቀለበት ውስጥ ያሉ የይለፍ ቃሎች
about-credit-hayro = PDFዎችን መመልከት እና ማተም
about-credit-calamine = የተመን ሉህ ቅድመ እይታዎች
about-credit-resvg = የSVG ሥዕሎች
about-credit-jiff = ቀኖች እና የሰዓት ሰቆች
about-credit-spellbook = የፊደል ማረሚያ፣ ከHelix አርታዒ
about-credit-smol = ብዙ ነገሮችን በአንድ ጊዜ መሥራት
about-all-libraries = Katna የሚጠቀምባቸው ሁሉም ቤተ-መጻሕፍት ({ $count })
about-library-authors = በ{ $authors }
about-license = Katna በGNU GPL፣ ስሪት 3 ወይም ከዚያ በኋላ ስር ነፃ ሶፍትዌር ነው።
about-close = ዝጋ

## What’s new (shown after an update)

whats-new-title = በKatna Mail ውስጥ ምን አዲስ ነገር አለ
whats-new-updated = ወደ ስሪት { $version } ተዘምኗል
whats-new-version = ስሪት { $version }
whats-new-more = { $count ->
    [one] እና በሙሉ የለውጥ መዝገቡ ውስጥ አንድ ተጨማሪ።
   *[other] እና በሙሉ የለውጥ መዝገቡ ውስጥ { $count } ተጨማሪ።
}
whats-new-changelog = ሙሉ የለውጥ መዝገብ
whats-new-got-it = ገባኝ

## First run: welcome page

onboarding-welcome-title = ወደ Katna Mail እንኳን ደህና መጡ
onboarding-welcome-lead = ደብዳቤዎ በራስዎ ኮምፒውተር ላይ፦ በፍጥነት የሚፈለግ፣ ከመስመር ውጭ የሚነበብ እና የግል።
onboarding-fast-title = ፈጣን፣ ከመስመር ውጭም ቢሆን
onboarding-fast-text = Katna የደብዳቤዎን ቅጂ እዚህ ያስቀምጣል፣ ስለዚህ መክፈቱ እና መፈለጉ ወዲያውኑ ነው፣ ግንኙነት ቢኖርም ባይኖርም።
onboarding-providers-title = ከደብዳቤዎ ጋር ይሠራል
onboarding-providers-text = Gmail፣ Outlook፣ Yahoo፣ iCloud እና ሌላ ማንኛውም የIMAP ወይም POP መለያ።
onboarding-private-title = የግል
onboarding-private-text = ደብዳቤዎ በቀጥታ ከአቅራቢዎ ወደዚህ ኮምፒውተር ይመጣል። ምንም የKatna አገልጋይ አያየውም።
onboarding-get-started = እንጀምር

## First run: adding an account

onboarding-service-checking = የKatna የጀርባ አገልግሎት በመፈተሽ ላይ…
onboarding-service-running = የKatna የጀርባ አገልግሎት እየሠራ ነው።
onboarding-service-missing = የKatna የጀርባ አገልግሎት እየሠራ አይደለም
onboarding-service-start = ደብዳቤዎን ያመጣል እንዲሁም ይልካል። ከተርሚናል ያስጀምሩት፣ ከዚያ እንደገና ይፈትሹ፦
onboarding-check-again = እንደገና ፈትሽ
onboarding-account-title = የደብዳቤ መለያዎን ያክሉ
onboarding-account-lead = የኢሜይል አድራሻዎን እና የይለፍ ቃልዎን ይተይቡ፣ Katna የአገልጋይ ቅንብሮቹን ያገኛል። Gmail፣ Yahoo እና iCloud በመለያዎ የደህንነት ቅንብሮች ውስጥ የሚሠራ የመተግበሪያ የይለፍ ቃል ያስፈልጋቸዋል።
onboarding-add-account = መለያ አክል
onboarding-back = ተመለስ

## First run: choosing the look

onboarding-look-title = የራስዎ ያድርጉት
onboarding-look-lead = ደብዳቤ እንዴት እንደሚከፈት እና Katna እንዴት እንደሚታይ ይምረጡ። እነዚህን በማንኛውም ጊዜ በፈጣን ቅንብሮች ውስጥ መቀየር ይችላሉ።
onboarding-reading-pane = የንባብ ክፍል
onboarding-pane-right = ከዝርዝሩ በስተቀኝ
onboarding-pane-none = ክፍፍል የለም
onboarding-theme = ገጽታ
onboarding-theme-system = ሥርዓት
onboarding-theme-light = ፈካ ያለ
onboarding-theme-dark = ጠቆር ያለ
onboarding-density = ጥግግት
onboarding-density-default = ነባሪ
onboarding-density-compact = የታመቀ
onboarding-continue = ቀጥል

## First run: done

onboarding-ready-title = ሁሉም ዝግጁ ነው
onboarding-ready-lead = Katna ደብዳቤዎን እያመጣ ነው። ሲደርስ ይታያል፣ አዲስ ደብዳቤም በራሱ ይመጣል።
onboarding-ready-lead-address = Katna የ{ $address } ደብዳቤን እያመጣ ነው። ሲደርስ ይታያል፣ አዲስ ደብዳቤም በራሱ ይመጣል።
onboarding-ready-tour = ሁሉም ነገር የት እንዳለ ለማየት የአንድ ደቂቃ ጉብኝት ያድርጉ?
onboarding-skip = ለአሁን ዝለል
onboarding-take-tour = ጉብኝቱን ጀምር

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Katnaን ለማሻሻል ያግዙ
share-lead = Katna ሲበላሽ፣ በዚህ ኮምፒውተር ላይ ሪፖርት ያስቀምጣል። እነዚህን ሪፖርቶች መላክ የተበላሸውን ለማስተካከል ይረዳል። ይህንን በማንኛውም ጊዜ በቅንብሮች > የተጠቃሚ ግብረመልስ ውስጥ መቀየር ይችላሉ።
share-sent = የሚላከው
share-sent-detail = በቅንብሮች ውስጥ ሊመለከቱት በሚችሉት መልኩ ያለው የብልሽት ሪፖርት፦ ምን እና በKatna ውስጥ የት እንደተበላሸ፣ ስሪቱ፣ የእርስዎ Linux ሥርዓት እና ዴስክቶፕ፣ እንዲሁም የደብዳቤ አቃፊዎችን ሊጠቅሱ የሚችሉ የKatna የመጨረሻ የምዝግብ ማስታወሻ መስመሮች።
share-never-sent = ፈጽሞ የማይላከው
share-never-sent-detail = መልዕክቶችዎ፣ እውቂያዎችዎ፣ የይለፍ ቃሎችዎ፣ የIP አድራሻዎ፣ የተጠቃሚ ስምዎ ወይም የኮምፒውተር ስምዎ። የኢሜይል አድራሻዎች ከሪፖርቱ ይወገዳሉ።
share-where = የት እንደሚሄድ
share-where-detail = በአውሮፓ ህብረት ውስጥ ወደሚቀመጠው በSentry ላይ ወዳለው የKatna የብልሽት መከታተያ። ሪፖርቶችን ከእርስዎ ጋር የሚያገናኝ ምንም መታወቂያ የለም።
share-dont-send = አትላክ
share-send = የብልሽት ሪፖርቶችን ላክ
share-sending = የብልሽት ሪፖርቶች ይላካሉ። እናመሰግናለን።
share-local = የብልሽት ሪፖርቶች በዚህ ኮምፒውተር ላይ ይቆያሉ።

## The tour (cards pointing at each part of the window)

tour-welcome-title = ወደ Katna Mail እንኳን ደህና መጡ
tour-welcome-text = የአንድ ደቂቃ ጉብኝት ሁሉም ነገር የት እንዳለ ያሳያል።
tour-not-now = አሁን አይደለም
tour-start = ጉብኝቱን ጀምር
tour-close = ዝጋ
tour-skip = ጉብኝቱን ዝለል
tour-back = ተመለስ
tour-done = ተጠናቋል
tour-next = ቀጣይ
tour-step = { $step } ከ{ $total }
tour-compose-title = መልዕክት ይጻፉ
tour-compose-text = ጻፍ አዲስ መልዕክትን ከታች በስተቀኝ ይከፍታል፣ ስለዚህ እየጻፉ ማንበብዎን መቀጠል ይችላሉ።
tour-search-title = ሁሉንም ደብዳቤዎን ይፈልጉ
tour-search-text = ፍለጋ ከመስመር ውጭም ይሠራል። በቀኝ ጫፍ ያለው አዝራር ማጣሪያዎችን ያክላል፦ ላኪ፣ ተቀባይ፣ ርዕሰ ጉዳይ፣ ቀኖች እና አባሪዎች።
tour-menu-title = አቃፊዎቹን አሳይ ወይም ደብቅ
tour-menu-text = ይህ አዝራር የአቃፊ ዝርዝሩን ያጥፈዋል። ተደብቆ ሳለ፣ አቃፊዎቹን ለማየት ጠቋሚውን በግራ በኩል ባለው ደብዳቤ ላይ ያሳርፉ።
tour-apps-title = የእርስዎ መተግበሪያዎች
tour-apps-text = ደብዳቤ አሁን እዚህ ይኖራል። ቀን መቁጠሪያ፣ እውቂያዎች፣ ተግባራት፣ ማስታወሻዎች እና ምግቦች በዚህ አሞሌ ውስጥ ይቀላቀሉታል።
tour-tabs-title = የገቢ መልዕክት ሳጥን ትሮች
tour-tabs-text = አዲስ ደብዳቤ ወደ ዋና፣ ማስተዋወቂያዎች፣ ማህበራዊ፣ ዝማኔዎች እና መድረኮች ይለያል። ትሮቹን በፈጣን ቅንብሮች ውስጥ ማጥፋት ይችላሉ።
tour-list-title = የእርስዎ መልዕክቶች
tour-list-text = ለማንበብ መልዕክትን ጠቅ ያድርጉ። ለፈጣን እርምጃዎች ጠቋሚውን በላዩ ላይ ያሳርፉ፣ ለተጨማሪ በቀኝ ጠቅ ያድርጉ፣ ወይም በአንድ ላይ እርምጃ ለመውሰድ ብዙዎችን ምልክት ያድርጉ።
tour-settings-title = ፈጣን ቅንብሮች
tour-settings-text = የንባብ ክፍሉን፣ ጥግግቱን እና ገጽታውን እዚህ ይቀይሩ። ጉብኝቱም ከዚያ እንደገና ሊጀመር ይችላል።
tour-account-title = የእርስዎ መለያ
tour-account-text = በየትኛው መለያ ውስጥ እንዳሉ ይመልከቱ፣ ሌላም ያክሉ።

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] የKatna የጀርባ አገልግሎት ባልተጠበቀ ሁኔታ ቆሟል።
    [one] የKatna የጀርባ አገልግሎት ባልተጠበቀ ሁኔታ ቆሟል። አንድ ተጨማሪ የብልሽት ሪፖርት ተቀምጧል።
   *[other] የKatna የጀርባ አገልግሎት ባልተጠበቀ ሁኔታ ቆሟል። { $more } ተጨማሪ የብልሽት ሪፖርቶች ተቀምጠዋል።
}
crash-mail = { $more ->
    [0] Katna Mail ባለፈው ጊዜ ባልተጠበቀ ሁኔታ ተዘግቷል።
    [one] Katna Mail ባለፈው ጊዜ ባልተጠበቀ ሁኔታ ተዘግቷል። አንድ ተጨማሪ የብልሽት ሪፖርት ተቀምጧል።
   *[other] Katna Mail ባለፈው ጊዜ ባልተጠበቀ ሁኔታ ተዘግቷል። { $more } ተጨማሪ የብልሽት ሪፖርቶች ተቀምጠዋል።
}
crash-view = ሪፖርቱን ተመልከት
crash-view-tooltip = በዚህ ኮምፒውተር ላይ የተቀመጠውን ሪፖርት ክፈት
crash-copy = ሪፖርቱን ቅዳ
crash-close = ዝጋ
