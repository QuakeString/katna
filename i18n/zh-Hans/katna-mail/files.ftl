# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = 搜索文件

## Left side (and chips on a phone)

files-all = 所有文件
files-pictures = 图片
files-pdfs = PDF
files-documents = 文档
files-sheets = 电子表格
files-slides = 幻灯片
files-other = 其他
files-accounts = 账号
files-drives = 网盘
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = 与我共享
files-shown = 显示
files-received = 已收到
files-sent = 我发送的

## Over the files

files-count = { $count ->
   *[other] { $count } 个文件 · { $size }
}
files-anyone = 任何人
files-from-person = 来自{ $name }
files-time-any = 任何时间
files-time-today = 今天
files-time-yesterday = 昨天
files-time-this-week = 本周
files-time-last-week = 上周
files-time-this-month = 本月
files-time-last-month = 上月
files-time-between = { $first } – { $last }
files-time-hint = 点击某一天，或拖动选择多天
files-time-summary = { $count ->
   *[other] { $days } · { $count } 个文件
}
files-time-clear = 清除
files-time-month-back = 上个月
files-time-month-on = 下个月
files-time-wheel = 滚动以移动这些日期，并保持天数不变
files-sort-newest = 最新优先
files-sort-oldest = 最早优先
files-sort-largest = 最大优先
files-sort-name = 按名称
files-grid = 卡片
files-list = 列表
files-this-week = 本周
files-undated = 无日期
files-me = 我
files-no-subject = （无主题）
files-loading = 正在从你的邮件中收集文件…
files-empty = 你邮件中的文件会显示在这里。
files-none-match = 没有匹配的文件。
files-load-failed = 读取文件失败：{ $error }

## A file's menu and buttons

files-open = 打开
files-open-with = 打开方式…
files-save = 保存…
files-show-mail = 显示邮件
files-mail-window = 在新窗口中打开邮件
files-forward = 转发文件
files-from-them = 来自{ $name }的文件
files-copy-name = 复制文件名
files-name-copied = 已复制文件名
files-downloading = 正在下载邮件…
files-download-failed = 无法下载此邮件。

## A cloud drive in place of the mail files

files-drive-mine = 我的云端硬盘
files-drive-mine-onedrive = 我的文件
files-drive-results = “{ $words }”
files-drive-count = { $folders ->
    [0] { $files } 个文件
   *[other] { $folders } 个文件夹 · { $files } 个文件
}
files-drive-folders = 文件夹
files-drive-files = 文件
files-drive-folder = 文件夹
files-drive-meta = { $what } · 编辑于 { $date }
files-drive-as-link = { $what } · 以链接形式
files-drive-google-doc = Google 文档
files-drive-google-sheet = Google 表格
files-drive-google-slides = Google 幻灯片
files-drive-google-drawing = Google 绘图
files-drive-fetching = 正在获取…
files-drive-loading = 正在打开网盘…
files-drive-empty = 此文件夹为空。
files-drive-unreachable = 无法连接到 { $drive }。
files-drive-try-again = 重试
files-drive-needs-permission = Katna 需要获得你的一次授权才能显示此网盘。请重新登录，并允许 Katna 查看你的文件。
files-drive-allow = 允许
files-drive-allow-failed = 登录未完成，因此网盘仍未打开。
files-drive-attach = 附加
files-drive-more = 更多
files-drive-download = 下载…
files-drive-open-web = 在 { $drive } 中打开
files-drive-copy-link = 复制链接
files-drive-link-copied = 已复制链接
files-drive-share = 共享…
files-drive-rename = 重命名
files-drive-trash = 移至回收站
files-drive-trashed = “{ $name }”已移至 { $drive } 回收站
files-drive-renamed = 已重命名为“{ $name }”
files-drive-getting = 正在从 { $drive } 获取 { $name }…
files-drive-get-failed = 无法获取 { $name }：{ $error }
files-drive-upload = 上传
files-drive-upload-files = 上传文件
files-drive-upload-folder = 上传文件夹
files-drive-upload-failed = 无法上传 { $name }：{ $error }
files-drive-upload-needs = 要上传文件，Katna 需要获得你的一次授权：请在“设置”>“默认应用”>“‘文件’页面”中点击“允许”。

## The Share dialog of a drive file or folder

files-share-title = 共享“{ $name }”
files-share-add = 按姓名或地址添加用户
files-share-not-address = “{ $text }”不是电子邮件地址
files-share-notify = 也让 { $drive } 给他们发送电子邮件
files-share-people = 有访问权限的用户
files-share-general = 常规访问权限
files-share-loading = 正在读取谁有访问权限…
files-share-restricted = 受限
files-share-restricted-about = 只有拥有访问权限的用户才能通过链接打开
files-share-anyone = 任何知道链接的人
files-share-anyone-can = { $role ->
    [editor] 任何知道链接的人都可以编辑
    [commenter] 任何知道链接的人都可以评论
   *[viewer] 任何知道链接的人都可以查看
}
files-share-anyone-about = { $role ->
    [editor] 互联网上任何知道链接的人都可以编辑
    [commenter] 互联网上任何知道链接的人都可以评论
   *[viewer] 互联网上任何知道链接的人都可以查看
}
files-share-role-owner = 所有者
files-share-role-editor = 编辑者
files-share-role-commenter = 评论者
files-share-role-viewer = 查看者
files-share-you = { $name }（你）
files-share-domain = { $domain } 的所有人
files-share-inherited = 来自其所在文件夹的访问权限
files-share-remove = 移除访问权限
files-share-copy-link = 复制链接
files-share-share = 共享
files-share-done = 完成
files-share-sharing = 正在共享…
files-share-shared = { $count ->
   *[other] 已与 { $count } 人共享
}
files-share-refused = { $drive } 无法与 { $addresses } 共享
files-share-failed = 无法更改共享设置：{ $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
   *[other] 正在上传 { $count } 项
}
files-tray-done = { $count ->
   *[other] 已完成 { $count } 项上传
}
files-tray-some-failed = 已上传 { $done } 项，{ $failed } 项失败
files-tray-minutes-left = { $minutes ->
   *[other] 大约还剩 { $minutes } 分钟
}
files-tray-seconds-left = 还剩不到一分钟
files-tray-starting = 正在开始…
files-tray-cancel-all = 全部取消
files-tray-cancel = 取消
files-tray-fold = 隐藏列表
files-tray-unfold = 显示列表
files-tray-close = 关闭
files-tray-progress = { $place } · { $sent } / { $size }
files-tray-in = 在 { $place } 中
files-tray-cancelled = 已取消
