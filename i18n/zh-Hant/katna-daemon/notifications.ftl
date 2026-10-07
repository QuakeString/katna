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
notify-follow-up-sent = 已傳送後續追蹤郵件
notify-follow-up-sent-to = 沒有人回覆「{ $subject }」，所以 Katna 已替你傳送後續追蹤郵件。
notify-follow-up-waiting = 未傳送後續追蹤郵件
notify-follow-up-waiting-to = 預定傳送時這部電腦處於關機狀態。「{ $subject }」已回到你的收件匣。

## Open and click tracking (only for mail sent with "Track opens and clicks")

notify-tracking-opened = { $who } 開啟了「{ $subject }」
notify-tracking-clicked = { $who } 點了「{ $subject }」中的連結

## An update of Katna is downloaded and ready to install

notify-update-ready = Katna Mail 有可用更新
notify-update-ready-body = 版本 { $version } 已下載完成。點擊更新即可安裝並重新啟動 Katna Mail。
notify-update = 更新

## Something needs the user, shown once per problem

notify-signed-out = 請重新登入
notify-signed-out-body = { $provider } 已將 Katna 登出 { $address }，郵件已停止同步。
notify-sign-in = 登入
notify-password-refused = 密碼遭拒
notify-password-refused-body = 郵件伺服器拒絕了 { $address } 的密碼，密碼可能已變更。
notify-new-password = 新密碼
notify-not-sent = 「{ $subject }」未能傳送
notify-not-sent-no-subject = 有一封郵件未能傳送
notify-not-sent-body = 郵件位於寄件匣中，可在那裡查看原因。
notify-open-outbox = 開啟寄件匣

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
notify-snooze-hour = 延後 1 小時
notify-snooze-tomorrow = 明天
notify-copy-code = 複製 { $code }
notify-link-verify = 前往 { $domain } 驗證
notify-link-confirm = 前往 { $domain } 確認
notify-link-activate = 前往 { $domain } 啟用

## After Archive on a notification: a short note in the same place

notify-archived = 已封存
notify-archived-count = { $count ->
   *[other] 已將 { $count } 封郵件移出收件匣
}
notify-undo = 復原

## After Copy on a notification: a short note, with the code under the title

notify-code-copied = 已複製驗證碼
notify-code-not-copied = 無法複製驗證碼

## After a reply typed into a notification: a note in the same place while
## it waits for the undo time

notify-reply-sent = 已傳送回覆給 { $name }
notify-open-in-katna = 在 Katna 中開啟
