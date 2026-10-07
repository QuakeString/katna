# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = Odłóż do…
snooze-later-today = Później dzisiaj
snooze-tomorrow = Jutro
snooze-this-weekend = W ten weekend
snooze-next-week = W przyszłym tygodniu
snooze-pick = Wybierz datę i godzinę
snooze-back = Wróć do godzin
snooze-type-placeholder = Wpisz godzinę
snooze-type-hint = Na przykład „wt 15:00”, „jutro” lub „za 2 godziny”
snooze-type-hint-unclear = Katna nie rozpoznaje tego jako godziny
snooze-type-unclear = „{ $text }” to nie jest godzina, którą Katna rozumie

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = Odłóż
remind-tab = Przypomnij mi
snooze-says = Ukrywa ją do tego czasu
remind-says = Zostawia ją na miejscu i powiadamia Cię
remind-before-due = Przed terminem
remind-note = Notatka (opcjonalnie)
remind-note-placeholder = Temat, jeśli zostawisz puste
toast-remind-set = Przypomnienie ustawione na { $date }
remind-chat-line = Przypomnienie { $date } · { $title }
remind-done = Gotowe
toast-remind-done = Przypomnienie wykonane
snooze-chat-line = Odłożone do { $date }
snooze-chat-change = Zmień

## The date and time picker

snooze-cancel = Anuluj
snooze-save = Zapisz
snooze-in-the-past = Wybierz godzinę późniejszą niż teraz.

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = Ponów, jeśli nikt nie odpowie…
follow-up-title = Ponów, jeśli nikt nie odpowie
follow-up-off = Wyłączone
follow-up-days = { $days ->
    [one] { $days } dzień
    [few] { $days } dni
    [many] { $days } dni
   *[other] { $days } dnia
}
follow-up-weeks = { $weeks ->
    [one] { $weeks } tydzień
    [few] { $weeks } tygodnie
    [many] { $weeks } tygodni
   *[other] { $weeks } tygodnia
}
follow-up-pick = Wybierz…
follow-up-pick-title = Ponów, jeśli nikt nie odpowie do
follow-up-remind = Przypomnij mi
follow-up-remind-note = Wątek wraca na górę Twoich Odebranych
follow-up-send = Wyślij za mnie monit
follow-up-send-note = Do tych samych osób, w tym samym wątku
follow-up-send-encrypted = Niedostępne dla poczty szyfrowanej
follow-up-text-placeholder = Co napisać
follow-up-text-named = Cześć { $name }, chcę tylko upewnić się, że moja wiadomość poniżej dotarła.
follow-up-text = Dzień dobry, chcę tylko upewnić się, że moja wiadomość poniżej dotarła.
follow-up-template = Użyj szablonu
follow-up-signature = Twój podpis zostanie dodany
follow-up-again = Jeśli nadal nikt nie odpowie, ponów znowu za
follow-up-note = Kończy się, gdy tylko ktokolwiek w wątku odpowie. Automatyczne odpowiedzi się nie liczą.
follow-up-note-send = Kończy się, gdy tylko ktokolwiek w wątku odpowie. Wychodzi w dni robocze od { $start } do { $end } i nigdy z ponad jednodniowym opóźnieniem.
follow-up-cancel = Anuluj
follow-up-done = Gotowe
follow-up-chip-send = Monit za { $time }
follow-up-chip-remind = Przypomnienie za { $time }
follow-up-chip-send-on = Monit { $date }
follow-up-chip-remind-on = Przypomnienie { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = Jeszcze bez odpowiedzi
follow-up-card-title-waiting = Twój monit czeka
follow-up-card-send = Katna wyśle Twój monit { $date }. Zatrzyma się, gdy ktoś odpowie.
follow-up-card-send-twice = Katna wyśle Twój monit { $date }, a później jeszcze raz. Zatrzyma się, gdy ktoś odpowie.
follow-up-card-remind = Jeśli nikt nie odpowie, ten wątek wróci do Twoich Odebranych { $date }.
follow-up-card-waiting = Termin minął, gdy Twój komputer był wyłączony, więc nie został wysłany z opóźnieniem. Wyślij go teraz, wybierz nowy termin albo go zatrzymaj.
follow-up-card-edit = Edytuj
follow-up-card-edit-title = Ponów w dniu
follow-up-card-send-now = Wyślij teraz
follow-up-card-stop = Zatrzymaj
follow-up-chat-send = Monit · { $date }, jeśli nikt nie odpowie
follow-up-chat-step = Monit { $step } z { $steps } · { $date }, jeśli nikt nie odpowie
follow-up-chat-waiting = Monit czeka · termin minął, gdy Twój komputer był wyłączony
follow-up-chat-remind = Z powrotem w Odebranych { $date }, jeśli brak odpowiedzi
toast-follow-up-sent = Monit wysłany
toast-follow-up-stopped = Monit zatrzymany
toast-follow-up-moved = Monit przeniesiony na { $date }

nudge-row = Wysłano { $days ->
    [one] 1 dzień temu
    [few] { $days } dni temu
    [many] { $days } dni temu
   *[other] { $days } dnia temu
}. Ponowić?
nudge-row-tip = Napisz monit do wszystkich w wątku
nudge-follow-up = Ponów
nudge-dismiss = Odrzuć
nudge-card-title = Jeszcze bez odpowiedzi
nudge-card-text = Zadałeś pytanie { $days ->
    [one] 1 dzień temu
    [few] { $days } dni temu
    [many] { $days } dni temu
   *[other] { $days } dnia temu
} i nikt nie odpowiedział.
nudge-chat-line = Wysłano { $days ->
    [one] 1 dzień temu
    [few] { $days } dni temu
    [many] { $days } dni temu
   *[other] { $days } dnia temu
}, jeszcze bez odpowiedzi
toast-nudge-dismissed = Ponaglenie odrzucone
