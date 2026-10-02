# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = 添加邮件账号
add-account-providers-intro = 选择你的邮件服务商，其余的交给 Katna。
add-account-provider-other = 其他邮箱
add-account-provider-other-detail = 任何 IMAP 或 POP3 账号
add-account-provider-google-detail = Gmail 和 Google Workspace
add-account-provider-microsoft-detail = Outlook 和 Microsoft 365
add-account-provider-mail = { $provider } Mail
add-account-form-title = 登录 { $provider }
add-account-form-title-other = 你的邮件账号
add-account-form-intro = Katna 会将你的密码保存在系统的密钥环中。
add-account-looking = 正在查找 { $address } 的邮件服务器…
add-account-address-intro = 输入你的电子邮件地址，Katna 会为你查找服务器。
add-account-servers-title = 服务器设置
add-account-servers-intro = Katna 为 { $address } 收取和发送邮件所用的服务器。
add-account-signing-in = 正在登录…
add-account-browser-title = 在浏览器中继续
add-account-browser-intro = Katna 已在你的浏览器中打开 { $provider } 登录页面。请在那里登录并允许 Katna 读取和发送你的邮件，然后回到这里。
add-account-browser-hint = 没有打开页面？请查看浏览器的窗口，或返回后重试。
add-account-stage-browser = 正在等待你在浏览器中登录…
add-account-stage-signing-in-at = 正在登录 { $server }…
add-account-help-app-password-link = 如何生成应用专用密码
add-account-help-turn-on-imap = 只有在 { $provider } 网页邮箱的设置中开启 IMAP 和 POP3 访问后，邮件应用才能登录。
add-account-help-turn-on-imap-link = 如何开启

## Add a mail account: fields

add-account-field-address = 电子邮件地址
add-account-receive-with = 收取邮件方式
add-account-imap-about = IMAP 将你的邮件和文件夹保存在服务器上，在所有设备上都一样。能选它时请选它。
add-account-pop3-about = POP3 会将邮件下载到这台电脑。你在这里阅读或移动的邮件，在服务器和你的其他设备上保持不变。
add-account-incoming = 收件（{ $protocol }）
add-account-outgoing = 发件（{ $protocol }）
add-account-field-server = 服务器
add-account-field-port = 端口
add-account-security-none = 无
add-account-security-none-warning = 未加密：你的密码和邮件在传输途中可能被读取。
add-account-field-username = 用户名
add-account-field-password = 密码
add-account-show-password = 显示密码
add-account-app-password-hint = { $provider } 在这里需要应用专用密码，而不是你在网页上使用的密码。请在你的 { $provider } 账号安全设置中生成一个。
add-account-field-name = 你的姓名（可选）
add-account-name-hint = 会显示给你写信的对象。
add-account-servers-pair = { $imap } 和 { $smtp }
add-account-servers-found = { $source ->
    [built-in] 服务器：{ $servers }，在 Katna 的服务商列表中找到。
    [provider] 服务器：{ $servers }，在你的邮件服务商设置中找到。
    [ispdb] 服务器：{ $servers }，在 Thunderbird 的服务商列表中找到。
    [dns] 服务器：{ $servers }，在你的域名 DNS 记录中找到。
   *[other] 服务器：{ $servers }，为推测结果；如果登录失败，请检查。
}
add-account-servers-entered = 服务器：{ $servers }，按输入内容。

## Add a mail account: buttons

add-account-sign-in-with = 使用 { $provider } 登录
add-account-sign-in-instead = 改用 { $provider } 登录
add-account-servers-button = 服务器设置
add-account-back = 返回
add-account-add = 添加账号
add-account-done = 完成
add-account-another = 添加其他账号
add-account-cancel = 取消

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] 请输入收件服务器。
   *[outgoing] 请输入发件服务器。
}
add-account-server-space = { $kind ->
    [incoming] 收件服务器名称中包含空格。
   *[outgoing] 发件服务器名称中包含空格。
}
add-account-port-invalid = { $kind ->
    [incoming] 收件端口必须是 { $min } 到 { $max } 之间的数字。
   *[outgoing] 发件端口必须是 { $min } 到 { $max } 之间的数字。
}
add-account-address-empty = 请输入电子邮件地址。
add-account-address-invalid = 请输入电子邮件地址，例如 { $example }。
add-account-not-found = Katna 找不到 { $address } 的服务器，因此填入了常用名称。请向你的邮件服务商核实。
add-account-password-empty = 请输入密码。
add-account-name-is-password = 姓名与密码相同。请在那里改为输入你希望别人看到的姓名。
add-account-app-password-refused = { $provider } 拒绝了该密码。它需要应用专用密码，而不是你在网页上使用的密码。
add-account-password-refused = 服务器拒绝了该密码。请检查后重试。
add-account-sign-in-refused = { $provider } 未允许 Katna 登录。请重试，并允许访问你的邮件。
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] 此版本的 Katna 暂时无法登录 Microsoft 账号。
    [Google] 此版本的 Katna 暂时无法登录 Google 账号。
   *[other] 此服务商只允许在其自己的页面上登录，Katna 暂时无法为其完成此操作。
}
add-account-smtp-not-found = Katna 找到了收取邮件的位置，但没有找到发送邮件的服务器。请输入发件服务器。

## Add a mail account: the last step

add-account-done-title = 你的账号已就绪
add-account-done-intro = Katna 正在获取你的邮件。新邮件到达后会立即显示。
add-account-done-sign-in = 登录方式
add-account-done-signed-in-with = 通过 { $provider }，在浏览器中
add-account-done-receiving = 收取邮件
add-account-done-sending = 发送邮件
add-account-done-on-server = 服务器上的邮件
add-account-done-kept = 保留，直到你在 Katna 中删除
add-account-done-pop3-hint = 可在“设置”>“账号”中更改服务器上邮件的处理方式。
add-account-done-zoho-title = 任务和日历
add-account-done-zoho-about = Zoho 将它们与邮件分开存放。用 Zoho 登录一次，即可将它们导入 Katna。
add-account-done-linked = 任务和日历已连接

## The account menu (from the account button on the top bar)

add-account-menu-another = 添加其他账号
app-menu = 主菜单
app-menu-back = 返回
