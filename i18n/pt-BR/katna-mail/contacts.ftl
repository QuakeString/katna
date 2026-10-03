# Katna Mail, Portuguese (Brazil) (Português (Brasil)): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = Contatos
contacts-frequent = Frequentes
contacts-other = Outros contatos
contacts-other-about = Pessoas para quem você enviou e-mails pelo Gmail, mas não salvou
contacts-other-email = Enviar e-mail
contacts-other-empty = Nenhum outro contato. As pessoas para quem você envia e-mails pelo Gmail, mas não salva, aparecem aqui.
contacts-other-allow = Para ver outros contatos, faça login novamente na sua conta do Gmail e permita que o Katna os veja.
contacts-labels = Marcadores
contacts-label-options = Opções do marcador
contacts-label-rename = Renomear marcador
contacts-label-email = Enviar e-mail para todos
contacts-label-delete = Excluir marcador
contacts-label-new = Novo marcador
contacts-label-name = Nome do marcador
contacts-label-button = Marcador
contacts-label-menu = Marcar como:
contacts-label-added = Adicionado a { $name }
contacts-label-removed = Removido de { $name }
contacts-label-renamed = Marcador renomeado para { $name }
contacts-label-deleted = Marcador excluído: { $name }
contacts-label-no-email = Ninguém neste marcador tem endereço de e-mail
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = Contas
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = Fazer login de novo para mostrar os contatos
contacts-account-signed-in = Login feito de novo em { $address }. Buscando seus contatos…
contacts-account-sign-in-refused = O { $provider } não deixou o Katna entrar. Tente de novo e permita o acesso aos seus contatos.
contacts-account-password = O servidor não aceitou a senha. Yahoo, iCloud, Zoho e outros precisam de uma senha de app.
contacts-account-change-password = Alterar senha
contacts-account-change-password-tooltip = Abrir Configurações > Contas
contacts-account-failed = Não foi possível ler os contatos.
# $reason is the server's own words, in English.
contacts-account-error = Não foi possível ler os contatos: { $reason }
contacts-account-none = Nenhum catálogo de endereços encontrado
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = Nenhum catálogo de endereços encontrado: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = O { $provider } só mostra os contatos ao Katna conectado com o { $provider }.
contacts-account-sign-in-with = Fazer login com o { $provider }
contacts-account-looking = Procurando contatos…
contacts-account-try-again = Tentar de novo
contacts-account-try-again-tooltip = Verificar agora os contatos desta conta de novo
contacts-account-fixing = Resolvendo…
contacts-manage = Corrigir e gerenciar
contacts-merge = Mesclar e corrigir
contacts-merge-about = { $count ->
    [one] { $count } sugestão: contatos que parecem ser a mesma pessoa
    [many] { $count } de sugestões: contatos que parecem ser a mesma pessoa
   *[other] { $count } sugestões: contatos que parecem ser a mesma pessoa
}
contacts-merge-none = Nenhuma duplicata. Contatos com o mesmo nome ou número de telefone aparecem aqui.
contacts-merge-count = { $count ->
    [one] { $count } contato
    [many] { $count } de contatos
   *[other] { $count } contatos
}
contacts-merge-all = Mesclar tudo
contacts-merge-button = Mesclar
contacts-merge-dismiss = Dispensar
contacts-merged = { $count ->
    [1] Contatos mesclados
    [one] { $count } mesclagem concluída
    [many] { $count } de mesclagens concluídas
   *[other] { $count } mesclagens concluídas
}
contacts-import = Importar
contacts-export = Exportar
contacts-import-file = Importar contatos de um arquivo vCard ou CSV
contacts-imported = { $count ->
    [one] { $count } contato importado para { $place }
    [many] { $count } de contatos importados para { $place }
   *[other] { $count } contatos importados para { $place }
}
contacts-imported-some = { $count ->
    [one] { $count } contato importado para { $place }; { $skipped } já salvos, ignorados
    [many] { $count } de contatos importados para { $place }; { $skipped } já salvos, ignorados
   *[other] { $count } contatos importados para { $place }; { $skipped } já salvos, ignorados
}
contacts-import-none = Nenhum contato encontrado em { $name }
contacts-import-all-saved = Todos em { $name } já estão salvos
contacts-import-failed = Não foi possível ler { $name }: { $error }
contacts-exported = { $count ->
    [one] { $count } contato exportado para { $path }
    [many] { $count } de contatos exportados para { $path }
   *[other] { $count } contatos exportados para { $path }
}
contacts-export-none = Nenhum contato para exportar
contacts-export-failed = Não foi possível exportar os contatos: { $error }
contacts-print = Imprimir
contacts-print-title = Contatos
contacts-print-none = Nenhum contato para imprimir
contacts-print-typed = { $value } ({ $kind })
contacts-print-birthday = Aniversário: { $day }
contacts-print-nickname = Apelido: { $name }
contacts-create = Novo contato

## Search and the list

contacts-search = Pesquisar contatos
contacts-loading = Carregando contatos…
contacts-empty = Nenhum contato salvo ainda. Os contatos que você salva no Gmail, no Outlook ou no seu serviço de e-mail aparecem aqui.
contacts-empty-no-books = Os contatos das suas contas aparecem aqui depois de sincronizados.
contacts-none-found = Nenhum contato corresponde à sua pesquisa.
contacts-starred = { $count ->
    [one] Contato com estrela ({ $count })
    [many] Contatos com estrela ({ $count })
   *[other] Contatos com estrela ({ $count })
}
contacts-count = Contatos ({ $count })
contacts-col-name = Nome
contacts-col-email = E-mail
contacts-col-phone = Número de telefone
contacts-col-job = Cargo e empresa
contacts-col-labels = Marcadores

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Permitir que o Katna leia os contatos de { $address }.
contacts-allow-many = { $more ->
    [one] Permitir que o Katna leia os contatos de { $address } e de mais { $more } conta.
    [many] Permitir que o Katna leia os contatos de { $address } e de mais { $more } de contas.
   *[other] Permitir que o Katna leia os contatos de { $address } e de mais { $more } contas.
}
contacts-allow-button = Permitir

## A contact's page

contacts-back = Voltar aos contatos
contacts-edit = Editar
contacts-delete = Excluir
contacts-qr = Compartilhar como código QR
contacts-qr-about = Escaneie isto com a câmera de um celular para salvar o contato.
contacts-qr-too-long = Este contato tem detalhes demais para caber em um código QR.
contacts-qr-done = Concluído
contacts-deleted = Contato excluído: { $name }
contacts-added = { $name } adicionado aos contatos
contacts-find-mail = E-mail
contacts-details = Detalhes do contato
contacts-saved-in = Salvo em
contacts-notes = Notas
contacts-birthday = Aniversário
contacts-nickname = Apelido
contacts-this-computer = Este computador
contacts-kind-home = Casa
contacts-kind-work = Trabalho
contacts-kind-mobile = Celular
contacts-kind-other = Outro
contacts-source-google = Contatos do Google
contacts-source-microsoft = Contatos do Outlook
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = Criar contato
contacts-edit-title = Editar contato
contacts-edit-save = Salvar
contacts-edit-saving = Salvando…
contacts-edit-cancel = Cancelar
contacts-saved = Contato salvo
contacts-edit-save-to = Salvar em
contacts-edit-changes-go-to = As alterações são salvas em { $place }.
contacts-edit-given = Nome
contacts-edit-family = Sobrenome
contacts-edit-company = Empresa
contacts-edit-job = Cargo
contacts-edit-email = E-mail
contacts-edit-phone = Telefone
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = Adicionar e-mail
contacts-edit-add-phone = Adicionar telefone
contacts-edit-street = Endereço
contacts-edit-city = Cidade
contacts-edit-postcode = CEP
contacts-edit-country = País
contacts-edit-birthday = Aniversário (YYYY-MM-DD)
contacts-edit-empty = Primeiro, adicione um nome, um e-mail ou um telefone.
