# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings page: its tabs

settings-tab-general = Geral
settings-tab-inbox = Caixa de entrada
settings-tab-accounts = Contas
settings-tab-katna-account = Conta Katna
settings-tab-subscriptions = Assinatura
settings-tab-appearance = Aparência
settings-tab-shortcuts = Atalhos
settings-tab-default-apps = Apps padrão
settings-tab-folders-rules = Pastas e regras
settings-tab-compose = Escrever
settings-tab-mcp-server = Servidor MCP
settings-tab-feedback = Feedback
settings-tab-experimental = Experimental

## Settings page: tabs still to come

settings-tab-folders-rules-coming = Crie, renomeie, mova e oculte pastas e marcadores, e escolha quais são sincronizados. As regras classificam, marcam, encaminham ou excluem novos e-mails automaticamente, por remetente, assunto ou palavras.
settings-tab-mcp-server-coming = Permita que assistentes de IA neste computador pesquisem, leiam e criem rascunhos dos seus e-mails, com a sua autorização.

## Settings > General

settings-general-conversations = Visualização de conversas
settings-general-conversations-group = Agrupar respostas ao mesmo e-mail
settings-general-conversations-group-detail = Uma linha por conversa na lista
settings-general-reading = Leitura
settings-general-newest-first = Mensagem mais recente primeiro
settings-general-newest-first-detail = A conversa começa pela resposta mais recente
settings-general-full-headers = Mostrar cabeçalhos completos
settings-general-full-headers-detail = De, para, cc, data e assunto abertos em cada mensagem
settings-general-full-names = Nomes completos dos destinatários
settings-general-full-names-detail = “para mim, Ada Lovelace” em vez de “para mim, Ada”
settings-translation = Tradução
settings-translation-detail = E-mails em outro idioma podem ser lidos no seu.
settings-translation-offer = Oferecer tradução
settings-translation-offer-detail = O texto de uma mensagem vai para o servidor do Katna para ser traduzido só quando você pede ou sempre traduz o idioma dela. Os anexos nunca vão.
settings-translation-reading = Traduzir para
settings-translation-always = Sempre traduzir
settings-translation-never = Nunca oferecer para
settings-translation-none = Nenhum ainda. Escolha na barra de tradução de uma mensagem.
settings-general-mark-read = Marcar como lida
settings-general-mark-read-now = Assim que é aberta
settings-general-mark-read-1s = Depois de aberta por 1 segundo
settings-general-mark-read-3s = Depois de aberta por 3 segundos
settings-general-mark-read-never = Só quando eu marcar como lida
settings-general-auto-advance = Avanço automático
settings-general-auto-advance-detail = Depois que você exclui, arquiva ou move a conversa aberta
settings-general-auto-advance-next = Abrir a próxima conversa
settings-general-auto-advance-previous = Abrir a conversa anterior
settings-general-auto-advance-list = Voltar para a lista
settings-general-confirm-delete = Exclusão
settings-general-confirm-delete-ask = Perguntar antes de excluir várias conversas
settings-general-confirm-delete-ask-detail = A exclusão permanente sempre pergunta
settings-general-reply-button = Botão de resposta
settings-general-reply-all = Responder a todos
settings-general-reply-all-detail = O botão de resposta ao lado de cada mensagem responde a todos, não só ao remetente
settings-general-remote-images = Imagens da web
settings-general-remote-images-detail = Carregar as imagens de uma mensagem informa ao remetente que você a abriu, quando e mais ou menos onde. Desativado, cada mensagem pergunta antes, e você sempre pode exibir as imagens de um remetente.
settings-general-remote-images-always = Sempre exibir imagens
settings-general-remote-images-always-detail = Em todas as mensagens, não só de remetentes confiáveis
settings-general-sending = Envio
settings-general-sending-detail = Quanto tempo uma mensagem enviada espera, para que o envio possa ser cancelado.
settings-general-sent-sound = Som ao enviar e-mail
settings-general-sent-sound-detail = Um som curto toca quando a mensagem sai.
settings-general-video-calls = Videochamadas
settings-general-video-calls-detail = Iniciar uma videochamada usa o Google Meet em contas do Gmail. As outras contas recebem uma sala do Jitsi Meet neste servidor; qualquer pessoa com o link pode participar.
settings-general-offline = E-mail off-line
settings-general-offline-detail = Os e-mails recentes são baixados por completo, para ler sem conexão. Os mais antigos são baixados quando você os abre.
settings-general-offline-days = { $count ->
    [one] { $count } dia
    [many] { $count } de dias
   *[other] { $count } dias
}
settings-general-offline-years = { $count ->
    [one] { $count } ano
    [many] { $count } de anos
   *[other] { $count } anos
}
settings-general-offline-all = Todos os e-mails
settings-general-offline-note = Escolher menos dias mantém os e-mails já baixados. Nada muda no servidor.
settings-general-notifications = Notificações
settings-general-notifications-detail = De novos e-mails na Caixa de entrada, mesmo com o Katna Mail fechado.
settings-general-new-mail = Notificar sobre novos e-mails
settings-general-new-mail-detail = Com Responder a todos, Marcar como lida e Arquivar
settings-general-new-mail-sound = Tocar um som
settings-general-new-mail-sound-detail = O som de novo e-mail da área de trabalho
settings-general-updates = Atualizações
settings-general-updates-detail = Instale uma nova versão em Sobre o Katna, ou na notificação de que ela está pronta.
settings-general-auto-download = Baixar atualizações automaticamente
settings-general-auto-download-detail = Nunca em uma conexão limitada. Nada é instalado até você pressionar Atualizar.
settings-general-reset-cache = Redefinir cache
settings-general-reset-cache-detail = Quando os e-mails parecem errados ou desatualizados, ou para liberar espaço em disco. Nada muda nos seus servidores de e-mail.
settings-general-desktop = Área de trabalho
settings-general-start-at-login = Iniciar o Katna ao fazer login
settings-general-start-at-login-detail = Sincroniza os e-mails e mostra as notificações de novos e-mails e o ícone da bandeja, sem abrir a janela
settings-general-login-window = Abrir também a janela do Katna Mail
settings-general-login-window-detail = A janela também abre ao fazer login
settings-general-tray = Mostrar o Katna na bandeja do sistema
settings-general-tray-detail = Com a contagem de não lidas e um menu
settings-general-unread-badge = Contagem de não lidas no ícone da barra de tarefas
settings-general-unread-badge-detail = Quantas mensagens da Caixa de entrada não foram lidas
settings-general-search-triggers = Pesquisar pela área de trabalho
settings-general-search-triggers-detail = Digite uma destas palavras e um espaço no KRunner ou na pesquisa do GNOME e depois o que procura, para pesquisar seus e-mails como a caixa de pesquisa daqui faz. Separe as palavras com vírgulas.
settings-general-search-triggers-none = Nenhuma palavra; só “mail:” funciona

## Settings > Inbox

settings-inbox-tabs = Guias da Caixa de entrada
settings-inbox-tabs-detail = Organize a Caixa de entrada em guias, como faz o site do seu provedor de e-mail.
settings-inbox-tabs-show = Mostrar guias da Caixa de entrada
settings-inbox-tabs-show-detail = Desativado, mostra uma lista para cada conta
settings-inbox-no-accounts = Adicione uma conta para escolher as guias dela.
settings-inbox-tabs-automatic = Automático: { $tabs } ({ $provider })
settings-inbox-tabs-off = Sem guias
settings-inbox-tabs-gmail = Principal, Promoções, Social, Atualizações, Fóruns
settings-inbox-tabs-focused = Destaques e Outros
settings-inbox-tabs-zoho = Caixa de entrada, Newsletters e Notificações
settings-inbox-tabs-shown = Guias exibidas. Os e-mails de uma guia desativada ficam em { $tab }.

## Settings > Appearance

settings-appearance-reading-pane = Painel de leitura
settings-appearance-reading-pane-detail = Onde uma conversa aberta é exibida.
settings-appearance-pane-right = À direita da lista
settings-appearance-pane-none = Sem divisão
settings-appearance-density = Densidade
settings-appearance-density-default = Padrão
settings-appearance-density-compact = Compacta
settings-appearance-scaling = Escala
settings-appearance-scaling-detail = Deixa tudo no Katna Mail maior ou menor, além da escala da própria área de trabalho: texto, ícones, espaçamento e divisórias. Os e-mails que você envia mantêm o próprio tamanho de fonte. Tamanhos muito pequenos podem dificultar o clique nos ícones.
settings-appearance-theme = Tema
settings-appearance-theme-system = Sistema
settings-appearance-theme-light = Claro
settings-appearance-theme-dark = Escuro
settings-appearance-desktop-colors = Cores da área de trabalho
settings-appearance-desktop-colors-use = Usar as cores da área de trabalho
settings-appearance-desktop-colors-use-detail = O esquema de cores e a cor de destaque da área de trabalho
settings-appearance-app-names = Nomes dos apps
settings-appearance-app-names-show = Mostrar nomes dos apps
settings-appearance-app-names-show-detail = Nomes sob os ícones dos apps, à esquerda
settings-appearance-sender-pictures = Imagens dos remetentes
settings-appearance-sender-pictures-show = Mostrar logotipos de empresas
settings-appearance-sender-pictures-show-detail = Buscados pelo domínio do remetente, nunca por mensagem, e guardados por uma semana
settings-appearance-important = Marcadores de importância
settings-appearance-important-show = Mostrar marcadores de importância
settings-appearance-important-show-detail = Ao lado de cada mensagem na lista
settings-appearance-message-width = Largura das mensagens
settings-appearance-message-width-limit = Limitar a largura das mensagens
settings-appearance-message-width-limit-detail = Linhas longas ficam mais fáceis de ler em uma janela larga
settings-appearance-mail-colors = Cores dos e-mails
settings-appearance-mail-colors-detail = A maioria dos e-mails é feita para uma página branca. Com um tema escuro, as cores são trocadas por cores escuras fáceis de ler; desativado, o e-mail mantém as cores do remetente em uma página clara.
settings-appearance-dark-mail = Cores escuras também nos e-mails
settings-appearance-dark-mail-detail = Só enquanto o tema estiver escuro
settings-appearance-attachment-previews = Visualização de anexos
settings-appearance-attachment-previews-show = Mostrar visualização dos anexos
settings-appearance-attachment-previews-show-detail = Uma pequena imagem do conteúdo de cada arquivo no cartão dele

## Settings > Default apps

settings-default-apps-intro = Onde os anexos abrem quando você clica neles. O visualizador também sempre pode abrir um arquivo em outro app. Os apps padrão da área de trabalho são definidos nas configurações dela.
settings-default-apps-pdf = Arquivos PDF
settings-default-apps-pdf-detail = Páginas, com zoom.
settings-default-apps-pictures = Imagens
settings-default-apps-pictures-detail = Fotos (na posição correta), PNG, GIF, WebP, BMP, TIFF e SVG.
settings-default-apps-text = Arquivos de texto
settings-default-apps-text-detail = Texto simples, logs, código e outros textos.
settings-default-apps-sheets = Planilhas
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) e CSV.
settings-default-apps-documents = Documentos
settings-default-apps-documents-detail = Word (docx, doc), texto OpenDocument (odt) e apresentações (pptx, ppt, odp).
settings-default-apps-katna = Visualizador do Katna Mail
settings-default-apps-system = O app padrão da área de trabalho
settings-default-apps-ask = Perguntar qual app todas as vezes
settings-default-apps-after-saving = Depois de salvar
settings-default-apps-show-folder = Mostrar arquivos salvos na pasta deles
settings-default-apps-show-folder-detail = Abre o gerenciador de arquivos com os anexos salvos selecionados

## Settings > Compose

settings-compose-send-from = Enviar novas mensagens de
settings-compose-send-from-detail = Novas mensagens começam por esta conta; a linha De escolhe outra. Respostas e encaminhamentos sempre saem da conta que recebeu a mensagem original.
settings-compose-send-from-current = A conta em que você está
settings-compose-send-on-replies = Enviar em respostas
settings-compose-send-on-replies-detail = O que Enviar faz em uma resposta ou encaminhamento. O menu ao lado de Enviar oferece a outra opção.
settings-compose-send-plain = Enviar
settings-compose-send-archive = Enviar e arquivar
settings-compose-signatures = Assinaturas
settings-compose-signatures-detail = Adicionada abaixo da sua mensagem, depois de uma linha “--”. Escolha outra na janela de escrita.
settings-compose-untitled = Sem título
settings-compose-signature-name = Nome, como Trabalho
settings-compose-signature-first = Minha assinatura
settings-compose-signature-numbered = Assinatura { $number }
settings-compose-signature-delete = Excluir
settings-compose-signature-deleted = Assinatura excluída
settings-compose-signature-new = Criar nova
settings-compose-no-signatures = Nenhuma assinatura ainda.
settings-compose-no-signature = Sem assinatura
settings-compose-for-new-mail = Para novos e-mails
settings-compose-for-replies = Para respostas e encaminhamentos
settings-compose-for-replies-detail = Em uma conversa em que você assinou uma mensagem, a resposta começa com essa assinatura.
settings-compose-format = Formato
settings-compose-plain-text = Escrever em texto simples
settings-compose-plain-text-detail = Novos e-mails começam sem formatação; a janela de escrita pode mudar isso
settings-compose-spelling = Ortografia
settings-compose-spell-check = Verificar a ortografia enquanto escrevo
settings-compose-spell-check-detail = Palavras com erro são sublinhadas, com sugestões no clique com o botão direito
settings-compose-spell-desktop = Idioma da área de trabalho ({ $language })
settings-compose-templates = Modelos
settings-compose-templates-detail = Salve e-mails que você escreve com frequência e comece um novo e-mail ou uma resposta a partir deles.
settings-compose-no-templates = Nenhum modelo ainda. Em uma mensagem, escolha Modelos e depois Salvar como modelo.
settings-compose-template-new = Criar novo
settings-compose-template-new-name = Novo modelo
settings-compose-template-subject = Assunto
settings-compose-template-text = Texto do modelo
settings-compose-template-fields = {"{"}first name{"}"}, {"{"}name{"}"} e {"{"}my name{"}"} são preenchidos com o nome do destinatário e o seu.
settings-compose-template-remove-file = Remover anexo
settings-compose-template-save = Salvar
settings-compose-template-saved = Modelo salvo
settings-compose-template-needs-name = Dê um nome ao modelo
settings-compose-template-delete = Excluir modelo
settings-compose-template-deleted = Modelo excluído
settings-compose-template-delete-failed = Não foi possível excluir o modelo: { $error }

## Settings > Shortcuts

settings-shortcuts-set = Conjunto de atalhos
settings-shortcuts-set-detail = Comece com as teclas de um app de e-mail que você conhece. Cmd é Ctrl aqui. Suas próprias alterações ficam por cima do conjunto, e Restaurar padrões volta às teclas do conjunto.
settings-shortcuts-single = Atalhos de uma tecla
settings-shortcuts-single-detail = Teclas sem Ctrl ou Alt, como no webmail: e arquiva, j e k navegam, / pesquisa. Funcionam na lista e na conversa aberta, nunca enquanto você digita.
settings-shortcuts-single-use = Usar atalhos de uma tecla
settings-shortcuts-single-use-detail = Os atalhos com Ctrl sempre funcionam
settings-shortcuts-how = Clique em uma tecla para alterá-la, ou em + para adicionar uma, e pressione as novas teclas. Esc cancela.
settings-shortcuts-restore = Restaurar padrões
settings-shortcuts-no-key = Sem tecla
settings-shortcuts-press = Pressione as teclas…
settings-shortcuts-then = { $keys } e depois…
settings-shortcuts-moved = { $keys } agora faz “{ $action }” em vez de “{ $previous }”.
settings-shortcuts-single-off = Os atalhos de uma tecla estão desativados, então esta tecla funciona quando forem ativados.
settings-shortcuts-restored = Todos os atalhos voltaram às teclas do conjunto.

## Settings search: the line under a result

settings-general-language-summary = Idioma do app, das datas e dos números
settings-general-reading-summary = Mensagem mais recente primeiro, cabeçalhos completos, nomes completos dos destinatários
settings-translation-summary = Traduzir e-mails em outros idiomas com o servidor do Katna, para o idioma que você escolher
settings-general-mark-read-summary = Quando uma conversa aberta é marcada como lida: na hora, depois de 1 ou 3 segundos, ou manualmente
settings-general-auto-advance-summary = O que abre depois que você exclui, arquiva ou move a conversa aberta: a próxima, a anterior ou a lista
settings-general-confirm-delete-summary = Perguntar antes de mover várias conversas para a Lixeira
settings-general-reply-button-summary = O botão de resposta ao lado de cada mensagem responde a todos
settings-general-remote-images-summary = Sempre exibir as imagens de todas as mensagens
settings-general-sending-summary = Cancelar envio: quanto tempo uma mensagem enviada espera, para que o envio possa ser cancelado
settings-general-video-calls-summary = O servidor do Jitsi Meet para novas videochamadas de contas sem Google Meet
settings-general-offline-summary = Quantos dias de e-mails recentes são baixados por completo, para ler sem conexão
settings-general-notifications-summary = Notificações de novos e-mails e o som delas
settings-general-updates-summary = Baixar novas versões do Katna sozinhas
settings-general-reset-cache-summary = Excluir os e-mails baixados, as imagens dos remetentes e o índice de pesquisa, e baixá-los de novo
settings-general-desktop-summary = Iniciar o Katna ao fazer login, o ícone da bandeja do sistema e a contagem de não lidas no ícone da barra de tarefas
settings-accounts-accounts-summary = Adicionar ou remover uma conta, ou alterar a imagem dela
settings-appearance-density-summary = Linhas padrão ou compactas na lista
settings-appearance-scaling-summary = Deixar tudo maior ou menor: texto, ícones, espaçamento e divisórias
settings-appearance-theme-summary = Sistema, claro ou escuro
settings-appearance-sender-pictures-summary = Logotipos de empresas, buscados pelo domínio do remetente
settings-appearance-important-summary = O marcador de importância ao lado de cada mensagem na lista
settings-appearance-mail-colors-summary = Cores escuras para e-mails em HTML em um tema escuro, ou as cores do remetente
settings-appearance-attachment-previews-summary = Uma pequena imagem do conteúdo de cada anexo
settings-shortcuts-set-summary = Comece com as teclas do Gmail, Inbox by Gmail, Apple Mail, Outlook ou Thunderbird
settings-shortcuts-single-summary = Teclas sem Ctrl ou Alt, como no webmail
settings-default-apps-pdf-summary = Onde os anexos PDF abrem
settings-default-apps-pictures-summary = Onde fotos e imagens abrem
settings-default-apps-text-summary = Onde texto simples, logs e código abrem
settings-default-apps-sheets-summary = Onde arquivos Excel, OpenDocument e CSV abrem
settings-default-apps-documents-summary = Onde documentos Word, texto OpenDocument e apresentações abrem
settings-default-apps-after-saving-summary = Mostrar anexos salvos na pasta deles
settings-compose-send-from-summary = A conta de onde saem os novos e-mails: a primeira, outra, ou aquela em que você está
settings-compose-send-on-replies-summary = Enviar, ou Enviar e arquivar a conversa, em respostas e encaminhamentos
settings-compose-signatures-summary = Adicionada abaixo da sua mensagem, depois de uma linha “--”
settings-compose-for-new-mail-summary = A assinatura com que os novos e-mails começam
settings-compose-for-replies-summary = A assinatura com que respostas e encaminhamentos começam
settings-compose-format-summary = Escrever novos e-mails em texto simples
settings-compose-spelling-summary = Verificar a ortografia ao escrever, e o idioma do dicionário
settings-general-search-triggers-summary = Palavras que pesquisam seus e-mails pelo KRunner ou pela pesquisa do GNOME
settings-compose-templates-summary = Salve e-mails que você escreve com frequência e comece um novo e-mail ou uma resposta a partir deles
settings-feedback-crash-reports-summary = Salvar relatórios de falhas neste computador quando o Katna Mail ou o serviço em segundo plano falhar
settings-feedback-saved-summary = Ver, copiar ou excluir os relatórios de falhas salvos neste computador
settings-feedback-help-improve-summary = Enviar relatórios de falhas para ajudar a corrigir o que deu errado; desativado a menos que você ative
settings-experimental-blur-summary = A área de trabalho aparece desfocada através da barra superior, e os menus ficam foscos
settings-search-shortcut = Atalho do teclado
settings-search-tab = Guia das configurações
settings-search-none = Nenhuma configuração corresponde a “{ $query }”.
settings-search-results = Configurações que correspondem a “{ $query }”

## Settings: opening at login

settings-open-at-login-failed = Não foi possível alterar a inicialização ao fazer login: { $error }

## Settings > General > Time

settings-time = Horário
settings-clock-language = Conforme o idioma
settings-clock-12 = 12 horas, como 2:05 PM
settings-clock-24 = 24 horas, como 14:05
settings-time-summary = Relógio de 12 ou 24 horas, ou conforme o idioma

## Settings > General > Default mail app, Settings > Compose > Grammar

settings-general-mail-app = App de e-mail padrão
settings-general-mail-app-detail = Links de e-mail em outros apps e em sites abrem uma nova mensagem aqui.
mail-app-is-default = O Katna Mail é seu app de e-mail padrão.
mail-app-is-other = Links de e-mail abrem em outro app.
mail-app-make-default = Tornar padrão
mail-app-make-default-failed = Não foi possível alterar o app de e-mail padrão.
settings-general-mail-app-summary = Abrir no Katna Mail os links de e-mail de outros apps e sites
settings-compose-grammar = Gramática
settings-compose-grammar-detail = Verificada neste computador com o Harper. Por enquanto só em inglês: textos em outros idiomas não são alterados.
settings-compose-grammar-check = Verificar a gramática
settings-compose-grammar-check-detail = Sublinhar erros de gramática enquanto você escreve, em inglês
settings-compose-suggestions = Sugestões de escrita
settings-compose-suggestions-detail = Aprendidas neste computador a partir dos e-mails que você enviou e do e-mail que você está respondendo; nada sai dele. Pressione Tab para aceitar uma sugestão ou continue digitando.
settings-compose-suggestions-on = Sugerir enquanto você escreve
settings-compose-suggestions-on-detail = Mostrar em cinza o provável restante de uma frase enquanto você digita
settings-compose-grammar-summary = Sublinhar erros de gramática enquanto você escreve, em inglês
settings-compose-suggestions-summary = Mostrar em cinza o provável restante de uma frase enquanto você digita
