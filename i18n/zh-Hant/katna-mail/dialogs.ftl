# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## About Katna (the dialog from the "i" button on the top bar)

about-tooltip = 關於 Katna
about-tagline = 適用於 Linux 桌面的郵件與日曆
about-whats-new = 最新消息

## Updates, in a box under the version in About (only in packages that
## update themselves). $version is a version such as 0.0.0.r236.g1a2b3c4.

about-update-not-checked = 尚未檢查更新
about-update-checking = 正在檢查更新…
about-update-up-to-date = Katna Mail 已是最新版本
about-update-check-failed = 無法檢查更新
about-update-available = 有可用版本 { $version }
about-update-downloading = 正在下載版本 { $version }… { $percent }%
about-update-download-failed = 版本 { $version } 的下載未完成
about-update-ready = 版本 { $version } 已可安裝
about-update-ready-detail = Katna Mail 將重新啟動以完成更新。
about-update-confirm = 要安裝版本 { $version } 嗎？
about-update-confirm-detail = Katna Mail 將會關閉、安裝更新，然後回到你離開時的位置重新開啟。你的電腦會要求輸入密碼。
about-update-installing = 正在安裝版本 { $version }…
about-update-installing-detail = 請在開啟的視窗中輸入你的密碼。
about-update-cancelled = 由於未輸入密碼，更新未安裝。
about-update-failed = 更新無法安裝：{ $error }
about-update-unsupported = 這份 Katna Mail 由你的套件管理員負責更新。
about-update-restart-failed = 更新已安裝，但 Katna Mail 無法重新開啟（{ $error }）。請自行開啟它。
about-update-check = 檢查更新
about-update-download = 下載
about-update-retry = 重試
about-update-button = 更新
about-update-restart = 更新並重新啟動
about-update-cancel = 稍後再說
about-changelog = 變更記錄
about-source = 原始碼
about-coffee = 請我喝杯咖啡
about-coffee-coffee = 咖啡？
about-coffee-tea = 茶？
about-coffee-pizza = 披薩？
about-coffee-nothing = 什麼都不要？真的？
about-coffee-water = 喝水就能活！！
about-coffee-thanks = 感謝使用 Katna
about-coming-soon = 即將推出
about-follow-me = 關注我：
about-love-title = 懷著對 Rust、KDE 和 Linux 的熱愛打造
about-love-text = Rust 讓撰寫快速又安全的郵件應用程式成為一種樂趣：Katna 沒有任何 unsafe 程式碼。KDE 的 Plasma 桌面及其 PIM 套件啟發了 Katna，而 Linux 與自由軟體社群則是它賴以立足的基礎。謝謝你們，也感謝下方這些程式庫。
about-kde-text = KDE 打造了 Katna 最有歸屬感的桌面。它由志工開發，並由像你這樣的人資助。如果你喜歡 Plasma 或 KDE 的應用程式，請考慮捐款給 KDE。
about-donate-kde = 捐款給 KDE
about-gpui-title = 以 Zed 專案的 GPUI 打造
about-gpui-text = Katna Mail 的整個介面都以 GPUI 打造，這是 Zed Industries 為 Zed 編輯器開發的快速、GPU 加速 UI 框架。你看到的每個像素、每段動畫和每個視窗都由它繪製。感謝 Zed 團隊以公開的方式開發它。Apache-2.0。
about-gpui-github = GitHub 上的 GPUI
about-personal-title = 個人專案
about-personal-text = Katna Mail 並不打算追求新穎或革命性。它是作者自己想要的郵件應用程式，功能和外觀借鏡了 Gmail、Mailspring 和 Thunderbird。它之所以能實現，全靠 LLM 已經進步到今天的程度。
about-built-on = 以自由軟體打造
about-credit-pimalaya = IMAP、SMTP 和登入（io-imap、io-smtp、io-sasl）
about-credit-imap-codec = 讀寫 IMAP
about-credit-tantivy = 搜尋
about-credit-sqlite = 郵件儲存
about-credit-rustls = 安全連線
about-credit-mail-parser = 讀取郵件，來自 Stalwart Labs
about-credit-html5ever = HTML 郵件，來自 Servo 專案
about-credit-zbus = 透過 D-Bus 和入口與桌面溝通
about-credit-oo7 = 桌面鑰匙圈中的密碼
about-credit-hayro = 檢視與列印 PDF
about-credit-calamine = 試算表預覽
about-credit-resvg = SVG 圖片
about-credit-jiff = 日期與時區
about-credit-spellbook = 拼字檢查，來自 Helix 編輯器
about-credit-smol = 同時處理多項工作
about-all-libraries = Katna 使用的所有程式庫（{ $count }）
about-library-authors = 作者：{ $authors }
about-license = Katna 是自由軟體，採用 GNU GPL 第 3 版或更新版本授權。
about-close = 關閉

## What’s new (shown after an update)

whats-new-title = Katna Mail 最新消息
whats-new-updated = 已更新至 { $version } 版
whats-new-version = { $version } 版
whats-new-more = { $count ->
   *[other] 完整變更記錄中還有 { $count } 項。
}
whats-new-changelog = 完整變更記錄
whats-new-got-it = 知道了

## First run: welcome page

onboarding-welcome-title = 歡迎使用 Katna Mail
onboarding-welcome-lead = 你的郵件就在你自己的電腦上：搜尋快速、離線可讀，而且保有隱私。
onboarding-fast-title = 快速，離線也一樣
onboarding-fast-text = Katna 會在這裡保存一份郵件副本，因此無論有沒有網路連線，開啟和搜尋都能即時完成。
onboarding-providers-title = 支援你的郵件
onboarding-providers-text = Gmail、Outlook、Yahoo、iCloud 以及其他任何 IMAP 或 POP 帳戶。
onboarding-private-title = 隱私
onboarding-private-text = 你的郵件會從郵件服務供應商直接傳到這台電腦。沒有任何 Katna 伺服器能看到它。
onboarding-get-started = 開始使用

## First run: adding an account

onboarding-service-checking = 正在檢查 Katna 背景服務…
onboarding-service-running = Katna 背景服務正在執行。
onboarding-service-missing = Katna 背景服務未執行
onboarding-service-start = 它負責接收和傳送你的郵件。請從終端機啟動它，然後再檢查一次：
onboarding-check-again = 再次檢查
onboarding-account-title = 新增你的郵件帳戶
onboarding-account-lead = 輸入你的電子郵件地址和密碼，Katna 就會找到伺服器設定。Gmail、Yahoo 和 iCloud 需要應用程式密碼，可在帳戶的安全性設定中產生。
onboarding-add-account = 新增帳戶
onboarding-back = 返回

## First run: choosing the look

onboarding-look-title = 打造專屬於你的樣子
onboarding-look-lead = 選擇郵件的開啟方式和 Katna 的外觀。你隨時都能在快速設定中變更。
onboarding-reading-pane = 閱讀窗格
onboarding-pane-right = 清單右側
onboarding-pane-none = 不分割
onboarding-theme = 主題
onboarding-theme-system = 跟隨系統
onboarding-theme-light = 淺色
onboarding-theme-dark = 深色
onboarding-density = 顯示密度
onboarding-density-default = 預設
onboarding-density-compact = 精簡
onboarding-continue = 繼續

## First run: done

onboarding-ready-title = 一切就緒
onboarding-ready-lead = Katna 正在接收你的郵件。郵件一到就會顯示，新郵件也會自動出現。
onboarding-ready-lead-address = Katna 正在接收 { $address } 的郵件。郵件一到就會顯示，新郵件也會自動出現。
onboarding-ready-tour = 花一分鐘導覽，看看所有功能在哪裡？
onboarding-skip = 暫時略過
onboarding-take-tour = 開始導覽

## Asking to send crash reports (on its own and on the first-run pages)

share-title = 協助改善 Katna
share-lead = Katna 當機時，會在這台電腦上儲存一份報告。傳送這些報告有助於修正問題。你隨時都能在「設定 > 使用者意見回饋」中變更。
share-sent = 會傳送的內容
share-sent-detail = 與你在設定中看到的完全相同的當機報告：當機的項目及其在 Katna 中的位置、版本、你的 Linux 系統和桌面，以及 Katna 最後幾行記錄（其中可能包含郵件資料夾名稱）。
share-never-sent = 絕不會傳送的內容
share-never-sent-detail = 你的郵件、聯絡人、密碼、IP 位址、使用者名稱或電腦名稱。電子郵件地址會從報告中移除。
share-where = 傳送到哪裡
share-where-detail = Katna 在 Sentry 上的當機追蹤系統，資料儲存在歐盟。沒有任何 ID 會將報告與你連結。
share-dont-send = 不要傳送
share-send = 傳送當機報告
share-sending = 將會傳送當機報告。謝謝。
share-local = 當機報告會留在這台電腦上。

## The tour (cards pointing at each part of the window)

tour-welcome-title = 歡迎使用 Katna Mail
tour-welcome-text = 一分鐘導覽，帶你看看所有功能在哪裡。
tour-not-now = 以後再說
tour-start = 開始導覽
tour-close = 關閉
tour-skip = 略過導覽
tour-back = 上一步
tour-done = 完成
tour-next = 下一步
tour-step = 第 { $step } 步，共 { $total } 步
tour-compose-title = 撰寫郵件
tour-compose-text = 「撰寫」會在右下角開啟新郵件，讓你一邊撰寫一邊繼續閱讀。
tour-search-title = 搜尋所有郵件
tour-search-text = 離線時也能搜尋。最右側的按鈕可以加入篩選條件：寄件者、收件者、主旨、日期和附件。
tour-menu-title = 顯示或隱藏資料夾
tour-menu-text = 這個按鈕會收合資料夾清單。隱藏時，將指標停在左側的「郵件」上即可查看資料夾。
tour-apps-title = 你的應用程式
tour-tabs-title = 收件匣分頁
tour-tabs-text = 新郵件會分類到「主要」、「促銷內容」、「社交網路」、「最新快訊」和「論壇」。你可以在快速設定中關閉這些分頁。
tour-list-title = 你的郵件
tour-list-text = 按一下郵件即可閱讀。將指標停在郵件上可使用快速動作，按右鍵可查看更多動作，也可以勾選多封郵件一起處理。
tour-settings-title = 快速設定
tour-settings-text = 在這裡變更閱讀窗格、顯示密度和主題。也可以從這裡重新開始導覽。
tour-account-title = 你的帳戶
tour-account-text = 查看你目前所在的帳戶，並新增其他帳戶。

## Crash notice (a bar at the bottom after a crash)

crash-daemon = { $more ->
    [0] Katna 背景服務意外停止。
   *[other] Katna 背景服務意外停止。另外還儲存了 { $more } 份當機報告。
}
crash-mail = { $more ->
    [0] Katna Mail 上次意外關閉。
   *[other] Katna Mail 上次意外關閉。另外還儲存了 { $more } 份當機報告。
}
crash-view = 檢視報告
crash-view-tooltip = 開啟儲存在這台電腦上的報告
crash-copy = 複製報告
crash-close = 關閉
sign-in-again-text = { $provider } 要求你重新登入 { $address }。
sign-in-again-button = 登入
sign-in-again-tooltip = 在瀏覽器中開啟 { $provider } 登入頁面
sign-in-again-waiting = 正在等待瀏覽器…
sign-in-again-close = 關閉
sign-in-again-done = 已重新登入 { $address }。正在接收你的郵件…
delete-ask-title = { $kind ->
    [conversation] { $count ->
       *[other] 將 { $count } 個會話群組移至垃圾桶？
    }
   *[message] { $count ->
       *[other] 將 { $count } 封郵件移至垃圾桶？
    }
}
delete-ask-body = { $count ->
   *[other] 你可以隨後立即復原，或以後從垃圾桶救回。
}
delete-ask-confirm = 移至垃圾桶
delete-forever-title = { $kind ->
    [conversation] { $count ->
       *[other] 永久刪除 { $count } 個會話群組？
    }
   *[message] { $count ->
       *[other] 永久刪除 { $count } 封郵件？
    }
}
delete-forever-body = { $count ->
   *[other] 伺服器上也會一併刪除，而且無法復原。
}
delete-forever-confirm = 永久刪除
delete-ask-dont-ask = 不再詢問
delete-ask-cancel = 取消
