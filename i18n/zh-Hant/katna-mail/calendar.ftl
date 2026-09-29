# Katna Mail, Chinese (Traditional, Taiwan) (繁體中文): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = 今天
calendar-today-tip = 前往今天
calendar-view-day = 日
calendar-view-week = 週
calendar-view-month = 月
calendar-view-schedule = 行程
calendar-options = 設定
calendar-density = 資訊密度
calendar-density-responsive = 隨螢幕自動調整
calendar-density-comfortable = 舒適
calendar-density-compact = 精簡
calendar-second-zone = 次要時區
calendar-zone-none = 無
calendar-zone = { $zone } （{ $offset }）
calendar-previous-day = 前一天
calendar-next-day = 後一天
calendar-previous-week = 上週
calendar-next-week = 下週
calendar-previous-month = 上個月
calendar-next-month = 下個月
calendar-previous-period = 較早
calendar-next-period = 較晚
calendar-title-months = { $first } – { $last }
calendar-loading = 載入中…
calendar-read-failed = 無法讀取日曆：{ $error }
calendar-local = 這部電腦
calendar-account-gone = 已移除的帳號
calendar-empty-title = 還沒有日曆
calendar-empty-text = 你 Google 和 Microsoft 帳號的日曆同步後會顯示在這裡，其他支援 CalDAV 的伺服器上的日曆也一樣。
calendar-schedule-empty = 接下來兩個月沒有任何行程。
calendar-no-title = (無標題)
calendar-all-day = 全天
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }，{ $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = 還有 { $count } 項
calendar-repeats = 重複
calendar-join = 加入
calendar-email-guests = 傳送電子郵件給訪客
calendar-running-late = 我會遲到
calendar-late-subject = 我會遲到：{ $title }
calendar-late-body = 抱歉，「{ $title }」我會晚幾分鐘到，馬上就到。
calendar-guests =
    { $count ->
       *[other] { $count } 位訪客
    }
calendar-guest-answers = { $yes } 人參加，{ $maybe } 人可能參加，{ $no } 人不參加，{ $waiting } 人未回覆
calendar-organizer = 主辦者
calendar-optional = 選填
calendar-open-web = 在瀏覽器中開啟
calendar-close = 關閉

## Adding, changing and deleting events.

calendar-add-title = 新增標題
calendar-add-location = 新增地點
calendar-add-notes = 新增說明
calendar-add-guests = 新增訪客
calendar-remove-guest = 移除
calendar-add-meet = 新增 Google Meet 視訊會議
calendar-add-teams = 新增 Teams 會議
calendar-has-call = 已新增視訊通話
calendar-weekday-day = { $weekday }，{ $day }
calendar-all-day-box = 整天
calendar-more-options = 更多選項
calendar-save = 儲存
calendar-saved = 活動已儲存
calendar-deleted = 活動已刪除
calendar-discard = 捨棄變更
calendar-edit = 編輯活動
calendar-delete = 刪除活動
calendar-event-details = 活動詳細資料
calendar-kind-event = 活動
calendar-kind-focus = 專注時間
calendar-kind-out-of-office = 不在辦公室
calendar-kind-working-location = 工作地點
calendar-working-home = 家
calendar-busy = 忙碌
calendar-free = 有空
calendar-cancel = 取消
calendar-ok = 確定
calendar-read-only = 你無法變更這個日曆中的活動
calendar-none-editable = 目前沒有可新增活動的日曆
calendar-no-such-time = 你所在的時區沒有這個時間
calendar-end-before-start = 活動的結束時間早於開始時間
calendar-repeat-never = 不重複
calendar-repeat-daily = 每天
calendar-repeat-weekly = 每週的 { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] 每月第一個 { $weekday }
        [2] 每月第二個 { $weekday }
        [3] 每月第三個 { $weekday }
        [4] 每月第四個 { $weekday }
       *[other] 每月最後一個 { $weekday }
    }
calendar-repeat-yearly = 每年的 { $day }
calendar-repeat-weekdays = 平日（週一至週五）
calendar-repeat-custom = 自訂
calendar-reminder-none = 不通知
calendar-reminder-at-start = 開始時
calendar-reminder-minutes =
    { $count ->
       *[other] { $count } 分鐘前
    }
calendar-reminder-hours =
    { $count ->
       *[other] { $count } 小時前
    }
calendar-reminder-days =
    { $count ->
       *[other] { $count } 天前
    }
calendar-scope-edit-title = 編輯重複活動
calendar-scope-delete-title = 刪除重複活動
calendar-scope-this = 這個活動
calendar-scope-following = 這個和後續活動
calendar-scope-all = 所有活動
calendar-scope-respond-title = 回覆重複活動
calendar-going = 是否參加？
calendar-answer-yes = 是
calendar-answer-no = 否
calendar-answer-maybe = 也許
calendar-answered-yes = 你會參加
calendar-answered-no = 你不會參加
calendar-answered-maybe = 你可能會參加

## The card at the top of a mail with an invitation.

calendar-invite = 邀請
calendar-invite-cancelled = 活動已取消
calendar-invite-reply = { $name } 已回覆
calendar-invite-reply-yes = { $name }：已接受
calendar-invite-reply-no = { $name }：已拒絕
calendar-invite-reply-maybe = { $name }：可能參加
calendar-invite-organizer = 主辦者：{ $name }
calendar-invite-open = 在日曆中開啟
calendar-invite-not-yet = 尚未出現在你的日曆中。同步後即可回覆。
calendar-invite-by-mail = 不在你的日曆中：你的回覆會以郵件傳送給主辦人。
calendar-mail-yes = 已接受：{ $title }
calendar-mail-yes-body = { $name } 已接受這項邀請。
calendar-mail-no = 已拒絕：{ $title }
calendar-mail-no-body = { $name } 已拒絕這項邀請。
calendar-mail-maybe = 暫定：{ $title }
calendar-mail-maybe-body = { $name } 已暫時接受這項邀請。
calendar-invite-your-day = 你的一天
calendar-invite-clashes =
    { $count ->
       *[other] 與 { $count } 個活動衝突
    }

## The day's agenda beside the mail.

agenda-show = 顯示當天行程
agenda-hide = 隱藏行程
agenda-today = 今天，{ $date }
agenda-day = { $weekday }，{ $date }
agenda-empty = 這一天沒有任何行程。
