# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = 新增郵件帳戶
add-account-providers-intro = 選擇你的郵件服務供應商，其餘的交給 Katna。
add-account-provider-other = 其他郵件
add-account-provider-other-detail = 任何 IMAP 或 POP3 帳戶
add-account-provider-google-detail = Gmail 和 Google Workspace
add-account-provider-microsoft-detail = Outlook 和 Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = 登入 { $provider }
add-account-form-title-other = 你的郵件帳戶
add-account-form-intro = Katna 會將你的密碼保存在系統的金鑰圈中。
add-account-looking = 正在尋找 { $address } 的郵件伺服器…
add-account-address-intro = 輸入你的電子郵件地址，Katna 會為你找到伺服器。
add-account-servers-title = 伺服器設定
add-account-servers-intro = Katna 為 { $address } 接收和傳送郵件所使用的伺服器。
add-account-signing-in = 正在登入…
add-account-browser-title = 在瀏覽器中繼續
add-account-browser-intro = Katna 已在你的瀏覽器中開啟 { $provider } 登入頁面。請在那裡登入並允許 Katna 讀取及傳送你的郵件，然後回到這裡。
add-account-browser-hint = 沒有開啟頁面？請查看瀏覽器的視窗，或返回後再試一次。
add-account-stage-browser = 正在等候你在瀏覽器中登入…
add-account-stage-signing-in-at = 正在登入 { $server }…
add-account-help-app-password-link = 如何產生應用程式密碼
add-account-help-turn-on-imap = { $provider } 必須先在網頁版郵件的設定中開啟 IMAP 和 POP3 存取，才允許郵件應用程式連線。
add-account-help-turn-on-imap-link = 如何開啟

## Add a mail account: fields

add-account-field-address = 電子郵件地址
add-account-receive-with = 接收郵件的方式
add-account-imap-about = IMAP 會將郵件和資料夾保留在伺服器上，在每部裝置上都一樣。可以的話請選擇它。
add-account-pop3-about = POP3 會將郵件下載到這部電腦。你在這裡閱讀或移動的郵件，在伺服器和其他裝置上會維持原狀。
add-account-incoming = 內送郵件（{ $protocol }）
add-account-outgoing = 外寄郵件（{ $protocol }）
add-account-field-server = 伺服器
add-account-field-port = 連接埠
add-account-security-none = 無
add-account-security-none-warning = 未加密：你的密碼和郵件在傳輸途中可能被讀取。
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

add-account-sign-in-with = 使用 { $provider } 登入
add-account-sign-in-instead = 改用 { $provider } 登入
add-account-servers-button = 伺服器設定
add-account-back = 返回
add-account-add = 新增帳戶
add-account-done = 完成
add-account-another = 新增其他帳戶
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
add-account-app-password-refused = { $provider } 拒絕了這組密碼。它需要應用程式密碼，而不是你在網頁上使用的密碼。
add-account-password-refused = 伺服器拒絕了這組密碼。請檢查後再試一次。
add-account-sign-in-refused = { $provider } 未允許 Katna 登入。請再試一次，並允許存取你的郵件。
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] 這個版本的 Katna 目前還無法登入 Microsoft 帳戶。
    [Google] 這個版本的 Katna 目前還無法登入 Google 帳戶。
   *[other] 這個服務供應商只允許在自己的頁面上登入，Katna 目前還無法為它這麼做。
}
add-account-smtp-not-found = Katna 找到了讀取郵件的伺服器，但找不到傳送郵件的伺服器。請輸入外寄伺服器。

## Add a mail account: the last step

add-account-done-title = 你的帳戶已準備就緒
add-account-done-intro = Katna 正在取得你的郵件。新郵件送達後就會顯示。
add-account-done-sign-in = 登入方式
add-account-done-signed-in-with = 在瀏覽器中使用 { $provider } 登入
add-account-done-receiving = 接收郵件
add-account-done-sending = 傳送郵件
add-account-done-on-server = 伺服器上的郵件
add-account-done-kept = 保留到你在 Katna 中刪除為止
add-account-done-pop3-hint = 可在「設定」>「帳戶」中變更伺服器上郵件的處理方式。
add-account-done-zoho-title = 工作和日曆
add-account-done-zoho-about = Zoho 將這些與郵件分開保存。使用 Zoho 登入一次，即可將它們帶進 Katna。
add-account-done-linked = 已連結工作和日曆

## The account menu (from the account button on the top bar)

add-account-menu-another = 新增其他帳戶
app-menu = 主選單
app-menu-back = 返回
