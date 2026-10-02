# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = Katna について
about-tagline = Linux デスクトップのためのメールとカレンダー
about-whats-new = 新機能

## Updates, in a box under the version in About (only in packages that
## update themselves). $version is a version such as 0.0.0.r236.g1a2b3c4.

about-update-not-checked = 更新はまだ確認されていません
about-update-checking = 更新を確認中…
about-update-up-to-date = Katna Mail は最新の状態です
about-update-check-failed = 更新を確認できませんでした
about-update-available = バージョン { $version } が利用できます
about-update-downloading = バージョン { $version } をダウンロード中… { $percent }%
about-update-download-failed = バージョン { $version } のダウンロードが完了しませんでした
about-update-ready = バージョン { $version } のインストール準備ができました
about-update-ready-detail = 更新を完了するため、Katna Mail が再起動します。
about-update-confirm = バージョン { $version } をインストールしますか？
about-update-confirm-detail = Katna Mail が終了し、更新をインストールしてから、元の状態で再び開きます。パソコンがパスワードを求めます。
about-update-installing = バージョン { $version } をインストール中…
about-update-installing-detail = 開いたウィンドウにパスワードを入力してください。
about-update-cancelled = パスワードが入力されなかったため、更新はインストールされませんでした。
about-update-failed = 更新をインストールできませんでした: { $error }
about-update-unsupported = このコピーの Katna Mail は、お使いのパッケージマネージャーによって更新されます。
about-update-restart-failed = 更新はインストールされましたが、Katna Mail を再び開けませんでした（{ $error }）。手動で開いてください。
about-update-check = 更新を確認
about-update-download = ダウンロード
about-update-retry = もう一度試す
about-update-button = 更新
about-update-restart = 更新して再起動
about-update-cancel = 今はしない
about-changelog = 変更履歴
about-source = ソースコード
about-coffee = コーヒーをおごる
about-coffee-coffee = コーヒー？
about-coffee-tea = お茶？
about-coffee-pizza = ピザ？
about-coffee-nothing = 何も？ 全然？
about-coffee-water = 水でがんばる！！
about-coffee-thanks = Katna を使ってくれてありがとう
about-coming-soon = 近日公開
about-follow-me = フォローはこちら:
about-love-title = Rust、KDE、Linux への愛を込めて
about-love-text = Rust のおかげで、速くて安全なメールアプリを楽しく書けます。Katna には unsafe コードがありません。KDE の Plasma デスクトップとその PIM スイートが Katna のお手本となり、Linux とフリーソフトウェアのコミュニティが Katna の土台を築いています。ありがとうございます。そして、以下のライブラリにも感謝します。
about-kde-text = KDE は Katna が最もなじむデスクトップを作っています。それはボランティアによって作られ、あなたのような人々の支援で成り立っています。Plasma や KDE のアプリを気に入っていただけたら、KDE への寄付をご検討ください。
about-donate-kde = KDE に寄付
about-gpui-title = Zed プロジェクトの GPUI で構築
about-gpui-text = Katna Mail のインターフェースはすべて、Zed Industries が Zed エディタのために作った、GPU で高速化された高速な UI フレームワーク GPUI の上に構築されています。目にするすべてのピクセル、アニメーション、ウィンドウは GPUI が描画しています。オープンに開発してくれた Zed チームに感謝します。Apache-2.0。
about-gpui-github = GitHub の GPUI
about-personal-title = 個人プロジェクト
about-personal-text = Katna Mail は、新しさや革新性を目指してはいません。作者が欲しかったメールアプリであり、機能や見た目は Gmail、Mailspring、Thunderbird から借りています。LLM がここまで進歩したからこそ実現できました。
about-built-on = フリーソフトウェアで構築
about-credit-pimalaya = IMAP、SMTP、サインイン（io-imap、io-smtp、io-sasl）
about-credit-imap-codec = IMAP の読み書き
about-credit-tantivy = 検索
about-credit-sqlite = メールの保存
about-credit-rustls = 安全な接続
about-credit-mail-parser = メールの読み取り（Stalwart Labs）
about-credit-html5ever = HTML メール（Servo プロジェクト）
about-credit-zbus = D-Bus とポータルによるデスクトップとの連携
about-credit-oo7 = デスクトップのキーリングへのパスワード保存
about-credit-hayro = PDF の表示と印刷
about-credit-calamine = スプレッドシートのプレビュー
about-credit-resvg = SVG 画像
about-credit-jiff = 日付とタイムゾーン
about-credit-spellbook = スペルチェック（Helix エディタ）
about-credit-smol = 多くの処理の同時実行
about-all-libraries = Katna が使うすべてのライブラリ（{ $count }）
about-library-authors = 作者: { $authors }
about-license = Katna は GNU GPL バージョン 3 以降のもとで配布されるフリーソフトウェアです。
about-close = 閉じる

## What’s new (shown after an update)

whats-new-title = Katna Mail の新機能
whats-new-updated = バージョン { $version } に更新しました
whats-new-version = バージョン { $version }
whats-new-more = ほかに { $count } 件の変更が完全な変更履歴にあります。
whats-new-changelog = 完全な変更履歴
whats-new-got-it = OK

## First run: welcome page

onboarding-welcome-title = Katna Mail へようこそ
onboarding-welcome-lead = メールをあなたのパソコンに。すばやく検索でき、オフラインでも読めて、プライバシーも守られます。
onboarding-fast-title = オフラインでも高速
onboarding-fast-text = Katna はメールのコピーをこのパソコンに保存するので、接続の有無にかかわらず、開くのも検索するのも一瞬です。
onboarding-providers-title = お使いのメールに対応
onboarding-providers-text = Gmail、Outlook、Yahoo、iCloud のほか、IMAP や POP のアカウントならどれでも使えます。
onboarding-private-title = プライベート
onboarding-private-text = メールはプロバイダーからこのパソコンへ直接届きます。Katna のサーバーがメールを見ることはありません。
onboarding-get-started = はじめる

## First run: adding an account

onboarding-service-checking = Katna のバックグラウンド サービスを確認しています…
onboarding-service-running = Katna のバックグラウンド サービスは実行中です。
onboarding-service-missing = Katna のバックグラウンド サービスが実行されていません
onboarding-service-start = このサービスがメールの受信と送信を行います。ターミナルから起動してから、もう一度確認してください:
onboarding-check-again = もう一度確認
onboarding-account-title = メールアカウントを追加
onboarding-account-lead = メールアドレスとパスワードを入力すると、Katna がサーバー設定を見つけます。Gmail、Yahoo、iCloud では、アカウントのセキュリティ設定で作成するアプリ パスワードが必要です。
onboarding-add-account = アカウントを追加
onboarding-back = 戻る

## First run: choosing the look

onboarding-look-title = 自分好みに
onboarding-look-lead = メールの開き方と Katna の見た目を選びましょう。クイック設定でいつでも変更できます。
onboarding-reading-pane = 閲覧ウィンドウ
onboarding-pane-right = リストの右
onboarding-pane-none = 分割なし
onboarding-theme = テーマ
onboarding-theme-system = システム
onboarding-theme-light = ライト
onboarding-theme-dark = ダーク
onboarding-density = 表示間隔
onboarding-density-default = デフォルト
onboarding-density-compact = コンパクト
onboarding-continue = 続ける

## First run: done

onboarding-ready-title = 準備ができました
onboarding-ready-lead = Katna がメールを取得しています。届いたものから表示され、新しいメールも自動的に表示されます。
onboarding-ready-lead-address = Katna が { $address } のメールを取得しています。届いたものから表示され、新しいメールも自動的に表示されます。
onboarding-ready-tour = 1 分間のツアーで、どこに何があるかご覧になりますか？
onboarding-skip = 今はスキップ
onboarding-take-tour = ツアーを見る

## Asking to send crash reports (on its own and on the first-run pages)

share-title = Katna の改善に協力する
share-lead = Katna がクラッシュすると、このパソコンにレポートが保存されます。これらのレポートを送信すると、問題の修正に役立ちます。この設定は、設定 > フィードバックでいつでも変更できます。
share-sent = 送信される内容
share-sent-detail = 設定で確認できるとおりのクラッシュレポート: クラッシュした内容と Katna 内の場所、バージョン、お使いの Linux システムとデスクトップ、そして Katna のログの最後の数行（メールフォルダ名が含まれることがあります）。
share-never-sent = 送信されないもの
share-never-sent-detail = メッセージ、連絡先、パスワード、IP アドレス、ユーザー名、コンピュータ名。メールアドレスはレポートから削除されます。
share-where = 送信先
share-where-detail = Sentry 上の Katna のクラッシュトラッカー（EU 内に保存）。レポートとあなたを結びつける ID はありません。
share-dont-send = 送信しない
share-send = クラッシュレポートを送信
share-sending = クラッシュレポートを送信します。ありがとうございます。
share-local = クラッシュレポートはこのパソコンに保存されたままになります。

## The tour (cards pointing at each part of the window)

tour-welcome-title = Katna Mail へようこそ
tour-welcome-text = 1 分間のツアーで、どこに何があるかご紹介します。
tour-not-now = 後で
tour-start = ツアーを見る
tour-close = 閉じる
tour-skip = ツアーをスキップ
tour-back = 戻る
tour-done = 完了
tour-next = 次へ
tour-step = { $step } / { $total }
tour-compose-title = メッセージを書く
tour-compose-text = 作成ボタンを押すと、新しいメッセージが右下に開くので、読みながら書けます。
tour-search-title = すべてのメールを検索
tour-search-text = 検索はオフラインでも使えます。右端のボタンで、差出人、宛先、件名、日付、添付ファイルのフィルタを追加できます。
tour-menu-title = フォルダの表示と非表示
tour-menu-text = このボタンでフォルダ一覧を折りたたみます。非表示のときは、左側のメールにポインタを合わせるとフォルダが表示されます。
tour-apps-title = アプリ
tour-tabs-title = 受信トレイのタブ
tour-tabs-text = 新しいメールは、メイン、プロモーション、ソーシャル、新着、フォーラムに振り分けられます。タブはクイック設定でオフにできます。
tour-list-title = メッセージ
tour-list-text = メッセージをクリックすると読めます。ポインタを合わせるとクイック操作、右クリックでその他の操作ができ、複数にチェックを入れるとまとめて操作できます。
tour-settings-title = クイック設定
tour-settings-text = 閲覧ウィンドウ、表示間隔、テーマはここで変更できます。ツアーもここからもう一度始められます。
tour-account-title = アカウント
tour-account-text = 今どのアカウントを見ているかを確認したり、別のアカウントを追加したりできます。

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Katna のバックグラウンド サービスが予期せず停止しました。
   *[other] Katna のバックグラウンド サービスが予期せず停止しました。ほかに { $more } 件のクラッシュレポートが保存されています。
}
crash-mail = { $more ->
    [0] 前回、Katna Mail が予期せず終了しました。
   *[other] 前回、Katna Mail が予期せず終了しました。ほかに { $more } 件のクラッシュレポートが保存されています。
}
crash-view = レポートを表示
crash-view-tooltip = このパソコンに保存されたレポートを開く
crash-copy = レポートをコピー
crash-close = 閉じる
sign-in-again-text = { $provider } から { $address } への再サインインを求められています。
sign-in-again-button = サインイン
sign-in-again-tooltip = ブラウザーで { $provider } のサインイン ページを開く
sign-in-again-waiting = ブラウザーを待っています…
sign-in-again-close = 閉じる
sign-in-again-done = { $address } に再度サインインしました。メールを取得しています…
delete-ask-title = { $kind ->
    [conversation] { $count } 件のスレッドをゴミ箱に移動しますか？
   *[message] { $count } 件のメールをゴミ箱に移動しますか？
}
delete-ask-body = { $count ->
   *[other] 直後なら元に戻せます。あとからゴミ箱に取りに行くこともできます。
}
delete-ask-confirm = ゴミ箱に移動
delete-forever-title = { $kind ->
    [conversation] { $count } 件のスレッドを完全に削除しますか？
   *[message] { $count } 件のメールを完全に削除しますか？
}
delete-forever-body = { $count ->
   *[other] サーバー上でも削除されます。元に戻すことはできません。
}
delete-forever-confirm = 完全に削除
delete-ask-dont-ask = 次回から確認しない
delete-ask-cancel = キャンセル
