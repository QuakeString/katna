# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Navigation (the folders pane)

nav-labels = 標籤
nav-folders = 資料夾
nav-label-new = 建立新標籤
nav-folder-new = 建立新資料夾
nav-menu-check-mail = 檢查新郵件
nav-menu-check-inbox = 檢查此收件匣
nav-unified-leave-out = 從整合收件匣中排除
nav-unified-bring-back = 重新加入整合收件匣
nav-menu-sign-in-again = 重新登入
nav-menu-new-mail = 從此帳戶寄送新郵件
nav-menu-account-settings = 帳戶設定
nav-account-checked = 已同步 · { $ago }檢查
nav-account-in-sync = 已同步
nav-account-connecting = 正在連線…
nav-account-offline = 離線，正在重試
nav-account-signed-out = { $provider } 登入已過期
nav-account-password-refused = 密碼遭拒
nav-account-storage = 已使用 { $total } 中的 { $used }
nav-menu-new-subfolder = 在其中建立新資料夾
nav-menu-new-sublabel = 在其中建立新標籤
nav-menu-rename = 重新命名
nav-menu-delete = 刪除
nav-menu-empty-trash = 清空垃圾桶
nav-account-unnamed = 帳戶 { $number }
nav-all-accounts = 所有帳戶
nav-expand = 顯示資料夾
nav-collapse = 隱藏資料夾
storage-used = 已使用 { $total } 中的 { $percent }%
storage-used-detail = { $address }：已使用 { $total } 中的 { $used }

## Special folders (the user's own folders keep their names)

folder-inbox = 收件匣
folder-starred = 已加星號
folder-snoozed = 已延後
folder-unread = 未讀取
folder-important = 重要
folder-drafts = 草稿
folder-sent = 寄件備份
folder-archive = 封存
folder-spam = 垃圾郵件
folder-trash = 垃圾桶
folder-all-mail = 所有郵件
folder-scheduled = 已排定
folder-waiting = 等候回覆
folder-waiting-short = 等候中
folder-reminders = 提醒
folder-outbox = 寄件匣
folder-activity = 動態

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
label-rename-title = 重新命名標籤
label-folder-rename-title = 重新命名資料夾
label-rename = 重新命名
label-renaming = 正在重新命名…
label-renamed = 標籤已重新命名為「{ $name }」。
label-folder-renamed = 資料夾已重新命名為「{ $name }」。

## Deleting a folder or label (asked first)

folder-delete-title = 要刪除「{ $name }」嗎？
folder-delete-body = { $count ->
    [0] 其中沒有郵件。資料夾會從伺服器上移除，因此網頁版郵件和你的手機上也會消失。
   *[other] { $kind ->
        [conversation] 其中的 { $count } 個會話群組會移至垃圾桶，你仍可以救回。
       *[message] 其中的 { $count } 封郵件會移至垃圾桶，你仍可以救回。
    }資料夾會從伺服器上移除，因此網頁版郵件和你的手機上也會消失。
}
folder-delete-forever-body = { $count ->
    [0] 其中沒有郵件。資料夾會從伺服器上移除，因此網頁版郵件和你的手機上也會消失。
   *[other] { $kind ->
        [conversation] 其中的 { $count } 個會話群組會被永久刪除，因為此帳戶沒有垃圾桶。
       *[message] 其中的 { $count } 封郵件會被永久刪除，因為此帳戶沒有垃圾桶。
    }資料夾會從伺服器上移除，因此網頁版郵件和你的手機上也會消失。
}
folder-delete-label-body = 標籤會被移除。其郵件仍保留在「所有郵件」和其他標籤中。
folder-delete-confirm = 刪除資料夾
folder-delete-label-confirm = 刪除標籤
folder-deleted = 已刪除資料夾「{ $name }」
label-deleted = 已刪除標籤「{ $name }」
