# Katna Mail, Russian (Русский): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Сегодня
calendar-today-tip = Перейти к сегодняшнему дню
calendar-view-day = День
calendar-view-week = Неделя
calendar-view-month = Месяц
calendar-view-year = Год
calendar-view-schedule = Расписание
calendar-view-days =
    { $count ->
        [one] { $count } день
        [few] { $count } дня
        [many] { $count } дней
       *[other] { $count } дня
    }
calendar-options = Параметры
calendar-density = Плотность
calendar-density-responsive = Адаптируется к экрану
calendar-density-comfortable = Комфортная
calendar-density-compact = Компактная
calendar-custom-days = Настраиваемый вид
calendar-second-zone = Второй часовой пояс
calendar-zone-none = Нет
calendar-zone = { $zone } ({ $offset })
calendar-share-free = Поделиться свободным временем
calendar-free-subject = Когда я свободен
calendar-free-intro = Вот время, когда я свободен ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = В ближайшие рабочие дни у меня нет свободного времени.
calendar-previous-day = Предыдущий день
calendar-next-day = Следующий день
calendar-previous-week = Предыдущая неделя
calendar-next-week = Следующая неделя
calendar-previous-month = Предыдущий месяц
calendar-next-month = Следующий месяц
calendar-previous-year = Предыдущий год
calendar-next-year = Следующий год
calendar-previous-period = Раньше
calendar-next-period = Позже
calendar-title-months = { $first } – { $last }
calendar-loading = Загрузка…
calendar-read-failed = Не удалось прочитать календарь: { $error }
calendar-sets = Наборы календарей
calendar-set-add = Сохранить показанные календари как набор
calendar-set-name = Название набора
calendar-set-remove = Удалить набор
calendar-local = Этот компьютер
calendar-account-gone = Удалённый аккаунт
calendar-account-sign-in = Войдите снова, чтобы показать календари
calendar-account-signed-in = Вход в { $address } выполнен снова. Загрузка календарей…
calendar-account-sign-in-refused = { $provider } не впустил Katna. Попробуйте снова и разрешите доступ к календарям.
calendar-account-refused = Сервер не принял пароль. Для Yahoo, iCloud, Zoho и других нужен пароль приложения.
calendar-account-change-password = Сменить пароль
calendar-account-change-password-tooltip = Открыть Настройки > Аккаунты
calendar-account-not-enabled = Доступ Katna к календарю ещё не включён.
calendar-account-failed = Не удалось прочитать календари.
calendar-account-error = Не удалось прочитать календари: { $reason }
calendar-account-none = Календари не найдены
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = Календари не найдены: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } показывает календари только Katna, вошедшей через { $provider }.
calendar-account-sign-in-with = Войти через { $provider }
calendar-account-looking = Поиск календарей…
calendar-account-try-again = Повторить попытку
calendar-account-try-again-tooltip = Сейчас снова проверить календари этого аккаунта
calendar-account-fixing = Исправляем…
calendar-birthdays = Дни рождения
calendar-birthday-of = День рождения: { $name }
calendar-empty-title = Календарей пока нет
calendar-empty-text = Katna показывает здесь календари ваших аккаунтов Google и Microsoft после синхронизации, а также календари других серверов с поддержкой CalDAV.
calendar-schedule-empty = На ближайшие два месяца ничего не запланировано.
calendar-search = Поиск мероприятий
calendar-search-past = Прошедшие мероприятия
calendar-search-none = Нет мероприятий, подходящих под запрос.
calendar-no-title = (Без названия)
calendar-all-day = Весь день
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = Ещё { $count }
calendar-peek-day = { $weekday }, { $day }
calendar-repeats = Повторяется
calendar-join = Присоединиться
calendar-join-with = Присоединиться через { $service }
calendar-email-guests = Написать гостям
calendar-running-late = Опаздываю
calendar-late-subject = Опаздываю: { $title }
calendar-late-body = Извините, я немного опаздываю на «{ $title }». Скоро буду.
calendar-guests =
    { $count ->
        [one] { $count } гость
        [few] { $count } гостя
        [many] { $count } гостей
       *[other] { $count } гостя
    }
calendar-guest-answers = { $yes } да, { $maybe } возможно, { $no } нет, { $waiting } ожидается ответ
calendar-organizer = Организатор
calendar-optional = Необязательный
calendar-open-web = Открыть в браузере
calendar-open-mail = Открыть письмо
calendar-open-contact = Открыть контакт
calendar-close = Закрыть

## Adding, changing and deleting events.

calendar-add-title = Добавить название
calendar-add-location = Добавить место
calendar-add-notes = Добавить описание
calendar-add-guests = Добавить гостей
calendar-remove-guest = Удалить
calendar-add-meet = Добавить видеовстречу Google Meet
calendar-add-teams = Добавить встречу в Teams
calendar-has-call = Видеовстреча добавлена
calendar-weekday-day = { $weekday }, { $day }
calendar-all-day-box = Весь день
calendar-more-options = Другие параметры
calendar-save = Сохранить
calendar-saved = Мероприятие сохранено
calendar-deleted = Мероприятие удалено
calendar-discard = Не сохранять изменения
calendar-edit = Изменить мероприятие
calendar-delete = Удалить мероприятие
calendar-event-details = Сведения о мероприятии
# Right-click menus on the calendar: on a free time or day, an event and
# a task.
calendar-menu-new-event = Новое мероприятие
# Shows the day right-clicked on its own, in the Day view.
calendar-menu-open-day = Открыть день
calendar-menu-duplicate = Дублировать
calendar-menu-color = Цвет
# The event takes its calendar's color.
calendar-menu-color-calendar = Цвет календаря
# A task's new due day, a week from today.
calendar-menu-in-a-week = Через неделю
# Event colors, by the names Google Calendar gives them.
calendar-color-tomato = Томат
calendar-color-flamingo = Фламинго
calendar-color-tangerine = Мандарин
calendar-color-banana = Банан
calendar-color-sage = Шалфей
calendar-color-basil = Базилик
calendar-color-peacock = Павлин
calendar-color-blueberry = Черника
calendar-color-lavender = Лаванда
calendar-color-grape = Виноград
calendar-color-graphite = Графит
calendar-menu-only-this = Показать только этот
calendar-menu-rename = Переименовать
calendar-menu-remove = Убрать из списка
calendar-menu-delete = Удалить
calendar-menu-new-calendar = Новый календарь
calendar-menu-show-all = Показать все
calendar-menu-hide-all = Скрыть все
calendar-menu-account-settings = Настройки аккаунта
calendar-why-main = Основной календарь
calendar-why-last = Единственный здесь
calendar-why-owner = Только владелец
calendar-why-contacts = Из контактов
calendar-why-unreached = Нет связи
calendar-name-placeholder = Название календаря
calendar-toast-added = Календарь «{ $name }» добавлен
calendar-toast-renamed = Календарь переименован
calendar-toast-recolored = Цвет календаря изменён
calendar-toast-deleted = Календарь «{ $name }» удалён
calendar-toast-removed = Календарь «{ $name }» убран из списка
calendar-edit-failed = Календарь не изменён: { $reason }
calendar-delete-title = Удалить «{ $name }»?
calendar-delete-confirm = Удалить
calendar-deleting = Удаление…
calendar-delete-heading = Будет удалено:
calendar-delete-events = Календарь и все его мероприятия
calendar-delete-shared = Для всех, с кем он открыт для совместного доступа
calendar-delete-server = Он удаляется из { $account } в почтовом сервисе, а не только в Katna.
calendar-delete-local = Он удаляется с этого компьютера.
calendar-remove-title = Убрать «{ $name }» из списка?
calendar-remove-confirm = Убрать
calendar-removing = Удаление из списка…
calendar-remove-heading = Что изменится:
calendar-remove-events = Вы перестанете видеть его мероприятия здесь и в других приложениях
calendar-remove-server = Календарь остаётся у владельца, и он может снова открыть вам доступ.
calendar-kind-event = Мероприятие
calendar-kind-task = Задача
calendar-kind-focus = Фокусировка
calendar-kind-out-of-office = Нет на месте
calendar-kind-working-location = Место работы
calendar-task-added = Задача добавлена
calendar-task-added-to = Задача добавлена в { $list }
calendar-task-list-local = На этом компьютере
calendar-working-home = Дома
calendar-busy = Занят
calendar-free = Свободен
calendar-cancel = Отмена
calendar-ok = ОК
calendar-read-only = Вы не можете изменять мероприятия в этом календаре
calendar-none-editable = Пока нет календаря, в который можно добавлять мероприятия
calendar-no-such-time = Этого времени нет в вашем часовом поясе
calendar-end-before-start = Мероприятие заканчивается раньше, чем начинается
calendar-repeat-never = Не повторяется
calendar-repeat-daily = Ежедневно
calendar-repeat-weekly = Еженедельно: { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] Ежемесячно: { $weekday }, первая неделя
        [2] Ежемесячно: { $weekday }, вторая неделя
        [3] Ежемесячно: { $weekday }, третья неделя
        [4] Ежемесячно: { $weekday }, четвёртая неделя
       *[other] Ежемесячно: { $weekday }, последняя неделя
    }
calendar-repeat-yearly = Ежегодно: { $day }
calendar-repeat-weekdays = Каждый будний день (с понедельника по пятницу)
calendar-repeat-custom = Настроить
calendar-reminder-none = Без уведомления
calendar-reminder-at-start = В момент начала
calendar-reminder-minutes =
    { $count ->
        [one] За { $count } минуту
        [few] За { $count } минуты
        [many] За { $count } минут
       *[other] За { $count } минуты
    }
calendar-reminder-hours =
    { $count ->
        [one] За { $count } час
        [few] За { $count } часа
        [many] За { $count } часов
       *[other] За { $count } часа
    }
calendar-reminder-days =
    { $count ->
        [one] За { $count } день
        [few] За { $count } дня
        [many] За { $count } дней
       *[other] За { $count } дня
    }
calendar-scope-edit-title = Изменить повторяющееся мероприятие
calendar-scope-delete-title = Удалить повторяющееся мероприятие
calendar-scope-this = Это мероприятие
calendar-scope-following = Это и последующие мероприятия
calendar-scope-all = Все мероприятия
calendar-scope-respond-title = Ответ для повторяющегося мероприятия
calendar-going = Вы придёте?
calendar-answer-yes = Да
calendar-answer-no = Нет
calendar-answer-maybe = Возможно
calendar-answered-yes = Вы придёте
calendar-answered-no = Вы не придёте
calendar-answered-maybe = Возможно, вы придёте

## The card at the top of a mail with an invitation.

calendar-invite = Приглашение
calendar-invite-cancelled = Мероприятие отменено
calendar-invite-reply = { $name }: ответ на приглашение
calendar-invite-reply-yes = { $name }: приглашение принято
calendar-invite-reply-no = { $name }: приглашение отклонено
calendar-invite-reply-maybe = { $name }: возможно, придёт
calendar-invite-organizer = Организатор: { $name }
calendar-invite-open = Открыть в Календаре
calendar-invite-not-yet = Пока нет в вашем календаре. Ответить можно будет после синхронизации.
calendar-invite-by-mail = Этого события нет в вашем календаре: ваш ответ будет отправлен организатору по почте.
calendar-mail-yes = Принято: { $title }
calendar-mail-yes-body = { $name }: приглашение принято.
calendar-mail-no = Отклонено: { $title }
calendar-mail-no-body = { $name }: приглашение отклонено.
calendar-mail-maybe = Под вопросом: { $title }
calendar-mail-maybe-body = { $name }: приглашение принято предварительно.
calendar-invite-your-day = Ваш день
calendar-invite-clashes =
    { $count ->
        [one] Пересекается с { $count } событием
        [few] Пересекается с { $count } событиями
        [many] Пересекается с { $count } событиями
       *[other] Пересекается с { $count } события
    }

## The day's agenda beside the mail.

agenda-show = Показать повестку дня
agenda-hide = Скрыть повестку дня
agenda-today = Сегодня, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = На этот день ничего не запланировано.
