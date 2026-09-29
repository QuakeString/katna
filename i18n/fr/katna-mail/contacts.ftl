# Katna Mail, French (Français): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Contacts
contacts-frequent = Fréquents
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
contacts-create = Créer un contact

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
