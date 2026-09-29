# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## A new-mail notification

notify-new-emails = { $count } 封新邮件
notify-and-more = 另外 { $count } 封
notify-no-subject = （无主题）
notify-unknown-sender = 未知发件人
notify-snooze-back = 延后的邮件已返回
notify-no-reply = 尚无回复
notify-no-reply-to = 没有人回复“{ $subject }”。
notify-tracking-opened = { $who } 打开了“{ $subject }”
notify-tracking-clicked = { $who } 点击了“{ $subject }”中的链接

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail 有可用更新
notify-update-ready-body = 版本 { $version } 已下载完成。点击更新即可安装并重启 Katna Mail。
notify-update = 更新
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

## The buttons of new-mail notifications and reminders

notify-open = 打开
notify-reply-all = 全部回复
notify-mark-read = 标记为已读
notify-mark-all-read = 全部标记为已读
notify-archive = 归档
