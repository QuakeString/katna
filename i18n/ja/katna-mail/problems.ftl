# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = メールサーバー
problems-signed-out = { $provider } により { $address } から Katna がサインアウトされました。メールの同期は停止しています。
problems-password-refused = { $provider } が { $address } のパスワードを拒否しました。パスワードが変更された可能性があります。
problems-no-answer = { $provider } が { $address } について応答していません。Katna は再試行を続けます。
problems-offline = オフラインです。メールはここで読めます。送信するメールはオンラインに戻るまで待機します。
problems-accounts-need-you = { $count ->
   *[other] { $count } 個のアカウントで対応が必要です
}
problems-show = 表示
problems-later = 後で
problems-new-password = 新しいパスワード
problems-try-again = 再試行

## The New password card

problems-password-title = 新しいパスワード
problems-password-detail = { $provider } が { $address } の保存済みパスワードを拒否しました。新しいパスワードを入力してください。Katna が確認してから保存します。
problems-password-placeholder = パスワード
problems-password-show = パスワードを表示
problems-password-hide = パスワードを非表示
problems-password-cancel = キャンセル
problems-password-save = 保存
problems-password-checking = 確認しています…
problems-password-refused-again = { $provider } はこのパスワードも拒否しました。確認してもう一度お試しください。
problems-password-saved = { $address } のパスワードを保存しました。メールを取得しています…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $address } のメールサーバーが { $count ->
   *[other] { $count } 件のメールの移動を受け付けなかったため、元の場所に戻しました。
}
problems-refused-flags = { $address } のメールサーバーが { $count ->
   *[other] { $count } 件のメールのマーク（既読、スターなど）を受け付けなかったため、元の状態に戻しました。
}
problems-refused-label = { $address } のメールサーバーが { $count ->
   *[other] { $count } 件のメールのラベル変更を受け付けなかったため、元の状態に戻しました。
}
problems-refused-delete = { $address } のメールサーバーが { $count ->
   *[other] { $count } 件のメールの削除を受け付けなかったため、元に戻しました。
}
problems-refused-other = { $address } のメールサーバーが { $count ->
   *[other] { $count } 件の変更を受け付けなかったため、Katna が元の状態に戻しました。
}
problems-details = 詳細

## Katna's background service (katna-daemon) isn't running

service-starting = Katna のバックグラウンド サービスを起動しています…
service-failed = Katna のバックグラウンド サービスが起動しないため、メールが同期されていません。
service-start-again = もう一度起動
service-started-again = Katna のバックグラウンド サービスが停止したため、再起動しました。
service-details-title = サービスが起動しない理由
service-details-body = これをコピーして、レポートと一緒に送ってください。メールやパスワードは含まれていません。
service-details-copy = コピー
service-details-close = 閉じる
service-not-running = Katna のバックグラウンド サービスが実行されていません。
service-no-answer = Katna のバックグラウンド サービスが応答しませんでした: { $error }
service-no-session = D-Bus セッションがありません: { $error }
