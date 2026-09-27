# Katna Mail, Swedish (Svenska).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Primär
tab-promotions = Kampanjer
tab-social = Socialt
tab-updates = Uppdateringar
tab-forums = Forum
tab-focused = Prioriterat
tab-other = Övrigt
tab-inbox = Inkorgen
tab-newsletters = Nyhetsbrev
tab-notifications = Aviseringar
tab-new = { $count ->
    [one] { $count } nytt
   *[other] { $count } nya
}
tab-provider-other = sorteras av Katna

## Mail list: toolbar

list-select = Markera
list-refresh = Uppdatera
list-more = Mer
list-mark-read = Markera som läst
list-mark-unread = Markera som oläst
list-move-to = Flytta till
list-archive = Arkivera
list-spam = Rapportera som skräppost
list-delete = Radera
list-snooze = Snooza
list-unsnooze = Avbryt snooze
list-newer = Nyare
list-older = Äldre
list-range = { $first }–{ $last } av { $total }
list-range-about = { $first }–{ $last } av cirka { $total }
list-results = Resultat för ”{ $query }”
list-results-corrected = Visar resultat för ”{ $query }”
list-search-instead = Sök i stället efter ”{ $query }”
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Alla
list-pick-none = Inga
list-pick-read = Lästa
list-pick-unread = Olästa
list-pick-starred = Stjärnmärkta
list-pick-unstarred = Utan stjärna

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } konversation har markerats.
       *[other] Alla { $count } konversationer har markerats.
    }
   *[message] { $count ->
        [one] { $count } meddelande har markerats.
       *[other] Alla { $count } meddelanden har markerats.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $count } konversation i { $folder } har markerats.
       *[other] Alla { $count } konversationer i { $folder } har markerats.
    }
   *[message] { $count ->
        [one] { $count } meddelande i { $folder } har markerats.
       *[other] Alla { $count } meddelanden i { $folder } har markerats.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] { $count } konversation på den här sidan har markerats.
       *[other] Alla { $count } konversationer på den här sidan har markerats.
    }
   *[message] { $count ->
        [one] { $count } meddelande på den här sidan har markerats.
       *[other] Alla { $count } meddelanden på den här sidan har markerats.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Markera { $count } konversation
       *[other] Markera alla { $count } konversationer
    }
   *[message] { $count ->
        [one] Markera { $count } meddelande
       *[other] Markera alla { $count } meddelanden
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Markera { $count } konversation i { $folder }
       *[other] Markera alla { $count } konversationer i { $folder }
    }
   *[message] { $count ->
        [one] Markera { $count } meddelande i { $folder }
       *[other] Markera alla { $count } meddelanden i { $folder }
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } läst konversation på den här sidan har markerats.
           *[other] Alla { $count } lästa konversationer på den här sidan har markerats.
        }
       *[message] { $count ->
            [one] { $count } läst meddelande på den här sidan har markerats.
           *[other] Alla { $count } lästa meddelanden på den här sidan har markerats.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } oläst konversation på den här sidan har markerats.
           *[other] Alla { $count } olästa konversationer på den här sidan har markerats.
        }
       *[message] { $count ->
            [one] { $count } oläst meddelande på den här sidan har markerats.
           *[other] Alla { $count } olästa meddelanden på den här sidan har markerats.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } stjärnmärkt konversation på den här sidan har markerats.
           *[other] Alla { $count } stjärnmärkta konversationer på den här sidan har markerats.
        }
       *[message] { $count ->
            [one] { $count } stjärnmärkt meddelande på den här sidan har markerats.
           *[other] Alla { $count } stjärnmärkta meddelanden på den här sidan har markerats.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } konversation utan stjärna på den här sidan har markerats.
           *[other] Alla { $count } konversationer utan stjärna på den här sidan har markerats.
        }
       *[message] { $count ->
            [one] { $count } meddelande utan stjärna på den här sidan har markerats.
           *[other] Alla { $count } meddelanden utan stjärna på den här sidan har markerats.
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Markera { $count } läst konversation
           *[other] Markera alla { $count } lästa konversationer
        }
       *[message] { $count ->
            [one] Markera { $count } läst meddelande
           *[other] Markera alla { $count } lästa meddelanden
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Markera { $count } oläst konversation
           *[other] Markera alla { $count } olästa konversationer
        }
       *[message] { $count ->
            [one] Markera { $count } oläst meddelande
           *[other] Markera alla { $count } olästa meddelanden
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Markera { $count } stjärnmärkt konversation
           *[other] Markera alla { $count } stjärnmärkta konversationer
        }
       *[message] { $count ->
            [one] Markera { $count } stjärnmärkt meddelande
           *[other] Markera alla { $count } stjärnmärkta meddelanden
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Markera { $count } konversation utan stjärna
           *[other] Markera alla { $count } konversationer utan stjärna
        }
       *[message] { $count ->
            [one] Markera { $count } meddelande utan stjärna
           *[other] Markera alla { $count } meddelanden utan stjärna
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Markera { $count } läst konversation i { $folder }
           *[other] Markera alla { $count } lästa konversationer i { $folder }
        }
       *[message] { $count ->
            [one] Markera { $count } läst meddelande i { $folder }
           *[other] Markera alla { $count } lästa meddelanden i { $folder }
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Markera { $count } oläst konversation i { $folder }
           *[other] Markera alla { $count } olästa konversationer i { $folder }
        }
       *[message] { $count ->
            [one] Markera { $count } oläst meddelande i { $folder }
           *[other] Markera alla { $count } olästa meddelanden i { $folder }
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Markera { $count } stjärnmärkt konversation i { $folder }
           *[other] Markera alla { $count } stjärnmärkta konversationer i { $folder }
        }
       *[message] { $count ->
            [one] Markera { $count } stjärnmärkt meddelande i { $folder }
           *[other] Markera alla { $count } stjärnmärkta meddelanden i { $folder }
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Markera { $count } konversation utan stjärna i { $folder }
           *[other] Markera alla { $count } konversationer utan stjärna i { $folder }
        }
       *[message] { $count ->
            [one] Markera { $count } meddelande utan stjärna i { $folder }
           *[other] Markera alla { $count } meddelanden utan stjärna i { $folder }
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } läst konversation är markerad.
           *[other] Alla { $count } lästa konversationer är markerade.
        }
       *[message] { $count ->
            [one] { $count } läst meddelande är markerat.
           *[other] Alla { $count } lästa meddelanden är markerade.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } oläst konversation är markerad.
           *[other] Alla { $count } olästa konversationer är markerade.
        }
       *[message] { $count ->
            [one] { $count } oläst meddelande är markerat.
           *[other] Alla { $count } olästa meddelanden är markerade.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } stjärnmärkt konversation är markerad.
           *[other] Alla { $count } stjärnmärkta konversationer är markerade.
        }
       *[message] { $count ->
            [one] { $count } stjärnmärkt meddelande är markerat.
           *[other] Alla { $count } stjärnmärkta meddelanden är markerade.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } konversation utan stjärna är markerad.
           *[other] Alla { $count } konversationer utan stjärna är markerade.
        }
       *[message] { $count ->
            [one] { $count } meddelande utan stjärna är markerat.
           *[other] Alla { $count } meddelanden utan stjärna är markerade.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } läst konversation i { $folder } är markerad.
           *[other] Alla { $count } lästa konversationer i { $folder } är markerade.
        }
       *[message] { $count ->
            [one] { $count } läst meddelande i { $folder } är markerat.
           *[other] Alla { $count } lästa meddelanden i { $folder } är markerade.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } oläst konversation i { $folder } är markerad.
           *[other] Alla { $count } olästa konversationer i { $folder } är markerade.
        }
       *[message] { $count ->
            [one] { $count } oläst meddelande i { $folder } är markerat.
           *[other] Alla { $count } olästa meddelanden i { $folder } är markerade.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } stjärnmärkt konversation i { $folder } är markerad.
           *[other] Alla { $count } stjärnmärkta konversationer i { $folder } är markerade.
        }
       *[message] { $count ->
            [one] { $count } stjärnmärkt meddelande i { $folder } är markerat.
           *[other] Alla { $count } stjärnmärkta meddelanden i { $folder } är markerade.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } konversation utan stjärna i { $folder } är markerad.
           *[other] Alla { $count } konversationer utan stjärna i { $folder } är markerade.
        }
       *[message] { $count ->
            [one] { $count } meddelande utan stjärna i { $folder } är markerat.
           *[other] Alla { $count } meddelanden utan stjärna i { $folder } är markerade.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Inga lästa konversationer här.
       *[message] Inga lästa meddelanden här.
    }
   *[unread] { $kind ->
        [conversation] Inga olästa konversationer här.
       *[message] Inga olästa meddelanden här.
    }
    [starred] { $kind ->
        [conversation] Inga stjärnmärkta konversationer här.
       *[message] Inga stjärnmärkta meddelanden här.
    }
    [unstarred] { $kind ->
        [conversation] Inga konversationer utan stjärna här.
       *[message] Inga meddelanden utan stjärna här.
    }
}
list-clear-selection = Rensa markering

## Mail list: empty states

list-empty-search = Inga meddelanden matchade sökningen.
list-empty-tab = Ingen e-post i { $tab }.
list-empty-tab-unknown = Ingen e-post på den här fliken.
list-empty-folder = Inga meddelanden i { $folder }.
list-empty-folder-unknown = Inga meddelanden i den här mappen.
list-first-sync = Hämtar din e-post…
list-first-sync-detail = Den visas här allt eftersom den kommer in.

## Mail list: lines

row-removed = Meddelandet har tagits bort.
row-starred = Stjärnmärkt
row-not-starred = Inte stjärnmärkt
row-important = Viktigt. Klicka för att markera som inte viktigt.
row-mark-important = Markera som viktigt
row-pinned = Fäst högst upp
row-tracking-none = Spårat. Inte öppnat än
row-tracking-opened = { $opened } av { $recipients } har öppnat
row-tracking-clicked = { $opened } av { $recipients } har öppnat, { $clicked } har följt en länk
row-pin = Fäst högst upp
row-unpin = Lossa
row-snoozed-until = Snoozad till { $when }

## Mail list: More menu and right-click menu

menu-reply = Svara
menu-reply-all = Svara alla
menu-forward = Vidarebefordra
menu-archive = Arkivera
menu-delete = Radera
menu-delete-forever = Radera permanent
menu-move-to-inbox = Flytta till Inkorgen
menu-spam = Rapportera som skräppost
menu-not-spam = Inte skräppost
menu-mark-read = Markera som läst
menu-mark-unread = Markera som oläst
menu-mark-all-read = Markera alla som lästa
menu-star = Lägg till stjärna
menu-unstar = Ta bort stjärna
menu-important = Markera som viktigt
menu-not-important = Markera som inte viktigt
menu-pin = Fäst högst upp
menu-unpin = Lossa
menu-snooze = Snooza
menu-unsnooze = Avbryt snooze
menu-print-all = Skriv ut alla
menu-new-window = Öppna i nytt fönster
menu-move-to = Flytta till
menu-move-to-heading = Flytta till:
menu-find-from = Hitta e-post från { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har arkiverats.
       *[other] { $count } konversationer har arkiverats.
    }
   *[message] { $count ->
        [one] Meddelandet har arkiverats.
       *[other] { $count } meddelanden har arkiverats.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har flyttats till papperskorgen.
       *[other] { $count } konversationer har flyttats till papperskorgen.
    }
   *[message] { $count ->
        [one] Meddelandet har flyttats till papperskorgen.
       *[other] { $count } meddelanden har flyttats till papperskorgen.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har flyttats.
       *[other] { $count } konversationer har flyttats.
    }
   *[message] { $count ->
        [one] Meddelandet har flyttats.
       *[other] { $count } meddelanden har flyttats.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har stjärnmärkts.
       *[other] { $count } konversationer har stjärnmärkts.
    }
   *[message] { $count ->
        [one] Meddelandet har stjärnmärkts.
       *[other] { $count } meddelanden har stjärnmärkts.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Stjärnan har tagits bort från konversationen.
       *[other] Stjärnan har tagits bort från { $count } konversationer.
    }
   *[message] { $count ->
        [one] Stjärnan har tagits bort från meddelandet.
       *[other] Stjärnan har tagits bort från { $count } meddelanden.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har markerats som viktig.
       *[other] { $count } konversationer har markerats som viktiga.
    }
   *[message] { $count ->
        [one] Meddelandet har markerats som viktigt.
       *[other] { $count } meddelanden har markerats som viktiga.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har markerats som inte viktig.
       *[other] { $count } konversationer har markerats som inte viktiga.
    }
   *[message] { $count ->
        [one] Meddelandet har markerats som inte viktigt.
       *[other] { $count } meddelanden har markerats som inte viktiga.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har fästs högst upp.
       *[other] { $count } konversationer har fästs högst upp.
    }
   *[message] { $count ->
        [one] Meddelandet har fästs högst upp.
       *[other] { $count } meddelanden har fästs högst upp.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har lossats.
       *[other] { $count } konversationer har lossats.
    }
   *[message] { $count ->
        [one] Meddelandet har lossats.
       *[other] { $count } meddelanden har lossats.
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har snoozats till { $when }.
       *[other] { $count } konversationer har snoozats till { $when }.
    }
   *[message] { $count ->
        [one] Meddelandet har snoozats till { $when }.
       *[other] { $count } meddelanden har snoozats till { $when }.
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] Konversationen är tillbaka i Inkorgen.
       *[other] { $count } konversationer är tillbaka i Inkorgen.
    }
   *[message] { $count ->
        [one] Meddelandet är tillbaka i Inkorgen.
       *[other] { $count } meddelanden är tillbaka i Inkorgen.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har rapporterats som skräppost.
       *[other] { $count } konversationer har rapporterats som skräppost.
    }
   *[message] { $count ->
        [one] Meddelandet har rapporterats som skräppost.
       *[other] { $count } meddelanden har rapporterats som skräppost.
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har markerats som inte skräppost och flyttats till inkorgen.
       *[other] { $count } konversationer har markerats som inte skräppost och flyttats till inkorgen.
    }
   *[message] { $count ->
        [one] Meddelandet har markerats som inte skräppost och flyttats till inkorgen.
       *[other] { $count } meddelanden har markerats som inte skräppost och flyttats till inkorgen.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har raderats permanent.
       *[other] { $count } konversationer har raderats permanent.
    }
   *[message] { $count ->
        [one] Meddelandet har raderats permanent.
       *[other] { $count } meddelanden har raderats permanent.
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har markerats som läst.
       *[other] { $count } konversationer har markerats som lästa.
    }
   *[message] { $count ->
        [one] Meddelandet har markerats som läst.
       *[other] { $count } meddelanden har markerats som lästa.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] Konversationen har markerats som oläst.
       *[other] { $count } konversationer har markerats som olästa.
    }
   *[message] { $count ->
        [one] Meddelandet har markerats som oläst.
       *[other] { $count } meddelanden har markerats som olästa.
    }
}
toast-undone = Åtgärden har ångrats.
toast-nothing-to-undo = Inget att ångra.
toast-cannot-undo-delete-forever = E-post som har raderats permanent kan inte återställas.
toast-send-undone = Skickandet har ångrats.
toast-too-late-to-undo-send = För sent att ångra: meddelandet har redan skickats.
toast-undo = Ångra
toast-no-spam-folder = Det här kontot har ingen skräppostmapp.
