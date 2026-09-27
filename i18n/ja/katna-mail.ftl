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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = 添付ファイルを見るには、このメールを開いてください。
text-copy = コピー
text-select-all = すべて選択

## Settings page: its tabs

settings-tab-general = 全般
settings-tab-inbox = 受信トレイ
settings-tab-accounts = アカウント
settings-tab-subscriptions = 登録
settings-tab-appearance = 外観
settings-tab-shortcuts = ショートカット
settings-tab-default-apps = デフォルトのアプリ
settings-tab-folders-rules = フォルダとルール
settings-tab-compose = 作成
settings-tab-mcp-server = MCP サーバー
settings-tab-feedback = フィードバック
settings-tab-experimental = 試験運用

## Settings page: tabs still to come

settings-tab-subscriptions-coming = 受け取っているニュースレターやメーリングリストを一覧し、ワンクリックで登録解除できます。
settings-tab-folders-rules-coming = フォルダやラベルの作成、名前の変更、移動、非表示、同期するものの選択ができます。ルールを使うと、新着メールを送信者、件名、キーワードで自動的に分類、ラベル付け、転送、削除できます。
settings-tab-mcp-server-coming = このパソコン上の AI アシスタントが、あなたの許可のもとでメールを検索、閲覧、下書きできるようにします。

## Settings > General

settings-general-conversations = スレッド表示
settings-general-conversations-group = 同じメールへの返信をまとめる
settings-general-conversations-group-detail = リストではスレッドごとに 1 行で表示します
settings-general-reading = 閲覧
settings-general-newest-first = 新しいメールを先頭に表示
settings-general-newest-first-detail = スレッドは最新の返信から始まります
settings-general-full-headers = 詳細なヘッダーを表示
settings-general-full-headers-detail = すべてのメールで From、To、Cc、日付、件名を開いて表示します
settings-general-full-names = 宛先をフルネームで表示
settings-general-full-names-detail = 「To: 自分、Ada」ではなく「To: 自分、Ada Lovelace」
settings-general-mark-read = 既読にするタイミング
settings-general-mark-read-now = 開いたらすぐ
settings-general-mark-read-1s = 開いてから 1 秒後
settings-general-mark-read-3s = 開いてから 3 秒後
settings-general-mark-read-never = 自分で既読にしたときのみ
settings-general-reply-button = 返信ボタン
settings-general-reply-all = 全員に返信
settings-general-reply-all-detail = 各メールの横にある返信ボタンで、送信者だけでなく全員に返信します
settings-general-remote-images = ウェブ上の画像
settings-general-remote-images-detail = メールの画像を読み込むと、あなたがメールを開いたこと、その日時、おおよその場所が送信者に伝わります。オフにすると、メールごとに先に確認します。送信者の画像はいつでも表示できます。
settings-general-remote-images-always = 画像を常に表示
settings-general-remote-images-always-detail = 信頼する送信者だけでなく、すべてのメールで表示します
settings-general-sending = 送信
settings-general-sending-detail = 送信したメールを取り消せるよう、送信を待つ時間です。
settings-general-offline = オフラインのメール
settings-general-offline-detail = 最近のメールは全体をダウンロードし、オフラインでも読めるようにします。それより古いメールは開いたときにダウンロードします。
settings-general-offline-days = { $count } 日
settings-general-offline-years = { $count } 年
settings-general-offline-all = すべてのメール
settings-general-offline-note = 日数を減らしても、ダウンロード済みのメールは残ります。サーバー上では何も変わりません。
settings-general-notifications = 通知
settings-general-notifications-detail = 受信トレイの新着メールについて、Katna Mail を閉じていても通知します。
settings-general-new-mail = 新着メールを通知する
settings-general-new-mail-detail = 「全員に返信」「既読にする」「アーカイブ」ボタン付き
settings-general-new-mail-sound = 通知音を鳴らす
settings-general-new-mail-sound-detail = デスクトップの新着メールの通知音
settings-general-desktop = デスクトップ
settings-general-open-at-login = ログイン時に Katna Mail を開く
settings-general-open-at-login-detail = どちらの場合も、サービスが動作していればログイン時にメールを同期します
settings-general-tray = システムトレイに Katna を表示
settings-general-tray-detail = 未読数とメニュー付き
settings-general-unread-badge = タスクバーのアイコンに未読数を表示
settings-general-unread-badge-detail = 受信トレイの未読メールの数

## Settings > Inbox

settings-inbox-tabs = 受信トレイのタブ
settings-inbox-tabs-detail = メールプロバイダのウェブサイトと同じように、受信トレイをタブに分類します。
settings-inbox-tabs-show = 受信トレイのタブを表示
settings-inbox-tabs-show-detail = オフにすると、アカウントごとに 1 つのリストで表示します
settings-inbox-no-accounts = タブを選ぶには、アカウントを追加してください。
settings-inbox-tabs-automatic = 自動: { $tabs }（{ $provider }）
settings-inbox-tabs-off = タブなし
settings-inbox-tabs-gmail = メイン、プロモーション、ソーシャル、新着、フォーラム
settings-inbox-tabs-focused = 優先とその他
settings-inbox-tabs-zoho = 受信トレイ、ニュースレター、通知
settings-inbox-tabs-shown = 表示するタブ。オフにしたタブのメールは「{ $tab }」に表示されます。

## Settings > Appearance

settings-appearance-reading-pane = 閲覧ウィンドウ
settings-appearance-reading-pane-detail = 開いたスレッドを表示する場所です。
settings-appearance-pane-right = リストの右
settings-appearance-pane-none = 分割なし
settings-appearance-density = 表示間隔
settings-appearance-density-default = デフォルト
settings-appearance-density-compact = コンパクト
settings-appearance-scaling = 拡大/縮小
settings-appearance-scaling-detail = デスクトップ自体の拡大率に加えて、Katna Mail の文字、アイコン、余白、区切り線をすべて大きくまたは小さくします。送信するメールの文字サイズは変わりません。小さくしすぎると、アイコンをクリックしにくくなります。
settings-appearance-theme = テーマ
settings-appearance-theme-system = デスクトップと同じ
settings-appearance-theme-light = ライト
settings-appearance-theme-dark = ダーク
settings-appearance-desktop-colors = デスクトップの色
settings-appearance-desktop-colors-use = デスクトップの色を使用
settings-appearance-desktop-colors-use-detail = デスクトップの配色とアクセントカラー
settings-appearance-app-names = アプリ名
settings-appearance-app-names-show = アプリ名を表示
settings-appearance-app-names-show-detail = 左端のアプリアイコンの下に名前を表示します
settings-appearance-sender-pictures = 送信者の画像
settings-appearance-sender-pictures-show = 会社のロゴを表示
settings-appearance-sender-pictures-show-detail = メールごとではなく送信者のドメインで検索し、1 週間保存します
settings-appearance-important = 重要マーク
settings-appearance-important-show = 重要マークを表示
settings-appearance-important-show-detail = リストの各メールの横に表示します
settings-appearance-message-width = メールの幅
settings-appearance-message-width-limit = メールの幅を制限
settings-appearance-message-width-limit-detail = 広いウィンドウでは長い行が読みやすくなります
settings-appearance-mail-colors = メールの色
settings-appearance-mail-colors-detail = ほとんどのメールは白い背景向けにデザインされています。ダークテーマでは、読みやすい暗い色に変えて表示します。オフにすると、明るい背景に送信者の色のまま表示します。
settings-appearance-dark-mail = メールもダークカラーで表示
settings-appearance-dark-mail-detail = ダークテーマのときのみ
settings-appearance-attachment-previews = 添付ファイルのプレビュー
settings-appearance-attachment-previews-show = 添付ファイルのプレビューを表示
settings-appearance-attachment-previews-show-detail = 各ファイルのカードに内容の小さな画像を表示します

## Settings > Default apps

settings-default-apps-intro = 添付ファイルをクリックしたときに開くアプリです。ビューアからはいつでも別のアプリでファイルを開けます。デスクトップのデフォルトのアプリは、デスクトップの設定で変更します。
settings-default-apps-pdf = PDF ファイル
settings-default-apps-pdf-detail = ページ表示、ズーム可能。
settings-default-apps-pictures = 画像
settings-default-apps-pictures-detail = 写真（正しい向きに回転）、PNG、GIF、WebP、BMP、TIFF、SVG。
settings-default-apps-text = テキストファイル
settings-default-apps-text-detail = プレーンテキスト、ログ、コードなどのテキスト。
settings-default-apps-sheets = スプレッドシート
settings-default-apps-sheets-detail = Excel（xlsx、xls）、OpenDocument（ods）、CSV。
settings-default-apps-documents = ドキュメント
settings-default-apps-documents-detail = Word（docx）と OpenDocument テキスト（odt）。
settings-default-apps-katna = Katna Mail のビューア
settings-default-apps-system = デスクトップのデフォルトのアプリ
settings-default-apps-ask = 毎回アプリを選択
settings-default-apps-after-saving = 保存後
settings-default-apps-show-folder = 保存したファイルをフォルダで表示
settings-default-apps-show-folder-detail = 保存した添付ファイルを選択した状態でファイルマネージャを開きます

## Settings > Compose

settings-compose-send-from = 新規メールの送信元
settings-compose-send-from-detail = 返信と転送は、常に表示中のアカウントから送信します。
settings-compose-send-from-current = 表示中のアカウント
settings-compose-send-on-replies = 返信時の送信
settings-compose-send-on-replies-detail = 返信や転送で「送信」ボタンが行う操作です。「送信」の横のメニューからもう一方を選べます。
settings-compose-send-plain = 送信
settings-compose-send-archive = 送信してアーカイブ
settings-compose-signatures = 署名
settings-compose-signatures-detail = メール本文の下、「--」の行の後に追加されます。作成画面で別の署名を選べます。
settings-compose-untitled = 無題
settings-compose-signature-name = 名前（例: 仕事用）
settings-compose-signature-first = 自分の署名
settings-compose-signature-numbered = 署名 { $number }
settings-compose-signature-delete = 削除
settings-compose-signature-deleted = 署名を削除しました
settings-compose-signature-new = 新規作成
settings-compose-no-signatures = 署名はまだありません。
settings-compose-no-signature = 署名なし
settings-compose-for-new-mail = 新規メール用
settings-compose-for-replies = 返信／転送用
settings-compose-for-replies-detail = 自分が署名したメールのあるスレッドでは、返信はその署名で始まります。
settings-compose-format = 形式
settings-compose-plain-text = プレーンテキストで作成
settings-compose-plain-text-detail = 新規メールは書式なしで始まります。作成画面で切り替えられます
settings-compose-spelling = スペル
settings-compose-spell-check = 入力中にスペルをチェック
settings-compose-spell-check-detail = スペルミスに下線が引かれ、右クリックで候補が表示されます
settings-compose-spell-desktop = デスクトップの言語（{ $language }）
settings-compose-templates = テンプレート
settings-compose-templates-detail = よく書くメールを保存し、新規メールや返信に使えます。

## Settings > Shortcuts

settings-shortcuts-set = ショートカットのセット
settings-shortcuts-set-detail = 使い慣れたメールアプリのキー割り当てから始めます。Cmd はここでは Ctrl です。自分で変更したキーはセットより優先され、「デフォルトに戻す」でセットのキーに戻ります。
settings-shortcuts-single = 1 キーのショートカット
settings-shortcuts-single-detail = ウェブメールのように Ctrl や Alt を使わないキーです。e でアーカイブ、j と k で移動、/ で検索。リストと開いたスレッドで使え、入力中は使えません。
settings-shortcuts-single-use = 1 キーのショートカットを使用
settings-shortcuts-single-use-detail = Ctrl のショートカットは常に使えます
settings-shortcuts-how = キーをクリックして変更するか、+ をクリックして追加し、新しいキーを押します。Esc でキャンセルします。
settings-shortcuts-restore = デフォルトに戻す
settings-shortcuts-no-key = キーなし
settings-shortcuts-press = キーを押してください…
settings-shortcuts-then = { $keys } の次に…
settings-shortcuts-moved = { $keys } は「{ $previous }」ではなく「{ $action }」になりました。
settings-shortcuts-single-off = 1 キーのショートカットがオフのため、このキーはオンにすると使えます。
settings-shortcuts-restored = すべてのショートカットをセットのキーに戻しました。

## Settings search: the line under a result

settings-general-language-summary = アプリ、日付、数値の言語
settings-general-reading-summary = 新しいメールを先頭に表示、詳細なヘッダー、宛先のフルネーム
settings-general-mark-read-summary = 開いたスレッドを既読にするタイミング: すぐ、1 秒後または 3 秒後、手動
settings-general-reply-button-summary = 各メールの横にある返信ボタンで全員に返信
settings-general-remote-images-summary = すべてのメールの画像を常に表示
settings-general-sending-summary = 送信取り消し: 送信したメールを取り消せるよう、送信を待つ時間
settings-general-offline-summary = オフラインで読めるよう、最近のメールを何日分ダウンロードするか
settings-general-notifications-summary = 新着メールの通知と通知音
settings-general-desktop-summary = ログイン時に Katna Mail を開く、システムトレイのアイコン、タスクバーのアイコンの未読数
settings-accounts-accounts-summary = アカウントの追加や削除、画像の変更
settings-appearance-density-summary = リストの行をデフォルトまたはコンパクトで表示
settings-appearance-scaling-summary = 文字、アイコン、余白、区切り線をすべて大きくまたは小さくします
settings-appearance-theme-summary = デスクトップと同じ、ライト、ダーク
settings-appearance-sender-pictures-summary = 送信者のドメインで検索した会社のロゴ
settings-appearance-important-summary = リストの各メールの横にある重要マーク
settings-appearance-mail-colors-summary = ダークテーマで HTML メールを暗い色にするか、送信者の色のままにするか
settings-appearance-attachment-previews-summary = 各添付ファイルの内容の小さな画像
settings-shortcuts-set-summary = Gmail、Inbox by Gmail、Apple Mail、Outlook、Thunderbird のキー割り当てから始める
settings-shortcuts-single-summary = ウェブメールのように Ctrl や Alt を使わないキー
settings-default-apps-pdf-summary = PDF の添付ファイルを開くアプリ
settings-default-apps-pictures-summary = 写真や画像を開くアプリ
settings-default-apps-text-summary = プレーンテキスト、ログ、コードを開くアプリ
settings-default-apps-sheets-summary = Excel、OpenDocument、CSV ファイルを開くアプリ
settings-default-apps-documents-summary = Word と OpenDocument テキストを開くアプリ
settings-default-apps-after-saving-summary = 保存した添付ファイルをフォルダで表示
settings-compose-send-from-summary = 新規メールを送信するアカウント: 表示中のアカウントか、常に同じアカウント
settings-compose-send-on-replies-summary = 返信や転送で「送信」か「送信してアーカイブ」か
settings-compose-signatures-summary = メール本文の下、「--」の行の後に追加
settings-compose-for-new-mail-summary = 新規メールに最初から入れる署名
settings-compose-for-replies-summary = 返信や転送に最初から入れる署名
settings-compose-format-summary = 新規メールをプレーンテキストで作成
settings-compose-spelling-summary = 入力中のスペルチェックと辞書の言語
settings-compose-templates-summary = 近日公開: よく書くメールを保存し、新規メールや返信に使えます
settings-feedback-crash-reports-summary = Katna Mail またはバックグラウンド サービスがクラッシュしたときに、クラッシュレポートをこのパソコンに保存
settings-feedback-saved-summary = このパソコンに保存したクラッシュレポートの表示、コピー、削除
settings-feedback-help-improve-summary = 問題の修正に役立てるためクラッシュレポートを送信（オンにしない限りオフ）
settings-experimental-blur-summary = 上部のバーからデスクトップがぼかして透けて見え、メニューはすりガラス風になります
settings-search-shortcut = キーボード ショートカット
settings-search-tab = 設定のタブ
settings-search-none = 「{ $query }」に一致する設定はありません。
settings-search-results = 「{ $query }」に一致する設定

## Quick settings (the panel that slides in from the right)

quick-title = クイック設定
quick-see-all = すべての設定を表示
quick-reading-pane = 閲覧ウィンドウ
quick-pane-right = リストの右
quick-pane-none = 分割なし
quick-density = 表示間隔
quick-density-default = デフォルト
quick-density-compact = コンパクト
quick-theme = テーマ
quick-theme-system = デスクトップと同じ
quick-theme-light = ライト
quick-theme-dark = ダーク
quick-desktop-colors = デスクトップの色
quick-desktop-colors-detail = デスクトップの配色とアクセントカラー
quick-app-names = アプリ名
quick-app-names-detail = 左端のアプリアイコンの下に名前を表示します
quick-inbox-tabs = 受信トレイのタブ
quick-inbox-tabs-detail = 各アカウントのメールプロバイダのタブ
quick-choose-tabs = タブを選択
quick-choose-tabs-detail = 設定でアカウントごとに選択
quick-sending = 送信
quick-undo-send = 送信取り消し
quick-undo-send-off = オフ
quick-undo-send-seconds = { $seconds } 秒
quick-signatures = 署名
quick-signatures-none = まだありません
quick-signatures-one = { $name }（デフォルトで使用）
quick-signatures-many = 署名 { $count } 件、デフォルトは { $name }
quick-signatures-no-default = { $count } 件、デフォルトなし
quick-signature-untitled = 無題
quick-threading = メールのスレッド表示
quick-conversation-view = スレッド表示
quick-conversation-view-detail = 同じメールへの返信をまとめる
quick-help = ヘルプ
quick-tour = ツアーを見る
quick-whats-new = 新機能
quick-about = Katna について

## Settings: opening at login

settings-open-at-login-failed = ログイン時に開く設定を変更できませんでした: { $error }

## Settings > Appearance > Scaling

scale-letter = あ
scale-percent = { $percent }%
scale-reset = { $percent }% に戻す

## Settings > Experimental > Look & Feel

look-intro = 試験中の機能です。変更または廃止される可能性があります。
look-heading = ルック アンド フィール
look-window-frame = ウィンドウ枠
look-window-frame-detail = タイトルバー、ウィンドウボタン、角、影を描画するのはどちらか。
look-frame-native-kde = ネイティブ: KDE の枠（Plasma テーマ）
look-frame-native = ネイティブ: デスクトップの枠
look-frame-katna = Katna: 上部のバーをタイトルバーにする
look-frame-katna-note-named = Katna が角を丸くし、独自の影を描画します。枠は { $desktop } のテーマに従わなくなりますが、ウィンドウルールは引き続き適用されます。
look-frame-katna-note = Katna が角を丸くし、独自の影を描画します。枠はデスクトップのテーマに従わなくなりますが、ウィンドウルールは引き続き適用されます。
look-frame-client-side = このデスクトップでは枠を各アプリに任せているため、Katna はすでに独自の枠を描画しています。
look-blurred-background = 背景のぼかし
look-blurred-background-detail = 上部のバーとフォルダからデスクトップがぼかして透けて見え、メニューとポップオーバーはすりガラス風になります。
look-blur = ウィンドウの背後をぼかす
look-blur-detail = メールは不透明なカードに表示されるので、文字のコントラストは保たれます
look-blur-off-kde = KDE のぼかし効果がオフです。「システム設定」の「ウィンドウ管理」>「デスクトップ効果」で「ぼかし」をオンにしてから、Katna Mail を開き直してください。
look-blur-none-gnome = GNOME はウィンドウの背後をぼかしません。
look-blur-none-x11 = お使いのウィンドウマネージャはウィンドウの背後をぼかしません。
look-blur-none-wayland = お使いのコンポジタはウィンドウの背後をぼかしません。

## Settings > User feedback (crash reports)

feedback-intro-sending = 問題の修正に役立てるため、新しいクラッシュレポートを送信します。それ以外のものがこのパソコンから送られることはありません。
feedback-intro-local = Katna はどこにも何も送信しません。クラッシュレポートはこのパソコンに保存され、内容を確認したりバグ報告に添付したりできます。
feedback-crash-reports = クラッシュレポート
feedback-crash-reports-detail = Katna Mail またはバックグラウンド サービスがクラッシュしたときに作成されます。
feedback-save = クラッシュレポートをこのパソコンに保存
feedback-save-detail = ホームフォルダ、ユーザー名、コンピュータ名、メールアドレスは含まれません
feedback-saved = 保存したクラッシュレポート
feedback-saved-detail = 最新の { $count } 件を保存します。
feedback-help-improve = Katna の改善に協力する
feedback-help-improve-detail = オンにしない限りオフです。ここでいつでもオフにできます。
feedback-send = クラッシュレポートを送信
feedback-send-detail = 保存したレポートを、ここで表示できる内容そのままで Katna のクラッシュトラッカー（Sentry、EU 内）に送ります。IP アドレス、メール、メールアドレスは送りません
feedback-none-saved = 保存したクラッシュレポートはありません。
feedback-delete-all = すべて削除
feedback-app-daemon = バックグラウンド サービス
feedback-report-sent = { $date } · 送信済み
feedback-view = 表示
feedback-view-tooltip = レポートを開く
feedback-copy-tooltip = コピーしてバグ報告に貼り付ける
feedback-copied = クラッシュレポートをコピーしました。
feedback-deleted-all = クラッシュレポートを削除しました。
feedback-read-failed = クラッシュレポートを読み込めませんでした: { $error }
feedback-delete-failed = クラッシュレポートを削除できませんでした: { $error }
feedback-delete-all-failed = クラッシュレポートを削除できませんでした: { $error }

## Menu bar (the KDE global menu)

desktop-menu-file = ファイル(_F)
desktop-menu-new-message = 新規メール(_N)
desktop-menu-quit = 終了(_Q)
desktop-menu-edit = 編集(_E)
desktop-menu-undo = 元に戻す(_U)
desktop-menu-select-all = すべて選択(_A)
desktop-menu-select-none = 選択を解除(_N)
desktop-menu-find = 検索(_F)…
desktop-menu-view = 表示(_V)
desktop-menu-folder-list = フォルダ一覧を表示(_F)
desktop-menu-refresh = 更新(_R)
desktop-menu-go = 移動(_G)
desktop-menu-inbox = 受信トレイ(_I)
desktop-menu-starred = スター付き(_S)
desktop-menu-sent = 送信済み(_E)
desktop-menu-drafts = 下書き(_D)
desktop-menu-all-mail = すべてのメール(_A)
desktop-menu-next = 次のスレッド(_N)
desktop-menu-previous = 前のスレッド(_P)
desktop-menu-message = メール(_M)
desktop-menu-open = 開く(_O)
desktop-menu-reply = 返信(_R)
desktop-menu-reply-all = 全員に返信(_A)
desktop-menu-forward = 転送(_F)
desktop-menu-archive = アーカイブ(_H)
desktop-menu-delete = 削除(_D)
desktop-menu-spam = 迷惑メールを報告(_S)
desktop-menu-move-to = 移動(_M)…
desktop-menu-mark-read = 既読にする(_E)
desktop-menu-mark-unread = 未読にする(_U)
desktop-menu-star = スターを付ける(_T)
desktop-menu-important = 重要マークを付ける(_P)
desktop-menu-not-important = 重要ではないとマーク(_N)
desktop-menu-settings = 設定(_S)
desktop-menu-quick-settings = クイック設定(_Q)
desktop-menu-configure = Katna Mail を設定(_C)…
desktop-menu-help = ヘルプ(_H)
desktop-menu-shortcuts = キーボード ショートカット(_K)
desktop-menu-whats-new = 新機能(_W)
desktop-menu-about = Katna について(_A)

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = 移動
shortcut-group-actions = 操作
shortcut-group-go-to = ジャンプ
shortcut-group-app = アプリケーション

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = 次のスレッド
shortcut-previous = 前のスレッド
shortcut-down = リストで下へ移動
shortcut-up = リストで上へ移動
shortcut-first = リストの先頭へ
shortcut-last = リストの末尾へ
shortcut-page-down = リストを 1 ページ下へ
shortcut-page-up = リストを 1 ページ上へ
shortcut-open = スレッドを開く
shortcut-back = リストに戻る
shortcut-scroll-down = 下へスクロール
shortcut-scroll-up = 上へスクロール
shortcut-scroll-page-down = 1 ページ下へスクロール
shortcut-scroll-page-up = 1 ページ上へスクロール
shortcut-compose = 作成
shortcut-reply = 返信
shortcut-reply-all = 全員に返信
shortcut-forward = 転送
shortcut-archive = アーカイブ
shortcut-delete = 削除
shortcut-spam = 迷惑メールを報告
shortcut-move-to = 移動
shortcut-mark-read = 既読にする
shortcut-mark-unread = 未読にする
shortcut-star = スターを付ける/外す
shortcut-important = 重要マークを付ける
shortcut-not-important = 重要ではないとマーク
shortcut-check = スレッドを選択
shortcut-select-all = すべてのスレッドを選択
shortcut-select-none = すべてのスレッドの選択を解除
shortcut-undo = 最後の操作を元に戻す
shortcut-go-inbox = 受信トレイ
shortcut-go-starred = スター付き
shortcut-go-sent = 送信済み
shortcut-go-drafts = 下書き
shortcut-go-all = すべてのメール
shortcut-search = メールを検索
shortcut-navigation = メニューを表示/折りたたむ
shortcut-quick-settings = クイック設定
shortcut-settings = すべての設定
shortcut-shortcuts = キーボード ショートカット
shortcut-reload = 新着メールを確認
shortcut-quit = 終了

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first } の次に { $second }

## Settings > Accounts

accounts-folder-pane = フォルダ ペイン
accounts-folder-pane-detail = 左側のペインにどのアカウントのフォルダを表示するか。
accounts-shown-one = 一度に 1 アカウント（アカウント カードで切り替え）
accounts-shown-all = すべてのアカウントを順に表示
accounts-row = アカウント
accounts-row-detail = アカウントを削除すると、このパソコン上にある Katna のメールのコピーが削除されます。メールはサーバーに残ります。
accounts-none = アカウントはまだありません。
accounts-kind-imported = インポート
accounts-picture-reset = デスクトップの画像を使用
accounts-picture-change = 画像を変更
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
