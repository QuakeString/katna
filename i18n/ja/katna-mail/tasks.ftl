# Katna Mail, Japanese (日本語): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = 作成
tasks-all = すべてのタスク
tasks-starred = スター付き
tasks-new-list = 新しいリストを作成
tasks-on-this-computer = このパソコン
tasks-my-tasks = マイタスク
tasks-list-name-placeholder = リスト名

## Lists and tasks

tasks-loading = タスクを読み込んでいます…
tasks-no-lists = タスクリストはここに表示されます。
tasks-add = タスクを追加
tasks-title-placeholder = タイトル
tasks-add-step = サブタスクを追加
tasks-empty = タスクはまだありません。上から追加してください。
tasks-starred-empty = タスクにスターを付けると、ここに表示されます。
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
tasks-toast-deleted = タスクを削除しました
tasks-toast-added = { $count ->
   *[other] { $count } 件をタスクに追加しました
}
tasks-mail-gone = そのメールは見つかりません。
tasks-toast-list-deleted = リストを削除しました
tasks-toast-moved = { $list } に移動しました
