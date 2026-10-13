# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > User feedback (crash reports)

feedback-intro-sending = Новые отчёты о сбоях отправляются, чтобы помочь исправить ошибки. Больше ничего не покидает этот компьютер.
feedback-intro-local = Katna ничего никуда не отправляет. Отчёты о сбоях остаются на этом компьютере — их можно посмотреть или приложить к сообщению об ошибке.
feedback-crash-reports = Отчёты о сбоях
feedback-crash-reports-detail = Создаются, когда Katna Mail или её фоновая служба аварийно завершается.
feedback-save = Сохранять отчёты о сбоях на этом компьютере
feedback-save-detail = Домашняя папка, имена пользователя и компьютера и адреса электронной почты не включаются
feedback-saved = Сохранённые отчёты о сбоях
feedback-saved-detail = { $count ->
    [one] Хранится { $count } последний отчёт.
    [few] Хранятся { $count } последних отчёта.
    [many] Хранятся { $count } последних отчётов.
   *[other] Хранятся { $count } последнего отчёта.
}
feedback-help-improve = Помочь улучшить Katna
feedback-help-improve-detail = Выключено, пока вы не включите, и здесь это можно выключить в любой момент.
feedback-send = Отправлять отчёты о сбоях
feedback-send-detail = Сохранённый отчёт — ровно такой, каким вы видите его здесь, — отправляется в систему отслеживания сбоев Katna (Sentry, в ЕС). Без IP-адреса, писем и адресов электронной почты
feedback-none-saved = Сохранённых отчётов о сбоях нет.
feedback-delete-all = Удалить все
feedback-app-daemon = Фоновая служба
feedback-report-sent = { $date } · Отправлен
feedback-view = Открыть
feedback-view-tooltip = Открыть отчёт
feedback-copy-tooltip = Скопировать, чтобы вставить в сообщение об ошибке
feedback-copied = Отчёт о сбое скопирован.
feedback-deleted-all = Отчёты о сбоях удалены.
feedback-read-failed = Не удалось прочитать отчёт о сбое: { $error }
feedback-delete-failed = Не удалось удалить отчёт о сбое: { $error }
feedback-delete-all-failed = Не удалось удалить отчёты о сбоях: { $error }
feedback-usage = Отправлять анонимную статистику использования
feedback-usage-detail = Раз в неделю: какими функциями вы пользовались, да или нет. Никаких чисел, адресов, имён и поисковых запросов
feedback-intro-sending-usage = Отправляются отчёты о сбоях и еженедельная статистика использования. Больше ничего не покидает этот компьютер.
feedback-intro-usage-only = Отправляется еженедельная статистика использования. Отчёты о сбоях остаются на этом компьютере.
feedback-counted = Что учитывается
feedback-counted-detail = По каждому пункту — да или нет за неделю.
feedback-counted-also = А также: версия Katna, семейство Linux, рабочий стол, масштаб экрана и число аккаунтов (1, 2–3, 4+)
feedback-see-report = Показать отчёт за эту неделю
feedback-hide-report = Скрыть отчёт за эту неделю
feedback-report-goes = Будет отправлен после окончания недели, { $date }, если статистика использования ещё включена.
feedback-install-id = ID установки { $id }
feedback-install-id-tooltip = Случайный, чтобы один компьютер не учитывался дважды за неделю. Меняется каждые 90 дней и никогда не отправляется с отчётами о сбоях или отзывами
feedback-install-id-reset = Сбросить
feedback-install-id-new = Создан новый ID установки.
feedback-report-copied = Отчёт скопирован.
feedback-send-feedback = Отзыв
feedback-send-feedback-detail = Проблема, идея — что угодно.
feedback-send-feedback-button = Отправить отзыв…
usage-feature-search-options = Параметры поиска
usage-feature-pins = Закреплённые письма
usage-feature-labels = Ярлыки
usage-feature-scheduled-send = Запланированная отправка
usage-feature-snooze = Откладывание и напоминания
usage-feature-encrypted = Зашифрованные письма
usage-feature-viewers = Встроенные просмотрщики
usage-feature-calendar = Календарь
usage-feature-contacts = Контакты
usage-feature-tasks-notes = Задачи и заметки
usage-feature-phone-layout = Вид для узкого экрана
usage-feature-own-frame = Собственная рамка окна Katna
send-feedback-title = Отправить отзыв
send-feedback-about = Тема
send-feedback-problem = Проблема
send-feedback-idea = Идея
send-feedback-other = Другое
send-feedback-message = Ваше сообщение
send-feedback-message-placeholder = Что случилось или чего бы вам хотелось?
send-feedback-reply = Адрес для ответа (необязательно)
send-feedback-reply-placeholder = you@example.org
send-feedback-system = Добавить версию Katna и сведения о системе
send-feedback-what-is-sent = Что отправляется
send-feedback-show = Показать
send-feedback-hide = Скрыть
send-feedback-where = Отправляется в ящик отзывов Katna в Sentry (ЕС). Без IP-адреса, аккаунтов, писем и ID установки.
send-feedback-cancel = Отмена
send-feedback-send = Отправить
send-feedback-sending = Отправка…
send-feedback-sent = Отзыв отправлен. Спасибо
send-feedback-failed = Не удалось отправить отзыв: { $error }
