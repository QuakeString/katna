# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = { $count ->
    [one] { $count } nowa wiadomość
    [few] { $count } nowe wiadomości
    [many] { $count } nowych wiadomości
   *[other] { $count } nowej wiadomości
}
notify-and-more = i { $count } więcej
notify-no-subject = (bez tematu)
notify-unknown-sender = Nieznany nadawca

## Reminders the user asked for (same buttons)

notify-snooze-back = Odłożona poczta wróciła
notify-no-reply = Jeszcze bez odpowiedzi
notify-no-reply-to = Nikt nie odpowiedział na „{ $subject }”.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = Otwarte przez { $who }: { $subject }
notify-tracking-clicked = Link kliknięty przez { $who }: { $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail można zaktualizować
notify-update-ready-body = Wersja { $version } jest pobrana. Aktualizacja instaluje ją i uruchamia ponownie Katna Mail.
notify-update = Aktualizuj

## Reminders of calendar events

notify-event-now = Teraz
notify-event-in-minutes = { $count ->
    [one] Za { $count } minutę
    [few] Za { $count } minuty
    [many] Za { $count } minut
   *[other] Za { $count } minuty
}
notify-event-in-hours = { $count ->
    [one] Za { $count } godzinę
    [few] Za { $count } godziny
    [many] Za { $count } godzin
   *[other] Za { $count } godziny
}
notify-event-in-days = { $count ->
    [1] Jutro
    [one] Za { $count } dzień
    [few] Za { $count } dni
    [many] Za { $count } dni
   *[other] Za { $count } dnia
}
notify-event-all-day = Cały dzień
notify-event-join = Dołącz
notify-event-snooze = Drzemka 5 min
notify-task-done = Oznacz jako wykonane

## The buttons of new-mail notifications and reminders

notify-open = Otwórz
notify-peek = Podgląd
notify-reply = Odpowiedz
notify-reply-placeholder = Odpowiedz: { $name }…
notify-send = Wyślij
notify-reply-all = Odpowiedz wszystkim
notify-mark-read = Oznacz jako przeczytane
notify-mark-all-read = Oznacz wszystkie jako przeczytane
notify-archive = Archiwizuj

## After Archive on a notification: a short note in the same place

notify-archived = Zarchiwizowano
notify-archived-count = { $count ->
    [one] { $count } wiadomość przeniesiona z Odebranych
    [few] { $count } wiadomości przeniesione z Odebranych
    [many] { $count } wiadomości przeniesionych z Odebranych
   *[other] { $count } wiadomości przeniesionej z Odebranych
}
notify-undo = Cofnij

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Wysłano odpowiedź do: { $name }
notify-open-in-katna = Otwórz w Katna
