# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = 新規メッセージ
compose-restore = 元のサイズに戻す
compose-minimize = 最小化
compose-exit-full-screen = 全画面表示を終了
compose-open-window = 新しいウィンドウで開く
compose-save-close = 保存して閉じる
compose-back-to-mail = メールのウィンドウに戻る
compose-pop-out-reply = 返信を別ウィンドウで開く
compose-edit-recipients = 宛先を編集
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Bcc: { $names }
compose-show-trimmed = 省略されたコンテンツを表示
compose-hide-trimmed = 省略されたコンテンツを非表示
compose-remove-trimmed = 引用テキストを削除
compose-trimmed-removed = 引用テキストを削除しました

## Recipients and subject

compose-to = To
compose-cc = Cc
compose-bcc = Bcc
compose-from = From
compose-from-choose = 別のアカウントから送信
compose-recipients = 宛先
compose-subject = 件名

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = 先に開いているメッセージを送信するか破棄してください。
compose-bad-address = 「{ $address }」はメールアドレスではありません。
compose-no-recipients = 宛先を 1 人以上追加してください。
compose-attachments-too-large = 添付ファイルの合計は { $size } です。メールサーバーが受け付けるのは { $limit } までです。
compose-no-account = メールを送信するアカウントを追加してください。
compose-past-time = 未来の日時を選んでください。
compose-scheduling = 予約しています…
compose-sending = 送信しています…
compose-scheduled = { $when } に送信予約しました
compose-sent-archived = 送信してアーカイブしました
compose-sent = メッセージを送信しました
compose-discarded = 下書きを破棄しました
compose-draft-saved = 下書きを保存しました
compose-draft-failed = 下書きを保存できませんでした: { $error }
compose-draft-not-opened = 下書きを開けませんでした。

## Attachments

compose-picker-insert = 挿入
compose-picker-attach = 添付
compose-file-too-large = { $name } は大きすぎます。1 通のメッセージに添付できるのは { $limit } までです。
compose-attachment-size = （{ $size }）
compose-remove-attachment = 添付ファイルを削除
compose-attachments-total = ファイル { $count } 個、{ $size }
compose-drop-files = ここにファイルをドロップ
compose-drop-here = ここにドロップ
compose-paste-keep-formatting = 書式を保持
compose-paste-table = 表
compose-paste-picture = 画像
compose-paste-plain-text = プレーンテキスト
compose-paste-inline = 本文に挿入
compose-paste-attachment = 添付ファイル

## Encryption and signing (the toggles by the recipients)

compose-encrypt = 暗号化
compose-encrypted = 暗号化済み: 受信者だけが読めます
compose-sign = 署名
compose-signed = 署名済み: あなたからのメールであることを受信者が確認できます
compose-track = 開封とクリックを追跡
compose-tracked = 追跡中: 各宛先がいつメールを開いたか、リンクをクリックしたかがわかります
compose-track-sign-in = 開封とクリックを追跡するには Katna アカウントにサインインしてください
compose-receipt = 開封確認を要求
compose-receipt-on = 開封確認を要求しました: 受信者のアプリで、開封確認を送るかどうか尋ねられる場合があります
compose-delivery = 配信確認を要求
compose-delivery-on = 配信確認を要求しました: 各宛先のサーバーがメールを受け付けると、メールサーバーからメールで通知されます
compose-delivery-unavailable = お使いのメールサーバーは配信確認を送信しません

## Spelling

spell-no-dictionary = { $language } のスペル辞書がインストールされていません（例: hunspell-en_us）。
spell-dictionary-error = スペル辞書: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = 「{ $words }」
grammar-add = 「{ $words }」を追加
grammar-remove = 「{ $words }」を削除
grammar-ignore = 無視

## Send checks (asked before a message goes out)

send-check-attachment-title = ファイルを添付するつもりでしたか？
send-check-attachment-text = 本文で添付ファイルに触れていますが、何も添付されていません。
send-check-attach = ファイルを添付
send-check-subject-title = 件名なしで送信しますか？
send-check-subject-text = このメッセージには件名がありません。
send-check-add-subject = 件名を追加
send-check-send-anyway = このまま送信
recipient-not-valid = 有効なメールアドレスではありません
recipient-show-address = アドレスを表示
recipient-remove = 削除
recipient-bad-title = アドレスを確認してください
recipient-bad-text = 「{ $address }」は有効なメールアドレスではありません。送信する前に修正するか削除してください。
recipient-bad-fix = 修正
