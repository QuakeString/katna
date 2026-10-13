# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = { $count ->
    [one] { $count } nytt mejl
   *[other] { $count } nya mejl
}
notify-and-more = och { $count } till
notify-no-subject = (inget ämne)
notify-unknown-sender = Okänd avsändare

## Reminders the user asked for (same buttons)

notify-snooze-back = Tillbaka från snooze
notify-no-reply = Inget svar än
notify-no-reply-to = Ingen har svarat på ”{ $subject }”.
notify-follow-up-sent = Uppföljning skickad
notify-follow-up-sent-to = Ingen hade svarat på ”{ $subject }”, så Katna följde upp.
notify-follow-up-waiting = Uppföljningen skickades inte
notify-follow-up-waiting-to = Den skulle ha skickats medan datorn var avstängd. ”{ $subject }” är tillbaka i Inkorgen.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } öppnade { $subject }
notify-tracking-clicked = { $who } klickade på en länk i { $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail kan uppdateras
notify-update-ready-body = Version { $version } har hämtats. Uppdatera installerar den och startar om Katna Mail.
notify-update = Uppdatera

## Something needs the user, shown once per problem

notify-signed-out = Logga in igen
notify-signed-out-body = { $provider } loggade ut Katna från { $address }. E-posten synkroniseras inte längre.
notify-sign-in = Logga in
notify-password-refused = Lösenordet nekades
notify-password-refused-body = E-postservern nekade lösenordet för { $address }. Det kan ha ändrats.
notify-new-password = Nytt lösenord
notify-not-sent = ”{ $subject }” skickades inte
notify-not-sent-no-subject = Ett meddelande skickades inte
notify-not-sent-body = Det ligger i Utkorgen, som visar varför.
notify-open-outbox = Öppna Utkorgen

## Reminders of calendar events

notify-event-now = Nu
notify-event-in-minutes = { $count ->
    [one] Om { $count } minut
   *[other] Om { $count } minuter
}
notify-event-in-hours = { $count ->
    [one] Om { $count } timme
   *[other] Om { $count } timmar
}
notify-event-in-days = { $count ->
    [1] I morgon
    [one] Om { $count } dag
   *[other] Om { $count } dagar
}
notify-event-all-day = Heldag
notify-event-join = Anslut
notify-event-snooze = Snooza 5 min
notify-task-done = Markera som klar

## The buttons of new-mail notifications and reminders

notify-open = Öppna
notify-peek = Kika
notify-reply = Svara
notify-reply-placeholder = Svara { $name }…
notify-send = Skicka
notify-reply-quote-header = Den { $date } skrev { $from }:
notify-reply-quote-header-no-date = { $from } skrev:
notify-reply-all = Svara alla
notify-mark-read = Markera som läst
notify-mark-all-read = Markera alla som lästa
notify-archive = Arkivera
notify-snooze-hour = Snooza 1 timme
notify-snooze-tomorrow = I morgon
notify-copy-code = Kopiera { $code }
notify-link-verify = Verifiera på { $domain }
notify-link-confirm = Bekräfta på { $domain }
notify-link-activate = Aktivera på { $domain }

## After Archive on a notification: a short note in the same place

notify-archived = Arkiverat
notify-archived-count = { $count ->
    [one] { $count } meddelande flyttades ut ur inkorgen
   *[other] { $count } meddelanden flyttades ut ur inkorgen
}
notify-undo = Ångra

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = Koden kopierades
notify-code-not-copied = Det gick inte att kopiera koden

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Svar skickat till { $name }
notify-open-in-katna = Öppna i Katna
