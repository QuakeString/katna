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
list-checking = 新着メールを確認中…
list-more = その他
list-mark-read = 既読にする
list-mark-unread = 未読にする
list-move-to = 移動
list-archive = アーカイブ
list-spam = 迷惑メールを報告
list-delete = 削除
list-snooze = スヌーズ
list-unsnooze = スヌーズを解除
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
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] このページの既読のスレッド { $count } 件がすべて選択されています。
       *[message] このページの既読のメール { $count } 件がすべて選択されています。
    }
   *[unread] { $kind ->
        [conversation] このページの未読のスレッド { $count } 件がすべて選択されています。
       *[message] このページの未読のメール { $count } 件がすべて選択されています。
    }
    [starred] { $kind ->
        [conversation] このページのスター付きのスレッド { $count } 件がすべて選択されています。
       *[message] このページのスター付きのメール { $count } 件がすべて選択されています。
    }
    [unstarred] { $kind ->
        [conversation] このページのスターなしのスレッド { $count } 件がすべて選択されています。
       *[message] このページのスターなしのメール { $count } 件がすべて選択されています。
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] 既読のスレッド { $count } 件をすべて選択
       *[message] 既読のメール { $count } 件をすべて選択
    }
   *[unread] { $kind ->
        [conversation] 未読のスレッド { $count } 件をすべて選択
       *[message] 未読のメール { $count } 件をすべて選択
    }
    [starred] { $kind ->
        [conversation] スター付きのスレッド { $count } 件をすべて選択
       *[message] スター付きのメール { $count } 件をすべて選択
    }
    [unstarred] { $kind ->
        [conversation] スターなしのスレッド { $count } 件をすべて選択
       *[message] スターなしのメール { $count } 件をすべて選択
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] 「{ $folder }」の既読のスレッド { $count } 件をすべて選択
       *[message] 「{ $folder }」の既読のメール { $count } 件をすべて選択
    }
   *[unread] { $kind ->
        [conversation] 「{ $folder }」の未読のスレッド { $count } 件をすべて選択
       *[message] 「{ $folder }」の未読のメール { $count } 件をすべて選択
    }
    [starred] { $kind ->
        [conversation] 「{ $folder }」のスター付きのスレッド { $count } 件をすべて選択
       *[message] 「{ $folder }」のスター付きのメール { $count } 件をすべて選択
    }
    [unstarred] { $kind ->
        [conversation] 「{ $folder }」のスターなしのスレッド { $count } 件をすべて選択
       *[message] 「{ $folder }」のスターなしのメール { $count } 件をすべて選択
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] 既読のスレッド { $count } 件がすべて選択されています。
       *[message] 既読のメール { $count } 件がすべて選択されています。
    }
   *[unread] { $kind ->
        [conversation] 未読のスレッド { $count } 件がすべて選択されています。
       *[message] 未読のメール { $count } 件がすべて選択されています。
    }
    [starred] { $kind ->
        [conversation] スター付きのスレッド { $count } 件がすべて選択されています。
       *[message] スター付きのメール { $count } 件がすべて選択されています。
    }
    [unstarred] { $kind ->
        [conversation] スターなしのスレッド { $count } 件がすべて選択されています。
       *[message] スターなしのメール { $count } 件がすべて選択されています。
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] 「{ $folder }」の既読のスレッド { $count } 件がすべて選択されています。
       *[message] 「{ $folder }」の既読のメール { $count } 件がすべて選択されています。
    }
   *[unread] { $kind ->
        [conversation] 「{ $folder }」の未読のスレッド { $count } 件がすべて選択されています。
       *[message] 「{ $folder }」の未読のメール { $count } 件がすべて選択されています。
    }
    [starred] { $kind ->
        [conversation] 「{ $folder }」のスター付きのスレッド { $count } 件がすべて選択されています。
       *[message] 「{ $folder }」のスター付きのメール { $count } 件がすべて選択されています。
    }
    [unstarred] { $kind ->
        [conversation] 「{ $folder }」のスターなしのスレッド { $count } 件がすべて選択されています。
       *[message] 「{ $folder }」のスターなしのメール { $count } 件がすべて選択されています。
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] ここに既読のスレッドはありません。
       *[message] ここに既読のメールはありません。
    }
   *[unread] { $kind ->
        [conversation] ここに未読のスレッドはありません。
       *[message] ここに未読のメールはありません。
    }
    [starred] { $kind ->
        [conversation] ここにスター付きのスレッドはありません。
       *[message] ここにスター付きのメールはありません。
    }
    [unstarred] { $kind ->
        [conversation] ここにスターなしのスレッドはありません。
       *[message] ここにスターなしのメールはありません。
    }
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
row-task = タスク
row-task-open = タスクを開く: { $title }
row-tracking-none = 追跡中。まだ開封されていません
row-tracking-opened = { $recipients } 人中 { $opened } 人が開封
row-tracking-clicked = { $recipients } 人中 { $opened } 人が開封、{ $clicked } 人がリンクをクリック
row-pin = 上部に固定
row-unpin = 固定を解除
row-snoozed-until = { $when } までスヌーズ中

## Mail list: More menu and right-click menu

menu-reply = 返信
menu-reply-all = 全員に返信
menu-forward = 転送
menu-archive = アーカイブ
menu-delete = 削除
menu-delete-forever = 完全に削除
menu-move-to-inbox = 受信トレイに移動
menu-spam = 迷惑メールを報告
menu-not-spam = 迷惑メールではない
menu-mark-read = 既読にする
menu-mark-unread = 未読にする
menu-mark-all-read = すべて既読にする
menu-star = スターを付ける
menu-unstar = スターを外す
menu-important = 重要マークを付ける
menu-not-important = 重要ではないとマーク
menu-pin = 上部に固定
menu-unpin = 固定を解除
menu-snooze = スヌーズ
menu-unsnooze = スヌーズを解除
menu-add-to-tasks = タスクに追加
menu-schedule-meeting = 会議を設定
menu-start-call = ビデオ通話を開始
menu-add-note = メモを追加
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
toast-snoozed = { $kind ->
    [conversation] { $count } 件のスレッドを { $when } までスヌーズしました。
   *[message] { $count } 件のメールを { $when } までスヌーズしました。
}
toast-unsnoozed = { $kind ->
    [conversation] { $count } 件のスレッドを受信トレイに戻しました。
   *[message] { $count } 件のメールを受信トレイに戻しました。
}
toast-spam = { $kind ->
    [conversation] { $count } 件のスレッドを迷惑メールとして報告しました。
   *[message] { $count } 件のメールを迷惑メールとして報告しました。
}
toast-not-spam = { $kind ->
    [conversation] { $count } 件のスレッドを迷惑メールではないとして受信トレイに移動しました。
   *[message] { $count } 件のメールを迷惑メールではないとして受信トレイに移動しました。
}
toast-deleted-forever = { $kind ->
    [conversation] { $count } 件のスレッドを完全に削除しました。
   *[message] { $count } 件のメールを完全に削除しました。
}
toast-marked-read = { $kind ->
    [conversation] { $count } 件のスレッドを既読にしました。
   *[message] { $count } 件のメールを既読にしました。
}
toast-marked-unread = { $kind ->
    [conversation] { $count } 件のスレッドを未読にしました。
   *[message] { $count } 件のメールを未読にしました。
}
toast-undone = 操作を元に戻しました。
toast-nothing-to-undo = 元に戻す操作はありません。
toast-cannot-undo-delete-forever = 完全に削除したメールは元に戻せません。
toast-send-undone = 送信を取り消しました。
toast-too-late-to-undo-send = 取り消すには遅すぎます: メールはすでに送信されました。
toast-undo = 元に戻す
toast-close = 閉じる
toast-no-spam-folder = このアカウントには迷惑メールフォルダがありません。
