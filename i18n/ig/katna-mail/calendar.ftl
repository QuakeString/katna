# Katna Mail, Igbo (Igbo): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Taa
calendar-today-tip = Gaa na taa
calendar-view-day = Ụbọchị
calendar-view-week = Izu
calendar-view-month = Onwa
calendar-view-year = Afọ
calendar-view-schedule = Nhazi oge
calendar-view-days =
    { $count ->
       *[other] Ụbọchị { $count }
    }
calendar-options = Nhọrọ
calendar-density = Njupụta
calendar-density-responsive = Na-eso ihuenyo gị
calendar-density-comfortable = Ọ dị mma
calendar-density-compact = Nke dị nso
calendar-custom-days = Ọhụhụ ahaziri
calendar-second-zone = Mpaghara oge nke abụọ
calendar-zone-none = Ọ dịghị
calendar-zone = { $zone } ({ $offset })
calendar-share-free = Kesaa oge ị nwere
calendar-free-subject = Oge m nwere
calendar-free-intro = Lee oge ụfọdụ m nwere ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = Enweghị m oge ọ bụla n'ụbọchị ọrụ ole na ole na-abịa.
calendar-previous-day = Ụbọchị gara aga
calendar-next-day = Ụbọchị na-esote
calendar-previous-week = Izu gara aga
calendar-next-week = Izu na-esote
calendar-previous-month = Onwa gara aga
calendar-next-month = Onwa na-esote
calendar-previous-year = Afọ gara aga
calendar-next-year = Afọ na-esote
calendar-previous-period = Nke gara aga
calendar-next-period = Nke ga-abịa
calendar-title-months = { $first } – { $last }
calendar-loading = Na-ebugo…
calendar-read-failed = Enweghị ike ịgụ kalịnda: { $error }
calendar-sets = Otu kalịnda
calendar-set-add = Chekwaa kalịnda ndị a na-egosi dị ka otu
calendar-set-name = Aha otu ahụ
calendar-set-remove = Wepụ otu
calendar-local = Na kọmputa a
calendar-account-gone = Akaụntụ e wepụrụ
calendar-account-sign-in = Banye ọzọ iji gosi kalịnda
calendar-account-signed-in = Abanyela ọzọ na { $address }. Na-enweta kalịnda gị…
calendar-account-sign-in-refused = { $provider } ekweghị ka Katna banye. Nwaa ọzọ, ma kwe ka ọ nweta kalịnda gị.
calendar-account-refused = Sava ahụ anabataghị okwuntughe ahụ. Yahoo, iCloud, Zoho na ndị ọzọ chọrọ okwuntughe ngwa.
calendar-account-change-password = Gbanwee okwuntughe
calendar-account-change-password-tooltip = Mepee Ntọala > Akaụntụ
calendar-account-not-enabled = Agbanyebeghị ohere kalịnda maka Katna.
calendar-account-failed = Enweghị ike ịgụ kalịnda.
calendar-account-error = Enweghị ike ịgụ kalịnda: { $reason }
calendar-account-none = Ahụghị kalịnda ọ bụla
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = Ahụghị kalịnda ọ bụla: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } na-egosi kalịnda naanị Katna banyere na { $provider }.
calendar-account-sign-in-with = Banye na { $provider }
calendar-account-looking = Na-achọ kalịnda…
calendar-account-try-again = Nwaa ọzọ
calendar-account-try-again-tooltip = Lelee kalịnda akaụntụ a ọzọ ugbu a
calendar-account-fixing = Na-arụ ọrụ na ya…
calendar-birthdays = Ụbọchị ọmụmụ
calendar-birthday-of = Ụbọchị ọmụmụ { $name }
calendar-empty-title = Enwebeghị kalịnda
calendar-empty-text = Katna na-egosi kalịnda akaụntụ Google na Microsoft gị ebe a ozugbo e mekọrịtara ha, yana nke sava ndị ọzọ na-enye CalDAV.
calendar-schedule-empty = Ọ nweghị ihe e mere atụmatụ n'ime ọnwa abụọ na-abịa.
calendar-search = Chọọ ihe omume
calendar-search-past = Ihe omume gara aga
calendar-search-none = Ọ dịghị ihe omume dabara na ọchụchọ gị.
calendar-no-title = (Enweghị aha)
calendar-all-day = Ogologo ụbọchị
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } ọzọ
calendar-peek-day = { $weekday }, { $day }
calendar-repeats = Na-agbaghachi
calendar-join = Sonye
calendar-join-with = Sonye site na { $service }
calendar-email-guests = Zipụ ndị ọbịa email
calendar-running-late = Ana m egbu oge
calendar-late-subject = Ana m egbu oge: { $title }
calendar-late-body = Ndo, ana m egbu oge ọtụtụ nkeji maka { $title }. Aga m abịarute n'oge na-adịghị anya.
calendar-guests =
    { $count ->
       *[other] { $count } ọbịa
    }
calendar-guest-answers = { $yes } ee, { $maybe } ikekwe, { $no } mba, { $waiting } na-eche
calendar-organizer = Onye nhazi
calendar-optional = Nhọrọ
calendar-open-web = Mepe na ihe nchọgharị
calendar-open-mail = Mepee ozi
calendar-open-contact = Meghee kọntaktị
calendar-close = Mechie

## Adding, changing and deleting events.

calendar-add-title = Tinye aha ihe omume
calendar-add-location = Tinye ebe
calendar-add-notes = Tinye nkọwa
calendar-add-guests = Tinye ndị ọbịa
calendar-remove-guest = Wepụ
calendar-add-meet = Tinye ọkpụkpọ vidiyo Google Meet
calendar-add-teams = Tinye nzukọ Teams
calendar-has-call = Etinyela ọkpụkpọ vidiyo
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Ogologo ụbọchị
calendar-more-options = Nhọrọ ndị ọzọ
calendar-save = Chekwaa
calendar-saved = Echekwara ihe omume
calendar-deleted = Ehichapụla ihe omume
calendar-discard = Tụfuo mgbanwe
calendar-edit = Dezie ihe omume
calendar-delete = Hichapụ ihe omume
calendar-event-details = Nkọwa ihe omume
# Right-click menus on the calendar: on a free time or day, an event and
# a task.
calendar-menu-new-event = Ihe omume ọhụrụ
# Shows the day right-clicked on its own, in the Day view.
calendar-menu-open-day = Mepee ụbọchị
calendar-menu-duplicate = Mepụta oyiri
calendar-menu-color = Agba
# The event takes its calendar's color.
calendar-menu-color-calendar = Agba kalịnda
# A task's new due day, a week from today.
calendar-menu-in-a-week = N'otu izu
# Event colors, by the names Google Calendar gives them.
calendar-color-tomato = Tomato
calendar-color-flamingo = Flamingo
calendar-color-tangerine = Tanjirin
calendar-color-banana = Unere
calendar-color-sage = Sage
calendar-color-basil = Nchuanwụ
calendar-color-peacock = Pikọk
calendar-color-blueberry = Bluberi
calendar-color-lavender = Lavenda
calendar-color-grape = Mkpụrụ vaịn
calendar-color-graphite = Graịt
calendar-menu-only-this = Gosi naanị nke a
calendar-menu-rename = Gbanwee aha
calendar-menu-remove = Wepụ na ndepụta
calendar-menu-delete = Hichapụ
calendar-menu-new-calendar = Kalịnda ọhụrụ
calendar-menu-show-all = Gosi niile
calendar-menu-hide-all = Zoo niile
calendar-menu-account-settings = Ntọala akaụntụ
calendar-why-main = Kalịnda isi
calendar-why-last = Naanị otu nọ ebe a
calendar-why-owner = Naanị onye nwe ya
calendar-why-contacts = Site na Kọntaktị
calendar-why-unreached = Erughị ya
calendar-name-placeholder = Aha kalịnda
calendar-toast-added = Etinyere “{ $name }”
calendar-toast-renamed = Agbanwere aha kalịnda
calendar-toast-recolored = Agbanwere agba kalịnda
calendar-toast-deleted = Ehichapụrụ “{ $name }”
calendar-toast-removed = Ewepụrụ “{ $name }” na ndepụta gị
calendar-edit-failed = Agbanweghị kalịnda ahụ: { $reason }
calendar-delete-title = Hichapụ “{ $name }”?
calendar-delete-confirm = Hichapụ
calendar-deleting = Na-ehichapụ…
calendar-delete-heading = Ehichapụrụ:
calendar-delete-events = Kalịnda ahụ na ihe omume ya niile
calendar-delete-shared = Maka onye ọ bụla e kekọrịtara ya
calendar-delete-server = A na-ehichapụ ya na { $account } n'ọrụ ozi, ọ bụghị naanị na Katna.
calendar-delete-local = A na-ehichapụ ya na kọmputa a.
calendar-remove-title = Wepụ “{ $name }” na ndepụta gị?
calendar-remove-confirm = Wepụ
calendar-removing = Na-ewepụ…
calendar-remove-heading = Ihe na-agbanwe:
calendar-remove-events = Ị kwụsịrị ịhụ ihe omume ya, ebe a na ngwa gị ndị ọzọ
calendar-remove-server = Kalịnda ahụ na-anọ n'aka onye nwe ya, onye nwere ike ịkekọrịta ya gị ọzọ.
calendar-kind-event = Ihe omume
calendar-kind-task = Ọrụ
calendar-kind-focus = Oge nlekwasị anya
calendar-kind-out-of-office = N'èzí ọfịs
calendar-kind-working-location = Ebe ọrụ
calendar-task-added = Etinyela ọrụ
calendar-task-added-to = Etinyela ọrụ na { $list }
calendar-task-list-local = Na kọmputa a
calendar-working-home = Ụlọ
calendar-busy = Nwere ọrụ
calendar-free = Nwere oge
calendar-cancel = Kagbuo
calendar-ok = Ọ dị mma
calendar-read-only = Ị nweghị ike ịgbanwe ihe omume dị na kalenda a
calendar-none-editable = Enweghị kalenda ị nwere ike itinye ihe omume na ya
calendar-no-such-time = Oge ahụ adịghị n'ógbè oge gị
calendar-end-before-start = Ihe omume ahụ ejedebe tupu ọ malite
calendar-repeat-never = Anaghị emegharị
calendar-repeat-daily = Kwa ụbọchị
calendar-repeat-weekly = Kwa izu: { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Kwa ọnwa: nke mbụ { $weekday }
        [2] Kwa ọnwa: nke abụọ { $weekday }
        [3] Kwa ọnwa: nke atọ { $weekday }
        [4] Kwa ọnwa: nke anọ { $weekday }
       *[other] Kwa ọnwa: nke ikpeazụ { $weekday }
    }
calendar-repeat-yearly = Kwa afọ: { $day }
calendar-repeat-weekdays = Kwa ụbọchị ọrụ (Mọnde ruo Fraịde)
calendar-repeat-custom = Nke gị
calendar-reminder-none = Enweghị ọkwa
calendar-reminder-at-start = Mgbe ọ na-amalite
calendar-reminder-minutes =
    { $count ->
       *[other] Nkeji { $count } tupu oge ahụ
    }
calendar-reminder-hours =
    { $count ->
       *[other] Awa { $count } tupu oge ahụ
    }
calendar-reminder-days =
    { $count ->
       *[other] Ụbọchị { $count } tupu oge ahụ
    }
calendar-scope-edit-title = Dezie ihe omume na-emegharị
calendar-scope-delete-title = Hichapụ ihe omume na-emegharị
calendar-scope-this = Ihe omume a
calendar-scope-following = Ihe omume a na nke ndị na-esote
calendar-scope-all = Ihe omume niile
calendar-scope-respond-title = Azịza maka ihe omume na-emegharị
calendar-going = Ị ga-aga?
calendar-answer-yes = Ee
calendar-answer-no = Mba
calendar-answer-maybe = Ikekwe
calendar-answered-yes = Ị na-aga
calendar-answered-no = Ị naghị aga
calendar-answered-maybe = Ị nwere ike ịga

## The card at the top of a mail with an invitation.

calendar-invite = Oku
calendar-invite-cancelled = Akagburu ihe omume
calendar-invite-reply = { $name } zara
calendar-invite-reply-yes = { $name } kwetara
calendar-invite-reply-no = { $name } jụrụ
calendar-invite-reply-maybe = { $name } nwere ike ịga
calendar-invite-organizer = Onye nhazi: { $name }
calendar-invite-open = Mepe na Kalịnda
calendar-invite-not-yet = Ọ nọghị na kalịnda gị ka. Ị ga-azaghachi ozugbo ọ kwekọrọ.
calendar-invite-by-mail = Ọ nọghị na kalịnda gị: azịza gị ga-aga n'aka onye nhazi site na ozi.
calendar-mail-yes = Anabatara: { $title }
calendar-mail-yes-body = { $name } anabatala oku a.
calendar-mail-no = Jụrụ: { $title }
calendar-mail-no-body = { $name } jụla oku a.
calendar-mail-maybe = Anabatara nwa oge: { $title }
calendar-mail-maybe-body = { $name } anabatala oku a nwa oge.
calendar-invite-your-day = Ụbọchị gị
calendar-invite-clashes =
    { $count ->
       *[other] Ọ na-ekwekọghị na ihe omume { $count }
    }

## The day's agenda beside the mail.

agenda-show = Gosi atụmatụ ụbọchị
agenda-hide = Zoo atụmatụ
agenda-today = Taa, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = Ọ nweghị ihe e mere atụmatụ n'ụbọchị a.
