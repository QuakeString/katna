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

## Leftovers from earlier areas (reading pane and its right-click menu)

attachment-open-message = 開啟這封郵件即可查看其中的附件。
text-copy = 複製
text-select-all = 全選

## Settings page: its tabs

settings-tab-general = 一般
settings-tab-inbox = 收件匣
settings-tab-accounts = 帳戶
settings-tab-subscriptions = 訂閱
settings-tab-appearance = 外觀
settings-tab-shortcuts = 快速鍵
settings-tab-default-apps = 預設應用程式
settings-tab-folders-rules = 資料夾和規則
settings-tab-compose = 撰寫
settings-tab-mcp-server = MCP 伺服器
settings-tab-feedback = 使用者意見回饋
settings-tab-experimental = 實驗性功能

## Settings page: tabs still to come

settings-tab-subscriptions-coming = 查看你收到的電子報和郵寄清單，按一下即可取消訂閱。
settings-tab-folders-rules-coming = 建立、重新命名、移動及隱藏資料夾和標籤，並選擇要同步哪些項目。規則會依寄件者、主旨或字詞，自動為新郵件分類、加上標籤、轉寄或刪除。
settings-tab-mcp-server-coming = 讓這台電腦上的 AI 助理在你同意下搜尋、閱讀你的郵件並撰寫草稿。

## Settings > General

settings-general-conversations = 會話群組檢視
settings-general-conversations-group = 將同一封郵件的回覆歸為一組
settings-general-conversations-group-detail = 清單中每個會話群組只佔一列
settings-general-reading = 閱讀
settings-general-newest-first = 最新郵件排在最前面
settings-general-newest-first-detail = 會話群組從最新的回覆開始顯示
settings-general-full-headers = 顯示完整標頭
settings-general-full-headers-detail = 每封郵件都會展開寄件者、收件者、副本、日期和主旨
settings-general-full-names = 收件者全名
settings-general-full-names-detail = 顯示「寄給我、Ada Lovelace」而非「寄給我、Ada」
settings-general-mark-read = 標示為已讀取
settings-general-mark-read-now = 開啟後立即標示
settings-general-mark-read-1s = 開啟 1 秒後
settings-general-mark-read-3s = 開啟 3 秒後
settings-general-mark-read-never = 只在我手動標示時
settings-general-reply-button = 回覆按鈕
settings-general-reply-all = 回覆所有人
settings-general-reply-all-detail = 每封郵件旁的回覆按鈕會回覆所有人，而不只是寄件者
settings-general-remote-images = 網路上的圖片
settings-general-remote-images-detail = 載入郵件中的圖片會讓寄件者知道你已開啟郵件、開啟的時間和大概的位置。關閉時，每封郵件都會先詢問你，而你隨時都能顯示某位寄件者的圖片。
settings-general-remote-images-always = 一律顯示圖片
settings-general-remote-images-always-detail = 所有郵件皆適用，不限於你信任的寄件者
settings-general-sending = 傳送
settings-general-sending-detail = 已傳送的郵件要等候多久，以便你取消傳送。
settings-general-offline = 離線郵件
settings-general-offline-detail = 近期的郵件會完整下載，讓你在沒有網路連線時也能閱讀。較舊的郵件會在你開啟時才下載。
settings-general-offline-days = { $count } 天
settings-general-offline-years = { $count } 年
settings-general-offline-all = 所有郵件
settings-general-offline-note = 選擇較少的天數時，已下載的郵件仍會保留。伺服器上的內容不會有任何變更。
settings-general-notifications = 通知
settings-general-notifications-detail = 收件匣收到新郵件時通知，即使 Katna Mail 已關閉也一樣。
settings-general-new-mail = 收到新郵件時通知我
settings-general-new-mail-detail = 附有「全部回覆」、「標示為已讀取」和「封存」按鈕
settings-general-new-mail-sound = 播放音效
settings-general-new-mail-sound-detail = 桌面環境的新郵件音效
settings-general-desktop = 桌面
settings-general-open-at-login = 登入時開啟 Katna Mail
settings-general-open-at-login-detail = 無論如何，只要背景服務在執行，登入後都會同步郵件
settings-general-tray = 在系統匣中顯示 Katna
settings-general-tray-detail = 顯示未讀取郵件數和選單
settings-general-unread-badge = 在工作列圖示上顯示未讀取郵件數
settings-general-unread-badge-detail = 收件匣中有多少封未讀取的郵件

## Settings > Inbox

settings-inbox-tabs = 收件匣分頁
settings-inbox-tabs-detail = 依照你的郵件服務供應商網站的方式，將收件匣分成多個分頁。
settings-inbox-tabs-show = 顯示收件匣分頁
settings-inbox-tabs-show-detail = 關閉時，每個帳戶都只顯示一份清單
settings-inbox-no-accounts = 新增帳戶後即可選擇其分頁。
settings-inbox-tabs-automatic = 自動：{ $tabs }（{ $provider }）
settings-inbox-tabs-off = 不使用分頁
settings-inbox-tabs-gmail = 主要、促銷內容、社交網路、最新快訊、論壇
settings-inbox-tabs-focused = 焦點和其他
settings-inbox-tabs-zoho = 收件匣、電子報和通知
settings-inbox-tabs-shown = 顯示的分頁。你關閉的分頁中的郵件會留在「{ $tab }」。

## Settings > Appearance

settings-appearance-reading-pane = 閱讀窗格
settings-appearance-reading-pane-detail = 開啟的會話群組要顯示在哪裡。
settings-appearance-pane-right = 清單右側
settings-appearance-pane-none = 不分割
settings-appearance-density = 顯示密度
settings-appearance-density-default = 預設
settings-appearance-density-compact = 精簡
settings-appearance-scaling = 縮放
settings-appearance-scaling-detail = 在桌面環境本身的縮放比例之上，放大或縮小 Katna Mail 中的所有內容：文字、圖示、間距和分隔線。你寄出的郵件會保留原本的字型大小。尺寸太小可能會讓圖示難以點選。
settings-appearance-theme = 主題
settings-appearance-theme-system = 與桌面相同
settings-appearance-theme-light = 淺色
settings-appearance-theme-dark = 深色
settings-appearance-desktop-colors = 桌面色彩
settings-appearance-desktop-colors-use = 使用桌面的色彩
settings-appearance-desktop-colors-use-detail = 桌面的配色和強調色
settings-appearance-app-names = 應用程式名稱
settings-appearance-app-names-show = 顯示應用程式名稱
settings-appearance-app-names-show-detail = 顯示在最左側應用程式圖示下方的名稱
settings-appearance-sender-pictures = 寄件者圖片
settings-appearance-sender-pictures-show = 顯示公司標誌
settings-appearance-sender-pictures-show-detail = 只依寄件者的網域查詢，絕不依郵件查詢，並保留一週
settings-appearance-important = 重要標記
settings-appearance-important-show = 顯示重要標記
settings-appearance-important-show-detail = 顯示在清單中每封郵件旁
settings-appearance-message-width = 郵件寬度
settings-appearance-message-width-limit = 限制郵件寬度
settings-appearance-message-width-limit-detail = 在寬視窗中，長行文字會比較容易閱讀
settings-appearance-mail-colors = 郵件色彩
settings-appearance-mail-colors-detail = 大部分郵件是以白色頁面設計的。使用深色主題時，郵件的色彩會改為易於閱讀的深色；關閉時，郵件會在淺色頁面上保留寄件者的色彩。
settings-appearance-dark-mail = 郵件也使用深色
settings-appearance-dark-mail-detail = 僅限使用深色主題時
settings-appearance-attachment-previews = 附件預覽
settings-appearance-attachment-previews-show = 顯示附件預覽
settings-appearance-attachment-previews-show-detail = 在每個檔案的資訊卡上顯示內容縮圖

## Settings > Default apps

settings-default-apps-intro = 按一下附件時要用哪個應用程式開啟。檢視器也隨時可以用其他應用程式開啟檔案。桌面環境的預設應用程式請在其本身的設定中變更。
settings-default-apps-pdf = PDF 檔案
settings-default-apps-pdf-detail = 分頁顯示，可縮放。
settings-default-apps-pictures = 圖片
settings-default-apps-pictures-detail = 相片（自動轉正）、PNG、GIF、WebP、BMP、TIFF 和 SVG。
settings-default-apps-text = 文字檔
settings-default-apps-text-detail = 純文字、記錄檔、程式碼和其他文字。
settings-default-apps-sheets = 試算表
settings-default-apps-sheets-detail = Excel（xlsx、xls）、OpenDocument（ods）和 CSV。
settings-default-apps-documents = 文件
settings-default-apps-documents-detail = Word（docx）和 OpenDocument 文字（odt）。
settings-default-apps-katna = Katna Mail 的檢視器
settings-default-apps-system = 桌面環境的預設應用程式
settings-default-apps-ask = 每次詢問要用哪個應用程式
settings-default-apps-after-saving = 儲存後
settings-default-apps-show-folder = 在資料夾中顯示已儲存的檔案
settings-default-apps-show-folder-detail = 開啟檔案管理員並選取已儲存的附件

## Settings > Compose

settings-compose-send-from = 新郵件的寄件帳戶
settings-compose-send-from-detail = 回覆和轉寄一律從你目前所在的帳戶寄出。
settings-compose-send-from-current = 目前所在的帳戶
settings-compose-send-on-replies = 回覆時的傳送方式
settings-compose-send-on-replies-detail = 回覆或轉寄時「傳送」按鈕的動作。「傳送」旁的選單提供另一個選項。
settings-compose-send-plain = 傳送
settings-compose-send-archive = 傳送並封存
settings-compose-signatures = 簽名
settings-compose-signatures-detail = 加在你的郵件下方「--」這一行之後。你可以在撰寫視窗中選擇其他簽名。
settings-compose-untitled = 未命名
settings-compose-signature-name = 名稱，例如「工作」
settings-compose-signature-first = 我的簽名
settings-compose-signature-numbered = 簽名 { $number }
settings-compose-signature-delete = 刪除
settings-compose-signature-deleted = 已刪除簽名
settings-compose-signature-new = 新建
settings-compose-no-signatures = 尚未建立任何簽名。
settings-compose-no-signature = 不使用簽名
settings-compose-for-new-mail = 用於新郵件
settings-compose-for-replies = 用於回覆和轉寄
settings-compose-for-replies-detail = 如果你曾在會話群組中的郵件使用某個簽名，回覆時會改用該簽名。
settings-compose-format = 格式
settings-compose-plain-text = 以純文字撰寫
settings-compose-plain-text-detail = 新郵件一開始不含格式；撰寫視窗中可以切換
settings-compose-spelling = 拼字
settings-compose-spell-check = 撰寫時檢查拼字
settings-compose-spell-check-detail = 拼錯的字詞會加上底線，按右鍵即可查看建議
settings-compose-spell-desktop = 桌面環境的語言（{ $language }）
settings-compose-templates = 範本
settings-compose-templates-detail = 儲存你常寫的郵件，並以此開始撰寫新郵件或回覆。

## Settings > Shortcuts

settings-shortcuts-set = 快速鍵組合
settings-shortcuts-set-detail = 從你熟悉的郵件應用程式按鍵開始。這裡的 Cmd 即為 Ctrl。你自己的變更會套用在組合之上，「還原預設值」會回到組合的按鍵。
settings-shortcuts-single = 單鍵快速鍵
settings-shortcuts-single-detail = 不需按 Ctrl 或 Alt 的按鍵，就像網頁版郵件一樣：e 封存、j 和 k 移動、/ 搜尋。可在清單和開啟的會話群組中使用，輸入文字時不會觸發。
settings-shortcuts-single-use = 使用單鍵快速鍵
settings-shortcuts-single-use-detail = Ctrl 快速鍵一律有效
settings-shortcuts-how = 按一下按鍵即可變更，或按一下 + 新增，然後按下新的按鍵。按 Esc 可取消。
settings-shortcuts-restore = 還原預設值
settings-shortcuts-no-key = 無按鍵
settings-shortcuts-press = 請按下按鍵…
settings-shortcuts-then = { $keys }，然後…
settings-shortcuts-moved = { $keys } 現在會執行「{ $action }」，而不是「{ $previous }」。
settings-shortcuts-single-off = 單鍵快速鍵目前已關閉，開啟後這個按鍵才會生效。
settings-shortcuts-restored = 所有快速鍵都已還原為組合的按鍵。

## Settings search: the line under a result

settings-general-language-summary = 應用程式、日期和數字的語言
settings-general-reading-summary = 最新郵件排在最前面、完整標頭、收件者全名
settings-general-mark-read-summary = 開啟的會話群組何時標示為已讀取：立即、1 或 3 秒後，或手動
settings-general-reply-button-summary = 每封郵件旁的回覆按鈕會回覆所有人
settings-general-remote-images-summary = 一律顯示所有郵件中的圖片
settings-general-sending-summary = 取消傳送：已傳送的郵件要等候多久，以便你取消傳送
settings-general-offline-summary = 要完整下載幾天內的近期郵件，以便離線閱讀
settings-general-notifications-summary = 新郵件通知及其音效
settings-general-desktop-summary = 登入時開啟 Katna Mail、系統匣圖示，以及工作列圖示上的未讀取郵件數
settings-accounts-accounts-summary = 新增或移除帳戶，或變更帳戶圖片
settings-appearance-density-summary = 清單中的列採用預設或精簡顯示
settings-appearance-scaling-summary = 放大或縮小所有內容：文字、圖示、間距和分隔線
settings-appearance-theme-summary = 與桌面相同、淺色或深色
settings-appearance-sender-pictures-summary = 依寄件者網域查詢的公司標誌
settings-appearance-important-summary = 清單中每封郵件旁的重要標記
settings-appearance-mail-colors-summary = 在深色主題中讓 HTML 郵件使用深色，或保留寄件者的色彩
settings-appearance-attachment-previews-summary = 每個附件內容的縮圖
settings-shortcuts-set-summary = 從 Gmail、Inbox by Gmail、Apple Mail、Outlook 或 Thunderbird 的按鍵開始
settings-shortcuts-single-summary = 不需按 Ctrl 或 Alt 的按鍵，就像網頁版郵件一樣
settings-default-apps-pdf-summary = PDF 附件要在哪裡開啟
settings-default-apps-pictures-summary = 相片和圖片要在哪裡開啟
settings-default-apps-text-summary = 純文字、記錄檔和程式碼要在哪裡開啟
settings-default-apps-sheets-summary = Excel、OpenDocument 和 CSV 檔案要在哪裡開啟
settings-default-apps-documents-summary = Word 和 OpenDocument 文字要在哪裡開啟
settings-default-apps-after-saving-summary = 在資料夾中顯示已儲存的附件
settings-compose-send-from-summary = 新郵件的寄件帳戶：目前所在的帳戶，或一律使用同一個帳戶
settings-compose-send-on-replies-summary = 回覆和轉寄時是「傳送」，還是「傳送並封存」會話群組
settings-compose-signatures-summary = 加在你的郵件下方「--」這一行之後
settings-compose-for-new-mail-summary = 新郵件預設使用的簽名
settings-compose-for-replies-summary = 回覆和轉寄預設使用的簽名
settings-compose-format-summary = 以純文字撰寫新郵件
settings-compose-spelling-summary = 撰寫時檢查拼字，以及字典的語言
settings-compose-templates-summary = 即將推出：儲存你常寫的郵件，並以此開始撰寫新郵件或回覆
settings-feedback-crash-reports-summary = Katna Mail 或其背景服務當機時，將當機報告儲存在這台電腦上
settings-feedback-saved-summary = 查看、複製或刪除這台電腦上儲存的當機報告
settings-feedback-help-improve-summary = 傳送當機報告以協助修正問題；除非你開啟，否則預設為關閉
settings-experimental-blur-summary = 透過頂端列看見模糊的桌面，選單則呈現毛玻璃效果
settings-search-shortcut = 鍵盤快速鍵
settings-search-tab = 設定分頁
settings-search-none = 沒有符合「{ $query }」的設定。
settings-search-results = 符合「{ $query }」的設定

## Quick settings (the panel that slides in from the right)

quick-title = 快速設定
quick-see-all = 查看所有設定
quick-reading-pane = 閱讀窗格
quick-pane-right = 清單右側
quick-pane-none = 不分割
quick-density = 顯示密度
quick-density-default = 預設
quick-density-compact = 精簡
quick-theme = 主題
quick-theme-system = 與桌面相同
quick-theme-light = 淺色
quick-theme-dark = 深色
quick-desktop-colors = 桌面色彩
quick-desktop-colors-detail = 桌面的配色和強調色
quick-app-names = 應用程式名稱
quick-app-names-detail = 顯示在最左側應用程式圖示下方的名稱
quick-inbox-tabs = 收件匣分頁
quick-inbox-tabs-detail = 各帳戶郵件服務供應商的分頁
quick-choose-tabs = 選擇分頁
quick-choose-tabs-detail = 在「設定」中依帳戶選擇
quick-sending = 傳送
quick-undo-send = 取消傳送
quick-undo-send-off = 關閉
quick-undo-send-seconds = { $seconds } 秒
quick-signatures = 簽名
quick-signatures-none = 尚未建立
quick-signatures-one = { $name }，預設使用
quick-signatures-many = { $count } 個簽名；預設使用 { $name }
quick-signatures-no-default = { $count } 個，未設定預設簽名
quick-signature-untitled = 未命名
quick-threading = 郵件串
quick-conversation-view = 會話群組檢視
quick-conversation-view-detail = 將同一封郵件的回覆歸為一組
quick-help = 說明
quick-tour = 開始導覽
quick-whats-new = 最新消息
quick-about = 關於 Katna

## Settings: opening at login

settings-open-at-login-failed = 無法變更登入時開啟的設定：{ $error }

## Settings > Appearance > Scaling

scale-letter = 字
scale-percent = { $percent }%
scale-reset = 恢復為 { $percent }%

## Settings > Experimental > Look & Feel

look-intro = 仍在試驗中的功能，日後可能會變更或移除。
look-heading = 外觀與風格
look-window-frame = 視窗框架
look-window-frame-detail = 由誰繪製標題列、視窗按鈕、圓角和陰影。
look-frame-native-kde = 原生：KDE 的框架，採用你的 Plasma 主題
look-frame-native = 原生：桌面環境的框架
look-frame-katna = Katna：頂端列成為標題列
look-frame-katna-note-named = Katna 會繪製圓角和自己的陰影。框架將不再跟隨 { $desktop } 主題；視窗規則仍然適用。
look-frame-katna-note = Katna 會繪製圓角和自己的陰影。框架將不再跟隨桌面主題；視窗規則仍然適用。
look-frame-client-side = 你的桌面環境讓每個應用程式自行繪製框架，因此 Katna 已經在繪製自己的框架。
look-blurred-background = 模糊背景
look-blurred-background-detail = 透過頂端列和資料夾看見模糊的桌面，選單和彈出視窗則呈現毛玻璃效果。
look-blur = 模糊視窗後方的內容
look-blur-detail = 郵件仍顯示在不透明的資訊卡上，讓文字保持清晰的對比
look-blur-off-kde = KDE 的模糊效果已關閉。請在「系統設定」>「視窗管理」>「桌面效果」中開啟「模糊」，然後重新開啟 Katna Mail。
look-blur-none-gnome = GNOME 不會模糊視窗後方的內容。
look-blur-none-x11 = 你的視窗管理員不會模糊視窗後方的內容。
look-blur-none-wayland = 你的合成器不會模糊視窗後方的內容。

## Settings > User feedback (crash reports)

feedback-intro-sending = 新的當機報告會傳送出去，以協助修正問題。其他資料都不會離開這台電腦。
feedback-intro-local = Katna 不會傳送任何資料。當機報告會留在這台電腦上，供你查看或附加到錯誤報告中。
feedback-crash-reports = 當機報告
feedback-crash-reports-detail = 在 Katna Mail 或其背景服務當機時產生。
feedback-save = 將當機報告儲存在這台電腦上
feedback-save-detail = 會排除你的主資料夾、使用者名稱、電腦名稱和電子郵件地址
feedback-saved = 已儲存的當機報告
feedback-saved-detail = 保留最新的 { $count } 份報告。
feedback-help-improve = 協助改善 Katna
feedback-help-improve-detail = 除非你開啟，否則預設為關閉，而且你隨時可以在這裡關閉。
feedback-send = 傳送當機報告
feedback-send-detail = 已儲存的報告會完全依照你在這裡看到的內容，傳送至 Katna 的當機追蹤系統（Sentry，位於歐盟）。不含 IP 位址、郵件或電子郵件地址
feedback-none-saved = 沒有已儲存的當機報告。
feedback-delete-all = 全部刪除
feedback-app-daemon = 背景服務
feedback-report-sent = { $date } · 已傳送
feedback-view = 查看
feedback-view-tooltip = 開啟報告
feedback-copy-tooltip = 複製後可貼到錯誤報告中
feedback-copied = 已複製當機報告。
feedback-deleted-all = 已刪除當機報告。
feedback-read-failed = 無法讀取當機報告：{ $error }
feedback-delete-failed = 無法刪除當機報告：{ $error }
feedback-delete-all-failed = 無法刪除當機報告：{ $error }

## Menu bar (the KDE global menu)

desktop-menu-file = 檔案(_F)
desktop-menu-new-message = 新郵件(_N)
desktop-menu-quit = 結束(_Q)
desktop-menu-edit = 編輯(_E)
desktop-menu-undo = 復原(_U)
desktop-menu-select-all = 全選(_A)
desktop-menu-select-none = 全部不選(_N)
desktop-menu-find = 尋找(_F)…
desktop-menu-view = 檢視(_V)
desktop-menu-folder-list = 顯示資料夾清單(_F)
desktop-menu-refresh = 重新整理(_R)
desktop-menu-go = 前往(_G)
desktop-menu-inbox = 收件匣(_I)
desktop-menu-starred = 已加星號(_S)
desktop-menu-sent = 寄件備份(_E)
desktop-menu-drafts = 草稿(_D)
desktop-menu-all-mail = 所有郵件(_A)
desktop-menu-next = 下一個會話群組(_N)
desktop-menu-previous = 上一個會話群組(_P)
desktop-menu-message = 郵件(_M)
desktop-menu-open = 開啟(_O)
desktop-menu-reply = 回覆(_R)
desktop-menu-reply-all = 全部回覆(_A)
desktop-menu-forward = 轉寄(_F)
desktop-menu-archive = 封存(_H)
desktop-menu-delete = 刪除(_D)
desktop-menu-spam = 檢舉垃圾郵件(_S)
desktop-menu-move-to = 移至(_M)…
desktop-menu-mark-read = 標示為已讀取(_E)
desktop-menu-mark-unread = 標示為未讀取(_U)
desktop-menu-star = 加上星號(_T)
desktop-menu-important = 標示為重要(_P)
desktop-menu-not-important = 標示為不重要(_N)
desktop-menu-settings = 設定(_S)
desktop-menu-quick-settings = 快速設定(_Q)
desktop-menu-configure = 設定 Katna Mail(_C)…
desktop-menu-help = 說明(_H)
desktop-menu-shortcuts = 鍵盤快速鍵(_K)
desktop-menu-whats-new = 最新消息(_W)
desktop-menu-about = 關於 Katna(_A)

## Settings > Keyboard shortcuts: the groups of the list

shortcut-group-moving = 移動
shortcut-group-actions = 動作
shortcut-group-go-to = 前往
shortcut-group-app = 應用程式

## Settings > Keyboard shortcuts: what each shortcut does

shortcut-next = 下一個會話群組
shortcut-previous = 上一個會話群組
shortcut-down = 在清單中下移
shortcut-up = 在清單中上移
shortcut-first = 清單中的第一個
shortcut-last = 清單中的最後一個
shortcut-page-down = 清單向下翻頁
shortcut-page-up = 清單向上翻頁
shortcut-open = 開啟會話群組
shortcut-back = 返回清單
shortcut-scroll-down = 向下捲動
shortcut-scroll-up = 向上捲動
shortcut-scroll-page-down = 向下捲動一頁
shortcut-scroll-page-up = 向上捲動一頁
shortcut-compose = 撰寫
shortcut-reply = 回覆
shortcut-reply-all = 全部回覆
shortcut-forward = 轉寄
shortcut-archive = 封存
shortcut-delete = 刪除
shortcut-spam = 檢舉垃圾郵件
shortcut-move-to = 移至
shortcut-mark-read = 標示為已讀取
shortcut-mark-unread = 標示為未讀取
shortcut-star = 加上或移除星號
shortcut-important = 標示為重要
shortcut-not-important = 標示為不重要
shortcut-check = 勾選會話群組
shortcut-select-all = 勾選所有會話群組
shortcut-select-none = 取消勾選所有會話群組
shortcut-undo = 復原上一個動作
shortcut-go-inbox = 收件匣
shortcut-go-starred = 已加星號
shortcut-go-sent = 寄件備份
shortcut-go-drafts = 草稿
shortcut-go-all = 所有郵件
shortcut-search = 搜尋郵件
shortcut-navigation = 顯示或收合選單
shortcut-quick-settings = 快速設定
shortcut-settings = 所有設定
shortcut-shortcuts = 鍵盤快速鍵
shortcut-reload = 檢查新郵件
shortcut-quit = 結束

## Keys pressed one after another, as a shortcut shows them ("G then I")

shortcut-sequence = { $first }，然後 { $second }

## Settings > Accounts

accounts-folder-pane = 資料夾窗格
accounts-folder-pane-detail = 左側窗格要顯示哪些帳戶的資料夾。
accounts-shown-one = 一次一個帳戶；在帳戶卡片中切換
accounts-shown-all = 所有帳戶，依序排列
accounts-row = 帳戶
accounts-row-detail = 移除帳戶會刪除 Katna 在這台電腦上保存的郵件副本。伺服器上的郵件仍會保留。
accounts-none = 尚未新增任何帳戶。
accounts-kind-imported = 已匯入
accounts-picture-reset = 使用桌面圖片
accounts-picture-change = 變更圖片
accounts-remove = 移除
accounts-delete-all-row = 刪除所有資料
accounts-delete-all-row-detail = 重新開始，就像全新安裝一樣。
accounts-delete-all-about = 從這台電腦刪除所有帳戶、所有儲存的郵件、聯絡人和日曆、搜尋索引、你的設定和已儲存的密碼。郵件伺服器上的內容不會有任何變更。
accounts-delete-all-open = 刪除所有 Katna 資料

## Settings > Accounts: snackbars after deleting

accounts-removed-local = 已從 Katna 移除 { $address }。
accounts-removed = 已從 Katna 移除 { $address }。其郵件仍保留在伺服器上。
accounts-all-deleted = 已從這台電腦刪除所有 Katna 資料。

## Settings > Accounts: the dialog that asks before deleting

accounts-remove-title = 要移除 { $address } 嗎？
accounts-remove-confirm = 移除帳戶
accounts-removing = 正在移除…
accounts-remove-local-mail = { $folders ->
    [0] 匯入這個帳戶的所有郵件
   *[other] 匯入這個帳戶的所有郵件，位於其 { $folders } 個資料夾中
}
accounts-remove-local-settings = 其 Katna 設定
accounts-remove-mail = { $folders ->
    [0] Katna 儲存的這個帳戶所有郵件
   *[other] Katna 儲存的這個帳戶所有郵件，位於其 { $folders } 個資料夾中
}
accounts-remove-outbox = 其在寄件匣中等待寄出的郵件
accounts-remove-settings = 其已儲存的密碼和 Katna 設定
accounts-delete-all-title = 要刪除所有 Katna 資料嗎？
accounts-delete-all-confirm = 全部刪除
accounts-deleting = 正在刪除…
accounts-delete-all-accounts = 所有帳戶，以及 Katna 儲存的所有郵件和附件
accounts-delete-all-contacts = 聯絡人、日曆和搜尋索引
accounts-delete-all-settings = 所有設定、簽名和鍵盤快速鍵
accounts-delete-all-passwords = 所有已儲存的密碼
accounts-deleted-heading = 將從這台電腦刪除：
accounts-cannot-undo = 這項操作無法復原。
accounts-server-delete-all = 郵件伺服器上的內容不會有任何變更：你的郵件仍會保留在伺服器上，再次新增帳戶時會重新下載。從檔案匯入的郵件只存在於 Katna 中；原始檔案不會受到影響。
accounts-server-local = 這些郵件是從檔案匯入的，因此 Katna 保有唯一的副本。原始檔案不會受到影響；重新匯入即可取回郵件。
accounts-server-remove = 郵件伺服器上的內容不會有任何變更：你的郵件仍會保留在伺服器上，再次新增這個帳戶時會重新下載。
accounts-confirm-word = 刪除
accounts-confirm-placeholder = 輸入「{ accounts-confirm-word }」
accounts-confirm-prompt = 如要確認，請輸入「{ accounts-confirm-word }」：
accounts-cancel = 取消
