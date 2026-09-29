# Katna Mail, Japanese (日本語): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = 作成
tasks-all = すべてのタスク
tasks-today = 今日
tasks-starred = スター付き
tasks-new-list = 新しいリストを作成
tasks-on-this-computer = このパソコン
tasks-my-tasks = マイタスク
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = もう一度サインインしてタスクを表示
tasks-account-signed-in = { $address } に再度サインインしました。タスクを取得しています…
tasks-account-sign-in-refused = { $provider } が Katna のアクセスを許可しませんでした。もう一度試して、タスクへのアクセスを許可してください。
tasks-account-refused = サーバーがパスワードを受け付けませんでした。Yahoo、iCloud、Zoho などではアプリ パスワードが必要です。
tasks-account-change-password = パスワードを変更
tasks-account-change-password-tooltip = 設定 > アカウント を開く
tasks-account-not-enabled = Katna のタスクへのアクセスはまだ有効になっていません。
tasks-account-failed = タスクリストを読み込めませんでした。
# $reason is the server's own words, in English.
tasks-account-error = タスクリストを読み込めませんでした: { $reason }
tasks-account-none = タスクリストが見つかりません
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
tasks-today-empty = 今日が期限のタスクはありません。
tasks-today-date = { $weekday }、{ $day }
tasks-overdue = 期限切れ
tasks-completed = { $count ->
   *[other] 完了 ({ $count })
}
tasks-list-options = リストのオプション
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
tasks-toast-added = { $count ->
   *[other] { $count } 件をタスクに追加しました
}
tasks-mail-gone = そのメールは見つかりません。
tasks-toast-list-deleted = リストを削除しました
tasks-toast-moved = { $list } に移動しました
tasks-toast-rescheduled = タスクの日時を変更しました
