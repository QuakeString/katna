# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = 文件夹窗格
accounts-folder-pane-detail = 左侧窗格显示哪些账号的文件夹。
accounts-shown-one = 一次显示一个账号；在账号卡片中切换
accounts-shown-all = 所有账号，依次显示
accounts-unified = 统一收件箱
accounts-unified-switch = 将所有账号的邮件一起显示
accounts-unified-switch-detail = “所有账号”位于文件夹窗格顶部，将每个账号的收件箱、已发送邮件等列在同一个列表中。其下方的账号初始为折叠状态。
accounts-row = 账号
accounts-row-detail = 文件夹窗格和账号菜单按此顺序列出账号；第一个为默认账号。移除账号会删除 Katna 在此电脑上保存的该账号邮件副本。邮件仍保留在服务器上。
accounts-none = 还没有账号。
accounts-pop3-row = 服务器上的邮件
accounts-pop3-row-detail = POP3 账号会将邮件下载到这台电脑。请选择之后如何处理服务器上的副本。
accounts-pop3-with-katna = 保留，直到我在 Katna 中删除
accounts-pop3-at-once = 下载后立即删除
accounts-pop3-after-days = { $count ->
   *[other] { $count } 天后删除
}
accounts-pop3-never = 从不删除
accounts-pop3-days-less = 减少天数
accounts-pop3-days-more = 增加天数
accounts-kind-imported = 已导入
accounts-picture-reset = 使用桌面头像
accounts-picture-change = 更改图片
accounts-picture-remove = 移除图片
account-color-red = 红色
account-color-pink = 粉色
account-color-magenta = 品红
account-color-brown = 棕色
account-color-olive = 橄榄绿
account-color-teal = 青色
account-color-indigo = 靛蓝
account-color-slate = 石板灰
account-color-menu = 颜色
accounts-rename = 重命名
accounts-name-save = 保存
accounts-name-cancel = 取消
accounts-name-placeholder = 你的姓名
accounts-rename-failed = 无法重命名账号：{ $error }
accounts-move-up = 上移
accounts-move-down = 下移
accounts-drag = 拖动以更改顺序
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

## Reset cache (Settings > General), in the same dialog

reset-cache-about = 删除 Katna 下载的邮件和附件、发件人图片和搜索索引，然后重新下载最近的邮件。账号、设置以及只存在于此电脑上的邮件会保留。
reset-cache-button = 重置缓存
reset-cache-title = 要重置缓存吗？
reset-cache-deleted = 删除后重新下载：
reset-cache-mail = 从你的 IMAP 服务器下载的邮件和附件：最近的邮件会立即重新下载，较早的邮件在你打开时下载
reset-cache-index = 搜索索引，会立即重建
reset-cache-pictures = 发件人图片
reset-cache-kept = 保留：你的账号、密码和设置；星标、标签、已读标记和置顶；草稿、发件箱以及尚未同步到服务器的更改；以及来自 POP3 账号或导入文件的邮件，这些邮件可能没有其他副本。你的邮件服务器上不会有任何变化。
reset-cache-confirm = 重置缓存
reset-cache-busy = 正在重置…
reset-cache-done = 缓存已重置。正在重新下载最近的邮件。
reset-cache-done-freed = 缓存已重置，释放了 { $size }。正在重新下载最近的邮件。
