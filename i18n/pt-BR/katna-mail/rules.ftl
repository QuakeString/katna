# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Regras
settings-rules-summary = Classificar, marcar, encaminhar ou silenciar novos e-mails automaticamente
settings-rules-intro = As regras classificam novos e-mails automaticamente, nesta ordem. Arraste para reordenar.
settings-rules-all-accounts = Todas as contas
settings-rules-new = Nova regra
settings-rules-none = Nenhuma regra ainda. Uma regra classifica novos e-mails automaticamente: por remetente, assunto ou palavras.
settings-rules-none-account = Nenhuma regra para esta conta ainda.
settings-rules-drag = Arraste para reordenar
settings-rules-edit = Editar regra
settings-rules-turn-off = Desativar esta regra
settings-rules-turn-on = Ativar esta regra

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = Regras prontas
settings-rules-starters-intro = Desativadas até você ativar uma. Elas valem para todas as suas contas; edite uma para alterá-la.
settings-rules-starter-turning-on = Ativando “{ $name }”…
settings-rules-starter-failed = Não foi possível ativar “{ $name }”: { $error }
rules-starter-promotions = Silenciar promoções
rules-starter-newsletters = Newsletters para Leitura
rules-starter-receipts = Recibos e faturas
rules-starter-deliveries = Entregas
rules-starter-train = Passagens de trem
rules-starter-flight = Passagens aéreas
rules-starter-codes = Códigos de uso único
rules-starter-security = Alertas de segurança
rules-starter-social = E-mails de redes sociais
rules-starter-invites = Convites da agenda
rules-starter-folder-reading = Leitura
rules-starter-folder-receipts = Recibos
rules-starter-folder-deliveries = Entregas
rules-starter-folder-travel = Viagens
rules-starter-folder-social = Social
rules-runs-katna = Executada no Katna
rules-runs-gmail = Executada no Gmail
rules-runs-sieve = Executada no servidor
rules-stopped = Parada
rules-error-folder-gone = A pasta que esta regra usa não existe mais. Edite a regra para escolher outra.
rules-error-no-archive = Esta conta não tem pasta de arquivo. Edite a regra para fazer outra coisa.
rules-error-no-trash = Esta conta não tem pasta Lixeira. Edite a regra para fazer outra coisa.
rules-error-cannot-send = Esta conta não pode enviar e-mails, então a regra não pode encaminhá-los.
rules-error-other = { $error }. Edite a regra e ative-a de novo.

settings-folders = Pastas
settings-folders-summary = Contagens de não lidas no painel de pastas
settings-folders-unread-counts = Contagem de não lidas em todas as pastas
settings-folders-unread-counts-detail = Desativado: só a Caixa de entrada mostra quantas estão não lidas

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } e { $next }
rules-summary-or = { $first } ou { $next }
rules-summary-more = mais { $count }
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Tem anexo
rules-summary-no-attachment = Não tem anexo
rules-summary-mailing-list = De uma lista de e-mails
rules-summary-not-mailing-list = Não é de uma lista de e-mails
rules-summary-tab = Na guia { $tab }
rules-summary-not-tab = Fora da guia { $tab }
rules-summary-move = mover para { $folder }
rules-summary-archive = pular a Caixa de entrada
rules-summary-trash = mover para a Lixeira
rules-summary-mark-read = marcar como lida
rules-summary-star = adicionar estrela
rules-summary-important = marcar como importante
rules-summary-label = marcar com { $label }
rules-summary-forward = encaminhar para { $address }
rules-summary-dont-notify = não notificar
rules-summary-read-after = { $count ->
    [one] marcar como lida depois de { $count } dia
    [many] marcar como lida depois de { $count } de dias
   *[other] marcar como lida depois de { $count } dias
}
rules-summary-folder-gone = uma pasta que não existe mais

## The rule editor

rules-editor-new-title = Nova regra
rules-editor-edit-title = Editar regra
rules-editor-name-hint = Nome da regra
rules-editor-when = Quando um novo e-mail atende a
rules-editor-of-these = destas condições:
rules-mode-all = todas
rules-mode-any = qualquer uma
rules-field-from = De
rules-field-to = Para
rules-field-cc = Cc
rules-field-any-recipient = Para ou Cc
rules-field-reply-to = Responder a
rules-field-subject = Assunto
rules-field-body = Texto
rules-field-attachment-name = Nome do anexo
rules-field-has-attachment = Tem anexo
rules-field-mailing-list = De uma lista de e-mails
rules-field-tab = Guia da Caixa de entrada
rules-comparator-contains = contém
rules-comparator-not-contains = não contém
rules-comparator-begins-with = começa com
rules-comparator-ends-with = termina com
rules-comparator-equals = é exatamente
rules-comparator-matches = corresponde ao padrão
rules-has-yes = sim
rules-has-no = não
rules-editor-value-hint = Palavras ou um endereço
rules-editor-add-condition = Adicionar uma condição
rules-editor-remove = Remover
rules-editor-then = Então:
rules-action-move = Mover para
rules-action-archive = Pular a Caixa de entrada (arquivar)
rules-action-trash = Mover para a Lixeira
rules-action-mark-read = Marcar como lida
rules-action-star = Adicionar estrela
rules-action-important = Marcar como importante
rules-action-label = Adicionar marcador
rules-action-forward = Encaminhar para
rules-action-dont-notify = Não notificar
rules-action-read-after = Marcar como lida depois de
rules-editor-choose-folder = Escolha uma pasta
rules-editor-choose-label = Escolha um marcador
rules-editor-new-folder = Nova: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = Endereço de e-mail
rules-editor-days = dias
rules-editor-add-action = Adicionar uma ação
rules-editor-stop = Parar aqui: as regras seguintes não são executadas neste e-mail
rules-editor-accounts = Contas:
rules-editor-accounts-none = Escolher contas
rules-editor-accounts-many = { $count ->
    [one] { $count } conta
    [many] { $count } de contas
   *[other] { $count } contas
}
rules-editor-matches = Corresponde a { $mails } dos últimos { $days } dias
rules-editor-mails = { $count ->
    [one] { $count } e-mail
    [many] { $count } de e-mails
   *[other] { $count } e-mails
}
rules-editor-counting = Contando os e-mails correspondentes…
rules-editor-show = Mostrá-los
rules-editor-also-apply = Aplicar também a estes { $count }
rules-editor-runs-katna = Executada no Katna, enquanto este computador estiver ligado.
rules-editor-runs-gmail = Executada no Gmail, então também funciona no seu celular e com este computador desligado.
rules-editor-runs-sieve = Executada no seu servidor de e-mail, então também funciona no seu celular e com este computador desligado.
rules-note-gmail-action = Executada no Katna: os filtros do Gmail não conseguem “{ $action }”.
rules-note-sieve-action = Executada no Katna: as regras do seu servidor de e-mail não conseguem “{ $action }”.
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Executada no Katna: os filtros do Gmail não conseguem testar “{ $test }” como o Katna faz.
rules-note-sieve-condition = Executada no Katna: as regras do seu servidor de e-mail não conseguem testar “{ $test }” como o Katna faz.
rules-note-order = Executada no Katna, como uma regra anterior da conta: as regras são executadas na ordem da lista.
rules-note-gmail-stop = Executada no Katna: os filtros do Gmail não conseguem impedir que as regras seguintes sejam executadas.
rules-note-gmail-forward = Executada no Katna: o Gmail só encaminha para endereços verificados nas configurações dele, e { $address } não é um deles.
rules-note-gmail-folder = Executada no Katna: o Gmail não tem marcador para uma pasta que esta regra usa.
rules-note-sieve-folder = Executada no Katna: seu servidor de e-mail não tem uma pasta que esta regra usa.
rules-note-gmail-sign-in = Executada no Katna até você fazer login no Google de novo e permitir que o Katna crie filtros no Gmail.
rules-note-sieve-other-script = Executada no Katna: outro script de regras (“{ $name }”) está ativo no seu servidor de e-mail.
rules-note-gmail-failed = Executada no Katna: o Gmail não a aceitou ({ $error }).
rules-note-sieve-failed = Executada no Katna: seu servidor de e-mail não a aceitou ({ $error }).
rules-editor-cancel = Cancelar
rules-editor-save = Salvar
rules-editor-saving = Salvando…
rules-editor-delete = Excluir regra
rules-editor-delete-ask = Excluir esta regra?
rules-editor-delete-keep = Manter
rules-editor-delete-confirm = Excluir
rules-editor-needs-folder = Escolha uma pasta para cada “Mover para” e um marcador para cada “Adicionar marcador”.
rules-editor-needs-days = “Marcar como lida depois de” precisa de um número de dias, de 1 a 3650.
rules-saved = Regra salva
rules-saved-applied = { $count ->
    [one] Regra salva e aplicada a { $count } e-mail
    [many] Regra salva e aplicada a { $count } de e-mails
   *[other] Regra salva e aplicada a { $count } e-mails
}
rules-apply-failed = Regra salva, mas não foi possível aplicá-la: { $error }
rules-deleted = Regra excluída
rules-delete-failed = Não foi possível excluir a regra: { $error }
rules-change-failed = Não foi possível alterar as regras: { $error }
