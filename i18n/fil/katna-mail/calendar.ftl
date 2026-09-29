# Katna Mail, Filipino (Filipino): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Ngayon
calendar-today-tip = Pumunta sa ngayon
calendar-view-day = Araw
calendar-view-week = Linggo
calendar-view-month = Buwan
calendar-view-schedule = Iskedyul
calendar-previous-day = Nakaraang araw
calendar-next-day = Susunod na araw
calendar-previous-week = Nakaraang linggo
calendar-next-week = Susunod na linggo
calendar-previous-month = Nakaraang buwan
calendar-next-month = Susunod na buwan
calendar-previous-period = Mas maaga
calendar-next-period = Mas huli
calendar-title-months = { $first } – { $last }
calendar-loading = Naglo-load…
calendar-read-failed = Hindi nabasa ang kalendaryo: { $error }
calendar-local = Sa computer na ito
calendar-account-gone = Inalis na account
calendar-birthdays = Mga kaarawan
calendar-birthday-of = Kaarawan ni { $name }
calendar-empty-title = Wala pang kalendaryo
calendar-empty-text = Ipinapakita rito ng Katna ang mga kalendaryo ng iyong mga Google at Microsoft account kapag na-sync na ang mga ito, pati ang sa iba pang server na may CalDAV.
calendar-schedule-empty = Walang nakaplano sa susunod na dalawang buwan.
calendar-no-title = (Walang pamagat)
calendar-all-day = Buong araw
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } pa
calendar-repeats = Umuulit
calendar-join = Sumali
calendar-email-guests = Mag-mail sa mga bisita
calendar-running-late = Mahuhuli ako
calendar-late-subject = Mahuhuli: { $title }
calendar-late-body = Pasensya na, mahuhuli ako nang ilang minuto sa { $title }. Darating din ako agad.
calendar-guests =
    { $count ->
        [one] { $count } bisita
       *[other] { $count } bisita
    }
calendar-guest-answers = { $yes } oo, { $maybe } baka, { $no } hindi, { $waiting } naghihintay
calendar-organizer = Organizer
calendar-optional = Opsyonal
calendar-open-web = Buksan sa browser
calendar-open-contact = Buksan ang contact
calendar-close = Isara

## Adding, changing and deleting events.

calendar-add-title = Magdagdag ng pamagat
calendar-add-location = Magdagdag ng lokasyon
calendar-add-notes = Magdagdag ng paglalarawan
calendar-add-guests = Magdagdag ng mga bisita
calendar-remove-guest = Alisin
calendar-add-meet = Magdagdag ng Google Meet video conferencing
calendar-add-teams = Magdagdag ng Teams meeting
calendar-has-call = Naidagdag ang video call
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Buong araw
calendar-more-options = Higit pang opsyon
calendar-save = I-save
calendar-saved = Na-save ang event
calendar-deleted = Na-delete ang event
calendar-discard = Itapon ang mga pagbabago
calendar-edit = I-edit ang event
calendar-delete = I-delete ang event
calendar-event-details = Mga detalye ng event
calendar-kind-event = Event
calendar-kind-focus = Focus time
calendar-kind-out-of-office = Wala sa opisina
calendar-kind-working-location = Lokasyon ng trabaho
calendar-working-home = Bahay
calendar-busy = Busy
calendar-free = Free
calendar-cancel = Kanselahin
calendar-ok = OK
calendar-read-only = Hindi mo mababago ang mga event sa kalendaryong ito
calendar-none-editable = Wala pang kalendaryong mapagdaragdagan mo ng event
calendar-no-such-time = Walang ganoong oras sa iyong time zone
calendar-end-before-start = Natatapos ang event bago ito magsimula
calendar-repeat-never = Hindi umuulit
calendar-repeat-daily = Araw-araw
calendar-repeat-weekly = Lingguhan tuwing { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Buwanan tuwing unang { $weekday }
        [2] Buwanan tuwing ikalawang { $weekday }
        [3] Buwanan tuwing ikatlong { $weekday }
        [4] Buwanan tuwing ikaapat na { $weekday }
       *[other] Buwanan tuwing huling { $weekday }
    }
calendar-repeat-yearly = Taunan tuwing { $day }
calendar-repeat-weekdays = Bawat araw ng trabaho (Lunes hanggang Biyernes)
calendar-repeat-custom = Custom
calendar-reminder-none = Walang notification
calendar-reminder-at-start = Sa simula
calendar-reminder-minutes =
    { $count ->
        [one] { $count } minuto bago
       *[other] { $count } minuto bago
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } oras bago
       *[other] { $count } oras bago
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } araw bago
       *[other] { $count } araw bago
    }
calendar-scope-edit-title = I-edit ang umuulit na event
calendar-scope-delete-title = I-delete ang umuulit na event
calendar-scope-this = Ang event na ito
calendar-scope-following = Ito at ang mga susunod na event
calendar-scope-all = Lahat ng event
calendar-scope-respond-title = Sagot para sa umuulit na event
calendar-going = Pupunta ka ba?
calendar-answer-yes = Oo
calendar-answer-no = Hindi
calendar-answer-maybe = Baka
calendar-answered-yes = Pupunta ka
calendar-answered-no = Hindi ka pupunta
calendar-answered-maybe = Baka pumunta ka

## The card at the top of a mail with an invitation.

calendar-invite = Imbitasyon
calendar-invite-cancelled = Nakansela ang event
calendar-invite-reply = Sumagot si { $name }
calendar-invite-reply-yes = Tinanggap ni { $name } ang imbitasyon
calendar-invite-reply-no = Tinanggihan ni { $name } ang imbitasyon
calendar-invite-reply-maybe = Baka pumunta si { $name }
calendar-invite-organizer = Inorganisa ni { $name }
calendar-invite-open = Buksan sa Kalendaryo
calendar-invite-not-yet = Wala pa sa kalendaryo mo. Makakasagot ka kapag na-sync na ito.
calendar-invite-by-mail = Wala sa kalendaryo mo: ipapadala ang sagot mo sa organizer sa pamamagitan ng email.
calendar-mail-yes = Tinanggap: { $title }
calendar-mail-yes-body = Tinanggap ni { $name } ang imbitasyong ito.
calendar-mail-no = Tinanggihan: { $title }
calendar-mail-no-body = Tinanggihan ni { $name } ang imbitasyong ito.
calendar-mail-maybe = Pansamantalang tinanggap: { $title }
calendar-mail-maybe-body = Pansamantalang tinanggap ni { $name } ang imbitasyong ito.
calendar-invite-your-day = Ang araw mo
calendar-invite-clashes =
    { $count ->
        [one] Nagsasalungatan sa { $count } event
       *[other] Nagsasalungatan sa { $count } event
    }

## The day's agenda beside the mail.

agenda-show = Ipakita ang agenda ng araw
agenda-hide = Itago ang agenda
agenda-today = Ngayon, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = Walang nakaplano sa araw na ito.
