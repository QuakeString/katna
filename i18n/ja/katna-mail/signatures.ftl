# Katna Mail, Japanese (日本語).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The signature being edited

signature-placeholder = 名前と、その下に添える内容

## Its formatting bar

signature-bold = 太字
signature-italic = 斜体
signature-underline = 下線
signature-link = リンク
signature-link-apply = 適用
signature-picture = 画像を挿入
signature-align-left = 左揃え
signature-align-center = 中央揃え
signature-align-right = 右揃え
signature-numbered-list = 番号付きリスト
signature-bulleted-list = 箇条書き
signature-remove-formatting = 書式をクリア

## Adding a picture

signature-picture-choose = 挿入
signature-picture-too-big = 署名に入れられる画像は { $size } までです。
signature-picture-kind = PNG、JPEG、GIF、WebP のいずれかの画像を選んでください。
signature-picture-unreadable = { $name }: { $error }

## Layouts

signature-layout = レイアウト
signature-layout-own = 自分で作成
signature-layout-classic = クラシック
signature-layout-logo-left = 左にロゴ
signature-layout-photo = 写真
signature-layout-band = カラーバンド
signature-layout-one-line = 1 行
signature-layout-centred = 中央揃え
signature-layout-banner = バナー付き
signature-layout-underline = 下線
signature-layout-side-bar = サイドバー
signature-layout-card = カード
signature-layout-monogram = モノグラム
signature-layout-plain = プレーンテキスト
signature-layout-mobile-label = M:
signature-layout-office-label = O:
signature-layout-email-label = E:
signature-layout-name = 名前
signature-layout-job = 役職
signature-layout-company = 会社
signature-layout-mobile = 携帯電話
signature-layout-office = 会社の電話
signature-layout-email = メール
signature-layout-website = ウェブサイト
signature-layout-address = 住所
signature-layout-pictures = 画像
signature-layout-logo = ロゴ
signature-layout-photo-picture = 写真
signature-layout-banner-picture = バナー
signature-layout-remove-picture = 削除
signature-layout-pages = ページ
signature-layout-page-placeholder = ページのアドレスを追加
signature-layout-colour = 色
signature-layout-picture-failed = { $name } は画像として使えませんでした。
signature-layout-preview = 受信者からの見え方
signature-layout-light = ライト
signature-layout-dark = ダーク
signature-layout-text = プレーンテキスト
signature-layout-inside = 画像はメールに埋め込んで送信されるため、ウェブ上の画像をオフにしている受信者にも表示されます。この画像により、メール 1 通あたり { $size } 増えます。
signature-layout-free = ほかのデザインにしたい場合は
signature-layout-edit = 手動で編集
signature-layout-edit-confirm = 手動で編集しますか？入力欄とレイアウトはなくなりますが、見た目はエディタで再現できる範囲で保たれます。
signature-layout-use-confirm = 「{ $layout }」レイアウトを使いますか？この署名の内容を使って、署名が置き換えられます。
signature-layout-use = レイアウトを使用
signature-layout-cancel = キャンセル

## Paste HTML

signature-html-title = HTML を貼り付け
signature-html-subtitle = ほかの場所でデザインした署名用
signature-html-placeholder = 署名の HTML をここに貼り付け
signature-html-name = 貼り付けた署名
signature-html-new = 新しい署名「{ $name }」として保存されます
signature-html-replaces = 「{ $name }」に上書き保存されます
signature-html-cancel = キャンセル
signature-html-save = 保存
signature-html-fetching = 画像をダウンロードしています…
signature-html-pictures-inside = { $count ->
   *[other] { $count } 枚の画像をダウンロードしてメールに埋め込みました（{ $size }）
}
signature-html-pictures-web = { $count ->
   *[other] { $count } 枚の画像をダウンロードできなかったため、受信者はウェブから読み込みます
}
signature-html-removed = スクリプト、フォーム、トラッキング ピクセルを削除しました（いずれにしてもメールアプリでブロックされます）
signature-html-style-sheet = スタイルシートを除外しました: メールでは各要素に直接書かれたスタイルだけが残ります
signature-html-links = ウェブサイト、メールアドレス、電話番号以外へのリンクを削除しました
signature-html-plain-text = テキストのみを表示するメールアプリ用に、プレーンテキスト版を作成しました

## Import

signature-import-title = インポート
signature-import-subtitle = Gmail、Thunderbird、Evolution、KMail から
signature-import-looking = 署名を探しています…
signature-import-none = 署名が見つかりませんでした。ほかのアプリの場合は、署名の HTML をコピーして「HTML を貼り付け」を使ってください。
signature-import-from = { $app } から
signature-import-already = Katna に登録済み
signature-import-gmail-sign-in = { $address }: Katna が Gmail の署名を読めるよう、設定 > アカウント でもう一度サインインしてください。
signature-import-gmail-failed = { $address }: { $error }
signature-import-cancel = キャンセル
signature-import-do = { $count ->
   *[other] { $count } 件の署名をインポート
}
signature-import-name = { $name }（{ $app }）
