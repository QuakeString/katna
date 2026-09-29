# Katna Mail, Hausa (Hausa): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Yau
calendar-today-tip = Je zuwa yau
calendar-view-day = Rana
calendar-view-week = Mako
calendar-view-month = Wata
calendar-view-year = Shekara
calendar-view-schedule = Jadawali
calendar-view-days =
    { $count ->
        [one] rana { $count }
       *[other] kwanaki { $count }
    }
calendar-options = Zaɓuka
calendar-density = Yawa
calendar-density-responsive = Yana daidaita da allonka
calendar-density-comfortable = Mai sauƙi
calendar-density-compact = Matsatsi
calendar-custom-days = Kallon na musamman
calendar-second-zone = Yankin lokaci na biyu
calendar-zone-none = Babu
calendar-zone = { $zone } ({ $offset })
calendar-share-free = Raba lokutan hutu
calendar-free-subject = Lokutan da nake hutu
calendar-free-intro = Ga wasu lokutan da nake hutu ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = Ba ni da lokacin hutu a cikin ƙalilan na kwanakin aiki masu zuwa.
calendar-previous-day = Ranar da ta gabata
calendar-next-day = Ranar gobe
calendar-previous-week = Makon da ya gabata
calendar-next-week = Mako mai zuwa
calendar-previous-month = Watan da ya gabata
calendar-next-month = Wata mai zuwa
calendar-previous-year = Shekarar da ta gabata
calendar-next-year = Shekara mai zuwa
calendar-previous-period = Na baya
calendar-next-period = Na gaba
calendar-title-months = { $first } – { $last }
calendar-loading = Ana lodawa…
calendar-read-failed = Ba a iya karanta kalanda ba: { $error }
calendar-sets = Rukunin kalanda
calendar-set-add = Ajiye kalandun da ake nunawa a matsayin rukuni
calendar-set-name = Sunan rukunin
calendar-set-remove = Cire rukuni
calendar-local = Wannan kwamfutar
calendar-account-gone = Asusun da aka cire
calendar-account-sign-in = Sake shiga don nuna kalandoji
calendar-account-signed-in = An sake shiga { $address }. Ana samo kalandojinku…
calendar-account-sign-in-refused = { $provider } bai bar Katna ya shiga ba. Ku sake gwadawa, kuma ku ba da izinin shiga kalandojinku.
calendar-account-refused = Sabar ba ta karɓi kalmar sirrin ba. Yahoo, iCloud, Zoho da wasu suna buƙatar kalmar sirrin manhaja.
calendar-account-change-password = Canza kalmar sirri
calendar-account-change-password-tooltip = Buɗe Saituna > Asusu
calendar-account-not-enabled = Ba a kunna damar shiga kalanda don Katna ba tukuna.
calendar-account-failed = Ba a iya karanta kalandojin ba.
calendar-account-error = Ba a iya karanta kalandojin ba: { $reason }
calendar-account-none = Ba a sami kalanda ba
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = Ba a sami kalanda ba: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } yana nuna kalandoji ga Katna ne kawai idan ya shiga da { $provider }.
calendar-account-sign-in-with = Shiga da { $provider }
calendar-account-looking = Ana neman kalandoji…
calendar-account-try-again = Sake gwadawa
calendar-account-try-again-tooltip = Sake duba kalandojin wannan asusun yanzu
calendar-account-fixing = Ana aiki a kai…
calendar-birthdays = Ranakun haihuwa
calendar-birthday-of = Ranar haihuwar { $name }
calendar-empty-title = Babu kalanda tukuna
calendar-empty-text = Katna yana nuna kalandar asusun Google da Microsoft naka a nan da zarar an daidaita su, tare da na sauran sabar da ke goyon bayan CalDAV.
calendar-schedule-empty = Babu abin da aka tsara a cikin watanni biyu masu zuwa.
calendar-search = Bincika taruka
calendar-search-past = Tarukan da suka wuce
calendar-search-none = Babu tarukan da suka dace da bincikenka.
calendar-no-title = (Babu take)
calendar-all-day = Duk rana
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = ƙarin { $count }
calendar-repeats = Yana maimaituwa
calendar-join = Shiga
calendar-email-guests = Aika wa baƙi wasiƙa
calendar-running-late = Ina makara
calendar-late-subject = Ina makara: { $title }
calendar-late-body = Yi haƙuri, zan yi ɗan jinkiri zuwa { $title } na 'yan mintuna. Zan iso nan ba da jimawa ba.
calendar-guests =
    { $count ->
        [one] baƙo { $count }
       *[other] baƙi { $count }
    }
calendar-guest-answers = eh { $yes }, wataƙila { $maybe }, a'a { $no }, ana jira { $waiting }
calendar-organizer = Mai shiryawa
calendar-optional = Na zaɓi
calendar-open-web = Buɗe a burauza
calendar-open-contact = Buɗe lambar sadarwa
calendar-close = Rufe

## Adding, changing and deleting events.

calendar-add-title = Ƙara take
calendar-add-location = Ƙara wuri
calendar-add-notes = Ƙara bayani
calendar-add-guests = Ƙara baƙi
calendar-remove-guest = Cire
calendar-add-meet = Ƙara taron bidiyo na Google Meet
calendar-add-teams = Ƙara taron Teams
calendar-has-call = An ƙara kiran bidiyo
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Duk rana
calendar-more-options = Ƙarin zaɓuɓɓuka
calendar-save = Ajiye
calendar-saved = An ajiye taron
calendar-deleted = An share taron
calendar-discard = Watsar da canje-canje
calendar-edit = Gyara taron
calendar-delete = Share taron
calendar-event-details = Bayanan taron
calendar-kind-event = Taron
calendar-kind-focus = Lokacin mai da hankali
calendar-kind-out-of-office = Ba a ofis
calendar-kind-working-location = Wurin aiki
calendar-working-home = Gida
calendar-busy = Yana da aiki
calendar-free = Babu aiki
calendar-cancel = Soke
calendar-ok = To
calendar-read-only = Ba za ka iya canza taruka a wannan kalanda ba
calendar-none-editable = Babu kalanda tukuna da za ka iya ƙara taruka a ciki
calendar-no-such-time = Wannan lokacin babu shi a yankin lokacinka
calendar-end-before-start = Taron yana ƙarewa kafin ya fara
calendar-repeat-never = Ba ya maimaituwa
calendar-repeat-daily = Kullum
calendar-repeat-weekly = Kowane mako a ranar { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Kowane wata a { $weekday } na farko
        [2] Kowane wata a { $weekday } na biyu
        [3] Kowane wata a { $weekday } na uku
        [4] Kowane wata a { $weekday } na huɗu
       *[other] Kowane wata a { $weekday } na ƙarshe
    }
calendar-repeat-yearly = Kowace shekara a { $day }
calendar-repeat-weekdays = Kowace ranar aiki (Litinin zuwa Juma'a)
calendar-repeat-custom = Na musamman
calendar-reminder-none = Babu sanarwa
calendar-reminder-at-start = A farkon lokaci
calendar-reminder-minutes =
    { $count ->
        [one] minti { $count } kafin
       *[other] mintuna { $count } kafin
    }
calendar-reminder-hours =
    { $count ->
        [one] awa { $count } kafin
       *[other] sa'o'i { $count } kafin
    }
calendar-reminder-days =
    { $count ->
        [one] rana { $count } kafin
       *[other] kwanaki { $count } kafin
    }
calendar-scope-edit-title = Gyara taron da ke maimaituwa
calendar-scope-delete-title = Share taron da ke maimaituwa
calendar-scope-this = Wannan taron
calendar-scope-following = Wannan taron da masu biye
calendar-scope-all = Dukkan taruka
calendar-scope-respond-title = Amsa don taron da ke maimaituwa
calendar-going = Za ka je?
calendar-answer-yes = Eh
calendar-answer-no = A'a
calendar-answer-maybe = Wataƙila
calendar-answered-yes = Za ka je
calendar-answered-no = Ba za ka je ba
calendar-answered-maybe = Wataƙila ka je

## The card at the top of a mail with an invitation.

calendar-invite = Gayyata
calendar-invite-cancelled = An soke taron
calendar-invite-reply = { $name }: amsa
calendar-invite-reply-yes = { $name }: Eh
calendar-invite-reply-no = { $name }: A'a
calendar-invite-reply-maybe = { $name }: Wataƙila
calendar-invite-organizer = Mai shiryawa: { $name }
calendar-invite-open = Buɗe a Kalanda
calendar-invite-not-yet = Ba ya cikin kalandarka tukuna. Za ka iya amsawa bayan ya yi sync.
calendar-invite-by-mail = Ba ya cikin kalandarka: amsarka za ta tafi wurin mai shiryawa ta imel.
calendar-mail-yes = An karɓa: { $title }
calendar-mail-yes-body = An karɓi wannan gayyata daga { $name }.
calendar-mail-no = An ƙi: { $title }
calendar-mail-no-body = An ƙi wannan gayyata daga { $name }.
calendar-mail-maybe = Na ɗan lokaci: { $title }
calendar-mail-maybe-body = An karɓi wannan gayyata na ɗan lokaci daga { $name }.
calendar-invite-your-day = Ranarka
calendar-invite-clashes =
    { $count ->
        [one] Yana karo da taro { $count }
       *[other] Yana karo da tarurruka { $count }
    }

## The day's agenda beside the mail.

agenda-show = Nuna jadawalin ranar
agenda-hide = Ɓoye jadawalin
agenda-today = Yau, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = Babu abin da aka tsara a wannan ranar.
