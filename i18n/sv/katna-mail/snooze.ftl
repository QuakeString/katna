# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = Snooza till…
snooze-later-today = Senare i dag
snooze-tomorrow = I morgon
snooze-this-weekend = I helgen
snooze-next-week = Nästa vecka
snooze-pick = Välj datum och tid
snooze-back = Tillbaka till tiderna
snooze-type-placeholder = Skriv en tid
snooze-type-hint = Till exempel ”tis 15”, ”i morgon” eller ”om 2 timmar”
snooze-type-hint-unclear = Katna kan inte läsa det som en tid
snooze-type-unclear = ”{ $text }” är inte en tid som Katna känner till

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = Snooza
remind-tab = Påminn mig
snooze-says = Döljer den till dess
remind-says = Låter den ligga kvar och aviserar dig
remind-before-due = Innan den förfaller
remind-note = Anteckning (valfri)
remind-note-placeholder = Ämnet, om den lämnas tom
toast-remind-set = Påminnelse inställd för { $date }
remind-chat-line = Påminnelse { $date } · { $title }
remind-done = Klar
toast-remind-done = Påminnelsen är klar
snooze-chat-line = Snoozad till { $date }
snooze-chat-change = Ändra

## The date and time picker

snooze-cancel = Avbryt
snooze-save = Spara
snooze-in-the-past = Välj en tid senare än nu.

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = Följ upp om ingen svarar…
follow-up-title = Följ upp om ingen svarar
follow-up-off = Av
follow-up-days = { $days ->
    [one] { $days } dag
   *[other] { $days } dagar
}
follow-up-weeks = { $weeks ->
    [one] { $weeks } vecka
   *[other] { $weeks } veckor
}
follow-up-pick = Välj…
follow-up-pick-title = Följ upp om ingen svarat senast
follow-up-remind = Påminn mig
follow-up-remind-note = Konversationen hamnar högst upp i din Inkorg igen
follow-up-send = Skicka en uppföljning åt mig
follow-up-send-note = Till samma personer, i samma konversation
follow-up-send-encrypted = Inte för krypterad e-post
follow-up-text-placeholder = Vad du vill skriva
follow-up-text-named = Hej { $name }, jag ville bara höra om du har sett mitt meddelande nedan.
follow-up-text = Hej, jag ville bara höra om du har sett mitt meddelande nedan.
follow-up-template = Använd en mall
follow-up-signature = Din signatur läggs till
follow-up-again = Om ingen svarar ändå, följ upp igen efter
follow-up-note = Stoppas så fort någon i konversationen svarar. Automatiska svar räknas inte.
follow-up-note-send = Stoppas så fort någon i konversationen svarar. Skickas vardagar från { $start } till { $end }, och aldrig mer än en dag för sent.
follow-up-cancel = Avbryt
follow-up-done = Klar
follow-up-chip-send = Uppföljning om { $time }
follow-up-chip-remind = Påminnelse om { $time }
follow-up-chip-send-on = Uppföljning { $date }
follow-up-chip-remind-on = Påminnelse { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = Inget svar än
follow-up-card-title-waiting = Din uppföljning väntar
follow-up-card-send = Katna skickar din uppföljning { $date }. Den stoppas när någon svarar.
follow-up-card-send-twice = Katna skickar din uppföljning { $date }, och sedan en gång till senare. Den stoppas när någon svarar.
follow-up-card-remind = Om ingen svarar hamnar den här konversationen i din Inkorg igen { $date }.
follow-up-card-waiting = Den förföll medan datorn var avstängd, så den skickades inte för sent. Skicka den nu, välj en ny tid eller stoppa den.
follow-up-card-edit = Redigera
follow-up-card-edit-title = Följ upp den
follow-up-card-send-now = Skicka nu
follow-up-card-stop = Stoppa
follow-up-chat-send = Uppföljning · { $date } om ingen svarar
follow-up-chat-step = Uppföljning { $step } av { $steps } · { $date } om ingen svarar
follow-up-chat-waiting = Uppföljning väntar · den förföll medan datorn var avstängd
follow-up-chat-remind = Tillbaka i Inkorgen { $date } om ingen svarar
toast-follow-up-sent = Uppföljningen har skickats
toast-follow-up-stopped = Uppföljningen har stoppats
toast-follow-up-moved = Uppföljningen har flyttats till { $date }

nudge-row = Skickat { $days ->
    [one] för 1 dag sedan
   *[other] för { $days } dagar sedan
}. Följa upp?
nudge-row-tip = Skriv en uppföljning till alla i den
nudge-follow-up = Följ upp
nudge-dismiss = Avfärda
nudge-card-title = Inget svar än
nudge-card-text = Du frågade något { $days ->
    [one] för 1 dag sedan
   *[other] för { $days } dagar sedan
} och ingen har svarat.
nudge-chat-line = Skickat { $days ->
    [one] för 1 dag sedan
   *[other] för { $days } dagar sedan
}, inget svar än
toast-nudge-dismissed = Knuffen har avfärdats
