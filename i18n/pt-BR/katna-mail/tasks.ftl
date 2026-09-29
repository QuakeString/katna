# Katna Mail, Portuguese (Brazil) (Português (Brasil)): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = Criar
tasks-all = Todas as tarefas
tasks-today = Hoje
tasks-starred = Com estrela
tasks-new-list = Criar nova lista
tasks-on-this-computer = Neste computador
tasks-my-tasks = Minhas tarefas
tasks-list-name-placeholder = Nome da lista

## Lists and tasks

tasks-loading = Lendo suas tarefas…
tasks-no-lists = Suas listas de tarefas aparecem aqui.
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
tasks-toast-deleted = Tarefa excluída
tasks-toast-added = { $count ->
    [one] Adicionada às Tarefas
    [many] { $count } tarefas adicionadas
   *[other] { $count } tarefas adicionadas
}
tasks-mail-gone = Esse e-mail não está mais aqui.
tasks-toast-list-deleted = Lista excluída
tasks-toast-moved = Tarefa movida para { $list }
tasks-toast-rescheduled = Tarefa reagendada
