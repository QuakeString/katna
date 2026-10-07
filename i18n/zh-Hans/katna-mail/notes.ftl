# Katna Mail, Chinese (Simplified) (简体中文): the Notes page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Side list and search

notes-view-notes = 笔记
notes-view-reminders = 提醒
notes-view-archive = 归档
notes-view-trash = 回收站
notes-edit-labels = 修改标签
notes-search = 搜索笔记
notes-loading = 正在打开你的笔记…

## Board

notes-take-a-note = 添加笔记…
notes-new-list = 新建清单
notes-new-note = 新建笔记
notes-pinned = 已置顶
notes-others = 其他
notes-empty = 你添加的笔记会显示在此处
notes-archive-empty = 已归档的笔记会显示在此处
notes-trash-empty = 回收站中没有笔记
notes-none-found = 没有匹配的笔记
notes-label-empty = 还没有带此标签的笔记
notes-reminders-empty = 设有即将到来的提醒的笔记会显示在此处
notes-trash-note = 回收站中的笔记会在 7 天后删除。
notes-empty-trash = 清空回收站
notes-ticked = { $count ->
   *[other] + { $count } 个已勾选项
}
notes-select = 选择笔记
notes-selected = { $count ->
   *[other] 已选择 { $count } 项
}
notes-select-clear = 清除选择

## A note's buttons

notes-pin = 置顶笔记
notes-unpin = 取消置顶笔记
notes-archive = 归档
notes-unarchive = 取消归档
notes-delete = 删除笔记
notes-restore = 恢复
notes-delete-forever = 永久删除
notes-color = 背景选项
notes-checkboxes = 显示/隐藏复选框
notes-labels = 标签
notes-close = 关闭
notes-more = 更多
notes-make-copy = 创建副本
notes-remind = 提醒我
notes-add-picture = 添加图片
notes-history = 版本历史记录
notes-ai = 帮我写
notes-send-as-mail = 作为邮件发送
notes-save-markdown = 另存为 Markdown
notes-save-pdf = 另存为 PDF

## The open note

notes-title = 标题
notes-edited = 编辑时间：{ $date }
notes-on-this-computer = 此计算机
notes-where = 笔记的保存位置
notes-untitled = 无标题笔记

## Pictures

notes-picture-choose = 添加图片
notes-picture-remove = 移除图片
notes-picture-too-big = 笔记中的图片最大为 { $size }
notes-picture-kind = 该文件不是 Katna 能显示的图片
notes-picture-unreadable = 无法读取 { $name }：{ $error }

## Reminders

notes-remind-me = 提醒我
notes-remind-off = 移除提醒
notes-remind-in-the-past = 请选择尚未过去的时间
notes-remind-today = 今天 { $time }
notes-remind-tomorrow = 明天 { $time }
notes-remind-weekday = { $day } { $time }
notes-reminder-set = 已设置提醒：{ $when }
notes-reminder-off = 已移除提醒

## Links between notes

notes-link-note = 链接笔记
notes-link-new = 新建笔记“{ $title }”
notes-linked-from = 链接来源
notes-link-gone = 该笔记已不存在

## Version history

notes-versions = 版本
notes-version-now = 当前
notes-version-here = 你，在此电脑上
notes-version-yesterday = 昨天 { $time }
notes-version-changes = { $count ->
   *[other] { $count } 处更改
}
notes-version-from = 来自 { $device }
notes-version-elsewhere = 来自其他设备
notes-version-created = 已创建
notes-version-restore = 恢复此版本
notes-version-restored = 已恢复版本
notes-history-none = 暂无更早的版本

## AI help

notes-ai-tidy = 整理文字
notes-ai-checklist = 转为清单
notes-ai-summarise = 总结
notes-ai-empty = 请先写点内容
notes-ai-tidied = 文字已整理。按 Ctrl+Z 可还原。
notes-ai-listed = 已转为清单。按 Ctrl+Z 可还原。
notes-ai-summarised = 已在顶部添加摘要

## Labels

notes-label-note = 为笔记添加标签
notes-label-name = 输入标签名称
notes-label-create = 创建“{ $name }”
notes-label-remove = 移除标签
notes-label-delete = 删除标签
notes-labels-none = 还没有标签。可通过笔记上的标签按钮添加。
notes-labels-done = 完成
notes-label-renamed = 标签已重命名为“{ $name }”
notes-label-deleted = 已删除标签“{ $name }”

## A note about a mail

notes-mail = 邮件
notes-open-mail = 打开邮件
notes-open-note = 打开笔记

## Meeting notes

notes-meeting-take = 记录会议笔记
notes-meeting-title = { $title } · { $date }
notes-meeting-attendees = 参会者：{ $names }
notes-meeting-notes = 笔记
notes-meeting-actions = 待办事项
notes-event = 活动
notes-open-event = 打开活动

## Formatting

notes-format = 格式
notes-format-heading-1 = 标题 1
notes-format-heading-2 = 标题 2
notes-format-normal = 正文
notes-format-bold = 加粗
notes-format-italic = 倾斜
notes-format-underline = 下划线
notes-format-quote = 引用
notes-format-code = 代码
notes-format-divider = 分隔线
notes-format-clear = 清除格式

## Tasks

notes-make-task = 设为任务

## Colors (tooltips)

notes-color-none = 无颜色
notes-color-coral = 珊瑚色
notes-color-peach = 桃色
notes-color-sand = 沙色
notes-color-mint = 薄荷色
notes-color-sage = 鼠尾草色
notes-color-fog = 雾色
notes-color-storm = 风暴色
notes-color-dusk = 黄昏色
notes-color-blossom = 花粉色
notes-color-clay = 陶土色
notes-color-chalk = 粉笔色

## Messages at the foot of the window

notes-archived = 笔记已归档
notes-unarchived = 笔记已取消归档
notes-trashed = 笔记已移至回收站
notes-restored = 笔记已恢复
notes-saved = 笔记已保存
notes-pinned-count = { $count ->
   *[other] 已置顶 { $count } 条笔记
}
notes-unpinned-count = { $count ->
   *[other] 已取消置顶 { $count } 条笔记
}
notes-colored-count = { $count ->
   *[other] 已更改 { $count } 条笔记的颜色
}
notes-archived-count = { $count ->
   *[other] 已归档 { $count } 条笔记
}
notes-unarchived-count = { $count ->
   *[other] 已取消归档 { $count } 条笔记
}
notes-trashed-count = { $count ->
   *[other] 已将 { $count } 条笔记移至回收站
}
notes-restored-count = { $count ->
   *[other] 已恢复 { $count } 条笔记
}
notes-copied-count = { $count ->
   *[other] 已创建 { $count } 个副本
}
notes-empty-discarded = 已舍弃空笔记
notes-mail-gone = 该邮件已不存在
notes-deleted-forever = { $count ->
   *[other] 已永久删除 { $count } 条笔记
}
