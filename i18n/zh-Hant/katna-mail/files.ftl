# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Top bar

files-search = 搜尋檔案

## Left side (and chips on a phone)

files-all = 所有檔案
files-pictures = 圖片
files-pdfs = PDF
files-documents = 文件
files-sheets = 試算表
files-slides = 簡報
files-other = 其他
files-accounts = 帳戶
files-drives = 雲端硬碟
files-drive-google = Google Drive
files-drive-onedrive = OneDrive
files-drive-shared = 與我共用
files-shown = 顯示
files-received = 已收到
files-sent = 我寄出的

## Over the files

files-count = { $count ->
   *[other] { $count } 個檔案 · { $size }
}
files-anyone = 任何人
files-from-person = 來自 { $name }
files-time-any = 不限時間
files-time-today = 今天
files-time-yesterday = 昨天
files-time-this-week = 本週
files-time-last-week = 上週
files-time-this-month = 本月
files-time-last-month = 上個月
files-time-between = { $first } – { $last }
files-time-hint = 按一下某一天，或拖曳選取多天
files-time-summary = { $count ->
   *[other] { $days } · { $count } 個檔案
}
files-time-clear = 清除
files-time-month-back = 上個月
files-time-month-on = 下個月
files-time-wheel = 捲動即可移動這些日期，並保持相同的天數
files-sort-newest = 最新的在前
files-sort-oldest = 最舊的在前
files-sort-largest = 最大的在前
files-sort-name = 依名稱
files-grid = 資訊卡
files-list = 清單
files-this-week = 本週
files-undated = 無日期
files-me = 我
files-no-subject = （無主旨）
files-loading = 正在從你的郵件中收集檔案…
files-empty = 郵件中的檔案會顯示在這裡。
files-none-match = 沒有符合的檔案。
files-load-failed = 無法讀取檔案：{ $error }

## A file's menu and buttons

files-open = 開啟
files-open-with = 開啟方式…
files-save = 儲存…
files-show-mail = 顯示郵件
files-mail-window = 在新視窗中開啟郵件
files-forward = 轉寄檔案
files-from-them = 來自 { $name } 的檔案
files-copy-name = 複製檔案名稱
files-name-copied = 已複製檔案名稱
files-downloading = 正在下載郵件…
files-download-failed = 無法下載這封郵件。

## A cloud drive in place of the mail files

files-drive-mine = 我的雲端硬碟
files-drive-mine-onedrive = 我的檔案
files-drive-results = 「{ $words }」
files-drive-count = { $folders ->
    [0] { $files } 個檔案
   *[other] { $folders } 個資料夾 · { $files } 個檔案
}
files-drive-folders = 資料夾
files-drive-files = 檔案
files-drive-folder = 資料夾
files-drive-meta = { $what } · 編輯於 { $date }
files-drive-as-link = { $what } · 以連結形式
files-drive-google-doc = Google 文件
files-drive-google-sheet = Google 試算表
files-drive-google-slides = Google 簡報
files-drive-google-drawing = Google 繪圖
files-drive-fetching = 正在取得…
files-drive-loading = 正在開啟雲端硬碟…
files-drive-empty = 這個資料夾是空的。
files-drive-unreachable = 無法連線到 { $drive }。
files-drive-try-again = 再試一次
files-drive-needs-permission = Katna 需要你授權一次才能顯示這個雲端硬碟。請重新登入，並允許 Katna 查看你的檔案。
files-drive-allow = 允許
files-drive-allow-failed = 登入未完成，因此雲端硬碟仍保持關閉。
files-drive-attach = 附加
files-drive-more = 更多
files-drive-download = 下載…
files-drive-open-web = 在 { $drive } 中開啟
files-drive-copy-link = 複製連結
files-drive-link-copied = 已複製連結
files-drive-share = 共用…
files-drive-rename = 重新命名
files-drive-trash = 移至垃圾桶
files-drive-trashed = 「{ $name }」已移至 { $drive } 垃圾桶
files-drive-renamed = 已重新命名為「{ $name }」
files-drive-getting = 正在從 { $drive } 取得 { $name }…
files-drive-get-failed = 無法取得 { $name }：{ $error }
files-drive-upload = 上傳
files-drive-upload-files = 上傳檔案
files-drive-upload-folder = 上傳資料夾
files-drive-upload-failed = 無法上傳 { $name }：{ $error }
files-drive-upload-needs = Katna 需要你授權一次才能上傳：請在「設定」>「預設應用程式」>「檔案頁面」中按「允許」。

## The Share dialog of a drive file or folder

files-share-title = 共用「{ $name }」
files-share-add = 依姓名或地址新增使用者
files-share-not-address = 「{ $text }」不是電子郵件地址
files-share-notify = 也讓 { $drive } 寄送電子郵件通知對方
files-share-people = 具有存取權的使用者
files-share-general = 一般存取權
files-share-loading = 正在讀取具有存取權的使用者…
files-share-restricted = 限制
files-share-restricted-about = 只有具備存取權的使用者才能透過連結開啟
files-share-anyone = 知道連結的任何人
files-share-anyone-can = { $role ->
    [editor] 知道連結的任何人都能編輯
    [commenter] 知道連結的任何人都能留言
   *[viewer] 知道連結的任何人都能檢視
}
files-share-anyone-about = { $role ->
    [editor] 網際網路上任何知道連結的人都能編輯
    [commenter] 網際網路上任何知道連結的人都能留言
   *[viewer] 網際網路上任何知道連結的人都能檢視
}
files-share-role-owner = 擁有者
files-share-role-editor = 編輯者
files-share-role-commenter = 加註者
files-share-role-viewer = 檢視者
files-share-you = { $name }（你）
files-share-domain = { $domain } 的所有人
files-share-inherited = 繼承自所在資料夾的存取權
files-share-remove = 移除存取權
files-share-copy-link = 複製連結
files-share-share = 共用
files-share-done = 完成
files-share-sharing = 正在共用…
files-share-shared = { $count ->
   *[other] 已與 { $count } 人共用
}
files-share-refused = { $drive } 無法與 { $addresses } 共用
files-share-failed = 無法變更共用設定：{ $error }

## The uploads tray, at the bottom right while files go up to a drive

files-tray-uploading = { $count ->
   *[other] 正在上傳 { $count } 個項目
}
files-tray-done = { $count ->
   *[other] 已完成 { $count } 項上傳
}
files-tray-some-failed = 已上傳 { $done } 個，{ $failed } 個失敗
files-tray-minutes-left = { $minutes ->
   *[other] 大約還剩 { $minutes } 分鐘
}
files-tray-seconds-left = 剩不到一分鐘
files-tray-starting = 正在開始…
files-tray-cancel-all = 全部取消
files-tray-cancel = 取消
files-tray-fold = 隱藏清單
files-tray-unfold = 顯示清單
files-tray-close = 關閉
files-tray-progress = { $place } · 已上傳 { $size } 中的 { $sent }
files-tray-in = 位於 { $place }
files-tray-cancelled = 已取消
