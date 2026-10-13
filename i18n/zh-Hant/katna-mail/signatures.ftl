# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The signature being edited

signature-placeholder = 你的姓名，以及想加在下方的任何內容

## Its formatting bar

signature-bold = 粗體
signature-italic = 斜體
signature-underline = 底線
signature-link = 連結
signature-link-apply = 套用
signature-picture = 插入圖片
signature-align-left = 靠左對齊
signature-align-center = 置中
signature-align-right = 靠右對齊
signature-numbered-list = 編號清單
signature-bulleted-list = 項目符號清單
signature-remove-formatting = 清除格式設定

## Adding a picture

signature-picture-choose = 插入
signature-picture-too-big = 簽名中的圖片最大可為 { $size }。
signature-picture-kind = 請選擇 PNG、JPEG、GIF 或 WebP 圖片。
signature-picture-unreadable = { $name }：{ $error }

## Layouts

signature-layout = 版面配置
signature-layout-own = 自訂
signature-layout-classic = 經典
signature-layout-logo-left = 標誌靠左
signature-layout-photo = 相片
signature-layout-band = 色帶
signature-layout-one-line = 單行
signature-layout-centred = 置中
signature-layout-banner = 附橫幅
signature-layout-underline = 底線
signature-layout-side-bar = 側邊條
signature-layout-card = 名片
signature-layout-monogram = 字母標誌
signature-layout-plain = 純文字
signature-layout-mobile-label = M:
signature-layout-office-label = O:
signature-layout-email-label = E:
signature-layout-name = 姓名
signature-layout-job = 職稱
signature-layout-company = 公司
signature-layout-mobile = 手機
signature-layout-office = 辦公室電話
signature-layout-email = 電子郵件
signature-layout-website = 網站
signature-layout-address = 地址
signature-layout-pictures = 圖片
signature-layout-logo = 標誌
signature-layout-photo-picture = 相片
signature-layout-banner-picture = 橫幅
signature-layout-remove-picture = 移除
signature-layout-pages = 頁面
signature-layout-page-placeholder = 新增頁面網址
signature-layout-colour = 顏色
signature-layout-picture-failed = 無法將 { $name } 當作圖片使用。
signature-layout-preview = 讀者看到的樣子
signature-layout-light = 淺色
signature-layout-dark = 深色
signature-layout-text = 純文字
signature-layout-inside = 圖片會內嵌在郵件中，因此即使對方關閉了網路圖片也能顯示。這張圖片會讓每封郵件增加 { $size }。
signature-layout-edit = 手動編輯
signature-layout-edit-confirm = 要手動編輯嗎？欄位和版面配置會移除，但會在編輯器能呈現的範圍內保留原本的外觀。
signature-layout-use-confirm = 要使用「{ $layout }」版面配置嗎？它會取代這個簽名，並以此簽名的內容填入。
signature-layout-use = 使用版面配置
signature-layout-cancel = 取消

## Paste HTML

signature-html-title = 貼上 HTML
signature-html-subtitle = 適用於你在其他地方設計的簽名
signature-html-placeholder = 在此貼上簽名的 HTML
signature-html-name = 貼上的簽名
signature-html-new = 儲存為新簽名「{ $name }」
signature-html-replaces = 覆寫「{ $name }」
signature-html-cancel = 取消
signature-html-save = 儲存
signature-html-fetching = 正在下載其中的圖片…
signature-html-pictures-inside = { $count ->
   *[other] 已下載 { $count } 張圖片並內嵌在郵件中（{ $size }）
}
signature-html-pictures-web = { $count ->
   *[other] 有 { $count } 張圖片無法下載，因此讀者會從網路載入
}
signature-html-removed = 已移除指令碼、表單和追蹤像素，郵件應用程式本來就會封鎖這些內容
signature-html-style-sheet = 已略過樣式表：郵件只會保留寫在各個部分上的樣式
signature-html-links = 已移除指向網站、電子郵件地址或電話以外位置的連結
signature-html-plain-text = 已從中產生純文字版本，供只顯示文字的郵件應用程式使用

## Import

signature-import-title = 匯入
signature-import-subtitle = 從 Gmail、Thunderbird、Evolution 和 KMail
signature-import-looking = 正在尋找簽名…
signature-import-none = 找不到任何簽名。若是其他應用程式，請複製其簽名的 HTML，然後使用「貼上 HTML」。
signature-import-from = 來自 { $app }
signature-import-already = 已在 Katna 中
signature-import-gmail-sign-in = { $address }：請在「設定 > 帳戶」中重新登入，讓 Katna 可以讀取 Gmail 的簽名。
signature-import-gmail-failed = { $address }：{ $error }
signature-import-cancel = 取消
signature-import-do = { $count ->
   *[other] 匯入 { $count } 個簽名
}
signature-import-name = { $name }（{ $app }）
