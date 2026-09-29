# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = 新着メール { $count } 件
notify-and-more = ほか { $count } 件
notify-no-subject = （件名なし）
notify-unknown-sender = 不明な送信者
notify-snooze-back = スヌーズから戻りました
notify-no-reply = まだ返信がありません
notify-no-reply-to = 「{ $subject }」に誰も返信していません。
notify-tracking-opened = { $who } が「{ $subject }」を開きました
notify-tracking-clicked = { $who } が「{ $subject }」のリンクをクリックしました

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail を更新できます
notify-update-ready-body = バージョン { $version } のダウンロードが完了しました。更新するとインストールされ、Katna Mail が再起動します。
notify-update = 更新
notify-event-now = 今
notify-event-in-minutes = { $count ->
   *[other] { $count } 分後
}
notify-event-in-hours = { $count ->
   *[other] { $count } 時間後
}
notify-event-in-days = { $count ->
    [1] 明日
   *[other] { $count } 日後
}
notify-event-all-day = 終日
notify-event-join = 参加
notify-event-snooze = 5 分後に再通知

## The buttons of new-mail notifications and reminders

notify-open = 開く
notify-reply-all = 全員に返信
notify-mark-read = 既読にする
notify-mark-all-read = すべて既読にする
notify-archive = アーカイブ
