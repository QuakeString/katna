# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = 閲覧
chat-view = スレッドをチャット形式で表示
chat-view-detail = 人とのメールをグループチャットのように読めます。メールごとに書かれた内容だけが吹き出しで表示され、自分のメールは右側に並びます。ニュースレターは通常の表示のままです。
chat-view-switch = スレッドをチャット形式で表示
chat-view-switch-detail = 引用されたメールと署名は、各吹き出しの ··· の中に隠れます
chat-switch-chat = チャット
chat-switch-mail = メール
chat-people = { $names }、自分 · { $count ->
   *[other] { $count } 件のメール
}
chat-people-heading = { $count ->
   *[other] このチャットのメンバー · { $count } 人
}
chat-member-mails = { $count ->
    [0] メールなし
   *[other] { $count } 件のメール
}
chat-today = 今日
chat-yesterday = 昨日
chat-added = { $who } が { $names } を追加しました
chat-renamed = { $who } が件名を「{ $subject }」に変更しました
chat-you = 自分
chat-not-downloaded = まだダウンロードされていません
chat-forwarded = 転送されたメール
chat-show-quoted = 引用されたメールと署名を表示
chat-hide-quoted = 引用されたメールと署名を非表示
chat-hide-dots = ··· を非表示
chat-show-card = 連絡先カードを表示
chat-reply-all = 全員に返信
chat-more = その他
chat-reply-only = { $name } さんだけに返信
chat-forward = 転送
chat-copy-text = テキストをコピー
chat-show-as-mail = メールとして表示
chat-pin = 上部に固定
chat-pin-file = ファイルを上部に固定
chat-unpin = 固定を解除
chat-unpin-file = ファイルの固定を解除
chat-pinned-of = 固定 { $count } 件中 { $at } 件目
chat-pins-all = すべての固定アイテム
chat-pins-heading = 固定済み · { $most } 件中 { $count } 件
chat-pins-drag = ドラッグして並べ替え
chat-pin-from-mail = { $name } からのメール · { $when }
chat-pin-from-file = { $name } からのファイル · { $when }
chat-pin-from-text = { $name } からのテキスト · { $when }
chat-pins-full = このチャットにはすでに 5 件固定されています
chat-pins-replace-title = 固定アイテムを置き換え
chat-pins-replace-hint = チャットに固定できるのは 5 件までです。外すものを選んでください。
chat-pins-replace = 置き換え
chat-pins-cancel = キャンセル
chat-undo = 元に戻す
chat-reply-to = { $names } に返信
chat-send = 送信 (Ctrl+Enter)
chat-attach = 添付
chat-attach-photo = 写真
chat-attach-file = ファイル
chat-attach-library = ファイルから
chat-attach-template = テンプレート
chat-attach-signature = 署名
chat-replying-to = { $name } さんに返信中
chat-reply-newest = 最新のメールに返信

## The attach picker (paperclip > From Files)

picker-title = ファイルから添付
picker-search = 名前、ユーザー、件名を検索
picker-search-drive = このドライブを検索
picker-mail-files = メールのファイル
picker-this-chat = このスレッド
picker-this-computer = このパソコン…
picker-in-chat = このスレッド
picker-recent = 最近
picker-preview = プレビュー
picker-cancel = キャンセル
picker-attach = 添付
picker-attach-count = { $count } 件を添付
picker-selected = { $count } 件選択中
picker-of-limit = / { $limit }
picker-in-mail = メール本体に { $size }
picker-drive-links = { $count ->
   *[other] { $count } 件を Google Drive のリンクとして
}
picker-onedrive-links = { $count ->
   *[other] { $count } 件を OneDrive のリンクとして
}
picker-over = { $size }。メールで送れる上限 { $limit } を超えています
picker-getting = { $count ->
   *[other] ドライブから { $count } 件のファイルを取得しています…
}
picker-some-failed = { $count ->
   *[other] { $count } 件のファイルを読み込めませんでした
}
