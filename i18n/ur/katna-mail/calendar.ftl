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
calendar-birthdays = سالگرہیں
calendar-birthday-of = { $name } کی سالگرہ
calendar-empty-title = ابھی کوئی کیلنڈر نہیں
calendar-empty-text = آپ کے Google اور Microsoft اکاؤنٹس کے کیلنڈر ہم آہنگ ہونے کے بعد یہاں دکھائے جائیں گے، اور CalDAV فراہم کرنے والے دوسرے سرورز کے بھی۔
calendar-schedule-empty = اگلے دو مہینوں میں کچھ منصوبہ بند نہیں ہے۔
calendar-no-title = (بغیر عنوان)
calendar-all-day = پورا دن
calendar-time-range = { $start } – { $end }
calendar-short-event = { $title }، { $time }
calendar-when = { $day } · { $time }
calendar-days-range = { $first } – { $last }
calendar-more = { $count } مزید
calendar-repeats = دہرایا جاتا ہے
calendar-join = شامل ہوں
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
calendar-kind-event = ایونٹ
calendar-kind-focus = فوکس ٹائم
calendar-kind-out-of-office = دفتر سے باہر
calendar-kind-working-location = کام کی جگہ
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
