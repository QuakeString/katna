# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings page: its tabs

settings-tab-general = Загальні
settings-tab-inbox = Вхідні
settings-tab-accounts = Облікові записи
settings-tab-subscriptions = Підписки
settings-tab-appearance = Вигляд
settings-tab-shortcuts = Комбінації клавіш
settings-tab-default-apps = Типові програми
settings-tab-folders-rules = Папки й правила
settings-tab-compose = Написання листів
settings-tab-mcp-server = Сервер MCP
settings-tab-feedback = Відгуки користувачів
settings-tab-experimental = Експериментальні

## Settings page: tabs still to come

settings-tab-subscriptions-coming = Переглядайте розсилки й списки розсилки, які отримуєте, і відписуйтеся одним клацанням.
settings-tab-folders-rules-coming = Створюйте, перейменовуйте, переміщуйте й приховуйте папки та мітки, вибирайте, які з них синхронізувати. Правила самі сортують, позначають мітками, пересилають або видаляють нові листи — за відправником, темою чи словами.
settings-tab-mcp-server-coming = Дозвольте помічникам зі ШІ на цьому комп’ютері шукати, читати й готувати чернетки ваших листів — з вашої згоди.

## Settings > General

settings-general-conversations = Ланцюжки листів
settings-general-conversations-group = Групувати відповіді на той самий лист
settings-general-conversations-group-detail = Один рядок на ланцюжок у списку
settings-general-reading = Читання
settings-general-newest-first = Спочатку найновіший лист
settings-general-newest-first-detail = Ланцюжок починається з останньої відповіді
settings-general-full-headers = Показувати повні заголовки
settings-general-full-headers-detail = Від, кому, копія, дата й тема відкриті в кожному листі
settings-general-full-names = Повні імена одержувачів
settings-general-full-names-detail = «мені, Ada Lovelace», а не «мені, Ada»
settings-general-mark-read = Позначати як прочитане
settings-general-mark-read-now = Одразу після відкриття
settings-general-mark-read-1s = Через 1 секунду після відкриття
settings-general-mark-read-3s = Через 3 секунди після відкриття
settings-general-mark-read-never = Лише коли я позначу сам
settings-general-reply-button = Кнопка «Відповісти»
settings-general-reply-all = Відповідати всім
settings-general-reply-all-detail = Кнопка відповіді біля кожного листа відповідає всім, а не лише відправникові
settings-general-remote-images = Зображення з інтернету
settings-general-remote-images-detail = Завантаження зображень листа повідомляє відправникові, що ви його відкрили, коли і приблизно де. Якщо вимкнено, кожен лист спершу запитує, і ви завжди можете показати зображення відправника.
settings-general-remote-images-always = Завжди показувати зображення
settings-general-remote-images-always-detail = У кожному листі, а не лише від надійних відправників
settings-general-sending = Надсилання
settings-general-sending-detail = Скільки надісланий лист чекає, щоб його можна було скасувати.
settings-general-offline = Пошта офлайн
settings-general-offline-detail = Нещодавні листи завантажуються повністю, щоб читати їх без з’єднання. Старіші завантажуються, коли ви їх відкриваєте.
settings-general-offline-days = { $count ->
    [one] { $count } день
    [few] { $count } дні
    [many] { $count } днів
   *[other] { $count } дня
}
settings-general-offline-years = { $count ->
    [one] { $count } рік
    [few] { $count } роки
    [many] { $count } років
   *[other] { $count } року
}
settings-general-offline-all = Уся пошта
settings-general-offline-note = Якщо вибрати менше днів, уже завантажені листи залишаться. На сервері нічого не змінюється.
settings-general-notifications = Сповіщення
settings-general-notifications-detail = Про нові листи у «Вхідних», навіть коли Katna Mail закрито.
settings-general-new-mail = Сповіщати про нові листи
settings-general-new-mail-detail = З кнопками «Відповісти всім», «Позначити як прочитане» й «Архівувати»
settings-general-new-mail-sound = Відтворювати звук
settings-general-new-mail-sound-detail = Звук нової пошти стільниці
settings-general-desktop = Стільниця
settings-general-open-at-login = Відкривати Katna Mail під час входу
settings-general-open-at-login-detail = Пошта однаково синхронізується під час входу, поки працює служба
settings-general-tray = Показувати Katna в системному лотку
settings-general-tray-detail = Із лічильником непрочитаних і меню
settings-general-unread-badge = Лічильник непрочитаних на значку панелі завдань
settings-general-unread-badge-detail = Скільки листів у «Вхідних» не прочитано

## Settings > Inbox

settings-inbox-tabs = Вкладки «Вхідних»
settings-inbox-tabs-detail = Сортувати вхідні за вкладками, як це робить сайт вашого поштового сервісу.
settings-inbox-tabs-show = Показувати вкладки «Вхідних»
settings-inbox-tabs-show-detail = Якщо вимкнено, для кожного облікового запису один список
settings-inbox-no-accounts = Додайте обліковий запис, щоб вибрати його вкладки.
settings-inbox-tabs-automatic = Автоматично: { $tabs } ({ $provider })
settings-inbox-tabs-off = Без вкладок
settings-inbox-tabs-gmail = Основні, Реклама, Соцмережі, Оновлення, Форуми
settings-inbox-tabs-focused = Пріоритетні та Інші
settings-inbox-tabs-zoho = Вхідні, Розсилки та Сповіщення
settings-inbox-tabs-shown = Показані вкладки. Листи з вимкненої вкладки залишаються у вкладці «{ $tab }».

## Settings > Appearance

settings-appearance-reading-pane = Панель читання
settings-appearance-reading-pane-detail = Де показується відкритий ланцюжок.
settings-appearance-pane-right = Праворуч від списку
settings-appearance-pane-none = Без поділу
settings-appearance-density = Щільність
settings-appearance-density-default = Типова
settings-appearance-density-compact = Компактна
settings-appearance-scaling = Масштаб
settings-appearance-scaling-detail = Робить усе в Katna Mail більшим або меншим, понад власний масштаб стільниці: текст, значки, відступи й роздільники. Листи, які ви надсилаєте, зберігають свій розмір шрифту. За дуже малого масштабу по значках важко влучити.
settings-appearance-theme = Тема
settings-appearance-theme-system = Системна
settings-appearance-theme-light = Світла
settings-appearance-theme-dark = Темна
settings-appearance-desktop-colors = Кольори стільниці
settings-appearance-desktop-colors-use = Використовувати кольори стільниці
settings-appearance-desktop-colors-use-detail = Колірна схема й колір акценту стільниці
settings-appearance-app-names = Назви програм
settings-appearance-app-names-show = Показувати назви програм
settings-appearance-app-names-show-detail = Підписи під значками програм ліворуч
settings-appearance-sender-pictures = Зображення відправників
settings-appearance-sender-pictures-show = Показувати логотипи компаній
settings-appearance-sender-pictures-show-detail = Шукаються за доменом відправника, ніколи за листом, і зберігаються тиждень
settings-appearance-important = Позначки важливості
settings-appearance-important-show = Показувати позначки важливості
settings-appearance-important-show-detail = Біля кожного листа в списку
settings-appearance-message-width = Ширина листа
settings-appearance-message-width-limit = Обмежити ширину листів
settings-appearance-message-width-limit-detail = Так у широкому вікні легше читати довгі рядки
settings-appearance-mail-colors = Кольори листів
settings-appearance-mail-colors-detail = Більшість листів розраховано на білу сторінку. У темній темі їхні кольори замінюються темними, які добре читаються; якщо вимкнено, лист зберігає кольори відправника на світлій сторінці.
settings-appearance-dark-mail = Темні кольори й для листів
settings-appearance-dark-mail-detail = Лише коли тема темна
settings-appearance-attachment-previews = Попередній перегляд вкладень
settings-appearance-attachment-previews-show = Показувати попередній перегляд вкладень
settings-appearance-attachment-previews-show-detail = Мініатюра вмісту кожного файлу на його картці

## Settings > Default apps

settings-default-apps-intro = Де відкриваються вкладення, коли ви їх клацаєте. Із переглядача файл завжди можна відкрити й в іншій програмі. Типові програми стільниці задаються в її власних налаштуваннях.
settings-default-apps-pdf = Файли PDF
settings-default-apps-pdf-detail = Сторінки з масштабуванням.
settings-default-apps-pictures = Зображення
settings-default-apps-pictures-detail = Фотографії (з правильною орієнтацією), PNG, GIF, WebP, BMP, TIFF і SVG.
settings-default-apps-text = Текстові файли
settings-default-apps-text-detail = Звичайний текст, журнали, код та інший текст.
settings-default-apps-sheets = Електронні таблиці
settings-default-apps-sheets-detail = Excel (xlsx, xls), OpenDocument (ods) і CSV.
settings-default-apps-documents = Документи
settings-default-apps-documents-detail = Word (docx, doc), текст OpenDocument (odt) і презентації (pptx, ppt, odp).
settings-default-apps-katna = Переглядач Katna Mail
settings-default-apps-system = Типова програма стільниці
settings-default-apps-ask = Щоразу запитувати, якою програмою
settings-default-apps-after-saving = Після збереження
settings-default-apps-show-folder = Показувати збережені файли в їхній папці
settings-default-apps-show-folder-detail = Відкриває файловий менеджер із позначеними збереженими вкладеннями

## Settings > Compose

settings-compose-send-from = Надсилати нові листи з
settings-compose-send-from-detail = Відповіді й пересилання завжди надсилаються з облікового запису, у якому ви перебуваєте.
settings-compose-send-from-current = Поточного облікового запису
settings-compose-send-on-replies = Надсилання відповідей
settings-compose-send-on-replies-detail = Що робить «Надіслати» у відповіді чи пересиланні. Інший варіант — у меню біля кнопки «Надіслати».
settings-compose-send-plain = Надіслати
settings-compose-send-archive = Надіслати й архівувати
settings-compose-signatures = Підписи
settings-compose-signatures-detail = Додається під вашим листом після рядка «--». Інший підпис можна вибрати у вікні створення листа.
settings-compose-untitled = Без назви
settings-compose-signature-name = Назва, наприклад «Робота»
settings-compose-signature-first = Мій підпис
settings-compose-signature-numbered = Підпис { $number }
settings-compose-signature-delete = Видалити
settings-compose-signature-deleted = Підпис видалено
settings-compose-signature-new = Створити
settings-compose-no-signatures = Підписів ще немає.
settings-compose-no-signature = Без підпису
settings-compose-for-new-mail = Для нових листів
settings-compose-for-replies = Для відповідей і пересилань
settings-compose-for-replies-detail = У ланцюжку, де ви вже підписали лист, відповідь натомість починається з того самого підпису.
settings-compose-format = Формат
settings-compose-plain-text = Писати звичайним текстом
settings-compose-plain-text-detail = Нові листи починаються без форматування; у вікні листа це можна змінити
settings-compose-spelling = Правопис
settings-compose-spell-check = Перевіряти правопис під час введення
settings-compose-spell-check-detail = Слова з помилками підкреслюються, варіанти — за клацанням правою кнопкою
settings-compose-spell-desktop = Мова стільниці ({ $language })
settings-compose-templates = Шаблони
settings-compose-templates-detail = Зберігайте листи, які часто пишете, і починайте з них новий лист або відповідь.

## Settings > Shortcuts

settings-shortcuts-set = Набір комбінацій
settings-shortcuts-set-detail = Почніть із клавіш знайомої поштової програми. Cmd тут — це Ctrl. Ваші зміни зберігаються поверх набору, а «Відновити типові» повертає клавіші набору.
settings-shortcuts-single = Комбінації з однієї клавіші
settings-shortcuts-single-detail = Клавіші без Ctrl чи Alt, як у вебпошті: e архівує, j і k переміщують, / шукає. Працюють у списку й відкритому ланцюжку, але не під час введення тексту.
settings-shortcuts-single-use = Використовувати комбінації з однієї клавіші
settings-shortcuts-single-use-detail = Комбінації з Ctrl працюють завжди
settings-shortcuts-how = Клацніть клавішу, щоб змінити її, або +, щоб додати, потім натисніть нові клавіші. Esc скасовує.
settings-shortcuts-restore = Відновити типові
settings-shortcuts-no-key = Немає клавіші
settings-shortcuts-press = Натисніть клавіші…
settings-shortcuts-then = { $keys }, потім…
settings-shortcuts-moved = { $keys } тепер виконує «{ $action }» замість «{ $previous }».
settings-shortcuts-single-off = Комбінації з однієї клавіші вимкнено, тож ця клавіша запрацює, коли ви їх увімкнете.
settings-shortcuts-restored = Усім комбінаціям знову призначено клавіші набору.

## Settings search: the line under a result

settings-general-language-summary = Мова програми, дат і чисел
settings-general-reading-summary = Спочатку найновіший лист, повні заголовки, повні імена одержувачів
settings-general-mark-read-summary = Коли відкритий ланцюжок позначається як прочитаний: одразу, через 1 чи 3 секунди або вручну
settings-general-reply-button-summary = Кнопка відповіді біля кожного листа відповідає всім
settings-general-remote-images-summary = Завжди показувати зображення в кожному листі
settings-general-sending-summary = Скасування надсилання: скільки надісланий лист чекає, щоб його можна було скасувати
settings-general-offline-summary = За скільки днів нещодавні листи завантажуються повністю, щоб читати їх без з’єднання
settings-general-notifications-summary = Сповіщення про нові листи та їхній звук
settings-general-desktop-summary = Відкриття Katna Mail під час входу, значок у системному лотку й лічильник непрочитаних на значку панелі завдань
settings-accounts-accounts-summary = Додати чи вилучити обліковий запис або змінити його зображення
settings-appearance-density-summary = Типові чи компактні рядки в списку
settings-appearance-scaling-summary = Зробити все більшим або меншим: текст, значки, відступи й роздільники
settings-appearance-theme-summary = Системна, світла чи темна
settings-appearance-sender-pictures-summary = Логотипи компаній, знайдені за доменом відправника
settings-appearance-important-summary = Позначка важливості біля кожного листа в списку
settings-appearance-mail-colors-summary = Темні кольори для HTML-листів у темній темі або кольори відправника
settings-appearance-attachment-previews-summary = Мініатюра вмісту кожного вкладення
settings-shortcuts-set-summary = Почати з клавіш Gmail, Inbox by Gmail, Apple Mail, Outlook або Thunderbird
settings-shortcuts-single-summary = Клавіші без Ctrl чи Alt, як у вебпошті
settings-default-apps-pdf-summary = Де відкриваються вкладення PDF
settings-default-apps-pictures-summary = Де відкриваються фотографії та зображення
settings-default-apps-text-summary = Де відкриваються звичайний текст, журнали й код
settings-default-apps-sheets-summary = Де відкриваються файли Excel, OpenDocument і CSV
settings-default-apps-documents-summary = Де відкриваються документи Word, текст OpenDocument і презентації
settings-default-apps-after-saving-summary = Показувати збережені вкладення в їхній папці
settings-compose-send-from-summary = Обліковий запис, з якого надсилаються нові листи: поточний або завжди той самий
settings-compose-send-on-replies-summary = «Надіслати» або «Надіслати й архівувати» ланцюжок у відповідях і пересиланнях
settings-compose-signatures-summary = Додається під вашим листом після рядка «--»
settings-compose-for-new-mail-summary = Підпис, з якого починаються нові листи
settings-compose-for-replies-summary = Підпис, з якого починаються відповіді й пересилання
settings-compose-format-summary = Писати нові листи звичайним текстом
settings-compose-spelling-summary = Перевірка правопису під час введення та мова словника
settings-compose-templates-summary = Незабаром: зберігайте листи, які часто пишете, і починайте з них новий лист або відповідь
settings-feedback-crash-reports-summary = Зберігати звіти про збої на цьому комп’ютері, коли Katna Mail або її фонова служба аварійно завершується
settings-feedback-saved-summary = Переглянути, скопіювати або видалити звіти про збої, збережені на цьому комп’ютері
settings-feedback-help-improve-summary = Надсилати звіти про збої, щоб допомогти виправити помилки; вимкнено, доки ви не ввімкнете
settings-experimental-blur-summary = Стільниця розмито просвічує крізь верхню панель, а меню — з матового скла
settings-search-shortcut = Комбінація клавіш
settings-search-tab = Вкладка налаштувань
settings-search-none = Немає налаштувань за запитом «{ $query }».
settings-search-results = Налаштування за запитом «{ $query }»

## Settings: opening at login

settings-open-at-login-failed = Не вдалося змінити відкриття під час входу: { $error }

## Settings > General > Time

settings-time = Час
settings-clock-language = Як прийнято в мові
settings-clock-12 = 12-годинний, наприклад 2:05 пп
settings-clock-24 = 24-годинний, наприклад 14:05
settings-time-summary = 12- або 24-годинний формат чи як прийнято в мові

## Settings > General > Default mail app, Settings > Compose > Grammar

settings-general-mail-app = Типова поштова програма
settings-general-mail-app-detail = Посилання на електронну пошту в інших програмах і на сайтах відкривають тут новий лист.
mail-app-is-default = Katna Mail — ваша типова поштова програма.
mail-app-is-other = Посилання на електронну пошту відкриваються в іншій програмі.
mail-app-make-default = Зробити типовою
mail-app-make-default-failed = Не вдалося змінити типову поштову програму.
settings-general-mail-app-summary = Відкривати посилання на електронну пошту з інших програм і сайтів у Katna Mail
settings-compose-grammar = Граматика
settings-compose-grammar-detail = Перевіряється на цьому комп’ютері за допомогою Harper. Поки що лише англійська: текст іншими мовами лишається без змін.
settings-compose-grammar-check = Перевіряти граматику
settings-compose-grammar-check-detail = Підкреслювати граматичні помилки під час введення, англійською
settings-compose-grammar-summary = Підкреслювати граматичні помилки під час введення, англійською
