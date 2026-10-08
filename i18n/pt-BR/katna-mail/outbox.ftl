# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Why a message has not gone out (one line under its subject)

outbox-not-sent = Não enviada porque { $reason }.
outbox-retrying = Ainda não enviada porque { $reason }. O Katna tenta de novo sozinho.
outbox-waiting-sign-in = Aguardando você fazer login de novo em { $address }. Ela é enviada em seguida.
outbox-waiting-password = Aguardando a nova senha de { $address }. Ela é enviada em seguida.
outbox-waiting-connection = Aguardando uma conexão. Ela é enviada quando você voltar a ficar on-line.

## Reasons, read from the mail server's answer (they follow "because")

outbox-reason-no-recipients = ela não tem destinatários
outbox-reason-address = um endereço para o qual ela é enviada não existe
outbox-reason-too-large = ela é grande demais para o servidor de e-mail
outbox-reason-blocked = o servidor de e-mail a bloqueou
outbox-reason-gone = a cópia dela neste computador não existe mais
outbox-reason-refused = o servidor de e-mail a recusou

## Buttons and notes

outbox-try-again = Tentar de novo
outbox-edit = Editar
outbox-delete = Excluir
outbox-deleted = Excluída da Caixa de saída
outbox-sending-again = Enviando de novo…

outbox-snackbar-not-sent = “{ $subject }” não foi enviada porque { $reason }.
outbox-open = Caixa de saída
