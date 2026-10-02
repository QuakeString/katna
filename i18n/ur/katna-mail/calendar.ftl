# Katna Mail, Urdu (اردو): the Calendar page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

calendar-today = آج
calendar-today-tip = آج پر جائیں
calendar-view-day = دن
calendar-view-week = ہفتہ
calendar-view-month = مہینہ
calendar-view-year = سال
calendar-view-schedule = شیڈول
calendar-view-days =
    { $count ->
        [one] { $count } دن
       *[other] { $count } دن
    }
calendar-options = اختیارات
calendar-density = کثافت
calendar-density-responsive = آپ کی اسکرین کے مطابق
calendar-density-comfortable = آرام دہ
calendar-density-compact = مختصر
calendar-custom-days = حسب ضرورت منظر
calendar-second-zone = دوسرا ٹائم زون
calendar-zone-none = کوئی نہیں
calendar-zone = { $zone } ({ $offset })
calendar-share-free = خالی اوقات شیئر کریں
calendar-free-subject = میرے خالی اوقات
calendar-free-intro = یہ میرے کچھ خالی اوقات ہیں ({ $zone }):
calendar-free-day = { $weekday } { $date }: { $times }
calendar-free-range = { $start } – { $end }
calendar-free-none = آئندہ چند کام کے دنوں میں میرے پاس کوئی خالی وقت نہیں ہے۔
calendar-previous-day = پچھلا دن
calendar-next-day = اگلا دن
calendar-previous-week = پچھلا ہفتہ
calendar-next-week = اگلا ہفتہ
calendar-previous-month = پچھلا مہینہ
calendar-next-month = اگلا مہینہ
calendar-previous-year = پچھلا سال
calendar-next-year = اگلا سال
calendar-previous-period = پہلے
calendar-next-period = بعد میں
calendar-title-months = { $first } – { $last }
calendar-loading = لوڈ ہو رہا ہے…
calendar-read-failed = کیلنڈر پڑھا نہیں جا سکا: { $error }
calendar-sets = کیلنڈر سیٹ
calendar-set-add = دکھائے جانے والے کیلنڈرز کو سیٹ کے طور پر محفوظ کریں
calendar-set-name = سیٹ کا نام
calendar-set-remove = سیٹ ہٹائیں
calendar-local = یہ کمپیوٹر
calendar-account-gone = ہٹایا گیا اکاؤنٹ
calendar-account-sign-in = کیلنڈرز دکھانے کے لیے دوبارہ سائن ان کریں
calendar-account-signed-in = { $address } میں دوبارہ سائن ان ہو گیا۔ آپ کے کیلنڈرز لائے جا رہے ہیں…
calendar-account-sign-in-refused = { $provider } نے Katna کو اندر نہیں آنے دیا۔ دوبارہ کوشش کریں، اور اپنے کیلنڈرز تک رسائی کی اجازت دیں۔
calendar-account-refused = سرور نے پاس ورڈ قبول نہیں کیا۔ Yahoo، iCloud، Zoho اور دیگر کو ایپ پاس ورڈ درکار ہے۔
calendar-account-change-password = پاس ورڈ بدلیں
calendar-account-change-password-tooltip = ترتیبات > اکاؤنٹس کھولیں
calendar-account-not-enabled = Katna کے لیے کیلنڈر تک رسائی ابھی آن نہیں کی گئی۔
calendar-account-failed = کیلنڈرز پڑھے نہیں جا سکے۔
calendar-account-error = کیلنڈرز پڑھے نہیں جا سکے: { $reason }
calendar-account-none = کوئی کیلنڈر نہیں ملا
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
calendar-account-none-why = کوئی کیلنڈر نہیں ملا: { $reason }
# A Gmail or Outlook account added with a password: its calendars need the
# provider's sign-in.
calendar-account-use-sign-in = { $provider } کیلنڈرز صرف اس Katna کو دکھاتا ہے جو { $provider } کے ساتھ سائن ان ہو۔
calendar-account-sign-in-with = { $provider } کے ساتھ سائن ان کریں
calendar-account-looking = کیلنڈرز تلاش کیے جا رہے ہیں…
calendar-account-try-again = دوبارہ کوشش کریں
calendar-account-try-again-tooltip = اس اکاؤنٹ کے کیلنڈرز ابھی دوبارہ چیک کریں
calendar-account-fixing = اس پر کام ہو رہا ہے…
calendar-birthdays = سالگرہیں
calendar-birthday-of = { $name } کی سالگرہ
calendar-empty-title = ابھی کوئی کیلنڈر نہیں
calendar-empty-text = آپ کے Google اور Microsoft اکاؤنٹس کے کیلنڈر ہم آہنگ ہونے کے بعد یہاں دکھائے جائیں گے، اور CalDAV فراہم کرنے والے دوسرے سرورز کے بھی۔
calendar-schedule-empty = اگلے دو مہینوں میں کچھ منصوبہ بند نہیں ہے۔
calendar-search = ایونٹس تلاش کریں
calendar-search-past = گزشتہ ایونٹس
calendar-search-none = آپ کی تلاش سے کوئی ایونٹ میل نہیں کھاتا۔
calendar-no-title = (بغیر عنوان)
calendar-all-day = پورا دن
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }، { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } مزید
calendar-peek-day = { $weekday }، { $day }
calendar-repeats = دہرایا جاتا ہے
calendar-join = شامل ہوں
calendar-join-with = { $service } کے ساتھ شامل ہوں
calendar-email-guests = مہمانوں کو ای میل کریں
calendar-running-late = دیر ہو رہی ہے
calendar-late-subject = دیر ہو رہی ہے: { $title }
calendar-late-body = معذرت، { $title } کے لیے مجھے چند منٹ کی دیر ہو رہی ہے۔ میرا جلد پہنچنا متوقع ہے۔
calendar-guests =
    { $count ->
        [one] { $count } مہمان
       *[other] { $count } مہمان
    }
calendar-guest-answers = { $yes } ہاں، { $maybe } شاید، { $no } نہیں، { $waiting } منتظر
calendar-organizer = منتظم
calendar-optional = اختیاری
calendar-open-web = براؤزر میں کھولیں
calendar-open-mail = میل کھولیں
calendar-open-contact = رابطہ کھولیں
calendar-close = بند کریں

## Adding, changing and deleting events.

calendar-add-title = عنوان شامل کریں
calendar-add-location = مقام شامل کریں
calendar-add-notes = تفصیل شامل کریں
calendar-add-guests = مہمان شامل کریں
calendar-remove-guest = ہٹائیں
calendar-add-meet = Google Meet ویڈیو کانفرنسنگ شامل کریں
calendar-add-teams = Teams میٹنگ شامل کریں
calendar-has-call = ویڈیو کال شامل کر دی گئی
calendar-weekday-day = { $weekday }، { $day }
calendar-all-day-box = پورا دن
calendar-more-options = مزید اختیارات
calendar-save = محفوظ کریں
calendar-saved = ایونٹ محفوظ ہو گیا
calendar-deleted = ایونٹ حذف ہو گیا
calendar-discard = تبدیلیاں رد کریں
calendar-edit = ایونٹ میں ترمیم کریں
calendar-delete = ایونٹ حذف کریں
calendar-event-details = ایونٹ کی تفصیلات
# Right-click menus on the calendar: on a free time or day, an event and
# a task.
calendar-menu-new-event = نیا ایونٹ
# Shows the day right-clicked on its own, in the Day view.
calendar-menu-open-day = دن کھولیں
calendar-menu-duplicate = ڈپلیکیٹ بنائیں
calendar-menu-color = رنگ
# The event takes its calendar's color.
calendar-menu-color-calendar = کیلنڈر کا رنگ
# A task's new due day, a week from today.
calendar-menu-in-a-week = ایک ہفتے میں
# Event colors, by the names Google Calendar gives them.
calendar-color-tomato = ٹماٹر
calendar-color-flamingo = فلیمنگو
calendar-color-tangerine = سنگترہ
calendar-color-banana = کیلا
calendar-color-sage = سیج
calendar-color-basil = تلسی
calendar-color-peacock = مور
calendar-color-blueberry = بلوبیری
calendar-color-lavender = لیونڈر
calendar-color-grape = انگور
calendar-color-graphite = گریفائٹ
calendar-menu-only-this = صرف یہ دکھائیں
calendar-menu-rename = نام بدلیں
calendar-menu-remove = فہرست سے ہٹائیں
calendar-menu-delete = حذف کریں
calendar-menu-new-calendar = نیا کیلنڈر
calendar-menu-show-all = سب دکھائیں
calendar-menu-hide-all = سب چھپائیں
calendar-menu-account-settings = اکاؤنٹ کی ترتیبات
calendar-why-main = مرکزی کیلنڈر
calendar-why-last = یہاں صرف یہی ہے
calendar-why-owner = صرف مالک
calendar-why-contacts = رابطوں سے
calendar-why-unreached = رسائی نہیں ہوئی
calendar-name-placeholder = کیلنڈر کا نام
calendar-toast-added = ”{ $name }“ شامل ہو گیا
calendar-toast-renamed = کیلنڈر کا نام بدل گیا
calendar-toast-recolored = کیلنڈر کا رنگ بدل گیا
calendar-toast-deleted = ”{ $name }“ حذف ہو گیا
calendar-toast-removed = ”{ $name }“ آپ کی فہرست سے ہٹا دیا گیا
calendar-edit-failed = کیلنڈر تبدیل نہیں ہوا: { $reason }
calendar-delete-title = ”{ $name }“ حذف کریں؟
calendar-delete-confirm = حذف کریں
calendar-deleting = حذف ہو رہا ہے…
calendar-delete-heading = حذف ہو گا:
calendar-delete-events = کیلنڈر اور اس کے تمام ایونٹس
calendar-delete-shared = ہر اس شخص کے لیے جس کے ساتھ یہ شیئر ہے
calendar-delete-server = یہ میل سروس پر { $account } سے حذف ہوتا ہے، صرف Katna میں نہیں۔
calendar-delete-local = یہ اس کمپیوٹر سے حذف ہوتا ہے۔
calendar-remove-title = ”{ $name }“ اپنی فہرست سے ہٹائیں؟
calendar-remove-confirm = ہٹائیں
calendar-removing = ہٹایا جا رہا ہے…
calendar-remove-heading = کیا بدلے گا:
calendar-remove-events = آپ کو اس کے ایونٹس دکھائی دینا بند ہو جاتے ہیں، یہاں اور آپ کی دوسری ایپس میں
calendar-remove-server = کیلنڈر اپنے مالک کے پاس رہتا ہے، جو اسے دوبارہ آپ کے ساتھ شیئر کر سکتا ہے۔
calendar-kind-event = ایونٹ
calendar-kind-task = کام
calendar-kind-focus = فوکس ٹائم
calendar-kind-out-of-office = دفتر سے باہر
calendar-kind-working-location = کام کی جگہ
calendar-task-added = کام شامل ہو گیا
calendar-task-added-to = کام { $list } میں شامل ہو گیا
calendar-task-list-local = اس کمپیوٹر پر
calendar-working-home = گھر
calendar-busy = مصروف
calendar-free = فارغ
calendar-cancel = منسوخ کریں
calendar-ok = ٹھیک ہے
calendar-read-only = آپ اس کیلنڈر کے ایونٹس تبدیل نہیں کر سکتے
calendar-none-editable = ابھی کوئی ایسا کیلنڈر نہیں جس میں ایونٹ شامل کیا جا سکے
calendar-no-such-time = آپ کے ٹائم زون میں یہ وقت موجود نہیں
calendar-end-before-start = ایونٹ شروع ہونے سے پہلے ختم ہو جاتا ہے
calendar-repeat-never = دہرایا نہیں جاتا
calendar-repeat-daily = روزانہ
calendar-repeat-weekly = ہفتہ وار بروز { $weekday }
calendar-repeat-monthly =
    { $nth ->
        [1] ماہانہ پہلا { $weekday }
        [2] ماہانہ دوسرا { $weekday }
        [3] ماہانہ تیسرا { $weekday }
        [4] ماہانہ چوتھا { $weekday }
       *[other] ماہانہ آخری { $weekday }
    }
calendar-repeat-yearly = سالانہ { $day } کو
calendar-repeat-weekdays = ہر ہفتے کے دن (پیر سے جمعہ)
calendar-repeat-custom = حسب ضرورت
calendar-reminder-none = کوئی اطلاع نہیں
calendar-reminder-at-start = آغاز پر
calendar-reminder-minutes =
    { $count ->
        [one] { $count } منٹ پہلے
       *[other] { $count } منٹ پہلے
    }
calendar-reminder-hours =
    { $count ->
        [one] { $count } گھنٹہ پہلے
       *[other] { $count } گھنٹے پہلے
    }
calendar-reminder-days =
    { $count ->
        [one] { $count } دن پہلے
       *[other] { $count } دن پہلے
    }
calendar-scope-edit-title = بار بار ہونے والے ایونٹ میں ترمیم کریں
calendar-scope-delete-title = بار بار ہونے والے ایونٹ کو حذف کریں
calendar-scope-this = یہ ایونٹ
calendar-scope-following = یہ اور بعد کے ایونٹس
calendar-scope-all = تمام ایونٹس
calendar-scope-respond-title = بار بار ہونے والے ایونٹ کا جواب
calendar-going = کیا آپ جا رہے ہیں؟
calendar-answer-yes = ہاں
calendar-answer-no = نہیں
calendar-answer-maybe = شاید
calendar-answered-yes = آپ جا رہے ہیں
calendar-answered-no = آپ نہیں جا رہے
calendar-answered-maybe = آپ شاید جائیں

## The card at the top of a mail with an invitation.

calendar-invite = دعوت
calendar-invite-cancelled = ایونٹ منسوخ ہو گیا
calendar-invite-reply = { $name }: جواب
calendar-invite-reply-yes = { $name }: قبول
calendar-invite-reply-no = { $name }: انکار
calendar-invite-reply-maybe = { $name }: شاید
calendar-invite-organizer = منتظم: { $name }
calendar-invite-open = کیلنڈر میں کھولیں
calendar-invite-not-yet = ابھی آپ کے کیلنڈر میں نہیں ہے۔ سنک ہونے کے بعد جواب دیا جا سکے گا۔
calendar-invite-by-mail = آپ کے کیلنڈر میں نہیں ہے: آپ کا جواب منتظم کو میل کے ذریعے بھیجا جائے گا۔
calendar-mail-yes = قبول کیا گیا: { $title }
calendar-mail-yes-body = { $name } کی طرف سے یہ دعوت قبول کی گئی ہے۔
calendar-mail-no = مسترد کیا گیا: { $title }
calendar-mail-no-body = { $name } کی طرف سے یہ دعوت مسترد کی گئی ہے۔
calendar-mail-maybe = عارضی: { $title }
calendar-mail-maybe-body = { $name } کی طرف سے یہ دعوت عارضی طور پر قبول کی گئی ہے۔
calendar-invite-your-day = آپ کا دن
calendar-invite-clashes =
    { $count ->
        [one] { $count } ایونٹ سے ٹکراتا ہے
       *[other] { $count } ایونٹس سے ٹکراتا ہے
    }

## The day's agenda beside the mail.

agenda-show = دن کا ایجنڈا دکھائیں
agenda-hide = ایجنڈا چھپائیں
agenda-today = آج، { $date }
agenda-day = { $weekday }، { $date }
agenda-empty = اس دن کچھ منصوبہ بند نہیں ہے۔
