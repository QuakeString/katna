# Katna Mail, Ukrainian (Українська).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count ->
    [one] { $count } новий лист
    [few] { $count } нові листи
    [many] { $count } нових листів
   *[other] { $count } нового листа
}
notify-and-more = і ще { $count }
notify-no-subject = (без теми)
notify-unknown-sender = Невідомий відправник
notify-snooze-back = Повернулося з відкладених
notify-no-reply = Відповіді ще немає
notify-no-reply-to = Ніхто не відповів на «{ $subject }».
notify-tracking-opened = { $who }: лист відкрито { $subject }
notify-tracking-clicked = { $who } переходить за посиланням у листі { $subject }

## Its buttons

notify-update-ready = Katna Mail можна оновити
notify-update-ready-body = Версію { $version } завантажено. Оновлення встановить її й перезапустить Katna Mail.
notify-update = Оновити
notify-open = Відкрити
notify-reply-all = Відповісти всім
notify-mark-read = Позначити як прочитане
notify-mark-all-read = Позначити все як прочитане
notify-archive = Архівувати
