# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = 标签
nav-folders = 文件夹
nav-label-new = 新建标签
nav-folder-new = 新建文件夹
nav-menu-check-mail = 检查新邮件
nav-menu-check-inbox = 检查此收件箱
nav-unified-leave-out = 不在统一收件箱中显示
nav-unified-bring-back = 重新在统一收件箱中显示
nav-menu-sign-in-again = 重新登录
nav-menu-new-mail = 从此账号写新邮件
nav-menu-account-settings = 账号设置
nav-account-checked = 已同步 · { $ago }检查
nav-account-in-sync = 已同步
nav-account-connecting = 正在连接…
nav-account-offline = 离线，正在重试
nav-account-signed-out = { $provider } 登录已过期
nav-account-password-refused = 密码被拒绝
nav-account-storage = 已使用 { $used }，共 { $total }
nav-menu-new-subfolder = 在其中新建文件夹
nav-menu-new-sublabel = 在其中新建标签
nav-menu-rename = 重命名
nav-menu-delete = 删除
nav-menu-empty-trash = 清空已删除邮件
nav-account-unnamed = 账号 { $number }
nav-all-accounts = 所有账号
nav-expand = 显示文件夹
nav-collapse = 隐藏文件夹
storage-used = 已使用 { $total } 中的 { $percent }%
storage-used-detail = { $address }：已使用 { $total } 中的 { $used }

## Special folders (the user's own folders keep their names)

folder-inbox = 收件箱
folder-starred = 已加星标
folder-snoozed = 已延后
folder-unread = 未读
folder-important = 重要
folder-drafts = 草稿
folder-sent = 已发送
folder-archive = 归档
folder-spam = 垃圾邮件
folder-trash = 已删除邮件
folder-all-mail = 所有邮件
folder-scheduled = 已安排
folder-waiting = 等待回复
folder-waiting-short = 等待中
folder-reminders = 提醒
folder-outbox = 发件箱
folder-activity = 动态
folder-not-on-account = 此账号没有这个文件夹。

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = 新建标签
label-folder-new-title = 新建文件夹
label-prompt = 请输入新标签名称：
label-folder-prompt = 请输入新文件夹名称：
label-name-hint = 标签名称
label-folder-name-hint = 文件夹名称
label-nest = 将标签嵌套在以下标签之下：
label-folder-nest = 将文件夹嵌套在以下文件夹之下：
label-cancel = 取消
label-create = 创建
label-creating = 正在创建…
label-created = 已创建标签“{ $name }”。
label-folder-created = 已创建文件夹“{ $name }”。
label-rename-title = 重命名标签
label-folder-rename-title = 重命名文件夹
label-rename = 重命名
label-renaming = 正在重命名…
label-renamed = 标签已重命名为“{ $name }”。
label-folder-renamed = 文件夹已重命名为“{ $name }”。

## Deleting a folder or label (asked first)

folder-delete-title = 要删除“{ $name }”吗？
folder-delete-body = { $count ->
    [0] 其中没有邮件。该文件夹将从服务器上移除，因此网页邮箱和你的手机上也会随之消失。
   *[other] { $kind ->
        [conversation] 其中的 { $count } 个会话将移至“已删除邮件”，你仍可将其找回。
       *[message] 其中的 { $count } 封邮件将移至“已删除邮件”，你仍可将其找回。
    }该文件夹将从服务器上移除，因此网页邮箱和你的手机上也会随之消失。
}
folder-delete-forever-body = { $count ->
    [0] 其中没有邮件。该文件夹将从服务器上移除，因此网页邮箱和你的手机上也会随之消失。
   *[other] { $kind ->
        [conversation] 其中的 { $count } 个会话将被永久删除；此账号没有“已删除邮件”文件夹。
       *[message] 其中的 { $count } 封邮件将被永久删除；此账号没有“已删除邮件”文件夹。
    }该文件夹将从服务器上移除，因此网页邮箱和你的手机上也会随之消失。
}
folder-delete-label-body = 该标签将被移除。其中的邮件仍保留在“所有邮件”及其其他标签中。
folder-delete-confirm = 删除文件夹
folder-delete-label-confirm = 删除标签
folder-deleted = 已删除文件夹“{ $name }”
label-deleted = 已删除标签“{ $name }”
