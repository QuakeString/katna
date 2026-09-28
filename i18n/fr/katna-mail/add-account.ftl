# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Ajouter un compte de messagerie
add-account-looking = Recherche des serveurs de messagerie de { $address }…
add-account-address-intro = Saisissez votre adresse e-mail. Katna trouve les serveurs pour vous.
add-account-servers-title = Paramètres du serveur
add-account-servers-intro = Où Katna lit et envoie le courrier de { $address }.
add-account-password-title = Saisissez votre mot de passe
add-account-signing-in = Connexion…
add-account-browser-title = Continuez dans votre navigateur
add-account-browser-intro = Katna a ouvert la page de connexion de { $provider } dans votre navigateur. Connectez-vous-y et autorisez Katna à lire et envoyer votre courrier, puis revenez ici.
add-account-browser-hint = Aucune page ne s’est ouverte ? Vérifiez les fenêtres de votre navigateur, ou revenez en arrière et réessayez.

## Add a mail account: fields

add-account-field-address = Adresse e-mail
add-account-incoming = Courrier entrant ({ $protocol })
add-account-outgoing = Courrier sortant ({ $protocol })
add-account-field-server = Serveur
add-account-field-port = Port
add-account-security-none = Aucune
add-account-field-username = Nom d’utilisateur
add-account-field-password = Mot de passe
add-account-show-password = Afficher le mot de passe
add-account-app-password-hint = { $provider } demande ici un mot de passe d’application, pas celui que vous utilisez sur le Web. Créez-en un dans les paramètres de sécurité de votre compte { $provider }.
add-account-field-name = Votre nom (facultatif)
add-account-name-hint = Affiché aux personnes à qui vous écrivez.
add-account-servers-pair = { $imap } et { $smtp }
add-account-servers-found = { $source ->
    [built-in] Serveurs : { $servers }, trouvés dans la liste de fournisseurs de Katna.
    [provider] Serveurs : { $servers }, trouvés dans les paramètres de votre fournisseur.
    [ispdb] Serveurs : { $servers }, trouvés dans la liste de fournisseurs de Thunderbird.
    [dns] Serveurs : { $servers }, trouvés dans les enregistrements DNS de votre domaine.
   *[other] Serveurs : { $servers }, devinés ; vérifiez-les si la connexion échoue.
}
add-account-servers-entered = Serveurs : { $servers }, tels que saisis.
add-account-or = ou
add-account-sign-in-with = Se connecter avec { $provider }
add-account-sign-in-instead = Se connecter plutôt avec { $provider }

## Add a mail account: buttons

add-account-servers-button = Paramètres du serveur
add-account-back = Retour
add-account-add = Ajouter le compte
add-account-next = Suivant
add-account-cancel = Annuler

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Saisissez le serveur entrant.
   *[outgoing] Saisissez le serveur sortant.
}
add-account-server-space = { $kind ->
    [incoming] Le nom du serveur entrant contient une espace.
   *[outgoing] Le nom du serveur sortant contient une espace.
}
add-account-port-invalid = { $kind ->
    [incoming] Le port entrant doit être un nombre de { $min } à { $max }.
   *[outgoing] Le port sortant doit être un nombre de { $min } à { $max }.
}
add-account-address-empty = Saisissez une adresse e-mail.
add-account-address-invalid = Saisissez une adresse e-mail comme { $example }.
add-account-not-found = Katna n’a pas trouvé les serveurs de { $address } et a donc rempli les noms habituels. Vérifiez-les auprès de votre fournisseur.
add-account-password-empty = Saisissez le mot de passe.
add-account-name-is-password = Le nom est identique au mot de passe. Saisissez plutôt votre nom, tel que les autres doivent le voir.
add-account-added = { $address } ajouté. Récupération de votre courrier…
add-account-app-password-refused = { $provider } a refusé le mot de passe. Il faut un mot de passe d’application, pas celui que vous utilisez sur le Web.
add-account-password-refused = Le serveur a refusé le mot de passe. Vérifiez-le et réessayez.
add-account-sign-in-refused = { $provider } n’a pas laissé entrer Katna. Réessayez, et autorisez l’accès à votre courrier.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Cette copie de Katna ne peut pas encore se connecter aux comptes Microsoft.
    [Google] Cette copie de Katna ne peut pas encore se connecter aux comptes Google.
   *[other] Ce fournisseur n’autorise la connexion que sur sa propre page, ce que Katna ne sait pas encore faire pour lui.
}
add-account-signed-in = Connecté avec { $provider }. Récupération de votre courrier…

## The account menu (from the account button on the top bar)

add-account-menu-another = Ajouter un autre compte
add-account-menu-manage = Gérer les comptes
app-menu = Menu principal
