# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = メールアカウントを追加
add-account-providers-intro = メールプロバイダーを選んでください。あとは Katna が見つけます。
add-account-provider-other = その他のメール
add-account-provider-other-detail = IMAP または POP3 のアカウント
add-account-provider-google-detail = Gmail と Google Workspace
add-account-provider-microsoft-detail = Outlook と Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = { $provider } にサインイン
add-account-form-title-other = メールアカウント
add-account-form-intro = Katna はパスワードをシステムのキーリングに保存します。
add-account-looking = { $address } のメールサーバーを探しています…
add-account-address-intro = メールアドレスを入力してください。Katna がサーバーを見つけます。
add-account-servers-title = サーバー設定
add-account-servers-intro = Katna が { $address } のメールを読み書きするサーバーです。
add-account-signing-in = サインインしています…
add-account-browser-title = ブラウザーで続行
add-account-browser-intro = Katna がブラウザーで { $provider } のサインイン ページを開きました。そこでサインインし、Katna にメールの読み取りと送信を許可してから、ここに戻ってください。
add-account-browser-hint = ページが開きませんか？ブラウザーのウィンドウを確認するか、戻ってもう一度お試しください。
add-account-stage-browser = ブラウザーでのサインインを待っています…
add-account-stage-signing-in-at = { $server } にサインインしています…
add-account-help-app-password-link = アプリ パスワードの作成方法
add-account-help-turn-on-imap = { $provider } では、ウェブメールの設定で IMAP と POP3 のアクセスをオンにしないと、メールアプリから接続できません。
add-account-help-turn-on-imap-link = オンにする方法

## Add a mail account: fields

add-account-field-address = メールアドレス
add-account-receive-with = メールの受信方法
add-account-imap-about = IMAP はメールとフォルダをサーバー上に保存し、どのデバイスでも同じ状態にします。できるだけこちらを選んでください。
add-account-pop3-about = POP3 はメールをこのパソコンにダウンロードします。ここで読んだり移動したりしても、サーバーやほかのデバイス上のメールはそのままです。
add-account-incoming = 受信メール（{ $protocol }）
add-account-outgoing = 送信メール（{ $protocol }）
add-account-field-server = サーバー
add-account-field-port = ポート
add-account-security-none = なし
add-account-security-none-warning = 暗号化なし: パスワードとメールが通信中に読み取られる可能性があります。
add-account-field-username = ユーザー名
add-account-field-password = パスワード
add-account-show-password = パスワードを表示
add-account-app-password-hint = { $provider } では、ウェブで使うパスワードではなく、アプリ パスワードが必要です。{ $provider } アカウントのセキュリティ設定で作成してください。
add-account-field-name = 名前（省略可）
add-account-name-hint = メールの送信相手に表示されます。
add-account-servers-pair = { $imap } と { $smtp }
add-account-servers-found = { $source ->
    [built-in] サーバー: { $servers }（Katna のプロバイダー一覧で見つかりました）
    [provider] サーバー: { $servers }（プロバイダーの設定で見つかりました）
    [ispdb] サーバー: { $servers }（Thunderbird のプロバイダー一覧で見つかりました）
    [dns] サーバー: { $servers }（ドメインの DNS レコードで見つかりました）
   *[other] サーバー: { $servers }（推測です。サインインできない場合は確認してください）
}
add-account-servers-entered = サーバー: { $servers }（入力どおり）

## Add a mail account: buttons

add-account-sign-in-with = { $provider } でサインイン
add-account-sign-in-instead = 代わりに { $provider } でサインイン
add-account-servers-button = サーバー設定
add-account-back = 戻る
add-account-add = アカウントを追加
add-account-done = 完了
add-account-another = 別のアカウントを追加
add-account-cancel = キャンセル

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] 受信サーバーを入力してください。
   *[outgoing] 送信サーバーを入力してください。
}
add-account-server-space = { $kind ->
    [incoming] 受信サーバー名にスペースが含まれています。
   *[outgoing] 送信サーバー名にスペースが含まれています。
}
add-account-port-invalid = { $kind ->
    [incoming] 受信ポートは { $min } から { $max } までの数値にしてください。
   *[outgoing] 送信ポートは { $min } から { $max } までの数値にしてください。
}
add-account-address-empty = メールアドレスを入力してください。
add-account-address-invalid = { $example } のようなメールアドレスを入力してください。
add-account-not-found = { $address } のサーバーが見つからなかったため、Katna は一般的な名前を入力しました。プロバイダーに確認してください。
add-account-password-empty = パスワードを入力してください。
add-account-name-is-password = 名前がパスワードと同じです。ここには、相手に表示される名前を入力してください。
add-account-app-password-refused = { $provider } がパスワードを拒否しました。ウェブで使うパスワードではなく、アプリ パスワードが必要です。
add-account-password-refused = サーバーがパスワードを拒否しました。確認して、もう一度お試しください。
add-account-sign-in-refused = { $provider } が Katna のアクセスを許可しませんでした。もう一度試して、メールへのアクセスを許可してください。
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] この Katna では、まだ Microsoft アカウントにサインインできません。
    [Google] この Katna では、まだ Google アカウントにサインインできません。
   *[other] このプロバイダーは自社のページでのサインインしか認めていませんが、Katna はまだこれに対応していません。
}
add-account-smtp-not-found = Katna はメールの受信先は見つけましたが、送信先が見つかりませんでした。送信サーバーを入力してください。

## Add a mail account: the last step

add-account-done-title = アカウントの準備ができました
add-account-done-intro = Katna がメールを取得しています。新しいメールは届きしだい表示されます。
add-account-done-sign-in = サインイン
add-account-done-signed-in-with = ブラウザーで { $provider } を使用
add-account-done-receiving = メールの受信
add-account-done-sending = メールの送信
add-account-done-on-server = サーバー上のメール
add-account-done-kept = Katna で削除するまで保持
add-account-done-pop3-hint = サーバー上のメールの扱いは「設定 > アカウント」で変更できます。
add-account-done-zoho-title = タスクとカレンダー
add-account-done-zoho-about = Zoho ではこれらがメールとは別になっています。Zoho で一度サインインすると Katna に取り込めます。
add-account-done-linked = タスクとカレンダーを接続しました

## The account menu (from the account button on the top bar)

add-account-menu-another = 別のアカウントを追加
app-menu = メイン メニュー
app-menu-back = 戻る
