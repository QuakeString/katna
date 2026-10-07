# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = Abrir a _Caixa de entrada
tray-new-message = _Nova mensagem
tray-new-task = Nova _tarefa
tray-new-note = Nova n_ota
tray-preferences = _Preferências
tray-quit = _Sair

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] Nenhum e-mail não lido
    [one] { $count } mensagem não lida
    [many] { $count } de mensagens não lidas
   *[other] { $count } mensagens não lidas
}

tray-password-refused = Nova senha necessária para { $address }
tray-signed-out = Faça login de novo em { $address }
tray-accounts-need-you = { $count } contas precisam de você
tray-not-sent = { $count ->
    [one] { $count } mensagem não foi enviada
    [many] { $count } de mensagens não foram enviadas
   *[other] { $count } mensagens não foram enviadas
}
