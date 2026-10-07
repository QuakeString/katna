# Katna Mail, Filipino (Filipino).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

top-brand = Katna

## App rail (and the bottom bar on a phone)

rail-mail = Mail
rail-calendar = Kalendaryo
rail-contacts = Mga Contact
rail-tasks = Mga Gawain
rail-notes = Mga Tala
rail-files = Mga File

## Rail right-click menu

rail-menu-open = Buksan ang { $app }
rail-menu-settings = Mga setting ng { $app }
rail-menu-turn-off = I-off ang { $app }…

## Turning an app off (Settings > Apps)

app-off-title = I-off ang { $app }?
app-off-body = Ihihinto ng Katna ang pag-sync ng { $app } at aalisin ito sa:
app-off-keep = Magtabi ng kopya sa computer na ito
app-off-keep-detail = Agad-agad ang pag-on ulit nito
app-off-remove = Alisin ang kopya sa computer na ito
app-off-remove-detail = Walang nagbabago sa iyong mga account, at ida-download ito ulit kapag in-on mo muli. Mananatili ang nasa computer na ito lang, o ang hindi pa naipapadala.
app-off-cancel = Kanselahin
app-off-confirm = I-off
app-off-done = Na-off ang { $app }
app-off-note = Naka-off ang { $app }
app-off-turn-on = I-on
app-off-leaves-calendar-rail = Ang rail at Ctrl+2
app-off-leaves-calendar-agenda = Ang agenda sa tabi ng iyong mail
app-off-leaves-calendar-meeting = Mag-iskedyul ng pulong, at Buksan sa Kalendaryo sa mga imbitasyon
app-off-leaves-calendar-reminders = Mga paalala ng event
app-off-leaves-calendar-desktop = Mga event sa KRunner at sa orasan ng desktop
app-off-leaves-contacts-rail = Ang rail at Ctrl+3
app-off-leaves-contacts-card = Idagdag sa mga contact sa card ng nagpadala
app-off-leaves-contacts-birthdays = Mga kaarawan sa Kalendaryo
app-off-leaves-tasks-rail = Ang rail at Ctrl+4
app-off-leaves-tasks-mail = Idagdag sa Mga Gawain sa mail, at Shift+T
app-off-leaves-tasks-calendar = Mga gawain sa Kalendaryo
app-off-leaves-tasks-tray = Bagong gawain sa tray, at Meta+Alt+T
app-off-leaves-tasks-reminders = Mga paalala ng gawain
app-off-leaves-notes-rail = Ang rail at Ctrl+5
app-off-leaves-notes-mail = Magdagdag ng tala sa mail
app-off-leaves-notes-meetings = Mga tala ng pulong sa mga event
app-off-leaves-notes-tray = Bagong tala sa tray, at Meta+Alt+N
app-off-leaves-notes-reminders = Mga paalala ng tala
app-off-leaves-files-rail = Ang rail at Ctrl+7
app-off-leaves-files-compose = Mga File kapag nag-a-attach sa Compose

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Malapit na
app-calendar-promise = Ang iyong mga CalDAV na kalendaryo, mga imbitasyon sa pulong mula sa iyong mail at mga paalala, katabi ng iyong inbox.
app-tasks-promise = Mga listahan ng gagawin na naka-sync sa CalDAV, at mga gawaing ginawa mula sa mail.
app-notes-promise = Mabibilis na tala, at mga tala sa isang mail o pag-uusap para sa ibang pagkakataon.

## Contacts page

app-contacts-loading = Kinukuha ang mga tao mula sa iyong mail…
app-contacts-empty = Lalabas dito ang mga taong kasulatan mo.
app-contacts-count = { $count ->
    [one] { $count } tao mula sa iyong mail, nauuna ang pinakamadalas mong kasulatan
   *[other] { $count } tao mula sa iyong mail, nauuna ang pinakamadalas mong kasulatan
}
app-contacts-top = { $count ->
    [one] Ang nangungunang { $count } tao mula sa iyong mail, nauuna ang pinakamadalas mong kasulatan
   *[other] Ang nangungunang { $count } tao mula sa iyong mail, nauuna ang pinakamadalas mong kasulatan
}
app-contacts-messages = { $count ->
    [one] { $count } mensahe
   *[other] { $count } mensahe
}
app-contacts-last = huli noong { $date }
