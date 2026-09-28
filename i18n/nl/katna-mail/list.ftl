# Katna Mail, Dutch (Nederlands).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Primair
tab-promotions = Reclame
tab-social = Sociaal
tab-updates = Updates
tab-forums = Forums
tab-focused = Prioriteit
tab-other = Overige
tab-inbox = Inbox
tab-newsletters = Nieuwsbrieven
tab-notifications = Meldingen
tab-new = { $count } nieuw
tab-provider-other = gesorteerd door Katna

## Mail list: toolbar

list-select = Selecteren
list-refresh = Vernieuwen
list-more = Meer
list-mark-read = Markeren als gelezen
list-mark-unread = Markeren als ongelezen
list-move-to = Verplaatsen naar
list-archive = Archiveren
list-spam = Spam melden
list-delete = Verwijderen
list-snooze = Snoozen
list-unsnooze = Snooze opheffen
list-newer = Nieuwer
list-older = Ouder
list-range = { $first }–{ $last } van { $total }
list-range-about = { $first }–{ $last } van ongeveer { $total }
list-results = Resultaten voor ‘{ $query }’
list-results-corrected = Resultaten weergegeven voor ‘{ $query }’
list-search-instead = In plaats daarvan zoeken naar ‘{ $query }’
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Alle
list-pick-none = Geen
list-pick-read = Gelezen
list-pick-unread = Ongelezen
list-pick-starred = Met ster
list-pick-unstarred = Zonder ster

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } gesprek is geselecteerd.
       *[other] Alle { $count } gesprekken zijn geselecteerd.
    }
   *[message] { $count ->
        [one] { $count } bericht is geselecteerd.
       *[other] Alle { $count } berichten zijn geselecteerd.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $count } gesprek in { $folder } is geselecteerd.
       *[other] Alle { $count } gesprekken in { $folder } zijn geselecteerd.
    }
   *[message] { $count ->
        [one] { $count } bericht in { $folder } is geselecteerd.
       *[other] Alle { $count } berichten in { $folder } zijn geselecteerd.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] { $count } gesprek op deze pagina is geselecteerd.
       *[other] Alle { $count } gesprekken op deze pagina zijn geselecteerd.
    }
   *[message] { $count ->
        [one] { $count } bericht op deze pagina is geselecteerd.
       *[other] Alle { $count } berichten op deze pagina zijn geselecteerd.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } gesprek selecteren
       *[other] Alle { $count } gesprekken selecteren
    }
   *[message] { $count ->
        [one] { $count } bericht selecteren
       *[other] Alle { $count } berichten selecteren
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $count } gesprek in { $folder } selecteren
       *[other] Alle { $count } gesprekken in { $folder } selecteren
    }
   *[message] { $count ->
        [one] { $count } bericht in { $folder } selecteren
       *[other] Alle { $count } berichten in { $folder } selecteren
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } gelezen gesprek op deze pagina is geselecteerd.
           *[other] Alle { $count } gelezen gesprekken op deze pagina zijn geselecteerd.
        }
       *[message] { $count ->
            [one] { $count } gelezen bericht op deze pagina is geselecteerd.
           *[other] Alle { $count } gelezen berichten op deze pagina zijn geselecteerd.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } ongelezen gesprek op deze pagina is geselecteerd.
           *[other] Alle { $count } ongelezen gesprekken op deze pagina zijn geselecteerd.
        }
       *[message] { $count ->
            [one] { $count } ongelezen bericht op deze pagina is geselecteerd.
           *[other] Alle { $count } ongelezen berichten op deze pagina zijn geselecteerd.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } gesprek met ster op deze pagina is geselecteerd.
           *[other] Alle { $count } gesprekken met ster op deze pagina zijn geselecteerd.
        }
       *[message] { $count ->
            [one] { $count } bericht met ster op deze pagina is geselecteerd.
           *[other] Alle { $count } berichten met ster op deze pagina zijn geselecteerd.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } gesprek zonder ster op deze pagina is geselecteerd.
           *[other] Alle { $count } gesprekken zonder ster op deze pagina zijn geselecteerd.
        }
       *[message] { $count ->
            [one] { $count } bericht zonder ster op deze pagina is geselecteerd.
           *[other] Alle { $count } berichten zonder ster op deze pagina zijn geselecteerd.
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } gelezen gesprek selecteren
           *[other] Alle { $count } gelezen gesprekken selecteren
        }
       *[message] { $count ->
            [one] { $count } gelezen bericht selecteren
           *[other] Alle { $count } gelezen berichten selecteren
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } ongelezen gesprek selecteren
           *[other] Alle { $count } ongelezen gesprekken selecteren
        }
       *[message] { $count ->
            [one] { $count } ongelezen bericht selecteren
           *[other] Alle { $count } ongelezen berichten selecteren
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } gesprek met ster selecteren
           *[other] Alle { $count } gesprekken met ster selecteren
        }
       *[message] { $count ->
            [one] { $count } bericht met ster selecteren
           *[other] Alle { $count } berichten met ster selecteren
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } gesprek zonder ster selecteren
           *[other] Alle { $count } gesprekken zonder ster selecteren
        }
       *[message] { $count ->
            [one] { $count } bericht zonder ster selecteren
           *[other] Alle { $count } berichten zonder ster selecteren
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } gelezen gesprek in { $folder } selecteren
           *[other] Alle { $count } gelezen gesprekken in { $folder } selecteren
        }
       *[message] { $count ->
            [one] { $count } gelezen bericht in { $folder } selecteren
           *[other] Alle { $count } gelezen berichten in { $folder } selecteren
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } ongelezen gesprek in { $folder } selecteren
           *[other] Alle { $count } ongelezen gesprekken in { $folder } selecteren
        }
       *[message] { $count ->
            [one] { $count } ongelezen bericht in { $folder } selecteren
           *[other] Alle { $count } ongelezen berichten in { $folder } selecteren
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } gesprek met ster in { $folder } selecteren
           *[other] Alle { $count } gesprekken met ster in { $folder } selecteren
        }
       *[message] { $count ->
            [one] { $count } bericht met ster in { $folder } selecteren
           *[other] Alle { $count } berichten met ster in { $folder } selecteren
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } gesprek zonder ster in { $folder } selecteren
           *[other] Alle { $count } gesprekken zonder ster in { $folder } selecteren
        }
       *[message] { $count ->
            [one] { $count } bericht zonder ster in { $folder } selecteren
           *[other] Alle { $count } berichten zonder ster in { $folder } selecteren
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } gelezen gesprek is geselecteerd.
           *[other] Alle { $count } gelezen gesprekken zijn geselecteerd.
        }
       *[message] { $count ->
            [one] { $count } gelezen bericht is geselecteerd.
           *[other] Alle { $count } gelezen berichten zijn geselecteerd.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } ongelezen gesprek is geselecteerd.
           *[other] Alle { $count } ongelezen gesprekken zijn geselecteerd.
        }
       *[message] { $count ->
            [one] { $count } ongelezen bericht is geselecteerd.
           *[other] Alle { $count } ongelezen berichten zijn geselecteerd.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } gesprek met ster is geselecteerd.
           *[other] Alle { $count } gesprekken met ster zijn geselecteerd.
        }
       *[message] { $count ->
            [one] { $count } bericht met ster is geselecteerd.
           *[other] Alle { $count } berichten met ster zijn geselecteerd.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } gesprek zonder ster is geselecteerd.
           *[other] Alle { $count } gesprekken zonder ster zijn geselecteerd.
        }
       *[message] { $count ->
            [one] { $count } bericht zonder ster is geselecteerd.
           *[other] Alle { $count } berichten zonder ster zijn geselecteerd.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } gelezen gesprek in { $folder } is geselecteerd.
           *[other] Alle { $count } gelezen gesprekken in { $folder } zijn geselecteerd.
        }
       *[message] { $count ->
            [one] { $count } gelezen bericht in { $folder } is geselecteerd.
           *[other] Alle { $count } gelezen berichten in { $folder } zijn geselecteerd.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } ongelezen gesprek in { $folder } is geselecteerd.
           *[other] Alle { $count } ongelezen gesprekken in { $folder } zijn geselecteerd.
        }
       *[message] { $count ->
            [one] { $count } ongelezen bericht in { $folder } is geselecteerd.
           *[other] Alle { $count } ongelezen berichten in { $folder } zijn geselecteerd.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } gesprek met ster in { $folder } is geselecteerd.
           *[other] Alle { $count } gesprekken met ster in { $folder } zijn geselecteerd.
        }
       *[message] { $count ->
            [one] { $count } bericht met ster in { $folder } is geselecteerd.
           *[other] Alle { $count } berichten met ster in { $folder } zijn geselecteerd.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } gesprek zonder ster in { $folder } is geselecteerd.
           *[other] Alle { $count } gesprekken zonder ster in { $folder } zijn geselecteerd.
        }
       *[message] { $count ->
            [one] { $count } bericht zonder ster in { $folder } is geselecteerd.
           *[other] Alle { $count } berichten zonder ster in { $folder } zijn geselecteerd.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Geen gelezen gesprekken hier.
       *[message] Geen gelezen berichten hier.
    }
   *[unread] { $kind ->
        [conversation] Geen ongelezen gesprekken hier.
       *[message] Geen ongelezen berichten hier.
    }
    [starred] { $kind ->
        [conversation] Geen gesprekken met ster hier.
       *[message] Geen berichten met ster hier.
    }
    [unstarred] { $kind ->
        [conversation] Geen gesprekken zonder ster hier.
       *[message] Geen berichten zonder ster hier.
    }
}
list-clear-selection = Selectie wissen

## Mail list: empty states

list-empty-search = Er zijn geen berichten die overeenkomen met je zoekopdracht.
list-empty-tab = Geen e-mail in { $tab }.
list-empty-tab-unknown = Geen e-mail op dit tabblad.
list-empty-folder = Geen berichten in { $folder }.
list-empty-folder-unknown = Geen berichten in deze map.
list-first-sync = Je e-mail ophalen…
list-first-sync-detail = Berichten verschijnen hier zodra ze binnenkomen.

## Mail list: lines

row-removed = Dit bericht is verwijderd.
row-starred = Met ster
row-not-starred = Zonder ster
row-important = Belangrijk. Klik om als niet belangrijk te markeren.
row-mark-important = Markeren als belangrijk
row-pinned = Bovenaan vastgezet
row-tracking-none = Gevolgd. Nog niet geopend
row-tracking-opened = Geopend door { $opened } van { $recipients }
row-tracking-clicked = Geopend door { $opened } van { $recipients }, een link gevolgd door { $clicked }
row-pin = Bovenaan vastzetten
row-unpin = Losmaken
row-snoozed-until = Gesnoozed tot { $when }

## Mail list: More menu and right-click menu

menu-reply = Beantwoorden
menu-reply-all = Allen beantwoorden
menu-forward = Doorsturen
menu-archive = Archiveren
menu-delete = Verwijderen
menu-delete-forever = Definitief verwijderen
menu-move-to-inbox = Verplaatsen naar Inbox
menu-spam = Spam melden
menu-not-spam = Geen spam
menu-mark-read = Markeren als gelezen
menu-mark-unread = Markeren als ongelezen
menu-mark-all-read = Alles markeren als gelezen
menu-star = Ster toevoegen
menu-unstar = Ster verwijderen
menu-important = Markeren als belangrijk
menu-not-important = Markeren als niet belangrijk
menu-pin = Bovenaan vastzetten
menu-unpin = Losmaken
menu-snooze = Snoozen
menu-unsnooze = Snooze opheffen
menu-print-all = Alles afdrukken
menu-new-window = Openen in nieuw venster
menu-move-to = Verplaatsen naar
menu-move-to-heading = Verplaatsen naar:
menu-find-from = E-mails van { $name } zoeken

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Gesprek gearchiveerd.
       *[other] { $count } gesprekken gearchiveerd.
    }
   *[message] { $count ->
        [one] Bericht gearchiveerd.
       *[other] { $count } berichten gearchiveerd.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Gesprek naar Prullenbak verplaatst.
       *[other] { $count } gesprekken naar Prullenbak verplaatst.
    }
   *[message] { $count ->
        [one] Bericht naar Prullenbak verplaatst.
       *[other] { $count } berichten naar Prullenbak verplaatst.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Gesprek verplaatst.
       *[other] { $count } gesprekken verplaatst.
    }
   *[message] { $count ->
        [one] Bericht verplaatst.
       *[other] { $count } berichten verplaatst.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Ster toegevoegd aan gesprek.
       *[other] Ster toegevoegd aan { $count } gesprekken.
    }
   *[message] { $count ->
        [one] Ster toegevoegd aan bericht.
       *[other] Ster toegevoegd aan { $count } berichten.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Ster verwijderd van gesprek.
       *[other] Ster verwijderd van { $count } gesprekken.
    }
   *[message] { $count ->
        [one] Ster verwijderd van bericht.
       *[other] Ster verwijderd van { $count } berichten.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Gesprek gemarkeerd als belangrijk.
       *[other] { $count } gesprekken gemarkeerd als belangrijk.
    }
   *[message] { $count ->
        [one] Bericht gemarkeerd als belangrijk.
       *[other] { $count } berichten gemarkeerd als belangrijk.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Gesprek gemarkeerd als niet belangrijk.
       *[other] { $count } gesprekken gemarkeerd als niet belangrijk.
    }
   *[message] { $count ->
        [one] Bericht gemarkeerd als niet belangrijk.
       *[other] { $count } berichten gemarkeerd als niet belangrijk.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Gesprek bovenaan vastgezet.
       *[other] { $count } gesprekken bovenaan vastgezet.
    }
   *[message] { $count ->
        [one] Bericht bovenaan vastgezet.
       *[other] { $count } berichten bovenaan vastgezet.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Gesprek losgemaakt.
       *[other] { $count } gesprekken losgemaakt.
    }
   *[message] { $count ->
        [one] Bericht losgemaakt.
       *[other] { $count } berichten losgemaakt.
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] Gesprek gesnoozed tot { $when }.
       *[other] { $count } gesprekken gesnoozed tot { $when }.
    }
   *[message] { $count ->
        [one] Bericht gesnoozed tot { $when }.
       *[other] { $count } berichten gesnoozed tot { $when }.
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] Gesprek terug in de Inbox.
       *[other] { $count } gesprekken terug in de Inbox.
    }
   *[message] { $count ->
        [one] Bericht terug in de Inbox.
       *[other] { $count } berichten terug in de Inbox.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Gesprek gemeld als spam.
       *[other] { $count } gesprekken gemeld als spam.
    }
   *[message] { $count ->
        [one] Bericht gemeld als spam.
       *[other] { $count } berichten gemeld als spam.
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] Gesprek gemarkeerd als geen spam en naar de inbox verplaatst.
       *[other] { $count } gesprekken gemarkeerd als geen spam en naar de inbox verplaatst.
    }
   *[message] { $count ->
        [one] Bericht gemarkeerd als geen spam en naar de inbox verplaatst.
       *[other] { $count } berichten gemarkeerd als geen spam en naar de inbox verplaatst.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Gesprek definitief verwijderd.
       *[other] { $count } gesprekken definitief verwijderd.
    }
   *[message] { $count ->
        [one] Bericht definitief verwijderd.
       *[other] { $count } berichten definitief verwijderd.
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] Gesprek gemarkeerd als gelezen.
       *[other] { $count } gesprekken gemarkeerd als gelezen.
    }
   *[message] { $count ->
        [one] Bericht gemarkeerd als gelezen.
       *[other] { $count } berichten gemarkeerd als gelezen.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] Gesprek gemarkeerd als ongelezen.
       *[other] { $count } gesprekken gemarkeerd als ongelezen.
    }
   *[message] { $count ->
        [one] Bericht gemarkeerd als ongelezen.
       *[other] { $count } berichten gemarkeerd als ongelezen.
    }
}
toast-undone = Actie ongedaan gemaakt.
toast-nothing-to-undo = Niets om ongedaan te maken.
toast-cannot-undo-delete-forever = Definitief verwijderde e-mail kan niet worden teruggehaald.
toast-send-undone = Verzenden ongedaan gemaakt.
toast-too-late-to-undo-send = Te laat om ongedaan te maken: het bericht is al verzonden.
toast-undo = Ongedaan maken
toast-no-spam-folder = Dit account heeft geen spammap.
