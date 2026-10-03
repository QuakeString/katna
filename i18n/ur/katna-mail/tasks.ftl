# Katna Mail, Urdu (اردو): the Tasks page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Left side

tasks-create = نیا کام
tasks-all = تمام کام
tasks-today = آج
tasks-starred = ستارے والے
tasks-new-list = نئی فہرست بنائیں
tasks-on-this-computer = اس کمپیوٹر پر
tasks-my-tasks = میرے کام
# The line under an account in the side list whose task lists could not
# come: why, and the one click that fixes it.
tasks-account-sign-in = کام دکھانے کے لیے دوبارہ سائن ان کریں
tasks-account-signed-in = { $address } میں دوبارہ سائن ان ہو گیا۔ آپ کے کام لائے جا رہے ہیں…
tasks-account-sign-in-refused = { $provider } نے Katna کو اندر نہیں آنے دیا۔ دوبارہ کوشش کریں، اور اپنے کاموں تک رسائی کی اجازت دیں۔
tasks-account-refused = سرور نے پاس ورڈ قبول نہیں کیا۔ Yahoo، iCloud، Zoho اور دیگر کو ایپ پاس ورڈ درکار ہے۔
tasks-account-change-password = پاس ورڈ بدلیں
tasks-account-change-password-tooltip = ترتیبات > اکاؤنٹس کھولیں
tasks-account-not-enabled = Katna کے لیے کاموں تک رسائی ابھی آن نہیں کی گئی۔
tasks-account-failed = کاموں کی فہرستیں پڑھی نہیں جا سکیں۔
# $reason is the server's own words, in English.
tasks-account-error = کاموں کی فہرستیں پڑھی نہیں جا سکیں: { $reason }
tasks-account-none = کاموں کی کوئی فہرست نہیں ملی
# $reason is what the server answered, in English: "calendar.zoho.in answered 404".
tasks-account-none-why = کاموں کی کوئی فہرست نہیں ملی: { $reason }
# A Gmail or Outlook account added with a password: its tasks need the
# provider's sign-in.
tasks-account-use-sign-in = { $provider } کام صرف اس Katna کو دکھاتا ہے جو { $provider } کے ساتھ سائن ان ہو۔
tasks-account-sign-in-with = { $provider } کے ساتھ سائن ان کریں
tasks-account-looking = کاموں کی فہرستیں تلاش کی جا رہی ہیں…
tasks-account-try-again = دوبارہ کوشش کریں
tasks-account-try-again-tooltip = اس اکاؤنٹ کے کام ابھی دوبارہ چیک کریں
tasks-account-fixing = اس پر کام ہو رہا ہے…
tasks-list-name-placeholder = فہرست کا نام

## Lists and tasks

tasks-loading = آپ کے کام پڑھے جا رہے ہیں…
tasks-no-lists = آپ کی کاموں کی فہرستیں یہاں نظر آئیں گی۔
tasks-search = کام تلاش کریں
tasks-search-none = آپ کی تلاش سے کوئی کام میل نہیں کھاتا۔
tasks-add = کام شامل کریں
tasks-title-placeholder = عنوان
tasks-add-step = ذیلی کام شامل کریں
tasks-empty = ابھی کوئی کام نہیں۔ اوپر ایک شامل کریں۔
tasks-starred-empty = کسی کام پر ستارہ لگائیں تاکہ وہ یہاں نظر آئے۔
tasks-today-empty = آج کوئی کام واجب الادا نہیں۔
tasks-today-date = { $weekday }، { $day }
tasks-overdue = تاخیر شدہ
tasks-completed = { $count ->
    [one] مکمل ({ $count })
   *[other] مکمل ({ $count })
}
tasks-list-options = فہرست کے اختیارات
tasks-rename-list = فہرست کا نام تبدیل کریں
tasks-delete-list = فہرست حذف کریں
tasks-mark-done = مکمل کے بطور نشان زد کریں
tasks-mark-open = نامکمل کے بطور نشان زد کریں
tasks-star = ستارہ لگائیں
tasks-unstar = ستارہ ہٹائیں
tasks-edit-title = عنوان میں ترمیم کریں
tasks-details = تفصیلات
tasks-delete = حذف کریں
tasks-move-to = { $list } میں منتقل کریں
tasks-from-mail = میل
tasks-open-mail = میل کھولیں
tasks-from-note = نوٹ
tasks-open-note = نوٹ کھولیں
tasks-note-gone = وہ نوٹ اب یہاں نہیں ہے۔
tasks-no-subject = (کوئی موضوع نہیں)

## The details dialog

tasks-notes-placeholder = تفصیلات شامل کریں
tasks-date = تاریخ
tasks-no-date = کوئی تاریخ نہیں
tasks-time-placeholder = وقت شامل کریں
tasks-repeat = دہرائیں
tasks-repeat-never = دہرایا نہیں جاتا
tasks-repeat-daily = روزانہ
tasks-repeat-weekly = ہفتہ وار
tasks-repeat-monthly = ماہانہ
tasks-repeat-yearly = سالانہ
tasks-repeat-other = حسب ضرورت
tasks-remind = مجھے یاد دلائیں
tasks-remind-off = یاد دہانی نہیں
tasks-remind-on-time = وقت پر
tasks-remind-morning = اسی دن، { $time }
tasks-remind-hour-before = ایک گھنٹہ پہلے
tasks-remind-day-before = ایک دن پہلے
tasks-cancel = منسوخ کریں
tasks-save = محفوظ کریں
tasks-not-a-time = “{ $text }” وقت نہیں ہے، مثال کے طور پر { $example }۔

## Due days

tasks-due-today = آج
tasks-due-tomorrow = آئندہ کل
tasks-due-yesterday = گزشتہ کل
tasks-due-at = { $day }، { $time }

## Notes at the bottom

tasks-toast-done = کام مکمل ہو گیا
tasks-toast-next = ہو گیا۔ اگلی بار { $date } کو
tasks-toast-deleted = کام حذف ہو گیا
tasks-toast-added = { $count ->
    [one] کاموں میں شامل ہو گیا
   *[other] { $count } کام شامل ہو گئے
}
tasks-mail-gone = وہ میل اب یہاں نہیں ہے۔
tasks-toast-list-deleted = فہرست حذف ہو گئی
tasks-toast-moved = { $list } میں منتقل ہو گیا
# A task dragged to another place in its own list.
tasks-toast-placed = کام منتقل ہو گیا
tasks-toast-rescheduled = کام کا وقت تبدیل کر دیا گیا
