# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = Posticipa a…
snooze-later-today = Più tardi oggi
snooze-tomorrow = Domani
snooze-this-weekend = Questo fine settimana
snooze-next-week = La prossima settimana
snooze-pick = Scegli data e ora
snooze-back = Torna agli orari
snooze-type-placeholder = Digita un orario
snooze-type-hint = Ad esempio «mar 15:00», «domani» o «tra 2 ore»
snooze-type-hint-unclear = Katna non riesce a leggerlo come un orario
snooze-type-unclear = «{ $text }» non è un orario che Katna conosce

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = Posticipa
remind-tab = Ricordamelo
snooze-says = La nasconde fino ad allora
remind-says = La lascia dov’è e ti avvisa
remind-before-due = Prima della scadenza
remind-note = Nota (facoltativa)
remind-note-placeholder = L’oggetto, se lasciata vuota
toast-remind-set = Promemoria impostato per { $date }
remind-chat-line = Promemoria { $date } · { $title }
remind-done = Fatto
toast-remind-done = Promemoria completato
snooze-chat-line = Posticipata a { $date }
snooze-chat-change = Cambia

## The date and time picker

snooze-cancel = Annulla
snooze-save = Salva
snooze-in-the-past = Scegli un orario successivo a quello attuale.

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = Sollecita se nessuno risponde…
follow-up-title = Sollecita se nessuno risponde
follow-up-off = Disattivato
follow-up-days = { $days ->
    [one] { $days } giorno
    [many] { $days } di giorni
   *[other] { $days } giorni
}
follow-up-weeks = { $weeks ->
    [one] { $weeks } settimana
    [many] { $weeks } di settimane
   *[other] { $weeks } settimane
}
follow-up-pick = Scegli…
follow-up-pick-title = Sollecita se nessuno risponde entro
follow-up-remind = Ricordamelo
follow-up-remind-note = La conversazione torna in cima alla tua Posta in arrivo
follow-up-send = Invia un sollecito al posto mio
follow-up-send-note = Alle stesse persone, nella stessa conversazione
follow-up-send-encrypted = Non per la posta crittografata
follow-up-text-placeholder = Cosa scrivere
follow-up-text-named = Ciao { $name }, volevo solo sapere se hai visto il mio messaggio qui sotto.
follow-up-text = Ciao, volevo solo sapere se hai visto il mio messaggio qui sotto.
follow-up-template = Usa un modello
follow-up-signature = Viene aggiunta la tua firma
follow-up-again = Se ancora nessuno risponde, sollecita di nuovo dopo
follow-up-note = Si ferma appena qualcuno nella conversazione risponde. Le risposte automatiche non contano.
follow-up-note-send = Si ferma appena qualcuno nella conversazione risponde. Parte nei giorni feriali dalle { $start } alle { $end }, e mai con più di un giorno di ritardo.
follow-up-cancel = Annulla
follow-up-done = Fatto
follow-up-chip-send = Sollecito tra { $time }
follow-up-chip-remind = Promemoria tra { $time }
follow-up-chip-send-on = Sollecito { $date }
follow-up-chip-remind-on = Promemoria { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = Ancora nessuna risposta
follow-up-card-title-waiting = Il tuo sollecito è in attesa
follow-up-card-send = Katna invia il tuo sollecito il { $date }. Si ferma quando qualcuno risponde.
follow-up-card-send-twice = Katna invia il tuo sollecito il { $date }, poi un’altra volta più avanti. Si ferma quando qualcuno risponde.
follow-up-card-remind = Se nessuno risponde, questa conversazione torna nella tua Posta in arrivo il { $date }.
follow-up-card-waiting = È scaduto mentre il computer era spento, quindi non è stato inviato in ritardo. Invialo ora, scegli un nuovo orario o interrompilo.
follow-up-card-edit = Modifica
follow-up-card-edit-title = Sollecita il
follow-up-card-send-now = Invia ora
follow-up-card-stop = Interrompi
follow-up-chat-send = Sollecito · { $date } se nessuno risponde
follow-up-chat-step = Sollecito { $step } di { $steps } · { $date } se nessuno risponde
follow-up-chat-waiting = Sollecito in attesa · è scaduto mentre il computer era spento
follow-up-chat-remind = Di nuovo in Posta in arrivo { $date } se nessuno risponde
toast-follow-up-sent = Sollecito inviato
toast-follow-up-stopped = Sollecito interrotto
toast-follow-up-moved = Sollecito spostato a { $date }

nudge-row = Inviata { $days ->
    [one] 1 giorno fa
    [many] { $days } di giorni fa
   *[other] { $days } giorni fa
}. Vuoi sollecitare?
nudge-row-tip = Scrivi un sollecito a tutti i partecipanti
nudge-follow-up = Sollecita
nudge-dismiss = Ignora
nudge-card-title = Ancora nessuna risposta
nudge-card-text = Hai fatto una domanda { $days ->
    [one] 1 giorno fa
    [many] { $days } di giorni fa
   *[other] { $days } giorni fa
} e nessuno ha risposto.
nudge-chat-line = Inviata { $days ->
    [one] 1 giorno fa
    [many] { $days } di giorni fa
   *[other] { $days } giorni fa
}, ancora nessuna risposta
toast-nudge-dismissed = Richiamo ignorato
