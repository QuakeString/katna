# Katna Mail, Swahili (Kiswahili): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Leo
calendar-today-tip = Nenda leo
calendar-view-day = Siku
calendar-view-week = Wiki
calendar-view-month = Mwezi
calendar-view-schedule = Ratiba
calendar-options = Chaguo
calendar-density = Msongamano
calendar-density-responsive = Hujirekebisha kulingana na skrini yako
calendar-density-comfortable = Starehe
calendar-density-compact = Fupi
calendar-second-zone = Saa za eneo la pili
calendar-zone-none = Hamna
calendar-zone = { $zone } ({ $offset })
calendar-previous-day = Siku iliyotangulia
calendar-next-day = Siku inayofuata
calendar-previous-week = Wiki iliyotangulia
calendar-next-week = Wiki ijayo
calendar-previous-month = Mwezi uliotangulia
calendar-next-month = Mwezi ujao
calendar-previous-period = Mapema zaidi
calendar-next-period = Baadaye zaidi
calendar-title-months = { $first } – { $last }
calendar-loading = Inapakia…
calendar-read-failed = Kalenda haikuweza kusomwa: { $error }
calendar-local = Kompyuta hii
calendar-account-gone = Akaunti iliyoondolewa
calendar-empty-title = Bado hakuna kalenda
calendar-empty-text = Katna huonyesha hapa kalenda za akaunti zako za Google na Microsoft zikishasawazishwa, pamoja na za seva nyingine zinazotumia CalDAV.
calendar-schedule-empty = Hakuna kilichopangwa kwa miezi miwili ijayo.
calendar-no-title = (Hakuna kichwa)
calendar-all-day = Siku nzima
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = zingine { $count }
calendar-repeats = Hujirudia
calendar-join = Jiunge
calendar-email-guests = Tuma barua pepe kwa wageni
calendar-running-late = Nimechelewa
calendar-late-subject = Nimechelewa: { $title }
calendar-late-body = Samahani, nitachelewa dakika chache kwa { $title }. Nitafika hivi karibuni.
calendar-guests =
    { $count ->
        [one] mgeni { $count }
       *[other] wageni { $count }
    }
calendar-guest-answers = ndiyo { $yes }, labda { $maybe }, hapana { $no }, wanasubiri { $waiting }
calendar-organizer = Mwandalizi
calendar-optional = Hiari
calendar-open-web = Fungua kwenye kivinjari
calendar-close = Funga

## Adding, changing and deleting events.

calendar-add-title = Ongeza kichwa
calendar-add-location = Ongeza mahali
calendar-add-notes = Ongeza maelezo
calendar-add-guests = Ongeza wageni
calendar-remove-guest = Ondoa
calendar-add-meet = Ongeza mkutano wa video wa Google Meet
calendar-add-teams = Ongeza mkutano wa Teams
calendar-has-call = Simu ya video imeongezwa
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Siku nzima
calendar-more-options = Chaguo zaidi
calendar-save = Hifadhi
calendar-saved = Tukio limehifadhiwa
calendar-deleted = Tukio limefutwa
calendar-discard = Tupa mabadiliko
calendar-edit = Hariri tukio
calendar-delete = Futa tukio
calendar-event-details = Maelezo ya tukio
calendar-kind-event = Tukio
calendar-kind-focus = Muda wa kuzingatia
calendar-kind-out-of-office = Nje ya ofisi
calendar-kind-working-location = Mahali pa kazi
calendar-working-home = Nyumbani
calendar-busy = Ana shughuli
calendar-free = Yuko huru
calendar-cancel = Ghairi
calendar-ok = Sawa
calendar-read-only = Huwezi kubadilisha matukio katika kalenda hii
calendar-none-editable = Bado huna kalenda unayoweza kuongeza matukio
calendar-no-such-time = Muda huo haupo katika saa za eneo lako
calendar-end-before-start = Tukio linaisha kabla ya kuanza
calendar-repeat-never = Halijirudii
calendar-repeat-daily = Kila siku
calendar-repeat-weekly = Kila wiki siku ya { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Kila mwezi { $weekday } ya kwanza
        [2] Kila mwezi { $weekday } ya pili
        [3] Kila mwezi { $weekday } ya tatu
        [4] Kila mwezi { $weekday } ya nne
       *[other] Kila mwezi { $weekday } ya mwisho
    }
calendar-repeat-yearly = Kila mwaka tarehe { $day }
calendar-repeat-weekdays = Kila siku ya kazi (Jumatatu hadi Ijumaa)
calendar-repeat-custom = Maalum
calendar-reminder-none = Hakuna arifa
calendar-reminder-at-start = Mwanzoni
calendar-reminder-minutes =
    { $count ->
        [one] dakika { $count } kabla
       *[other] dakika { $count } kabla
    }
calendar-reminder-hours =
    { $count ->
        [one] saa { $count } kabla
       *[other] saa { $count } kabla
    }
calendar-reminder-days =
    { $count ->
        [one] siku { $count } kabla
       *[other] siku { $count } kabla
    }
calendar-scope-edit-title = Hariri tukio linalojirudia
calendar-scope-delete-title = Futa tukio linalojirudia
calendar-scope-this = Tukio hili
calendar-scope-following = Tukio hili na yanayofuata
calendar-scope-all = Matukio yote
calendar-scope-respond-title = Jibu kwa tukio linalojirudia
calendar-going = Utaenda?
calendar-answer-yes = Ndiyo
calendar-answer-no = Hapana
calendar-answer-maybe = Labda
calendar-answered-yes = Unaenda
calendar-answered-no = Huendi
calendar-answered-maybe = Huenda ukaenda

## The card at the top of a mail with an invitation.

calendar-invite = Mwaliko
calendar-invite-cancelled = Tukio limeghairiwa
calendar-invite-reply = { $name } amejibu
calendar-invite-reply-yes = { $name } amekubali
calendar-invite-reply-no = { $name } amekataa
calendar-invite-reply-maybe = { $name } huenda akaenda
calendar-invite-organizer = Imeandaliwa na { $name }
calendar-invite-open = Fungua kwenye Kalenda
calendar-invite-not-yet = Bado haipo kwenye kalenda yako. Unaweza kujibu itakapolandanishwa.
calendar-invite-by-mail = Haipo kwenye kalenda yako: jibu lako litatumwa kwa mwandaaji kwa barua pepe.
calendar-mail-yes = Imekubaliwa: { $title }
calendar-mail-yes-body = Mwaliko huu umekubaliwa na { $name }.
calendar-mail-no = Imekataliwa: { $title }
calendar-mail-no-body = Mwaliko huu umekataliwa na { $name }.
calendar-mail-maybe = Kwa muda: { $title }
calendar-mail-maybe-body = Mwaliko huu umekubaliwa kwa muda na { $name }.
calendar-invite-your-day = Siku yako
calendar-invite-clashes =
    { $count ->
        [one] Inagongana na tukio { $count }
       *[other] Inagongana na matukio { $count }
    }

## The day's agenda beside the mail.

agenda-show = Onyesha ratiba ya siku
agenda-hide = Ficha ratiba
agenda-today = Leo, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = Hakuna kilichopangwa siku hii.
