# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Reading pane: toolbar

reader-close = 關閉
reader-back = 返回
reader-mark-unread = 標示為未讀取
reader-move-to = 移至
reader-snooze = 延後
reader-remind = 提醒我
reader-more = 更多
reader-original-colors = 顯示原始色彩
reader-dark-colors = 以深色顯示
reader-print-all = 全部列印
reader-new-window = 在新視窗中開啟
reader-position = 第 { $position } 個，共 { $total } 個
reader-newer = 較新
reader-older = 較舊

## Reading pane: the conversation

reader-removed = 這個會話群組已遭移除。
reader-no-subject = （無主旨）
reader-collapse-all = 全部收合
reader-expand-all = 全部展開
reader-unknown-sender = （不明寄件者）
reader-date-ago = { $date }（{ $ago }）
reader-sending = 正在傳送…
reader-me = 我
reader-to = 寄給 { $names }
reader-to-label = 寄給
reader-tick-delivered = 已送達 { $when }
reader-tick-no-bounce = 已寄出 { $when }；沒有退信，因此很可能已送達
reader-tick-bounced = 未送達：{ $when } 退信
reader-tick-read = 已讀 { $when }（已讀回條）
reader-tick-opened = 已開啟，最近一次在 { $when }（開信追蹤）
reader-starred = 已加星號
reader-chip-remove = 移除 { $label }
reader-not-starred = 未加星號
reader-too-long = 郵件過長，無法完整顯示。
reader-encrypted-images = 加密郵件一律不會載入網路上的圖片。
reader-window-failed = 無法開啟新視窗。

## Reading pane: message details (opened from "to me")

reader-details-from = 寄件者：
reader-details-to = 收件者：
reader-details-cc = 副本：
reader-details-date = 日期：
reader-details-subject = 主旨：

## Reading pane: downloading a message

reader-downloading = 正在從伺服器下載這封郵件…
reader-download-failed = 無法下載這封郵件。
reader-download-failed-reason = 無法下載這封郵件。{ $reason }
reader-download-offline = 此帳戶目前離線。請連上網路以下載這封郵件。
reader-try-again = 再試一次

## Reply row

reply-reply = 回覆
reply-reply-all = 全部回覆
reply-forward = 轉寄

## Encrypted and signed mail

security-decrypting = 正在解密…
security-checking = 正在檢查簽章…
security-partly-encrypted = 這封郵件只有部分內容經過加密。其餘內容是在保護範圍外加入的，可能來自任何人。
security-partly-signed = 這封郵件只有部分內容經過簽署。其餘內容是在保護範圍外加入的，可能來自任何人。
security-encrypted = 加密郵件
security-encrypted-smime = 加密郵件（S/MIME）
security-no-key = 無法解密這封郵件：它是用你沒有的金鑰加密的。
security-cancelled = 已取消解密。
security-damaged = 無法解密這封郵件：加密資料已損毀或遭到變更。
security-decrypt-unavailable = 無法解密這封郵件：請安裝 { $tool } 以閱讀加密郵件。
security-decrypt-failed = 無法解密這封郵件：{ $reason }
security-unknown-signer = 不明簽署者
security-signed-verified = 由 { $signer } 簽署 · 已驗證
security-signed-not-sender = 由 { $signer } 簽署，但此人並非寄件者
security-signed-untrusted = 由 { $signer } 簽署，使用的金鑰已被你標示為不信任
security-signed-unverified = 由 { $signer } 簽署 · 金鑰未經驗證
security-bad-signature = 簽章無效：這封郵件在簽署後遭到變更，或簽章是偽造的。
security-signature-expired = 由 { $signer } 簽署 · 簽章已過期
security-key-expired = 由 { $signer } 簽署 · 金鑰之後已過期
security-key-revoked = 由 { $signer } 簽署，使用的金鑰已被撤銷
security-missing-key = 使用你沒有的金鑰簽署，因此無法檢查
security-missing-key-id = 使用你沒有的金鑰（{ $key }）簽署，因此無法檢查
security-signature-unavailable = 已簽署；請安裝 { $tool } 以檢查簽章
security-signature-error = 無法檢查簽章。
security-look-up-key = 查詢金鑰

## The key popover: the details of the key a message was signed with, and
## a key to import (looked up, or attached to the message)

key-card-verified = 簽章已驗證
key-card-verified-detail = 簽章有效，而且你信任這把金鑰。
key-card-unverified = 簽章未經驗證
key-card-unverified-detail = 簽章有效，但無法確認這把金鑰屬於對方。請與對方核對指紋，再到 GnuPG 中信任這把金鑰（使用 Kleopatra 或 gpg --edit-key）。
key-card-not-sender = 由他人簽署
key-card-not-sender-detail = 簽章有效，但這把金鑰不屬於寄件者。
key-card-untrusted = 金鑰不受信任
key-card-untrusted-detail = 你已在 GnuPG 中將這把金鑰標示為不信任。
key-card-signature-expired = 簽章已過期
key-card-signature-expired-detail = 簽章原本有效，但已經過期。
key-card-key-expired = 金鑰已過期
key-card-key-expired-detail = 簽章有效，但金鑰之後已過期。
key-card-key-revoked = 金鑰已撤銷
key-card-key-revoked-detail = 這把金鑰已被擁有者撤銷，因此無法信任這個簽章。
key-card-bad = 簽章無效
key-card-bad-detail = 這封郵件在簽署後遭到變更，或簽章是偽造的。
key-card-signed-by = 簽署者
key-card-belongs-to = 屬於
key-card-fingerprint = 指紋
key-card-signed = 簽署時間
key-card-key = 金鑰
key-card-kind = { $standard }，{ $algorithm }
key-card-created = 建立時間
key-card-expires = 到期時間
key-card-never = 永不
key-card-issued-by = 簽發者
key-card-found-in = 來源
key-card-keyring = 你的 GnuPG 金鑰圈
key-card-copy = 複製指紋
key-card-import-title = 要匯入這把金鑰嗎？
key-card-from-directory = 在 { $domain } 的金鑰目錄中找到。
key-card-from-attachment = 來自附件 { $name }。
key-card-import-note = 匯入後，Katna 就能檢查這個人的簽章，並寄送加密郵件給對方。若要完全信任這把金鑰，請與對方核對指紋。
key-card-cancel = 取消
key-card-import = 匯入金鑰
key-card-looking-up = 正在查詢金鑰…
key-card-looking-up-detail = 正在向 { $domain } 的金鑰目錄查詢。
key-card-not-found = 找不到金鑰
key-card-not-found-detail = { $domain } 沒有為這個地址發布金鑰。請寄件者把他們的金鑰寄給你。
key-card-not-kept = 找到的金鑰無法使用。
key-card-failed = 無法取得金鑰

## Open and click tracking and read receipts (the eye's popover beside a
## sent message's star, and the line above a read receipt)

tracking-opened = { $who } 開啟了 { $count } 次，最近一次在 { $when }
tracking-opens-clicks = { $who } 開啟了 { $opens } 次，點開連結 { $clicks } 次，最近一次在 { $when }
tracking-clicked = { $who } 點開連結 { $clicks } 次，最近一次在 { $when }
tracking-maybe-opened = { $who } 可能已開啟（Apple Mail 為保護隱私會載入圖片）
tracking-seen-none = 還沒有人開啟或點開連結
tracking-receipt = { $who } 傳回了已讀回條
tracking-receipt-read = { $who } 已閱讀（已讀回條），{ $when }
tracking-receipt-displayed = 已讀回條：{ $who } 開啟了你的郵件
tracking-receipt-other = 已讀回條：{ $who } 未開啟就刪除或處理了你的郵件

## Remote images and pictures

remote-hidden = 這封郵件中的圖片已隱藏。
remote-hidden-unconfirmed = 圖片已隱藏：無法確認寄件者。
remote-show = 顯示圖片
remote-always-show = 一律顯示這位寄件者的圖片
remote-picture-use = 使用
remote-picture-too-big = 請選擇 8 MB 以下的圖片。
remote-picture-type = 請選擇 PNG、JPEG、GIF、WebP 或 SVG 圖片。
remote-picture-read-failed = 無法讀取圖片：{ $error }
remote-picture-keep-failed = 無法保存圖片：{ $error }
remote-picture-remove-failed = 無法移除圖片：{ $error }

## Attachments

attachment-count = { $count } 個附件
attachment-save = 儲存
attachment-forward = 轉寄
attachment-save-all = 全部儲存
attachment-save-all-tooltip = 將所有附件儲存至資料夾
attachment-save-here = 儲存在這裡
attachment-not-downloaded = 這封郵件未下載。
attachment-open-message = 開啟這封郵件即可查看其中的附件。
attachment-not-found = 在郵件中找不到這個附件。
attachment-read-failed = 無法讀取 { $name }
attachment-numbered = 附件 { $number }
attachment-saved-all = 已將 { $count } 個檔案儲存至 { $place }
attachment-saved-some = 已將 { $saved } 個檔案（共 { $total } 個）儲存至 { $place }。無法儲存 { $failed }
attachment-saved-to = 已儲存至 { $path }
attachment-save-failed = 無法儲存 { $name }：{ $error }
attachment-open-failed = 無法開啟 { $name }：{ $error }
attachment-risky = 這個檔案可能會執行程式，因此 Katna 不會開啟它。請改為儲存。
attachment-encrypted-open = 這個檔案是以加密形式收到的。請先儲存，再用其他程式開啟。

## Printing

print-failed = 無法列印：{ $error }
print-no-font = 找不到字型
print-opened-as-pdf = 已開啟為 PDF，請從那裡列印。
print-preview-title = 預覽列印
print-preview-laying-out = 正在排列頁面…
print-preview-pages = { $count } 頁
print-preview-more = 還有 { $count } 頁
print-preview-failed = 無法顯示頁面
print-preview-paper = 紙張
print-preview-a4 = A4
print-preview-letter = 信紙
print-preview-layout = 版面
print-preview-as-shown = 與畫面相同
print-preview-simple = 純文字
print-preview-backgrounds = 背景
print-preview-cancel = 取消
print-preview-print = 列印
print-not-downloaded = （尚未下載。）
print-encrypted = （已加密。請在 Katna Mail 中開啟以列印其內文。）
print-to = 收件者：{ $addresses }
print-cc = 副本：{ $addresses }

## Message text (right-click menu in the reading pane)

text-pin = 釘選到頂端
text-copy-address = 複製地址
text-copy = 複製
text-select-all = 全選
