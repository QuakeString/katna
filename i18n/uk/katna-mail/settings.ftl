# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings page: its tabs

settings-tab-general = Загальні
settings-tab-notifications = Сповіщення
settings-tab-inbox = Вхідні
settings-tab-accounts = Облікові записи
settings-tab-katna-account = Обліковий запис Katna
settings-tab-subscriptions = Підписка
settings-tab-appearance = Вигляд
settings-tab-shortcuts = Комбінації клавіш
settings-tab-default-apps = Типові програми
settings-tab-folders-rules = Папки й правила
settings-tab-compose = Написання листів
settings-tab-mcp-server = Сервер MCP
settings-tab-feedback = Відгуки користувачів
settings-tab-experimental = Експериментальні

## Settings page: tabs still to come

settings-tab-folders-rules-coming = Створюйте, перейменовуйте, переміщуйте й приховуйте папки та мітки, вибирайте, які з них синхронізувати. Правила самі сортують, позначають мітками, пересилають або видаляють нові листи — за відправником, темою чи словами.

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
settings-translation = Переклад
settings-translation-detail = Листи іншою мовою можна читати вашою.
settings-translation-offer = Пропонувати переклад
settings-translation-offer-detail = Текст листа надсилається на сервер Katna для перекладу, лише коли ви просите або завжди перекладаєте його мову. Вкладення не надсилаються ніколи.
settings-translation-reading = Перекладати на
settings-translation-always = Завжди перекладати
settings-translation-never = Ніколи не пропонувати для
settings-translation-none = Поки нічого. Виберіть на панелі перекладу над листом.
settings-general-mark-read = Позначати як прочитане
settings-general-mark-read-now = Одразу після відкриття
settings-general-mark-read-1s = Через 1 секунду після відкриття
settings-general-mark-read-3s = Через 3 секунди після відкриття
settings-general-mark-read-never = Лише коли я позначу сам
settings-general-auto-advance = Автоперехід
settings-general-auto-advance-detail = Після того як ви видалите, заархівуєте чи перемістите відкритий ланцюжок
settings-general-auto-advance-next = Відкрити наступний ланцюжок
settings-general-auto-advance-previous = Відкрити попередній ланцюжок
settings-general-auto-advance-list = Повернутися до списку
settings-general-confirm-delete = Видалення
settings-general-confirm-delete-ask = Запитувати перед видаленням кількох ланцюжків
settings-general-confirm-delete-ask-detail = Видалення назавжди запитує завжди
settings-general-reply-button = Кнопка «Відповісти»
settings-general-reply-all = Відповідати всім
settings-general-reply-all-detail = Кнопка відповіді біля кожного листа відповідає всім, а не лише відправникові
settings-general-remote-images = Зображення з інтернету
settings-general-remote-images-detail = Завантаження зображень листа повідомляє відправникові, що ви його відкрили, коли і приблизно де. Якщо вимкнено, кожен лист спершу запитує, і ви завжди можете показати зображення відправника.
settings-general-remote-images-always = Завжди показувати зображення
settings-general-remote-images-always-detail = У кожному листі, а не лише від надійних відправників
settings-general-sending = Надсилання
settings-general-sending-detail = Скільки надісланий лист чекає, щоб його можна було скасувати.
settings-general-video-calls = Відеодзвінки
settings-general-video-calls-detail = Функція «Розпочати відеодзвінок» використовує Google Meet для облікових записів Gmail. Інші облікові записи отримують кімнату Jitsi Meet на цьому сервері; приєднатися може кожен, хто має посилання.
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
settings-general-notifications-detail = Про нові листи в папках зі сповіщеннями, навіть коли Katna Mail закрито.
settings-general-new-mail = Сповіщати про нові листи
settings-general-new-mail-detail = З кнопками «Відповісти всім», «Позначити як прочитане» й «Архівувати»
settings-notifications-sounds = Звуки
settings-notifications-sounds-detail = Зі звукової теми стільниці. Папки, ланцюжки й відправники без сповіщень залишаються беззвучними, як і все в режимі «Не турбувати».
sounds-new-mail = Нові листи
sounds-new-mail-detail = У папках зі сповіщеннями
sounds-reminders = Нагадування
sounds-reminders-detail = Події календаря й завдання
sounds-mail-back = Лист повернувся у «Вхідні»
sounds-mail-back-detail = Відкладені листи й листи, на які ніхто не відповів
sounds-sent = Лист надіслано
sounds-sent-detail = Коли лист вирушив
sounds-not-sent = Лист не надіслано
sounds-not-sent-detail = Коли надсилання не вдалося
sounds-play = Відтворити
sound-katna-chime = Дзвіночок Katna
sound-new-email = Новий лист
sound-new-message = Нове повідомлення
sound-sent = Надіслано
sound-alarm = Будильник
sound-bell = Дзвін
sound-complete = Завершено
sound-information = Інформація
sound-warning = Попередження
sound-error = Помилка
sound-reminder = Нагадування
sound-default = Сповіщення
settings-general-updates = Оновлення
settings-general-updates-detail = Встановіть нову версію в розділі «Про Katna» або зі сповіщення про її готовність.
settings-general-auto-download = Завантажувати оновлення автоматично
settings-general-auto-download-detail = Ніколи через лімітне з’єднання. Нічого не встановлюється, поки ви не натиснете «Оновити».
settings-general-reset-cache = Скинути кеш
settings-general-reset-cache-detail = Коли пошта виглядає неправильною чи застарілою або щоб звільнити місце на диску. На ваших поштових серверах нічого не змінюється.
settings-general-desktop = Стільниця
settings-general-start-at-login = Запускати Katna під час входу
settings-general-start-at-login-detail = Синхронізує пошту й показує сповіщення про нові листи та значок у лотку, не відкриваючи вікна
settings-general-login-window = Відкривати також вікно Katna Mail
settings-general-login-window-detail = Вікно теж відкривається під час входу
settings-general-tray = Показувати Katna в системному лотку
settings-general-tray-detail = Із лічильником непрочитаних і меню
settings-general-tray-color = Кольоровий значок у лотку
settings-general-tray-color-detail = Якщо вимкнено, він одноколірний, як інші значки панелі. Лічильник непрочитаних залишається червоним.
settings-general-unread-badge = Лічильник непрочитаних на значку панелі завдань
settings-general-unread-badge-detail = Непрочитані листи в папках, що враховуються в лічильнику
settings-notifications-count = Лічильник на панелі завдань
settings-notifications-count-detail = А також лічильник на значку в лотку.
settings-notifications-notify = Сповіщати
settings-notifications-counts = Лічильник
settings-notifications-muted = Без сповіщень
settings-notifications-muted-detail = Папки, облікові записи, ланцюжки й відправники, нові листи з яких не сповіщають і не враховуються в лічильнику.
settings-notifications-nothing-muted = Сповіщення ніде не вимкнено. Вимкніть їх для папки через її контекстне меню або дзвіночок над списком.
settings-notifications-until = До { $when }
settings-notifications-until-unmuted = Доки ви не ввімкнете знову
settings-notifications-a-conversation = Ланцюжок
settings-general-search-triggers = Пошук зі стільниці
settings-general-search-triggers-detail = Введіть у KRunner або в пошуку GNOME одне з цих слів і пробіл, а потім те, що треба знайти, щоб шукати в пошті так само, як у тутешньому полі пошуку. Розділяйте слова комами.
settings-general-search-triggers-none = Слів немає; працює лише «mail:»

## Settings > Inbox

settings-inbox-tabs = Вкладки «Вхідних»
settings-inbox-tabs-detail = Сортувати вхідні за вкладками, як це робить сайт вашого поштового сервісу.
settings-inbox-tabs-show = Показувати вкладки «Вхідних»
settings-inbox-tabs-show-detail = Якщо вимкнено, для кожного облікового запису один список
settings-inbox-no-accounts = Додайте обліковий запис, щоб вибрати його вкладки.
settings-inbox-unified = Єдині «Вхідні»
settings-inbox-unified-detail = Вкладки, спільні для всіх облікових записів. Кожен лист показується у вкладці свого типу; листи з вкладки, яку обліковий запис вимикає, залишаються в його першій вкладці.
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
settings-appearance-theme = Режим
settings-appearance-theme-system = Системна
settings-appearance-theme-light = Світла
settings-appearance-theme-dark = Темна
settings-appearance-theme-forced = Колірна схема, вибрана нижче, має лише світлий або лише темний варіант, тож вона й визначає режим.
settings-appearance-colors = Кольори
settings-appearance-colors-detail = Кожна схема має світлий і темний варіанти, тож «Режим» працює з будь-якою. Попередній перегляд показує обидва варіанти з кольором акценту, вибраним нижче.
settings-appearance-colors-built-in = Вбудовані
settings-appearance-colors-from-system = З вашої системи
settings-appearance-colors-system = Системні
settings-appearance-colors-system-detail = Як на стільниці
settings-appearance-colors-light-only = Лише світла
settings-appearance-colors-dark-only = Лише темна
settings-appearance-colors-yours = Ваші
scheme-customize-card = Налаштувати…
scheme-customize-card-detail = На основі вибраної
scheme-import-card = Імпортувати…
scheme-import-card-detail = Файл Katna або KDE
settings-appearance-accent = Акцент
settings-appearance-accent-detail = Колір кнопки «Написати», активної програми на панелі програм, лічильників і виділення
settings-appearance-accent-scheme = Зі схеми
settings-appearance-accent-system = Системний
settings-appearance-accent-more = Інші кольори
scheme-katna = Katna
scheme-clear = Прозора
scheme-graphite = Графіт
scheme-nord = Nord
scheme-solarized = Solarized
scheme-dracula = Dracula
scheme-gruvbox = Gruvbox
scheme-catppuccin = Catppuccin
scheme-tokyo-night = Tokyo Night
scheme-one = One
scheme-rose-pine = Rosé Pine
scheme-everforest = Everforest
scheme-kanagawa = Kanagawa
scheme-ayu = Ayu
scheme-copy-name = { $name } (копія)
scheme-customize = Налаштувати
scheme-edit = Редагувати
scheme-duplicate = Дублювати
scheme-export = Експортувати
scheme-delete = Видалити
scheme-deleted = «{ $name }» видалено
scheme-exported = «{ $name }» збережено
scheme-import = Імпортувати колірну схему
scheme-import-failed = Katna не може прочитати цю колірну схему: { $error }
scheme-editor-new = Нова колірна схема
scheme-editor-edit = Редагувати колірну схему
scheme-editor-name = Назва
scheme-editor-light = Світлий варіант
scheme-editor-dark = Темний варіант
scheme-editor-make-dark = Створити темний зі світлого
scheme-editor-add-dark = Додати темний варіант
scheme-editor-add-light = Додати світлий варіант
scheme-editor-remove-side = Вилучити цей варіант
scheme-editor-readable = Легко читати
scheme-editor-hard-to-read = Важко читати: { $colors }
scheme-picker-dropper = Взяти з екрана
scheme-picker-in-scheme = У цій схемі
scheme-picker-recent = Нещодавні
scheme-picker-system = Системна палітра…
scheme-seed-page = Сторінка
scheme-seed-cards = Картки
scheme-seed-text = Текст
scheme-seed-faint = Блідий текст
scheme-seed-accent = Акцент
scheme-seed-bar-text = Текст верхньої панелі
scheme-seed-on-accent = Текст на акценті
scheme-seed-error = Помилка
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
settings-files-page = Сторінка «Файли»
settings-files-page-detail = Які вкладення показує сторінка «Файли»
settings-files-leave-out-small = Пропускати маленькі зображення
settings-files-leave-out-small-detail = Логотипи й значки в підписах, які є в багатьох листах
settings-files-smaller-than = Менші за
settings-files-kb = КБ
settings-files-narrower-than = або вужчі чи нижчі за
settings-files-px = пкс
settings-files-more-tip = Більше
settings-files-less-tip = Менше
settings-files-sizes-note = Розміри в пікселях визначаються, коли лист завантажено; до того його зображення оцінюються лише за розміром файлу.
settings-files-drives = Диски у «Файлах»
settings-files-drives-detail = Власний диск кожного облікового запису на сторінці «Файли»
settings-files-drive-needs = { $address } · Katna потрібен дозвіл один раз

## Settings > Compose

settings-compose-send-from = Надсилати нові листи з
settings-compose-send-from-detail = Нові листи починаються з цього облікового запису; у рядку Від можна вибрати інший. Відповіді й пересилання завжди надсилаються з облікового запису, на який надійшов початковий лист.
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
settings-compose-no-templates = Шаблонів ще немає. У листі виберіть Шаблони, а потім Зберегти як шаблон.
settings-compose-template-new = Створити
settings-compose-template-new-name = Новий шаблон
settings-compose-template-subject = Тема
settings-compose-template-text = Текст шаблону
settings-compose-template-fields = {"{"}first name{"}"}, {"{"}name{"}"} і {"{"}my name{"}"} заповнюються іменем одержувача та вашим іменем.
settings-compose-template-remove-file = Вилучити вкладення
settings-compose-template-save = Зберегти
settings-compose-template-saved = Шаблон збережено
settings-compose-template-needs-name = Дайте шаблону назву
settings-compose-template-delete = Видалити шаблон
settings-compose-template-deleted = Шаблон видалено
settings-compose-template-delete-failed = Не вдалося видалити шаблон: { $error }

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
settings-translation-summary = Переклад листів іншими мовами на вибрану вами мову через сервер Katna
settings-general-mark-read-summary = Коли відкритий ланцюжок позначається як прочитаний: одразу, через 1 чи 3 секунди або вручну
settings-general-auto-advance-summary = Що відкривається після того, як ви видалите, заархівуєте чи перемістите відкритий ланцюжок: наступний, попередній або список
settings-general-confirm-delete-summary = Запитувати перед переміщенням кількох ланцюжків у кошик
settings-general-reply-button-summary = Кнопка відповіді біля кожного листа відповідає всім
settings-general-remote-images-summary = Завжди показувати зображення в кожному листі
settings-general-sending-summary = Скасування надсилання: скільки надісланий лист чекає, щоб його можна було скасувати
settings-general-video-calls-summary = Сервер Jitsi Meet для нових відеодзвінків з облікових записів без Google Meet
settings-general-offline-summary = За скільки днів нещодавні листи завантажуються повністю, щоб читати їх без з’єднання
settings-general-notifications-summary = Сповіщення про нові листи
settings-notifications-sounds-summary = Звук для нових листів, нагадувань, надісланих і ненадісланих листів
settings-general-updates-summary = Завантажувати нові версії Katna самостійно
settings-general-reset-cache-summary = Видалити завантажену пошту, зображення відправників і пошуковий індекс і завантажити їх знову
settings-general-desktop-summary = Запуск Katna під час входу й значок у системному лотку
settings-notifications-count-summary = Лічильник непрочитаних на значку панелі завдань
settings-notifications-muted-summary = Увімкнути сповіщення для папок, облікових записів, ланцюжків і відправників
settings-accounts-accounts-summary = Додати чи вилучити обліковий запис або змінити його зображення
settings-appearance-density-summary = Типові чи компактні рядки в списку
settings-appearance-scaling-summary = Зробити все більшим або меншим: текст, значки, відступи й роздільники
settings-appearance-theme-summary = Системна, світла чи темна
settings-appearance-colors-summary = Колірні схеми: стільниці, Katna або вбудовані, як-от Nord чи Solarized
settings-appearance-accent-summary = Колір вибраної папки, кнопки «Написати» й лічильників
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
settings-files-page-summary = Не показувати на сторінці «Файли» маленькі зображення, як-от логотипи з підписів, і вибрати, які диски вона показує
settings-compose-send-from-summary = Обліковий запис, з якого надсилаються нові листи: перший, інший або поточний
settings-compose-send-on-replies-summary = «Надіслати» або «Надіслати й архівувати» ланцюжок у відповідях і пересиланнях
settings-compose-signatures-summary = Додається під вашим листом після рядка «--»
settings-compose-for-new-mail-summary = Підпис, з якого починаються нові листи
settings-compose-for-replies-summary = Підпис, з якого починаються відповіді й пересилання
settings-compose-format-summary = Писати нові листи звичайним текстом
settings-compose-spelling-summary = Перевірка правопису під час введення та мова словника
settings-general-search-triggers-summary = Слова для пошуку в пошті з KRunner або пошуку GNOME
settings-compose-templates-summary = Зберігайте листи, які часто пишете, і починайте з них новий лист або відповідь
settings-feedback-crash-reports-summary = Зберігати звіти про збої на цьому комп’ютері, коли Katna Mail або її фонова служба аварійно завершується
settings-feedback-saved-summary = Переглянути, скопіювати або видалити звіти про збої, збережені на цьому комп’ютері
settings-feedback-help-improve-summary = Надсилати звіти про збої, щоб допомогти виправити помилки; вимкнено, доки ви не ввімкнете
settings-experimental-blur-summary = Розмивати тло вікна, робити матовими меню й діалоги або й те, й інше
settings-search-shortcut = Комбінація клавіш
settings-search-tab = Вкладка налаштувань
settings-search-none = Немає налаштувань за запитом «{ $query }».
settings-search-results = Налаштування за запитом «{ $query }»

## Settings: opening at login

settings-open-at-login-failed = Не вдалося змінити запуск під час входу: { $error }

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
settings-compose-suggestions = Підказки під час письма
settings-compose-suggestions-detail = Фрази вивчаються на цьому комп’ютері з листів, які ви надіслали, і листів, на які ви відповідаєте. Натисніть Tab, щоб прийняти підказку, або пишіть далі.
settings-compose-suggestions-on = Підказувати під час письма
settings-compose-suggestions-on-detail = Показувати ймовірне продовження фрази сірим під час введення
settings-ai-autocomplete = Довші підказки за допомогою ШІ
settings-ai-autocomplete-detail = Завершувати речення, коли ви робите паузу, за допомогою ШІ, вибраного нижче. Ніколи для зашифрованої пошти.
settings-ai-answered = Використовувати лист, на який ви відповідаєте
settings-ai-answered-detail = Точніші здогадки щодо імен і дат; надсилається більше тексту
settings-ai = Допомога з письмом за допомогою ШІ
settings-ai-detail = Виділіть текст у листі й натисніть іскорку (або Ctrl+J), щоб перефразувати його. Надсилається лише вибраний текст, і нічого не зберігається.
settings-ai-katna = Katna AI
settings-ai-own = Власний ключ
settings-ai-off = Вимкнено
settings-ai-katna-detail = Безкоштовно 30 днів від першого використання, потім $5 на місяць. Використовує ваш обліковий запис Katna.
settings-ai-own-detail = Ваш ключ надсилається лише вашому сервісу. Він зберігається в системному сховищі ключів, а не в налаштуваннях Katna.
settings-ai-service = Ваш сервіс ШІ
settings-ai-service-detail = У пункті «Інший» працює будь-який сервіс з API, сумісним з OpenAI, наприклад Ollama чи LM Studio на цьому комп’ютері.
settings-ai-other = Інший
settings-ai-address = Адреса
settings-ai-model = Модель
settings-ai-models = Моделі цього сервісу
settings-ai-key = Ключ API
settings-ai-key-paste = Вставте свій ключ
settings-ai-key-save = Зберегти ключ
settings-ai-key-saved = Ключ збережено.
settings-ai-key-remove = Вилучити
settings-ai-key-none = Ключ ще не збережено.
settings-ai-key-saved-toast = Ключ збережено
settings-ai-key-removed = Ключ вилучено
settings-ai-encrypted-title = Зашифрована пошта
settings-ai-encrypted = Пропонувати перефразування в зашифрованих листах
settings-ai-encrypted-detail = Щоразу запитує, перш ніж надіслати текст зашифрованого листа
settings-compose-grammar-summary = Підкреслювати граматичні помилки під час введення, англійською
settings-compose-suggestions-summary = Показувати ймовірне продовження фрази сірим під час введення
settings-ai-summary = Перефразування виділеного тексту й завершення речень за допомогою Katna AI або власного ключа
