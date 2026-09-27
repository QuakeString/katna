# Katna Mail, Russian (Русский).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } новое письмо
    [few] { $count } новых письма
    [many] { $count } новых писем
   *[other] { $count } нового письма
}
notify-and-more = и ещё { $count }
notify-no-subject = (без темы)
notify-unknown-sender = Неизвестный отправитель
notify-snooze-back = Отложенная почта вернулась
notify-no-reply = Ответа пока нет
notify-no-reply-to = Никто не ответил на «{ $subject }».
notify-tracking-opened = Письмо «{ $subject }» открыто: { $who }
notify-tracking-clicked = Переход по ссылке в «{ $subject }»: { $who }

## Its buttons

notify-open = Открыть
notify-reply-all = Ответить всем
notify-mark-read = Отметить как прочитанное
notify-mark-all-read = Отметить все как прочитанные
notify-archive = Архивировать
