# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = 延後至…
snooze-later-today = 今天稍晚
snooze-tomorrow = 明天
snooze-this-weekend = 本週末
snooze-next-week = 下週
snooze-pick = 選擇日期和時間
snooze-back = 返回時間選項
snooze-type-placeholder = 輸入時間
snooze-type-hint = 例如「tue 3pm」、「tomorrow」或「in 2 hours」
snooze-type-hint-unclear = Katna 無法將其解讀為時間
snooze-type-unclear = Katna 無法將「{ $text }」解讀為時間

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = 延後
remind-tab = 提醒我
snooze-says = 在此之前隱藏
remind-says = 保留在原位並通知你
remind-before-due = 到期之前
remind-note = 備註（選填）
remind-note-placeholder = 留空時使用主旨
toast-remind-set = 已設定提醒：{ $date }
remind-chat-line = 提醒 { $date } · { $title }
remind-done = 完成
toast-remind-done = 提醒已完成
snooze-chat-line = 已延後至 { $date }
snooze-chat-change = 變更

## The date and time picker

snooze-cancel = 取消
snooze-save = 儲存
snooze-in-the-past = 請選擇晚於現在的時間。

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = 無人回覆時後續追蹤…
follow-up-title = 無人回覆時後續追蹤
follow-up-off = 關閉
follow-up-days = { $days ->
   *[other] { $days } 天
}
follow-up-weeks = { $weeks ->
   *[other] { $weeks } 週
}
follow-up-pick = 選擇…
follow-up-pick-title = 若到此時仍無人回覆就後續追蹤
follow-up-remind = 提醒我
follow-up-remind-note = 會話群組會回到收件匣頂端
follow-up-send = 替我傳送後續追蹤郵件
follow-up-send-note = 寄給相同的收件者，並在同一個會話群組中
follow-up-send-encrypted = 不適用於加密郵件
follow-up-text-placeholder = 要寫些什麼
follow-up-text-named = { $name } 你好，想確認一下你是否看到了我下面的郵件。
follow-up-text = 你好，想確認一下你是否看到了我下面的郵件。
follow-up-template = 使用範本
follow-up-signature = 會加上你的簽名
follow-up-again = 如果仍無人回覆，再次後續追蹤的間隔
follow-up-note = 會話群組中任何人一回覆就會停止。自動回覆不算在內。
follow-up-note-send = 會話群組中任何人一回覆就會停止。只在平日 { $start } 至 { $end } 之間寄出，最多延遲一天。
follow-up-cancel = 取消
follow-up-done = 完成
follow-up-chip-send = { $time }後追蹤
follow-up-chip-remind = { $time }後提醒
follow-up-chip-send-on = 後續追蹤 { $date }
follow-up-chip-remind-on = 提醒 { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = 尚無回覆
follow-up-card-title-waiting = 你的後續追蹤郵件正在等候
follow-up-card-send = Katna 會在 { $date } 寄出你的後續追蹤郵件。有任何人回覆就會停止。
follow-up-card-send-twice = Katna 會在 { $date } 寄出你的後續追蹤郵件，之後會再寄一次。有任何人回覆就會停止。
follow-up-card-remind = 如果沒有人回覆，這個會話群組會在 { $date } 回到你的收件匣。
follow-up-card-waiting = 預定寄出時你的電腦處於關機狀態，因此沒有延遲寄出。你可以立即寄出、選擇新時間，或停止。
follow-up-card-edit = 編輯
follow-up-card-edit-title = 後續追蹤時間
follow-up-card-send-now = 立即傳送
follow-up-card-stop = 停止
follow-up-chat-send = 後續追蹤 · 若無人回覆，於 { $date } 寄出
follow-up-chat-step = 後續追蹤 { $step }/{ $steps } · 若無人回覆，於 { $date } 寄出
follow-up-chat-waiting = 後續追蹤待處理 · 預定時間電腦處於關機狀態
follow-up-chat-remind = 若無人回覆，於 { $date } 回到收件匣
toast-follow-up-sent = 已傳送後續追蹤郵件
toast-follow-up-stopped = 已停止後續追蹤
toast-follow-up-moved = 後續追蹤已改至 { $date }
nudge-row = { $days ->
   *[other] { $days } 天前寄出
}。要後續追蹤嗎？
nudge-row-tip = 寫一封後續追蹤郵件給其中的所有人
nudge-follow-up = 後續追蹤
nudge-dismiss = 關閉
nudge-card-title = 尚無回覆
nudge-card-text = 你在{ $days ->
   *[other] { $days } 天前
}提出了問題，但沒有人回答。
nudge-chat-line = { $days ->
   *[other] { $days } 天前寄出
}，尚無回覆
toast-nudge-dismissed = 已關閉提醒
