# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = 新增郵件帳戶
add-account-looking = 正在尋找 { $address } 的郵件伺服器…
add-account-address-intro = 輸入你的電子郵件地址，Katna 會為你找到伺服器。
add-account-servers-title = 伺服器設定
add-account-servers-intro = Katna 為 { $address } 接收和傳送郵件所使用的伺服器。
add-account-password-title = 輸入你的密碼
add-account-signing-in = 正在登入…

## Add a mail account: fields

add-account-field-address = 電子郵件地址
add-account-incoming = 內送郵件（{ $protocol }）
add-account-outgoing = 外寄郵件（{ $protocol }）
add-account-field-server = 伺服器
add-account-field-port = 連接埠
add-account-security-none = 無
add-account-field-username = 使用者名稱
add-account-field-password = 密碼
add-account-show-password = 顯示密碼
add-account-app-password-hint = { $provider } 在這裡需要應用程式密碼，而不是你在網頁上使用的密碼。請在你的 { $provider } 帳戶安全性設定中產生一組。
add-account-field-name = 你的姓名（選填）
add-account-name-hint = 會顯示給你寫信的對象。
add-account-servers-pair = { $imap } 和 { $smtp }
add-account-servers-found = { $source ->
    [built-in] 伺服器：{ $servers }，在 Katna 的供應商清單中找到。
    [provider] 伺服器：{ $servers }，在你的郵件服務供應商設定中找到。
    [ispdb] 伺服器：{ $servers }，在 Thunderbird 的供應商清單中找到。
    [dns] 伺服器：{ $servers }，在你的網域 DNS 記錄中找到。
   *[other] 伺服器：{ $servers }，為推測結果；如果登入失敗，請檢查。
}
add-account-servers-entered = 伺服器：{ $servers }，依輸入內容。

## Add a mail account: buttons

add-account-servers-button = 伺服器設定
add-account-back = 返回
add-account-add = 新增帳戶
add-account-next = 下一步
add-account-cancel = 取消

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] 請輸入內送伺服器。
   *[outgoing] 請輸入外寄伺服器。
}
add-account-server-space = { $kind ->
    [incoming] 內送伺服器名稱中含有空格。
   *[outgoing] 外寄伺服器名稱中含有空格。
}
add-account-port-invalid = { $kind ->
    [incoming] 內送連接埠必須是 { $min } 到 { $max } 之間的數字。
   *[outgoing] 外寄連接埠必須是 { $min } 到 { $max } 之間的數字。
}
add-account-address-empty = 請輸入電子郵件地址。
add-account-address-invalid = 請輸入電子郵件地址，例如 { $example }。
add-account-not-found = Katna 找不到 { $address } 的伺服器，因此填入了常用名稱。請向你的郵件服務供應商確認。
add-account-password-empty = 請輸入密碼。
add-account-name-is-password = 姓名與密碼相同。請改在那裡輸入你希望別人看到的姓名。
add-account-added = 已新增 { $address }。正在接收你的郵件…
add-account-app-password-refused = { $provider } 拒絕了這組密碼。它需要應用程式密碼，而不是你在網頁上使用的密碼。
add-account-password-refused = 伺服器拒絕了這組密碼。請檢查後再試一次。

## The account menu (from the account button on the top bar)

add-account-menu-another = 新增其他帳戶
add-account-menu-manage = 管理帳戶
