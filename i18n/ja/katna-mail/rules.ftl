# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = ルール
settings-rules-summary = 新着メールを自動で分類、ラベル付け、転送、通知オフにする
settings-rules-intro = ルールは新着メールをこの順番で自動的に分類します。ドラッグで並べ替えられます。
settings-rules-all-accounts = すべてのアカウント
settings-rules-new = 新しいルール
settings-rules-none = ルールはまだありません。ルールを使うと、新着メールを送信者、件名、キーワードで自動的に分類できます。
settings-rules-none-account = このアカウントのルールはまだありません。
settings-rules-drag = ドラッグして並べ替え
settings-rules-edit = ルールを編集
settings-rules-turn-off = このルールをオフにする
settings-rules-turn-on = このルールをオンにする

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = おすすめのルール
settings-rules-starters-intro = オンにするまでは無効です。すべてのアカウントで使え、編集して変更できます。
settings-rules-starter-turning-on = 「{ $name }」をオンにしています…
settings-rules-starter-failed = 「{ $name }」をオンにできませんでした: { $error }
rules-starter-promotions = プロモーションの通知をオフ
rules-starter-newsletters = ニュースレターを「あとで読む」へ
rules-starter-receipts = 領収書と請求書
rules-starter-deliveries = 配送
rules-starter-train = 電車の切符
rules-starter-flight = 航空券
rules-starter-codes = ワンタイムコード
rules-starter-security = セキュリティ通知
rules-starter-social = ソーシャルのメール
rules-starter-invites = カレンダーの招待
rules-starter-folder-reading = あとで読む
rules-starter-folder-receipts = 領収書
rules-starter-folder-deliveries = 配送
rules-starter-folder-travel = 旅行
rules-starter-folder-social = ソーシャル
rules-runs-katna = Katna で実行
rules-runs-gmail = Gmail で実行
rules-runs-sieve = サーバーで実行
rules-stopped = 停止中
rules-error-folder-gone = このルールで使うフォルダが存在しません。ルールを編集して別のフォルダを選んでください。
rules-error-no-archive = このアカウントにはアーカイブ フォルダがありません。ルールを編集して別の操作を選んでください。
rules-error-no-trash = このアカウントにはゴミ箱フォルダがありません。ルールを編集して別の操作を選んでください。
rules-error-cannot-send = このアカウントはメールを送信できないため、ルールで転送できません。
rules-error-other = { $error }。ルールを編集してもう一度オンにしてください。
settings-folders = フォルダ
settings-folders-summary = フォルダ ペインの未読数
settings-folders-unread-counts = すべてのフォルダに未読数を表示
settings-folders-unread-counts-detail = オフ: 未読数は受信トレイにのみ表示

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first }、かつ { $next }
rules-summary-or = { $first }、または { $next }
rules-summary-more = 他 { $count } 件
rules-summary-list = { $first }、{ $next }
rules-summary-condition = { $field } が「{ $value }」{ $comparator }
rules-summary-has-attachment = 添付ファイルあり
rules-summary-no-attachment = 添付ファイルなし
rules-summary-mailing-list = メーリングリストから
rules-summary-not-mailing-list = メーリングリスト以外から
rules-summary-tab = 「{ $tab }」タブ内
rules-summary-not-tab = 「{ $tab }」タブ以外
rules-summary-move = { $folder } に移動
rules-summary-archive = 受信トレイをスキップ
rules-summary-trash = ゴミ箱に移動
rules-summary-mark-read = 既読にする
rules-summary-star = スターを付ける
rules-summary-important = 重要マークを付ける
rules-summary-label = ラベル「{ $label }」を付ける
rules-summary-forward = { $address } に転送
rules-summary-dont-notify = 通知しない
rules-summary-read-after = { $count ->
   *[other] { $count } 日後に既読にする
}
rules-summary-folder-gone = 削除されたフォルダ

## The rule editor

rules-editor-new-title = 新しいルール
rules-editor-edit-title = ルールを編集
rules-editor-name-hint = ルール名
rules-editor-when = 新着メールが次の
rules-editor-of-these = 条件に一致したとき:
rules-mode-all = すべての
rules-mode-any = いずれかの
rules-field-from = From
rules-field-to = To
rules-field-cc = Cc
rules-field-any-recipient = To または Cc
rules-field-reply-to = Reply-To
rules-field-subject = 件名
rules-field-body = 本文
rules-field-attachment-name = 添付ファイル名
rules-field-has-attachment = 添付ファイルあり
rules-field-mailing-list = メーリングリストから
rules-field-tab = 受信トレイのタブ
rules-comparator-contains = を含む
rules-comparator-not-contains = を含まない
rules-comparator-begins-with = で始まる
rules-comparator-ends-with = で終わる
rules-comparator-equals = と完全に一致する
rules-comparator-matches = のパターンに一致する
rules-has-yes = はい
rules-has-no = いいえ
rules-editor-value-hint = キーワードまたはアドレス
rules-editor-add-condition = 条件を追加
rules-editor-remove = 削除
rules-editor-then = 実行する操作:
rules-action-move = 移動先
rules-action-archive = 受信トレイをスキップ（アーカイブ）
rules-action-trash = ゴミ箱に移動
rules-action-mark-read = 既読にする
rules-action-star = スターを付ける
rules-action-important = 重要マークを付ける
rules-action-label = ラベルを付ける
rules-action-forward = 転送先
rules-action-dont-notify = 通知しない
rules-action-read-after = 次の日数後に既読にする
rules-editor-choose-folder = フォルダを選択
rules-editor-choose-label = ラベルを選択
rules-editor-new-folder = 新規: { $name }
rules-editor-folder-of = { $folder }（{ $account }）
rules-editor-forward-hint = メールアドレス
rules-editor-days = 日
rules-editor-add-action = 操作を追加
rules-editor-stop = ここで終了: このメールには以降のルールを適用しない
rules-editor-accounts = アカウント:
rules-editor-accounts-none = アカウントを選択
rules-editor-accounts-many = { $count ->
   *[other] { $count } 個のアカウント
}
rules-editor-matches = 過去 { $days } 日間の { $mails } に一致
rules-editor-mails = { $count ->
   *[other] { $count } 件のメール
}
rules-editor-counting = 一致するメールを数えています…
rules-editor-show = 表示
rules-editor-also-apply = この { $count } 件にも適用
rules-editor-runs-katna = Katna で実行されます。このパソコンの電源が入っている間のみ動作します。
rules-editor-runs-gmail = Gmail で実行されるため、スマートフォンでも、このパソコンの電源が切れていても動作します。
rules-editor-runs-sieve = メールサーバーで実行されるため、スマートフォンでも、このパソコンの電源が切れていても動作します。
rules-note-gmail-action = Katna で実行: Gmail のフィルタでは「{ $action }」ができません。
rules-note-sieve-action = Katna で実行: メールサーバーのルールでは「{ $action }」ができません。
rules-note-test = { $field } が…{ $comparator }
rules-note-gmail-condition = Katna で実行: Gmail のフィルタでは「{ $test }」を Katna と同じように判定できません。
rules-note-sieve-condition = Katna で実行: メールサーバーのルールでは「{ $test }」を Katna と同じように判定できません。
rules-note-order = Katna で実行: このアカウントの前のルールが Katna で実行されるためです。ルールはリストの順に実行されます。
rules-note-gmail-stop = Katna で実行: Gmail のフィルタでは以降のルールの実行を止められません。
rules-note-gmail-forward = Katna で実行: Gmail は設定で確認済みのアドレスにしか転送できず、{ $address } は確認済みではありません。
rules-note-gmail-folder = Katna で実行: このルールで使うフォルダに対応するラベルが Gmail にありません。
rules-note-sieve-folder = Katna で実行: このルールで使うフォルダがメールサーバーにありません。
rules-note-gmail-sign-in = Google にもう一度サインインして Katna に Gmail のフィルタの作成を許可するまでは、Katna で実行されます。
rules-note-sieve-other-script = Katna で実行: メールサーバーで別のルール スクリプト（「{ $name }」）が有効になっています。
rules-note-gmail-failed = Katna で実行: Gmail が受け付けませんでした（{ $error }）。
rules-note-sieve-failed = Katna で実行: メールサーバーが受け付けませんでした（{ $error }）。
rules-editor-cancel = キャンセル
rules-editor-save = 保存
rules-editor-saving = 保存しています…
rules-editor-delete = ルールを削除
rules-editor-delete-ask = このルールを削除しますか？
rules-editor-delete-keep = 残す
rules-editor-delete-confirm = 削除
rules-editor-needs-folder = 「移動先」にはフォルダを、「ラベルを付ける」にはラベルをそれぞれ選んでください。
rules-editor-needs-days = 「次の日数後に既読にする」には 1～3650 の日数を指定してください。
rules-saved = ルールを保存しました
rules-saved-applied = { $count ->
   *[other] ルールを保存し、{ $count } 件のメールに適用しました
}
rules-apply-failed = ルールを保存しましたが、適用できませんでした: { $error }
rules-deleted = ルールを削除しました
rules-delete-failed = ルールを削除できませんでした: { $error }
rules-change-failed = ルールを変更できませんでした: { $error }
