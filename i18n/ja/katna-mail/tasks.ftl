# Katna Mail, Japanese (日本語): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = 新しいタスク
tasks-all = すべてのタスク
tasks-today = 今日
tasks-upcoming = 今後
tasks-starred = スター付き
tasks-completed-view = 完了済み
tasks-new-list = 新しいリストを作成
tasks-labels-heading = ラベル
tasks-on-this-computer = このパソコン
tasks-my-tasks = マイタスク
tasks-account-sign-in = もう一度サインインしてタスクを表示
tasks-account-signed-in = { $address } に再度サインインしました。タスクを取得しています…
tasks-account-sign-in-refused = { $provider } が Katna のアクセスを許可しませんでした。もう一度試して、タスクへのアクセスを許可してください。
tasks-account-refused = サーバーがパスワードを受け付けませんでした。Yahoo、iCloud、Zoho などではアプリ パスワードが必要です。
tasks-account-change-password = パスワードを変更
tasks-account-change-password-tooltip = 新しいパスワードを入力してください。Katna がサーバーで確認します
tasks-account-not-enabled = Katna のタスクへのアクセスはまだ有効になっていません。
tasks-account-failed = タスクリストを読み込めませんでした。
tasks-account-error = タスクリストを読み込めませんでした: { $reason }
tasks-account-none = タスクリストが見つかりません
tasks-account-none-why = タスクリストが見つかりません: { $reason }
tasks-account-use-sign-in = { $provider } のタスクは、{ $provider } でサインインした Katna にのみ表示されます。
tasks-account-sign-in-with = { $provider } でサインイン
tasks-account-looking = タスクリストを探しています…
tasks-account-try-again = 再試行
tasks-account-try-again-tooltip = このアカウントのタスクを今すぐ再確認
tasks-account-fixing = 対応しています…
tasks-list-name-placeholder = リスト名

## Lists and tasks

tasks-loading = タスクを読み込んでいます…
tasks-no-lists = タスクリストはここに表示されます。
tasks-search = タスクを検索
tasks-search-none = 検索条件に一致するタスクはありません。
tasks-add = タスクを追加
tasks-title-placeholder = タイトル
tasks-add-step = サブタスクを追加
tasks-empty = タスクはまだありません。上から追加してください。
tasks-starred-empty = タスクにスターを付けると、ここに表示されます。
tasks-label-empty = このラベルの未完了のタスクはありません。
tasks-today-empty = 今日が期限のタスクはありません。
tasks-completed-empty = 完了したタスクがここに表示されます。
tasks-upcoming-add = { $day }のタスクを追加
tasks-upcoming-overdue-day = { $day }（{ $weekday }）
tasks-from-mail-quiet = メールから
tasks-from-note-quiet = メモから
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }、{ $day }
tasks-overdue = 期限切れ
tasks-completed = { $count ->
   *[other] 完了 ({ $count })
}
tasks-list-options = リストのオプション
tasks-sort-by = 並べ替え
tasks-sort-my-order = カスタム順
tasks-sort-date = 日付
tasks-sort-starred = 最近スターを付けた順
tasks-sort-title = タイトル
tasks-rename-list = リストの名前を変更
tasks-delete-list = リストを削除
tasks-mark-done = 完了にする
tasks-mark-open = 未完了にする
tasks-star = スターを付ける
tasks-unstar = スターを外す
tasks-edit-title = タイトルを編集
tasks-details = 詳細
tasks-delete = 削除
tasks-move-to = { $list } に移動
tasks-from-mail = メール
tasks-open-mail = メールを開く
tasks-from-note = メモ
tasks-open-note = メモを開く
tasks-note-gone = そのメモは見つかりません。
tasks-no-subject = （件名なし）

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
   *[other] { $count } 件を選択中
}
tasks-select-clear = 選択を解除
tasks-select-move = リストに移動
tasks-select-date = 日付を設定
tasks-next-week = 来週

## The details dialog

tasks-notes-placeholder = 詳細を追加
tasks-date = 日付
tasks-no-date = 日付なし
tasks-time-placeholder = 時刻を追加
tasks-repeat = 繰り返し
tasks-repeat-never = 繰り返さない
tasks-repeat-daily = 毎日
tasks-repeat-weekly = 毎週
tasks-repeat-monthly = 毎月
tasks-repeat-yearly = 毎年
tasks-repeat-other = カスタム
tasks-remind = リマインダー
tasks-remind-off = 通知しない
tasks-remind-on-time = その時刻に
tasks-remind-morning = 当日 { $time }
tasks-remind-hour-before = 1 時間前
tasks-remind-day-before = 前日
tasks-label-add = ラベルを追加
tasks-label-task = タスクにラベルを付ける
tasks-files-attach = ファイルを添付
tasks-files-pick = 添付
tasks-file-open = 開く
tasks-file-remove = ファイルを削除
tasks-file-here = このパソコンのみ
tasks-cancel = キャンセル
tasks-save = 保存
tasks-not-a-time = 「{ $text }」は時刻ではありません。例: { $example }

## Due days

tasks-due-today = 今日
tasks-due-tomorrow = 明日
tasks-due-yesterday = 昨日
tasks-due-at = { $day } { $time }

## Notes at the bottom

tasks-toast-done = タスクを完了しました
tasks-toast-next = 完了しました。次回は { $date }
tasks-toast-deleted = タスクを削除しました
tasks-files-added = { $count ->
   *[other] { $count } 個のファイルを添付しました
}
tasks-file-removed = 「{ $name }」を削除しました
tasks-files-left-out = 添付されませんでした: { $names }。タスクに添付できるのは { $limit } までのファイルで、フォルダは添付できません。
tasks-file-missing = そのファイルはもうありません。
tasks-toast-added = { $count ->
   *[other] { $count } 件をタスクに追加しました
}
tasks-mail-gone = そのメールは見つかりません。
tasks-toast-list-deleted = リストを削除しました
tasks-toast-moved = { $list } に移動しました
tasks-toast-placed = タスクを移動しました
tasks-toast-rescheduled = タスクの日時を変更しました
tasks-toast-rescheduled-several = { $count ->
   *[other] { $count } 件のタスクの日時を変更しました
}
tasks-toast-done-several = { $count ->
   *[other] { $count } 件のタスクを完了しました
}
tasks-toast-open-several = { $count ->
   *[other] { $count } 件のタスクを未完了にしました
}
tasks-toast-starred = { $count ->
   *[other] { $count } 件のタスクにスターを付けました
}
tasks-toast-unstarred = { $count ->
   *[other] { $count } 件のタスクのスターを外しました
}
tasks-toast-deleted-several = { $count ->
   *[other] { $count } 件のタスクを削除しました
}
