# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

top-brand = Katna

## App rail (and the bottom bar on a phone)

rail-mail = E-mail
rail-calendar = Agenda
rail-contacts = Contatos
rail-tasks = Tarefas
rail-notes = Notas
rail-files = Arquivos

## Rail right-click menu

rail-menu-open = Abrir { $app }
rail-menu-settings = Configurações de { $app }
rail-menu-turn-off = Desativar { $app }…

## Turning an app off (Settings > Apps)

app-off-title = Desativar { $app }?
app-off-body = O Katna para de sincronizar { $app } e o remove de:
app-off-keep = Manter uma cópia neste computador
app-off-keep-detail = Reativar é instantâneo
app-off-remove = Remover a cópia deste computador
app-off-remove-detail = Nada muda nas suas contas, e reativar baixa tudo de novo. O que só está neste computador, ou ainda não foi enviado, continua aqui.
app-off-cancel = Cancelar
app-off-confirm = Desativar
app-off-done = { $app } desativado
app-off-note = { $app } está desativado
app-off-turn-on = Ativar
app-off-leaves-calendar-rail = A barra lateral e Ctrl+2
app-off-leaves-calendar-agenda = A agenda ao lado dos seus e-mails
app-off-leaves-calendar-meeting = Agendar reunião, e Abrir na Agenda nos convites
app-off-leaves-calendar-reminders = Lembretes de eventos
app-off-leaves-calendar-desktop = Eventos no KRunner e no relógio da área de trabalho
app-off-leaves-contacts-rail = A barra lateral e Ctrl+3
app-off-leaves-contacts-card = Adicionar aos contatos no cartão de um remetente
app-off-leaves-contacts-birthdays = Aniversários na Agenda
app-off-leaves-tasks-rail = A barra lateral e Ctrl+4
app-off-leaves-tasks-mail = Adicionar às Tarefas nos e-mails, e Shift+T
app-off-leaves-tasks-calendar = Tarefas na Agenda
app-off-leaves-tasks-tray = Nova tarefa na bandeja, e Meta+Alt+T
app-off-leaves-tasks-reminders = Lembretes de tarefas
app-off-leaves-notes-rail = A barra lateral e Ctrl+5
app-off-leaves-notes-mail = Adicionar uma nota nos e-mails
app-off-leaves-notes-meetings = Anotações de reunião nos eventos
app-off-leaves-notes-tray = Nova nota na bandeja, e Meta+Alt+N
app-off-leaves-notes-reminders = Lembretes de notas
app-off-leaves-files-rail = A barra lateral e Ctrl+7
app-off-leaves-files-compose = Arquivos ao anexar em Escrever

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Em breve
app-calendar-promise = Suas agendas CalDAV, convites de reunião do seu e-mail e lembretes, ao lado da sua caixa de entrada.
app-tasks-promise = Listas de tarefas sincronizadas com CalDAV e tarefas criadas a partir de e-mails.
app-notes-promise = Notas rápidas e notas sobre um e-mail ou uma conversa para depois.

## Contacts page

app-contacts-loading = Reunindo pessoas do seu e-mail…
app-contacts-empty = As pessoas com quem você troca e-mails aparecem aqui.
app-contacts-count = { $count ->
    [one] { $count } pessoa do seu e-mail, as mais frequentes primeiro
    [many] { $count } de pessoas do seu e-mail, as mais frequentes primeiro
   *[other] { $count } pessoas do seu e-mail, as mais frequentes primeiro
}
app-contacts-top = { $count ->
    [one] A pessoa mais frequente do seu e-mail
    [many] As { $count } de pessoas mais frequentes do seu e-mail, as mais frequentes primeiro
   *[other] As { $count } pessoas mais frequentes do seu e-mail, as mais frequentes primeiro
}
app-contacts-messages = { $count ->
    [one] { $count } mensagem
    [many] { $count } de mensagens
   *[other] { $count } mensagens
}
app-contacts-last = última vez: { $date }
