# Katna Mail, Japanese (日本語): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = 連絡先
contacts-frequent = よく使う連絡先
contacts-other = その他の連絡先
contacts-other-about = Gmail でメールを送ったが保存していない相手
contacts-other-email = メールを送信
contacts-other-empty = その他の連絡先はありません。Gmail でメールを送ったが保存していない相手がここに表示されます。
contacts-other-allow = その他の連絡先を表示するには、Gmail アカウントに再度ログインして、Katna による表示を許可してください。
contacts-labels = ラベル
contacts-label-options = ラベルのオプション
contacts-label-rename = ラベル名を変更
contacts-label-email = 全員にメールを送信
contacts-label-delete = ラベルを削除
contacts-label-new = 新しいラベル
contacts-label-name = ラベル名
contacts-label-button = ラベル
contacts-label-menu = ラベルを付ける:
contacts-label-added = { $name } に追加しました
contacts-label-removed = { $name } から削除しました
contacts-label-renamed = ラベル名を { $name } に変更しました
contacts-label-deleted = ラベル { $name } を削除しました
contacts-label-no-email = このラベルに、メールアドレスのある人はいません
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = アカウント
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = もう一度サインインして連絡先を表示
contacts-account-signed-in = { $address } に再度サインインしました。連絡先を取得しています…
contacts-account-sign-in-refused = { $provider } が Katna のアクセスを許可しませんでした。もう一度試して、連絡先へのアクセスを許可してください。
contacts-account-password = サーバーがパスワードを受け付けませんでした。Yahoo、iCloud、Zoho などではアプリ パスワードが必要です。
contacts-account-change-password = パスワードを変更
contacts-account-change-password-tooltip = 設定 > アカウント を開く
contacts-account-failed = 連絡先を読み込めませんでした。
# $reason is the server's own words, in English.
contacts-account-error = 連絡先を読み込めませんでした: { $reason }
contacts-account-none = アドレス帳が見つかりません
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = アドレス帳が見つかりません: { $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider } の連絡先は、{ $provider } でサインインした Katna にのみ表示されます。
contacts-account-sign-in-with = { $provider } でサインイン
contacts-account-looking = 連絡先を探しています…
contacts-account-try-again = 再試行
contacts-account-try-again-tooltip = このアカウントの連絡先を今すぐ再確認
contacts-account-fixing = 対応しています…
contacts-manage = 修正と管理
contacts-merge = 統合と修正
contacts-merge-about = { $count ->
   *[other] { $count } 件の候補: 同一人物とみられる連絡先
}
contacts-merge-none = 重複はありません。名前や電話番号が同じ連絡先がここに表示されます。
contacts-merge-count = { $count ->
   *[other] { $count } 件の連絡先
}
contacts-merge-all = すべて統合
contacts-merge-button = 統合
contacts-merge-dismiss = 無視
contacts-merged = { $count ->
    [1] 連絡先を統合しました
   *[other] { $count } 件を統合しました
}
contacts-import = インポート
contacts-export = エクスポート
contacts-import-file = vCard または CSV ファイルから連絡先をインポート
contacts-imported = { $count ->
   *[other] { $place } に { $count } 件の連絡先をインポートしました
}
contacts-imported-some = { $count ->
   *[other] { $place } に { $count } 件の連絡先をインポートしました。保存済みの { $skipped } 件は除外しました
}
contacts-import-none = { $name } に連絡先が見つかりません
contacts-import-all-saved = { $name } のユーザーはすべて保存済みです
contacts-import-failed = { $name } を読み込めませんでした: { $error }
contacts-exported = { $count ->
   *[other] { $path } に { $count } 件の連絡先をエクスポートしました
}
contacts-export-none = エクスポートする連絡先がありません
contacts-export-failed = 連絡先をエクスポートできませんでした: { $error }
contacts-print = 印刷
contacts-print-title = 連絡先
contacts-print-none = 印刷する連絡先がありません
contacts-print-typed = { $value }（{ $kind }）
contacts-print-birthday = 誕生日: { $day }
contacts-print-nickname = ニックネーム: { $name }
contacts-create = 連絡先を作成

## Search and the list

contacts-search = 連絡先を検索
contacts-loading = 連絡先を読み込んでいます…
contacts-empty = 保存済みの連絡先はまだありません。Gmail、Outlook、お使いのメールサービスに保存した連絡先がここに表示されます。
contacts-empty-no-books = アカウントの連絡先は、同期が完了するとここに表示されます。
contacts-none-found = 検索に一致する連絡先はありません。
contacts-starred = { $count ->
   *[other] スター付きの連絡先 ({ $count })
}
contacts-count = 連絡先 ({ $count })
contacts-col-name = 名前
contacts-col-email = メールアドレス
contacts-col-phone = 電話番号
contacts-col-job = 役職と会社
contacts-col-labels = ラベル

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Katna に { $address } の連絡先の読み取りを許可します。
contacts-allow-many = { $more ->
   *[other] Katna に { $address } ほか { $more } 件のアカウントの連絡先の読み取りを許可します。
}
contacts-allow-button = 許可

## A contact's page

contacts-back = 連絡先に戻る
contacts-edit = 編集
contacts-delete = 削除
contacts-qr = QR コードで共有
contacts-qr-about = スマートフォンのカメラでスキャンすると、連絡先を保存できます。
contacts-qr-too-long = この連絡先は情報が多すぎて QR コードに収まりません。
contacts-qr-done = 完了
contacts-deleted = { $name } を削除しました
contacts-added = { $name } を連絡先に追加しました
contacts-find-mail = メール
contacts-details = 連絡先の詳細
contacts-saved-in = 保存先
contacts-notes = メモ
contacts-birthday = 誕生日
contacts-nickname = ニックネーム
contacts-this-computer = このコンピューター
contacts-kind-home = 自宅
contacts-kind-work = 勤務先
contacts-kind-mobile = 携帯
contacts-kind-other = その他
contacts-source-google = Google コンタクト
contacts-source-microsoft = Outlook の連絡先
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = 連絡先を作成
contacts-edit-title = 連絡先を編集
contacts-edit-save = 保存
contacts-edit-saving = 保存中…
contacts-edit-cancel = キャンセル
contacts-saved = 連絡先を保存しました
contacts-edit-save-to = 保存先
contacts-edit-changes-go-to = 変更は { $place } に保存されます。
contacts-edit-given = 名
contacts-edit-family = 姓
contacts-edit-company = 会社
contacts-edit-job = 役職
contacts-edit-email = メール
contacts-edit-phone = 電話
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = メールを追加
contacts-edit-add-phone = 電話番号を追加
contacts-edit-street = 番地・町名
contacts-edit-city = 市区町村
contacts-edit-postcode = 郵便番号
contacts-edit-country = 国
contacts-edit-birthday = 誕生日 (YYYY-MM-DD)
contacts-edit-empty = 名前、メールアドレス、電話番号のいずれかを先に追加してください。
