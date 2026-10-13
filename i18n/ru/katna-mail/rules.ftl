# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Правила
settings-rules-summary = Сами сортируют, помечают ярлыками, пересылают или заглушают новые письма
settings-rules-intro = Правила сами сортируют новые письма в этом порядке. Перетащите, чтобы изменить порядок.
settings-rules-all-accounts = Все аккаунты
settings-rules-new = Новое правило
settings-rules-none = Правил пока нет. Правило само сортирует новые письма: по отправителю, теме или словам.
settings-rules-none-account = Для этого аккаунта правил пока нет.
settings-rules-drag = Перетащите, чтобы изменить порядок
settings-rules-edit = Изменить правило
settings-rules-turn-off = Выключить это правило
settings-rules-turn-on = Включить это правило

## Starter rules: offered under the user's own rules, switched off.

## Turning one on makes it one of the user's rules.

settings-rules-starters = Готовые правила
settings-rules-starters-intro = Выключены, пока вы их не включите. Работают для всех ваших аккаунтов; чтобы изменить правило, отредактируйте его.
settings-rules-starter-turning-on = Включаем «{ $name }»…
settings-rules-starter-failed = Не удалось включить «{ $name }»: { $error }
rules-starter-promotions = Промоакции без звука
rules-starter-newsletters = Рассылки — в «Чтение»
rules-starter-receipts = Чеки и счета
rules-starter-deliveries = Доставки
rules-starter-train = Билеты на поезд
rules-starter-flight = Авиабилеты
rules-starter-codes = Одноразовые коды
rules-starter-security = Оповещения безопасности
rules-starter-social = Письма из соцсетей
rules-starter-invites = Приглашения в календарь
rules-starter-folder-reading = Чтение
rules-starter-folder-receipts = Чеки
rules-starter-folder-deliveries = Доставки
rules-starter-folder-travel = Поездки
rules-starter-folder-social = Соцсети
rules-runs-katna = Работает в Katna
rules-runs-gmail = Работает в Gmail
rules-runs-sieve = Работает на сервере
rules-stopped = Остановлено
rules-error-folder-gone = Папки, которую использует это правило, больше нет. Измените правило и выберите другую.
rules-error-no-archive = В этом аккаунте нет папки архива. Измените правило, чтобы оно делало что-то другое.
rules-error-no-trash = В этом аккаунте нет корзины. Измените правило, чтобы оно делало что-то другое.
rules-error-cannot-send = Этот аккаунт не может отправлять письма, поэтому правило не может их пересылать.
rules-error-other = { $error }. Измените правило и снова включите его.
settings-folders = Папки
settings-folders-summary = Счётчики непрочитанных на панели папок
settings-folders-unread-counts = Счётчик непрочитанных у каждой папки
settings-folders-unread-counts-detail = Выкл.: число непрочитанных показывают только «Входящие»

## A rule in one line, on its row: "From contains substack.com → skip the

## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } и { $next }
rules-summary-or = { $first } или { $next }
rules-summary-more = ещё { $count }
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Есть вложение
rules-summary-no-attachment = Нет вложений
rules-summary-mailing-list = Из списка рассылки
rules-summary-not-mailing-list = Не из списка рассылки
rules-summary-tab = На вкладке «{ $tab }»
rules-summary-not-tab = Не на вкладке «{ $tab }»
rules-summary-move = переместить в { $folder }
rules-summary-archive = пропустить «Входящие»
rules-summary-trash = переместить в корзину
rules-summary-mark-read = отметить как прочитанное
rules-summary-star = пометить
rules-summary-important = отметить как важное
rules-summary-label = добавить ярлык { $label }
rules-summary-forward = переслать на { $address }
rules-summary-dont-notify = не уведомлять
rules-summary-read-after = { $count ->
    [one] отметить как прочитанное через { $count } день
    [few] отметить как прочитанное через { $count } дня
    [many] отметить как прочитанное через { $count } дней
   *[other] отметить как прочитанное через { $count } дня
}
rules-summary-folder-gone = несуществующая папка

## The rule editor

rules-editor-new-title = Новое правило
rules-editor-edit-title = Изменить правило
rules-editor-name-hint = Название правила
rules-editor-when = Когда новое письмо соответствует
rules-editor-of-these = из этих условий:
rules-mode-all = всем
rules-mode-any = любому
rules-field-from = От
rules-field-to = Кому
rules-field-cc = Копия
rules-field-any-recipient = Кому или Копия
rules-field-reply-to = Ответить на
rules-field-subject = Тема
rules-field-body = Текст
rules-field-attachment-name = Имя вложения
rules-field-has-attachment = Есть вложение
rules-field-mailing-list = Из списка рассылки
rules-field-tab = Вкладка «Входящих»
rules-comparator-contains = содержит
rules-comparator-not-contains = не содержит
rules-comparator-begins-with = начинается с
rules-comparator-ends-with = заканчивается на
rules-comparator-equals = точно равно
rules-comparator-matches = соответствует шаблону
rules-has-yes = да
rules-has-no = нет
rules-editor-value-hint = Слова или адрес
rules-editor-add-condition = Добавить условие
rules-editor-remove = Удалить
rules-editor-then = Тогда:
rules-action-move = Переместить в
rules-action-archive = Пропустить «Входящие» (архивировать)
rules-action-trash = Переместить в корзину
rules-action-mark-read = Отметить как прочитанное
rules-action-star = Пометить
rules-action-important = Отметить как важное
rules-action-label = Добавить ярлык
rules-action-forward = Переслать на
rules-action-dont-notify = Не уведомлять
rules-action-read-after = Отметить как прочитанное через
rules-editor-choose-folder = Выберите папку
rules-editor-choose-label = Выберите ярлык
rules-editor-new-folder = Новая: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = Адрес электронной почты
rules-editor-days = дн.
rules-editor-add-action = Добавить действие
rules-editor-stop = Остановиться здесь: следующие правила к этому письму не применяются
rules-editor-accounts = Аккаунты:
rules-editor-accounts-none = Выберите аккаунты
rules-editor-accounts-many = { $count ->
    [one] { $count } аккаунт
    [few] { $count } аккаунта
    [many] { $count } аккаунтов
   *[other] { $count } аккаунта
}
rules-editor-matches = Подходит: { $mails } за последние { $days } дн.
rules-editor-mails = { $count ->
    [one] { $count } письмо
    [few] { $count } письма
    [many] { $count } писем
   *[other] { $count } письма
}
rules-editor-counting = Считаем подходящие письма…
rules-editor-show = Показать их
rules-editor-also-apply = Применить и к этим ({ $count })
rules-editor-runs-katna = Работает в Katna, пока этот компьютер включён.
rules-editor-runs-gmail = Работает в Gmail, поэтому действует и на телефоне, и при выключенном компьютере.
rules-editor-runs-sieve = Работает на вашем почтовом сервере, поэтому действует и на телефоне, и при выключенном компьютере.
rules-note-gmail-action = Работает в Katna: фильтры Gmail не поддерживают действие «{ $action }».
rules-note-sieve-action = Работает в Katna: правила вашего почтового сервера не поддерживают действие «{ $action }».
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Работает в Katna: фильтры Gmail не умеют проверять «{ $test }» так, как Katna.
rules-note-sieve-condition = Работает в Katna: правила вашего почтового сервера не умеют проверять «{ $test }» так, как Katna.
rules-note-order = Работает в Katna, как и одно из предыдущих правил аккаунта: правила выполняются по порядку списка.
rules-note-gmail-stop = Работает в Katna: фильтры Gmail не умеют останавливать следующие правила.
rules-note-gmail-forward = Работает в Katna: Gmail пересылает только на адреса, подтверждённые в его настройках, а { $address } не из их числа.
rules-note-gmail-folder = Работает в Katna: в Gmail нет ярлыка для папки, которую использует это правило.
rules-note-sieve-folder = Работает в Katna: на вашем почтовом сервере нет папки, которую использует это правило.
rules-note-gmail-sign-in = Работает в Katna, пока вы снова не войдёте в Google и не разрешите Katna создавать фильтры Gmail.
rules-note-sieve-other-script = Работает в Katna: на вашем почтовом сервере активен другой сценарий правил («{ $name }»).
rules-note-gmail-failed = Работает в Katna: Gmail его не принял ({ $error }).
rules-note-sieve-failed = Работает в Katna: ваш почтовый сервер его не принял ({ $error }).
rules-editor-cancel = Отмена
rules-editor-save = Сохранить
rules-editor-saving = Сохранение…
rules-editor-delete = Удалить правило
rules-editor-delete-ask = Удалить это правило?
rules-editor-delete-keep = Оставить
rules-editor-delete-confirm = Удалить
rules-editor-needs-folder = Выберите папку для каждого действия «Переместить в» и ярлык для каждого «Добавить ярлык».
rules-editor-needs-days = Для «Отметить как прочитанное через» нужно число дней от 1 до 3650.
rules-saved = Правило сохранено
rules-saved-applied = { $count ->
    [one] Правило сохранено и применено к { $count } письму
    [few] Правило сохранено и применено к { $count } письмам
    [many] Правило сохранено и применено к { $count } письмам
   *[other] Правило сохранено и применено к { $count } письма
}
rules-apply-failed = Правило сохранено, но применить его не удалось: { $error }
rules-deleted = Правило удалено
rules-delete-failed = Не удалось удалить правило: { $error }
rules-change-failed = Не удалось изменить правила: { $error }
