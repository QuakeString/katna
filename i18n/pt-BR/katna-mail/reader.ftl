# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Fechar
reader-back = Voltar
reader-mark-unread = Marcar como não lida
reader-move-to = Mover para
reader-snooze = Adiar
reader-remind = Lembrar-me
reader-more = Mais
reader-original-colors = Mostrar cores originais
reader-dark-colors = Mostrar em cores escuras
reader-print-all = Imprimir tudo
reader-new-window = Em nova janela
reader-position = { $position } de { $total }
reader-newer = Mais recente
reader-older = Mais antiga

## Reading pane: the conversation

reader-removed = Esta conversa foi removida.
reader-no-subject = (sem assunto)
reader-collapse-all = Recolher tudo
reader-expand-all = Expandir tudo
reader-unknown-sender = (remetente desconhecido)
reader-date-ago = { $date } ({ $ago })
reader-sending = Enviando…
reader-me = mim
reader-to = para { $names }
reader-to-label = para
reader-tick-delivered = Entregue { $when }
reader-tick-no-bounce = Enviado { $when }; nenhuma devolução voltou, então muito provavelmente chegou
reader-tick-bounced = Não entregue: devolvido { $when }
reader-tick-read = Lido { $when } (confirmação de leitura)
reader-tick-opened = Aberto, última vez { $when } (rastreamento de aberturas)
reader-starred = Com estrela
reader-chip-remove = Remover { $label }
reader-not-starred = Sem estrela
reader-too-long = A mensagem é longa demais para ser exibida por completo.
reader-encrypted-images = Imagens da web nunca são carregadas em e-mails criptografados.
reader-window-failed = Não foi possível abrir uma nova janela.

## Reading pane: message details (opened from "to me")

reader-details-from = de:
reader-details-to = para:
reader-details-cc = cc:
reader-details-date = data:
reader-details-subject = assunto:

## Reading pane: downloading a message

reader-downloading = Baixando esta mensagem do servidor…
reader-download-failed = Não foi possível baixar esta mensagem.
reader-download-failed-reason = Não foi possível baixar esta mensagem. { $reason }
reader-download-offline = Esta conta está off-line. Fique on-line para baixar esta mensagem.
reader-try-again = Tentar novamente

## Reply row

reply-reply = Responder
reply-reply-all = Responder a todos
reply-forward = Encaminhar

## Encrypted and signed mail

security-decrypting = Descriptografando…
security-checking = Verificando a assinatura…
security-partly-encrypted = Apenas parte desta mensagem está criptografada. O restante foi adicionado fora da proteção e pode ter vindo de qualquer pessoa.
security-partly-signed = Apenas parte desta mensagem está assinada. O restante foi adicionado fora da proteção e pode ter vindo de qualquer pessoa.
security-encrypted = Mensagem criptografada
security-encrypted-smime = Mensagem criptografada (S/MIME)
security-no-key = Não é possível descriptografar esta mensagem: ela foi criptografada para uma chave que você não tem.
security-cancelled = A descriptografia foi cancelada.
security-damaged = Não é possível descriptografar esta mensagem: os dados criptografados estão danificados ou foram alterados.
security-decrypt-unavailable = Não é possível descriptografar esta mensagem: instale { $tool } para ler e-mails criptografados.
security-decrypt-failed = Não é possível descriptografar esta mensagem: { $reason }
security-unknown-signer = um signatário desconhecido
security-signed-verified = Assinada por { $signer } · verificada
security-signed-not-sender = Assinada por { $signer }, que não é o remetente
security-signed-untrusted = Assinada por { $signer }, com uma chave que você marcou como não confiável
security-signed-unverified = Assinada por { $signer } · a chave não foi verificada
security-bad-signature = Assinatura inválida: esta mensagem foi alterada depois de assinada, ou a assinatura foi falsificada.
security-signature-expired = Assinada por { $signer } · a assinatura expirou
security-key-expired = Assinada por { $signer } · a chave expirou desde então
security-key-revoked = Assinada por { $signer } com uma chave que foi revogada
security-missing-key = Assinada com uma chave que você não tem, por isso não pode ser verificada
security-missing-key-id = Assinada com uma chave que você não tem ({ $key }), por isso não pode ser verificada
security-signature-unavailable = Assinada; instale { $tool } para verificar a assinatura
security-signature-error = Não foi possível verificar a assinatura.
security-look-up-key = Procurar chave

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = Assinatura verificada
key-card-verified-detail = A assinatura é válida e você confia nesta chave.
key-card-unverified = Assinatura não verificada
key-card-unverified-detail = A assinatura é válida, mas nada confirma que a chave é dessa pessoa. Compare a impressão digital com ela e depois confie na chave no GnuPG (Kleopatra ou gpg --edit-key).
key-card-not-sender = Assinada por outra pessoa
key-card-not-sender-detail = A assinatura é válida, mas a chave não é do remetente.
key-card-untrusted = Chave não confiável
key-card-untrusted-detail = Você marcou esta chave como não confiável no GnuPG.
key-card-signature-expired = Assinatura expirada
key-card-signature-expired-detail = A assinatura era válida, mas expirou.
key-card-key-expired = Chave expirada
key-card-key-expired-detail = A assinatura é válida, mas a chave expirou desde então.
key-card-key-revoked = Chave revogada
key-card-key-revoked-detail = O dono revogou esta chave, por isso a assinatura não é confiável.
key-card-bad = Assinatura inválida
key-card-bad-detail = Esta mensagem foi alterada depois de assinada, ou a assinatura foi falsificada.
key-card-signed-by = Assinada por
key-card-belongs-to = Pertence a
key-card-fingerprint = Impressão digital
key-card-signed = Assinada em
key-card-key = Chave
key-card-kind = { $standard }, { $algorithm }
key-card-created = Criada em
key-card-expires = Expira em
key-card-never = Nunca
key-card-issued-by = Emitido por
key-card-found-in = Encontrada em
key-card-keyring = Seu chaveiro do GnuPG
key-card-copy = Copiar impressão digital
key-card-import-title = Importar esta chave?
key-card-from-directory = Encontrada no diretório de chaves de { $domain }.
key-card-from-attachment = Do anexo { $name }.
key-card-import-note = Assim o Katna poderá verificar as assinaturas desta pessoa e criptografar e-mails para ela. Para confiar totalmente na chave, compare a impressão digital com ela.
key-card-cancel = Cancelar
key-card-import = Importar chave
key-card-looking-up = Procurando a chave…
key-card-looking-up-detail = Consultando o diretório de chaves de { $domain }.
key-card-not-found = Nenhuma chave encontrada
key-card-not-found-detail = { $domain } não publica uma chave para este endereço. Peça ao remetente que envie a dele.
key-card-not-kept = A chave encontrada não pode ser usada.
key-card-failed = Não foi possível obter a chave

## Sender checks: the banner on mail that failed the checks the user's mail
## provider ran on its sender (DMARC, DKIM, SPF), and the "?" on the picture
## of a sender nothing confirmed

sender-failed-title = Talvez isto não seja de { $domain }
sender-failed-body = Não passou nas verificações de remetente de { $provider }. Cuidado com links, anexos e respostas.
sender-provider-unknown = seu provedor de e-mail
sender-details = Detalhes
sender-details-hide = Ocultar detalhes
sender-looks-safe = Parece seguro
sender-move-to-spam = Mover para spam
sender-checked-by = Verificado por { $provider }
sender-checked-by-server = Verificado por { $provider } ({ $server })
sender-dmarc = Domínio do remetente (DMARC)
sender-dkim = Assinatura (DKIM)
sender-spf = Servidor de envio (SPF)
sender-result-pass = Aprovado
sender-result-fail = Reprovado
sender-result-unsure = Incerto
sender-result-none = Nenhum
sender-result-missing = Não verificado
sender-dmarc-pass = { $domain } confirma este remetente.
sender-dmarc-fail = O e-mail não corresponde à forma como { $domain } diz que envia seus e-mails.
sender-dmarc-none = { $domain } não publica regras para seus e-mails.
sender-dkim-pass = Assinado por { $domain }.
sender-dkim-fail = A assinatura de { $domain } não corresponde ao e-mail.
sender-dkim-none = A mensagem não foi assinada.
sender-spf-pass = Enviado de um servidor que { $domain } lista.
sender-spf-fail = Enviado de um servidor que { $domain } não lista.
sender-spf-none = { $domain } não lista seus servidores.
sender-check-unsure = A verificação não deu uma resposta clara.
sender-unconfirmed = { $provider } não conseguiu confirmar que isto veio de { $domain }. Qualquer pessoa pode escrever qualquer remetente.
sender-link-title = Abrir este link?
sender-link-body = Este e-mail não passou nas verificações de remetente. O link leva para { $host }:
sender-link-cancel = Cancelar
sender-link-open = Abrir

## Open and click tracking and read receipts (the eye's popover beside a
## sent message's star, and the line above a read receipt)

tracking-opened = { $who } abriu { $count ->
    [one] uma vez
    [many] { $count } de vezes
   *[other] { $count } vezes
}, última vez { $when }
tracking-opens-clicks = { $who } abriu { $opens ->
    [one] uma vez
    [many] { $opens } de vezes
   *[other] { $opens } vezes
} e acessou um link { $clicks ->
    [one] uma vez
    [many] { $clicks } de vezes
   *[other] { $clicks } vezes
}, última vez { $when }
tracking-clicked = { $who } acessou um link { $clicks ->
    [one] uma vez
    [many] { $clicks } de vezes
   *[other] { $clicks } vezes
}, última vez { $when }
tracking-maybe-opened = { $who } pode ter aberto (o Apple Mail carrega imagens por privacidade)
tracking-seen-none = Ninguém abriu nem acessou um link ainda
tracking-receipt = { $who } enviou uma confirmação de leitura
tracking-receipt-read = { $who } leu (confirmação de leitura), { $when }
tracking-receipt-displayed = Confirmação de leitura: { $who } abriu sua mensagem
tracking-receipt-other = Confirmação de leitura: { $who } excluiu ou tratou sua mensagem sem abri-la

## Remote images and pictures

remote-hidden = As imagens desta mensagem estão ocultas.
remote-hidden-unconfirmed = Imagens ocultas: não foi possível confirmar o remetente.
remote-hidden-failed = Imagens ocultas: este e-mail não passou nas verificações de remetente.
remote-show = Exibir imagens
remote-always-show = Sempre exibir deste remetente
remote-picture-use = Usar
remote-picture-too-big = Escolha uma imagem de até 8 MB.
remote-picture-type = Escolha uma imagem PNG, JPEG, GIF, WebP ou SVG.
remote-picture-read-failed = Não é possível ler a imagem: { $error }
remote-picture-keep-failed = Não é possível guardar a imagem: { $error }
remote-picture-remove-failed = Não é possível remover a imagem: { $error }

## Attachments

attachment-count = { $count ->
    [one] Um anexo
    [many] { $count } de anexos
   *[other] { $count } anexos
}
attachment-save = Salvar
attachment-forward = Encaminhar
attachment-save-all = Salvar tudo
attachment-save-all-tooltip = Salvar todos os anexos em uma pasta
attachment-save-here = Salvar aqui
attachment-not-downloaded = Esta mensagem não foi baixada.
attachment-open-message = Abra esta mensagem para ler os anexos.
attachment-not-found = Este anexo não foi encontrado na mensagem.
attachment-read-failed = Não foi possível ler { $name }
attachment-numbered = anexo { $number }
attachment-saved-all = { $count ->
    [one] { $count } arquivo salvo em { $place }
    [many] { $count } de arquivos salvos em { $place }
   *[other] { $count } arquivos salvos em { $place }
}
attachment-saved-some = { $total ->
    [one] { $saved } de { $total } arquivo salvo em { $place }. Não foi possível salvar { $failed }
    [many] { $saved } de { $total } de arquivos salvos em { $place }. Não foi possível salvar { $failed }
   *[other] { $saved } de { $total } arquivos salvos em { $place }. Não foi possível salvar { $failed }
}
attachment-saved-to = Salvo em { $path }
attachment-save-failed = Não foi possível salvar { $name }: { $error }
attachment-open-failed = Não foi possível abrir { $name }: { $error }
attachment-risky = Este arquivo pode executar um programa, por isso o Katna não o abre. Salve-o em vez disso.
attachment-encrypted-open = Este arquivo veio criptografado. Salve-o para abri-lo em outro lugar.

## Printing

print-failed = Não foi possível imprimir: { $error }
print-no-font = nenhuma fonte foi encontrada
print-opened-as-pdf = Aberto como PDF para imprimir a partir dele.

print-preview-title = Visualização de impressão
print-preview-laying-out = Diagramando as páginas…
print-preview-pages = { $count ->
    [one] { $count } página
    [many] { $count } de páginas
   *[other] { $count } páginas
}
print-preview-more = { $count ->
    [one] e mais { $count } página
    [many] e mais { $count } de páginas
   *[other] e mais { $count } páginas
}
print-preview-failed = não foi possível mostrar as páginas
print-preview-paper = Papel
print-preview-a4 = A4
print-preview-letter = Carta
print-preview-layout = Layout
print-preview-as-shown = Como exibido
print-preview-simple = Somente texto
print-preview-backgrounds = Fundos
print-preview-cancel = Cancelar
print-preview-print = Imprimir
print-not-downloaded = (Ainda não baixada.)
print-encrypted = (Criptografada. Abra no Katna Mail para imprimir o texto.)
print-to = Para: { $addresses }
print-cc = Cc: { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = Fixar no topo
text-copy-address = Copiar endereço
text-copy = Copiar
text-select-all = Selecionar tudo
