# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = Mappenvenster
accounts-folder-pane-detail = Van welke accounts het venster links de mappen toont.
accounts-shown-one = Eén account tegelijk; wissel via de accountkaart
accounts-shown-all = Alle accounts, na elkaar
accounts-row = Accounts
accounts-row-detail = Het mappenvenster en het accountmenu tonen de accounts in deze volgorde; het eerste is de standaard. Als je een account verwijdert, wordt de kopie van de e-mail die Katna op deze computer heeft verwijderd. De e-mail blijft op de server.
accounts-none = Nog geen accounts.
accounts-kind-imported = Geïmporteerd
accounts-picture-reset = Desktopafbeelding gebruiken
accounts-picture-change = Afbeelding wijzigen
accounts-picture-remove = Afbeelding verwijderen
accounts-rename = Naam wijzigen
accounts-name-save = Opslaan
accounts-name-cancel = Annuleren
accounts-name-placeholder = Je naam
accounts-rename-failed = Kan de naam van het account niet wijzigen: { $error }
accounts-move-up = Omhoog
accounts-move-down = Omlaag
accounts-drag = Sleep om de volgorde te wijzigen
accounts-remove = Verwijderen
accounts-delete-all-row = Alle gegevens verwijderen
accounts-delete-all-row-detail = Opnieuw beginnen, zoals bij een nieuwe installatie.
accounts-delete-all-about = Verwijdert van deze computer elk account, alle opgeslagen e-mail, contacten en agenda’s, de zoekindex, je instellingen en opgeslagen wachtwoorden. Er verandert niets op je mailservers.
accounts-delete-all-open = Alle Katna-gegevens verwijderen

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } is verwijderd uit Katna.
accounts-removed = { $address } is verwijderd uit Katna. De e-mail staat nog op de server.
accounts-all-deleted = Alle Katna-gegevens zijn van deze computer verwijderd.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } verwijderen?
accounts-remove-confirm = Account verwijderen
accounts-removing = Verwijderen…
accounts-remove-local-mail = { $folders ->
    [0] Alle e-mail die in dit account is geïmporteerd
    [one] Alle e-mail die in dit account is geïmporteerd, in de map ervan
   *[other] Alle e-mail die in dit account is geïmporteerd, in de { $folders } mappen ervan
}
accounts-remove-local-settings = De Katna-instellingen ervan
accounts-remove-mail = { $folders ->
    [0] Alle e-mail van dit account die Katna heeft opgeslagen
    [one] Alle e-mail van dit account die Katna heeft opgeslagen, in de map ervan
   *[other] Alle e-mail van dit account die Katna heeft opgeslagen, in de { $folders } mappen ervan
}
accounts-remove-outbox = De berichten ervan die in het postvak UIT wachten
accounts-remove-settings = Het opgeslagen wachtwoord en de Katna-instellingen ervan
accounts-delete-all-title = Alle Katna-gegevens verwijderen?
accounts-delete-all-confirm = Alles verwijderen
accounts-deleting = Verwijderen…
accounts-delete-all-accounts = Elk account, en alle e-mail en bijlagen die Katna heeft opgeslagen
accounts-delete-all-contacts = Contacten, agenda’s en de zoekindex
accounts-delete-all-settings = Alle instellingen, handtekeningen en sneltoetsen
accounts-delete-all-passwords = Elk opgeslagen wachtwoord
accounts-deleted-heading = Verwijderd van deze computer:
accounts-cannot-undo = Dit kan niet ongedaan worden gemaakt.
accounts-server-delete-all = Er verandert niets op je mailservers: je e-mail blijft daar, en als je een account opnieuw toevoegt, wordt die opnieuw gedownload. E-mail die uit bestanden is geïmporteerd, staat alleen in Katna; de bestanden blijven onaangeroerd.
accounts-server-local = Deze e-mail is uit bestanden geïmporteerd, dus Katna heeft de enige kopie. De bestanden waar die vandaan komt, blijven onaangeroerd; importeer ze opnieuw om de e-mail terug te krijgen.
accounts-server-remove = Er verandert niets op de mailserver: je e-mail blijft daar, en als je het account opnieuw toevoegt, wordt die opnieuw gedownload.
accounts-confirm-word = verwijderen
accounts-confirm-placeholder = Typ ‘{ accounts-confirm-word }’
accounts-confirm-prompt = Typ ‘{ accounts-confirm-word }’ om te bevestigen:
accounts-cancel = Annuleren
