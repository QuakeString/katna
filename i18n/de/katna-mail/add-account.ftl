# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = E-Mail-Konto hinzufügen
add-account-providers-intro = Wählen Sie Ihren E-Mail-Anbieter. Den Rest findet Katna.
add-account-provider-other = Andere E-Mail
add-account-provider-other-detail = Jedes IMAP- oder POP3-Konto
add-account-provider-google-detail = Gmail und Google Workspace
add-account-provider-microsoft-detail = Outlook und Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = Bei { $provider } anmelden
add-account-form-title-other = Ihr E-Mail-Konto
add-account-form-intro = Katna speichert Ihr Passwort im Schlüsselbund Ihres Systems.
add-account-looking = Mailserver für { $address } werden gesucht…
add-account-address-intro = Geben Sie Ihre E-Mail-Adresse ein. Katna findet die Server für Sie.
add-account-servers-title = Servereinstellungen
add-account-servers-intro = Wo Katna E-Mails für { $address } liest und sendet.
add-account-signing-in = Anmeldung läuft…
add-account-browser-title = Im Browser fortfahren
add-account-browser-intro = Katna hat die Anmeldeseite von { $provider } in Ihrem Browser geöffnet. Melden Sie sich dort an, erlauben Sie Katna, Ihre E-Mails zu lesen und zu senden, und kehren Sie dann hierher zurück.
add-account-browser-hint = Keine Seite geöffnet? Sehen Sie in den Fenstern Ihres Browsers nach, oder gehen Sie zurück und versuchen Sie es erneut.
add-account-stage-browser = Warten auf Ihre Anmeldung im Browser…
add-account-stage-signing-in-at = Anmeldung bei { $server }…
add-account-help-app-password-link = So erstellen Sie ein App-Passwort
add-account-help-turn-on-imap = { $provider } lässt E-Mail-Apps erst zu, wenn der IMAP- und POP3-Zugriff in den Einstellungen des Webmails eingeschaltet ist.
add-account-help-turn-on-imap-link = So schalten Sie ihn ein

## Add a mail account: fields

add-account-field-address = E-Mail-Adresse
add-account-receive-with = E-Mails empfangen mit
add-account-imap-about = IMAP behält Ihre E-Mails und Ordner auf dem Server, auf jedem Gerät gleich. Wählen Sie es, wenn möglich.
add-account-pop3-about = POP3 lädt Ihre E-Mails auf diesen Computer herunter. E-Mails, die Sie hier lesen oder verschieben, bleiben auf dem Server und Ihren anderen Geräten unverändert.
add-account-incoming = Eingehende E-Mails ({ $protocol })
add-account-outgoing = Ausgehende E-Mails ({ $protocol })
add-account-field-server = Server
add-account-field-port = Port
add-account-security-none = Keine
add-account-security-none-warning = Nicht verschlüsselt: Ihr Passwort und Ihre E-Mails können unterwegs mitgelesen werden.
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

add-account-sign-in-with = Mit { $provider } anmelden
add-account-sign-in-instead = Stattdessen mit { $provider } anmelden

add-account-servers-button = Servereinstellungen
add-account-back = Zurück
add-account-add = Konto hinzufügen
add-account-done = Fertig
add-account-another = Weiteres Konto hinzufügen
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
add-account-app-password-refused = { $provider } hat das Passwort abgelehnt. Nötig ist ein App-Passwort, nicht das Passwort, das Sie im Web verwenden.
add-account-password-refused = Der Server hat das Passwort abgelehnt. Prüfen Sie es und versuchen Sie es erneut.
add-account-sign-in-refused = { $provider } hat Katna keinen Zugang gewährt. Versuchen Sie es erneut und erlauben Sie den Zugriff auf Ihre E-Mails.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Diese Version von Katna kann sich noch nicht bei Microsoft-Konten anmelden.
    [Google] Diese Version von Katna kann sich noch nicht bei Google-Konten anmelden.
   *[other] Dieser Anbieter erlaubt die Anmeldung nur auf seiner eigenen Seite, und das kann Katna für ihn noch nicht.
}
add-account-smtp-not-found = Katna hat gefunden, wo Ihre E-Mails gelesen werden, aber nicht, wohin sie gesendet werden. Geben Sie den Postausgangsserver ein.

## Add a mail account: the last step

add-account-done-title = Ihr Konto ist bereit
add-account-done-intro = Katna ruft jetzt Ihre E-Mails ab. Neue E-Mails erscheinen, sobald sie eintreffen.
add-account-done-sign-in = Anmeldung
add-account-done-signed-in-with = Mit { $provider }, in Ihrem Browser
add-account-done-receiving = E-Mails empfangen
add-account-done-sending = E-Mails senden
add-account-done-on-server = E-Mails auf dem Server
add-account-done-kept = Bleiben, bis Sie sie in Katna löschen
add-account-done-pop3-hint = Was mit E-Mails auf dem Server geschieht, ändern Sie unter Einstellungen > Konten.
add-account-done-zoho-title = Aufgaben und Kalender
add-account-done-zoho-about = Zoho hält diese getrennt von den E-Mails. Melden Sie sich einmal mit Zoho an, um sie in Katna zu holen.
add-account-done-linked = Aufgaben und Kalender verbunden

## The account menu (from the account button on the top bar)

add-account-menu-another = Weiteres Konto hinzufügen
app-menu = Hauptmenü
app-menu-back = Zurück
