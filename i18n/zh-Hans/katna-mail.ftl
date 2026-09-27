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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = 打开此邮件即可查看其附件。
text-copy = 复制
text-select-all = 全选

## Settings page: its tabs

settings-tab-general = 常规
settings-tab-inbox = 收件箱
settings-tab-accounts = 账号
settings-tab-subscriptions = 订阅
settings-tab-appearance = 外观
settings-tab-shortcuts = 快捷键
settings-tab-default-apps = 默认应用
settings-tab-folders-rules = 文件夹和规则
settings-tab-compose = 写邮件
settings-tab-mcp-server = MCP 服务器
settings-tab-feedback = 用户反馈
settings-tab-experimental = 实验性功能

## Settings page: tabs still to come

settings-tab-subscriptions-coming = 查看你收到的新闻通讯和邮件列表，一键退订。
settings-tab-folders-rules-coming = 创建、重命名、移动和隐藏文件夹和标签，并选择要同步哪些。规则会按发件人、主题或字词自动对新邮件进行分类、添加标签、转发或删除。
settings-tab-mcp-server-coming = 经你同意后，让此电脑上的 AI 助手搜索、阅读和起草你的邮件。

## Settings > General

settings-general-conversations = 会话视图
settings-general-conversations-group = 将对同一邮件的回复归为一组
settings-general-conversations-group-detail = 列表中每个会话显示为一行
settings-general-reading = 阅读
settings-general-newest-first = 最新邮件优先
settings-general-newest-first-detail = 会话从最新的回复开始显示
settings-general-full-headers = 显示完整邮件头
settings-general-full-headers-detail = 每封邮件都展开显示发件人、收件人、抄送、日期和主题
settings-general-full-names = 收件人全名
settings-general-full-names-detail = 显示“发送至 我、Ada Lovelace”而不是“发送至 我、Ada”
settings-general-mark-read = 标记为已读
settings-general-mark-read-now = 打开后立即标记
settings-general-mark-read-1s = 打开 1 秒后
settings-general-mark-read-3s = 打开 3 秒后
settings-general-mark-read-never = 仅在我手动标记时
settings-general-reply-button = 回复按钮
settings-general-reply-all = 回复所有人
settings-general-reply-all-detail = 每封邮件旁的回复按钮会回复所有人，而不仅是发件人
settings-general-remote-images = 网络图片
settings-general-remote-images-detail = 加载邮件中的图片会让发件人知道你打开了邮件、打开的时间和大致位置。关闭后，每封邮件都会先询问，你也随时可以显示某个发件人的图片。
settings-general-remote-images-always = 始终显示图片
settings-general-remote-images-always-detail = 在所有邮件中显示，而不仅是来自你信任的发件人的邮件
settings-general-sending = 发送
settings-general-sending-detail = 已发送的邮件会等待一段时间，以便撤回。
settings-general-offline = 离线邮件
settings-general-offline-detail = 最近的邮件会完整下载，无需联网即可阅读。较早的邮件在打开时下载。
settings-general-offline-days = { $count } 天
settings-general-offline-years = { $count } 年
settings-general-offline-all = 所有邮件
settings-general-offline-note = 减少天数会保留已下载的邮件。服务器上不会有任何变化。
settings-general-notifications = 通知
settings-general-notifications-detail = 收件箱中有新邮件时通知，即使 Katna Mail 已关闭。
settings-general-new-mail = 收到新邮件时通知我
settings-general-new-mail-detail = 带有“全部回复”“标记为已读”和“归档”按钮
settings-general-new-mail-sound = 播放提示音
settings-general-new-mail-sound-detail = 桌面的新邮件提示音
settings-general-desktop = 桌面
settings-general-open-at-login = 登录时打开 Katna Mail
settings-general-open-at-login-detail = 无论是否打开，只要服务在运行，登录时都会同步邮件
settings-general-tray = 在系统托盘中显示 Katna
settings-general-tray-detail = 显示未读数并提供菜单
settings-general-unread-badge = 在任务栏图标上显示未读数
settings-general-unread-badge-detail = 收件箱中未读邮件的数量

## Settings > Inbox

settings-inbox-tabs = 收件箱标签页
settings-inbox-tabs-detail = 像你的邮件服务商网站那样，将收件箱分到不同的标签页。
settings-inbox-tabs-show = 显示收件箱标签页
settings-inbox-tabs-show-detail = 关闭后，每个账号只显示一个列表
settings-inbox-no-accounts = 添加账号后即可选择其标签页。
settings-inbox-tabs-automatic = 自动：{ $tabs }（{ $provider }）
settings-inbox-tabs-off = 无标签页
settings-inbox-tabs-gmail = 主要、推广、社交、动态、论坛
settings-inbox-tabs-focused = 重点和其他
settings-inbox-tabs-zoho = 收件箱、新闻通讯和通知
settings-inbox-tabs-shown = 要显示的标签页。关闭的标签页中的邮件会留在“{ $tab }”中。

## Settings > Appearance

settings-appearance-reading-pane = 阅读窗格
settings-appearance-reading-pane-detail = 打开的会话显示的位置。
settings-appearance-pane-right = 列表右侧
settings-appearance-pane-none = 不拆分
settings-appearance-density = 显示密度
settings-appearance-density-default = 默认
settings-appearance-density-compact = 紧凑
settings-appearance-scaling = 缩放
settings-appearance-scaling-detail = 在桌面自身缩放的基础上，放大或缩小 Katna Mail 中的所有内容：文字、图标、间距和分隔线。你发送的邮件保持其原有字号。缩得太小可能会让图标难以点击。
settings-appearance-theme = 主题
settings-appearance-theme-system = 与桌面相同
settings-appearance-theme-light = 浅色
settings-appearance-theme-dark = 深色
settings-appearance-desktop-colors = 桌面颜色
settings-appearance-desktop-colors-use = 使用桌面颜色
settings-appearance-desktop-colors-use-detail = 桌面的配色方案和强调色
settings-appearance-app-names = 应用名称
settings-appearance-app-names-show = 显示应用名称
settings-appearance-app-names-show-detail = 在最左侧的应用图标下方显示名称
settings-appearance-sender-pictures = 发件人图片
settings-appearance-sender-pictures-show = 显示公司徽标
settings-appearance-sender-pictures-show-detail = 按发件人的域名查找（从不按邮件查找），并保留一周
settings-appearance-important = 重要标记
settings-appearance-important-show = 显示重要标记
settings-appearance-important-show-detail = 显示在列表中每封邮件旁边
settings-appearance-message-width = 邮件宽度
settings-appearance-message-width-limit = 限制邮件宽度
settings-appearance-message-width-limit-detail = 在宽窗口中，长行更易于阅读
settings-appearance-mail-colors = 邮件颜色
settings-appearance-mail-colors-detail = 大多数邮件是为白色页面设计的。使用深色主题时，其颜色会改为便于阅读的深色；关闭后，邮件在浅色页面上保留发件人设定的颜色。
settings-appearance-dark-mail = 邮件也使用深色
settings-appearance-dark-mail-detail = 仅在深色主题下
settings-appearance-attachment-previews = 附件预览
settings-appearance-attachment-previews-show = 显示附件预览
settings-appearance-attachment-previews-show-detail = 在每个文件的卡片上显示其内容的小图

## Settings > Default apps

settings-default-apps-intro = 点击附件时用来打开它的应用。你也随时可以在查看器中用其他应用打开文件。桌面的默认应用在桌面自己的设置中设定。
settings-default-apps-pdf = PDF 文件
settings-default-apps-pdf-detail = 分页显示，可缩放。
settings-default-apps-pictures = 图片
settings-default-apps-pictures-detail = 照片（自动转正）、PNG、GIF、WebP、BMP、TIFF 和 SVG。
settings-default-apps-text = 文本文件
settings-default-apps-text-detail = 纯文本、日志、代码和其他文本。
settings-default-apps-sheets = 电子表格
settings-default-apps-sheets-detail = Excel（xlsx、xls）、OpenDocument（ods）和 CSV。
settings-default-apps-documents = 文档
settings-default-apps-documents-detail = Word（docx）和 OpenDocument 文本（odt）。
settings-default-apps-katna = Katna Mail 查看器
settings-default-apps-system = 桌面的默认应用
settings-default-apps-ask = 每次询问使用哪个应用
settings-default-apps-after-saving = 保存后
settings-default-apps-show-folder = 在文件夹中显示已保存的文件
settings-default-apps-show-folder-detail = 打开文件管理器并选中已保存的附件

## Settings > Compose

settings-compose-send-from = 发送新邮件时使用
settings-compose-send-from-detail = 回复和转发始终从你当前所在的账号发出。
settings-compose-send-from-current = 当前所在的账号
settings-compose-send-on-replies = 回复时发送
settings-compose-send-on-replies-detail = 回复或转发时“发送”按钮执行的操作。“发送”旁的菜单提供另一种操作。
settings-compose-send-plain = 发送
settings-compose-send-archive = 发送并归档
settings-compose-signatures = 签名
settings-compose-signatures-detail = 添加在邮件正文下方的“--”行之后。可在写邮件窗口中选择其他签名。
settings-compose-untitled = 未命名
settings-compose-signature-name = 名称，例如“工作”
settings-compose-signature-first = 我的签名
settings-compose-signature-numbered = 签名 { $number }
settings-compose-signature-delete = 删除
settings-compose-signature-deleted = 已删除签名
settings-compose-signature-new = 新建
settings-compose-no-signatures = 还没有签名。
settings-compose-no-signature = 无签名
settings-compose-for-new-mail = 用于新邮件
settings-compose-for-replies = 用于回复和转发
settings-compose-for-replies-detail = 在你曾签名的会话中，回复会改用那个签名。
settings-compose-format = 格式
settings-compose-plain-text = 以纯文本撰写
settings-compose-plain-text-detail = 新邮件默认不带格式；可在写邮件窗口中切换
settings-compose-spelling = 拼写
settings-compose-spell-check = 撰写时检查拼写
settings-compose-spell-check-detail = 拼错的字词会加下划线，右键点击可查看建议
settings-compose-spell-desktop = 桌面的语言（{ $language }）
settings-compose-templates = 模板
settings-compose-templates-detail = 保存你经常写的邮件，并以它开始新邮件或回复。

## Settings > Shortcuts

settings-shortcuts-set = 快捷键方案
settings-shortcuts-set-detail = 从你熟悉的邮件应用的按键开始。这里的 Cmd 即 Ctrl。你自己的更改优先于方案，“恢复默认设置”会还原为方案的按键。
settings-shortcuts-single = 单键快捷键
settings-shortcuts-single-detail = 与网页邮箱一样，不带 Ctrl 或 Alt 的按键：e 归档，j 和 k 移动，/ 搜索。它们在列表和打开的会话中有效，输入文字时无效。
settings-shortcuts-single-use = 使用单键快捷键
settings-shortcuts-single-use-detail = Ctrl 快捷键始终有效
settings-shortcuts-how = 点击按键可更改，点击 + 可添加，然后按下新按键。按 Esc 取消。
settings-shortcuts-restore = 恢复默认设置
settings-shortcuts-no-key = 无按键
settings-shortcuts-press = 请按键…
settings-shortcuts-then = { $keys }，然后…
settings-shortcuts-moved = { $keys } 现在执行“{ $action }”，而不是“{ $previous }”。
settings-shortcuts-single-off = 单键快捷键已关闭，开启后此按键才会生效。
settings-shortcuts-restored = 所有快捷键已恢复为方案的按键。

## Settings search: the line under a result

settings-general-language-summary = 应用、日期和数字的语言
settings-general-reading-summary = 最新邮件优先、完整邮件头、收件人全名
settings-general-mark-read-summary = 打开的会话何时标记为已读：立即、1 秒或 3 秒后，或手动标记
settings-general-reply-button-summary = 每封邮件旁的回复按钮回复所有人
settings-general-remote-images-summary = 始终显示所有邮件中的图片
settings-general-sending-summary = 撤消发送：已发送的邮件等待多久，以便撤回
settings-general-offline-summary = 完整下载最近多少天的邮件，以便离线阅读
settings-general-notifications-summary = 新邮件通知及其提示音
settings-general-desktop-summary = 登录时打开 Katna Mail、系统托盘图标和任务栏图标上的未读数
settings-accounts-accounts-summary = 添加或移除账号，或更改其图片
settings-appearance-density-summary = 列表中的行采用默认或紧凑样式
settings-appearance-scaling-summary = 放大或缩小所有内容：文字、图标、间距和分隔线
settings-appearance-theme-summary = 与桌面相同、浅色或深色
settings-appearance-sender-pictures-summary = 按发件人域名查找的公司徽标
settings-appearance-important-summary = 列表中每封邮件旁的重要标记
settings-appearance-mail-colors-summary = 在深色主题下为 HTML 邮件使用深色，或保留发件人的颜色
settings-appearance-attachment-previews-summary = 每个附件内容的小图
settings-shortcuts-set-summary = 从 Gmail、Inbox by Gmail、Apple Mail、Outlook 或 Thunderbird 的按键开始
settings-shortcuts-single-summary = 与网页邮箱一样，不带 Ctrl 或 Alt 的按键
settings-default-apps-pdf-summary = PDF 附件的打开方式
settings-default-apps-pictures-summary = 照片和图片的打开方式
settings-default-apps-text-summary = 纯文本、日志和代码的打开方式
settings-default-apps-sheets-summary = Excel、OpenDocument 和 CSV 文件的打开方式
settings-default-apps-documents-summary = Word 和 OpenDocument 文本的打开方式
settings-default-apps-after-saving-summary = 在文件夹中显示已保存的附件
settings-compose-send-from-summary = 新邮件从哪个账号发出：你当前所在的账号，或始终使用同一个账号
settings-compose-send-on-replies-summary = 回复和转发时“发送”，或“发送并归档”会话
settings-compose-signatures-summary = 添加在邮件正文下方的“--”行之后
settings-compose-for-new-mail-summary = 新邮件默认使用的签名
settings-compose-for-replies-summary = 回复和转发默认使用的签名
settings-compose-format-summary = 以纯文本撰写新邮件
settings-compose-spelling-summary = 撰写时检查拼写，以及词典的语言
settings-compose-templates-summary = 即将推出：保存你经常写的邮件，并以它开始新邮件或回复
settings-feedback-crash-reports-summary = Katna Mail 或其后台服务崩溃时，将崩溃报告保存在此电脑上
settings-feedback-saved-summary = 查看、复制或删除保存在此电脑上的崩溃报告
settings-feedback-help-improve-summary = 发送崩溃报告以帮助修复问题；除非你开启，否则保持关闭
settings-experimental-blur-summary = 透过顶栏可看到模糊的桌面，菜单呈磨砂玻璃效果
settings-search-shortcut = 键盘快捷键
settings-search-tab = 设置标签页
settings-search-none = 没有与“{ $query }”匹配的设置。
settings-search-results = 与“{ $query }”匹配的设置

## Quick settings (the panel that slides in from the right)

quick-title = 快速设置
quick-see-all = 查看所有设置
quick-reading-pane = 阅读窗格
quick-pane-right = 列表右侧
quick-pane-none = 不拆分
quick-density = 显示密度
quick-density-default = 默认
quick-density-compact = 紧凑
quick-theme = 主题
quick-theme-system = 与桌面相同
quick-theme-light = 浅色
quick-theme-dark = 深色
quick-desktop-colors = 桌面颜色
quick-desktop-colors-detail = 桌面的配色方案和强调色
quick-app-names = 应用名称
quick-app-names-detail = 在最左侧的应用图标下方显示名称
quick-inbox-tabs = 收件箱标签页
quick-inbox-tabs-detail = 每个账号的邮件服务商的标签页
quick-choose-tabs = 选择标签页
quick-choose-tabs-detail = 在设置中按账号选择
quick-sending = 发送
quick-undo-send = 撤消发送
quick-undo-send-off = 关闭
quick-undo-send-seconds = { $seconds } 秒
quick-signatures = 签名
quick-signatures-none = 暂无
quick-signatures-one = { $name }，默认使用
quick-signatures-many = { $count } 个签名；默认为 { $name }
quick-signatures-no-default = { $count } 个，无默认签名
quick-signature-untitled = 未命名
quick-threading = 邮件会话
quick-conversation-view = 会话视图
quick-conversation-view-detail = 将对同一邮件的回复归为一组
quick-help = 帮助
quick-tour = 参观导览
quick-whats-new = 新功能
quick-about = 关于 Katna

## Settings: opening at login

settings-open-at-login-failed = 无法更改登录时打开的设置：{ $error }

## Settings > Appearance > Scaling

scale-letter = 字
scale-percent = { $percent }%
scale-reset = 恢复为 { $percent }%

## Settings > Experimental > Look & Feel

look-intro = 仍在试验中的功能。它们可能会更改或移除。
look-heading = 外观和感觉
look-window-frame = 窗口边框
look-window-frame-detail = 由谁绘制标题栏、窗口按钮、圆角和阴影。
look-frame-native-kde = 原生：KDE 的边框，使用你的 Plasma 主题
look-frame-native = 原生：桌面的边框
look-frame-katna = Katna：顶栏变为标题栏
look-frame-katna-note-named = Katna 会绘制圆角和自己的阴影。边框不再跟随 { $desktop } 主题；窗口规则仍然有效。
look-frame-katna-note = Katna 会绘制圆角和自己的阴影。边框不再跟随桌面主题；窗口规则仍然有效。
look-frame-client-side = 你的桌面让每个应用自行绘制边框，因此 Katna 已在绘制自己的边框。
look-blurred-background = 模糊背景
look-blurred-background-detail = 透过顶栏和文件夹可看到模糊的桌面，菜单和弹出框呈磨砂玻璃效果。
look-blur = 模糊窗口后方的内容
look-blur-detail = 邮件仍显示在不透明的卡片上，因此文字保持清晰的对比度
look-blur-off-kde = KDE 的模糊特效已关闭。请在“系统设置”>“窗口管理”>“桌面特效”中开启“模糊”，然后重新打开 Katna Mail。
look-blur-none-gnome = GNOME 不会模糊窗口后方的内容。
look-blur-none-x11 = 你的窗口管理器不会模糊窗口后方的内容。
look-blur-none-wayland = 你的合成器不会模糊窗口后方的内容。

## Settings > User feedback (crash reports)

feedback-intro-sending = 新的崩溃报告会被发送，以帮助修复问题。除此之外，不会有任何内容离开此电脑。
feedback-intro-local = Katna 不会向任何地方发送任何内容。崩溃报告保存在此电脑上，供你查看或附加到错误报告中。
feedback-crash-reports = 崩溃报告
feedback-crash-reports-detail = 在 Katna Mail 或其后台服务崩溃时生成。
feedback-save = 将崩溃报告保存在此电脑上
feedback-save-detail = 不包含你的主文件夹、用户名、计算机名和电子邮件地址
feedback-saved = 已保存的崩溃报告
feedback-saved-detail = 保留最新的 { $count } 份。
feedback-help-improve = 帮助改进 Katna
feedback-help-improve-detail = 除非你开启，否则保持关闭；你随时可以在这里关闭。
feedback-send = 发送崩溃报告
feedback-send-detail = 已保存的报告会按你在这里看到的原样发送到 Katna 的崩溃跟踪服务（Sentry，位于欧盟）。不包含 IP 地址、邮件或电子邮件地址
feedback-none-saved = 没有已保存的崩溃报告。
feedback-delete-all = 全部删除
feedback-app-daemon = 后台服务
feedback-report-sent = { $date } · 已发送
feedback-view = 查看
feedback-view-tooltip = 打开报告
feedback-copy-tooltip = 复制后粘贴到错误报告中
feedback-copied = 已复制崩溃报告。
feedback-deleted-all = 已删除崩溃报告。
feedback-read-failed = 无法读取崩溃报告：{ $error }
feedback-delete-failed = 无法删除崩溃报告：{ $error }
feedback-delete-all-failed = 无法删除崩溃报告：{ $error }

## Menu bar (the KDE global menu)

desktop-menu-file = 文件(_F)
desktop-menu-new-message = 新邮件(_N)
desktop-menu-quit = 退出(_Q)
desktop-menu-edit = 编辑(_E)
desktop-menu-undo = 撤消(_U)
desktop-menu-select-all = 全选(_A)
desktop-menu-select-none = 取消全选(_N)
desktop-menu-find = 查找(_F)…
desktop-menu-view = 查看(_V)
desktop-menu-folder-list = 显示文件夹列表(_F)
desktop-menu-refresh = 刷新(_R)
desktop-menu-go = 转到(_G)
desktop-menu-inbox = 收件箱(_I)
desktop-menu-starred = 已加星标(_S)
desktop-menu-sent = 已发送(_E)
desktop-menu-drafts = 草稿(_D)
desktop-menu-all-mail = 所有邮件(_A)
desktop-menu-next = 下一个会话(_N)
desktop-menu-previous = 上一个会话(_P)
desktop-menu-message = 邮件(_M)
desktop-menu-open = 打开(_O)
desktop-menu-reply = 回复(_R)
desktop-menu-reply-all = 全部回复(_A)
desktop-menu-forward = 转发(_F)
desktop-menu-archive = 归档(_H)
desktop-menu-delete = 删除(_D)
desktop-menu-spam = 举报垃圾邮件(_S)
desktop-menu-move-to = 移至(_M)…
desktop-menu-mark-read = 标记为已读(_E)
desktop-menu-mark-unread = 标记为未读(_U)
desktop-menu-star = 加星标(_T)
desktop-menu-important = 标记为重要(_P)
desktop-menu-not-important = 标记为不重要(_N)
desktop-menu-settings = 设置(_S)
desktop-menu-quick-settings = 快速设置(_Q)
desktop-menu-configure = 配置 Katna Mail(_C)…
desktop-menu-help = 帮助(_H)
desktop-menu-shortcuts = 键盘快捷键(_K)
desktop-menu-whats-new = 新功能(_W)
desktop-menu-about = 关于 Katna(_A)

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = 导航
shortcut-group-actions = 操作
shortcut-group-go-to = 转到
shortcut-group-app = 应用

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = 下一个会话
shortcut-previous = 上一个会话
shortcut-down = 在列表中下移
shortcut-up = 在列表中上移
shortcut-first = 列表中的第一项
shortcut-last = 列表中的最后一项
shortcut-page-down = 列表向下翻页
shortcut-page-up = 列表向上翻页
shortcut-open = 打开会话
shortcut-back = 返回列表
shortcut-scroll-down = 向下滚动
shortcut-scroll-up = 向上滚动
shortcut-scroll-page-down = 向下滚动一页
shortcut-scroll-page-up = 向上滚动一页
shortcut-compose = 写邮件
shortcut-reply = 回复
shortcut-reply-all = 全部回复
shortcut-forward = 转发
shortcut-archive = 归档
shortcut-delete = 删除
shortcut-spam = 举报垃圾邮件
shortcut-move-to = 移至
shortcut-mark-read = 标记为已读
shortcut-mark-unread = 标记为未读
shortcut-star = 加星标或移除星标
shortcut-important = 标记为重要
shortcut-not-important = 标记为不重要
shortcut-check = 勾选会话
shortcut-select-all = 勾选所有会话
shortcut-select-none = 取消勾选所有会话
shortcut-undo = 撤消上一项操作
shortcut-go-inbox = 收件箱
shortcut-go-starred = 已加星标
shortcut-go-sent = 已发送
shortcut-go-drafts = 草稿
shortcut-go-all = 所有邮件
shortcut-search = 搜索邮件
shortcut-navigation = 显示或收起菜单
shortcut-quick-settings = 快速设置
shortcut-settings = 所有设置
shortcut-shortcuts = 键盘快捷键
shortcut-reload = 检查新邮件
shortcut-quit = 退出

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first }，然后按 { $second }

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
