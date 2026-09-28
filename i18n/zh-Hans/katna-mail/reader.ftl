# Katna Mail, Chinese (Simplified) (简体中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = 关闭
reader-back = 返回
reader-mark-unread = 标记为未读
reader-move-to = 移至
reader-more = 更多
reader-original-colors = 显示原始颜色
reader-dark-colors = 以深色显示
reader-print-all = 全部打印
reader-new-window = 在新窗口中打开
reader-position = 第 { $position } 个，共 { $total } 个
reader-newer = 较新
reader-older = 较早

## Reading pane: the conversation

reader-removed = 此会话已被移除。
reader-no-subject = （无主题）
reader-collapse-all = 全部收起
reader-expand-all = 全部展开
reader-unknown-sender = （未知发件人）
reader-date-ago = { $date }（{ $ago }）
reader-me = 我
reader-to = 发送至 { $names }
reader-to-label = 发送至
reader-tick-delivered = 已送达 { $when }
reader-tick-no-bounce = 已发送 { $when }；没有退信，因此很可能已送达
reader-tick-bounced = 未送达：{ $when } 退信
reader-tick-read = 已读 { $when }（已读回执）
reader-tick-opened = 已打开，最近一次在 { $when }（打开跟踪）
reader-starred = 已加星标
reader-not-starred = 未加星标
reader-too-long = 邮件太长，无法完整显示。
reader-encrypted-images = 加密邮件中绝不会加载网络图片。
reader-window-failed = 无法打开新窗口。

## Reading pane: message details (opened from "to me")

reader-details-from = 发件人：
reader-details-to = 收件人：
reader-details-cc = 抄送：
reader-details-date = 日期：
reader-details-subject = 主题：

## Reading pane: downloading a message

reader-downloading = 正在从服务器下载此邮件…
reader-download-failed = 无法下载此邮件。
reader-try-again = 重试

## Reply row

reply-reply = 回复
reply-reply-all = 全部回复
reply-forward = 转发

## Encrypted and signed mail

security-decrypting = 正在解密…
security-checking = 正在检查签名…
security-partly-encrypted = 此邮件只有部分内容经过加密。其余内容是在保护范围之外添加的，可能来自任何人。
security-partly-signed = 此邮件只有部分内容经过签名。其余内容是在保护范围之外添加的，可能来自任何人。
security-encrypted = 加密邮件
security-encrypted-smime = 加密邮件（S/MIME）
security-no-key = 无法解密此邮件：它是用您没有的密钥加密的。
security-cancelled = 已取消解密。
security-damaged = 无法解密此邮件：加密数据已损坏或被更改。
security-decrypt-unavailable = 无法解密此邮件：请安装 { $tool } 以阅读加密邮件。
security-decrypt-failed = 无法解密此邮件：{ $reason }
security-unknown-signer = 未知签名者
security-signed-verified = 由 { $signer } 签名 · 已验证
security-signed-not-sender = 由 { $signer } 签名，但此人不是发件人
security-signed-untrusted = 由 { $signer } 签名，使用的密钥已被您标记为不可信
security-signed-unverified = 由 { $signer } 签名 · 密钥未经验证
security-bad-signature = 签名无效：此邮件在签名后被更改过，或签名是伪造的。
security-signature-expired = 由 { $signer } 签名 · 签名已过期
security-key-expired = 由 { $signer } 签名 · 密钥此后已过期
security-key-revoked = 由 { $signer } 签名，使用的密钥已被吊销
security-missing-key = 使用您没有的密钥签名，因此无法检查
security-missing-key-id = 使用您没有的密钥（{ $key }）签名，因此无法检查
security-signature-unavailable = 已签名；请安装 { $tool } 以检查签名
security-signature-error = 无法检查签名。
tracking-opened = { $who } 打开了 { $count } 次，最近一次在 { $when }
tracking-opens-clicks = { $who } 打开了 { $opens } 次，点开链接 { $clicks } 次，最近一次在 { $when }
tracking-clicked = { $who } 点开链接 { $clicks } 次，最近一次在 { $when }
tracking-maybe-opened = { $who } 可能已打开（Apple Mail 为保护隐私会加载图片）
tracking-not-opened = { $who } 尚未打开
tracking-receipt = { $who } 发回了已读回执
tracking-receipt-displayed = 已读回执：{ $who } 打开了你的邮件
tracking-receipt-other = 已读回执：{ $who } 未打开就删除或处理了你的邮件

## Remote images and pictures

remote-hidden = 此邮件中的图片已隐藏。
remote-show = 显示图片
remote-always-show = 始终显示此发件人的图片
remote-picture-use = 使用
remote-picture-too-big = 请选择不超过 8 MB 的图片。
remote-picture-type = 请选择 PNG、JPEG、GIF、WebP 或 SVG 图片。
remote-picture-read-failed = 无法读取图片：{ $error }
remote-picture-keep-failed = 无法保存图片：{ $error }
remote-picture-remove-failed = 无法移除图片：{ $error }

## Attachments

attachment-count = { $count } 个附件
attachment-save = 保存
attachment-save-all = 全部保存
attachment-save-all-tooltip = 将所有附件保存到一个文件夹
attachment-save-here = 保存到此处
attachment-not-downloaded = 此邮件未下载。
attachment-not-found = 在邮件中找不到此附件。
attachment-read-failed = 无法读取 { $name }
attachment-numbered = 附件 { $number }
attachment-saved-all = 已将 { $count } 个文件保存到 { $place }
attachment-saved-some = 已将 { $saved } 个文件（共 { $total } 个）保存到 { $place }。无法保存 { $failed }
attachment-saved-to = 已保存到 { $path }
attachment-save-failed = 无法保存 { $name }：{ $error }
attachment-open-failed = 无法打开 { $name }：{ $error }
attachment-risky = 此文件可能会运行程序，因此 Katna 不会打开它。请改为保存。
attachment-encrypted-open = 此文件是以加密形式收到的。请保存后在其他地方打开。

## Printing

print-failed = 无法打印：{ $error }
print-no-font = 找不到字体
print-opened-as-pdf = 已作为 PDF 打开，请从那里打印。
print-preview-title = 打印预览
print-preview-laying-out = 正在排版页面…
print-preview-pages = { $count } 页
print-preview-more = 还有 { $count } 页
print-preview-failed = 无法显示页面
print-preview-paper = 纸张
print-preview-a4 = A4
print-preview-letter = 信纸
print-preview-layout = 版式
print-preview-as-shown = 与显示一致
print-preview-simple = 纯文本
print-preview-backgrounds = 背景
print-preview-cancel = 取消
print-preview-print = 打印
print-not-downloaded = （尚未下载。）
print-encrypted = （已加密。请在 Katna Mail 中打开以打印其文本。）
print-to = 收件人：{ $addresses }
print-cc = 抄送：{ $addresses }

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = 打开此邮件即可查看其附件。
text-copy = 复制
text-select-all = 全选
