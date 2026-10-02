# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = Пошук файлів

## Left side (and chips on a phone)

files-all = Усі файли
files-pictures = Зображення
files-pdfs = PDF
files-documents = Документи
files-sheets = Електронні таблиці
files-slides = Презентації
files-other = Інші
files-accounts = Облікові записи
files-drives = Диски
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = Доступні мені
files-shown = Показано
files-received = Отримані
files-sent = Надіслані мною

## Over the files

files-count = { $count ->
    [one] { $count } файл · { $size }
    [few] { $count } файли · { $size }
    [many] { $count } файлів · { $size }
   *[other] { $count } файлу · { $size }
}
files-anyone = Будь-хто
files-from-person = Від { $name }
files-time-any = Будь-коли
files-time-today = Сьогодні
files-time-yesterday = Учора
files-time-this-week = Цього тижня
files-time-last-week = Минулого тижня
files-time-this-month = Цього місяця
files-time-last-month = Минулого місяця
files-time-between = { $first } – { $last }
files-time-hint = Клацніть день або протягніть по кількох днях
files-time-summary = { $count ->
    [one] { $days } · { $count } файл
    [few] { $days } · { $count } файли
    [many] { $days } · { $count } файлів
   *[other] { $days } · { $count } файлу
}
files-time-clear = Очистити
files-time-month-back = Попередній місяць
files-time-month-on = Наступний місяць
files-time-wheel = Прокрутіть, щоб зсунути ці дати, зберігши тривалість
files-sort-newest = Спочатку нові
files-sort-oldest = Спочатку старі
files-sort-largest = Спочатку найбільші
files-sort-name = За назвою
files-grid = Картки
files-list = Список
files-this-week = Цього тижня
files-undated = Без дати
files-me = Я
files-no-subject = (без теми)
files-loading = Збираємо файли з вашої пошти…
files-empty = Тут з’являться файли з вашої пошти.
files-none-match = Немає відповідних файлів.
files-load-failed = Не вдалося прочитати файли: { $error }

## A file's menu and buttons

files-open = Відкрити
files-open-with = Відкрити за допомогою…
files-save = Зберегти…
files-show-mail = Показати лист
files-mail-window = Відкрити лист у новому вікні
files-forward = Переслати файл
files-from-them = Файли від { $name }
files-copy-name = Копіювати назву файлу
files-name-copied = Назву файлу скопійовано
files-downloading = Завантаження листа…
files-download-failed = Не вдалося завантажити цей лист.

## A cloud drive in place of the mail files

files-drive-mine = Мій диск
files-drive-mine-onedrive = Мої файли
files-drive-results = «{ $words }»
files-drive-count = { $folders ->
    [0] { $files ->
        [one] { $files } файл
        [few] { $files } файли
        [many] { $files } файлів
       *[other] { $files } файлу
    }
    [one] { $folders } папка · { $files ->
        [one] { $files } файл
        [few] { $files } файли
        [many] { $files } файлів
       *[other] { $files } файлу
    }
    [few] { $folders } папки · { $files ->
        [one] { $files } файл
        [few] { $files } файли
        [many] { $files } файлів
       *[other] { $files } файлу
    }
    [many] { $folders } папок · { $files ->
        [one] { $files } файл
        [few] { $files } файли
        [many] { $files } файлів
       *[other] { $files } файлу
    }
   *[other] { $folders } папки · { $files ->
        [one] { $files } файл
        [few] { $files } файли
        [many] { $files } файлів
       *[other] { $files } файлу
    }
}
files-drive-folders = Папки
files-drive-files = Файли
files-drive-folder = Папка
files-drive-meta = { $what } · змінено { $date }
files-drive-as-link = { $what } · як посилання
files-drive-google-doc = Google Документ
files-drive-google-sheet = Google Таблиця
files-drive-google-slides = Google Презентація
files-drive-google-drawing = Google Малюнок
files-drive-fetching = Отримання…
files-drive-loading = Відкриття диска…
files-drive-empty = Ця папка порожня.
files-drive-unreachable = Не вдалося зв’язатися з { $drive }.
files-drive-try-again = Спробувати ще раз
files-drive-needs-permission = Щоб показати цей диск, Katna потрібен ваш дозвіл один раз. Увійдіть знову й дозвольте Katna бачити ваші файли.
files-drive-allow = Дозволити
files-drive-allow-failed = Вхід не завершено, тому диск залишається закритим.
files-drive-attach = Вкласти
files-drive-more = Більше
files-drive-download = Завантажити…
files-drive-open-web = Відкрити в { $drive }
files-drive-copy-link = Копіювати посилання
files-drive-link-copied = Посилання скопійовано
files-drive-share = Поділитися…
files-drive-rename = Перейменувати
files-drive-trash = Перемістити в кошик
files-drive-trashed = «{ $name }» у кошику { $drive }
files-drive-renamed = Перейменовано на «{ $name }»
files-drive-getting = Отримання { $name } з { $drive }…
files-drive-get-failed = Не вдалося отримати { $name }: { $error }
files-drive-upload = Вивантажити
files-drive-upload-files = Вивантажити файли
files-drive-upload-folder = Вивантажити папку
files-drive-upload-failed = Не вдалося вивантажити { $name }: { $error }
files-drive-upload-needs = Щоб вивантажувати, Katna потрібен ваш дозвіл один раз: натисніть «Дозволити» у розділі Налаштування › Типові програми › Сторінка «Файли».

## The Share dialog of a drive file or folder

files-share-title = Поділитися «{ $name }»
files-share-add = Додайте людей за іменем або адресою
files-share-not-address = «{ $text }» — це не адреса електронної пошти
files-share-notify = Хай { $drive } також надішле їм лист
files-share-people = Люди з доступом
files-share-general = Загальний доступ
files-share-loading = Перевіряємо, хто має доступ…
files-share-restricted = Обмежений
files-share-restricted-about = Відкрити за посиланням можуть лише люди з доступом
files-share-anyone = Усі, хто має посилання
files-share-anyone-can = { $role ->
    [editor] Усі, хто має посилання, можуть редагувати
    [commenter] Усі, хто має посилання, можуть коментувати
   *[viewer] Усі, хто має посилання, можуть переглядати
}
files-share-anyone-about = { $role ->
    [editor] Будь-хто в інтернеті, хто має посилання, може редагувати
    [commenter] Будь-хто в інтернеті, хто має посилання, може коментувати
   *[viewer] Будь-хто в інтернеті, хто має посилання, може переглядати
}
files-share-role-owner = Власник
files-share-role-editor = Редактор
files-share-role-commenter = Коментатор
files-share-role-viewer = Читач
files-share-you = { $name } (ви)
files-share-domain = Усі в { $domain }
files-share-inherited = Доступ від папки, у якій він лежить
files-share-remove = Скасувати доступ
files-share-copy-link = Копіювати посилання
files-share-share = Поділитися
files-share-done = Готово
files-share-sharing = Надання доступу…
files-share-shared = { $count ->
    [one] Доступ надано { $count } людині
    [few] Доступ надано { $count } людям
    [many] Доступ надано { $count } людям
   *[other] Доступ надано { $count } людини
}
files-share-refused = { $drive } не зміг надати доступ: { $addresses }
files-share-failed = Не вдалося змінити доступ: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
    [one] Вивантаження { $count } елемента
    [few] Вивантаження { $count } елементів
    [many] Вивантаження { $count } елементів
   *[other] Вивантаження { $count } елемента
}
files-tray-done = { $count ->
    [one] { $count } вивантаження завершено
    [few] { $count } вивантаження завершено
    [many] { $count } вивантажень завершено
   *[other] { $count } вивантаження завершено
}
files-tray-some-failed = Вивантажено: { $done }, не вдалося: { $failed }
files-tray-minutes-left = { $minutes ->
    [one] Залишилося близько { $minutes } хвилини
    [few] Залишилося близько { $minutes } хвилин
    [many] Залишилося близько { $minutes } хвилин
   *[other] Залишилося близько { $minutes } хвилини
}
files-tray-seconds-left = Залишилося менше хвилини
files-tray-starting = Початок…
files-tray-cancel-all = Скасувати все
files-tray-cancel = Скасувати
files-tray-fold = Сховати список
files-tray-unfold = Показати список
files-tray-close = Закрити
files-tray-progress = { $place } · { $sent } із { $size }
files-tray-in = У { $place }
files-tray-cancelled = Скасовано
