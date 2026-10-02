# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = 新邮件
compose-restore = 还原
compose-minimize = 最小化
compose-exit-full-screen = 退出全屏
compose-open-window = 在新窗口中打开
compose-save-close = 保存并关闭
compose-back-to-mail = 返回邮件窗口
compose-pop-out-reply = 弹出回复
compose-edit-recipients = 编辑收件人
compose-summary-cc = 抄送：{ $names }
compose-summary-bcc = 密送：{ $names }
compose-more-recipients = 另外 { $count } 人
compose-show-trimmed = 显示被截去的内容
compose-hide-trimmed = 隐藏被截去的内容
compose-remove-trimmed = 移除引用的文字
compose-trimmed-removed = 已移除引用的文字

## Recipients and subject

compose-to = 收件人
compose-cc = 抄送
compose-bcc = 密送
compose-from = 发件人
compose-from-choose = 从其他账号发送
compose-recipients = 收件人
compose-subject = 主题

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = 请先发送或舍弃已打开的邮件。
compose-bad-address = “{ $address }”不是电子邮件地址。
compose-no-recipients = 请至少添加一位收件人。
compose-attachments-too-large = 附件共 { $size }；邮件服务器最多接受 { $limit }。
compose-no-account = 请添加一个用于发送邮件的账号。
compose-past-time = 请选择将来的时间。
compose-scheduling = 正在安排…
compose-sending = 正在发送…
compose-scheduled = 已安排在 { $when } 发送
compose-sent-archived = 已发送并归档
compose-sent = 邮件已发送
compose-discarded = 已舍弃草稿
compose-draft-saved = 草稿已保存
compose-draft-saving = 正在保存…
compose-draft-failed = 无法保存草稿：{ $error }
compose-draft-not-opened = 无法打开草稿。

## Attachments

compose-picker-insert = 插入
compose-picker-attach = 添加
compose-file-too-large = { $name } 太大：一封邮件最多可携带 { $limit }。
compose-forward-files-missing = 被转发邮件的文件尚未下载，因此未附加。
compose-attachment-size = （{ $size }）
compose-remove-attachment = 移除附件
compose-attachment-open-tip = 打开查看
compose-attachments-total = { $count } 个文件，共 { $size }
compose-drive-note = { $name } 超过 { $limit }，因此会存入你的 Google Drive，邮件中会附上链接。
compose-drive-tip = 在你的 Google Drive 中；邮件中会附上链接
compose-drive-uploading = 正在上传 { $percent }%
compose-drive-allow = 允许 Drive
compose-drive-allow-tip = 重新使用 Google 登录，让 Katna 可以把大文件放入你的 Drive
compose-drive-retry = 重试
compose-drive-sends-when-uploaded = { $name } 上传完成后即发送
compose-drive-not-uploaded = { $name } 尚未上传到 Google Drive
compose-drive-share-failed = 无法在 Google Drive 中共享这些文件：{ $error }
compose-drive-share-title = 与所有人共享这些文件？
compose-drive-share-text = { $count ->
   *[other] Google Drive 无法与没有 Google 账号的 { $addresses } 共享这些文件。改为让任何拥有链接的人都可以打开。
}
compose-drive-share-link = 通过链接共享
compose-drive-send-without = 不共享直接发送
compose-drive-share-cancel = 取消
compose-drive-card-detail = { $size } · Google Drive
compose-drive-card-name = Google Drive
compose-onedrive-note = { $name } 超过 { $limit }，因此会存入你的 OneDrive，邮件中会附上链接。
compose-onedrive-tip = 在你的 OneDrive 中；邮件中会附上链接
compose-onedrive-allow = 允许 OneDrive
compose-onedrive-allow-tip = 重新使用 Microsoft 登录，让 Katna 可以把大文件放入你的 OneDrive
compose-onedrive-not-uploaded = { $name } 尚未上传到 OneDrive
compose-onedrive-share-failed = 无法在 OneDrive 中共享这些文件：{ $error }
compose-onedrive-share-text = { $count ->
   *[other] OneDrive 无法与 { $addresses } 共享这些文件。改为让任何拥有链接的人都可以打开。
}
compose-onedrive-card-detail = { $size } · OneDrive
compose-onedrive-card-name = OneDrive
compose-drop-files = 将文件拖放到此处
compose-drop-here = 拖放到此处

## Paste options (a small bar under what was just pasted or dropped)

compose-paste-keep-formatting = 保留格式
compose-paste-table = 表格
compose-paste-picture = 图片
compose-paste-plain-text = 纯文本
compose-paste-inline = 嵌入正文
compose-paste-attachment = 附件

## Encryption and signing (the toggles by the recipients)

compose-encrypt = 加密
compose-encrypted = 已加密：只有收件人可以阅读
compose-sign = 签署
compose-signed = 已签署：收件人可以验证邮件确实来自你

## Open and click tracking and read receipts (toggles after Sign)

compose-track = 跟踪打开和点击
compose-tracked = 已跟踪：每位收件人打开邮件或点开链接时，你都能看到
compose-track-clicks = 跟踪链接点击（纯文本邮件无法显示是否被打开）
compose-tracked-clicks = 已跟踪：每位收件人点开链接时，你都能看到
compose-track-sign-in = 登录 Katna 账号即可跟踪打开和点击
compose-receipt = 请求已读回执
compose-receipt-on = 已请求已读回执：收件人的应用可能会询问对方是否发送回执
compose-delivery = 请求送达回执
compose-delivery-on = 已请求送达回执：每位收件人的服务器接收邮件时，你的邮件服务器会发邮件通知你
compose-delivery-unavailable = 你的邮件服务器不发送送达回执

## Spelling

spell-no-dictionary = 未安装 { $language } 的拼写词典（例如 hunspell-en_us）。
spell-dictionary-error = 拼写词典：{ $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = “{ $words }”
grammar-add = 添加“{ $words }”
grammar-remove = 删除“{ $words }”
grammar-ignore = 忽略

## Send checks (asked before a message goes out)

send-check-attachment-title = 你是否要添加附件？
send-check-attachment-text = 你在邮件中提到了附件，但没有添加任何附件。
send-check-attach = 添加文件
send-check-subject-title = 不填主题就发送？
send-check-subject-text = 这封邮件没有主题。
send-check-add-subject = 添加主题
send-check-send-anyway = 仍然发送

## Recipients (To, Cc and Bcc)

recipient-not-valid = 不是有效的电子邮件地址
recipient-show-address = 显示地址
recipient-remove = 移除
recipient-bad-title = 检查地址
recipient-bad-text = “{ $address }”不是有效的电子邮件地址。请在发送前修正或移除它。
recipient-bad-fix = 修正
