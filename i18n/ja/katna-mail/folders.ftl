# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = ラベル
nav-folders = フォルダ
nav-label-new = 新しいラベルを作成
nav-folder-new = 新しいフォルダを作成
nav-menu-check-mail = 新着メールを確認
nav-menu-check-inbox = この受信トレイを確認
nav-unified-leave-out = 統合受信トレイから除外
nav-unified-bring-back = 統合受信トレイに戻す
nav-menu-sign-in-again = もう一度サインイン
nav-menu-new-mail = このアカウントから新規メール
nav-menu-account-settings = アカウント設定
nav-account-checked = 同期済み · { $ago }に確認
nav-account-in-sync = 同期済み
nav-account-connecting = 接続しています…
nav-account-offline = オフライン。再試行しています
nav-account-signed-out = { $provider } のサインインの期限が切れました
nav-account-password-refused = パスワードが拒否されました
nav-account-storage = { $total } 中 { $used } を使用
nav-menu-new-subfolder = 内側に新しいフォルダ
nav-menu-new-sublabel = 内側に新しいラベル
nav-menu-rename = 名前を変更
nav-menu-delete = 削除
nav-menu-empty-trash = ゴミ箱を空にする
nav-account-unnamed = アカウント { $number }
nav-all-accounts = すべてのアカウント
nav-expand = フォルダを表示
nav-collapse = フォルダを隠す
storage-used = { $total } 中 { $percent }% 使用
storage-used-detail = { $address }: { $total } 中 { $used } 使用

## Special folders (the user's own folders keep their names)

folder-inbox = 受信トレイ
folder-starred = スター付き
folder-snoozed = スヌーズ中
folder-unread = 未読
folder-important = 重要
folder-drafts = 下書き
folder-sent = 送信済み
folder-archive = アーカイブ
folder-spam = 迷惑メール
folder-trash = ゴミ箱
folder-all-mail = すべてのメール
folder-scheduled = 予定
folder-waiting = 返信待ち
folder-waiting-short = 返信待ち
folder-reminders = リマインダー
folder-outbox = 送信トレイ
folder-activity = アクティビティ
folder-not-on-account = このアカウントにはそのフォルダがありません。

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = 新しいラベル
label-folder-new-title = 新しいフォルダ
label-prompt = 新しいラベル名を入力してください:
label-folder-prompt = 新しいフォルダ名を入力してください:
label-name-hint = ラベル名
label-folder-name-hint = フォルダ名
label-nest = 次のラベルの下位にネスト:
label-folder-nest = 次のフォルダの下位にネスト:
label-cancel = キャンセル
label-create = 作成
label-creating = 作成しています…
label-created = ラベル「{ $name }」を作成しました。
label-folder-created = フォルダ「{ $name }」を作成しました。
label-rename-title = ラベル名を変更
label-folder-rename-title = フォルダ名を変更
label-rename = 名前を変更
label-renaming = 名前を変更しています…
label-renamed = ラベル名を「{ $name }」に変更しました。
label-folder-renamed = フォルダ名を「{ $name }」に変更しました。

## Deleting a folder or label (asked first)

folder-delete-title = 「{ $name }」を削除しますか？
folder-delete-body = { $count ->
    [0] メールはありません。フォルダはサーバーから削除されるため、ウェブメールやスマートフォンからも消えます。
   *[other] { $kind ->
        [conversation] { $count } 件のスレッドはゴミ箱に移動するので、後で元に戻せます。
       *[message] { $count } 件のメールはゴミ箱に移動するので、後で元に戻せます。
    }フォルダはサーバーから削除されるため、ウェブメールやスマートフォンからも消えます。
}
folder-delete-forever-body = { $count ->
    [0] メールはありません。フォルダはサーバーから削除されるため、ウェブメールやスマートフォンからも消えます。
   *[other] { $kind ->
        [conversation] このアカウントにはゴミ箱がないため、{ $count } 件のスレッドは完全に削除されます。
       *[message] このアカウントにはゴミ箱がないため、{ $count } 件のメールは完全に削除されます。
    }フォルダはサーバーから削除されるため、ウェブメールやスマートフォンからも消えます。
}
folder-delete-label-body = ラベルが削除されます。メールは「すべてのメール」とほかのラベルに残ります。
folder-delete-confirm = フォルダを削除
folder-delete-label-confirm = ラベルを削除
folder-deleted = フォルダ「{ $name }」を削除しました
label-deleted = ラベル「{ $name }」を削除しました
