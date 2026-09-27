# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversa lida está selecionada.
            [many] Todas as { $count } de conversas lidas estão selecionadas.
           *[other] Todas as { $count } conversas lidas estão selecionadas.
        }
       *[message] { $count ->
            [one] { $count } mensagem lida está selecionada.
            [many] Todas as { $count } de mensagens lidas estão selecionadas.
           *[other] Todas as { $count } mensagens lidas estão selecionadas.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversa não lida está selecionada.
            [many] Todas as { $count } de conversas não lidas estão selecionadas.
           *[other] Todas as { $count } conversas não lidas estão selecionadas.
        }
       *[message] { $count ->
            [one] { $count } mensagem não lida está selecionada.
            [many] Todas as { $count } de mensagens não lidas estão selecionadas.
           *[other] Todas as { $count } mensagens não lidas estão selecionadas.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversa com estrela está selecionada.
            [many] Todas as { $count } de conversas com estrela estão selecionadas.
           *[other] Todas as { $count } conversas com estrela estão selecionadas.
        }
       *[message] { $count ->
            [one] { $count } mensagem com estrela está selecionada.
            [many] Todas as { $count } de mensagens com estrela estão selecionadas.
           *[other] Todas as { $count } mensagens com estrela estão selecionadas.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversa sem estrela está selecionada.
            [many] Todas as { $count } de conversas sem estrela estão selecionadas.
           *[other] Todas as { $count } conversas sem estrela estão selecionadas.
        }
       *[message] { $count ->
            [one] { $count } mensagem sem estrela está selecionada.
            [many] Todas as { $count } de mensagens sem estrela estão selecionadas.
           *[other] Todas as { $count } mensagens sem estrela estão selecionadas.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversa lida em { $folder } está selecionada.
            [many] Todas as { $count } de conversas lidas em { $folder } estão selecionadas.
           *[other] Todas as { $count } conversas lidas em { $folder } estão selecionadas.
        }
       *[message] { $count ->
            [one] { $count } mensagem lida em { $folder } está selecionada.
            [many] Todas as { $count } de mensagens lidas em { $folder } estão selecionadas.
           *[other] Todas as { $count } mensagens lidas em { $folder } estão selecionadas.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversa não lida em { $folder } está selecionada.
            [many] Todas as { $count } de conversas não lidas em { $folder } estão selecionadas.
           *[other] Todas as { $count } conversas não lidas em { $folder } estão selecionadas.
        }
       *[message] { $count ->
            [one] { $count } mensagem não lida em { $folder } está selecionada.
            [many] Todas as { $count } de mensagens não lidas em { $folder } estão selecionadas.
           *[other] Todas as { $count } mensagens não lidas em { $folder } estão selecionadas.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversa com estrela em { $folder } está selecionada.
            [many] Todas as { $count } de conversas com estrela em { $folder } estão selecionadas.
           *[other] Todas as { $count } conversas com estrela em { $folder } estão selecionadas.
        }
       *[message] { $count ->
            [one] { $count } mensagem com estrela em { $folder } está selecionada.
            [many] Todas as { $count } de mensagens com estrela em { $folder } estão selecionadas.
           *[other] Todas as { $count } mensagens com estrela em { $folder } estão selecionadas.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] { $count } conversa sem estrela em { $folder } está selecionada.
            [many] Todas as { $count } de conversas sem estrela em { $folder } estão selecionadas.
           *[other] Todas as { $count } conversas sem estrela em { $folder } estão selecionadas.
        }
       *[message] { $count ->
            [one] { $count } mensagem sem estrela em { $folder } está selecionada.
            [many] Todas as { $count } de mensagens sem estrela em { $folder } estão selecionadas.
           *[other] Todas as { $count } mensagens sem estrela em { $folder } estão selecionadas.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] Nenhuma conversa lida aqui.
       *[message] Nenhuma mensagem lida aqui.
    }
   *[unread] { $kind ->
        [conversation] Nenhuma conversa não lida aqui.
       *[message] Nenhuma mensagem não lida aqui.
    }
    [starred] { $kind ->
        [conversation] Nenhuma conversa com estrela aqui.
       *[message] Nenhuma mensagem com estrela aqui.
    }
    [unstarred] { $kind ->
        [conversation] Nenhuma conversa sem estrela aqui.
       *[message] Nenhuma mensagem sem estrela aqui.
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
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] Conversa marcada como lida.
        [many] { $count } de conversas marcadas como lidas.
       *[other] { $count } conversas marcadas como lidas.
    }
   *[message] { $count ->
        [one] Mensagem marcada como lida.
        [many] { $count } de mensagens marcadas como lidas.
       *[other] { $count } mensagens marcadas como lidas.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] Conversa marcada como não lida.
        [many] { $count } de conversas marcadas como não lidas.
       *[other] { $count } conversas marcadas como não lidas.
    }
   *[message] { $count ->
        [one] Mensagem marcada como não lida.
        [many] { $count } de mensagens marcadas como não lidas.
       *[other] { $count } mensagens marcadas como não lidas.
    }
}
toast-undone = Ação desfeita.
toast-nothing-to-undo = Nada para desfazer.
toast-cannot-undo-delete-forever = E-mails excluídos permanentemente não podem ser recuperados.
toast-send-undone = Envio cancelado.
toast-too-late-to-undo-send = Tarde demais para cancelar: a mensagem já foi enviada.
toast-undo = Desfazer
toast-no-spam-folder = Esta conta não tem pasta de spam.
