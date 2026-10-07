# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = Der E-Mail-Server

problems-signed-out = { $provider } hat Katna von { $address } abgemeldet. E-Mails werden nicht mehr synchronisiert.
problems-password-refused = { $provider } hat das Passwort für { $address } abgelehnt. Es wurde vielleicht geändert.
problems-no-answer = { $provider } antwortet nicht für { $address }. Katna versucht es weiter.
problems-offline = Sie sind offline. Ihre E-Mails sind weiterhin hier, und E-Mails, die Sie senden, warten, bis Sie wieder online sind.
problems-accounts-need-you = { $count ->
    [one] 1 Konto braucht Sie
   *[other] { $count } Konten brauchen Sie
}
problems-show = Anzeigen
problems-later = Später
problems-new-password = Neues Passwort
problems-try-again = Erneut versuchen

## The New password card

problems-password-title = Neues Passwort
problems-password-detail = { $provider } hat das gespeicherte Passwort für { $address } abgelehnt. Geben Sie das neue ein; Katna prüft es, bevor es gespeichert wird.
problems-password-placeholder = Passwort
problems-password-show = Passwort anzeigen
problems-password-hide = Passwort verbergen
problems-password-cancel = Abbrechen
problems-password-save = Speichern
problems-password-checking = Wird geprüft…
problems-password-refused-again = { $provider } hat auch dieses Passwort abgelehnt. Prüfen Sie es und versuchen Sie es erneut.
problems-password-saved = Passwort für { $address } gespeichert. Ihre E-Mails werden abgerufen…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = Der E-Mail-Server von { $address } hat das Verschieben { $count ->
    [one] einer Nachricht nicht akzeptiert, daher ist sie wieder dort, wo sie war.
   *[other] von { $count } Nachrichten nicht akzeptiert, daher sind sie wieder dort, wo sie waren.
}
problems-refused-flags = Der E-Mail-Server von { $address } hat das Markieren { $count ->
    [one] einer Nachricht (gelesen, markiert…) nicht akzeptiert, daher ist sie wieder wie vorher.
   *[other] von { $count } Nachrichten (gelesen, markiert…) nicht akzeptiert, daher sind sie wieder wie vorher.
}
problems-refused-label = Der E-Mail-Server von { $address } hat das Ändern der Labels { $count ->
    [one] einer Nachricht nicht akzeptiert, daher ist sie wieder wie vorher.
   *[other] von { $count } Nachrichten nicht akzeptiert, daher sind sie wieder wie vorher.
}
problems-refused-delete = Der E-Mail-Server von { $address } hat das Löschen { $count ->
    [one] einer Nachricht nicht akzeptiert, daher ist sie wieder da.
   *[other] von { $count } Nachrichten nicht akzeptiert, daher sind sie wieder da.
}
problems-refused-other = Der E-Mail-Server von { $address } hat { $count ->
    [one] eine Änderung nicht akzeptiert, daher hat Katna sie rückgängig gemacht.
   *[other] { $count } Änderungen nicht akzeptiert, daher hat Katna sie rückgängig gemacht.
}
problems-details = Details

## Katna's background service (katna-daemon) isn't running

service-starting = Der Katna-Hintergrunddienst wird gestartet…
service-failed = Der Katna-Hintergrunddienst startet nicht, daher werden keine E-Mails synchronisiert.
service-start-again = Erneut starten
service-started-again = Der Katna-Hintergrunddienst wurde beendet und erneut gestartet.
service-details-title = Warum der Dienst nicht startet
service-details-body = Kopieren Sie dies und senden Sie es mit Ihrem Bericht. Es enthält keine E-Mails oder Passwörter.
service-details-copy = Kopieren
service-details-close = Schließen
