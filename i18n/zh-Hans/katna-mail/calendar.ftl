# Katna Mail, Chinese (Simplified) (简体中文): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = 今天
calendar-today-tip = 转到今天
calendar-view-day = 日
calendar-view-week = 周
calendar-view-month = 月
calendar-view-year = 年
calendar-view-schedule = 日程
calendar-view-days =
    { $count ->
       *[other] { $count } 天
    }
calendar-options = 设置
calendar-density = 信息密度
calendar-density-responsive = 根据屏幕自动调整
calendar-density-comfortable = 舒适
calendar-density-compact = 紧凑
calendar-custom-days = 自定义视图
calendar-second-zone = 辅助时区
calendar-zone-none = 无
calendar-zone = { $zone } （{ $offset }）
calendar-share-free = 分享空闲时间
calendar-free-subject = 我的空闲时间
calendar-free-intro = 以下是我的一些空闲时间（{ $zone }）：
calendar-free-day = { $weekday } { $date }：{ $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = 接下来几个工作日我都没有空闲时间。
calendar-previous-day = 前一天
calendar-next-day = 后一天
calendar-previous-week = 上一周
calendar-next-week = 下一周
calendar-previous-month = 上个月
calendar-next-month = 下个月
calendar-previous-year = 上一年
calendar-next-year = 下一年
calendar-previous-period = 更早
calendar-next-period = 更晚
calendar-title-months = { $first } – { $last }
calendar-loading = 正在加载…
calendar-read-failed = 无法读取日历：{ $error }
calendar-sets = 日历组
calendar-set-add = 将显示的日历保存为组
calendar-set-name = 组名称
calendar-set-remove = 移除组
calendar-local = 此计算机
calendar-account-gone = 已移除的账号
calendar-account-sign-in = 重新登录以显示日历
calendar-account-signed-in = 已重新登录 { $address }。正在获取你的日历…
calendar-account-sign-in-refused = { $provider } 未允许 Katna 访问。请重试，并允许访问你的日历。
calendar-account-refused = 服务器未接受该密码。Yahoo、iCloud、Zoho 等需要应用专用密码。
calendar-account-change-password = 更改密码
calendar-account-change-password-tooltip = 输入新密码；Katna 会向服务器验证
calendar-account-not-enabled = Katna 的日历访问权限尚未开启。
calendar-account-failed = 无法读取日历。
calendar-account-error = 无法读取日历：{ $reason }
calendar-account-none = 未找到日历
calendar-account-none-why = 未找到日历：{ $reason }
calendar-account-use-sign-in = { $provider } 只向使用 { $provider } 登录的 Katna 显示日历。
calendar-account-sign-in-with = 使用 { $provider } 登录
calendar-account-looking = 正在查找日历…
calendar-account-try-again = 重试
calendar-account-try-again-tooltip = 立即重新检查此账号的日历
calendar-account-fixing = 正在处理…
calendar-birthdays = 生日
calendar-tasks = 任务
calendar-birthday-of = { $name }的生日
calendar-empty-title = 还没有日历
calendar-empty-text = 您 Google 和 Microsoft 账号的日历同步后会显示在这里，其他支持 CalDAV 的服务器上的日历也一样。
calendar-schedule-empty = 接下来两个月没有任何安排。
calendar-search = 搜索活动
calendar-search-past = 过去的活动
calendar-search-none = 没有与搜索匹配的活动。
calendar-no-title = (无标题)
calendar-all-day = 全天
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }，{ $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = 还有 { $count } 项
calendar-peek-day = { $weekday }，{ $day }
calendar-repeats = 重复
calendar-join = 加入
calendar-join-with = 通过 { $service } 加入
calendar-email-guests = 向嘉宾发送电子邮件
calendar-running-late = 我会迟到
calendar-late-subject = 我会迟到：{ $title }
calendar-late-body = 抱歉，“{ $title }”我会晚几分钟到，马上就到。
calendar-guests =
    { $count ->
       *[other] { $count } 位嘉宾
    }
calendar-guest-answers = { $yes } 人参加，{ $maybe } 人可能参加，{ $no } 人不参加，{ $waiting } 人未回复
calendar-organizer = 组织者
calendar-optional = 可选
calendar-open-web = 在浏览器中打开
calendar-open-mail = 打开邮件
calendar-open-contact = 打开联系人
calendar-close = 关闭

## Adding, changing and deleting events.

calendar-add-title = 添加标题
calendar-add-location = 添加地点
calendar-add-notes = 添加说明
calendar-add-guests = 添加嘉宾
calendar-remove-guest = 移除
calendar-add-meet = 添加 Google Meet 视频会议
calendar-add-teams = 添加 Teams 会议
calendar-has-call = 已添加视频通话
calendar-weekday-day = { $weekday }，{ $day }
calendar-schedule-day = { $month }，{ $weekday }
calendar-all-day-box = 全天
calendar-more-options = 更多选项
calendar-save = 保存
calendar-saved = 活动已保存
calendar-deleted = 活动已删除
calendar-discard = 放弃更改
calendar-edit = 修改活动
calendar-delete = 删除活动
calendar-event-details = 活动详情
calendar-menu-new-event = 新建活动
calendar-event-window-title = 新建活动
calendar-menu-open-day = 打开这一天
calendar-menu-duplicate = 复制
calendar-menu-color = 颜色
calendar-menu-color-calendar = 日历颜色
calendar-menu-in-a-week = 一周后
calendar-color-tomato = 番茄红
calendar-color-flamingo = 火烈鸟粉
calendar-color-tangerine = 橘黄
calendar-color-banana = 香蕉黄
calendar-color-sage = 鼠尾草绿
calendar-color-basil = 罗勒绿
calendar-color-peacock = 孔雀蓝
calendar-color-blueberry = 蓝莓蓝
calendar-color-lavender = 薰衣草紫
calendar-color-grape = 葡萄紫
calendar-color-graphite = 石墨灰
calendar-menu-only-this = 只显示此日历
calendar-menu-rename = 重命名
calendar-menu-remove = 从列表中移除
calendar-menu-delete = 删除
calendar-menu-new-calendar = 新建日历
calendar-menu-show-all = 全部显示
calendar-menu-hide-all = 全部隐藏
calendar-menu-account-settings = 账号设置
calendar-why-main = 主日历
calendar-why-last = 仅剩这一个
calendar-why-owner = 仅限所有者
calendar-why-contacts = 来自联系人
calendar-why-unreached = 无法连接
calendar-name-placeholder = 日历名称
calendar-toast-added = 已添加“{ $name }”
calendar-toast-renamed = 已重命名日历
calendar-toast-recolored = 已更改日历颜色
calendar-toast-deleted = 已删除“{ $name }”
calendar-toast-removed = 已从你的列表中移除“{ $name }”
calendar-edit-failed = 日历未更改：{ $reason }
calendar-delete-title = 删除“{ $name }”？
calendar-delete-confirm = 删除
calendar-deleting = 正在删除…
calendar-delete-heading = 将删除：
calendar-delete-events = 此日历及其所有活动
calendar-delete-shared = 对所有与之共享的人
calendar-delete-server = 它会从邮件服务的 { $account } 中删除，而不仅是在 Katna 中。
calendar-delete-local = 它会从这台电脑中删除。
calendar-remove-title = 从你的列表中移除“{ $name }”？
calendar-remove-confirm = 移除
calendar-removing = 正在移除…
calendar-remove-heading = 会有以下变化：
calendar-remove-events = 你将不再看到它的活动，在这里和你的其他应用中都是如此
calendar-remove-server = 日历仍归其所有者所有，对方可以再次与你共享。
calendar-kind-event = 活动
calendar-kind-task = 任务
calendar-kind-focus = 专注时间
calendar-kind-out-of-office = 外出
calendar-kind-working-location = 工作地点
calendar-task-added = 已添加任务
calendar-task-added-to = 已将任务添加到 { $list }
calendar-task-list-local = 此电脑
calendar-working-home = 家
calendar-busy = 忙碌
calendar-free = 空闲
calendar-cancel = 取消
calendar-ok = 确定
calendar-read-only = 您无法修改此日历中的活动
calendar-none-editable = 还没有可添加活动的日历
calendar-no-such-time = 您所在的时区没有这个时间
calendar-end-before-start = 活动的结束时间早于开始时间
calendar-repeat-never = 不重复
calendar-repeat-daily = 每天
calendar-repeat-weekly = 每周 { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] 每月第一个 { $weekday }
        [2] 每月第二个 { $weekday }
        [3] 每月第三个 { $weekday }
        [4] 每月第四个 { $weekday }
       *[other] 每月最后一个 { $weekday }
    }
calendar-repeat-yearly = 每年 { $day }
calendar-repeat-weekdays = 每个工作日（周一至周五）
calendar-repeat-custom = 自定义
calendar-reminder-none = 不通知
calendar-reminder-at-start = 开始时
calendar-reminder-minutes =
    { $count ->
       *[other] { $count } 分钟前
    }
calendar-reminder-hours =
    { $count ->
       *[other] { $count } 小时前
    }
calendar-reminder-days =
    { $count ->
       *[other] { $count } 天前
    }
calendar-scope-edit-title = 修改重复活动
calendar-scope-delete-title = 删除重复活动
calendar-scope-this = 此活动
calendar-scope-following = 此活动及后续活动
calendar-scope-all = 所有活动
calendar-scope-respond-title = 回复重复活动
calendar-going = 是否参加？
calendar-answer-yes = 是
calendar-answer-no = 否
calendar-answer-maybe = 可能
calendar-answered-yes = 您会参加
calendar-answered-no = 您不会参加
calendar-answered-maybe = 您可能会参加

## The card at the top of a mail with an invitation.

calendar-invite = 邀请
calendar-invite-cancelled = 活动已取消
calendar-invite-reply = { $name } 已回复
calendar-invite-reply-yes = { $name }：已接受
calendar-invite-reply-no = { $name }：已拒绝
calendar-invite-reply-maybe = { $name }：可能参加
calendar-invite-organizer = 组织者：{ $name }
calendar-invite-open = 在日历中打开
calendar-invite-not-yet = 尚未出现在您的日历中。同步后即可回复。
calendar-invite-by-mail = 不在您的日历中：您的回复将通过邮件发送给组织者。
calendar-mail-yes = 已接受：{ $title }
calendar-mail-yes-body = { $name } 已接受此邀请。
calendar-mail-no = 已拒绝：{ $title }
calendar-mail-no-body = { $name } 已拒绝此邀请。
calendar-mail-maybe = 暂定：{ $title }
calendar-mail-maybe-body = { $name } 已暂时接受此邀请。
calendar-invite-your-day = 您的一天
calendar-invite-clashes =
    { $count ->
       *[other] 与 { $count } 个活动冲突
    }

## The day's agenda beside the mail.

agenda-show = 显示当天日程
agenda-hide = 隐藏日程
agenda-today = 今天，{ $date }
agenda-day = { $weekday }，{ $date }
agenda-empty = 这一天没有任何安排。
