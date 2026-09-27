# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Про Katna
about-tagline = Пошта й календар для стільниці Linux
about-whats-new = Що нового
about-changelog = Журнал змін
about-source = Вихідний код
about-coffee = Пригостіть мене кавою
about-coming-soon = Незабаром
about-follow = Стежте за автором
about-love-title = Створено з любов’ю до Rust, KDE і Linux
about-love-text = Завдяки Rust писати швидку й безпечну поштову програму — справжня радість: у Katna немає жодного unsafe-коду. Стільниця Plasma від KDE та її набір PIM надихнули Katna, а Linux і спільнота вільного програмного забезпечення створюють ґрунт, на якому вона стоїть. Дякуємо вам, і дякуємо бібліотекам, наведеним нижче.
about-kde-text = KDE створює стільницю, на якій Katna почувається найбільше як удома. Її роблять волонтери, а фінансують такі люди, як ви. Якщо вам подобаються Plasma чи програми KDE, будь ласка, подумайте про пожертву для KDE.
about-donate-kde = Підтримати KDE
about-gpui-title = Побудовано на GPUI з проєкту Zed
about-gpui-text = Увесь інтерфейс Katna Mail побудовано на GPUI — швидкому фреймворку інтерфейсу з прискоренням на GPU, який Zed Industries створила для редактора Zed. Кожен піксель, кожну анімацію й кожне вікно, які ви бачите, малює він. Дякуємо, команда Zed, що створюєте його відкрито. Apache-2.0.
about-gpui-github = GPUI на GitHub
about-personal-title = Особистий проєкт
about-personal-text = Katna Mail не намагається бути новою чи революційною. Це поштова програма, якої хотів її автор, а її можливості й вигляд запозичено з Gmail, Mailspring і Thunderbird. Вона стала можливою лише завдяки тому, як далеко просунулися LLM.
about-built-on = ПОБУДОВАНО НА ВІЛЬНОМУ ПЗ
about-credit-pimalaya = IMAP, SMTP і вхід (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = Читання й запис IMAP
about-credit-tantivy = Пошук
about-credit-sqlite = Сховище пошти
about-credit-rustls = Захищені з’єднання
about-credit-mail-parser = Читання пошти, від Stalwart Labs
about-credit-html5ever = HTML-листи, з проєкту Servo
about-credit-zbus = Зв’язок зі стільницею через D-Bus і портали
about-credit-oo7 = Паролі у сховищі ключів стільниці
about-credit-hayro = Перегляд і друк PDF
about-credit-calamine = Попередній перегляд електронних таблиць
about-credit-resvg = Зображення SVG
about-credit-jiff = Дати й часові пояси
about-credit-spellbook = Перевірка правопису, з редактора Helix
about-credit-smol = Багато справ одночасно
about-all-libraries = Усі бібліотеки, які використовує Katna ({ $count })
about-library-authors = автори: { $authors }
about-license = Katna — вільне програмне забезпечення за ліцензією GNU GPL версії 3 або новішої.
about-close = Закрити

## What’s new (shown after an update)

whats-new-title = Що нового в Katna Mail
whats-new-updated = Оновлено до версії { $version }
whats-new-version = Версія { $version }
whats-new-more = { $count ->
    [one] І ще { $count } зміна в повному журналі змін.
    [few] І ще { $count } зміни в повному журналі змін.
    [many] І ще { $count } змін у повному журналі змін.
   *[other] І ще { $count } зміни в повному журналі змін.
}
whats-new-changelog = Повний журнал змін
whats-new-got-it = Зрозуміло

## First run: welcome page

onboarding-welcome-title = Вітаємо в Katna Mail
onboarding-welcome-lead = Ваша пошта на вашому власному комп’ютері: швидкий пошук, читання без мережі й приватність.
onboarding-fast-title = Швидко, навіть без мережі
onboarding-fast-text = Katna зберігає копію вашої пошти тут, тож відкриття й пошук миттєві — із з’єднанням чи без нього.
onboarding-providers-title = Працює з вашою поштою
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud і будь-який інший обліковий запис IMAP або POP.
onboarding-private-title = Приватно
onboarding-private-text = Ваша пошта надходить від постачальника прямо на цей комп’ютер. Жоден сервер Katna її не бачить.
onboarding-get-started = Почати

## First run: adding an account

onboarding-service-checking = Перевірка фонової служби Katna…
onboarding-service-running = Фонова служба Katna працює.
onboarding-service-missing = Фонова служба Katna не працює
onboarding-service-start = Вона отримує й надсилає вашу пошту. Запустіть її з термінала, а потім перевірте ще раз:
onboarding-check-again = Перевірити ще раз
onboarding-account-title = Додайте свій поштовий обліковий запис
onboarding-account-lead = Введіть адресу електронної пошти й пароль, і Katna знайде налаштування сервера. Для Gmail, Yahoo та iCloud потрібен пароль застосунку, створений у налаштуваннях безпеки вашого облікового запису.
onboarding-add-account = Додати обліковий запис
onboarding-back = Назад

## First run: choosing the look

onboarding-look-title = Налаштуйте під себе
onboarding-look-lead = Виберіть, як відкриватимуться листи й як виглядатиме Katna. Змінити це можна будь-коли у швидких налаштуваннях.
onboarding-reading-pane = Панель читання
onboarding-pane-right = Праворуч від списку
onboarding-pane-none = Без поділу
onboarding-theme = Тема
onboarding-theme-system = Системна
onboarding-theme-light = Світла
onboarding-theme-dark = Темна
onboarding-density = Щільність
onboarding-density-default = Типова
onboarding-density-compact = Компактна
onboarding-continue = Продовжити

## First run: done

onboarding-ready-title = Усе готово
onboarding-ready-lead = Katna отримує вашу пошту. Листи з’являються в міру надходження, а нова пошта з’являтиметься сама.
onboarding-ready-lead-address = Katna отримує пошту { $address }. Листи з’являються в міру надходження, а нова пошта з’являтиметься сама.
onboarding-ready-tour = Пройти хвилинне ознайомлення, щоб побачити, де що розташовано?
onboarding-skip = Поки що пропустити
onboarding-take-tour = Пройти ознайомлення

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Допоможіть покращити Katna
share-lead = Коли Katna дає збій, вона зберігає звіт на цьому комп’ютері. Надсилання цих звітів допомагає виправити те, що пішло не так. Змінити це можна будь-коли в Налаштування > Відгуки користувачів.
share-sent = Що надсилається
share-sent-detail = Звіт про збій у тому вигляді, у якому його можна переглянути в Налаштуваннях: що дало збій і де в Katna, версія, ваша система Linux і стільниця, а також останні рядки журналу Katna, у яких можуть згадуватися поштові теки.
share-never-sent = Що ніколи не надсилається
share-never-sent-detail = Ваші листи, контакти, паролі, IP-адреса, ім’я користувача чи назва комп’ютера. Адреси електронної пошти вилучаються зі звіту.
share-where = Куди він потрапляє
share-where-detail = До системи відстеження збоїв Katna в Sentry, що зберігає дані в ЄС. Жоден ідентифікатор не пов’язує звіти з вами.
share-dont-send = Не надсилати
share-send = Надсилати звіти про збої
share-sending = Звіти про збої надсилатимуться. Дякуємо.
share-local = Звіти про збої залишаються на цьому комп’ютері.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Вітаємо в Katna Mail
tour-welcome-text = Хвилинне ознайомлення покаже, де що розташовано.
tour-not-now = Не зараз
tour-start = Пройти ознайомлення
tour-close = Закрити
tour-skip = Пропустити ознайомлення
tour-back = Назад
tour-done = Готово
tour-next = Далі
tour-step = { $step } з { $total }
tour-compose-title = Напишіть лист
tour-compose-text = «Написати» відкриває новий лист у нижньому правому куті, тож ви можете читати далі, поки пишете.
tour-search-title = Шукайте в усій пошті
tour-search-text = Пошук працює й без мережі. Кнопка в правому кінці додає фільтри: відправник, одержувач, тема, дати й вкладення.
tour-menu-title = Показати чи сховати теки
tour-menu-text = Ця кнопка згортає список тек. Поки його сховано, наведіть вказівник на «Пошта» ліворуч, щоб побачити теки.
tour-apps-title = Ваші програми
tour-apps-text = Тепер тут живе Пошта. Згодом до неї на цій панелі приєднаються Календар, Контакти, Завдання, Нотатки та Стрічки.
tour-tabs-title = Вкладки «Вхідних»
tour-tabs-text = Нова пошта розподіляється між вкладками «Основні», «Реклама», «Соцмережі», «Оновлення» та «Форуми». Вимкнути вкладки можна у швидких налаштуваннях.
tour-list-title = Ваші листи
tour-list-text = Клацніть лист, щоб прочитати його. Наведіть на нього вказівник для швидких дій, клацніть правою кнопкою для інших дій або позначте кілька, щоб працювати з ними разом.
tour-settings-title = Швидкі налаштування
tour-settings-text = Тут можна змінити панель читання, щільність і тему. Звідти ж можна знову запустити ознайомлення.
tour-account-title = Ваш обліковий запис
tour-account-text = Подивіться, у якому обліковому записі ви перебуваєте, і додайте ще один.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Фонова служба Katna неочікувано зупинилася.
    [one] Фонова служба Katna неочікувано зупинилася. Збережено ще { $more } звіт про збій.
    [few] Фонова служба Katna неочікувано зупинилася. Збережено ще { $more } звіти про збої.
    [many] Фонова служба Katna неочікувано зупинилася. Збережено ще { $more } звітів про збої.
   *[other] Фонова служба Katna неочікувано зупинилася. Збережено ще { $more } звіту про збої.
}
crash-mail = { $more ->
    [0] Минулого разу Katna Mail неочікувано закрилася.
    [one] Минулого разу Katna Mail неочікувано закрилася. Збережено ще { $more } звіт про збій.
    [few] Минулого разу Katna Mail неочікувано закрилася. Збережено ще { $more } звіти про збої.
    [many] Минулого разу Katna Mail неочікувано закрилася. Збережено ще { $more } звітів про збої.
   *[other] Минулого разу Katna Mail неочікувано закрилася. Збережено ще { $more } звіту про збої.
}
crash-view = Переглянути звіт
crash-view-tooltip = Відкрити звіт, збережений на цьому комп’ютері
crash-copy = Копіювати звіт
crash-close = Закрити
