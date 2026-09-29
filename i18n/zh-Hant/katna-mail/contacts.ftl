# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = 聯絡人
contacts-frequent = 常用聯絡人
contacts-other = 其他聯絡人
contacts-other-about = 你寄過信但未儲存的 Gmail 聯絡人
contacts-other-email = 傳送電子郵件
contacts-other-empty = 沒有其他聯絡人。你透過 Gmail 寄過信但未儲存的人會顯示在這裡。
contacts-other-allow = 如要查看其他聯絡人，請重新登入你的 Gmail 帳戶，並允許 Katna 查看。
contacts-labels = 標籤
contacts-label-options = 標籤選項
contacts-label-rename = 重新命名標籤
contacts-label-email = 寄信給所有人
contacts-label-delete = 刪除標籤
contacts-label-new = 新增標籤
contacts-label-name = 標籤名稱
contacts-label-button = 標籤
contacts-label-menu = 標籤為：
contacts-label-added = 已新增至「{ $name }」
contacts-label-removed = 已從「{ $name }」移除
contacts-label-renamed = 標籤已重新命名為「{ $name }」
contacts-label-deleted = 已刪除標籤「{ $name }」
contacts-label-no-email = 此標籤下沒有人有電子郵件地址
# The column's list of mail accounts, each with the people saved in it.
contacts-accounts = 帳戶
# The line under an account whose contacts did not come: why, and the one
# click that fixes it.
contacts-account-sign-in = 重新登入以顯示聯絡人
contacts-account-signed-in = 已重新登入 { $address }。正在取得你的聯絡人…
contacts-account-sign-in-refused = { $provider } 未允許 Katna 存取。請再試一次，並允許存取你的聯絡人。
contacts-account-password = 伺服器未接受這組密碼。Yahoo、iCloud、Zoho 等需要應用程式密碼。
contacts-account-change-password = 變更密碼
contacts-account-change-password-tooltip = 開啟「設定 > 帳戶」
contacts-account-failed = 無法讀取聯絡人。
# $reason is the server's own words, in English.
contacts-account-error = 無法讀取聯絡人：{ $reason }
contacts-account-none = 找不到通訊錄
# $reason is what the server answered, in English: "CardDAV https://dav.example.in/: status 404".
contacts-account-none-why = 找不到通訊錄：{ $reason }
# A Gmail or Outlook account added with a password: its contacts need the
# provider's sign-in.
contacts-account-use-sign-in = { $provider } 只向使用 { $provider } 登入的 Katna 顯示聯絡人。
contacts-account-sign-in-with = 使用 { $provider } 登入
contacts-account-looking = 正在尋找聯絡人…
contacts-account-try-again = 再試一次
contacts-account-try-again-tooltip = 立即重新檢查此帳戶的聯絡人
contacts-account-fixing = 正在處理…
contacts-manage = 修正及管理
contacts-merge = 合併及修正
contacts-merge-about = { $count ->
   *[other] { $count } 項建議：看起來是同一個人的聯絡人
}
contacts-merge-none = 沒有重複項目。姓名或電話號碼相同的聯絡人會顯示在這裡。
contacts-merge-count = { $count ->
   *[other] { $count } 位聯絡人
}
contacts-merge-all = 全部合併
contacts-merge-button = 合併
contacts-merge-dismiss = 忽略
contacts-merged = { $count ->
    [1] 已合併聯絡人
   *[other] 已完成 { $count } 次合併
}
contacts-import = 匯入
contacts-export = 匯出
contacts-import-file = 從 vCard 或 CSV 檔案匯入聯絡人
contacts-imported = { $count ->
   *[other] 已將 { $count } 位聯絡人匯入「{ $place }」
}
contacts-imported-some = { $count ->
   *[other] 已將 { $count } 位聯絡人匯入「{ $place }」；已儲存的 { $skipped } 位已略過
}
contacts-import-none = 在 { $name } 中找不到聯絡人
contacts-import-all-saved = { $name } 中的所有人都已儲存
contacts-import-failed = 無法讀取 { $name }：{ $error }
contacts-exported = { $count ->
   *[other] 已將 { $count } 位聯絡人匯出至 { $path }
}
contacts-export-none = 沒有可匯出的聯絡人
contacts-export-failed = 無法匯出聯絡人：{ $error }
contacts-print = 列印
contacts-print-title = 聯絡人
contacts-print-none = 沒有可列印的聯絡人
contacts-print-typed = { $value }（{ $kind }）
contacts-print-birthday = 生日：{ $day }
contacts-print-nickname = 暱稱：{ $name }
contacts-create = 建立聯絡人

## Search and the list

contacts-search = 搜尋聯絡人
contacts-loading = 正在載入聯絡人…
contacts-empty = 尚無已儲存的聯絡人。您在 Gmail、Outlook 或郵件服務中儲存的聯絡人會顯示在這裡。
contacts-empty-no-books = 帳號中的聯絡人同步完成後會顯示在這裡。
contacts-none-found = 沒有與搜尋相符的聯絡人。
contacts-starred = { $count ->
   *[other] 已加星號的聯絡人 ({ $count })
}
contacts-count = 聯絡人 ({ $count })
contacts-col-name = 姓名
contacts-col-email = 電子郵件
contacts-col-phone = 電話號碼
contacts-col-job = 職稱和公司
contacts-col-labels = 標籤

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = 允許 Katna 讀取 { $address } 的聯絡人。
contacts-allow-many = { $more ->
   *[other] 允許 Katna 讀取 { $address } 及另外 { $more } 個帳號的聯絡人。
}
contacts-allow-button = 允許

## A contact's page

contacts-back = 返回聯絡人
contacts-edit = 編輯
contacts-delete = 刪除
contacts-qr = 以 QR 碼分享
contacts-qr-about = 用手機相機掃描即可儲存這位聯絡人。
contacts-qr-too-long = 這位聯絡人的詳細資料過多，無法放入 QR 碼。
contacts-qr-done = 完成
contacts-deleted = 已刪除 { $name }
contacts-added = 已將 { $name } 新增至聯絡人
contacts-find-mail = 郵件
contacts-details = 聯絡人詳細資料
contacts-saved-in = 儲存位置
contacts-notes = 記事
contacts-birthday = 生日
contacts-nickname = 暱稱
contacts-this-computer = 這部電腦
contacts-kind-home = 住家
contacts-kind-work = 公司
contacts-kind-mobile = 手機
contacts-kind-other = 其他
contacts-source-google = Google 聯絡人
contacts-source-microsoft = Outlook 聯絡人
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = 建立聯絡人
contacts-edit-title = 編輯聯絡人
contacts-edit-save = 儲存
contacts-edit-saving = 正在儲存…
contacts-edit-cancel = 取消
contacts-saved = 已儲存聯絡人
contacts-edit-save-to = 儲存至
contacts-edit-changes-go-to = 變更會儲存至 { $place }。
contacts-edit-given = 名字
contacts-edit-family = 姓氏
contacts-edit-company = 公司
contacts-edit-job = 職稱
contacts-edit-email = 電子郵件
contacts-edit-phone = 電話
contacts-edit-with-kind = { $field }（{ $kind }）
contacts-edit-add-email = 新增電子郵件
contacts-edit-add-phone = 新增電話
contacts-edit-street = 街道地址
contacts-edit-city = 城市
contacts-edit-postcode = 郵遞區號
contacts-edit-country = 國家/地區
contacts-edit-birthday = 生日（YYYY-MM-DD）
contacts-edit-empty = 請先新增姓名、電子郵件或電話號碼。
