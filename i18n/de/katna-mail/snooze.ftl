# Katna Mail, German (Deutsch).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.
snooze-until = Zurückstellen bis…
snooze-later-today = Später heute
snooze-tomorrow = Morgen
snooze-this-weekend = Dieses Wochenende
snooze-next-week = Nächste Woche
snooze-pick = Datum und Uhrzeit wählen
snooze-cancel = Abbrechen
snooze-save = Speichern
snooze-in-the-past = Wählen Sie einen späteren Zeitpunkt als jetzt.
follow-up-title = Erinnern, wenn keine Antwort kommt
follow-up-off = Nicht erinnern
follow-up-days = { $days ->
    [one] Nach { $days } Tag
   *[other] Nach { $days } Tagen
}

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = Zurückstellen bis…
snooze-later-today = Später heute
snooze-tomorrow = Morgen
snooze-this-weekend = Dieses Wochenende
snooze-next-week = Nächste Woche
snooze-pick = Datum und Uhrzeit wählen
snooze-back = Zurück zu den Zeiten
snooze-type-placeholder = Zeit eingeben
snooze-type-hint = Etwa „Di 15 Uhr“, „morgen“ oder „in 2 Stunden“
snooze-type-hint-unclear = Katna kann das nicht als Zeit lesen
snooze-type-unclear = „{ $text }“ ist keine Zeit, die Katna kennt

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = Zurückstellen
remind-tab = Erinnern
snooze-says = Blendet sie bis dahin aus
remind-says = Lässt sie, wo sie ist, und benachrichtigt Sie
remind-before-due = Vor der Fälligkeit
remind-note = Notiz (optional)
remind-note-placeholder = Der Betreff, wenn leer
toast-remind-set = Erinnerung für { $date } eingerichtet
remind-chat-line = Erinnerung { $date } · { $title }
remind-done = Erledigt
toast-remind-done = Erinnerung erledigt
snooze-chat-line = Zurückgestellt bis { $date }
snooze-chat-change = Ändern

## The date and time picker

snooze-cancel = Abbrechen
snooze-save = Speichern
snooze-in-the-past = Wählen Sie einen späteren Zeitpunkt als jetzt.

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = Nachfassen, wenn keine Antwort kommt…
follow-up-title = Nachfassen, wenn keine Antwort kommt
follow-up-off = Aus
follow-up-days = { $days ->
    [one] { $days } Tag
   *[other] { $days } Tage
}
follow-up-weeks = { $weeks ->
    [one] { $weeks } Woche
   *[other] { $weeks } Wochen
}
follow-up-pick = Wählen…
follow-up-pick-title = Nachfassen, wenn keine Antwort kommt bis
follow-up-remind = Erinnern
follow-up-remind-note = Die Konversation rückt wieder an den Anfang Ihres Posteingangs
follow-up-send = Für mich nachfassen
follow-up-send-note = An dieselben Personen, in derselben Konversation
follow-up-send-encrypted = Nicht für verschlüsselte E-Mails
follow-up-text-placeholder = Was Sie schreiben möchten
follow-up-text-named = Hallo { $name }, ich wollte nur nachfragen, ob Sie meine Nachricht unten gesehen haben.
follow-up-text = Hallo, ich wollte nur nachfragen, ob Sie meine Nachricht unten gesehen haben.
follow-up-template = Vorlage verwenden
follow-up-signature = Ihre Signatur wird hinzugefügt
follow-up-again = Wenn immer noch keine Antwort kommt, erneut nachfassen nach
follow-up-note = Endet, sobald jemand in der Konversation antwortet. Automatische Antworten zählen nicht.
follow-up-note-send = Endet, sobald jemand in der Konversation antwortet. Wird werktags zwischen { $start } und { $end } gesendet und nie mehr als einen Tag zu spät.
follow-up-cancel = Abbrechen
follow-up-done = Fertig
follow-up-chip-send = Nachfassen in { $time }
follow-up-chip-remind = Erinnerung in { $time }
follow-up-chip-send-on = Nachfassen { $date }
follow-up-chip-remind-on = Erinnerung { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = Noch keine Antwort
follow-up-card-title-waiting = Ihre Nachfass-E-Mail wartet
follow-up-card-send = Katna sendet Ihre Nachfass-E-Mail am { $date }. Das endet, sobald jemand antwortet.
follow-up-card-send-twice = Katna sendet Ihre Nachfass-E-Mail am { $date } und später noch einmal. Das endet, sobald jemand antwortet.
follow-up-card-remind = Wenn niemand antwortet, kehrt diese Konversation am { $date } in Ihren Posteingang zurück.
follow-up-card-waiting = Sie wurde fällig, während Ihr Computer aus war, und wurde daher nicht verspätet gesendet. Senden Sie sie jetzt, wählen Sie eine neue Zeit oder beenden Sie sie.
follow-up-card-edit = Bearbeiten
follow-up-card-edit-title = Nachfassen am
follow-up-card-send-now = Jetzt senden
follow-up-card-stop = Beenden
follow-up-chat-send = Nachfassen · { $date }, wenn niemand antwortet
follow-up-chat-step = Nachfassen { $step } von { $steps } · { $date }, wenn niemand antwortet
follow-up-chat-waiting = Nachfass-E-Mail wartet · sie wurde fällig, während Ihr Computer aus war
follow-up-chat-remind = Zurück im Posteingang { $date }, wenn keine Antwort kommt
toast-follow-up-sent = Nachfass-E-Mail gesendet
toast-follow-up-stopped = Nachfassen beendet
toast-follow-up-moved = Nachfassen auf { $date } verschoben

nudge-row = Gesendet { $days ->
    [one] vor 1 Tag
   *[other] vor { $days } Tagen
}. Nachfassen?
nudge-row-tip = Allen in der Konversation eine Nachfass-E-Mail schreiben
nudge-follow-up = Nachfassen
nudge-dismiss = Verwerfen
nudge-card-title = Noch keine Antwort
nudge-card-text = Sie haben { $days ->
    [one] vor 1 Tag
   *[other] vor { $days } Tagen
} etwas gefragt, und niemand hat geantwortet.
nudge-chat-line = Gesendet { $days ->
    [one] vor 1 Tag
   *[other] vor { $days } Tagen
}, noch keine Antwort
toast-nudge-dismissed = Hinweis verworfen
