# Katna Mail, Chinese (Simplified) (简体中文): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = 创建
tasks-all = 所有任务
tasks-today = 今天
tasks-starred = 已加星标
tasks-new-list = 创建新列表
tasks-on-this-computer = 此电脑
tasks-my-tasks = 我的任务
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = 重新登录以显示任务
tasks-account-signed-in = 已重新登录 { $address }。正在获取您的任务…
tasks-account-sign-in-refused = { $provider } 未允许 Katna 访问。请重试，并允许访问您的任务。
tasks-account-refused = 服务器未接受该密码。Yahoo、iCloud、Zoho 等需要应用专用密码。
tasks-account-change-password = 更改密码
tasks-account-change-password-tooltip = 打开“设置 > 账号”
tasks-account-not-enabled = Katna 的任务访问权限尚未开启。
tasks-account-failed = 无法读取任务列表。
# $reason is the server's own words, in English.
tasks-account-error = 无法读取任务列表：{ $reason }
tasks-account-none = 未找到任务列表
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = 未找到任务列表：{ $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } 只向使用 { $provider } 登录的 Katna 显示任务。
tasks-account-sign-in-with = 使用 { $provider } 登录
tasks-account-looking = 正在查找任务列表…
tasks-account-try-again = 重试
tasks-account-try-again-tooltip = 立即重新检查此账号的任务
tasks-account-fixing = 正在处理…
tasks-list-name-placeholder = 列表名称

## Lists and tasks

tasks-loading = 正在读取您的任务…
tasks-no-lists = 您的任务列表会显示在这里。
tasks-search = 搜索任务
tasks-search-none = 没有与搜索匹配的任务。
tasks-add = 添加任务
tasks-title-placeholder = 标题
tasks-add-step = 添加子任务
tasks-empty = 还没有任务。请在上方添加。
tasks-starred-empty = 为任务加星标后，就会显示在这里。
tasks-today-empty = 今天没有到期的任务。
tasks-today-date = { $weekday }，{ $day }
tasks-overdue = 已逾期
tasks-completed = { $count ->
   *[other] 已完成 ({ $count })
}
tasks-list-options = 列表选项
tasks-rename-list = 重命名列表
tasks-delete-list = 删除列表
tasks-mark-done = 标记为已完成
tasks-mark-open = 标记为未完成
tasks-star = 加星标
tasks-unstar = 移除星标
tasks-edit-title = 修改标题
tasks-details = 详细信息
tasks-delete = 删除
tasks-move-to = 移至 { $list }
tasks-from-mail = 邮件
tasks-open-mail = 打开邮件
tasks-from-note = 笔记
tasks-open-note = 打开笔记
tasks-note-gone = 该笔记已不存在。
tasks-no-subject = （无主题）

## The details dialog

tasks-notes-placeholder = 添加详细信息
tasks-date = 日期
tasks-no-date = 无日期
tasks-time-placeholder = 添加时间
tasks-repeat = 重复
tasks-repeat-never = 不重复
tasks-repeat-daily = 每天
tasks-repeat-weekly = 每周
tasks-repeat-monthly = 每月
tasks-repeat-yearly = 每年
tasks-repeat-other = 自定义
tasks-remind = 提醒我
tasks-remind-off = 不提醒
tasks-remind-on-time = 准时
tasks-remind-morning = 当天 { $time }
tasks-remind-hour-before = 提前 1 小时
tasks-remind-day-before = 提前 1 天
tasks-cancel = 取消
tasks-save = 保存
tasks-not-a-time = “{ $text }”不是有效的时间，例如 { $example }。

## Due days

tasks-due-today = 今天
tasks-due-tomorrow = 明天
tasks-due-yesterday = 昨天
tasks-due-at = { $day } { $time }

## Notes at the bottom

tasks-toast-done = 任务已完成
tasks-toast-next = 已完成。下一次在 { $date }
tasks-toast-deleted = 任务已删除
tasks-toast-added = { $count ->
   *[other] 已添加 { $count } 项任务
}
tasks-mail-gone = 该邮件已不存在。
tasks-toast-list-deleted = 列表已删除
tasks-toast-moved = 已移至 { $list }
# A task dragged to another place in its own list.
tasks-toast-placed = 任务已移动
tasks-toast-rescheduled = 任务已重新安排
