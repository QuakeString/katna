# Katna Mail, Japanese (日本語): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = 今日
calendar-today-tip = 今日に移動
calendar-view-day = 日
calendar-view-week = 週
calendar-view-month = 月
calendar-view-year = 年
calendar-view-schedule = スケジュール
calendar-view-days =
    { $count ->
       *[other] { $count } 日間
    }
calendar-options = 設定
calendar-density = 情報密度
calendar-density-responsive = 画面に合わせて調整
calendar-density-comfortable = ゆったり
calendar-density-compact = コンパクト
calendar-custom-days = カスタム表示
calendar-second-zone = セカンダリ タイムゾーン
calendar-zone-none = なし
calendar-zone = { $zone } （{ $offset }）
calendar-share-free = 空き時間を共有
calendar-free-subject = 空いている時間
calendar-free-intro = 空いている時間をお知らせします（{ $zone }）：
calendar-free-day = { $weekday } { $date }：{ $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = 今後数営業日に空き時間はありません。
calendar-previous-day = 前の日
calendar-next-day = 次の日
calendar-previous-week = 前の週
calendar-next-week = 次の週
calendar-previous-month = 前の月
calendar-next-month = 次の月
calendar-previous-year = 前の年
calendar-next-year = 次の年
calendar-previous-period = 前へ
calendar-next-period = 次へ
calendar-title-months = { $first } – { $last }
calendar-loading = 読み込み中…
calendar-read-failed = カレンダーを読み込めませんでした: { $error }
calendar-sets = カレンダー セット
calendar-set-add = 表示中のカレンダーをセットとして保存
calendar-set-name = セットの名前
calendar-set-remove = セットを削除
calendar-local = このコンピューター
calendar-account-gone = 削除されたアカウント
calendar-account-sign-in = もう一度サインインしてカレンダーを表示
calendar-account-signed-in = { $address } に再度サインインしました。カレンダーを取得しています…
calendar-account-sign-in-refused = { $provider } が Katna のアクセスを許可しませんでした。もう一度試して、カレンダーへのアクセスを許可してください。
calendar-account-refused = サーバーがパスワードを受け付けませんでした。Yahoo、iCloud、Zoho などではアプリ パスワードが必要です。
calendar-account-change-password = パスワードを変更
calendar-account-change-password-tooltip = 新しいパスワードを入力してください。Katna がサーバーで確認します
calendar-account-not-enabled = Katna のカレンダーへのアクセスはまだ有効になっていません。
calendar-account-failed = カレンダーを読み込めませんでした。
calendar-account-error = カレンダーを読み込めませんでした: { $reason }
calendar-account-none = カレンダーが見つかりません
calendar-account-none-why = カレンダーが見つかりません: { $reason }
calendar-account-use-sign-in = { $provider } のカレンダーは、{ $provider } でサインインした Katna にのみ表示されます。
calendar-account-sign-in-with = { $provider } でサインイン
calendar-account-looking = カレンダーを探しています…
calendar-account-try-again = 再試行
calendar-account-try-again-tooltip = このアカウントのカレンダーを今すぐ再確認
calendar-account-fixing = 対応しています…
calendar-birthdays = 誕生日
calendar-tasks = タスク
calendar-birthday-of = { $name }さんの誕生日
calendar-empty-title = カレンダーはまだありません
calendar-empty-text = Google や Microsoft アカウントのカレンダーは同期が完了するとここに表示されます。CalDAV に対応する他のサーバーのカレンダーも表示されます。
calendar-schedule-empty = 今後 2 か月間の予定はありません。
calendar-search = 予定を検索
calendar-search-past = 過去の予定
calendar-search-none = 検索条件に一致する予定はありません。
calendar-no-title = (タイトルなし)
calendar-all-day = 終日
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }、{ $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = 他 { $count } 件
calendar-peek-day = { $day }（{ $weekday }）
calendar-repeats = 繰り返し
calendar-join = 参加
calendar-join-with = { $service } で参加
calendar-email-guests = ゲストにメールを送信
calendar-running-late = 遅れます
calendar-late-subject = 遅れます: { $title }
calendar-late-body = { $title } に数分遅れます。申し訳ありません。すぐに向かいます。
calendar-guests =
    { $count ->
       *[other] ゲスト { $count } 人
    }
calendar-guest-answers = はい { $yes }、未定 { $maybe }、いいえ { $no }、返信待ち { $waiting }
calendar-organizer = 主催者
calendar-optional = 任意
calendar-open-web = ブラウザで開く
calendar-open-mail = メールを開く
calendar-open-contact = 連絡先を開く
calendar-close = 閉じる

## Adding, changing and deleting events.

calendar-add-title = タイトルを追加
calendar-add-location = 場所を追加
calendar-add-notes = 説明を追加
calendar-add-guests = ゲストを追加
calendar-remove-guest = 削除
calendar-add-meet = Google Meet のビデオ会議を追加
calendar-add-teams = Teams 会議を追加
calendar-has-call = ビデオ通話を追加済み
calendar-weekday-day = { $weekday }、{ $day }
calendar-all-day-box = 終日
calendar-more-options = その他のオプション
calendar-save = 保存
calendar-saved = 予定を保存しました
calendar-deleted = 予定を削除しました
calendar-discard = 変更を破棄
calendar-edit = 予定を編集
calendar-delete = 予定を削除
calendar-event-details = 予定の詳細
calendar-menu-new-event = 新しい予定
calendar-event-window-title = 新しい予定
calendar-menu-open-day = 日を開く
calendar-menu-duplicate = 複製
calendar-menu-color = 色
calendar-menu-color-calendar = カレンダーの色
calendar-menu-in-a-week = 1 週間後
calendar-color-tomato = トマト
calendar-color-flamingo = フラミンゴ
calendar-color-tangerine = ミカン
calendar-color-banana = バナナ
calendar-color-sage = セージ
calendar-color-basil = バジル
calendar-color-peacock = ピーコック
calendar-color-blueberry = ブルーベリー
calendar-color-lavender = ラベンダー
calendar-color-grape = ブドウ
calendar-color-graphite = グラファイト
calendar-menu-only-this = これだけを表示
calendar-menu-rename = 名前を変更
calendar-menu-remove = リストから削除
calendar-menu-delete = 削除
calendar-menu-new-calendar = 新しいカレンダー
calendar-menu-show-all = すべて表示
calendar-menu-hide-all = すべて非表示
calendar-menu-account-settings = アカウント設定
calendar-why-main = メインのカレンダー
calendar-why-last = これ 1 つのみ
calendar-why-owner = オーナーのみ
calendar-why-contacts = 連絡先から
calendar-why-unreached = 接続できません
calendar-name-placeholder = カレンダー名
calendar-toast-added = 「{ $name }」を追加しました
calendar-toast-renamed = カレンダーの名前を変更しました
calendar-toast-recolored = カレンダーの色を変更しました
calendar-toast-deleted = 「{ $name }」を削除しました
calendar-toast-removed = 「{ $name }」をリストから削除しました
calendar-edit-failed = カレンダーを変更できませんでした: { $reason }
calendar-delete-title = 「{ $name }」を削除しますか？
calendar-delete-confirm = 削除
calendar-deleting = 削除しています…
calendar-delete-heading = 削除されるもの:
calendar-delete-events = カレンダーとそのすべての予定
calendar-delete-shared = 共有しているすべての人から
calendar-delete-server = Katna 上だけでなく、メールサービス上の { $account } からも削除されます。
calendar-delete-local = このパソコンから削除されます。
calendar-remove-title = 「{ $name }」をリストから削除しますか？
calendar-remove-confirm = 削除
calendar-removing = 削除しています…
calendar-remove-heading = 変わること:
calendar-remove-events = ここやほかのアプリで、このカレンダーの予定が表示されなくなります
calendar-remove-server = カレンダーはオーナーの手元に残り、オーナーが再び共有することもできます。
calendar-kind-event = 予定
calendar-kind-task = タスク
calendar-kind-focus = 集中時間
calendar-kind-out-of-office = 不在
calendar-kind-working-location = 勤務地
calendar-task-added = タスクを追加しました
calendar-task-added-to = { $list } にタスクを追加しました
calendar-task-list-local = このコンピューター
calendar-working-home = 自宅
calendar-busy = 予定あり
calendar-free = 予定なし
calendar-cancel = キャンセル
calendar-ok = OK
calendar-read-only = このカレンダーの予定は変更できません
calendar-none-editable = 予定を追加できるカレンダーがまだありません
calendar-no-such-time = その時刻はお使いのタイムゾーンには存在しません
calendar-end-before-start = 予定の終了が開始より前になっています
calendar-repeat-never = 繰り返さない
calendar-repeat-daily = 毎日
calendar-repeat-weekly = 毎週 { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] 毎月第 1 { $weekday }
        [2] 毎月第 2 { $weekday }
        [3] 毎月第 3 { $weekday }
        [4] 毎月第 4 { $weekday }
       *[other] 毎月最終 { $weekday }
    }
calendar-repeat-yearly = 毎年 { $day }
calendar-repeat-weekdays = 平日（月曜日～金曜日）
calendar-repeat-custom = カスタム
calendar-reminder-none = 通知なし
calendar-reminder-at-start = 開始時
calendar-reminder-minutes =
    { $count ->
       *[other] { $count } 分前
    }
calendar-reminder-hours =
    { $count ->
       *[other] { $count } 時間前
    }
calendar-reminder-days =
    { $count ->
       *[other] { $count } 日前
    }
calendar-scope-edit-title = 定期的な予定を編集
calendar-scope-delete-title = 定期的な予定を削除
calendar-scope-this = この予定
calendar-scope-following = この予定以降のすべての予定
calendar-scope-all = すべての予定
calendar-scope-respond-title = 定期的な予定への返信
calendar-going = 参加しますか？
calendar-answer-yes = はい
calendar-answer-no = いいえ
calendar-answer-maybe = 未定
calendar-answered-yes = 参加します
calendar-answered-no = 参加しません
calendar-answered-maybe = 参加未定です

## The card at the top of a mail with an invitation.

calendar-invite = 招待
calendar-invite-cancelled = 予定はキャンセルされました
calendar-invite-reply = { $name } さんが返信しました
calendar-invite-reply-yes = { $name } さん: 承諾
calendar-invite-reply-no = { $name } さん: 辞退
calendar-invite-reply-maybe = { $name } さん: 未定
calendar-invite-organizer = 主催者: { $name }
calendar-invite-open = カレンダーで開く
calendar-invite-not-yet = まだカレンダーにありません。同期されると返信できます。
calendar-invite-by-mail = カレンダーにない招待: 返信は主催者にメールで送られます。
calendar-mail-yes = 承諾: { $title }
calendar-mail-yes-body = { $name } さんがこの招待を承諾しました。
calendar-mail-no = 辞退: { $title }
calendar-mail-no-body = { $name } さんがこの招待を辞退しました。
calendar-mail-maybe = 仮承諾: { $title }
calendar-mail-maybe-body = { $name } さんがこの招待を仮承諾しました。
calendar-invite-your-day = あなたの一日
calendar-invite-clashes =
    { $count ->
       *[other] { $count } 件の予定と重複しています
    }

## The day's agenda beside the mail.

agenda-show = 今日の予定を表示
agenda-hide = 予定を非表示
agenda-today = 今日、{ $date }
agenda-day = { $weekday }、{ $date }
agenda-empty = この日の予定はありません。
