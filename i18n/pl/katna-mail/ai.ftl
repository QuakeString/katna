# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

compose-ai-rephrase-tip = Przeredaguj (Ctrl+J)
compose-ai-tone-clearer = Jaśniej
compose-ai-tone-shorter = Krócej
compose-ai-tone-friendlier = Przyjaźniej
compose-ai-tone-formal = Formalnie
compose-ai-tone-grammar = Popraw gramatykę
compose-ai-tone-longer = Dłużej
compose-ai-custom = Powiedz, jak…
compose-ai-more = Więcej sposobów
compose-ai-replace = Zastąp
compose-ai-again = Spróbuj ponownie
compose-ai-below = Dodaj poniżej
compose-ai-copy = Kopiuj
compose-ai-cancel = Anuluj
compose-ai-rephrase = Przeredaguj
compose-ai-replaced = Przeredagowano
compose-ai-added = Dodano poniżej
compose-ai-copied = Skopiowano
compose-ai-katna = Katna AI
compose-ai-own = Twoja usługa AI
compose-ai-trial-left = { $service } · { $days ->
    [one] został 1 darmowy dzień
    [few] zostały { $days } darmowe dni
    [many] zostało { $days } darmowych dni
   *[other] zostało { $days } darmowego dnia
}
compose-ai-encrypted = Ta wiadomość zostanie zaszyfrowana. Przeredagowanie wysyła zaznaczony tekst do { $service } bez szyfrowania. Przeredagować mimo to?
compose-ai-sign-in = Katna AI wymaga konta Katna. Zaloguj się, aby z niego korzystać, albo użyj własnego klucza.
compose-ai-pay = Twój darmowy miesiąc Katna AI się skończył. Kosztuje 5 USD miesięcznie, możesz też użyć własnego klucza.
compose-ai-too-many = Na razie zbyt wiele żądań. Spróbuj ponownie za chwilę.
compose-ai-no-key = Dodaj klucz { $service } w Ustawieniach, aby przeredagowywać.
compose-ai-bad-key = { $service } nie przyjął Twojego klucza. Sprawdź go w Ustawieniach.
compose-ai-off = Pomoc w pisaniu z AI jest wyłączona w Ustawieniach.
compose-ai-failed = Nie udało się połączyć z { $service }. Spróbuj ponownie.
compose-ai-try-again = Spróbuj ponownie
compose-ai-open-settings = Otwórz Ustawienia
compose-ai-write-reply-tip = Napisz odpowiedź (Ctrl+J)
compose-ai-write-note-tip = Napisz notatkę (Ctrl+J)
compose-ai-rephrase-empty-tip = Wpisz coś, aby przeredagować
compose-ai-write-reply = Napisz odpowiedź
compose-ai-write-note = Napisz notatkę
compose-ai-write-from = { $count ->
    [one] z 1 wiadomości
    [few] z { $count } wiadomości
    [many] z { $count } wiadomości
   *[other] z { $count } wiadomości
}
compose-ai-write-ideas = Pomysły z wątku
compose-ai-write-own = Albo powiedz, co ma zawierać…
compose-ai-write-short = Krótko
compose-ai-write-longer = Dłużej
compose-ai-write-friendly = Przyjaźnie
compose-ai-write-formal = Formalnie
compose-ai-write-insert = Wstaw
compose-ai-write-back = Inne pomysły
compose-ai-written = Dodano wersję roboczą
compose-ai-write-encrypted = Ten wątek jest zaszyfrowany. Napisanie odpowiedzi wysyła jego wiadomości do { $service } bez szyfrowania. Napisać mimo to?
compose-ai-write-anyway = Napisz
compose-ai-write-encrypted-off = Ten wątek jest zaszyfrowany, a Ustawienia wyłączają pomoc w pisaniu dla zaszyfrowanej poczty.
compose-ai-subject-tip = Przeredaguj temat
compose-ai-subject-title = Inne sformułowania
compose-ai-subject-done = Zmieniono temat

## Summing up a conversation: the list's right-click menu, the reading
## pane's sparkle, the chat's strip and the card each opens.

summary-summarize = Podsumuj
summary-hide = Ukryj podsumowanie
summary-close = Zamknij
summary-fold = Zwiń
summary-title = Podsumowanie
summary-mails = { $count ->
    [one] 1 wiadomość
    [few] { $count } wiadomości
    [many] { $count } wiadomości
   *[other] { $count } wiadomości
}
summary-of-mails = { $count } z { $total } wiadomości
summary-peek-count = { $mails ->
    [one] 1 wiadomość
    [few] { $mails } wiadomości
    [many] { $mails } wiadomości
   *[other] { $mails } wiadomości
} · { $people ->
    [one] 1 osoba
    [few] { $people } osoby
    [many] { $people } osób
   *[other] { $people } osoby
}
summary-catch-up = { $count ->
    [one] 1 nowa od ostatniego czytania
    [few] { $count } nowe od ostatniego czytania
    [many] { $count } nowych od ostatniego czytania
   *[other] { $count } nowej od ostatniego czytania
}
summary-strip-newer = { $count ->
    [one] 1 nowa od tego czasu · { $gist }
    [few] { $count } nowe od tego czasu · { $gist }
    [many] { $count } nowych od tego czasu · { $gist }
   *[other] { $count } nowej od tego czasu · { $gist }
}
summary-add-new = { $count ->
    [one] Dodaj 1 nową
    [few] Dodaj { $count } nowe
    [many] Dodaj { $count } nowych
   *[other] Dodaj { $count } nowej
}
summary-point-settled = Ustalone
summary-point-money = Pieniądze
summary-point-dates = Terminy
summary-point-next = Dalej
summary-point-open = Otwarte
summary-files = Pliki
summary-for-you = Dla Ciebie
summary-from-mail = { $name }, { $date }
summary-you = Ty
summary-made = { $service } · { $time }
summary-not-read = { $service } · nie oznaczono jako przeczytane
summary-copy = Kopiuj
summary-copied = Skopiowano podsumowanie
summary-again = Podsumuj ponownie
summary-open = Otwórz
summary-open-tip = Otwórz wątek
summary-reply = Odpowiedz
summary-reply-tip = Napisz odpowiedź z AI
summary-reply-to = Odpowiedz: { $name }
summary-reply-summary = Podsumowanie
summary-reply-send = Wyślij
summary-reply-open = Otwórz
summary-asking = Pytanie { $service }…
summary-stop = Zatrzymaj
summary-cancel = Anuluj
summary-send = Wyślij i podsumuj
summary-ask-short = Czeka na Twoją zgodę
summary-encrypted = Ten wątek jest zaszyfrowany. Podsumowanie wysyła jego tekst do { $service } bez szyfrowania.
summary-encrypted-off = Ten wątek jest zaszyfrowany, a Ustawienia wyłączają pomoc w pisaniu dla zaszyfrowanej poczty.
summary-try-again = Spróbuj ponownie
summary-open-settings = Otwórz Ustawienia
