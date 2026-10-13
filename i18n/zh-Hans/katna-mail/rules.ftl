# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = 规则
settings-rules-summary = 自动对新邮件进行分类、添加标签、转发或静音
settings-rules-intro = 规则会按此顺序自动整理新邮件。拖动可调整顺序。
settings-rules-all-accounts = 所有账号
settings-rules-new = 新建规则
settings-rules-none = 还没有规则。规则会按发件人、主题或字词自动整理新邮件。
settings-rules-none-account = 此账号还没有规则。
settings-rules-drag = 拖动以调整顺序
settings-rules-edit = 编辑规则
settings-rules-turn-off = 关闭此规则
settings-rules-turn-on = 开启此规则

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = 预设规则
settings-rules-starters-intro = 在你开启之前处于关闭状态。它们适用于你的所有账号；编辑即可修改。
settings-rules-starter-turning-on = 正在开启“{ $name }”…
settings-rules-starter-failed = 无法开启“{ $name }”：{ $error }
rules-starter-promotions = 推广邮件静音
rules-starter-newsletters = 新闻通讯移至“阅读”
rules-starter-receipts = 收据和发票
rules-starter-deliveries = 快递物流
rules-starter-train = 火车票
rules-starter-flight = 机票
rules-starter-codes = 一次性验证码
rules-starter-security = 安全提醒
rules-starter-social = 社交邮件
rules-starter-invites = 日历邀请
rules-starter-folder-reading = 阅读
rules-starter-folder-receipts = 收据
rules-starter-folder-deliveries = 快递物流
rules-starter-folder-travel = 出行
rules-starter-folder-social = 社交
rules-runs-katna = 在 Katna 中运行
rules-runs-gmail = 在 Gmail 上运行
rules-runs-sieve = 在服务器上运行
rules-stopped = 已停止
rules-error-folder-gone = 此规则使用的文件夹已不存在。请编辑规则选择其他文件夹。
rules-error-no-archive = 此账号没有归档文件夹。请编辑规则改为执行其他操作。
rules-error-no-trash = 此账号没有“已删除邮件”文件夹。请编辑规则改为执行其他操作。
rules-error-cannot-send = 此账号无法发送邮件，因此规则无法转发。
rules-error-other = { $error }。请编辑规则并重新开启。
settings-folders = 文件夹
settings-folders-summary = 文件夹窗格中的未读数
settings-folders-unread-counts = 在每个文件夹上显示未读数
settings-folders-unread-counts-detail = 关闭：仅收件箱显示未读数

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first }且{ $next }
rules-summary-or = { $first }或{ $next }
rules-summary-more = 另外 { $count } 个
rules-summary-list = { $first }，{ $next }
rules-summary-condition = { $field }{ $comparator }{ $value }
rules-summary-has-attachment = 有附件
rules-summary-no-attachment = 无附件
rules-summary-mailing-list = 来自邮件列表
rules-summary-not-mailing-list = 不是来自邮件列表
rules-summary-tab = 在“{ $tab }”标签页中
rules-summary-not-tab = 不在“{ $tab }”标签页中
rules-summary-move = 移至 { $folder }
rules-summary-archive = 跳过收件箱
rules-summary-trash = 移至“已删除邮件”
rules-summary-mark-read = 标记为已读
rules-summary-star = 加星标
rules-summary-important = 标记为重要
rules-summary-label = 添加标签 { $label }
rules-summary-forward = 转发至 { $address }
rules-summary-dont-notify = 不通知
rules-summary-read-after = { $count ->
   *[other] { $count } 天后标记为已读
}
rules-summary-folder-gone = 已不存在的文件夹

## The rule editor

rules-editor-new-title = 新建规则
rules-editor-edit-title = 编辑规则
rules-editor-name-hint = 规则名称
rules-editor-when = 当新邮件符合以下
rules-editor-of-these = 条件时：
rules-mode-all = 全部
rules-mode-any = 任一
rules-field-from = 发件人
rules-field-to = 收件人
rules-field-cc = 抄送
rules-field-any-recipient = 收件人或抄送
rules-field-reply-to = 回复地址
rules-field-subject = 主题
rules-field-body = 正文
rules-field-attachment-name = 附件名称
rules-field-has-attachment = 有附件
rules-field-mailing-list = 来自邮件列表
rules-field-tab = 收件箱标签页
rules-comparator-contains = 包含
rules-comparator-not-contains = 不包含
rules-comparator-begins-with = 开头是
rules-comparator-ends-with = 结尾是
rules-comparator-equals = 完全等于
rules-comparator-matches = 匹配模式
rules-has-yes = 是
rules-has-no = 否
rules-editor-value-hint = 字词或地址
rules-editor-add-condition = 添加条件
rules-editor-remove = 移除
rules-editor-then = 则：
rules-action-move = 移至
rules-action-archive = 跳过收件箱（归档）
rules-action-trash = 移至“已删除邮件”
rules-action-mark-read = 标记为已读
rules-action-star = 加星标
rules-action-important = 标记为重要
rules-action-label = 添加标签
rules-action-forward = 转发至
rules-action-dont-notify = 不通知
rules-action-read-after = 延迟标记为已读
rules-editor-choose-folder = 选择文件夹
rules-editor-choose-label = 选择标签
rules-editor-new-folder = 新建：{ $name }
rules-editor-folder-of = { $folder }（{ $account }）
rules-editor-forward-hint = 电子邮件地址
rules-editor-days = 天后
rules-editor-add-action = 添加操作
rules-editor-stop = 到此为止：后续规则不再处理此邮件
rules-editor-accounts = 账号：
rules-editor-accounts-none = 选择账号
rules-editor-accounts-many = { $count ->
   *[other] { $count } 个账号
}
rules-editor-matches = 符合过去 { $days } 天内的 { $mails }
rules-editor-mails = { $count ->
   *[other] { $count } 封邮件
}
rules-editor-counting = 正在统计符合条件的邮件…
rules-editor-show = 显示这些邮件
rules-editor-also-apply = 同时应用于这 { $count } 封邮件
rules-editor-runs-katna = 在 Katna 中运行，仅在此电脑开机时有效。
rules-editor-runs-gmail = 在 Gmail 上运行，因此在手机上以及此电脑关机时也有效。
rules-editor-runs-sieve = 在你的邮件服务器上运行，因此在手机上以及此电脑关机时也有效。
rules-note-gmail-action = 在 Katna 中运行：Gmail 过滤器无法执行“{ $action }”。
rules-note-sieve-action = 在 Katna 中运行：你的邮件服务器规则无法执行“{ $action }”。
rules-note-test = { $field }{ $comparator }
rules-note-gmail-condition = 在 Katna 中运行：Gmail 过滤器无法像 Katna 一样判断“{ $test }”。
rules-note-sieve-condition = 在 Katna 中运行：你的邮件服务器规则无法像 Katna 一样判断“{ $test }”。
rules-note-order = 在 Katna 中运行，与此账号中更靠前的一条规则一样：规则按列表顺序运行。
rules-note-gmail-stop = 在 Katna 中运行：Gmail 过滤器无法阻止后续规则运行。
rules-note-gmail-forward = 在 Katna 中运行：Gmail 只会转发到在其设置中验证过的地址，而 { $address } 不是。
rules-note-gmail-folder = 在 Katna 中运行：Gmail 中没有与此规则所用文件夹对应的标签。
rules-note-sieve-folder = 在 Katna 中运行：你的邮件服务器上没有此规则所用的文件夹。
rules-note-gmail-sign-in = 在 Katna 中运行，直到你重新登录 Google 并允许 Katna 创建 Gmail 过滤器。
rules-note-sieve-other-script = 在 Katna 中运行：你的邮件服务器上已启用另一个规则脚本（“{ $name }”）。
rules-note-gmail-failed = 在 Katna 中运行：Gmail 未接受此规则（{ $error }）。
rules-note-sieve-failed = 在 Katna 中运行：你的邮件服务器未接受此规则（{ $error }）。
rules-editor-cancel = 取消
rules-editor-save = 保存
rules-editor-saving = 正在保存…
rules-editor-delete = 删除规则
rules-editor-delete-ask = 要删除此规则吗？
rules-editor-delete-keep = 保留
rules-editor-delete-confirm = 删除
rules-editor-needs-folder = 请为每个“移至”选择文件夹，为每个“添加标签”选择标签。
rules-editor-needs-days = “延迟标记为已读”需要填写天数，范围为 1 至 3650。
rules-saved = 规则已保存
rules-saved-applied = { $count ->
   *[other] 规则已保存，并已应用于 { $count } 封邮件
}
rules-apply-failed = 规则已保存，但应用失败：{ $error }
rules-deleted = 规则已删除
rules-delete-failed = 无法删除规则：{ $error }
rules-change-failed = 无法更改规则：{ $error }
