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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Abra esta mensagem para ler os anexos.
text-copy = Copiar
text-select-all = Selecionar tudo

## Settings page: its tabs

settings-tab-general = Geral
settings-tab-inbox = Caixa de entrada
settings-tab-accounts = Contas
settings-tab-subscriptions = Inscrições
settings-tab-appearance = Aparência
settings-tab-shortcuts = Atalhos
settings-tab-default-apps = Apps padrão
settings-tab-folders-rules = Pastas e regras
settings-tab-compose = Escrever
settings-tab-mcp-server = Servidor MCP
settings-tab-feedback = Feedback
settings-tab-experimental = Experimental

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Veja as newsletters e listas de e-mails que você recebe e cancele a inscrição com um clique.
settings-tab-folders-rules-coming = Crie, renomeie, mova e oculte pastas e marcadores, e escolha quais são sincronizados. As regras classificam, marcam, encaminham ou excluem novos e-mails automaticamente, por remetente, assunto ou palavras.
settings-tab-mcp-server-coming = Permita que assistentes de IA neste computador pesquisem, leiam e criem rascunhos dos seus e-mails, com a sua autorização.

## Settings > General

settings-general-conversations = Visualização de conversas
settings-general-conversations-group = Agrupar respostas ao mesmo e-mail
settings-general-conversations-group-detail = Uma linha por conversa na lista
settings-general-reading = Leitura
settings-general-newest-first = Mensagem mais recente primeiro
settings-general-newest-first-detail = A conversa começa pela resposta mais recente
settings-general-full-headers = Mostrar cabeçalhos completos
settings-general-full-headers-detail = De, para, cc, data e assunto abertos em cada mensagem
settings-general-full-names = Nomes completos dos destinatários
settings-general-full-names-detail = “para mim, Ada Lovelace” em vez de “para mim, Ada”
settings-general-mark-read = Marcar como lida
settings-general-mark-read-now = Assim que é aberta
settings-general-mark-read-1s = Depois de aberta por 1 segundo
settings-general-mark-read-3s = Depois de aberta por 3 segundos
settings-general-mark-read-never = Só quando eu marcar como lida
settings-general-reply-button = Botão de resposta
settings-general-reply-all = Responder a todos
settings-general-reply-all-detail = O botão de resposta ao lado de cada mensagem responde a todos, não só ao remetente
settings-general-remote-images = Imagens da web
settings-general-remote-images-detail = Carregar as imagens de uma mensagem informa ao remetente que você a abriu, quando e mais ou menos onde. Desativado, cada mensagem pergunta antes, e você sempre pode exibir as imagens de um remetente.
settings-general-remote-images-always = Sempre exibir imagens
settings-general-remote-images-always-detail = Em todas as mensagens, não só de remetentes confiáveis
settings-general-sending = Envio
settings-general-sending-detail = Quanto tempo uma mensagem enviada espera, para que o envio possa ser cancelado.
settings-general-offline = E-mail off-line
settings-general-offline-detail = Os e-mails recentes são baixados por completo, para ler sem conexão. Os mais antigos são baixados quando você os abre.
settings-general-offline-days = { $count ->
    [one] { $count } dia
    [many] { $count } de dias
   *[other] { $count } dias
}
settings-general-offline-years = { $count ->
    [one] { $count } ano
    [many] { $count } de anos
   *[other] { $count } anos
}
settings-general-offline-all = Todos os e-mails
settings-general-offline-note = Escolher menos dias mantém os e-mails já baixados. Nada muda no servidor.
settings-general-notifications = Notificações
settings-general-notifications-detail = De novos e-mails na Caixa de entrada, mesmo com o Katna Mail fechado.
settings-general-new-mail = Notificar sobre novos e-mails
settings-general-new-mail-detail = Com Responder a todos, Marcar como lida e Arquivar
settings-general-new-mail-sound = Tocar um som
settings-general-new-mail-sound-detail = O som de novo e-mail da área de trabalho
settings-general-desktop = Área de trabalho
settings-general-open-at-login = Abrir o Katna Mail ao fazer login
settings-general-open-at-login-detail = O e-mail é sincronizado ao fazer login de qualquer forma, enquanto o serviço estiver em execução
settings-general-tray = Mostrar o Katna na bandeja do sistema
settings-general-tray-detail = Com a contagem de não lidas e um menu
settings-general-unread-badge = Contagem de não lidas no ícone da barra de tarefas
settings-general-unread-badge-detail = Quantas mensagens da Caixa de entrada não foram lidas

## Settings > Inbox

settings-inbox-tabs = Guias da Caixa de entrada
settings-inbox-tabs-detail = Organize a Caixa de entrada em guias, como faz o site do seu provedor de e-mail.
settings-inbox-tabs-show = Mostrar guias da Caixa de entrada
settings-inbox-tabs-show-detail = Desativado, mostra uma lista para cada conta
settings-inbox-no-accounts = Adicione uma conta para escolher as guias dela.
settings-inbox-tabs-automatic = Automático: { $tabs } ({ $provider })
settings-inbox-tabs-off = Sem guias
settings-inbox-tabs-gmail = Principal, Promoções, Social, Atualizações, Fóruns
settings-inbox-tabs-focused = Destaques e Outros
settings-inbox-tabs-zoho = Caixa de entrada, Newsletters e Notificações
settings-inbox-tabs-shown = Guias exibidas. Os e-mails de uma guia desativada ficam em { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Painel de leitura
settings-appearance-reading-pane-detail = Onde uma conversa aberta é exibida.
settings-appearance-pane-right = À direita da lista
settings-appearance-pane-none = Sem divisão
settings-appearance-density = Densidade
settings-appearance-density-default = Padrão
settings-appearance-density-compact = Compacta
settings-appearance-scaling = Escala
settings-appearance-scaling-detail = Deixa tudo no Katna Mail maior ou menor, além da escala da própria área de trabalho: texto, ícones, espaçamento e divisórias. Os e-mails que você envia mantêm o próprio tamanho de fonte. Tamanhos muito pequenos podem dificultar o clique nos ícones.
settings-appearance-theme = Tema
settings-appearance-theme-system = Igual à área de trabalho
settings-appearance-theme-light = Claro
settings-appearance-theme-dark = Escuro
settings-appearance-desktop-colors = Cores da área de trabalho
settings-appearance-desktop-colors-use = Usar as cores da área de trabalho
settings-appearance-desktop-colors-use-detail = O esquema de cores e a cor de destaque da área de trabalho
settings-appearance-app-names = Nomes dos apps
settings-appearance-app-names-show = Mostrar nomes dos apps
settings-appearance-app-names-show-detail = Nomes sob os ícones dos apps, à esquerda
settings-appearance-sender-pictures = Imagens dos remetentes
settings-appearance-sender-pictures-show = Mostrar logotipos de empresas
settings-appearance-sender-pictures-show-detail = Buscados pelo domínio do remetente, nunca por mensagem, e guardados por uma semana
settings-appearance-important = Marcadores de importância
settings-appearance-important-show = Mostrar marcadores de importância
settings-appearance-important-show-detail = Ao lado de cada mensagem na lista
settings-appearance-message-width = Largura das mensagens
settings-appearance-message-width-limit = Limitar a largura das mensagens
settings-appearance-message-width-limit-detail = Linhas longas ficam mais fáceis de ler em uma janela larga
settings-appearance-mail-colors = Cores dos e-mails
settings-appearance-mail-colors-detail = A maioria dos e-mails é feita para uma página branca. Com um tema escuro, as cores são trocadas por cores escuras fáceis de ler; desativado, o e-mail mantém as cores do remetente em uma página clara.
settings-appearance-dark-mail = Cores escuras também nos e-mails
settings-appearance-dark-mail-detail = Só enquanto o tema estiver escuro
settings-appearance-attachment-previews = Visualização de anexos
settings-appearance-attachment-previews-show = Mostrar visualização dos anexos
settings-appearance-attachment-previews-show-detail = Uma pequena imagem do conteúdo de cada arquivo no cartão dele

## Settings > Default apps

settings-default-apps-intro = Onde os anexos abrem quando você clica neles. O visualizador também sempre pode abrir um arquivo em outro app. Os apps padrão da área de trabalho são definidos nas configurações dela.
settings-default-apps-pdf = Arquivos PDF
settings-default-apps-pdf-detail = Páginas, com zoom.
settings-default-apps-pictures = Imagens
settings-default-apps-pictures-detail = Fotos (na posição correta), PNG, GIF, WebP, BMP, TIFF e SVG.
settings-default-apps-text = Arquivos de texto
settings-default-apps-text-detail = Texto simples, logs, código e outros textos.
settings-default-apps-sheets = Planilhas
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) e CSV.
settings-default-apps-documents = Documentos
settings-default-apps-documents-detail = Word (docx) e texto OpenDocument (odt).
settings-default-apps-katna = Visualizador do Katna Mail
settings-default-apps-system = O app padrão da área de trabalho
settings-default-apps-ask = Perguntar qual app todas as vezes
settings-default-apps-after-saving = Depois de salvar
settings-default-apps-show-folder = Mostrar arquivos salvos na pasta deles
settings-default-apps-show-folder-detail = Abre o gerenciador de arquivos com os anexos salvos selecionados

## Settings > Compose

settings-compose-send-from = Enviar novas mensagens de
settings-compose-send-from-detail = Respostas e encaminhamentos sempre saem da conta em que você está.
settings-compose-send-from-current = A conta em que você está
settings-compose-send-on-replies = Enviar em respostas
settings-compose-send-on-replies-detail = O que Enviar faz em uma resposta ou encaminhamento. O menu ao lado de Enviar oferece a outra opção.
settings-compose-send-plain = Enviar
settings-compose-send-archive = Enviar e arquivar
settings-compose-signatures = Assinaturas
settings-compose-signatures-detail = Adicionada abaixo da sua mensagem, depois de uma linha “--”. Escolha outra na janela de escrita.
settings-compose-untitled = Sem título
settings-compose-signature-name = Nome, como Trabalho
settings-compose-signature-first = Minha assinatura
settings-compose-signature-numbered = Assinatura { $number }
settings-compose-signature-delete = Excluir
settings-compose-signature-deleted = Assinatura excluída
settings-compose-signature-new = Criar nova
settings-compose-no-signatures = Nenhuma assinatura ainda.
settings-compose-no-signature = Sem assinatura
settings-compose-for-new-mail = Para novos e-mails
settings-compose-for-replies = Para respostas e encaminhamentos
settings-compose-for-replies-detail = Em uma conversa em que você assinou uma mensagem, a resposta começa com essa assinatura.
settings-compose-format = Formato
settings-compose-plain-text = Escrever em texto simples
settings-compose-plain-text-detail = Novos e-mails começam sem formatação; a janela de escrita pode mudar isso
settings-compose-spelling = Ortografia
settings-compose-spell-check = Verificar a ortografia enquanto escrevo
settings-compose-spell-check-detail = Palavras com erro são sublinhadas, com sugestões no clique com o botão direito
settings-compose-spell-desktop = Idioma da área de trabalho ({ $language })
settings-compose-templates = Modelos
settings-compose-templates-detail = Salve e-mails que você escreve com frequência e comece um novo e-mail ou uma resposta a partir deles.

## Settings > Shortcuts

settings-shortcuts-set = Conjunto de atalhos
settings-shortcuts-set-detail = Comece com as teclas de um app de e-mail que você conhece. Cmd é Ctrl aqui. Suas próprias alterações ficam por cima do conjunto, e Restaurar padrões volta às teclas do conjunto.
settings-shortcuts-single = Atalhos de uma tecla
settings-shortcuts-single-detail = Teclas sem Ctrl ou Alt, como no webmail: e arquiva, j e k navegam, / pesquisa. Funcionam na lista e na conversa aberta, nunca enquanto você digita.
settings-shortcuts-single-use = Usar atalhos de uma tecla
settings-shortcuts-single-use-detail = Os atalhos com Ctrl sempre funcionam
settings-shortcuts-how = Clique em uma tecla para alterá-la, ou em + para adicionar uma, e pressione as novas teclas. Esc cancela.
settings-shortcuts-restore = Restaurar padrões
settings-shortcuts-no-key = Sem tecla
settings-shortcuts-press = Pressione as teclas…
settings-shortcuts-then = { $keys } e depois…
settings-shortcuts-moved = { $keys } agora faz “{ $action }” em vez de “{ $previous }”.
settings-shortcuts-single-off = Os atalhos de uma tecla estão desativados, então esta tecla funciona quando forem ativados.
settings-shortcuts-restored = Todos os atalhos voltaram às teclas do conjunto.

## Settings search: the line under a result

settings-general-language-summary = Idioma do app, das datas e dos números
settings-general-reading-summary = Mensagem mais recente primeiro, cabeçalhos completos, nomes completos dos destinatários
settings-general-mark-read-summary = Quando uma conversa aberta é marcada como lida: na hora, depois de 1 ou 3 segundos, ou manualmente
settings-general-reply-button-summary = O botão de resposta ao lado de cada mensagem responde a todos
settings-general-remote-images-summary = Sempre exibir as imagens de todas as mensagens
settings-general-sending-summary = Cancelar envio: quanto tempo uma mensagem enviada espera, para que o envio possa ser cancelado
settings-general-offline-summary = Quantos dias de e-mails recentes são baixados por completo, para ler sem conexão
settings-general-notifications-summary = Notificações de novos e-mails e o som delas
settings-general-desktop-summary = Abrir o Katna Mail ao fazer login, o ícone da bandeja do sistema e a contagem de não lidas no ícone da barra de tarefas
settings-accounts-accounts-summary = Adicionar ou remover uma conta, ou alterar a imagem dela
settings-appearance-density-summary = Linhas padrão ou compactas na lista
settings-appearance-scaling-summary = Deixar tudo maior ou menor: texto, ícones, espaçamento e divisórias
settings-appearance-theme-summary = Igual à área de trabalho, claro ou escuro
settings-appearance-sender-pictures-summary = Logotipos de empresas, buscados pelo domínio do remetente
settings-appearance-important-summary = O marcador de importância ao lado de cada mensagem na lista
settings-appearance-mail-colors-summary = Cores escuras para e-mails em HTML em um tema escuro, ou as cores do remetente
settings-appearance-attachment-previews-summary = Uma pequena imagem do conteúdo de cada anexo
settings-shortcuts-set-summary = Comece com as teclas do Gmail, Inbox by Gmail, Apple Mail, Outlook ou Thunderbird
settings-shortcuts-single-summary = Teclas sem Ctrl ou Alt, como no webmail
settings-default-apps-pdf-summary = Onde os anexos PDF abrem
settings-default-apps-pictures-summary = Onde fotos e imagens abrem
settings-default-apps-text-summary = Onde texto simples, logs e código abrem
settings-default-apps-sheets-summary = Onde arquivos Excel, OpenDocument e CSV abrem
settings-default-apps-documents-summary = Onde documentos Word e texto OpenDocument abrem
settings-default-apps-after-saving-summary = Mostrar anexos salvos na pasta deles
settings-compose-send-from-summary = A conta de onde saem os novos e-mails: aquela em que você está, ou sempre a mesma
settings-compose-send-on-replies-summary = Enviar, ou Enviar e arquivar a conversa, em respostas e encaminhamentos
settings-compose-signatures-summary = Adicionada abaixo da sua mensagem, depois de uma linha “--”
settings-compose-for-new-mail-summary = A assinatura com que os novos e-mails começam
settings-compose-for-replies-summary = A assinatura com que respostas e encaminhamentos começam
settings-compose-format-summary = Escrever novos e-mails em texto simples
settings-compose-spelling-summary = Verificar a ortografia ao escrever, e o idioma do dicionário
settings-compose-templates-summary = Em breve: salve e-mails que você escreve com frequência e comece um novo e-mail ou uma resposta a partir deles
settings-feedback-crash-reports-summary = Salvar relatórios de falhas neste computador quando o Katna Mail ou o serviço em segundo plano falhar
settings-feedback-saved-summary = Ver, copiar ou excluir os relatórios de falhas salvos neste computador
settings-feedback-help-improve-summary = Enviar relatórios de falhas para ajudar a corrigir o que deu errado; desativado a menos que você ative
settings-experimental-blur-summary = A área de trabalho aparece desfocada através da barra superior, e os menus ficam foscos
settings-search-shortcut = Atalho do teclado
settings-search-tab = Guia das configurações
settings-search-none = Nenhuma configuração corresponde a “{ $query }”.
settings-search-results = Configurações que correspondem a “{ $query }”
## Quick settings (the panel that slides in from the right)

quick-title = Configurações rápidas
quick-see-all = Ver todas as configurações
quick-reading-pane = Painel de leitura
quick-pane-right = À direita da lista
quick-pane-none = Sem divisão
quick-density = Densidade
quick-density-default = Padrão
quick-density-compact = Compacta
quick-theme = Tema
quick-theme-system = Igual à área de trabalho
quick-theme-light = Claro
quick-theme-dark = Escuro
quick-desktop-colors = Cores da área de trabalho
quick-desktop-colors-detail = O esquema de cores e a cor de destaque da área de trabalho
quick-app-names = Nomes dos apps
quick-app-names-detail = Nomes sob os ícones dos apps, à esquerda
quick-inbox-tabs = Guias da Caixa de entrada
quick-inbox-tabs-detail = As guias do provedor de e-mail de cada conta
quick-choose-tabs = Escolher guias
quick-choose-tabs-detail = Por conta, nas Configurações
quick-sending = Envio
quick-undo-send = Cancelar envio
quick-undo-send-off = Desativado
quick-undo-send-seconds = { $seconds } s
quick-signatures = Assinaturas
quick-signatures-none = Nenhuma ainda
quick-signatures-one = { $name }, usada por padrão
quick-signatures-many = { $count ->
    [one] { $count } assinatura; { $name } por padrão
    [many] { $count } de assinaturas; { $name } por padrão
   *[other] { $count } assinaturas; { $name } por padrão
}
quick-signatures-no-default = { $count ->
    [one] { $count }, nenhuma por padrão
    [many] { $count }, nenhuma por padrão
   *[other] { $count }, nenhuma por padrão
}
quick-signature-untitled = Sem título
quick-threading = Agrupamento de e-mails
quick-conversation-view = Visualização de conversas
quick-conversation-view-detail = Agrupar respostas ao mesmo e-mail
quick-help = Ajuda
quick-tour = Fazer o tour
quick-whats-new = Novidades
quick-about = Sobre o Katna

## Settings: opening at login

settings-open-at-login-failed = Não foi possível alterar a abertura ao fazer login: { $error }

## Settings > Appearance > Scaling

scale-letter = A
scale-percent = { $percent }%
scale-reset = Voltar a { $percent }%

## Settings > Experimental > Look & Feel

look-intro = Recursos ainda em teste. Podem mudar ou deixar de existir.
look-heading = Aparência e comportamento
look-window-frame = Moldura da janela
look-window-frame-detail = Quem desenha a barra de título, os botões da janela, os cantos e a sombra.
look-frame-native-kde = Nativa: a moldura do KDE, no seu tema do Plasma
look-frame-native = Nativa: a moldura da área de trabalho
look-frame-katna = Katna: a barra superior vira a barra de título
look-frame-katna-note-named = O Katna desenha cantos arredondados e sombra própria. A moldura deixa de seguir o tema do { $desktop }; as regras de janela continuam valendo.
look-frame-katna-note = O Katna desenha cantos arredondados e sombra própria. A moldura deixa de seguir o tema da área de trabalho; as regras de janela continuam valendo.
look-frame-client-side = Sua área de trabalho deixa a moldura a cargo de cada app, então o Katna já desenha a sua própria.
look-blurred-background = Fundo desfocado
look-blurred-background-detail = A área de trabalho aparece desfocada através da barra superior e das pastas, e os menus e popovers são de vidro fosco.
look-blur = Desfocar o que está atrás da janela
look-blur-detail = Os e-mails ficam em cartões opacos, para que o texto mantenha o contraste
look-blur-off-kde = O efeito de desfoque do KDE está desativado. Ative Desfoque em Configurações do sistema, Gerenciamento de janelas, Efeitos da área de trabalho e abra o Katna Mail novamente.
look-blur-none-gnome = O GNOME não desfoca o que está atrás das janelas.
look-blur-none-x11 = Seu gerenciador de janelas não desfoca o que está atrás das janelas.
look-blur-none-wayland = Seu compositor não desfoca o que está atrás das janelas.

## Settings > User feedback (crash reports)

feedback-intro-sending = Novos relatórios de falhas são enviados para ajudar a corrigir o que deu errado. Nada mais sai deste computador.
feedback-intro-local = O Katna não envia nada a lugar nenhum. Os relatórios de falhas ficam neste computador, para você consultar ou anexar a um relato de bug.
feedback-crash-reports = Relatórios de falhas
feedback-crash-reports-detail = Gerados quando o Katna Mail ou o serviço em segundo plano falha.
feedback-save = Salvar relatórios de falhas neste computador
feedback-save-detail = Sua pasta pessoal, os nomes de usuário e do computador e os endereços de e-mail são omitidos
feedback-saved = Relatórios de falhas salvos
feedback-saved-detail = { $count ->
    [one] O mais recente é mantido.
    [many] Os { $count } de mais recentes são mantidos.
   *[other] Os { $count } mais recentes são mantidos.
}
feedback-help-improve = Ajude a melhorar o Katna
feedback-help-improve-detail = Desativado a menos que você ative, e você pode desativar aqui a qualquer momento.
feedback-send = Enviar relatórios de falhas
feedback-send-detail = O relatório salvo, exatamente como você pode vê-lo aqui, vai para o rastreador de falhas do Katna (Sentry, na UE). Nenhum endereço IP, mensagem ou endereço de e-mail
feedback-none-saved = Nenhum relatório de falha foi salvo.
feedback-delete-all = Excluir tudo
feedback-app-daemon = Serviço em segundo plano
feedback-report-sent = { $date } · Enviado
feedback-view = Ver
feedback-view-tooltip = Abrir o relatório
feedback-copy-tooltip = Copiar para colar em um relato de bug
feedback-copied = Relatório de falha copiado.
feedback-deleted-all = Relatórios de falhas excluídos.
feedback-read-failed = Não foi possível ler o relatório de falha: { $error }
feedback-delete-failed = Não foi possível excluir o relatório de falha: { $error }
feedback-delete-all-failed = Não foi possível excluir os relatórios de falhas: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = _Arquivo
desktop-menu-new-message = _Nova mensagem
desktop-menu-quit = _Sair
desktop-menu-edit = _Editar
desktop-menu-undo = _Desfazer
desktop-menu-select-all = Selecionar _tudo
desktop-menu-select-none = Selecionar _nenhuma
desktop-menu-find = _Localizar…
desktop-menu-view = E_xibir
desktop-menu-folder-list = Mostrar lista de _pastas
desktop-menu-refresh = _Atualizar
desktop-menu-go = _Ir
desktop-menu-inbox = _Caixa de entrada
desktop-menu-starred = Com _estrela
desktop-menu-sent = _Enviados
desktop-menu-drafts = _Rascunhos
desktop-menu-all-mail = _Todos os e-mails
desktop-menu-next = _Próxima conversa
desktop-menu-previous = Conversa _anterior
desktop-menu-message = _Mensagem
desktop-menu-open = _Abrir
desktop-menu-reply = _Responder
desktop-menu-reply-all = Responder a _todos
desktop-menu-forward = _Encaminhar
desktop-menu-archive = Arq_uivar
desktop-menu-delete = E_xcluir
desktop-menu-spam = _Denunciar spam
desktop-menu-move-to = _Mover para…
desktop-menu-mark-read = Marcar como _lida
desktop-menu-mark-unread = Marcar como _não lida
desktop-menu-star = E_strela
desktop-menu-important = Marcar como _importante
desktop-menu-not-important = Marcar como não i_mportante
desktop-menu-settings = _Configurações
desktop-menu-quick-settings = Configurações _rápidas
desktop-menu-configure = _Configurar o Katna Mail…
desktop-menu-help = Aj_uda
desktop-menu-shortcuts = Atalhos do _teclado
desktop-menu-whats-new = _Novidades
desktop-menu-about = _Sobre o Katna
## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = Navegação
shortcut-group-actions = Ações
shortcut-group-go-to = Ir para
shortcut-group-app = Aplicativo

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = Próxima conversa
shortcut-previous = Conversa anterior
shortcut-down = Descer na lista
shortcut-up = Subir na lista
shortcut-first = Primeira da lista
shortcut-last = Última da lista
shortcut-page-down = Descer uma página na lista
shortcut-page-up = Subir uma página na lista
shortcut-open = Abrir conversa
shortcut-back = Voltar para a lista
shortcut-scroll-down = Rolar para baixo
shortcut-scroll-up = Rolar para cima
shortcut-scroll-page-down = Rolar uma página para baixo
shortcut-scroll-page-up = Rolar uma página para cima
shortcut-compose = Escrever
shortcut-reply = Responder
shortcut-reply-all = Responder a todos
shortcut-forward = Encaminhar
shortcut-archive = Arquivar
shortcut-delete = Excluir
shortcut-spam = Denunciar spam
shortcut-move-to = Mover para
shortcut-mark-read = Marcar como lida
shortcut-mark-unread = Marcar como não lida
shortcut-star = Adicionar ou remover estrela
shortcut-important = Marcar como importante
shortcut-not-important = Marcar como não importante
shortcut-check = Marcar a conversa
shortcut-select-all = Marcar todas as conversas
shortcut-select-none = Desmarcar todas as conversas
shortcut-undo = Desfazer a última ação
shortcut-go-inbox = Caixa de entrada
shortcut-go-starred = Com estrela
shortcut-go-sent = Enviados
shortcut-go-drafts = Rascunhos
shortcut-go-all = Todos os e-mails
shortcut-search = Pesquisar e-mail
shortcut-navigation = Mostrar ou recolher o menu
shortcut-quick-settings = Configurações rápidas
shortcut-settings = Todas as configurações
shortcut-shortcuts = Atalhos do teclado
shortcut-reload = Verificar novos e-mails
shortcut-quit = Sair

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } e depois { $second }

## Settings > Accounts

accounts-folder-pane = Painel de pastas
accounts-folder-pane-detail = De quais contas o painel à esquerda mostra as pastas.
accounts-shown-one = Uma conta por vez; troque no cartão da conta
accounts-shown-all = Todas as contas, uma após a outra
accounts-row = Contas
accounts-row-detail = Remover uma conta exclui a cópia dos e-mails dela que o Katna tem neste computador. Os e-mails continuam no servidor.
accounts-none = Nenhuma conta ainda.
accounts-kind-imported = Importada
accounts-picture-reset = Usar imagem da área de trabalho
accounts-picture-change = Alterar imagem
accounts-remove = Remover
accounts-delete-all-row = Excluir todos os dados
accounts-delete-all-row-detail = Começar do zero, como em uma nova instalação.
accounts-delete-all-about = Exclui deste computador todas as contas, todos os e-mails armazenados, contatos e agendas, o índice de pesquisa, suas configurações e senhas salvas. Nada muda nos seus servidores de e-mail.
accounts-delete-all-open = Excluir todos os dados do Katna

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } foi removida do Katna.
accounts-removed = { $address } foi removida do Katna. Os e-mails dela continuam no servidor.
accounts-all-deleted = Todos os dados do Katna foram excluídos deste computador.

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = Remover { $address }?
accounts-remove-confirm = Remover conta
accounts-removing = Removendo…
accounts-remove-local-mail = { $folders ->
    [0] Todos os e-mails importados para esta conta
    [one] Todos os e-mails importados para esta conta, na pasta dela
    [many] Todos os e-mails importados para esta conta, nas { $folders } de pastas dela
   *[other] Todos os e-mails importados para esta conta, nas { $folders } pastas dela
}
accounts-remove-local-settings = As configurações dela no Katna
accounts-remove-mail = { $folders ->
    [0] Todos os e-mails desta conta armazenados pelo Katna
    [one] Todos os e-mails desta conta armazenados pelo Katna, na pasta dela
    [many] Todos os e-mails desta conta armazenados pelo Katna, nas { $folders } de pastas dela
   *[other] Todos os e-mails desta conta armazenados pelo Katna, nas { $folders } pastas dela
}
accounts-remove-outbox = As mensagens dela aguardando na caixa de saída
accounts-remove-settings = A senha salva e as configurações dela no Katna
accounts-delete-all-title = Excluir todos os dados do Katna?
accounts-delete-all-confirm = Excluir tudo
accounts-deleting = Excluindo…
accounts-delete-all-accounts = Todas as contas, e todos os e-mails e anexos armazenados pelo Katna
accounts-delete-all-contacts = Contatos, agendas e o índice de pesquisa
accounts-delete-all-settings = Todas as configurações, assinaturas e atalhos do teclado
accounts-delete-all-passwords = Todas as senhas salvas
accounts-deleted-heading = Excluído deste computador:
accounts-cannot-undo = Não é possível desfazer esta ação.
accounts-server-delete-all = Nada muda nos seus servidores de e-mail: seus e-mails continuam lá, e adicionar uma conta de novo os baixa novamente. Os e-mails importados de arquivos só existem no Katna; os arquivos não são alterados.
accounts-server-local = Estes e-mails foram importados de arquivos, então o Katna tem a única cópia. Os arquivos de origem não são alterados; importe-os de novo para recuperá-los.
accounts-server-remove = Nada muda no servidor de e-mail: seus e-mails continuam lá, e adicionar a conta de novo os baixa novamente.
accounts-confirm-word = excluir
accounts-confirm-placeholder = Digite “{ accounts-confirm-word }”
accounts-confirm-prompt = Para confirmar, digite “{ accounts-confirm-word }”:
accounts-cancel = Cancelar
