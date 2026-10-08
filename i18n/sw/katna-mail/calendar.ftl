# Katna Mail, Swahili (Kiswahili): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Leo
calendar-today-tip = Nenda leo
calendar-view-day = Siku
calendar-view-week = Wiki
calendar-view-month = Mwezi
calendar-view-year = Mwaka
calendar-view-schedule = Ratiba
calendar-view-days =
    { $count ->
        [one] siku { $count }
       *[other] siku { $count }
    }
calendar-options = Chaguo
calendar-density = Msongamano
calendar-density-responsive = Hujirekebisha kulingana na skrini yako
calendar-density-comfortable = Starehe
calendar-density-compact = Fupi
calendar-custom-days = Mwonekano maalum
calendar-second-zone = Saa za eneo la pili
calendar-zone-none = Hamna
calendar-zone = { $zone } ({ $offset })
calendar-share-free = Shiriki nyakati zisizo na shughuli
calendar-free-subject = Nyakati ambazo sina shughuli
calendar-free-intro = Hizi ni baadhi ya nyakati ambazo sina shughuli ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = Sina wakati wowote usio na shughuli katika siku chache zijazo za kazi.
calendar-previous-day = Siku iliyotangulia
calendar-next-day = Siku inayofuata
calendar-previous-week = Wiki iliyotangulia
calendar-next-week = Wiki ijayo
calendar-previous-month = Mwezi uliotangulia
calendar-next-month = Mwezi ujao
calendar-previous-year = Mwaka uliotangulia
calendar-next-year = Mwaka ujao
calendar-previous-period = Mapema zaidi
calendar-next-period = Baadaye zaidi
calendar-title-months = { $first } – { $last }
calendar-loading = Inapakia…
calendar-read-failed = Kalenda haikuweza kusomwa: { $error }
calendar-sets = Seti za kalenda
calendar-set-add = Hifadhi kalenda zinazoonyeshwa kama seti
calendar-set-name = Jina la seti
calendar-set-remove = Ondoa seti
calendar-local = Kompyuta hii
calendar-account-gone = Akaunti iliyoondolewa
calendar-account-sign-in = Ingia tena ili kuonyesha kalenda
calendar-account-signed-in = Umeingia tena kwenye { $address }. Inapata kalenda zako…
calendar-account-sign-in-refused = { $provider } haikuruhusu Katna kuingia. Jaribu tena, na uruhusu ufikiaji wa kalenda zako.
calendar-account-refused = Seva haikukubali nenosiri. Yahoo, iCloud, Zoho na nyinginezo zinahitaji nenosiri la programu.
calendar-account-change-password = Badilisha nenosiri
calendar-account-change-password-tooltip = Andika nenosiri jipya; Katna hulikagua na seva
calendar-account-not-enabled = Ufikiaji wa kalenda kwa Katna bado haujawashwa.
calendar-account-failed = Kalenda hazikuweza kusomwa.
calendar-account-error = Kalenda hazikuweza kusomwa: { $reason }
calendar-account-none = Hakuna kalenda zilizopatikana
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = Hakuna kalenda zilizopatikana: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } huonyesha kalenda kwa Katna iliyoingia kwa { $provider } pekee.
calendar-account-sign-in-with = Ingia kwa { $provider }
calendar-account-looking = Inatafuta kalenda…
calendar-account-try-again = Jaribu tena
calendar-account-try-again-tooltip = Kagua kalenda za akaunti hii tena sasa
calendar-account-fixing = Inashughulikia…
calendar-birthdays = Siku za kuzaliwa
calendar-tasks = Majukumu
calendar-birthday-of = Siku ya kuzaliwa ya { $name }
calendar-empty-title = Bado hakuna kalenda
calendar-empty-text = Katna huonyesha hapa kalenda za akaunti zako za Google na Microsoft zikishasawazishwa, pamoja na za seva nyingine zinazotumia CalDAV.
calendar-schedule-empty = Hakuna kilichopangwa kwa miezi miwili ijayo.
calendar-search = Tafuta matukio
calendar-search-past = Matukio yaliyopita
calendar-search-none = Hakuna matukio yanayolingana na utafutaji wako.
calendar-no-title = (Hakuna kichwa)
calendar-all-day = Siku nzima
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = zingine { $count }
calendar-peek-day = { $weekday }, { $day }
calendar-repeats = Hujirudia
calendar-join = Jiunge
calendar-join-with = Jiunge kwa { $service }
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
calendar-open-mail = Fungua barua
calendar-open-contact = Fungua anwani
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
# Right-click menus on the calendar: on a free time or day, an event and
# a task.
calendar-menu-new-event = Tukio jipya
calendar-event-window-title = Tukio jipya
# Shows the day right-clicked on its own, in the Day view.
calendar-menu-open-day = Fungua siku
calendar-menu-duplicate = Nakili
calendar-menu-color = Rangi
# The event takes its calendar's color.
calendar-menu-color-calendar = Rangi ya kalenda
# A task's new due day, a week from today.
calendar-menu-in-a-week = Baada ya wiki moja
# Event colors, by the names Google Calendar gives them.
calendar-color-tomato = Nyanya
calendar-color-flamingo = Heroe
calendar-color-tangerine = Chenza
calendar-color-banana = Ndizi
calendar-color-sage = Kijani kijivu
calendar-color-basil = Mrehani
calendar-color-peacock = Tausi
calendar-color-blueberry = Beri ya buluu
calendar-color-lavender = Lavenda
calendar-color-grape = Zabibu
calendar-color-graphite = Grafiti
calendar-menu-only-this = Onyesha hii tu
calendar-menu-rename = Badilisha jina
calendar-menu-remove = Ondoa kwenye orodha
calendar-menu-delete = Futa
calendar-menu-new-calendar = Kalenda mpya
calendar-menu-show-all = Onyesha zote
calendar-menu-hide-all = Ficha zote
calendar-menu-account-settings = Mipangilio ya akaunti
calendar-why-main = Kalenda kuu
calendar-why-last = Iko moja tu hapa
calendar-why-owner = Mmiliki pekee
calendar-why-contacts = Kutoka Anwani
calendar-why-unreached = Haikufikiwa
calendar-name-placeholder = Jina la kalenda
calendar-toast-added = “{ $name }” imeongezwa
calendar-toast-renamed = Jina la kalenda limebadilishwa
calendar-toast-recolored = Rangi ya kalenda imebadilishwa
calendar-toast-deleted = “{ $name }” imefutwa
calendar-toast-removed = “{ $name }” imeondolewa kwenye orodha yako
calendar-edit-failed = Kalenda haikubadilishwa: { $reason }
calendar-delete-title = Ufute “{ $name }”?
calendar-delete-confirm = Futa
calendar-deleting = Inafuta…
calendar-delete-heading = Kitakachofutwa:
calendar-delete-events = Kalenda na matukio yake yote
calendar-delete-shared = Kwa kila mtu aliyeshirikiwa nayo
calendar-delete-server = Inafutwa kutoka { $account } kwenye huduma ya barua, si katika Katna pekee.
calendar-delete-local = Inafutwa kutoka kwenye kompyuta hii.
calendar-remove-title = Uondoe “{ $name }” kwenye orodha yako?
calendar-remove-confirm = Ondoa
calendar-removing = Inaondoa…
calendar-remove-heading = Kinachobadilika:
calendar-remove-events = Hutaona tena matukio yake, hapa na kwenye programu zako nyingine
calendar-remove-server = Kalenda inabaki kwa mmiliki wake, anayeweza kukushirikisha tena.
calendar-kind-event = Tukio
calendar-kind-task = Jukumu
calendar-kind-focus = Muda wa kuzingatia
calendar-kind-out-of-office = Nje ya ofisi
calendar-kind-working-location = Mahali pa kazi
calendar-task-added = Jukumu limeongezwa
calendar-task-added-to = Jukumu limeongezwa kwenye { $list }
calendar-task-list-local = Kwenye kompyuta hii
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
