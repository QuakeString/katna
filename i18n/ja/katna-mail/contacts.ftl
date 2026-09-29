# Katna Mail, Japanese (日本語): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = 連絡先
contacts-frequent = よく使う連絡先
contacts-labels = ラベル

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
