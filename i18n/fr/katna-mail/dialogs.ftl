# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = À propos de Katna
about-tagline = Courrier et calendrier pour le bureau Linux
about-whats-new = Nouveautés
about-changelog = Journal des modifications
about-source = Code source
about-coffee = Offrez-moi un café
about-coming-soon = Bientôt disponible
about-coffee-scan = Ou scannez le code avec votre téléphone.
about-follow = Suivre l’auteur
about-love-title = Fait avec amour pour Rust, KDE et Linux
about-love-text = Avec Rust, écrire une application de messagerie rapide et sûre est un plaisir : Katna ne contient aucun code unsafe. Le bureau Plasma de KDE et sa suite PIM ont inspiré Katna, et Linux et la communauté du logiciel libre sont le socle sur lequel il repose. Merci, et merci aux bibliothèques ci-dessous.
about-kde-text = KDE crée le bureau sur lequel Katna se sent le plus chez lui. Il est fait par des bénévoles et financé par des personnes comme vous. Si vous aimez Plasma ou les applications de KDE, pensez à faire un don à KDE.
about-donate-kde = Faire un don à KDE
about-gpui-title = Construit sur GPUI, issu du projet Zed
about-gpui-text = Toute l’interface de Katna Mail est construite sur GPUI, le framework d’interface rapide et accéléré par GPU que Zed Industries a créé pour l’éditeur Zed. Chaque pixel, chaque animation et chaque fenêtre que vous voyez est dessiné par lui. Merci à l’équipe Zed de le développer en open source. Apache-2.0.
about-gpui-github = GPUI sur GitHub
about-personal-title = Un projet personnel
about-personal-text = Katna Mail ne cherche pas à être nouveau ou révolutionnaire. C’est l’application de messagerie que son auteur voulait, et ses fonctions et son apparence sont empruntées à Gmail, Mailspring et Thunderbird. Il n’a été possible que grâce aux progrès des LLM.
about-built-on = BÂTI SUR DES LOGICIELS LIBRES
about-credit-pimalaya = IMAP, SMTP et connexion (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = Lecture et écriture d’IMAP
about-credit-tantivy = Recherche
about-credit-sqlite = Le stockage du courrier
about-credit-rustls = Connexions sécurisées
about-credit-mail-parser = Lecture du courrier, par Stalwart Labs
about-credit-html5ever = Courrier HTML, issu du projet Servo
about-credit-zbus = Dialogue avec le bureau via D-Bus et les portails
about-credit-oo7 = Mots de passe dans le trousseau du bureau
about-credit-hayro = Affichage et impression des PDF
about-credit-calamine = Aperçus des feuilles de calcul
about-credit-resvg = Images SVG
about-credit-jiff = Dates et fuseaux horaires
about-credit-spellbook = Correcteur orthographique, issu de l’éditeur Helix
about-credit-smol = Faire beaucoup de choses à la fois
about-all-libraries = Toutes les bibliothèques utilisées par Katna ({ $count })
about-library-authors = par { $authors }
about-license = Katna est un logiciel libre sous licence GNU GPL, version 3 ou ultérieure.
about-close = Fermer

## What’s new (shown after an update)

whats-new-title = Nouveautés de Katna Mail
whats-new-updated = Mis à jour vers la version { $version }
whats-new-version = Version { $version }
whats-new-more = { $count ->
    [one] Et { $count } autre dans le journal complet des modifications.
    [many] Et { $count } d’autres dans le journal complet des modifications.
   *[other] Et { $count } autres dans le journal complet des modifications.
}
whats-new-changelog = Journal complet des modifications
whats-new-got-it = Compris

## First run: welcome page

onboarding-welcome-title = Bienvenue dans Katna Mail
onboarding-welcome-lead = Votre courrier sur votre propre ordinateur : rapide à rechercher, lisible hors ligne et privé.
onboarding-fast-title = Rapide, même hors ligne
onboarding-fast-text = Katna garde ici une copie de votre courrier : l’ouvrir et y chercher est instantané, avec ou sans connexion.
onboarding-providers-title = Compatible avec votre messagerie
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud et tout autre compte IMAP ou POP.
onboarding-private-title = Privé
onboarding-private-text = Votre courrier va directement de votre fournisseur à cet ordinateur. Aucun serveur Katna ne le voit.
onboarding-get-started = Commencer

## First run: adding an account

onboarding-service-checking = Vérification du service d’arrière-plan de Katna…
onboarding-service-running = Le service d’arrière-plan de Katna fonctionne.
onboarding-service-missing = Le service d’arrière-plan de Katna ne fonctionne pas
onboarding-service-start = Il récupère et envoie votre courrier. Lancez-le depuis un terminal, puis vérifiez à nouveau :
onboarding-check-again = Vérifier à nouveau
onboarding-account-title = Ajoutez votre compte de messagerie
onboarding-account-lead = Saisissez votre adresse e-mail et votre mot de passe, et Katna trouve les paramètres du serveur. Gmail, Yahoo et iCloud demandent un mot de passe d’application, créé dans les paramètres de sécurité de votre compte.
onboarding-add-account = Ajouter un compte
onboarding-back = Retour

## First run: choosing the look

onboarding-look-title = À votre goût
onboarding-look-lead = Choisissez comment le courrier s’ouvre et à quoi ressemble Katna. Vous pouvez changer cela à tout moment dans les paramètres rapides.
onboarding-reading-pane = Volet de lecture
onboarding-pane-right = À droite de la liste
onboarding-pane-none = Aucune séparation
onboarding-theme = Thème
onboarding-theme-system = Système
onboarding-theme-light = Clair
onboarding-theme-dark = Sombre
onboarding-density = Densité
onboarding-density-default = Par défaut
onboarding-density-compact = Compacte
onboarding-continue = Continuer

## First run: done

onboarding-ready-title = Tout est prêt
onboarding-ready-lead = Katna récupère votre courrier. Il s’affiche au fur et à mesure qu’il arrive, et le nouveau courrier apparaît de lui-même.
onboarding-ready-lead-address = Katna récupère le courrier de { $address }. Il s’affiche au fur et à mesure qu’il arrive, et le nouveau courrier apparaît de lui-même.
onboarding-ready-tour = Faire une visite d’une minute pour voir où tout se trouve ?
onboarding-skip = Plus tard
onboarding-take-tour = Faire la visite guidée

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Aidez à améliorer Katna
share-lead = Quand Katna plante, il enregistre un rapport sur cet ordinateur. Envoyer ces rapports aide à corriger ce qui n’a pas fonctionné. Vous pouvez changer cela à tout moment dans Paramètres > Retours des utilisateurs.
share-sent = Ce qui est envoyé
share-sent-detail = Le rapport de plantage tel que vous pouvez le voir dans les Paramètres : ce qui a planté et où dans Katna, la version, votre système Linux et votre bureau, et les dernières lignes du journal de Katna, qui peuvent citer des dossiers de courrier.
share-never-sent = Ce qui n’est jamais envoyé
share-never-sent-detail = Vos messages, contacts, mots de passe, adresse IP, nom d’utilisateur ou nom d’ordinateur. Les adresses e-mail sont retirées du rapport.
share-where = Où il va
share-where-detail = Au système de suivi des plantages de Katna chez Sentry, stocké dans l’UE. Aucun identifiant ne relie les rapports à vous.
share-dont-send = Ne pas envoyer
share-send = Envoyer les rapports de plantage
share-sending = Les rapports de plantage seront envoyés. Merci.
share-local = Les rapports de plantage restent sur cet ordinateur.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Bienvenue dans Katna Mail
tour-welcome-text = Une visite d’une minute vous montre où tout se trouve.
tour-not-now = Pas maintenant
tour-start = Faire la visite guidée
tour-close = Fermer
tour-skip = Passer la visite
tour-back = Retour
tour-done = Terminé
tour-next = Suivant
tour-step = { $step } sur { $total }
tour-compose-title = Écrire un message
tour-compose-text = « Nouveau message » ouvre un message en bas à droite, pour que vous puissiez continuer à lire pendant que vous écrivez.
tour-search-title = Rechercher dans tout votre courrier
tour-search-text = La recherche fonctionne aussi hors ligne. Le bouton à l’extrémité droite ajoute des filtres : expéditeur, destinataire, objet, dates et pièces jointes.
tour-menu-title = Afficher ou masquer les dossiers
tour-menu-text = Ce bouton replie la liste des dossiers. Quand elle est masquée, placez le pointeur sur Courrier à gauche pour voir les dossiers.
tour-apps-title = Vos applications
tour-apps-text = Le courrier est ici désormais. Calendrier, Contacts, Tâches, Notes et Flux le rejoindront dans cette barre.
tour-tabs-title = Onglets de la boîte de réception
tour-tabs-text = Le nouveau courrier est trié dans Principale, Promotions, Réseaux sociaux, Notifications et Forums. Vous pouvez désactiver les onglets dans les paramètres rapides.
tour-list-title = Vos messages
tour-list-text = Cliquez sur un message pour le lire. Survolez-le pour les actions rapides, faites un clic droit pour en voir plus, ou cochez-en plusieurs pour agir sur eux ensemble.
tour-settings-title = Paramètres rapides
tour-settings-text = Changez ici le volet de lecture, la densité et le thème. La visite peut aussi être relancée depuis cet endroit.
tour-account-title = Votre compte
tour-account-text = Voyez dans quel compte vous êtes et ajoutez-en un autre.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Le service d’arrière-plan de Katna s’est arrêté de façon inattendue.
    [one] Le service d’arrière-plan de Katna s’est arrêté de façon inattendue. { $more } autre rapport de plantage est enregistré.
    [many] Le service d’arrière-plan de Katna s’est arrêté de façon inattendue. { $more } d’autres rapports de plantage sont enregistrés.
   *[other] Le service d’arrière-plan de Katna s’est arrêté de façon inattendue. { $more } autres rapports de plantage sont enregistrés.
}
crash-mail = { $more ->
    [0] Katna Mail s’est fermé de façon inattendue la dernière fois.
    [one] Katna Mail s’est fermé de façon inattendue la dernière fois. { $more } autre rapport de plantage est enregistré.
    [many] Katna Mail s’est fermé de façon inattendue la dernière fois. { $more } d’autres rapports de plantage sont enregistrés.
   *[other] Katna Mail s’est fermé de façon inattendue la dernière fois. { $more } autres rapports de plantage sont enregistrés.
}
crash-view = Voir le rapport
crash-view-tooltip = Ouvrir le rapport, enregistré sur cet ordinateur
crash-copy = Copier le rapport
crash-close = Fermer
