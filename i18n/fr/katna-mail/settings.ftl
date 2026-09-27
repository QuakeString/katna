# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings page: its tabs

settings-tab-general = Général
settings-tab-inbox = Boîte de réception
settings-tab-accounts = Comptes
settings-tab-subscriptions = Abonnements
settings-tab-appearance = Apparence
settings-tab-shortcuts = Raccourcis
settings-tab-default-apps = Applications par défaut
settings-tab-folders-rules = Dossiers et règles
settings-tab-compose = Rédaction
settings-tab-mcp-server = Serveur MCP
settings-tab-feedback = Retours des utilisateurs
settings-tab-experimental = Expérimental

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Consultez les newsletters et les listes de diffusion que vous recevez, et désabonnez-vous en un clic.
settings-tab-folders-rules-coming = Créez, renommez, déplacez et masquez des dossiers et des libellés, et choisissez ceux qui sont synchronisés. Les règles trient, libellent, transfèrent ou suppriment automatiquement les nouveaux messages, selon l’expéditeur, l’objet ou des mots.
settings-tab-mcp-server-coming = Permettez aux assistants IA de cet ordinateur de rechercher, lire et rédiger vos messages, avec votre accord.

## Settings > General

settings-general-conversations = Mode Conversation
settings-general-conversations-group = Regrouper les réponses à un même message
settings-general-conversations-group-detail = Une ligne par conversation dans la liste
settings-general-reading = Lecture
settings-general-newest-first = Message le plus récent en premier
settings-general-newest-first-detail = Une conversation commence par sa dernière réponse
settings-general-full-headers = Afficher les en-têtes complets
settings-general-full-headers-detail = De, à, cc, date et objet affichés pour chaque message
settings-general-full-names = Noms complets des destinataires
settings-general-full-names-detail = « à moi, Ada Lovelace » plutôt que « à moi, Ada »
settings-general-mark-read = Marquer comme lu
settings-general-mark-read-now = Dès l’ouverture
settings-general-mark-read-1s = Après 1 seconde d’ouverture
settings-general-mark-read-3s = Après 3 secondes d’ouverture
settings-general-mark-read-never = Uniquement manuellement
settings-general-reply-button = Bouton Répondre
settings-general-reply-all = Répondre à tous
settings-general-reply-all-detail = Le bouton de réponse à côté de chaque message répond à tous, pas seulement à l’expéditeur
settings-general-remote-images = Images provenant du Web
settings-general-remote-images-detail = Charger les images d’un message indique à son expéditeur que vous l’avez ouvert, quand et à peu près où. Si l’option est désactivée, chaque message vous demande d’abord, et vous pouvez toujours afficher les images d’un expéditeur.
settings-general-remote-images-always = Toujours afficher les images
settings-general-remote-images-always-detail = Dans tous les messages, pas seulement ceux des expéditeurs de confiance
settings-general-sending = Envoi
settings-general-sending-detail = Délai pendant lequel un message envoyé patiente, pour pouvoir être annulé.
settings-general-offline = Messages hors connexion
settings-general-offline-detail = Les messages récents sont téléchargés en entier pour être lus sans connexion. Les plus anciens sont téléchargés à l’ouverture.
settings-general-offline-days = { $count ->
    [one] { $count } jour
    [many] { $count } de jours
   *[other] { $count } jours
}
settings-general-offline-years = { $count ->
    [one] { $count } an
    [many] { $count } d’années
   *[other] { $count } ans
}
settings-general-offline-all = Tous les messages
settings-general-offline-note = Choisir moins de jours conserve les messages déjà téléchargés. Rien ne change sur le serveur.
settings-general-notifications = Notifications
settings-general-notifications-detail = Pour les nouveaux messages de la boîte de réception, même lorsque Katna Mail est fermé.
settings-general-new-mail = M’avertir des nouveaux messages
settings-general-new-mail-detail = Avec Répondre à tous, Marquer comme lu et Archiver
settings-general-new-mail-sound = Émettre un son
settings-general-new-mail-sound-detail = Le son de nouveau message du bureau
settings-general-desktop = Bureau
settings-general-start-at-login = Démarrer Katna à la connexion
settings-general-start-at-login-detail = Synchronise les messages et affiche les notifications de nouveaux messages et l’icône de la zone de notification, sans ouvrir la fenêtre
settings-general-login-window = Ouvrir aussi la fenêtre de Katna Mail
settings-general-login-window-detail = La fenêtre s’ouvre également à la connexion
settings-general-tray = Afficher Katna dans la zone de notification
settings-general-tray-detail = Avec le nombre de messages non lus et un menu
settings-general-unread-badge = Nombre de non-lus sur l’icône de la barre des tâches
settings-general-unread-badge-detail = Nombre de messages non lus dans la boîte de réception

## Settings > Inbox

settings-inbox-tabs = Onglets de la boîte de réception
settings-inbox-tabs-detail = Triez la boîte de réception en onglets, comme le fait le site Web de votre fournisseur de messagerie.
settings-inbox-tabs-show = Afficher les onglets de la boîte de réception
settings-inbox-tabs-show-detail = Désactivé, une seule liste pour tous les comptes
settings-inbox-no-accounts = Ajoutez un compte pour choisir ses onglets.
settings-inbox-tabs-automatic = Automatique : { $tabs } ({ $provider })
settings-inbox-tabs-off = Aucun onglet
settings-inbox-tabs-gmail = Principale, Promotions, Réseaux sociaux, Notifications, Forums
settings-inbox-tabs-focused = Prioritaire et Autres
settings-inbox-tabs-zoho = Boîte de réception, Newsletters et Notifications
settings-inbox-tabs-shown = Onglets affichés. Les messages d’un onglet désactivé restent dans { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Volet de lecture
settings-appearance-reading-pane-detail = Emplacement d’une conversation ouverte.
settings-appearance-pane-right = À droite de la liste
settings-appearance-pane-none = Aucune séparation
settings-appearance-density = Densité
settings-appearance-density-default = Par défaut
settings-appearance-density-compact = Compacte
settings-appearance-scaling = Mise à l’échelle
settings-appearance-scaling-detail = Agrandit ou réduit tout dans Katna Mail, en plus de l’échelle du bureau : texte, icônes, espacements et séparateurs. Les messages que vous envoyez gardent leur propre taille de police. Avec de très petites tailles, il peut être difficile de cliquer sur les icônes.
settings-appearance-theme = Thème
settings-appearance-theme-system = Système
settings-appearance-theme-light = Clair
settings-appearance-theme-dark = Sombre
settings-appearance-desktop-colors = Couleurs du bureau
settings-appearance-desktop-colors-use = Utiliser les couleurs du bureau
settings-appearance-desktop-colors-use-detail = Le jeu de couleurs et la couleur d’accentuation du bureau
settings-appearance-app-names = Noms des applications
settings-appearance-app-names-show = Afficher le nom des applications
settings-appearance-app-names-show-detail = Noms sous les icônes des applications, tout à gauche
settings-appearance-sender-pictures = Photos des expéditeurs
settings-appearance-sender-pictures-show = Afficher les logos des entreprises
settings-appearance-sender-pictures-show-detail = Recherchés d’après le domaine de l’expéditeur, jamais d’après le message, et conservés une semaine
settings-appearance-important = Marqueurs d’importance
settings-appearance-important-show = Afficher les marqueurs d’importance
settings-appearance-important-show-detail = À côté de chaque message de la liste
settings-appearance-message-width = Largeur des messages
settings-appearance-message-width-limit = Limiter la largeur des messages
settings-appearance-message-width-limit-detail = Les lignes longues sont plus faciles à lire dans une fenêtre large
settings-appearance-mail-colors = Couleurs des messages
settings-appearance-mail-colors-detail = La plupart des messages sont conçus pour une page blanche. Avec un thème sombre, leurs couleurs sont remplacées par des couleurs sombres bien lisibles ; si l’option est désactivée, ils gardent les couleurs de l’expéditeur sur une page claire.
settings-appearance-dark-mail = Couleurs sombres aussi pour les messages
settings-appearance-dark-mail-detail = Uniquement avec le thème sombre
settings-appearance-attachment-previews = Aperçus des pièces jointes
settings-appearance-attachment-previews-show = Afficher un aperçu des pièces jointes
settings-appearance-attachment-previews-show-detail = Une miniature du contenu de chaque fichier sur sa fiche

## Settings > Default apps

settings-default-apps-intro = Où s’ouvrent les pièces jointes quand vous cliquez dessus. La visionneuse peut aussi toujours ouvrir un fichier dans une autre application. Les applications par défaut du bureau se règlent dans ses propres paramètres.
settings-default-apps-pdf = Fichiers PDF
settings-default-apps-pdf-detail = Pages, avec zoom.
settings-default-apps-pictures = Images
settings-default-apps-pictures-detail = Photos (redressées), PNG, GIF, WebP, BMP, TIFF et SVG.
settings-default-apps-text = Fichiers texte
settings-default-apps-text-detail = Texte brut, journaux, code et autres textes.
settings-default-apps-sheets = Feuilles de calcul
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) et CSV.
settings-default-apps-documents = Documents
settings-default-apps-documents-detail = Word (docx, doc), texte OpenDocument (odt) et présentations (pptx, ppt, odp).
settings-default-apps-katna = Visionneuse de Katna Mail
settings-default-apps-system = Application par défaut du bureau
settings-default-apps-ask = Demander l’application à chaque fois
settings-default-apps-after-saving = Après l’enregistrement
settings-default-apps-show-folder = Afficher les fichiers enregistrés dans leur dossier
settings-default-apps-show-folder-detail = Ouvre le gestionnaire de fichiers avec les pièces jointes enregistrées sélectionnées

## Settings > Compose

settings-compose-send-from = Envoyer les nouveaux messages depuis
settings-compose-send-from-detail = Les réponses et les transferts partent toujours du compte dans lequel vous vous trouvez.
settings-compose-send-from-current = Le compte dans lequel vous êtes
settings-compose-send-on-replies = Envoi des réponses
settings-compose-send-on-replies-detail = Ce que fait Envoyer pour une réponse ou un transfert. Le menu à côté d’Envoyer propose l’autre option.
settings-compose-send-plain = Envoyer
settings-compose-send-archive = Envoyer et archiver
settings-compose-signatures = Signatures
settings-compose-signatures-detail = Ajoutée sous votre message, après une ligne « -- ». Choisissez-en une autre dans la fenêtre de rédaction.
settings-compose-untitled = Sans titre
settings-compose-signature-name = Nom, par exemple Travail
settings-compose-signature-first = Ma signature
settings-compose-signature-numbered = Signature { $number }
settings-compose-signature-delete = Supprimer
settings-compose-signature-deleted = Signature supprimée
settings-compose-signature-new = Créer
settings-compose-no-signatures = Aucune signature pour l’instant.
settings-compose-no-signature = Aucune signature
settings-compose-for-new-mail = Pour les nouveaux messages
settings-compose-for-replies = Pour les réponses et les transferts
settings-compose-for-replies-detail = Dans une conversation où vous avez signé un message, une réponse commence plutôt par cette signature.
settings-compose-format = Format
settings-compose-plain-text = Écrire en texte brut
settings-compose-plain-text-detail = Les nouveaux messages commencent sans mise en forme ; la fenêtre de rédaction permet d’en changer
settings-compose-spelling = Orthographe
settings-compose-spell-check = Vérifier l’orthographe pendant la saisie
settings-compose-spell-check-detail = Les mots mal orthographiés sont soulignés, avec des suggestions par clic droit
settings-compose-spell-desktop = Langue du bureau ({ $language })
settings-compose-templates = Modèles
settings-compose-templates-detail = Enregistrez les messages que vous écrivez souvent, et partez-en pour un nouveau message ou une réponse.

## Settings > Shortcuts

settings-shortcuts-set = Jeu de raccourcis
settings-shortcuts-set-detail = Partez des touches d’une application de messagerie que vous connaissez. Ici, Cmd correspond à Ctrl. Vos propres modifications s’appliquent par-dessus le jeu, et Rétablir les valeurs par défaut revient aux touches du jeu.
settings-shortcuts-single = Raccourcis à une touche
settings-shortcuts-single-detail = Des touches sans Ctrl ni Alt, comme dans un webmail : e archive, j et k déplacent, / recherche. Ils fonctionnent dans la liste et dans la conversation ouverte, jamais pendant la saisie.
settings-shortcuts-single-use = Utiliser les raccourcis à une touche
settings-shortcuts-single-use-detail = Les raccourcis avec Ctrl fonctionnent toujours
settings-shortcuts-how = Cliquez sur une touche pour la modifier, ou sur + pour en ajouter une, puis appuyez sur les nouvelles touches. Échap annule.
settings-shortcuts-restore = Rétablir les valeurs par défaut
settings-shortcuts-no-key = Aucune touche
settings-shortcuts-press = Appuyez sur des touches…
settings-shortcuts-then = { $keys } puis…
settings-shortcuts-moved = { $keys } effectue désormais « { $action } » au lieu de « { $previous } ».
settings-shortcuts-single-off = Les raccourcis à une touche sont désactivés ; cette touche fonctionnera une fois qu’ils seront activés.
settings-shortcuts-restored = Tous les raccourcis ont retrouvé les touches de leur jeu.

## Settings search: the line under a result

settings-general-language-summary = Langue de l’application, des dates et des nombres
settings-general-reading-summary = Message le plus récent en premier, en-têtes complets, noms complets des destinataires
settings-general-mark-read-summary = Quand une conversation ouverte est marquée comme lue : immédiatement, après 1 ou 3 secondes, ou manuellement
settings-general-reply-button-summary = Le bouton de réponse à côté de chaque message répond à tous
settings-general-remote-images-summary = Toujours afficher les images de tous les messages
settings-general-sending-summary = Annuler l’envoi : délai pendant lequel un message envoyé patiente, pour pouvoir être annulé
settings-general-offline-summary = Nombre de jours de messages récents téléchargés en entier, pour les lire sans connexion
settings-general-notifications-summary = Notifications de nouveaux messages et leur son
settings-general-desktop-summary = Démarrer Katna à la connexion, l’icône de la zone de notification et le nombre de non-lus sur l’icône de la barre des tâches
settings-accounts-accounts-summary = Ajouter ou supprimer un compte, ou changer sa photo
settings-appearance-density-summary = Lignes par défaut ou compactes dans la liste
settings-appearance-scaling-summary = Tout agrandir ou réduire : texte, icônes, espacements et séparateurs
settings-appearance-theme-summary = Système, clair ou sombre
settings-appearance-sender-pictures-summary = Logos des entreprises, recherchés d’après le domaine de l’expéditeur
settings-appearance-important-summary = Le marqueur d’importance à côté de chaque message de la liste
settings-appearance-mail-colors-summary = Couleurs sombres pour les messages HTML avec un thème sombre, ou couleurs de l’expéditeur
settings-appearance-attachment-previews-summary = Une miniature du contenu de chaque pièce jointe
settings-shortcuts-set-summary = Partir des touches de Gmail, Inbox by Gmail, Apple Mail, Outlook ou Thunderbird
settings-shortcuts-single-summary = Des touches sans Ctrl ni Alt, comme dans un webmail
settings-default-apps-pdf-summary = Où s’ouvrent les pièces jointes PDF
settings-default-apps-pictures-summary = Où s’ouvrent les photos et les images
settings-default-apps-text-summary = Où s’ouvrent le texte brut, les journaux et le code
settings-default-apps-sheets-summary = Où s’ouvrent les fichiers Excel, OpenDocument et CSV
settings-default-apps-documents-summary = Où s’ouvrent les textes Word et OpenDocument et les présentations
settings-default-apps-after-saving-summary = Afficher les pièces jointes enregistrées dans leur dossier
settings-compose-send-from-summary = Le compte d’où partent les nouveaux messages : celui dans lequel vous êtes, ou toujours le même
settings-compose-send-on-replies-summary = Envoyer, ou Envoyer et archiver la conversation, pour les réponses et les transferts
settings-compose-signatures-summary = Ajoutée sous votre message, après une ligne « -- »
settings-compose-for-new-mail-summary = La signature par laquelle commencent les nouveaux messages
settings-compose-for-replies-summary = La signature par laquelle commencent les réponses et les transferts
settings-compose-format-summary = Écrire les nouveaux messages en texte brut
settings-compose-spelling-summary = Vérifier l’orthographe pendant la saisie, et la langue du dictionnaire
settings-compose-templates-summary = Bientôt : enregistrer les messages que vous écrivez souvent, et en partir pour un nouveau message ou une réponse
settings-feedback-crash-reports-summary = Enregistrer des rapports de plantage sur cet ordinateur quand Katna Mail ou son service d’arrière-plan plante
settings-feedback-saved-summary = Afficher, copier ou supprimer les rapports de plantage enregistrés sur cet ordinateur
settings-feedback-help-improve-summary = Envoyer les rapports de plantage pour aider à corriger le problème ; désactivé sauf si vous l’activez
settings-experimental-blur-summary = Le bureau transparaît, flouté, à travers la barre supérieure, et les menus sont en verre dépoli
settings-search-shortcut = Raccourci clavier
settings-search-tab = Onglet des paramètres
settings-search-none = Aucun paramètre ne correspond à « { $query } ».
settings-search-results = Paramètres correspondant à « { $query } »

## Settings: opening at login

settings-open-at-login-failed = Impossible de modifier le démarrage à la connexion : { $error }

## Settings > General > Time

settings-time = Heure
settings-clock-language = Selon l’usage de la langue
settings-clock-12 = 12 heures, par exemple 2:05 PM
settings-clock-24 = 24 heures, par exemple 14:05
settings-time-summary = Format 12 heures ou 24 heures, ou selon l’usage de la langue

## Settings > General > Default mail app, Settings > Compose > Grammar

settings-general-mail-app = Application de messagerie par défaut
settings-general-mail-app-detail = Les liens e-mail des autres applications et des sites web ouvrent ici un nouveau message.
mail-app-is-default = Katna Mail est votre application de messagerie par défaut.
mail-app-is-other = Les liens e-mail s’ouvrent dans une autre application.
mail-app-make-default = Définir par défaut
mail-app-make-default-failed = Impossible de changer l’application de messagerie par défaut.
settings-general-mail-app-summary = Ouvrir dans Katna Mail les liens e-mail des autres applications et des sites web
settings-compose-grammar = Grammaire
settings-compose-grammar-detail = Vérifiée sur cet ordinateur avec Harper. En anglais uniquement pour l’instant : le texte dans d’autres langues n’est pas modifié.
settings-compose-grammar-check = Vérifier la grammaire
settings-compose-grammar-check-detail = Souligner les fautes de grammaire pendant la saisie, en anglais
settings-compose-suggestions = Suggestions d’écriture
settings-compose-suggestions-detail = Apprises sur cet ordinateur à partir des messages que vous avez envoyés et de ceux auxquels vous répondez ; rien n’en sort. Appuyez sur Tab pour accepter une suggestion, ou continuez à écrire.
settings-compose-suggestions-on = Suggérer pendant la saisie
settings-compose-suggestions-on-detail = Afficher en gris la suite probable d’une phrase pendant la saisie
settings-compose-grammar-summary = Souligner les fautes de grammaire pendant la saisie, en anglais
settings-compose-suggestions-summary = Afficher en gris la suite probable d’une phrase pendant la saisie
