# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } novo e-mail
    [many] { $count } de novos e-mails
   *[other] { $count } novos e-mails
}
notify-and-more = e mais { $count }
notify-no-subject = (sem assunto)
notify-unknown-sender = Remetente desconhecido
notify-snooze-back = De volta do adiamento
notify-no-reply = Ainda sem resposta
notify-no-reply-to = Ninguém respondeu a “{ $subject }”.
notify-tracking-opened = { $who } abriu { $subject }
notify-tracking-clicked = { $who } clicou em um link em { $subject }

notify-update-ready = O Katna Mail pode ser atualizado
notify-update-ready-body = A versão { $version } foi baixada. Atualizar a instala e reinicia o Katna Mail.
notify-update = Atualizar

## Its buttons

notify-open = Abrir
notify-reply-all = Responder a todos
notify-mark-read = Marcar como lida
notify-mark-all-read = Marcar todas como lidas
notify-archive = Arquivar
