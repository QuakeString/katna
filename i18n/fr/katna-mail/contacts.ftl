# Katna Mail, French (Français): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Contacts
contacts-frequent = Fréquents
contacts-other = Autres contacts
contacts-other-about = Personnes à qui vous avez écrit depuis Gmail mais que vous n’avez pas enregistrées
contacts-other-email = Envoyer un e-mail
contacts-other-empty = Aucun autre contact. Les personnes à qui vous écrivez depuis Gmail sans les enregistrer apparaissent ici.
contacts-other-allow = Pour voir les autres contacts, reconnectez-vous à votre compte Gmail et autorisez Katna à y accéder.
contacts-labels = Libellés
contacts-label-options = Options du libellé
contacts-label-rename = Renommer le libellé
contacts-label-email = Envoyer un e-mail à tous
contacts-label-delete = Supprimer le libellé
contacts-label-new = Nouveau libellé
contacts-label-name = Nom du libellé
contacts-label-button = Libellé
contacts-label-menu = Attribuer le libellé :
contacts-label-added = Ajouté à { $name }
contacts-label-removed = Retiré de { $name }
contacts-label-renamed = Libellé renommé en { $name }
contacts-label-deleted = Libellé supprimé : { $name }
contacts-label-no-email = Personne dans ce libellé n’a d’adresse e-mail
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = Comptes
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = Reconnectez-vous pour afficher les contacts
contacts-account-signed-in = Reconnecté à { $address }. Récupération de vos contacts…
contacts-account-sign-in-refused = { $provider } n’a pas laissé entrer Katna. Réessayez, et autorisez l’accès à vos contacts.
contacts-account-password = Le serveur n’a pas accepté le mot de passe. Yahoo, iCloud, Zoho et d’autres demandent un mot de passe d’application.
contacts-account-change-password = Changer le mot de passe
contacts-account-change-password-tooltip = Saisissez le nouveau mot de passe ; Katna le vérifie auprès du serveur
contacts-account-failed = Impossible de lire les contacts.
# $reason is the server's own words, in English.
contacts-account-error = Impossible de lire les contacts : { $reason }
contacts-account-none = Aucun carnet d’adresses trouvé
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = Aucun carnet d’adresses trouvé: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider } ne montre les contacts qu’à Katna connecté avec { $provider }.
contacts-account-sign-in-with = Se connecter avec { $provider }
contacts-account-looking = Recherche des contacts…
contacts-account-try-again = Réessayer
contacts-account-try-again-tooltip = Vérifier à nouveau les contacts de ce compte maintenant
contacts-account-fixing = En cours…
contacts-manage = Corriger et gérer
contacts-merge = Fusionner et corriger
contacts-merge-about = { $count ->
    [one] { $count } suggestion : des contacts qui semblent être la même personne
    [many] { $count } de suggestions : des contacts qui semblent être la même personne
   *[other] { $count } suggestions : des contacts qui semblent être la même personne
}
contacts-merge-none = Aucun doublon. Les contacts portant le même nom ou le même numéro de téléphone apparaissent ici.
contacts-merge-count = { $count ->
    [one] { $count } contact
    [many] { $count } de contacts
   *[other] { $count } contacts
}
contacts-merge-all = Tout fusionner
contacts-merge-button = Fusionner
contacts-merge-dismiss = Ignorer
contacts-merged = { $count ->
    [1] Contacts fusionnés
    [one] { $count } fusion effectuée
    [many] { $count } de fusions effectuées
   *[other] { $count } fusions effectuées
}
contacts-import = Importer
contacts-export = Exporter
contacts-import-file = Importer des contacts depuis un fichier vCard ou CSV
contacts-imported = { $count ->
    [one] { $count } contact importé dans { $place }
    [many] { $count } de contacts importés dans { $place }
   *[other] { $count } contacts importés dans { $place }
}
contacts-imported-some = { $count ->
    [one] { $count } contact importé dans { $place } ; { $skipped } déjà enregistrés, ignorés
    [many] { $count } de contacts importés dans { $place } ; { $skipped } déjà enregistrés, ignorés
   *[other] { $count } contacts importés dans { $place } ; { $skipped } déjà enregistrés, ignorés
}
contacts-import-none = Aucun contact trouvé dans { $name }
contacts-import-all-saved = Toutes les personnes de { $name } sont déjà enregistrées
contacts-import-failed = Impossible de lire { $name } : { $error }
contacts-exported = { $count ->
    [one] { $count } contact exporté vers { $path }
    [many] { $count } de contacts exportés vers { $path }
   *[other] { $count } contacts exportés vers { $path }
}
contacts-export-none = Aucun contact à exporter
contacts-export-failed = Impossible d’exporter les contacts : { $error }
contacts-print = Imprimer
contacts-print-title = Contacts
contacts-print-none = Aucun contact à imprimer
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = Anniversaire : { $day }
contacts-print-nickname = Surnom : { $name }
contacts-create = Nouveau contact

## Search and the list

contacts-search = Rechercher dans les contacts
contacts-loading = Chargement des contacts…
contacts-empty = Aucun contact enregistré pour le moment. Les contacts que vous enregistrez dans Gmail, Outlook ou votre service de messagerie apparaissent ici.
contacts-empty-no-books = Les contacts de vos comptes apparaîtront ici une fois synchronisés.
contacts-none-found = Aucun contact ne correspond à votre recherche.
contacts-starred = { $count ->
    [one] Contact suivi ({ $count })
    [many] Contacts suivis ({ $count })
   *[other] Contacts suivis ({ $count })
}
contacts-count = Contacts ({ $count })
contacts-col-name = Nom
contacts-col-email = E-mail
contacts-col-phone = Numéro de téléphone
contacts-col-job = Poste et entreprise
contacts-col-labels = Libellés

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Autoriser Katna à lire les contacts de { $address }.
contacts-allow-many = { $more ->
    [one] Autoriser Katna à lire les contacts de { $address } et de { $more } autre compte.
    [many] Autoriser Katna à lire les contacts de { $address } et de { $more } d’autres comptes.
   *[other] Autoriser Katna à lire les contacts de { $address } et de { $more } autres comptes.
}
contacts-allow-button = Autoriser

## A contact's page

contacts-back = Retour aux contacts
contacts-edit = Modifier
contacts-delete = Supprimer
contacts-qr = Partager sous forme de code QR
contacts-qr-about = Scannez ce code avec l’appareil photo d’un téléphone pour enregistrer le contact.
contacts-qr-too-long = Ce contact contient trop de détails pour tenir dans un code QR.
contacts-qr-done = Terminé
contacts-deleted = Contact supprimé : { $name }
contacts-added = { $name } ajouté aux contacts
contacts-find-mail = Courrier
contacts-details = Coordonnées
contacts-saved-in = Enregistré dans
contacts-notes = Notes
contacts-birthday = Anniversaire
contacts-nickname = Surnom
contacts-this-computer = Cet ordinateur
contacts-kind-home = Domicile
contacts-kind-work = Travail
contacts-kind-mobile = Mobile
contacts-kind-other = Autre
contacts-source-google = Google Contacts
contacts-source-microsoft = Contacts Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Créer un contact
contacts-edit-title = Modifier le contact
contacts-edit-save = Enregistrer
contacts-edit-saving = Enregistrement…
contacts-edit-cancel = Annuler
contacts-saved = Contact enregistré
contacts-edit-save-to = Enregistrer dans
contacts-edit-changes-go-to = Les modifications sont enregistrées dans { $place }.
contacts-edit-given = Prénom
contacts-edit-family = Nom de famille
contacts-edit-company = Entreprise
contacts-edit-job = Poste
contacts-edit-email = E-mail
contacts-edit-phone = Téléphone
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = Ajouter une adresse e-mail
contacts-edit-add-phone = Ajouter un numéro de téléphone
contacts-edit-street = Adresse
contacts-edit-city = Ville
contacts-edit-postcode = Code postal
contacts-edit-country = Pays
contacts-edit-birthday = Anniversaire (YYYY-MM-DD)
contacts-edit-empty = Ajoutez d’abord un nom, une adresse e-mail ou un numéro de téléphone.
