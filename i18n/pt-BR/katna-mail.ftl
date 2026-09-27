# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = Idioma: { $language }
language-tooltip-system = Idioma: { $language }, conforme o sistema
language-search = Pesquisar idioma
language-system-default = Padrão do sistema
language-system-now = Atualmente: { $language }
language-no-match = Nenhum idioma corresponde a “{ $query }”
language-machine = Tradução automática. Ajude a melhorar
language-setting = Idioma
language-setting-detail = O idioma de menus, botões e mensagens, e o formato de datas e números. “Padrão do sistema” segue a área de trabalho.

## Dates and sizes

ago-just-now = agora mesmo
ago-minutes = { $count ->
    [one] há { $count } minuto
    [many] há { $count } de minutos
   *[other] há { $count } minutos
}
ago-hours = { $count ->
    [one] há { $count } hora
    [many] há { $count } de horas
   *[other] há { $count } horas
}
ago-days = { $count ->
    [one] há { $count } dia
    [many] há { $count } de dias
   *[other] há { $count } dias
}
size-bytes = { $count ->
    [one] { $count } byte
    [many] { $count } de bytes
   *[other] { $count } bytes
}
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = Ocultar pastas
folders-show = Mostrar pastas
compose = Escrever
search = Pesquisar
search-mail = Pesquisar e-mail
search-settings = Pesquisar configurações
search-clear = Limpar pesquisa
search-options-show = Mostrar opções de pesquisa
settings = Configurações
account-add = Adicionar uma conta

## App rail (and the bottom bar on a phone)

rail-mail = E-mail
rail-calendar = Agenda
rail-contacts = Contatos
rail-tasks = Tarefas
rail-notes = Notas
rail-feeds = Feeds

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = Em breve
app-calendar-promise = Suas agendas CalDAV, convites de reunião do seu e-mail e lembretes, ao lado da sua caixa de entrada.
app-tasks-promise = Listas de tarefas sincronizadas com CalDAV e tarefas criadas a partir de e-mails.
app-notes-promise = Notas rápidas e notas sobre um e-mail ou uma conversa para depois.
app-feeds-promise = Leia feeds RSS e Atom ao lado do seu e-mail.

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

## Navigation (the folders pane)

nav-labels = Marcadores
nav-folders = Pastas
nav-label-new = Criar novo marcador
nav-folder-new = Criar nova pasta
nav-account-unnamed = Conta { $number }
nav-tab-new = { $count ->
    [one] { $count } nova
    [many] { $count } novas
   *[other] { $count } novas
}

## Special folders (the user's own folders keep their names)

folder-inbox = Caixa de entrada
folder-starred = Com estrela
folder-drafts = Rascunhos
folder-sent = Enviados
folder-archive = Arquivo
folder-spam = Spam
folder-trash = Lixeira
folder-all-mail = Todos os e-mails
folder-scheduled = Programados

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = Novo marcador
label-folder-new-title = Nova pasta
label-prompt = Digite o nome do novo marcador:
label-folder-prompt = Digite o nome da nova pasta:
label-name-hint = Nome do marcador
label-folder-name-hint = Nome da pasta
label-nest = Aninhar marcador em:
label-folder-nest = Aninhar pasta em:
label-cancel = Cancelar
label-create = Criar
label-creating = Criando…
label-created = Marcador “{ $name }” criado.
label-folder-created = Pasta “{ $name }” criada.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Principal
tab-promotions = Promoções
tab-social = Social
tab-updates = Atualizações
tab-forums = Fóruns
tab-focused = Destaques
tab-other = Outros
tab-inbox = Caixa de entrada
tab-newsletters = Newsletters
tab-notifications = Notificações
tab-new = { $count ->
    [one] { $count } nova
    [many] { $count } novas
   *[other] { $count } novas
}
tab-provider-other = classificado pelo Katna

## Mail list: toolbar

list-select = Selecionar
list-refresh = Atualizar
list-more = Mais
list-mark-read = Marcar como lida
list-mark-unread = Marcar como não lida
list-move-to = Mover para
list-archive = Arquivar
list-spam = Denunciar spam
list-delete = Excluir
list-newer = Mais recentes
list-older = Mais antigas
list-range = { $first }–{ $last } de { $total }
list-range-about = { $first }–{ $last } de aproximadamente { $total }
list-results = Resultados para “{ $query }”
list-results-corrected = Mostrando resultados para “{ $query }”
list-search-instead = Pesquisar “{ $query }”
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Todas
list-pick-none = Nenhuma
list-pick-read = Lidas
list-pick-unread = Não lidas
list-pick-starred = Com estrela
list-pick-unstarred = Sem estrela

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] { $count } conversa está selecionada.
        [many] Todas as { $count } de conversas estão selecionadas.
       *[other] Todas as { $count } conversas estão selecionadas.
    }
   *[message] { $count ->
        [one] { $count } mensagem está selecionada.
        [many] Todas as { $count } de mensagens estão selecionadas.
       *[other] Todas as { $count } mensagens estão selecionadas.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] { $count } conversa em { $folder } está selecionada.
        [many] Todas as { $count } de conversas em { $folder } estão selecionadas.
       *[other] Todas as { $count } conversas em { $folder } estão selecionadas.
    }
   *[message] { $count ->
        [one] { $count } mensagem em { $folder } está selecionada.
        [many] Todas as { $count } de mensagens em { $folder } estão selecionadas.
       *[other] Todas as { $count } mensagens em { $folder } estão selecionadas.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] { $count } conversa nesta página está selecionada.
        [many] Todas as { $count } de conversas nesta página estão selecionadas.
       *[other] Todas as { $count } conversas nesta página estão selecionadas.
    }
   *[message] { $count ->
        [one] { $count } mensagem nesta página está selecionada.
        [many] Todas as { $count } de mensagens nesta página estão selecionadas.
       *[other] Todas as { $count } mensagens nesta página estão selecionadas.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Selecionar { $count } conversa
        [many] Selecionar todas as { $count } de conversas
       *[other] Selecionar todas as { $count } conversas
    }
   *[message] { $count ->
        [one] Selecionar { $count } mensagem
        [many] Selecionar todas as { $count } de mensagens
       *[other] Selecionar todas as { $count } mensagens
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Selecionar { $count } conversa em { $folder }
        [many] Selecionar todas as { $count } de conversas em { $folder }
       *[other] Selecionar todas as { $count } conversas em { $folder }
    }
   *[message] { $count ->
        [one] Selecionar { $count } mensagem em { $folder }
        [many] Selecionar todas as { $count } de mensagens em { $folder }
       *[other] Selecionar todas as { $count } mensagens em { $folder }
    }
}
list-clear-selection = Limpar seleção

## Mail list: empty states

list-empty-search = Nenhuma mensagem corresponde à sua pesquisa.
list-empty-tab = Nenhum e-mail em { $tab }.
list-empty-tab-unknown = Nenhum e-mail nesta guia.
list-empty-folder = Nenhuma mensagem em { $folder }.
list-empty-folder-unknown = Nenhuma mensagem nesta pasta.
list-first-sync = Buscando seus e-mails…
list-first-sync-detail = Eles aparecem aqui à medida que chegam.

## Mail list: lines

row-removed = Esta mensagem foi removida.
row-starred = Com estrela
row-not-starred = Sem estrela
row-important = Importante. Clique para marcar como não importante.
row-mark-important = Marcar como importante
row-pinned = Fixada no topo
row-pin = Fixar no topo
row-unpin = Desafixar

## Mail list: More menu and right-click menu

menu-reply = Responder
menu-reply-all = Responder a todos
menu-forward = Encaminhar
menu-archive = Arquivar
menu-delete = Excluir
menu-spam = Denunciar spam
menu-mark-read = Marcar como lida
menu-mark-unread = Marcar como não lida
menu-mark-all-read = Marcar todas como lidas
menu-star = Adicionar estrela
menu-unstar = Remover estrela
menu-important = Marcar como importante
menu-not-important = Marcar como não importante
menu-pin = Fixar no topo
menu-unpin = Desafixar
menu-print-all = Imprimir tudo
menu-new-window = Abrir em nova janela
menu-move-to = Mover para
menu-move-to-heading = Mover para:
menu-find-from = Encontrar e-mails de { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Conversa arquivada.
        [many] { $count } de conversas arquivadas.
       *[other] { $count } conversas arquivadas.
    }
   *[message] { $count ->
        [one] Mensagem arquivada.
        [many] { $count } de mensagens arquivadas.
       *[other] { $count } mensagens arquivadas.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Conversa movida para a Lixeira.
        [many] { $count } de conversas movidas para a Lixeira.
       *[other] { $count } conversas movidas para a Lixeira.
    }
   *[message] { $count ->
        [one] Mensagem movida para a Lixeira.
        [many] { $count } de mensagens movidas para a Lixeira.
       *[other] { $count } mensagens movidas para a Lixeira.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Conversa movida.
        [many] { $count } de conversas movidas.
       *[other] { $count } conversas movidas.
    }
   *[message] { $count ->
        [one] Mensagem movida.
        [many] { $count } de mensagens movidas.
       *[other] { $count } mensagens movidas.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Conversa marcada com estrela.
        [many] { $count } de conversas marcadas com estrela.
       *[other] { $count } conversas marcadas com estrela.
    }
   *[message] { $count ->
        [one] Mensagem marcada com estrela.
        [many] { $count } de mensagens marcadas com estrela.
       *[other] { $count } mensagens marcadas com estrela.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Estrela removida da conversa.
        [many] Estrela removida de { $count } de conversas.
       *[other] Estrela removida de { $count } conversas.
    }
   *[message] { $count ->
        [one] Estrela removida da mensagem.
        [many] Estrela removida de { $count } de mensagens.
       *[other] Estrela removida de { $count } mensagens.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Conversa marcada como importante.
        [many] { $count } de conversas marcadas como importantes.
       *[other] { $count } conversas marcadas como importantes.
    }
   *[message] { $count ->
        [one] Mensagem marcada como importante.
        [many] { $count } de mensagens marcadas como importantes.
       *[other] { $count } mensagens marcadas como importantes.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Conversa marcada como não importante.
        [many] { $count } de conversas marcadas como não importantes.
       *[other] { $count } conversas marcadas como não importantes.
    }
   *[message] { $count ->
        [one] Mensagem marcada como não importante.
        [many] { $count } de mensagens marcadas como não importantes.
       *[other] { $count } mensagens marcadas como não importantes.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Conversa fixada no topo.
        [many] { $count } de conversas fixadas no topo.
       *[other] { $count } conversas fixadas no topo.
    }
   *[message] { $count ->
        [one] Mensagem fixada no topo.
        [many] { $count } de mensagens fixadas no topo.
       *[other] { $count } mensagens fixadas no topo.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Conversa desafixada.
        [many] { $count } de conversas desafixadas.
       *[other] { $count } conversas desafixadas.
    }
   *[message] { $count ->
        [one] Mensagem desafixada.
        [many] { $count } de mensagens desafixadas.
       *[other] { $count } mensagens desafixadas.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Conversa denunciada como spam.
        [many] { $count } de conversas denunciadas como spam.
       *[other] { $count } conversas denunciadas como spam.
    }
   *[message] { $count ->
        [one] Mensagem denunciada como spam.
        [many] { $count } de mensagens denunciadas como spam.
       *[other] { $count } mensagens denunciadas como spam.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Conversa excluída permanentemente.
        [many] { $count } de conversas excluídas permanentemente.
       *[other] { $count } conversas excluídas permanentemente.
    }
   *[message] { $count ->
        [one] Mensagem excluída permanentemente.
        [many] { $count } de mensagens excluídas permanentemente.
       *[other] { $count } mensagens excluídas permanentemente.
    }
}
toast-undone = Ação desfeita.
toast-undo = Desfazer
toast-no-spam-folder = Esta conta não tem pasta de spam.

## Reading pane: toolbar

reader-close = Fechar
reader-back = Voltar
reader-mark-unread = Marcar como não lida
reader-move-to = Mover para
reader-more = Mais
reader-print-all = Imprimir tudo
reader-new-window = Em nova janela
reader-position = { $position } de { $total }
reader-newer = Mais recente
reader-older = Mais antiga

## Reading pane: the conversation

reader-removed = Esta conversa foi removida.
reader-no-subject = (sem assunto)
reader-collapse-all = Recolher tudo
reader-expand-all = Expandir tudo
reader-unknown-sender = (remetente desconhecido)
reader-date-ago = { $date } ({ $ago })
reader-me = mim
reader-to = para { $names }
reader-starred = Com estrela
reader-not-starred = Sem estrela
reader-too-long = A mensagem é longa demais para ser exibida por completo.
reader-encrypted-images = Imagens da web nunca são carregadas em e-mails criptografados.
reader-window-failed = Não foi possível abrir uma nova janela.

## Reading pane: message details (opened from "to me")

reader-details-from = de:
reader-details-to = para:
reader-details-cc = cc:
reader-details-date = data:
reader-details-subject = assunto:

## Reading pane: downloading a message

reader-downloading = Baixando esta mensagem do servidor…
reader-download-failed = Não foi possível baixar esta mensagem.
reader-try-again = Tentar novamente

## Reply row

reply-reply = Responder
reply-reply-all = Responder a todos
reply-forward = Encaminhar

## Encrypted and signed mail

security-decrypting = Descriptografando…
security-checking = Verificando a assinatura…
security-partly-encrypted = Apenas parte desta mensagem está criptografada. O restante foi adicionado fora da proteção e pode ter vindo de qualquer pessoa.
security-partly-signed = Apenas parte desta mensagem está assinada. O restante foi adicionado fora da proteção e pode ter vindo de qualquer pessoa.
security-encrypted = Mensagem criptografada
security-encrypted-smime = Mensagem criptografada (S/MIME)
security-no-key = Não é possível descriptografar esta mensagem: ela foi criptografada para uma chave que você não tem.
security-cancelled = A descriptografia foi cancelada.
security-damaged = Não é possível descriptografar esta mensagem: os dados criptografados estão danificados ou foram alterados.
security-decrypt-unavailable = Não é possível descriptografar esta mensagem: instale { $tool } para ler e-mails criptografados.
security-decrypt-failed = Não é possível descriptografar esta mensagem: { $reason }
security-unknown-signer = um signatário desconhecido
security-signed-verified = Assinada por { $signer } · verificada
security-signed-not-sender = Assinada por { $signer }, que não é o remetente
security-signed-untrusted = Assinada por { $signer }, com uma chave que você marcou como não confiável
security-signed-unverified = Assinada por { $signer } · a chave não foi verificada
security-bad-signature = Assinatura inválida: esta mensagem foi alterada depois de assinada, ou a assinatura foi falsificada.
security-signature-expired = Assinada por { $signer } · a assinatura expirou
security-key-expired = Assinada por { $signer } · a chave expirou desde então
security-key-revoked = Assinada por { $signer } com uma chave que foi revogada
security-missing-key = Assinada com uma chave que você não tem, por isso não pode ser verificada
security-missing-key-id = Assinada com uma chave que você não tem ({ $key }), por isso não pode ser verificada
security-signature-unavailable = Assinada; instale { $tool } para verificar a assinatura
security-signature-error = Não foi possível verificar a assinatura.

## Remote images and pictures

remote-hidden = As imagens desta mensagem estão ocultas.
remote-show = Exibir imagens
remote-always-show = Sempre exibir deste remetente
remote-picture-use = Usar
remote-picture-too-big = Escolha uma imagem de até 8 MB.
remote-picture-type = Escolha uma imagem PNG, JPEG, GIF, WebP ou SVG.
remote-picture-read-failed = Não é possível ler a imagem: { $error }
remote-picture-keep-failed = Não é possível guardar a imagem: { $error }
remote-picture-remove-failed = Não é possível remover a imagem: { $error }

## Attachments

attachment-count = { $count ->
    [one] Um anexo
    [many] { $count } de anexos
   *[other] { $count } anexos
}
attachment-save = Salvar
attachment-save-all = Salvar tudo
attachment-save-all-tooltip = Salvar todos os anexos em uma pasta
attachment-save-here = Salvar aqui
attachment-not-downloaded = Esta mensagem não foi baixada.
attachment-not-found = Este anexo não foi encontrado na mensagem.
attachment-read-failed = Não foi possível ler { $name }
attachment-numbered = anexo { $number }
attachment-saved-all = { $count ->
    [one] { $count } arquivo salvo em { $place }
    [many] { $count } de arquivos salvos em { $place }
   *[other] { $count } arquivos salvos em { $place }
}
attachment-saved-some = { $total ->
    [one] { $saved } de { $total } arquivo salvo em { $place }. Não foi possível salvar { $failed }
    [many] { $saved } de { $total } de arquivos salvos em { $place }. Não foi possível salvar { $failed }
   *[other] { $saved } de { $total } arquivos salvos em { $place }. Não foi possível salvar { $failed }
}
attachment-saved-to = Salvo em { $path }
attachment-save-failed = Não foi possível salvar { $name }: { $error }
attachment-open-failed = Não foi possível abrir { $name }: { $error }
attachment-risky = Este arquivo pode executar um programa, por isso o Katna não o abre. Salve-o em vez disso.
attachment-encrypted-open = Este arquivo veio criptografado. Salve-o para abri-lo em outro lugar.

## Printing

print-failed = Não foi possível imprimir: { $error }
print-no-font = nenhuma fonte foi encontrada
print-opened-as-pdf = Aberto como PDF para imprimir a partir dele.
print-not-downloaded = (Ainda não baixada.)
print-encrypted = (Criptografada. Abra no Katna Mail para imprimir o texto.)
print-to = Para: { $addresses }
print-cc = Cc: { $addresses }
