# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = 阅读
chat-view = 以聊天形式显示会话
chat-view-detail = 人与人之间的邮件读起来就像群聊：每封邮件一个气泡，只显示写下的内容，你自己的邮件在右侧。新闻通讯保持常规视图。
chat-view-switch = 以聊天形式显示会话
chat-view-switch-detail = 引用的邮件和签名收在每个气泡的 ··· 后面
chat-switch-chat = 聊天
chat-switch-mail = 邮件
chat-people = { $names } 和你 · { $count ->
   *[other] { $count } 封邮件
}
chat-people-heading = { $count ->
   *[other] 此聊天中 · { $count } 人
}
chat-member-mails = { $count ->
    [0] 没有邮件
   *[other] { $count } 封邮件
}
chat-today = 今天
chat-yesterday = 昨天
chat-added = { $who } 添加了 { $names }
chat-renamed = { $who } 将主题更改为“{ $subject }”
chat-you = 你
chat-not-downloaded = 尚未下载
chat-forwarded = 已转发
chat-show-quoted = 显示引用的邮件和签名
chat-hide-quoted = 隐藏引用的邮件和签名
chat-hide-dots = 隐藏 ···
chat-show-card = 显示其名片
chat-reply-all = 回复所有人
chat-more = 更多
chat-reply-only = 仅回复{ $name }
chat-forward = 转发
chat-copy-text = 复制文字
chat-show-as-mail = 以邮件形式显示
chat-go-down = 转到最新邮件
chat-pin = 置顶
chat-pin-file = 置顶文件
chat-unpin = 取消置顶
chat-unpin-file = 取消置顶文件
chat-pinned-of = 置顶 { $at } / { $count }
chat-pins-all = 所有置顶
chat-pins-heading = 已置顶 · { $count } / { $most }
chat-pins-drag = 拖动以重新排序
chat-pin-from-mail = 来自{ $name }的邮件 · { $when }
chat-pin-from-file = 来自{ $name }的文件 · { $when }
chat-pin-from-text = 来自{ $name }的文字 · { $when }
chat-pins-full = 此聊天已有 5 个置顶
chat-pins-replace-title = 替换置顶
chat-pins-replace-hint = 每个聊天最多可置顶 5 项。请选择要取消的一项。
chat-pins-replace = 替换
chat-pins-cancel = 取消
chat-undo = 撤消
chat-reply-to = 回复{ $names }
chat-send = 发送 (Ctrl+Enter)。右键单击或长按可查看更多选项
chat-send-now = 立即发送
chat-attach = 附加
chat-attach-photo = 照片
chat-attach-file = 文件
chat-attach-library = 从“文件”添加
chat-attach-template = 模板
chat-attach-signature = 签名
chat-replying-to = 正在回复{ $name }
chat-reply-newest = 回复最新的邮件

## The attach picker (paperclip > From Files)

picker-title = 从“文件”附加
picker-search = 搜索名称、人员、主题
picker-search-drive = 搜索此网盘
picker-mail-files = 邮件文件
picker-this-chat = 此会话
picker-this-computer = 这台电脑…
picker-in-chat = 此会话中
picker-recent = 最近
picker-preview = 预览
picker-cancel = 取消
picker-attach = 附加
picker-attach-count = 附加 { $count } 个
picker-selected = 已选择 { $count } 个
picker-of-limit = / { $limit }
picker-in-mail = 邮件中 { $size }
picker-drive-links = { $count ->
   *[other] { $count } 个以 Google Drive 链接形式
}
picker-onedrive-links = { $count ->
   *[other] { $count } 个以 OneDrive 链接形式
}
picker-over = { $size }，超过了邮件可携带的 { $limit }
picker-getting = { $count ->
   *[other] 正在从网盘获取 { $count } 个文件…
}
picker-some-failed = { $count ->
   *[other] { $count } 个文件无法读取
}
