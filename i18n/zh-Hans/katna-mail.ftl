# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = 语言：{ $language }
language-tooltip-system = 语言：{ $language }（跟随系统）
language-search = 搜索语言
language-system-default = 系统默认
language-system-now = 当前为{ $language }
language-no-match = 没有与“{ $query }”匹配的语言
language-machine = 机器翻译，欢迎帮助改进
language-setting = 语言
language-setting-detail = 菜单、按钮和消息使用的语言，以及日期和数字的格式。“系统默认”会跟随桌面设置。

## Dates and sizes

ago-just-now = 刚刚
ago-minutes = { $count } 分钟前
ago-hours = { $count } 小时前
ago-days = { $count } 天前
size-bytes = { $count } 字节
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = 隐藏文件夹
folders-show = 显示文件夹
compose = 写邮件
search = 搜索
search-mail = 搜索邮件
search-settings = 搜索设置
search-clear = 清除搜索
search-options-show = 显示搜索选项
settings = 设置
account-add = 添加账号

## App rail (and the bottom bar on a phone)

rail-mail = 邮件
rail-calendar = 日历
rail-contacts = 联系人
rail-tasks = 任务
rail-notes = 笔记
rail-feeds = 订阅源

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = 即将推出
app-calendar-promise = 您的 CalDAV 日历、邮件中的会议邀请和提醒，都在收件箱旁边。
app-tasks-promise = 与 CalDAV 同步的待办事项列表，以及由邮件创建的任务。
app-notes-promise = 快速笔记，以及为邮件或会话记下的备注，方便日后查看。
app-feeds-promise = 在邮件旁边阅读 RSS 和 Atom 订阅源。

## Contacts page

app-contacts-loading = 正在从您的邮件中收集联系人…
app-contacts-empty = 与您有邮件往来的人会显示在这里。
app-contacts-count = 来自您邮件的 { $count } 位联系人，往来最多的排在前面
app-contacts-top = 来自您邮件的前 { $count } 位联系人，往来最多的排在前面
app-contacts-messages = { $count } 封邮件
app-contacts-last = 最近：{ $date }

## Navigation (the folders pane)

nav-labels = 标签
nav-folders = 文件夹
nav-label-new = 新建标签
nav-folder-new = 新建文件夹
nav-account-unnamed = 账号 { $number }
nav-tab-new = { $count } 封新邮件

## Special folders (the user's own folders keep their names)

folder-inbox = 收件箱
folder-starred = 已加星标
folder-drafts = 草稿
folder-sent = 已发送
folder-archive = 归档
folder-spam = 垃圾邮件
folder-trash = 已删除邮件
folder-all-mail = 所有邮件
folder-scheduled = 已安排

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

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = 主要
tab-promotions = 推广
tab-social = 社交
tab-updates = 动态
tab-forums = 论坛
tab-focused = 重点
tab-other = 其他
tab-inbox = 收件箱
tab-newsletters = 新闻通讯
tab-notifications = 通知
tab-new = { $count } 封新邮件
tab-provider-other = 由 Katna 分类

## Mail list: toolbar

list-select = 选择
list-refresh = 刷新
list-more = 更多
list-mark-read = 标记为已读
list-mark-unread = 标记为未读
list-move-to = 移至
list-archive = 归档
list-spam = 举报垃圾邮件
list-delete = 删除
list-newer = 较新
list-older = 较早
list-range = 第 { $first }–{ $last } 行，共 { $total } 行
list-range-about = 第 { $first }–{ $last } 行，共约 { $total } 行
list-results = “{ $query }”的搜索结果
list-results-corrected = 显示的是“{ $query }”的搜索结果
list-search-instead = 仍然搜索“{ $query }”
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = 全部
list-pick-none = 无
list-pick-read = 已读
list-pick-unread = 未读
list-pick-starred = 已加星标
list-pick-unstarred = 未加星标

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] 已选择全部 { $count } 个会话。
   *[message] 已选择全部 { $count } 封邮件。
}
list-selected-all-in = { $kind ->
    [conversation] 已选择“{ $folder }”中的全部 { $count } 个会话。
   *[message] 已选择“{ $folder }”中的全部 { $count } 封邮件。
}
list-selected-screen = { $kind ->
    [conversation] 已选择此页上的全部 { $count } 个会话。
   *[message] 已选择此页上的全部 { $count } 封邮件。
}
list-select-all = { $kind ->
    [conversation] 选择全部 { $count } 个会话
   *[message] 选择全部 { $count } 封邮件
}
list-select-all-in = { $kind ->
    [conversation] 选择“{ $folder }”中的全部 { $count } 个会话
   *[message] 选择“{ $folder }”中的全部 { $count } 封邮件
}
list-clear-selection = 清除选择

## Mail list: empty states

list-empty-search = 没有与搜索条件匹配的邮件。
list-empty-tab = “{ $tab }”中没有邮件。
list-empty-tab-unknown = 此标签页中没有邮件。
list-empty-folder = “{ $folder }”中没有邮件。
list-empty-folder-unknown = 此文件夹中没有邮件。
list-first-sync = 正在获取您的邮件…
list-first-sync-detail = 邮件到达后会显示在这里。

## Mail list: lines

row-removed = 此邮件已被移除。
row-starred = 已加星标
row-not-starred = 未加星标
row-important = 重要。点击可标记为不重要。
row-mark-important = 标记为重要
row-pinned = 已置顶
row-pin = 置顶
row-unpin = 取消置顶

## Mail list: More menu and right-click menu

menu-reply = 回复
menu-reply-all = 全部回复
menu-forward = 转发
menu-archive = 归档
menu-delete = 删除
menu-spam = 举报垃圾邮件
menu-mark-read = 标记为已读
menu-mark-unread = 标记为未读
menu-mark-all-read = 全部标记为已读
menu-star = 加星标
menu-unstar = 移除星标
menu-important = 标记为重要
menu-not-important = 标记为不重要
menu-pin = 置顶
menu-unpin = 取消置顶
menu-print-all = 全部打印
menu-new-window = 在新窗口中打开
menu-move-to = 移至
menu-move-to-heading = 移至：
menu-find-from = 查找来自 { $name } 的邮件

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] 已归档 { $count } 个会话。
   *[message] 已归档 { $count } 封邮件。
}
toast-trashed = { $kind ->
    [conversation] 已将 { $count } 个会话移至已删除邮件。
   *[message] 已将 { $count } 封邮件移至已删除邮件。
}
toast-moved = { $kind ->
    [conversation] 已移动 { $count } 个会话。
   *[message] 已移动 { $count } 封邮件。
}
toast-starred = { $kind ->
    [conversation] 已为 { $count } 个会话加星标。
   *[message] 已为 { $count } 封邮件加星标。
}
toast-unstarred = { $kind ->
    [conversation] 已移除 { $count } 个会话的星标。
   *[message] 已移除 { $count } 封邮件的星标。
}
toast-important = { $kind ->
    [conversation] 已将 { $count } 个会话标记为重要。
   *[message] 已将 { $count } 封邮件标记为重要。
}
toast-not-important = { $kind ->
    [conversation] 已将 { $count } 个会话标记为不重要。
   *[message] 已将 { $count } 封邮件标记为不重要。
}
toast-pinned = { $kind ->
    [conversation] 已置顶 { $count } 个会话。
   *[message] 已置顶 { $count } 封邮件。
}
toast-unpinned = { $kind ->
    [conversation] 已取消置顶 { $count } 个会话。
   *[message] 已取消置顶 { $count } 封邮件。
}
toast-spam = { $kind ->
    [conversation] 已将 { $count } 个会话举报为垃圾邮件。
   *[message] 已将 { $count } 封邮件举报为垃圾邮件。
}
toast-deleted-forever = { $kind ->
    [conversation] 已永久删除 { $count } 个会话。
   *[message] 已永久删除 { $count } 封邮件。
}
toast-undone = 已撤消操作。
toast-undo = 撤消
toast-no-spam-folder = 此账号没有垃圾邮件文件夹。

## Reading pane: toolbar

reader-close = 关闭
reader-back = 返回
reader-mark-unread = 标记为未读
reader-move-to = 移至
reader-more = 更多
reader-print-all = 全部打印
reader-new-window = 在新窗口中打开
reader-position = 第 { $position } 个，共 { $total } 个
reader-newer = 较新
reader-older = 较早

## Reading pane: the conversation

reader-removed = 此会话已被移除。
reader-no-subject = （无主题）
reader-collapse-all = 全部收起
reader-expand-all = 全部展开
reader-unknown-sender = （未知发件人）
reader-date-ago = { $date }（{ $ago }）
reader-me = 我
reader-to = 发送至 { $names }
reader-starred = 已加星标
reader-not-starred = 未加星标
reader-too-long = 邮件太长，无法完整显示。
reader-encrypted-images = 加密邮件中绝不会加载网络图片。
reader-window-failed = 无法打开新窗口。

## Reading pane: message details (opened from "to me")

reader-details-from = 发件人：
reader-details-to = 收件人：
reader-details-cc = 抄送：
reader-details-date = 日期：
reader-details-subject = 主题：

## Reading pane: downloading a message

reader-downloading = 正在从服务器下载此邮件…
reader-download-failed = 无法下载此邮件。
reader-try-again = 重试

## Reply row

reply-reply = 回复
reply-reply-all = 全部回复
reply-forward = 转发

## Encrypted and signed mail

security-decrypting = 正在解密…
security-checking = 正在检查签名…
security-partly-encrypted = 此邮件只有部分内容经过加密。其余内容是在保护范围之外添加的，可能来自任何人。
security-partly-signed = 此邮件只有部分内容经过签名。其余内容是在保护范围之外添加的，可能来自任何人。
security-encrypted = 加密邮件
security-encrypted-smime = 加密邮件（S/MIME）
security-no-key = 无法解密此邮件：它是用您没有的密钥加密的。
security-cancelled = 已取消解密。
security-damaged = 无法解密此邮件：加密数据已损坏或被更改。
security-decrypt-unavailable = 无法解密此邮件：请安装 { $tool } 以阅读加密邮件。
security-decrypt-failed = 无法解密此邮件：{ $reason }
security-unknown-signer = 未知签名者
security-signed-verified = 由 { $signer } 签名 · 已验证
security-signed-not-sender = 由 { $signer } 签名，但此人不是发件人
security-signed-untrusted = 由 { $signer } 签名，使用的密钥已被您标记为不可信
security-signed-unverified = 由 { $signer } 签名 · 密钥未经验证
security-bad-signature = 签名无效：此邮件在签名后被更改过，或签名是伪造的。
security-signature-expired = 由 { $signer } 签名 · 签名已过期
security-key-expired = 由 { $signer } 签名 · 密钥此后已过期
security-key-revoked = 由 { $signer } 签名，使用的密钥已被吊销
security-missing-key = 使用您没有的密钥签名，因此无法检查
security-missing-key-id = 使用您没有的密钥（{ $key }）签名，因此无法检查
security-signature-unavailable = 已签名；请安装 { $tool } 以检查签名
security-signature-error = 无法检查签名。

## Remote images and pictures

remote-hidden = 此邮件中的图片已隐藏。
remote-show = 显示图片
remote-always-show = 始终显示此发件人的图片
remote-picture-use = 使用
remote-picture-too-big = 请选择不超过 8 MB 的图片。
remote-picture-type = 请选择 PNG、JPEG、GIF、WebP 或 SVG 图片。
remote-picture-read-failed = 无法读取图片：{ $error }
remote-picture-keep-failed = 无法保存图片：{ $error }
remote-picture-remove-failed = 无法移除图片：{ $error }

## Attachments

attachment-count = { $count } 个附件
attachment-save = 保存
attachment-save-all = 全部保存
attachment-save-all-tooltip = 将所有附件保存到一个文件夹
attachment-save-here = 保存到此处
attachment-not-downloaded = 此邮件未下载。
attachment-not-found = 在邮件中找不到此附件。
attachment-read-failed = 无法读取 { $name }
attachment-numbered = 附件 { $number }
attachment-saved-all = 已将 { $count } 个文件保存到 { $place }
attachment-saved-some = 已将 { $saved } 个文件（共 { $total } 个）保存到 { $place }。无法保存 { $failed }
attachment-saved-to = 已保存到 { $path }
attachment-save-failed = 无法保存 { $name }：{ $error }
attachment-open-failed = 无法打开 { $name }：{ $error }
attachment-risky = 此文件可能会运行程序，因此 Katna 不会打开它。请改为保存。
attachment-encrypted-open = 此文件是以加密形式收到的。请保存后在其他地方打开。

## Printing

print-failed = 无法打印：{ $error }
print-no-font = 找不到字体
print-opened-as-pdf = 已作为 PDF 打开，请从那里打印。
print-not-downloaded = （尚未下载。）
print-encrypted = （已加密。请在 Katna Mail 中打开以打印其文本。）
print-to = 收件人：{ $addresses }
print-cc = 抄送：{ $addresses }
