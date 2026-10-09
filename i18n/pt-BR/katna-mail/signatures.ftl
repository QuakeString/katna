# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The signature being edited

signature-placeholder = Seu nome e o que quiser acrescentar abaixo dele

## Its formatting bar

signature-bold = Negrito
signature-italic = Itálico
signature-underline = Sublinhado
signature-link = Link
signature-link-apply = Aplicar
signature-picture = Inserir imagem
signature-align-left = Alinhar à esquerda
signature-align-center = Centralizar
signature-align-right = Alinhar à direita
signature-numbered-list = Lista numerada
signature-bulleted-list = Lista com marcadores
signature-remove-formatting = Remover formatação

## Adding a picture

signature-picture-choose = Inserir
signature-picture-too-big = As imagens de uma assinatura podem ter até { $size }.
signature-picture-kind = Escolha uma imagem PNG, JPEG, GIF ou WebP.
signature-picture-unreadable = { $name }: { $error }

## Layouts

signature-layout = Layout
signature-layout-own = Seu próprio
signature-layout-classic = Clássico
signature-layout-logo-left = Logotipo à esquerda
signature-layout-photo = Foto
signature-layout-band = Faixa colorida
signature-layout-one-line = Uma linha
signature-layout-centred = Centralizado
signature-layout-banner = Com banner
signature-layout-underline = Sublinhado
signature-layout-side-bar = Barra lateral
signature-layout-card = Cartão
signature-layout-monogram = Monograma
signature-layout-plain = Texto simples
signature-layout-mobile-label = C:
signature-layout-office-label = T:
signature-layout-email-label = E:
signature-layout-name = Nome
signature-layout-job = Cargo
signature-layout-company = Empresa
signature-layout-mobile = Celular
signature-layout-office = Escritório
signature-layout-email = E-mail
signature-layout-website = Site
signature-layout-address = Endereço
signature-layout-pictures = Imagens
signature-layout-logo = Logotipo
signature-layout-photo-picture = Foto
signature-layout-banner-picture = Banner
signature-layout-remove-picture = Remover
signature-layout-pages = Páginas
signature-layout-page-placeholder = Adicione o endereço de uma página
signature-layout-colour = Cor
signature-layout-picture-failed = { $name } não pôde ser usado como imagem.
signature-layout-preview = Como o destinatário vê
signature-layout-light = Claro
signature-layout-dark = Escuro
signature-layout-text = Texto simples
signature-layout-inside = As imagens vão dentro do e-mail, então aparecem mesmo onde as imagens da web estão desativadas. Esta acrescenta { $size } a cada e-mail.
signature-layout-edit = Editar manualmente
signature-layout-edit-confirm = Editar manualmente? Os campos e o layout dela somem, e ela mantém a aparência até onde o editor consegue.
signature-layout-use-confirm = Usar o layout { $layout }? Ele substitui esta assinatura, preenchido a partir dela.
signature-layout-use = Usar layout
signature-layout-cancel = Cancelar

## Paste HTML

signature-html-title = Colar HTML
signature-html-subtitle = Para uma assinatura que você criou em outro lugar
signature-html-placeholder = Cole aqui o HTML da assinatura
signature-html-name = Colada
signature-html-new = Salva como nova assinatura, “{ $name }”
signature-html-replaces = Substitui “{ $name }”
signature-html-cancel = Cancelar
signature-html-save = Salvar
signature-html-fetching = Baixando as imagens dela…
signature-html-pictures-inside = { $count ->
    [one] { $count } imagem baixada e colocada dentro do e-mail ({ $size })
    [many] { $count } de imagens baixadas e colocadas dentro do e-mail ({ $size })
   *[other] { $count } imagens baixadas e colocadas dentro do e-mail ({ $size })
}
signature-html-pictures-web = { $count ->
    [one] { $count } imagem não pôde ser baixada, então os destinatários a carregam da web
    [many] { $count } de imagens não puderam ser baixadas, então os destinatários as carregam da web
   *[other] { $count } imagens não puderam ser baixadas, então os destinatários as carregam da web
}
signature-html-removed = Scripts, formulários e pixels de rastreamento removidos, que os apps de e-mail bloqueiam de qualquer forma
signature-html-style-sheet = Uma folha de estilos ficou de fora: o e-mail só mantém os estilos escritos em cada parte
signature-html-links = Links removidos que levavam a algo que não é um site, um endereço ou um telefone
signature-html-plain-text = Versão em texto simples criada a partir dela, para apps de e-mail que só mostram texto

## Import

signature-import-title = Importar
signature-import-subtitle = Do Gmail, Thunderbird, Evolution e KMail
signature-import-looking = Procurando assinaturas…
signature-import-none = Nenhuma assinatura encontrada. Para outro app, copie o HTML da assinatura dele e use Colar HTML.
signature-import-from = Do { $app }
signature-import-already = já está no Katna
signature-import-gmail-sign-in = { $address }: faça login de novo em Configurações > Contas para que o Katna possa ler as assinaturas do Gmail.
signature-import-gmail-failed = { $address }: { $error }
signature-import-cancel = Cancelar
signature-import-do = { $count ->
    [one] Importar { $count } assinatura
    [many] Importar { $count } de assinaturas
   *[other] Importar { $count } assinaturas
}
signature-import-name = { $name } ({ $app })
