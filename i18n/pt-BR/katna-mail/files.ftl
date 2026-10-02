# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Pesquisar arquivos

## Left side (and chips on a phone)

files-all = Todos os arquivos
files-pictures = Imagens
files-pdfs = PDFs
files-documents = Documentos
files-sheets = Planilhas
files-slides = Apresentações
files-other = Outros
files-accounts = Contas
files-drives = Drives
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Compartilhados comigo
files-shown = Exibidos
files-received = Recebidos
files-sent = Enviados por mim

## Over the files

files-count = { $count ->
    [one] { $count } arquivo · { $size }
    [many] { $count } de arquivos · { $size }
   *[other] { $count } arquivos · { $size }
}
files-anyone = Qualquer pessoa
files-from-person = De { $name }
files-time-any = Qualquer data
files-time-today = Hoje
files-time-yesterday = Ontem
files-time-this-week = Esta semana
files-time-last-week = Semana passada
files-time-this-month = Este mês
files-time-last-month = Mês passado
files-time-between = { $first } – { $last }
files-time-hint = Clique em um dia ou arraste por vários dias
files-time-summary = { $count ->
    [one] { $days } · { $count } arquivo
    [many] { $days } · { $count } de arquivos
   *[other] { $days } · { $count } arquivos
}
files-time-clear = Limpar
files-time-month-back = Mês anterior
files-time-month-on = Próximo mês
files-time-wheel = Role para mover estas datas, mantendo a duração
files-sort-newest = Mais recentes primeiro
files-sort-oldest = Mais antigos primeiro
files-sort-largest = Maiores primeiro
files-sort-name = Por nome
files-grid = Cartões
files-list = Lista
files-this-week = Esta semana
files-undated = Sem data
files-me = Eu
files-no-subject = (sem assunto)
files-loading = Reunindo os arquivos dos seus e-mails…
files-empty = Os arquivos dos seus e-mails aparecem aqui.
files-none-match = Nenhum arquivo corresponde.
files-load-failed = Falha ao ler os arquivos: { $error }

## A file's menu and buttons

files-open = Abrir
files-open-with = Abrir com…
files-save = Salvar…
files-show-mail = Mostrar o e-mail
files-mail-window = Abrir o e-mail em uma nova janela
files-forward = Encaminhar o arquivo
files-from-them = Arquivos de { $name }
files-copy-name = Copiar nome do arquivo
files-name-copied = Nome do arquivo copiado
files-downloading = Baixando o e-mail…
files-download-failed = Não foi possível baixar este e-mail.

## A cloud drive in place of the mail files

files-drive-mine = Meu Drive
files-drive-mine-onedrive = Meus arquivos
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files ->
        [one] 1 arquivo
        [many] { $files } de arquivos
       *[other] { $files } arquivos
    }
    [one] 1 pasta · { $files ->
        [one] 1 arquivo
        [many] { $files } de arquivos
       *[other] { $files } arquivos
    }
    [many] { $folders } de pastas · { $files ->
        [one] 1 arquivo
        [many] { $files } de arquivos
       *[other] { $files } arquivos
    }
   *[other] { $folders } pastas · { $files ->
        [one] 1 arquivo
        [many] { $files } de arquivos
       *[other] { $files } arquivos
    }
}
files-drive-folders = Pastas
files-drive-files = Arquivos
files-drive-folder = Pasta
files-drive-meta = { $what } · Editado em { $date }
files-drive-as-link = { $what } · como link
files-drive-google-doc = Documento Google
files-drive-google-sheet = Planilha Google
files-drive-google-slides = Apresentação Google
files-drive-google-drawing = Desenho Google
files-drive-fetching = Buscando…
files-drive-loading = Abrindo o drive…
files-drive-empty = Esta pasta está vazia.
files-drive-unreachable = Não foi possível acessar o { $drive }.
files-drive-try-again = Tentar de novo
files-drive-needs-permission = O Katna precisa da sua permissão uma vez para mostrar este drive. Faça login de novo e permita que o Katna veja seus arquivos.
files-drive-allow = Permitir
files-drive-allow-failed = O login não foi concluído, então o drive continua fechado.
files-drive-attach = Anexar
files-drive-more = Mais
files-drive-download = Baixar…
files-drive-open-web = Abrir no { $drive }
files-drive-copy-link = Copiar link
files-drive-link-copied = Link copiado
files-drive-share = Compartilhar…
files-drive-rename = Renomear
files-drive-trash = Mover para a lixeira
files-drive-trashed = “{ $name }” está na lixeira do { $drive }
files-drive-renamed = Renomeado para “{ $name }”
files-drive-getting = Buscando { $name } no { $drive }…
files-drive-get-failed = Não foi possível buscar { $name }: { $error }
files-drive-upload = Enviar
files-drive-upload-files = Enviar arquivos
files-drive-upload-folder = Enviar pasta
files-drive-upload-failed = Não foi possível enviar { $name }: { $error }
files-drive-upload-needs = Para enviar, o Katna precisa da sua permissão uma vez: pressione Permitir em Configurações › Apps padrão › Página Arquivos.

## The Share dialog of a drive file or folder

files-share-title = Compartilhar “{ $name }”
files-share-add = Adicione pessoas por nome ou endereço
files-share-not-address = “{ $text }” não é um endereço de e-mail
files-share-notify = Deixar o { $drive } avisá-las por e-mail também
files-share-people = Pessoas com acesso
files-share-general = Acesso geral
files-share-loading = Verificando quem tem acesso…
files-share-restricted = Restrito
files-share-restricted-about = Só pessoas com acesso podem abrir com o link
files-share-anyone = Qualquer pessoa com o link
files-share-anyone-can = { $role ->
    [editor] Qualquer pessoa com o link pode editar
    [commenter] Qualquer pessoa com o link pode comentar
   *[viewer] Qualquer pessoa com o link pode ver
}
files-share-anyone-about = { $role ->
    [editor] Qualquer pessoa na internet com o link pode editar
    [commenter] Qualquer pessoa na internet com o link pode comentar
   *[viewer] Qualquer pessoa na internet com o link pode ver
}
files-share-role-owner = Proprietário
files-share-role-editor = Editor
files-share-role-commenter = Comentarista
files-share-role-viewer = Leitor
files-share-you = { $name } (você)
files-share-domain = Todos em { $domain }
files-share-inherited = Acesso por uma pasta em que está
files-share-remove = Remover acesso
files-share-copy-link = Copiar link
files-share-share = Compartilhar
files-share-done = Concluído
files-share-sharing = Compartilhando…
files-share-shared = { $count ->
    [one] Compartilhado com 1 pessoa
    [many] Compartilhado com { $count } de pessoas
   *[other] Compartilhado com { $count } pessoas
}
files-share-refused = O { $drive } não conseguiu compartilhar com { $addresses }
files-share-failed = Não foi possível alterar o compartilhamento: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] Enviando 1 item
    [many] Enviando { $count } de itens
   *[other] Enviando { $count } itens
}
files-tray-done = { $count ->
    [one] 1 envio concluído
    [many] { $count } de envios concluídos
   *[other] { $count } envios concluídos
}
files-tray-some-failed = { $done } enviados, { $failed } com falha
files-tray-minutes-left = { $minutes ->
    [one] Falta cerca de um minuto
    [many] Faltam cerca de { $minutes } de minutos
   *[other] Faltam cerca de { $minutes } minutos
}
files-tray-seconds-left = Falta menos de um minuto
files-tray-starting = Iniciando…
files-tray-cancel-all = Cancelar tudo
files-tray-cancel = Cancelar
files-tray-fold = Ocultar a lista
files-tray-unfold = Mostrar a lista
files-tray-close = Fechar
files-tray-progress = { $place } · { $sent } de { $size }
files-tray-in = Em { $place }
files-tray-cancelled = Cancelado
