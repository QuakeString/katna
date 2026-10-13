# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Taking an account offline and back

offline-go-offline = 转为离线
offline-for-hour = 1 小时
offline-until-tomorrow = 直到明天
offline-until-online = 直到我手动开启
offline-go-online = 上线
offline-work-offline = 离线工作

## How an offline account shows

offline-state = 离线
offline-until = 离线至 { $time }
offline-until-day = 离线至{ $day } { $time }
offline-waiting = { $state } · { $count ->
   *[other] { $count } 项等待中
}
offline-click-online = 点击以上线
offline-account-tip = { $account } 已离线
offline-tag = 离线
offline-compose = { $account } 已离线。此邮件会在发件箱中等待，账号恢复在线后发出。

## Settings > Accounts

offline-settings-row = 已连接
offline-settings-detail = 关闭某个账号可让 Katna 停止连接它。其邮件仍保留在此处可供阅读，期间所做的操作会在你重新开启后发出。
