# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = Czytanie
chat-view = Wątki jako czaty
chat-view-detail = Poczta między ludźmi czyta się jak czat grupowy: dymek dla każdej wiadomości tylko z tym, co napisano, Twoje po prawej. Biuletyny zachowują zwykły widok.
chat-view-switch = Pokazuj wątki jako czaty
chat-view-switch-detail = Cytowana poczta i podpisy czekają za ··· w każdym dymku

chat-switch-chat = Czat
chat-switch-mail = Poczta
chat-people = { $names } i Ty · { $count ->
    [one] { $count } wiadomość
    [few] { $count } wiadomości
    [many] { $count } wiadomości
   *[other] { $count } wiadomości
}
chat-people-heading = { $count ->
    [one] W tym czacie · { $count } osoba
    [few] W tym czacie · { $count } osoby
    [many] W tym czacie · { $count } osób
   *[other] W tym czacie · { $count } osoby
}
chat-member-mails = { $count ->
    [0] Brak wiadomości
    [one] { $count } wiadomość
    [few] { $count } wiadomości
    [many] { $count } wiadomości
   *[other] { $count } wiadomości
}
chat-today = Dzisiaj
chat-yesterday = Wczoraj
chat-added = { $who } dodaje: { $names }
chat-renamed = { $who } zmienia temat na „{ $subject }”
chat-you = Ty
chat-not-downloaded = Jeszcze nie pobrano
chat-forwarded = Przekazana dalej
chat-show-quoted = Pokaż cytowaną pocztę i podpis
chat-hide-quoted = Ukryj cytowaną pocztę i podpis
chat-hide-dots = Ukryj ···
chat-show-card = Pokaż wizytówkę
chat-reply-all = Odpowiedz wszystkim
chat-more = Więcej
chat-reply-only = Odpowiedz tylko: { $name }
chat-forward = Przekaż dalej
chat-copy-text = Kopiuj tekst
chat-show-as-mail = Pokaż jako wiadomość
chat-pin = Przypnij na górze
chat-pin-file = Przypnij plik na górze
chat-unpin = Odepnij
chat-unpin-file = Odepnij plik
chat-pinned-of = Przypięte { $at } z { $count }
chat-pins-all = Wszystkie przypięte
chat-pins-heading = Przypięte · { $count } z { $most }
chat-pins-drag = Przeciągnij, aby zmienić kolejność
chat-pin-from-mail = Wiadomość od: { $name } · { $when }
chat-pin-from-file = Plik od: { $name } · { $when }
chat-pin-from-text = Tekst od: { $name } · { $when }
chat-pins-full = Ten czat ma już 5 przypiętych
chat-pins-replace-title = Zastąp przypięty element
chat-pins-replace-hint = Czat mieści do 5 przypiętych elementów. Wybierz ten do odpięcia.
chat-pins-replace = Zastąp
chat-pins-cancel = Anuluj
chat-undo = Cofnij

chat-reply-to = Odpowiedz: { $names }
chat-send = Wyślij (Ctrl+Enter)
chat-attach = Załącz
chat-attach-photo = Zdjęcie
chat-attach-file = Plik
chat-attach-library = Z Plików
chat-attach-template = Szablon
chat-attach-signature = Podpis
chat-replying-to = Odpowiedź do: { $name }
chat-reply-newest = Odpowiedz na najnowszą wiadomość

## The attach picker (paperclip > From Files)

picker-title = Załącz z Plików
picker-search = Szukaj nazw, osób, tematów
picker-search-drive = Przeszukaj ten dysk
picker-mail-files = Pliki z poczty
picker-this-chat = Ten wątek
picker-this-computer = Ten komputer…
picker-in-chat = W TYM WĄTKU
picker-recent = OSTATNIE
picker-preview = Podgląd
picker-cancel = Anuluj
picker-attach = Załącz
picker-attach-count = Załącz { $count }
picker-selected = Zaznaczono: { $count }
picker-of-limit = z { $limit }
picker-in-mail = { $size } w wiadomości
picker-drive-links = { $count ->
    [one] 1 jako link Google Drive
    [few] { $count } jako linki Google Drive
    [many] { $count } jako linki Google Drive
   *[other] { $count } jako linki Google Drive
}
picker-onedrive-links = { $count ->
    [one] 1 jako link OneDrive
    [few] { $count } jako linki OneDrive
    [many] { $count } jako linki OneDrive
   *[other] { $count } jako linki OneDrive
}
picker-over = { $size }, więcej niż { $limit }, które mieści wiadomość
picker-getting = { $count ->
    [one] Pobieranie pliku z dysku…
    [few] Pobieranie { $count } plików z dysku…
    [many] Pobieranie { $count } plików z dysku…
   *[other] Pobieranie { $count } pliku z dysku…
}
picker-some-failed = { $count ->
    [one] Nie udało się odczytać jednego pliku
    [few] Nie udało się odczytać { $count } plików
    [many] Nie udało się odczytać { $count } plików
   *[other] Nie udało się odczytać { $count } pliku
}
