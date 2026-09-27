# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = 文件夹窗格
accounts-folder-pane-detail = 左侧窗格显示哪些账号的文件夹。
accounts-shown-one = 一次显示一个账号；在账号卡片中切换
accounts-shown-all = 所有账号，依次显示
accounts-row = 账号
accounts-row-detail = 移除账号会删除 Katna 在此电脑上保存的该账号邮件副本。邮件仍保留在服务器上。
accounts-none = 还没有账号。
accounts-kind-imported = 已导入
accounts-picture-reset = 使用桌面头像
accounts-picture-change = 更改图片
accounts-remove = 移除
accounts-delete-all-row = 删除所有数据
accounts-delete-all-row-detail = 从头开始，如同全新安装。
accounts-delete-all-about = 从此电脑中删除所有账号、所有已保存的邮件、联系人和日历、搜索索引、你的设置以及已保存的密码。你的邮件服务器上不会有任何变化。
accounts-delete-all-open = 删除所有 Katna 数据

## Settings > Accounts: snackbars after deleting

accounts-removed-local = 已从 Katna 中移除 { $address }。
accounts-removed = 已从 Katna 中移除 { $address }。其邮件仍在服务器上。
accounts-all-deleted = 已从此电脑中删除所有 Katna 数据。

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = 要移除 { $address } 吗？
accounts-remove-confirm = 移除账号
accounts-removing = 正在移除…
accounts-remove-local-mail = { $folders ->
    [0] 导入到此账号的所有邮件
   *[other] 导入到此账号 { $folders } 个文件夹中的所有邮件
}
accounts-remove-local-settings = 此账号的 Katna 设置
accounts-remove-mail = { $folders ->
    [0] Katna 保存的此账号所有邮件
   *[other] Katna 保存在此账号 { $folders } 个文件夹中的所有邮件
}
accounts-remove-outbox = 此账号在发件箱中等待发送的邮件
accounts-remove-settings = 此账号已保存的密码和 Katna 设置
accounts-delete-all-title = 要删除所有 Katna 数据吗？
accounts-delete-all-confirm = 全部删除
accounts-deleting = 正在删除…
accounts-delete-all-accounts = 所有账号，以及 Katna 保存的所有邮件和附件
accounts-delete-all-contacts = 联系人、日历和搜索索引
accounts-delete-all-settings = 所有设置、签名和键盘快捷键
accounts-delete-all-passwords = 所有已保存的密码
accounts-deleted-heading = 将从此电脑中删除：
accounts-cannot-undo = 此操作无法撤消。
accounts-server-delete-all = 你的邮件服务器上不会有任何变化：邮件仍保留在那里，再次添加账号会重新下载。从文件导入的邮件只存在于 Katna 中；原文件不受影响。
accounts-server-local = 这些邮件是从文件导入的，因此只有 Katna 中有副本。原文件不受影响；重新导入即可恢复。
accounts-server-remove = 邮件服务器上不会有任何变化：邮件仍保留在那里，再次添加此账号会重新下载。
accounts-confirm-word = 删除
accounts-confirm-placeholder = 输入“{ accounts-confirm-word }”
accounts-confirm-prompt = 请输入“{ accounts-confirm-word }”以确认：
accounts-cancel = 取消
