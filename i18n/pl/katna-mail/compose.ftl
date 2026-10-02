# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = Nowa wiadomość
compose-restore = Przywróć
compose-minimize = Minimalizuj
compose-exit-full-screen = Zamknij pełny ekran
compose-open-window = Otwórz w nowym oknie
compose-save-close = Zapisz i zamknij
compose-back-to-mail = Wróć do okna poczty
compose-pop-out-reply = Otwórz odpowiedź w osobnym oknie
compose-edit-recipients = Edytuj odbiorców
compose-summary-cc = DW: { $names }
compose-summary-bcc = UDW: { $names }
compose-more-recipients = jeszcze { $count }
compose-show-trimmed = Pokaż przyciętą treść
compose-hide-trimmed = Ukryj przyciętą treść
compose-remove-trimmed = Usuń cytowany tekst
compose-trimmed-removed = Usunięto cytowany tekst

## Recipients and subject

compose-to = Do
compose-cc = DW
compose-bcc = UDW
compose-from = Od
compose-from-choose = Wyślij z innego konta
compose-recipients = Odbiorcy
compose-subject = Temat

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = Najpierw wyślij lub odrzuć otwartą wiadomość.
compose-bad-address = „{ $address }” nie jest adresem e-mail.
compose-no-recipients = Dodaj co najmniej jednego odbiorcę.
compose-attachments-too-large = Załączniki mają { $size }; serwery poczty przyjmują do { $limit }.
compose-no-account = Dodaj konto, z którego będzie wysyłana poczta.
compose-past-time = Wybierz godzinę w przyszłości.
compose-scheduling = Planowanie…
compose-sending = Wysyłanie…
compose-scheduled = Wysyłka zaplanowana na { $when }
compose-sent-archived = Wysłano i zarchiwizowano
compose-sent = Wiadomość wysłana
compose-discarded = Wersja robocza odrzucona
compose-draft-saved = Zapisano wersję roboczą
compose-draft-saving = Zapisywanie…
compose-draft-failed = Nie udało się zapisać wersji roboczej: { $error }
compose-draft-not-opened = Nie udało się otworzyć wersji roboczej.

## Attachments

compose-picker-insert = Wstaw
compose-picker-attach = Załącz
compose-file-too-large = Plik { $name } jest za duży: wiadomość może zawierać do { $limit }.
compose-forward-files-missing = Pliki przekazywanej wiadomości nie są pobrane, więc nie zostały załączone.
compose-attachment-size = ({ $size })
compose-remove-attachment = Usuń załącznik
compose-attachments-total = { $count ->
    [one] { $count } plik, { $size }
    [few] { $count } pliki, { $size }
    [many] { $count } plików, { $size }
   *[other] { $count } pliku, { $size }
}
compose-drive-note = Plik { $name } przekracza { $limit }, więc trafia na Twój Dysk Google Drive, a wiadomość zawiera link.
compose-drive-tip = Na Twoim Google Drive; wiadomość zawiera link
compose-drive-uploading = Przesyłanie: { $percent }%
compose-drive-allow = Zezwól na Drive
compose-drive-allow-tip = Zaloguj się ponownie przez Google, aby Katna mogła umieszczać duże pliki na Twoim Drive
compose-drive-retry = Spróbuj ponownie
compose-drive-sends-when-uploaded = Wyślemy, gdy plik { $name } zostanie przesłany
compose-drive-not-uploaded = Pliku { $name } nie ma jeszcze w Google Drive
compose-drive-share-failed = Nie udało się udostępnić plików w Google Drive: { $error }
compose-drive-share-title = Udostępnić pliki wszystkim?
compose-drive-share-text = { $count ->
    [one] Google Drive nie może udostępnić plików adresatowi { $addresses }, który nie ma konta Google. Zamiast tego każdy, kto ma link, będzie mógł je otworzyć.
    [few] Google Drive nie może udostępnić plików adresatom { $addresses }, którzy nie mają konta Google. Zamiast tego każdy, kto ma link, będzie mógł je otworzyć.
    [many] Google Drive nie może udostępnić plików adresatom { $addresses }, którzy nie mają konta Google. Zamiast tego każdy, kto ma link, będzie mógł je otworzyć.
   *[other] Google Drive nie może udostępnić plików adresatom { $addresses }, którzy nie mają konta Google. Zamiast tego każdy, kto ma link, będzie mógł je otworzyć.
}
compose-drive-share-link = Udostępnij przez link
compose-drive-send-without = Wyślij bez udostępniania
compose-drive-share-cancel = Anuluj
compose-drive-card-detail = { $size } · Google Drive
compose-drive-card-name = Google Drive
compose-onedrive-note = Plik { $name } przekracza { $limit }, więc trafia na Twój OneDrive, a wiadomość zawiera link.
compose-onedrive-tip = Na Twoim OneDrive; wiadomość zawiera link
compose-onedrive-allow = Zezwól na OneDrive
compose-onedrive-allow-tip = Zaloguj się ponownie przez Microsoft, aby Katna mogła umieszczać duże pliki na Twoim OneDrive
compose-onedrive-not-uploaded = Pliku { $name } nie ma jeszcze w OneDrive
compose-onedrive-share-failed = Nie udało się udostępnić plików w OneDrive: { $error }
compose-onedrive-share-text = { $count ->
    [one] OneDrive nie może udostępnić plików adresatom { $addresses }. Zamiast tego każdy, kto ma link, będzie mógł je otworzyć.
    [few] OneDrive nie może udostępnić plików adresatom { $addresses }. Zamiast tego każdy, kto ma link, będzie mógł je otworzyć.
    [many] OneDrive nie może udostępnić plików adresatom { $addresses }. Zamiast tego każdy, kto ma link, będzie mógł je otworzyć.
   *[other] OneDrive nie może udostępnić plików adresatom { $addresses }. Zamiast tego każdy, kto ma link, będzie mógł je otworzyć.
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-onedrive-card-name = OneDrive
compose-drop-files = Upuść pliki tutaj
compose-drop-here = Upuść tutaj

## Paste options (a small bar under what was just pasted or dropped)

compose-paste-keep-formatting = Zachowaj formatowanie
compose-paste-table = Tabela
compose-paste-picture = Obraz
compose-paste-plain-text = Zwykły tekst
compose-paste-inline = W treści
compose-paste-attachment = Załącznik

## Encryption and signing (the toggles by the recipients)

compose-encrypt = Zaszyfruj
compose-encrypted = Zaszyfrowane: tylko odbiorcy mogą to przeczytać
compose-sign = Podpisz
compose-signed = Podpisane: odbiorcy mogą sprawdzić, że pochodzi od Ciebie

## Open and click tracking and read receipts (toggles after Sign)

compose-track = Śledź otwarcia i kliknięcia
compose-tracked = Śledzone: zobaczysz, kiedy każdy odbiorca ją otworzy lub kliknie link
compose-track-clicks = Śledź kliknięcia linków (zwykły tekst nie pokazuje otwarć)
compose-tracked-clicks = Śledzone: zobaczysz, kiedy każdy odbiorca kliknie link
compose-track-sign-in = Zaloguj się na konto Katna, aby śledzić otwarcia i kliknięcia
compose-receipt = Poproś o potwierdzenie przeczytania
compose-receipt-on = Poproszono o potwierdzenie przeczytania: aplikacja odbiorcy może zapytać go o jego wysłanie
compose-delivery = Poproś o potwierdzenie dostarczenia
compose-delivery-on = Poproszono o potwierdzenie dostarczenia: serwer poczty wyśle Ci e-mail, gdy serwer każdego odbiorcy przyjmie wiadomość
compose-delivery-unavailable = Twój serwer poczty nie wysyła potwierdzeń dostarczenia

## Spelling

spell-no-dictionary = Nie zainstalowano słownika pisowni dla { $language } (na przykład hunspell-en_us).
spell-dictionary-error = Słownik pisowni: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = „{ $words }”
grammar-add = Dodaj „{ $words }”
grammar-remove = Usuń „{ $words }”
grammar-ignore = Ignoruj

## Send checks (asked before a message goes out)

send-check-attachment-title = Czy chcesz załączyć pliki?
send-check-attachment-text = Wspominasz o załączniku, ale nic nie jest załączone.
send-check-attach = Załącz plik
send-check-subject-title = Wysłać bez tematu?
send-check-subject-text = Ta wiadomość nie ma tematu.
send-check-add-subject = Dodaj temat
send-check-send-anyway = Wyślij mimo to

## Recipients (To, Cc and Bcc)

recipient-not-valid = Nieprawidłowy adres e-mail
recipient-show-address = Pokaż adres
recipient-remove = Usuń
recipient-bad-title = Sprawdź adres
recipient-bad-text = „{ $address }” nie jest prawidłowym adresem e-mail. Popraw go lub usuń przed wysłaniem.
recipient-bad-fix = Popraw
