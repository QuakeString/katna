# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The tray icon's menu

tray-open-inbox = 打开收件箱(_I)
tray-new-message = 新邮件(_N)
tray-new-task = 新建任务(_T)
tray-new-note = 新建笔记(_O)
tray-preferences = 设置(_S)
tray-quit = 退出(_Q)

## The tray icon's tooltip, under "Katna Mail"

tray-unread = { $count ->
    [0] 没有未读邮件
   *[other] { $count } 封未读邮件
}
tray-password-refused = { $address } 需要新密码
tray-signed-out = 请重新登录 { $address }
tray-accounts-need-you = { $count } 个账号需要你处理
tray-not-sent = { $count ->
   *[other] { $count } 封邮件未发送
}
