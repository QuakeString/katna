# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
