# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = 关于 Katna
about-tagline = 适用于 Linux 桌面的邮件和日历
about-whats-new = 新功能

## Updates, in a box under the version in About (only in packages that
## update themselves). $version is a version such as 0.0.0.r236.g1a2b3c4.

about-update-not-checked = 尚未检查更新
about-update-checking = 正在检查更新…
about-update-up-to-date = Katna Mail 已是最新版本
about-update-check-failed = 无法检查更新
about-update-available = 有可用版本 { $version }
about-update-downloading = 正在下载版本 { $version }… { $percent }%
about-update-ready = 版本 { $version } 已可安装
about-update-ready-detail = Katna Mail 将重新启动以完成更新。
about-update-confirm = 要安装版本 { $version } 吗？
about-update-confirm-detail = Katna Mail 将会关闭、安装更新，然后回到你离开时的位置重新打开。你的电脑会要求输入密码。
about-update-installing = 正在安装版本 { $version }…
about-update-installing-detail = 请在打开的窗口中输入你的密码。
about-update-cancelled = 由于未输入密码，更新未被安装。
about-update-failed = 更新无法安装：{ $error }
about-update-unsupported = 这份 Katna Mail 由你的软件包管理器负责更新。
about-update-restart-failed = 更新已安装，但 Katna Mail 无法重新打开（{ $error }）。请手动打开它。
about-update-check = 检查更新
about-update-download = 下载
about-update-button = 更新
about-update-restart = 更新并重启
about-update-cancel = 暂不
about-changelog = 更新日志
about-source = 源代码
about-coffee = 请我喝杯咖啡
about-coming-soon = 即将推出
about-follow = 关注作者
about-love-title = 怀着对 Rust、KDE 和 Linux 的热爱而打造
about-love-text = Rust 让编写一款快速又安全的邮件应用成为一种乐趣：Katna 没有任何 unsafe 代码。KDE 的 Plasma 桌面及其 PIM 套件启发了 Katna，而 Linux 和自由软件社区则是它立足的根基。谢谢你们，也感谢下面这些库。
about-kde-text = KDE 打造了 Katna 最有归属感的桌面。它由志愿者开发，由像你这样的人资助。如果你喜欢 Plasma 或 KDE 的应用，请考虑向 KDE 捐款。
about-donate-kde = 向 KDE 捐款
about-gpui-title = 基于 Zed 项目的 GPUI 构建
about-gpui-text = Katna Mail 的整个界面都基于 GPUI 构建，这是 Zed Industries 为 Zed 编辑器打造的快速、GPU 加速的 UI 框架。你看到的每一个像素、每一段动画和每一个窗口都由它绘制。感谢 Zed 团队以开放的方式构建它。Apache-2.0。
about-gpui-github = GitHub 上的 GPUI
about-personal-title = 一个个人项目
about-personal-text = Katna Mail 并不试图做到新颖或革命性。它是作者自己想要的邮件应用，功能和外观借鉴了 Gmail、Mailspring 和 Thunderbird。它之所以能实现，全靠 LLM 已经发展到了今天的程度。
about-built-on = 基于自由软件构建
about-credit-pimalaya = IMAP、SMTP 和登录（io-imap、io-smtp、io-sasl）
about-credit-imap-codec = 读写 IMAP
about-credit-tantivy = 搜索
about-credit-sqlite = 邮件存储
about-credit-rustls = 安全连接
about-credit-mail-parser = 读取邮件，来自 Stalwart Labs
about-credit-html5ever = HTML 邮件，来自 Servo 项目
about-credit-zbus = 通过 D-Bus 和门户与桌面通信
about-credit-oo7 = 桌面密钥环中的密码
about-credit-hayro = 查看和打印 PDF
about-credit-calamine = 电子表格预览
about-credit-resvg = SVG 图片
about-credit-jiff = 日期和时区
about-credit-spellbook = 拼写检查，来自 Helix 编辑器
about-credit-smol = 同时处理多项任务
about-all-libraries = Katna 使用的所有库（{ $count }）
about-library-authors = 作者：{ $authors }
about-license = Katna 是自由软件，采用 GNU GPL 第 3 版或更高版本授权。
about-close = 关闭

## What’s new (shown after an update)

whats-new-title = Katna Mail 新功能
whats-new-updated = 已更新到版本 { $version }
whats-new-version = 版本 { $version }
whats-new-more = { $count ->
   *[other] 完整更新日志中还有 { $count } 项。
}
whats-new-changelog = 完整更新日志
whats-new-got-it = 知道了

## First run: welcome page

onboarding-welcome-title = 欢迎使用 Katna Mail
onboarding-welcome-lead = 你的邮件就在你自己的电脑上：搜索快速、离线可读、保护隐私。
onboarding-fast-title = 快速，离线也一样
onboarding-fast-text = Katna 会在本机保存一份邮件副本，因此无论有没有网络连接，打开和搜索都能瞬间完成。
onboarding-providers-title = 支持你的邮箱
onboarding-providers-text = Gmail、Outlook、Yahoo、iCloud 以及任何其他 IMAP 或 POP 账号。
onboarding-private-title = 隐私
onboarding-private-text = 你的邮件从邮件服务商直接传到这台电脑。没有任何 Katna 服务器能看到它。
onboarding-get-started = 开始使用

## First run: adding an account

onboarding-service-checking = 正在检查 Katna 后台服务…
onboarding-service-running = Katna 后台服务正在运行。
onboarding-service-missing = Katna 后台服务未运行
onboarding-service-start = 它负责收取和发送你的邮件。请在终端中启动它，然后再检查一次：
onboarding-check-again = 再次检查
onboarding-account-title = 添加你的邮件账号
onboarding-account-lead = 输入你的电子邮件地址和密码，Katna 会自动找到服务器设置。Gmail、Yahoo 和 iCloud 需要应用专用密码，可在账号的安全设置中生成。
onboarding-add-account = 添加账号
onboarding-back = 返回

## First run: choosing the look

onboarding-look-title = 打造你的专属风格
onboarding-look-lead = 选择邮件的打开方式和 Katna 的外观。你可以随时在快速设置中更改。
onboarding-reading-pane = 阅读窗格
onboarding-pane-right = 列表右侧
onboarding-pane-none = 不拆分
onboarding-theme = 主题
onboarding-theme-system = 跟随系统
onboarding-theme-light = 浅色
onboarding-theme-dark = 深色
onboarding-density = 显示密度
onboarding-density-default = 默认
onboarding-density-compact = 紧凑
onboarding-continue = 继续

## First run: done

onboarding-ready-title = 一切就绪
onboarding-ready-lead = Katna 正在收取你的邮件。邮件到达后就会显示，新邮件也会自动出现。
onboarding-ready-lead-address = Katna 正在收取 { $address } 的邮件。邮件到达后就会显示，新邮件也会自动出现。
onboarding-ready-tour = 花一分钟参观导览，看看各项功能都在哪里？
onboarding-skip = 暂时跳过
onboarding-take-tour = 参观导览

## Asking to send crash reports (on its own and on the first-run pages)

share-title = 帮助改进 Katna
share-lead = Katna 崩溃时，会在这台电脑上保存一份报告。发送这些报告有助于修复问题。你可以随时在“设置 > 用户反馈”中更改此选项。
share-sent = 会发送什么
share-sent-detail = 与你在设置中看到的完全相同的崩溃报告：崩溃的内容及其在 Katna 中的位置、版本、你的 Linux 系统和桌面，以及 Katna 最后几行日志（其中可能包含邮件文件夹名称）。
share-never-sent = 绝不会发送什么
share-never-sent-detail = 你的邮件、联系人、密码、IP 地址、用户名或电脑名称。电子邮件地址会从报告中移除。
share-where = 发送到哪里
share-where-detail = Katna 在 Sentry 上的崩溃跟踪服务，数据存储在欧盟。没有任何 ID 会将报告与你关联。
share-dont-send = 不发送
share-send = 发送崩溃报告
share-sending = 将会发送崩溃报告。谢谢。
share-local = 崩溃报告会保留在这台电脑上。

## The tour (cards pointing at each part of the window)

tour-welcome-title = 欢迎使用 Katna Mail
tour-welcome-text = 一分钟导览，带你看看各项功能都在哪里。
tour-not-now = 以后再说
tour-start = 参观导览
tour-close = 关闭
tour-skip = 跳过导览
tour-back = 上一步
tour-done = 完成
tour-next = 下一步
tour-step = 第 { $step } 步，共 { $total } 步
tour-compose-title = 写邮件
tour-compose-text = “写邮件”会在右下角打开一封新邮件，让你边写边继续阅读。
tour-search-title = 搜索所有邮件
tour-search-text = 搜索在离线时也能使用。最右侧的按钮可以添加筛选条件：发件人、收件人、主题、日期和附件。
tour-menu-title = 显示或隐藏文件夹
tour-menu-text = 此按钮可以收起文件夹列表。隐藏时，将指针停在左侧的“邮件”上即可查看文件夹。
tour-apps-title = 你的应用
tour-apps-text = 邮件现在就在这里。日历、联系人、任务、笔记和订阅源将陆续加入这一栏。
tour-tabs-title = 收件箱标签页
tour-tabs-text = 新邮件会被分类到“主要”“推广”“社交”“动态”和“论坛”中。你可以在快速设置中关闭这些标签页。
tour-list-title = 你的邮件
tour-list-text = 点击邮件即可阅读。将指针悬停在邮件上可使用快捷操作，右键点击可查看更多操作，也可以勾选多封邮件一起处理。
tour-settings-title = 快速设置
tour-settings-text = 在这里更改阅读窗格、显示密度和主题。也可以从这里重新开始导览。
tour-account-title = 你的账号
tour-account-text = 查看你当前所在的账号，并添加其他账号。

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Katna 后台服务意外停止。
   *[other] Katna 后台服务意外停止。另外还保存了 { $more } 份崩溃报告。
}
crash-mail = { $more ->
    [0] Katna Mail 上次意外关闭。
   *[other] Katna Mail 上次意外关闭。另外还保存了 { $more } 份崩溃报告。
}
crash-view = 查看报告
crash-view-tooltip = 打开保存在这台电脑上的报告
crash-copy = 复制报告
crash-close = 关闭
sign-in-again-text = { $provider } 要求你重新登录 { $address }。
sign-in-again-button = 登录
sign-in-again-tooltip = 在浏览器中打开 { $provider } 登录页面
sign-in-again-waiting = 正在等待浏览器…
sign-in-again-close = 关闭
sign-in-again-done = 已重新登录 { $address }。正在收取你的邮件…
delete-ask-title = { $kind ->
    [conversation] { $count ->
       *[other] 将 { $count } 个会话移至已删除邮件？
    }
   *[message] { $count ->
       *[other] 将 { $count } 封邮件移至已删除邮件？
    }
}
delete-ask-body = { $count ->
   *[other] 你可以随后立即撤销，或以后从已删除邮件中找回。
}
delete-ask-confirm = 移至已删除邮件
delete-forever-title = { $kind ->
    [conversation] { $count ->
       *[other] 永久删除 { $count } 个会话？
    }
   *[message] { $count ->
       *[other] 永久删除 { $count } 封邮件？
    }
}
delete-forever-body = { $count ->
   *[other] 服务器上也会一并删除，且无法撤销。
}
delete-forever-confirm = 永久删除
delete-ask-dont-ask = 不再询问
delete-ask-cancel = 取消
