# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings page: its tabs

settings-tab-general = Общие
settings-tab-inbox = Входящие
settings-tab-accounts = Аккаунты
settings-tab-katna-account = Аккаунт Katna
settings-tab-subscriptions = Подписки
settings-tab-appearance = Внешний вид
settings-tab-shortcuts = Быстрые клавиши
settings-tab-default-apps = Приложения по умолчанию
settings-tab-folders-rules = Папки и правила
settings-tab-compose = Написание писем
settings-tab-mcp-server = Сервер MCP
settings-tab-feedback = Отзывы пользователей
settings-tab-experimental = Экспериментальные

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Смотрите, какие рассылки и списки рассылки вы получаете, и отписывайтесь в один клик.
settings-tab-folders-rules-coming = Создавайте, переименовывайте, перемещайте и скрывайте папки и ярлыки, выбирайте, какие из них синхронизировать. Правила сами сортируют, помечают ярлыками, пересылают или удаляют новые письма — по отправителю, теме или словам.
settings-tab-mcp-server-coming = Разрешите ИИ-помощникам на этом компьютере искать, читать и составлять черновики ваших писем — с вашего согласия.

## Settings > General

settings-general-conversations = Цепочки писем
settings-general-conversations-group = Группировать ответы на одно письмо
settings-general-conversations-group-detail = Одна строка на цепочку в списке
settings-general-reading = Чтение
settings-general-newest-first = Сначала новые письма
settings-general-newest-first-detail = Цепочка начинается с последнего ответа
settings-general-full-headers = Показывать заголовки полностью
settings-general-full-headers-detail = От кого, кому, копия, дата и тема открыты в каждом письме
settings-general-full-names = Полные имена получателей
settings-general-full-names-detail = «мне, Ada Lovelace», а не «мне, Ada»
settings-translation = Перевод
settings-translation-detail = Письма на другом языке можно читать на вашем.
settings-translation-offer = Предлагать перевод
settings-translation-offer-detail = Текст письма отправляется на сервер Katna для перевода, только когда вы об этом просите или всегда переводите его язык. Вложения не отправляются никогда.
settings-translation-reading = Переводить на
settings-translation-always = Всегда переводить
settings-translation-never = Никогда не предлагать для
settings-translation-none = Пока нет. Выберите на панели перевода над письмом.
settings-general-mark-read = Отмечать как прочитанное
settings-general-mark-read-now = Сразу при открытии
settings-general-mark-read-1s = Через 1 секунду после открытия
settings-general-mark-read-3s = Через 3 секунды после открытия
settings-general-mark-read-never = Только когда я отмечу сам
settings-general-auto-advance = Автопереход
settings-general-auto-advance-detail = После удаления, архивирования или перемещения открытой цепочки
settings-general-auto-advance-next = Открыть следующую цепочку
settings-general-auto-advance-previous = Открыть предыдущую цепочку
settings-general-auto-advance-list = Вернуться к списку
settings-general-reply-button = Кнопка «Ответить»
settings-general-reply-all = Отвечать всем
settings-general-reply-all-detail = Кнопка ответа рядом с каждым письмом отвечает всем, а не только отправителю
settings-general-remote-images = Изображения из интернета
settings-general-remote-images-detail = Загрузка изображений письма сообщает отправителю, что вы его открыли, когда и примерно где. Если выключено, каждое письмо сначала спрашивает, и вы всегда можете показать изображения отправителя.
settings-general-remote-images-always = Всегда показывать изображения
settings-general-remote-images-always-detail = В каждом письме, а не только от надёжных отправителей
settings-general-sending = Отправка
settings-general-sending-detail = Сколько отправленное письмо ждёт, чтобы его можно было отменить.
settings-general-offline = Почта офлайн
settings-general-offline-detail = Недавние письма загружаются целиком, чтобы читать их без подключения. Более старые загружаются, когда вы их открываете.
settings-general-offline-days = { $count ->
    [one] { $count } день
    [few] { $count } дня
    [many] { $count } дней
   *[other] { $count } дня
}
settings-general-offline-years = { $count ->
    [one] { $count } год
    [few] { $count } года
    [many] { $count } лет
   *[other] { $count } года
}
settings-general-offline-all = Вся почта
settings-general-offline-note = Если выбрать меньше дней, уже загруженные письма останутся. На сервере ничего не меняется.
settings-general-notifications = Уведомления
settings-general-notifications-detail = О новых письмах во «Входящих», даже когда Katna Mail закрыта.
settings-general-new-mail = Уведомлять о новых письмах
settings-general-new-mail-detail = С кнопками «Ответить всем», «Отметить как прочитанное» и «Архивировать»
settings-general-new-mail-sound = Воспроизводить звук
settings-general-new-mail-sound-detail = Звук новой почты рабочего стола
settings-general-reset-cache = Сброс кэша
settings-general-reset-cache-detail = Когда почта выглядит неправильно или устаревшей, или чтобы освободить место на диске. На почтовых серверах ничего не меняется.
settings-general-desktop = Рабочий стол
settings-general-start-at-login = Запускать Katna при входе в систему
settings-general-start-at-login-detail = Синхронизирует почту и показывает уведомления о новых письмах и значок в лотке, не открывая окно
settings-general-login-window = Открывать также окно Katna Mail
settings-general-login-window-detail = Окно тоже открывается при входе в систему
settings-general-tray = Показывать Katna в системном лотке
settings-general-tray-detail = Со счётчиком непрочитанных и меню
settings-general-unread-badge = Счётчик непрочитанных на значке в панели задач
settings-general-unread-badge-detail = Сколько писем во «Входящих» не прочитано
settings-general-search-triggers = Поиск с рабочего стола
settings-general-search-triggers-detail = Введите в KRunner или в поиске GNOME одно из этих слов и пробел, а затем то, что нужно найти, чтобы искать в почте так же, как в здешней строке поиска. Разделяйте слова запятыми.
settings-general-search-triggers-none = Слов нет; работает только «mail:»

## Settings > Inbox

settings-inbox-tabs = Вкладки «Входящих»
settings-inbox-tabs-detail = Сортировать входящие по вкладкам, как это делает сайт вашего почтового сервиса.
settings-inbox-tabs-show = Показывать вкладки «Входящих»
settings-inbox-tabs-show-detail = Если выключено, для каждого аккаунта один список
settings-inbox-no-accounts = Добавьте аккаунт, чтобы выбрать его вкладки.
settings-inbox-tabs-automatic = Автоматически: { $tabs } ({ $provider })
settings-inbox-tabs-off = Без вкладок
settings-inbox-tabs-gmail = Несортированные, Промоакции, Соцсети, Оповещения, Форумы
settings-inbox-tabs-focused = Отсортированные и Другие
settings-inbox-tabs-zoho = Входящие, Рассылки и Уведомления
settings-inbox-tabs-shown = Показанные вкладки. Письма с выключенной вкладки остаются во вкладке «{ $tab }».

## Settings > Appearance

settings-appearance-reading-pane = Область просмотра
settings-appearance-reading-pane-detail = Где показывается открытая цепочка.
settings-appearance-pane-right = Справа от списка
settings-appearance-pane-none = Без разделения
settings-appearance-density = Плотность
settings-appearance-density-default = Обычная
settings-appearance-density-compact = Компактная
settings-appearance-scaling = Масштаб
settings-appearance-scaling-detail = Делает всё в Katna Mail крупнее или мельче поверх масштаба рабочего стола: текст, значки, отступы и разделители. Отправляемые письма сохраняют свой размер шрифта. При очень малом масштабе по значкам трудно попасть.
settings-appearance-theme = Тема
settings-appearance-theme-system = Системная
settings-appearance-theme-light = Светлая
settings-appearance-theme-dark = Тёмная
settings-appearance-desktop-colors = Цвета рабочего стола
settings-appearance-desktop-colors-use = Использовать цвета рабочего стола
settings-appearance-desktop-colors-use-detail = Цветовая схема и акцентный цвет рабочего стола
settings-appearance-app-names = Названия приложений
settings-appearance-app-names-show = Показывать названия приложений
settings-appearance-app-names-show-detail = Подписи под значками приложений слева
settings-appearance-sender-pictures = Изображения отправителей
settings-appearance-sender-pictures-show = Показывать логотипы компаний
settings-appearance-sender-pictures-show-detail = Ищутся по домену отправителя, никогда по письму, и хранятся неделю
settings-appearance-important = Маркеры важности
settings-appearance-important-show = Показывать маркеры важности
settings-appearance-important-show-detail = Рядом с каждым письмом в списке
settings-appearance-message-width = Ширина письма
settings-appearance-message-width-limit = Ограничить ширину писем
settings-appearance-message-width-limit-detail = Так в широком окне легче читать длинные строки
settings-appearance-mail-colors = Цвета писем
settings-appearance-mail-colors-detail = Большинство писем рассчитаны на белый фон. В тёмной теме их цвета заменяются тёмными, удобными для чтения; если выключено, письмо сохраняет цвета отправителя на светлом фоне.
settings-appearance-dark-mail = Тёмные цвета и для писем
settings-appearance-dark-mail-detail = Только когда тема тёмная
settings-appearance-attachment-previews = Предпросмотр вложений
settings-appearance-attachment-previews-show = Показывать предпросмотр вложений
settings-appearance-attachment-previews-show-detail = Миниатюра содержимого каждого файла на его карточке

## Settings > Default apps

settings-default-apps-intro = Где открываются вложения, когда вы на них нажимаете. Из окна просмотра файл всегда можно открыть и в другом приложении. Приложения по умолчанию для рабочего стола задаются в его собственных настройках.
settings-default-apps-pdf = Файлы PDF
settings-default-apps-pdf-detail = Страницы с масштабированием.
settings-default-apps-pictures = Изображения
settings-default-apps-pictures-detail = Фотографии (с правильной ориентацией), PNG, GIF, WebP, BMP, TIFF и SVG.
settings-default-apps-text = Текстовые файлы
settings-default-apps-text-detail = Обычный текст, журналы, код и другой текст.
settings-default-apps-sheets = Таблицы
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) и CSV.
settings-default-apps-documents = Документы
settings-default-apps-documents-detail = Word (docx, doc), текст OpenDocument (odt) и презентации (pptx, ppt, odp).
settings-default-apps-katna = Просмотрщик Katna Mail
settings-default-apps-system = Приложение по умолчанию на рабочем столе
settings-default-apps-ask = Каждый раз спрашивать, каким приложением
settings-default-apps-after-saving = После сохранения
settings-default-apps-show-folder = Показывать сохранённые файлы в их папке
settings-default-apps-show-folder-detail = Открывает файловый менеджер с выделенными сохранёнными вложениями

## Settings > Compose

settings-compose-send-from = Отправлять новые письма с
settings-compose-send-from-detail = Новые письма начинаются с этого аккаунта; в строке «От» можно выбрать другой. Ответы и пересылки всегда уходят с аккаунта, на который пришло исходное письмо.
settings-compose-send-from-current = Текущего аккаунта
settings-compose-send-on-replies = Отправка ответов
settings-compose-send-on-replies-detail = Что делает кнопка «Отправить» при ответе или пересылке. Другой вариант — в меню рядом с ней.
settings-compose-send-plain = Отправить
settings-compose-send-archive = Отправить и архивировать
settings-compose-signatures = Подписи
settings-compose-signatures-detail = Добавляется под вашим письмом после строки «--». Другую подпись можно выбрать в окне нового письма.
settings-compose-untitled = Без названия
settings-compose-signature-name = Название, например «Работа»
settings-compose-signature-first = Моя подпись
settings-compose-signature-numbered = Подпись { $number }
settings-compose-signature-delete = Удалить
settings-compose-signature-deleted = Подпись удалена
settings-compose-signature-new = Создать
settings-compose-no-signatures = Подписей пока нет.
settings-compose-no-signature = Без подписи
settings-compose-for-new-mail = Для новых писем
settings-compose-for-replies = Для ответов и пересылок
settings-compose-for-replies-detail = В цепочке, где вы уже подписали письмо, ответ начинается с той же подписи.
settings-compose-format = Формат
settings-compose-plain-text = Писать обычным текстом
settings-compose-plain-text-detail = Новые письма начинаются без форматирования; в окне письма это можно переключить
settings-compose-spelling = Орфография
settings-compose-spell-check = Проверять орфографию при вводе
settings-compose-spell-check-detail = Слова с ошибками подчёркиваются, варианты — по правому щелчку
settings-compose-spell-desktop = Язык рабочего стола ({ $language })
settings-compose-templates = Шаблоны
settings-compose-templates-detail = Сохраняйте письма, которые часто пишете, и начинайте с них новое письмо или ответ.
settings-compose-no-templates = Шаблонов пока нет. В письме выберите «Шаблоны», затем «Сохранить как шаблон».
settings-compose-template-new = Создать
settings-compose-template-new-name = Новый шаблон
settings-compose-template-subject = Тема
settings-compose-template-text = Текст шаблона
settings-compose-template-fields = Вместо {"{"}first name{"}"}, {"{"}name{"}"} и {"{"}my name{"}"} подставляются имя получателя и ваше имя.
settings-compose-template-remove-file = Удалить вложение
settings-compose-template-save = Сохранить
settings-compose-template-saved = Шаблон сохранён
settings-compose-template-needs-name = Дайте шаблону название
settings-compose-template-delete = Удалить шаблон
settings-compose-template-deleted = Шаблон удалён
settings-compose-template-delete-failed = Не удалось удалить шаблон: { $error }

## Settings > Shortcuts

settings-shortcuts-set = Набор сочетаний
settings-shortcuts-set-detail = Начните с клавиш знакомого почтового приложения. Cmd здесь — это Ctrl. Ваши изменения сохраняются поверх набора, а «Восстановить по умолчанию» возвращает клавиши набора.
settings-shortcuts-single = Сочетания из одной клавиши
settings-shortcuts-single-detail = Клавиши без Ctrl и Alt, как в веб-почте: e архивирует, j и k перемещают, / ищет. Работают в списке и в открытой цепочке, но не во время ввода текста.
settings-shortcuts-single-use = Использовать сочетания из одной клавиши
settings-shortcuts-single-use-detail = Сочетания с Ctrl работают всегда
settings-shortcuts-how = Нажмите на клавишу, чтобы изменить её, или на +, чтобы добавить, затем нажмите новые клавиши. Esc — отмена.
settings-shortcuts-restore = Восстановить по умолчанию
settings-shortcuts-no-key = Нет клавиши
settings-shortcuts-press = Нажмите клавиши…
settings-shortcuts-then = { $keys }, затем…
settings-shortcuts-moved = { $keys } теперь выполняет «{ $action }» вместо «{ $previous }».
settings-shortcuts-single-off = Сочетания из одной клавиши выключены, поэтому эта клавиша заработает, когда вы их включите.
settings-shortcuts-restored = Всем сочетаниям снова назначены клавиши набора.

## Settings search: the line under a result

settings-general-language-summary = Язык приложения, дат и чисел
settings-general-reading-summary = Сначала новые письма, полные заголовки, полные имена получателей
settings-translation-summary = Перевод писем на других языках через сервер Katna на выбранный вами язык
settings-general-mark-read-summary = Когда открытая цепочка отмечается как прочитанная: сразу, через 1 или 3 секунды или вручную
settings-general-auto-advance-summary = Что открывается после удаления, архивирования или перемещения открытой цепочки: следующая, предыдущая или список
settings-general-reply-button-summary = Кнопка ответа рядом с каждым письмом отвечает всем
settings-general-remote-images-summary = Всегда показывать изображения в каждом письме
settings-general-sending-summary = Отмена отправки: сколько отправленное письмо ждёт, чтобы его можно было отменить
settings-general-offline-summary = За сколько дней недавние письма загружаются целиком, чтобы читать их без подключения
settings-general-notifications-summary = Уведомления о новых письмах и их звук
settings-general-reset-cache-summary = Удалить загруженную почту, изображения отправителей и поисковый индекс и загрузить их снова
settings-general-desktop-summary = Запуск Katna при входе в систему, значок в системном лотке и счётчик непрочитанных на значке в панели задач
settings-accounts-accounts-summary = Добавить или удалить аккаунт либо сменить его изображение
settings-appearance-density-summary = Обычные или компактные строки в списке
settings-appearance-scaling-summary = Сделать всё крупнее или мельче: текст, значки, отступы и разделители
settings-appearance-theme-summary = Системная, светлая или тёмная
settings-appearance-sender-pictures-summary = Логотипы компаний, найденные по домену отправителя
settings-appearance-important-summary = Маркер важности рядом с каждым письмом в списке
settings-appearance-mail-colors-summary = Тёмные цвета для HTML-писем в тёмной теме или цвета отправителя
settings-appearance-attachment-previews-summary = Миниатюра содержимого каждого вложения
settings-shortcuts-set-summary = Начать с клавиш Gmail, Inbox by Gmail, Apple Mail, Outlook или Thunderbird
settings-shortcuts-single-summary = Клавиши без Ctrl и Alt, как в веб-почте
settings-default-apps-pdf-summary = Где открываются вложения PDF
settings-default-apps-pictures-summary = Где открываются фотографии и изображения
settings-default-apps-text-summary = Где открываются обычный текст, журналы и код
settings-default-apps-sheets-summary = Где открываются файлы Excel, OpenDocument и CSV
settings-default-apps-documents-summary = Где открываются документы Word и OpenDocument и презентации
settings-default-apps-after-saving-summary = Показывать сохранённые вложения в их папке
settings-compose-send-from-summary = Аккаунт, с которого уходят новые письма: первый, другой или текущий
settings-compose-send-on-replies-summary = «Отправить» или «Отправить и архивировать» цепочку при ответах и пересылках
settings-compose-signatures-summary = Добавляется под вашим письмом после строки «--»
settings-compose-for-new-mail-summary = Подпись, с которой начинаются новые письма
settings-compose-for-replies-summary = Подпись, с которой начинаются ответы и пересылки
settings-compose-format-summary = Писать новые письма обычным текстом
settings-compose-spelling-summary = Проверка орфографии при вводе и язык словаря
settings-general-search-triggers-summary = Слова для поиска в почте из KRunner или поиска GNOME
settings-compose-templates-summary = Сохраняйте письма, которые часто пишете, и начинайте с них новое письмо или ответ
settings-feedback-crash-reports-summary = Сохранять отчёты о сбоях на этом компьютере, когда Katna Mail или её фоновая служба аварийно завершается
settings-feedback-saved-summary = Просмотр, копирование и удаление отчётов о сбоях, сохранённых на этом компьютере
settings-feedback-help-improve-summary = Отправлять отчёты о сбоях, чтобы помочь исправить ошибки; выключено, пока вы не включите
settings-experimental-blur-summary = Рабочий стол размыто просвечивает сквозь верхнюю панель, а меню — из матового стекла
settings-search-shortcut = Сочетание клавиш
settings-search-tab = Вкладка настроек
settings-search-none = Нет настроек по запросу «{ $query }».
settings-search-results = Настройки по запросу «{ $query }»

## Settings: opening at login

settings-open-at-login-failed = Не удалось изменить запуск при входе в систему: { $error }

## Settings > General > Time

settings-time = Время
settings-clock-language = Как принято в языке
settings-clock-12 = 12-часовой, например 2:05 PM
settings-clock-24 = 24-часовой, например 14:05
settings-time-summary = 12- или 24-часовой формат или как принято в языке

## Settings > General > Default mail app, Settings > Compose > Grammar

settings-general-mail-app = Почтовое приложение по умолчанию
settings-general-mail-app-detail = Ссылки на электронную почту в других приложениях и на сайтах открывают здесь новое письмо.
mail-app-is-default = Katna Mail — ваше почтовое приложение по умолчанию.
mail-app-is-other = Ссылки на электронную почту открываются в другом приложении.
mail-app-make-default = Использовать по умолчанию
mail-app-make-default-failed = Не удалось изменить почтовое приложение по умолчанию.
settings-general-mail-app-summary = Открывать ссылки на электронную почту из других приложений и с сайтов в Katna Mail
settings-compose-grammar = Грамматика
settings-compose-grammar-detail = Проверяется на этом компьютере с помощью Harper. Пока только английский: текст на других языках остаётся без изменений.
settings-compose-grammar-check = Проверять грамматику
settings-compose-grammar-check-detail = Подчёркивать грамматические ошибки при вводе, на английском
settings-compose-suggestions = Подсказки при вводе
settings-compose-suggestions-detail = Обучаются на этом компьютере по отправленным вами письмам и письму, на которое вы отвечаете; ничего не покидает компьютер. Нажмите Tab, чтобы принять подсказку, или продолжайте печатать.
settings-compose-suggestions-on = Подсказывать при вводе
settings-compose-suggestions-on-detail = Показывать серым вероятное продолжение фразы, пока вы печатаете
settings-compose-grammar-summary = Подчёркивать грамматические ошибки при вводе, на английском
settings-compose-suggestions-summary = Показывать серым вероятное продолжение фразы, пока вы печатаете
