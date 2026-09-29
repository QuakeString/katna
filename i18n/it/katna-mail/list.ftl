# Katna Mail, Italian (Italiano).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Principale
tab-promotions = Promozioni
tab-social = Social
tab-updates = Aggiornamenti
tab-forums = Forum
tab-focused = Evidenziata
tab-other = Altra
tab-inbox = Posta in arrivo
tab-newsletters = Newsletter
tab-notifications = Notifiche
tab-new = { $count ->
    [one] { $count } nuovo
    [many] { $count } nuovi
   *[other] { $count } nuovi
}
tab-provider-other = ordinata da Katna

## Mail list: toolbar

list-select = Seleziona
list-refresh = Aggiorna
list-checking = Controllo della nuova posta…
list-more = Altro
list-mark-read = Segna come già letto
list-mark-unread = Segna come da leggere
list-move-to = Sposta in
list-archive = Archivia
list-spam = Segnala come spam
list-delete = Elimina
list-snooze = Posticipa
list-unsnooze = Annulla posticipo
list-newer = Più recenti
list-older = Meno recenti
list-range = { $first }–{ $last } di { $total }
list-range-about = { $first }–{ $last } di circa { $total }
list-results = Risultati per «{ $query }»
list-results-corrected = Sono mostrati i risultati per «{ $query }»
list-search-instead = Cerca invece «{ $query }»
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Tutti
list-pick-none = Nessuno
list-pick-read = Già letti
list-pick-unread = Da leggere
list-pick-starred = Speciali
list-pick-unstarred = Non speciali

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } conversazione selezionata.
        [many] Tutte le { $count } di conversazioni sono selezionate.
       *[other] Tutte le { $count } conversazioni sono selezionate.
    }
   *[message] { $count ->
        [one] { $count } messaggio selezionato.
        [many] Tutti i { $count } di messaggi sono selezionati.
       *[other] Tutti i { $count } messaggi sono selezionati.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $count } conversazione in { $folder } selezionata.
        [many] Tutte le { $count } di conversazioni in { $folder } sono selezionate.
       *[other] Tutte le { $count } conversazioni in { $folder } sono selezionate.
    }
   *[message] { $count ->
        [one] { $count } messaggio in { $folder } selezionato.
        [many] Tutti i { $count } di messaggi in { $folder } sono selezionati.
       *[other] Tutti i { $count } messaggi in { $folder } sono selezionati.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] { $count } conversazione in questa pagina selezionata.
        [many] Tutte le { $count } di conversazioni in questa pagina sono selezionate.
       *[other] Tutte le { $count } conversazioni in questa pagina sono selezionate.
    }
   *[message] { $count ->
        [one] { $count } messaggio in questa pagina selezionato.
        [many] Tutti i { $count } di messaggi in questa pagina sono selezionati.
       *[other] Tutti i { $count } messaggi in questa pagina sono selezionati.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Seleziona { $count } conversazione
        [many] Seleziona tutte le { $count } di conversazioni
       *[other] Seleziona tutte le { $count } conversazioni
    }
   *[message] { $count ->
        [one] Seleziona { $count } messaggio
        [many] Seleziona tutti i { $count } di messaggi
       *[other] Seleziona tutti i { $count } messaggi
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Seleziona { $count } conversazione in { $folder }
        [many] Seleziona tutte le { $count } di conversazioni in { $folder }
       *[other] Seleziona tutte le { $count } conversazioni in { $folder }
    }
   *[message] { $count ->
        [one] Seleziona { $count } messaggio in { $folder }
        [many] Seleziona tutti i { $count } di messaggi in { $folder }
       *[other] Seleziona tutti i { $count } messaggi in { $folder }
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversazione già letta in questa pagina selezionata.
            [many] Tutte le { $count } di conversazioni già lette in questa pagina sono selezionate.
           *[other] Tutte le { $count } conversazioni già lette in questa pagina sono selezionate.
        }
       *[message] { $count ->
            [one] { $count } messaggio già letto in questa pagina selezionato.
            [many] Tutti i { $count } di messaggi già letti in questa pagina sono selezionati.
           *[other] Tutti i { $count } messaggi già letti in questa pagina sono selezionati.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversazione da leggere in questa pagina selezionata.
            [many] Tutte le { $count } di conversazioni da leggere in questa pagina sono selezionate.
           *[other] Tutte le { $count } conversazioni da leggere in questa pagina sono selezionate.
        }
       *[message] { $count ->
            [one] { $count } messaggio da leggere in questa pagina selezionato.
            [many] Tutti i { $count } di messaggi da leggere in questa pagina sono selezionati.
           *[other] Tutti i { $count } messaggi da leggere in questa pagina sono selezionati.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversazione speciale in questa pagina selezionata.
            [many] Tutte le { $count } di conversazioni speciali in questa pagina sono selezionate.
           *[other] Tutte le { $count } conversazioni speciali in questa pagina sono selezionate.
        }
       *[message] { $count ->
            [one] { $count } messaggio speciale in questa pagina selezionato.
            [many] Tutti i { $count } di messaggi speciali in questa pagina sono selezionati.
           *[other] Tutti i { $count } messaggi speciali in questa pagina sono selezionati.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversazione non speciale in questa pagina selezionata.
            [many] Tutte le { $count } di conversazioni non speciali in questa pagina sono selezionate.
           *[other] Tutte le { $count } conversazioni non speciali in questa pagina sono selezionate.
        }
       *[message] { $count ->
            [one] { $count } messaggio non speciale in questa pagina selezionato.
            [many] Tutti i { $count } di messaggi non speciali in questa pagina sono selezionati.
           *[other] Tutti i { $count } messaggi non speciali in questa pagina sono selezionati.
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Seleziona { $count } conversazione già letta
            [many] Seleziona tutte le { $count } di conversazioni già lette
           *[other] Seleziona tutte le { $count } conversazioni già lette
        }
       *[message] { $count ->
            [one] Seleziona { $count } messaggio già letto
            [many] Seleziona tutti i { $count } di messaggi già letti
           *[other] Seleziona tutti i { $count } messaggi già letti
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Seleziona { $count } conversazione da leggere
            [many] Seleziona tutte le { $count } di conversazioni da leggere
           *[other] Seleziona tutte le { $count } conversazioni da leggere
        }
       *[message] { $count ->
            [one] Seleziona { $count } messaggio da leggere
            [many] Seleziona tutti i { $count } di messaggi da leggere
           *[other] Seleziona tutti i { $count } messaggi da leggere
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Seleziona { $count } conversazione speciale
            [many] Seleziona tutte le { $count } di conversazioni speciali
           *[other] Seleziona tutte le { $count } conversazioni speciali
        }
       *[message] { $count ->
            [one] Seleziona { $count } messaggio speciale
            [many] Seleziona tutti i { $count } di messaggi speciali
           *[other] Seleziona tutti i { $count } messaggi speciali
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Seleziona { $count } conversazione non speciale
            [many] Seleziona tutte le { $count } di conversazioni non speciali
           *[other] Seleziona tutte le { $count } conversazioni non speciali
        }
       *[message] { $count ->
            [one] Seleziona { $count } messaggio non speciale
            [many] Seleziona tutti i { $count } di messaggi non speciali
           *[other] Seleziona tutti i { $count } messaggi non speciali
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Seleziona { $count } conversazione già letta in { $folder }
            [many] Seleziona tutte le { $count } di conversazioni già lette in { $folder }
           *[other] Seleziona tutte le { $count } conversazioni già lette in { $folder }
        }
       *[message] { $count ->
            [one] Seleziona { $count } messaggio già letto in { $folder }
            [many] Seleziona tutti i { $count } di messaggi già letti in { $folder }
           *[other] Seleziona tutti i { $count } messaggi già letti in { $folder }
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Seleziona { $count } conversazione da leggere in { $folder }
            [many] Seleziona tutte le { $count } di conversazioni da leggere in { $folder }
           *[other] Seleziona tutte le { $count } conversazioni da leggere in { $folder }
        }
       *[message] { $count ->
            [one] Seleziona { $count } messaggio da leggere in { $folder }
            [many] Seleziona tutti i { $count } di messaggi da leggere in { $folder }
           *[other] Seleziona tutti i { $count } messaggi da leggere in { $folder }
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Seleziona { $count } conversazione speciale in { $folder }
            [many] Seleziona tutte le { $count } di conversazioni speciali in { $folder }
           *[other] Seleziona tutte le { $count } conversazioni speciali in { $folder }
        }
       *[message] { $count ->
            [one] Seleziona { $count } messaggio speciale in { $folder }
            [many] Seleziona tutti i { $count } di messaggi speciali in { $folder }
           *[other] Seleziona tutti i { $count } messaggi speciali in { $folder }
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Seleziona { $count } conversazione non speciale in { $folder }
            [many] Seleziona tutte le { $count } di conversazioni non speciali in { $folder }
           *[other] Seleziona tutte le { $count } conversazioni non speciali in { $folder }
        }
       *[message] { $count ->
            [one] Seleziona { $count } messaggio non speciale in { $folder }
            [many] Seleziona tutti i { $count } di messaggi non speciali in { $folder }
           *[other] Seleziona tutti i { $count } messaggi non speciali in { $folder }
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversazione già letta selezionata.
            [many] Tutte le { $count } di conversazioni già lette sono selezionate.
           *[other] Tutte le { $count } conversazioni già lette sono selezionate.
        }
       *[message] { $count ->
            [one] { $count } messaggio già letto selezionato.
            [many] Tutti i { $count } di messaggi già letti sono selezionati.
           *[other] Tutti i { $count } messaggi già letti sono selezionati.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversazione da leggere selezionata.
            [many] Tutte le { $count } di conversazioni da leggere sono selezionate.
           *[other] Tutte le { $count } conversazioni da leggere sono selezionate.
        }
       *[message] { $count ->
            [one] { $count } messaggio da leggere selezionato.
            [many] Tutti i { $count } di messaggi da leggere sono selezionati.
           *[other] Tutti i { $count } messaggi da leggere sono selezionati.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversazione speciale selezionata.
            [many] Tutte le { $count } di conversazioni speciali sono selezionate.
           *[other] Tutte le { $count } conversazioni speciali sono selezionate.
        }
       *[message] { $count ->
            [one] { $count } messaggio speciale selezionato.
            [many] Tutti i { $count } di messaggi speciali sono selezionati.
           *[other] Tutti i { $count } messaggi speciali sono selezionati.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversazione non speciale selezionata.
            [many] Tutte le { $count } di conversazioni non speciali sono selezionate.
           *[other] Tutte le { $count } conversazioni non speciali sono selezionate.
        }
       *[message] { $count ->
            [one] { $count } messaggio non speciale selezionato.
            [many] Tutti i { $count } di messaggi non speciali sono selezionati.
           *[other] Tutti i { $count } messaggi non speciali sono selezionati.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversazione già letta in { $folder } selezionata.
            [many] Tutte le { $count } di conversazioni già lette in { $folder } sono selezionate.
           *[other] Tutte le { $count } conversazioni già lette in { $folder } sono selezionate.
        }
       *[message] { $count ->
            [one] { $count } messaggio già letto in { $folder } selezionato.
            [many] Tutti i { $count } di messaggi già letti in { $folder } sono selezionati.
           *[other] Tutti i { $count } messaggi già letti in { $folder } sono selezionati.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversazione da leggere in { $folder } selezionata.
            [many] Tutte le { $count } di conversazioni da leggere in { $folder } sono selezionate.
           *[other] Tutte le { $count } conversazioni da leggere in { $folder } sono selezionate.
        }
       *[message] { $count ->
            [one] { $count } messaggio da leggere in { $folder } selezionato.
            [many] Tutti i { $count } di messaggi da leggere in { $folder } sono selezionati.
           *[other] Tutti i { $count } messaggi da leggere in { $folder } sono selezionati.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversazione speciale in { $folder } selezionata.
            [many] Tutte le { $count } di conversazioni speciali in { $folder } sono selezionate.
           *[other] Tutte le { $count } conversazioni speciali in { $folder } sono selezionate.
        }
       *[message] { $count ->
            [one] { $count } messaggio speciale in { $folder } selezionato.
            [many] Tutti i { $count } di messaggi speciali in { $folder } sono selezionati.
           *[other] Tutti i { $count } messaggi speciali in { $folder } sono selezionati.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversazione non speciale in { $folder } selezionata.
            [many] Tutte le { $count } di conversazioni non speciali in { $folder } sono selezionate.
           *[other] Tutte le { $count } conversazioni non speciali in { $folder } sono selezionate.
        }
       *[message] { $count ->
            [one] { $count } messaggio non speciale in { $folder } selezionato.
            [many] Tutti i { $count } di messaggi non speciali in { $folder } sono selezionati.
           *[other] Tutti i { $count } messaggi non speciali in { $folder } sono selezionati.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Nessuna conversazione già letta qui.
       *[message] Nessun messaggio già letto qui.
    }
   *[unread] { $kind ->
        [conversation] Nessuna conversazione da leggere qui.
       *[message] Nessun messaggio da leggere qui.
    }
    [starred] { $kind ->
        [conversation] Nessuna conversazione speciale qui.
       *[message] Nessun messaggio speciale qui.
    }
    [unstarred] { $kind ->
        [conversation] Nessuna conversazione non speciale qui.
       *[message] Nessun messaggio non speciale qui.
    }
}
list-clear-selection = Annulla selezione

## Mail list: empty states

list-empty-search = Nessun messaggio corrisponde alla ricerca.
list-empty-tab = Nessun messaggio in { $tab }.
list-empty-tab-unknown = Nessun messaggio in questa scheda.
list-empty-folder = Nessun messaggio in { $folder }.
list-empty-folder-unknown = Nessun messaggio in questa cartella.
list-first-sync = Recupero della posta…
list-first-sync-detail = I messaggi compaiono qui man mano che arrivano.

## Mail list: lines

row-removed = Questo messaggio è stato rimosso.
row-starred = Speciale
row-not-starred = Non speciale
row-important = Importante. Fai clic per contrassegnare come non importante.
row-mark-important = Contrassegna come importante
row-pinned = Fissato in alto
row-task = Attività
row-task-open = Apri l'attività: { $title }
row-tracking-none = Tracciato. Non ancora aperto
row-tracking-opened = Aperto da { $opened } su { $recipients }
row-tracking-clicked = Aperto da { $opened } su { $recipients }, un link seguito da { $clicked }
row-pin = Fissa in alto
row-unpin = Sblocca
row-snoozed-until = Posticipato a { $when }

## Mail list: More menu and right-click menu

menu-reply = Rispondi
menu-reply-all = Rispondi a tutti
menu-forward = Inoltra
menu-archive = Archivia
menu-delete = Elimina
menu-delete-forever = Elimina definitivamente
menu-move-to-inbox = Sposta in Posta in arrivo
menu-spam = Segnala come spam
menu-not-spam = Non è spam
menu-mark-read = Segna come già letto
menu-mark-unread = Segna come da leggere
menu-mark-all-read = Segna tutti come già letti
menu-star = Aggiungi a Speciali
menu-unstar = Rimuovi da Speciali
menu-important = Contrassegna come importante
menu-not-important = Contrassegna come non importante
menu-pin = Fissa in alto
menu-unpin = Sblocca
menu-snooze = Posticipa
menu-unsnooze = Annulla posticipo
menu-add-to-tasks = Aggiungi ad Attività
menu-schedule-meeting = Pianifica una riunione
menu-start-call = Avvia una videochiamata
menu-add-note = Aggiungi una nota
menu-print-all = Stampa tutto
menu-new-window = Apri in una nuova finestra
menu-move-to = Sposta in
# Opens a submenu: Add to Tasks, Add a note, Schedule a meeting and Start a
# video call.
menu-follow-up = Dai seguito
# Opens a submenu of the rarer actions: Report spam, Mark as important and
# Pin to top.
menu-more = Altro
menu-move-to-heading = Sposta in:
menu-find-from = Trova email da { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Conversazione archiviata.
        [many] { $count } di conversazioni archiviate.
       *[other] { $count } conversazioni archiviate.
    }
   *[message] { $count ->
        [one] Messaggio archiviato.
        [many] { $count } di messaggi archiviati.
       *[other] { $count } messaggi archiviati.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Conversazione spostata nel Cestino.
        [many] { $count } di conversazioni spostate nel Cestino.
       *[other] { $count } conversazioni spostate nel Cestino.
    }
   *[message] { $count ->
        [one] Messaggio spostato nel Cestino.
        [many] { $count } di messaggi spostati nel Cestino.
       *[other] { $count } messaggi spostati nel Cestino.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Conversazione spostata.
        [many] { $count } di conversazioni spostate.
       *[other] { $count } conversazioni spostate.
    }
   *[message] { $count ->
        [one] Messaggio spostato.
        [many] { $count } di messaggi spostati.
       *[other] { $count } messaggi spostati.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Conversazione aggiunta a Speciali.
        [many] { $count } di conversazioni aggiunte a Speciali.
       *[other] { $count } conversazioni aggiunte a Speciali.
    }
   *[message] { $count ->
        [one] Messaggio aggiunto a Speciali.
        [many] { $count } di messaggi aggiunti a Speciali.
       *[other] { $count } messaggi aggiunti a Speciali.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Conversazione rimossa da Speciali.
        [many] { $count } di conversazioni rimosse da Speciali.
       *[other] { $count } conversazioni rimosse da Speciali.
    }
   *[message] { $count ->
        [one] Messaggio rimosso da Speciali.
        [many] { $count } di messaggi rimossi da Speciali.
       *[other] { $count } messaggi rimossi da Speciali.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Conversazione contrassegnata come importante.
        [many] { $count } di conversazioni contrassegnate come importanti.
       *[other] { $count } conversazioni contrassegnate come importanti.
    }
   *[message] { $count ->
        [one] Messaggio contrassegnato come importante.
        [many] { $count } di messaggi contrassegnati come importanti.
       *[other] { $count } messaggi contrassegnati come importanti.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Conversazione contrassegnata come non importante.
        [many] { $count } di conversazioni contrassegnate come non importanti.
       *[other] { $count } conversazioni contrassegnate come non importanti.
    }
   *[message] { $count ->
        [one] Messaggio contrassegnato come non importante.
        [many] { $count } di messaggi contrassegnati come non importanti.
       *[other] { $count } messaggi contrassegnati come non importanti.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Conversazione fissata in alto.
        [many] { $count } di conversazioni fissate in alto.
       *[other] { $count } conversazioni fissate in alto.
    }
   *[message] { $count ->
        [one] Messaggio fissato in alto.
        [many] { $count } di messaggi fissati in alto.
       *[other] { $count } messaggi fissati in alto.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Conversazione sbloccata.
        [many] { $count } di conversazioni sbloccate.
       *[other] { $count } conversazioni sbloccate.
    }
   *[message] { $count ->
        [one] Messaggio sbloccato.
        [many] { $count } di messaggi sbloccati.
       *[other] { $count } messaggi sbloccati.
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] Conversazione posticipata a { $when }.
        [many] { $count } di conversazioni posticipate a { $when }.
       *[other] { $count } conversazioni posticipate a { $when }.
    }
   *[message] { $count ->
        [one] Messaggio posticipato a { $when }.
        [many] { $count } di messaggi posticipati a { $when }.
       *[other] { $count } messaggi posticipati a { $when }.
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] Conversazione tornata in Posta in arrivo.
        [many] { $count } di conversazioni tornate in Posta in arrivo.
       *[other] { $count } conversazioni tornate in Posta in arrivo.
    }
   *[message] { $count ->
        [one] Messaggio tornato in Posta in arrivo.
        [many] { $count } di messaggi tornati in Posta in arrivo.
       *[other] { $count } messaggi tornati in Posta in arrivo.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Conversazione segnalata come spam.
        [many] { $count } di conversazioni segnalate come spam.
       *[other] { $count } conversazioni segnalate come spam.
    }
   *[message] { $count ->
        [one] Messaggio segnalato come spam.
        [many] { $count } di messaggi segnalati come spam.
       *[other] { $count } messaggi segnalati come spam.
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] Conversazione segnalata come non spam e spostata in Posta in arrivo.
        [many] { $count } di conversazioni segnalate come non spam e spostate in Posta in arrivo.
       *[other] { $count } conversazioni segnalate come non spam e spostate in Posta in arrivo.
    }
   *[message] { $count ->
        [one] Messaggio segnalato come non spam e spostato in Posta in arrivo.
        [many] { $count } di messaggi segnalati come non spam e spostati in Posta in arrivo.
       *[other] { $count } messaggi segnalati come non spam e spostati in Posta in arrivo.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Conversazione eliminata definitivamente.
        [many] { $count } di conversazioni eliminate definitivamente.
       *[other] { $count } conversazioni eliminate definitivamente.
    }
   *[message] { $count ->
        [one] Messaggio eliminato definitivamente.
        [many] { $count } di messaggi eliminati definitivamente.
       *[other] { $count } messaggi eliminati definitivamente.
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] Conversazione segnata come già letta.
        [many] { $count } di conversazioni segnate come già lette.
       *[other] { $count } conversazioni segnate come già lette.
    }
   *[message] { $count ->
        [one] Messaggio segnato come già letto.
        [many] { $count } di messaggi segnati come già letti.
       *[other] { $count } messaggi segnati come già letti.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] Conversazione segnata come da leggere.
        [many] { $count } di conversazioni segnate come da leggere.
       *[other] { $count } conversazioni segnate come da leggere.
    }
   *[message] { $count ->
        [one] Messaggio segnato come da leggere.
        [many] { $count } di messaggi segnati come da leggere.
       *[other] { $count } messaggi segnati come da leggere.
    }
}
toast-undone = Azione annullata.
toast-nothing-to-undo = Niente da annullare.
toast-cannot-undo-delete-forever = La posta eliminata definitivamente non si può recuperare.
toast-send-undone = Invio annullato.
toast-too-late-to-undo-send = Troppo tardi per annullare: il messaggio è già stato inviato.
toast-undo = Annulla
toast-close = Chiudi
toast-no-spam-folder = Questo account non ha una cartella Spam.
