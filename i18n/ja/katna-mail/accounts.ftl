# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = フォルダ ペイン
accounts-folder-pane-detail = 左側のペインにどのアカウントのフォルダを表示するか。
accounts-shown-one = 一度に 1 アカウント（アカウント カードで切り替え）
accounts-shown-all = すべてのアカウントを順に表示
accounts-unified = 統合受信トレイ
accounts-unified-switch = すべてのアカウントのメールをまとめて表示
accounts-unified-switch-detail = 「すべてのアカウント」がフォルダ ペインの先頭に表示され、各アカウントの受信トレイや送信済みメールなどを一つのリストにまとめます。その下のアカウントは折りたたまれた状態で始まります。
accounts-row = アカウント
accounts-row-detail = フォルダ ペインとアカウント メニューには、この順序でアカウントが表示されます。最初のアカウントが既定になります。アカウントを削除すると、このパソコン上にある Katna のメールのコピーが削除されます。メールはサーバーに残ります。
accounts-none = アカウントはまだありません。
accounts-pop3-row = サーバー上のメール
accounts-pop3-row-detail = POP3 アカウントはメールをこのパソコンにダウンロードします。その後サーバー上のコピーをどうするか選んでください。
accounts-pop3-with-katna = Katna で削除するまで残す
accounts-pop3-at-once = ダウンロードしたらすぐ削除
accounts-pop3-after-days = { $count ->
   *[other] { $count } 日後に削除
}
accounts-pop3-never = 削除しない
accounts-pop3-days-less = 日数を減らす
accounts-pop3-days-more = 日数を増やす
accounts-kind-imported = インポート
accounts-picture-reset = デスクトップの画像を使用
accounts-picture-change = 画像を変更
accounts-picture-remove = 画像を削除
account-color-red = レッド
account-color-pink = ピンク
account-color-magenta = マゼンタ
account-color-brown = ブラウン
account-color-olive = オリーブ
account-color-teal = ティール
account-color-indigo = インディゴ
account-color-slate = スレート
account-color-menu = 色
accounts-rename = 名前を変更
accounts-name-save = 保存
accounts-name-cancel = キャンセル
accounts-name-placeholder = あなたの名前
accounts-rename-failed = アカウントの名前を変更できませんでした: { $error }
accounts-move-up = 上へ移動
accounts-move-down = 下へ移動
accounts-drag = ドラッグして順序を変更
accounts-remove = 削除
accounts-delete-all-row = すべてのデータを削除
accounts-delete-all-row-detail = 新しくインストールしたときの状態からやり直します。
accounts-delete-all-about = すべてのアカウント、保存されているすべてのメール、連絡先、カレンダー、検索インデックス、設定、保存したパスワードをこのパソコンから削除します。メールサーバー上では何も変わりません。
accounts-delete-all-open = Katna のデータをすべて削除

## Settings > Accounts: snackbars after deleting

accounts-removed-local = { $address } を Katna から削除しました。
accounts-removed = { $address } を Katna から削除しました。メールはサーバーに残っています。
accounts-all-deleted = Katna のデータをすべてこのパソコンから削除しました。

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = { $address } を削除しますか？
accounts-remove-confirm = アカウントを削除
accounts-removing = 削除しています…
accounts-remove-local-mail = { $folders ->
    [0] このアカウントにインポートしたすべてのメール
   *[other] このアカウントの { $folders } 個のフォルダにインポートしたすべてのメール
}
accounts-remove-local-settings = このアカウントの Katna の設定
accounts-remove-mail = { $folders ->
    [0] Katna が保存しているこのアカウントのすべてのメール
   *[other] Katna が { $folders } 個のフォルダに保存しているこのアカウントのすべてのメール
}
accounts-remove-outbox = 送信トレイで送信を待っているメール
accounts-remove-settings = 保存したパスワードと Katna の設定
accounts-delete-all-title = Katna のデータをすべて削除しますか？
accounts-delete-all-confirm = すべて削除
accounts-deleting = 削除しています…
accounts-delete-all-accounts = すべてのアカウントと、Katna が保存しているすべてのメールと添付ファイル
accounts-delete-all-contacts = 連絡先、カレンダー、検索インデックス
accounts-delete-all-settings = すべての設定、署名、キーボード ショートカット
accounts-delete-all-passwords = 保存したすべてのパスワード
accounts-deleted-heading = このパソコンから削除されるもの:
accounts-cannot-undo = この操作は元に戻せません。
accounts-server-delete-all = メールサーバー上では何も変わりません。メールはサーバーに残り、アカウントを追加し直すと再びダウンロードされます。ファイルからインポートしたメールは Katna の中にしかありません。元のファイルには触れません。
accounts-server-local = このメールはファイルからインポートしたもので、Katna にしかコピーがありません。元のファイルには触れないので、もう一度インポートすれば元に戻せます。
accounts-server-remove = メールサーバー上では何も変わりません。メールはサーバーに残り、アカウントを追加し直すと再びダウンロードされます。
accounts-confirm-word = 削除
accounts-confirm-placeholder = 「{ accounts-confirm-word }」と入力
accounts-confirm-prompt = 確認のため「{ accounts-confirm-word }」と入力してください:
accounts-cancel = キャンセル

## Reset cache (Settings > General), in the same dialog

reset-cache-about = Katna がダウンロードしたメールと添付ファイル、送信者の画像、検索インデックスを削除し、最近のメールをダウンロードし直します。アカウント、設定、このパソコンにしかないメールは残ります。
reset-cache-button = キャッシュをリセット
reset-cache-title = キャッシュをリセットしますか？
reset-cache-deleted = 削除してからダウンロードし直すもの:
reset-cache-mail = IMAP サーバーからダウンロードしたメールと添付ファイル: 最近のメールはすぐに、それより古いメールは開いたときにダウンロードし直します
reset-cache-index = 検索インデックス (すぐに再構築します)
reset-cache-pictures = 送信者の画像
reset-cache-kept = 残るもの: アカウント、パスワード、設定、スター、ラベル、既読の状態、固定、下書き、送信トレイ、まだサーバーに反映されていない変更、POP3 アカウントやインポートしたファイルのメール (ほかにコピーがない場合があります)。メールサーバー上では何も変わりません。
reset-cache-confirm = キャッシュをリセット
reset-cache-busy = リセットしています…
reset-cache-done = キャッシュをリセットしました。最近のメールをダウンロードし直しています。
reset-cache-done-freed = キャッシュをリセットし、{ $size } を解放しました。最近のメールをダウンロードし直しています。
