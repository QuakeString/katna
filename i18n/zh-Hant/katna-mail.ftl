# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Language picker (top bar and Settings > General)

language-tooltip = 語言：{ $language }
language-tooltip-system = 語言：{ $language }（依照系統設定）
language-search = 搜尋語言
language-system-default = 系統預設
language-system-now = 目前為{ $language }
language-no-match = 沒有符合「{ $query }」的語言
language-machine = 本翻譯由機器產生，歡迎協助改進
language-setting = 語言
language-setting-detail = 選單、按鈕和訊息的語言，以及日期和數字的格式。「系統預設」會依照桌面環境的設定。

## Dates and sizes

ago-just-now = 剛剛
ago-minutes = { $count } 分鐘前
ago-hours = { $count } 小時前
ago-days = { $count } 天前
size-bytes = { $count } 位元組
size-kb = { $size } KB
size-mb = { $size } MB
size-gb = { $size } GB
size-tb = { $size } TB

## Top bar

folders-hide = 隱藏資料夾
folders-show = 顯示資料夾
compose = 撰寫
search = 搜尋
search-mail = 搜尋郵件
search-settings = 搜尋設定
search-clear = 清除搜尋內容
search-options-show = 顯示搜尋選項
settings = 設定
account-add = 新增帳戶

## App rail (and the bottom bar on a phone)

rail-mail = 郵件
rail-calendar = 日曆
rail-contacts = 聯絡人
rail-tasks = 工作
rail-notes = 記事
rail-feeds = 資訊來源

## Pages of apps still to come

app-page-title = Katna { $app }
app-coming-soon = 即將推出
app-calendar-promise = 你的 CalDAV 日曆、郵件中的會議邀請和提醒，全都在收件匣旁邊。
app-tasks-promise = 與 CalDAV 同步的待辦清單，以及從郵件建立的工作。
app-notes-promise = 快速記事，以及為郵件或會話群組留下的記事，方便日後查看。
app-feeds-promise = 在郵件旁閱讀 RSS 和 Atom 資訊來源。

## Contacts page

app-contacts-loading = 正在從你的郵件中收集聯絡人…
app-contacts-empty = 與你有郵件往來的人會顯示在這裡。
app-contacts-count = 來自你郵件的 { $count } 位聯絡人，往來最多的排在最前面
app-contacts-top = 來自你郵件的前 { $count } 位聯絡人，往來最多的排在最前面
app-contacts-messages = { $count } 封郵件
app-contacts-last = 最近：{ $date }

## Navigation (the folders pane)

nav-labels = 標籤
nav-folders = 資料夾
nav-label-new = 建立新標籤
nav-folder-new = 建立新資料夾
nav-account-unnamed = 帳戶 { $number }
nav-tab-new = { $count } 封新郵件

## Special folders (the user's own folders keep their names)

folder-inbox = 收件匣
folder-starred = 已加星號
folder-drafts = 草稿
folder-sent = 寄件備份
folder-archive = 封存
folder-spam = 垃圾郵件
folder-trash = 垃圾桶
folder-all-mail = 所有郵件
folder-scheduled = 已排定

## New label / new folder dialog (Gmail accounts have labels, others folders)

label-new-title = 新增標籤
label-folder-new-title = 新增資料夾
label-prompt = 請輸入新的標籤名稱：
label-folder-prompt = 請輸入新的資料夾名稱：
label-name-hint = 標籤名稱
label-folder-name-hint = 資料夾名稱
label-nest = 將標籤置於以下標籤之下：
label-folder-nest = 將資料夾置於以下資料夾之下：
label-cancel = 取消
label-create = 建立
label-creating = 正在建立…
label-created = 已建立標籤「{ $name }」。
label-folder-created = 已建立資料夾「{ $name }」。

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = 主要
tab-promotions = 促銷內容
tab-social = 社交網路
tab-updates = 最新快訊
tab-forums = 論壇
tab-focused = 焦點
tab-other = 其他
tab-inbox = 收件匣
tab-newsletters = 電子報
tab-notifications = 通知
tab-new = { $count } 封新郵件
tab-provider-other = 由 Katna 分類

## Mail list: toolbar

list-select = 選取
list-refresh = 重新整理
list-more = 更多
list-mark-read = 標示為已讀取
list-mark-unread = 標示為未讀取
list-move-to = 移至
list-archive = 封存
list-spam = 檢舉垃圾郵件
list-delete = 刪除
list-newer = 較新
list-older = 較舊
list-range = 第 { $first }–{ $last } 列，共 { $total } 列
list-range-about = 第 { $first }–{ $last } 列，共約 { $total } 列
list-results = 「{ $query }」的搜尋結果
list-results-corrected = 目前顯示的是「{ $query }」的搜尋結果
list-search-instead = 改為搜尋「{ $query }」
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = 全部
list-pick-none = 無
list-pick-read = 已讀取
list-pick-unread = 未讀取
list-pick-starred = 已加星號
list-pick-unstarred = 未加星號

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] 已選取全部 { $count } 個會話群組。
   *[message] 已選取全部 { $count } 封郵件。
}
list-selected-all-in = { $kind ->
    [conversation] 已選取「{ $folder }」中的全部 { $count } 個會話群組。
   *[message] 已選取「{ $folder }」中的全部 { $count } 封郵件。
}
list-selected-screen = { $kind ->
    [conversation] 已選取此頁上的全部 { $count } 個會話群組。
   *[message] 已選取此頁上的全部 { $count } 封郵件。
}
list-select-all = { $kind ->
    [conversation] 選取全部 { $count } 個會話群組
   *[message] 選取全部 { $count } 封郵件
}
list-select-all-in = { $kind ->
    [conversation] 選取「{ $folder }」中的全部 { $count } 個會話群組
   *[message] 選取「{ $folder }」中的全部 { $count } 封郵件
}
list-clear-selection = 清除選取

## Mail list: empty states

list-empty-search = 沒有符合搜尋條件的郵件。
list-empty-tab = 「{ $tab }」中沒有郵件。
list-empty-tab-unknown = 這個分頁中沒有郵件。
list-empty-folder = 「{ $folder }」中沒有郵件。
list-empty-folder-unknown = 這個資料夾中沒有郵件。
list-first-sync = 正在取得你的郵件…
list-first-sync-detail = 郵件送達後會顯示在這裡。

## Mail list: lines

row-removed = 這封郵件已遭移除。
row-starred = 已加星號
row-not-starred = 未加星號
row-important = 重要。按一下即可標示為不重要。
row-mark-important = 標示為重要
row-pinned = 已置頂
row-pin = 置頂
row-unpin = 取消置頂

## Mail list: More menu and right-click menu

menu-reply = 回覆
menu-reply-all = 全部回覆
menu-forward = 轉寄
menu-archive = 封存
menu-delete = 刪除
menu-spam = 檢舉垃圾郵件
menu-mark-read = 標示為已讀取
menu-mark-unread = 標示為未讀取
menu-mark-all-read = 全部標示為已讀取
menu-star = 加上星號
menu-unstar = 移除星號
menu-important = 標示為重要
menu-not-important = 標示為不重要
menu-pin = 置頂
menu-unpin = 取消置頂
menu-print-all = 全部列印
menu-new-window = 在新視窗中開啟
menu-move-to = 移至
menu-move-to-heading = 移至：
menu-find-from = 搜尋來自 { $name } 的郵件

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] 已封存 { $count } 個會話群組。
   *[message] 已封存 { $count } 封郵件。
}
toast-trashed = { $kind ->
    [conversation] 已將 { $count } 個會話群組移至垃圾桶。
   *[message] 已將 { $count } 封郵件移至垃圾桶。
}
toast-moved = { $kind ->
    [conversation] 已移動 { $count } 個會話群組。
   *[message] 已移動 { $count } 封郵件。
}
toast-starred = { $kind ->
    [conversation] 已為 { $count } 個會話群組加上星號。
   *[message] 已為 { $count } 封郵件加上星號。
}
toast-unstarred = { $kind ->
    [conversation] 已移除 { $count } 個會話群組的星號。
   *[message] 已移除 { $count } 封郵件的星號。
}
toast-important = { $kind ->
    [conversation] 已將 { $count } 個會話群組標示為重要。
   *[message] 已將 { $count } 封郵件標示為重要。
}
toast-not-important = { $kind ->
    [conversation] 已將 { $count } 個會話群組標示為不重要。
   *[message] 已將 { $count } 封郵件標示為不重要。
}
toast-pinned = { $kind ->
    [conversation] 已置頂 { $count } 個會話群組。
   *[message] 已置頂 { $count } 封郵件。
}
toast-unpinned = { $kind ->
    [conversation] 已取消置頂 { $count } 個會話群組。
   *[message] 已取消置頂 { $count } 封郵件。
}
toast-spam = { $kind ->
    [conversation] 已將 { $count } 個會話群組檢舉為垃圾郵件。
   *[message] 已將 { $count } 封郵件檢舉為垃圾郵件。
}
toast-deleted-forever = { $kind ->
    [conversation] 已永久刪除 { $count } 個會話群組。
   *[message] 已永久刪除 { $count } 封郵件。
}
toast-undone = 已復原動作。
toast-undo = 復原
toast-no-spam-folder = 這個帳戶沒有垃圾郵件資料夾。

## Reading pane: toolbar

reader-close = 關閉
reader-back = 返回
reader-mark-unread = 標示為未讀取
reader-move-to = 移至
reader-more = 更多
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
reader-me = 我
reader-to = 寄給 { $names }
reader-starred = 已加星號
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

## Remote images and pictures

remote-hidden = 這封郵件中的圖片已隱藏。
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
attachment-save-all = 全部儲存
attachment-save-all-tooltip = 將所有附件儲存至資料夾
attachment-save-here = 儲存在這裡
attachment-not-downloaded = 這封郵件未下載。
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
print-not-downloaded = （尚未下載。）
print-encrypted = （已加密。請在 Katna Mail 中開啟以列印其內文。）
print-to = 收件者：{ $addresses }
print-cc = 副本：{ $addresses }
