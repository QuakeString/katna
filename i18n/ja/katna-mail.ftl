# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = 言語: { $language }
language-tooltip-system = 言語: { $language }（システムに従う）
language-search = 言語を検索
language-system-default = システムのデフォルト
language-system-now = 現在は{ $language }
language-no-match = 「{ $query }」に一致する言語はありません
language-machine = 機械翻訳です。改善にご協力ください
language-setting = 言語
language-setting-detail = メニュー、ボタン、メッセージの言語と、日付や数値の形式です。「システムのデフォルト」ではデスクトップの設定に従います。

## Dates and sizes

ago-just-now = たった今
ago-minutes = { $count } 分前
ago-hours = { $count } 時間前
ago-days = { $count } 日前
size-bytes = { $count } バイト
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = フォルダを非表示
folders-show = フォルダを表示
compose = 作成
search = 検索
search-mail = メールを検索
search-settings = 設定を検索
search-clear = 検索をクリア
search-options-show = 検索オプションを表示
settings = 設定
account-add = アカウントを追加

## App rail (and the bottom bar on a phone)

rail-mail = メール
rail-calendar = カレンダー
rail-contacts = 連絡先
rail-tasks = タスク
rail-notes = メモ
rail-feeds = フィード

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = 近日公開
app-calendar-promise = CalDAV カレンダー、メールで届いた会議の招待、リマインダーを受信トレイのすぐ隣に。
app-tasks-promise = CalDAV と同期する ToDo リストと、メールから作成するタスク。
app-notes-promise = すばやく書けるメモと、あとで見返すためのメールやスレッドへのメモ。
app-feeds-promise = メールの隣で RSS や Atom のフィードを読めます。

## Contacts page

app-contacts-loading = メールから連絡先を集めています…
app-contacts-empty = メールでやり取りした人がここに表示されます。
app-contacts-count = メールでやり取りした { $count } 人（やり取りの多い順）
app-contacts-top = メールでやり取りした上位 { $count } 人（やり取りの多い順）
app-contacts-messages = { $count } 件のメール
app-contacts-last = 最終: { $date }

## Navigation (the folders pane)

nav-labels = ラベル
nav-folders = フォルダ
nav-label-new = 新しいラベルを作成
nav-folder-new = 新しいフォルダを作成
nav-account-unnamed = アカウント { $number }
nav-tab-new = 新着 { $count } 件

## Special folders (the user's own folders keep their names)

folder-inbox = 受信トレイ
folder-starred = スター付き
folder-drafts = 下書き
folder-sent = 送信済み
folder-archive = アーカイブ
folder-spam = 迷惑メール
folder-trash = ゴミ箱
folder-all-mail = すべてのメール
folder-scheduled = 予定

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

## Reading pane: toolbar

reader-close = 閉じる
reader-back = 戻る
reader-mark-unread = 未読にする
reader-move-to = 移動
reader-more = その他
reader-print-all = すべて印刷
reader-new-window = 新しいウィンドウで開く
reader-position = { $position } / { $total }
reader-newer = 新しい
reader-older = 古い

## Reading pane: the conversation

reader-removed = このスレッドは削除されました。
reader-no-subject = （件名なし）
reader-collapse-all = すべて折りたたむ
reader-expand-all = すべて展開
reader-unknown-sender = （不明な送信者）
reader-date-ago = { $date }（{ $ago }）
reader-me = 自分
reader-to = To: { $names }
reader-starred = スター付き
reader-not-starred = スターなし
reader-too-long = メールが長すぎるため、すべてを表示できません。
reader-encrypted-images = 暗号化されたメールでは、ウェブ上の画像は読み込まれません。
reader-window-failed = 新しいウィンドウを開けませんでした。

## Reading pane: message details (opened from "to me")

reader-details-from = 送信元:
reader-details-to = 宛先:
reader-details-cc = Cc:
reader-details-date = 日付:
reader-details-subject = 件名:

## Reading pane: downloading a message

reader-downloading = このメールをサーバーからダウンロードしています…
reader-download-failed = このメールをダウンロードできませんでした。
reader-try-again = 再試行

## Reply row

reply-reply = 返信
reply-reply-all = 全員に返信
reply-forward = 転送

## Encrypted and signed mail

security-decrypting = 復号しています…
security-checking = 署名を確認しています…
security-partly-encrypted = このメールは一部だけが暗号化されています。残りの部分は保護の外で追加されたもので、誰でも追加できた可能性があります。
security-partly-signed = このメールは一部だけが署名されています。残りの部分は保護の外で追加されたもので、誰でも追加できた可能性があります。
security-encrypted = 暗号化されたメール
security-encrypted-smime = 暗号化されたメール（S/MIME）
security-no-key = このメールを復号できません: お持ちでない鍵で暗号化されています。
security-cancelled = 復号がキャンセルされました。
security-damaged = このメールを復号できません: 暗号化されたデータが破損しているか、変更されています。
security-decrypt-unavailable = このメールを復号できません: 暗号化されたメールを読むには { $tool } をインストールしてください。
security-decrypt-failed = このメールを復号できません: { $reason }
security-unknown-signer = 不明な署名者
security-signed-verified = { $signer } による署名 · 検証済み
security-signed-not-sender = { $signer } による署名（送信者ではありません）
security-signed-untrusted = { $signer } による署名（信頼しないとマークした鍵）
security-signed-unverified = { $signer } による署名 · 鍵は未検証です
security-bad-signature = 不正な署名: このメールは署名後に変更されたか、署名が偽造されています。
security-signature-expired = { $signer } による署名 · 署名の有効期限が切れています
security-key-expired = { $signer } による署名 · その後、鍵の有効期限が切れています
security-key-revoked = { $signer } による署名（失効した鍵）
security-missing-key = お持ちでない鍵で署名されているため、確認できません
security-missing-key-id = お持ちでない鍵（{ $key }）で署名されているため、確認できません
security-signature-unavailable = 署名付き。署名を確認するには { $tool } をインストールしてください
security-signature-error = 署名を確認できませんでした。

## Remote images and pictures

remote-hidden = このメールの画像は表示されていません。
remote-show = 画像を表示
remote-always-show = この送信者からの画像を常に表示
remote-picture-use = 使用
remote-picture-too-big = 8 MB 以下の画像を選んでください。
remote-picture-type = PNG、JPEG、GIF、WebP、SVG のいずれかの画像を選んでください。
remote-picture-read-failed = 画像を読み込めません: { $error }
remote-picture-keep-failed = 画像を保存できません: { $error }
remote-picture-remove-failed = 画像を削除できません: { $error }

## Attachments

attachment-count = 添付ファイル { $count } 件
attachment-save = 保存
attachment-save-all = すべて保存
attachment-save-all-tooltip = すべての添付ファイルをフォルダに保存
attachment-save-here = ここに保存
attachment-not-downloaded = このメールはダウンロードされていません。
attachment-not-found = この添付ファイルがメール内に見つかりませんでした。
attachment-read-failed = { $name } を読み込めませんでした
attachment-numbered = 添付ファイル { $number }
attachment-saved-all = { $count } 件のファイルを { $place } に保存しました
attachment-saved-some = { $total } 件中 { $saved } 件のファイルを { $place } に保存しました。{ $failed } を保存できませんでした
attachment-saved-to = { $path } に保存しました
attachment-save-failed = { $name } を保存できませんでした: { $error }
attachment-open-failed = { $name } を開けませんでした: { $error }
attachment-risky = このファイルはプログラムを実行する可能性があるため、Katna では開きません。代わりに保存してください。
attachment-encrypted-open = このファイルは暗号化されて届きました。保存してから別のアプリで開いてください。

## Printing

print-failed = 印刷できませんでした: { $error }
print-no-font = フォントが見つかりませんでした
print-opened-as-pdf = PDF として開きました。そこから印刷してください。
print-not-downloaded = （まだダウンロードされていません。）
print-encrypted = （暗号化されています。本文を印刷するには Katna Mail で開いてください。）
print-to = To: { $addresses }
print-cc = Cc: { $addresses }
