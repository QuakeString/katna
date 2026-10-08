# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = Поштовий сервер
problems-signed-out = { $provider } вийшов із Katna в { $address }. Синхронізацію пошти зупинено.
problems-password-refused = { $provider } не прийняв пароль для { $address }. Можливо, його змінено.
problems-no-answer = { $provider } не відповідає для { $address }. Katna продовжує спроби.
problems-offline = Ви офлайн. Ваша пошта все ще тут, а листи, які ви надсилаєте, чекатимуть, доки ви повернетеся.
problems-accounts-need-you = { $count ->
    [one] { $count } обліковий запис потребує уваги
    [few] { $count } облікові записи потребують уваги
    [many] { $count } облікових записів потребують уваги
   *[other] { $count } облікового запису потребують уваги
}
problems-show = Показати
problems-later = Пізніше
problems-new-password = Новий пароль
problems-try-again = Спробувати знову

## The New password card

problems-password-title = Новий пароль
problems-password-detail = { $provider } не прийняв збережений пароль для { $address }. Введіть новий; Katna перевірить його, перш ніж зберегти.
problems-password-placeholder = Пароль
problems-password-show = Показати пароль
problems-password-hide = Сховати пароль
problems-password-cancel = Скасувати
problems-password-save = Зберегти
problems-password-checking = Перевірка…
problems-password-refused-again = { $provider } не прийняв і цей пароль. Перевірте його й спробуйте знову.
problems-password-saved = Пароль для { $address } збережено. Отримання пошти…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = Поштовий сервер { $address } не дозволив перемістити { $count ->
    [one] { $count } лист, тож його повернуто на місце.
    [few] { $count } листи, тож їх повернуто на місце.
    [many] { $count } листів, тож їх повернуто на місце.
   *[other] { $count } листа, тож їх повернуто на місце.
}
problems-refused-flags = Поштовий сервер { $address } не дозволив позначити { $count ->
    [one] { $count } лист (прочитане, зірочка…), тож його повернуто як було.
    [few] { $count } листи (прочитане, зірочка…), тож їх повернуто як було.
    [many] { $count } листів (прочитане, зірочка…), тож їх повернуто як було.
   *[other] { $count } листа (прочитане, зірочка…), тож їх повернуто як було.
}
problems-refused-label = Поштовий сервер { $address } не дозволив змінити мітки { $count ->
    [one] { $count } листа, тож його повернуто як було.
    [few] { $count } листів, тож їх повернуто як було.
    [many] { $count } листів, тож їх повернуто як було.
   *[other] { $count } листа, тож їх повернуто як було.
}
problems-refused-delete = Поштовий сервер { $address } не дозволив видалити { $count ->
    [one] { $count } лист, тож його повернуто.
    [few] { $count } листи, тож їх повернуто.
    [many] { $count } листів, тож їх повернуто.
   *[other] { $count } листа, тож їх повернуто.
}
problems-refused-other = Поштовий сервер { $address } не прийняв { $count ->
    [one] { $count } зміну, тож Katna повернула все як було.
    [few] { $count } зміни, тож Katna повернула все як було.
    [many] { $count } змін, тож Katna повернула все як було.
   *[other] { $count } зміни, тож Katna повернула все як було.
}
problems-details = Подробиці

## Katna's background service (katna-daemon) isn't running

service-starting = Запуск фонової служби Katna…
service-failed = Фонова служба Katna не запускається, тому пошта не синхронізується.
service-start-again = Запустити знову
service-started-again = Фонова служба Katna зупинилася й була запущена знову.
service-details-title = Чому служба не запускається
service-details-body = Скопіюйте це й надішліть разом зі звітом. Тут немає ні листів, ні паролів.
service-details-copy = Копіювати
service-details-close = Закрити
