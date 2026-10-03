# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Ajouter un compte de messagerie
add-account-providers-intro = Choisissez votre fournisseur de messagerie. Katna s’occupe du reste.
add-account-provider-other = Autre messagerie
add-account-provider-other-detail = Tout compte IMAP ou POP3
add-account-provider-google-detail = Gmail et Google Workspace
add-account-provider-microsoft-detail = Outlook et Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = Se connecter à { $provider }
add-account-form-title-other = Votre compte de messagerie
add-account-form-intro = Katna conserve votre mot de passe dans le trousseau de votre système.
add-account-looking = Recherche des serveurs de messagerie de { $address }…
add-account-address-intro = Saisissez votre adresse e-mail. Katna trouve les serveurs pour vous.
add-account-servers-title = Paramètres du serveur
add-account-servers-intro = Où Katna lit et envoie le courrier de { $address }.
add-account-signing-in = Connexion…
add-account-browser-title = Continuez dans votre navigateur
add-account-browser-intro = Katna a ouvert la page de connexion de { $provider } dans votre navigateur. Connectez-vous-y et autorisez Katna à lire et envoyer votre courrier, puis revenez ici.
add-account-browser-hint = Aucune page ne s’est ouverte ? Vérifiez les fenêtres de votre navigateur, ou revenez en arrière et réessayez.
add-account-stage-browser = En attente de votre connexion dans le navigateur…
add-account-stage-signing-in-at = Connexion à { $server }…
add-account-help-app-password-link = Comment créer un mot de passe d’application
add-account-help-turn-on-imap = { $provider } n’accepte les applications de messagerie qu’une fois l’accès IMAP et POP3 activé dans les paramètres de son webmail.
add-account-help-turn-on-imap-link = Comment l’activer

## Add a mail account: fields

add-account-field-address = Adresse e-mail
add-account-receive-with = Recevoir les messages avec
add-account-imap-about = IMAP garde vos messages et vos dossiers sur le serveur, identiques sur chaque appareil. Choisissez-le si vous le pouvez.
add-account-pop3-about = POP3 télécharge vos messages sur cet ordinateur. Les messages que vous lisez ou déplacez ici restent inchangés sur le serveur et sur vos autres appareils.
add-account-incoming = Courrier entrant ({ $protocol })
add-account-outgoing = Courrier sortant ({ $protocol })
add-account-field-server = Serveur
add-account-field-port = Port
add-account-security-none = Aucune
add-account-security-none-warning = Non chiffrée : votre mot de passe et vos e-mails peuvent être lus en chemin.
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

## Add a mail account: buttons

add-account-sign-in-with = Se connecter avec { $provider }
add-account-sign-in-instead = Se connecter plutôt avec { $provider }

add-account-servers-button = Paramètres du serveur
add-account-back = Retour
add-account-add = Ajouter le compte
add-account-done = Terminé
add-account-another = Ajouter un autre compte
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
add-account-app-password-refused = { $provider } a refusé le mot de passe. Il faut un mot de passe d’application, pas celui que vous utilisez sur le Web.
add-account-password-refused = Le serveur a refusé le mot de passe. Vérifiez-le et réessayez.
add-account-sign-in-refused = { $provider } n’a pas laissé entrer Katna. Réessayez, et autorisez l’accès à votre courrier.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Cette copie de Katna ne peut pas encore se connecter aux comptes Microsoft.
    [Google] Cette copie de Katna ne peut pas encore se connecter aux comptes Google.
   *[other] Ce fournisseur n’autorise la connexion que sur sa propre page, ce que Katna ne sait pas encore faire pour lui.
}
add-account-smtp-not-found = Katna a trouvé où lire vos messages, mais pas où les envoyer. Saisissez le serveur sortant.

## Add a mail account: the last step

add-account-done-title = Votre compte est prêt
add-account-done-intro = Katna récupère maintenant vos messages. Les nouveaux s’affichent dès leur arrivée.
add-account-done-sign-in = Connexion
add-account-done-signed-in-with = Avec { $provider }, dans votre navigateur
add-account-done-receiving = Réception des messages
add-account-done-sending = Envoi des messages
add-account-done-on-server = Messages sur le serveur
add-account-done-kept = Conservés jusqu’à ce que vous les supprimiez dans Katna
add-account-done-pop3-hint = Modifiez ce qu’il advient des messages sur le serveur dans Paramètres > Comptes.
add-account-done-zoho-title = Tâches et calendriers
add-account-done-zoho-about = Zoho les garde à part du courrier. Connectez-vous une fois avec Zoho pour les intégrer à Katna.
add-account-done-linked = Tâches et calendriers connectés

## The account menu (from the account button on the top bar)

add-account-menu-another = Ajouter un autre compte
app-menu = Menu principal
app-menu-back = Retour
