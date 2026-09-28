# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = Adicionar uma conta de e-mail
add-account-looking = Procurando os servidores de e-mail de { $address }…
add-account-address-intro = Digite seu endereço de e-mail. O Katna encontra os servidores para você.
add-account-servers-title = Configurações do servidor
add-account-servers-intro = Onde o Katna lê e envia e-mails de { $address }.
add-account-password-title = Digite sua senha
add-account-signing-in = Fazendo login…
add-account-browser-title = Continue no navegador
add-account-browser-intro = O Katna abriu a página de login do { $provider } no seu navegador. Faça login lá e permita que o Katna leia e envie seus e-mails, depois volte aqui.
add-account-browser-hint = Nenhuma página abriu? Confira as janelas do navegador ou volte e tente de novo.

## Add a mail account: fields

add-account-field-address = Endereço de e-mail
add-account-incoming = E-mails recebidos ({ $protocol })
add-account-outgoing = E-mails enviados ({ $protocol })
add-account-field-server = Servidor
add-account-field-port = Porta
add-account-security-none = Nenhuma
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
add-account-or = ou
add-account-sign-in-with = Fazer login com o { $provider }
add-account-sign-in-instead = Fazer login com o { $provider } em vez disso

## Add a mail account: buttons

add-account-servers-button = Configurações do servidor
add-account-back = Voltar
add-account-add = Adicionar conta
add-account-next = Próximo
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
add-account-added = { $address } adicionado. Buscando seus e-mails…
add-account-app-password-refused = O { $provider } recusou a senha. É preciso uma senha de app, não a que você usa na web.
add-account-password-refused = O servidor recusou a senha. Confira-a e tente de novo.
add-account-sign-in-refused = O { $provider } não deixou o Katna entrar. Tente de novo e permita o acesso aos seus e-mails.
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] Esta cópia do Katna ainda não consegue fazer login em contas Microsoft.
    [Google] Esta cópia do Katna ainda não consegue fazer login em contas Google.
   *[other] Este provedor só permite fazer login na própria página, o que o Katna ainda não consegue fazer para ele.
}
add-account-signed-in = Login feito com o { $provider }. Buscando seus e-mails…

## The account menu (from the account button on the top bar)

add-account-menu-another = Adicionar outra conta
add-account-menu-manage = Gerenciar contas
