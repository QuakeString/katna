# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > User feedback (crash reports)

feedback-intro-sending = Nieuwe crashrapporten worden verstuurd om te helpen oplossen wat er misging. Verder verlaat niets deze computer.
feedback-intro-local = Katna verstuurt nergens iets naartoe. Crashrapporten blijven op deze computer, zodat je ze kunt bekijken of bij een bugmelding kunt voegen.
feedback-crash-reports = Crashrapporten
feedback-crash-reports-detail = Aangemaakt als Katna Mail of de achtergrondservice crasht.
feedback-save = Crashrapporten op deze computer bewaren
feedback-save-detail = Je persoonlijke map, gebruikers- en computernamen en e-mailadressen worden weggelaten
feedback-saved = Bewaarde crashrapporten
feedback-saved-detail = { $count ->
    [one] Het nieuwste rapport wordt bewaard.
   *[other] De nieuwste { $count } worden bewaard.
}
feedback-help-improve = Help Katna te verbeteren
feedback-help-improve-detail = Uit tenzij je het aanzet, en je kunt het hier altijd weer uitzetten.
feedback-send = Crashrapporten versturen
feedback-send-detail = Het bewaarde rapport, precies zoals je het hier kunt bekijken, gaat naar de crashtracker van Katna (Sentry, in de EU). Geen IP-adres, berichten of e-mailadressen
feedback-none-saved = Er zijn geen crashrapporten bewaard.
feedback-delete-all = Alles verwijderen
feedback-app-daemon = Achtergrondservice
feedback-report-sent = { $date } · Verstuurd
feedback-view = Bekijken
feedback-view-tooltip = Het rapport openen
feedback-copy-tooltip = Kopiëren om in een bugmelding te plakken
feedback-copied = Crashrapport gekopieerd.
feedback-deleted-all = Crashrapporten verwijderd.
feedback-read-failed = Kan het crashrapport niet lezen: { $error }
feedback-delete-failed = Kan het crashrapport niet verwijderen: { $error }
feedback-delete-all-failed = Kan de crashrapporten niet verwijderen: { $error }

## Settings > User feedback (usage statistics)

feedback-usage = Anonieme gebruiksstatistieken versturen
feedback-usage-detail = Eén keer per week: welke functies je hebt gebruikt, ja of nee. Nooit aantallen, adressen, namen of zoekwoorden
feedback-intro-sending-usage = Crashrapporten en wekelijkse gebruiksstatistieken worden verstuurd. Verder verlaat niets deze computer.
feedback-intro-usage-only = Wekelijkse gebruiksstatistieken worden verstuurd. Crashrapporten blijven op deze computer.
feedback-counted = Wat er wordt geteld
feedback-counted-detail = Elk is ja of nee voor die week.
feedback-counted-also = Verder: de versie van Katna, de Linux-familie, de desktop, de schermschaal en het aantal accounts (1, 2–3, 4+)
feedback-see-report = Rapport van deze week bekijken
feedback-hide-report = Rapport van deze week verbergen
feedback-report-goes = Verstuurd na afloop van de week, op { $date }, als gebruiksstatistieken dan nog aan staan.
feedback-install-id = Installatie-ID { $id }
feedback-install-id-tooltip = Willekeurig, zodat één computer niet twee keer in een week wordt geteld. Verandert elke 90 dagen en wordt nooit met crashrapporten of feedback meegestuurd
feedback-install-id-reset = Opnieuw instellen
feedback-install-id-new = Nieuwe installatie-ID aangemaakt.
feedback-report-copied = Rapport gekopieerd.
feedback-send-feedback = Feedback
feedback-send-feedback-detail = Een probleem, een idee, wat dan ook.
feedback-send-feedback-button = Feedback versturen…
usage-feature-search-options = Zoekopties
usage-feature-pins = Vastgezette e-mail
usage-feature-labels = Labels
usage-feature-scheduled-send = Verzending plannen
usage-feature-snooze = Snoozen en herinneringen
usage-feature-encrypted = Versleutelde e-mail
usage-feature-viewers = Ingebouwde viewers
usage-feature-calendar = Agenda
usage-feature-contacts = Contacten
usage-feature-tasks-notes = Taken en Notities
usage-feature-phone-layout = Indeling voor telefoonbreedte
usage-feature-own-frame = Eigen vensterrand van Katna

## Help > Send feedback

send-feedback-title = Feedback versturen
send-feedback-about = Over
send-feedback-problem = Probleem
send-feedback-idea = Idee
send-feedback-other = Iets anders
send-feedback-message = Je bericht
send-feedback-message-placeholder = Wat is er gebeurd, of wat zou je willen?
send-feedback-reply = E-mailadres voor een antwoord (optioneel)
send-feedback-reply-placeholder = jij@example.org
send-feedback-system = De versie van Katna en je systeem meesturen
send-feedback-what-is-sent = Wat er wordt verstuurd
send-feedback-show = Tonen
send-feedback-hide = Verbergen
send-feedback-where = Verstuurd naar de feedbackinbox van Katna bij Sentry (EU). Geen IP-adres, accounts, berichten of installatie-ID.
send-feedback-cancel = Annuleren
send-feedback-send = Versturen
send-feedback-sending = Versturen…
send-feedback-sent = Feedback verstuurd. Bedankt
send-feedback-failed = Kan feedback niet versturen: { $error }
