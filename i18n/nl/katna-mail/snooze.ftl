# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = Snoozen tot…
snooze-later-today = Later vandaag
snooze-tomorrow = Morgen
snooze-this-weekend = Dit weekend
snooze-next-week = Volgende week
snooze-pick = Datum en tijd kiezen
snooze-back = Terug naar de tijden
snooze-type-placeholder = Typ een tijd
snooze-type-hint = Zoals “di 15:00”, “morgen” of “over 2 uur”
snooze-type-hint-unclear = Katna kan dat niet als tijd lezen
snooze-type-unclear = “{ $text }” is geen tijd die Katna kent

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = Snoozen
remind-tab = Herinner me
snooze-says = Verbergt het tot dan
remind-says = Laat het staan waar het staat en geeft je een melding
remind-before-due = Voor de deadline
remind-note = Notitie (optioneel)
remind-note-placeholder = Het onderwerp, als je dit leeg laat
toast-remind-set = Herinnering ingesteld voor { $date }
remind-chat-line = Herinnering { $date } · { $title }
remind-done = Klaar
toast-remind-done = Herinnering afgerond
snooze-chat-line = Gesnoozed tot { $date }
snooze-chat-change = Wijzigen

## The date and time picker

snooze-cancel = Annuleren
snooze-save = Opslaan
snooze-in-the-past = Kies een tijd later dan nu.

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = Opvolgen als er geen antwoord komt…
follow-up-title = Opvolgen als er geen antwoord komt
follow-up-off = Uit
follow-up-days = { $days ->
    [one] { $days } dag
   *[other] { $days } dagen
}
follow-up-weeks = { $weeks ->
    [one] { $weeks } week
   *[other] { $weeks } weken
}
follow-up-pick = Kiezen…
follow-up-pick-title = Opvolgen als er geen antwoord is op
follow-up-remind = Herinner me
follow-up-remind-note = Het gesprek komt terug bovenaan je inbox
follow-up-send = Een opvolgbericht voor me sturen
follow-up-send-note = Aan dezelfde mensen, in hetzelfde gesprek
follow-up-send-encrypted = Niet voor versleutelde e-mail
follow-up-text-placeholder = Wat je wilt schrijven
follow-up-text-named = Hoi { $name }, ik wilde even checken of je mijn bericht hieronder hebt gezien.
follow-up-text = Hoi, ik wilde even checken of je mijn bericht hieronder hebt gezien.
follow-up-template = Een sjabloon gebruiken
follow-up-signature = Je handtekening wordt toegevoegd
follow-up-again = Als er nog steeds geen antwoord is, opnieuw opvolgen na
follow-up-note = Stopt zodra iemand in het gesprek antwoordt. Automatische antwoorden tellen niet mee.
follow-up-note-send = Stopt zodra iemand in het gesprek antwoordt. Gaat op werkdagen van { $start } tot { $end } de deur uit, en nooit meer dan een dag te laat.
follow-up-cancel = Annuleren
follow-up-done = Klaar
follow-up-chip-send = Opvolgbericht over { $time }
follow-up-chip-remind = Herinnering over { $time }
follow-up-chip-send-on = Opvolgbericht { $date }
follow-up-chip-remind-on = Herinnering { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = Nog geen antwoord
follow-up-card-title-waiting = Je opvolgbericht wacht
follow-up-card-send = Katna stuurt je opvolgbericht op { $date }. Het stopt zodra iemand antwoordt.
follow-up-card-send-twice = Katna stuurt je opvolgbericht op { $date }, en later nog een keer. Het stopt zodra iemand antwoordt.
follow-up-card-remind = Als niemand antwoordt, komt dit gesprek op { $date } terug in je inbox.
follow-up-card-waiting = Het was aan de beurt terwijl je computer uit stond, dus het is niet te laat verstuurd. Verstuur het nu, kies een nieuwe tijd of stop het.
follow-up-card-edit = Bewerken
follow-up-card-edit-title = Opvolgen op
follow-up-card-send-now = Nu versturen
follow-up-card-stop = Stoppen
follow-up-chat-send = Opvolgbericht · { $date } als niemand antwoordt
follow-up-chat-step = Opvolgbericht { $step } van { $steps } · { $date } als niemand antwoordt
follow-up-chat-waiting = Opvolgbericht wacht · het was aan de beurt terwijl je computer uit stond
follow-up-chat-remind = Terug in de inbox op { $date } als er geen antwoord komt
toast-follow-up-sent = Opvolgbericht verstuurd
toast-follow-up-stopped = Opvolgbericht gestopt
toast-follow-up-moved = Opvolgbericht verplaatst naar { $date }

nudge-row = { $days ->
    [one] 1 dag geleden
   *[other] { $days } dagen geleden
} verstuurd. Opvolgen?
nudge-row-tip = Een opvolgbericht schrijven aan iedereen in het gesprek
nudge-follow-up = Opvolgen
nudge-dismiss = Negeren
nudge-card-title = Nog geen antwoord
nudge-card-text = Je hebt { $days ->
    [one] 1 dag geleden
   *[other] { $days } dagen geleden
} iets gevraagd en niemand heeft geantwoord.
nudge-chat-line = { $days ->
    [one] 1 dag geleden
   *[other] { $days } dagen geleden
} verstuurd, nog geen antwoord
toast-nudge-dismissed = Duwtje genegeerd
