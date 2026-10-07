# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The signature being edited

signature-placeholder = 你的姓名，以及要写在下面的任何内容

## Its formatting bar

signature-bold = 加粗
signature-italic = 倾斜
signature-underline = 下划线
signature-link = 链接
signature-link-apply = 应用
signature-picture = 插入图片
signature-align-left = 左对齐
signature-align-center = 居中
signature-align-right = 右对齐
signature-numbered-list = 编号列表
signature-bulleted-list = 项目符号列表
signature-remove-formatting = 清除格式

## Adding a picture

signature-picture-choose = 插入
signature-picture-too-big = 签名中的图片最大为 { $size }。
signature-picture-kind = 请选择 PNG、JPEG、GIF 或 WebP 图片。
signature-picture-unreadable = { $name }：{ $error }

## Layouts

signature-layout = 布局
signature-layout-own = 自定义
signature-layout-classic = 经典
signature-layout-logo-left = 标志居左
signature-layout-photo = 照片
signature-layout-band = 色带
signature-layout-one-line = 单行
signature-layout-centred = 居中
signature-layout-banner = 带横幅
signature-layout-underline = 下划线
signature-layout-side-bar = 侧边条
signature-layout-card = 卡片
signature-layout-monogram = 首字母
signature-layout-plain = 纯文本
signature-layout-mobile-label = M:
signature-layout-office-label = O:
signature-layout-email-label = E:
signature-layout-name = 姓名
signature-layout-job = 职位
signature-layout-company = 公司
signature-layout-mobile = 手机
signature-layout-office = 办公电话
signature-layout-email = 电子邮件
signature-layout-website = 网站
signature-layout-address = 地址
signature-layout-pictures = 图片
signature-layout-logo = 标志
signature-layout-photo-picture = 照片
signature-layout-banner-picture = 横幅
signature-layout-remove-picture = 移除
signature-layout-pages = 主页
signature-layout-page-placeholder = 添加主页地址
signature-layout-colour = 颜色
signature-layout-picture-failed = 无法将 { $name } 用作图片。
signature-layout-preview = 收件人看到的样子
signature-layout-light = 浅色
signature-layout-dark = 深色
signature-layout-text = 纯文本
signature-layout-inside = 图片会嵌入邮件中发送，因此即使对方关闭了网络图片也能显示。这会让每封邮件增加 { $size }。
signature-layout-free = 想要其他样式？
signature-layout-edit = 手动编辑
signature-layout-edit-confirm = 要手动编辑吗？其字段和布局将被移除，外观会在编辑器所能支持的范围内保留。
signature-layout-use-confirm = 要使用“{ $layout }”布局吗？它会替换此签名，并用此签名的内容填充。
signature-layout-use = 使用布局
signature-layout-cancel = 取消

## Paste HTML

signature-html-title = 粘贴 HTML
signature-html-subtitle = 适用于在其他地方设计的签名
signature-html-placeholder = 在此粘贴签名的 HTML
signature-html-name = 已粘贴
signature-html-new = 将另存为新签名“{ $name }”
signature-html-replaces = 将覆盖“{ $name }”
signature-html-cancel = 取消
signature-html-save = 保存
signature-html-fetching = 正在下载其中的图片…
signature-html-pictures-inside = { $count ->
   *[other] 已下载 { $count } 张图片并嵌入邮件中（{ $size }）
}
signature-html-pictures-web = { $count ->
   *[other] 有 { $count } 张图片无法下载，因此收件人会从网络加载
}
signature-html-removed = 已移除脚本、表单和跟踪像素，邮件应用本来也会阻止这些内容
signature-html-style-sheet = 已省略样式表：邮件只保留写在各元素上的样式
signature-html-links = 已移除指向网站、邮件地址或电话以外位置的链接
signature-html-plain-text = 已据此生成纯文本版本，供只显示文本的邮件应用使用

## Import

signature-import-title = 导入
signature-import-subtitle = 从 Gmail、Thunderbird、Evolution 和 KMail 导入
signature-import-looking = 正在查找签名…
signature-import-none = 未找到签名。如需从其他应用导入，请复制其签名的 HTML 并使用“粘贴 HTML”。
signature-import-from = 来自 { $app }
signature-import-already = 已在 Katna 中
signature-import-gmail-sign-in = { $address }：请在“设置 > 账号”中重新登录，以便 Katna 读取 Gmail 的签名。
signature-import-gmail-failed = { $address }：{ $error }
signature-import-cancel = 取消
signature-import-do = { $count ->
   *[other] 导入 { $count } 个签名
}
signature-import-name = { $name }（{ $app }）
