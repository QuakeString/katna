# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } nytt mejl
   *[other] { $count } nya mejl
}
notify-and-more = och { $count } till
notify-no-subject = (inget ämne)
notify-unknown-sender = Okänd avsändare
notify-snooze-back = Tillbaka från snooze
notify-no-reply = Inget svar än
notify-no-reply-to = Ingen har svarat på ”{ $subject }”.
notify-tracking-opened = { $who } öppnade { $subject }
notify-tracking-clicked = { $who } klickade på en länk i { $subject }

notify-update-ready = Katna Mail kan uppdateras
notify-update-ready-body = Version { $version } har hämtats. Uppdatera installerar den och startar om Katna Mail.
notify-update = Uppdatera

## Its buttons

notify-open = Öppna
notify-reply-all = Svara alla
notify-mark-read = Markera som läst
notify-mark-all-read = Markera alla som lästa
notify-archive = Arkivera
