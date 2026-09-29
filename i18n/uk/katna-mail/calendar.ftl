# Katna Mail, Ukrainian (Українська): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = Сьогодні
calendar-today-tip = Перейти до сьогоднішнього дня
calendar-view-day = День
calendar-view-week = Тиждень
calendar-view-month = Місяць
calendar-view-schedule = Розклад
calendar-previous-day = Попередній день
calendar-next-day = Наступний день
calendar-previous-week = Попередній тиждень
calendar-next-week = Наступний тиждень
calendar-previous-month = Попередній місяць
calendar-next-month = Наступний місяць
calendar-previous-period = Раніше
calendar-next-period = Пізніше
calendar-title-months = { $first } – { $last }
calendar-loading = Завантаження…
calendar-read-failed = Не вдалося прочитати календар: { $error }
calendar-local = Цей комп’ютер
calendar-account-gone = Видалений обліковий запис
calendar-empty-title = Календарів ще немає
calendar-empty-text = Katna показує тут календарі ваших облікових записів Google і Microsoft після синхронізації, а також календарі інших серверів із підтримкою CalDAV.
calendar-schedule-empty = На найближчі два місяці нічого не заплановано.
calendar-no-title = (Без назви)
calendar-all-day = Увесь день
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }, { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = Ще { $count }
calendar-repeats = Повторюється
calendar-join = Приєднатися
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
