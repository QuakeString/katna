# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = О Katna
about-tagline = Почта и календарь для рабочего стола Linux
about-whats-new = Что нового
about-changelog = Список изменений
about-source = Исходный код
about-coffee = Угостить кофе
about-coming-soon = Скоро
about-follow = Следите за автором
about-love-title = Сделано с любовью к Rust, KDE и Linux
about-love-text = С Rust писать быстрое и безопасное почтовое приложение — одно удовольствие: в Katna нет unsafe-кода. Рабочий стол Plasma от KDE и его набор PIM вдохновили Katna, а Linux и сообщество свободного ПО — та почва, на которой она стоит. Спасибо, и спасибо библиотекам ниже.
about-kde-text = KDE создаёт рабочий стол, на котором Katna чувствует себя как дома. Его делают добровольцы, а финансируют такие люди, как вы. Если вам нравится Plasma или приложения KDE, подумайте о пожертвовании KDE.
about-donate-kde = Поддержать KDE
about-gpui-title = Построено на GPUI из проекта Zed
about-gpui-text = Весь интерфейс Katna Mail построен на GPUI — быстром UI-фреймворке с ускорением на GPU, который Zed Industries создала для редактора Zed. Каждый пиксель, анимацию и окно, которые вы видите, рисует он. Спасибо команде Zed за то, что разрабатывает его открыто. Apache-2.0.
about-gpui-github = GPUI на GitHub
about-personal-title = Личный проект
about-personal-text = Katna Mail не пытается быть новой или революционной. Это почтовое приложение, которое хотел сам автор, а его возможности и внешний вид позаимствованы у Gmail, Mailspring и Thunderbird. Оно стало возможным только благодаря тому, как далеко продвинулись LLM.
about-built-on = ПОСТРОЕНО НА СВОБОДНОМ ПО
about-credit-pimalaya = IMAP, SMTP и вход (io-imap, io-smtp, io-sasl)
about-credit-imap-codec = Чтение и запись IMAP
about-credit-tantivy = Поиск
about-credit-sqlite = Хранилище почты
about-credit-rustls = Защищённые соединения
about-credit-mail-parser = Чтение писем, от Stalwart Labs
about-credit-html5ever = HTML-письма, из проекта Servo
about-credit-zbus = Связь с рабочим столом через D-Bus и порталы
about-credit-oo7 = Пароли в связке ключей рабочего стола
about-credit-hayro = Просмотр и печать PDF
about-credit-calamine = Предпросмотр электронных таблиц
about-credit-resvg = Изображения SVG
about-credit-jiff = Даты и часовые пояса
about-credit-spellbook = Проверка орфографии, из редактора Helix
about-credit-smol = Много дел одновременно
about-all-libraries = Все библиотеки, которые использует Katna ({ $count })
about-library-authors = авторы: { $authors }
about-license = Katna — свободное ПО под лицензией GNU GPL версии 3 или более поздней.
about-close = Закрыть

## What’s new (shown after an update)

whats-new-title = Что нового в Katna Mail
whats-new-updated = Обновлено до версии { $version }
whats-new-version = Версия { $version }
whats-new-more = { $count ->
    [one] И ещё { $count } изменение в полном списке изменений.
    [few] И ещё { $count } изменения в полном списке изменений.
    [many] И ещё { $count } изменений в полном списке изменений.
   *[other] И ещё { $count } изменения в полном списке изменений.
}
whats-new-changelog = Полный список изменений
whats-new-got-it = Понятно

## First run: welcome page

onboarding-welcome-title = Добро пожаловать в Katna Mail
onboarding-welcome-lead = Ваша почта на вашем компьютере: быстрый поиск, чтение без сети и конфиденциальность.
onboarding-fast-title = Быстро, даже без сети
onboarding-fast-text = Katna хранит здесь копию вашей почты, поэтому открытие и поиск происходят мгновенно — с подключением или без.
onboarding-providers-title = Работает с вашей почтой
onboarding-providers-text = Gmail, Outlook, Yahoo, iCloud и любой другой аккаунт IMAP или POP.
onboarding-private-title = Конфиденциально
onboarding-private-text = Почта идёт напрямую от вашего провайдера на этот компьютер. Ни один сервер Katna её не видит.
onboarding-get-started = Начать

## First run: adding an account

onboarding-service-checking = Проверка фоновой службы Katna…
onboarding-service-running = Фоновая служба Katna работает.
onboarding-service-missing = Фоновая служба Katna не запущена
onboarding-service-start = Она получает и отправляет вашу почту. Запустите её из терминала и проверьте снова:
onboarding-check-again = Проверить снова
onboarding-account-title = Добавьте почтовый аккаунт
onboarding-account-lead = Введите адрес электронной почты и пароль, и Katna найдёт настройки сервера. Для Gmail, Yahoo и iCloud нужен пароль приложения, который создаётся в настройках безопасности аккаунта.
onboarding-add-account = Добавить аккаунт
onboarding-back = Назад

## First run: choosing the look

onboarding-look-title = Настройте под себя
onboarding-look-lead = Выберите, как открываются письма и как выглядит Katna. Это можно изменить в любой момент в быстрых настройках.
onboarding-reading-pane = Область просмотра
onboarding-pane-right = Справа от списка
onboarding-pane-none = Без разделения
onboarding-theme = Тема
onboarding-theme-system = Системная
onboarding-theme-light = Светлая
onboarding-theme-dark = Тёмная
onboarding-density = Плотность
onboarding-density-default = Обычная
onboarding-density-compact = Компактная
onboarding-continue = Продолжить

## First run: done

onboarding-ready-title = Всё готово
onboarding-ready-lead = Katna получает вашу почту. Письма появляются по мере загрузки, а новые приходят сами.
onboarding-ready-lead-address = Katna получает почту { $address }. Письма появляются по мере загрузки, а новые приходят сами.
onboarding-ready-tour = Пройти минутный обзор, чтобы узнать, где что находится?
onboarding-skip = Пропустить
onboarding-take-tour = Пройти обзор

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Помогите улучшить Katna
share-lead = При сбое Katna сохраняет отчёт на этом компьютере. Отправка этих отчётов помогает исправить ошибки. Это можно изменить в любой момент в разделе Настройки > Отзывы пользователей.
share-sent = Что отправляется
share-sent-detail = Отчёт о сбое в том виде, в каком его можно посмотреть в настройках: что и где в Katna дало сбой, версия, ваша система Linux и рабочий стол, а также последние строки журнала Katna, где могут упоминаться почтовые папки.
share-never-sent = Что никогда не отправляется
share-never-sent-detail = Ваши письма, контакты, пароли, IP-адрес, имя пользователя и имя компьютера. Адреса электронной почты удаляются из отчёта.
share-where = Куда отправляется
share-where-detail = В систему отслеживания сбоев Katna в Sentry, данные хранятся в ЕС. Никакой идентификатор не связывает отчёты с вами.
share-dont-send = Не отправлять
share-send = Отправлять отчёты о сбоях
share-sending = Отчёты о сбоях будут отправляться. Спасибо.
share-local = Отчёты о сбоях остаются на этом компьютере.

## The tour (cards pointing at each part of the window)

tour-welcome-title = Добро пожаловать в Katna Mail
tour-welcome-text = Минутный обзор покажет, где что находится.
tour-not-now = Не сейчас
tour-start = Пройти обзор
tour-close = Закрыть
tour-skip = Пропустить обзор
tour-back = Назад
tour-done = Готово
tour-next = Далее
tour-step = { $step } из { $total }
tour-compose-title = Напишите письмо
tour-compose-text = «Написать» открывает новое письмо в правом нижнем углу, так что можно читать дальше, пока пишете.
tour-search-title = Ищите по всей почте
tour-search-text = Поиск работает и без сети. Кнопка справа добавляет фильтры: отправитель, получатель, тема, даты и вложения.
tour-menu-title = Показ и скрытие папок
tour-menu-text = Эта кнопка сворачивает список папок. Пока он скрыт, задержите указатель на «Почте» слева, чтобы увидеть папки.
tour-apps-title = Ваши приложения
tour-apps-text = Здесь теперь живёт Почта. Календарь, Контакты, Задачи, Заметки и Ленты присоединятся к ней на этой панели.
tour-tabs-title = Вкладки «Входящих»
tour-tabs-text = Новая почта раскладывается по вкладкам «Несортированные», «Промоакции», «Соцсети», «Оповещения» и «Форумы». Вкладки можно отключить в быстрых настройках.
tour-list-title = Ваши письма
tour-list-text = Щёлкните письмо, чтобы прочитать его. Наведите на него указатель для быстрых действий, щёлкните правой кнопкой для других или отметьте несколько, чтобы действовать сразу над всеми.
tour-settings-title = Быстрые настройки
tour-settings-text = Здесь меняются область просмотра, плотность и тема. Отсюда же можно снова запустить обзор.
tour-account-title = Ваш аккаунт
tour-account-text = Посмотрите, в каком вы аккаунте, и добавьте ещё один.

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Фоновая служба Katna неожиданно остановилась.
    [one] Фоновая служба Katna неожиданно остановилась. Сохранён ещё { $more } отчёт о сбое.
    [few] Фоновая служба Katna неожиданно остановилась. Сохранено ещё { $more } отчёта о сбоях.
    [many] Фоновая служба Katna неожиданно остановилась. Сохранено ещё { $more } отчётов о сбоях.
   *[other] Фоновая служба Katna неожиданно остановилась. Сохранено ещё { $more } отчёта о сбоях.
}
crash-mail = { $more ->
    [0] В прошлый раз Katna Mail неожиданно закрылась.
    [one] В прошлый раз Katna Mail неожиданно закрылась. Сохранён ещё { $more } отчёт о сбое.
    [few] В прошлый раз Katna Mail неожиданно закрылась. Сохранено ещё { $more } отчёта о сбоях.
    [many] В прошлый раз Katna Mail неожиданно закрылась. Сохранено ещё { $more } отчётов о сбоях.
   *[other] В прошлый раз Katna Mail неожиданно закрылась. Сохранено ещё { $more } отчёта о сбоях.
}
crash-view = Открыть отчёт
crash-view-tooltip = Открыть отчёт, сохранённый на этом компьютере
crash-copy = Копировать отчёт
crash-close = Закрыть
