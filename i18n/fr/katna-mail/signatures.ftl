# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The signature being edited

signature-placeholder = Votre nom, et ce que vous voulez ajouter en dessous

## Its formatting bar

signature-bold = Gras
signature-italic = Italique
signature-underline = Souligné
signature-link = Lien
signature-link-apply = Appliquer
signature-picture = Insérer une image
signature-align-left = Aligner à gauche
signature-align-center = Centrer
signature-align-right = Aligner à droite
signature-numbered-list = Liste numérotée
signature-bulleted-list = Liste à puces
signature-remove-formatting = Effacer la mise en forme

## Adding a picture

signature-picture-choose = Insérer
signature-picture-too-big = Les images d’une signature peuvent faire jusqu’à { $size }.
signature-picture-kind = Choisissez une image PNG, JPEG, GIF ou WebP.
signature-picture-unreadable = { $name } : { $error }

## Layouts

signature-layout = Mise en page
signature-layout-own = La vôtre
signature-layout-classic = Classique
signature-layout-logo-left = Logo à gauche
signature-layout-photo = Photo
signature-layout-band = Bande de couleur
signature-layout-one-line = Une ligne
signature-layout-centred = Centrée
signature-layout-banner = Avec bannière
signature-layout-underline = Soulignée
signature-layout-side-bar = Barre latérale
signature-layout-card = Carte
signature-layout-monogram = Monogramme
signature-layout-plain = Texte brut
signature-layout-mobile-label = M :
signature-layout-office-label = B :
signature-layout-email-label = E :
signature-layout-name = Nom
signature-layout-job = Fonction
signature-layout-company = Entreprise
signature-layout-mobile = Mobile
signature-layout-office = Bureau
signature-layout-email = E-mail
signature-layout-website = Site web
signature-layout-address = Adresse
signature-layout-pictures = Images
signature-layout-logo = Logo
signature-layout-photo-picture = Photo
signature-layout-banner-picture = Bannière
signature-layout-remove-picture = Retirer
signature-layout-pages = Pages
signature-layout-page-placeholder = Ajouter l’adresse d’une page
signature-layout-colour = Couleur
signature-layout-picture-failed = { $name } n’a pas pu être utilisé comme image.
signature-layout-preview = Ce que voit le destinataire
signature-layout-light = Clair
signature-layout-dark = Sombre
signature-layout-text = Texte brut
signature-layout-inside = Les images sont envoyées dans le message, elles s’affichent donc même là où les images distantes sont désactivées. Celle-ci ajoute { $size } à chaque message.
signature-layout-edit = Modifier à la main
signature-layout-edit-confirm = La modifier à la main ? Ses champs et sa mise en page disparaissent, et elle garde son apparence autant que l’éditeur le permet.
signature-layout-use-confirm = Utiliser la mise en page { $layout } ? Elle remplace cette signature, remplie à partir de celle-ci.
signature-layout-use = Utiliser la mise en page
signature-layout-cancel = Annuler

## Paste HTML

signature-html-title = Coller du HTML
signature-html-subtitle = Pour une signature conçue ailleurs
signature-html-placeholder = Collez ici le HTML de la signature
signature-html-name = Collée
signature-html-new = Enregistrée comme nouvelle signature, « { $name } »
signature-html-replaces = Remplace « { $name } »
signature-html-cancel = Annuler
signature-html-save = Enregistrer
signature-html-fetching = Téléchargement de ses images…
signature-html-pictures-inside = { $count ->
    [one] { $count } image téléchargée et intégrée au message ({ $size })
    [many] { $count } d’images téléchargées et intégrées au message ({ $size })
   *[other] { $count } images téléchargées et intégrées au message ({ $size })
}
signature-html-pictures-web = { $count ->
    [one] { $count } image n’a pas pu être téléchargée, les destinataires la chargent donc depuis le web
    [many] { $count } d’images n’ont pas pu être téléchargées, les destinataires les chargent donc depuis le web
   *[other] { $count } images n’ont pas pu être téléchargées, les destinataires les chargent donc depuis le web
}
signature-html-removed = Scripts, formulaires et pixels de suivi retirés, que les applications de messagerie bloquent de toute façon
signature-html-style-sheet = Feuille de style ignorée : les messages ne conservent que les styles écrits sur chaque élément
signature-html-links = Liens retirés qui ne menaient ni à un site web, ni à une adresse, ni à un téléphone
signature-html-plain-text = Version texte brut créée à partir de celle-ci, pour les applications de messagerie qui n’affichent que le texte

## Import

signature-import-title = Importer
signature-import-subtitle = Depuis Gmail, Thunderbird, Evolution et KMail
signature-import-looking = Recherche de signatures…
signature-import-none = Aucune signature trouvée. Pour une autre application, copiez le HTML de sa signature et utilisez Coller du HTML.
signature-import-from = Depuis { $app }
signature-import-already = déjà dans Katna
signature-import-gmail-sign-in = { $address } : reconnectez-vous dans Paramètres > Comptes pour que Katna puisse lire les signatures Gmail.
signature-import-gmail-failed = { $address } : { $error }
signature-import-cancel = Annuler
signature-import-do = { $count ->
    [one] Importer { $count } signature
    [many] Importer { $count } de signatures
   *[other] Importer { $count } signatures
}
signature-import-name = { $name } ({ $app })
