# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Principale
tab-promotions = Promotions
tab-social = Réseaux sociaux
tab-updates = Notifications
tab-forums = Forums
tab-focused = Prioritaire
tab-other = Autres
tab-inbox = Boîte de réception
tab-newsletters = Newsletters
tab-notifications = Notifications
tab-new = { $count ->
    [one] { $count } nouveau
    [many] { $count } de nouveaux
   *[other] { $count } nouveaux
}
tab-provider-other = tri par Katna

## Mail list: toolbar

list-select = Sélectionner
list-refresh = Actualiser
list-checking = Recherche de nouveaux messages…
list-more = Plus
list-mark-read = Marquer comme lu
list-mark-unread = Marquer comme non lu
list-move-to = Déplacer vers
list-archive = Archiver
list-spam = Signaler comme spam
list-delete = Supprimer
list-snooze = Mettre en attente
list-unsnooze = Annuler la mise en attente
list-newer = Plus récents
list-older = Plus anciens
list-range = { $first }–{ $last } sur { $total }
list-range-about = { $first }–{ $last } sur environ { $total }
list-results = Résultats pour « { $query } »
list-results-corrected = Affichage des résultats pour « { $query } »
list-search-instead = Rechercher plutôt « { $query } »
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Tous
list-pick-none = Aucun
list-pick-read = Lus
list-pick-unread = Non lus
list-pick-starred = Suivis
list-pick-unstarred = Non suivis

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } conversation est sélectionnée.
        [many] Les { $count } de conversations sont sélectionnées.
       *[other] Les { $count } conversations sont sélectionnées.
    }
   *[message] { $count ->
        [one] { $count } message est sélectionné.
        [many] Les { $count } de messages sont sélectionnés.
       *[other] Les { $count } messages sont sélectionnés.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $count } conversation est sélectionnée dans { $folder }.
        [many] Les { $count } de conversations dans { $folder } sont sélectionnées.
       *[other] Les { $count } conversations dans { $folder } sont sélectionnées.
    }
   *[message] { $count ->
        [one] { $count } message est sélectionné dans { $folder }.
        [many] Les { $count } de messages dans { $folder } sont sélectionnés.
       *[other] Les { $count } messages dans { $folder } sont sélectionnés.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] { $count } conversation est sélectionnée sur cette page.
        [many] Les { $count } de conversations de cette page sont sélectionnées.
       *[other] Les { $count } conversations de cette page sont sélectionnées.
    }
   *[message] { $count ->
        [one] { $count } message est sélectionné sur cette page.
        [many] Les { $count } de messages de cette page sont sélectionnés.
       *[other] Les { $count } messages de cette page sont sélectionnés.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Sélectionner { $count } conversation
        [many] Sélectionner les { $count } de conversations
       *[other] Sélectionner les { $count } conversations
    }
   *[message] { $count ->
        [one] Sélectionner { $count } message
        [many] Sélectionner les { $count } de messages
       *[other] Sélectionner les { $count } messages
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Sélectionner { $count } conversation dans { $folder }
        [many] Sélectionner les { $count } de conversations dans { $folder }
       *[other] Sélectionner les { $count } conversations dans { $folder }
    }
   *[message] { $count ->
        [one] Sélectionner { $count } message dans { $folder }
        [many] Sélectionner les { $count } de messages dans { $folder }
       *[other] Sélectionner les { $count } messages dans { $folder }
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversation lue est sélectionnée sur cette page.
            [many] Les { $count } de conversations lues de cette page sont sélectionnées.
           *[other] Les { $count } conversations lues de cette page sont sélectionnées.
        }
       *[message] { $count ->
            [one] { $count } message lu est sélectionné sur cette page.
            [many] Les { $count } de messages lus de cette page sont sélectionnés.
           *[other] Les { $count } messages lus de cette page sont sélectionnés.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversation non lue est sélectionnée sur cette page.
            [many] Les { $count } de conversations non lues de cette page sont sélectionnées.
           *[other] Les { $count } conversations non lues de cette page sont sélectionnées.
        }
       *[message] { $count ->
            [one] { $count } message non lu est sélectionné sur cette page.
            [many] Les { $count } de messages non lus de cette page sont sélectionnés.
           *[other] Les { $count } messages non lus de cette page sont sélectionnés.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversation suivie est sélectionnée sur cette page.
            [many] Les { $count } de conversations suivies de cette page sont sélectionnées.
           *[other] Les { $count } conversations suivies de cette page sont sélectionnées.
        }
       *[message] { $count ->
            [one] { $count } message suivi est sélectionné sur cette page.
            [many] Les { $count } de messages suivis de cette page sont sélectionnés.
           *[other] Les { $count } messages suivis de cette page sont sélectionnés.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversation non suivie est sélectionnée sur cette page.
            [many] Les { $count } de conversations non suivies de cette page sont sélectionnées.
           *[other] Les { $count } conversations non suivies de cette page sont sélectionnées.
        }
       *[message] { $count ->
            [one] { $count } message non suivi est sélectionné sur cette page.
            [many] Les { $count } de messages non suivis de cette page sont sélectionnés.
           *[other] Les { $count } messages non suivis de cette page sont sélectionnés.
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Sélectionner { $count } conversation lue
            [many] Sélectionner les { $count } de conversations lues
           *[other] Sélectionner les { $count } conversations lues
        }
       *[message] { $count ->
            [one] Sélectionner { $count } message lu
            [many] Sélectionner les { $count } de messages lus
           *[other] Sélectionner les { $count } messages lus
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Sélectionner { $count } conversation non lue
            [many] Sélectionner les { $count } de conversations non lues
           *[other] Sélectionner les { $count } conversations non lues
        }
       *[message] { $count ->
            [one] Sélectionner { $count } message non lu
            [many] Sélectionner les { $count } de messages non lus
           *[other] Sélectionner les { $count } messages non lus
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Sélectionner { $count } conversation suivie
            [many] Sélectionner les { $count } de conversations suivies
           *[other] Sélectionner les { $count } conversations suivies
        }
       *[message] { $count ->
            [one] Sélectionner { $count } message suivi
            [many] Sélectionner les { $count } de messages suivis
           *[other] Sélectionner les { $count } messages suivis
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Sélectionner { $count } conversation non suivie
            [many] Sélectionner les { $count } de conversations non suivies
           *[other] Sélectionner les { $count } conversations non suivies
        }
       *[message] { $count ->
            [one] Sélectionner { $count } message non suivi
            [many] Sélectionner les { $count } de messages non suivis
           *[other] Sélectionner les { $count } messages non suivis
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Sélectionner { $count } conversation lue dans { $folder }
            [many] Sélectionner les { $count } de conversations lues dans { $folder }
           *[other] Sélectionner les { $count } conversations lues dans { $folder }
        }
       *[message] { $count ->
            [one] Sélectionner { $count } message lu dans { $folder }
            [many] Sélectionner les { $count } de messages lus dans { $folder }
           *[other] Sélectionner les { $count } messages lus dans { $folder }
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Sélectionner { $count } conversation non lue dans { $folder }
            [many] Sélectionner les { $count } de conversations non lues dans { $folder }
           *[other] Sélectionner les { $count } conversations non lues dans { $folder }
        }
       *[message] { $count ->
            [one] Sélectionner { $count } message non lu dans { $folder }
            [many] Sélectionner les { $count } de messages non lus dans { $folder }
           *[other] Sélectionner les { $count } messages non lus dans { $folder }
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Sélectionner { $count } conversation suivie dans { $folder }
            [many] Sélectionner les { $count } de conversations suivies dans { $folder }
           *[other] Sélectionner les { $count } conversations suivies dans { $folder }
        }
       *[message] { $count ->
            [one] Sélectionner { $count } message suivi dans { $folder }
            [many] Sélectionner les { $count } de messages suivis dans { $folder }
           *[other] Sélectionner les { $count } messages suivis dans { $folder }
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Sélectionner { $count } conversation non suivie dans { $folder }
            [many] Sélectionner les { $count } de conversations non suivies dans { $folder }
           *[other] Sélectionner les { $count } conversations non suivies dans { $folder }
        }
       *[message] { $count ->
            [one] Sélectionner { $count } message non suivi dans { $folder }
            [many] Sélectionner les { $count } de messages non suivis dans { $folder }
           *[other] Sélectionner les { $count } messages non suivis dans { $folder }
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversation lue est sélectionnée.
            [many] Les { $count } de conversations lues sont sélectionnées.
           *[other] Les { $count } conversations lues sont sélectionnées.
        }
       *[message] { $count ->
            [one] { $count } message lu est sélectionné.
            [many] Les { $count } de messages lus sont sélectionnés.
           *[other] Les { $count } messages lus sont sélectionnés.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversation non lue est sélectionnée.
            [many] Les { $count } de conversations non lues sont sélectionnées.
           *[other] Les { $count } conversations non lues sont sélectionnées.
        }
       *[message] { $count ->
            [one] { $count } message non lu est sélectionné.
            [many] Les { $count } de messages non lus sont sélectionnés.
           *[other] Les { $count } messages non lus sont sélectionnés.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversation suivie est sélectionnée.
            [many] Les { $count } de conversations suivies sont sélectionnées.
           *[other] Les { $count } conversations suivies sont sélectionnées.
        }
       *[message] { $count ->
            [one] { $count } message suivi est sélectionné.
            [many] Les { $count } de messages suivis sont sélectionnés.
           *[other] Les { $count } messages suivis sont sélectionnés.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversation non suivie est sélectionnée.
            [many] Les { $count } de conversations non suivies sont sélectionnées.
           *[other] Les { $count } conversations non suivies sont sélectionnées.
        }
       *[message] { $count ->
            [one] { $count } message non suivi est sélectionné.
            [many] Les { $count } de messages non suivis sont sélectionnés.
           *[other] Les { $count } messages non suivis sont sélectionnés.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversation lue dans { $folder } est sélectionnée.
            [many] Les { $count } de conversations lues dans { $folder } sont sélectionnées.
           *[other] Les { $count } conversations lues dans { $folder } sont sélectionnées.
        }
       *[message] { $count ->
            [one] { $count } message lu dans { $folder } est sélectionné.
            [many] Les { $count } de messages lus dans { $folder } sont sélectionnés.
           *[other] Les { $count } messages lus dans { $folder } sont sélectionnés.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversation non lue dans { $folder } est sélectionnée.
            [many] Les { $count } de conversations non lues dans { $folder } sont sélectionnées.
           *[other] Les { $count } conversations non lues dans { $folder } sont sélectionnées.
        }
       *[message] { $count ->
            [one] { $count } message non lu dans { $folder } est sélectionné.
            [many] Les { $count } de messages non lus dans { $folder } sont sélectionnés.
           *[other] Les { $count } messages non lus dans { $folder } sont sélectionnés.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversation suivie dans { $folder } est sélectionnée.
            [many] Les { $count } de conversations suivies dans { $folder } sont sélectionnées.
           *[other] Les { $count } conversations suivies dans { $folder } sont sélectionnées.
        }
       *[message] { $count ->
            [one] { $count } message suivi dans { $folder } est sélectionné.
            [many] Les { $count } de messages suivis dans { $folder } sont sélectionnés.
           *[other] Les { $count } messages suivis dans { $folder } sont sélectionnés.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversation non suivie dans { $folder } est sélectionnée.
            [many] Les { $count } de conversations non suivies dans { $folder } sont sélectionnées.
           *[other] Les { $count } conversations non suivies dans { $folder } sont sélectionnées.
        }
       *[message] { $count ->
            [one] { $count } message non suivi dans { $folder } est sélectionné.
            [many] Les { $count } de messages non suivis dans { $folder } sont sélectionnés.
           *[other] Les { $count } messages non suivis dans { $folder } sont sélectionnés.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Aucune conversation lue ici.
       *[message] Aucun message lu ici.
    }
   *[unread] { $kind ->
        [conversation] Aucune conversation non lue ici.
       *[message] Aucun message non lu ici.
    }
    [starred] { $kind ->
        [conversation] Aucune conversation suivie ici.
       *[message] Aucun message suivi ici.
    }
    [unstarred] { $kind ->
        [conversation] Aucune conversation non suivie ici.
       *[message] Aucun message non suivi ici.
    }
}
list-clear-selection = Effacer la sélection

## Mail list: empty states

list-empty-search = Aucun message ne correspond à votre recherche.
list-empty-tab = Aucun message dans { $tab }.
list-empty-tab-unknown = Aucun message dans cet onglet.
list-empty-folder = Aucun message dans { $folder }.
list-empty-folder-unknown = Aucun message dans ce dossier.
list-first-sync = Récupération de vos messages…
list-first-sync-detail = Ils s’affichent ici au fur et à mesure de leur arrivée.

## Mail list: lines

row-removed = Ce message a été supprimé.
row-starred = Suivi
row-not-starred = Non suivi
row-important = Important. Cliquez pour le marquer comme non important.
row-mark-important = Marquer comme important
row-pinned = Épinglé en haut
row-task = Tâche
row-task-open = Ouvrir la tâche : { $title }
row-tracking-none = Suivi. Pas encore ouvert
row-tracking-opened = Ouvert par { $opened } sur { $recipients }
row-tracking-clicked = Ouvert par { $opened } sur { $recipients }, un lien suivi par { $clicked }
row-pin = Épingler en haut
row-unpin = Désépingler
row-snoozed-until = En attente jusqu’à { $when }

## Mail list: More menu and right-click menu

menu-reply = Répondre
menu-reply-all = Répondre à tous
menu-forward = Transférer
menu-archive = Archiver
menu-delete = Supprimer
menu-delete-forever = Supprimer définitivement
menu-move-to-inbox = Déplacer vers la boîte de réception
menu-spam = Signaler comme spam
menu-not-spam = Pas un spam
menu-mark-read = Marquer comme lu
menu-mark-unread = Marquer comme non lu
menu-mark-all-read = Tout marquer comme lu
menu-star = Ajouter le suivi
menu-unstar = Supprimer le suivi
menu-important = Marquer comme important
menu-not-important = Marquer comme non important
menu-pin = Épingler en haut
menu-unpin = Désépingler
menu-snooze = Mettre en attente
menu-unsnooze = Annuler la mise en attente
menu-add-to-tasks = Ajouter aux tâches
menu-schedule-meeting = Planifier une réunion
menu-add-note = Ajouter une note
menu-print-all = Tout imprimer
menu-new-window = Ouvrir dans une nouvelle fenêtre
menu-move-to = Déplacer vers
menu-move-to-heading = Déplacer vers :
menu-find-from = Rechercher les e-mails de { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Conversation archivée.
        [many] { $count } de conversations archivées.
       *[other] { $count } conversations archivées.
    }
   *[message] { $count ->
        [one] Message archivé.
        [many] { $count } de messages archivés.
       *[other] { $count } messages archivés.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Conversation placée dans la corbeille.
        [many] { $count } de conversations placées dans la corbeille.
       *[other] { $count } conversations placées dans la corbeille.
    }
   *[message] { $count ->
        [one] Message placé dans la corbeille.
        [many] { $count } de messages placés dans la corbeille.
       *[other] { $count } messages placés dans la corbeille.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Conversation déplacée.
        [many] { $count } de conversations déplacées.
       *[other] { $count } conversations déplacées.
    }
   *[message] { $count ->
        [one] Message déplacé.
        [many] { $count } de messages déplacés.
       *[other] { $count } messages déplacés.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Conversation suivie.
        [many] { $count } de conversations suivies.
       *[other] { $count } conversations suivies.
    }
   *[message] { $count ->
        [one] Message suivi.
        [many] { $count } de messages suivis.
       *[other] { $count } messages suivis.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Suivi retiré de la conversation.
        [many] Suivi retiré de { $count } de conversations.
       *[other] Suivi retiré de { $count } conversations.
    }
   *[message] { $count ->
        [one] Suivi retiré du message.
        [many] Suivi retiré de { $count } de messages.
       *[other] Suivi retiré de { $count } messages.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Conversation marquée comme importante.
        [many] { $count } de conversations marquées comme importantes.
       *[other] { $count } conversations marquées comme importantes.
    }
   *[message] { $count ->
        [one] Message marqué comme important.
        [many] { $count } de messages marqués comme importants.
       *[other] { $count } messages marqués comme importants.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Conversation marquée comme non importante.
        [many] { $count } de conversations marquées comme non importantes.
       *[other] { $count } conversations marquées comme non importantes.
    }
   *[message] { $count ->
        [one] Message marqué comme non important.
        [many] { $count } de messages marqués comme non importants.
       *[other] { $count } messages marqués comme non importants.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Conversation épinglée en haut.
        [many] { $count } de conversations épinglées en haut.
       *[other] { $count } conversations épinglées en haut.
    }
   *[message] { $count ->
        [one] Message épinglé en haut.
        [many] { $count } de messages épinglés en haut.
       *[other] { $count } messages épinglés en haut.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Conversation désépinglée.
        [many] { $count } de conversations désépinglées.
       *[other] { $count } conversations désépinglées.
    }
   *[message] { $count ->
        [one] Message désépinglé.
        [many] { $count } de messages désépinglés.
       *[other] { $count } messages désépinglés.
    }
}
toast-snoozed = { $kind ->
    [conversation] { $count ->
        [one] Conversation mise en attente jusqu’à { $when }.
        [many] { $count } de conversations mises en attente jusqu’à { $when }.
       *[other] { $count } conversations mises en attente jusqu’à { $when }.
    }
   *[message] { $count ->
        [one] Message mis en attente jusqu’à { $when }.
        [many] { $count } de messages mis en attente jusqu’à { $when }.
       *[other] { $count } messages mis en attente jusqu’à { $when }.
    }
}
toast-unsnoozed = { $kind ->
    [conversation] { $count ->
        [one] Conversation de retour dans la boîte de réception.
        [many] { $count } de conversations de retour dans la boîte de réception.
       *[other] { $count } conversations de retour dans la boîte de réception.
    }
   *[message] { $count ->
        [one] Message de retour dans la boîte de réception.
        [many] { $count } de messages de retour dans la boîte de réception.
       *[other] { $count } messages de retour dans la boîte de réception.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Conversation signalée comme spam.
        [many] { $count } de conversations signalées comme spam.
       *[other] { $count } conversations signalées comme spam.
    }
   *[message] { $count ->
        [one] Message signalé comme spam.
        [many] { $count } de messages signalés comme spam.
       *[other] { $count } messages signalés comme spam.
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] Conversation marquée comme non-spam et déplacée vers la boîte de réception.
        [many] { $count } de conversations marquées comme non-spam et déplacées vers la boîte de réception.
       *[other] { $count } conversations marquées comme non-spam et déplacées vers la boîte de réception.
    }
   *[message] { $count ->
        [one] Message marqué comme non-spam et déplacé vers la boîte de réception.
        [many] { $count } de messages marqués comme non-spam et déplacés vers la boîte de réception.
       *[other] { $count } messages marqués comme non-spam et déplacés vers la boîte de réception.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Conversation supprimée définitivement.
        [many] { $count } de conversations supprimées définitivement.
       *[other] { $count } conversations supprimées définitivement.
    }
   *[message] { $count ->
        [one] Message supprimé définitivement.
        [many] { $count } de messages supprimés définitivement.
       *[other] { $count } messages supprimés définitivement.
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] Conversation marquée comme lue.
        [many] { $count } de conversations marquées comme lues.
       *[other] { $count } conversations marquées comme lues.
    }
   *[message] { $count ->
        [one] Message marqué comme lu.
        [many] { $count } de messages marqués comme lus.
       *[other] { $count } messages marqués comme lus.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] Conversation marquée comme non lue.
        [many] { $count } de conversations marquées comme non lues.
       *[other] { $count } conversations marquées comme non lues.
    }
   *[message] { $count ->
        [one] Message marqué comme non lu.
        [many] { $count } de messages marqués comme non lus.
       *[other] { $count } messages marqués comme non lus.
    }
}
toast-undone = Action annulée.
toast-nothing-to-undo = Rien à annuler.
toast-cannot-undo-delete-forever = Les messages supprimés définitivement ne peuvent pas être récupérés.
toast-send-undone = Envoi annulé.
toast-too-late-to-undo-send = Trop tard pour annuler : le message a déjà été envoyé.
toast-undo = Annuler
toast-close = Fermer
toast-no-spam-folder = Ce compte n’a pas de dossier de spam.
