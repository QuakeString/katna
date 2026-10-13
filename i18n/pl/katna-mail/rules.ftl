# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Reguły
settings-rules-summary = Samodzielnie sortuj, etykietuj, przekazuj dalej lub wyciszaj nową pocztę
settings-rules-intro = Reguły same sortują nową pocztę, w tej kolejności. Przeciągnij, aby zmienić kolejność.
settings-rules-all-accounts = Wszystkie konta
settings-rules-new = Nowa reguła
settings-rules-none = Nie ma jeszcze reguł. Reguła sama sortuje nową pocztę: według nadawcy, tematu lub słów.
settings-rules-none-account = To konto nie ma jeszcze reguł.
settings-rules-drag = Przeciągnij, aby zmienić kolejność
settings-rules-edit = Edytuj regułę
settings-rules-turn-off = Wyłącz tę regułę
settings-rules-turn-on = Włącz tę regułę

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = Gotowe reguły
settings-rules-starters-intro = Wyłączone, dopóki którejś nie włączysz. Działają dla wszystkich Twoich kont; edytuj regułę, aby ją zmienić.
settings-rules-starter-turning-on = Włączanie reguły „{ $name }”…
settings-rules-starter-failed = Nie udało się włączyć reguły „{ $name }”: { $error }
rules-starter-promotions = Wyciszone promocje
rules-starter-newsletters = Newslettery do przeczytania
rules-starter-receipts = Paragony i faktury
rules-starter-deliveries = Przesyłki
rules-starter-train = Bilety kolejowe
rules-starter-flight = Bilety lotnicze
rules-starter-codes = Kody jednorazowe
rules-starter-security = Alerty bezpieczeństwa
rules-starter-social = Poczta z serwisów społecznościowych
rules-starter-invites = Zaproszenia z kalendarza
rules-starter-folder-reading = Do przeczytania
rules-starter-folder-receipts = Paragony
rules-starter-folder-deliveries = Przesyłki
rules-starter-folder-travel = Podróże
rules-starter-folder-social = Społecznościowe
rules-runs-katna = Działa w Katna
rules-runs-gmail = Działa w Gmail
rules-runs-sieve = Działa na serwerze
rules-stopped = Zatrzymana
rules-error-folder-gone = Folder używany przez tę regułę już nie istnieje. Edytuj regułę, aby wybrać inny.
rules-error-no-archive = To konto nie ma folderu archiwum. Edytuj regułę, aby robiła coś innego.
rules-error-no-trash = To konto nie ma folderu Kosz. Edytuj regułę, aby robiła coś innego.
rules-error-cannot-send = To konto nie może wysyłać poczty, więc reguła nie może jej przekazywać dalej.
rules-error-other = { $error }. Edytuj regułę i włącz ją ponownie.

settings-folders = Foldery
settings-folders-summary = Liczba nieprzeczytanych w panelu folderów
settings-folders-unread-counts = Liczba nieprzeczytanych przy każdym folderze
settings-folders-unread-counts-detail = Wyłączone: tylko Odebrane pokazują, ile jest nieprzeczytanych

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } i { $next }
rules-summary-or = { $first } lub { $next }
rules-summary-more = jeszcze { $count }
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Ma załącznik
rules-summary-no-attachment = Nie ma załącznika
rules-summary-mailing-list = Z listy dyskusyjnej
rules-summary-not-mailing-list = Nie z listy dyskusyjnej
rules-summary-tab = Na karcie { $tab }
rules-summary-not-tab = Nie na karcie { $tab }
rules-summary-move = przenieś do { $folder }
rules-summary-archive = pomiń Odebrane
rules-summary-trash = przenieś do kosza
rules-summary-mark-read = oznacz jako przeczytane
rules-summary-star = oznacz gwiazdką
rules-summary-important = oznacz jako ważne
rules-summary-label = dodaj etykietę { $label }
rules-summary-forward = przekaż do { $address }
rules-summary-dont-notify = nie powiadamiaj
rules-summary-read-after = { $count ->
    [one] oznacz jako przeczytane po { $count } dniu
    [few] oznacz jako przeczytane po { $count } dniach
    [many] oznacz jako przeczytane po { $count } dniach
   *[other] oznacz jako przeczytane po { $count } dnia
}
rules-summary-folder-gone = usunięty folder

## The rule editor

rules-editor-new-title = Nowa reguła
rules-editor-edit-title = Edytuj regułę
rules-editor-name-hint = Nazwa reguły
rules-editor-when = Gdy nowa wiadomość spełnia
rules-editor-of-these = z tych warunków:
rules-mode-all = wszystkie
rules-mode-any = dowolny
rules-field-from = Od
rules-field-to = Do
rules-field-cc = DW
rules-field-any-recipient = Do lub DW
rules-field-reply-to = Odpowiedź do
rules-field-subject = Temat
rules-field-body = Treść
rules-field-attachment-name = Nazwa załącznika
rules-field-has-attachment = Ma załącznik
rules-field-mailing-list = Z listy dyskusyjnej
rules-field-tab = Karta Odebranych
rules-comparator-contains = zawiera
rules-comparator-not-contains = nie zawiera
rules-comparator-begins-with = zaczyna się od
rules-comparator-ends-with = kończy się na
rules-comparator-equals = jest dokładnie
rules-comparator-matches = pasuje do wzorca
rules-has-yes = tak
rules-has-no = nie
rules-editor-value-hint = Słowa lub adres
rules-editor-add-condition = Dodaj warunek
rules-editor-remove = Usuń
rules-editor-then = Wtedy:
rules-action-move = Przenieś do
rules-action-archive = Pomiń Odebrane (archiwizuj)
rules-action-trash = Przenieś do kosza
rules-action-mark-read = Oznacz jako przeczytane
rules-action-star = Oznacz gwiazdką
rules-action-important = Oznacz jako ważne
rules-action-label = Dodaj etykietę
rules-action-forward = Przekaż do
rules-action-dont-notify = Nie powiadamiaj
rules-action-read-after = Oznacz jako przeczytane po
rules-editor-choose-folder = Wybierz folder
rules-editor-choose-label = Wybierz etykietę
rules-editor-new-folder = Nowy: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = Adres e-mail
rules-editor-days = dniach
rules-editor-add-action = Dodaj działanie
rules-editor-stop = Zatrzymaj tutaj: kolejne reguły nie działają na tę wiadomość
rules-editor-accounts = Konta:
rules-editor-accounts-none = Wybierz konta
rules-editor-accounts-many = { $count ->
    [one] { $count } konto
    [few] { $count } konta
    [many] { $count } kont
   *[other] { $count } konta
}
rules-editor-matches = Pasuje do: { $mails } z ostatnich { $days } dni
rules-editor-mails = { $count ->
    [one] { $count } wiadomość
    [few] { $count } wiadomości
    [many] { $count } wiadomości
   *[other] { $count } wiadomości
}
rules-editor-counting = Liczenie pasującej poczty…
rules-editor-show = Pokaż je
rules-editor-also-apply = Zastosuj też do tych wiadomości ({ $count })
rules-editor-runs-katna = Działa w Katna, gdy ten komputer jest włączony.
rules-editor-runs-gmail = Działa w Gmail, więc działa też na telefonie i przy wyłączonym komputerze.
rules-editor-runs-sieve = Działa na Twoim serwerze poczty, więc działa też na telefonie i przy wyłączonym komputerze.
rules-note-gmail-action = Działa w Katna: filtry Gmail nie potrafią wykonać „{ $action }”.
rules-note-sieve-action = Działa w Katna: reguły Twojego serwera poczty nie potrafią wykonać „{ $action }”.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Działa w Katna: filtry Gmail nie potrafią sprawdzić „{ $test }” tak jak Katna.
rules-note-sieve-condition = Działa w Katna: reguły Twojego serwera poczty nie potrafią sprawdzić „{ $test }” tak jak Katna.
rules-note-order = Działa w Katna, tak jak wcześniejsza reguła tego konta: reguły działają w kolejności z listy.
rules-note-gmail-stop = Działa w Katna: filtry Gmail nie potrafią powstrzymać kolejnych reguł.
rules-note-gmail-forward = Działa w Katna: Gmail przekazuje tylko na adresy zweryfikowane w jego ustawieniach, a { $address } do nich nie należy.
rules-note-gmail-folder = Działa w Katna: Gmail nie ma etykiety dla folderu używanego przez tę regułę.
rules-note-sieve-folder = Działa w Katna: Twój serwer poczty nie ma folderu używanego przez tę regułę.
rules-note-gmail-sign-in = Działa w Katna, dopóki nie zalogujesz się ponownie w Google i nie pozwolisz Katna tworzyć filtrów Gmail.
rules-note-sieve-other-script = Działa w Katna: na Twoim serwerze poczty jest aktywny inny skrypt reguł („{ $name }”).
rules-note-gmail-failed = Działa w Katna: Gmail jej nie przyjął ({ $error }).
rules-note-sieve-failed = Działa w Katna: Twój serwer poczty jej nie przyjął ({ $error }).
rules-editor-cancel = Anuluj
rules-editor-save = Zapisz
rules-editor-saving = Zapisywanie…
rules-editor-delete = Usuń regułę
rules-editor-delete-ask = Usunąć tę regułę?
rules-editor-delete-keep = Zachowaj
rules-editor-delete-confirm = Usuń
rules-editor-needs-folder = Wybierz folder dla każdego „Przenieś do” i etykietę dla każdego „Dodaj etykietę”.
rules-editor-needs-days = „Oznacz jako przeczytane po” wymaga liczby dni od 1 do 3650.
rules-saved = Reguła zapisana
rules-saved-applied = { $count ->
    [one] Reguła zapisana i zastosowana do { $count } wiadomości
    [few] Reguła zapisana i zastosowana do { $count } wiadomości
    [many] Reguła zapisana i zastosowana do { $count } wiadomości
   *[other] Reguła zapisana i zastosowana do { $count } wiadomości
}
rules-apply-failed = Reguła zapisana, ale nie udało się jej zastosować: { $error }
rules-deleted = Reguła usunięta
rules-delete-failed = Nie udało się usunąć reguły: { $error }
rules-change-failed = Nie udało się zmienić reguł: { $error }
