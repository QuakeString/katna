# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = Painel de pastas
accounts-folder-pane-detail = De quais contas o painel à esquerda mostra as pastas.
accounts-shown-one = Uma conta por vez; troque no cartão da conta
accounts-shown-all = Todas as contas, uma após a outra
accounts-unified = Caixa de entrada unificada
accounts-unified-switch = Mostrar juntos os e-mails de todas as contas
accounts-unified-switch-detail = “Todas as contas” fica no topo do painel de pastas, com a caixa de entrada, os e-mails enviados e mais de cada conta em uma só lista. As contas abaixo começam recolhidas.
accounts-row = Contas
accounts-row-detail = O painel de pastas e o menu de contas mostram as contas nesta ordem; a primeira é a padrão. Remover uma conta exclui a cópia dos e-mails dela que o Katna tem neste computador. Os e-mails continuam no servidor.
accounts-none = Nenhuma conta ainda.
accounts-pop3-row = E-mails no servidor
accounts-pop3-row-detail = Contas POP3 baixam os e-mails para este computador. Escolha o que acontece depois com a cópia no servidor.
accounts-pop3-with-katna = Manter até eu excluir no Katna
accounts-pop3-at-once = Excluir assim que for baixado
accounts-pop3-after-days = { $count ->
    [one] Excluir depois de { $count } dia
    [many] Excluir depois de { $count } de dias
   *[other] Excluir depois de { $count } dias
}
accounts-pop3-never = Nunca excluir
accounts-pop3-days-less = Menos dias
accounts-pop3-days-more = Mais dias
accounts-kind-imported = Importada
accounts-picture-reset = Usar imagem da área de trabalho
accounts-picture-change = Alterar imagem
accounts-picture-remove = Remover imagem
account-color-red = Vermelho
account-color-pink = Rosa
account-color-magenta = Magenta
account-color-brown = Marrom
account-color-olive = Oliva
account-color-teal = Azul-petróleo
account-color-indigo = Índigo
account-color-slate = Ardósia
account-color-menu = Cor
accounts-rename = Renomear
accounts-name-save = Salvar
accounts-name-cancel = Cancelar
accounts-name-placeholder = Seu nome
accounts-rename-failed = Não foi possível renomear a conta: { $error }
accounts-move-up = Mover para cima
accounts-move-down = Mover para baixo
accounts-drag = Arraste para mudar a ordem
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

## Reset cache (Settings > General), in the same dialog

reset-cache-about = Exclui os e-mails e anexos que o Katna baixou, as imagens dos remetentes e o índice de pesquisa, e depois baixa de novo os e-mails recentes. Contas, configurações e e-mails que estão só neste computador continuam.
reset-cache-button = Redefinir cache
reset-cache-title = Redefinir o cache?
reset-cache-deleted = Excluídos e baixados de novo:
reset-cache-mail = E-mails e anexos baixados dos seus servidores IMAP: os recentes são baixados de novo agora, os mais antigos quando você os abrir
reset-cache-index = O índice de pesquisa, que é reconstruído na hora
reset-cache-pictures = Imagens dos remetentes
reset-cache-kept = Continuam: suas contas, senhas e configurações; estrelas, marcadores, marcas de lida e fixações; rascunhos, a caixa de saída e alterações que ainda não estão no servidor; e e-mails de contas POP3 ou de arquivos importados, que podem não ter outra cópia. Nada muda nos seus servidores de e-mail.
reset-cache-confirm = Redefinir cache
reset-cache-busy = Redefinindo…
reset-cache-done = O cache foi redefinido. Os e-mails recentes estão sendo baixados de novo.
reset-cache-done-freed = O cache foi redefinido e { $size } foram liberados. Os e-mails recentes estão sendo baixados de novo.
