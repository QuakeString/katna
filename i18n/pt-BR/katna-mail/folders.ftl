# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = Marcadores
nav-folders = Pastas
nav-label-new = Criar novo marcador
nav-folder-new = Criar nova pasta
nav-menu-check-mail = Verificar novos e-mails
nav-menu-check-inbox = Verificar esta Caixa de entrada
nav-unified-leave-out = Deixar fora da Caixa de entrada unificada
nav-unified-bring-back = Trazer de volta à Caixa de entrada unificada
nav-menu-sign-in-again = Fazer login de novo
nav-menu-new-mail = Novo e-mail desta conta
nav-menu-account-settings = Configurações da conta
nav-account-checked = Sincronizada · verificada { $ago }
nav-account-in-sync = Sincronizada
nav-account-connecting = Conectando…
nav-account-offline = Off-line, tentando de novo
nav-account-signed-out = Login do { $provider } expirado
nav-account-password-refused = Senha recusada
nav-account-storage = { $used } de { $total } usados
nav-menu-new-subfolder = Nova pasta dentro
nav-menu-new-sublabel = Novo marcador dentro
nav-menu-rename = Renomear
nav-menu-delete = Excluir
nav-menu-empty-trash = Esvaziar a lixeira
nav-account-unnamed = Conta { $number }
nav-all-accounts = Todas as contas
nav-expand = Mostrar pastas
nav-collapse = Ocultar pastas
storage-used = { $percent }% de { $total } usados
storage-used-detail = { $address }: { $used } de { $total } usados

## Special folders (the user's own folders keep their names)

folder-inbox = Caixa de entrada
folder-starred = Com estrela
folder-snoozed = Adiados
folder-unread = Não lidas
folder-important = Importantes
folder-drafts = Rascunhos
folder-sent = Enviados
folder-archive = Arquivo
folder-spam = Spam
folder-trash = Lixeira
folder-all-mail = Todos os e-mails
folder-scheduled = Programados
folder-waiting = Aguardando resposta
folder-waiting-short = Aguardando
folder-reminders = Lembretes
folder-outbox = Caixa de saída
folder-activity = Atividade
folder-not-on-account = Esta conta não tem essa pasta.

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
label-rename-title = Renomear marcador
label-folder-rename-title = Renomear pasta
label-rename = Renomear
label-renaming = Renomeando…
label-renamed = Marcador renomeado para “{ $name }”.
label-folder-renamed = Pasta renomeada para “{ $name }”.

## Deleting a folder or label (asked first)

folder-delete-title = Excluir “{ $name }”?
folder-delete-body = { $count ->
    [0] Ela não tem e-mails. A pasta é removida do servidor, então o webmail e o seu celular também a perdem.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] A { $count } conversa dela vai para a Lixeira, então você ainda pode recuperá-la.
            [many] As { $count } de conversas dela vão para a Lixeira, então você ainda pode recuperá-las.
           *[other] As { $count } conversas dela vão para a Lixeira, então você ainda pode recuperá-las.
        }
       *[message] { $count ->
            [one] A { $count } mensagem dela vai para a Lixeira, então você ainda pode recuperá-la.
            [many] As { $count } de mensagens dela vão para a Lixeira, então você ainda pode recuperá-las.
           *[other] As { $count } mensagens dela vão para a Lixeira, então você ainda pode recuperá-las.
        }
    } A pasta é removida do servidor, então o webmail e o seu celular também a perdem.
}
folder-delete-forever-body = { $count ->
    [0] Ela não tem e-mails. A pasta é removida do servidor, então o webmail e o seu celular também a perdem.
   *[other] { $kind ->
        [conversation] { $count ->
            [one] A { $count } conversa dela é excluída permanentemente; esta conta não tem Lixeira.
            [many] As { $count } de conversas dela são excluídas permanentemente; esta conta não tem Lixeira.
           *[other] As { $count } conversas dela são excluídas permanentemente; esta conta não tem Lixeira.
        }
       *[message] { $count ->
            [one] A { $count } mensagem dela é excluída permanentemente; esta conta não tem Lixeira.
            [many] As { $count } de mensagens dela são excluídas permanentemente; esta conta não tem Lixeira.
           *[other] As { $count } mensagens dela são excluídas permanentemente; esta conta não tem Lixeira.
        }
    } A pasta é removida do servidor, então o webmail e o seu celular também a perdem.
}
folder-delete-label-body = O marcador é removido. Os e-mails dele continuam em Todos os e-mails e nos outros marcadores.
folder-delete-confirm = Excluir pasta
folder-delete-label-confirm = Excluir marcador
folder-deleted = Pasta “{ $name }” excluída
label-deleted = Marcador “{ $name }” excluído
