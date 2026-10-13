# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
tab-provider-other = 由 Katna 分类

## Mail list: toolbar

list-select = 选择
list-refresh = 刷新
list-back-to-top = 回到顶部
list-checking = 正在检查新邮件…
list-more = 更多
list-mark-read = 标记为已读
list-mark-unread = 标记为未读
list-move-to = 移至
list-archive = 归档
list-spam = 举报垃圾邮件
list-delete = 删除
list-snooze = 延后
list-unsnooze = 取消延后
list-newer = 较新
list-older = 较早
list-range = 第 { $first }–{ $last } 行，共 { $total } 行
list-range-about = 第 { $first }–{ $last } 行，共约 { $total } 行
list-results = “{ $query }”的搜索结果
list-results-corrected = 显示的是“{ $query }”的搜索结果
list-search-instead = 仍然搜索“{ $query }”
list-search-no-index = 搜索尚未就绪：索引尚未建立。
list-search-not-ready = 搜索尚未就绪：{ $error }
list-files-more = +{ $count }
list-replied = 你已回复

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
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] 已选择此页上的全部 { $count } 个已读会话。
       *[message] 已选择此页上的全部 { $count } 封已读邮件。
    }
   *[unread] { $kind ->
        [conversation] 已选择此页上的全部 { $count } 个未读会话。
       *[message] 已选择此页上的全部 { $count } 封未读邮件。
    }
    [starred] { $kind ->
        [conversation] 已选择此页上的全部 { $count } 个已加星标的会话。
       *[message] 已选择此页上的全部 { $count } 封已加星标的邮件。
    }
    [unstarred] { $kind ->
        [conversation] 已选择此页上的全部 { $count } 个未加星标的会话。
       *[message] 已选择此页上的全部 { $count } 封未加星标的邮件。
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] 选择全部 { $count } 个已读会话
       *[message] 选择全部 { $count } 封已读邮件
    }
   *[unread] { $kind ->
        [conversation] 选择全部 { $count } 个未读会话
       *[message] 选择全部 { $count } 封未读邮件
    }
    [starred] { $kind ->
        [conversation] 选择全部 { $count } 个已加星标的会话
       *[message] 选择全部 { $count } 封已加星标的邮件
    }
    [unstarred] { $kind ->
        [conversation] 选择全部 { $count } 个未加星标的会话
       *[message] 选择全部 { $count } 封未加星标的邮件
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] 选择“{ $folder }”中的全部 { $count } 个已读会话
       *[message] 选择“{ $folder }”中的全部 { $count } 封已读邮件
    }
   *[unread] { $kind ->
        [conversation] 选择“{ $folder }”中的全部 { $count } 个未读会话
       *[message] 选择“{ $folder }”中的全部 { $count } 封未读邮件
    }
    [starred] { $kind ->
        [conversation] 选择“{ $folder }”中的全部 { $count } 个已加星标的会话
       *[message] 选择“{ $folder }”中的全部 { $count } 封已加星标的邮件
    }
    [unstarred] { $kind ->
        [conversation] 选择“{ $folder }”中的全部 { $count } 个未加星标的会话
       *[message] 选择“{ $folder }”中的全部 { $count } 封未加星标的邮件
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] 已选择全部 { $count } 个已读会话。
       *[message] 已选择全部 { $count } 封已读邮件。
    }
   *[unread] { $kind ->
        [conversation] 已选择全部 { $count } 个未读会话。
       *[message] 已选择全部 { $count } 封未读邮件。
    }
    [starred] { $kind ->
        [conversation] 已选择全部 { $count } 个已加星标的会话。
       *[message] 已选择全部 { $count } 封已加星标的邮件。
    }
    [unstarred] { $kind ->
        [conversation] 已选择全部 { $count } 个未加星标的会话。
       *[message] 已选择全部 { $count } 封未加星标的邮件。
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] 已选择“{ $folder }”中的全部 { $count } 个已读会话。
       *[message] 已选择“{ $folder }”中的全部 { $count } 封已读邮件。
    }
   *[unread] { $kind ->
        [conversation] 已选择“{ $folder }”中的全部 { $count } 个未读会话。
       *[message] 已选择“{ $folder }”中的全部 { $count } 封未读邮件。
    }
    [starred] { $kind ->
        [conversation] 已选择“{ $folder }”中的全部 { $count } 个已加星标的会话。
       *[message] 已选择“{ $folder }”中的全部 { $count } 封已加星标的邮件。
    }
    [unstarred] { $kind ->
        [conversation] 已选择“{ $folder }”中的全部 { $count } 个未加星标的会话。
       *[message] 已选择“{ $folder }”中的全部 { $count } 封未加星标的邮件。
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] 这里没有已读会话。
       *[message] 这里没有已读邮件。
    }
   *[unread] { $kind ->
        [conversation] 这里没有未读会话。
       *[message] 这里没有未读邮件。
    }
    [starred] { $kind ->
        [conversation] 这里没有已加星标的会话。
       *[message] 这里没有已加星标的邮件。
    }
    [unstarred] { $kind ->
        [conversation] 这里没有未加星标的会话。
       *[message] 这里没有未加星标的邮件。
    }
}
list-clear-selection = 清除选择

## Mail list: empty states

list-empty-search = 没有与搜索条件匹配的邮件。
list-empty-tab = “{ $tab }”中没有邮件。
list-empty-tab-unknown = 此标签页中没有邮件。
list-empty-folder = “{ $folder }”中没有邮件。
list-empty-folder-unknown = 此文件夹中没有邮件。
list-empty-waiting = 没有等待回复的邮件。
list-empty-reminders = 没有提醒。在邮件上按 H 可添加提醒。
list-first-sync = 正在获取您的邮件…
list-first-sync-detail = 邮件到达后会显示在这里。
list-store-unreadable = 无法打开邮件存储

## Mail list: lines

row-no-subject = （无主题）
row-unknown-sender = （未知发件人）
row-to = 收件人：
row-no-recipients = （无收件人）
row-names-separator = {"、"}
row-me = 我
row-removed = 此邮件已被移除。
row-starred = 已加星标
row-not-starred = 未加星标
row-important = 重要。点击可标记为不重要。
row-mark-important = 标记为重要
row-pinned = 已置顶
row-task = 任务
row-task-open = 打开任务：{ $title }
row-tracking-none = 已跟踪。尚未打开
row-tracking-opened = { $recipients } 人中有 { $opened } 人打开
row-tracking-clicked = { $recipients } 人中有 { $opened } 人打开，{ $clicked } 人点开了链接
row-pin = 置顶
row-unpin = 取消置顶
row-snoozed-until = 延后至 { $when }
row-snoozed-day-time = { $day } { $time }
snoozed-group-today = 今天
snoozed-group-tomorrow = 明天
snoozed-group-this-week = 本周
snoozed-group-later = 以后
row-follow-up-step = 第 { $step } 次跟进，共 { $steps } 次 · { $date }
row-follow-up-waiting = 跟进待发送
row-reminder = 提醒 { $date }

## Mail list: More menu and right-click menu

menu-reply = 回复
menu-reply-all = 全部回复
menu-forward = 转发
menu-archive = 归档
menu-delete = 删除
menu-delete-forever = 永久删除
menu-move-to-inbox = 移至收件箱
menu-spam = 举报垃圾邮件
menu-not-spam = 不是垃圾邮件
menu-mark-read = 标记为已读
menu-mark-unread = 标记为未读
menu-mark-all-read = 全部标记为已读
menu-star = 加星标
menu-unstar = 移除星标
menu-important = 标记为重要
menu-not-important = 标记为不重要
menu-pin = 置顶
menu-unpin = 取消置顶
menu-snooze = 延后
menu-remind = 提醒我
menu-unsnooze = 取消延后
menu-add-to-tasks = 添加到任务
menu-schedule-meeting = 安排会议
menu-start-call = 开始视频通话
menu-add-note = 添加笔记
menu-print-all = 全部打印
menu-new-window = 在新窗口中打开
menu-move-to = 移至
menu-follow-up = 跟进
menu-more = 更多
menu-move-to-heading = 移至：
menu-move-to-search = 移至…
menu-label-as = 添加标签
menu-label-as-search = 添加标签…
menu-no-folder = 没有名为“{ $name }”的文件夹
menu-no-label = 没有名为“{ $name }”的标签
menu-create-folder = 创建“{ $name }”
menu-always-move = 始终将来自 { $name } 的邮件移至此处
toast-always-move-failed = 邮件已移动，但规则未创建：{ $error }
drag-mail = { $kind ->
    [conversation] { $count ->
       *[other] { $count } 个会话
    }
   *[message] { $count ->
       *[other] { $count } 封邮件
    }
}
menu-find-from = 查找来自 { $name } 的邮件
menu-make-rule = 创建规则…

## Snackbar after an action on mail in the list

toast-key-imported = 已导入密钥
toast-key-updated = 您已有此密钥；现已更新为最新
toast-key-removed = 已移除密钥
toast-key-not-removed = 无法移除密钥
toast-fingerprint-copied = 已复制指纹
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
toast-label-added = 已添加标签“{ $label }”。
toast-label-removed = 已移除标签“{ $label }”。
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
toast-snoozed = { $kind ->
    [conversation] 已将 { $count } 个会话延后至 { $when }。
   *[message] 已将 { $count } 封邮件延后至 { $when }。
}
toast-unsnoozed = { $kind ->
    [conversation] { $count } 个会话已返回收件箱。
   *[message] { $count } 封邮件已返回收件箱。
}
toast-spam = { $kind ->
    [conversation] 已将 { $count } 个会话举报为垃圾邮件。
   *[message] 已将 { $count } 封邮件举报为垃圾邮件。
}
toast-not-spam = { $kind ->
    [conversation] 已将 { $count } 个会话标记为非垃圾邮件并移至收件箱。
   *[message] 已将 { $count } 封邮件标记为非垃圾邮件并移至收件箱。
}
toast-deleted-forever = { $kind ->
    [conversation] 已永久删除 { $count } 个会话。
   *[message] 已永久删除 { $count } 封邮件。
}
toast-marked-read = { $kind ->
    [conversation] 已将 { $count } 个会话标记为已读。
   *[message] 已将 { $count } 封邮件标记为已读。
}
toast-marked-unread = { $kind ->
    [conversation] 已将 { $count } 个会话标记为未读。
   *[message] 已将 { $count } 封邮件标记为未读。
}
toast-undone = 已撤消操作。
toast-nothing-to-undo = 没有可撤消的操作。
toast-cannot-undo-delete-forever = 永久删除的邮件无法恢复。
toast-send-undone = 已撤消发送。
toast-too-late-to-undo-send = 来不及撤消：邮件已经发出。
toast-undo = 撤消
toast-close = 关闭
toast-no-spam-folder = 此账号没有垃圾邮件文件夹。
