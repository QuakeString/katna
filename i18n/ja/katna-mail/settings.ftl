# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
settings-appearance-theme-system = システム
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
settings-default-apps-documents-detail = Word（docx、doc）、OpenDocument テキスト（odt）、スライド（pptx、ppt、odp）。
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
settings-appearance-theme-summary = システム、ライト、ダーク
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
settings-default-apps-documents-summary = Word、OpenDocument テキスト、スライドを開くアプリ
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

## Settings: opening at login

settings-open-at-login-failed = ログイン時に開く設定を変更できませんでした: { $error }

## Settings > General > Time

settings-time = 時刻
settings-clock-language = 言語の表記に従う
settings-clock-12 = 12時間制（例: 午後2:05）
settings-clock-24 = 24時間制（例: 14:05）
settings-time-summary = 12時間制、24時間制、または言語の表記に従う

## Settings > General > Default mail app, Settings > Compose > Grammar

settings-general-mail-app = デフォルトのメールアプリ
settings-general-mail-app-detail = ほかのアプリやウェブサイトのメールリンクから、ここで新規メールが開きます。
mail-app-is-default = Katna Mail がデフォルトのメールアプリです。
mail-app-is-other = メールリンクはほかのアプリで開きます。
mail-app-make-default = デフォルトにする
mail-app-make-default-failed = デフォルトのメールアプリを変更できませんでした。
settings-general-mail-app-summary = ほかのアプリやウェブサイトのメールリンクを Katna Mail で開く
settings-compose-grammar = 文法
settings-compose-grammar-detail = このパソコン上で Harper がチェックします。現在は英語のみに対応し、ほかの言語のテキストはそのままです。
settings-compose-grammar-check = 文法をチェック
settings-compose-grammar-check-detail = 入力中に文法の誤りに下線を引く（英語）
settings-compose-grammar-summary = 入力中に文法の誤りに下線を引く（英語）
