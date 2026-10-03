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
compose-more-recipients = 他 { $count } 人
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
compose-draft-saving = 保存中…
compose-draft-failed = 下書きを保存できませんでした: { $error }
compose-draft-not-opened = 下書きを開けませんでした。

## Attachments

compose-picker-insert = 挿入
compose-picker-attach = 添付
compose-file-too-large = { $name } は大きすぎます。1 通のメッセージに添付できるのは { $limit } までです。
compose-forward-files-missing = 転送するメールのファイルがダウンロードされていないため、添付されていません。
compose-attachment-size = （{ $size }）
compose-remove-attachment = 添付ファイルを削除
compose-attachment-open-tip = 開いて確認
compose-attachments-total = ファイル { $count } 個、{ $size }
compose-drive-note = { $name } は { $limit } を超えているため、Google Drive に保存され、メッセージにはそのリンクが付きます。
compose-drive-tip = Google Drive 内にあります。メッセージにはリンクが付きます
compose-drive-uploading = アップロード中 { $percent }%
compose-drive-allow = Drive を許可
compose-drive-allow-tip = Google でもう一度サインインすると、Katna が大きなファイルを Drive に置けるようになります
compose-drive-retry = 再試行
compose-drive-sends-when-uploaded = { $name } のアップロードが終わり次第、送信します
compose-drive-not-uploaded = { $name } はまだ Google Drive にありません
compose-drive-share-failed = Google Drive でファイルを共有できませんでした: { $error }
compose-drive-share-title = ファイルを全員に共有しますか？
compose-drive-share-text = { $count ->
   *[other] Google Drive では、Google アカウントを持たない { $addresses } とファイルを共有できません。代わりに、リンクを知っている人なら誰でも開けるようになります。
}
compose-drive-share-link = リンクで共有
compose-drive-send-without = 共有せずに送信
compose-drive-share-cancel = キャンセル
compose-drive-card-detail = { $size } · Google Drive
compose-drive-card-name = Google Drive
compose-onedrive-note = { $name } は { $limit } を超えているため、OneDrive に保存され、メッセージにはそのリンクが付きます。
compose-onedrive-tip = OneDrive 内にあります。メッセージにはリンクが付きます
compose-onedrive-allow = OneDrive を許可
compose-onedrive-allow-tip = Microsoft でもう一度サインインすると、Katna が大きなファイルを OneDrive に置けるようになります
compose-onedrive-not-uploaded = { $name } はまだ OneDrive にありません
compose-onedrive-share-failed = OneDrive でファイルを共有できませんでした: { $error }
compose-onedrive-share-text = { $count ->
   *[other] OneDrive では { $addresses } とファイルを共有できません。代わりに、リンクを知っている人なら誰でも開けるようになります。
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-onedrive-card-name = OneDrive
compose-drop-files = ここにファイルをドロップ
compose-drop-here = ここにドロップ

## Paste options (a small bar under what was just pasted or dropped)

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

## Open and click tracking and read receipts (toggles after Sign)

compose-track = 開封とクリックを追跡
compose-tracked = 追跡中: 各宛先がいつメールを開いたか、リンクをクリックしたかがわかります
compose-track-clicks = リンクのクリックを追跡（プレーンテキストでは開封を確認できません）
compose-tracked-clicks = 追跡中: 各宛先がいつリンクをクリックしたかがわかります
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

## Recipients (To, Cc and Bcc)

recipient-not-valid = 有効なメールアドレスではありません
recipient-show-address = アドレスを表示
recipient-remove = 削除
recipient-bad-title = アドレスを確認してください
recipient-bad-text = 「{ $address }」は有効なメールアドレスではありません。送信する前に修正するか削除してください。
recipient-bad-fix = 修正
