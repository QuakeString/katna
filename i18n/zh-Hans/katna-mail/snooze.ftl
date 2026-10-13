# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = 延后至…
snooze-later-today = 今天晚些时候
snooze-tomorrow = 明天
snooze-this-weekend = 本周末
snooze-next-week = 下周
snooze-pick = 选择日期和时间
snooze-back = 返回时间列表
snooze-type-placeholder = 输入时间
snooze-type-hint = 例如“周二下午3点”“明天”或“2 小时后”
snooze-type-hint-unclear = Katna 无法将其识别为时间
snooze-type-unclear = Katna 无法识别“{ $text }”这个时间

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = 延后
remind-tab = 提醒我
snooze-says = 在此之前隐藏
remind-says = 保留在原位并通知你
remind-before-due = 到期之前
remind-note = 备注（可选）
remind-note-placeholder = 留空则使用主题
toast-remind-set = 已设置提醒：{ $date }
remind-chat-line = 提醒 { $date } · { $title }
remind-done = 完成
toast-remind-done = 提醒已完成
snooze-chat-line = 已延后至 { $date }
snooze-chat-change = 更改

## The date and time picker

snooze-cancel = 取消
snooze-save = 保存
snooze-in-the-past = 请选择晚于现在的时间。

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = 无人回复时跟进…
follow-up-title = 无人回复时跟进
follow-up-off = 关闭
follow-up-days = { $days ->
   *[other] { $days } 天
}
follow-up-weeks = { $weeks ->
   *[other] { $weeks } 周
}
follow-up-pick = 选择…
follow-up-pick-title = 在此时间前无人回复则跟进
follow-up-remind = 提醒我
follow-up-remind-note = 该会话会回到收件箱顶部
follow-up-send = 替我发送跟进邮件
follow-up-send-note = 发给相同的收件人，位于同一会话中
follow-up-send-encrypted = 不适用于加密邮件
follow-up-text-placeholder = 要写的内容
follow-up-text-named = { $name }，你好，想确认一下你是否看到了我下面的邮件。
follow-up-text = 你好，想确认一下你是否看到了我下面的邮件。
follow-up-template = 使用模板
follow-up-signature = 会附上你的签名
follow-up-again = 如果仍无人回复，再次跟进的间隔
follow-up-note = 只要会话中有任何人回复就会停止。自动回复不计在内。
follow-up-note-send = 只要会话中有任何人回复就会停止。在工作日 { $start } 至 { $end } 之间发出，最多延迟一天。
follow-up-cancel = 取消
follow-up-done = 完成
follow-up-chip-send = { $time }后跟进
follow-up-chip-remind = { $time }后提醒
follow-up-chip-send-on = 跟进 { $date }
follow-up-chip-remind-on = 提醒 { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = 尚无回复
follow-up-card-title-waiting = 你的跟进邮件正在等待
follow-up-card-send = Katna 将于 { $date } 发送你的跟进邮件。有人回复即停止。
follow-up-card-send-twice = Katna 将于 { $date } 发送你的跟进邮件，之后还会再发送一次。有人回复即停止。
follow-up-card-remind = 如果无人回复，此会话将于 { $date } 回到你的收件箱。
follow-up-card-waiting = 到期时你的电脑处于关机状态，因此没有延迟发送。你可以立即发送、选择新时间或停止跟进。
follow-up-card-edit = 编辑
follow-up-card-edit-title = 跟进时间
follow-up-card-send-now = 立即发送
follow-up-card-stop = 停止
follow-up-chat-send = 跟进 · 如果无人回复，将于 { $date } 发送
follow-up-chat-step = 第 { $step } 次跟进，共 { $steps } 次 · 如果无人回复，将于 { $date } 发送
follow-up-chat-waiting = 跟进待发送 · 到期时你的电脑处于关机状态
follow-up-chat-remind = 如果无人回复，将于 { $date } 回到收件箱
toast-follow-up-sent = 已发送跟进邮件
toast-follow-up-stopped = 已停止跟进
toast-follow-up-moved = 跟进已改到 { $date }
nudge-row = { $days ->
   *[other] { $days } 天前发送
}。要跟进吗？
nudge-row-tip = 给会话中的所有人写一封跟进邮件
nudge-follow-up = 跟进
nudge-dismiss = 忽略
nudge-card-title = 尚无回复
nudge-card-text = 你在 { $days ->
   *[other] { $days } 天前
}提出了问题，但没有人回答。
nudge-chat-line = { $days ->
   *[other] { $days } 天前发送
}，尚无回复
toast-nudge-dismissed = 已忽略提醒
