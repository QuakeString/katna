# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = Fechar
reader-back = Voltar
reader-mark-unread = Marcar como não lida
reader-move-to = Mover para
reader-more = Mais
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
reader-me = mim
reader-to = para { $names }
reader-starred = Com estrela
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

## Remote images and pictures

remote-hidden = As imagens desta mensagem estão ocultas.
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
attachment-save-all = Salvar tudo
attachment-save-all-tooltip = Salvar todos os anexos em uma pasta
attachment-save-here = Salvar aqui
attachment-not-downloaded = Esta mensagem não foi baixada.
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
print-preview-cancel = Cancelar
print-preview-print = Imprimir
print-not-downloaded = (Ainda não baixada.)
print-encrypted = (Criptografada. Abra no Katna Mail para imprimir o texto.)
print-to = Para: { $addresses }
print-cc = Cc: { $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = Abra esta mensagem para ler os anexos.
text-copy = Copiar
text-select-all = Selecionar tudo
