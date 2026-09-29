# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = メールアカウントを追加
add-account-looking = { $address } のメールサーバーを探しています…
add-account-address-intro = メールアドレスを入力してください。Katna がサーバーを見つけます。
add-account-servers-title = サーバー設定
add-account-servers-intro = Katna が { $address } のメールを読み書きするサーバーです。
add-account-password-title = パスワードを入力
add-account-signing-in = サインインしています…
add-account-browser-title = ブラウザーで続行
add-account-browser-intro = Katna がブラウザーで { $provider } のサインイン ページを開きました。そこでサインインし、Katna にメールの読み取りと送信を許可してから、ここに戻ってください。
add-account-browser-hint = ページが開きませんか？ブラウザーのウィンドウを確認するか、戻ってもう一度お試しください。

## Add a mail account: fields

add-account-field-address = メールアドレス
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
add-account-or = または
add-account-sign-in-with = { $provider } でサインイン
add-account-sign-in-instead = 代わりに { $provider } でサインイン

## Add a mail account: buttons

add-account-servers-button = サーバー設定
add-account-back = 戻る
add-account-add = アカウントを追加
add-account-next = 次へ
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
add-account-added = { $address } を追加しました。メールを取得しています…
add-account-app-password-refused = { $provider } がパスワードを拒否しました。ウェブで使うパスワードではなく、アプリ パスワードが必要です。
add-account-password-refused = サーバーがパスワードを拒否しました。確認して、もう一度お試しください。
add-account-sign-in-refused = { $provider } が Katna のアクセスを許可しませんでした。もう一度試して、メールへのアクセスを許可してください。
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] この Katna では、まだ Microsoft アカウントにサインインできません。
    [Google] この Katna では、まだ Google アカウントにサインインできません。
   *[other] このプロバイダーは自社のページでのサインインしか認めていませんが、Katna はまだこれに対応していません。
}
add-account-signed-in = { $provider } でサインインしました。メールを取得しています…

## The account menu (from the account button on the top bar)

add-account-menu-another = 別のアカウントを追加
add-account-menu-manage = アカウントを管理
app-menu = メイン メニュー
app-menu-back = 戻る
