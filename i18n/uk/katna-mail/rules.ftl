# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = Правила
settings-rules-summary = Самостійно сортувати, позначати мітками, пересилати чи приглушувати нові листи
settings-rules-intro = Правила самі сортують нові листи в такому порядку. Перетягніть, щоб змінити порядок.
settings-rules-all-accounts = Усі облікові записи
settings-rules-new = Нове правило
settings-rules-none = Правил ще немає. Правило саме сортує нові листи: за відправником, темою чи словами.
settings-rules-none-account = Для цього облікового запису правил ще немає.
settings-rules-drag = Перетягніть, щоб змінити порядок
settings-rules-edit = Змінити правило
settings-rules-turn-off = Вимкнути це правило
settings-rules-turn-on = Увімкнути це правило

## Starter rules: offered under the user's own rules, switched off.

## Turning one on makes it one of the user's rules.

settings-rules-starters = Готові правила
settings-rules-starters-intro = Вимкнені, доки ви не ввімкнете котресь. Вони діють для всіх ваших облікових записів; змініть правило, щоб його налаштувати.
settings-rules-starter-turning-on = Вмикання «{ $name }»…
settings-rules-starter-failed = Не вдалося ввімкнути «{ $name }»: { $error }
rules-starter-promotions = Тихі рекламні листи
rules-starter-newsletters = Розсилки в «Читання»
rules-starter-receipts = Чеки й рахунки
rules-starter-deliveries = Доставки
rules-starter-train = Квитки на поїзд
rules-starter-flight = Авіаквитки
rules-starter-codes = Одноразові коди
rules-starter-security = Сповіщення безпеки
rules-starter-social = Листи із соцмереж
rules-starter-invites = Запрошення в календар
rules-starter-folder-reading = Читання
rules-starter-folder-receipts = Чеки
rules-starter-folder-deliveries = Доставки
rules-starter-folder-travel = Подорожі
rules-starter-folder-social = Соцмережі
rules-runs-katna = Працює в Katna
rules-runs-gmail = Працює в Gmail
rules-runs-sieve = Працює на сервері
rules-stopped = Зупинено
rules-error-folder-gone = Папки, яку використовує це правило, більше немає. Змініть правило й виберіть іншу.
rules-error-no-archive = У цьому обліковому записі немає папки архіву. Змініть правило, щоб воно робило щось інше.
rules-error-no-trash = У цьому обліковому записі немає кошика. Змініть правило, щоб воно робило щось інше.
rules-error-cannot-send = Цей обліковий запис не може надсилати листи, тому правило не може їх пересилати.
rules-error-other = { $error }. Змініть правило й увімкніть його знову.
settings-folders = Папки
settings-folders-summary = Лічильники непрочитаних на панелі папок
settings-folders-unread-counts = Лічильник непрочитаних на кожній папці
settings-folders-unread-counts-detail = Якщо вимкнено, кількість непрочитаних показують лише «Вхідні»

## A rule in one line, on its row: "From contains substack.com → skip the

## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first } і { $next }
rules-summary-or = { $first } або { $next }
rules-summary-more = ще { $count }
rules-summary-list = { $first }, { $next }
rules-summary-condition = { $field } { $comparator } { $value }
rules-summary-has-attachment = Має вкладення
rules-summary-no-attachment = Не має вкладень
rules-summary-mailing-list = Зі списку розсилки
rules-summary-not-mailing-list = Не зі списку розсилки
rules-summary-tab = У вкладці «{ $tab }»
rules-summary-not-tab = Не у вкладці «{ $tab }»
rules-summary-move = перемістити в { $folder }
rules-summary-archive = оминути «Вхідні»
rules-summary-trash = перемістити в кошик
rules-summary-mark-read = позначити як прочитане
rules-summary-star = позначити зірочкою
rules-summary-important = позначити як важливе
rules-summary-label = позначити міткою { $label }
rules-summary-forward = переслати на { $address }
rules-summary-dont-notify = не сповіщати
rules-summary-read-after = { $count ->
    [one] позначити як прочитане через { $count } день
    [few] позначити як прочитане через { $count } дні
    [many] позначити як прочитане через { $count } днів
   *[other] позначити як прочитане через { $count } дня
}
rules-summary-folder-gone = папку, якої вже немає

## The rule editor

rules-editor-new-title = Нове правило
rules-editor-edit-title = Змінити правило
rules-editor-name-hint = Назва правила
rules-editor-when = Коли новий лист відповідає
rules-editor-of-these = з цих умов:
rules-mode-all = усім
rules-mode-any = будь-якій
rules-field-from = Від
rules-field-to = Кому
rules-field-cc = Копія
rules-field-any-recipient = Кому або Копія
rules-field-reply-to = Адреса для відповіді
rules-field-subject = Тема
rules-field-body = Текст
rules-field-attachment-name = Назва вкладення
rules-field-has-attachment = Має вкладення
rules-field-mailing-list = Зі списку розсилки
rules-field-tab = Вкладка «Вхідних»
rules-comparator-contains = містить
rules-comparator-not-contains = не містить
rules-comparator-begins-with = починається з
rules-comparator-ends-with = закінчується на
rules-comparator-equals = точно дорівнює
rules-comparator-matches = відповідає шаблону
rules-has-yes = так
rules-has-no = ні
rules-editor-value-hint = Слова або адреса
rules-editor-add-condition = Додати умову
rules-editor-remove = Прибрати
rules-editor-then = Тоді:
rules-action-move = Перемістити в
rules-action-archive = Оминути «Вхідні» (архівувати)
rules-action-trash = Перемістити в кошик
rules-action-mark-read = Позначити як прочитане
rules-action-star = Позначити зірочкою
rules-action-important = Позначити як важливе
rules-action-label = Додати мітку
rules-action-forward = Переслати на
rules-action-dont-notify = Не сповіщати
rules-action-read-after = Позначити як прочитане через
rules-editor-choose-folder = Виберіть папку
rules-editor-choose-label = Виберіть мітку
rules-editor-new-folder = Нова: { $name }
rules-editor-folder-of = { $folder } ({ $account })
rules-editor-forward-hint = Адреса електронної пошти
rules-editor-days = днів
rules-editor-add-action = Додати дію
rules-editor-stop = Зупинитися тут: наступні правила не застосовуються до цього листа
rules-editor-accounts = Облікові записи:
rules-editor-accounts-none = Виберіть облікові записи
rules-editor-accounts-many = { $count ->
    [one] { $count } обліковий запис
    [few] { $count } облікові записи
    [many] { $count } облікових записів
   *[other] { $count } облікового запису
}
rules-editor-matches = Відповідає { $mails } за останні { $days } дн.
rules-editor-mails = { $count ->
    [one] { $count } лист
    [few] { $count } листи
    [many] { $count } листів
   *[other] { $count } листа
}
rules-editor-counting = Підрахунок відповідних листів…
rules-editor-show = Показати їх
rules-editor-also-apply = Застосувати також до цих листів ({ $count })
rules-editor-runs-katna = Працює в Katna, поки цей комп’ютер увімкнено.
rules-editor-runs-gmail = Працює в Gmail, тож діє й на телефоні, і коли цей комп’ютер вимкнено.
rules-editor-runs-sieve = Працює на вашому поштовому сервері, тож діє й на телефоні, і коли цей комп’ютер вимкнено.
rules-note-gmail-action = Працює в Katna: фільтри Gmail не вміють «{ $action }».
rules-note-sieve-action = Працює в Katna: правила вашого поштового сервера не вміють «{ $action }».
rules-note-test = { $field } { $comparator }
rules-note-gmail-condition = Працює в Katna: фільтри Gmail не можуть перевіряти «{ $test }» так, як Katna.
rules-note-sieve-condition = Працює в Katna: правила вашого поштового сервера не можуть перевіряти «{ $test }» так, як Katna.
rules-note-order = Працює в Katna, як і попереднє правило цього облікового запису: правила виконуються в порядку списку.
rules-note-gmail-stop = Працює в Katna: фільтри Gmail не можуть зупинити виконання наступних правил.
rules-note-gmail-forward = Працює в Katna: Gmail пересилає лише на адреси, підтверджені в його налаштуваннях, а { $address } не з них.
rules-note-gmail-folder = Працює в Katna: у Gmail немає мітки для папки, яку використовує це правило.
rules-note-sieve-folder = Працює в Katna: на вашому поштовому сервері немає папки, яку використовує це правило.
rules-note-gmail-sign-in = Працює в Katna, доки ви знову не ввійдете в Google і не дозволите Katna створювати фільтри Gmail.
rules-note-sieve-other-script = Працює в Katna: на вашому поштовому сервері активний інший скрипт правил («{ $name }»).
rules-note-gmail-failed = Працює в Katna: Gmail його не прийняв ({ $error }).
rules-note-sieve-failed = Працює в Katna: ваш поштовий сервер його не прийняв ({ $error }).
rules-editor-cancel = Скасувати
rules-editor-save = Зберегти
rules-editor-saving = Збереження…
rules-editor-delete = Видалити правило
rules-editor-delete-ask = Видалити це правило?
rules-editor-delete-keep = Залишити
rules-editor-delete-confirm = Видалити
rules-editor-needs-folder = Виберіть папку для кожної дії «Перемістити в» і мітку для кожної дії «Додати мітку».
rules-editor-needs-days = Для «Позначити як прочитане через» потрібна кількість днів від 1 до 3650.
rules-saved = Правило збережено
rules-saved-applied = { $count ->
    [one] Правило збережено й застосовано до { $count } листа
    [few] Правило збережено й застосовано до { $count } листів
    [many] Правило збережено й застосовано до { $count } листів
   *[other] Правило збережено й застосовано до { $count } листа
}
rules-apply-failed = Правило збережено, але застосувати його не вдалося: { $error }
rules-deleted = Правило видалено
rules-delete-failed = Не вдалося видалити правило: { $error }
rules-change-failed = Не вдалося змінити правила: { $error }
