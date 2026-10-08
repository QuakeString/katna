# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = 新着メール { $count } 件
notify-and-more = ほか { $count } 件
notify-no-subject = （件名なし）
notify-unknown-sender = 不明な送信者

## Reminders the user asked for (same buttons)

notify-snooze-back = スヌーズから戻りました
notify-no-reply = まだ返信がありません
notify-no-reply-to = 「{ $subject }」に誰も返信していません。
notify-follow-up-sent = フォローアップを送信しました
notify-follow-up-sent-to = 「{ $subject }」に誰も返信していなかったため、Katna がフォローアップを送信しました。
notify-follow-up-waiting = フォローアップは送信されませんでした
notify-follow-up-waiting-to = このパソコンの電源が切れている間に送信予定時刻になりました。「{ $subject }」は受信トレイに戻っています。

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } が「{ $subject }」を開きました
notify-tracking-clicked = { $who } が「{ $subject }」のリンクをクリックしました

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail を更新できます
notify-update-ready-body = バージョン { $version } のダウンロードが完了しました。更新するとインストールされ、Katna Mail が再起動します。
notify-update = 更新

## Something needs the user, shown once per problem

notify-signed-out = もう一度サインインしてください
notify-signed-out-body = { $provider } により { $address } から Katna がサインアウトされました。メールの同期は停止しています。
notify-sign-in = サインイン
notify-password-refused = パスワードが拒否されました
notify-password-refused-body = メールサーバーが { $address } のパスワードを拒否しました。パスワードが変更された可能性があります。
notify-new-password = 新しいパスワード
notify-not-sent = 「{ $subject }」は送信されませんでした
notify-not-sent-no-subject = メールが送信されませんでした
notify-not-sent-body = 送信トレイにあります。理由もそこで確認できます。
notify-open-outbox = 送信トレイを開く

## Reminders of calendar events

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
notify-task-done = 完了にする

## The buttons of new-mail notifications and reminders

notify-open = 開く
notify-peek = プレビュー
notify-reply = 返信
notify-reply-placeholder = { $name } さんに返信…
notify-send = 送信
notify-reply-quote-header = { $date }、{ $from } が書きました:
notify-reply-quote-header-no-date = { $from } が書きました:
notify-reply-all = 全員に返信
notify-mark-read = 既読にする
notify-mark-all-read = すべて既読にする
notify-archive = アーカイブ
notify-snooze-hour = 1 時間スヌーズ
notify-snooze-tomorrow = 明日
notify-copy-code = { $code } をコピー
notify-link-verify = { $domain } で確認
notify-link-confirm = { $domain } で承認
notify-link-activate = { $domain } で有効化

## After Archive on a notification: a short note in the same place

notify-archived = アーカイブしました
notify-archived-count = { $count ->
   *[other] { $count } 件のメールを受信トレイから移動しました
}
notify-undo = 元に戻す

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = コードをコピーしました
notify-code-not-copied = コードをコピーできませんでした

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = { $name } さんに返信しました
notify-open-in-katna = Katna で開く
