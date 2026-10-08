# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Lines at the top of the mail list

problems-the-server = 邮件服务器
problems-signed-out = { $provider } 已将 Katna 从 { $address } 退出登录。邮件已停止同步。
problems-password-refused = { $provider } 拒绝了 { $address } 的密码。密码可能已更改。
problems-no-answer = { $provider } 没有响应 { $address } 的请求。Katna 会继续尝试。
problems-offline = 你已离线。你的邮件仍在这里，发送的邮件会等到你恢复联网后发出。
problems-accounts-need-you = { $count ->
   *[other] { $count } 个账号需要你处理
}
problems-show = 显示
problems-later = 稍后
problems-new-password = 新密码
problems-try-again = 重试

## The New password card

problems-password-title = 新密码
problems-password-detail = { $provider } 拒绝了 { $address } 已保存的密码。请输入新密码；Katna 会先验证再保存。
problems-password-placeholder = 密码
problems-password-show = 显示密码
problems-password-hide = 隐藏密码
problems-password-cancel = 取消
problems-password-save = 保存
problems-password-checking = 正在验证…
problems-password-refused-again = { $provider } 也拒绝了此密码。请检查后重试。
problems-password-saved = 已保存 { $address } 的密码。正在获取你的邮件…

## When a mail server refuses a change for good (a note at the bottom)

problems-refused-move = { $address } 的邮件服务器不接受移动 { $count ->
   *[other] { $count } 封邮件，因此它们已回到原来的位置。
}
problems-refused-flags = { $address } 的邮件服务器不接受标记 { $count ->
   *[other] { $count } 封邮件（已读、星标等），因此它们已恢复原状。
}
problems-refused-label = { $address } 的邮件服务器不接受更改 { $count ->
   *[other] { $count } 封邮件的标签，因此它们已恢复原状。
}
problems-refused-delete = { $address } 的邮件服务器不接受删除 { $count ->
   *[other] { $count } 封邮件，因此它们已恢复。
}
problems-refused-other = { $address } 的邮件服务器不接受 { $count ->
   *[other] { $count } 项更改，因此 Katna 已将其恢复原状。
}
problems-details = 详细信息

## Katna's background service (katna-daemon) isn't running

service-starting = 正在启动 Katna 后台服务…
service-failed = Katna 后台服务无法启动，因此邮件未在同步。
service-start-again = 重新启动
service-started-again = Katna 后台服务已停止，现已重新启动。
service-details-title = 服务无法启动的原因
service-details-body = 复制以下内容并随报告一起发送。其中不含任何邮件或密码。
service-details-copy = 复制
service-details-close = 关闭
