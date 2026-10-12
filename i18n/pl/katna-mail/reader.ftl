# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Zamknij
reader-back = Wstecz
reader-mark-unread = Oznacz jako nieprzeczytane
reader-move-to = Przenieś do
reader-snooze = Odłóż
reader-remind = Przypomnij mi
reader-more = Więcej
reader-original-colors = Pokaż oryginalne kolory
reader-dark-colors = Pokaż w ciemnych kolorach
reader-print-all = Drukuj wszystko
reader-new-window = W nowym oknie
reader-position = { $position } z { $total }
reader-newer = Nowszy
reader-older = Starszy

## Reading pane: the conversation

reader-removed = Ten wątek został usunięty.
reader-no-subject = (bez tematu)
reader-collapse-all = Zwiń wszystko
reader-expand-all = Rozwiń wszystko
reader-unknown-sender = (nieznany nadawca)
reader-date-ago = { $date } ({ $ago })
reader-sending = Wysyłanie…
reader-me = mnie
reader-to = do { $names }
reader-to-label = do
reader-tick-delivered = Dostarczono { $when }
reader-tick-no-bounce = Wysłano { $when }; nie przyszedł zwrot, więc wiadomość najpewniej dotarła
reader-tick-bounced = Nie dostarczono: zwrócona { $when }
reader-tick-read = Przeczytano { $when } (potwierdzenie przeczytania)
reader-tick-opened = Otwarto, ostatnio { $when } (śledzenie otwarć)
reader-starred = Oznaczone gwiazdką
reader-chip-remove = Usuń { $label }
reader-not-starred = Bez gwiazdki
reader-too-long = Wiadomość jest zbyt długa, aby wyświetlić ją w całości.
reader-encrypted-images = Obrazy z internetu nigdy nie są wczytywane w zaszyfrowanej poczcie.
reader-window-failed = Nie udało się otworzyć nowego okna.

## Reading pane: message details (opened from "to me")

reader-details-from = od:
reader-details-to = do:
reader-details-cc = dw:
reader-details-date = data:
reader-details-subject = temat:

## Reading pane: downloading a message

reader-downloading = Pobieranie tej wiadomości z serwera…
reader-download-failed = Nie udało się pobrać tej wiadomości.
reader-download-failed-reason = Nie udało się pobrać tej wiadomości. { $reason }
reader-download-offline = To konto jest offline. Przejdź w tryb online, aby pobrać tę wiadomość.
reader-try-again = Spróbuj ponownie

## Reply row

reply-reply = Odpowiedz
reply-reply-all = Odpowiedz wszystkim
reply-forward = Przekaż dalej

## Encrypted and signed mail

security-decrypting = Odszyfrowywanie…
security-checking = Sprawdzanie podpisu…
security-partly-encrypted = Tylko część tej wiadomości jest zaszyfrowana. Reszta została dodana poza ochroną i może pochodzić od kogokolwiek.
security-partly-signed = Tylko część tej wiadomości jest podpisana. Reszta została dodana poza ochroną i może pochodzić od kogokolwiek.
security-encrypted = Zaszyfrowana wiadomość
security-encrypted-smime = Zaszyfrowana wiadomość (S/MIME)
security-no-key = Nie można odszyfrować tej wiadomości: została zaszyfrowana dla klucza, którego nie masz.
security-cancelled = Odszyfrowywanie zostało anulowane.
security-damaged = Nie można odszyfrować tej wiadomości: zaszyfrowane dane są uszkodzone lub zostały zmienione.
security-decrypt-unavailable = Nie można odszyfrować tej wiadomości: zainstaluj { $tool }, aby czytać zaszyfrowaną pocztę.
security-decrypt-failed = Nie można odszyfrować tej wiadomości: { $reason }
security-unknown-signer = nieznany podpisujący
security-signed-verified = Podpis: { $signer } · zweryfikowany
security-signed-not-sender = Podpis: { $signer } · podpisujący nie jest nadawcą
security-signed-untrusted = Podpis: { $signer } · klucz oznaczony przez Ciebie jako niezaufany
security-signed-unverified = Podpis: { $signer } · klucz nie jest zweryfikowany
security-bad-signature = Nieprawidłowy podpis: ta wiadomość została zmieniona po podpisaniu lub podpis jest sfałszowany.
security-signature-expired = Podpis: { $signer } · podpis wygasł
security-key-expired = Podpis: { $signer } · klucz od tego czasu wygasł
security-key-revoked = Podpis: { $signer } · klucz został unieważniony
security-missing-key = Podpisano kluczem, którego nie masz, więc nie można tego sprawdzić
security-missing-key-id = Podpisano kluczem, którego nie masz ({ $key }), więc nie można tego sprawdzić
security-signature-unavailable = Podpisano; zainstaluj { $tool }, aby sprawdzić podpis
security-signature-error = Nie udało się sprawdzić podpisu.
security-look-up-key = Wyszukaj klucz

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = Zweryfikowany podpis
key-card-verified-detail = Podpis jest prawidłowy i ufasz temu kluczowi.
key-card-unverified = Podpis niezweryfikowany
key-card-unverified-detail = Podpis jest prawidłowy, ale nic nie potwierdza, że klucz należy do tej osoby. Porównaj z nią odcisk palca, a potem oznacz klucz jako zaufany w GnuPG (Kleopatra lub gpg --edit-key).
key-card-not-sender = Podpisane przez kogoś innego
key-card-not-sender-detail = Podpis jest prawidłowy, ale klucz nie należy do nadawcy.
key-card-untrusted = Klucz niezaufany
key-card-untrusted-detail = Oznaczyłeś ten klucz w GnuPG jako niezaufany.
key-card-signature-expired = Podpis wygasł
key-card-signature-expired-detail = Podpis był prawidłowy, ale wygasł.
key-card-key-expired = Klucz wygasł
key-card-key-expired-detail = Podpis jest prawidłowy, ale klucz od tego czasu wygasł.
key-card-key-revoked = Klucz unieważniony
key-card-key-revoked-detail = Właściciel unieważnił ten klucz, więc podpisowi nie można ufać.
key-card-bad = Nieprawidłowy podpis
key-card-bad-detail = Ta wiadomość została zmieniona po podpisaniu lub podpis jest sfałszowany.
key-card-signed-by = Podpisane przez
key-card-belongs-to = Należy do
key-card-fingerprint = Odcisk palca
key-card-signed = Podpisano
key-card-key = Klucz
key-card-kind = { $standard }, { $algorithm }
key-card-created = Utworzono
key-card-expires = Wygasa
key-card-never = Nigdy
key-card-issued-by = Wystawca
key-card-found-in = Znaleziono w
key-card-keyring = Twoja baza kluczy GnuPG
key-card-copy = Kopiuj odcisk palca
key-card-import-title = Zaimportować ten klucz?
key-card-from-directory = Znaleziono w katalogu kluczy domeny { $domain }.
key-card-from-attachment = Z załącznika { $name }.
key-card-import-note = Katna będzie mogła wtedy sprawdzać podpisy tej osoby i szyfrować do niej pocztę. Aby w pełni zaufać kluczowi, porównaj z nią odcisk palca.
key-card-cancel = Anuluj
key-card-import = Importuj klucz
key-card-looking-up = Wyszukiwanie klucza…
key-card-looking-up-detail = Pytanie katalogu kluczy domeny { $domain }.
key-card-not-found = Nie znaleziono klucza
key-card-not-found-detail = Domena { $domain } nie publikuje klucza dla tego adresu. Poproś nadawcę, aby wysłał ci swój.
key-card-not-kept = Znalezionego klucza nie można użyć.
key-card-failed = Nie udało się pobrać klucza

## Sender checks: the banner on mail that failed the checks the user's mail
## provider ran on its sender (DMARC, DKIM, SPF), and the "?" on the picture
## of a sender nothing confirmed

sender-failed-title = To może nie pochodzić z { $domain }
sender-failed-body = Wiadomość nie przeszła kontroli nadawcy w { $provider }. Uważaj na linki, załączniki i odpowiedzi.
sender-provider-unknown = twoim dostawcy poczty
sender-details = Szczegóły
sender-details-hide = Ukryj szczegóły
sender-looks-safe = Wygląda bezpiecznie
sender-move-to-spam = Przenieś do spamu
sender-checked-by = Sprawdzone w { $provider }
sender-checked-by-server = Sprawdzone w { $provider } ({ $server })
sender-dmarc = Domena nadawcy (DMARC)
sender-dkim = Podpis (DKIM)
sender-spf = Serwer wysyłający (SPF)
sender-result-pass = Zaliczone
sender-result-fail = Niezaliczone
sender-result-unsure = Niepewne
sender-result-none = Brak
sender-result-missing = Nie sprawdzono
sender-dmarc-pass = { $domain } potwierdza tego nadawcę.
sender-dmarc-fail = Wiadomość nie zgadza się z tym, jak według { $domain } wysyłana jest jej poczta.
sender-dmarc-none = { $domain } nie publikuje żadnych zasad dla swojej poczty.
sender-dkim-pass = Podpisane przez { $domain }.
sender-dkim-fail = Podpis od { $domain } nie pasuje do wiadomości.
sender-dkim-none = Wiadomość nie była podpisana.
sender-spf-pass = Wysłane z serwera wymienionego przez { $domain }.
sender-spf-fail = Wysłane z serwera, którego { $domain } nie wymienia.
sender-spf-none = { $domain } nie wymienia swoich serwerów.
sender-check-unsure = Kontrola nie dała jednoznacznej odpowiedzi.
sender-unconfirmed = W { $provider } nie udało się potwierdzić, że to pochodzi z { $domain }. Każdy może wpisać dowolnego nadawcę.
sender-link-title = Otworzyć ten link?
sender-link-body = Ta wiadomość nie przeszła kontroli nadawcy. Link prowadzi do { $host }:
sender-link-cancel = Anuluj
sender-link-open = Otwórz

## Open and click tracking and read receipts (the eye's popover beside a
## sent message's star, and the line above a read receipt)

tracking-opened = Otwarte przez { $who } { $count ->
    [one] raz
    [few] { $count } razy
    [many] { $count } razy
   *[other] { $count } razy
}, ostatnio { $when }
tracking-opens-clicks = Otwarte przez { $who } { $opens ->
    [one] raz
    [few] { $opens } razy
    [many] { $opens } razy
   *[other] { $opens } razy
}, link kliknięty { $clicks ->
    [one] raz
    [few] { $clicks } razy
    [many] { $clicks } razy
   *[other] { $clicks } razy
}, ostatnio { $when }
tracking-clicked = Link kliknięty przez { $who } { $clicks ->
    [one] raz
    [few] { $clicks } razy
    [many] { $clicks } razy
   *[other] { $clicks } razy
}, ostatnio { $when }
tracking-maybe-opened = Możliwe otwarcie przez { $who } (Apple Mail wczytuje obrazy dla ochrony prywatności)
tracking-seen-none = Nikt jeszcze nie otworzył wiadomości ani nie kliknął linku
tracking-receipt = Potwierdzenie przeczytania od { $who }
tracking-receipt-read = { $who } przeczytał(a) (potwierdzenie przeczytania), { $when }
tracking-receipt-displayed = Potwierdzenie przeczytania: Twoja wiadomość została otwarta przez { $who }
tracking-receipt-other = Potwierdzenie przeczytania: Twoja wiadomość została usunięta lub obsłużona przez { $who } bez otwierania

## Remote images and pictures

remote-hidden = Obrazy w tej wiadomości są ukryte.
remote-hidden-unconfirmed = Obrazy są ukryte: nie udało się potwierdzić nadawcy.
remote-hidden-failed = Obrazy są ukryte: ta wiadomość nie przeszła kontroli nadawcy.
remote-show = Pokaż obrazy
remote-always-show = Zawsze pokazuj od tego nadawcy
remote-picture-use = Użyj
remote-picture-too-big = Wybierz obraz o rozmiarze do 8 MB.
remote-picture-type = Wybierz obraz PNG, JPEG, GIF, WebP lub SVG.
remote-picture-read-failed = Nie można odczytać obrazu: { $error }
remote-picture-keep-failed = Nie można zachować obrazu: { $error }
remote-picture-remove-failed = Nie można usunąć obrazu: { $error }

## Attachments

attachment-count = { $count ->
    [one] Jeden załącznik
    [few] { $count } załączniki
    [many] { $count } załączników
   *[other] { $count } załącznika
}
attachment-save = Zapisz
attachment-forward = Przekaż dalej
attachment-save-all = Zapisz wszystkie
attachment-save-all-tooltip = Zapisz wszystkie załączniki w folderze
attachment-save-here = Zapisz tutaj
attachment-not-downloaded = Ta wiadomość nie jest pobrana.
attachment-open-message = Otwórz tę wiadomość, aby odczytać jej załączniki.
attachment-not-found = Nie znaleziono tego załącznika w wiadomości.
attachment-read-failed = Nie udało się odczytać pliku { $name }
attachment-numbered = załącznik { $number }
attachment-saved-all = { $count ->
    [one] Zapisano { $count } plik w folderze { $place }
    [few] Zapisano { $count } pliki w folderze { $place }
    [many] Zapisano { $count } plików w folderze { $place }
   *[other] Zapisano { $count } pliku w folderze { $place }
}
attachment-saved-some = { $total ->
    [one] Zapisano { $saved } z { $total } pliku w folderze { $place }. Nie udało się zapisać: { $failed }
    [few] Zapisano { $saved } z { $total } plików w folderze { $place }. Nie udało się zapisać: { $failed }
    [many] Zapisano { $saved } z { $total } plików w folderze { $place }. Nie udało się zapisać: { $failed }
   *[other] Zapisano { $saved } z { $total } pliku w folderze { $place }. Nie udało się zapisać: { $failed }
}
attachment-saved-to = Zapisano w { $path }
attachment-save-failed = Nie udało się zapisać pliku { $name }: { $error }
attachment-open-failed = Nie udało się otworzyć pliku { $name }: { $error }
attachment-risky = Ten plik może uruchomić program, więc Katna go nie otwiera. Zamiast tego zapisz go.
attachment-encrypted-open = Ten plik dotarł zaszyfrowany. Zapisz go, aby otworzyć go w innym miejscu.

## Printing

print-failed = Nie udało się wydrukować: { $error }
print-no-font = nie znaleziono czcionki
print-opened-as-pdf = Otwarto jako PDF, aby wydrukować z tego miejsca.

print-preview-title = Podgląd wydruku
print-preview-laying-out = Układanie stron…
print-preview-pages = { $count ->
    [one] { $count } strona
    [few] { $count } strony
    [many] { $count } stron
   *[other] { $count } strony
}
print-preview-more = { $count ->
    [one] i jeszcze { $count } strona
    [few] i jeszcze { $count } strony
    [many] i jeszcze { $count } stron
   *[other] i jeszcze { $count } strony
}
print-preview-failed = nie udało się pokazać stron
print-preview-paper = Papier
print-preview-a4 = A4
print-preview-letter = Letter
print-preview-layout = Układ
print-preview-as-shown = Jak na ekranie
print-preview-simple = Tylko tekst
print-preview-backgrounds = Tła
print-preview-cancel = Anuluj
print-preview-print = Drukuj
print-not-downloaded = (Jeszcze nie pobrano.)
print-encrypted = (Zaszyfrowana. Otwórz ją w Katna Mail, aby wydrukować jej treść.)
print-to = Do: { $addresses }
print-cc = DW: { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = Przypnij na górze
text-copy-address = Kopiuj adres
text-copy = Kopiuj
text-select-all = Zaznacz wszystko
