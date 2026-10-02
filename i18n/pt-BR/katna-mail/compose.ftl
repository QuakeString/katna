# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = Nova mensagem
compose-restore = Restaurar
compose-minimize = Minimizar
compose-exit-full-screen = Sair da tela cheia
compose-open-window = Abrir em uma nova janela
compose-save-close = Salvar e fechar
compose-back-to-mail = Voltar à janela de e-mail
compose-pop-out-reply = Abrir resposta em outra janela
compose-edit-recipients = Editar destinatários
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Cco: { $names }
compose-more-recipients = mais { $count }
compose-show-trimmed = Mostrar conteúdo cortado
compose-hide-trimmed = Ocultar conteúdo cortado
compose-remove-trimmed = Remover texto citado
compose-trimmed-removed = Texto citado removido

## Recipients and subject

compose-to = Para
compose-cc = Cc
compose-bcc = Cco
compose-from = De
compose-from-choose = Enviar de outra conta
compose-recipients = Destinatários
compose-subject = Assunto

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = Envie ou descarte primeiro a mensagem aberta.
compose-bad-address = “{ $address }” não é um endereço de e-mail.
compose-no-recipients = Adicione pelo menos um destinatário.
compose-attachments-too-large = Os anexos somam { $size }; os servidores de e-mail aceitam até { $limit }.
compose-no-account = Adicione uma conta para enviar e-mails.
compose-past-time = Escolha um horário no futuro.
compose-scheduling = Programando…
compose-sending = Enviando…
compose-scheduled = Envio programado para { $when }
compose-sent-archived = Enviada e arquivada
compose-sent = Mensagem enviada
compose-discarded = Rascunho descartado
compose-draft-saved = Rascunho salvo
compose-draft-saving = Salvando…
compose-draft-failed = Não foi possível salvar o rascunho: { $error }
compose-draft-not-opened = Não foi possível abrir o rascunho.

## Attachments

compose-picker-insert = Inserir
compose-picker-attach = Anexar
compose-file-too-large = { $name } é grande demais: uma mensagem pode levar até { $limit }.
compose-forward-files-missing = Os arquivos da mensagem encaminhada não foram baixados, então não estão anexados.
compose-attachment-size = ({ $size })
compose-remove-attachment = Remover anexo
compose-attachment-open-tip = Abrir para conferir
compose-attachments-total = { $count ->
    [one] { $count } arquivo, { $size }
    [many] { $count } de arquivos, { $size }
   *[other] { $count } arquivos, { $size }
}
compose-drive-note = { $name } passa de { $limit }, então vai para o seu Google Drive e a mensagem leva um link.
compose-drive-tip = No seu Google Drive; a mensagem leva um link
compose-drive-uploading = Enviando { $percent }%
compose-drive-allow = Permitir o Drive
compose-drive-allow-tip = Entre de novo com o Google para o Katna poder colocar arquivos grandes no seu Drive
compose-drive-retry = Tentar de novo
compose-drive-sends-when-uploaded = Será enviada quando { $name } terminar de subir
compose-drive-not-uploaded = { $name } ainda não está no Google Drive
compose-drive-share-failed = Não foi possível compartilhar os arquivos no Google Drive: { $error }
compose-drive-share-title = Compartilhar os arquivos com todos?
compose-drive-share-text = { $count ->
    [one] O Google Drive não pode compartilhar os arquivos com { $addresses }, que não tem conta Google. Em vez disso, qualquer pessoa com o link poderá abri-los.
    [many] O Google Drive não pode compartilhar os arquivos com { $addresses }, que não têm conta Google. Em vez disso, qualquer pessoa com o link poderá abri-los.
   *[other] O Google Drive não pode compartilhar os arquivos com { $addresses }, que não têm conta Google. Em vez disso, qualquer pessoa com o link poderá abri-los.
}
compose-drive-share-link = Compartilhar com link
compose-drive-send-without = Enviar sem compartilhar
compose-drive-share-cancel = Cancelar
compose-drive-card-detail = { $size } · Google Drive
compose-drive-card-name = Google Drive
compose-onedrive-note = { $name } passa de { $limit }, então vai para o seu OneDrive e a mensagem leva um link.
compose-onedrive-tip = No seu OneDrive; a mensagem leva um link
compose-onedrive-allow = Permitir o OneDrive
compose-onedrive-allow-tip = Entre de novo com a Microsoft para o Katna poder colocar arquivos grandes no seu OneDrive
compose-onedrive-not-uploaded = { $name } ainda não está no OneDrive
compose-onedrive-share-failed = Não foi possível compartilhar os arquivos no OneDrive: { $error }
compose-onedrive-share-text = { $count ->
    [one] O OneDrive não pode compartilhar os arquivos com { $addresses }. Em vez disso, qualquer pessoa com o link poderá abri-los.
    [many] O OneDrive não pode compartilhar os arquivos com { $addresses }. Em vez disso, qualquer pessoa com o link poderá abri-los.
   *[other] O OneDrive não pode compartilhar os arquivos com { $addresses }. Em vez disso, qualquer pessoa com o link poderá abri-los.
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-onedrive-card-name = OneDrive
compose-drop-files = Solte os arquivos aqui
compose-drop-here = Solte aqui

## Paste options (a small bar under what was just pasted or dropped)

compose-paste-keep-formatting = Manter formatação
compose-paste-table = Tabela
compose-paste-picture = Imagem
compose-paste-plain-text = Texto simples
compose-paste-inline = No texto
compose-paste-attachment = Anexo

## Encryption and signing (the toggles by the recipients)

compose-encrypt = Criptografar
compose-encrypted = Criptografada: só os destinatários podem lê-la
compose-sign = Assinar
compose-signed = Assinada: os destinatários podem confirmar que é sua

## Open and click tracking and read receipts (toggles after Sign)

compose-track = Rastrear aberturas e cliques
compose-tracked = Rastreada: você vê quando cada destinatário a abre ou acessa um link
compose-track-clicks = Rastrear cliques em links (texto simples não mostra aberturas)
compose-tracked-clicks = Rastreada: você vê quando cada destinatário acessa um link
compose-track-sign-in = Faça login em uma conta Katna para rastrear aberturas e cliques
compose-receipt = Pedir confirmação de leitura
compose-receipt-on = Confirmação de leitura pedida: o app do destinatário pode pedir que ele envie uma
compose-delivery = Pedir confirmação de entrega
compose-delivery-on = Confirmação de entrega pedida: seu servidor de e-mail vai enviar um e-mail quando o servidor de cada destinatário aceitar a mensagem
compose-delivery-unavailable = Seu servidor de e-mail não envia confirmações de entrega

## Spelling

spell-no-dictionary = Nenhum dicionário ortográfico para { $language } está instalado (por exemplo, hunspell-en_us).
spell-dictionary-error = Dicionário ortográfico: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = Adicionar “{ $words }”
grammar-remove = Remover “{ $words }”
grammar-ignore = Ignorar

## Send checks (asked before a message goes out)

send-check-attachment-title = Você queria anexar arquivos?
send-check-attachment-text = Você mencionou um anexo, mas nada foi anexado.
send-check-attach = Anexar um arquivo
send-check-subject-title = Enviar sem assunto?
send-check-subject-text = Esta mensagem não tem assunto.
send-check-add-subject = Adicionar assunto
send-check-send-anyway = Enviar assim mesmo

## Recipients (To, Cc and Bcc)

recipient-not-valid = Não é um endereço de e-mail válido
recipient-show-address = Mostrar endereço
recipient-remove = Remover
recipient-bad-title = Verifique o endereço
recipient-bad-text = “{ $address }” não é um endereço de e-mail válido. Corrija ou remova antes de enviar.
recipient-bad-fix = Corrigir
