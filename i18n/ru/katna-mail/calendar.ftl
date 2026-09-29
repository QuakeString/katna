# Katna Mail, Russian (Русский): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Сегодня
calendar-today-tip = Перейти к сегодняшнему дню
calendar-view-day = День
calendar-view-week = Неделя
calendar-view-month = Месяц
calendar-view-schedule = Расписание
calendar-previous-day = Предыдущий день
calendar-next-day = Следующий день
calendar-previous-week = Предыдущая неделя
calendar-next-week = Следующая неделя
calendar-previous-month = Предыдущий месяц
calendar-next-month = Следующий месяц
calendar-previous-period = Раньше
calendar-next-period = Позже
calendar-title-months = { $first } – { $last }
calendar-loading = Загрузка…
calendar-read-failed = Не удалось прочитать календарь: { $error }
calendar-local = Этот компьютер
calendar-account-gone = Удалённый аккаунт
calendar-empty-title = Календарей пока нет
calendar-empty-text = Katna показывает здесь календари ваших аккаунтов Google и Microsoft после синхронизации, а также календари других серверов с поддержкой CalDAV.
calendar-schedule-empty = На ближайшие два месяца ничего не запланировано.
calendar-no-title = (Без названия)
calendar-all-day = Весь день
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = Ещё { $count }
calendar-repeats = Повторяется
calendar-join = Присоединиться
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

## The day's agenda beside the mail.

agenda-show = Показать повестку дня
agenda-hide = Скрыть повестку дня
agenda-today = Сегодня, { $date }
agenda-day = { $weekday }, { $date }
agenda-empty = На этот день ничего не запланировано.
