# Katna Mail, Urdu (اردو).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Compose window: title bar

compose-new-message = نیا پیغام
compose-restore = بحال کریں
compose-minimize = چھوٹا کریں
compose-exit-full-screen = فل اسکرین سے باہر نکلیں
compose-open-window = نئی ونڈو میں کھولیں
compose-save-close = محفوظ کر کے بند کریں
compose-back-to-mail = میل ونڈو پر واپس جائیں
compose-pop-out-reply = جواب الگ کھولیں
compose-edit-recipients = وصول کنندگان میں ترمیم کریں
compose-summary-cc = Cc: { $names }
compose-summary-bcc = Bcc: { $names }
compose-more-recipients = { $count } مزید
compose-show-trimmed = کاٹا گیا مواد دکھائیں
compose-hide-trimmed = کاٹا گیا مواد چھپائیں
compose-remove-trimmed = حوالہ دیا گیا متن ہٹائیں
compose-trimmed-removed = حوالہ دیا گیا متن ہٹا دیا گیا

## Recipients and subject

compose-to = بنام
compose-cc = Cc
compose-bcc = Bcc
compose-from = منجانب
compose-from-choose = کسی اور اکاؤنٹ سے بھیجیں
compose-recipients = وصول کنندگان
compose-subject = موضوع

## Sending (the notes at the bottom of the window)

compose-open-elsewhere = پہلے کھلا ہوا پیغام بھیجیں یا رد کریں۔
compose-bad-address = ”{ $address }“ ای میل پتہ نہیں ہے۔
compose-no-recipients = کم از کم ایک وصول کنندہ شامل کریں۔
compose-attachments-too-large = اٹیچمنٹس { $size } ہیں؛ میل سرورز زیادہ سے زیادہ { $limit } قبول کرتے ہیں۔
compose-no-account = میل بھیجنے کے لیے ایک اکاؤنٹ شامل کریں۔
compose-past-time = مستقبل کا کوئی وقت منتخب کریں۔
compose-scheduling = شیڈول کیا جا رہا ہے…
compose-sending = بھیجا جا رہا ہے…
compose-scheduled = { $when } کو بھیجنے کے لیے شیڈول کیا گیا
compose-sent-archived = بھیج کر آرکائیو کر دیا گیا
compose-sent = پیغام بھیج دیا گیا
compose-discarded = ڈرافٹ رد کر دیا گیا
compose-draft-saved = ڈرافٹ محفوظ ہو گیا
compose-draft-failed = ڈرافٹ محفوظ نہیں ہو سکا: { $error }
compose-draft-not-opened = ڈرافٹ کھولا نہیں جا سکا۔

## Attachments

compose-picker-insert = داخل کریں
compose-picker-attach = منسلک کریں
compose-file-too-large = { $name } بہت بڑی ہے: ایک پیغام زیادہ سے زیادہ { $limit } لے جا سکتا ہے۔
compose-attachment-size = ({ $size })
compose-remove-attachment = اٹیچمنٹ ہٹائیں
compose-attachments-total = { $count ->
    [one] { $count } فائل، { $size }
   *[other] { $count } فائلیں، { $size }
}
compose-drive-note = { $name } کا سائز { $limit } سے زیادہ ہے، اس لیے اسے آپ کے Google Drive میں رکھا جاتا ہے اور پیغام میں اس کا لنک شامل ہوتا ہے۔
compose-drive-tip = آپ کے Google Drive میں؛ پیغام میں لنک شامل ہے
compose-drive-uploading = اپ لوڈ ہو رہا ہے { $percent }%
compose-drive-allow = Drive کی اجازت دیں
compose-drive-allow-tip = Google سے دوبارہ سائن ان کریں تاکہ Katna بڑی فائلیں آپ کے Drive میں رکھ سکے
compose-drive-retry = دوبارہ کوشش کریں
compose-drive-sends-when-uploaded = { $name } کے اپ لوڈ ہونے پر پیغام بھیجا جائے گا
compose-drive-not-uploaded = { $name } ابھی Google Drive میں نہیں ہے
compose-drive-share-failed = Google Drive میں فائلیں شیئر نہیں ہو سکیں: { $error }
compose-drive-share-title = فائلیں سب کے ساتھ شیئر کریں؟
compose-drive-share-text = { $count ->
    [one] Google Drive فائلیں { $addresses } کے ساتھ شیئر نہیں کر سکتا، جس کا Google اکاؤنٹ نہیں ہے۔ اس کے بجائے لنک رکھنے والا کوئی بھی انہیں کھول سکتا ہے۔
   *[other] Google Drive فائلیں { $addresses } کے ساتھ شیئر نہیں کر سکتا، جن کا Google اکاؤنٹ نہیں ہے۔ اس کے بجائے لنک رکھنے والا کوئی بھی انہیں کھول سکتا ہے۔
}
compose-drive-share-link = لنک کے ساتھ شیئر کریں
compose-drive-send-without = شیئر کیے بغیر بھیجیں
compose-drive-share-cancel = منسوخ کریں
compose-drive-card-detail = { $size } · Google Drive
compose-drop-files = فائلیں یہاں چھوڑیں
compose-drop-here = یہاں چھوڑیں
compose-paste-keep-formatting = فارمیٹنگ برقرار رکھیں
compose-paste-table = ٹیبل
compose-paste-picture = تصویر
compose-paste-plain-text = سادہ متن
compose-paste-inline = متن میں
compose-paste-attachment = اٹیچمنٹ

## Encryption and signing (the toggles by the recipients)

compose-encrypt = مرموز کریں
compose-encrypted = مرموز: صرف وصول کنندگان اسے پڑھ سکتے ہیں
compose-sign = دستخط کریں
compose-signed = دستخط شدہ: وصول کنندگان جانچ سکتے ہیں کہ یہ آپ کی طرف سے ہے
compose-track = کھولنے اور کلکس کو ٹریک کریں
compose-tracked = ٹریک ہو رہا ہے: آپ دیکھیں گے کہ ہر وصول کنندہ اسے کب کھولتا ہے یا لنک کھولتا ہے
compose-track-clicks = لنک کلکس کو ٹریک کریں (سادہ متن میں کھولنا نہیں دکھایا جا سکتا)
compose-tracked-clicks = ٹریک ہو رہا ہے: آپ دیکھیں گے کہ ہر وصول کنندہ کب لنک کھولتا ہے
compose-track-sign-in = کھولنے اور کلکس کو ٹریک کرنے کے لیے Katna اکاؤنٹ میں سائن ان کریں
compose-receipt = پڑھنے کی رسید کی درخواست کریں
compose-receipt-on = پڑھنے کی رسید کی درخواست کی گئی: وصول کنندہ کی ایپ اس سے رسید بھیجنے کو کہہ سکتی ہے
compose-delivery = ڈیلیوری کی رسید کی درخواست کریں
compose-delivery-on = ڈیلیوری کی رسید کی درخواست کی گئی: جب ہر وصول کنندہ کا سرور پیغام قبول کرے گا تو آپ کا میل سرور آپ کو ای میل بھیجے گا
compose-delivery-unavailable = آپ کا میل سرور ڈیلیوری کی رسیدیں نہیں بھیجتا

## Spelling

spell-no-dictionary = { $language } کے لیے کوئی املا لغت انسٹال نہیں ہے (مثال کے طور پر hunspell-en_us)۔
spell-dictionary-error = املا لغت: { $error }

## Grammar checking (the right-click menu on a grammar mistake)

grammar-replace = ”{ $words }“
grammar-add = ”{ $words }“ شامل کریں
grammar-remove = ”{ $words }“ ہٹائیں
grammar-ignore = نظر انداز کریں

## Send checks (asked before a message goes out)

send-check-attachment-title = کیا آپ فائلیں منسلک کرنا چاہتے تھے؟
send-check-attachment-text = آپ نے اٹیچمنٹ کا ذکر کیا، لیکن کچھ منسلک نہیں ہے۔
send-check-attach = فائل منسلک کریں
send-check-subject-title = موضوع کے بغیر بھیجیں؟
send-check-subject-text = اس پیغام کا کوئی موضوع نہیں ہے۔
send-check-add-subject = موضوع شامل کریں
send-check-send-anyway = پھر بھی بھیجیں
recipient-not-valid = درست ای میل پتہ نہیں
recipient-show-address = پتہ دکھائیں
recipient-remove = ہٹائیں
recipient-bad-title = پتہ چیک کریں
recipient-bad-text = ”{ $address }“ درست ای میل پتہ نہیں ہے۔ بھیجنے سے پہلے اسے درست کریں یا ہٹا دیں۔
recipient-bad-fix = درست کریں
