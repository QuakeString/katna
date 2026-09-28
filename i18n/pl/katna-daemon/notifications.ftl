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

## Its buttons

notify-open = Otwórz
notify-reply-all = Odpowiedz wszystkim
notify-mark-read = Oznacz jako przeczytane
notify-mark-all-read = Oznacz wszystkie jako przeczytane
notify-archive = Archiwizuj
