# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Поиск файлов

## Left side (and chips on a phone)

files-all = Все файлы
files-pictures = Изображения
files-pdfs = PDF
files-documents = Документы
files-sheets = Таблицы
files-slides = Презентации
files-other = Другое
files-accounts = Аккаунты
files-drives = Диски
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Доступные мне
files-shown = Показывать
files-received = Полученные
files-sent = Отправленные мной

## Over the files

files-count = { $count ->
    [one] { $count } файл · { $size }
    [few] { $count } файла · { $size }
    [many] { $count } файлов · { $size }
   *[other] { $count } файла · { $size }
}
files-anyone = Все
files-from-person = От { $name }
files-time-any = За всё время
files-time-today = Сегодня
files-time-yesterday = Вчера
files-time-this-week = На этой неделе
files-time-last-week = На прошлой неделе
files-time-this-month = В этом месяце
files-time-last-month = В прошлом месяце
files-time-between = { $first } – { $last }
files-time-hint = Нажмите на день или проведите по нескольким дням
files-time-summary = { $count ->
    [one] { $days } · { $count } файл
    [few] { $days } · { $count } файла
    [many] { $days } · { $count } файлов
   *[other] { $days } · { $count } файла
}
files-time-clear = Сбросить
files-time-month-back = Предыдущий месяц
files-time-month-on = Следующий месяц
files-time-wheel = Прокрутите, чтобы сдвинуть даты, сохраняя длину периода
files-sort-newest = Сначала новые
files-sort-oldest = Сначала старые
files-sort-largest = Сначала большие
files-sort-name = По названию
files-grid = Карточки
files-list = Список
files-this-week = Эта неделя
files-undated = Без даты
files-me = Я
files-no-subject = (без темы)
files-loading = Собираем файлы из вашей почты…
files-empty = Здесь появятся файлы из вашей почты.
files-none-match = Подходящих файлов нет.
files-load-failed = Не удалось прочитать файлы: { $error }

## A file's menu and buttons

files-open = Открыть
files-open-with = Открыть с помощью…
files-save = Сохранить…
files-show-mail = Показать письмо
files-mail-window = Открыть письмо в новом окне
files-forward = Переслать файл
files-from-them = Файлы от { $name }
files-copy-name = Копировать имя файла
files-name-copied = Имя файла скопировано
files-downloading = Загрузка письма…
files-download-failed = Не удалось загрузить это письмо.

## A cloud drive in place of the mail files

files-drive-mine = Мой диск
files-drive-mine-onedrive = Мои файлы
files-drive-results = «{ $words }»
files-drive-count = { $folders ->
    [0] { $files ->
        [one] { $files } файл
        [few] { $files } файла
        [many] { $files } файлов
       *[other] { $files } файла
    }
    [one] { $folders } папка · { $files ->
        [one] { $files } файл
        [few] { $files } файла
        [many] { $files } файлов
       *[other] { $files } файла
    }
    [few] { $folders } папки · { $files ->
        [one] { $files } файл
        [few] { $files } файла
        [many] { $files } файлов
       *[other] { $files } файла
    }
    [many] { $folders } папок · { $files ->
        [one] { $files } файл
        [few] { $files } файла
        [many] { $files } файлов
       *[other] { $files } файла
    }
   *[other] { $folders } папки · { $files ->
        [one] { $files } файл
        [few] { $files } файла
        [many] { $files } файлов
       *[other] { $files } файла
    }
}
files-drive-folders = Папки
files-drive-files = Файлы
files-drive-folder = Папка
files-drive-meta = { $what } · Изменено { $date }
files-drive-as-link = { $what } · ссылкой
files-drive-google-doc = Google Документ
files-drive-google-sheet = Google Таблица
files-drive-google-slides = Google Презентация
files-drive-google-drawing = Google Рисунок
files-drive-fetching = Получение…
files-drive-loading = Открытие диска…
files-drive-empty = Эта папка пуста.
files-drive-unreachable = Не удаётся связаться с { $drive }.
files-drive-try-again = Повторить
files-drive-needs-permission = Чтобы показать этот диск, Katna один раз нужно ваше разрешение. Войдите снова и разрешите Katna видеть ваши файлы.
files-drive-allow = Разрешить
files-drive-allow-failed = Вход не завершён, поэтому диск остаётся закрытым.
files-drive-attach = Прикрепить
files-drive-more = Ещё
files-drive-download = Скачать…
files-drive-open-web = Открыть в { $drive }
files-drive-copy-link = Копировать ссылку
files-drive-link-copied = Ссылка скопирована
files-drive-share = Поделиться…
files-drive-rename = Переименовать
files-drive-trash = Переместить в корзину
files-drive-trashed = «{ $name }» в корзине { $drive }
files-drive-renamed = Переименовано в «{ $name }»
files-drive-getting = Получение { $name } из { $drive }…
files-drive-get-failed = Не удалось получить { $name }: { $error }
files-drive-upload = Загрузить
files-drive-upload-files = Загрузить файлы
files-drive-upload-folder = Загрузить папку
files-drive-upload-failed = Не удалось загрузить { $name }: { $error }
files-drive-upload-needs = Для загрузки Katna один раз нужно ваше разрешение: нажмите «Разрешить» в «Настройки» → «Приложения по умолчанию» → «Страница файлов».

## The Share dialog of a drive file or folder

files-share-title = Доступ к «{ $name }»
files-share-add = Добавьте людей по имени или адресу
files-share-not-address = «{ $text }» — не адрес электронной почты
files-share-notify = Пусть { $drive } тоже отправит им письмо
files-share-people = Люди с доступом
files-share-general = Общий доступ
files-share-loading = Проверяем, у кого есть доступ…
files-share-restricted = Ограниченный
files-share-restricted-about = Открыть по ссылке могут только люди с доступом
files-share-anyone = Все, у кого есть ссылка
files-share-anyone-can = { $role ->
    [editor] Все, у кого есть ссылка, могут редактировать
    [commenter] Все, у кого есть ссылка, могут комментировать
   *[viewer] Все, у кого есть ссылка, могут просматривать
}
files-share-anyone-about = { $role ->
    [editor] Все в интернете, у кого есть ссылка, могут редактировать
    [commenter] Все в интернете, у кого есть ссылка, могут комментировать
   *[viewer] Все в интернете, у кого есть ссылка, могут просматривать
}
files-share-role-owner = Владелец
files-share-role-editor = Редактор
files-share-role-commenter = Комментатор
files-share-role-viewer = Читатель
files-share-you = { $name } (вы)
files-share-domain = Все в { $domain }
files-share-inherited = Доступ от папки, в которой лежит файл
files-share-remove = Закрыть доступ
files-share-copy-link = Копировать ссылку
files-share-share = Открыть доступ
files-share-done = Готово
files-share-close = Закрыть
files-share-sharing = Открытие доступа…
files-share-shared = { $count ->
    [one] Доступ открыт { $count } человеку
    [few] Доступ открыт { $count } людям
    [many] Доступ открыт { $count } людям
   *[other] Доступ открыт { $count } человека
}
files-share-refused = { $drive } не удалось открыть доступ для { $addresses }
files-share-failed = Не удалось изменить доступ: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] Загрузка { $count } элемента
    [few] Загрузка { $count } элементов
    [many] Загрузка { $count } элементов
   *[other] Загрузка { $count } элемента
}
files-tray-done = { $count ->
    [one] { $count } загрузка завершена
    [few] { $count } загрузки завершены
    [many] { $count } загрузок завершено
   *[other] { $count } загрузки завершено
}
files-tray-some-failed = Загружено: { $done }, с ошибкой: { $failed }
files-tray-minutes-left = { $minutes ->
    [one] Осталось около { $minutes } минуты
    [few] Осталось около { $minutes } минут
    [many] Осталось около { $minutes } минут
   *[other] Осталось около { $minutes } минуты
}
files-tray-seconds-left = Осталось меньше минуты
files-tray-starting = Запуск…
files-tray-cancel-all = Отменить все
files-tray-cancel = Отмена
files-tray-fold = Скрыть список
files-tray-unfold = Показать список
files-tray-close = Закрыть
files-tray-progress = { $place } · { $sent } из { $size }
files-tray-in = В { $place }
files-tray-cancelled = Отменено
