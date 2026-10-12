# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = { $count ->
    [one] { $count } novo e-mail
    [many] { $count } de novos e-mails
   *[other] { $count } novos e-mails
}
notify-and-more = e mais { $count }
notify-no-subject = (sem assunto)
notify-unknown-sender = Remetente desconhecido

## Reminders the user asked for (same buttons)

notify-snooze-back = De volta do adiamento
notify-no-reply = Ainda sem resposta
notify-no-reply-to = Ninguém respondeu a “{ $subject }”.
notify-follow-up-sent = Acompanhamento enviado
notify-follow-up-sent-to = Ninguém tinha respondido a “{ $subject }”, então o Katna enviou um acompanhamento.
notify-follow-up-waiting = Acompanhamento não enviado
notify-follow-up-waiting-to = Ele estava previsto enquanto este computador estava desligado. “{ $subject }” voltou para a sua Caixa de entrada.

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } abriu { $subject }
notify-tracking-clicked = { $who } clicou em um link em { $subject }

## An update of Katna is downloaded and ready to install

notify-update-ready = O Katna Mail pode ser atualizado
notify-update-ready-body = A versão { $version } foi baixada. Atualizar a instala e reinicia o Katna Mail.
notify-update = Atualizar

## Something needs the user, shown once per problem

notify-signed-out = Faça login de novo
notify-signed-out-body = O { $provider } desconectou o Katna de { $address }. Os e-mails pararam de sincronizar.
notify-sign-in = Fazer login
notify-password-refused = Senha recusada
notify-password-refused-body = O servidor de e-mail recusou a senha de { $address }. Ela pode ter mudado.
notify-new-password = Nova senha
notify-not-sent = “{ $subject }” não foi enviado
notify-not-sent-no-subject = Uma mensagem não foi enviada
notify-not-sent-body = Ela está na Caixa de saída, que explica o motivo.
notify-open-outbox = Abrir a Caixa de saída

## Reminders of calendar events

notify-event-now = Agora
notify-event-in-minutes = { $count ->
    [one] Em { $count } minuto
    [many] Em { $count } de minutos
   *[other] Em { $count } minutos
}
notify-event-in-hours = { $count ->
    [one] Em { $count } hora
    [many] Em { $count } de horas
   *[other] Em { $count } horas
}
notify-event-in-days = { $count ->
    [1] Amanhã
    [one] Em { $count } dia
    [many] Em { $count } de dias
   *[other] Em { $count } dias
}
notify-event-all-day = Dia inteiro
notify-event-join = Participar
notify-event-snooze = Adiar 5 min
notify-task-done = Marcar como concluída

## The buttons of new-mail notifications and reminders

notify-open = Abrir
notify-peek = Espiar
notify-reply = Responder
notify-reply-placeholder = Responder a { $name }…
notify-send = Enviar
notify-reply-quote-header = Em { $date }, { $from } escreveu:
notify-reply-quote-header-no-date = { $from } escreveu:
notify-reply-all = Responder a todos
notify-mark-read = Marcar como lida
notify-mark-all-read = Marcar todas como lidas
notify-archive = Arquivar
notify-snooze-hour = Adiar 1 hora
notify-snooze-tomorrow = Amanhã
notify-copy-code = Copiar { $code }
notify-link-verify = Verificar em { $domain }
notify-link-confirm = Confirmar em { $domain }
notify-link-activate = Ativar em { $domain }

## After Archive on a notification: a short note in the same place

notify-archived = Arquivado
notify-archived-count = { $count ->
    [one] { $count } mensagem saiu da Caixa de entrada
    [many] { $count } de mensagens saíram da Caixa de entrada
   *[other] { $count } mensagens saíram da Caixa de entrada
}
notify-undo = Desfazer

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = Código copiado
notify-code-not-copied = Não foi possível copiar o código

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = Resposta enviada para { $name }
notify-open-in-katna = Abrir no Katna
