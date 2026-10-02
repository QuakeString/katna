# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

chat-heading = 閱讀
chat-view = 以聊天方式顯示會話群組
chat-view-detail = 人與人之間的郵件會像群組聊天一樣呈現：每封郵件一個對話泡泡，只顯示實際寫下的內容，你自己的郵件在右側。電子報會維持一般檢視。
chat-view-switch = 以聊天方式顯示會話群組
chat-view-switch-detail = 引用的郵件和簽名會收在每個對話泡泡的 ··· 後方
chat-switch-chat = 聊天
chat-switch-mail = 郵件
chat-people = { $names } 和你 · { $count ->
   *[other] { $count } 封郵件
}
chat-people-heading = { $count ->
   *[other] 此聊天中的成員 · { $count } 人
}
chat-member-mails = { $count ->
    [0] 沒有郵件
   *[other] { $count } 封郵件
}
chat-today = 今天
chat-yesterday = 昨天
chat-added = { $who } 新增了 { $names }
chat-renamed = { $who } 將主旨變更為「{ $subject }」
chat-you = 你
chat-not-downloaded = 尚未下載
chat-forwarded = 轉寄的郵件
chat-show-quoted = 顯示引用的郵件和簽名
chat-hide-quoted = 隱藏引用的郵件和簽名
chat-hide-dots = 隱藏 ···
chat-show-card = 顯示對方的資訊卡
chat-reply-all = 回覆所有人
chat-more = 更多
chat-reply-only = 只回覆 { $name }
chat-forward = 轉寄
chat-copy-text = 複製文字
chat-show-as-mail = 以郵件方式顯示
chat-pin = 釘選到頂端
chat-pin-file = 將檔案釘選到頂端
chat-unpin = 取消釘選
chat-unpin-file = 取消釘選檔案
chat-pinned-of = 已釘選：第 { $at } 個，共 { $count } 個
chat-pins-all = 所有釘選項目
chat-pins-heading = 已釘選 · { $count }/{ $most }
chat-pins-drag = 拖曳即可重新排序
chat-pin-from-mail = 來自 { $name } 的郵件 · { $when }
chat-pin-from-file = 來自 { $name } 的檔案 · { $when }
chat-pin-from-text = 來自 { $name } 的文字 · { $when }
chat-pins-full = 此聊天已有 5 個釘選項目
chat-pins-replace-title = 取代釘選項目
chat-pins-replace-hint = 每個聊天最多可釘選 5 個項目。請選擇要移除的項目。
chat-pins-replace = 取代
chat-pins-cancel = 取消
chat-undo = 復原
chat-reply-to = 回覆 { $names }
chat-send = 傳送（Ctrl+Enter）
chat-attach = 附加
chat-attach-photo = 相片
chat-attach-file = 檔案
chat-attach-library = 從「檔案」
chat-attach-template = 範本
chat-attach-signature = 簽名
chat-replying-to = 正在回覆 { $name }
chat-reply-newest = 回覆最新的郵件

## The attach picker (paperclip > From Files)

picker-title = 從「檔案」附加
picker-search = 搜尋名稱、使用者和主旨
picker-search-drive = 搜尋此雲端硬碟
picker-mail-files = 郵件中的檔案
picker-this-chat = 這個會話群組
picker-this-computer = 這部電腦…
picker-in-chat = 在這個會話群組中
picker-recent = 最近
picker-preview = 預覽
picker-cancel = 取消
picker-attach = 附加
picker-attach-count = 附加 { $count } 個
picker-selected = 已選取 { $count } 個
picker-of-limit = / { $limit }
picker-in-mail = 郵件內 { $size }
picker-drive-links = { $count ->
   *[other] { $count } 個以 Google Drive 連結形式
}
picker-onedrive-links = { $count ->
   *[other] { $count } 個以 OneDrive 連結形式
}
picker-over = { $size }，超過郵件可容納的 { $limit }
picker-getting = { $count ->
   *[other] 正在從雲端硬碟取得 { $count } 個檔案…
}
picker-some-failed = { $count ->
   *[other] 有 { $count } 個檔案無法讀取
}
