# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = 閉じる
reader-back = 戻る
reader-mark-unread = 未読にする
reader-move-to = 移動
reader-snooze = スヌーズ
reader-remind = リマインド
reader-more = その他
reader-original-colors = 元の色で表示
reader-dark-colors = 暗い色で表示
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
reader-sending = 送信しています…
reader-me = 自分
reader-to = To: { $names }
reader-to-label = To:
reader-tick-delivered = 配信済み { $when }
reader-tick-no-bounce = 送信 { $when }。エラーメールが返ってこなかったため、ほぼ確実に届いています
reader-tick-bounced = 未配信: { $when } にエラーメールが返ってきました
reader-tick-read = 開封済み { $when }（開封確認）
reader-tick-opened = 開封、最終 { $when }（開封の追跡）
reader-starred = スター付き
reader-chip-remove = { $label } を外す
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
reader-download-failed-reason = このメールをダウンロードできませんでした。{ $reason }
reader-download-offline = このアカウントはオフラインです。このメールをダウンロードするにはオンラインにしてください。
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
security-look-up-key = 鍵を検索

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = 検証済みの署名
key-card-verified-detail = 署名は正しく、この鍵は信頼済みです。
key-card-unverified = 未検証の署名
key-card-unverified-detail = 署名は正しいものの、この鍵が本人のものであることは確認されていません。相手とフィンガープリントを照合してから、GnuPG（Kleopatra または gpg --edit-key）でこの鍵を信頼してください。
key-card-not-sender = 別の人による署名
key-card-not-sender-detail = 署名は正しいものの、送信者の鍵ではありません。
key-card-untrusted = 信頼していない鍵
key-card-untrusted-detail = この鍵は GnuPG で信頼しないとマークされています。
key-card-signature-expired = 署名の有効期限切れ
key-card-signature-expired-detail = 署名は正しかったものの、有効期限が切れています。
key-card-key-expired = 鍵の有効期限切れ
key-card-key-expired-detail = 署名は正しいものの、その後、鍵の有効期限が切れています。
key-card-key-revoked = 失効した鍵
key-card-key-revoked-detail = 所有者がこの鍵を失効させたため、署名は信頼できません。
key-card-bad = 不正な署名
key-card-bad-detail = このメールは署名後に変更されたか、署名が偽造されています。
key-card-signed-by = 署名者
key-card-belongs-to = 所有者
key-card-fingerprint = フィンガープリント
key-card-signed = 署名日時
key-card-key = 鍵
key-card-kind = { $standard }、{ $algorithm }
key-card-created = 作成日
key-card-expires = 有効期限
key-card-never = なし
key-card-issued-by = 発行者
key-card-found-in = 入手元
key-card-keyring = GnuPG キーリング
key-card-copy = フィンガープリントをコピー
key-card-import-title = この鍵をインポートしますか？
key-card-from-directory = { $domain } の鍵ディレクトリで見つかりました。
key-card-from-attachment = 添付ファイル { $name } から。
key-card-import-note = インポートすると、Katna でこの人の署名を確認したり、この人宛てのメールを暗号化したりできます。鍵を完全に信頼するには、相手とフィンガープリントを照合してください。
key-card-cancel = キャンセル
key-card-import = 鍵をインポート
key-card-looking-up = 鍵を検索しています…
key-card-looking-up-detail = { $domain } の鍵ディレクトリに問い合わせています。
key-card-not-found = 鍵が見つかりません
key-card-not-found-detail = { $domain } はこのアドレスの鍵を公開していません。送信者に鍵を送ってもらうよう依頼してください。
key-card-not-kept = 見つかった鍵は使用できません。
key-card-failed = 鍵を取得できませんでした

## Sender checks: the banner on mail that failed the checks the user's mail
## provider ran on its sender (DMARC, DKIM, SPF), and the "?" on the picture
## of a sender nothing confirmed

sender-failed-title = このメールは { $domain } からのものではない可能性があります
sender-failed-body = { $provider } の送信者チェックに合格しませんでした。リンク、添付ファイル、返信には注意してください。
sender-provider-unknown = お使いのメールサービス
sender-details = 詳細
sender-details-hide = 詳細を非表示
sender-looks-safe = 安全そう
sender-move-to-spam = 迷惑メールに移動
sender-checked-by = { $provider } が確認
sender-checked-by-server = { $provider } が確認（{ $server }）
sender-dmarc = 送信者ドメイン（DMARC）
sender-dkim = 署名（DKIM）
sender-spf = 送信サーバー（SPF）
sender-result-pass = 合格
sender-result-fail = 不合格
sender-result-unsure = 不明
sender-result-none = なし
sender-result-missing = 未確認
sender-dmarc-pass = { $domain } がこの送信者を確認しています。
sender-dmarc-fail = このメールは、{ $domain } が示す送信方法と一致しません。
sender-dmarc-none = { $domain } はメールに関するルールを公開していません。
sender-dkim-pass = { $domain } が署名しています。
sender-dkim-fail = { $domain } の署名がメールと一致しません。
sender-dkim-none = このメールには署名がありません。
sender-spf-pass = { $domain } が登録しているサーバーから送信されました。
sender-spf-fail = { $domain } が登録していないサーバーから送信されました。
sender-spf-none = { $domain } は送信サーバーを登録していません。
sender-check-unsure = チェックで明確な結果が得られませんでした。
sender-unconfirmed = { $provider } は、このメールが { $domain } から送られたことを確認できませんでした。送信者は誰でも自由に書けます。
sender-link-title = このリンクを開きますか？
sender-link-body = このメールは送信者チェックに合格しませんでした。リンク先は { $host } です:
sender-link-cancel = キャンセル
sender-link-open = 開く

## Open and click tracking and read receipts (the eye's popover beside a
## sent message's star, and the line above a read receipt)

tracking-opened = { $who } が { $count } 回開きました（最終: { $when }）
tracking-opens-clicks = { $who } が { $opens } 回開き、リンクを { $clicks } 回クリックしました（最終: { $when }）
tracking-clicked = { $who } がリンクを { $clicks } 回クリックしました（最終: { $when }）
tracking-maybe-opened = { $who } が開いた可能性があります（Apple Mail はプライバシー保護のために画像を読み込みます）
tracking-seen-none = まだ誰も開いておらず、リンクもクリックされていません
tracking-receipt = { $who } から開封確認が届きました
tracking-receipt-read = { $who } が読みました（開封確認）、{ $when }
tracking-receipt-displayed = 開封確認: { $who } があなたのメッセージを開きました
tracking-receipt-other = 開封確認: { $who } はあなたのメッセージを開かずに削除または処理しました

## Remote images and pictures

remote-hidden = このメールの画像は表示されていません。
remote-hidden-unconfirmed = 画像を表示していません。送信者を確認できませんでした。
remote-hidden-failed = 画像を表示していません。このメールは送信者チェックに合格しませんでした。
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
attachment-forward = 転送
attachment-save-all = すべて保存
attachment-save-all-tooltip = すべての添付ファイルをフォルダに保存
attachment-save-here = ここに保存
attachment-not-downloaded = このメールはダウンロードされていません。
attachment-open-message = 添付ファイルを見るには、このメールを開いてください。
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
print-preview-title = 印刷プレビュー
print-preview-laying-out = ページをレイアウトしています…
print-preview-pages = { $count } ページ
print-preview-more = ほか { $count } ページ
print-preview-failed = ページを表示できませんでした
print-preview-paper = 用紙
print-preview-a4 = A4
print-preview-letter = レター
print-preview-layout = レイアウト
print-preview-as-shown = 表示のまま
print-preview-simple = テキストのみ
print-preview-backgrounds = 背景
print-preview-cancel = キャンセル
print-preview-print = 印刷
print-not-downloaded = （まだダウンロードされていません。）
print-encrypted = （暗号化されています。本文を印刷するには Katna Mail で開いてください。）
print-to = To: { $addresses }
print-cc = Cc: { $addresses }

## Message text (right-click menu in the reading pane)

text-pin = 上部に固定
text-copy-address = アドレスをコピー
text-copy = コピー
text-select-all = すべて選択
