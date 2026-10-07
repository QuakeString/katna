# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = O servidor de e-mail

problems-signed-out = { $provider } desconectou o Katna de { $address }. Os e-mails pararam de sincronizar.
problems-password-refused = { $provider } recusou a senha de { $address }. Ela pode ter mudado.
problems-no-answer = { $provider } não está respondendo para { $address }. O Katna continua tentando.
problems-offline = Você está off-line. Seus e-mails continuam aqui, e os que você enviar aguardam até você voltar.
problems-accounts-need-you = { $count ->
    [one] 1 conta precisa de você
    [many] { $count } de contas precisam de você
   *[other] { $count } contas precisam de você
}
problems-show = Mostrar
problems-later = Mais tarde
problems-new-password = Nova senha
problems-try-again = Tentar de novo

## The New password card

problems-password-title = Nova senha
problems-password-detail = { $provider } recusou a senha salva de { $address }. Digite a nova; o Katna a verifica antes de guardá-la.
problems-password-placeholder = Senha
problems-password-show = Mostrar senha
problems-password-hide = Ocultar senha
problems-password-cancel = Cancelar
problems-password-save = Salvar
problems-password-checking = Verificando…
problems-password-refused-again = { $provider } também recusou esta senha. Confira e tente de novo.
problems-password-saved = Senha salva para { $address }. Buscando seus e-mails…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = O servidor de e-mail de { $address } não aceitou mover { $count ->
    [one] uma mensagem, então ela voltou para onde estava.
    [many] { $count } de mensagens, então elas voltaram para onde estavam.
   *[other] { $count } mensagens, então elas voltaram para onde estavam.
}
problems-refused-flags = O servidor de e-mail de { $address } não aceitou marcar { $count ->
    [one] uma mensagem (lida, com estrela…), então ela voltou a ser como era.
    [many] { $count } de mensagens (lidas, com estrela…), então elas voltaram a ser como eram.
   *[other] { $count } mensagens (lidas, com estrela…), então elas voltaram a ser como eram.
}
problems-refused-label = O servidor de e-mail de { $address } não aceitou alterar os marcadores de { $count ->
    [one] uma mensagem, então ela voltou a ser como era.
    [many] { $count } de mensagens, então elas voltaram a ser como eram.
   *[other] { $count } mensagens, então elas voltaram a ser como eram.
}
problems-refused-delete = O servidor de e-mail de { $address } não aceitou excluir { $count ->
    [one] uma mensagem, então ela voltou.
    [many] { $count } de mensagens, então elas voltaram.
   *[other] { $count } mensagens, então elas voltaram.
}
problems-refused-other = O servidor de e-mail de { $address } não aceitou { $count ->
    [one] uma alteração, então o Katna a desfez.
    [many] { $count } de alterações, então o Katna as desfez.
   *[other] { $count } alterações, então o Katna as desfez.
}
problems-details = Detalhes

## Katna's background service (katna-daemon) isn't running

service-starting = Iniciando o serviço em segundo plano do Katna…
service-failed = O serviço em segundo plano do Katna não inicia, então os e-mails não estão sincronizando.
service-start-again = Iniciar de novo
service-started-again = O serviço em segundo plano do Katna parou e foi iniciado de novo.
service-details-title = Por que o serviço não inicia
service-details-body = Copie isto e envie com o seu relato. Não contém e-mails nem senhas.
service-details-copy = Copiar
service-details-close = Fechar
