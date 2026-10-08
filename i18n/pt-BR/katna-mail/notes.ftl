# Katna Mail, Portuguese (Brazil) (Português (Brasil)): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = Notas
notes-view-reminders = Lembretes
notes-view-archive = Arquivo
notes-view-trash = Lixeira
notes-edit-labels = Editar marcadores
notes-search = Pesquisar notas
notes-loading = Abrindo suas notas…

## Board

notes-take-a-note = Criar uma nota…
notes-new-list = Nova lista
notes-new-note = Nova nota
notes-pinned = Fixadas
notes-others = Outras
notes-empty = As notas que você adicionar aparecerão aqui
notes-archive-empty = Suas notas arquivadas aparecerão aqui
notes-trash-empty = Nenhuma nota na lixeira
notes-none-found = Nenhuma nota correspondente
notes-label-empty = Ainda não há notas com este marcador
notes-reminders-empty = As notas com lembretes futuros aparecem aqui
notes-trash-note = As notas na lixeira são excluídas após 7 dias.
notes-empty-trash = Esvaziar lixeira
notes-ticked = { $count ->
    [one] + { $count } item marcado
    [many] + { $count } itens marcados
   *[other] + { $count } itens marcados
}
notes-select = Selecionar nota
notes-selected = { $count ->
    [one] { $count } selecionada
    [many] { $count } de selecionadas
   *[other] { $count } selecionadas
}
notes-select-clear = Limpar seleção

## A note's buttons

notes-pin = Fixar nota
notes-unpin = Desafixar nota
notes-archive = Arquivar
notes-unarchive = Desarquivar
notes-delete = Excluir nota
notes-restore = Restaurar
notes-delete-forever = Excluir permanentemente
notes-color = Cor do plano de fundo
notes-checkboxes = Mostrar ou ocultar caixas de seleção
notes-labels = Marcadores
notes-close = Fechar
notes-more = Mais
notes-make-copy = Fazer uma cópia
notes-remind = Lembrar-me
notes-add-picture = Adicionar imagem
notes-history = Histórico de versões
notes-ai = Ajude-me a escrever
notes-send-as-mail = Enviar como e-mail
notes-save-markdown = Salvar como Markdown
notes-save-pdf = Salvar como PDF

## The open note

notes-title = Título
notes-edited = Editada: { $date }
notes-on-this-computer = Neste computador
notes-where = Onde esta nota é mantida
notes-untitled = Nota sem título

## Pictures

notes-picture-choose = Adicionar imagens
notes-picture-remove = Remover imagem
notes-picture-too-big = Imagens de até { $size } podem entrar em uma nota
notes-picture-kind = Esse arquivo não é uma imagem que o Katna consiga mostrar
notes-picture-unreadable = Não foi possível ler { $name }: { $error }

## Reminders

notes-remind-me = Lembrar-me
notes-remind-off = Remover lembrete
notes-remind-in-the-past = Escolha um horário que ainda não passou
notes-remind-today = Hoje, { $time }
notes-remind-tomorrow = Amanhã, { $time }
notes-remind-weekday = { $day }, { $time }
notes-reminder-set = Lembrete definido para { $when }
notes-reminder-off = Lembrete removido

## Links between notes

notes-link-note = Vincular uma nota
notes-link-new = Nova nota “{ $title }”
notes-linked-from = Vinculada a partir de
notes-link-gone = Essa nota não está mais aqui

## Version history

notes-versions = Versões
notes-version-now = Agora
notes-version-here = Você, neste computador
notes-version-yesterday = Ontem, { $time }
notes-version-changes = { $count ->
    [one] { $count } alteração
    [many] { $count } de alterações
   *[other] { $count } alterações
}
notes-version-from = De { $device }
notes-version-elsewhere = De outro dispositivo
notes-version-created = Criada
notes-version-restore = Restaurar esta versão
notes-version-restored = Versão restaurada
notes-history-none = Ainda não há versões anteriores

## AI help

notes-ai-tidy = Organizar o texto
notes-ai-checklist = Transformar em lista de verificação
notes-ai-summarise = Resumir
notes-ai-empty = Escreva algo primeiro
notes-ai-tidied = Texto organizado. Ctrl+Z desfaz.
notes-ai-listed = Transformada em lista de verificação. Ctrl+Z desfaz.
notes-ai-summarised = Resumo adicionado no início

## Labels

notes-label-note = Marcar nota
notes-label-name = Digite o nome do marcador
notes-label-create = Criar “{ $name }”
notes-label-remove = Remover marcador
notes-label-delete = Excluir marcador
notes-labels-none = Ainda não há marcadores. Adicione um pelo botão de marcador de uma nota.
notes-labels-done = Concluído
notes-label-renamed = Marcador renomeado para “{ $name }”
notes-label-deleted = Marcador “{ $name }” excluído

## A note about a mail

notes-mail = E-mail
notes-open-mail = Abrir o e-mail
notes-open-note = Abrir a nota

## Meeting notes

notes-meeting-take = Fazer anotações da reunião
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = Participantes: { $names }
notes-meeting-notes = Anotações
notes-meeting-actions = Itens de ação
notes-event = Evento
notes-open-event = Abrir o evento

## Formatting

notes-format = Formatação
notes-format-heading-1 = Título 1
notes-format-heading-2 = Título 2
notes-format-normal = Texto normal
notes-format-bold = Negrito
notes-format-italic = Itálico
notes-format-underline = Sublinhado
notes-format-quote = Citação
notes-format-code = Código
notes-format-divider = Divisor
notes-format-clear = Limpar formatação

## Tasks

notes-make-task = Transformar em tarefa

## Colors (tooltips)

notes-color-none = Sem cor
notes-color-coral = Coral
notes-color-peach = Pêssego
notes-color-sand = Areia
notes-color-mint = Menta
notes-color-sage = Sálvia
notes-color-fog = Névoa
notes-color-storm = Tempestade
notes-color-dusk = Crepúsculo
notes-color-blossom = Flor
notes-color-clay = Argila
notes-color-chalk = Giz

## Messages at the foot of the window

notes-archived = Nota arquivada
notes-unarchived = Nota desarquivada
notes-trashed = Nota movida para a lixeira
notes-restored = Nota restaurada
notes-saved = Nota salva
notes-pinned-count = { $count ->
    [one] Nota fixada
    [many] { $count } de notas fixadas
   *[other] { $count } notas fixadas
}
notes-unpinned-count = { $count ->
    [one] Nota desafixada
    [many] { $count } de notas desafixadas
   *[other] { $count } notas desafixadas
}
notes-colored-count = { $count ->
    [one] Cor alterada
    [many] Cor alterada em { $count } de notas
   *[other] Cor alterada em { $count } notas
}
notes-archived-count = { $count ->
    [one] Nota arquivada
    [many] { $count } de notas arquivadas
   *[other] { $count } notas arquivadas
}
notes-unarchived-count = { $count ->
    [one] Nota desarquivada
    [many] { $count } de notas desarquivadas
   *[other] { $count } notas desarquivadas
}
notes-trashed-count = { $count ->
    [one] Nota movida para a lixeira
    [many] { $count } de notas movidas para a lixeira
   *[other] { $count } notas movidas para a lixeira
}
notes-restored-count = { $count ->
    [one] Nota restaurada
    [many] { $count } de notas restauradas
   *[other] { $count } notas restauradas
}
notes-copied-count = { $count ->
    [one] Cópia criada
    [many] { $count } de cópias criadas
   *[other] { $count } cópias criadas
}
notes-empty-discarded = Nota vazia descartada
notes-mail-gone = Esse e-mail não está mais aqui
notes-deleted-forever = { $count ->
    [one] Nota excluída permanentemente
    [many] { $count } notas excluídas permanentemente
   *[other] { $count } notas excluídas permanentemente
}
