# Katna Mail, Portuguese (Brazil) (Português (Brasil)): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Nova tarefa
tasks-all = Todas as tarefas
tasks-today = Hoje
tasks-starred = Com estrela
tasks-new-list = Criar nova lista
tasks-on-this-computer = Neste computador
tasks-my-tasks = Minhas tarefas
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = Fazer login de novo para mostrar as tarefas
tasks-account-signed-in = Login feito de novo em { $address }. Buscando suas tarefas…
tasks-account-sign-in-refused = O { $provider } não deixou o Katna entrar. Tente de novo e permita o acesso às suas tarefas.
tasks-account-refused = O servidor não aceitou a senha. Yahoo, iCloud, Zoho e outros precisam de uma senha de app.
tasks-account-change-password = Alterar senha
tasks-account-change-password-tooltip = Abrir Configurações > Contas
tasks-account-not-enabled = O acesso do Katna às tarefas ainda não está ativado.
tasks-account-failed = Não foi possível ler as listas de tarefas.
# $reason is the server's own words, in English.
tasks-account-error = Não foi possível ler as listas de tarefas: { $reason }
tasks-account-none = Nenhuma lista de tarefas encontrada
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = Nenhuma lista de tarefas encontrada: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = O { $provider } só mostra as tarefas ao Katna conectado com o { $provider }.
tasks-account-sign-in-with = Fazer login com o { $provider }
tasks-account-looking = Procurando listas de tarefas…
tasks-account-try-again = Tentar de novo
tasks-account-try-again-tooltip = Verificar agora as tarefas desta conta de novo
tasks-account-fixing = Resolvendo…
tasks-list-name-placeholder = Nome da lista

## Lists and tasks

tasks-loading = Lendo suas tarefas…
tasks-no-lists = Suas listas de tarefas aparecem aqui.
tasks-search = Pesquisar tarefas
tasks-search-none = Nenhuma tarefa corresponde à sua pesquisa.
tasks-add = Adicionar uma tarefa
tasks-title-placeholder = Título
tasks-add-step = Adicionar uma subtarefa
tasks-empty = Nenhuma tarefa ainda. Adicione uma acima.
tasks-starred-empty = Marque uma tarefa com estrela para vê-la aqui.
tasks-today-empty = Nada vence hoje.
tasks-today-date = { $weekday }, { $day }
tasks-overdue = Atrasadas
tasks-completed = { $count ->
    [one] Concluídas ({ $count })
    [many] Concluídas ({ $count })
   *[other] Concluídas ({ $count })
}
tasks-list-options = Opções da lista
tasks-rename-list = Renomear lista
tasks-delete-list = Excluir lista
tasks-mark-done = Marcar como concluída
tasks-mark-open = Marcar como não concluída
tasks-star = Marcar com estrela
tasks-unstar = Remover estrela
tasks-edit-title = Editar título
tasks-details = Detalhes
tasks-delete = Excluir
tasks-move-to = Mover para { $list }
tasks-from-mail = E-mail
tasks-open-mail = Abrir o e-mail
tasks-from-note = Nota
tasks-open-note = Abrir a nota
tasks-note-gone = Essa nota não está mais aqui.
tasks-no-subject = (sem assunto)

## The details dialog

tasks-notes-placeholder = Adicionar detalhes
tasks-date = Data
tasks-no-date = Sem data
tasks-time-placeholder = Adicionar horário
tasks-repeat = Repetir
tasks-repeat-never = Não se repete
tasks-repeat-daily = Diariamente
tasks-repeat-weekly = Semanalmente
tasks-repeat-monthly = Mensalmente
tasks-repeat-yearly = Anualmente
tasks-repeat-other = Personalizado
tasks-remind = Lembrar
tasks-remind-off = Não lembrar
tasks-remind-on-time = Na hora da tarefa
tasks-remind-morning = No dia, { $time }
tasks-remind-hour-before = Uma hora antes
tasks-remind-day-before = Um dia antes
tasks-cancel = Cancelar
tasks-save = Salvar
tasks-not-a-time = “{ $text }” não é um horário, por exemplo { $example }.

## Due days

tasks-due-today = Hoje
tasks-due-tomorrow = Amanhã
tasks-due-yesterday = Ontem
tasks-due-at = { $day }, { $time }

## Notes at the bottom

tasks-toast-done = Tarefa concluída
tasks-toast-next = Concluída. A próxima é em { $date }
tasks-toast-deleted = Tarefa excluída
tasks-toast-added = { $count ->
    [one] Adicionada às Tarefas
    [many] { $count } tarefas adicionadas
   *[other] { $count } tarefas adicionadas
}
tasks-mail-gone = Esse e-mail não está mais aqui.
tasks-toast-list-deleted = Lista excluída
tasks-toast-moved = Tarefa movida para { $list }
# A task dragged to another place in its own list.
tasks-toast-placed = Tarefa movida
tasks-toast-rescheduled = Tarefa reagendada
