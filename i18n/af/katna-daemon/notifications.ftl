# Katna Mail, Afrikaans (Afrikaans).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } nuwe e-pos
   *[other] { $count } nuwe e-posse
}
notify-and-more = en nog { $count }
notify-no-subject = (geen onderwerp)
notify-unknown-sender = Onbekende sender
notify-snooze-back = Terug uit sluimer
notify-no-reply = Nog geen antwoord nie
notify-no-reply-to = Niemand het nog op “{ $subject }” geantwoord nie.
notify-tracking-opened = { $who } het { $subject } oopgemaak
notify-tracking-clicked = { $who } het op 'n skakel in { $subject } geklik

## Its buttons

notify-open = Maak oop
notify-reply-all = Antwoord almal
notify-mark-read = Merk as gelees
notify-mark-all-read = Merk almal as gelees
notify-archive = Argiveer
