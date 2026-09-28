# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings page: its tabs

settings-tab-general = 常规
settings-tab-inbox = 收件箱
settings-tab-accounts = 账号
settings-tab-katna-account = Katna 账号
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
settings-translation = 翻译
settings-translation-detail = 其他语言的邮件可以用你的语言阅读。
settings-translation-offer = 提供翻译
settings-translation-offer-detail = 只有在你要求翻译，或该语言设为始终翻译时，邮件正文才会发送到 Katna 的服务器进行翻译。附件永远不会发送。
settings-translation-reading = 翻译为
settings-translation-always = 始终翻译
settings-translation-never = 从不提供翻译
settings-translation-none = 还没有。可在邮件的翻译栏中选择。
settings-general-mark-read = 标记为已读
settings-general-mark-read-now = 打开后立即标记
settings-general-mark-read-1s = 打开 1 秒后
settings-general-mark-read-3s = 打开 3 秒后
settings-general-mark-read-never = 仅在我手动标记时
settings-general-auto-advance = 自动前进
settings-general-auto-advance-detail = 在你删除、归档或移动打开的会话后
settings-general-auto-advance-next = 打开下一个会话
settings-general-auto-advance-previous = 打开上一个会话
settings-general-auto-advance-list = 返回列表
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
settings-general-reset-cache = 重置缓存
settings-general-reset-cache-detail = 当邮件显示有误或已过时，或需要释放磁盘空间时使用。你的邮件服务器上不会有任何变化。
settings-general-desktop = 桌面
settings-general-start-at-login = 登录时启动 Katna
settings-general-start-at-login-detail = 同步邮件，显示新邮件通知和托盘图标，但不打开窗口
settings-general-login-window = 同时打开 Katna Mail 窗口
settings-general-login-window-detail = 登录时也会打开窗口
settings-general-tray = 在系统托盘中显示 Katna
settings-general-tray-detail = 显示未读数并提供菜单
settings-general-unread-badge = 在任务栏图标上显示未读数
settings-general-unread-badge-detail = 收件箱中未读邮件的数量
settings-general-search-triggers = 从桌面搜索
settings-general-search-triggers-detail = 在 KRunner 或 GNOME 搜索中输入其中一个词和一个空格，再输入要找的内容，即可像这里的搜索框一样搜索邮件。多个词之间用逗号分隔。
settings-general-search-triggers-none = 没有设置词；只有“mail:”可用

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
settings-appearance-theme-system = 跟随系统
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
settings-default-apps-documents-detail = Word（docx、doc）、OpenDocument 文本（odt）和幻灯片（pptx、ppt、odp）。
settings-default-apps-katna = Katna Mail 查看器
settings-default-apps-system = 桌面的默认应用
settings-default-apps-ask = 每次询问使用哪个应用
settings-default-apps-after-saving = 保存后
settings-default-apps-show-folder = 在文件夹中显示已保存的文件
settings-default-apps-show-folder-detail = 打开文件管理器并选中已保存的附件

## Settings > Compose

settings-compose-send-from = 发送新邮件时使用
settings-compose-send-from-detail = 新邮件从此账号开始，可在发件人一行选择其他账号。回复和转发始终从收到原邮件的账号发出。
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
settings-compose-no-templates = 还没有模板。在邮件中选择“模板”，然后选择“存为模板”。
settings-compose-template-new = 新建
settings-compose-template-new-name = 新模板
settings-compose-template-subject = 主题
settings-compose-template-text = 模板正文
settings-compose-template-fields = {"{"}first name{"}"}、{"{"}name{"}"} 和 {"{"}my name{"}"} 会填写为收件人的名字和你的名字。
settings-compose-template-remove-file = 移除附件
settings-compose-template-save = 保存
settings-compose-template-saved = 模板已保存
settings-compose-template-needs-name = 请为模板命名
settings-compose-template-delete = 删除模板
settings-compose-template-deleted = 已删除模板
settings-compose-template-delete-failed = 无法删除模板：{ $error }

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
settings-translation-summary = 用 Katna 的服务器将其他语言的邮件翻译成你选择的语言
settings-general-mark-read-summary = 打开的会话何时标记为已读：立即、1 秒或 3 秒后，或手动标记
settings-general-auto-advance-summary = 删除、归档或移动打开的会话后打开什么：下一个、上一个或列表
settings-general-reply-button-summary = 每封邮件旁的回复按钮回复所有人
settings-general-remote-images-summary = 始终显示所有邮件中的图片
settings-general-sending-summary = 撤消发送：已发送的邮件等待多久，以便撤回
settings-general-offline-summary = 完整下载最近多少天的邮件，以便离线阅读
settings-general-notifications-summary = 新邮件通知及其提示音
settings-general-reset-cache-summary = 删除已下载的邮件、发件人图片和搜索索引，然后重新下载
settings-general-desktop-summary = 登录时启动 Katna、系统托盘图标和任务栏图标上的未读数
settings-accounts-accounts-summary = 添加或移除账号，或更改其图片
settings-appearance-density-summary = 列表中的行采用默认或紧凑样式
settings-appearance-scaling-summary = 放大或缩小所有内容：文字、图标、间距和分隔线
settings-appearance-theme-summary = 跟随系统、浅色或深色
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
settings-default-apps-documents-summary = Word、OpenDocument 文本和幻灯片的打开方式
settings-default-apps-after-saving-summary = 在文件夹中显示已保存的附件
settings-compose-send-from-summary = 新邮件从哪个账号发出：第一个账号、其他账号，或你当前所在的账号
settings-compose-send-on-replies-summary = 回复和转发时“发送”，或“发送并归档”会话
settings-compose-signatures-summary = 添加在邮件正文下方的“--”行之后
settings-compose-for-new-mail-summary = 新邮件默认使用的签名
settings-compose-for-replies-summary = 回复和转发默认使用的签名
settings-compose-format-summary = 以纯文本撰写新邮件
settings-compose-spelling-summary = 撰写时检查拼写，以及词典的语言
settings-general-search-triggers-summary = 可在 KRunner 或 GNOME 搜索中搜索邮件的词
settings-compose-templates-summary = 保存你经常写的邮件，并以它开始新邮件或回复
settings-feedback-crash-reports-summary = Katna Mail 或其后台服务崩溃时，将崩溃报告保存在此电脑上
settings-feedback-saved-summary = 查看、复制或删除保存在此电脑上的崩溃报告
settings-feedback-help-improve-summary = 发送崩溃报告以帮助修复问题；除非你开启，否则保持关闭
settings-experimental-blur-summary = 透过顶栏可看到模糊的桌面，菜单呈磨砂玻璃效果
settings-search-shortcut = 键盘快捷键
settings-search-tab = 设置标签页
settings-search-none = 没有与“{ $query }”匹配的设置。
settings-search-results = 与“{ $query }”匹配的设置

## Settings: opening at login

settings-open-at-login-failed = 无法更改登录时启动的设置：{ $error }

## Settings > General > Time

settings-time = 时间
settings-clock-language = 按语言习惯
settings-clock-12 = 12 小时制，例如下午2:05
settings-clock-24 = 24 小时制，例如 14:05
settings-time-summary = 12 小时制或 24 小时制，或按语言习惯

## Settings > General > Default mail app, Settings > Compose > Grammar

settings-general-mail-app = 默认邮件应用
settings-general-mail-app-detail = 其他应用和网站中的电子邮件链接会在这里打开一封新邮件。
mail-app-is-default = Katna Mail 是你的默认邮件应用。
mail-app-is-other = 电子邮件链接会在其他应用中打开。
mail-app-make-default = 设为默认
mail-app-make-default-failed = 无法更改默认邮件应用。
settings-general-mail-app-summary = 在 Katna Mail 中打开其他应用和网站中的电子邮件链接
settings-compose-grammar = 语法
settings-compose-grammar-detail = 在这台电脑上用 Harper 检查。目前仅支持英语：其他语言的文字保持不变。
settings-compose-grammar-check = 检查语法
settings-compose-grammar-check-detail = 撰写时为语法错误加下划线（英语）
settings-compose-suggestions = 写作建议
settings-compose-suggestions-detail = 在这台电脑上从你发出的邮件和你正在回复的邮件中学习，任何内容都不会离开这台电脑。按 Tab 采纳建议，或继续输入。
settings-compose-suggestions-on = 撰写时提供建议
settings-compose-suggestions-on-detail = 输入时以灰色显示短语可能的后续内容
settings-compose-grammar-summary = 撰写时为语法错误加下划线（英语）
settings-compose-suggestions-summary = 输入时以灰色显示短语可能的后续内容
