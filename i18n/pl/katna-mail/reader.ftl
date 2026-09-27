# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Zamknij
reader-back = Wstecz
reader-mark-unread = Oznacz jako nieprzeczytane
reader-move-to = Przenieś do
reader-more = Więcej
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
reader-me = mnie
reader-to = do { $names }
reader-starred = Oznaczone gwiazdką
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

## Remote images and pictures

remote-hidden = Obrazy w tej wiadomości są ukryte.
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
attachment-save-all = Zapisz wszystkie
attachment-save-all-tooltip = Zapisz wszystkie załączniki w folderze
attachment-save-here = Zapisz tutaj
attachment-not-downloaded = Ta wiadomość nie jest pobrana.
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
print-preview-cancel = Anuluj
print-preview-print = Drukuj
print-not-downloaded = (Jeszcze nie pobrano.)
print-encrypted = (Zaszyfrowana. Otwórz ją w Katna Mail, aby wydrukować jej treść.)
print-to = Do: { $addresses }
print-cc = DW: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Otwórz tę wiadomość, aby odczytać jej załączniki.
text-copy = Kopiuj
text-select-all = Zaznacz wszystko
