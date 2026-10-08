# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = Le serveur de messagerie

problems-signed-out = { $provider } a déconnecté Katna de { $address }. La synchronisation du courrier s’est arrêtée.
problems-password-refused = { $provider } a refusé le mot de passe de { $address }. Il a peut-être changé.
problems-no-answer = { $provider } ne répond pas pour { $address }. Katna continue d’essayer.
problems-offline = Vous êtes hors ligne. Votre courrier est toujours là, et les messages que vous envoyez attendent votre retour en ligne.
problems-accounts-need-you = { $count ->
    [one] 1 compte a besoin de vous
    [many] { $count } de comptes ont besoin de vous
   *[other] { $count } comptes ont besoin de vous
}
problems-show = Afficher
problems-later = Plus tard
problems-new-password = Nouveau mot de passe
problems-try-again = Réessayer

## The New password card

problems-password-title = Nouveau mot de passe
problems-password-detail = { $provider } a refusé le mot de passe enregistré pour { $address }. Saisissez le nouveau ; Katna le vérifie avant de le conserver.
problems-password-placeholder = Mot de passe
problems-password-show = Afficher le mot de passe
problems-password-hide = Masquer le mot de passe
problems-password-cancel = Annuler
problems-password-save = Enregistrer
problems-password-checking = Vérification…
problems-password-refused-again = { $provider } a aussi refusé ce mot de passe. Vérifiez-le et réessayez.
problems-password-saved = Mot de passe enregistré pour { $address }. Récupération de votre courrier…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = Le serveur de messagerie de { $address } n’a pas accepté le déplacement { $count ->
    [one] d’un message, il est donc revenu là où il était.
    [many] de { $count } de messages, ils sont donc revenus là où ils étaient.
   *[other] de { $count } messages, ils sont donc revenus là où ils étaient.
}
problems-refused-flags = Le serveur de messagerie de { $address } n’a pas accepté de marquer { $count ->
    [one] un message (lu, suivi…), il est donc revenu à son état précédent.
    [many] { $count } de messages (lus, suivis…), ils sont donc revenus à leur état précédent.
   *[other] { $count } messages (lus, suivis…), ils sont donc revenus à leur état précédent.
}
problems-refused-label = Le serveur de messagerie de { $address } n’a pas accepté de modifier les libellés { $count ->
    [one] d’un message, il est donc revenu à son état précédent.
    [many] de { $count } de messages, ils sont donc revenus à leur état précédent.
   *[other] de { $count } messages, ils sont donc revenus à leur état précédent.
}
problems-refused-delete = Le serveur de messagerie de { $address } n’a pas accepté la suppression { $count ->
    [one] d’un message, il est donc revenu.
    [many] de { $count } de messages, ils sont donc revenus.
   *[other] de { $count } messages, ils sont donc revenus.
}
problems-refused-other = Le serveur de messagerie de { $address } n’a pas accepté { $count ->
    [one] une modification, Katna l’a donc annulée.
    [many] { $count } de modifications, Katna les a donc annulées.
   *[other] { $count } modifications, Katna les a donc annulées.
}
problems-details = Détails

## Katna's background service (katna-daemon) isn't running

service-starting = Démarrage du service d’arrière-plan de Katna…
service-failed = Le service d’arrière-plan de Katna ne démarre pas, le courrier n’est donc pas synchronisé.
service-start-again = Redémarrer
service-started-again = Le service d’arrière-plan de Katna s’était arrêté et a été redémarré.
service-details-title = Pourquoi le service ne démarre pas
service-details-body = Copiez ceci et envoyez-le avec votre rapport. Il ne contient ni messages ni mots de passe.
service-details-copy = Copier
service-details-close = Fermer
service-not-running = Le service d’arrière-plan de Katna n’est pas en cours d’exécution.
service-no-answer = Le service d’arrière-plan de Katna n’a pas répondu : { $error }
service-no-session = Aucune session D-Bus : { $error }
