# Katna Mail, French (Français): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Contacts
contacts-frequent = Fréquents
contacts-labels = Libellés

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
