# Katna Mail, Ukrainian (Українська): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Сьогодні
calendar-today-tip = Перейти до сьогоднішнього дня
calendar-view-day = День
calendar-view-week = Тиждень
calendar-view-month = Місяць
calendar-view-year = Рік
calendar-view-schedule = Розклад
calendar-view-days =
    { $count ->
        [one] { $count } день
        [few] { $count } дні
        [many] { $count } днів
       *[other] { $count } дня
    }
calendar-options = Параметри
calendar-density = Щільність
calendar-density-responsive = Адаптується до екрана
calendar-density-comfortable = Зручна
calendar-density-compact = Компактна
calendar-custom-days = Власний вигляд
calendar-second-zone = Другий часовий пояс
calendar-zone-none = Немає
calendar-zone = { $zone } ({ $offset })
calendar-share-free = Поділитися вільним часом
calendar-free-subject = Коли я вільний
calendar-free-intro = Ось час, коли я вільний ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = У найближчі робочі дні я не маю вільного часу.
calendar-previous-day = Попередній день
calendar-next-day = Наступний день
calendar-previous-week = Попередній тиждень
calendar-next-week = Наступний тиждень
calendar-previous-month = Попередній місяць
calendar-next-month = Наступний місяць
calendar-previous-year = Попередній рік
calendar-next-year = Наступний рік
calendar-previous-period = Раніше
calendar-next-period = Пізніше
calendar-title-months = { $first } – { $last }
calendar-loading = Завантаження…
calendar-read-failed = Не вдалося прочитати календар: { $error }
calendar-sets = Набори календарів
calendar-set-add = Зберегти показані календарі як набір
calendar-set-name = Назва набору
calendar-set-remove = Видалити набір
calendar-local = Цей комп’ютер
calendar-account-gone = Видалений обліковий запис
calendar-account-sign-in = Увійдіть знову, щоб показати календарі
calendar-account-signed-in = Знову виконано вхід в { $address }. Отримання календарів…
calendar-account-sign-in-refused = { $provider } не впустив Katna. Спробуйте ще раз і дозвольте доступ до календарів.
calendar-account-refused = Сервер не прийняв пароль. Для Yahoo, iCloud, Zoho та інших потрібен пароль застосунку.
calendar-account-change-password = Змінити пароль
calendar-account-change-password-tooltip = Введіть новий пароль; Katna перевірить його на сервері
calendar-account-not-enabled = Доступ Katna до календаря ще не ввімкнено.
calendar-account-failed = Не вдалося прочитати календарі.
calendar-account-error = Не вдалося прочитати календарі: { $reason }
calendar-account-none = Календарів не знайдено
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = Календарів не знайдено: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } показує календарі лише Katna, що увійшла через { $provider }.
calendar-account-sign-in-with = Увійти через { $provider }
calendar-account-looking = Пошук календарів…
calendar-account-try-again = Повторити спробу
calendar-account-try-again-tooltip = Перевірити календарі цього облікового запису ще раз зараз
calendar-account-fixing = Виправляємо…
calendar-birthdays = Дні народження
calendar-tasks = Завдання
calendar-birthday-of = День народження: { $name }
calendar-empty-title = Календарів ще немає
calendar-empty-text = Katna показує тут календарі ваших облікових записів Google і Microsoft після синхронізації, а також календарі інших серверів із підтримкою CalDAV.
calendar-schedule-empty = На найближчі два місяці нічого не заплановано.
calendar-search = Пошук подій
calendar-search-past = Минулі події
calendar-search-none = Жодна подія не відповідає запиту.
calendar-no-title = (Без назви)
calendar-all-day = Увесь день
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = Ще { $count }
calendar-peek-day = { $weekday }, { $day }
calendar-repeats = Повторюється
calendar-join = Приєднатися
calendar-join-with = Приєднатися через { $service }
calendar-email-guests = Написати гостям
calendar-running-late = Запізнююся
calendar-late-subject = Запізнююся: { $title }
calendar-late-body = Вибачте, я трохи запізнююся на «{ $title }». Скоро буду.
calendar-guests =
    { $count ->
        [one] { $count } гість
        [few] { $count } гості
        [many] { $count } гостей
       *[other] { $count } гостя
    }
calendar-guest-answers = { $yes } так, { $maybe } можливо, { $no } ні, { $waiting } очікується відповідь
calendar-organizer = Організатор
calendar-optional = Необов’язковий
calendar-open-web = Відкрити в браузері
calendar-open-mail = Відкрити лист
calendar-open-contact = Відкрити контакт
calendar-close = Закрити

## Adding, changing and deleting events.

calendar-add-title = Додати назву
calendar-add-location = Додати місце
calendar-add-notes = Додати опис
calendar-add-guests = Додати гостей
calendar-remove-guest = Видалити
calendar-add-meet = Додати відеоконференцію Google Meet
calendar-add-teams = Додати зустріч у Teams
calendar-has-call = Відеозв’язок додано
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Увесь день
calendar-more-options = Інші опції
calendar-save = Зберегти
calendar-saved = Подію збережено
calendar-deleted = Подію видалено
calendar-discard = Не зберігати зміни
calendar-edit = Змінити подію
calendar-delete = Видалити подію
calendar-event-details = Відомості про подію
# Right-click menus on the calendar: on a free time or day, an event and
# a task.
calendar-menu-new-event = Нова подія
calendar-event-window-title = Нова подія
# Shows the day right-clicked on its own, in the Day view.
calendar-menu-open-day = Відкрити день
calendar-menu-duplicate = Дублювати
calendar-menu-color = Колір
# The event takes its calendar's color.
calendar-menu-color-calendar = Колір календаря
# A task's new due day, a week from today.
calendar-menu-in-a-week = Через тиждень
# Event colors, by the names Google Calendar gives them.
calendar-color-tomato = Томат
calendar-color-flamingo = Фламінго
calendar-color-tangerine = Мандарин
calendar-color-banana = Банан
calendar-color-sage = Шавлія
calendar-color-basil = Базилік
calendar-color-peacock = Павич
calendar-color-blueberry = Чорниця
calendar-color-lavender = Лаванда
calendar-color-grape = Виноград
calendar-color-graphite = Графіт
calendar-menu-only-this = Показати лише цей
calendar-menu-rename = Перейменувати
calendar-menu-remove = Вилучити зі списку
calendar-menu-delete = Видалити
calendar-menu-new-calendar = Новий календар
calendar-menu-show-all = Показати всі
calendar-menu-hide-all = Сховати всі
calendar-menu-account-settings = Налаштування облікового запису
calendar-why-main = Основний календар
calendar-why-last = Єдиний тут
calendar-why-owner = Лише для власника
calendar-why-contacts = З контактів
calendar-why-unreached = Немає зв’язку
calendar-name-placeholder = Назва календаря
calendar-toast-added = «{ $name }» додано
calendar-toast-renamed = Календар перейменовано
calendar-toast-recolored = Колір календаря змінено
calendar-toast-deleted = «{ $name }» видалено
calendar-toast-removed = «{ $name }» вилучено з вашого списку
calendar-edit-failed = Календар не змінено: { $reason }
calendar-delete-title = Видалити «{ $name }»?
calendar-delete-confirm = Видалити
calendar-deleting = Видалення…
calendar-delete-heading = Буде видалено:
calendar-delete-events = Календар і всі його події
calendar-delete-shared = Для всіх, з ким до нього надано спільний доступ
calendar-delete-server = Його буде видалено з { $account } у поштовому сервісі, а не лише в Katna.
calendar-delete-local = Його буде видалено з цього комп’ютера.
calendar-remove-title = Вилучити «{ $name }» з вашого списку?
calendar-remove-confirm = Вилучити
calendar-removing = Вилучення…
calendar-remove-heading = Що зміниться:
calendar-remove-events = Ви більше не бачитимете його події ні тут, ні в інших своїх програмах
calendar-remove-server = Календар залишиться у власника, який зможе знову надати вам доступ.
calendar-kind-event = Подія
calendar-kind-task = Завдання
calendar-kind-focus = Час для зосередження
calendar-kind-out-of-office = Поза офісом
calendar-kind-working-location = Місце роботи
calendar-task-added = Завдання додано
calendar-task-added-to = Завдання додано до { $list }
calendar-task-list-local = На цьому комп’ютері
calendar-working-home = Вдома
calendar-busy = Зайнятий
calendar-free = Вільний
calendar-cancel = Скасувати
calendar-ok = OK
calendar-read-only = Ви не можете змінювати події в цьому календарі
calendar-none-editable = Ще немає календаря, до якого можна додавати події
calendar-no-such-time = Цього часу немає у вашому часовому поясі
calendar-end-before-start = Подія закінчується раніше, ніж починається
calendar-repeat-never = Не повторюється
calendar-repeat-daily = Щодня
calendar-repeat-weekly = Щотижня: { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Щомісяця: { $weekday }, перший тиждень
        [2] Щомісяця: { $weekday }, другий тиждень
        [3] Щомісяця: { $weekday }, третій тиждень
        [4] Щомісяця: { $weekday }, четвертий тиждень
       *[other] Щомісяця: { $weekday }, останній тиждень
    }
calendar-repeat-yearly = Щороку: { $day }
calendar-repeat-weekdays = Кожного буднього дня (з понеділка по п’ятницю)
calendar-repeat-custom = Налаштувати
calendar-reminder-none = Без сповіщення
calendar-reminder-at-start = На початку
calendar-reminder-minutes =
    { $count ->
        [one] За { $count } хвилину
        [few] За { $count } хвилини
        [many] За { $count } хвилин
       *[other] За { $count } хвилини
    }
calendar-reminder-hours =
    { $count ->
        [one] За { $count } годину
        [few] За { $count } години
        [many] За { $count } годин
       *[other] За { $count } години
    }
calendar-reminder-days =
    { $count ->
        [one] За { $count } день
        [few] За { $count } дні
        [many] За { $count } днів
       *[other] За { $count } дня
    }
calendar-scope-edit-title = Змінити подію, що повторюється
calendar-scope-delete-title = Видалити подію, що повторюється
calendar-scope-this = Ця подія
calendar-scope-following = Ця й наступні події
calendar-scope-all = Усі події
calendar-scope-respond-title = Відповідь для події, що повторюється
calendar-going = Ви прийдете?
calendar-answer-yes = Так
calendar-answer-no = Ні
calendar-answer-maybe = Можливо
calendar-answered-yes = Ви прийдете
calendar-answered-no = Ви не прийдете
calendar-answered-maybe = Можливо, ви прийдете

## The card at the top of a mail with an invitation.

calendar-invite = Запрошення
calendar-invite-cancelled = Подію скасовано
calendar-invite-reply = { $name }: відповідь на запрошення
calendar-invite-reply-yes = { $name }: запрошення прийнято
calendar-invite-reply-no = { $name }: запрошення відхилено
calendar-invite-reply-maybe = { $name }: можливо, прийде
calendar-invite-organizer = Організатор: { $name }
calendar-invite-open = Відкрити в Календарі
calendar-invite-not-yet = Ще немає у вашому календарі. Відповісти можна буде після синхронізації.
calendar-invite-by-mail = Цієї події немає у вашому календарі: вашу відповідь буде надіслано організатору поштою.
calendar-mail-yes = Прийнято: { $title }
calendar-mail-yes-body = { $name }: запрошення прийнято.
calendar-mail-no = Відхилено: { $title }
calendar-mail-no-body = { $name }: запрошення відхилено.
calendar-mail-maybe = Під питанням: { $title }
calendar-mail-maybe-body = { $name }: запрошення прийнято попередньо.
calendar-invite-your-day = Ваш день
calendar-invite-clashes =
    { $count ->
        [one] Збігається з { $count } подією
        [few] Збігається з { $count } подіями
        [many] Збігається з { $count } подіями
       *[other] Збігається з { $count } події
    }

## The day's agenda beside the mail.

agenda-show = Показати порядок денний
agenda-hide = Сховати порядок денний
agenda-today = Сьогодні, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = На цей день нічого не заплановано.
