# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = 搜索选项
search-options-close = 关闭
search-from = 发件人
search-to = 收件人
search-subject = 主题
search-has-words = 包含字词
search-without = 不包含
search-date-within = 日期范围
search-has-attachment = 有附件
search-attachment-custom = 自定义
search-attachment-image = 图片
search-attachment-custom-hint = 输入扩展名，例如 png，然后按空格键
search-attachment-remove = 移除
search-clear-filter = 清除筛选

## Search options: "Date within" choices

search-within-any = 任何时间
search-within-days = { $count ->
   *[other] { $count } 天
}
search-within-weeks = { $count ->
   *[other] { $count } 周
}
search-within-months = { $count ->
   *[other] { $count } 个月
}
search-within-years = { $count ->
   *[other] { $count } 年
}
search-within-custom = 自定义

## Search options: custom dates (the calendar popover)

search-dates-on = 在该日
search-dates-before = 之前
search-dates-since = 从该日起
search-dates-between = 之间
search-dates-from = 开始
search-dates-to = 结束
search-dates-placeholder = YYYY-MM-DD
search-dates-missing = 请选择日期
search-dates-unreadable = 请使用类似 2026-09-01 的日期
search-dates-out-of-range = 该日期超出范围
search-dates-chip-before = { $date } 之前
search-dates-chip-since = { $date } 起
search-dates-chip-between = { $first }–{ $last }
search-dates-cancel = 取消
search-dates-done = 完成
search-dates-month-back = 上个月
search-dates-month-on = 下个月
search-dates-year-back = 上一年
search-dates-year-on = 下一年

## More results on server: under the results, mail found by asking the
## mail server, for mail that is not downloaded to this computer yet.

search-server-more = 服务器上的更多结果
search-server-searching = 正在服务器上搜索邮件…
search-server-empty-searching = 这里还没有内容。正在服务器上搜索邮件…
search-server-nothing = 服务器上没有更多结果
search-server-failed = 无法搜索服务器。
search-server-again = 重试
