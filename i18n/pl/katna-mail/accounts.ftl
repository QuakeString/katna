# Katna Mail, Polish (Polski).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = Panel folderów
accounts-folder-pane-detail = Foldery których kont pokazuje panel po lewej.
accounts-shown-one = Jedno konto naraz; przełączaj na karcie konta
accounts-shown-all = Wszystkie konta, jedno po drugim
accounts-row = Konta
accounts-row-detail = Panel folderów i menu kont pokazują konta w tej kolejności; pierwsze jest domyślne. Usunięcie konta usuwa kopię jego poczty przechowywaną przez Katna na tym komputerze. Poczta zostaje na serwerze.
accounts-none = Nie ma jeszcze kont.
accounts-kind-imported = Zaimportowane
accounts-picture-reset = Użyj zdjęcia z pulpitu
accounts-picture-change = Zmień zdjęcie
accounts-picture-remove = Usuń zdjęcie
accounts-rename = Zmień nazwę
accounts-name-save = Zapisz
accounts-name-cancel = Anuluj
accounts-name-placeholder = Twoje imię i nazwisko
accounts-rename-failed = Nie udało się zmienić nazwy konta: { $error }
accounts-move-up = Przenieś w górę
accounts-move-down = Przenieś w dół
accounts-drag = Przeciągnij, aby zmienić kolejność
accounts-remove = Usuń
accounts-delete-all-row = Usuń wszystkie dane
accounts-delete-all-row-detail = Zacznij od nowa, jak po nowej instalacji.
accounts-delete-all-about = Usuwa z tego komputera wszystkie konta, całą zapisaną pocztę, kontakty i kalendarze, indeks wyszukiwania, Twoje ustawienia i zapisane hasła. Na Twoich serwerach poczty nic się nie zmienia.
accounts-delete-all-open = Usuń wszystkie dane Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = Usunięto { $address } z Katna.
accounts-removed = Usunięto { $address } z Katna. Jego poczta nadal jest na serwerze.
accounts-all-deleted = Usunięto z tego komputera wszystkie dane Katna.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Usunąć { $address }?
accounts-remove-confirm = Usuń konto
accounts-removing = Usuwanie…
accounts-remove-local-mail = { $folders ->
    [0] Cała poczta zaimportowana do tego konta
    [one] Cała poczta zaimportowana do tego konta, w jego folderze
    [few] Cała poczta zaimportowana do tego konta, w jego { $folders } folderach
    [many] Cała poczta zaimportowana do tego konta, w jego { $folders } folderach
   *[other] Cała poczta zaimportowana do tego konta, w jego { $folders } folderach
}
accounts-remove-local-settings = Jego ustawienia Katna
accounts-remove-mail = { $folders ->
    [0] Cała poczta tego konta przechowywana przez Katna
    [one] Cała poczta tego konta przechowywana przez Katna, w jego folderze
    [few] Cała poczta tego konta przechowywana przez Katna, w jego { $folders } folderach
    [many] Cała poczta tego konta przechowywana przez Katna, w jego { $folders } folderach
   *[other] Cała poczta tego konta przechowywana przez Katna, w jego { $folders } folderach
}
accounts-remove-outbox = Jego wiadomości czekające w skrzynce nadawczej
accounts-remove-settings = Jego zapisane hasło i ustawienia Katna
accounts-delete-all-title = Usunąć wszystkie dane Katna?
accounts-delete-all-confirm = Usuń wszystko
accounts-deleting = Usuwanie…
accounts-delete-all-accounts = Wszystkie konta oraz cała poczta i załączniki przechowywane przez Katna
accounts-delete-all-contacts = Kontakty, kalendarze i indeks wyszukiwania
accounts-delete-all-settings = Wszystkie ustawienia, podpisy i skróty klawiszowe
accounts-delete-all-passwords = Wszystkie zapisane hasła
accounts-deleted-heading = Zostanie usunięte z tego komputera:
accounts-cannot-undo = Tej operacji nie można cofnąć.
accounts-server-delete-all = Na Twoich serwerach poczty nic się nie zmienia: poczta tam zostaje, a ponowne dodanie konta pobierze ją znowu. Poczta zaimportowana z plików jest tylko w Katna; same pliki pozostają nietknięte.
accounts-server-local = Ta poczta została zaimportowana z plików, więc Katna ma jedyną kopię. Pliki, z których pochodzi, pozostają nietknięte; zaimportuj je ponownie, aby ją odzyskać.
accounts-server-remove = Na serwerze poczty nic się nie zmienia: poczta tam zostaje, a ponowne dodanie konta pobierze ją znowu.
accounts-confirm-word = usuń
accounts-confirm-placeholder = Wpisz „{ accounts-confirm-word }”
accounts-confirm-prompt = Aby potwierdzić, wpisz „{ accounts-confirm-word }”:
accounts-cancel = Anuluj
