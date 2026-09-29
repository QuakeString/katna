# Katna Mail, Urdu (اردو): the Contacts page.
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## The column at the left

contacts-all = رابطے
contacts-frequent = اکثر
contacts-other = دیگر رابطے
contacts-other-about = وہ لوگ جنہیں آپ نے Gmail سے میل کیا مگر محفوظ نہیں کیا
contacts-other-email = ای میل بھیجیں
contacts-other-empty = کوئی دیگر رابطہ نہیں ہے۔ جن لوگوں کو آپ Gmail سے میل کرتے ہیں مگر محفوظ نہیں کرتے، وہ یہاں نظر آئیں گے۔
contacts-other-allow = دیگر رابطے دیکھنے کے لیے اپنے Gmail اکاؤنٹ میں دوبارہ سائن اِن کریں اور Katna کو انہیں دیکھنے کی اجازت دیں۔
contacts-labels = لیبلز
contacts-label-options = لیبل کے اختیارات
contacts-label-rename = لیبل کا نام بدلیں
contacts-label-email = سب کو ای میل کریں
contacts-label-delete = لیبل حذف کریں
contacts-label-new = نیا لیبل
contacts-label-name = لیبل کا نام
contacts-label-button = لیبل
contacts-label-menu = لیبل بطور:
contacts-label-added = { $name } میں شامل کیا گیا
contacts-label-removed = { $name } سے ہٹایا گیا
contacts-label-renamed = لیبل کا نام بدل کر { $name } کر دیا گیا
contacts-label-deleted = لیبل { $name } حذف ہو گیا
contacts-label-no-email = اس لیبل میں کسی کا ای میل پتہ نہیں ہے
contacts-manage = درست کریں اور نظم کریں
contacts-merge = ضم کریں اور درست کریں
contacts-merge-about = { $count ->
   *[other] { $count } تجویز: ایسے رابطے جو ایک ہی شخص لگتے ہیں
}
contacts-merge-none = کوئی نقل نہیں۔ ایک ہی نام یا فون نمبر والے رابطے یہاں نظر آئیں گے۔
contacts-merge-count = { $count ->
    [one] { $count } رابطہ
   *[other] { $count } رابطے
}
contacts-merge-all = سب ضم کریں
contacts-merge-button = ضم کریں
contacts-merge-dismiss = مسترد کریں
contacts-merged = { $count ->
    [1] رابطے ضم ہو گئے
   *[other] { $count } انضمام مکمل ہوئے
}
contacts-import = درآمد کریں
contacts-export = برآمد کریں
contacts-import-title = vCard فائل سے رابطے درآمد کریں
contacts-imported = { $count ->
    [one] { $count } رابطہ { $place } میں درآمد ہو گیا
   *[other] { $count } رابطے { $place } میں درآمد ہو گئے
}
contacts-imported-some = { $count ->
    [one] { $count } رابطہ { $place } میں درآمد ہو گیا؛ پہلے سے محفوظ { $skipped } چھوڑ دیے گئے
   *[other] { $count } رابطے { $place } میں درآمد ہو گئے؛ پہلے سے محفوظ { $skipped } چھوڑ دیے گئے
}
contacts-import-none = { $name } میں کوئی رابطہ نہیں ملا
contacts-import-all-saved = { $name } کے سب لوگ پہلے سے محفوظ ہیں
contacts-import-failed = { $name } کو پڑھا نہیں جا سکا: { $error }
contacts-exported = { $count ->
    [one] { $count } رابطہ { $path } میں برآمد ہو گیا
   *[other] { $count } رابطے { $path } میں برآمد ہو گئے
}
contacts-export-none = برآمد کرنے کے لیے کوئی رابطہ نہیں
contacts-export-failed = رابطے برآمد نہیں ہو سکے: { $error }
contacts-create = رابطہ بنائیں

## Search and the list

contacts-search = رابطے تلاش کریں
contacts-loading = رابطے لوڈ ہو رہے ہیں…
contacts-empty = ابھی کوئی محفوظ رابطہ نہیں ہے۔ جو رابطے آپ Gmail، Outlook یا اپنی میل سروس میں محفوظ کرتے ہیں وہ یہاں نظر آئیں گے۔
contacts-empty-no-books = آپ کے اکاؤنٹس کے رابطے ہم آہنگ ہونے کے بعد یہاں نظر آئیں گے۔
contacts-none-found = آپ کی تلاش سے کوئی رابطہ میل نہیں کھاتا۔
contacts-starred = { $count ->
    [one] ستارے والا رابطہ ({ $count })
   *[other] ستارے والے رابطے ({ $count })
}
contacts-count = رابطے ({ $count })
contacts-col-name = نام
contacts-col-email = ای میل
contacts-col-phone = فون نمبر
contacts-col-job = عہدہ اور کمپنی
contacts-col-labels = لیبلز

## Asking to allow contacts, for accounts signed in before Katna read them

contacts-allow = Katna کو { $address } کے رابطے پڑھنے کی اجازت دیں۔
contacts-allow-many = { $more ->
    [one] Katna کو { $address } اور { $more } مزید اکاؤنٹ کے رابطے پڑھنے کی اجازت دیں۔
   *[other] Katna کو { $address } اور { $more } مزید اکاؤنٹس کے رابطے پڑھنے کی اجازت دیں۔
}
contacts-allow-button = اجازت دیں

## A contact's page

contacts-back = رابطوں پر واپس جائیں
contacts-edit = ترمیم کریں
contacts-delete = حذف کریں
contacts-deleted = { $name } حذف ہو گیا
contacts-added = { $name } کو رابطوں میں شامل کیا گیا
contacts-find-mail = میل
contacts-details = رابطے کی تفصیلات
contacts-saved-in = محفوظ کردہ در
contacts-notes = نوٹس
contacts-birthday = سالگرہ
contacts-nickname = عرفیت
contacts-this-computer = یہ کمپیوٹر
contacts-kind-home = گھر
contacts-kind-work = کام
contacts-kind-mobile = موبائل
contacts-kind-other = دیگر
contacts-source-google = Google رابطے
contacts-source-microsoft = Outlook رابطے
contacts-source-carddav = CardDAV

## Creating and changing a contact

contacts-edit-new-title = رابطہ بنائیں
contacts-edit-title = رابطے میں ترمیم کریں
contacts-edit-save = محفوظ کریں
contacts-edit-saving = محفوظ کیا جا رہا ہے…
contacts-edit-cancel = منسوخ کریں
contacts-saved = رابطہ محفوظ ہو گیا
contacts-edit-save-to = اس میں محفوظ کریں
contacts-edit-changes-go-to = تبدیلیاں { $place } میں محفوظ کی جاتی ہیں۔
contacts-edit-given = پہلا نام
contacts-edit-family = آخری نام
contacts-edit-company = کمپنی
contacts-edit-job = عہدہ
contacts-edit-email = ای میل
contacts-edit-phone = فون
contacts-edit-with-kind = { $field } ({ $kind })
contacts-edit-add-email = ای میل شامل کریں
contacts-edit-add-phone = فون شامل کریں
contacts-edit-street = گلی کا پتا
contacts-edit-city = شہر
contacts-edit-postcode = پوسٹل کوڈ
contacts-edit-country = ملک
contacts-edit-birthday = سالگرہ (YYYY-MM-DD)
contacts-edit-empty = پہلے نام، ای میل یا فون نمبر شامل کریں۔
