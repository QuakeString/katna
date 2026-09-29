# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } nowa wiadomość
    [few] { $count } nowe wiadomości
    [many] { $count } nowych wiadomości
   *[other] { $count } nowej wiadomości
}
notify-and-more = i { $count } więcej
notify-no-subject = (bez tematu)
notify-unknown-sender = Nieznany nadawca
notify-snooze-back = Odłożona poczta wróciła
notify-no-reply = Jeszcze bez odpowiedzi
notify-no-reply-to = Nikt nie odpowiedział na „{ $subject }”.
notify-tracking-opened = Otwarte przez { $who }: { $subject }
notify-tracking-clicked = Link kliknięty przez { $who }: { $subject }

notify-update-ready = Katna Mail można zaktualizować
notify-update-ready-body = Wersja { $version } jest pobrana. Aktualizacja instaluje ją i uruchamia ponownie Katna Mail.
notify-update = Aktualizuj
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

## Its buttons

notify-open = Otwórz
notify-reply-all = Odpowiedz wszystkim
notify-mark-read = Oznacz jako przeczytane
notify-mark-all-read = Oznacz wszystkie jako przeczytane
notify-archive = Archiwizuj
