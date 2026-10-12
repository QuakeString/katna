# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > User feedback (crash reports)

feedback-intro-sending = Nowe raporty o awariach są wysyłane, aby pomóc naprawić błędy. Nic więcej nie opuszcza tego komputera.
feedback-intro-local = Katna niczego nigdzie nie wysyła. Raporty o awariach zostają na tym komputerze, abyś mógł je przejrzeć lub dołączyć do zgłoszenia błędu.
feedback-crash-reports = Raporty o awariach
feedback-crash-reports-detail = Tworzone, gdy Katna Mail lub jej usługa w tle ulegnie awarii.
feedback-save = Zapisuj raporty o awariach na tym komputerze
feedback-save-detail = Folder domowy, nazwy użytkownika i komputera oraz adresy e-mail są pomijane
feedback-saved = Zapisane raporty o awariach
feedback-saved-detail = { $count ->
    [one] Przechowywany jest najnowszy raport.
    [few] Przechowywane są { $count } najnowsze raporty.
    [many] Przechowywanych jest { $count } najnowszych raportów.
   *[other] Przechowywane są najnowsze raporty ({ $count }).
}
feedback-help-improve = Pomóż ulepszyć Katna
feedback-help-improve-detail = Wyłączone, dopóki tego nie włączysz, a w każdej chwili możesz to tutaj wyłączyć.
feedback-send = Wysyłaj raporty o awariach
feedback-send-detail = Zapisany raport, dokładnie taki, jaki możesz tu wyświetlić, trafia do systemu śledzenia awarii Katna (Sentry, w UE). Bez adresu IP, wiadomości i adresów e-mail
feedback-none-saved = Nie ma zapisanych raportów o awariach.
feedback-delete-all = Usuń wszystkie
feedback-app-daemon = Usługa w tle
feedback-report-sent = { $date } · Wysłano
feedback-view = Wyświetl
feedback-view-tooltip = Otwórz raport
feedback-copy-tooltip = Skopiuj, aby wkleić do zgłoszenia błędu
feedback-copied = Skopiowano raport o awarii.
feedback-deleted-all = Usunięto raporty o awariach.
feedback-read-failed = Nie udało się odczytać raportu o awarii: { $error }
feedback-delete-failed = Nie udało się usunąć raportu o awarii: { $error }
feedback-delete-all-failed = Nie udało się usunąć raportów o awariach: { $error }

## Settings > User feedback (usage statistics)

feedback-usage = Wysyłaj anonimowe statystyki użycia
feedback-usage-detail = Raz w tygodniu: z których funkcji korzystałeś, tak lub nie. Nigdy liczby, adresy, nazwy ani wyszukiwane słowa
feedback-intro-sending-usage = Wysyłane są raporty o awariach i cotygodniowe statystyki użycia. Nic więcej nie opuszcza tego komputera.
feedback-intro-usage-only = Wysyłane są cotygodniowe statystyki użycia. Raporty o awariach zostają na tym komputerze.
feedback-counted = Co jest liczone
feedback-counted-detail = Każda pozycja to tak lub nie w danym tygodniu.
feedback-counted-also = Ponadto: wersja Katna, rodzina systemu Linux, środowisko pulpitu, skala ekranu i liczba kont (1, 2–3, 4+)
feedback-see-report = Zobacz raport z tego tygodnia
feedback-hide-report = Ukryj raport z tego tygodnia
feedback-report-goes = Wysyłany po zakończeniu tygodnia, { $date }, jeśli statystyki użycia są nadal włączone.
feedback-install-id = Identyfikator instalacji { $id }
feedback-install-id-tooltip = Losowy, aby jeden komputer nie był liczony dwa razy w tygodniu. Zmienia się co 90 dni i nigdy nie jest wysyłany z raportami o awariach ani z opiniami
feedback-install-id-reset = Resetuj
feedback-install-id-new = Utworzono nowy identyfikator instalacji.
feedback-report-copied = Skopiowano raport.
feedback-send-feedback = Opinia
feedback-send-feedback-detail = Problem, pomysł, cokolwiek.
feedback-send-feedback-button = Wyślij opinię…
usage-feature-search-options = Opcje wyszukiwania
usage-feature-pins = Przypięta poczta
usage-feature-labels = Etykiety
usage-feature-scheduled-send = Zaplanowane wysyłanie
usage-feature-snooze = Odkładanie i przypomnienia
usage-feature-encrypted = Poczta szyfrowana
usage-feature-viewers = Wbudowane przeglądarki plików
usage-feature-calendar = Kalendarz
usage-feature-contacts = Kontakty
usage-feature-tasks-notes = Zadania i Notatki
usage-feature-phone-layout = Układ na szerokość telefonu
usage-feature-own-frame = Własna ramka okna Katna

## Help > Send feedback

send-feedback-title = Wyślij opinię
send-feedback-about = Dotyczy
send-feedback-problem = Problem
send-feedback-idea = Pomysł
send-feedback-other = Coś innego
send-feedback-message = Twoja wiadomość
send-feedback-message-placeholder = Co się stało albo czego byś chciał?
send-feedback-reply = E-mail do odpowiedzi (opcjonalnie)
send-feedback-reply-placeholder = ty@example.org
send-feedback-system = Dołącz wersję Katna i informacje o systemie
send-feedback-what-is-sent = Co jest wysyłane
send-feedback-show = Pokaż
send-feedback-hide = Ukryj
send-feedback-where = Wysyłane do skrzynki opinii Katna w Sentry (UE). Bez adresu IP, kont, wiadomości i identyfikatora instalacji.
send-feedback-cancel = Anuluj
send-feedback-send = Wyślij
send-feedback-sending = Wysyłanie…
send-feedback-sent = Wysłano opinię. Dziękujemy
send-feedback-failed = Nie udało się wysłać opinii: { $error }
