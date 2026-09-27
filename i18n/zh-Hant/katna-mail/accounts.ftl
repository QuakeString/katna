# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

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
