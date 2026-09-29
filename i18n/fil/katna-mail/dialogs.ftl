# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Tungkol sa Katna
about-tagline = Mail at kalendaryo para sa Linux desktop
about-whats-new = Ano’ng bago
about-update-not-checked = Hindi pa nasusuri ang mga update
about-update-checking = Sinusuri ang mga update…
about-update-up-to-date = Updated na ang Katna Mail
about-update-check-failed = Hindi masuri ang mga update
about-update-available = Available ang bersyon { $version }
about-update-downloading = Dina-download ang bersyon { $version }… { $percent }%
about-update-download-failed = Hindi natapos ang pag-download ng bersyon { $version }
about-update-ready = Handa nang i-install ang bersyon { $version }
about-update-ready-detail = Mag-re-restart ang Katna Mail para tapusin ang update.
about-update-confirm = I-install ang bersyon { $version }?
about-update-confirm-detail = Magsasara ang Katna Mail, iini-install ang update, at magbubukas ulit ito sa kinahinto mo. Hihingin ng computer mo ang password mo.
about-update-installing = Ini-install ang bersyon { $version }…
about-update-installing-detail = Ilagay ang password mo sa window na nabuksan.
about-update-cancelled = Hindi na-install ang update, dahil hindi ibinigay ang password.
about-update-failed = Hindi ma-install ang update: { $error }
about-update-unsupported = Ina-update ang kopyang ito ng Katna Mail ng package manager mo.
about-update-restart-failed = Na-install na ang update, pero hindi mabuksan ulit ang Katna Mail ({ $error }). Buksan mo ito mismo.
about-update-check = Suriin ang mga update
about-update-download = I-download
about-update-retry = Subukan ulit
about-update-button = I-update
about-update-restart = I-update at i-restart
about-update-cancel = Huwag muna
about-changelog = Changelog
about-source = Source code
about-coffee = Ilibre ako ng kape
about-coffee-coffee = Kape?
about-coffee-tea = Tsaa?
about-coffee-pizza = Pizza?
about-coffee-nothing = Wala? Wala talaga?
about-coffee-water = Kaya ko na ang tubig lang!!
about-coffee-thanks = Salamat sa paggamit ng Katna
about-coming-soon = Malapit na
about-follow-me = Sundan ako sa
about-love-title = Ginawa nang may pagmamahal para sa Rust, KDE at Linux
about-love-text = Dahil sa Rust, masayang isulat ang isang mabilis at ligtas na mail app: walang unsafe code ang Katna. Ang Plasma desktop ng KDE at ang PIM suite nito ang nagbigay-inspirasyon sa Katna, at ang Linux at ang komunidad ng free software ang bumubuo sa lupang tinatayuan nito. Salamat, at salamat sa mga library sa ibaba.
about-kde-text = Binubuo ng KDE ang desktop kung saan pinaka-at-home ang Katna, at gawa ito ng mga boluntaryo at pinopondohan ng mga taong tulad mo. Kung gusto mo ang Plasma o ang mga app ng KDE, pag-isipang mag-donate sa KDE.
about-donate-kde = Mag-donate sa KDE
about-gpui-title = Binuo sa GPUI, mula sa proyektong Zed
about-gpui-text = Ang buong interface ng Katna Mail ay binuo sa GPUI, ang mabilis at GPU-accelerated na UI framework na ginawa ng Zed Industries para sa Zed editor. Bawat pixel, animation at window na nakikita mo ay iginuguhit nito. Salamat, Zed team, sa pagbuo nito nang bukas. Apache-2.0.
about-gpui-github = GPUI sa GitHub
about-personal-title = Isang personal na proyekto
about-personal-text = Hindi sinusubukan ng Katna Mail na maging bago o rebolusyonaryo. Ito ang mail app na gusto ng may-akda nito, at hiniram ang mga feature at itsura nito mula sa Gmail, Mailspring at Thunderbird. Naging posible lang ito dahil sa layo ng narating ng mga LLM.
about-built-on = BINUO SA FREE SOFTWARE
about-credit-pimalaya = IMAP, SMTP at pag-sign in (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = Pagbasa at pagsulat ng IMAP
about-credit-tantivy = Paghahanap
about-credit-sqlite = Ang imbakan ng mail
about-credit-rustls = Mga secure na koneksyon
about-credit-mail-parser = Pagbasa ng mail, mula sa Stalwart Labs
about-credit-html5ever = HTML na mail, mula sa proyektong Servo
about-credit-zbus = Pakikipag-usap sa desktop sa pamamagitan ng D-Bus at mga portal
about-credit-oo7 = Mga password sa keyring ng desktop
about-credit-hayro = Pagtingin at pag-print ng mga PDF
about-credit-calamine = Mga preview ng spreadsheet
about-credit-resvg = Mga larawang SVG
about-credit-jiff = Mga petsa at time zone
about-credit-spellbook = Spell check, mula sa Helix editor
about-credit-smol = Paggawa ng maraming bagay nang sabay-sabay
about-all-libraries = Bawat library na ginagamit ng Katna ({ $count })
about-library-authors = ni { $authors }
about-license = Free software ang Katna sa ilalim ng GNU GPL, bersyon 3 o mas bago.
about-close = Isara

## What’s new (shown after an update)

whats-new-title = Ano’ng bago sa Katna Mail
whats-new-updated = Na-update sa bersyon { $version }
whats-new-version = Bersyon { $version }
whats-new-more = { $count ->
    [one] At { $count } pa sa buong changelog.
   *[other] At { $count } pa sa buong changelog.
}
whats-new-changelog = Buong changelog
whats-new-got-it = Sige

## First run: welcome page

onboarding-welcome-title = Maligayang pagdating sa Katna Mail
onboarding-welcome-lead = Ang mail mo sa sarili mong computer: mabilis hanapin, nababasa kahit offline at pribado.
onboarding-fast-title = Mabilis, kahit offline
onboarding-fast-text = Nagtatago ang Katna ng kopya ng mail mo rito, kaya agad-agad ang pagbukas at paghahanap dito, may koneksyon man o wala.
onboarding-providers-title = Gumagana sa mail mo
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud at anumang ibang IMAP o POP na account.
onboarding-private-title = Pribado
onboarding-private-text = Diretsong pumupunta ang mail mo mula sa iyong provider papunta sa computer na ito. Walang server ng Katna na nakakakita nito.
onboarding-get-started = Magsimula

## First run: adding an account

onboarding-service-checking = Tinitingnan ang serbisyo sa background ng Katna…
onboarding-service-running = Tumatakbo ang serbisyo sa background ng Katna.
onboarding-service-missing = Hindi tumatakbo ang serbisyo sa background ng Katna
onboarding-service-start = Ito ang kumukuha at nagpapadala ng mail mo. Simulan ito mula sa isang terminal, pagkatapos ay tingnan ulit:
onboarding-check-again = Tingnan ulit
onboarding-account-title = Idagdag ang iyong mail account
onboarding-account-lead = I-type ang iyong email address at password, at hahanapin ng Katna ang mga setting ng server. Kailangan ng Gmail, Yahoo at iCloud ng app password, na ginagawa sa mga setting ng seguridad ng account mo.
onboarding-add-account = Magdagdag ng account
onboarding-back = Bumalik

## First run: choosing the look

onboarding-look-title = Gawin itong iyo
onboarding-look-lead = Piliin kung paano bumubukas ang mail at kung ano ang itsura ng Katna. Mababago mo ang mga ito anumang oras sa mabilisang setting.
onboarding-reading-pane = Pane ng pagbabasa
onboarding-pane-right = Sa kanan ng listahan
onboarding-pane-none = Walang hati
onboarding-theme = Tema
onboarding-theme-system = System
onboarding-theme-light = Maliwanag
onboarding-theme-dark = Madilim
onboarding-density = Density
onboarding-density-default = Default
onboarding-density-compact = Compact
onboarding-continue = Magpatuloy

## First run: done

onboarding-ready-title = Handa ka na
onboarding-ready-lead = Kinukuha ng Katna ang mail mo. Lumalabas ito habang dumarating, at kusang lumalabas ang bagong mail.
onboarding-ready-lead-address = Kinukuha ng Katna ang mail ng { $address }. Lumalabas ito habang dumarating, at kusang lumalabas ang bagong mail.
onboarding-ready-tour = Mag-tour nang isang minuto para makita kung nasaan ang lahat?
onboarding-skip = Laktawan muna
onboarding-take-tour = Mag-tour

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Tumulong na pahusayin ang Katna
share-lead = Kapag nag-crash ang Katna, nagse-save ito ng ulat sa computer na ito. Nakakatulong ang pagpapadala ng mga ulat na ito para maayos ang nagkaproblema. Mababago mo ito anumang oras sa Mga setting > Feedback ng user.
share-sent = Ano ang ipinapadala
share-sent-detail = Ang ulat ng pag-crash gaya ng makikita mo sa Mga setting: kung ano ang nag-crash at saan sa Katna, ang bersyon, ang iyong Linux system at desktop, at ang mga huling linya ng log ng Katna, na maaaring magbanggit ng mga mail folder.
share-never-sent = Ano ang hindi kailanman ipinapadala
share-never-sent-detail = Ang iyong mga mensahe, contact, password, IP address, user name o pangalan ng computer. Inaalis sa ulat ang mga email address.
share-where = Saan ito napupunta
share-where-detail = Sa crash tracker ng Katna sa Sentry, na naka-store sa EU. Walang ID na nag-uugnay sa mga ulat sa iyo.
share-dont-send = Huwag ipadala
share-send = Ipadala ang mga ulat ng pag-crash
share-sending = Ipapadala ang mga ulat ng pag-crash. Salamat.
share-local = Mananatili sa computer na ito ang mga ulat ng pag-crash.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Maligayang pagdating sa Katna Mail
tour-welcome-text = Ipinapakita ng isang minutong tour kung nasaan ang lahat.
tour-not-now = Hindi ngayon
tour-start = Mag-tour
tour-close = Isara
tour-skip = Laktawan ang tour
tour-back = Bumalik
tour-done = Tapos na
tour-next = Susunod
tour-step = { $step } sa { $total }
tour-compose-title = Sumulat ng mensahe
tour-compose-text = Nagbubukas ang Mag-compose ng bagong mensahe sa kanang ibaba, kaya puwede kang magpatuloy sa pagbabasa habang sumusulat.
tour-search-title = Hanapin sa lahat ng mail mo
tour-search-text = Gumagana rin ang paghahanap kahit offline. Nagdadagdag ng mga filter ang button sa dulong kanan: sender, recipient, subject, mga petsa at mga attachment.
tour-menu-title = Ipakita o itago ang mga folder
tour-menu-text = Itinutupi ng button na ito ang listahan ng folder. Habang nakatago ito, ipatong ang pointer sa Mail sa kaliwa para makita ang mga folder.
tour-apps-title = Ang iyong mga app
tour-apps-text = Dito na nakatira ang Mail. Sasama rito sa bar na ito ang Kalendaryo, Mga Contact, Mga Gawain, Mga Tala at Mga Feed.
tour-tabs-title = Mga tab ng inbox
tour-tabs-text = Inaayos ang bagong mail sa Pangunahin, Mga Promosyon, Social, Mga Update at Mga Forum. Puwede mong i-off ang mga tab sa mabilisang setting.
tour-list-title = Ang iyong mga mensahe
tour-list-text = I-click ang isang mensahe para basahin ito. I-hover ito para sa mabilisang aksyon, i-right click para sa higit pa, o lagyan ng tsek ang ilan para sabay-sabay silang aksyunan.
tour-settings-title = Mabilisang setting
tour-settings-text = Palitan dito ang pane ng pagbabasa, density at tema. Doon din puwedeng simulan ulit ang tour.
tour-account-title = Ang iyong account
tour-account-text = Tingnan kung aling account ka naroroon, at magdagdag ng isa pa.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Hindi inaasahang huminto ang serbisyo sa background ng Katna.
    [one] Hindi inaasahang huminto ang serbisyo sa background ng Katna. May { $more } pang naka-save na ulat ng pag-crash.
   *[other] Hindi inaasahang huminto ang serbisyo sa background ng Katna. May { $more } pang naka-save na ulat ng pag-crash.
}
crash-mail = { $more ->
    [0] Hindi inaasahang nagsara ang Katna Mail noong huling beses.
    [one] Hindi inaasahang nagsara ang Katna Mail noong huling beses. May { $more } pang naka-save na ulat ng pag-crash.
   *[other] Hindi inaasahang nagsara ang Katna Mail noong huling beses. May { $more } pang naka-save na ulat ng pag-crash.
}
crash-view = Tingnan ang ulat
crash-view-tooltip = Buksan ang ulat, na naka-save sa computer na ito
crash-copy = Kopyahin ang ulat
crash-close = Isara
sign-in-again-text = Hinihiling ng { $provider } na mag-sign in ka ulit sa { $address }.
sign-in-again-button = Mag-sign in
sign-in-again-tooltip = Buksan ang sign-in page ng { $provider } sa iyong browser
sign-in-again-waiting = Hinihintay ang iyong browser…
sign-in-again-close = Isara
sign-in-again-done = Naka-sign in ulit sa { $address }. Kinukuha ang mail mo…
delete-ask-title = { $kind ->
    [conversation] { $count ->
        [one] Ilipat sa Basurahan ang pag-uusap na ito?
       *[other] Ilipat sa Basurahan ang { $count } pag-uusap?
    }
   *[message] { $count ->
        [one] Ilipat sa Basurahan ang mensaheng ito?
       *[other] Ilipat sa Basurahan ang { $count } mensahe?
    }
}
delete-ask-body = { $count ->
    [one] Puwede mo itong i-undo kaagad pagkatapos, o ibalik ito mula sa Basurahan mamaya.
   *[other] Puwede mo itong i-undo kaagad pagkatapos, o ibalik ang mga ito mula sa Basurahan mamaya.
}
delete-ask-confirm = Ilipat sa Basurahan
delete-forever-title = { $kind ->
    [conversation] { $count ->
        [one] I-delete nang permanente ang pag-uusap na ito?
       *[other] I-delete nang permanente ang { $count } pag-uusap?
    }
   *[message] { $count ->
        [one] I-delete nang permanente ang mensaheng ito?
       *[other] I-delete nang permanente ang { $count } mensahe?
    }
}
delete-forever-body = { $count ->
    [one] Nadedelete rin ito sa server. Hindi na ito maibabalik.
   *[other] Nadedelete rin ang mga ito sa server. Hindi na ito maibabalik.
}
delete-forever-confirm = I-delete nang permanente
delete-ask-dont-ask = Huwag nang magtanong
delete-ask-cancel = Kanselahin
