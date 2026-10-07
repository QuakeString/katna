# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = Adiar até…
snooze-later-today = Mais tarde hoje
snooze-tomorrow = Amanhã
snooze-this-weekend = Neste fim de semana
snooze-next-week = Na próxima semana
snooze-pick = Escolher data e hora
snooze-back = Voltar aos horários
snooze-type-placeholder = Digite um horário
snooze-type-hint = Como “ter 15h”, “amanhã” ou “em 2 horas”
snooze-type-hint-unclear = O Katna não consegue entender isso como um horário
snooze-type-unclear = “{ $text }” não é um horário que o Katna conheça

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = Adiar
remind-tab = Lembrar-me
snooze-says = Oculta até lá
remind-says = Mantém onde está e notifica você
remind-before-due = Antes do prazo
remind-note = Nota (opcional)
remind-note-placeholder = O assunto, se ficar vazia
toast-remind-set = Lembrete definido para { $date }
remind-chat-line = Lembrete { $date } · { $title }
remind-done = Concluído
toast-remind-done = Lembrete concluído
snooze-chat-line = Adiado até { $date }
snooze-chat-change = Alterar

## The date and time picker

snooze-cancel = Cancelar
snooze-save = Salvar
snooze-in-the-past = Escolha um horário depois de agora.

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = Acompanhar se não houver resposta…
follow-up-title = Acompanhar se não houver resposta
follow-up-off = Desativado
follow-up-days = { $days ->
    [one] { $days } dia
    [many] { $days } de dias
   *[other] { $days } dias
}
follow-up-weeks = { $weeks ->
    [one] { $weeks } semana
    [many] { $weeks } de semanas
   *[other] { $weeks } semanas
}
follow-up-pick = Escolher…
follow-up-pick-title = Acompanhar se não houver resposta até
follow-up-remind = Lembrar-me
follow-up-remind-note = A conversa volta para o topo da sua Caixa de entrada
follow-up-send = Enviar um acompanhamento por mim
follow-up-send-note = Para as mesmas pessoas, na mesma conversa
follow-up-send-encrypted = Não para e-mails criptografados
follow-up-text-placeholder = O que escrever
follow-up-text-named = Olá, { $name }, só confirmando se você viu minha mensagem abaixo.
follow-up-text = Olá, só confirmando se você viu minha mensagem abaixo.
follow-up-template = Usar um modelo
follow-up-signature = Sua assinatura é adicionada
follow-up-again = Se ainda não houver resposta, acompanhar de novo depois de
follow-up-note = Para assim que alguém na conversa responder. Respostas automáticas não contam.
follow-up-note-send = Para assim que alguém na conversa responder. Sai em dias úteis das { $start } às { $end }, e nunca com mais de um dia de atraso.
follow-up-cancel = Cancelar
follow-up-done = Concluído
follow-up-chip-send = Acompanhamento em { $time }
follow-up-chip-remind = Lembrete em { $time }
follow-up-chip-send-on = Acompanhamento { $date }
follow-up-chip-remind-on = Lembrete { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = Ainda sem resposta
follow-up-card-title-waiting = Seu acompanhamento está esperando
follow-up-card-send = O Katna envia seu acompanhamento em { $date }. Ele para quando alguém responder.
follow-up-card-send-twice = O Katna envia seu acompanhamento em { $date } e mais uma vez depois. Ele para quando alguém responder.
follow-up-card-remind = Se ninguém responder, esta conversa volta para a sua Caixa de entrada em { $date }.
follow-up-card-waiting = O prazo venceu enquanto seu computador estava desligado, então ele não foi enviado com atraso. Envie agora, escolha um novo horário ou cancele.
follow-up-card-edit = Editar
follow-up-card-edit-title = Acompanhar em
follow-up-card-send-now = Enviar agora
follow-up-card-stop = Parar
follow-up-chat-send = Acompanhamento · { $date } se ninguém responder
follow-up-chat-step = Acompanhamento { $step } de { $steps } · { $date } se ninguém responder
follow-up-chat-waiting = Acompanhamento esperando · o prazo venceu enquanto seu computador estava desligado
follow-up-chat-remind = De volta na Caixa de entrada { $date } se não houver resposta
toast-follow-up-sent = Acompanhamento enviado
toast-follow-up-stopped = Acompanhamento cancelado
toast-follow-up-moved = Acompanhamento remarcado para { $date }

nudge-row = Enviado { $days ->
    [one] há 1 dia
    [many] há { $days } de dias
   *[other] há { $days } dias
}. Acompanhar?
nudge-row-tip = Escrever um acompanhamento para todos na conversa
nudge-follow-up = Acompanhar
nudge-dismiss = Dispensar
nudge-card-title = Ainda sem resposta
nudge-card-text = Você perguntou algo { $days ->
    [one] há 1 dia
    [many] há { $days } de dias
   *[other] há { $days } dias
} e ninguém respondeu.
nudge-chat-line = Enviado { $days ->
    [one] há 1 dia
    [many] há { $days } de dias
   *[other] há { $days } dias
}, ainda sem resposta
toast-nudge-dismissed = Lembrete de resposta dispensado
