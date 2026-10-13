# Katna Mail, Chinese (Simplified) (简体中文): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = 新建任务
tasks-all = 所有任务
tasks-today = 今天
tasks-upcoming = 即将到来
tasks-starred = 已加星标
tasks-completed-view = 已完成
tasks-new-list = 创建新列表
tasks-labels-heading = 标签
tasks-on-this-computer = 此电脑
tasks-my-tasks = 我的任务
tasks-account-sign-in = 重新登录以显示任务
tasks-account-signed-in = 已重新登录 { $address }。正在获取您的任务…
tasks-account-sign-in-refused = { $provider } 未允许 Katna 访问。请重试，并允许访问您的任务。
tasks-account-refused = 服务器未接受该密码。Yahoo、iCloud、Zoho 等需要应用专用密码。
tasks-account-change-password = 更改密码
tasks-account-change-password-tooltip = 输入新密码；Katna 会向服务器验证
tasks-account-not-enabled = Katna 的任务访问权限尚未开启。
tasks-account-failed = 无法读取任务列表。
tasks-account-error = 无法读取任务列表：{ $reason }
tasks-account-none = 未找到任务列表
tasks-account-none-why = 未找到任务列表：{ $reason }
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
tasks-label-empty = 没有带此标签的未完成任务。
tasks-today-empty = 今天没有到期的任务。
tasks-completed-empty = 你完成的任务会显示在此处。
tasks-upcoming-add = 为{ $day }添加任务
tasks-upcoming-overdue-day = { $day } { $weekday }
tasks-from-mail-quiet = 来自邮件
tasks-from-note-quiet = 来自笔记
tasks-steps-done = { $done }/{ $count }
tasks-today-date = { $weekday }，{ $day }
tasks-overdue = 已逾期
tasks-completed = { $count ->
   *[other] 已完成 ({ $count })
}
tasks-list-options = 列表选项
tasks-sort-by = 排序方式
tasks-sort-my-order = 我的排序
tasks-sort-date = 日期
tasks-sort-starred = 最近加星标
tasks-sort-title = 标题
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

## Several tasks selected (Ctrl+click, Shift+click)

tasks-selected = { $count ->
   *[other] 已选择 { $count } 项
}
tasks-select-clear = 清除选择
tasks-select-move = 移至清单
tasks-select-date = 设置日期
tasks-next-week = 下周

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
tasks-label-add = 添加标签
tasks-label-task = 为任务添加标签
tasks-files-attach = 附加文件
tasks-files-pick = 附加
tasks-file-open = 打开
tasks-file-remove = 移除文件
tasks-file-here = 仅在此电脑上
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
tasks-files-added = { $count ->
   *[other] 已附加 { $count } 个文件
}
tasks-file-removed = 已移除“{ $name }”
tasks-files-left-out = 未附加：{ $names }。任务只能附加不超过 { $limit } 的文件，不能附加文件夹。
tasks-file-missing = 该文件已不存在。
tasks-toast-added = { $count ->
   *[other] 已添加 { $count } 项任务
}
tasks-mail-gone = 该邮件已不存在。
tasks-toast-list-deleted = 列表已删除
tasks-toast-moved = 已移至 { $list }
tasks-toast-placed = 任务已移动
tasks-toast-rescheduled = 任务已重新安排
tasks-toast-rescheduled-several = { $count ->
   *[other] 已重新安排 { $count } 个任务
}
tasks-toast-done-several = { $count ->
   *[other] 已完成 { $count } 个任务
}
tasks-toast-open-several = { $count ->
   *[other] 已将 { $count } 个任务标记为未完成
}
tasks-toast-starred = { $count ->
   *[other] 已为 { $count } 个任务加星标
}
tasks-toast-unstarred = { $count ->
   *[other] 已移除 { $count } 个任务的星标
}
tasks-toast-deleted-several = { $count ->
   *[other] 已删除 { $count } 个任务
}
