# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = Ouvrir la _boîte de réception
tray-new-message = _Nouveau message
tray-new-task = Nouvelle _tâche
tray-new-note = Nouvelle _note
tray-preferences = _Préférences
tray-quit = _Quitter

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] Aucun message non lu
    [one] { $count } message non lu
    [many] { $count } de messages non lus
   *[other] { $count } messages non lus
}

tray-password-refused = Nouveau mot de passe requis pour { $address }
tray-signed-out = Reconnectez-vous à { $address }
tray-accounts-need-you = { $count } comptes ont besoin de vous
tray-not-sent = { $count ->
    [one] { $count } message n’a pas été envoyé
    [many] { $count } de messages n’ont pas été envoyés
   *[other] { $count } messages n’ont pas été envoyés
}
