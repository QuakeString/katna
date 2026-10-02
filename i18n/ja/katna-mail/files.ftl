# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = ファイルを検索

## Left side (and chips on a phone)

files-all = すべてのファイル
files-pictures = 画像
files-pdfs = PDF
files-documents = ドキュメント
files-sheets = スプレッドシート
files-slides = スライド
files-other = その他
files-accounts = アカウント
files-drives = ドライブ
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = 共有アイテム
files-shown = 表示
files-received = 受信
files-sent = 自分が送信

## Over the files

files-count = { $count ->
   *[other] { $count } 件のファイル · { $size }
}
files-anyone = すべての人
files-from-person = { $name } から
files-time-any = 期間指定なし
files-time-today = 今日
files-time-yesterday = 昨日
files-time-this-week = 今週
files-time-last-week = 先週
files-time-this-month = 今月
files-time-last-month = 先月
files-time-between = { $first } – { $last }
files-time-hint = 日付をクリックするか、ドラッグして複数の日を選択
files-time-summary = { $count ->
   *[other] { $days } · { $count } 件のファイル
}
files-time-clear = クリア
files-time-month-back = 前の月
files-time-month-on = 次の月
files-time-wheel = スクロールすると、期間の長さを保ったまま日付を移動します
files-sort-newest = 新しい順
files-sort-oldest = 古い順
files-sort-largest = サイズの大きい順
files-sort-name = 名前順
files-grid = カード
files-list = リスト
files-this-week = 今週
files-undated = 日付なし
files-me = 自分
files-no-subject = （件名なし）
files-loading = メールからファイルを集めています…
files-empty = メールのファイルがここに表示されます。
files-none-match = 一致するファイルはありません。
files-load-failed = ファイルを読み込めませんでした: { $error }

## A file's menu and buttons

files-open = 開く
files-open-with = アプリで開く…
files-save = 保存…
files-show-mail = メールを表示
files-mail-window = 新しいウィンドウでメールを開く
files-forward = ファイルを転送
files-from-them = { $name } からのファイル
files-copy-name = ファイル名をコピー
files-name-copied = ファイル名をコピーしました
files-downloading = メールをダウンロードしています…
files-download-failed = このメールをダウンロードできませんでした。

## A cloud drive in place of the mail files

files-drive-mine = マイドライブ
files-drive-mine-onedrive = 自分のファイル
files-drive-results = 「{ $words }」
files-drive-count = { $folders ->
    [0] { $files } 件のファイル
   *[other] { $folders } 個のフォルダ · { $files } 件のファイル
}
files-drive-folders = フォルダ
files-drive-files = ファイル
files-drive-folder = フォルダ
files-drive-meta = { $what } · { $date } に編集
files-drive-as-link = { $what } · リンクとして
files-drive-google-doc = Google ドキュメント
files-drive-google-sheet = Google スプレッドシート
files-drive-google-slides = Google スライド
files-drive-google-drawing = Google 図形描画
files-drive-fetching = 取得しています…
files-drive-loading = ドライブを開いています…
files-drive-empty = このフォルダは空です。
files-drive-unreachable = { $drive } に接続できません。
files-drive-try-again = 再試行
files-drive-needs-permission = このドライブを表示するには、一度だけ許可が必要です。もう一度サインインして、Katna によるファイルの表示を許可してください。
files-drive-allow = 許可
files-drive-allow-failed = サインインが完了しなかったため、ドライブは開けません。
files-drive-attach = 添付
files-drive-more = その他
files-drive-download = ダウンロード…
files-drive-open-web = { $drive } で開く
files-drive-copy-link = リンクをコピー
files-drive-link-copied = リンクをコピーしました
files-drive-share = 共有…
files-drive-rename = 名前を変更
files-drive-trash = ゴミ箱に移動
files-drive-trashed = 「{ $name }」を { $drive } のゴミ箱に移動しました
files-drive-renamed = 名前を「{ $name }」に変更しました
files-drive-getting = { $drive } から { $name } を取得しています…
files-drive-get-failed = { $name } を取得できませんでした: { $error }
files-drive-upload = アップロード
files-drive-upload-files = ファイルをアップロード
files-drive-upload-folder = フォルダをアップロード
files-drive-upload-failed = { $name } をアップロードできませんでした: { $error }
files-drive-upload-needs = アップロードするには一度だけ許可が必要です。「設定 > デフォルトのアプリ > ファイルページ」で「許可」を押してください。

## The Share dialog of a drive file or folder

files-share-title = 「{ $name }」を共有
files-share-add = 名前またはアドレスでユーザーを追加
files-share-not-address = 「{ $text }」はメールアドレスではありません
files-share-notify = { $drive } からもメールで通知する
files-share-people = アクセスできるユーザー
files-share-general = 一般的なアクセス
files-share-loading = アクセスできるユーザーを確認しています…
files-share-restricted = 制限付き
files-share-restricted-about = アクセス権のあるユーザーのみ、リンクから開くことができます
files-share-anyone = リンクを知っている全員
files-share-anyone-can = { $role ->
    [editor] リンクを知っている全員が編集できます
    [commenter] リンクを知っている全員がコメントできます
   *[viewer] リンクを知っている全員が閲覧できます
}
files-share-anyone-about = { $role ->
    [editor] インターネット上でリンクを知っている全員が編集できます
    [commenter] インターネット上でリンクを知っている全員がコメントできます
   *[viewer] インターネット上でリンクを知っている全員が閲覧できます
}
files-share-role-owner = オーナー
files-share-role-editor = 編集者
files-share-role-commenter = 閲覧者（コメント可）
files-share-role-viewer = 閲覧者
files-share-you = { $name }（自分）
files-share-domain = { $domain } の全員
files-share-inherited = 親フォルダからのアクセス
files-share-remove = アクセス権を削除
files-share-copy-link = リンクをコピー
files-share-share = 共有
files-share-done = 完了
files-share-close = 閉じる
files-share-sharing = 共有しています…
files-share-shared = { $count ->
   *[other] { $count } 人と共有しました
}
files-share-refused = { $drive } で { $addresses } と共有できませんでした
files-share-failed = 共有設定を変更できませんでした: { $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
   *[other] { $count } 件のアイテムをアップロード中
}
files-tray-done = { $count ->
   *[other] { $count } 件のアップロードが完了しました
}
files-tray-some-failed = { $done } 件アップロード、{ $failed } 件失敗
files-tray-minutes-left = { $minutes ->
   *[other] 残り約 { $minutes } 分
}
files-tray-seconds-left = 残り 1 分未満
files-tray-starting = 開始しています…
files-tray-cancel-all = すべてキャンセル
files-tray-cancel = キャンセル
files-tray-fold = リストを非表示
files-tray-unfold = リストを表示
files-tray-close = 閉じる
files-tray-progress = { $place } · { $size } 中 { $sent }
files-tray-in = { $place } 内
files-tray-cancelled = キャンセルしました
