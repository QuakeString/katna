# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Accounts

accounts-folder-pane = 資料夾窗格
accounts-folder-pane-detail = 左側窗格要顯示哪些帳戶的資料夾。
accounts-shown-one = 一次一個帳戶；在帳戶卡片中切換
accounts-shown-all = 所有帳戶，依序排列
accounts-unified = 整合收件匣
accounts-unified-switch = 一併顯示所有帳戶的郵件
accounts-unified-switch-detail = 「所有帳戶」位於資料夾窗格頂端，將每個帳戶的收件匣、寄件備份等列在同一個清單中。其下方的帳戶一開始為收合狀態。
accounts-row = 帳戶
accounts-row-detail = 資料夾窗格和帳戶選單會依此順序列出帳戶；第一個為預設帳戶。移除帳戶會刪除 Katna 在這台電腦上保存的郵件副本。伺服器上的郵件仍會保留。
accounts-none = 尚未新增任何帳戶。
accounts-kind-imported = 已匯入
accounts-picture-reset = 使用桌面圖片
accounts-picture-change = 變更圖片
accounts-picture-remove = 移除圖片
accounts-rename = 重新命名
accounts-name-save = 儲存
accounts-name-cancel = 取消
accounts-name-placeholder = 你的姓名
accounts-rename-failed = 無法重新命名帳戶：{ $error }
accounts-move-up = 上移
accounts-move-down = 下移
accounts-drag = 拖曳以變更順序
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
reset-cache-about = 刪除 Katna 下載的郵件和附件、寄件者圖片和搜尋索引，然後重新下載近期郵件。帳戶、設定以及只存在於這台電腦上的郵件會保留。
reset-cache-button = 重設快取
reset-cache-title = 要重設快取嗎？
reset-cache-deleted = 刪除後重新下載：
reset-cache-mail = 從你的 IMAP 伺服器下載的郵件和附件：近期郵件會立即重新下載，較舊的郵件則在你開啟時下載
reset-cache-index = 搜尋索引，會立即重建
reset-cache-pictures = 寄件者圖片
reset-cache-kept = 保留：你的帳戶、密碼和設定；星號、標籤、已讀取標示和置頂；草稿、寄件匣以及尚未同步到伺服器的變更；以及來自 POP3 帳戶或匯入檔案的郵件，這些郵件可能沒有其他副本。郵件伺服器上的內容不會有任何變更。
reset-cache-confirm = 重設快取
reset-cache-busy = 正在重設…
reset-cache-done = 快取已重設。正在重新下載近期郵件。
reset-cache-done-freed = 快取已重設，釋出了 { $size }。正在重新下載近期郵件。
