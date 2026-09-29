# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Langue : { $language }
language-tooltip-system = Langue : { $language }, selon le système
language-search = Rechercher une langue
language-system-default = Langue du système
language-system-now = Actuellement : { $language }
language-no-match = Aucune langue ne correspond à « { $query } »
language-machine = Traduction automatique. Aidez-nous à l’améliorer
language-setting = Langue
language-setting-detail = Langue des menus, des boutons et des messages, et format des dates et des nombres. « Langue du système » suit le bureau.

## Dates and sizes

ago-just-now = à l’instant
ago-minutes = { $count ->
    [one] il y a { $count } minute
    [many] il y a { $count } de minutes
   *[other] il y a { $count } minutes
}
ago-hours = { $count ->
    [one] il y a { $count } heure
    [many] il y a { $count } d’heures
   *[other] il y a { $count } heures
}
ago-days = { $count ->
    [one] il y a { $count } jour
    [many] il y a { $count } de jours
   *[other] il y a { $count } jours
}
size-bytes = { $count ->
    [one] { $count } octet
    [many] { $count } d’octets
   *[other] { $count } octets
}
size-kb = { $size } Ko
size-mb = { $size } Mo
size-gb = { $size } Go
size-tb = { $size } To

## Top bar

folders-hide = Masquer les dossiers
folders-show = Afficher les dossiers
side-pane-hide = Masquer le panneau latéral
side-pane-show = Afficher le panneau latéral
compose = Nouveau message
search = Rechercher
search-mail = Rechercher dans les messages
search-settings = Rechercher dans les paramètres
search-clear = Effacer la recherche
search-options-show = Afficher les options de recherche
settings = Paramètres
account-add = Ajouter un compte
account-wheel-hint = Faites défiler pour changer de compte
