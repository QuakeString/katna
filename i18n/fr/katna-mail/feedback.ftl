# Katna Mail, French (Français).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > User feedback (crash reports)

feedback-intro-sending = Les nouveaux rapports de plantage sont envoyés pour aider à corriger le problème. Rien d’autre ne quitte cet ordinateur.
feedback-intro-local = Katna n’envoie rien nulle part. Les rapports de plantage restent sur cet ordinateur, pour que vous puissiez les consulter ou les joindre à un rapport de bug.
feedback-crash-reports = Rapports de plantage
feedback-crash-reports-detail = Créés quand Katna Mail ou son service d’arrière-plan plante.
feedback-save = Enregistrer les rapports de plantage sur cet ordinateur
feedback-save-detail = Votre dossier personnel, vos noms d’utilisateur et d’ordinateur et les adresses e-mail en sont retirés
feedback-saved = Rapports de plantage enregistrés
feedback-saved-detail = { $count ->
    [one] Seul le plus récent est conservé.
    [many] Les { $count } de plus récents sont conservés.
   *[other] Les { $count } plus récents sont conservés.
}
feedback-help-improve = Aider à améliorer Katna
feedback-help-improve-detail = Désactivé sauf si vous l’activez, et vous pouvez le désactiver ici à tout moment.
feedback-send = Envoyer les rapports de plantage
feedback-send-detail = Le rapport enregistré, tel que vous pouvez le voir ici, est envoyé à l’outil de suivi des plantages de Katna (Sentry, dans l’UE). Aucune adresse IP, aucun message ni aucune adresse e-mail
feedback-none-saved = Aucun rapport de plantage enregistré.
feedback-delete-all = Tout supprimer
feedback-app-daemon = Service d’arrière-plan
feedback-report-sent = { $date } · Envoyé
feedback-view = Afficher
feedback-view-tooltip = Ouvrir le rapport
feedback-copy-tooltip = Le copier pour le coller dans un rapport de bug
feedback-copied = Rapport de plantage copié.
feedback-deleted-all = Rapports de plantage supprimés.
feedback-read-failed = Impossible de lire le rapport de plantage : { $error }
feedback-delete-failed = Impossible de supprimer le rapport de plantage : { $error }
feedback-delete-all-failed = Impossible de supprimer les rapports de plantage : { $error }

## Settings > User feedback (usage statistics)

feedback-usage = Envoyer des statistiques d’utilisation anonymes
feedback-usage-detail = Une fois par semaine : quelles fonctions vous avez utilisées, oui ou non. Jamais de nombres, d’adresses, de noms ni de mots recherchés
feedback-intro-sending-usage = Les rapports de plantage et les statistiques d’utilisation hebdomadaires sont envoyés. Rien d’autre ne quitte cet ordinateur.
feedback-intro-usage-only = Les statistiques d’utilisation hebdomadaires sont envoyées. Les rapports de plantage restent sur cet ordinateur.
feedback-counted = Ce qui est compté
feedback-counted-detail = Chaque élément vaut oui ou non pour la semaine.
feedback-counted-also = Également : la version de Katna, la famille Linux, le bureau, l’échelle de l’écran et le nombre de comptes (1, 2–3, 4+)
feedback-see-report = Voir le rapport de cette semaine
feedback-hide-report = Masquer le rapport de cette semaine
feedback-report-goes = Envoyé à la fin de la semaine, le { $date }, si les statistiques d’utilisation sont toujours activées.
feedback-install-id = ID d’installation { $id }
feedback-install-id-tooltip = Aléatoire, pour qu’un même ordinateur ne soit pas compté deux fois dans une semaine. Il change tous les 90 jours et n’est jamais envoyé avec les rapports de plantage ni les retours
feedback-install-id-reset = Réinitialiser
feedback-install-id-new = Nouvel ID d’installation créé.
feedback-report-copied = Rapport copié.
feedback-send-feedback = Retours
feedback-send-feedback-detail = Un problème, une idée, n’importe quoi.
feedback-send-feedback-button = Envoyer un retour…
usage-feature-search-options = Options de recherche
usage-feature-pins = Messages épinglés
usage-feature-labels = Libellés
usage-feature-scheduled-send = Envoi programmé
usage-feature-snooze = Mise en attente et rappels
usage-feature-encrypted = Messages chiffrés
usage-feature-viewers = Visionneuses intégrées
usage-feature-calendar = Calendrier
usage-feature-contacts = Contacts
usage-feature-tasks-notes = Tâches et Notes
usage-feature-phone-layout = Disposition pour téléphone
usage-feature-own-frame = Cadre de fenêtre propre à Katna

## Help > Send feedback

send-feedback-title = Envoyer un retour
send-feedback-about = Sujet
send-feedback-problem = Problème
send-feedback-idea = Idée
send-feedback-other = Autre chose
send-feedback-message = Votre message
send-feedback-message-placeholder = Que s’est-il passé, ou que souhaiteriez-vous ?
send-feedback-reply = E-mail pour une réponse (facultatif)
send-feedback-reply-placeholder = vous@example.org
send-feedback-system = Inclure la version de Katna et votre système
send-feedback-what-is-sent = Ce qui est envoyé
send-feedback-show = Afficher
send-feedback-hide = Masquer
send-feedback-where = Envoyé à la boîte de retours de Katna chez Sentry (UE). Aucune adresse IP, aucun compte, aucun message ni ID d’installation.
send-feedback-cancel = Annuler
send-feedback-send = Envoyer
send-feedback-sending = Envoi…
send-feedback-sent = Retour envoyé. Merci
send-feedback-failed = Impossible d’envoyer le retour : { $error }
