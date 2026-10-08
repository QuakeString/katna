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
notify-follow-up-sent = Opvolg gestuur
notify-follow-up-sent-to = Niemand het op “{ $subject }” geantwoord nie, so Katna het opgevolg.
notify-follow-up-waiting = Opvolg nie gestuur nie
notify-follow-up-waiting-to = Dit was verskuldig terwyl hierdie rekenaar af was. “{ $subject }” is terug in jou inkassie.
notify-tracking-opened = { $who } het { $subject } oopgemaak
notify-tracking-clicked = { $who } het op 'n skakel in { $subject } geklik

## Its buttons

notify-update-ready = Katna Mail kan opgedateer word
notify-update-ready-body = Weergawe { $version } is afgelaai. Opdateer installeer dit en herbegin Katna Mail.
notify-update = Opdateer

## Something needs the user, shown once per problem

notify-signed-out = Meld weer aan
notify-signed-out-body = { $provider } het Katna by { $address } afgemeld. E-pos sinkroniseer nie meer nie.
notify-sign-in = Meld aan
notify-password-refused = Wagwoord geweier
notify-password-refused-body = Die e-posbediener het die wagwoord vir { $address } geweier. Dit het dalk verander.
notify-new-password = Nuwe wagwoord
notify-not-sent = “{ $subject }” is nie gestuur nie
notify-not-sent-no-subject = 'n Boodskap is nie gestuur nie
notify-not-sent-body = Dit is in die Uitkassie, wat sê hoekom.
notify-open-outbox = Maak Uitkassie oop
notify-event-now = Nou
notify-event-in-minutes = { $count ->
    [one] Oor { $count } minuut
   *[other] Oor { $count } minute
}
notify-event-in-hours = { $count ->
   *[other] Oor { $count } uur
}
notify-event-in-days = { $count ->
    [1] Môre
    [one] Oor { $count } dag
   *[other] Oor { $count } dae
}
notify-event-all-day = Heeldag
notify-event-join = Sluit aan
notify-event-snooze = Sluimer 5 min
notify-task-done = Merk as klaar
notify-open = Maak oop
notify-peek = Loer
notify-reply = Antwoord
notify-reply-placeholder = Antwoord { $name }…
notify-send = Stuur
notify-reply-all = Antwoord almal
notify-mark-read = Merk as gelees
notify-mark-all-read = Merk almal as gelees
notify-archive = Argiveer
notify-snooze-hour = Sluimer 1 uur
notify-snooze-tomorrow = Môre
notify-copy-code = Kopieer { $code }
notify-link-verify = Verifieer op { $domain }
notify-link-confirm = Bevestig op { $domain }
notify-link-activate = Aktiveer op { $domain }

## After Archive on a notification: a short note in the same place

notify-archived = Geargiveer
notify-archived-count = { $count ->
    [one] { $count } boodskap uit die inkassie geskuif
   *[other] { $count } boodskappe uit die inkassie geskuif
}
notify-undo = Ontdoen

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = Kode gekopieer
notify-code-not-copied = Kon nie die kode kopieer nie

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Antwoord aan { $name } gestuur
notify-open-in-katna = Maak in Katna oop
