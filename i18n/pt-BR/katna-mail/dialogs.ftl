# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Sobre o Katna
about-tagline = E-mail e agenda para o desktop Linux
about-version = Katna Mail { $version }
about-copy-version = Copiar detalhes da versão
about-version-copied = Copiado
about-version-built = Compilado: { $date }
about-version-system = Sistema: { $system }
about-whats-new = Novidades

## Updates, in a box under the version in About (only in packages that
## update themselves). $version is a version such as 0.0.0.r236.g1a2b3c4.

about-update-not-checked = As atualizações ainda não foram verificadas
about-update-checking = Verificando atualizações…
about-update-up-to-date = O Katna Mail está atualizado
about-update-check-failed = Não foi possível verificar atualizações
about-update-available = A versão { $version } está disponível
about-update-downloading = Baixando a versão { $version }… { $percent }%
about-update-download-failed = O download da versão { $version } não foi concluído
about-update-ready = A versão { $version } está pronta para instalar
about-update-ready-detail = O Katna Mail reinicia para concluir a atualização.
about-update-confirm = Instalar a versão { $version }?
about-update-confirm-detail = O Katna Mail vai fechar, instalar a atualização e abrir de novo de onde você parou. Seu computador vai pedir sua senha.
about-update-confirm-detail-windows = O Katna Mail vai fechar, instalar a atualização e abrir de novo em instantes.
about-update-installing = Instalando a versão { $version }…
about-update-installing-detail = Digite sua senha na janela que abriu.
about-update-installing-detail-windows = O Katna Mail fecha agora e abre de novo assim que a atualização for instalada.
about-update-cancelled = A atualização não foi instalada, porque a senha não foi informada.
about-update-failed = Não foi possível instalar a atualização: { $error }
about-update-not-self-updating = Esta cópia do Katna Mail não se atualiza sozinha. Atualize-a da mesma forma como você a instalou.
about-update-restart-failed = A atualização está instalada, mas o Katna Mail não conseguiu abrir de novo ({ $error }). Abra-o você mesmo.
about-update-check = Verificar atualizações
about-update-download = Baixar
about-update-retry = Tentar de novo
about-update-button = Atualizar
about-update-restart = Atualizar e reiniciar
about-update-cancel = Agora não
about-changelog = Registro de alterações
about-source = Código-fonte
about-debug-report = Copiar relatório de depuração
about-debug-report-tip = Versões, a verificação após atualizações e linhas recentes do log, para um relatório de bug. Sem e-mails nem senhas.
about-coffee = Me pague um café
about-coffee-coffee = Café?
about-coffee-tea = Chá?
about-coffee-pizza = Pizza?
about-coffee-nothing = Nada? Nadinha?
about-coffee-water = Eu sobrevivo com água!!
about-coffee-thanks = Obrigado por usar o Katna
about-coming-soon = Em breve
about-follow-me = Siga-me em
about-love-title = Feito com amor para Rust, KDE e Linux
about-love-text = Com Rust, escrever um app de e-mail rápido e seguro é um prazer: o Katna não tem código unsafe. O desktop Plasma do KDE e sua suíte PIM inspiraram o Katna, e o Linux e a comunidade de software livre constroem o chão em que ele se apoia. Obrigado, e obrigado às bibliotecas abaixo.
about-kde-text = O KDE faz o desktop em que o Katna se sente mais em casa, criado por voluntários e financiado por pessoas como você. Se você gosta do Plasma ou dos apps do KDE, considere fazer uma doação ao KDE.
about-donate-kde = Doar ao KDE
about-gpui-title = Feito com GPUI, do projeto Zed
about-gpui-text = Toda a interface do Katna Mail é feita com GPUI, o framework de interface rápido e acelerado por GPU que a Zed Industries criou para o editor Zed. Cada pixel, animação e janela que você vê é desenhado por ele. Obrigado, equipe do Zed, por desenvolvê-lo de forma aberta. Apache-2.0.
about-gpui-github = GPUI no GitHub
about-personal-title = Um projeto pessoal
about-personal-text = O Katna Mail não tenta ser novo nem revolucionário. É o app de e-mail que o autor queria, e seus recursos e visual foram emprestados do Gmail, do Mailspring e do Thunderbird. Ele só foi possível graças ao quanto os LLMs avançaram.
about-built-on = FEITO COM SOFTWARE LIVRE
about-credit-pimalaya = IMAP, SMTP e login (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = Leitura e escrita de IMAP
about-credit-tantivy = Pesquisa
about-credit-sqlite = O armazenamento de e-mails
about-credit-rustls = Conexões seguras
about-credit-mail-parser = Leitura de e-mails, da Stalwart Labs
about-credit-html5ever = E-mails em HTML, do projeto Servo
about-credit-zbus = Comunicação com o desktop via D-Bus e portais
about-credit-oo7 = Senhas no chaveiro do desktop
about-credit-hayro = Visualização e impressão de PDFs
about-credit-calamine = Prévias de planilhas
about-credit-resvg = Imagens SVG
about-credit-jiff = Datas e fusos horários
about-credit-spellbook = Verificação ortográfica, do editor Helix
about-credit-smol = Fazer muitas coisas ao mesmo tempo
about-credit-color-schemes = As paletas dos esquemas de cores incluídos
about-all-libraries = Todas as bibliotecas que o Katna usa ({ $count })
about-library-authors = por { $authors }
about-license = O Katna é software livre sob a GNU GPL, versão 3 ou posterior.
about-close = Fechar

## What’s new (shown after an update)

whats-new-title = Novidades do Katna Mail
whats-new-updated = Atualizado para a versão { $version }
whats-new-version = Versão { $version }
whats-new-more = { $count ->
    [one] E mais { $count } no registro de alterações completo.
    [many] E mais { $count } no registro de alterações completo.
   *[other] E mais { $count } no registro de alterações completo.
}
whats-new-changelog = Registro de alterações completo
whats-new-got-it = Entendi

## First run: welcome page

onboarding-welcome-title = Boas-vindas ao Katna Mail
onboarding-welcome-lead = Seus e-mails no seu próprio computador: rápidos de pesquisar, legíveis off-line e privados.
onboarding-fast-title = Rápido, mesmo off-line
onboarding-fast-text = O Katna guarda uma cópia dos seus e-mails aqui, então abri-los e pesquisá-los é instantâneo, com ou sem conexão.
onboarding-providers-title = Funciona com seu e-mail
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud e qualquer outra conta IMAP ou POP.
onboarding-private-title = Privado
onboarding-private-text = Seus e-mails vão direto do seu provedor para este computador. Nenhum servidor do Katna os vê.
onboarding-get-started = Começar

## First run: adding an account

onboarding-service-checking = Verificando o serviço em segundo plano do Katna…
onboarding-service-running = O serviço em segundo plano do Katna está em execução.
onboarding-service-missing = O serviço em segundo plano do Katna não está em execução
onboarding-service-start = Ele busca e envia seus e-mails. Inicie-o em um terminal e verifique de novo:
onboarding-check-again = Verificar de novo
onboarding-account-title = Adicione sua conta de e-mail
onboarding-account-lead = Digite seu endereço de e-mail e sua senha, e o Katna encontra as configurações do servidor. Gmail, Yahoo e iCloud precisam de uma senha de app, criada nas configurações de segurança da sua conta.
onboarding-add-account = Adicionar uma conta
onboarding-back = Voltar

## First run: choosing the look

onboarding-look-title = Deixe do seu jeito
onboarding-look-lead = Escolha como os e-mails abrem e a aparência do Katna. Você pode mudar isso quando quiser nas configurações rápidas.
onboarding-reading-pane = Painel de leitura
onboarding-pane-right = À direita da lista
onboarding-pane-none = Sem divisão
onboarding-theme = Tema
onboarding-theme-system = Sistema
onboarding-theme-light = Claro
onboarding-theme-dark = Escuro
onboarding-density = Densidade
onboarding-density-default = Padrão
onboarding-density-compact = Compacta
onboarding-continue = Continuar

## First start: the Katna account page. A Katna account is an account on
## Katna's own server, not a mail account; see katna-account.ftl.

onboarding-katna-title = Aproveite mais com uma conta Katna
onboarding-katna-lead = É opcional. Ela ativa os recursos on-line do Katna, e você pode criá-la depois em Configurações > Assinatura.
onboarding-katna-receipts-title = Confirmações de leitura
onboarding-katna-receipts-text = Veja quando as pessoas abrem os e-mails que você envia.
onboarding-katna-links-title = Rastreamento de links
onboarding-katna-links-text = Veja quais links dos seus e-mails são clicados.
onboarding-katna-activity-title = Atividade
onboarding-katna-activity-text = Aberturas e cliques de tudo o que você enviou, em um só lugar.
onboarding-katna-translate-title = Tradução automática
onboarding-katna-translate-text = Leia no seu idioma e-mails escritos em outros idiomas.
onboarding-katna-private = Ela tem a própria senha. Os logins dos seus e-mails nunca saem deste computador.

## First run: done

onboarding-ready-title = Tudo pronto
onboarding-ready-lead = O Katna está buscando seus e-mails. Eles aparecem conforme chegam, e os novos e-mails aparecem sozinhos.
onboarding-ready-lead-address = O Katna está buscando os e-mails de { $address }. Eles aparecem conforme chegam, e os novos e-mails aparecem sozinhos.
onboarding-apps = Apps que você vai usar
onboarding-ready-tour = Quer fazer um tour de um minuto para ver onde fica cada coisa?
onboarding-skip = Pular por enquanto
onboarding-take-tour = Fazer o tour

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Ajude a melhorar o Katna
share-lead = Quando o Katna falha, ele salva um relatório neste computador. Enviar esses relatórios ajuda a corrigir o que deu errado. Você pode mudar isso quando quiser em Configurações > Feedback.
share-sent = O que é enviado
share-sent-detail = O relatório de falha como você pode vê-lo em Configurações: o que falhou e onde no Katna, a versão, seu sistema Linux e desktop, e as últimas linhas de log do Katna, que podem citar pastas de e-mail.
share-never-sent = O que nunca é enviado
share-never-sent-detail = Suas mensagens, contatos, senhas, endereço IP, nome de usuário ou nome do computador. Os endereços de e-mail são removidos do relatório.
share-where = Para onde vai
share-where-detail = O rastreador de falhas do Katna no Sentry, armazenado na UE. Nenhum ID associa os relatórios a você.
share-dont-send = Não enviar
share-send = Enviar relatórios de falhas
share-sending = Os relatórios de falhas serão enviados. Obrigado.
share-local = Os relatórios de falhas ficam neste computador.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Boas-vindas ao Katna Mail
tour-welcome-text = Um tour de um minuto mostra onde fica cada coisa.
tour-not-now = Agora não
tour-start = Fazer o tour
tour-close = Fechar
tour-skip = Pular tour
tour-back = Voltar
tour-done = Concluir
tour-next = Próximo
tour-step = { $step } de { $total }
tour-compose-title = Escreva uma mensagem
tour-compose-text = Escrever abre uma nova mensagem no canto inferior direito, para você continuar lendo enquanto escreve.
tour-search-title = Pesquise todos os seus e-mails
tour-search-text = A pesquisa também funciona off-line. O botão na ponta direita adiciona filtros: remetente, destinatário, assunto, datas e anexos.
tour-menu-title = Mostre ou oculte as pastas
tour-menu-text = Este botão recolhe a lista de pastas. Enquanto ela estiver oculta, pare o ponteiro sobre E-mail, à esquerda, para ver as pastas.
tour-apps-title = Seus apps
tour-apps-text = O e-mail fica aqui, ao lado de Agenda, Contatos, Tarefas, Notas e Arquivos.
tour-tabs-title = Guias da Caixa de entrada
tour-tabs-text = Os novos e-mails são separados em Principal, Promoções, Social, Atualizações e Fóruns. Você pode desativar as guias nas configurações rápidas.
tour-list-title = Suas mensagens
tour-list-text = Clique em uma mensagem para lê-la. Passe o ponteiro sobre ela para ver ações rápidas, clique com o botão direito para mais opções ou marque várias para agir sobre elas de uma vez.
tour-settings-title = Configurações rápidas
tour-settings-text = Mude aqui o painel de leitura, a densidade e o tema. O tour também pode ser iniciado de novo a partir daqui.
tour-account-title = Sua conta
tour-account-text = Veja em qual conta você está e adicione outra.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] O serviço em segundo plano do Katna parou inesperadamente.
    [one] O serviço em segundo plano do Katna parou inesperadamente. Há mais { $more } relatório de falha salvo.
    [many] O serviço em segundo plano do Katna parou inesperadamente. Há mais { $more } de relatórios de falhas salvos.
   *[other] O serviço em segundo plano do Katna parou inesperadamente. Há mais { $more } relatórios de falhas salvos.
}
crash-mail = { $more ->
    [0] O Katna Mail fechou inesperadamente da última vez.
    [one] O Katna Mail fechou inesperadamente da última vez. Há mais { $more } relatório de falha salvo.
    [many] O Katna Mail fechou inesperadamente da última vez. Há mais { $more } de relatórios de falhas salvos.
   *[other] O Katna Mail fechou inesperadamente da última vez. Há mais { $more } relatórios de falhas salvos.
}
crash-view = Ver relatório
crash-view-tooltip = Abrir o relatório, salvo neste computador
crash-copy = Copiar relatório
crash-close = Fechar

## Sign in again (a bar at the bottom when Google or Microsoft stopped
## letting an account in; $provider: Google or Microsoft)

sign-in-again-button = Fazer login
sign-in-again-tooltip = Abrir a página de login do { $provider } no navegador
sign-in-again-waiting = Aguardando o navegador…
google-api-off = A { $api } está desativada no projeto do Google Cloud do Katna.
google-api-turn-on = Ativar
google-api-turn-on-tooltip = Abra o Google Cloud para ativar a { $api } e depois pressione Tentar de novo
sign-in-again-done = Login feito de novo em { $address }. Buscando seus e-mails…

## Before deleting several conversations, or deleting for good

delete-ask-title = { $kind ->
    [conversation] { $count ->
        [one] Mover esta conversa para a Lixeira?
        [many] Mover { $count } conversas para a Lixeira?
       *[other] Mover { $count } conversas para a Lixeira?
    }
   *[message] { $count ->
        [one] Mover esta mensagem para a Lixeira?
        [many] Mover { $count } mensagens para a Lixeira?
       *[other] Mover { $count } mensagens para a Lixeira?
    }
}
delete-ask-body = { $count ->
    [one] Você pode desfazer logo depois ou trazê-la de volta da Lixeira mais tarde.
    [many] Você pode desfazer logo depois ou trazê-las de volta da Lixeira mais tarde.
   *[other] Você pode desfazer logo depois ou trazê-las de volta da Lixeira mais tarde.
}
delete-ask-confirm = Mover para a Lixeira
delete-forever-title = { $kind ->
    [conversation] { $count ->
        [one] Excluir esta conversa permanentemente?
        [many] Excluir { $count } conversas permanentemente?
       *[other] Excluir { $count } conversas permanentemente?
    }
   *[message] { $count ->
        [one] Excluir esta mensagem permanentemente?
        [many] Excluir { $count } mensagens permanentemente?
       *[other] Excluir { $count } mensagens permanentemente?
    }
}
delete-forever-body = { $count ->
    [one] Ela também é excluída no servidor. Isso não pode ser desfeito.
    [many] Elas também são excluídas no servidor. Isso não pode ser desfeito.
   *[other] Elas também são excluídas no servidor. Isso não pode ser desfeito.
}
delete-forever-confirm = Excluir permanentemente
delete-ask-dont-ask = Não perguntar novamente
delete-ask-cancel = Cancelar
