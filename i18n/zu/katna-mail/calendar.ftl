# Katna Mail, Zulu (isiZulu): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Namuhla
calendar-today-tip = Iya kuNamuhla
calendar-view-day = Usuku
calendar-view-week = Iviki
calendar-view-month = Inyanga
calendar-view-year = Unyaka
calendar-view-schedule = Uhlelo
calendar-view-days =
    { $count ->
        [one] Usuku { $count }
       *[other] Izinsuku { $count }
    }
calendar-options = Izinketho
calendar-density = Ukuminyana
calendar-density-responsive = Iyavumelana nesikrini sakho
calendar-density-comfortable = Kunethezekile
calendar-density-compact = Kuminyene
calendar-custom-days = Ukubuka ngokwezifiso
calendar-second-zone = Indawo yesikhathi yesibili
calendar-zone-none = Lutho
calendar-zone = { $zone } ({ $offset })
calendar-share-free = Yabelana ngezikhathi zokukhululeka
calendar-free-subject = Izikhathi engikhululekile ngazo
calendar-free-intro = Nazi izikhathi ezimbalwa engikhululekile ngazo ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = Anginaso isikhathi sokukhululeka ezinsukwini ezimbalwa zomsebenzi ezizayo.
calendar-previous-day = Usuku olwedlule
calendar-next-day = Usuku olulandelayo
calendar-previous-week = Iviki eledlule
calendar-next-week = Iviki elilandelayo
calendar-previous-month = Inyanga edlule
calendar-next-month = Inyanga elandelayo
calendar-previous-year = Unyaka odlule
calendar-next-year = Unyaka olandelayo
calendar-previous-period = Ngaphambili
calendar-next-period = Kamuva
calendar-title-months = { $first } – { $last }
calendar-loading = Iyalayisha…
calendar-read-failed = Ikhalenda alikwazanga ukufundwa: { $error }
calendar-sets = Amasethi amakhalenda
calendar-set-add = Londoloza amakhalenda aboniswayo njengesethi
calendar-set-name = Igama lesethi
calendar-set-remove = Susa isethi
calendar-local = Kule khompyutha
calendar-account-gone = I-akhawunti isusiwe
calendar-account-sign-in = Ngena futhi ukuze ubonise amakhalenda
calendar-account-signed-in = Ungene futhi ku-{ $address }. Kutholwa amakhalenda akho…
calendar-account-sign-in-refused = I-{ $provider } ayizange ivumele i-Katna ingene. Zama futhi, bese uvumela ukufinyelela kumakhalenda akho.
calendar-account-refused = Iseva ayizange yamukele iphasiwedi. I-Yahoo, i-iCloud, i-Zoho nabanye badinga iphasiwedi yohlelo lokusebenza.
calendar-account-change-password = Shintsha iphasiwedi
calendar-account-change-password-tooltip = Vula Izilungiselelo > Ama-akhawunti
calendar-account-not-enabled = Ukufinyelela kwekhalenda kwe-Katna akukavulwa.
calendar-account-failed = Amakhalenda awakwazanga ukufundwa.
calendar-account-error = Amakhalenda awakwazanga ukufundwa: { $reason }
calendar-account-none = Awekho amakhalenda atholakele
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = Awekho amakhalenda atholakele: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = I-{ $provider } ibonisa amakhalenda kuphela ku-Katna engene nge-{ $provider }.
calendar-account-sign-in-with = Ngena nge-{ $provider }
calendar-account-looking = Kufunwa amakhalenda…
calendar-account-try-again = Zama futhi
calendar-account-try-again-tooltip = Hlola amakhalenda ale akhawunti futhi manje
calendar-account-fixing = Kuyasebenzwa kukho…
calendar-birthdays = Osuku lokuzalwa
calendar-birthday-of = Usuku lokuzalwa luka-{ $name }
calendar-empty-title = Awukho amakhalenda okwamanje
calendar-empty-text = I-Katna ibonisa amakhalenda ama-akhawunti akho e-Google ne-Microsoft lapha ngemva kokuvumelanisa, kanye nawamanye amaseva anikeza i-CalDAV.
calendar-schedule-empty = Akukho okuhleliwe ezinyangeni ezimbili ezizayo.
calendar-search = Sesha imicimbi
calendar-search-past = Imicimbi edlule
calendar-search-none = Ayikho imicimbi efana nosesho lwakho.
calendar-no-title = (Alukho isihloko)
calendar-all-day = Usuku lonke
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } ngaphezulu
calendar-repeats = Iyaphindaphinda
calendar-join = Joyina
calendar-email-guests = Thumela izihambeli i-imeyili
calendar-running-late = Ngiyephuza
calendar-late-subject = Ngiyephuza: { $title }
calendar-late-body = Ngiyaxolisa, ngiphuza imizuzu embalwa ku-{ $title }. Ngizofika maduze.
calendar-guests =
    { $count ->
        [one] Isihambeli { $count }
       *[other] Izihambeli { $count }
    }
calendar-guest-answers = { $yes } yebo, { $maybe } mhlawumbe, { $no } cha, { $waiting } okulindile
calendar-organizer = Umhleli
calendar-optional = Okungakhethwa
calendar-open-web = Vula esiphequluli
calendar-open-contact = Vula oxhumana naye
calendar-close = Vala

## Adding, changing and deleting events.

calendar-add-title = Engeza isihloko
calendar-add-location = Engeza indawo
calendar-add-notes = Engeza incazelo
calendar-add-guests = Engeza izihambeli
calendar-remove-guest = Susa
calendar-add-meet = Engeza ikholi yevidiyo ye-Google Meet
calendar-add-teams = Engeza umhlangano we-Teams
calendar-has-call = Ikholi yevidiyo ingeziwe
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Usuku lonke
calendar-more-options = Okunye okungakhethwa
calendar-save = Londoloza
calendar-saved = Umcimbi ulondoloziwe
calendar-deleted = Umcimbi ususiwe
calendar-discard = Lahla izinguquko
calendar-edit = Hlela umcimbi
calendar-delete = Susa umcimbi
calendar-event-details = Imininingwane yomcimbi
calendar-kind-event = Umcimbi
calendar-kind-focus = Isikhathi sokugxila
calendar-kind-out-of-office = Ngingekho ehhovisi
calendar-kind-working-location = Indawo yokusebenza
calendar-working-home = Ekhaya
calendar-busy = Umatasa
calendar-free = Ukhululekile
calendar-cancel = Khansela
calendar-ok = Kulungile
calendar-read-only = Awukwazi ukushintsha imicimbi kule khalenda
calendar-none-editable = Akukabikho ikhalenda ongengeza kulo imicimbi
calendar-no-such-time = Leyo nkathi ayikho endaweni yakho yesikhathi
calendar-end-before-start = Umcimbi uphela ungakaqali
calendar-repeat-never = Akuphindwa
calendar-repeat-daily = Nsuku zonke
calendar-repeat-weekly = Njalo ngeviki: { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Njalo ngenyanga: okokuqala { $weekday }
        [2] Njalo ngenyanga: okwesibili { $weekday }
        [3] Njalo ngenyanga: okwesithathu { $weekday }
        [4] Njalo ngenyanga: okwesine { $weekday }
       *[other] Njalo ngenyanga: okokugcina { $weekday }
    }
calendar-repeat-yearly = Njalo ngonyaka: { $day }
calendar-repeat-weekdays = Yonke insuku zomsebenzi (uMsombuluko kuya kuLwesihlanu)
calendar-repeat-custom = Ngokwezifiso
calendar-reminder-none = Akukho isaziso
calendar-reminder-at-start = Ekuqaleni
calendar-reminder-minutes =
    { $count ->
        [one] Umzuzu { $count } ngaphambili
       *[other] Imizuzu { $count } ngaphambili
    }
calendar-reminder-hours =
    { $count ->
        [one] Ihora { $count } ngaphambili
       *[other] Amahora { $count } ngaphambili
    }
calendar-reminder-days =
    { $count ->
        [one] Usuku { $count } ngaphambili
       *[other] Izinsuku { $count } ngaphambili
    }
calendar-scope-edit-title = Hlela umcimbi obuyelelwayo
calendar-scope-delete-title = Susa umcimbi obuyelelwayo
calendar-scope-this = Lo mcimbi
calendar-scope-following = Lo nemicimbi elandelayo
calendar-scope-all = Yonke imicimbi
calendar-scope-respond-title = Impendulo yomcimbi obuyelelwayo
calendar-going = Uyahamba?
calendar-answer-yes = Yebo
calendar-answer-no = Cha
calendar-answer-maybe = Mhlawumbe
calendar-answered-yes = Uyahamba
calendar-answered-no = Awuhambi
calendar-answered-maybe = Ungahamba

## The card at the top of a mail with an invitation.

calendar-invite = Isimemo
calendar-invite-cancelled = Umcimbi ukhanselwe
calendar-invite-reply = { $name } uphendule
calendar-invite-reply-yes = { $name } wamukele
calendar-invite-reply-no = { $name } wenqabile
calendar-invite-reply-maybe = { $name } angahamba
calendar-invite-organizer = Kuhlelwe ngu-{ $name }
calendar-invite-open = Vula ku-Khalenda
calendar-invite-not-yet = Akukho ekhalendeni lakho okwamanje. Ungaphendula uma isivumelanisiwe.
calendar-invite-by-mail = Akukho ekhalendeni lakho: impendulo yakho iya kumhleli ngeposi.
calendar-mail-yes = Yamukelwe: { $title }
calendar-mail-yes-body = { $name } wamukele lesi simemo.
calendar-mail-no = Yenqatshiwe: { $title }
calendar-mail-no-body = { $name } wenqabile lesi simemo.
calendar-mail-maybe = Yamukelwe okwesikhashana: { $title }
calendar-mail-maybe-body = { $name } wamukele lesi simemo okwesikhashana.
calendar-invite-your-day = Usuku lwakho
calendar-invite-clashes =
    { $count ->
        [one] Kuphambana nomcimbi { $count }
       *[other] Kuphambana nemicimbi { $count }
    }

## The day's agenda beside the mail.

agenda-show = Bonisa uhlelo losuku
agenda-hide = Fihla uhlelo
agenda-today = Namuhla, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = Akukho okuhleliwe kulolu suku.
