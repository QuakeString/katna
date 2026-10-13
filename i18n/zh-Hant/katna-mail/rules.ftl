# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Settings > Folders & rules

settings-rules = 規則
settings-rules-summary = 自動為新郵件分類、加上標籤、轉寄或設為靜音
settings-rules-intro = 規則會依此順序自動整理新郵件。拖曳即可重新排序。
settings-rules-all-accounts = 所有帳戶
settings-rules-new = 新增規則
settings-rules-none = 尚未建立規則。規則會依寄件者、主旨或字詞自動整理新郵件。
settings-rules-none-account = 此帳戶尚未建立規則。
settings-rules-drag = 拖曳以重新排序
settings-rules-edit = 編輯規則
settings-rules-turn-off = 關閉此規則
settings-rules-turn-on = 開啟此規則

## Starter rules: offered under the user's own rules, switched off.
## Turning one on makes it one of the user's rules.

settings-rules-starters = 入門規則
settings-rules-starters-intro = 開啟前不會生效。這些規則適用於你所有的帳戶；如需變更，請編輯規則。
settings-rules-starter-turning-on = 正在開啟「{ $name }」…
settings-rules-starter-failed = 無法開啟「{ $name }」：{ $error }
rules-starter-promotions = 促銷郵件不通知
rules-starter-newsletters = 電子報移至「閱讀」
rules-starter-receipts = 收據和發票
rules-starter-deliveries = 包裹配送
rules-starter-train = 火車票
rules-starter-flight = 機票
rules-starter-codes = 一次性驗證碼
rules-starter-security = 安全性警示
rules-starter-social = 社交郵件
rules-starter-invites = 日曆邀請
rules-starter-folder-reading = 閱讀
rules-starter-folder-receipts = 收據
rules-starter-folder-deliveries = 包裹配送
rules-starter-folder-travel = 旅行
rules-starter-folder-social = 社交
rules-runs-katna = 在 Katna 中執行
rules-runs-gmail = 在 Gmail 上執行
rules-runs-sieve = 在伺服器上執行
rules-stopped = 已停止
rules-error-folder-gone = 此規則使用的資料夾已不存在。請編輯規則以選擇其他資料夾。
rules-error-no-archive = 此帳戶沒有封存資料夾。請編輯規則以執行其他動作。
rules-error-no-trash = 此帳戶沒有垃圾桶資料夾。請編輯規則以執行其他動作。
rules-error-cannot-send = 此帳戶無法傳送郵件，因此規則無法轉寄郵件。
rules-error-other = { $error }。請編輯規則並重新開啟。
settings-folders = 資料夾
settings-folders-summary = 資料夾窗格中的未讀取郵件數
settings-folders-unread-counts = 每個資料夾都顯示未讀取郵件數
settings-folders-unread-counts-detail = 關閉時，只有收件匣會顯示未讀取郵件數

## A rule in one line, on its row: "From contains substack.com → skip the
## inbox, label Reading".

rules-summary = { $when } → { $then }
rules-summary-and = { $first }且{ $next }
rules-summary-or = { $first }或{ $next }
rules-summary-more = 其他 { $count } 個
rules-summary-list = { $first }、{ $next }
rules-summary-condition = { $field }{ $comparator }{ $value }
rules-summary-has-attachment = 有附件
rules-summary-no-attachment = 沒有附件
rules-summary-mailing-list = 來自郵寄清單
rules-summary-not-mailing-list = 不是來自郵寄清單
rules-summary-tab = 在「{ $tab }」分頁中
rules-summary-not-tab = 不在「{ $tab }」分頁中
rules-summary-move = 移至 { $folder }
rules-summary-archive = 略過收件匣
rules-summary-trash = 移至垃圾桶
rules-summary-mark-read = 標示為已讀取
rules-summary-star = 加上星號
rules-summary-important = 標示為重要
rules-summary-label = 加上標籤 { $label }
rules-summary-forward = 轉寄給 { $address }
rules-summary-dont-notify = 不通知
rules-summary-read-after = { $count ->
   *[other] { $count } 天後標示為已讀取
}
rules-summary-folder-gone = 已不存在的資料夾

## The rule editor

rules-editor-new-title = 新增規則
rules-editor-edit-title = 編輯規則
rules-editor-name-hint = 規則名稱
rules-editor-when = 當新郵件符合以下
rules-editor-of-these = 條件時：
rules-mode-all = 所有
rules-mode-any = 任一
rules-field-from = 寄件者
rules-field-to = 收件者
rules-field-cc = 副本
rules-field-any-recipient = 收件者或副本
rules-field-reply-to = 回覆地址
rules-field-subject = 主旨
rules-field-body = 內文
rules-field-attachment-name = 附件名稱
rules-field-has-attachment = 有附件
rules-field-mailing-list = 來自郵寄清單
rules-field-tab = 收件匣分頁
rules-comparator-contains = 包含
rules-comparator-not-contains = 不包含
rules-comparator-begins-with = 開頭為
rules-comparator-ends-with = 結尾為
rules-comparator-equals = 完全符合
rules-comparator-matches = 符合模式
rules-has-yes = 是
rules-has-no = 否
rules-editor-value-hint = 字詞或地址
rules-editor-add-condition = 新增條件
rules-editor-remove = 移除
rules-editor-then = 接著：
rules-action-move = 移至
rules-action-archive = 略過收件匣（封存）
rules-action-trash = 移至垃圾桶
rules-action-mark-read = 標示為已讀取
rules-action-star = 加上星號
rules-action-important = 標示為重要
rules-action-label = 加上標籤
rules-action-forward = 轉寄給
rules-action-dont-notify = 不通知
rules-action-read-after = 標示為已讀取的天數
rules-editor-choose-folder = 選擇資料夾
rules-editor-choose-label = 選擇標籤
rules-editor-new-folder = 新增：{ $name }
rules-editor-folder-of = { $folder }（{ $account }）
rules-editor-forward-hint = 電子郵件地址
rules-editor-days = 天
rules-editor-add-action = 新增動作
rules-editor-stop = 到此為止：後面的規則不會套用到這封郵件
rules-editor-accounts = 帳戶：
rules-editor-accounts-none = 選擇帳戶
rules-editor-accounts-many = { $count ->
   *[other] { $count } 個帳戶
}
rules-editor-matches = 符合過去 { $days } 天內的 { $mails }
rules-editor-mails = { $count ->
   *[other] { $count } 封郵件
}
rules-editor-counting = 正在計算符合的郵件…
rules-editor-show = 顯示這些郵件
rules-editor-also-apply = 也套用到這 { $count } 封郵件
rules-editor-runs-katna = 在 Katna 中執行，僅在這部電腦開機時運作。
rules-editor-runs-gmail = 在 Gmail 上執行，因此在你的手機上以及這部電腦關機時也能運作。
rules-editor-runs-sieve = 在你的郵件伺服器上執行，因此在你的手機上以及這部電腦關機時也能運作。
rules-note-gmail-action = 在 Katna 中執行：Gmail 篩選器無法執行「{ $action }」。
rules-note-sieve-action = 在 Katna 中執行：你的郵件伺服器規則無法執行「{ $action }」。
rules-note-test = { $field }{ $comparator }
rules-note-gmail-condition = 在 Katna 中執行：Gmail 篩選器無法像 Katna 一樣檢查「{ $test }」。
rules-note-sieve-condition = 在 Katna 中執行：你的郵件伺服器規則無法像 Katna 一樣檢查「{ $test }」。
rules-note-order = 在 Katna 中執行，因為此帳戶較前面的規則也在 Katna 中執行：規則會依清單順序執行。
rules-note-gmail-stop = 在 Katna 中執行：Gmail 篩選器無法阻止後面的規則執行。
rules-note-gmail-forward = 在 Katna 中執行：Gmail 只會轉寄到已在其設定中驗證的地址，而 { $address } 不在其中。
rules-note-gmail-folder = 在 Katna 中執行：Gmail 沒有此規則所用資料夾對應的標籤。
rules-note-sieve-folder = 在 Katna 中執行：你的郵件伺服器上沒有此規則所用的資料夾。
rules-note-gmail-sign-in = 在你重新登入 Google 並允許 Katna 建立 Gmail 篩選器之前，會在 Katna 中執行。
rules-note-sieve-other-script = 在 Katna 中執行：你的郵件伺服器上已啟用另一個規則指令碼（「{ $name }」）。
rules-note-gmail-failed = 在 Katna 中執行：Gmail 未接受此規則（{ $error }）。
rules-note-sieve-failed = 在 Katna 中執行：你的郵件伺服器未接受此規則（{ $error }）。
rules-editor-cancel = 取消
rules-editor-save = 儲存
rules-editor-saving = 正在儲存…
rules-editor-delete = 刪除規則
rules-editor-delete-ask = 要刪除這條規則嗎？
rules-editor-delete-keep = 保留
rules-editor-delete-confirm = 刪除
rules-editor-needs-folder = 請為每個「移至」選擇資料夾，並為每個「加上標籤」選擇標籤。
rules-editor-needs-days = 「標示為已讀取的天數」需要介於 1 到 3650 之間的天數。
rules-saved = 規則已儲存
rules-saved-applied = { $count ->
   *[other] 規則已儲存，並已套用到 { $count } 封郵件
}
rules-apply-failed = 規則已儲存，但套用失敗：{ $error }
rules-deleted = 已刪除規則
rules-delete-failed = 無法刪除規則：{ $error }
rules-change-failed = 無法變更規則：{ $error }
