# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = E-Mail-Konto hinzufügen
add-account-looking = Mailserver für { $address } werden gesucht…
add-account-address-intro = Geben Sie Ihre E-Mail-Adresse ein. Katna findet die Server für Sie.
add-account-servers-title = Servereinstellungen
add-account-servers-intro = Wo Katna E-Mails für { $address } liest und sendet.
add-account-password-title = Passwort eingeben
add-account-signing-in = Anmeldung läuft…

## Add a mail account: fields

add-account-field-address = E-Mail-Adresse
add-account-incoming = Eingehende E-Mails ({ $protocol })
add-account-outgoing = Ausgehende E-Mails ({ $protocol })
add-account-field-server = Server
add-account-field-port = Port
add-account-security-none = Keine
add-account-field-username = Benutzername
add-account-field-password = Passwort
add-account-show-password = Passwort anzeigen
add-account-app-password-hint = { $provider } braucht hier ein App-Passwort, nicht das Passwort, das Sie im Web verwenden. Erstellen Sie eines in den Sicherheitseinstellungen Ihres { $provider }-Kontos.
add-account-field-name = Ihr Name (optional)
add-account-name-hint = Wird den Personen angezeigt, denen Sie schreiben.
add-account-servers-pair = { $imap } und { $smtp }
add-account-servers-found = { $source ->
    [built-in] Server: { $servers }, gefunden in Katnas Anbieterliste.
    [provider] Server: { $servers }, gefunden in den Einstellungen Ihres Anbieters.
    [ispdb] Server: { $servers }, gefunden in Thunderbirds Anbieterliste.
    [dns] Server: { $servers }, gefunden in den DNS-Einträgen Ihrer Domain.
   *[other] Server: { $servers }, geschätzt; prüfen Sie sie, falls die Anmeldung fehlschlägt.
}
add-account-servers-entered = Server: { $servers }, wie eingegeben.

## Add a mail account: buttons

add-account-servers-button = Servereinstellungen
add-account-back = Zurück
add-account-add = Konto hinzufügen
add-account-next = Weiter
add-account-cancel = Abbrechen

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Geben Sie den Server für eingehende E-Mails ein.
   *[outgoing] Geben Sie den Server für ausgehende E-Mails ein.
}
add-account-server-space = { $kind ->
    [incoming] Der Name des Servers für eingehende E-Mails enthält ein Leerzeichen.
   *[outgoing] Der Name des Servers für ausgehende E-Mails enthält ein Leerzeichen.
}
add-account-port-invalid = { $kind ->
    [incoming] Der Port für eingehende E-Mails muss eine Zahl von { $min } bis { $max } sein.
   *[outgoing] Der Port für ausgehende E-Mails muss eine Zahl von { $min } bis { $max } sein.
}
add-account-address-empty = Geben Sie eine E-Mail-Adresse ein.
add-account-address-invalid = Geben Sie eine E-Mail-Adresse wie { $example } ein.
add-account-not-found = Katna konnte die Server für { $address } nicht finden und hat die üblichen Namen eingetragen. Prüfen Sie sie bei Ihrem Anbieter.
add-account-password-empty = Geben Sie das Passwort ein.
add-account-name-is-password = Der Name ist derselbe wie das Passwort. Geben Sie dort stattdessen Ihren Namen ein, so wie andere ihn sehen sollen.
add-account-added = { $address } hinzugefügt. E-Mails werden abgerufen…
add-account-app-password-refused = { $provider } hat das Passwort abgelehnt. Nötig ist ein App-Passwort, nicht das Passwort, das Sie im Web verwenden.
add-account-password-refused = Der Server hat das Passwort abgelehnt. Prüfen Sie es und versuchen Sie es erneut.

## The account menu (from the account button on the top bar)

add-account-menu-another = Weiteres Konto hinzufügen
add-account-menu-manage = Konten verwalten
