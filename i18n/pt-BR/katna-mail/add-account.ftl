# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Adicionar uma conta de e-mail
add-account-providers-intro = Escolha seu provedor de e-mail. O Katna descobre o resto.
add-account-provider-other = Outro e-mail
add-account-provider-other-detail = Qualquer conta IMAP ou POP3
add-account-provider-google-detail = Gmail e Google Workspace
add-account-provider-microsoft-detail = Outlook e Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = Fazer login no { $provider }
add-account-form-title-other = Sua conta de e-mail
add-account-form-intro = O Katna guarda sua senha no chaveiro do sistema.
add-account-looking = Procurando os servidores de e-mail de { $address }…
add-account-address-intro = Digite seu endereço de e-mail. O Katna encontra os servidores para você.
add-account-servers-title = Configurações do servidor
add-account-servers-intro = Onde o Katna lê e envia e-mails de { $address }.
add-account-signing-in = Fazendo login…
add-account-browser-title = Continue no navegador
add-account-browser-intro = O Katna abriu a página de login do { $provider } no seu navegador. Faça login lá e permita que o Katna leia e envie seus e-mails, depois volte aqui.
add-account-browser-hint = Nenhuma página abriu? Confira as janelas do navegador ou volte e tente de novo.
add-account-stage-browser = Aguardando você fazer login no navegador…
add-account-stage-signing-in-at = Fazendo login em { $server }…
add-account-help-app-password-link = Como criar uma senha de app
add-account-help-turn-on-imap = O { $provider } só deixa apps de e-mail entrarem depois que o acesso IMAP e POP3 é ativado nas configurações do webmail dele.
add-account-help-turn-on-imap-link = Como ativar

## Add a mail account: fields

add-account-field-address = Endereço de e-mail
add-account-receive-with = Receber e-mails com
add-account-imap-about = O IMAP mantém seus e-mails e pastas no servidor, iguais em todos os dispositivos. Escolha-o quando puder.
add-account-pop3-about = O POP3 baixa seus e-mails para este computador. Os e-mails que você lê ou move aqui continuam como estão no servidor e nos seus outros dispositivos.
add-account-incoming = E-mails recebidos ({ $protocol })
add-account-outgoing = E-mails enviados ({ $protocol })
add-account-field-server = Servidor
add-account-field-port = Porta
add-account-security-none = Nenhuma
add-account-security-none-warning = Sem criptografia: sua senha e seus e-mails podem ser lidos no caminho.
add-account-field-username = Nome de usuário
add-account-field-password = Senha
add-account-show-password = Mostrar senha
add-account-app-password-hint = O { $provider } precisa de uma senha de app aqui, não da que você usa na web. Crie uma nas configurações de segurança da sua conta { $provider }.
add-account-field-name = Seu nome (opcional)
add-account-name-hint = Mostrado às pessoas para quem você escreve.
add-account-servers-pair = { $imap } e { $smtp }
add-account-servers-found = { $source ->
    [built-in] Servidores: { $servers }, encontrados na lista de provedores do Katna.
    [provider] Servidores: { $servers }, encontrados nas configurações do seu provedor.
    [ispdb] Servidores: { $servers }, encontrados na lista de provedores do Thunderbird.
    [dns] Servidores: { $servers }, encontrados nos registros DNS do seu domínio.
   *[other] Servidores: { $servers }, estimados; confira-os se o login falhar.
}
add-account-servers-entered = Servidores: { $servers }, como digitados.

## Add a mail account: buttons

add-account-sign-in-with = Fazer login com o { $provider }
add-account-sign-in-instead = Fazer login com o { $provider } em vez disso

add-account-servers-button = Configurações do servidor
add-account-back = Voltar
add-account-add = Adicionar conta
add-account-done = Concluído
add-account-another = Adicionar outra conta
add-account-cancel = Cancelar

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] Digite o servidor de entrada.
   *[outgoing] Digite o servidor de saída.
}
add-account-server-space = { $kind ->
    [incoming] O nome do servidor de entrada tem um espaço.
   *[outgoing] O nome do servidor de saída tem um espaço.
}
add-account-port-invalid = { $kind ->
    [incoming] A porta de entrada deve ser um número de { $min } a { $max }.
   *[outgoing] A porta de saída deve ser um número de { $min } a { $max }.
}
add-account-address-empty = Digite um endereço de e-mail.
add-account-address-invalid = Digite um endereço de e-mail como { $example }.
add-account-not-found = O Katna não encontrou os servidores de { $address }, então preencheu os nomes usuais. Confira-os com seu provedor.
add-account-password-empty = Digite a senha.
add-account-name-is-password = O nome é igual à senha. Digite ali o seu nome, do jeito que as pessoas devem vê-lo.
add-account-app-password-refused = O { $provider } recusou a senha. É preciso uma senha de app, não a que você usa na web.
add-account-password-refused = O servidor recusou a senha. Confira-a e tente de novo.
add-account-sign-in-refused = O { $provider } não deixou o Katna entrar. Tente de novo e permita o acesso aos seus e-mails.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Esta cópia do Katna ainda não consegue fazer login em contas Microsoft.
    [Google] Esta cópia do Katna ainda não consegue fazer login em contas Google.
   *[other] Este provedor só permite fazer login na própria página, o que o Katna ainda não consegue fazer para ele.
}
add-account-smtp-not-found = O Katna encontrou onde ler seus e-mails, mas não onde enviá-los. Informe o servidor de saída.

## Add a mail account: the last step

add-account-done-title = Sua conta está pronta
add-account-done-intro = O Katna está buscando seus e-mails agora. Os novos aparecem à medida que chegam.
add-account-done-sign-in = Login
add-account-done-signed-in-with = Com { $provider }, no seu navegador
add-account-done-receiving = Recebimento de e-mails
add-account-done-sending = Envio de e-mails
add-account-done-on-server = E-mails no servidor
add-account-done-kept = Mantidos até você excluí-los no Katna
add-account-done-pop3-hint = Altere o que acontece com os e-mails no servidor em Configurações > Contas.
add-account-done-zoho-title = Tarefas e agendas
add-account-done-zoho-about = O Zoho mantém isso separado dos e-mails. Faça login com o Zoho uma vez para trazê-los para o Katna.
add-account-done-linked = Tarefas e agendas conectadas

## The account menu (from the account button on the top bar)

add-account-menu-another = Adicionar outra conta
app-menu = Menu principal
app-menu-back = Voltar
