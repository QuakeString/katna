# Katna Mail, Afrikaans (Afrikaans).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.
snooze-until = Sluimer tot…
snooze-later-today = Later vandag
snooze-tomorrow = Môre
snooze-this-weekend = Hierdie naweek
snooze-next-week = Volgende week
snooze-pick = Kies datum en tyd
snooze-back = Terug na die tye
snooze-type-placeholder = Tik 'n tyd
snooze-type-hint = Soos “tue 3pm”, “tomorrow” of “in 2 hours”
snooze-type-hint-unclear = Katna kan dit nie as 'n tyd lees nie
snooze-type-unclear = “{ $text }” is nie 'n tyd wat Katna ken nie

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = Sluimer
remind-tab = Herinner my
snooze-says = Versteek dit tot dan
remind-says = Hou dit waar dit is en stel jou in kennis
remind-before-due = Voor die sperdatum
remind-note = Nota (opsioneel)
remind-note-placeholder = Die onderwerp, as dit leeg gelaat word
toast-remind-set = Herinnering gestel vir { $date }
remind-chat-line = Herinnering { $date } · { $title }
remind-done = Klaar
toast-remind-done = Herinnering afgehandel
snooze-chat-line = Gesluimer tot { $date }
snooze-chat-change = Verander
snooze-cancel = Kanselleer
snooze-save = Stoor
snooze-in-the-past = Kies 'n tyd later as nou.

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = Volg op as niemand antwoord nie…
follow-up-title = Volg op as niemand antwoord nie
follow-up-off = Af
follow-up-days = { $days ->
    [one] { $days } dag
   *[other] { $days } dae
}
follow-up-weeks = { $weeks ->
    [one] { $weeks } week
   *[other] { $weeks } weke
}
follow-up-pick = Kies…
follow-up-pick-title = Volg op as niemand antwoord nie teen
follow-up-remind = Herinner my
follow-up-remind-note = Die gesprek kom terug na bo-aan jou inkassie
follow-up-send = Stuur 'n opvolg namens my
follow-up-send-note = Aan dieselfde mense, in dieselfde gesprek
follow-up-send-encrypted = Nie vir geënkripteerde e-pos nie
follow-up-text-placeholder = Wat om te skryf
follow-up-text-named = Hallo { $name }, ek wil net seker maak jy het my boodskap hieronder gesien.
follow-up-text = Hallo, ek wil net seker maak jy het my boodskap hieronder gesien.
follow-up-template = Gebruik 'n sjabloon
follow-up-signature = Jou handtekening word bygevoeg
follow-up-again = As daar steeds geen antwoord is nie, volg weer op na
follow-up-note = Stop sodra enigiemand in die gesprek antwoord. Outomatiese antwoorde tel nie.
follow-up-note-send = Stop sodra enigiemand in die gesprek antwoord. Gaan op weeksdae uit van { $start } tot { $end }, en nooit meer as 'n dag laat nie.
follow-up-cancel = Kanselleer
follow-up-done = Klaar
follow-up-chip-send = Opvolg oor { $time }
follow-up-chip-remind = Herinnering oor { $time }
follow-up-chip-send-on = Opvolg { $date }
follow-up-chip-remind-on = Herinnering { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = Nog geen antwoord nie
follow-up-card-title-waiting = Jou opvolg wag
follow-up-card-send = Katna stuur jou opvolg op { $date }. Dit stop wanneer enigiemand antwoord.
follow-up-card-send-twice = Katna stuur jou opvolg op { $date }, en later nog een keer. Dit stop wanneer enigiemand antwoord.
follow-up-card-remind = As niemand antwoord nie, kom hierdie gesprek op { $date } terug na jou inkassie.
follow-up-card-waiting = Dit was verskuldig terwyl jou rekenaar af was, so dit is nie laat gestuur nie. Stuur dit nou, kies 'n nuwe tyd, of stop dit.
follow-up-card-edit = Wysig
follow-up-card-edit-title = Opvolg op
follow-up-card-send-now = Stuur nou
follow-up-card-stop = Stop
follow-up-chat-send = Opvolg · { $date } as niemand antwoord nie
follow-up-chat-step = Opvolg { $step } van { $steps } · { $date } as niemand antwoord nie
follow-up-chat-waiting = Opvolg wag · dit was verskuldig terwyl jou rekenaar af was
follow-up-chat-remind = Terug in Inkassie { $date } as niemand antwoord nie
toast-follow-up-sent = Opvolg gestuur
toast-follow-up-stopped = Opvolg gestop
toast-follow-up-moved = Opvolg geskuif na { $date }
nudge-row = { $days ->
    [one] 1 dag gelede gestuur
   *[other] { $days } dae gelede gestuur
}. Volg op?
nudge-row-tip = Skryf 'n opvolg aan almal daarin
nudge-follow-up = Volg op
nudge-dismiss = Wys weg
nudge-card-title = Nog geen antwoord nie
nudge-card-text = Jy het { $days ->
    [one] 1 dag gelede
   *[other] { $days } dae gelede
} iets gevra en niemand het geantwoord nie.
nudge-chat-line = { $days ->
    [one] 1 dag gelede gestuur
   *[other] { $days } dae gelede gestuur
}, nog geen antwoord nie
toast-nudge-dismissed = Porretjie weggewys
