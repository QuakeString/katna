# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = Leitura
chat-view = Conversas como chats
chat-view-detail = E-mails entre pessoas são lidos como um chat em grupo: um balão para cada e-mail com só o que foi escrito, os seus à direita. Newsletters mantêm a visualização de sempre.
chat-view-switch = Mostrar conversas como chats
chat-view-switch-detail = O e-mail citado e as assinaturas ficam atrás de ··· em cada balão

chat-switch-chat = Chat
chat-switch-mail = E-mail
chat-people = { $names } e você · { $count ->
    [one] { $count } e-mail
    [many] { $count } de e-mails
   *[other] { $count } e-mails
}
chat-people-heading = { $count ->
    [one] Neste chat · { $count } pessoa
    [many] Neste chat · { $count } de pessoas
   *[other] Neste chat · { $count } pessoas
}
chat-member-mails = { $count ->
    [0] Nenhum e-mail
    [one] { $count } e-mail
    [many] { $count } de e-mails
   *[other] { $count } e-mails
}
chat-today = Hoje
chat-yesterday = Ontem
chat-added = { $who } adicionou { $names }
chat-renamed = { $who } mudou o assunto para “{ $subject }”
chat-you = Você
chat-not-downloaded = Ainda não baixado
chat-forwarded = Encaminhado
chat-show-quoted = Mostrar o e-mail citado e a assinatura
chat-hide-quoted = Ocultar o e-mail citado e a assinatura
chat-hide-dots = Ocultar ···
chat-show-card = Mostrar o cartão
chat-reply-all = Responder a todos
chat-more = Mais
chat-reply-only = Responder só a { $name }
chat-forward = Encaminhar
chat-copy-text = Copiar texto
chat-show-as-mail = Mostrar como e-mail
chat-pin = Fixar no topo
chat-pin-file = Fixar arquivo no topo
chat-unpin = Desafixar
chat-unpin-file = Desafixar arquivo
chat-pinned-of = Fixado { $at } de { $count }
chat-pins-all = Todos os fixados
chat-pins-heading = Fixados · { $count } de { $most }
chat-pins-drag = Arraste para reordenar
chat-pin-from-mail = E-mail de { $name } · { $when }
chat-pin-from-file = Arquivo de { $name } · { $when }
chat-pin-from-text = Texto de { $name } · { $when }
chat-pins-full = Este chat já tem 5 itens fixados
chat-pins-replace-title = Substituir um item fixado
chat-pins-replace-hint = Um chat tem até 5 itens fixados. Escolha o que será retirado.
chat-pins-replace = Substituir
chat-pins-cancel = Cancelar
chat-undo = Desfazer

chat-reply-to = Responder a { $names }
chat-send = Enviar (Ctrl+Enter)
chat-attach = Anexar
chat-attach-photo = Foto
chat-attach-file = Arquivo
chat-attach-library = De Arquivos
chat-attach-template = Modelo
chat-attach-signature = Assinatura
chat-replying-to = Respondendo a { $name }
chat-reply-newest = Responder ao e-mail mais recente

## The attach picker (paperclip > From Files)

picker-title = Anexar de Arquivos
picker-search = Pesquisar nomes, pessoas, assuntos
picker-search-drive = Pesquisar neste drive
picker-mail-files = Arquivos dos e-mails
picker-this-chat = Esta conversa
picker-this-computer = Este computador…
picker-in-chat = NESTA CONVERSA
picker-recent = RECENTES
picker-preview = Visualizar
picker-cancel = Cancelar
picker-attach = Anexar
picker-attach-count = Anexar { $count }
picker-selected = { $count } selecionados
picker-of-limit = de { $limit }
picker-in-mail = { $size } no e-mail
picker-drive-links = { $count ->
    [one] 1 como link do Google Drive
    [many] { $count } de arquivos como links do Google Drive
   *[other] { $count } como links do Google Drive
}
picker-onedrive-links = { $count ->
    [one] 1 como link do OneDrive
    [many] { $count } de arquivos como links do OneDrive
   *[other] { $count } como links do OneDrive
}
picker-over = { $size }, mais que os { $limit } que um e-mail comporta
picker-getting = { $count ->
    [one] Buscando o arquivo no drive…
    [many] Buscando { $count } de arquivos no drive…
   *[other] Buscando { $count } arquivos no drive…
}
picker-some-failed = { $count ->
    [one] Não foi possível ler um arquivo
    [many] Não foi possível ler { $count } de arquivos
   *[other] Não foi possível ler { $count } arquivos
}
