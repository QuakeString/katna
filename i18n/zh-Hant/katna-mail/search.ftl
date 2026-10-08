# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Search options (the panel from the button at the right of the search box)

search-options = 搜尋選項
search-options-close = 關閉
search-from = 寄件者
search-to = 收件者
search-subject = 主旨
search-has-words = 包含字詞
search-without = 不包含
search-date-within = 日期範圍
search-has-attachment = 含附件
search-attachment-custom = 自訂
search-attachment-image = 圖片
search-attachment-custom-hint = 輸入副檔名，例如 png，然後按空白鍵
search-attachment-remove = 移除
search-clear-filter = 清除篩選條件

## Search options: "Date within" choices

search-within-any = 不限時間
search-within-days = { $count ->
   *[other] { $count } 天
}
search-within-weeks = { $count ->
   *[other] { $count } 週
}
search-within-months = { $count ->
   *[other] { $count } 個月
}
search-within-years = { $count ->
   *[other] { $count } 年
}
search-within-custom = 自訂

## Search options: custom dates (the calendar popover)

search-dates-on = 在該日
search-dates-before = 之前
search-dates-since = 從該日起
search-dates-between = 之間
search-dates-from = 開始
search-dates-to = 結束
search-dates-placeholder = YYYY-MM-DD
search-dates-missing = 請選擇日期
search-dates-unreadable = 請使用類似 2026-09-01 的日期
search-dates-out-of-range = 該日期超出範圍
search-dates-chip-before = { $date } 之前
search-dates-chip-since = { $date } 起
search-dates-chip-between = { $first }–{ $last }
search-dates-cancel = 取消
search-dates-done = 完成
search-dates-month-back = 上個月
search-dates-month-on = 下個月
search-dates-year-back = 去年
search-dates-year-on = 明年

## More results on server: under the results, mail found by asking the
## mail server, for mail that is not downloaded to this computer yet.

search-server-more = 伺服器上的更多結果
search-server-searching = 正在搜尋伺服器上的郵件…
search-server-empty-searching = 目前還沒有內容。正在搜尋伺服器上的郵件…
search-server-nothing = 伺服器上沒有更多結果
search-server-failed = 無法搜尋伺服器。
search-server-again = 再試一次
