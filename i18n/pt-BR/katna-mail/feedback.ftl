# Katna Mail, Portuguese (Brazil) (Português (Brasil)).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > User feedback (crash reports)

feedback-intro-sending = Novos relatórios de falhas são enviados para ajudar a corrigir o que deu errado. Nada mais sai deste computador.
feedback-intro-local = O Katna não envia nada a lugar nenhum. Os relatórios de falhas ficam neste computador, para você consultar ou anexar a um relato de bug.
feedback-crash-reports = Relatórios de falhas
feedback-crash-reports-detail = Gerados quando o Katna Mail ou o serviço em segundo plano falha.
feedback-save = Salvar relatórios de falhas neste computador
feedback-save-detail = Sua pasta pessoal, os nomes de usuário e do computador e os endereços de e-mail são omitidos
feedback-saved = Relatórios de falhas salvos
feedback-saved-detail = { $count ->
    [one] O mais recente é mantido.
    [many] Os { $count } de mais recentes são mantidos.
   *[other] Os { $count } mais recentes são mantidos.
}
feedback-help-improve = Ajude a melhorar o Katna
feedback-help-improve-detail = Desativado a menos que você ative, e você pode desativar aqui a qualquer momento.
feedback-send = Enviar relatórios de falhas
feedback-send-detail = O relatório salvo, exatamente como você pode vê-lo aqui, vai para o rastreador de falhas do Katna (Sentry, na UE). Nenhum endereço IP, mensagem ou endereço de e-mail
feedback-none-saved = Nenhum relatório de falha foi salvo.
feedback-delete-all = Excluir tudo
feedback-app-daemon = Serviço em segundo plano
feedback-report-sent = { $date } · Enviado
feedback-view = Ver
feedback-view-tooltip = Abrir o relatório
feedback-copy-tooltip = Copiar para colar em um relato de bug
feedback-copied = Relatório de falha copiado.
feedback-deleted-all = Relatórios de falhas excluídos.
feedback-read-failed = Não foi possível ler o relatório de falha: { $error }
feedback-delete-failed = Não foi possível excluir o relatório de falha: { $error }
feedback-delete-all-failed = Não foi possível excluir os relatórios de falhas: { $error }

## Settings > User feedback (usage statistics)

feedback-usage = Enviar estatísticas de uso anônimas
feedback-usage-detail = Uma vez por semana: quais recursos você usou, sim ou não. Nunca contagens, endereços, nomes ou termos de pesquisa
feedback-intro-sending-usage = Relatórios de falhas e estatísticas de uso semanais são enviados. Nada mais sai deste computador.
feedback-intro-usage-only = Estatísticas de uso semanais são enviadas. Os relatórios de falhas ficam neste computador.
feedback-counted = O que é contado
feedback-counted-detail = Cada item é sim ou não para a semana.
feedback-counted-also = Também: a versão do Katna, a família do Linux, a área de trabalho, a escala da tela e quantas contas (1, 2–3, 4+)
feedback-see-report = Ver o relatório desta semana
feedback-hide-report = Ocultar o relatório desta semana
feedback-report-goes = Enviado depois que a semana termina, em { $date }, se as estatísticas de uso ainda estiverem ativadas.
feedback-install-id = ID de instalação { $id }
feedback-install-id-tooltip = Aleatório, para que um computador não seja contado duas vezes na mesma semana. Muda a cada 90 dias e nunca é enviado com relatórios de falhas ou feedback
feedback-install-id-reset = Redefinir
feedback-install-id-new = Novo ID de instalação criado.
feedback-report-copied = Relatório copiado.
feedback-send-feedback = Feedback
feedback-send-feedback-detail = Um problema, uma ideia, qualquer coisa.
feedback-send-feedback-button = Enviar feedback…
usage-feature-search-options = Opções de pesquisa
usage-feature-pins = E-mails fixados
usage-feature-labels = Marcadores
usage-feature-scheduled-send = Envio programado
usage-feature-snooze = Adiar e lembretes
usage-feature-encrypted = E-mails criptografados
usage-feature-viewers = Visualizadores integrados
usage-feature-calendar = Agenda
usage-feature-contacts = Contatos
usage-feature-tasks-notes = Tarefas e Notas
usage-feature-phone-layout = Layout para tela de celular
usage-feature-own-frame = Moldura de janela própria do Katna

## Help > Send feedback

send-feedback-title = Enviar feedback
send-feedback-about = Sobre
send-feedback-problem = Problema
send-feedback-idea = Ideia
send-feedback-other = Outra coisa
send-feedback-message = Sua mensagem
send-feedback-message-placeholder = O que aconteceu, ou o que você gostaria?
send-feedback-reply = E-mail para resposta (opcional)
send-feedback-reply-placeholder = voce@example.org
send-feedback-system = Incluir a versão do Katna e o seu sistema
send-feedback-what-is-sent = O que é enviado
send-feedback-show = Mostrar
send-feedback-hide = Ocultar
send-feedback-where = Enviado à caixa de feedback do Katna no Sentry (UE). Nenhum endereço IP, conta, mensagem ou ID de instalação.
send-feedback-cancel = Cancelar
send-feedback-send = Enviar
send-feedback-sending = Enviando…
send-feedback-sent = Feedback enviado. Obrigado
send-feedback-failed = Não foi possível enviar o feedback: { $error }
