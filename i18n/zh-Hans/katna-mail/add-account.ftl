# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = 添加邮件账号
add-account-looking = 正在查找 { $address } 的邮件服务器…
add-account-address-intro = 输入你的电子邮件地址，Katna 会为你查找服务器。
add-account-servers-title = 服务器设置
add-account-servers-intro = Katna 为 { $address } 收取和发送邮件所用的服务器。
add-account-password-title = 输入你的密码
add-account-signing-in = 正在登录…
add-account-browser-title = 在浏览器中继续
add-account-browser-intro = Katna 已在你的浏览器中打开 { $provider } 登录页面。请在那里登录并允许 Katna 读取和发送你的邮件，然后回到这里。
add-account-browser-hint = 没有打开页面？请查看浏览器的窗口，或返回后重试。

## Add a mail account: fields

add-account-field-address = 电子邮件地址
add-account-incoming = 收件（{ $protocol }）
add-account-outgoing = 发件（{ $protocol }）
add-account-field-server = 服务器
add-account-field-port = 端口
add-account-security-none = 无
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
add-account-or = 或
add-account-sign-in-with = 使用 { $provider } 登录
add-account-sign-in-instead = 改用 { $provider } 登录

## Add a mail account: buttons

add-account-servers-button = 服务器设置
add-account-back = 返回
add-account-add = 添加账号
add-account-next = 下一步
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
add-account-added = 已添加 { $address }。正在收取你的邮件…
add-account-app-password-refused = { $provider } 拒绝了该密码。它需要应用专用密码，而不是你在网页上使用的密码。
add-account-password-refused = 服务器拒绝了该密码。请检查后重试。
add-account-sign-in-refused = { $provider } 未允许 Katna 登录。请重试，并允许访问你的邮件。
add-account-sign-in-unavailable = { $provider ->
    [Microsoft] 此版本的 Katna 暂时无法登录 Microsoft 账号。
    [Google] 此版本的 Katna 暂时无法登录 Google 账号。
   *[other] 此服务商只允许在其自己的页面上登录，Katna 暂时无法为其完成此操作。
}
add-account-signed-in = 已使用 { $provider } 登录。正在收取你的邮件…

## The account menu (from the account button on the top bar)

add-account-menu-another = 添加其他账号
add-account-menu-manage = 管理账号
app-menu = 主菜单
