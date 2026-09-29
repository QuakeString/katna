# Katna Mail, Swahili (Kiswahili): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Unda
tasks-all = Majukumu yote
tasks-today = Leo
tasks-starred = Yenye nyota
tasks-new-list = Unda orodha mpya
tasks-on-this-computer = Kwenye kompyuta hii
tasks-my-tasks = Majukumu Yangu
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Ingia tena ili kuonyesha majukumu
tasks-account-signed-in = Umeingia tena kwenye { $address }. Inapata majukumu yako…
tasks-account-sign-in-refused = { $provider } haikuruhusu Katna kuingia. Jaribu tena, na uruhusu ufikiaji wa majukumu yako.
tasks-account-refused = Seva haikukubali nenosiri. Yahoo, iCloud, Zoho na nyinginezo zinahitaji nenosiri la programu.
tasks-account-change-password = Badilisha nenosiri
tasks-account-change-password-tooltip = Fungua Mipangilio > Akaunti
tasks-account-not-enabled = Ufikiaji wa majukumu kwa Katna bado haujawashwa.
tasks-account-failed = Orodha za majukumu hazikuweza kusomwa.
# $reason is the server's own words, in English.
tasks-account-error = Orodha za majukumu hazikuweza kusomwa: { $reason }
tasks-account-none = Hakuna orodha za majukumu zilizopatikana
tasks-account-looking = Inatafuta orodha za majukumu…
tasks-account-try-again = Jaribu tena
tasks-account-try-again-tooltip = Kagua majukumu ya akaunti hii tena sasa
tasks-account-fixing = Inashughulikia…
tasks-list-name-placeholder = Jina la orodha

## Lists and tasks

tasks-loading = Inasoma majukumu yako…
tasks-no-lists = Orodha za majukumu yako zitaonekana hapa.
tasks-search = Tafuta majukumu
tasks-search-none = Hakuna majukumu yanayolingana na utafutaji wako.
tasks-add = Ongeza jukumu
tasks-title-placeholder = Kichwa
tasks-add-step = Ongeza jukumu dogo
tasks-empty = Bado hakuna majukumu. Ongeza moja hapo juu.
tasks-starred-empty = Weka nyota kwenye jukumu ili kuliona hapa.
tasks-today-empty = Hakuna kinachostahili leo.
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Zilizochelewa
tasks-completed = { $count ->
    [one] Yaliyokamilika ({ $count })
   *[other] Yaliyokamilika ({ $count })
}
tasks-list-options = Chaguo za orodha
tasks-rename-list = Badilisha jina la orodha
tasks-delete-list = Futa orodha
tasks-mark-done = Weka alama kuwa limekamilika
tasks-mark-open = Weka alama kuwa halijakamilika
tasks-star = Weka nyota
tasks-unstar = Ondoa nyota
tasks-edit-title = Hariri kichwa
tasks-details = Maelezo
tasks-delete = Futa
tasks-move-to = Hamishia { $list }
tasks-from-mail = Barua
tasks-open-mail = Fungua barua
tasks-from-note = Dokezo
tasks-open-note = Fungua dokezo
tasks-note-gone = Dokezo hilo halipo hapa tena.
tasks-no-subject = (hakuna mada)

## The details dialog

tasks-notes-placeholder = Ongeza maelezo
tasks-date = Tarehe
tasks-no-date = Hakuna tarehe
tasks-time-placeholder = Ongeza saa
tasks-repeat = Rudia
tasks-repeat-never = Haijirudii
tasks-repeat-daily = Kila siku
tasks-repeat-weekly = Kila wiki
tasks-repeat-monthly = Kila mwezi
tasks-repeat-yearly = Kila mwaka
tasks-repeat-other = Maalum
tasks-remind = Nikumbushe
tasks-remind-off = Usinikumbushe
tasks-remind-on-time = Wakati huo
tasks-remind-morning = Siku hiyo, { $time }
tasks-remind-hour-before = Saa moja kabla
tasks-remind-day-before = Siku moja kabla
tasks-cancel = Ghairi
tasks-save = Hifadhi
tasks-not-a-time = “{ $text }” si saa, kwa mfano { $example }.

## Due days

tasks-due-today = Leo
tasks-due-tomorrow = Kesho
tasks-due-yesterday = Jana
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Jukumu limekamilika
tasks-toast-next = Imekamilika. Inayofuata ni { $date }
tasks-toast-deleted = Jukumu limefutwa
tasks-toast-added = { $count ->
    [one] Imeongezwa kwenye Majukumu
   *[other] Majukumu { $count } yameongezwa
}
tasks-mail-gone = Barua hiyo haipo hapa tena.
tasks-toast-list-deleted = Orodha imefutwa
tasks-toast-moved = Limehamishiwa { $list }
tasks-toast-rescheduled = Kazi imepangwa upya
