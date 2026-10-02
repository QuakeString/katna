# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = { $count } 封新郵件
notify-and-more = 另外 { $count } 封
notify-no-subject = （無主旨）
notify-unknown-sender = 不明寄件者

## Reminders the user asked for (same buttons)

notify-snooze-back = 延後的郵件已返回
notify-no-reply = 尚無回覆
notify-no-reply-to = 沒有人回覆「{ $subject }」。

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } 開啟了「{ $subject }」
notify-tracking-clicked = { $who } 點了「{ $subject }」中的連結

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail 有可用更新
notify-update-ready-body = 版本 { $version } 已下載完成。點擊更新即可安裝並重新啟動 Katna Mail。
notify-update = 更新

## Reminders of calendar events

notify-event-now = 現在
notify-event-in-minutes = { $count ->
   *[other] { $count } 分鐘後
}
notify-event-in-hours = { $count ->
   *[other] { $count } 小時後
}
notify-event-in-days = { $count ->
    [1] 明天
   *[other] { $count } 天後
}
notify-event-all-day = 全天
notify-event-join = 加入
notify-event-snooze = 延後 5 分鐘
notify-task-done = 標示為已完成

## The buttons of new-mail notifications and reminders

notify-open = 開啟
notify-peek = 預覽
notify-reply = 回覆
notify-reply-placeholder = 回覆 { $name }…
notify-send = 傳送
notify-reply-all = 全部回覆
notify-mark-read = 標示為已讀取
notify-mark-all-read = 全部標示為已讀取
notify-archive = 封存

## After Archive on a notification: a short note in the same place

notify-archived = 已封存
notify-archived-count = { $count ->
   *[other] 已將 { $count } 封郵件移出收件匣
}
notify-undo = 復原

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = 已傳送回覆給 { $name }
notify-open-in-katna = 在 Katna 中開啟
