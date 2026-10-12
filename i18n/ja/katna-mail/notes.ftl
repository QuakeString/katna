# Katna Mail, Japanese (日本語): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = メモ
notes-view-reminders = リマインダー
notes-view-archive = アーカイブ
notes-view-trash = ゴミ箱
notes-edit-labels = ラベルを編集
notes-search = メモを検索
notes-loading = メモを開いています…

## Board

notes-take-a-note = メモを入力…
notes-new-list = 新しいリスト
notes-new-note = 新しいメモ
notes-pinned = 固定済み
notes-others = その他
notes-empty = 追加したメモがここに表示されます
notes-archive-empty = アーカイブしたメモがここに表示されます
notes-trash-empty = ゴミ箱にメモはありません
notes-none-found = 一致するメモはありません
notes-label-empty = このラベルのメモはまだありません
notes-reminders-empty = リマインダーが近いメモがここに表示されます
notes-trash-note = ゴミ箱内のメモは 7 日後に削除されます。
notes-empty-trash = ゴミ箱を空にする
notes-ticked = { $count ->
   *[other] + チェック済み { $count } 件
}
notes-select = メモを選択
notes-selected = { $count ->
   *[other] { $count } 件を選択中
}
notes-select-clear = 選択を解除

## A note's buttons

notes-pin = メモを固定
notes-unpin = メモの固定を解除
notes-archive = アーカイブ
notes-unarchive = アーカイブ解除
notes-delete = メモを削除
notes-restore = 復元
notes-delete-forever = 完全に削除
notes-color = 背景オプション
notes-checkboxes = チェックボックスを表示/非表示
notes-labels = ラベル
notes-close = 閉じる
notes-more = その他
notes-make-copy = コピーを作成
notes-remind = リマインド
notes-add-picture = 画像を追加
notes-history = バージョン履歴
notes-ai = 文章作成をサポート
notes-send-as-mail = メールとして送信
notes-save-markdown = Markdown として保存
notes-save-pdf = PDF として保存

## The open note

notes-title = タイトル
notes-edited = 編集日: { $date }
notes-on-this-computer = このコンピューター
notes-where = メモの保存先
notes-untitled = 無題のメモ

## Pictures

notes-picture-choose = 画像を追加
notes-picture-remove = 画像を削除
notes-picture-too-big = メモに追加できる画像は { $size } までです
notes-picture-kind = Katna で表示できる画像ファイルではありません
notes-picture-unreadable = { $name } を読み込めませんでした: { $error }

## Reminders

notes-remind-me = リマインド
notes-remind-off = リマインダーを削除
notes-remind-in-the-past = まだ過ぎていない時刻を選んでください
notes-remind-today = 今日 { $time }
notes-remind-tomorrow = 明日 { $time }
notes-remind-weekday = { $day } { $time }
notes-reminder-set = リマインダーを { $when } に設定しました
notes-reminder-off = リマインダーを削除しました

## Links between notes

notes-link-note = メモをリンク
notes-link-new = 新しいメモ「{ $title }」
notes-linked-from = リンク元
notes-link-gone = そのメモはもうありません
notes-new-note-gone = 新しいメモが見つかりません。

## Version history

notes-versions = バージョン
notes-version-now = 現在
notes-version-here = 自分（このパソコン）
notes-version-yesterday = 昨日 { $time }
notes-version-changes = { $count ->
   *[other] { $count } か所の変更
}
notes-version-from = { $device } から
notes-version-elsewhere = 別のデバイスから
notes-version-created = 作成
notes-version-restore = このバージョンを復元
notes-version-restored = バージョンを復元しました
notes-history-none = 以前のバージョンはまだありません

## AI help

notes-ai-tidy = 文章を整える
notes-ai-checklist = チェックリストにする
notes-ai-summarise = 要約
notes-ai-empty = 先に何か書いてください
notes-ai-tidied = 文章を整えました。Ctrl+Z で元に戻せます。
notes-ai-listed = チェックリストにしました。Ctrl+Z で元に戻せます。
notes-ai-summarised = 先頭に要約を追加しました

## Labels

notes-label-note = メモにラベルを付ける
notes-label-name = ラベル名を入力
notes-label-create = 「{ $name }」を作成
notes-label-remove = ラベルを外す
notes-label-delete = ラベルを削除
notes-labels-none = ラベルはまだありません。メモのラベルボタンから追加できます。
notes-labels-done = 完了
notes-label-renamed = ラベル名を「{ $name }」に変更しました
notes-label-deleted = ラベル「{ $name }」を削除しました

## A note about a mail

notes-mail = メール
notes-open-mail = メールを開く
notes-open-note = メモを開く

## Meeting notes

notes-meeting-take = 会議メモを作成
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = 参加者: { $names }
notes-meeting-notes = メモ
notes-meeting-actions = アクション アイテム
notes-event = 予定
notes-open-event = 予定を開く

## Formatting

notes-format = 書式
notes-format-heading-1 = 見出し 1
notes-format-heading-2 = 見出し 2
notes-format-normal = 標準テキスト
notes-format-bold = 太字
notes-format-italic = 斜体
notes-format-underline = 下線
notes-format-quote = 引用
notes-format-code = コード
notes-format-divider = 区切り線
notes-format-clear = 書式をクリア

## Tasks

notes-make-task = タスクにする

## Colors (tooltips)

notes-color-none = 色なし
notes-color-coral = コーラル
notes-color-peach = ピーチ
notes-color-sand = サンド
notes-color-mint = ミント
notes-color-sage = セージ
notes-color-fog = フォグ
notes-color-storm = ストーム
notes-color-dusk = ダスク
notes-color-blossom = ブロッサム
notes-color-clay = クレイ
notes-color-chalk = チョーク

## Messages at the foot of the window

notes-archived = メモをアーカイブしました
notes-unarchived = メモのアーカイブを解除しました
notes-trashed = メモをゴミ箱に移動しました
notes-restored = メモを復元しました
notes-saved = メモを保存しました
notes-pinned-count = { $count ->
   *[other] { $count } 件のメモを固定しました
}
notes-unpinned-count = { $count ->
   *[other] { $count } 件のメモの固定を解除しました
}
notes-colored-count = { $count ->
   *[other] { $count } 件のメモの色を変更しました
}
notes-archived-count = { $count ->
   *[other] { $count } 件のメモをアーカイブしました
}
notes-unarchived-count = { $count ->
   *[other] { $count } 件のメモのアーカイブを解除しました
}
notes-trashed-count = { $count ->
   *[other] { $count } 件のメモをゴミ箱に移動しました
}
notes-restored-count = { $count ->
   *[other] { $count } 件のメモを復元しました
}
notes-copied-count = { $count ->
   *[other] { $count } 件のコピーを作成しました
}
notes-empty-discarded = 空のメモを破棄しました
notes-mail-gone = そのメールは見つかりません
notes-deleted-forever = { $count ->
   *[other] { $count } 件のメモを完全に削除しました
}
