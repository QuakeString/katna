# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = メイン
tab-promotions = プロモーション
tab-social = ソーシャル
tab-updates = 新着
tab-forums = フォーラム
tab-focused = 優先
tab-other = その他
tab-inbox = 受信トレイ
tab-newsletters = ニュースレター
tab-notifications = 通知
tab-new = 新着 { $count } 件
tab-provider-other = Katna が分類

## Mail list: toolbar

list-select = 選択
list-refresh = 更新
list-more = その他
list-mark-read = 既読にする
list-mark-unread = 未読にする
list-move-to = 移動
list-archive = アーカイブ
list-spam = 迷惑メールを報告
list-delete = 削除
list-newer = 新しい
list-older = 古い
list-range = { $total } 件中 { $first }–{ $last } 件
list-range-about = 約 { $total } 件中 { $first }–{ $last } 件
list-results = 「{ $query }」の検索結果
list-results-corrected = 「{ $query }」の検索結果を表示しています
list-search-instead = 「{ $query }」で検索する
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = すべて
list-pick-none = 選択しない
list-pick-read = 既読
list-pick-unread = 未読
list-pick-starred = スター付き
list-pick-unstarred = スターなし

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count } 件のスレッドがすべて選択されています。
   *[message] { $count } 件のメールがすべて選択されています。
}
list-selected-all-in = { $kind ->
    [conversation] 「{ $folder }」の { $count } 件のスレッドがすべて選択されています。
   *[message] 「{ $folder }」の { $count } 件のメールがすべて選択されています。
}
list-selected-screen = { $kind ->
    [conversation] このページの { $count } 件のスレッドがすべて選択されています。
   *[message] このページの { $count } 件のメールがすべて選択されています。
}
list-select-all = { $kind ->
    [conversation] { $count } 件のスレッドをすべて選択
   *[message] { $count } 件のメールをすべて選択
}
list-select-all-in = { $kind ->
    [conversation] 「{ $folder }」の { $count } 件のスレッドをすべて選択
   *[message] 「{ $folder }」の { $count } 件のメールをすべて選択
}
list-clear-selection = 選択を解除

## Mail list: empty states

list-empty-search = 検索条件に一致するメールはありません。
list-empty-tab = 「{ $tab }」にメールはありません。
list-empty-tab-unknown = このタブにメールはありません。
list-empty-folder = 「{ $folder }」にメールはありません。
list-empty-folder-unknown = このフォルダにメールはありません。
list-first-sync = メールを取得しています…
list-first-sync-detail = 届いたメールから順にここに表示されます。

## Mail list: lines

row-removed = このメールは削除されました。
row-starred = スター付き
row-not-starred = スターなし
row-important = 重要。クリックすると重要ではないとマークします。
row-mark-important = 重要マークを付ける
row-pinned = 上部に固定済み
row-pin = 上部に固定
row-unpin = 固定を解除

## Mail list: More menu and right-click menu

menu-reply = 返信
menu-reply-all = 全員に返信
menu-forward = 転送
menu-archive = アーカイブ
menu-delete = 削除
menu-spam = 迷惑メールを報告
menu-mark-read = 既読にする
menu-mark-unread = 未読にする
menu-mark-all-read = すべて既読にする
menu-star = スターを付ける
menu-unstar = スターを外す
menu-important = 重要マークを付ける
menu-not-important = 重要ではないとマーク
menu-pin = 上部に固定
menu-unpin = 固定を解除
menu-print-all = すべて印刷
menu-new-window = 新しいウィンドウで開く
menu-move-to = 移動
menu-move-to-heading = 移動先:
menu-find-from = { $name } からのメールを検索

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count } 件のスレッドをアーカイブしました。
   *[message] { $count } 件のメールをアーカイブしました。
}
toast-trashed = { $kind ->
    [conversation] { $count } 件のスレッドをゴミ箱に移動しました。
   *[message] { $count } 件のメールをゴミ箱に移動しました。
}
toast-moved = { $kind ->
    [conversation] { $count } 件のスレッドを移動しました。
   *[message] { $count } 件のメールを移動しました。
}
toast-starred = { $kind ->
    [conversation] { $count } 件のスレッドにスターを付けました。
   *[message] { $count } 件のメールにスターを付けました。
}
toast-unstarred = { $kind ->
    [conversation] { $count } 件のスレッドのスターを外しました。
   *[message] { $count } 件のメールのスターを外しました。
}
toast-important = { $kind ->
    [conversation] { $count } 件のスレッドに重要マークを付けました。
   *[message] { $count } 件のメールに重要マークを付けました。
}
toast-not-important = { $kind ->
    [conversation] { $count } 件のスレッドを重要ではないとマークしました。
   *[message] { $count } 件のメールを重要ではないとマークしました。
}
toast-pinned = { $kind ->
    [conversation] { $count } 件のスレッドを上部に固定しました。
   *[message] { $count } 件のメールを上部に固定しました。
}
toast-unpinned = { $kind ->
    [conversation] { $count } 件のスレッドの固定を解除しました。
   *[message] { $count } 件のメールの固定を解除しました。
}
toast-spam = { $kind ->
    [conversation] { $count } 件のスレッドを迷惑メールとして報告しました。
   *[message] { $count } 件のメールを迷惑メールとして報告しました。
}
toast-deleted-forever = { $kind ->
    [conversation] { $count } 件のスレッドを完全に削除しました。
   *[message] { $count } 件のメールを完全に削除しました。
}
toast-undone = 操作を元に戻しました。
toast-undo = 元に戻す
toast-no-spam-folder = このアカウントには迷惑メールフォルダがありません。
