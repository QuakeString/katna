# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The snooze menu (the clock on a line, or Snooze in the right-click menu)

snooze-until = スヌーズの期限…
snooze-later-today = 今日の後ほど
snooze-tomorrow = 明日
snooze-this-weekend = 今週末
snooze-next-week = 来週
snooze-pick = 日付と時間を選択
snooze-back = 時刻の一覧に戻る
snooze-type-placeholder = 時刻を入力
snooze-type-hint = 例: 「火 15時」「明日」「2 時間後」
snooze-type-hint-unclear = Katna はそれを時刻として読み取れません
snooze-type-unclear = 「{ $text }」は Katna が認識できる時刻ではありません

## Snooze and Remind me on mail: one menu, switched at its top (B and H)

snooze-tab = スヌーズ
remind-tab = リマインド
snooze-says = その時刻まで非表示にします
remind-says = そのままの場所に残して通知します
remind-before-due = 期限の前
remind-note = メモ（省略可）
remind-note-placeholder = 空欄の場合は件名
toast-remind-set = リマインダーを { $date } に設定しました
remind-chat-line = リマインダー { $date } · { $title }
remind-done = 完了
toast-remind-done = リマインダーを完了しました
snooze-chat-line = { $date } までスヌーズ中
snooze-chat-change = 変更

## The date and time picker

snooze-cancel = キャンセル
snooze-save = 保存
snooze-in-the-past = 現在より後の時刻を選択してください。

## Follow up if no reply: compose's send menu, its popover and the chip
## beside Send

follow-up-menu = 返信がなければフォローアップ…
follow-up-title = 返信がなければフォローアップ
follow-up-off = オフ
follow-up-days = { $days ->
   *[other] { $days } 日
}
follow-up-weeks = { $weeks ->
   *[other] { $weeks } 週間
}
follow-up-pick = 選択…
follow-up-pick-title = 次の日時までに返信がなければフォローアップ
follow-up-remind = リマインド
follow-up-remind-note = スレッドが受信トレイの先頭に戻ります
follow-up-send = フォローアップを自動送信
follow-up-send-note = 同じ宛先に、同じスレッドで送信します
follow-up-send-encrypted = 暗号化されたメールでは使えません
follow-up-text-placeholder = 本文
follow-up-text-named = { $name } さん、下記のメッセージをご確認いただけましたでしょうか。
follow-up-text = 下記のメッセージをご確認いただけましたでしょうか。
follow-up-template = テンプレートを使用
follow-up-signature = 署名が追加されます
follow-up-again = それでも返信がなければ、次の期間後にもう一度フォローアップ
follow-up-note = スレッドの誰かが返信した時点で停止します。自動返信は含みません。
follow-up-note-send = スレッドの誰かが返信した時点で停止します。平日の { $start }～{ $end } に送信し、予定より 1 日以上遅れることはありません。
follow-up-cancel = キャンセル
follow-up-done = 完了
follow-up-chip-send = { $time }後にフォローアップ
follow-up-chip-remind = { $time }後にリマインダー
follow-up-chip-send-on = フォローアップ { $date }
follow-up-chip-remind-on = リマインダー { $date }

## The card over an open conversation a follow-up waits on.

follow-up-card-title = まだ返信がありません
follow-up-card-title-waiting = フォローアップが待機中です
follow-up-card-send = Katna が { $date } にフォローアップを送信します。誰かが返信すると停止します。
follow-up-card-send-twice = Katna が { $date } にフォローアップを送信し、その後もう一度送信します。誰かが返信すると停止します。
follow-up-card-remind = 誰も返信しなければ、このスレッドは { $date } に受信トレイに戻ります。
follow-up-card-waiting = パソコンの電源が切れている間に予定時刻になったため、遅れて送信はしませんでした。今すぐ送信するか、新しい時刻を選ぶか、停止してください。
follow-up-card-edit = 編集
follow-up-card-edit-title = フォローアップの日時
follow-up-card-send-now = 今すぐ送信
follow-up-card-stop = 停止
follow-up-chat-send = フォローアップ · 誰も返信しなければ { $date }
follow-up-chat-step = フォローアップ { $step }/{ $steps } · 誰も返信しなければ { $date }
follow-up-chat-waiting = フォローアップ待機中 · パソコンの電源が切れている間に予定時刻になりました
follow-up-chat-remind = 返信がなければ { $date } に受信トレイに戻ります
toast-follow-up-sent = フォローアップを送信しました
toast-follow-up-stopped = フォローアップを停止しました
toast-follow-up-moved = フォローアップを { $date } に変更しました
nudge-row = { $days ->
   *[other] { $days } 日前
}に送信。フォローアップしますか？
nudge-row-tip = 全員にフォローアップを書く
nudge-follow-up = フォローアップ
nudge-dismiss = 閉じる
nudge-card-title = まだ返信がありません
nudge-card-text = { $days ->
   *[other] { $days } 日前
}に質問しましたが、まだ誰も答えていません。
nudge-chat-line = { $days ->
   *[other] { $days } 日前
}に送信、まだ返信がありません
toast-nudge-dismissed = ナッジを閉じました
