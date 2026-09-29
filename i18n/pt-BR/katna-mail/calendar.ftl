# Katna Mail, Portuguese (Brazil) (Português (Brasil)): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Hoje
calendar-today-tip = Ir para hoje
calendar-view-day = Dia
calendar-view-week = Semana
calendar-view-month = Mês
calendar-view-year = Ano
calendar-view-schedule = Programação
calendar-view-days =
    { $count ->
        [one] { $count } dia
        [many] { $count } de dias
       *[other] { $count } dias
    }
calendar-options = Opções
calendar-density = Densidade
calendar-density-responsive = Responsivo à tela
calendar-density-comfortable = Confortável
calendar-density-compact = Compacto
calendar-custom-days = Visualização personalizada
calendar-second-zone = Segundo fuso horário
calendar-zone-none = Nenhum
calendar-zone = { $zone } ({ $offset })
calendar-share-free = Compartilhar horários livres
calendar-free-subject = Horários em que estou livre
calendar-free-intro = Aqui estão alguns horários em que estou livre ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = Não tenho horário livre nos próximos dias úteis.
calendar-previous-day = Dia anterior
calendar-next-day = Próximo dia
calendar-previous-week = Semana anterior
calendar-next-week = Próxima semana
calendar-previous-month = Mês anterior
calendar-next-month = Próximo mês
calendar-previous-year = Ano anterior
calendar-next-year = Próximo ano
calendar-previous-period = Mais cedo
calendar-next-period = Mais tarde
calendar-title-months = { $first } – { $last }
calendar-loading = Carregando…
calendar-read-failed = Não foi possível ler a agenda: { $error }
calendar-sets = Conjuntos de agendas
calendar-set-add = Salvar as agendas exibidas como um conjunto
calendar-set-name = Nome do conjunto
calendar-set-remove = Remover conjunto
calendar-local = Este computador
calendar-account-gone = Conta removida
calendar-birthdays = Aniversários
calendar-birthday-of = Aniversário de { $name }
calendar-empty-title = Ainda não há agendas
calendar-empty-text = O Katna mostra aqui as agendas das suas contas do Google e da Microsoft assim que forem sincronizadas, e as de outros servidores que oferecem CalDAV.
calendar-schedule-empty = Nada planejado para os próximos dois meses.
calendar-no-title = (Sem título)
calendar-all-day = Dia inteiro
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = mais { $count }
calendar-repeats = Se repete
calendar-join = Participar
calendar-email-guests = Enviar e-mail aos convidados
calendar-running-late = Vou me atrasar
calendar-late-subject = Atraso: { $title }
calendar-late-body = Desculpem, vou me atrasar alguns minutos para { $title }. Chego logo.
calendar-guests =
    { $count ->
        [one] { $count } convidado
        [many] { $count } de convidados
       *[other] { $count } convidados
    }
calendar-guest-answers = { $yes } sim, { $maybe } talvez, { $no } não, { $waiting } aguardando
calendar-organizer = Organizador
calendar-optional = Opcional
calendar-open-web = Abrir no navegador
calendar-open-contact = Abrir contato
calendar-close = Fechar

## Adding, changing and deleting events.

calendar-add-title = Adicionar título
calendar-add-location = Adicionar local
calendar-add-notes = Adicionar descrição
calendar-add-guests = Adicionar convidados
calendar-remove-guest = Remover
calendar-add-meet = Adicionar videoconferência do Google Meet
calendar-add-teams = Adicionar reunião do Teams
calendar-has-call = Videochamada adicionada
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Dia inteiro
calendar-more-options = Mais opções
calendar-save = Salvar
calendar-saved = Evento salvo
calendar-deleted = Evento excluído
calendar-discard = Descartar alterações
calendar-edit = Editar evento
calendar-delete = Excluir evento
calendar-event-details = Detalhes do evento
calendar-kind-event = Evento
calendar-kind-focus = Tempo de foco
calendar-kind-out-of-office = Ausente
calendar-kind-working-location = Local de trabalho
calendar-working-home = Casa
calendar-busy = Ocupado
calendar-free = Disponível
calendar-cancel = Cancelar
calendar-ok = OK
calendar-read-only = Você não pode alterar eventos nesta agenda
calendar-none-editable = Ainda não há nenhuma agenda à qual você possa adicionar eventos
calendar-no-such-time = Esse horário não existe no seu fuso horário
calendar-end-before-start = O evento termina antes de começar
calendar-repeat-never = Não se repete
calendar-repeat-daily = Diariamente
calendar-repeat-weekly = Semanalmente: { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Mensalmente: { $weekday }, primeira semana
        [2] Mensalmente: { $weekday }, segunda semana
        [3] Mensalmente: { $weekday }, terceira semana
        [4] Mensalmente: { $weekday }, quarta semana
       *[other] Mensalmente: { $weekday }, última semana
    }
calendar-repeat-yearly = Anualmente em { $day }
calendar-repeat-weekdays = Todos os dias úteis (segunda a sexta-feira)
calendar-repeat-custom = Personalizado
calendar-reminder-none = Nenhuma notificação
calendar-reminder-at-start = No início
calendar-reminder-minutes =
    { $count ->
        [one] { $count } minuto antes
        [many] { $count } de minutos antes
       *[other] { $count } minutos antes
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } hora antes
        [many] { $count } de horas antes
       *[other] { $count } horas antes
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } dia antes
        [many] { $count } de dias antes
       *[other] { $count } dias antes
    }
calendar-scope-edit-title = Editar evento recorrente
calendar-scope-delete-title = Excluir evento recorrente
calendar-scope-this = Este evento
calendar-scope-following = Este e os próximos eventos
calendar-scope-all = Todos os eventos
calendar-scope-respond-title = Resposta para um evento recorrente
calendar-going = Você vai?
calendar-answer-yes = Sim
calendar-answer-no = Não
calendar-answer-maybe = Talvez
calendar-answered-yes = Você vai
calendar-answered-no = Você não vai
calendar-answered-maybe = Você talvez vá

## The card at the top of a mail with an invitation.

calendar-invite = Convite
calendar-invite-cancelled = Evento cancelado
calendar-invite-reply = { $name } respondeu
calendar-invite-reply-yes = { $name } aceitou
calendar-invite-reply-no = { $name } recusou
calendar-invite-reply-maybe = { $name } talvez vá
calendar-invite-organizer = Organizado por { $name }
calendar-invite-open = Abrir na Agenda
calendar-invite-not-yet = Ainda não está na sua agenda. Você poderá responder depois da sincronização.
calendar-invite-by-mail = Não está na sua agenda: sua resposta vai para o organizador por e-mail.
calendar-mail-yes = Aceito: { $title }
calendar-mail-yes-body = { $name } aceitou este convite.
calendar-mail-no = Recusado: { $title }
calendar-mail-no-body = { $name } recusou este convite.
calendar-mail-maybe = Talvez: { $title }
calendar-mail-maybe-body = { $name } respondeu Talvez a este convite.
calendar-invite-your-day = Seu dia
calendar-invite-clashes =
    { $count ->
        [one] Conflita com { $count } evento
        [many] Conflita com { $count } de eventos
       *[other] Conflita com { $count } eventos
    }

## The day's agenda beside the mail.

agenda-show = Mostrar a agenda do dia
agenda-hide = Ocultar a agenda
agenda-today = Hoje, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = Nada planejado para este dia.
