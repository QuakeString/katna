# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification (the desktop shows it, even with Katna Mail closed)

notify-new-emails = { $count } 封新邮件
notify-and-more = 另外 { $count } 封
notify-no-subject = （无主题）
notify-unknown-sender = 未知发件人

## Reminders the user asked for (same buttons)

notify-snooze-back = 延后的邮件已返回
notify-no-reply = 尚无回复
notify-no-reply-to = 没有人回复“{ $subject }”。

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } 打开了“{ $subject }”
notify-tracking-clicked = { $who } 点击了“{ $subject }”中的链接

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail 有可用更新
notify-update-ready-body = 版本 { $version } 已下载完成。点击更新即可安装并重启 Katna Mail。
notify-update = 更新

## Reminders of calendar events

notify-event-now = 现在
notify-event-in-minutes = { $count ->
   *[other] { $count } 分钟后
}
notify-event-in-hours = { $count ->
   *[other] { $count } 小时后
}
notify-event-in-days = { $count ->
    [1] 明天
   *[other] { $count } 天后
}
notify-event-all-day = 全天
notify-event-join = 加入
notify-event-snooze = 延后 5 分钟
notify-task-done = 标记为已完成

## The buttons of new-mail notifications and reminders

notify-open = 打开
notify-peek = 预览
notify-reply = 回复
notify-reply-placeholder = 回复{ $name }…
notify-send = 发送
notify-reply-all = 全部回复
notify-mark-read = 标记为已读
notify-mark-all-read = 全部标记为已读
notify-archive = 归档

## After Archive on a notification: a short note in the same place

notify-archived = 已归档
notify-archived-count = { $count ->
   *[other] 已将 { $count } 封邮件移出收件箱
}
notify-undo = 撤消

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = 已回复{ $name }
notify-open-in-katna = 在 Katna 中打开
